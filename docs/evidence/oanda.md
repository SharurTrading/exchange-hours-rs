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
[`oanda_fx.md`](oanda_fx.md)'s — including the 2024-02-21 page-print
horizon and the changeover the operator's machine channel dates to opening
day 2024-02-11 (`MARKET-DATA`, recorded there) — and this file adds only
what the venue row adds.

## Revision rows

- 2024-02-11 — T2 — `MARKET-DATA` — the venue clock's changeover to the
  current `17:05 - 16:59` grid is dated to this New York opening day by the
  operator's own machine channel (artifacts retained privately; see
  [`oanda_fx.md`](oanda_fx.md)).

No `revisions!` block: the module selects eras on the dated constant; a
dormant identity owes its current schedule rather than a worked-up history
(LAW-BOUNDED-WORK).

## What the venue row adds

- The `exchanges!` row `Oanda => "oanda"` with the routing arm in
  `presets/historical.rs`, so a consumer venue map (the planned oanda-rs
  adapter, oanda-rs#30; the platform map, SharurPlatform#1046) can admit the
  OANDA namespace as one `Exchange`.
- The venue holiday table mirrors `oanda_fx` verbatim — same rows, same
  window 2024-02-21..2026-10-30, same tiers and document ids — which
  `holidays/routing.rs` states as `Some(super::oanda_fx::TABLE)`.

## Dated selectors

The venue clock mirrors [`oanda_fx.md`](oanda_fx.md)'s eras; the module's
era-bound constants are keyed observations, not effective days
(LAW-NO-FABRICATED-DATES):

- 2023-06-05 — T1 — the 2023-06-05 05:35 UTC page print (`OANDA-UK-HOURS-2023-JUN-WAYBACK`) — the last observation of the earlier `17:03-16:58` grid, taken inside the session opened 2023-06-04; the era-bound constant keys this observation.
- 2024-02-20 — T1 — the 2024-02-21 00:55 UTC page print (`OANDA-UK-HOURS-2024-WAYBACK`) — the first observation of the current `17:05-16:59` grid, taken inside the session opened 2024-02-20; the era-bound constant keys this observation.

See [`oanda_fx.md`](oanda_fx.md) for the spans, the sourced intersection and
the sweep record.

### Documents

This row cites the same artifacts as [`oanda_fx.md`](oanda_fx.md); its
`### Documents` table governs both.

## Sources

This row cites the operator's own pages through [`oanda_fx.md`](oanda_fx.md)'s
`### Documents` table; no separate artifact backs the venue row.

## Gaps and residual risks

See [`oanda_fx.md`](oanda_fx.md): the family's witnessed normal-week eras
(the earlier grids served from each book's first witness, the changeovers
dated by the operator's machine channel to opening day 2024-02-11 New York
and 2024-02-12 Singapore), the carried pre-witness weeks, and the holiday
window's edges are the family's gaps and stay the family's.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).
