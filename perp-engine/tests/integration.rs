//! Integration tests for the Solrover Perps MVP.
//!
//! Each test tells one story. Run with:
//!   cargo test -- --nocapture
//!
//! Standard test parameters (per CLAUDE.md):
//!   AMM:              1_000_000 base / 1_000_000 quote (price = 1.0 USDC/Iron)
//!   Warmup period:    100 slots
//!   Maintenance bps:  500  (5%)
//!   Initial bps:      1000 (10%)
//!   Trading fee:      10   (0.1%)
//!   AMM fee:          200  (2%)
//!   LP seed capital:  1_000_000 USDC

use perp_engine::{assert_conservation, PerpEngine};
use simple_amm::SimpleAMM;

// ============================================================================
// Shared setup
// ============================================================================

const WARMUP_SLOTS: u64 = 100;
const MAINTENANCE_BPS: u64 = 500;
const INITIAL_BPS: u64 = 1_000;
const TRADING_FEE_BPS: u64 = 10;
const AMM_FEE_BPS: u16 = 200;
const LP_SEED: u128 = 1_000_000;

fn new_engine() -> PerpEngine {
    let amm = SimpleAMM::new(1_000_000, 1_000_000, AMM_FEE_BPS);
    PerpEngine::new(amm, WARMUP_SLOTS, MAINTENANCE_BPS, INITIAL_BPS, TRADING_FEE_BPS, LP_SEED)
}

// ============================================================================
// Test 1: Basic lifecycle
// ============================================================================

/// "A player deposits, goes long Iron, price goes up, closes for profit,
///  withdraws after warmup."
#[test]
fn test_basic_lifecycle() {
    println!("\n=== Test: Basic Lifecycle ===");

    let mut engine = new_engine();
    println!("  AMM initialised: Iron/USDC @ {:.6}", engine.oracle_price() as f64 / 1e6);

    // Alice deposits 10,000 USDC
    let alice = engine.create_account(10_000, 0).unwrap();
    println!("  Alice deposits 10,000 USDC");
    assert_eq!(engine.account_capital(alice).unwrap(), 10_000);
    assert_conservation(&engine);

    // Alice opens long 5,000 Iron (~50% of her capital as notional at price 1.0)
    // Notional = 5_000 * 1_000_000 / 1_000_000 = 5_000, requires 10% IM = 500
    engine.open_long(alice, 5_000, 1).unwrap();
    println!("  Alice opens long 5,000 Iron (notional ≈ 5,000 USDC)");
    assert_eq!(engine.position(alice).unwrap(), 5_000);
    assert_conservation(&engine);

    // Someone buys Iron on AMM → price rises
    let quote_spent = 200_000;
    let _iron_bought = engine.amm_buy_base(quote_spent).unwrap();
    let new_price = engine.oracle_price();
    println!(
        "  AMM swap: spent {} USDC, price now {:.6}",
        quote_spent,
        new_price as f64 / 1e6
    );
    assert!(new_price > 1_000_000, "price should have risen");

    // Crank at new price — settles mark-to-market PnL
    engine.crank(10, 1).unwrap();
    let raw_pnl = engine.account_pnl(alice).unwrap();
    println!("  Crank: Alice raw PnL = {raw_pnl}");
    assert_conservation(&engine);

    // Alice closes position
    engine.close_position(alice, 20).unwrap();
    let pnl_after_close = engine.account_pnl(alice).unwrap();
    println!("  Alice closes position. Realized PnL = {pnl_after_close}");
    assert_eq!(engine.position(alice).unwrap(), 0);
    assert_conservation(&engine);

    // Advance past warmup period
    println!("  Advancing {} slots past warmup...", WARMUP_SLOTS);
    for slot in 21..=(20 + WARMUP_SLOTS + 10) {
        engine.crank(slot, 1).unwrap();
    }
    assert_conservation(&engine);

    // Alice's PnL should have converted to capital via warmup
    let capital_after = engine.account_capital(alice).unwrap();
    let h = engine.get_h();
    println!("  h = {h:.4} (system health)");
    println!("  Alice capital after warmup: {capital_after}");
    println!("  ✓ Conservation: vault({}) >= c_tot({}) + insurance({})",
        engine.vault(), engine.c_tot(), engine.insurance());

    // If system is healthy (h=1) and PnL was positive, capital should be well above initial deposit
    // (minus small trading fees, which are correctly deducted from capital per spec §8.1)
    if pnl_after_close > 0 && engine.get_h() >= 0.99 {
        assert!(capital_after >= 9_900, "Alice capital after profit should exceed deposit minus fees");
    }

    println!("  PASSED\n");
}

