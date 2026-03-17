# Solrover — Executive Brief
*For C-Level Distribution*

---

## 1. What We Are Building

**Solrover** is a cross-platform survival game built for Web3 — designed from the ground up to be a genuinely fun, high-quality experience first, with blockchain mechanics that emerge naturally from gameplay rather than being bolted on.

Players drive a rover across a deadly, sun-scorched planet, commanding a squad of AI Crabbots to harvest resources, craft fuel, and survive. The core loop is tight and engaging: gather Algae, Iron, Copper, Plastic, and other raw materials; process them; keep the rover moving. Dying is cheap. Playing smart rewards you.

The game targets **Web browsers (mobile-first), iOS, Android, and Desktop** — built in Unity for performance and broad accessibility. Our immediate milestone is a polished, playable web experience optimized for mobile.

---

## 2. The Business Model — An On-Chain Commodity Market

Most Web3 games fail because their economy is inflationary by design: tokens are printed as rewards, liquidity is artificial, and the team retains permanent control over pricing. When hype fades, the economy collapses.

**We do not do that.**

Every base resource in Solrover (Algae, Iron, Copper, Plastic, etc.) is its own **on-chain asset** traded on a real, decentralized market. This is not a "play-to-earn" token scheme — it is a commodity exchange where prices are set by actual supply and demand generated through gameplay.

### Dual-Liquidity Model

| Tier | Mechanism | Fee | Who Controls It |
|---|---|---|---|
| Base liquidity | Standard AMM | 2% | Team |
| Deep liquidity | Player-driven concentrated liquidity | 1% | Players |

As players contribute concentrated liquidity, team control over pricing decreases proportionally. The goal is an economy that self-governs at scale.

### Factories — Yield for Skilled Players

Players who accumulate resources can build in-game **Factories**. A Factory is mechanically a concentrated liquidity position on a resource's trading pair. The player earns a share of trading fees proportional to their position depth.

This creates a second economic loop beyond survival:
- Play → Harvest resources → Build Factory → Earn yield from trading fees → Reinvest

The yield is real (derived from trading activity), sustainable (fee-backed, not printed), and earned (gated behind actual gameplay progression). No inflationary token mechanics.

---

## 3. The Risk Infrastructure — Percolator

Powering the financial layer of Solrover's on-chain economy is **Percolator**, a formally verified perpetual DEX risk engine that we have vendored and are adapting as a core infrastructure component.

### The Problem It Solves

Any exchange that allows leveraged positions or complex financial primitives needs a mechanism to handle insolvency — what happens when a losing position exceeds the loser's collateral. The industry standard is **Auto-Deleveraging (ADL)**: when the insurance fund runs dry, the exchange forcibly closes profitable positions without user consent. This is deeply unpopular, destroys trust, and is hard to reason about or audit.

Percolator replaces ADL with a mathematically cleaner model built on a single insight:

> **Treat profit as what it really is in a stressed exchange: a junior claim on a shared balance sheet.**

### One Vault, Two Claim Classes

| Claim | Type | Behaviour |
|---|---|---|
| **Capital** (deposited collateral) | Senior | Withdrawable immediately |
| **Profit** (unrealized/realized gains) | Junior | Must "warm up" through a time-gated conversion before withdrawal |

A single global ratio `h` continuously measures how much of all profits are actually backed by vault residual value:

```
Residual  = max(0, Vault − Total Capital − Insurance Fund)

        h = min(Residual, Total Positive PnL) / Total Positive PnL
```

- If the system is fully solvent: `h = 1` — all profits are fully backed.
- If stressed: `h < 1` — every profitable account takes a proportional haircut on profit extraction. No winners are singled out. No positions are forcibly closed. The system self-heals as losses are realized and residual improves.

### Why This Matters for Solrover

Solrover's commodity market involves:
- Players providing concentrated liquidity (LP positions with real exposure)
- Factory yields that accrue over time
- A dual-liquidity structure where player capital and team capital coexist in the same vault

Without a sound risk engine, a single large loss event or oracle manipulation attempt could cascade through the system and wipe player capital — a catastrophic event for a game that is explicitly built on trust and sustainability.

Percolator provides the mathematical guarantee that **no single account's insolvency can directly reduce any other account's protected principal.** Player deposits are senior. Only the profit layer — the "bonus on top" — absorbs systemic stress, and only proportionally, and only temporarily.

### Key Properties (Formally Verified)

