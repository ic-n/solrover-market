//! Integration tests for the SolRover on-chain perp market.
//!
//! These tests run against a live Solana validator.
//! Start surfpool first:
//!   surfpool start --manifest-file-path ./txtx.yml
//! Then run:
//!   anchor test --skip-local-validator
//!
//! Or run against surfpool's simnet directly:
//!   ANCHOR_WALLET=~/.config/solana/id.json cargo test -- --nocapture
//!
//! Test parameters (match CLAUDE.md standard values):
//!   Oracle price: 1_000_000 (1.0 USDC/Iron, scaled by 1e6)
//!   Deposit:      10_000_000 lamports (10 mSOL for easy mental math)
//!   Position:     5_000 base units  (notional = 5_000 @ price 1.0)
//!   IM:           1_000 bps (10%)   → requires 500 lamports margin
//!   MM:             500 bps (5%)
//!   Fee:             10 bps (0.1%)  → 5 lamports on 5_000 notional

use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    Client, Cluster,
};
use perp_program::{Market, UserAccount};

// ─── helpers ────────────────────────────────────────────────────────────────

const PROGRAM_ID: &str = "7kzDLnYDxvtX5HAK2jPcGBqToTCv5L5eVXA4Wf7qBn1b";

/// Standard test params
const ORACLE_PRICE: u64 = 1_000_000; // 1.0 scaled by 1e6
const ORACLE_PRICE_HIGH: u64 = 1_100_000; // 1.1 — 10% up
const DEPOSIT: u64 = 10_000_000; // 10 mSOL
const POSITION_SIZE: i64 = 5_000; // base units
const MAINTENANCE_BPS: u64 = 500;
const INITIAL_BPS: u64 = 1_000;
const FEE_BPS: u64 = 10;

fn program_id() -> Pubkey {
    Pubkey::from_str(PROGRAM_ID).unwrap()
}

fn wallet() -> Keypair {
    let path = std::env::var("ANCHOR_WALLET")
        .unwrap_or_else(|_| shellexpand::tilde("~/.config/solana/id.json").to_string());
    read_keypair_file(&path).expect("failed to read wallet keypair")
}

fn market_pda(authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"market", authority.as_ref()], &program_id())
}

fn user_pda(market: &Pubkey, owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"user", market.as_ref(), owner.as_ref()],
        &program_id(),
    )
}

fn fetch_market(client: &anchor_client::Program<std::rc::Rc<Keypair>>, pda: Pubkey) -> Market {
    client.account::<Market>(pda).expect("market account not found")
}

fn fetch_user(
    client: &anchor_client::Program<std::rc::Rc<Keypair>>,
    pda: Pubkey,
) -> UserAccount {
    client.account::<UserAccount>(pda).expect("user account not found")
}

fn assert_conservation(m: &Market) {
    assert!(
        m.vault >= m.c_tot.saturating_add(m.insurance),
        "CONSERVATION VIOLATED: vault={} c_tot={} insurance={}",
        m.vault, m.c_tot, m.insurance,
    );
}

// ─── tests ──────────────────────────────────────────────────────────────────

