<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_metals` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's spot-metals book, the XAU and XAG instrument families and their
crosses: Sunday 18:05 through Friday 16:59 New York time, with the printed
16:59-18:05 daily gap and no other maintenance. The printed pair implies the
daily gap exactly as the FX row's footnote states its own six-minute one.
Holiday boundaries below are the operator's literal dated statements,
including 2025 closes at 13:00 and 14:30; no uniform futures inset is assumed.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and no unconditional changeover day is asserted. Historical completion
remains tracked in #332 under the dated current OANDA scope exception.

## The normal week

The hours-of-operation page prints every metals row identically:

> `XAU/USD | USD | New York | 100 | Sun-Fri: 18:05 - 16:59`

with the note:

> `**These instruments also have a break and are closed from 06:59 to 07:05 Auckland time`

The row prints no `***` daylight-saving marker, so the endpoints track New
York local time and `America/New_York` carries the transitions. The NZD-pair
Auckland break is stated per instrument and stays caller catalog data. The
rows cover XAU and XAG crosses against ten quote currencies; all share one
grid.

## Holidays

**Coverage:** 2024-02-21..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The page's `US energies and metals` group covers this book as well as the
commodity-metals book. On 2026-05-25 the operator explicitly distinguishes
the gold/silver reopen at 18:05 from the group's 18:01. That exception applies
where it is stated; other dated group reopens remain the literal printed
18:00 or 18:01, and standalone `Metals` reopens include this book. A holiday
statement overrides the normal week: the 2025-01-01 18:04 reopen
and the Holy Thursday 17:00 close are encoded numerically, even though the
page calls them "late" and "early" against today's normal 18:05/16:59 grid.
No rounding or retrospective extension of the 2026-05-25 parenthetical is
used. Evening reopens belong to the following trade date.

### 2024

`OANDA-UK-CARTER-2025-WAYBACK` gives these literal Eastern-time
arrangements. Evening reopens attach to the following trade date; normal
closes remain 16:59 except Thanksgiving Friday's explicit 14:45.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-03-29 | closed | — | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Good Friday: US energies and metals closed |
| 2024-05-27 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Memorial Day group close |
| 2024-05-28 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Memorial Day group reopen |
| 2024-06-19 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Juneteenth group close |
| 2024-06-20 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Juneteenth group reopen |
| 2024-07-04 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Independence Day group close |
| 2024-07-05 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Independence Day group reopen |
| 2024-09-02 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Labor Day group close |
| 2024-09-03 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Labor Day group reopen |
| 2024-11-28 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Thanksgiving Thursday group close |
| 2024-11-29 | replacement blocks | prior day 18:00..14:45 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Thanksgiving Thursday reopen and Friday close |
| 2024-12-24 | early close | 13:44 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas Eve: Metals close early 13:44 |
| 2024-12-25 | closed | — | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas Day until 18:04 evening reopen |
| 2024-12-26 | replacement blocks | prior day 18:04..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas Day: Metals open late 18:04 |

These fourteen T1 rows extend coverage to 2024-02-21, the current normal
week's sourced boundary, and increase this table from 32 to 46 rows. The
archive prints January and February 2024 arrangements too, but their
normal-week history remains unresolved, so this extension starts at the
existing normal-week knowledge boundary. The operator's changes-only page
supplies audited normal days where it states no change to this metals book.
Artifact: `normal-weeks/oanda-history-audit-2026-10-11/oanda-uk-holidays-20250326184653.html`.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | — | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day section: the US holiday groups reopen at the evening open, so the trade date's session does not trade |
| 2025-01-02 | replacement blocks | `18:04; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-01-20 | early close | `14:30` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day group early close, previously omitted. |
| 2025-01-21 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-02-17 | early close | `close early at 2.30 pm and reopen at 6 pm` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day, the group's instant (ET) |
| 2025-02-18 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-04-17 | replacement blocks | `18:05; 17:00` | `OANDA-UK-HOLIDAYS` | T1 | Holy Thursday explicitly closes 17:00 ET, one minute later than the normal close; clipping cannot express this. |
| 2025-04-18 | closed | `are closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday, `US energies and metals are closed` |
| 2025-05-26 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day |
| 2025-05-27 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-06-19 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2025-06-20 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-07-04 | early close | `close early at 1 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2025-09-01 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |
| 2025-09-02 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-11-27 | early close | `Close early at 2:30 PM, reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Thanksgiving Day |
| 2025-11-28 | replacement blocks | `18:00; 14:45` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-12-24 | early close | `Metals: close at 13:44` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Eve, the page's own split row |
| 2025-12-25 | closed | `Metals: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day: the stated metals evening reopen is keyed to the following trade date, so this trade date's session does not trade |

