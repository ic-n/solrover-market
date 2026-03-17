# CLAUDE.md — Project Context for AI Assistants

## What This Is

Solrover Perps MVP — a Rust workspace that wires a minimal perpetual futures market on top of the **Percolator** risk engine, using a simple AMM as the price oracle. The goal is a passing `cargo test` that demonstrates Percolator working end-to-end with Solrover's resource economy model.

This is a demo/proof-of-concept, not a production deployment. No Solana program, no frontend, no game integration yet.

---

## Project Structure

```
web3/
├── Cargo.toml                        # workspace root (excludes vendor/percolator)
├── claude.md                         # this file
├── roadmap.md                        # phased plan
├── simple-amm/
│   └── src/lib.rs                    # constant-product AMM (~200 lines, 8 unit tests)
├── perp-engine/
│   ├── src/lib.rs                    # PerpEngine wrapper + assert_conservation helper
│   └── tests/integration.rs         # 6 integration tests (all passing)
├── vendor/
│   └── percolator/                   # vendored risk engine (DO NOT MODIFY)
│       ├── src/percolator.rs
│       └── src/i128.rs
└── docs/integration/                 # reference docs
```

### Phase status

- **Phase 0** ✅ — workspace + vendor wired, `cargo build` clean
- **Phase 1** ✅ — `simple-amm` complete, all unit tests pass
- **Phase 2** ✅ — `perp-engine` wrapper complete
- **Phase 3** ✅ — all 6 integration tests pass (`cargo test -- --nocapture`)
- **Phase 4** — polish (README, clippy, fmt) — not yet done

---

## Workspace Notes

- `vendor/percolator` must be **excluded** from the workspace root (it declares its own workspace); add `exclude = ["vendor/percolator"]` to `Cargo.toml`.
- Use `features = ["test"]` on the percolator dependency to get `MAX_ACCOUNTS = 64` for tests.
- Set `max_crank_staleness_slots = u64::MAX` in `RiskParams` to disable the crank-freshness check in the test harness.

---

## Key Dependencies

- **Percolator** — vendored at `vendor/percolator/`. This is the risk engine. It is a pure Rust library, `no_std` compatible, zero runtime deps. **Do not modify vendored code.** If the API doesn't expose what you need, write adapter types in `perp-engine`.
- **No other external dependencies.** The AMM and engine wrapper are pure Rust. No async, no tokio, no serde (unless needed for test fixtures). Keep it minimal.

---

## Percolator Spec — Critical Rules

The spec lives at `vendor/percolator/spec.md`. It is **normative** — if the spec says MUST, the code must comply. Key things to internalize:

### Types and Scaling

- Prices: `u64`, quote per 1 base, scaled by `1e6`. So price = 1.0 USDC/Iron → `1_000_000u64`.
- Positions: `i128`, in base units. Positive = long, negative = short.
- Capital/PnL amounts: `u128` / `i128`, in quote token atomic units.
- Notional: `|pos| * price / 1e6` (computed in u128).
- All arithmetic must use checked/saturating ops. Overflow → error, never corrupt state.

### The h Ratio (§3.2)

```
Residual = max(0, V - C_tot - I)
h_num = min(Residual, PNL_pos_tot)
h_den = PNL_pos_tot
```

If `PNL_pos_tot == 0`, then `h = 1` (use `h_num = 1, h_den = 1`).

### Effective PnL (§3.3)

```
effective_pnl_i = floor(max(PNL_i, 0) * h_num / h_den)
```

All winners share the same proportional haircut. No rankings, no queues.

### Account Touch Order (§10.1) — MUST follow exactly

1. Set current_slot
2. Accrue global funding index, settle funding into PnL
3. Mark-to-oracle: settle unrealized PnL, update entry price
4. Charge maintenance fees
5. Settle losses (negative PnL pays from capital)
6. Convert warmable profits to capital (warmup)
7. Sweep fee debt from newly available capital

### Warmup (§5)