/// Basic lifecycle: deposit → long → price rises → close → profit realized.
#[test]
fn test_basic_lifecycle() {
    println!("\n=== Test: Basic Lifecycle (on-chain validator) ===");

    let payer = std::rc::Rc::new(wallet());
    let client = Client::new_with_options(
        Cluster::Localnet,
        payer.clone(),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id()).unwrap();
    let authority = payer.pubkey();

    let (market_key, _) = market_pda(&authority);
    let (user_key, _) = user_pda(&market_key, &authority);

    // ── initialize ──────────────────────────────────────────────────────────
    let tx = program
        .request()
        .accounts(perp_program::accounts::Initialize {
            market: market_key,
            authority,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(perp_program::instruction::Initialize {
            oracle_price: ORACLE_PRICE,
            maintenance_bps: MAINTENANCE_BPS,
            initial_bps: INITIAL_BPS,
            fee_bps: FEE_BPS,
        })
        .send()
        .expect("initialize failed");
    println!("  Initialize tx: {tx}");

    let m = fetch_market(&program, market_key);
    assert_eq!(m.oracle_price, ORACLE_PRICE);
    assert_eq!(m.vault, 0);
    assert_conservation(&m);
    println!("  Market initialized: oracle={:.6}", m.oracle_price as f64 / 1e6);

    // ── deposit ─────────────────────────────────────────────────────────────
    let tx = program
        .request()
        .accounts(perp_program::accounts::Deposit {
            market: market_key,
            user_account: user_key,
            owner: authority,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(perp_program::instruction::Deposit { amount: DEPOSIT })
        .send()
        .expect("deposit failed");
    println!("  Deposit tx: {tx}");

    let m = fetch_market(&program, market_key);
    let u = fetch_user(&program, user_key);
    assert_eq!(u.capital, DEPOSIT);
    assert_eq!(m.vault, DEPOSIT);
    assert_eq!(m.c_tot, DEPOSIT);
    assert_conservation(&m);
    println!("  Deposited {} lamports | vault={}", DEPOSIT, m.vault);

    // ── open long ───────────────────────────────────────────────────────────
    let tx = program
        .request()
        .accounts(perp_program::accounts::OpenPosition {
            market: market_key,
            user_account: user_key,
            owner: authority,
        })
        .args(perp_program::instruction::OpenPosition { size: POSITION_SIZE })
        .send()
        .expect("open_position failed");
    println!("  OpenPosition tx: {tx}");

    let m = fetch_market(&program, market_key);
    let u = fetch_user(&program, user_key);
    assert_eq!(u.position, POSITION_SIZE);
    assert_conservation(&m);
    println!(
        "  Long {} base @ {:.6} | equity={} insurance={}",
        POSITION_SIZE,
        m.oracle_price as f64 / 1e6,
        u.capital,
        m.insurance,
    );

    // ── price rises: update oracle ──────────────────────────────────────────
    let tx = program
        .request()
        .accounts(perp_program::accounts::UpdateOracle {
            market: market_key,
            authority,
        })
        .args(perp_program::instruction::UpdateOracle {
            new_price: ORACLE_PRICE_HIGH,
        })
        .send()
        .expect("update_oracle failed");
    println!("  UpdateOracle tx: {tx}");

    // ── settle mark-to-oracle ───────────────────────────────────────────────
    let tx = program
        .request()
        .accounts(perp_program::accounts::SettleMark {
            market: market_key,
            user_account: user_key,
            owner: authority,
        })
        .args(perp_program::instruction::SettleMark {})
        .send()
        .expect("settle_mark failed");
    println!("  SettleMark tx: {tx}");

    let u = fetch_user(&program, user_key);
    // Expected unrealized PnL: (1_100_000 - 1_000_000) * 5_000 / 1_000_000 = 500
    let expected_pnl = 500i64;
    assert_eq!(
        u.unrealized_pnl, expected_pnl,
        "unrealized PnL should be {expected_pnl}"
    );
    println!("  Unrealized PnL = {} (expected {})", u.unrealized_pnl, expected_pnl);

    // ── close position ──────────────────────────────────────────────────────
    let capital_before_close = u.capital;
    let tx = program
        .request()
        .accounts(perp_program::accounts::ClosePosition {
            market: market_key,
            user_account: user_key,
            owner: authority,
        })
        .args(perp_program::instruction::ClosePosition {})
        .send()
        .expect("close_position failed");
    println!("  ClosePosition tx: {tx}");

    let m = fetch_market(&program, market_key);
    let u = fetch_user(&program, user_key);
    assert_eq!(u.position, 0);
    assert_conservation(&m);

    // Capital should have increased by ~500 (realized PnL) minus closing fee
    let notional_close = POSITION_SIZE.unsigned_abs() as u64 * ORACLE_PRICE_HIGH / 1_000_000;
    let close_fee = (notional_close * FEE_BPS + 9_999) / 10_000;
    let expected_capital_after = capital_before_close + 500 - close_fee;
    assert_eq!(u.capital, expected_capital_after);
    println!(
        "  Closed: realized_pnl=500 fee={} capital={} (expected {})",
        close_fee, u.capital, expected_capital_after,
    );
    println!(
        "  ✓ Conservation: vault({}) >= c_tot({}) + insurance({})",
        m.vault, m.c_tot, m.insurance,
    );
    println!("  PASSED\n");
}

/// Conservation invariant holds after deposit + open + close.
#[test]
fn test_conservation_invariant() {
    println!("\n=== Test: Conservation Invariant ===");

    let payer = std::rc::Rc::new(wallet());
    let client = Client::new_with_options(
        Cluster::Localnet,
        payer.clone(),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id()).unwrap();
    let authority = payer.pubkey();

    // Use a fresh market by deriving with a different seed using a fresh keypair
    // so this test is independent of test_basic_lifecycle.
    let fresh_auth = std::rc::Rc::new(Keypair::new());
    let fresh_program = client.program(program_id()).unwrap();
    let _ = fresh_program; // suppress unused warning
    let auth_key = fresh_auth.pubkey();

    // Fund fresh_auth from payer via the system program
    let transfer_ix = anchor_client::solana_sdk::system_instruction::transfer(
        &authority,
        &auth_key,
        50_000_000, // 50 mSOL for rent + deposits
    );
    program
        .request()
        .instruction(transfer_ix)
        .send()
        .expect("fund fresh_auth failed");

    let fresh_client = Client::new_with_options(
        Cluster::Localnet,
        fresh_auth.clone(),
        CommitmentConfig::confirmed(),
    );
    let prog = fresh_client.program(program_id()).unwrap();

    let (mkt, _) = market_pda(&auth_key);
    let (usr, _) = user_pda(&mkt, &auth_key);

    // Initialize
    prog.request()
        .accounts(perp_program::accounts::Initialize {
            market: mkt,
            authority: auth_key,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(perp_program::instruction::Initialize {
            oracle_price: ORACLE_PRICE,
            maintenance_bps: MAINTENANCE_BPS,
            initial_bps: INITIAL_BPS,
            fee_bps: FEE_BPS,
        })
        .send()
        .expect("initialize");

    assert_conservation(&fetch_market(&prog, mkt));
    println!("  After initialize: ✓");

    // Deposit
    prog.request()
        .accounts(perp_program::accounts::Deposit {
            market: mkt,
            user_account: usr,
            owner: auth_key,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(perp_program::instruction::Deposit { amount: DEPOSIT })
        .send()
        .expect("deposit");

    assert_conservation(&fetch_market(&prog, mkt));
    println!("  After deposit: ✓");

    // Open long
    prog.request()
        .accounts(perp_program::accounts::OpenPosition {
            market: mkt,
            user_account: usr,
            owner: auth_key,
        })
        .args(perp_program::instruction::OpenPosition { size: POSITION_SIZE })
        .send()
        .expect("open_position");

    assert_conservation(&fetch_market(&prog, mkt));
    println!("  After open_position: ✓");

    // Oracle up
    prog.request()
        .accounts(perp_program::accounts::UpdateOracle {
            market: mkt,
            authority: auth_key,
        })
        .args(perp_program::instruction::UpdateOracle {
            new_price: ORACLE_PRICE_HIGH,
        })
        .send()
        .expect("update_oracle");

    assert_conservation(&fetch_market(&prog, mkt));
    println!("  After update_oracle (price +10%): ✓");

    // Close
    prog.request()
        .accounts(perp_program::accounts::ClosePosition {
            market: mkt,
            user_account: usr,
            owner: auth_key,
        })
        .args(perp_program::instruction::ClosePosition {})
        .send()
        .expect("close_position");

    let m = fetch_market(&prog, mkt);
    assert_conservation(&m);
    println!("  After close_position: ✓");
    println!(
        "  Final: vault={} c_tot={} insurance={}",
        m.vault, m.c_tot, m.insurance,
    );
    println!("  PASSED\n");
}

/// Margin check: opening an oversized position must be rejected.
#[test]
fn test_margin_check_rejects_oversized_position() {
    println!("\n=== Test: Margin Check Rejects Oversized Position ===");

    let payer = std::rc::Rc::new(wallet());
    let client = Client::new_with_options(
        Cluster::Localnet,
        payer.clone(),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id()).unwrap();
    let authority = payer.pubkey();

    let fresh_auth = std::rc::Rc::new(Keypair::new());
    let auth_key = fresh_auth.pubkey();

    // Fund fresh authority
    let transfer_ix = anchor_client::solana_sdk::system_instruction::transfer(
        &authority,
        &auth_key,
        50_000_000,
    );
    program
        .request()
        .instruction(transfer_ix)
        .send()
        .expect("fund");

    let fresh_client = Client::new_with_options(
        Cluster::Localnet,
        fresh_auth,
        CommitmentConfig::confirmed(),
    );
    let prog = fresh_client.program(program_id()).unwrap();

    let (mkt, _) = market_pda(&auth_key);
    let (usr, _) = user_pda(&mkt, &auth_key);

    prog.request()
        .accounts(perp_program::accounts::Initialize {
            market: mkt,
            authority: auth_key,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(perp_program::instruction::Initialize {
            oracle_price: ORACLE_PRICE,
            maintenance_bps: MAINTENANCE_BPS,
            initial_bps: INITIAL_BPS,
            fee_bps: FEE_BPS,
        })
        .send()
        .expect("initialize");

    // Deposit small amount — 1_000 lamports
    prog.request()
        .accounts(perp_program::accounts::Deposit {
            market: mkt,
            user_account: usr,
            owner: auth_key,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(perp_program::instruction::Deposit { amount: 1_000 })
        .send()
        .expect("deposit");

    // Try to open a huge position: notional = 100_000 * 1_000_000 / 1_000_000 = 100_000
    // IM required = 100_000 * 10% = 10_000 >> 1_000 capital  → must fail
    let result = prog
        .request()
        .accounts(perp_program::accounts::OpenPosition {
            market: mkt,
            user_account: usr,
            owner: auth_key,
        })
        .args(perp_program::instruction::OpenPosition { size: 100_000 })
        .send();

    assert!(result.is_err(), "oversized position should have been rejected");
    println!("  ✓ Oversized position correctly rejected (IM check enforced)");
    println!("  PASSED\n");
}
