<!-- SPDX-License-Identifier: MIT-0 -->

# `cfe` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cfe.rs`](../../src/calendar/schedules/futures/us/cfe.rs)
- **Source sets:** [`US-CFE`](../schedules/sources.md#us-cfe)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

VIX futures normal-week history is complete from January 2010. Old-system pre-opens have exact sourced onsets; randomized new-system queue starts use conservative latest edges of 16:00:03/16:45:03 from 2018-02-25 and 16:00:06/16:45:06 from 2018-08-12.

## Revision rows

- 2010-12-10 — T1 — Cboe SR-CFE-2010-013 — a 07:20–08:30 CT extended session is added ahead of the unchanged 08:30–15:15 RTH.
- 2011-09-26 — T1 — Cboe SR-CFE-2011-019 — the extended session start moves from 07:20 to 07:00 CT.
- 2013-10-28 — T1 — Cboe IC13-041 — the Monday–Thursday 15:30–16:15 CT session opens, with its 15:29–15:30 pre-open queue.
- 2013-11-04 — T1 — Cboe IC13-041 — the morning open moves from 07:00 to 02:00 CT.
- 2014-06-22 — T1 — Cboe RG-CFE-2014-020 — near-24-hour VX trading begins, adding the Sunday 16:15–17:00 CT pre-open.
- 2018-02-25 — T1 — Cboe RG-CFE-2018-005 — system migration: a 15:15–15:30 CT queue, 15:30–16:00 ETH, and the 16:00:03 / 16:45:03 CT queue edges.
- 2018-08-12 — T1 — Cboe C2018071603 — TAS queue commencement widens to six seconds, so the conservative edges become 16:00:06 and 16:45:06 CT.
- 2021-12-06 — T1 — Cboe C2021102603 — the current grid: RTH 08:30–15:00 CT, ETH 15:00–16:00 and 17:00–08:30, queues at 16:00:06 and 16:45:06 CT.

## Holidays

**Coverage:** 2017-04-10..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves the `cfe` venue and the `cfe_vix` key: Cboe publishes one holiday schedule for all CFE futures, and VIX futures are the only family the crate routes to the venue, so the venue intersection is that one family's own table.

Two document families key the window. The **rules page** — the operator's own `CFE Holiday Schedule` page on `cfe.cboe.com/about-cfe/holiday-calendar`, read through Wayback `id_` captures — prints the complete holiday calendar for the years it shows (2017, then 2018 and 2019, then 2019 and 2020) and the per-holiday-type hours tables in session language; it keys the 2017 rows and the 2018 New Year's Day row, because no CFE notice for those dates survives. The **per-holiday notices** under `cdn.cboe.com/resources/schedule_update/<publication year>/` state each 2018-2026 holiday's session instants and the observed-day arrangements, and key every other row. The document ids record which: `CBOE-SU-<holiday year>-<HOLIDAY>` for a per-holiday notice, `cfe-holiday-calendar @<capture instant>` for a rules-page capture, and `CBOE-HOURS-USFUT-2026` for the 2026 page. A notice's id year is the **holiday year the notice governs**, not the year of the directory it is served from: Cboe files each New Year's and MLK notice under the *previous* year, so the notices governing 2025-01-01 and 2025-01-20 are served from `schedule_update/2024/` and the ones governing 2026-01-01 and 2026-01-19 from `schedule_update/2025/`.

**Shapes that repeat across the whole window.** The Monday/Thursday and mid-week floating holidays keep the overnight leg running to 10:30 CT with no regular session (`early close 10:30`), 2017 through 2026 alike — the rules page's Monday-holiday chart and each year's notice agree. Good Friday alternates between a full closure (2017, 2018, 2019, 2020, 2022, 2024, 2025 — each stated outright by its own document) and an early close at the regular open (2021, 2023, 2026 — the overnight leg stops at 8:30 a.m., Regular `None`). New Year's Day and Christmas falling Monday-Thursday print no holiday-day session at all and reopen at 17:00 CT on the holiday, so the trade date is `Closed` and the prior-evening leg is deleted with it. The Independence Day and Christmas eves close at 12:15 CT where the year's notice says so (2019-07-03, 2023-07-03, 2024-07-03, and the December eves of 2018, 2019, 2020 and 2024) and trade normally where it does not (2018-07-03, whose notice's holiday leg opens 5:00 p.m. Tuesday, and 2022-12-23, whose notice prints the normal 3:00/4:00 PM closes). Juneteenth enters the observed set in 2022; the operator's own notice states CFE traded unadjusted hours in 2021.

**Gaps, 2010-01-01..2017-04-09 (unaudited span).** The window opens at 2017-04-10 because that is the first day the earliest surviving operator artifact — the Wayback capture of the CFE Holiday Schedule page — can speak about: it prints the complete 2017 calendar, so the rows from Good Friday 2017-04-14 onward are sourced, but no CFE notice, circular or rules-page capture that keys an earlier holiday's session survives to read. The span is **unaudited**, not audited normal: outside the coverage window the table has no answer, and 2017-01-02 (New Year's Day observed), 2017-01-16 and 2017-02-20 (both named by the 2017 calendar but before the first capture) ship no row. This is a Phase 2 record of the 2026-09-27 amendment's restored 2010 floor, not a claim. **Closing condition:** a pre-April-2017 CFE holiday circular or notice set, or a rules-page capture from before 2017-04-10 that prints the governing calendar and hours charts.

### 2017

The 2017 rows are read from the operator's own `CFE Holiday Schedule` page, whose captures print the complete **2017 CFE Holiday Calendar** — `New Year's Day – Monday, January 2 (observed)` through `Christmas Day – Monday, December 25`, nine dates — and the per-holiday-type hours tables. Each row below cites the latest capture that precedes its holiday; the five captures used here (2017-04-10, 2017-06-26, 2017-11-13, 2017-12-29 and, as corroboration, 2019-12-15) print the same 2017 calendar, and their hours tables agree except the Independence Day Extended leg — the April edition prints `5:00 p.m. (July 3)` where the June edition prints `3:30 p.m. (July 3)`; the row below keys on the June printing, and the shipped 10:30 close holds under both.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | `If the holiday is on a Friday: New Year's Day, Good Friday, Independence Day and Christmas — Extended None / Regular None` | `cfe-holiday-calendar @2017-04-10T21:04:29Z` | T1 | The 2017 calendar names `Good Friday – Friday, April 14`; the Friday chart prints no session at all for the day, so the Thursday-evening leg is deleted with it |
| 2017-05-29 | early close | `Domestic Holidays Always Observed on Mondays … Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `cfe-holiday-calendar @2017-04-10T21:04:29Z` | T1 | The 2017 calendar names `Memorial Day – Monday, May 29`; 10:30 CT |
| 2017-07-03 | unsourced | `The Exchange will typically close at 12:15 p.m. on July 3 (the day before Independence Day) and December 24 (Christmas Eve). Holiday closures and shortened holiday trading hours will be announced by circular.` | `cfe-holiday-calendar @2017-06-26T00:36:49Z` | T1 | The page states the eve close only as a default ("typically") and defers the year's answer to a circular; no 2017 CFE circular or notice survives (see the gap note below), while 2018 is witnessed as a normal-trading July 3 by its notice's leg structure and 2019, 2023 and 2024 as 12:15 years by their notices — so the day is withheld rather than guessed |
| 2017-07-04 | early close | `If Independence Day is on a Monday - Thursday: Extended 3:30 p.m. (July 3) to 10:30 a.m.* / Regular None` | `cfe-holiday-calendar @2017-06-26T00:36:49Z` | T1 | The 2017 calendar names `Independence Day – Tuesday, July 4`; the July 3 evening leg is trade date 2017-07-04's own and stops at 10:30, and no July 3 row is shipped because the chart prints none for it |
| 2017-09-04 | early close | `Domestic Holidays Always Observed on Mondays … Regular None, extended to 10:30 a.m.` | `cfe-holiday-calendar @2017-06-26T00:36:49Z` | T1 | The 2017 calendar names `Labor Day – Monday, September 4`; 10:30 CT |
| 2017-11-23 | early close | `Thanksgiving … Extended 3:30 p.m. (Wednesday) to 10:30 a.m. / Regular None` | `cfe-holiday-calendar @2017-11-13T01:40:35Z` | T1 | The 2017 calendar names `Thanksgiving Day – Thursday, November 23`; the holiday trade date's own leg stops at 10:30 CT. The chart's `3:30 p.m. (Wednesday)` start lumps the pre-migration Wednesday tail session (15:30-16:15 CT) into one cell with the evening leg; under the crate's trade-date key that tail session closes 16:15 Wednesday and belongs to Wednesday's own trade date, so the cell and the 2018 notices' `5:00 p.m. (Wednesday)` wording state the same trade-date boundary |
| 2017-11-24 | early close | `Thanksgiving … Friday: Regular 8:30 a.m. to 12:15 p.m.` | `cfe-holiday-calendar @2017-11-13T01:40:35Z` | T1 | The chart prints the Friday regular session 8:30-12:15, so only the close moves; 12:15 CT |
| 2017-12-25 | closed | `If New Years Day or Christmas is on a Monday - Thursday: Extended 5:00 p.m. (on holiday) to 8:30 a.m. (day after holiday) / Regular 8:30 a.m. to 3:15 p.m. (day after holiday)` | `cfe-holiday-calendar @2017-11-13T01:40:35Z` | T1 | The 2017 calendar names `Christmas Day – Monday, December 25`; the chart prints no holiday-day session and the first block opens 17:00 CT on the holiday itself, so the trade date is deleted and the Sunday-evening leg with it. December 24 2017 was a Sunday, so the `typically 12:15` Christmas-Eve default has no session to shorten |

**The 2017-07-03 withholding.** The rules page is the only surviving operator document that touches the day, and it states a default, not the year's arrangement: `typically close at 12:15 p.m. on July 3`. The same page defers `shortened holiday trading hours` to circulars, and no 2017 CFE circular or notice survives — the `schedule_update/2017` directory holds only options-exchange `Cboe-Holiday-Reminder` PDFs, the archived CFE circular series (`cfe.cboe.com/publish/CFEinfocirc*`) carries no 2017 holiday content, and the live general-circular index begins at `CFE-IC-2017-001`. The neighbouring years prove the default is year-dependent, not a rule: 2018-07-03 traded normally (the 2018 notice's holiday leg opens `5:00 p.m. (Tuesday)`, which a 12:15 close would have deleted) while 2019-07-03, 2023-07-03 and 2024-07-03 closed at 12:15 by their notices' own statements. So 2017-07-03 ships `Unsourced`: the operator published nothing this crate could read, not proof of a normal day. Closing condition: a 2017 CFE holiday circular or notice, or a rules-page capture that states the day's arrangement unconditionally.

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `If New Years Day or Christmas is on a Monday - Thursday: Extended 5:00 p.m. (on holiday) to 8:30 a.m. (day after holiday)` | `cfe-holiday-calendar @2017-12-29T07:06:24Z` | T1 | The page's `2018 Exchange Holiday Calendar` names `New Year's Day – Monday, January 1`; the Monday-Thursday chart prints no holiday-day session, so the trade date is deleted and the Sunday-evening leg with it |
| 2018-01-15 | early close | `Monday, January 15, 2018: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2018-MLK` | T1 | Cboe event date 2018-01-15; the Sunday-evening leg is trade date 2018-01-15's own and stops at 10:30, and the Monday-evening leg is trade date 2018-01-16's |
| 2018-02-19 | early close | `Monday, February 19, 2018: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2018-PRESIDENTS` | T1 | Cboe event date 2018-02-19; same two-leg shape as MLK Day |
| 2018-03-30 | closed | `Trading will be closed for all CFE products on Friday, March 30, 2018.` | `CBOE-SU-2018-GOOD-FRIDAY` | T1 | Cboe event date 2018-03-30; the notice states the closure outright and the reopen `at 5:00 p.m. on April 1, 2018 with the regularly scheduled start of extended trading hours` |
| 2018-05-28 | early close | `Monday, May 28, 2018: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2018-MEMORIAL` | T1 | Cboe event date 2018-05-28 |
| 2018-07-04 | early close | `Wednesday, July 4, 2018: Extended 5:00 p.m. (Tuesday) to 10:30 a.m. / Regular None` | `CBOE-SU-2018-INDEPENDENCE` | T1 | Cboe event date 2018-07-04; the Tuesday-evening leg is trade date 2018-07-04's own and stops at 10:30. The same leg proves 2018-07-03 traded normally — a 12:15 Tuesday close would have deleted it |
| 2018-09-03 | early close | `Monday, September 3, 2018: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2018-LABOR` | T1 | Cboe event date 2018-09-03 |
| 2018-11-22 | early close | `Thursday, November 22, 2018: Extended 5:00 p.m. (Wednesday) to 10:30 a.m. / Regular None` | `CBOE-SU-2018-THANKSGIVING` | T1 | Cboe event date 2018-11-22; the notice also states `CFE will have normal trading hours … on Wednesday, November 21, 2018` |
| 2018-11-23 | early close | `Friday, November 23, 2018: Regular 8:30 a.m. to 12:15 p.m.` | `CBOE-SU-2018-THANKSGIVING` | T1 | Cboe prints the Friday regular session 8:30-12:15, so only the close moves |
| 2018-12-24 | early close | `Trading in all CFE products will close at 12:15 p.m. on Monday, December 24, 2018.` | `CBOE-SU-2018-CHRISTMAS` | T1 | Cboe trade date named verbatim; the first open is the normal Sunday 17:00 CT, so only the close moves |
| 2018-12-25 | closed | `CFE will reopen following the Christmas holiday at 5:00 p.m. on Tuesday, December 25, 2018 with the start of extended trading hours` | `CBOE-SU-2018-CHRISTMAS` | T1 | Cboe event date 2018-12-25; the reopen on the holiday itself means no session belongs to the trade date |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `CFE will reopen following the New Year's Day holiday at 5:00 p.m. on Tuesday, January 1, 2019` and `CFE will have normal trading hours for all products on Monday, December 31, 2018.` | `CBOE-SU-2019-NEW-YEAR` | T1 | Cboe event date 2019-01-01; the reopen on the holiday itself means no session belongs to the trade date, and the notice states the prior Monday's normal hours |
| 2019-01-21 | early close | `Monday, January 21, 2019: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2019-MLK` | T1 | Cboe event date 2019-01-21 |
| 2019-02-18 | early close | `Monday, February 18, 2019: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2019-PRESIDENTS` | T1 | Cboe event date 2019-02-18 |
| 2019-04-19 | closed | `Trading will be closed for all CFE products on Friday, April 19, 2019.` | `CBOE-SU-2019-GOOD-FRIDAY` | T1 | Cboe event date 2019-04-19; the notice states the closure and the Sunday 2019-04-21 reopen |
| 2019-05-27 | early close | `Monday, May 27, 2019: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2019-MEMORIAL` | T1 | Cboe event date 2019-05-27 |
| 2019-07-03 | early close | `Trading in all CFE products will close at 12:15 p.m. on Wednesday, July 3, 2019.` | `CBOE-SU-2019-INDEPENDENCE` | T1 | Cboe trade date named verbatim; the 12:15 eve before the Thursday holiday |
| 2019-07-04 | early close | `Thursday, July 4, 2019: Extended 5:00 p.m. (Wednesday) to 10:30 a.m. / Regular None` | `CBOE-SU-2019-INDEPENDENCE` | T1 | Cboe event date 2019-07-04 |
| 2019-09-02 | early close | `Monday, September 2, 2019: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2019-LABOR` | T1 | Cboe event date 2019-09-02 |
| 2019-11-28 | early close | `Thursday, November 28, 2019: Extended 5:00 p.m. (Wednesday) to 10:30 a.m. / Regular None` | `CBOE-SU-2019-THANKSGIVING` | T1 | Cboe event date 2019-11-28; the notice also states normal hours Wednesday 2019-11-27 |
| 2019-11-29 | early close | `Friday, November 29, 2019: Regular 8:30 a.m. to 12:15 p.m.` | `CBOE-SU-2019-THANKSGIVING` | T1 | Cboe prints the Friday regular session 8:30-12:15, so only the close moves |
| 2019-12-24 | early close | `Tuesday, December 24, 2019 … VX, AMW, AMB, VA, and VXTY futures 12:15 p.m.` | `CBOE-SU-2019-CHRISTMAS` | T1 | Cboe event date 2019-12-24; the notice's per-product closing table names 12:15 for the VX row, so only the close moves |
| 2019-12-25 | closed | `CFE will reopen following the Christmas holiday at 5:00 p.m. on Wednesday, December 25, 2019` | `CBOE-SU-2019-CHRISTMAS` | T1 | Cboe event date 2019-12-25; the reopen on the holiday itself means no session belongs to the trade date |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `CFE will reopen following the New Year's Day holiday at 5:00 p.m. on Wednesday, January 1, 2020` and `CFE will have normal trading hours for all products on Tuesday, December 31, 2019.` | `CBOE-SU-2020-NEW-YEAR` | T1 | Cboe event date 2020-01-01; the reopen on the holiday itself means no session belongs to the trade date |
| 2020-01-20 | early close | `Monday, January 20, 2020: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2020-MLK` | T1 | Cboe event date 2020-01-20 |
| 2020-02-17 | early close | `Monday, February 17, 2020: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2020-PRESIDENTS` | T1 | Cboe event date 2020-02-17 |
| 2020-04-10 | closed | `Trading will be closed for all CFE products on Friday, April 10, 2020.` | `CBOE-SU-2020-GOOD-FRIDAY` | T1 | Cboe event date 2020-04-10; the notice states the closure and the Sunday 2020-04-12 reopen |
| 2020-05-25 | early close | `Monday, May 25, 2020: Extended 5:00 p.m. (Sunday) to 10:30 a.m. CT / Regular None` | `CBOE-SU-2020-MEMORIAL` | T1 | Cboe event date 2020-05-25 |
| 2020-07-03 | early close | `Friday, July 3, 2020: Extended 5:00 p.m. (Thursday) to 10:30 a.m. CT / Regular None` — `the Independence Day holiday is being observed on July 3, 2020` | `CBOE-SU-2020-INDEPENDENCE` | T1 | Cboe event date observed Friday 2020-07-03; the notice also states normal hours Thursday 2020-07-02, so the Thursday-evening leg is the observed day's own and stops at 10:30 |
| 2020-09-07 | early close | `Monday, September 7, 2020: Extended 5:00 p.m. (Sunday) to 10:30 a.m. CT / Regular None` | `CBOE-SU-2020-LABOR` | T1 | Cboe event date 2020-09-07 |
| 2020-11-26 | early close | `Thursday, November 26, 2020: Extended 5:00 p.m. (Wednesday) to 10:30 a.m. / Regular None` | `CBOE-SU-2020-THANKSGIVING` | T1 | Cboe event date 2020-11-26; the notice also states normal hours Wednesday 2020-11-25 |
| 2020-11-27 | early close | `Friday, November 27, 2020: Regular 8:30 a.m. to 12:15 p.m.` | `CBOE-SU-2020-THANKSGIVING` | T1 | Cboe prints the Friday regular session 8:30-12:15, so only the close moves |
| 2020-12-24 | early close | `Thursday, December 24, 2020 … VX, VXM, AMERIBOR (AMW, AMB1, AMB3), and VA/VAO futures 12:15 p.m.` | `CBOE-SU-2020-CHRISTMAS` | T1 | Cboe event date 2020-12-24; the notice's per-product closing table names 12:15 for the VX row |
| 2020-12-25 | closed | `Trading will be closed for all CFE products on Friday, December 25, 2020. There will be no extended trading hours on the evening of Thursday, December 24, 2020.` | `CBOE-SU-2020-CHRISTMAS` | T1 | Cboe event date 2020-12-25; the closure deletes the Thursday-evening leg |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Trading will be closed for all CFE products on Friday, January 1, 2021. There will be no extended trading hours on the evening of Thursday, December 31, 2020.` | `CBOE-SU-2021-NEW-YEAR` | T1 | Cboe event date 2021-01-01; the closure deletes the Thursday-evening leg |
| 2021-01-18 | early close | `Monday, January 18, 2021: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2021-MLK` | T1 | Cboe event date 2021-01-18 |
| 2021-02-15 | early close | `Monday, February 15, 2021: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2021-PRESIDENTS` | T1 | Cboe event date 2021-02-15 |
| 2021-04-02 | early close | `Friday, April 2, 2021: Extended 5:00 p.m. (Thursday) to 8:30 a.m. / Regular None` | `CBOE-SU-2021-GOOD-FRIDAY` | T1 | Cboe event date 2021-04-02; the overnight leg ends at the regular open and no regular session runs — the 2026 shape, not the closure shape |
| 2021-05-31 | early close | `Monday, May 31, 2021: Extended 5:00 p.m. (Sunday) to 10:30 a.m. CT / Regular None` | `CBOE-SU-2021-MEMORIAL` | T1 | Cboe event date 2021-05-31 |
| 2021-07-05 | early close | `Monday, July 5, 2021: Extended 5:00 p.m. (Sunday) to 10:30 a.m. CT / Regular None` | `CBOE-SU-2021-INDEPENDENCE` | T1 | Cboe event date observed Monday 2021-07-05; the Sunday-evening leg is the observed day's own and stops at 10:30 (July 4 was a Sunday, so no eve session exists to shorten) |
| 2021-09-06 | early close | `Monday, September 6, 2021: Extended 5:00 p.m. (Sunday) to 10:30 a.m. CT / Regular None` | `CBOE-SU-2021-LABOR` | T1 | Cboe event date 2021-09-06 |
| 2021-11-25 | early close | `Thursday, November 25, 2021: Extended 5:00 p.m. (Wednesday) to 10:30 a.m. / Regular None` | `CBOE-SU-2021-THANKSGIVING` | T1 | Cboe event date 2021-11-25 |
| 2021-11-26 | early close | `Friday, November 26, 2021: Regular 8:30 a.m. to 12:15 p.m.` | `CBOE-SU-2021-THANKSGIVING` | T1 | Cboe prints the Friday regular session 8:30-12:15, so only the close moves |
| 2021-12-24 | closed | `Trading will be closed for all CFE products on Friday, December 24, 2021. There will be no extended trading hours on the evening of Thursday, December 23, 2021.` | `CBOE-SU-2021-CHRISTMAS` | T1 | Cboe event date observed Friday 2021-12-24 (Christmas fell on a Saturday); the notice states the closure of the observed day itself and the deletion of the Thursday-evening leg, and normal hours Monday 2021-12-27 from the Sunday 17:00 reopen |

**2021 witnesses with no row.** Two Cboe-wide notices state CFE days this table ships no row for, and both are listed in `### Documents` below as their witnesses: `C2021061701` states that `Cboe will not be adjusting trading hours for … CFE for Friday, June 18, 2021 or Monday, June 21, 2021` (Juneteenth's first year — the observed set gained it only in 2022), and `C2021121601` states that CFE `will follow normal trading hours` on Friday, December 31, 2021 (New Year's Eve before the Saturday holiday). Inside the window these dates are audited normal by those notices rather than by silence.

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-17 | early close | `Monday, January 17, 2022: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2022-MLK` | T1 | Cboe event date 2022-01-17 |
| 2022-02-21 | early close | `Monday, February 21, 2022: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2022-PRESIDENTS` | T1 | Cboe event date 2022-02-21 |
| 2022-04-15 | closed | `Trading will be closed for all CFE products on Friday, April 15th, 2022.` | `CBOE-SU-2022-GOOD-FRIDAY` | T1 | Cboe event date 2022-04-15; the notice states the closure and the Sunday 2022-04-17 reopen |
| 2022-05-30 | early close | `Monday, May 30, 2022: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2022-MEMORIAL` | T1 | Cboe event date 2022-05-30 |
| 2022-06-20 | early close | `Monday, June 20, 2022: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2022-JUNETEENTH` | T1 | Cboe event date observed Monday 2022-06-20 (Juneteenth fell on a Sunday); the Sunday-evening leg is the observed day's own and stops at 10:30 |
| 2022-07-04 | early close | `SUNDAY JULY 3, 2022 ETH Start 5:00 PM … MONDAY JULY 4, 2022 ETH Close 10:30 AM; MONDAY … RTH` — the VX row reads `5:00 PM / 10:30 AM` | `CBOE-SU-2022-INDEPENDENCE` | T1 | Cboe event date 2022-07-04; the notice's table states `observed on Monday, July 4, 2022`, the Sunday-evening leg is the holiday's own and stops at 10:30, and the Trade Date row names Tuesday, July 5, 2022 for the next group |
| 2022-09-05 | early close | `Monday, September 5, 2022: Extended 5:00 p.m. (Sunday) to 10:30 a.m. / Regular None` | `CBOE-SU-2022-LABOR` | T1 | Cboe event date 2022-09-05; the notice states `observed on Monday, September 5, 2022` |
| 2022-11-24 | early close | `WEDNESDAY NOVEMBER 23, 2022 ETH Start 5:00 PM, ETH Close 10:30 AM` | `CBOE-SU-2022-THANKSGIVING` | T1 | Cboe event date 2022-11-24; the Wednesday-evening leg is trade date 2022-11-24's own and stops at 10:30 |
| 2022-11-25 | early close | `THURSDAY NOVEMBER 24, 2022 … RTH Start 8:30 AM, RTH Close 12:15 PM` — `Trade Date Friday, November 25, 2022` | `CBOE-SU-2022-THANKSGIVING` | T1 | Cboe trade date named verbatim; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2022-12-26 | closed | `FRIDAY DECEMBER 23, 2022 RTH Close 3:00 PM, ETH Close 4:00 PM; SUNDAY DECEMBER 25, 2022 … MONDAY DECEMBER 26, 2022 … all empty` | `CBOE-SU-2022-CHRISTMAS` | T1 | Cboe event date observed Monday 2022-12-26 (Christmas fell on a Sunday); the notice's table prints the normal Friday 12-23 closes beside it (so 2022-12-23 is audited normal by its own notice) and no Sunday-evening or Monday session |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `FRIDAY DECEMBER 30, 2022 RTH Close 3:00 PM, ETH Close 4:00 PM; MONDAY JANUARY 2, 2023 … all empty` | `CBOE-SU-2023-NEW-YEAR` | T1 | Cboe event date observed Monday 2023-01-02 (New Year's Day fell on a Sunday); the notice's table prints the normal Friday 12-30 closes beside it (so 2022-12-30 is audited normal by its own notice) and no Sunday-evening or Monday session |
| 2023-01-16 | early close | `MONDAY JANUARY 16, 2023 … ETH Close 10:30 AM` — `Trade Date Tuesday, January 17, 2023` | `CBOE-SU-2023-MLK` | T1 | Cboe event date 2023-01-16; the Sunday-evening leg is the holiday's own and stops at 10:30 |
| 2023-02-20 | early close | `Monday, February 20, 2023 … 10:30 AM` | `CBOE-SU-2023-PRESIDENTS` | T1 | Cboe event date 2023-02-20 |
| 2023-04-07 | early close | `Thursday, April 6, 2023: ETH Close 8:30 AM` | `CBOE-SU-2023-GOOD-FRIDAY` | T1 | Cboe event date 2023-04-07; the overnight leg ends at the regular open and no regular session runs |
| 2023-05-29 | early close | `Monday, May 29, 2023 … 10:30 AM` | `CBOE-SU-2023-MEMORIAL` | T1 | Cboe event date 2023-05-29 |
| 2023-06-19 | early close | `Monday, June 19, 2023 … 10:30 AM` | `CBOE-SU-2023-JUNETEENTH` | T1 | Cboe event date 2023-06-19 |
| 2023-07-03 | early close | `TUESDAY JULY 2 … MONDAY JULY 3, 2023 … RTH Close 12:15 PM` — `Trade Date Monday, July 3, 2023` | `CBOE-SU-2023-INDEPENDENCE` | T1 | Cboe trade date named verbatim; the notice's table closes the Monday RTH at 12:15, the `typically 12:15` July 3 eve before the Tuesday holiday |
| 2023-07-04 | early close | `MONDAY JULY 3, 2023 ETH Start 5:00 PM, ETH Close 10:30 AM` | `CBOE-SU-2023-INDEPENDENCE` | T1 | Cboe event date 2023-07-04; the Monday-evening leg is the holiday's own and stops at 10:30 |
| 2023-09-04 | early close | `Monday, September 4, 2023 … 10:30 AM` | `CBOE-SU-2023-LABOR` | T1 | Cboe event date 2023-09-04 |
| 2023-11-23 | early close | `WEDNESDAY NOVEMBER 22, 2023 ETH Start 5:00 PM, ETH Close 10:30 AM` | `CBOE-SU-2023-THANKSGIVING` | T1 | Cboe event date 2023-11-23; the Wednesday-evening leg is the holiday's own |
| 2023-11-24 | early close | `THURSDAY NOVEMBER 23, 2023 … RTH Close 12:15 PM` — `Trade Date Friday, November 24, 2023` | `CBOE-SU-2023-THANKSGIVING` | T1 | Cboe trade date named verbatim; the first open is the normal Thursday 17:00 CT |
| 2023-12-25 | closed | `SUNDAY DECEMBER 24, 2023 … MONDAY DECEMBER 25, 2023 … all empty` — `Trade Date Tuesday, December 26, 2023` | `CBOE-SU-2023-CHRISTMAS` | T1 | Cboe event date 2023-12-25; the notice's table prints no Sunday-evening or Monday session, so the trade date is deleted with the Sunday-evening leg (December 24 was a Sunday, so the `typically 12:15` Christmas-Eve default has no session to shorten) |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `SUNDAY DECEMBER 31, 2023 … MONDAY JANUARY 1, 2024 … all empty` | `CBOE-SU-2024-NEW-YEAR` | T1 | Cboe event date 2024-01-01; the notice's table prints no Sunday-evening or Monday session, so the trade date is deleted with the Sunday-evening leg |
| 2024-01-15 | early close | `MONDAY JANUARY 15, 2024 … ETH Close 10:30 AM` | `CBOE-SU-2024-MLK` | T1 | Cboe event date 2024-01-15 |
| 2024-02-19 | early close | `Monday, February 19, 2024 … 10:30 AM` | `CBOE-SU-2024-PRESIDENTS` | T1 | Cboe event date 2024-02-19 |
| 2024-03-29 | closed | `...will be observed on Friday, March 29, 2024` — the `FRIDAY MARCH 29, 2024` column prints no RTH Start or RTH Close, and `Trade Date Thursday, March 28, 2024 … Monday, April 1, 2024` | `CBOE-SU-2024-GOOD-FRIDAY` | T1 | Cboe event date 2024-03-29; the notice's table prints no Friday session and its Trade Date row skips the day, so no session belongs to it |
| 2024-05-27 | early close | `Monday, May 27, 2024 … 10:30 AM` | `CBOE-SU-2024-MEMORIAL` | T1 | Cboe event date 2024-05-27 |
| 2024-06-19 | early close | `Wednesday, June 19, 2024 … 10:30 AM` | `CBOE-SU-2024-JUNETEENTH` | T1 | Cboe event date 2024-06-19 |
| 2024-07-03 | early close | `TUESDAY JULY 2, 2024 ETH Start 5:00 PM, RTH Start 8:30 AM, RTH Close 12:15 PM` — `Trade Date Wednesday, July 3, 2024` | `CBOE-SU-2024-INDEPENDENCE` | T1 | Cboe trade date named verbatim; the notice's table closes the Wednesday RTH at 12:15, the July 3 eve before the Thursday holiday |
| 2024-07-04 | early close | `WEDNESDAY JULY 3, 2024 ETH Start 5:00 PM, ETH Close 10:30 AM` | `CBOE-SU-2024-INDEPENDENCE` | T1 | Cboe event date 2024-07-04; the Wednesday-evening leg is the holiday's own and stops at 10:30 |
| 2024-09-02 | early close | `Monday, September 2, 2024 … 10:30 AM` | `CBOE-SU-2024-LABOR` | T1 | Cboe event date 2024-09-02 |
| 2024-11-28 | early close | `WEDNESDAY NOVEMBER 27, 2024 ETH Start 5:00 PM, ETH Close 10:30 AM` | `CBOE-SU-2024-THANKSGIVING` | T1 | Cboe event date 2024-11-28; the Wednesday-evening leg is the holiday's own |
| 2024-11-29 | early close | `THURSDAY NOVEMBER 28, 2024 … RTH Close 12:15 PM` — `Trade Date Friday, November 29, 2024` | `CBOE-SU-2024-THANKSGIVING` | T1 | Cboe trade date named verbatim; the first open is the normal Thursday 17:00 CT |
| 2024-12-24 | early close | `MONDAY DECEMBER 23, 2024 ETH Start 5:00 PM, RTH Start 8:30 AM, RTH Close 12:15 PM` — `Trade Date Tuesday, December 24, 2024` | `CBOE-SU-2024-CHRISTMAS` | T1 | Cboe trade date named verbatim; the notice's table closes the Tuesday RTH at 12:15, the Christmas Eve half day, and prints no Tuesday-evening leg because trade date 2024-12-25 is closed |
| 2024-12-25 | closed | `TUESDAY DECEMBER 24, 2024 ETH Start, ETH Close empty; WEDNESDAY DECEMBER 25, 2024 … empty` — `Trade Date Thursday, December 26, 2024` | `CBOE-SU-2024-CHRISTMAS` | T1 | Cboe event date 2024-12-25; the notice's table prints no Tuesday-evening or Wednesday session |