- Profits are NOT immediately withdrawable. They convert to capital over `warmup_period_slots`.
- Slope: `max(1, floor(AvailGross / T))` per slot.
- After mark-to-market increases AvailGross, slope MUST be updated BEFORE conversion.

### Trading Fees (§8.1)

- `fee = ceil(notional * fee_bps / 10_000)` — **ceiling division**, prevents micro-trade evasion.
- Fees go to insurance fund, NOT socialized through h.

### Margin (§9)

- Maintenance: `Eq_mtm_net > notional * maintenance_bps / 10_000`
- Initial (risk-increasing only): `Eq_mtm_net >= notional * initial_bps / 10_000`
- Risk-increasing = magnitude increases OR position flips sign.

### Trade Execution (§10.4)

1. Touch both accounts at oracle price
2. Apply position deltas
3. Compute zero-sum PnL, apply via `set_pnl`
4. Charge trading fees to insurance
5. Update warmup slopes if positive PnL increased
6. Update funding rate if inputs changed
7. Post-trade margin check (IM if risk-increasing, MM otherwise)
8. Fee-debt sweep

### Key Invariants (must hold after every operation)

- `V >= C_tot + I` (vault conservation)
- `sum(effective_pnl_i) <= Residual` (no over-withdrawal)
- One account's loss never touches another's capital (principal protection)

---

## SimpleAMM — Design Notes

Constant-product AMM (`xy = k`). Minimal, no frills.

```rust
pub struct SimpleAMM {
    reserve_base: u128,
    reserve_quote: u128,
    lp_supply: u128,
    fee_bps: u16,
}
```

### Price Output

`spot_price()` must return `u64` in Percolator format:

```rust
pub fn spot_price(&self) -> u64 {
    // quote per 1 base, scaled by 1e6
    (self.reserve_quote * 1_000_000 / self.reserve_base) as u64
}
```

### Swap Math

Standard constant-product with fee:

```
amount_in_after_fee = amount_in * (10_000 - fee_bps) / 10_000
amount_out = reserve_out * amount_in_after_fee / (reserve_in + amount_in_after_fee)
```

Update reserves after swap. Assert `k` doesn't decrease (rounding is OK).

---

## PerpEngine — Design Notes

This is a **thin orchestration layer**, not a reimplementation. Every method should map to a Percolator function call.

### LP Counterparty Model (MVP)

Use a single "house" LP account that takes the opposite side of every user trade. When Alice goes long 100 Iron, the LP goes short 100 Iron. PnL is zero-sum between them.

The LP account uses the same Percolator account mechanics (capital, PnL, warmup). For MVP, seed it with enough capital that it doesn't get liquidated in normal test scenarios.

### Slot Advancement

There's no real blockchain. The test harness controls time by passing `slot: u64` to every operation. Tests advance slots manually to simulate warmup passage, funding accrual, etc.

### Funding Rate (MVP)

Keep it simple. Options in order of preference:

1. Fixed rate (e.g., 1 bps/slot toward balanced OI) — simplest, fine for demo
2. Derived from LP inventory imbalance — more realistic but more code

For the tests to tell a good story, a fixed small rate is fine. The anti-retroactivity property is what matters, and that's already enforced by Percolator.

---

## Test Guidelines

### What the Tests Prove

Each test tells one story. The test name and println output should make the narrative obvious to someone reading terminal output. Use `println!` generously — this is the deliverable.

### Test Output Format

```rust
println!("\n=== Test: Haircut Under Stress ===");
println!("  Alice deposits 10,000 USDC");
// ... operations ...
println!("  h = {:.4} (system stressed)", engine.get_h());
println!("  Alice capital: {} ✓ (UNTOUCHED)", alice_capital);
println!("  ✓ Conservation: vault({}) >= c_tot({}) + insurance({})", v, c, i);
```

### Assertion Style

Always assert the Percolator invariants after state-changing operations:

```rust
fn assert_conservation(engine: &PerpEngine) {
    let v = engine.vault();
    let c = engine.c_tot();
    let i = engine.insurance();
    assert!(v >= c + i, "CONSERVATION VIOLATED: V={v}, C_tot={c}, I={i}");
}
```