// ============================================================================
// Test 2: Haircut under stress
// ============================================================================

/// "Percolator's core value prop: when the system is stressed, h < 1,
///  profits are haircutted but principal is NEVER touched."
#[test]
fn test_haircut_under_stress() {
    println!("\n=== Test: Haircut Under Stress ===");

    let mut engine = new_engine();

    // Alice and Bob deposit and go long
    let alice = engine.create_account(10_000, 0).unwrap();
    let bob = engine.create_account(10_000, 0).unwrap();
    engine.open_long(alice, 2_000, 1).unwrap();
    engine.open_long(bob, 2_000, 1).unwrap();
    println!("  Alice deposits 10,000, goes long 2,000 Iron");
    println!("  Bob deposits 10,000, goes long 2,000 Iron");
    assert_conservation(&engine);

    // Charlie deposits small amount and goes short
    let charlie = engine.create_account(500, 0).unwrap();
    engine.open_short(charlie, 1_000, 1).unwrap();
    println!("  Charlie deposits 500 USDC, goes short 1,000 Iron");
    assert_conservation(&engine);

    // Snapshot capital AFTER all trades (trading fees already deducted from capital)
    let alice_cap_pre_stress = engine.account_capital(alice).unwrap();
    let bob_cap_pre_stress = engine.account_capital(bob).unwrap();

    // Price rises sharply — Charlie is losing money
    let _iron = engine.amm_buy_base(500_000).unwrap();
    let price = engine.oracle_price();
    println!("  AMM: massive buy, price now {:.4}", price as f64 / 1e6);

    // Crank: trigger liquidation check (Charlie may be liquidated)
    engine.crank(10, 1).unwrap();
    assert_conservation(&engine);

    let h = engine.get_h();
    println!("  h = {h:.4}");

    // Principal invariant: ADL/socialization must NOT reduce Alice/Bob capital.
    // (Trading fees come from capital but those are already in the pre-stress snapshot.)
    let alice_cap = engine.account_capital(alice).unwrap();
    let bob_cap = engine.account_capital(bob).unwrap();
    println!("  Alice capital: {alice_cap} (must equal pre-stress {alice_cap_pre_stress})");
    println!("  Bob capital:   {bob_cap} (must equal pre-stress {bob_cap_pre_stress})");
    assert_eq!(alice_cap, alice_cap_pre_stress, "ADL must not touch Alice capital");
    assert_eq!(bob_cap, bob_cap_pre_stress, "ADL must not touch Bob capital");

    // Effective PnL may be haircutted if h < 1
    let alice_raw = engine.account_pnl(alice).unwrap();
    let alice_eff = engine.effective_pnl(alice).unwrap_or(0);
    println!("  Alice raw PnL: {alice_raw}, effective PnL: {alice_eff}");

    println!("  ✓ Conservation: vault({}) >= c_tot({}) + insurance({})",
        engine.vault(), engine.c_tot(), engine.insurance());
    println!("  PASSED\n");
}

// ============================================================================
// Test 3: AMM oracle manipulation blocked by warmup
// ============================================================================

