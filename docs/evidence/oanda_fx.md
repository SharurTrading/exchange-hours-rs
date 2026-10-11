<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_fx` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's FX book, all FX products except TRY pairs: Sunday 17:05 through Friday
16:59 New York time, with a six-minute 16:59–17:05 daily break and no other
maintenance. The book trades straight through the daily maintenance halts of
every futures clock an FX hedge might route to (CME FX futures halt 16:00–17:00
CT daily), which is why this is a book key rather than a futures mapping — the
maintainer's 2026-10-10 decision recorded in SharurPlatform#1046.


## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and a dormant identity owes its current
schedule rather than a worked-up history (LAW-BOUNDED-WORK).

## The normal week

The hours-of-operation page (UK entity, OANDA Europe Limited, FCA 542572)
prints one row for the whole group:

> `All FX products (except TRY Pairs) | New York | 100,000 | Sun-Fri: 17:05 - 16:59*`

with the footnote:

> `*Forex instruments open on Sunday at 17:05 and close for the week on Friday 16:59 (New York time). There will be a six minute break between 16:59 - 17:05.`

and the note:

> `Note: There is an additional 6-minute break for NZD pairs starting at 06:59 Auckland time.`

Interpretation, each step from the operator's own words:

- The row prints **no `***` daylight-saving marker**, so both endpoints track
  New York local time: the anchor zone is `America/New_York`, DST-observing,
  and the zone carries the transitions — no seasonal selector and no
  fixed-offset zone is owed (the maintainer's zone-anchoring rule of
  2026-10-10). The single `*` is a footnote pointer, not a DST marker.
- The encoding is Sunday–Thursday 17:05 wrapping to the next local day 16:59
  (end-exclusive), Friday with no rule, `has_daily_close`/`has_weekend_close`
  both true. The six-minute gap between the close and the next 17:05 open is
  the stated break; it is a same-trade-date `Closed` gap, not maintenance.
- The NZD-pair break (06:59–07:05 `Pacific/Auckland`) is stated **per
  instrument** (`XAU/NZD`, `XAG/NZD` and NZD FX pairs), not for the book: it
  is caller catalog data, and no rule in the grid states it.
- TRY pairs are carved out of this row by name and carry their own key,
  `oanda_try_fx`.

## Holidays

**Coverage:** 2025-01-01..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The page prints the FX group on exactly three dates:

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `FX open late 17:05` | `OANDA-UK-HOLIDAYS` | T1 | the holiday trade date's session does not trade; the reopen instant is the next session's normal open, so the trade date itself is a full closure |
| 2025-12-25 | closed | `Forex: open at 17:05` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day, the same derivation |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Forex: open at 17:05` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day, the same derivation; the page's 2025-12-31 section states `Forex: close at 16:59`, the no-reopen at the normal daily close instant that needs no row of its own |

The raw quotations, in the page's own layout:

> `Wednesday 1st January 2025` — `FX open late 17:05`
> `Thursday, 25 December 2025 - Christmas Day` — `Forex: open at 17:05`
> `Thursday, 1 January 2026 - New Year’s Day` — `Forex: open at 17:05`

with New Year's Eve stated as:

> `Wednesday, 31 December 2025 - New Year’s Eve` — `Forex: close at 16:59`

