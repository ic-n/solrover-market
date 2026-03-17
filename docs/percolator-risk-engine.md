# Percolator Risk Engine — Technical Reference

> **Status:** Educational research project — Apache-2.0. Not yet audited for production. Do not use with real funds without a full independent audit.

This document is a technical reference for engineering and product teams. For the C-level summary, see [executive-brief.md](./executive-brief.md).

---

## What Is Percolator?

Percolator is a **perpetual DEX risk engine** written in Rust. It manages a single quote-token vault and provides:

- Leveraged perpetual positions (long/short)
- LP accounts (market-making)
- A provably safe insolvency model that replaces Auto-Deleveraging (ADL)
- Time-gated profit withdrawal (warmup)
- Permissionless keeper-crank architecture

It is designed to run as a Solana on-chain program (SBF/BPF compatible, no runtime dependencies).

---

## Core Design: Protected Principal + Junior Profit Claims

### Account Balance Sheet

Every account holds two separate claims:

```
Account equity = Capital (C_i) + effective PnL
```

| Field | Type | Description |
|---|---|---|
| `capital` (`C_i`) | `u128` | Protected principal — senior claim, always withdrawable |
| `pnl` (`PNL_i`) | `i128` | Realized PnL — junior claim, subject to haircut and warmup |
| `reserved_pnl` (`R_i`) | `u64` | PnL reserved for pending withdrawal (optional) |
| `warmup_started_at_slot` | `u64` | When warmup began |
| `warmup_slope_per_step` | `u128` | Rate at which PnL converts to capital per slot |

### Global State

| Field | Description |
|---|---|
| `vault` (`V`) | Total tokens in the program-owned vault |
| `c_tot` | Sum of all accounts' capital (O(1) aggregate) |
| `pnl_pos_tot` | Sum of all positive PnL across all accounts (O(1) aggregate) |
| `insurance_fund` | Insurance balance (senior claim within vault) |

---

## The Haircut Ratio `h`

```
Residual  = max(0, V − c_tot − I)

h_num = min(Residual, pnl_pos_tot)
h_den = pnl_pos_tot

h = h_num / h_den   ∈ [0, 1]
```

- `h = 1`: System fully solvent — all profits fully backed.
- `h < 1`: System stressed — each profitable account's effective PnL is scaled by `h`.
- `h` updates continuously on every state change. No trigger threshold. No manual intervention.

### Effective PnL per Account

```
effective_pnl_i = floor(max(PNL_i, 0) × h_num / h_den)
```

All winners share the same proportional haircut. No rankings, no queues, no position closures.

---

## Warmup: Time-Gated Profit Conversion

Profits are **not immediately withdrawable**. They must convert to protected capital through a warmup process:

```
WarmableGross_i = min(AvailGross_i, warmup_slope × elapsed_slots)

payout = floor(WarmableGross_i × h_num / h_den)
```

After conversion:
- `pnl` decreases by `WarmableGross_i`
- `capital` increases by `payout` (the haircut-adjusted amount)

**Oracle manipulation protection:** Even if an attacker inflates PnL via oracle distortion, they cannot extract it before warmup matures and `h` must support it.

**Warmup slope reset:** When new profits accrue, the slope resets proportionally. Formally proven that this does not create a "forever warmup" trap — conversion still completes in bounded time.

---

## Insolvency Handling vs. ADL

| Scenario | ADL (Industry Standard) | Percolator |
|---|---|---|
| Insurance fund depletes | Forcibly close profitable positions | `h` falls; profit extraction proportionally reduced |
| User experience | Position deleted without consent | Withdrawable amount reduced; position intact |
| Trigger | Discrete event (insurance depletion) | Continuous (`h` adjusts on every state change) |
| Recovery | Manual re-entry by user | Automatic as `Residual` improves |
| Auditable? | Complex waterfall logic | Single ratio `h`, one formula |

---

## Loss Settlement

When `PNL_i < 0`:

1. Pay losses from the account's own `capital` first (`principal protection`)
2. If capital is exhausted and loss remains: write off to zero via `set_pnl(i, 0)`
3. Write-off is reflected globally as `Residual < pnl_pos_tot` → `h < 1`
4. **No other account's capital is touched**

