//! SolRover Perp Market — on-chain Anchor program
//!
//! A minimal perpetual futures market that mirrors the Percolator spec
//! (CLAUDE.md) in on-chain form:
//!   - Price scaled by 1e6 (same as Percolator)
//!   - Ceiling-division fees (spec §8.1)
//!   - IM/MM margin checks (spec §9)
//!   - Vault conservation: V >= C_tot + I at every state change
//!   - Zero-sum PnL between user and implicit LP (vault residual)

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("7kzDLnYDxvtX5HAK2jPcGBqToTCv5L5eVXA4Wf7qBn1b");

const PRICE_SCALE: u64 = 1_000_000; // quote per 1 base, matches Percolator

// ============================================================================
// Error codes
// ============================================================================

#[error_code]
pub enum PerpError {
    #[msg("Insufficient initial margin")]
    InsufficientInitialMargin,
    #[msg("Insufficient maintenance margin")]
    InsufficientMaintenanceMargin,
    #[msg("No open position")]
    NoOpenPosition,
    #[msg("Insufficient capital")]
    InsufficientCapital,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Vault conservation violated: vault < c_tot + insurance")]
    ConservationViolated,
    #[msg("Oracle price must be > 0")]
    InvalidPrice,
    #[msg("Position size must be non-zero")]
    ZeroSize,
    #[msg("Maintenance bps must be less than initial bps")]
    InvalidMarginParams,
}

// ============================================================================
// State accounts
// ============================================================================

#[account]
pub struct Market {
    pub authority: Pubkey,    // 32 — who can update oracle & params
    pub oracle_price: u64,    //  8 — current price, quote/base * 1e6
    pub vault: u64,           //  8 — total lamports deposited
    pub insurance: u64,       //  8 — insurance fund (fee sink)
    pub c_tot: u64,           //  8 — sum of all user capitals
    pub pnl_pos_tot: i64,     //  8 — sum of all positive unrealized PnL
    pub maintenance_bps: u64, //  8 — MM in basis points (e.g. 500 = 5%)
    pub initial_bps: u64,     //  8 — IM in basis points (e.g. 1000 = 10%)
    pub fee_bps: u64,         //  8 — trading fee in bps (e.g. 10 = 0.1%)
    pub bump: u8,             //  1
}                             // = 97 bytes data + 8 discriminator = 105

#[account]
pub struct UserAccount {
    pub market: Pubkey,       // 32 — which market this account belongs to
    pub owner: Pubkey,        // 32 — signer / owner
    pub capital: u64,         //  8 — deposited lamports (principal + settled PnL)
    pub position: i64,        //  8 — base units, positive=long negative=short
    pub entry_price: u64,     //  8 — oracle price when position was opened
    pub unrealized_pnl: i64,  //  8 — last settled mark-to-oracle PnL
    pub bump: u8,             //  1
}                             // = 97 bytes data + 8 discriminator = 105

// ============================================================================
// Program
// ============================================================================

#[program]
pub mod perp_program {
    use super::*;

    /// Create a new perp market.
    pub fn initialize(
        ctx: Context<Initialize>,
        oracle_price: u64,
        maintenance_bps: u64,
        initial_bps: u64,
        fee_bps: u64,
    ) -> Result<()> {
        require!(oracle_price > 0, PerpError::InvalidPrice);
        require!(maintenance_bps < initial_bps, PerpError::InvalidMarginParams);

        let m = &mut ctx.accounts.market;
        m.authority = ctx.accounts.authority.key();
        m.oracle_price = oracle_price;
        m.vault = 0;
        m.insurance = 0;
        m.c_tot = 0;
        m.pnl_pos_tot = 0;
        m.maintenance_bps = maintenance_bps;
        m.initial_bps = initial_bps;
        m.fee_bps = fee_bps;
        m.bump = ctx.bumps.market;

        msg!(
            "Market initialized: oracle={} IM={}bps MM={}bps fee={}bps",
            oracle_price, initial_bps, maintenance_bps, fee_bps
        );
        Ok(())
    }