| 2025-12-26 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Metals: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: same derivation as Christmas Day |
| 2026-01-02 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2026-01-19 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day, the group's instant |
| 2026-01-20 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2026-02-16 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day |
| 2026-02-17 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2026-04-03 | closed | `US Energies and metals: Closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2026-05-25 | early close | `Close at 14:29; reopen at 18:01 (Gold, Silver and their crosses reopen at 18:05)` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day: the page's own gold/silver callout |
| 2026-06-19 | early close | `Close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2026-07-03 | early close | `Close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2026-09-07 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |

| 2026-09-08 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |

## Dated selectors

The module serves the witnessed normal-week eras; `profile_at` keys each
opening day to its era. `sourcing.rs` declares the normal week witnessed from
**2021-12-08**, the operator's archived page print that first lists the exact
`XAU/USD` and `XAG/USD` pair rows.

The spans, and what each rests on:

- **Opening days 2021-12-08 through 2023-06-04 — the earlier USD-pair grid
  `18:03 - 16:58`.** The 2021-12-08 capture prints
  `Gold (XAU/USD) ... Sun-Fri: 18:03 - 16:58*` and the matching silver row
  (`OANDA-UK-HOURS-2021-DEC-WAYBACK`); the 2023-06-05 capture still prints
  `XAU/USD ... 18:03 - 16:58` (`OANDA-UK-HOURS-2023-JUN-WAYBACK`), and the
  audit's intervening replays print the identical pair grid. The era's cross
  rows print an `18:10` open (XAU/NZD with its Auckland break): those
  per-instrument minutes stay caller instrument data, disclosed here as the
  era's residual.
- **Opening days 2023-06-05 through 2024-02-10 — the earlier grid.** The
  2023-06-05 print is the last earlier-grid page observation, and the
  operator's own machine channel witnesses the grid per day through the
  2024-02-08 opening day (the section below); the span serves the earlier
  USD-pair state, open `18:03`, close `16:58`.
- **Opening days from 2024-02-11 — the current grid `18:05 - 16:59`.** The
  changeover is dated 2024-02-11 (opening day) by the operator's machine
  channel under the `MARKET-DATA` convention (the section below); the
  2024-02-21 capture prints it (`OANDA-UK-HOURS-2024-WAYBACK`), and every
  later replay and the 2026-10-10 capture corroborate; all rows share one
  grid again.
- **Below the 2021-12-08 witness**: carried. The 2021-03-03..2021-08-18 page
  generation prints coarser `All gold instruments (XAU)` /
  `All silver instruments (XAG)` rows at `18:00 - 17:00` — a different table
  scope, not the book's exact pair rows — so the book's first complete
  witness stays 2021-12-08 (#332).

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page, captured 2026-10-10 22:36 UTC and rendered by the maintainer to
  PDF the same UTC date.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOLIDAYS` | 2025-01-01 .. 2026-10-30 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours> | captured 2026-10-10 22:44 | T1 | `c7137ce3da4cc19d0431955feedc10fb6b9731214ad3cfc7966160e72f8559da` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |
| `OANDA-UK-CARTER-2025-WAYBACK` | 2025-01-09 | <https://web.archive.org/web/20250326184653id_/https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | capture 2025-03-26; retrieved 2026-10-11 UTC | T1 | `467313685d4d64a4b95e5112a58d0fa04583f0abe988b1478d41fddf8b04cf1c` |
| `OANDA-UK-HOURS-2021-DEC-WAYBACK` | 2021-12-08 observation; not a cutover | <https://web.archive.org/web/20211208123011id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2021-12-08; retrieved 2026-10-11 UTC | T1 | `63e6d54029b72e0ef1ec32f92b8c3412bd24edf05df9fda9fa2b4dce98cb78fb` |
| `OANDA-UK-HOURS-2023-JUN-WAYBACK` | 2023-06-05 observation; not a cutover | <https://web.archive.org/web/20230605093523id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2023-06-05; retrieved 2026-10-11 UTC | T1 | `98ddc07cf38727b2a861fd1f48c503b1b597054cbec9d1dde4b3dde1801ffb56` |

Store paths: see [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- **RESOLVED 2026-10-16 (UTC): the changeover's effective day.** The
  disputed 2023-06-05..2024-02-20 span served as the sourced intersection;
  the operator's own machine channel now dates the changeover to opening
  day 2024-02-11 and the span serves the earlier pair grid exactly (section
  below, `MARKET-DATA`). The 2021-03-03..2021-08-18 coarse gold/silver rows
  and the era's cross opens stay #332 residuals.
- Dates where the page prints no US-group row at all are audited normal for
  this book: the page's US vocabulary is rows-only, so its silence affirms no
  change.

## Changeover dated by the operator's machine channel — 2026-10-16 UTC

The publication-channel hunts (recorded in
[`oanda_fx.md`](oanda_fx.md)) closed negative, and the undated span served
the sourced intersection. On 2026-10-16 (UTC), under the maintainer's
direction, the changeover was dated from the operator's own machine channel
— the practice-environment candle service (`api-fxpractice.oanda.com`, the
`/v3/accounts/<id>/instruments/XAU_USD/candles` endpoint, read as bytes
through the `oanda-rs` client and paced by its own admission control), the
same T2 class the MLK-2023 closure used under the `MARKET-DATA`
convention. The feed carries the operator's own trade-dated timeline on
both flanks of the boundary; the confirming raw responses are retained
privately, so the id deliberately does not resolve in the research store.
The host is the practice environment; its candle history is
account-independent, and confirming the live host with a live read-only key
remains a residual.

Method. For every calendar day from 2023-06-06 through 2024-02-19 the probe
fetched that day's M1 mid candles across the daily close-and-reopen window
and classified the day by which grid's reopen minute the feed shows: the
earlier pair grid reopens at 18:03 New York, the current grid at 18:05.
Result: every reopen day in the span shows the earlier grid through opening
day Thursday 2024-02-08, and every opening day from Sunday 2024-02-11 shows
the current grid, whose first tick bucket lands in the 18:04 minute — the
sub-five-second lead before the published 18:05 open. No opening day
classifies against its neighbours' era; the quiet days are Fridays,
Saturdays, the 2023-11-12 thin Sunday, and the holiday dates the holiday
layer owns (zero-candle 2023-11-24, the 2023-12-24 and 2023-12-31 holiday
Sundays). Second-resolution S5 confirmation across the shared US books:
2024-02-08 opens at 18:00:00 New York sharp (the legacy whole-hour cousins)
and its close minute ticks deep past 16:59:15, which the current 16:59
close forbids; from 2024-02-13 the opens lead at 17:00:55/18:00:55 and the
close minutes end within five seconds of 16:59. The metals book shares that
engine. The M1 per-day records and the S5 shapes were retained privately
with the retrieval date. The boundary is keyed to the local opening day of
the first current-grid session — a day-level revision keyed to the opening
day never splits a running session (LAW-NO-FABRICATED-DATES).
`MARKET-DATA` — T2, confirming artifacts retained privately, retrieval
2026-10-16 UTC.

## Publication audit — 2026-10-11 UTC

The source's minute values are literal. The 2025 18:00 ET US reopens and
20:00 ET Brent reopens are not rounded to 18:01 or 20:01. Evening opens
attach to the following trade date. Replacement blocks retain the entire
trading day, including any next-day early close and Brent's late segment.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).