---

## Fee Architecture

### Trading Fees (Senior)
- `fee = ceil(notional × fee_bps / 10_000)` — ceiling division prevents micro-trade evasion
- Route directly to insurance fund
- Not socialized via `h`

### Maintenance Fees
- Charged per slot from `fee_credits`
- If `fee_credits` go negative → `FeeDebt_i` accrues
- Fee debt is a **margin liability**: reduces effective equity, can trigger liquidation
- Fee debt is swept from capital whenever capital becomes available (post-conversion)

---

## Margin System

```
Eq_mtm_net_i = max(0, Eq_mtm_i − FeeDebt_i)

Maintenance margin: Eq_mtm_net_i > Notional × MM_bps / 10_000
Initial margin:     Eq_mtm_net_i ≥ Notional × IM_bps / 10_000
```

Initial margin is required for:
- Risk-increasing trades (`|new_pos| > |old_pos|`)
- Position flips (long→short or short→long, regardless of size change)

---

## Keeper Crank

A permissionless on-chain crank that:
- Accrues funding index
- Settles a bounded window of accounts (configurable budget per call)
- Triggers warmup conversion for touched accounts (preventing zombie PnL poisoning)
- Liquidates unhealthy accounts
- Garbage-collects dust accounts

**Critical property:** The system never requires `OI = 0` or admin intervention to resume. The crank is optional for user operation but required for liveness of abandoned accounts.

---

## Funding (Anti-Retroactivity)

Funding rate is piecewise-constant between rate-change events. The stored rate (`funding_rate_bps_per_slot_last`) applies to the interval starting at `last_funding_slot`.

**Rule:** Any operation that might change funding rate inputs must:
1. Accrue funding to current slot using the *stored* rate
2. Apply the operation
3. Store the new rate for the *next* interval

This prevents retroactive rate manipulation — an adversary cannot delay a crank, change state, and have the new rate apply to the elapsed period.

---

## Formal Verification Summary

| Category | Proofs | Coverage |
|---|---|---|
| Core invariants (conservation, isolation, no-teleport) | 11 inductive | All principal operations |
| Strong symbolic proofs | 138 | Lifecycle sequences, funding, warmup, liquidation, fees |
| Unit/regression | 2 | Specific concrete scenarios |
| **Total** | **151** | **151/151 pass (100%)** |

Key proven properties:
- `V ≥ c_tot + I` at all times (vault conservation)
- `Σ effective_pnl_i ≤ Residual` (no over-withdrawal of profits)
- One account's loss never touches another's `capital`
- Zero panics or overflows at extreme values (max price, max position size)
- Funding anti-retroactivity holds under adversarial crank timing
- Warmup converts in bounded time (liveness)

Run verification locally:
```bash
cargo install --locked kani-verifier
cargo kani setup
cargo kani
```

---

## External Audit Findings — Status

All findings resolved as of February 2026:

| Finding | Severity | Status |
|---|---|---|
| G — Stale haircut after trade (value destruction) | HIGH | Fixed: two-pass settlement in `execute_trade` |
| C — Fee debt traps accounts permanently | MEDIUM | Fixed: `close_account` forgives residual debt |
| A1–A9, B1–B3 — Aggregate maintenance, vacuous proofs | Various | All fixed |
| Flaw 1: "Free option" debt wipe | Critical (claimed) | **Not exploitable** — formally proven |
| Flaw 2: Phantom margin equity | Critical (claimed) | **Not exploitable** — formally proven |
| Flaw 3: Forever warmup trap | Critical (claimed) | **Not exploitable** — formally proven |
| 5 security audit coverage gaps | High/Critical | All closed with 18 new proofs |

---

## References

- Tarun Chitra, *Autodeleveraging: Impossibilities and Optimization*, arXiv:2512.01112, 2025
- Kani Rust Verifier: https://model-checking.github.io/kani/
- Percolator spec: [`vendor/percolator/spec.md`](../vendor/percolator/spec.md)
- Percolator audit log: [`vendor/percolator/audit.md`](../vendor/percolator/audit.md)