### 2025

The 2025 block is the eleven CFE notices the operator's own `schedule-update/2025` listing names, plus the four notices governing 2025's first two holidays that the same listing files under 2024 (Cboe publishes a New Year and an MLK notice in the December before the holiday year) and the National Day of Mourning notice. Every row below was read from the notice's own PDF bytes with `pdftotext -layout` **and** `-bbox` word coordinates, because the 2025 corpus has no consolidated table: unlike 2026 there is no single CSV or page that states all of these dates, and one notice in the set has a layout the store's parser could not read at all.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `TUESDAY DECEMBER 31, 2024` `RTH Close 3:00 PM` / `ETH Close 4:00 PM`; `WEDNESDAY JANUARY 1, 2025` both cells empty; `THURSDAY JANUARY 2, 2025` `ETH Start 5:00 PM` — `Trade Date Tuesday, December 31, 2024 … Thursday, January 2, 2025` | `CBOE-SU-2025-NEW-YEAR` | T1 | Cboe event date 2025-01-01; no cell is printed under the holiday column and the Trade Date row skips it, so no session belongs to it |
| 2025-01-09 | replacement blocks | `5:00 p.m. (Wednesday, January 8, 2025) to 8:30 a.m. (Extended Trading Hours) for VX, VXM, VXT, and VXMT` / `No Regular Trading Hours for VX, VXM, VXT, and VXMT` | `CBOE-SU-2025-MOURNING` | T1 | Cboe trade date named verbatim — “for trade date Thursday, January 9, 2025”; the only session is one extended block opening 17:00 CT on 2025-01-08, so the row states `extended(-1, 17:00, 08:30)` and nothing else |
| 2025-01-20 | early close | `SUNDAY JANUARY 19, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY JANUARY 20, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-MLK` | T1 | Cboe event date 2025-01-20; the Sunday-evening leg is trade date 2025-01-20's own and stops at 10:30, and the Monday-evening leg is trade date 2025-01-21's |
| 2025-02-17 | early close | `SUNDAY FEBRUARY 16, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY FEBRUARY 17, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-PRESIDENTS` | T1 | Cboe event date 2025-02-17; same two-leg shape as MLK Day |
| 2025-04-18 | closed | `THURSDAY APRIL 17, 2025` `RTH Close 3:00 PM`, `ETH Close 4:00 PM`; `FRIDAY APRIL 18, 2025` **no cell at all**; `SUNDAY APRIL 20, 2025` **no cell at all**; `MONDAY APRIL 21, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM` — `Trade Date Thursday, April 17, 2025 … Monday, April 21, 2025` | `CBOE-SU-2025-GOOD-FRIDAY` | T1 | Cboe event date 2025-04-18; see the interpretive step below — the Friday and Sunday trade dates are both absent from the notice's Trade Date row |
| 2025-05-26 | early close | `SUNDAY MAY 25, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY MAY 26, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-MEMORIAL` | T1 | Cboe event date 2025-05-26 |
| 2025-06-19 | early close | `WEDNESDAY JUNE 18, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `THURSDAY JUNE 19, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-JUNETEENTH` | T1 | Cboe event date 2025-06-19; a Thursday, so the Wednesday-evening leg is its own |
| 2025-07-03 | early close | `WEDNESDAY JULY 2, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `THURSDAY JULY 3, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM` — 10:30 CT | `CBOE-SU-2025-INDEPENDENCE` | T1 | Cboe event date 2025-07-04 observed Friday, so the shortened day lands on the Thursday trade date that opened 2025-07-02 at 17:00 CT |
| 2025-07-04 | closed | `FRIDAY JULY 4, 2025` **no cell at all**; `SUNDAY JULY 6, 2025` `ETH Start 5:00 PM`; `MONDAY JULY 7, 2025` `RTH Start 8:30 AM` — `Trade Date Thursday, July 3, 2025 … Monday, July 7, 2025` | `CBOE-SU-2025-INDEPENDENCE` | T1 | Cboe event date 2025-07-04; the holiday column prints no session and the Trade Date row skips it |
| 2025-09-01 | early close | `SUNDAY AUGUST 31, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY SEPTEMBER 1, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-LABOR` | T1 | Cboe event date 2025-09-01 |
| 2025-11-27 | early close | `WEDNESDAY NOVEMBER 26, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `THURSDAY NOVEMBER 27, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 12:15 PM` — 10:30 CT | `CBOE-SU-2025-THANKSGIVING` | T1 | Cboe event date 2025-11-27; the Wednesday-evening leg is trade date 2025-11-27's own and stops at 10:30, and the Thursday-evening leg belongs to trade date 2025-11-28 |
| 2025-11-28 | early close | `FRIDAY NOVEMBER 28, 2025` `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `Trade Date … Friday, November 28, 2025` — 12:15 CT | `CBOE-SU-2025-THANKSGIVING` | T1 | Cboe trade date named verbatim; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2025-12-24 | early close | `WEDNESDAY DECEMBER 24, 2025` `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `Trade Date Wednesday, December 24, 2025 … Friday, December 26, 2025` — 12:15 CT | `CBOE-SU-2025-CHRISTMAS` | T1 | Cboe trade date named verbatim; the Wednesday column carries no ETH close, because the Thursday-evening leg would belong to the closed trade date 2025-12-25 |
| 2025-12-25 | closed | `THURSDAY DECEMBER 25, 2025` `ETH Start`, `ETH Close`, `RTH Start`, `RTH Close` all empty; `FRIDAY DECEMBER 26, 2025` `ETH Start 5:00 PM` | `CBOE-SU-2025-CHRISTMAS` | T1 | Cboe event date 2025-12-25; every cell under the holiday column is empty and the Trade Date row skips it |

**Supersession, 2025-01-09.** Two mourning artifacts are on disk. The base notice `2025__Cboe-to-Observe-National-Day-of-Mourning-on-Thursday-January-9-2025.pdf` (sha256 `21b2158711940f8291ca1478a463edd02123bb5e4530d679d6c59a155525558e`) is **superseded** by the `Update-` file the row cites (`ecabcf3d…`), which is why the id resolves to the update: the update is the artifact whose CFE TRADING SCHEDULE table states the session. Both are listed in the table below so the supersession is auditable, and both were retrieved in the same pass on 2026-09-26, so the update is not a later correction of a value this file once recorded differently.

**Two 2025 differences from the 2026 shapes.**

1. **Good Friday 2025-04-18 is a closure, where 2026-04-03 is an early close at the regular open.** Cboe's 2025 notice prints the `FRIDAY APRIL 18, 2025` column header but no cell beneath it: the Thursday column carries `RTH Close 3:00 PM` and `ETH Close 4:00 PM`, and the next printed times are Monday's `ETH Start 5:00 PM` and `RTH Start 8:30 AM`, with the `SUNDAY APRIL 20, 2025` column likewise empty. Its Trade Date row reads `Thursday, April 17, 2025 … Monday, April 21, 2025` — **2025-04-18 is not named as a trade date at all**, although the same notice's other rows name their event date explicitly. The 2026 notice of the same holiday states the opposite shape in its own bytes: `FRIDAY APRIL 3, 2026` carries `ETH Close 8:30 AM`, and the row is `None` for RTH, which is the 08:30 early close the crate ships for that date. The 2025 bytes therefore state no close for 2025-04-18, and the crate derives the 2025 row from the 2025 bytes rather than copying the 2026 shape. No 2025 notice names a trade date for that Friday, and no ETH Close appears under it at any x-position in the PDF.
2. **2025-01-09 is a trading day with no regular session.** The mourning notice states the session in prose rather than in the two-column holiday table, so the scalar vocabulary cannot state it: `Closed` would delete a session the operator kept open, and an `EarlyClose` at 08:30 would leave the ordinary 08:30-15:00 CT regular session standing. `ReplacementBlocks` states the day exactly — one `extended(-1, 17:00, 08:30)` block and nothing else — so the 15:00-16:00 CT extended window is absent, the regular session is absent, and trading resumes with the ordinary 2025-01-10 trade date, which the notice states: `will resume a normal trading schedule for trade date Friday, January 10, 2025`.

**Settlement is not a session boundary here (LAW-SESSION-NOT-EXPIRY).** The mourning notice adds: `The Thursday, January 9, 2025, business day will end at 8:30 a.m. CT on January 9, 2025, and the daily settlement prices of VX, VXM, IBHY, IBIG, and IEMD futures will be determined at that time.` That instant coincides with the block's close and is **not** modelled as a boundary of its own; the close is sourced in session language by `to 8:30 a.m. (Extended Trading Hours)`, and the settlement sentence is quoted here only so a reader can check that the coincidence was noticed rather than overlooked. The notice's `TAS transaction prices in VXT and VXMT … will be based on the daily settlement prices of VX and VXM futures determined at 8:30 a.m.` is the same instant read for a different product and likewise adds nothing.

**Out-of-scope sibling rows on 2025-01-09.** The mourning notice's CFE TRADING SCHEDULE table also lists `UX Options on Futures`, `IBYO and IBGO Options on Futures` and `VA Futures`, each reading `None`. Those three are options and variance contracts on the same exchange, not VIX futures: the `cfe` venue row's documented scope is VIX futures (see **Scope** under Gaps and residual risks), and `cfe_vix` is the key the consumer routes VX to. They are recorded here so the table's shape is complete, and no row ships for them.

### Documents

The rules-page captures that key the 2017 rows and 2018-01-01 were retrieved as Wayback `id_` replays on 2026-09-29 and are saved under `holidays/raw/cfe-2010-2024/rulespage/` with their digests in that directory's `INDEX.md`. The 2018-2024 notices below are the operator's own CDN PDFs saved under `holidays/raw/cfe-2010-2025/live/` in the 2026-09-21 UTC pass, listed with their URLs and sha256s in that store's `live_index.json`; the pass was recorded at day granularity, so their retrieval cells state the day only. `C2021061701` and `C2021121601` are the two 2021 witnesses the year's prose cites — no row of this module keys them, and they are listed so the citation lands on bytes. Every 2025 artifact below was retrieved from the operator's own CDN in one pass on the date recorded, saved under `holidays/raw/cfe-2010-2025/live/` in the research store, and listed with its URL and sha256 in that store's `live_index.json`. Ids follow `CBOE-SU-<holiday year>-<HOLIDAY>`; `CBOE-SU-2025-MOURNING-BASE` is the superseded twin and is no row's document. The 2026 id is repeated here because one `### Documents` table resolves every id this module cites, and its own evidence is the 2026 section below.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `cfe-holiday-calendar @2017-04-10T21:04:29Z` | 2017-04-10 .. 2026-12-31 | <https://web.archive.org/web/20170410210429id_/http://cfe.cboe.com/about-cfe/holiday-calendar> | Wayback `id_` replay of capture `20170410210429`, retrieved 2026-09-29 UTC | T1 | `e21e574eb0a960337a855c323d314304cf58dc45b5afc135dbb813e32e2212d1` |
| `cfe-holiday-calendar @2017-06-26T00:36:49Z` | 2017-04-10 .. 2026-12-31 | <https://web.archive.org/web/20170626003649id_/http://cfe.cboe.com/about-cfe/holiday-calendar> | Wayback `id_` replay of capture `20170626003649`, retrieved 2026-09-29 UTC | T1 | `041bfa10177880270b2057d50c4bf7674937268638437e5d25929cb64c62d327` |
| `cfe-holiday-calendar @2017-11-13T01:40:35Z` | 2017-04-10 .. 2026-12-31 | <https://web.archive.org/web/20171113014035id_/http://cfe.cboe.com/about-cfe/holiday-calendar> | Wayback `id_` replay of capture `20171113014035`, retrieved 2026-09-29 UTC | T1 | `c3a6a1a715002766918063a0fe1870a41e6799a0d6e43d8e5e1ff2a25c093f05` |
| `cfe-holiday-calendar @2017-12-29T07:06:24Z` | 2017-04-10 .. 2026-12-31 | <https://web.archive.org/web/20171229070624id_/http://cfe.cboe.com/about-cfe/holiday-calendar> | Wayback `id_` replay of capture `20171229070624`, retrieved 2026-09-29 UTC | T1 | `4e803f6358871afc1f71d5c0f481e812255761612600b958ecaa8bbf29f3d5f8` |
| `cfe-holiday-calendar @2019-12-15T13:30:22Z` | 2017-04-10 .. 2026-12-31 | <https://web.archive.org/web/20191215133022id_/https://cfe.cboe.com/about-cfe/holiday-calendar> | Wayback `id_` replay of capture `20191215133022`, retrieved 2026-09-29 UTC | T1 | `938c104860a164f051aa3b0429c39273ddea1525bea205dfdd7fd16a9ce8f6cf` |
| `CBOE-SU-2018-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/CFE-Modified-Trading-Hours-for-MLK-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `055f9e5022f6e414e043566242a5414b366e82f4fa20a43465d7eedd88c0db66` |
| `CBOE-SU-2018-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/CFE-Modified-Trading-Hours-for-Presidents-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `1540b8b58aa5e0b797d9d8454ec24b41257a33a02778554be3d744967295f38f` |
| `CBOE-SU-2018-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/CFE-Modified-Trading-Hours-for-Good-Friday-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `1dea4732b0eed6066048196042f8b069006dc59a472212ce1c2c7d99df3f0314` |
| `CBOE-SU-2018-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/CFE-Modified-Trading-Hours-for-Memorial-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `a50b52e068ccb6e872d330bd2b8cd7a9f537bd0dd2c80f933dc9453d4d34c162` |
| `CBOE-SU-2018-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/Modified-Trading-Hours-for-Independence-Day-Notice.pdf> | retrieved 2026-09-21 UTC | T1 | `604386f89c66896f15d1ddca86d5fe96606b17068aef4962c1a9cc76a0114c53` |
| `CBOE-SU-2018-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/Modified-Trading-Hours-for-Labor-Day-Notice-2018.pdf> | retrieved 2026-09-21 UTC | T1 | `94f7be3fd5edf08866cd20fe7c421cf0b9e0092b12abfc8489b467b269565b1f` |
| `CBOE-SU-2018-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/Modified-Trading-Hours-for-Thanksgiving-Notice-2018.pdf> | retrieved 2026-09-21 UTC | T1 | `7714eb290cef2ce233e693af3e28b9966ec7915f4515c868e20a712c39d127cf` |
| `CBOE-SU-2018-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2018/Modified-Trading-Hours-for-Christmas-Holiday-Notice-2018.pdf> | retrieved 2026-09-21 UTC | T1 | `48fa42c6ad39efe973d848ff3a9857d6cc6e1d3692c00bd03ae080472fb4bb77` |
| `CBOE-SU-2019-NEW-YEAR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/Modified-Trading-Hours-for-New-Year-s-Day-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `3dbbd9cf9fddd87a0c519e322b97d630edb709d1c675c5a618192c3243138023` |
| `CBOE-SU-2019-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-MLK-Day-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `3d9505a904d635fd9636bc348317186448ca3827a4013fb95b11351397731a67` |
| `CBOE-SU-2019-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-Presidents-Day-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `f6ca8d61087f6e0efe328eb31120b3f8a83ffda9258ab11c15d1531994e48afc` |
| `CBOE-SU-2019-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-Good-Friday-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `7d2ec39d8767b016e8f37dc806e9c8e6aa2d030c6fe84546259e96db8b604055` |
| `CBOE-SU-2019-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-Memorial-Day-Holiday-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `4a97f7d51b11717c1f7a0fd34d995ca12232fffe282a01ceae714eb2aaacd5f2` |
| `CBOE-SU-2019-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-Independence-Day-Notice.pdf> | retrieved 2026-09-21 UTC | T1 | `200e99c1ad23364bebac71d03fba5e574760757809231761eedab39019db8c62` |
| `CBOE-SU-2019-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-Labor-Day-Notice-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `4186d6c566674f3c957ae779219ae92d50a4a7a9d1f72cc17e6969138579e303` |
| `CBOE-SU-2019-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/CFE-Modified-Trading-Hours-for-Thanksgiving-Holiday-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `0312f6089de24dbee46fd0b7f4c4106358a3bf5cdcc772e8a2dc371391093baf` |
| `CBOE-SU-2019-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2019/Modified-Trading-Hours-for-Christmas-Holiday-2019.pdf> | retrieved 2026-09-21 UTC | T1 | `79d98922725d6e53654a4fa4525f6313707e98c8b3921c5db140b6c0c39cb935` |
| `CBOE-SU-2020-NEW-YEAR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-the-New-Year-s-Day-Holiday-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `9503ca3ce80f1b9b034f0ce275a9ba1d00925569fa33fc0e52dff9b9debb9025` |
| `CBOE-SU-2020-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-MLK-Day-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `a28d513c8c6a09a20cd284314cf6efc15b7c204d467d146c83877e8856e17c15` |
| `CBOE-SU-2020-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-Presidents-Day-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `99c0c8e0b7397d066f6f4cdd611bd4758bf59570c6a520d83cc8a641d7565c79` |
| `CBOE-SU-2020-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-the-Good-Friday-Holiday-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `c17f02cf8148422932392683af9f8235fb9a3fdec88e0f701c9aa6f96aefa6b3` |
| `CBOE-SU-2020-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-Memorial-Day-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `bcf9076064f1730d9ff4fff73bb450f1c36f0c5a9ddc43c77a121c697f42d43a` |
| `CBOE-SU-2020-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-Independence-Day-Holiday-2020-Notice.pdf> | retrieved 2026-09-21 UTC | T1 | `af109b7d78d93d91f850e20448ec56fa9b378276cfd4a5f1afc6c79d8f346ce4` |
| `CBOE-SU-2020-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-Labor-Day-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `d98e94ca9683d91ca475bc7dd8c4605bfb616ba339ea84ec42d68578ba00990a` |
| `CBOE-SU-2020-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/CFE-Modified-Trading-Hours-for-Thanksgiving-Holiday-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `5a41b7dae6c7a0ed4f25a471895de76b6c6b2a92abddc67a36c53218d772e3a0` |
| `CBOE-SU-2020-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/Modified-Trading-Hours-for-Christmas-Holiday-Notice-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `4ea972879139c57054952415308f34d3f073df340cc5db73462f97fa9b256c9d` |
| `CBOE-SU-2021-NEW-YEAR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2020/Modified-Trading-Hours-for-New-Years-Holiday-Notice-2020.pdf> | retrieved 2026-09-21 UTC | T1 | `66a1278c03476f026a9842848e3d6dc05c30052c771bacaf99f08e0f981a3d9d` |
| `CBOE-SU-2021-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-MLK-Day-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `7eb12e247901ca07aa1f9f5f921077e2e75bc8c3c5b97fb544b8d0d4fbe1b5ee` |
| `CBOE-SU-2021-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-Presidents-Day-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `f7aea2d465033acfb0444277c7a94c6e169f304592fc2a83cd822c7aadb67085` |
| `CBOE-SU-2021-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-Good-Friday-2021-Notice.pdf> | retrieved 2026-09-21 UTC | T1 | `861a2b1738553bf162f2c21d610773f49b02f3cd5ccfda31deb0789ae5c35d0c` |
| `CBOE-SU-2021-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-Memorial-Day-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `e2fdeabd93a35bcf30eb0cb06f4dc349d0336b778ef288dbcb8daa2552c10f8f` |
| `CBOE-SU-2021-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-the-Observed-Independence-Day-Holiday-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `1896e5ba1c36797d5a90fa255395abade56918e34f58b0af70de84b9eb9d1f47` |
| `CBOE-SU-2021-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-Labor-Day-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `ff62a5b0c514750b57b4be83e2f58cb20d0be3387b2c50b74d32c372a8b6b551` |
| `CBOE-SU-2021-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-Thanksgiving-Holiday-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `201946236d219ceebb3f05e57d4ce313afe1802b5f704c7d2100953da5f9a284` |
| `CBOE-SU-2021-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/CFE-Modified-Trading-Hours-for-Christmas-Holiday-Notice-2021.pdf> | retrieved 2026-09-21 UTC | T1 | `a7b9f1d0d6d4c3e55122d58a86b35a4f31adee4dd2882b1c175e292c566ac200` |
| `C2021061701` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/Cboe-Juneteenth-Trading-Schedule-Update.pdf> | retrieved 2026-09-21 UTC | T1 | `1c9b3748c17af90ccb3ce22663ab0b73f342373d864110cfbb2cb04e449f06eb` |
| `C2021121601` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2021/Cboe-Schedule-for-New-Year-s-Holiday-2021-Notice.pdf> | retrieved 2026-09-21 UTC | T1 | `1cd5dd167a8f380aeddf2eb82244da5ed38538775b3ab9802604c952d5943b3f` |
| `CBOE-SU-2022-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-MLK-Day-2022.pdf> | retrieved 2026-09-21 UTC | T1 | `c4288a49f50df619d8ef3ade2862da7dd16cd47128034daaec1d28762110ef1a` |
| `CBOE-SU-2022-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-Presidents-Day-2022.pdf> | retrieved 2026-09-21 UTC | T1 | `1eff49b2dba66ea1643aef269ed5a6691648861e43cd55a74f5d1bc7ff5545ef` |
| `CBOE-SU-2022-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-Good-Friday-Holiday-2022.pdf> | retrieved 2026-09-21 UTC | T1 | `56624e06580eb1468701d97d04c1ee16ea2857931d5468f4cb1143a1af26587d` |
| `CBOE-SU-2022-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-Memorial-Day-2022.pdf> | retrieved 2026-09-21 UTC | T1 | `49106ac852d79655666a3d826dea791298dfc05554a70eb2677ff2ead39707d2` |
| `CBOE-SU-2022-JUNETEENTH` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-Juneteenth-National-Independence-Day-Holiday-2022.pdf> | retrieved 2026-09-21 UTC | T1 | `05b98dc4453b814601b6cfd0ee798333052044d13d13d98591ed8183906ed1a6` |
| `CBOE-SU-2022-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-Independence-Day-Holiday-2022.pdf> | retrieved 2026-09-21 UTC | T1 | `33b7a640c3f3b4e6bf147279fc45c75097608bea798e64c916fcc3697714390a` |
| `CBOE-SU-2022-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-Labor-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `721c1acfdeef8a648118c290acbbfc9f346cd72f1c66f00fff12547ee97c0622` |
| `CBOE-SU-2022-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-Thanksgiving-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `3813966981c563c6adcbf32b1443a0f27bc6860043d939b9244fff0b2b976635` |
| `CBOE-SU-2022-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-Christmas-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `a70c5a5a0ea0409f8688ccea5ee57ab01d52aacc1c0c6097390caff04afca36f` |
| `CBOE-SU-2023-NEW-YEAR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-New-Year-s-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `18dd40807c985db9e0c36086500dcada58eeea61dd15b1063e281bf88d752b74` |
| `CBOE-SU-2023-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2022/CFE-Modified-Trading-Hours-for-the-Martin-Luther-King-Jr-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `2beeabcdfc5f3754f972f67f075833fa2bbfba42596a449d693ee3d3eac84c2f` |
| `CBOE-SU-2023-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Presidents-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `28ff62b136b04344421b82828c44de864c37c8790c6de0c308db1162a58204c1` |
| `CBOE-SU-2023-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Good-Friday-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `693ade16f6f48c8fecc9095f68ad2fc33c849701bb610c061fde6640ef771de1` |
| `CBOE-SU-2023-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Memorial-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `00bade7b37d4a2a5c382f51ac3342eef3f84e516cbfb3acc29241abbd32ffeca` |
| `CBOE-SU-2023-JUNETEENTH` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Juneteenth-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `b132258718106d8a3407ca0a25a34fd7c7cce285eda63ba17a855e99d1f2a892` |
| `CBOE-SU-2023-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Independence-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `acb880c5cc795b1f1daa452e1f4d7ebe97daaf643918108a78c306e4b20ebd01` |
| `CBOE-SU-2023-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Labor-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `5c857aa34ff8fd5ab539d0161473a91c25c5465732d8b27619beb49cd13f83b7` |
| `CBOE-SU-2023-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Thanksgiving-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `2356df4ab7b7e710ebe2a956ee0205c20ddc7138b4cd21a6fe1729f8d380c941` |
| `CBOE-SU-2023-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Christmas-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `79f9c15cdb292c32843d463f4611cecdd071438a5a7eae8b798b21025f24d2f4` |
| `CBOE-SU-2024-NEW-YEAR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-New-Year-s-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `65a86697e6f66b30e9fa8341a1690dbb84edfb7bfd5bc4a9030c0081d563eea6` |
| `CBOE-SU-2024-MLK` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2023/CFE-Modified-Trading-Hours-for-the-Martin-Luther-King-Jr-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `7b84f59ad00c7e5d11144892a45d0e835d436f47f544bc7e89ded7de477c9436` |
| `CBOE-SU-2024-PRESIDENTS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Presidents-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `fa4844497e12de050494756af49151b5eace8d85f48fa63a8acf8a8d9308f07c` |
| `CBOE-SU-2024-GOOD-FRIDAY` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Good-Friday-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `1f7eafe6e47764d96736c17c02b8551e9a4328d030bbc632f0b6681d09e47429` |
| `CBOE-SU-2024-MEMORIAL` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Memorial-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `cece86eafc260113d0c916ac154cb316a4b4686c7f89ab4114a2381a5d55b3c4` |
| `CBOE-SU-2024-JUNETEENTH` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Juneteenth-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `29342c8d1fc2da05a99594c560e62eb334ad21098b214259d5088861c89b5d13` |
| `CBOE-SU-2024-INDEPENDENCE` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Independence-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `bd8cd57d22242d2478c025990230e390c3869c4f8625bdba542e0fbb41c05a3e` |
| `CBOE-SU-2024-LABOR` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Labor-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `05be90a5d64a400b4c601c30ed7dabe89dac4e81c3e6d6e2ccba1566bb1b14ea` |
| `CBOE-SU-2024-THANKSGIVING` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Thanksgiving-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `ed9ea4f219e06577b718fad8abdde4f05ff92094d189d45d7c35949554493e1e` |
| `CBOE-SU-2024-CHRISTMAS` | 2017-04-10 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Christmas-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `77da9491c1c9ae80c4558d7b21805c81bb12a4239228fc17be2186e6d53e6f4b` |
| `CBOE-SU-2025-NEW-YEAR` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-New-Year-s-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `9bbaa3bb3f8484356a756bd3bd04677450d6215ff5e96c0e514a63a4aac3e7d7` |
| `CBOE-SU-2025-MLK` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Martin-Luther-King-Jr-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `6f2a45b2948628dc2e0867c94790d23cae9bce1728b5953ce81d6e0c5ed1732c` |
| `CBOE-SU-2025-MOURNING` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/Update-Cboe-to-Observe-National-Day-of-Mourning-on-Thursday-January-9-2025.pdf> | retrieved 2026-09-26 UTC | T1 | `ecabcf3de38b3eaa3a9c7b8a384082a66405401299fba8bc733e44061bbfc6ba` |
| `CBOE-SU-2025-MOURNING-BASE` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/Cboe-to-Observe-National-Day-of-Mourning-on-Thursday-January-9-2025.pdf> | retrieved 2026-09-26 UTC | T1 | `21b2158711940f8291ca1478a463edd02123bb5e4530d679d6c59a155525558e` |
| `CBOE-SU-2025-PRESIDENTS` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Presidents-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `258c88e938471039fdd0f66f92efa11b4c22e65ce46fb848da6e16f068766ad2` |
| `CBOE-SU-2025-GOOD-FRIDAY` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Good-Friday-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `cf1cb9146747b3db9af7add58b0706a19da49ffe32a7352bba3cd811a780f7d1` |
| `CBOE-SU-2025-MEMORIAL` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Memorial-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `bef7ff2f0dc5c9da962ffd6a38e6026e2229df4b513f65cc9c743f285ede7970` |
| `CBOE-SU-2025-JUNETEENTH` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Juneteenth-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `dd85b8b307ccfc441d857f121ebaa860b3779a77511feb77ef72ef2e5a9a3ad9` |
| `CBOE-SU-2025-INDEPENDENCE` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Independence-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `6ffd33e4ca688d57a0eab247689fff3a34464a8374d5a2ae08f1d56221586c7a` |
| `CBOE-SU-2025-LABOR` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Labor-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `d5ca2ecc6f7ee6ac5d68503b667192d51ff28aedac91770153e1a6b47c56f2c7` |
| `CBOE-SU-2025-THANKSGIVING` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Thanksgiving-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `585ae94eb540d1ca810a8684ecd63d29e9e7969ea0ff37a74c9cc82e88acea55` |
| `CBOE-SU-2025-CHRISTMAS` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Christmas-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `f2fef1107ab5e774aec3c8c3fb0cda5354b16d3608a9e3b1752ff3dd34fbc3f5` |
| `CBOE-HOURS-USFUT-2026` | 2025-01-01 .. 2026-12-31 | <https://www.cboe.com/about/hours/us-futures> | retrieved 2026-09-12 04:55 UTC | T1 | `b1b38ba66a870c06b9169eed58329407cc0ea23903a44d96e5284f7bf97e7759` |