Interpretation: "open at 17:05" on the holiday names the reopen instant,
which is the **next** session's normal open — the holiday trade date's own
session (normally the prior evening 17:05 to that day's 16:59) does not
trade, so each of the three dates ships a `Closed` row. The 2025-12-31 cell
states the matching no-reopen at the normal daily close instant and needs no
row of its own. Every other date in the window carries no FX row, which on a
page titled "the changes in trading hours for upcoming holidays" is the
operator's statement that the book runs its normal grid — Christmas Eve
2025, US holidays, Thanksgiving and all APAC/European holidays included
(each of those dates prints rows for other instrument groups and none for
FX, so the silence is against a page that speaks when something changes).

## Dated selectors

None. The module has one sourced grid and `profile_at` returns it at every
instant; `sourcing.rs` declares the normal week witnessed from **2024-02-21**,
the operator's own archived page print of the current `Sun-Fri: 17:05 - 16:59`
grid with the six-minute break, corroborated by the 2026-10-10 capture.

The spans, and what each rests on:

- **From 2024-02-21**: the 2024 Wayback capture prints the current grid, and
  the 2026-10-10 capture corroborates it; the dated holiday sections
  2025-01-01..2026-10-30 sit inside this span, so every shipped holiday row
  answers.
- **2024-02-21 back to the first audited holiday window's start (2025-01-01
  is inside the span above; the residual is the carried edge immediately
  below 2024-02-21 down to nothing)**: no residual — the bridge applies below
  the *holiday window*, not below the horizon, and the window lies wholly
  inside the sourced span.
- **Below 2024-02-21**: carried. The 2022-05-18 capture prints the earlier
  `Sun-Fri: 17:03 - 16:58` grid (five-minute 16:58-17:03 break, NZD break
  from 06:58 Auckland), so the changeover to the current grid is undated and
  the crate refuses the span rather than serving either grid there. Dating
  the changeover, or closing the hunts for it negative and serving the two
  grids' intersection per the 2026-10-04 rule, is the activation obligation.

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page for the UK entity, captured 2026-10-10 22:36 UTC (HTML) and
  rendered by the maintainer to PDF the same UTC date. The PDF's
  `11/10/2026, 09:22` header is the maintainer's local clock (UTC+11),
  i.e. 2026-10-10 22:22 UTC.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOURS-HTML` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | captured 2026-10-10 22:36 | T1 | `60f6143dd5d678b1a34a9937aa938d2884380a33bf3542a4c72f8f03fa3607b7` |
| `OANDA-UK-HOLIDAYS` | 2025-01-01 .. 2026-10-30 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours> | captured 2026-10-10 22:44 | T1 | `c7137ce3da4cc19d0431955feedc10fb6b9731214ad3cfc7966160e72f8559da` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |
| `OANDA-UK-HOURS-2021-WAYBACK` | 2021-03-03 | <https://web.archive.org/web/20210303210448id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20210303210448`, retrieved 2026-10-10 UTC; prints no Forex table | T1 | `ec5987239f2c79dd29fcfc40abe5a0d58c6e4ed446fa7dea653f1791b186cbed` |

Store paths (research store, wave-1 capture):
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation.pdf`,
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation.html`,
`holidays/raw/oanda-uk-holiday-hours-2026-10-10/oanda-uk-holiday-trading-hours.html`.

## Gaps and residual risks

- **The pre-2024-02-21 normal week is carried, not sourced.** The 2022-05-18
  capture prints the earlier `17:03 - 16:58` grid, so the changeover to the
  current grid is undated and the crate refuses that span on the date-aware
  surface rather than serving either grid. Closing condition: a dated
  operator artifact for the changeover, or the hunts closing negative (then
  the 2026-10-04 intersection rule applies). Activation (served tier)
  requires this to close. The 2021-03-03 capture prints no Forex table, so
  nothing witnesses the grid's state before 2022-05-18 at all.
- **The holiday window's edges are the page's own.** 2024 and earlier
  holiday arrangements are unworked up (the page's earliest section is
  2025-01-01) and 2026-10-31 onward is unpublished. Dates outside
  2025-01-01..2026-10-30 refuse typed on the date-aware surface.
- The page's overall note — "these times are subject to change during
  daylight saving time and certain public holidays. There may be occasions
  within market hours when trading may be halted" — states in-market halts
  exist operationally; those are not scheduled hours and are not modelled.
- Intraday-quote holidays for specific FX pairs beyond the page's rows (for
  example reduced liquidity on US holidays) are not closures and ship no
  row; the page states none.
