<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_abu_dhabi` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_abu_dhabi.rs`](../../src/calendar/schedules/futures/international/ice_abu_dhabi.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Murban Crude Oil Futures only: closed before the sourced 2021-03-29 launch; the New-York-locked grid and pre-open are translated into `Asia/Dubai` with date-aware US DST selection.

## Revision rows

None. ice_abu_dhabi.rs encodes its dated cutovers as constants rather than revisions! tuples, so it has no dated revision row.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2021-03-29 — T1 — IFAD circular 21/003 (`IFAD_LAUNCH`) — Murban Crude Oil Futures launch; the profile is a sourced closure before this day.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.ice.com/publicdocs/abu_dhabi/circulars/IFAD%20Circular%20-%2021003%20-%20Trading%20information%20publication.pdf> — IFAD circular 21/003, launching Murban Crude Oil Futures on 2021-03-29 and publishing the normal Monday–Friday grid plus the two-hour-earlier Monday trading-day open.
- <https://www.ice.com/products/75443578/Murban-Crude-Oil-Futures/> — live contract page giving the same 20:00–18:00 New York schedule and 19:45 pre-open.
- <https://www.ice.com/publicdocs/abu_dhabi/circulars/2026.03_-_IFAD_Trading_Hours_Change_Final.pdf> — IFAD annual daylight-time circular confirming that the grid follows US Eastern time when London and New York clocks are temporarily misaligned.

## Gaps and residual risks

- The module encodes the launch as the named constant `IFAD_LAUNCH` (2021-03-29) plus a daylight-time reference selector rather than as a `revisions!` tuple, so this file records no revision-row bullets even though the identity has a dated launch.
- The row is scoped to Murban Crude Oil Futures (ADM) only; other IFAD contracts require their own profile.
- Murban publishes no tradeable phase outside its near-24-hour session, so `extended` is empty by design.