**Retrieval times are date-level for the twelve 2025 id rows.** The store records the 2025 pass at day granularity only: `live_index.json` carries no capture timestamp per artifact, and the filesystem timestamps that remain are the retrieval host's local clock, not UTC. The date each was fetched is on record; the wall-clock time is not, so none is written here rather than a time being inferred from a file mtime (LAW-UTC-DATES, LAW-NO-FABRICATED-DATES). `CBOE-HOURS-USFUT-2026`'s time is the one the store already recorded.

**Why the 2025 window needs no `Unsourced` row.** The eleven notices cover the eleven dates above, and every other 2025 trade date is a date no Cboe artifact modifies — the operator's own publication pattern is one notice per holiday it observes, so a date with no notice is audited normal rather than unworked. The store's `derived_rows.json` is **not** the source of this table: it carries only nine 2025 rows, missing 2025-11-28, 2025-12-24 and 2025-12-25, because `parsed.json` marks the Christmas notice `"unparsed": true` and its stacked sub-headers break the store's extractor. Those three rows were re-derived by hand from that notice's bytes, and the same pass re-checked the other nine.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day,2026-01-01,None,5:00 PM (Thu) to 8:30 AM (Fri)` | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-01-01; the only block printed is the Thursday-evening leg, which belongs to trade date 2026-01-02 |
| 2026-01-19 | early close | `Martin Luther King Jr. Day,2026-01-19,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-01-19; the Sunday-evening leg is trade date 2026-01-19's own and stops at 10:30, and the Monday-evening leg is trade date 2026-01-20's |
| 2026-02-16 | early close | `Presidents' Day,2026-02-16,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-02-16; same two-leg shape as MLK Day |
| 2026-04-03 | early close | `Good Friday,2026-04-03,None,5:00 PM (Thu) to 8:30 AM (Fri)` — 08:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-04-03; the Thursday-evening leg is trade date 2026-04-03's own and stops at its normal regular open |
| 2026-05-25 | early close | `Memorial Day,2026-05-25,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-05-25 |
| 2026-06-19 | early close | `Juneteenth Holiday,2026-06-19,None,5:00 PM (Thu) to 10:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-06-19; a Friday, so there is no evening leg to reassign |
| 2026-07-03 | early close | `Independence Day Observed,2026-07-03,None,5:00 PM (Thu) to 10:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-07-03 |
| 2026-09-07 | early close | `Labor Day,2026-09-07,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-09-07 |
| 2026-11-26 | early close | `Thanksgiving Day,2026-11-26,None,5:00 PM (Wed) to 10:30 AM (Thu) and 5:00 PM (Thu) to 8:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-11-26; the Thursday-evening leg belongs to trade date 2026-11-27 |
| 2026-11-27 | early close | `Thanksgiving Early Close,2026-11-27,08:30:00 - 12:15:00` — 12:15 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-11-27; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2026-12-24 | early close | `Christmas Early Close,2026-12-24,08:30:00 - 12:15:00,5:00 PM (Wed) to 8:30 AM (Thu)` — 12:15 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-12-24; no Thursday-evening leg is printed because trade date 2026-12-25 is closed |
| 2026-12-25 | closed | `Christmas Day,2026-12-25,None,None` | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-12-25; both columns empty |