### Test Parameters

Use round numbers that make the narrative readable:

- Initial AMM: `1_000_000` base / `1_000_000` quote (price = 1.0)
- Deposits: `10_000`, `50_000` — easy mental math
- Warmup period: `100` slots
- Maintenance margin: `500` bps (5%)
- Initial margin: `1000` bps (10%)
- Trading fee: `10` bps (0.1%)
- AMM fee: `200` bps (2%)

---

## Common Pitfalls

1. **Don't reimplement Percolator math.** If you find yourself writing haircut ratio calculations, stop. Call the Percolator function.

2. **`set_pnl` is mandatory.** Every PnL modification must go through `set_pnl` to maintain `PNL_pos_tot`. Direct field assignment will corrupt global aggregates.

3. **Touch order matters.** §10.1 specifies exact ordering. Funding before mark. Mark before fees. Fees before loss settlement. Loss settlement before conversion. Conversion before fee-debt sweep. Do not reorder.

4. **Ceiling division for fees.** `fee = (notional * bps + 9999) / 10000` — not floor. This prevents micro-trade fee evasion. If `bps > 0 && notional > 0`, fee must be `>= 1`.

5. **Warmup slope update before conversion.** If mark-to-market increases AvailGross, update slope FIRST, then convert. Stale slope can allow overwithdrawal of newly-realized profits.

6. **h uses pre-conversion state.** When converting profit to capital, compute `h_num/h_den` from global state BEFORE modifying PNL_i or C_i.

7. **Price scaling.** AMM spot_price returns `u64` scaled by `1e6`. Position sizes are in base units. Notional = `|pos| * price / 1e6`. Don't double-scale.

8. **Checked arithmetic everywhere.** Use `.checked_mul()`, `.checked_div()`, `.saturating_sub()`. Overflow must error, not wrap.

9. **Trading fees come from capital, not PnL.** `execute_trade` deducts `fee` from `user.capital` (spec §8.1). Test assertions that check "principal is untouched" should snapshot capital *after* trades, then compare before/after the stressful event — not against the raw deposit amount.

10. **NoOpMatcher → exec_price == oracle_price → trade PnL = 0.** With `NoOpMatcher`, `execute_trade` always sets `exec_price = oracle_price`. After `settle_mark_to_oracle`, `entry_price` is already at oracle, so `price_diff = 0` and trade PnL is zero. All profit/loss accrues via subsequent `settle_mark_to_oracle` calls (on touch or crank).

11. **`I128`/`U128` wrappers.** Percolator uses BPF-safe wrapper types. Access raw values with `.get()` (returns `i128`/`u128`). Construct with `I128::new(val)` / `U128::new(val)`. Constants: `I128::ZERO`, `U128::ZERO`.

---

## Build and Test Commands

```bash
# build everything
cargo build

# run all tests with narrative output
cargo test -- --nocapture

# run a specific test
cargo test test_haircut_under_stress -- --nocapture

# lint
cargo clippy -- -D warnings

# format
cargo fmt

# run Percolator's own verification (optional, requires kani)
cd vendor/percolator && cargo kani
```

---

## What Not to Build

- **No Solana program.** No Anchor, no account layouts, no PDAs. This is pure Rust library + tests.
- **No async.** No tokio, no runtime. Everything is synchronous.
- **No serialization.** No serde, no borsh. In-memory structs only.
- **No frontend.** The deliverable is `cargo test` output.
- **No multi-market.** One perp market (Iron/USDC). Multi-market is trivially multiple `PerpEngine` instances later.
- **No external oracle.** Price comes from SimpleAMM. No Pyth, no Switchboard.

---

## References

- `vendor/percolator/spec.md` — the normative spec. Read this before writing any engine code.
- `roadmap.md` — phased plan with exit criteria.
- `vendor/percolator/README.md` — high-level context on the ADL replacement model.
- Tarun Chitra, _Autodeleveraging: Impossibilities and Optimization_, arXiv:2512.01112, 2025.
