<!-- SPDX-License-Identifier: MIT-0 -->

# `asx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`asx.rs`](../../src/calendar/schedules/equities/apac/asx.rs)
- **Source sets:** [`APAC-ASX`](../schedules/sources.md#apac-asx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Cash-market envelope; the pre-2025 staggered opening preserves both the earliest regular edge and latest randomized auction edge.

## Revision rows

- 2025-06-23 — T1 — ASX SR15 notice 0473.25.05 — Service Release 15: a single Opening Single Price Auction, which ASX's timetable prints at 09:59:00–09:59:45 with Normal Trading nominally from 09:59:45, replaces the five staggered symbol-group opens, and Post Close 16:11:00–16:21:30 is added. `ASX_PROFILE_CURRENT` encodes the resulting opening minute as one `extended` rule 09:59:00–10:00:00 and starts `regular` at 10:00:00; see the opening-edge note under Gaps.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours> — ASX cash-market trading hours, the current phase timetable: Pre-open 07:00:00–09:59:00, Opening Single Price Auction 09:59:00–09:59:45, Open (Normal Trading) 09:59:45–16:00:00, Pre-CSPA 16:00:00–16:10:00, Closing Single Price Auction 16:10:00–16:11:00, Post Close 16:11:00–16:21:30.
- <https://www.asxonline.com/public/notices/2025/may/0473.25.05.html> — ASX notice 0473.25.05, the Service Release 15 notice dating the change to 2025-06-23.
- <https://www.asxonline.com/content/dam/asxonline/public/notices/2025/april/asx-sr15asx-operating-rule-procedure-amendments.pdf> — SR15 marked operating-rule procedure amendments, which carry the pre-SR15 text: five symbol groups opening at nominal times from 10:00 through 10:09, each randomized by ±15 seconds, with the CSPA ending at 16:12.
- ASX Operating Rules Procedures Appendix 4013 — the operator's phase definitions
  for the cash market. No URL for this appendix is recorded anywhere in this
  repository — neither the owner module nor the `APAC-ASX` entry in
  [sources.md](../schedules/sources.md#apac-asx) carries one — so the appendix
  cannot be re-opened from a link here; re-verification goes through the
  `APAC-ASX` operating-rules entry point in
  [sources.md](../schedules/sources.md#apac-asx).
- <https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> — the operator's own annual trading calendar (this page's own holiday sheets), retrieved 2026-09-28 (UTC) as `holidays/raw/equities/asx/2025-2027/live_asx_cash_market_trading_calendar.html` (research store), sha256 `adb2344c…`; the live page renders the 2026 and 2027 sheets.
- The operator's pre-2025 trading-calendar sheets and their Wayback `id_` replays — `about/operational/trading_calendar/asx/<year>` (2010-2011), `trading_services/asx-trading-calendar-<year>` (2012), `about/asx-trading-calendar-<year>` (2013-2019), the `www2.asx.com.au` cash-market trading-calendar page (2020-2022) and the `www.asx.com.au` page (2023-2024) — retrieved 2026-09-29 (UTC) as `holidays/raw/equities/asx/2010-2024/` (research store), digests in the `### Documents` table below. Each sheet prints the operator's own `Public Holiday Dates for <year>` table with its `Trading Day`, settlement and `Business Day` columns and the `CLOSE EARLY` footnotes.

## Holidays

**Coverage:** 2010-01-01..2027-12-31 (inclusive trade dates, one audited
window: every year 2010-2027 has its own operator sheet). Tier: T1
throughout.

ASX has published one annual `Trading calendar` sheet per year across the
whole window. The 2025-2027 rows come from the operator's current
cash-market trading-calendar page (the 2025 sheet at its 2025-04-16 replay,
the 2026 and 2027 sheets on the live page retrieved 2026-09-28 UTC). The
2010-2024 rows come from that year's own operator sheet, retrieved
2026-09-29 (UTC) as Wayback `id_` replays — verbatim bytes of the operator's
own statement, so T1 (LAW-PUBLIC-SOURCES) — per era: the
`about/operational/trading_calendar/asx/<year>` pages (2010-2011), the
`trading_services/asx-trading-calendar-<year>` page (2012), the
`about/asx-trading-calendar-<year>` pages (2013-2019), the
`www2.asx.com.au` cash-market trading-calendar page (2020-2022) and the
`www.asx.com.au` page (2023-2024). Artifacts and digests:
`holidays/raw/equities/asx/2010-2024/` (research store) and the `###
Documents` table below.

**The venue answer is the sheet's own `Trading Day` column, and only its
`CLOSED` and `CLOSE EARLY` rows ship.** The 2010-2019 sheets are
state-by-state tables (`Public Holiday`, date, `Applies to the following
States`, `Trading Day`, settlement columns, `Bus. Day`): a row that closes
individual states while printing `OPEN` — the Tasmanian show days and cups,
the WA and QLD Labour Days, the Melbourne Cup — is a settlement fact, not a
market closure, and ships no row. A `CLOSED` row is the venue's own closure
whatever mix of states the holiday applies to (the sheet's "ALL except WA"
Queen's Birthday rows print `CLOSED`), and a `CLOSE EARLY` row states its
own instant through the sheet's footnote, `Normal trading ceases at 14:10
(Sydney time)` — the same instant the current era prints — so the row clips
the envelope at 14:10 and the afternoon blocks are gone with it. The 2020
and later sheets carry the same columns without the state list. Three
sheets print no early close at all — 2017 has no `Last Business Day` rows,
and the 2022 and 2023 sheets print both year-end rows `OPEN` (`YES` business
days) — so 2017, 2022 and 2023 ship no early close, rather than carrying the
neighbouring years' pattern. Weekend `ANZAC Day` rows (2015 and 2020
Saturdays, the 2021 Sunday) restate closures the Mon-Fri normal week has
already made and ship as printed, like the 2026 Saturday row.

The sheets name each closure in their own `CLOSED` cell and each half day as
`CLOSE EARLY` with numbered footnotes that state the instant themselves.
ANZAC Day 2027 is the one 2025-2027 entry the module deliberately does not
row: the 2027 sheet prints `OPEN` against Monday 26 April with the footnote
`Substitute for Sunday 25 April`, so that Monday is an audited-normal
trading day. ASX's standing note (`ASX Limited reserves the right to declare
additional Non-Business Days … without notice`) is the operator's own
alteration clause and is covered by the monthly watch; the 2022 National Day
of Mourning is the clause in action — the sheet gained the `National Day of
Mourning for Her Majesty the Queen, Thursday 22 September` row between its
July and October 2022 replays (the July replay is in the store as
corroboration), and the October sheet keys the row.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `New Year's Day` — `Friday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2010` | T1 | ASX event date printed verbatim |
| 2010-01-26 | closed | `Australia Day` — `Tuesday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2010` | T1 | ASX event date printed verbatim |
| 2010-04-02 | closed | `Good Friday` — `Friday 2 April` — `ALL` — `CLOSED` | `ASX-CAL-2010` | T1 | ASX event date printed verbatim |
| 2010-04-05 | closed | `Easter Monday` — `Monday 5 April` — `ALL` — `CLOSED` | `ASX-CAL-2010` | T1 | ASX event date printed verbatim |
| 2010-04-26 | closed | `ANZAC Day Holiday` — `Monday 26 April` — `ALL` — `CLOSED` | `ASX-CAL-2010` | T1 | the sheet's own observed date for the Sunday 25 April holiday |
| 2010-06-14 | closed | `Queen's Birthday` — `Monday 14 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2010` | T1 | ASX event date printed verbatim; the Trading Day column is the venue's own answer |
| 2010-12-24 | early close | `Last Business Day before Christmas Day` — `Friday 24 December` — `ALL` — `CLOSE EARLY` `[15]` — footnote: `Normal trading ceases at 14:10 (Sydney time)` | `ASX-CAL-2010` | T1 | ASX trade date named verbatim; the row clips the envelope at the sheet's own 14:10 |
| 2010-12-27 | closed | `Christmas Day / Boxing Day` — `Monday 27 December` — `WA, SA, TAS, VIC, ACT / NSW & QLD` — `CLOSED` | `ASX-CAL-2010` | T1 | the sheet's own observed date for the Saturday 25 December holiday |
| 2010-12-28 | closed | `Boxing Day / Christmas Day / Proclamation Day` — `Tuesday 28 December` — `WA, SA, TAS, VIC, ACT / QLD / SA` — `CLOSED` | `ASX-CAL-2010` | T1 | the sheet's own observed date for the Sunday 26 December holiday |
| 2010-12-31 | early close | `Last Business Day of the Year` — `Friday 31 December` — `ALL` — `CLOSE EARLY` `[16]` — 14:10 (Sydney time) | `ASX-CAL-2010` | T1 | same reading as 2010-12-24 |

The sheet's state rows that print `OPEN` (the TAS cups and shows, the WA and
QLD Labour Days, Foundation Day, the Melbourne Cup) ship no rows: the venue's
Trading Day column answers `OPEN` on them.

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | `New Year's Day` — `Monday 3 January` — `ALL` — `CLOSED` | `ASX-CAL-2011` | T1 | the sheet's own observed date for the Saturday 1 January holiday |
| 2011-01-26 | closed | `Australia Day` — `Wednesday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2011` | T1 | ASX event date printed verbatim |
| 2011-04-22 | closed | `Good Friday` — `Friday 22 April` — `ALL` — `CLOSED` | `ASX-CAL-2011` | T1 | ASX event date printed verbatim |
| 2011-04-25 | closed | `Easter Monday / ANZAC Day Holiday` — `Monday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2011` | T1 | the sheet prints both events on one date; one row, the date printed verbatim |
| 2011-04-26 | closed | `Easter Tuesday / Public Holiday` — `Tuesday 26 April` — `ALL / NSW, VIC, SA, WA & ACT` — `CLOSED`; footnote: `In recognition of the congruence of Anzac Day and Easter Monday` | `ASX-CAL-2011` | T1 | the sheet's own one-off congruence holiday, printed `CLOSED` |
| 2011-06-13 | closed | `Queen's Birthday` — `Monday 13 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2011` | T1 | ASX event date printed verbatim |
| 2011-12-23 | early close | `Last Business Day before Christmas Day` — `Friday 23 December` — `ALL` — `CLOSE EARLY` `[16]` — 14:10 (Sydney time) | `ASX-CAL-2011` | T1 | ASX trade date named verbatim |
| 2011-12-26 | closed | `Christmas Day / Boxing Day` — `Monday 26 December` — `ALL / VIC & TAS` — `CLOSED` | `ASX-CAL-2011` | T1 | the sheet's own observed date for the Saturday 25 December holiday |
| 2011-12-27 | closed | `Boxing Day / Proclamation Day / Christmas Day` — `Tuesday 27 December` — `NSW, QLD, WA, ACT / SA / VIC & TAS` — `CLOSED` | `ASX-CAL-2011` | T1 | the sheet's own observed date for the Sunday 26 December holiday |
| 2011-12-30 | early close | `Last Business Day of the Year` — `Friday 30 December` — `ALL` — `CLOSE EARLY` `[17]` — 14:10 (Sydney time) | `ASX-CAL-2011` | T1 | same reading as 2011-12-23 |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `New Year's Day` — `Monday 2 January` — `ALL` — `CLOSED` | `ASX-CAL-2012` | T1 | the sheet's own observed date for the Sunday 1 January holiday |
| 2012-01-26 | closed | `Australia Day` — `Thursday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-04-06 | closed | `Good Friday` — `Friday 6 April` — `ALL` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-04-09 | closed | `Easter Monday` — `Monday 9 April` — `ALL` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-04-25 | closed | `ANZAC Day` — `Wednesday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-06-11 | closed | `Queen's Birthday` — `Monday 11 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-12-24 | early close | `Last Business Day before Christmas Day` — `Monday 24 December` — `ALL` — `CLOSE EARLY` `[15]` — 14:10 (Sydney time) | `ASX-CAL-2012` | T1 | ASX trade date named verbatim |
| 2012-12-25 | closed | `Christmas Day` — `Tuesday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-12-26 | closed | `Boxing Day / Proclamation Day` — `Wednesday 26 December` — `ALL / SA` — `CLOSED` | `ASX-CAL-2012` | T1 | ASX event date printed verbatim |
| 2012-12-31 | early close | `Last Business Day of the Year` — `Monday 31 December` — `ALL` — `CLOSE EARLY` `[16]` — 14:10 (Sydney time) | `ASX-CAL-2012` | T1 | same reading as 2012-12-24 |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `New Year's Day` — `Tuesday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-01-28 | closed | `Australia Day` — `Monday 28 January` — `ALL` — `CLOSED` | `ASX-CAL-2013` | T1 | the sheet's own observed date for the Saturday 26 January holiday |
| 2013-03-29 | closed | `Good Friday` — `Friday 29 March` — `ALL` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-04-01 | closed | `Easter Monday` — `Monday 1 April` — `ALL` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-04-25 | closed | `ANZAC Day` — `Thursday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-06-10 | closed | `Queen's Birthday` — `Monday 10 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-12-24 | early close | `Last Business Day before Christmas Day` — `Tuesday 24 December` — `ALL` — `CLOSE EARLY` `[15]` — 14:10 (Sydney time) | `ASX-CAL-2013` | T1 | ASX trade date named verbatim |
| 2013-12-25 | closed | `Christmas Day` — `Wednesday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-12-26 | closed | `Boxing Day / Proclamation Day` — `Thursday 26 December` — `ALL / SA` — `CLOSED` | `ASX-CAL-2013` | T1 | ASX event date printed verbatim |
| 2013-12-31 | early close | `Last Business Day of the Year` — `Thursday 31 December` — `ALL` — `CLOSE EARLY` `[16]` — 14:10 (Sydney time) | `ASX-CAL-2013` | T1 | same reading as 2013-12-24 |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `New Year's Day` — `Wednesday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-01-27 | closed | `Australia Day` — `Monday 27 January` — `ALL` — `CLOSED` | `ASX-CAL-2014` | T1 | the sheet's own observed date for the Sunday 26 January holiday |
| 2014-04-18 | closed | `Good Friday` — `Friday 18 April` — `ALL` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-04-21 | closed | `Easter Monday` — `Monday 21 April` — `ALL` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-04-25 | closed | `ANZAC Day` — `Friday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-06-09 | closed | `Queen's Birthday` — `Monday 9 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-12-24 | early close | `Last Business Day before Christmas Day` — `Wednesday 24 December` — `ALL` — `CLOSE EARLY` `[15]` — 14:10 (Sydney time) | `ASX-CAL-2014` | T1 | ASX trade date named verbatim |
| 2014-12-25 | closed | `Christmas Day` — `Thursday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-12-26 | closed | `Boxing Day / Proclamation Day` — `Friday 26 December` — `ALL / SA` — `CLOSED` | `ASX-CAL-2014` | T1 | ASX event date printed verbatim |
| 2014-12-31 | early close | `Last Business Day of the Year` — `Wednesday 31 December` — `ALL` — `CLOSE EARLY` `[16]` — 14:10 (Sydney time) | `ASX-CAL-2014` | T1 | same reading as 2014-12-24 |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `New Year's Day` — `Thursday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2015` | T1 | ASX event date printed verbatim |
| 2015-01-26 | closed | `Australia Day` — `Monday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2015` | T1 | ASX event date printed verbatim |
| 2015-04-03 | closed | `Good Friday` — `Friday 3 April` — `ALL` — `CLOSED` | `ASX-CAL-2015` | T1 | ASX event date printed verbatim |
| 2015-04-06 | closed | `Easter Monday` — `Monday 6 April` — `ALL` — `CLOSED` | `ASX-CAL-2015` | T1 | ASX event date printed verbatim |
| 2015-04-25 | closed | `ANZAC Day` — `Saturday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2015` | T1 | the sheet prints the Saturday closure; the row restates the closure the Mon-Fri normal week has already made |
| 2015-06-08 | closed | `Queen's Birthday` — `Monday 8 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2015` | T1 | ASX event date printed verbatim |
| 2015-12-24 | early close | `Last Business Day before Christmas Day` — `Thursday 24 December` — `ALL` — `CLOSE EARLY` `[15]` — 14:10 (Sydney time) | `ASX-CAL-2015` | T1 | ASX trade date named verbatim |
| 2015-12-25 | closed | `Christmas Day` — `Friday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2015` | T1 | ASX event date printed verbatim |
| 2015-12-28 | closed | `Boxing Day / Proclamation Day` — `Monday 28 December` — `ALL / SA` — `CLOSED` | `ASX-CAL-2015` | T1 | the sheet's own observed date for the Saturday 26 December holiday |
| 2015-12-31 | early close | `Last Business Day of the Year` — `Thursday 31 December` — `ALL` — `CLOSE EARLY` `[17]` — 14:10 (Sydney time) | `ASX-CAL-2015` | T1 | same reading as 2015-12-24 |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `New Year's Day` — `Friday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2016` | T1 | ASX event date printed verbatim |
| 2016-01-26 | closed | `Australia Day` — `Tuesday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2016` | T1 | ASX event date printed verbatim |
| 2016-03-25 | closed | `Good Friday` — `Friday 25 March` — `ALL` — `CLOSED` | `ASX-CAL-2016` | T1 | ASX event date printed verbatim |
| 2016-03-28 | closed | `Easter Monday` — `Monday 28 March` — `ALL` — `CLOSED` | `ASX-CAL-2016` | T1 | ASX event date printed verbatim |
| 2016-04-25 | closed | `ANZAC Day` — `Monday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2016` | T1 | ASX event date printed verbatim |
| 2016-06-13 | closed | `Queen's Birthday` — `Monday 13 June` — `ALL except WA` — `CLOSED` | `ASX-CAL-2016` | T1 | ASX event date printed verbatim |
| 2016-12-23 | early close | `Last Business Day before Christmas Day` — `Friday 23 December` — `ALL` — `CLOSE EARLY` `[15]` — 14:10 (Sydney time) | `ASX-CAL-2016` | T1 | ASX trade date named verbatim |
| 2016-12-26 | closed | `Christmas Day` `[16]` — `Monday 26 December` — `ALL` — `CLOSED` | `ASX-CAL-2016` | T1 | the sheet's own observed date for the Sunday 25 December holiday |
| 2016-12-27 | closed | `Boxing Day / Proclamation Day` `[17]` — `Tuesday 27 December` — `ALL / SA` — `CLOSED` | `ASX-CAL-2016` | T1 | the sheet's own observed date |
| 2016-12-30 | early close | `Last Business Day of the Year` — `Friday 30 December` — `ALL` — `CLOSE EARLY` `[18]` — 14:10 (Sydney time) | `ASX-CAL-2016` | T1 | same reading as 2016-12-23 |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `New Year's Day` `[3]` — `Monday 2 January` — `ALL` — `CLOSED`; footnote: `When New Year's Day falls on a Sunday` | `ASX-CAL-2017` | T1 | the sheet's own observed date |
| 2017-01-26 | closed | `Australia Day` — `Thursday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2017` | T1 | ASX event date printed verbatim |
| 2017-04-14 | closed | `Good Friday` — `Friday 14 April` — `ALL` — `CLOSED` | `ASX-CAL-2017` | T1 | ASX event date printed verbatim |
| 2017-04-17 | closed | `Easter Monday` — `Monday 17 April` — `ALL` — `CLOSED` | `ASX-CAL-2017` | T1 | ASX event date printed verbatim |
| 2017-04-25 | closed | `ANZAC Day` — `Tuesday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2017` | T1 | ASX event date printed verbatim |
| 2017-06-12 | closed | `Queen's Birthday` — `Monday 12 June` — `ALL except WA & QLD` — `CLOSED` | `ASX-CAL-2017` | T1 | ASX event date printed verbatim |
| 2017-12-25 | closed | `Christmas Day` — `Monday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2017` | T1 | ASX event date printed verbatim |
| 2017-12-26 | closed | `Boxing Day / Proclamation Day / Christmas Day` `[3]` — `Tuesday 26 December` — `ALL except SA` — `CLOSED` | `ASX-CAL-2017` | T1 | the sheet's own observed date for the Monday 25 December holiday in the states that observe it then |

The 2017 sheet is the one year with no `Last Business Day` rows at all: its
table ends at Boxing Day and prints no `CLOSE EARLY` and no 14:10 footnote,
so no early close ships for 2017.

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `New Year's Day` — `Monday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-01-26 | closed | `Australia Day` — `Friday 26 January` — `ALL` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-03-30 | closed | `Good Friday` — `Friday 30 March` — `ALL` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-04-02 | closed | `Easter Monday` — `Monday 2 April` — `ALL` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-04-25 | closed | `ANZAC Day` — `Wednesday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-06-11 | closed | `Queen's Birthday` — `Monday 11 June` — `ALL except WA & QLD` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-12-24 | early close | `Last Business Day before Christmas Day` — `Monday 24 December` — `ALL` — `CLOSE EARLY` `[3]` — 14:10 (Sydney time) | `ASX-CAL-2018` | T1 | ASX trade date named verbatim |
| 2018-12-25 | closed | `Christmas Day` — `Tuesday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-12-26 | closed | `Boxing Day / Proclamation Day` — `Wednesday 26 December` — `ALL except SA & WA` — `CLOSED` | `ASX-CAL-2018` | T1 | ASX event date printed verbatim |
| 2018-12-31 | early close | `Last Business Day of the Year` — `Monday 31 December` — `ALL` — `CLOSE EARLY` `[3]` — 14:10 (Sydney time) | `ASX-CAL-2018` | T1 | same reading as 2018-12-24 |

The 2018 sheet was published in advance — the operator's own page as captured
2017-11-30, before the sheet's first trading day.

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `New Year's Day` — `Tuesday 1 January` — `ALL` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-01-28 | closed | `Australia Day` `[3]` — `Monday 28 January` — `ALL` — `CLOSED` | `ASX-CAL-2019` | T1 | the sheet's own observed date for the Saturday 26 January holiday |
| 2019-04-19 | closed | `Good Friday` — `Friday 19 April` — `ALL` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-04-22 | closed | `Easter Monday` — `Monday 22 April` — `ALL` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-04-25 | closed | `ANZAC Day` — `Thursday 25 April` — `ALL` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-06-10 | closed | `Queen's Birthday` — `Monday 10 June` — `ALL except WA & QLD` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-12-24 | early close | `Last Business Day before Christmas Day` — `Tuesday 24 December` — `ALL` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-2019` | T1 | ASX trade date named verbatim |
| 2019-12-25 | closed | `Christmas Day` — `Wednesday 25 December` — `ALL` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-12-26 | closed | `Boxing Day / Proclamation Day` — `Thursday 26 December` — `ALL except SA / SA` — `CLOSED` | `ASX-CAL-2019` | T1 | ASX event date printed verbatim |
| 2019-12-31 | early close | `Last Business Day of the Year` — `Tuesday 31 December` — `ALL` — `CLOSE EARLY` `[5]` — 14:10 (Sydney time) | `ASX-CAL-2019` | T1 | same reading as 2019-12-24 |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `New Year's Day` — `Wednesday 1 January` — `CLOSED` | `ASX-CAL-2020` | T1 | ASX event date printed verbatim |
| 2020-01-27 | closed | `Australia Day` `[3]` — `Monday 27 January` — `CLOSED`; footnote: `As 26 Jan falls on a Sunday, the following Monday is observed` | `ASX-CAL-2020` | T1 | the sheet's own observed date and footnote |
| 2020-04-10 | closed | `Good Friday` — `Friday 10 April` — `CLOSED` | `ASX-CAL-2020` | T1 | ASX event date printed verbatim |
| 2020-04-13 | closed | `Easter Monday` — `Monday 13 April` — `CLOSED` | `ASX-CAL-2020` | T1 | ASX event date printed verbatim |
| 2020-04-25 | closed | `ANZAC Day` `[6]` — `Saturday 25 April` — `CLOSED`; footnote: `No substitute holiday on Monday 27th April` | `ASX-CAL-2020` | T1 | the sheet prints the Saturday closure and states no substitution; the row restates the closure the Mon-Fri normal week has already made |
| 2020-06-08 | closed | `Queen's Birthday` — `Monday 8 June` — `CLOSED` | `ASX-CAL-2020` | T1 | ASX event date printed verbatim |
| 2020-12-24 | early close | `Last Business Day before Christmas Day` — `Thursday 24 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-2020` | T1 | ASX trade date named verbatim |
| 2020-12-25 | closed | `Christmas Day` — `Friday 25 December` — `CLOSED` | `ASX-CAL-2020` | T1 | ASX event date printed verbatim |
| 2020-12-28 | closed | `Boxing Day` — `Monday 28 December` — `CLOSED` | `ASX-CAL-2020` | T1 | the sheet's own observed date for the Saturday 26 December holiday |
| 2020-12-31 | early close | `Last Business Day of the Year` — `Thursday 31 December` — `CLOSE EARLY` `[5]` — 14:10 (Sydney time) | `ASX-CAL-2020` | T1 | same reading as 2020-12-24 |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `New Year's Day` — `Friday 1 January` — `CLOSED` | `ASX-CAL-2021` | T1 | ASX event date printed verbatim |
| 2021-01-26 | closed | `Australia Day` — `Tuesday 26 January` — `CLOSED` | `ASX-CAL-2021` | T1 | ASX event date printed verbatim |
| 2021-04-02 | closed | `Good Friday` — `Friday 2 April` — `CLOSED` | `ASX-CAL-2021` | T1 | ASX event date printed verbatim |
| 2021-04-05 | closed | `Easter Monday` — `Monday 5 April` — `CLOSED` | `ASX-CAL-2021` | T1 | ASX event date printed verbatim |
| 2021-04-25 | closed | `ANZAC Day` — `Sunday 25 April` — `CLOSED` | `ASX-CAL-2021` | T1 | the sheet prints the Sunday closure; the row restates the closure the Mon-Fri normal week has already made |
| 2021-06-14 | closed | `Queen's Birthday` — `Monday 14 June` — `CLOSED` | `ASX-CAL-2021` | T1 | ASX event date printed verbatim |
| 2021-12-24 | early close | `Last Business Day before Christmas Day` — `Friday 24 December` — `CLOSE EARLY` `[3]` — 14:10 (Sydney time) | `ASX-CAL-2021` | T1 | ASX trade date named verbatim |
| 2021-12-27 | closed | `Christmas Day` `[4]` — `Monday 27 December` — `CLOSED`; footnote: `Substitute for Saturday 25 December` | `ASX-CAL-2021` | T1 | the sheet's own substitute date |
| 2021-12-28 | closed | `Boxing Day` `[5]` — `Tuesday 28 December` — `CLOSED`; footnote: `Substitute for Sunday 26 December` | `ASX-CAL-2021` | T1 | the sheet's own substitute date |
| 2021-12-31 | early close | `Last Business Day of the Year` — `Friday 31 December` — `CLOSE EARLY` `[6]` — 14:10 (Sydney time) | `ASX-CAL-2021` | T1 | same reading as 2021-12-24 |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-03 | closed | `New Year's Day` `[3]` — `Monday 3 January` — `CLOSED`; footnote: `Substitute for Saturday 1 January` | `ASX-CAL-2022` | T1 | the sheet's own substitute date |
| 2022-01-26 | closed | `Australia Day` — `Wednesday 26 January` — `CLOSED` | `ASX-CAL-2022` | T1 | ASX event date printed verbatim |
| 2022-04-15 | closed | `Good Friday` — `Friday 15 April` — `CLOSED` | `ASX-CAL-2022` | T1 | ASX event date printed verbatim |
| 2022-04-18 | closed | `Easter Monday` — `Monday 18 April` — `CLOSED` | `ASX-CAL-2022` | T1 | ASX event date printed verbatim |
| 2022-04-25 | closed | `ANZAC Day` — `Monday 25 April` — `CLOSED` | `ASX-CAL-2022` | T1 | ASX event date printed verbatim |
| 2022-06-13 | closed | `Queen's Birthday` — `Monday 13 June` — `CLOSED` | `ASX-CAL-2022` | T1 | ASX event date printed verbatim |
| 2022-09-22 | closed | `National Day of Mourning for Her Majesty the Queen` — `Thursday 22 September` — `CLOSED` | `ASX-CAL-2022` | T1 | the sheet's own unscheduled closure, added between the July and October 2022 replays (see the coverage paragraph) |
| 2022-12-26 | closed | `Boxing Day` — `Monday 26 December` — `CLOSED` | `ASX-CAL-2022` | T1 | ASX event date printed verbatim |
| 2022-12-27 | closed | `Christmas Day` `[4]` — `Tuesday 27 December` — `CLOSED`; footnote: `Substitute for Sunday 25 December` | `ASX-CAL-2022` | T1 | the sheet's own substitute date |

The 2022 sheet prints both year-end rows `OPEN` (`Friday 23 December` and
`Friday 30 December`, business days `YES`), so no early close ships for 2022;
the `OPEN` rows are audited-normal trading days. The July 2022 replay
(`wb_20220704090642`, in the store) prints the same sheet without the
mourning-day row and corroborates every other cell.

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `New Year's Day` `[3]` — `Monday 2 January` — `CLOSED`; footnote: `Substitute for Sunday 1 January` | `ASX-CAL-2023` | T1 | the sheet's own substitute date |
| 2023-01-26 | closed | `Australia Day` — `Thursday 26 January` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |
| 2023-04-07 | closed | `Good Friday` — `Friday 7 April` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |
| 2023-04-10 | closed | `Easter Monday` — `Monday 10 April` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |
| 2023-04-25 | closed | `ANZAC Day` — `Tuesday 25 April` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |
| 2023-06-12 | closed | `King's Birthday` — `Monday 12 June` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |
| 2023-12-25 | closed | `Christmas Day` — `Monday 25 December` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |
| 2023-12-26 | closed | `Boxing Day` — `Tuesday 26 December` — `CLOSED` | `ASX-CAL-2023` | T1 | ASX event date printed verbatim |

The 2023 sheet prints both year-end rows `OPEN` (`Friday 22 December` and
`Friday 29 December`, business days `YES`), so no early close ships for 2023;
the `OPEN` rows are audited-normal trading days.

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `New Year's Day` — `Monday 1 January` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-01-26 | closed | `Australia Day` — `Friday 26 January` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-03-29 | closed | `Good Friday` — `Friday 29 March` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-04-01 | closed | `Easter Monday` — `Monday 1 April` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-04-25 | closed | `ANZAC Day` — `Thursday 25 April` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-06-10 | closed | `King's Birthday` — `Monday 10 June` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-12-24 | early close | `Last Business Day before Christmas Day` — `Tuesday 24 December` — `CLOSE EARLY` `[3]` — 14:10 (Sydney time) | `ASX-CAL-2024` | T1 | ASX trade date named verbatim |
| 2024-12-25 | closed | `Christmas Day` — `Wednesday 25 December` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-12-26 | closed | `Boxing Day` — `Thursday 26 December` — `CLOSED` | `ASX-CAL-2024` | T1 | ASX event date printed verbatim |
| 2024-12-31 | early close | `Last Business Day of the Year` — `Tuesday 31 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-2024` | T1 | same reading as 2024-12-24 |

The 2024 capture also renders the operator's 2025 sheet, cell-for-cell
identical to `ASX-CAL-2025`'s, and corroborates it.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day` — `Wednesday 1 January` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim; `A Trading Day means ASX Trade is open for trading`, and no session belongs to the date |
| 2025-01-27 | closed | `Australia Day` — `Monday 27 January` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim (the sheet's own observed date for Sunday 26 January) |
| 2025-04-18 | closed | `Good Friday` — `Friday 18 April` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-04-21 | closed | `Easter Monday` — `Monday 21 April` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-04-25 | closed | `ANZAC Day` — `Friday 25 April` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-06-09 | closed | `King's Birthday` — `Monday 9 June` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-12-24 | early close | `Last Business day before Christmas Day` — `Wednesday 24 December` — `CLOSE EARLY` `[3]` — footnote: `Normal trading ceases at 14:10 (Sydney time)` | `ASX-CAL-2025` | T1 | ASX trade date named verbatim; the row clips the envelope at the sheet's own 14:10 |
| 2025-12-25 | closed | `Christmas Day` — `Thursday 25 December` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-12-26 | closed | `Boxing Day` — `Friday 26 December` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-12-31 | early close | `Last Business day of the Year` — `Wednesday 31 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-2025` | T1 | same reading as 2025-12-24 |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day` — `Thursday 1 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-01-26 | closed | `Australia Day` — `Monday 26 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-04-03 | closed | `Good Friday` — `Friday 3 April` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-04-06 | closed | `Easter Monday` — `Monday 6 April` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-04-25 | closed | `ANZAC Day` — `Saturday 25 April` — `CLOSED`, no substitute footnote | `ASX-CAL-LIVE` | T1 | the sheet prints the Saturday closure and states no substitution; the row restates a closure the Mon-Fri normal week already makes, so it changes no answer — recorded so the transcription matches the sheet line for line |
| 2026-06-08 | closed | `King's Birthday` — `Monday 8 June` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-12-24 | early close | `Last Business day before Christmas Day` — `Thursday 24 December` — `CLOSE EARLY` `[3]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | ASX trade date named verbatim |
| 2026-12-25 | closed | `Christmas Day` — `Friday 25 December` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-12-28 | closed | `Boxing Day` — `Monday 28 December` — `CLOSED` | `ASX-CAL-LIVE` | T1 | the sheet's own date for the Saturday 26 December holiday |
| 2026-12-31 | early close | `Last Business day of the Year` — `Thursday 31 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | same reading as 2026-12-24 |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day` — `Friday 1 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-01-26 | closed | `Australia Day` — `Tuesday 26 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-03-26 | closed | `Good Friday` — `Friday 26 March` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-03-29 | closed | `Easter Monday` — `Monday 29 March` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-06-14 | closed | `King's Birthday` — `Monday 14 June` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-12-24 | early close | `Last Business day before Christmas Day` — `Friday 24 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | ASX trade date named verbatim |
| 2027-12-27 | closed | `Christmas Day` — `Monday 27 December` — `CLOSED` `[5]` — `Substitute for Saturday 25 December` | `ASX-CAL-LIVE` | T1 | the sheet's own substitute date, printed in the event row |
| 2027-12-28 | closed | `Boxing Day` — `Tuesday 28 December` — `CLOSED` `[6]` — `Substitute for Sunday 26 December` | `ASX-CAL-LIVE` | T1 | the sheet's own substitute date |
| 2027-12-31 | early close | `Last Business day of the Year` — `Friday 31 December` — `CLOSE EARLY` `[7]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | same reading as 2027-12-24 |

**ANZAC Day 2027 ships no row.** The 2027 sheet prints `ANZAC Day` /
`Monday 26 April` / `OPEN` `[3]` / Settlement / Settlement / `YES`, with the
footnote `Substitute for Sunday 25 April`. The sheet states a trading day, so
the Monday is audited normal: recording it as a row would claim a closure the
operator's own bytes deny. No other sheet entry is without a row; ASX's
standing note (`ASX Limited reserves the right to declare additional
Non-Business Days … without notice`) is the operator's own alteration clause
and is covered by the monthly watch.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `ASX-CAL-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100128204223id_/http://www.asx.com.au/about/operational/trading_calendar/asx/2010.htm> | Wayback `id_` replay of capture `20100128204223`, retrieved 2026-09-29 04:46:47 UTC | T1 | `9c7a10ec0c70a107c1681ed913eb75ca83db423e785a3a67a75d4e6b243a8e43` |
| `ASX-CAL-2011` | 2011-01-03 .. 2011-12-30 | <https://web.archive.org/web/20110222083832id_/http://www.asx.com.au/about/asx-trading-calendar-2011.htm> | Wayback `id_` replay of capture `20110222083832`, retrieved 2026-09-29 04:46:49 UTC | T1 | `dd87c3f5f34189c8f1001d47fd7750f5642bdd585762d197851bc1b7a01464f6` |
| `ASX-CAL-2012` | 2012-01-02 .. 2012-12-31 | <https://web.archive.org/web/20111003145947id_/http://www.asx.com.au/trading_services/asx-trading-calendar-2012.htm> | Wayback `id_` replay of capture `20111003145947`, retrieved 2026-09-29 04:46:51 UTC | T1 | `4647719aa62b75a3edd1df1fedd64f18e0e25af0a285a8bcf345df4422cc2f49` |
| `ASX-CAL-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20131007101528id_/http://www.asx.com.au/about/asx-trading-calendar-2013.htm> | Wayback `id_` replay of capture `20131007101528`, retrieved 2026-09-29 04:48:10 UTC | T1 | `742815c38a02939df7288ad3debdfda96859cbfef2206b131f6b9e4f04017e5b` |
| `ASX-CAL-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20131223094442id_/http://www.asx.com.au/about/asx-trading-calendar-2014.htm> | Wayback `id_` replay of capture `20131223094442`, retrieved 2026-09-29 04:48:12 UTC | T1 | `d513d1f7e5e2272fc687ebed49916ab84c03eca9c792ef367413423eff10a4e6` |
| `ASX-CAL-2015` | 2015-01-01 .. 2015-12-31 | <https://web.archive.org/web/20141130070011id_/http://www.asx.com.au/about/asx-trading-calendar-2015.htm> | Wayback `id_` replay of capture `20141130070011`, retrieved 2026-09-29 04:48:15 UTC | T1 | `6c9968c3839b5247bf21bb921c467639d1f33f735cf5b0b1caacbe87b15f8be0` |
| `ASX-CAL-2016` | 2016-01-01 .. 2016-12-30 | <https://web.archive.org/web/20151005225533id_/http://www.asx.com.au/about/asx-trading-calendar-2016.htm> | Wayback `id_` replay of capture `20151005225533`, retrieved 2026-09-29 04:48:18 UTC | T1 | `c3ce42dbf436375a324aa2887777b4985600bacd3ba45f2d65b42fbfb74249eb` |
| `ASX-CAL-2017` | 2017-01-02 .. 2017-12-26 | <https://web.archive.org/web/20170417230107id_/http://www.asx.com.au/about/asx-trading-calendar-2017.htm> | Wayback `id_` replay of capture `20170417230107`, retrieved 2026-09-29 04:48:27 UTC | T1 | `c31a32fa8cd7f396fce3e7e50a57fcc8962b41132d4a5198edf8854af668db34` |
| `ASX-CAL-2018` | 2018-01-01 .. 2018-12-31 | <https://web.archive.org/web/20171130111721id_/http://www.asx.com.au/about/asx-trading-calendar-2018.htm> | Wayback `id_` replay of capture `20171130111721`, retrieved 2026-09-29 04:48:30 UTC | T1 | `3711528e0b8625a86f59e98d9ad9084df2954c70bbc51be0008d902dfdb1ee44` |
| `ASX-CAL-2019` | 2019-01-01 .. 2019-12-31 | <https://web.archive.org/web/20190821015905id_/https://www.asx.com.au/about/asx-trading-calendar-2019.htm> | Wayback `id_` replay of capture `20190821015905`, retrieved 2026-09-29 04:48:32 UTC | T1 | `8a7ddb8f457f569173e06afdcc669d59ba7ccecf761483fe5f85d169a054fe4c` |
| `ASX-CAL-2020` | 2020-01-01 .. 2020-12-31 | <https://web.archive.org/web/20201022091634id_/https://www2.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20201022091634`, retrieved 2026-09-29 04:48:35 UTC | T1 | `50d80e0783d00455d6139a1f0130d2438c3e66e771459dec50da02f570503af1` |
| `ASX-CAL-2021` | 2021-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210127034401id_/https://www2.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20210127034401`, retrieved 2026-09-29 04:48:47 UTC | T1 | `aa7d0918d5951718232b2b5dd5290e9aad27db2f01bea08bd2325f5439ee34f2` |
| `ASX-CAL-2022` | 2022-01-03 .. 2022-12-30 | <https://web.archive.org/web/20221018155834id_/https://www2.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20221018155834`, retrieved 2026-09-29 04:53:10 UTC | T1 | `f7914433315f8199e698c877a2ede306c56230994e3296ef2a4cd002fd49468e` |
| `ASX-CAL-2023` | 2023-01-02 .. 2023-12-29 | <https://web.archive.org/web/20230806122301id_/https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20230806122301`, retrieved 2026-09-29 04:48:52 UTC | T1 | `ce0453dd3ef052808cb9d0c5d68ffa3878fa2aa7eb3c904162dd54ad5c6f234e` |
| `ASX-CAL-2024` | 2024-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240824000246id_/https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20240824000246`, retrieved 2026-09-29 04:49:09 UTC (stored after decoding the replay's gzip content-encoding; the digest is of the stored decoded bytes) | T1 | `e4959274c9a937e157e5f5d2300a5d88d75f4c69404654b703ccbae4a9b3295c` |
| `ASX-CAL-2025` | 2025-01-01 .. 2027-12-31 | <https://web.archive.org/web/20250416082951id_/https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20250416082951`, retrieved 2026-09-28 01:51 UTC | T1 | `d24de6d6f6ec1864480de6f2f75cf4a3650b30f6eda8daa354fd1bfa1302d66d` |
| `ASX-CAL-LIVE` | 2025-01-01 .. 2027-12-31 | <https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | retrieved 2026-09-28 01:00 UTC | T1 | `adb2344ca5e13dcbfb8de9b0cf40334c992f4ffb660026b22bca7d21d964cd19` |

`ASX-CAL-2025`'s replayed page renders only the 2025 sheet; its window cell
names the table window it keys rows inside, not the years it prints.
`ASX-CAL-2022`'s sheet carries the National Day of Mourning row; the earlier
July 2022 replay (`wb_20220704090642`, `680b0c86…`, in the store, cited by no
row) prints the same sheet without it and corroborates every other cell. The
2020 replay also renders the operator's 2021 sheet and corroborates
`ASX-CAL-2021`; the 2022-01 replay renders the 2021 sheet with updated
footnote text and corroborates it likewise. The store's
`holidays/raw/equities/asx/2025-2027/` also holds the calendar hub page
(`7ea4e039…`) through which the trading-calendar URL was located.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-SR15 baseline rests only on the SR15 marked procedure amendments, whose publication day is not recorded in the repository (the notice index places it in April 2025). The ledger horizon is therefore 2025-06-23, the first day at which this row's state is sourced, with everything below it carried. Closing condition: read the marked amendments' own publication date, or find an earlier dated ASX procedure edition stating the staggered-open table; either would move the horizon earlier. Tracked as #231, the carried-horizon tracker this scope shares with `nasdaq` (LAW-FOLLOW-UPS-ARE-ISSUES).
- **The opening edge, stated against what the module encodes.** ASX's cash-market timetable prints *nominal* boundaries: Opening Single Price Auction 09:59:00–09:59:45, then Open (Normal Trading) 09:59:45–16:00:00. `asx.rs` does not encode 09:59:45 as the start of `regular`. `ASX_EXTENDED_CURRENT` carries one rule over the whole opening minute, 09:59:00–10:00:00, and `ASX_REGULAR` runs 10:00:00–16:00:00, so the crate reports the market open from 09:59:00 — the auction matches, so a price can print there — and defers *continuous* trading to 10:00:00, the latest instant at which it can have begun. That is the conservative envelope AGENTS.md's *Exchange-level boundaries, not per-security auction outcomes* calls for: the uncross is randomised per security around the nominal 09:59:45 handoff, so naming any second inside 09:59:45–10:00:00 as the continuous-trading start would imply ticker-level uncross timing the exchange does not publish. Nothing is under-reported as closed by this choice; only the `regular`/`extended` split inside that minute is conservative.
- Both single-price auctions match, and in Post Close "ASX matches orders at the CSPA price", so the opening auction, the CSPA and Post Close are all tradeable `extended`: 09:59:00–10:00:00 on the open side and 16:10:00–16:21:30 on the close side, the latter merging CSPA 16:10–16:11 with Post Close 16:11–16:21:30 into one rule.
- Pre-open is `extended`, not `order_entry`: ASX Trade does not match in it, but overnight and overseas trades report until 09:45 and other allowable trades may be reported under the Operating Rules, so a price can print.
- The only order-entry-only window is Pre-CSPA 16:00–16:10, in which continuous matching ceases and only entry and amendment are accepted.
- **Pre-SR15 era, same convention.** `ASX_EXTENDED_PRE_2025_06_23` spans 09:59:45–10:09:15 — Group 1's nominal 10:00 open less its ±15-second randomization, through Group 5's nominal 10:09 plus the same — while `ASX_REGULAR` is shared with the current era and still starts at 10:00:00. The envelope therefore covers every group's possible open and the `regular` edge names Group 1's nominal transition, not any group's realised one.