Percolator has been formally verified using **Kani**, a Rust model checker that generates SAT/CBMC proofs. As of the latest audit:

| Metric | Result |
|---|---|
| Total Kani proofs | 151 |
| Passing | 151 (100%) |
| Failures | 0 |
| External audit gaps closed | 5 critical/high coverage gaps |
| External review flaws rebutted | 3 claimed critical flaws formally disproven |

**Proven invariants include:**

- **Conservation:** Total withdrawable value never exceeds vault token balance.
- **Principal protection:** One account's insolvency cannot reduce another account's protected capital — ever.
- **Oracle manipulation safety:** Profits created by short-lived price distortion cannot be withdrawn before a time-gated warmup period expires.
- **Profit-first haircuts:** In undercollateralization, haircuts apply only to junior profit claims. Principal is never touched.
- **No zombie poisoning:** Abandoned accounts with stale PnL cannot indefinitely depress the global `h` ratio for all other users. A permissionless keeper crank ensures accounting progress without admin intervention.
- **Liveness:** The system never requires "all open interest = 0" or manual admin recovery to resume safe withdrawals.
- **Zero panics at extreme values:** Formally verified to never overflow or panic at boundary inputs (maximum prices, maximum position sizes).

### Architecture

- **Language:** Rust, `no_std` compatible (deployable as a Solana on-chain program via SBF/BPF)
- **Dependencies:** Zero runtime dependencies — pure library
- **License:** Apache-2.0 (open source, commercially usable)
- **Formal verification:** Kani model checker (SAT-based, not just property-based testing)
- **Overflow safety:** All arithmetic uses checked/saturating operations; overflow returns an error rather than corrupting state

---

## 4. Competitive Differentiation

| Dimension | Industry Standard | Solrover |
|---|---|---|
| Economy design | Inflationary token rewards | Real commodity market, fee-backed yields |
| Liquidity control | Team-controlled forever | Progressively player-controlled |
| Risk model | ADL (forced position closes) | Percolator haircut model (proportional, no forced closes) |
| Risk verification | Testing / audit | Formal proof (151 Kani proofs, 100% pass) |
| Player trust | "Trust us, there's an insurance fund" | Mathematical guarantee: principal is senior |
| Platform | Single platform or low-quality ports | Unity: web (mobile-first), iOS, Android, Desktop |

---

## 5. Technical Architecture Summary

```
┌─────────────────────────────────────────────────────┐
│                     Unity Game Client                │
│           Web (mobile-optimized) · iOS · Android · Desktop   │
└────────────────────────┬────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────┐
│                  On-Chain Layer (Solana)             │
│                                                     │
│  ┌─────────────────┐   ┌───────────────────────────┐│
│  │  Resource AMMs   │   │  Concentrated Liquidity   ││
│  │  (2% fee, team)  │   │  Pools / Factories (1%)   ││
│  └────────┬────────┘   └────────────┬──────────────┘│
│           └──────────────┬──────────┘               │
│                          │                          │
│           ┌──────────────▼──────────────┐           │
│           │     Percolator Risk Engine  │           │
│           │  (Formally verified, Rust)  │           │
│           │  Principal protection · h   │           │
│           │  Warmup · Fee debt · Crank  │           │
│           └─────────────────────────────┘           │
└─────────────────────────────────────────────────────┘
```

---

## 6. Current Status

| Area | Status |
|---|---|
| Game (Unity) | In development — web/mobile build in progress |
| On-chain economy design | Complete (dual-liquidity, Factories model) |
| Percolator risk engine | Vendored, formally verified (151/151 proofs pass) |
| External security audit | Completed — all gaps closed, all critical findings rebutted |
| Target platform | Web browser (mobile-first), then iOS/Android/Desktop |

---

## 7. The Vision

We are building an honest game — one where people play because it is fun, where the economy rewards skill and patience rather than speculation, and where the on-chain structure is auditable, provably sound, and respects player capital.

Solrover is not a JPG sale dressed as a game. It is not a token pump. It is a survival game with a real economy, built by people who believe the best Web3 experiences are the ones where the blockchain is invisible to casual players and trustworthy to sophisticated ones.

> Our goal: a top-tier game backed by a capital-efficient, formally verified on-chain structure that respects the player's time and effort.

---

*This document references the Percolator risk engine (Apache-2.0), a research-stage component. Technical specifications subject to change as development progresses.*