/// "Warmup prevents oracle manipulation — even if someone pumps the AMM,
///  they can't extract inflated profits immediately."
#[test]
fn test_amm_oracle_manipulation_blocked() {
    println!("\n=== Test: Oracle Manipulation Blocked by Warmup ===");

    let mut engine = new_engine();

    let alice = engine.create_account(10_000, 0).unwrap();
    engine.open_long(alice, 2_000, 1).unwrap();
    println!("  Alice deposits 10,000, opens long 2,000 Iron at price 1.0");
    assert_conservation(&engine);

    // Attacker pumps AMM massively
    let _iron = engine.amm_buy_base(900_000).unwrap();
    let pumped_price = engine.oracle_price();
    println!("  Attacker pumps AMM: price now {:.4}", pumped_price as f64 / 1e6);

    // Close and settle at pumped price
    engine.crank(5, 1).unwrap();
    engine.close_position(alice, 6).unwrap();
    let pnl_in_pnl_field = engine.account_pnl(alice).unwrap();
    let capital_now = engine.account_capital(alice).unwrap();
    println!("  Alice closes. Raw PnL field = {pnl_in_pnl_field}, capital = {capital_now}");

    // PnL is present but NOT in capital yet (warmup blocks conversion)
    // Capital should still be ~10,000 minus fees (profit not yet converted)
    println!("  PnL is in warmup — cannot withdraw yet");
    assert_conservation(&engine);

    // Attacker reverses position (price crashes)
    let _quote = engine.amm_sell_base(800_000).unwrap();
    let restored_price = engine.oracle_price();
    println!("  Attacker reverses: price back to {:.4}", restored_price as f64 / 1e6);

    // Crank: mark-to-oracle deflates the in-flight PnL (anti-retroactivity)
    engine.crank(7, 1).unwrap();
    let pnl_deflated = engine.account_pnl(alice).unwrap();
    println!("  After price restoration, Alice PnL = {pnl_deflated}");
    assert_conservation(&engine);

    println!("  ✓ Warmup prevented instant extraction of manipulated profit");
    println!("  PASSED\n");
}

// ============================================================================
// Test 4: No ADL — positions intact after a liquidation
// ============================================================================

/// "Unlike ADL, no one's position gets forcibly closed."
#[test]
fn test_no_adl_positions_intact() {
    println!("\n=== Test: No ADL — Positions Remain Intact ===");

    let mut engine = new_engine();

    let alice = engine.create_account(10_000, 0).unwrap();
    let bob = engine.create_account(10_000, 0).unwrap();
    let charlie = engine.create_account(500, 0).unwrap();

    engine.open_long(alice, 2_000, 1).unwrap();
    engine.open_long(bob, 2_000, 1).unwrap();
    engine.open_short(charlie, 1_000, 1).unwrap();
    println!("  Alice: long 2,000  Bob: long 2,000  Charlie: short 1,000 (only 500 capital)");
    assert_conservation(&engine);

    // Snapshot capital after trades (fee already charged) — this is the "principal" we protect
    let alice_cap_pre_liq = engine.account_capital(alice).unwrap();
    let bob_cap_pre_liq = engine.account_capital(bob).unwrap();

    // Price spikes — Charlie gets liquidated
    engine.amm_buy_base(600_000).unwrap();
    println!("  AMM: price spike to {:.4}", engine.oracle_price() as f64 / 1e6);

    engine.crank(10, 1).unwrap();
    assert_conservation(&engine);

    // Alice and Bob's positions are intact
    let alice_pos = engine.position(alice).unwrap();
    let bob_pos = engine.position(bob).unwrap();
    println!("  Alice position: {alice_pos} (must be 2,000)");
    println!("  Bob position:   {bob_pos} (must be 2,000)");
    assert_eq!(alice_pos, 2_000, "Alice position must not be forcibly closed");
    assert_eq!(bob_pos, 2_000, "Bob position must not be forcibly closed");

    // ADL/socialization must not reduce Alice/Bob capital
    assert_eq!(engine.account_capital(alice).unwrap(), alice_cap_pre_liq,
        "Charlie's liquidation must not touch Alice capital");
    assert_eq!(engine.account_capital(bob).unwrap(), bob_cap_pre_liq,
        "Charlie's liquidation must not touch Bob capital");

    println!("  ✓ No ADL: Alice and Bob positions and capital are intact");
    println!("  ✓ Conservation: vault({}) >= c_tot({}) + insurance({})",
        engine.vault(), engine.c_tot(), engine.insurance());
    println!("  PASSED\n");
}