**Gaps, 2025:** none in this window. Every date Cboe's 2025 notices modify ships a row, and every other 2025 trade date is audited normal. No 2025 row is a late open.

**Interpretive steps, 2025.** The 2025 corpus is per-holiday notices rather than one table, and their layouts are not uniform, so each row below was read from its own notice's bytes at word coordinates. On the seven Monday or Thursday holidays whose regular cell is empty while an extended block is printed — MLK, Presidents, Memorial, Juneteenth, Labor, Thanksgiving and the Thursday half day's own leg — the crate reads an **early close** of the leg that opened at 17:00 CT the previous evening, exactly as the 2026 section reads the same shape; the Trade Date row of each notice confirms it, naming the day after the holiday. Three dates are read differently, each because its own notice prints something the others do not: **2025-01-01** and **2025-07-04** print no cell under the holiday column and are skipped by the Trade Date row, so they are `closed`; **2025-04-18** prints neither a Friday close nor a Friday trade date and is likewise `closed`; and **2025-01-09** is stated in prose as a trading day with no regular hours and ships as `ReplacementBlocks`. The 2025-11-28 and 2025-12-24 half days are `09:00` rows in the sense the 2026 section means: Cboe prints `RTH Start 8:30 AM` and a shortened close, so only the close moves. This reading is fenced per date in `tests/futures_family_boundaries/holidays_cfe_vix.rs`.

