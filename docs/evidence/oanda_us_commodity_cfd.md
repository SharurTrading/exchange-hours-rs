<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_us_commodity_cfd` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's US-anchored energy and commodity-metals book — Natural Gas, West
Texas Oil, Platinum, Palladium and Copper — on the same Sunday-to-Friday
clock as the index book: New York 18:01-16:59, the printed 16:59-18:01 daily
gap, no other maintenance.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and a dormant identity owes its current
schedule rather than a worked-up history (LAW-BOUNDED-WORK).

## The normal week

The page prints the identical pair across the group:

> `Natural Gas | USD | New York | 2,500 | Sun - Fri 18:01 - 16:59`
> `Copper | USD | Chicago | 2,500 | Sun - Fri 17:01 - 15:59`

> `Platinum (XPT) | USD | New York | 100 | Sun - Fri 18:01 - 16:59`

The instants sit one minute inside the futures grid's 17:00-16:00 CT — the
hedge-desk relationship: the book opens and closes just inside the hedge
market's own day, and its holiday early closes below follow the same one-
minute inset (12:59 against 13:00, 14:29 against 14:30). A separate key from
the index book exists because the operator's own holiday page gives this
group its own instants (`US energies and metals` at 14:29 while indices sit
at 12:59), and because the standalone `US energies`/`Metals` rows on the
Christmas sections name this book.

## Holidays

**Coverage:** 2025-01-01..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The group is the page's `US energies and metals` (2026 sections, `US
energies and metals` 2025), including the standalone `US energies:` and
`Metals:` rows the Christmas and New Year sections print — `open at 18:01`
on the holidays themselves and `open late 18:04` on New Year's Day 2025 and
Christmas Eve 2025, both late against this book's own 18:01 open. Where a
section splits energy from metals (Christmas Eve 2025: both `13:44`) the
rows agree.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `US energies open late 18:04`, `Metals open late 18:04` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: the holiday trade date is closed; the following trade date opens at the explicitly stated 18:04 |
| 2025-01-02 | late open | `18:04` | `OANDA-UK-HOLIDAYS` | T1 | 2025-01-01 evening reopen is keyed to 2025-01-02, whose opening day is the holiday. |
| 2025-01-20 | early close | `14:30` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day group early close, previously omitted. |
| 2025-01-21 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-02-17 | early close | `close early at 2.30 pm and reopen at 6 pm` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day |
| 2025-02-18 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-04-17 | replacement blocks | `18:01; 17:00` | `OANDA-UK-HOLIDAYS` | T1 | Holy Thursday explicitly closes 17:00 ET, one minute later than the normal close; clipping cannot express this. |
| 2025-04-18 | closed | `are closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2025-05-26 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day |
| 2025-05-27 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-06-19 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2025-06-20 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-07-04 | early close | `close early at 1 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2025-09-01 | early close | `Close early at 2:30 PM, reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |
| 2025-09-02 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-11-27 | early close | `Close early at 2:30 PM, reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Thanksgiving Day |
| 2025-11-28 | replacement blocks | `18:00; 14:45` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-12-24 | early close | `US energies: close at 13:44`, `Metals: close at 13:44` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Eve, the page's own split rows agreeing |
| 2025-12-25 | closed | `US energies: open at 18:01`, `Metals: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day: the reopens are the book's own evening open |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `US energies: open at 18:01`, `Metals: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: same derivation as Christmas Day |
| 2026-01-19 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day |
| 2026-02-16 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day |
| 2026-04-03 | closed | `US Energies and metals: Closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2026-05-25 | early close | `Close at 14:29; reopen at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day; the gold/silver callout belongs to `oanda_metals` |
| 2026-06-19 | early close | `Close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2026-07-03 | early close | `Close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2026-09-07 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |

## Dated selectors

None. One sourced grid; `sourcing.rs` declares the normal week witnessed from
**2024-02-21**, the operator's archived page print of the current grid. The
2022-05-18 capture prints the futures-exact `17:00 - 16:00` /
`18:00 - 17:00` pairs (no one-minute inset), so the changeover is undated and
the span below the 2024 witness is carried, not served.

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page, captured 2026-10-10 22:36 UTC and rendered by the maintainer to
  PDF the same UTC date.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

### Documents

The Documents table governing this row is
[`oanda_fx.md`](oanda_fx.md)'s, which cites the hours page (PDF and HTML),
the holiday page, and the 2024 and 2022 Wayback captures; the digests there
govern here too.

## Gaps and residual risks

- **The pre-2024-02-21 normal week is carried, not sourced.** The 2022
  capture's futures-exact instants and the undated changeover refuse the
  span on the date-aware surface. Closing condition: dated operator
  artifacts, or hunts closed negative (then the 2026-10-04 intersection
  rule applies).
- The `Metals:` label's coverage of the commodity-metals listings rests on
  the 18:01/18:04 reopen instants; if OANDA prints an explicit
  `Copper:`-style row that disagrees, the split lands in a per-leg key.
- US Agricultural arrangements (`Corn, Wheat and Soybeans`, `Sugar`,
  `Commodities` in the 2025 sections) belong to `globex_grains` and
  `ice_us_sugar`, not this book.

## Publication audit — 2026-10-11 UTC

The source's minute values are literal. The 2025 18:00 ET US reopens and
20:00 ET Brent reopens are not rounded to 18:01 or 20:01. Evening opens
attach to the following trade date. Replacement blocks retain the entire
trading day, including any next-day early close and Brent's late segment.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).