    /// Deposit lamports into the vault and credit user capital.
    /// Creates the UserAccount PDA on first deposit (init_if_needed).
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, PerpError::InsufficientCapital);

        // Transfer SOL from user to market PDA (vault)
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.market.to_account_info(),
                },
            ),
            amount,
        )?;

        // Init user account fields on first deposit
        let user = &mut ctx.accounts.user_account;
        if user.owner == Pubkey::default() {
            user.market = ctx.accounts.market.key();
            user.owner = ctx.accounts.owner.key();
            user.bump = ctx.bumps.user_account;
        }
        user.capital = user.capital.checked_add(amount).ok_or(PerpError::Overflow)?;

        let m = &mut ctx.accounts.market;
        m.vault = m.vault.checked_add(amount).ok_or(PerpError::Overflow)?;
        m.c_tot = m.c_tot.checked_add(amount).ok_or(PerpError::Overflow)?;

        check_conservation(m)?;
        msg!("Deposit {} lamports | capital={} vault={}", amount, user.capital, m.vault);
        Ok(())
    }

    /// Withdraw lamports from vault back to user.
    /// If a position is open, checks MM margin is still satisfied post-withdrawal.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user_account;
        require!(user.capital >= amount, PerpError::InsufficientCapital);

        // MM check if position is open
        if user.position != 0 {
            let oracle = ctx.accounts.market.oracle_price;
            let notional = compute_notional(user.position.unsigned_abs(), oracle)?;
            let equity_after = user.capital.saturating_sub(amount) as i64
                + user.unrealized_pnl;
            let mm_req = (notional as i64)
                .checked_mul(ctx.accounts.market.maintenance_bps as i64)
                .ok_or(PerpError::Overflow)?
                / 10_000;
            require!(equity_after >= mm_req, PerpError::InsufficientMaintenanceMargin);
        }

        // Transfer lamports out of market PDA to user
        **ctx.accounts.market.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += amount;

        let m = &mut ctx.accounts.market;
        let user = &mut ctx.accounts.user_account;
        user.capital -= amount;
        m.vault -= amount;
        m.c_tot -= amount;

        check_conservation(m)?;
        msg!("Withdraw {} lamports | capital={} vault={}", amount, user.capital, m.vault);
        Ok(())
    }

    /// Open a position. Positive size = long, negative = short.
    /// Charges trading fee (ceiling division, spec §8.1) and checks IM (spec §9).
    /// MVP: only one position at a time per user.
    pub fn open_position(ctx: Context<OpenPosition>, size: i64) -> Result<()> {
        require!(size != 0, PerpError::ZeroSize);
        require!(
            ctx.accounts.user_account.position == 0,
            PerpError::NoOpenPosition
        );

        let oracle = ctx.accounts.market.oracle_price;
        let abs_size = size.unsigned_abs();
        let notional = compute_notional(abs_size, oracle)?;
        let fee = compute_fee(notional, ctx.accounts.market.fee_bps)?;
        let im_req =
            notional * ctx.accounts.market.initial_bps / 10_000;

        let user = &mut ctx.accounts.user_account;
        require!(user.capital >= fee, PerpError::InsufficientInitialMargin);
        user.capital -= fee;
        require!(user.capital >= im_req, PerpError::InsufficientInitialMargin);

        user.position = size;
        user.entry_price = oracle;
        user.unrealized_pnl = 0;

        let m = &mut ctx.accounts.market;
        m.c_tot -= fee;
        m.insurance += fee;

        check_conservation(m)?;
        msg!(
            "Open pos size={} entry={} notional={} fee={} equity={}",
            size, oracle, notional, fee, user.capital
        );
        Ok(())
    }

    /// Close the entire position and realize PnL.
    /// PnL = (close_price − entry_price) × position / 1e6
    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        let user_pos = ctx.accounts.user_account.position;
        require!(user_pos != 0, PerpError::NoOpenPosition);

        let oracle = ctx.accounts.market.oracle_price;
        let fee_bps = ctx.accounts.market.fee_bps;
        let abs_size = user_pos.unsigned_abs();
        let notional = compute_notional(abs_size, oracle)?;
        let fee = compute_fee(notional, fee_bps)?;

        // Realized PnL: positive = profit, negative = loss
        let entry = ctx.accounts.user_account.entry_price as i64;
        let close = oracle as i64;
        let realized: i64 = (close - entry)
            .checked_mul(user_pos)
            .ok_or(PerpError::Overflow)?
            .checked_div(PRICE_SCALE as i64)
            .ok_or(PerpError::Overflow)?;

        let user = &mut ctx.accounts.user_account;
        if realized >= 0 {
            user.capital = user
                .capital
                .checked_add(realized as u64)
                .ok_or(PerpError::Overflow)?;
        } else {
            user.capital = user.capital.saturating_sub(realized.unsigned_abs());
        }
        user.capital = user.capital.saturating_sub(fee);
        user.position = 0;
        user.entry_price = 0;
        user.unrealized_pnl = 0;

        let m = &mut ctx.accounts.market;
        // Zero-sum: user gain/loss comes from/goes to LP residual via c_tot
        if realized >= 0 {
            m.c_tot = m.c_tot.checked_add(realized as u64).ok_or(PerpError::Overflow)?;
        } else {
            m.c_tot = m.c_tot.saturating_sub(realized.unsigned_abs());
        }
        m.c_tot = m.c_tot.saturating_sub(fee);
        m.insurance = m.insurance.checked_add(fee).ok_or(PerpError::Overflow)?;

        check_conservation(m)?;
        msg!(
            "Close pos entry={} close={} realized={} fee={} capital={}",
            entry, close, realized, fee, user.capital
        );
        Ok(())
    }

    /// Update oracle price (authority only).
    pub fn update_oracle(ctx: Context<UpdateOracle>, new_price: u64) -> Result<()> {
        require!(new_price > 0, PerpError::InvalidPrice);
        ctx.accounts.market.oracle_price = new_price;
        msg!("Oracle → {}", new_price);
        Ok(())
    }

    /// Settle mark-to-oracle: update unrealized_pnl for an open position.
    /// Lamports do not move — PnL is realized on close_position.
    pub fn settle_mark(ctx: Context<SettleMark>) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        if user.position == 0 {
            return Ok(());
        }
        let oracle = ctx.accounts.market.oracle_price as i64;
        let entry = user.entry_price as i64;
        user.unrealized_pnl = (oracle - entry)
            .checked_mul(user.position)
            .ok_or(PerpError::Overflow)?
            .checked_div(PRICE_SCALE as i64)
            .ok_or(PerpError::Overflow)?;
        msg!("Settle mark: unrealized_pnl={}", user.unrealized_pnl);
        Ok(())
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn compute_notional(abs_size: u64, price: u64) -> Result<u64> {
    abs_size
        .checked_mul(price)
        .ok_or(error!(PerpError::Overflow))?
        .checked_div(PRICE_SCALE)
        .ok_or(error!(PerpError::Overflow))
}