**Gaps, 2026:** none inside the window, and the horizon is the operator's. Cboe has published no 2027 CFE holiday schedule: verified 2026-09-26 UTC, <https://www.cboe.com/about/hours/us-futures> carries only a “2026 Futures Holiday Schedule” heading, and its CSV twin <https://www.cboe.com/us/futures/holidays/csv/> lists 2026 rows only (`# Generated: 2026:09:18 17:47:39` at the first check). Re-checked 2026-09-29 UTC with fresh live reads of both channels (artifacts under `holidays/raw/cfe/forward-2027/`): the CSV has regenerated with the same twelve 2026 rows (`# Generated: 2026:09:29 16:16:41`) and still names no 2027 date. Nothing is being withheld by this crate — there is simply no 2027 schedule yet. **Closing condition:** the Cboe 2027 futures holiday schedule, at which point the window extends to 2027-12-31. Re-checked monthly per LAW-WATCH. No row in this window changes intraday phase topology, and no row is a late open.

**Interpretive steps, 2026:** Cboe prints two cells per holiday, `Regular Trading Hours` and `Extended Trading Hours`. On nine of these dates the regular cell reads `None` while the extended cell still names a block. On the crate's trade-date key that is an **early close** of the leg which opened at 17:00 CT the previous evening, not a closure: the block printed as “5:00 PM (Sun) to 10:30 AM (Mon)” *is* trade date Monday's own session, and the second block “5:00 PM (Mon) to 8:30 AM (Tue)” belongs to the next trade date and needs no row. Two dates are different. 2026-01-01 is `closed` because the only block printed is the Thursday-evening leg of trade date 2026-01-02, so no session at all belongs to 2026-01-01; and 2026-12-25 prints `None,None`. This reading answers the retrieval's own advisory on 2026-01-01 and is fenced in `tests/futures_family_boundaries/holidays_cfe_vix.rs`.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below. All times are Chicago time.

- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf> — Cboe SR-CFE-2010-013, the 2010-12-10 extended-session filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf> — Cboe SR-CFE-2011-019, the 2011-09-26 07:00 start — T1.
- <https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx> — the 2013 phased-hours announcement, describing the predecessor 07:00–15:15 trading day and the exact phase shapes — T1.
- <https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx> — Cboe's year-end retrospective, recording the actual 2013-10-28 and 2013-11-04 launches — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf> — Cboe IC13-041, pinning both 2013 phases and publishing the 15:29–15:30 weekday pre-open queue — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf> — Cboe SR-CFE-2013-032, the 2013 extended-hours filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf> — Cboe SR-CFE-2013-034, the 2013 extended-hours filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf> — Cboe SR-CFE-2014-010, introducing nearly-24-hour VX trading — T1.
- <https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx> — the 2014 round-the-clock launch announcement — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf> — Cboe IC14-036, publishing the Sunday 16:15–17:00 pre-open and the retained 15:29–15:30 weekday pre-open — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf> — Cboe RG-CFE-2014-020, pinning the launch to Sunday 2014-06-22 — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf> — Cboe SR-CFE-2017-017, the migration-era phase enumeration — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf> — Cboe RG-CFE-2018-005, confirming the migration completed Sunday 2018-02-25 for business date Monday 2018-02-26 — T1.
- <https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf> — Cboe C2018071603, the 2018-08-12 TAS queue-commencement change — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf> — rule certification CFE-2021-028 behind Cboe notice C2021102603, the current queues and trading phases — T1.
- <https://www.cboe.com/about/hours/us-futures> — CFE trading hours, the current-schedule monitoring entry point — T1.
- <https://www.cboe.com/tradable-products/vix/vix-futures/specifications> — VIX futures specifications — T1.
- <https://www.cboe.com/markets/us/futures/regulation/circulars/cfe/regulatory/> — CFE regulatory circulars, the watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The normal-week history is primary-supported from
  the January-2010 floor, so the row is `Primary` and its horizon is the floor
  itself rather than a carried-back date.
