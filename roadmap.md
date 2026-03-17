# Solrover Perps MVP — Roadmap

**Goal:** A `cargo test` that passes in terminal, demonstrating a minimal perpetual futures market powered by Percolator with an AMM-derived price oracle. Something postable for feedback — proof the engine works end-to-end with Solrover's resource economy.

**Non-goals (for now):** Solana deployment, frontend, game integration, multi-market.

---

## Architecture (MVP)

```
┌──────────────────────────────┐
│       Test Harness           │
│  (cargo test, integration)   │
└──────────┬───────────────────┘
           │
    ┌──────▼──────┐      price feed      ┌─────────────────┐
    │  Percolator  │◄────────────────────│   SimpleAMM      │
    │  Risk Engine │                      │  (xy=k pool)    │
    │  (vendored)  │                      │  Iron / USDC    │
    └──────────────┘                      └─────────────────┘
```

Three crates, one workspace:

| Crate         | What it does                                                                                | Lines (est.) |
| ------------- | ------------------------------------------------------------------------------------------- | ------------ |
| `simple-amm`  | Constant-product AMM (`xy=k`). Swap, add/remove liquidity, quote price. Pure Rust, no deps. | ~200–300     |
| `perp-engine` | Thin wrapper that wires AMM price into Percolator. Deposit, withdraw, trade, crank.         | ~400–600     |
| `tests/`      | Integration tests that tell the story end-to-end.                                           | ~300–500     |

**Total new code: ~900–1400 lines of Rust.**

---

## Phases

### Phase 0 — Workspace + Vendor Percolator

**Time: day 1**

- [ ] Create cargo workspace: `solrover-perps/`
- [ ] Vendor Percolator as a dependency (git submodule or path dep from `vendor/percolator`)
- [ ] Confirm `cargo build` compiles clean
- [ ] Confirm Percolator types are importable (`use percolator::*`)

**Exit criteria:** `cargo build` succeeds, Percolator types accessible.

---

### Phase 1 — SimpleAMM

**Time: 1–2 days**

A minimal constant-product AMM. This is the price oracle for the perp market.

```rust
pub struct SimpleAMM {
    reserve_base: u128,   // e.g. Iron
    reserve_quote: u128,  // e.g. USDC
    lp_supply: u128,
    fee_bps: u16,         // e.g. 200 = 2%
}
```

**Functions:**

| Function                        | What                                                        |
| ------------------------------- | ----------------------------------------------------------- |
| `new(base, quote, fee_bps)`     | Initialize pool with reserves                               |
| `swap_base_to_quote(amount_in)` | Sell base, get quote                                        |
| `swap_quote_to_base(amount_in)` | Buy base, spend quote                                       |
| `spot_price() -> u64`           | Current price in Percolator format (`quote_per_base * 1e6`) |
| `add_liquidity(base, quote)`    | Deposit both sides, get LP tokens                           |
| `remove_liquidity(lp_amount)`   | Burn LP tokens, get reserves back                           |

**Key detail:** `spot_price()` returns the value Percolator expects as `oracle_price` — a `u64` scaled by `1e6`.

**Tests (unit):**

- [ ] Swap moves price correctly
- [ ] Price after large buy > price before
- [ ] Fees are collected (output < theoretical no-fee output)
- [ ] `spot_price()` matches `reserve_quote * 1e6 / reserve_base`

**Exit criteria:** `cargo test -p simple-amm` all green.

---

### Phase 2 — PerpEngine wrapper

**Time: 2–3 days**

Thin orchestration layer. This is NOT reimplementing Percolator — it's calling Percolator's functions with the AMM price plugged in.

```rust
pub struct PerpEngine {
    // Percolator state
    engine: percolator::EngineState,  // vault, c_tot, pnl_pos_tot, insurance, funding...
    accounts: HashMap<AccountId, percolator::AccountState>,

    // Oracle source
    amm: SimpleAMM,

    // Config
    warmup_slots: u64,
    maintenance_bps: u16,
    initial_bps: u16,
    trading_fee_bps: u16,
}
```

**Operations (each maps ~1:1 to spec §10):**

