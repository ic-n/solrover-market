//! PerpEngine — thin orchestration layer that wires SimpleAMM as the oracle
//! into the Percolator risk engine.
//!
//! Every method maps ~1:1 to a Percolator spec section.  No math is
//! reimplemented here — we just call Percolator functions with the AMM price.

use std::collections::HashMap;

use percolator::{NoOpMatcher, RiskEngine, RiskError, RiskParams, U128};
use simple_amm::{AmmError, SimpleAMM};

pub type AccountId = u64;

// ============================================================================
// Error type
// ============================================================================

#[derive(Debug)]
pub enum EngineError {
    Risk(RiskError),
    Amm(AmmError),
    AccountNotFound,
    NoPosition,
}

impl From<RiskError> for EngineError {
    fn from(e: RiskError) -> Self {
        EngineError::Risk(e)
    }
}

impl From<AmmError> for EngineError {
    fn from(e: AmmError) -> Self {
        EngineError::Amm(e)
    }
}

// ============================================================================
// PerpEngine
// ============================================================================

/// Minimal perp market: one Iron/USDC market, LP as house counterparty.
pub struct PerpEngine {
    /// Percolator risk engine (holds all account state, vault, insurance)
    engine: Box<RiskEngine>,

    /// AMM that serves as the oracle price feed
    amm: SimpleAMM,

    /// Slab index of the house LP account
    lp_idx: u16,

    /// Map from our external AccountId → Percolator slab index
    accounts: HashMap<AccountId, u16>,

    /// Monotonically-increasing ID counter for user accounts
    next_id: AccountId,
}

impl PerpEngine {
    /// Create a new perp engine.
    ///
    /// - `amm`: initialised AMM (defines the starting oracle price)
    /// - `warmup_slots`: slots before PnL becomes withdrawable (spec §5)
    /// - `maintenance_bps`: maintenance margin in basis points (e.g. 500 = 5%)
    /// - `initial_bps`: initial margin in basis points (e.g. 1000 = 10%)
    /// - `trading_fee_bps`: fee charged per trade (e.g. 10 = 0.1%)
    /// - `lp_seed_capital`: USDC seeded into the LP account as its capital buffer
    pub fn new(
        amm: SimpleAMM,
        warmup_slots: u64,
        maintenance_bps: u64,
        initial_bps: u64,
        trading_fee_bps: u64,
        lp_seed_capital: u128,
    ) -> Self {
        assert!(
            maintenance_bps < initial_bps,
            "maintenance_bps must be < initial_bps"
        );

        let params = RiskParams {
            warmup_period_slots: warmup_slots,
            maintenance_margin_bps: maintenance_bps,
            initial_margin_bps: initial_bps,
            trading_fee_bps,
            max_accounts: percolator::MAX_ACCOUNTS as u64,
            new_account_fee: U128::ZERO,
            risk_reduction_threshold: U128::ZERO,
            maintenance_fee_per_slot: U128::ZERO,
            // u64::MAX disables the crank-freshness check — fine for test harness
            max_crank_staleness_slots: u64::MAX,
            liquidation_fee_bps: 50,
            liquidation_fee_cap: U128::new(1_000_000_000),
            liquidation_buffer_bps: 100,
            min_liquidation_abs: U128::new(1),
        };

        let mut engine = Box::new(RiskEngine::new(params));

        // Add house LP account (NoOpMatcher — zeroed matcher fields are ignored)
        let lp_idx = engine
            .add_lp([0u8; 32], [0u8; 32], 0)
            .expect("failed to create LP account");

        // Seed LP with enough capital to be the counterparty in normal tests
        engine
            .deposit(lp_idx, lp_seed_capital, 0)
            .expect("failed to seed LP capital");

        Self {
            engine,
            amm,
            lp_idx,
            accounts: HashMap::new(),
            next_id: 1,
        }
    }

    // -------------------------------------------------------------------------
    // Oracle
    // -------------------------------------------------------------------------

    /// Current oracle price from the AMM (Percolator format: quote/base * 1e6).
    pub fn oracle_price(&self) -> u64 {
        self.amm.spot_price()
    }

    // -------------------------------------------------------------------------
    // Account lifecycle
    // -------------------------------------------------------------------------

    /// Open a new account and deposit `amount` of quote token (USDC) capital.
    /// Returns the new AccountId.
    pub fn create_account(&mut self, amount: u128, slot: u64) -> Result<AccountId, EngineError> {
        let idx = self.engine.add_user(0)?;
        let id = self.next_id;
        self.next_id += 1;
        self.accounts.insert(id, idx);
        self.engine.deposit(idx, amount, slot)?;
        Ok(id)
    }

    /// Deposit additional capital into an existing account.
    pub fn deposit(
        &mut self,
        id: AccountId,
        amount: u128,
        slot: u64,
    ) -> Result<(), EngineError> {
        let idx = self.user_idx(id)?;
        Ok(self.engine.deposit(idx, amount, slot)?)
    }

    /// Withdraw `amount` of capital from an account (subject to margin checks).
    /// Percolator will run a full account touch (funding + mark + warmup) first.
    pub fn withdraw(
        &mut self,
        id: AccountId,
        amount: u128,
        slot: u64,
    ) -> Result<(), EngineError> {
        let idx = self.user_idx(id)?;
        let price = self.amm.spot_price();
        Ok(self.engine.withdraw(idx, amount, slot, price)?)
    }

    // -------------------------------------------------------------------------
    // Trading (spec §10.4)
    // -------------------------------------------------------------------------