- **Randomized queue starts are modelled conservatively, not exactly.** From
  2018-02-25 CFE begins each opening queue at a randomized instant within three
  seconds of the nominal boundary, and from 2018-08-12 within six seconds for
  TAS contracts. The profile publishes the latest acceptance edge
  (16:00:03 / 16:45:03, then 16:00:06 / 16:45:06 CT) because each queue follows a
  closed or suspended period, so a caller never sees an open queue that CFE had
  not yet started. No source states a per-contract instant, and none is claimed.
- **Scope.** The `cfe` venue default is specifically VIX futures. Any other CFE
  contract family needs its own review and its own profile; a listing venue is
  not a venue-wide clock.

## Module narrative (moved from src/calendar/schedules/futures/us/cfe.rs on 2026-09-12 UTC)

CFE (VIX) — current schedule, effective 2021-12-06.

RTH is 08:30–15:00 CT. ETH runs 15:00–16:00 and, from Sunday plus
Monday–Thursday, 17:00–08:30. Order-entry queues run Sunday 16:00–17:00 and
Monday–Thursday 16:45–17:00, with starts randomized through six seconds after
the nominal boundary. Because each queue follows a closed/suspended period,
the profile uses the conservative latest 16:00:06 and 16:45:06 edges. The
change removed the former 15:15–15:30 queue and 15:00–15:15 RTH segment.