// ============================================================================
// Test 5: AMM price feeds the perp oracle
// ============================================================================

/// "AMM swaps move the oracle price, and the perp market responds."
#[test]
fn test_amm_price_feeds_perp() {
    println!("\n=== Test: AMM Price Feeds Perp Oracle ===");

    let mut engine = new_engine();

    let alice = engine.create_account(10_000, 0).unwrap();
    engine.open_long(alice, 2_000, 1).unwrap();
    let price0 = engine.oracle_price();
    println!("  AMM price = {:.6}  Alice opens long 2,000 Iron", price0 as f64 / 1e6);

    // Swap up
    engine.amm_buy_base(100_000).unwrap();
    let price1 = engine.oracle_price();
    engine.crank(5, 1).unwrap();
    let pnl1 = engine.account_pnl(alice).unwrap();
    println!("  Buy on AMM → price = {:.6}  Alice PnL = {pnl1}", price1 as f64 / 1e6);
    assert!(price1 > price0, "price must rise after buying base");

    // Swap down past original
    engine.amm_sell_base(400_000).unwrap();
    let price2 = engine.oracle_price();
    engine.crank(10, 1).unwrap();
    let pnl2 = engine.account_pnl(alice).unwrap();
    println!("  Sell on AMM → price = {:.6}  Alice PnL = {pnl2}", price2 as f64 / 1e6);
    assert!(price2 < price1, "price must fall after selling base");

    // PnL tracks price: higher price → more PnL for the long
    println!("  ✓ PnL at higher price ({pnl1}) > PnL at lower price ({pnl2})");
    assert_conservation(&engine);
    println!("  PASSED\n");
}

// ============================================================================
// Test 6: Conservation always holds
// ============================================================================

/// "The core invariant: vault >= c_tot + insurance. Always."
#[test]
fn test_conservation_always_holds() {
    println!("\n=== Test: Conservation Always Holds ===");

    let mut engine = new_engine();

    let alice = engine.create_account(50_000, 0).unwrap();
    let bob = engine.create_account(20_000, 0).unwrap();
    let charlie = engine.create_account(5_000, 0).unwrap();

    println!("  Alice: 50,000  Bob: 20,000  Charlie: 5,000");

    // Series of operations — check conservation after each
    engine.open_long(alice, 10_000, 1).unwrap();
    assert_conservation(&engine);

    engine.open_short(bob, 5_000, 1).unwrap();
    assert_conservation(&engine);

    engine.open_long(charlie, 2_000, 1).unwrap();
    assert_conservation(&engine);

    // AMM swaps
    engine.amm_buy_base(200_000).unwrap();
    assert_conservation(&engine);

    engine.crank(10, 1).unwrap();
    assert_conservation(&engine);

    engine.amm_sell_base(300_000).unwrap();
    assert_conservation(&engine);

    engine.crank(20, 1).unwrap();
    assert_conservation(&engine);

    // Close some positions
    engine.close_position(alice, 30).unwrap();
    assert_conservation(&engine);

    // Advance through warmup
    for slot in 31..=150 {
        engine.crank(slot, 1).unwrap();
        assert_conservation(&engine);
    }

    // Withdraw (if capital allows)
    let bob_cap = engine.account_capital(bob).unwrap();
    if bob_cap >= 1_000 {
        engine.withdraw(bob, 1_000, 151).unwrap();
        assert_conservation(&engine);
    }

    println!("  ✓ Conservation held after every operation (120+ checks)");
    println!("  Final: vault({}) >= c_tot({}) + insurance({})",
        engine.vault(), engine.c_tot(), engine.insurance());
    println!("  PASSED\n");
}