| Method                       | Spec section | What it does                                        |
| ---------------------------- | ------------ | --------------------------------------------------- |
| `deposit(id, amount)`        | §10.2        | Transfer to vault, credit capital                   |
| `withdraw(id, amount, slot)` | §10.3        | Touch account, check margin, return capital         |
| `open_long(id, size, slot)`  | §10.4        | Execute trade: user goes long vs LP account         |
| `open_short(id, size, slot)` | §10.4        | Execute trade: user goes short vs LP account        |
| `close_position(id, slot)`   | §10.4        | Close at oracle, settle PnL                         |
| `crank(slot)`                | §10.5        | Accrue funding, touch accounts, liquidate if needed |
| `amm_swap(...)`              | —            | Swap on AMM (changes oracle price for perp)         |
| `get_oracle_price()`         | —            | `amm.spot_price()`                                  |
| `get_health_ratio()`         | §3.2         | Return current `h`                                  |
| `get_account_equity(id)`     | §3.3         | Effective equity after haircut                      |

**The LP counterparty:** For MVP, use a single "house" LP account that takes the other side of every trade. This is how most perp DEXs bootstrap — the LP pool is the counterparty. Percolator already supports LP accounts with the same capital/PnL mechanics.

**Implementation detail — trade flow:**

```
1. Read oracle_price from AMM
2. touch_account_full(trader, oracle_price, slot)
3. touch_account_full(lp, oracle_price, slot)
4. Apply position deltas (trader gets +size, LP gets -size)
5. Charge trading fee to insurance (ceil division per §8.1)
6. Update warmup slopes if PnL increased
7. Post-trade margin check (IM if risk-increasing, MM otherwise)
```

This is exactly spec §10.4 — nothing invented, just plumbing.

**Exit criteria:** `cargo test -p perp-engine` compiles and basic deposit/withdraw works.

---

### Phase 3 — Integration Tests (the demo)

**Time: 2–3 days**

These are the tests you post. Each tells a clear story.

#### Test 1: `test_basic_lifecycle`

> "A player deposits, goes long Iron, price goes up, closes for profit, withdraws after warmup."

```
1. Initialize AMM: 1M Iron / 1M USDC (price = 1.0)
2. Alice deposits 10,000 USDC into perp vault
3. Alice opens 5x long Iron (50,000 notional)
4. Simulate: someone buys Iron on AMM → price rises to 1.20
5. Crank at new price
6. Alice closes position → PnL is positive
7. Advance slots past warmup period
8. Crank again (converts profit to capital)
9. Alice withdraws → gets back deposit + haircutted profit
10. Assert: vault balance >= c_tot + insurance (conservation)
```

#### Test 2: `test_haircut_under_stress`

> "Percolator's core value prop: when the system is stressed, h < 1, profits are haircutted but principal is NEVER touched."

```
1. Alice deposits 10,000, goes long
2. Bob deposits 10,000, goes long
3. Charlie deposits 1,000, goes short (undercollateralized counterparty)
4. Price rises sharply → Charlie is liquidated, loss exceeds his capital
5. Assert: h < 1 (system stressed)
6. Assert: Alice.capital == 10,000 (principal untouched!)
7. Assert: Bob.capital == 10,000 (principal untouched!)
8. Assert: Alice.effective_pnl < Alice.raw_pnl (haircutted)
9. Assert: vault >= c_tot + insurance (conservation holds)
```

#### Test 3: `test_amm_oracle_manipulation_blocked`

> "Warmup prevents oracle manipulation — even if someone pumps the AMM, they can't extract inflated profits immediately."

```
1. Alice deposits 10,000, opens long
2. Attacker does massive AMM buy → price spikes to 5.0
3. Alice tries to close + withdraw immediately
4. Assert: profit is in PnL but NOT yet in capital (warmup blocks it)
5. Attacker reverses AMM trade → price returns to 1.0
6. Crank at restored price → Alice's PnL deflates
7. Assert: Alice could not extract the inflated profit
```

#### Test 4: `test_no_adl_positions_intact`

> "Unlike ADL, no one's position gets forcibly closed."

```
1. Setup: multiple traders, one goes bust
2. Liquidation happens via crank
3. Assert: all other traders' positions are INTACT
4. Assert: all other traders' capital is INTACT
5. Assert: only h changed (profits haircutted, not positions closed)
```

#### Test 5: `test_amm_price_feeds_perp`

> "AMM swaps move the oracle price, and the perp market responds."

```
1. AMM price = 1.0, Alice is long
2. Swap on AMM → price = 1.10
3. Crank → Alice's mark-to-market equity increases
4. Swap on AMM → price = 0.90
5. Crank → Alice's equity decreases
6. Assert: perp PnL tracks AMM price movement
```

#### Test 6: `test_conservation_always_holds`

> "The core invariant: vault >= c_tot + insurance. Always."

```
1. Run 100 random operations (deposits, trades, swaps, cranks, withdrawals)
2. After EVERY operation, assert: V >= C_tot + I
3. After EVERY operation, assert: sum(effective_pnl) <= Residual
```