Sources: Cboe notice C2021102603, effective 2021-12-06; rule certification
CFE-2021-028 (all times in Chicago time).
<https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf>

ORDER-ENTRY CLASSIFICATION. The notice quoted above calls the Sunday
16:00-17:00 and Monday-Thursday 16:45-17:00 windows "order-entry queues":
CFE accepts non-market orders that cannot execute until trading resumes at
17:00, so no trade can print inside them. They are `order_entry`. The
15:00-16:00 and 17:00-08:30 ETH windows match and stay in `extended`.

At the January-2010 audit floor, VX traded 08:30-15:15 CT. CFE then
introduced a 07:20-08:30 extended session effective 2010-12-10 and moved
that start to 07:00 effective 2011-09-26. The filings state both day-level
effective dates and preserve the 08:30-15:15 regular session.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf>

CFE expanded VX hours in two phases during 2013. Its announcement describes
the predecessor 07:00–15:15 trading day and the exact phase shapes; Cboe's
year-end retrospective records the actual launches as 2013-10-28 for the
Monday–Thursday 15:30–16:15 session and 2013-11-04 for the move from a 07:00
to a 02:00 morning open. IC13-041 pins those phases to 2013-10-28 and
2013-11-04 and publishes the 15:29–15:30 Monday–Thursday pre-open queue in
both. RTH remained 08:30–15:15 throughout.
<https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx>
<https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx>
<https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf>

IC13-041 publishes 15:29-15:30 as a Monday-Thursday "pre-open queue" ahead of
the 15:30 session, so it accepts orders without matching and is `order_entry`.

CFE-2014-010 introduced nearly-24-hour VX trading on Sunday 2014-06-22.
IC14-036 publishes the resulting 16:15–17:00 Sunday pre-open and retained
15:29–15:30 weekday pre-open; RG-CFE-2014-020 pins the launch to that Sunday.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf>
<https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx>
<https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf>
<https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf>

IC14-036 names both the Sunday 16:15-17:00 phase and the retained 15:29-15:30
weekday phase as pre-opens: orders queue, nothing matches.

SR-CFE-2017-017 tied a revised VX schedule to CFE's system migration:
08:30–15:15 RTH, a 15:15–15:30 order-entry-only queue, 15:30–16:00 ETH, a
16:00–16:45 weekday suspension, a 16:45–17:00 queue, then 17:00–08:30 ETH.
Sunday has a 16:00–17:00 opening queue. The queues accept non-market orders
that cannot execute until trading resumes, so the crate classifies them as
`order_entry` under the order-entry-phase convention — `cfe.rs` puts all three
(the 15:15–15:30 weekday queue and the Sunday 16:00 and weekday 16:45 opening
queues) in `CFE_ORDER_ENTRY_2018_02_25`, never in `extended`. The new-system opening
queues begin at randomized instants through three seconds after the nominal
boundary, so their conservative edges are 16:00:03 and 16:45:03. RG18-005
confirms that the migration completed Sunday 2018-02-25, for business date
Monday 2018-02-26.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf>
<https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf>
The filing above enumerates the migration-era phases separately: RTH ends at
15:15, 15:15-15:30 is an "order-entry-only queue", 15:30-16:00 is ETH, and
the 16:00/16:45 opening queues run to the 17:00 ETH open. Only the two ETH
windows match, so the 15:15-15:30 queue is split out of the former merged
15:15-16:00 rule and joins the evening queues in `order_entry`.

C2018071603 changed TAS queue commencement to a randomized instant three to
six seconds after the nominal Sunday 16:00 and weekday 16:45 boundaries,
effective with the Sunday 2018-08-12 opening. Non-TAS queues remained within
zero to three seconds. The all-contract profile therefore advances its
conservative latest edge from three to six seconds on that opening day.
<https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf>
Only the queue-commencement seconds change here, so the matching grid is the
one `CFE_EXT_2018_02_25` already carries.

Row evidence — each revision's day-level effective date and the primary
source that states it (full quotations sit in the blocks above):

```text
  2010-12-10 "Cboe SR-CFE-2010-013"
    https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf
  2011-09-26 "Cboe SR-CFE-2011-019"
    https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf
  2013-10-28 "Cboe IC13-041" and 2013-11-04 "Cboe IC13-041"
    https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf
  2014-06-22 "Cboe RG-CFE-2014-020"
    https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf
  2018-02-25 "Cboe RG-CFE-2018-005"
    https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf
  2018-08-12 "Cboe C2018071603"
    https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf
  2021-12-06 "Cboe C2021102603"
    https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf
```

The 2018-02-25 tuple is keyed to the Sunday implementation; the revised
weekday hours first occur on CFE's Monday 2018-02-26 business date.