    /// Open a long position of `size` base units for `id` vs the house LP.
    pub fn open_long(
        &mut self,
        id: AccountId,
        size: u128,
        slot: u64,
    ) -> Result<(), EngineError> {
        let idx = self.user_idx(id)?;
        let price = self.amm.spot_price();
        Ok(self.engine.execute_trade(
            &NoOpMatcher,
            self.lp_idx,
            idx,
            slot,
            price,
            size as i128,
        )?)
    }

    /// Open a short position of `size` base units for `id` vs the house LP.
    pub fn open_short(
        &mut self,
        id: AccountId,
        size: u128,
        slot: u64,
    ) -> Result<(), EngineError> {
        let idx = self.user_idx(id)?;
        let price = self.amm.spot_price();
        Ok(self.engine.execute_trade(
            &NoOpMatcher,
            self.lp_idx,
            idx,
            slot,
            price,
            -(size as i128),
        )?)
    }

    /// Close the entire position for `id` at the current oracle price.
    ///
    /// Execute in the opposite direction: if user is long X, trade -X.
    pub fn close_position(&mut self, id: AccountId, slot: u64) -> Result<(), EngineError> {
        let idx = self.user_idx(id)?;
        let current_pos = self.engine.accounts[idx as usize].position_size.get();
        if current_pos == 0 {
            return Err(EngineError::NoPosition);
        }
        let price = self.amm.spot_price();
        Ok(self.engine.execute_trade(
            &NoOpMatcher,
            self.lp_idx,
            idx,
            slot,
            price,
            -current_pos,
        )?)
    }

    // -------------------------------------------------------------------------
    // Keeper crank (spec §10.5)
    // -------------------------------------------------------------------------

    /// Advance the engine state: accrue funding, mark all accounts, liquidate
    /// undercollateralised positions.
    ///
    /// In the test harness, call this whenever you advance slots to keep the
    /// engine fresh and trigger warmup/liquidation logic.
    pub fn crank(&mut self, slot: u64, funding_rate_bps: i64) -> Result<(), EngineError> {
        let price = self.amm.spot_price();
        self.engine
            .keeper_crank(self.lp_idx, slot, price, funding_rate_bps, false)?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // AMM — swap to move the oracle price
    // -------------------------------------------------------------------------

    /// Spend `quote_in` USDC to buy base (Iron). Increases the oracle price.
    pub fn amm_buy_base(&mut self, quote_in: u128) -> Result<u128, EngineError> {
        Ok(self.amm.swap_quote_to_base(quote_in)?)
    }

    /// Sell `base_in` Iron to receive USDC. Decreases the oracle price.
    pub fn amm_sell_base(&mut self, base_in: u128) -> Result<u128, EngineError> {
        Ok(self.amm.swap_base_to_quote(base_in)?)
    }

    // -------------------------------------------------------------------------
    // Invariant accessors
    // -------------------------------------------------------------------------

    /// Total vault balance (all funds in the engine).
    pub fn vault(&self) -> u128 {
        self.engine.vault.get()
    }

    /// Sum of all account capital (C_tot).
    pub fn c_tot(&self) -> u128 {
        self.engine.c_tot.get()
    }

    /// Insurance fund balance.
    pub fn insurance(&self) -> u128 {
        self.engine.insurance_fund.balance.get()
    }

    /// Sum of all positive PnL across accounts (PNL_pos_tot).
    pub fn pnl_pos_tot(&self) -> u128 {
        self.engine.pnl_pos_tot.get()
    }

    /// h ratio numerator and denominator (spec §3.2).
    pub fn haircut_ratio(&self) -> (u128, u128) {
        self.engine.haircut_ratio()
    }

    /// h as a float — convenient for test output.
    pub fn get_h(&self) -> f64 {
        let (num, den) = self.haircut_ratio();
        if den == 0 {
            1.0
        } else {
            num as f64 / den as f64
        }
    }

    /// Capital held in an account.
    pub fn account_capital(&self, id: AccountId) -> Option<u128> {
        self.accounts
            .get(&id)
            .map(|&idx| self.engine.accounts[idx as usize].capital.get())
    }

    /// Raw (pre-haircut) PnL in an account.
    pub fn account_pnl(&self, id: AccountId) -> Option<i128> {
        self.accounts
            .get(&id)
            .map(|&idx| self.engine.accounts[idx as usize].pnl.get())
    }

    /// Effective (post-haircut) positive PnL for an account (spec §3.3).
    pub fn effective_pnl(&self, id: AccountId) -> Option<u128> {
        self.accounts.get(&id).map(|&idx| {
            let pnl = self.engine.accounts[idx as usize].pnl.get();
            self.engine.effective_pos_pnl(pnl)
        })
    }

    /// Position size (positive = long, negative = short, 0 = flat).
    pub fn position(&self, id: AccountId) -> Option<i128> {
        self.accounts
            .get(&id)
            .map(|&idx| self.engine.accounts[idx as usize].position_size.get())
    }

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

    fn user_idx(&self, id: AccountId) -> Result<u16, EngineError> {
        self.accounts
            .get(&id)
            .copied()
            .ok_or(EngineError::AccountNotFound)
    }
}

// ============================================================================
// Conservation invariant helper (used in tests)
// ============================================================================

/// Assert V >= C_tot + I. Panics with a descriptive message on violation.
pub fn assert_conservation(engine: &PerpEngine) {
    let v = engine.vault();
    let c = engine.c_tot();
    let i = engine.insurance();
    assert!(
        v >= c + i,
        "CONSERVATION VIOLATED: V={v}, C_tot={c}, I={i}"
    );
}