**Exit criteria:** `cargo test` — all 6 integration tests green. Terminal output tells the story.

---

### Phase 4 — Polish for posting

**Time: 1 day**

- [ ] Clean up test output with `println!` narrative (so terminal tells a story)
- [ ] Add a top-level README explaining what this demonstrates
- [ ] Make sure `cargo test -- --nocapture` produces readable output like:

```
=== Solrover Perps MVP: Percolator + AMM Oracle ===

--- Test: Basic Lifecycle ---
  AMM initialized: Iron/USDC @ 1.000000
  Alice deposits 10,000 USDC
  Alice opens 5x long (50,000 notional)
  AMM swap: price moves to 1.200000
  Crank: Alice mark-to-market PnL = +8,333
  Alice closes position. Realized PnL = +8,333
  Warmup: advancing 100 slots...
  Crank: profit conversion. h = 1.00 (fully backed)
  Alice withdraws: 18,333 USDC (10,000 capital + 8,333 converted profit)
  ✓ Conservation: vault(18,333) >= c_tot(0) + insurance(...)
  PASSED

--- Test: Haircut Under Stress ---
  ...
  Charlie liquidated. Loss = 5,000 (exceeds 1,000 capital)
  h = 0.80 (system stressed — 4,000 unbacked)
  Alice capital: 10,000 ✓ (UNTOUCHED)
  Alice effective PnL: 4,000 (raw: 5,000, haircut: 20%)
  ✓ No position was forcibly closed
  ✓ Conservation holds
  PASSED
```

- [ ] `cargo clippy` clean
- [ ] `cargo fmt`

**Exit criteria:** Clean repo, readable test output, ready to push and share.

---

## Timeline Summary

| Phase     | What               | Days          |
| --------- | ------------------ | ------------- |
| 0         | Workspace + vendor | 1             |
| 1         | SimpleAMM          | 1–2           |
| 2         | PerpEngine wrapper | 2–3           |
| 3         | Integration tests  | 2–3           |
| 4         | Polish             | 1             |
| **Total** |                    | **7–10 days** |

One Rust dev, part-time: ~2 weeks.
One Rust dev, full-time: ~1 week.

---

## What this proves

When you post this, the message is:

> **"First formally verified perp engine running against a real AMM oracle."**
>
> - No ADL. No forced position closes. Ever.
> - Principal is senior — mathematically guaranteed, not "trust our insurance fund."
> - 151 Kani proofs. All pass.
> - Stress tested: haircuts work, conservation holds, oracle manipulation blocked.
> - All Rust, no_std compatible, ready for Solana SBF/BPF.
>
> `cargo test` — see for yourself.

---

## After MVP (not in scope, but for context)

| Next step                                       | Effort    | Why                              |
| ----------------------------------------------- | --------- | -------------------------------- |
| Solana program shell (Anchor)                   | 3–4 weeks | Devnet deployment                |
| Wire AMM oracle to Solrover's actual CLMM pools | 1 week    | Real price feed                  |
| Multi-market (Iron-PERP, Algae-PERP, etc.)      | 1 week    | One engine instance per resource |
| Keeper crank bot                                | 2–3 days  | Permissionless, runs on a timer  |
| Trading UI                                      | 4–6 weeks | Web frontend for perp trading    |
| Game integration                                | 2–3 weeks | Access perps from the rover UI   |

---

## Dependencies

- Percolator source (vendored from `github.com/aeyakovenko/percolator`, branch `ak-adl`)
- Rust toolchain (stable)
- For verification (optional): `cargo install --locked kani-verifier && cargo kani setup`

---

## Risk / Open Questions

1. **Percolator API surface:** Need to confirm the public API exposes enough to wire from outside. If it's tightly coupled to a specific account layout, we may need thin adapter types. (Check `pub fn` signatures on the lib.)

2. **LP counterparty model:** MVP uses a single house LP. For production, need to decide: shared LP vault (like GMX) or individual LP accounts? Percolator supports both but the wrapper differs.

3. **Funding rate source:** For MVP, use a simple fixed rate or derive from LP inventory imbalance. Production would use the AMM price vs. perp mark price divergence.

4. **Warmup period tuning:** Spec says `T = warmup_period_slots`. For testnet, something like 100 slots (~40 seconds on Solana) is reasonable. For the test harness, we just advance slot counters.

---

_This roadmap targets the simplest possible artifact that demonstrates Percolator working with Solrover's economy model. Ship the test, get feedback, iterate._
