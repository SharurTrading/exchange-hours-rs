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

## Sources

- <https://www.ice.com/publicdocs/abu_dhabi/circulars/IFAD%20Circular%20-%2021003%20-%20Trading%20information%20publication.pdf> — IFAD circular 21/003, launching Murban Crude Oil Futures on 2021-03-29 and publishing the normal Monday–Friday grid plus the two-hour-earlier Monday trading-day open.
- <https://www.ice.com/products/75443578/Murban-Crude-Oil-Futures/> — live contract page giving the same 20:00–18:00 New York schedule and 19:45 pre-open.
- <https://www.ice.com/publicdocs/abu_dhabi/circulars/2026.03_-_IFAD_Trading_Hours_Change_Final.pdf> — IFAD annual daylight-time circular confirming that the grid follows US Eastern time when London and New York clocks are temporarily misaligned.

## Gaps and residual risks

- The module encodes the launch as the named constant `IFAD_LAUNCH` (2021-03-29) plus a daylight-time reference selector rather than as a `revisions!` tuple, so this file records no revision-row bullets even though the identity has a dated launch.
- The row is scoped to Murban Crude Oil Futures (ADM) only; other IFAD contracts require their own profile.
- Murban publishes no tradeable phase outside its near-24-hour session, so `extended` is empty by design.
