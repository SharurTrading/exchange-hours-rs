<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's own FX and CFD book as a venue identity (OANDA Europe Limited, the
`uk-en` entity; precedent: `binance_futures` models an operator-run book).
The venue's clock is the FX (ex-TRY) grid — Sunday 17:05 to Friday 16:59 New
York time with the six-minute daily break — so `hours_for_exchange` routes
to the `oanda_fx` family's `profile_at`, and the venue's holiday table is
the verbatim mirror of that family's table, as the venue profile-clock rule
requires. The grid, its evidence and its gaps are
[`oanda_fx.md`](oanda_fx.md)'s — including the 2024-02-21 horizon (the
operator's own archived page print of the current grid; the 2022 capture's
earlier grid and its undated changeover are the recorded gap below it) — and
this file adds only what the venue row adds.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and a dormant identity owes its current
schedule rather than a worked-up history (LAW-BOUNDED-WORK).

## What the venue row adds

- The `exchanges!` row `Oanda => "oanda"` with the routing arm in
  `presets/historical.rs`, so a consumer venue map (the planned oanda-rs
  adapter, oanda-rs#30; the platform map, SharurPlatform#1046) can admit the
  OANDA namespace as one `Exchange`.
- The venue holiday table mirrors `oanda_fx` verbatim — same rows, same
  window 2025-01-01..2026-10-30, same tiers and document ids — which
  `holidays/routing.rs` states as `Some(super::oanda_fx::TABLE)`.

## Dated selectors

None — see [`oanda_fx.md`](oanda_fx.md).

### Documents

This row cites the same artifacts as [`oanda_fx.md`](oanda_fx.md); its
`### Documents` table governs both.

## Sources

This row cites the operator's own pages through [`oanda_fx.md`](oanda_fx.md)'s
`### Documents` table; no separate artifact backs the venue row.

## Gaps and residual risks

See [`oanda_fx.md`](oanda_fx.md): the carried pre-2024-02-21 normal week,
the 2024-02-21..2025-01-01 bridged residual, and the holiday window's edges
are the family's gaps and stay the family's.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).