/// Ceiling division: ceil(notional * fee_bps / 10_000) — spec §8.1
fn compute_fee(notional: u64, fee_bps: u64) -> Result<u64> {
    if fee_bps == 0 || notional == 0 {
        return Ok(0);
    }
    notional
        .checked_mul(fee_bps)
        .ok_or(error!(PerpError::Overflow))
        .map(|n| (n + 9_999) / 10_000)
}

/// V >= C_tot + I — the core conservation invariant
fn check_conservation(m: &Market) -> Result<()> {
    require!(
        m.vault >= m.c_tot.saturating_add(m.insurance),
        PerpError::ConservationViolated
    );
    Ok(())
}

// ============================================================================
// Instruction contexts
// ============================================================================

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 97,
        seeds = [b"market", authority.key().as_ref()],
        bump,
    )]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"market", market.authority.as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 97,
        seeds = [b"user", market.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"market", market.authority.as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"user", market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(
        mut,
        seeds = [b"market", market.authority.as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"user", market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(
        mut,
        seeds = [b"market", market.authority.as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"user", market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateOracle<'info> {
    #[account(
        mut,
        seeds = [b"market", market.authority.as_ref()],
        bump = market.bump,
        has_one = authority,
    )]
    pub market: Account<'info, Market>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SettleMark<'info> {
    #[account(
        seeds = [b"market", market.authority.as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"user", market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub owner: Signer<'info>,
}
