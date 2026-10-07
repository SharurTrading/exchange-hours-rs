// SPDX-License-Identifier: MIT-0

//! Cboe Futures Exchange holiday rows over three audited windows:
//! 2014-12-24..2015-01-02, 2015-02-15..2016-01-20 and 2017-04-10..2026.
//!
//! Keyed by the crate's own venue-local trade date in `US/Central` (design
//! memo D1). CFE's trading day wraps: the session for trade date `D` opens on
//! `D-1` (17:00 CT Sunday for Mondays, and — on the pre-migration grid in
//! force 2014-06-22..2018-02-24 — 15:30 CT Monday-Thursday), runs through the
//! regular session and ends with the extended window on `D`. Cboe's holiday
//! tables are keyed by civil date and print two cells per row, `Regular
//! Trading Hours` and `Extended Trading Hours`, so a row whose regular cell
//! reads `None` and whose extended cell still names a block is the overnight
//! leg of that trade date stopping early — an **early close**, not a closure.
//!
//! The whole block is **T1**. Three document families key the windows: the
//! operator's own `CFE Holiday Schedule` page on `cfe.cboe.com` (Wayback `id_`
//! captures 2017-04-10 through 2019-12-15), which prints the complete 2017
//! holiday calendar and the per-holiday-type hours tables; the per-holiday CFE
//! notices under `cdn.cboe.com/resources/schedule_update/<publication
//! year>/`, which state each 2018-2026 holiday's session instants and carry
//! the observed-day arrangements the page's default rules only describe; and —
//! recovered 2026-10-06 UTC from `ir.cboe.com` and the operator's own circular
//! series — the CBOE Holdings holiday press releases of 2014-2017 (each
//! printing a `CBOE, C2 and CFE Trading Schedule` table) and the CFE
//! information circulars IC15-011, IC15-023, IC15-028 and IC15-039, which
//! state the 2014 Christmas and 2015 arrangements the same way. Coverage stops
//! at 2026-12-31 because Cboe has published no 2027 schedule; the remaining
//! unaudited spans (2010-01-01..2014-12-23, 2015-01-03..2015-02-14 and
//! 2016-01-21..2017-04-09) are recorded in
//! [`docs/evidence/cfe.md`](../../../../../docs/evidence/cfe.md) and
//! [`docs/evidence/cfe_vix.md`](../../../../../docs/evidence/cfe_vix.md).
//!
//! Era shapes the per-date reader should know. The Monday/Thursday and mid-week
//! floating holidays keep the overnight leg running to 10:30 CT with no regular
//! session (`early close 10:30`), 2015 and 2017 through 2026 alike. Good Friday
//! is a closure in 2017-2018-2019-2020-2022-2024-2025 and an early close at
//! the regular open in 2021-2023-2026 — each year's own notice states which;
//! 2015-04-03 is the one pre-2017 early close, at the operator's own printed
//! 08:15 CT. Independence Day and Christmas eves close at 12:15 CT where the
//! operator's notice says so (2014-12-24, 2015-12-24, 2019-07-03, 2023-07-03,
//! 2024-07-03, the December eves of 2018-2020 and 2024) and trade normally
//! where it does not (2018-07-03, whose notice's holiday leg opens 5:00 p.m.
//! Tuesday, and 2022-12-23, whose notice prints the normal 3:00/4:00 PM
//! closes). New Year's Day and Christmas Monday-Thursday print no holiday-day
//! session at all and reopen at 17:00 CT on the holiday, so the trade date is
//! `Closed` and the prior-evening leg is deleted. 2021-12-24 (Christmas
//! observed Friday) is a full closure with the Thursday-evening leg deleted.
//! Juneteenth enters the set in 2022; the operator's own 2021 notice states
//! CFE traded unadjusted hours that year.
//!
//! **The pre-migration next-day legs.** On the 2014-06-22 grid the next
//! business day's session begins 15:30 CT the previous afternoon, but every
//! pre-migration holiday chart and notice prints the day-after column as
//! beginning **5:00 p.m. on the holiday** (`Tuesday: Extended 5:00 p.m.
//! (Monday) to 8:30 a.m.`) — the holiday's own 15:30-17:00 stretch did not
//! trade. Each such day-after trade date therefore ships a `LateOpen` (or,
//! where the day after is itself a half day, a `LateOpenAndEarlyClose`) row
//! keyed to the same document that keys the holiday, and without it the table
//! would claim trading the operator's own bytes deny. After the 2018-02-25
//! migration the native evening open is 17:00 CT, so no post-migration row
//! needs the companion.
//!
//! One table serves both the `cfe` venue and the `cfe_vix` key: Cboe publishes
//! one schedule for all CFE futures, and VIX futures are the only family the
//! crate routes to the venue, so the design memo's venue intersection (D17) is
//! that one family's own table.
//!
//! Two 2025 rows are not the 2026 shapes. Good Friday 2025-04-18 prints no
//! Friday close and no Friday trade date, so it is a **closure** where the 2026
//! date is an early close at the regular open. The National Day of Mourning
//! 2025-01-09 is a **trading day with no regular hours at all**: the only
//! session is extended trading from 17:00 CT on 2025-01-08 to 08:30 CT on
//! 2025-01-09, which no scalar clip can state, so it ships as a
//! `ReplacementBlocks` row.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, ReplacementBlocks, Unsourced};
use super::fences::{early_close, late_open, late_open_and_early_close};
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The complete 2025-01-09 trading day, relative to that trade date.
///
/// Cboe states it as one extended session — `5:00 p.m. (Wednesday, January 8,
/// 2025) to 8:30 a.m. (Extended Trading Hours) for VX, VXM, VXT, and VXMT` —
/// and `No Regular Trading Hours for VX, VXM, VXT, and VXMT`. So the day is one
/// wrapping extended block and nothing else: no regular block, and no
/// 15:00-16:00 CT extended window, because the session has already ended.
///
/// Cboe also states that `The Thursday, January 9, 2025, business day will end
/// at 8:30 a.m. CT on January 9, 2025, and the daily settlement prices of VX,
/// VXM, IBHY, IBIG, and IEMD futures will be determined at that time.` That
/// settlement instant coincides with the close and is therefore **not** a
/// boundary of its own (LAW-SESSION-NOT-EXPIRY); nothing is modelled for it.
///
/// Evidence: `docs/evidence/cfe.md`, `docs/evidence/cfe_vix.md`.
// The row's bounds are read from this declaration by
// `a_replacement_rows_printed_instants_are_the_block_bounds_it_ships`, which
// parses one `ExceptionBlock::<kind>(offset, open, close)` call per line, so the
// constructor stays on one line.
#[rustfmt::skip]
static MOURNING_2025_01_09_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60)];

/// CFE's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of a Cboe CFE holiday notice, of the operator's own
/// `CFE Holiday Schedule` page as captured by the Wayback mirror, of a CBOE
/// Holdings holiday press release, or of a CFE information circular. A date
/// inside a window with no row is audited normal; 2017-07-03 is the one
/// `Unsourced` date — the page's own `typically close at 12:15 p.m. on July 3`
/// sentence leaves the day open and no controlling circular survives. The late
/// November and Christmas-eve rows carry rows because Cboe prints them as half
/// days, not because they are holidays.
// Evidence: docs/evidence/cfe.md, docs/evidence/cfe_vix.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [
        (2014, 12, 24) ..= (2015, 1, 2),
        (2015, 2, 15) ..= (2016, 1, 20),
        (2017, 4, 10) ..= (2026, 12, 31),
    ],
    rows: [
        // 2014-12-24 - T1 - CBOE-PR-2014-CHRISTMAS-NEW-YEAR - Christmas Eve:
        // the release's CFE column closes VX at 12:15 p.m. CT (12:00/12:12 are
        // the other products' rows), and VX/VXT stay closed until 17:00 CT on
        // 12/25.
        (2014, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-PR-2014-CHRISTMAS-NEW-YEAR"),
        // 2014-12-25 - T1 - CBOE-PR-2014-CHRISTMAS-NEW-YEAR - Christmas Day:
        // no holiday-day session; trading resumed 5:00 p.m. CT on the holiday
        // for business day Friday 12/26, so the trade date is deleted with its
        // prior-evening wrap.
        (2014, 12, 25, Closed, T1, "CBOE-PR-2014-CHRISTMAS-NEW-YEAR"),
        // 2014-12-26 - T1 - CBOE-PR-2014-CHRISTMAS-NEW-YEAR - the release
        // prints the Friday leg as `5:00 p.m. CT on Thur, 12/25 to 8:30 a.m.
        // CT on Fri, 12/26 - Extended trading hours`, so the day's first open
        // moves from the standing 15:30 CT to 17:00 CT on the holiday.
        (2014, 12, 26, late_open(17 * 3_600), T1, "CBOE-PR-2014-CHRISTMAS-NEW-YEAR"),
        // 2015-01-01 - T1 - CBOE-PR-2014-CHRISTMAS-NEW-YEAR - New Year's Day:
        // no holiday-day session; the Thursday-evening leg belongs to trade
        // date 1/2 and the release deletes the wrap (`closed from 3:15 p.m.
        // CT on Wed, 12/31 until 5:00 p.m. CT on Thur, 1/1`).
        (2015, 1, 1, Closed, T1, "CBOE-PR-2014-CHRISTMAS-NEW-YEAR"),
        // 2015-01-02 - T1 - CBOE-PR-2014-CHRISTMAS-NEW-YEAR - the release
        // prints the Friday leg as `5:00 p.m. CT on Thur, 1/1 to 8:30 a.m. CT
        // on Fri, 1/2`, so the day's first open moves to 17:00 CT on the
        // holiday.
        (2015, 1, 2, late_open(17 * 3_600), T1, "CBOE-PR-2014-CHRISTMAS-NEW-YEAR"),
        // 2015-02-16 - T1 - CBOE-PR-2015-PRESIDENTS - Presidents' Day: the
        // release halts VX at 10:30 a.m. CT (Regular `None`).
        (2015, 2, 16, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-PR-2015-PRESIDENTS"),
        // 2015-02-17 - T1 - CBOE-PR-2015-PRESIDENTS - the release prints the
        // Tuesday leg as resuming 5:00 p.m. CT on the holiday for business day
        // Tuesday, so the day's first open moves from 15:30 CT to 17:00 CT.
        (2015, 2, 17, late_open(17 * 3_600), T1, "CBOE-PR-2015-PRESIDENTS"),
        // 2015-04-03 - T1 - CFE-IC15-011 - Good Friday: the circular's own
        // holiday-session table runs one extended session `3:30 p.m.
        // (Thursday) to 8:15 a.m. (Friday)` with Regular `None` — the 08:15
        // close is the operator's own printed instant (the employment-report
        // window), not the later 08:30 shape.
        (2015, 4, 3, early_close(8 * 3_600 + 15 * 60), T1, "CFE-IC15-011"),
        // 2015-05-25 - T1 - CFE-IC15-023 - Memorial Day: the circular prints
        // Extended `5:00 p.m. (Sunday) to 10:30 a.m.` / Regular `None`.
        (2015, 5, 25, early_close(10 * 3_600 + 30 * 60), T1, "CFE-IC15-023"),
        // 2015-05-26 - T1 - CFE-IC15-023 - the circular's Tuesday column
        // prints Extended `5:00 p.m. (Monday) to 8:30 a.m.`, so the day's
        // first open moves from 15:30 CT to 17:00 CT on the holiday.
        (2015, 5, 26, late_open(17 * 3_600), T1, "CFE-IC15-023"),
        // 2015-07-03 - T1 - CFE-IC15-028 - Independence Day observed Friday:
        // the circular states CFE closed for trading in all products, the
        // Thursday close 3:15 p.m., and a normal Sunday 17:00 reopen.
        (2015, 7, 3, Closed, T1, "CFE-IC15-028"),
        // 2015-09-07 - T1 - CFE-IC15-039 - Labor Day: the circular prints
        // Extended `5:00 p.m. (Sunday) to 10:30 a.m.` / Regular `None`.
        (2015, 9, 7, early_close(10 * 3_600 + 30 * 60), T1, "CFE-IC15-039"),
        // 2015-09-08 - T1 - CFE-IC15-039 - the circular's Tuesday column
        // prints Extended `5:00 p.m. (Monday) to 8:30 a.m.`, so the day's
        // first open moves from 15:30 CT to 17:00 CT on the holiday.
        (2015, 9, 8, late_open(17 * 3_600), T1, "CFE-IC15-039"),
        // 2015-11-26 - T1 - CBOE-PR-2015-THANKSGIVING - Thanksgiving Day: the
        // release halts VX at 10:30 a.m. CT (the leg that opened 15:30 CT
        // Wednesday).
        (2015, 11, 26, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-PR-2015-THANKSGIVING"),
        // 2015-11-27 - T1 - CBOE-PR-2015-THANKSGIVING - Thanksgiving half day:
        // the release prints the Friday leg as resuming 5:00 p.m. CT on the
        // holiday for business day Friday and closing 12:15 p.m. CT, so both
        // boundaries move.
        (2015, 11, 27, late_open_and_early_close(17 * 3_600, 12 * 3_600 + 15 * 60), T1, "CBOE-PR-2015-THANKSGIVING"),
        // 2015-12-24 - T1 - CBOE-PR-2015-CHRISTMAS-NEW-YEAR - Christmas Eve:
        // the release closes VX at 12:15 p.m. CT and holds VX/VXT closed
        // until 5:00 p.m. CT on Sunday 12/27.
        (2015, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-PR-2015-CHRISTMAS-NEW-YEAR"),
        // 2015-12-25 - T1 - CBOE-PR-2015-CHRISTMAS-NEW-YEAR - Christmas Day:
        // no holiday-day session; trading resumed 5:00 p.m. CT on Sunday
        // 12/27 for business day 12/28, so the trade date is deleted with its
        // prior-evening wrap.
        (2015, 12, 25, Closed, T1, "CBOE-PR-2015-CHRISTMAS-NEW-YEAR"),
        // 2016-01-01 - T1 - CBOE-PR-2015-CHRISTMAS-NEW-YEAR - New Year's Day:
        // no holiday-day session; the release deletes the wrap (`closed from
        // 3:15 p.m. CT on Thursday, 12/31 until 5:00 p.m. CT on Sunday, 1/3`).
        (2016, 1, 1, Closed, T1, "CBOE-PR-2015-CHRISTMAS-NEW-YEAR"),
        // 2016-01-18 - T1 - CBOE-PR-2016-MLK - MLK Jr. Day: the release halts
        // VX at 10:30 a.m. CT (Regular `None`).
        (2016, 1, 18, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-PR-2016-MLK"),
        // 2016-01-19 - T1 - CBOE-PR-2016-MLK - the release prints the Tuesday
        // leg as resuming 5:00 p.m. CT on the holiday for business day
        // Tuesday, so the day's first open moves from 15:30 CT to 17:00 CT.
        (2016, 1, 19, late_open(17 * 3_600), T1, "CBOE-PR-2016-MLK"),
        // 2017-04-14 - T1 - cfe-holiday-calendar @2017-04-10T21:04:29Z - Good
        // Friday: the Friday-holiday chart prints Extended `None` and Regular
        // `None`, so the day is deleted outright.
        (2017, 4, 14, Closed, T1, "cfe-holiday-calendar @2017-04-10T21:04:29Z"),
        // 2017-05-29 - T1 - cfe-holiday-calendar @2017-04-10T21:04:29Z -
        // Memorial Day, 10:30 CT (the Monday-holiday chart: Regular `None`,
        // extended to 10:30 a.m.).
        (2017, 5, 29, early_close(10 * 3_600 + 30 * 60), T1, "cfe-holiday-calendar @2017-04-10T21:04:29Z"),
        // 2017-05-30 - T1 - cfe-holiday-calendar @2017-04-10T21:04:29Z - the
        // chart's Tuesday column prints Extended `5:00 p.m. (Monday) to 8:30
        // a.m.`, so the day's first open moves from 15:30 CT to 17:00 CT on
        // the holiday.
        (2017, 5, 30, late_open(17 * 3_600), T1, "cfe-holiday-calendar @2017-04-10T21:04:29Z"),
        // 2017-07-03 - T1 - cfe-holiday-calendar @2017-06-26T00:36:49Z - the
        // page's own `typically close at 12:15 p.m. on July 3` leaves the day
        // open and no controlling circular survives; not a closure, not a
        // proven normal day.
        (2017, 7, 3, Unsourced, T1, "cfe-holiday-calendar @2017-06-26T00:36:49Z"),
        // 2017-07-04 - T1 - cfe-holiday-calendar @2017-06-26T00:36:49Z -
        // Independence Day on a Tuesday: the Tuesday-Thursday chart prints
        // Regular `None` with the July 3 evening leg to 10:30 a.m.
        (2017, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "cfe-holiday-calendar @2017-06-26T00:36:49Z"),
        // 2017-07-05 - T1 - cfe-holiday-calendar @2017-06-26T00:36:49Z - the
        // chart's July 5 column prints Extended `5:00 p.m. (July 4) to 8:30
        // a.m.`, so the day's first open moves from 15:30 CT to 17:00 CT on
        // the holiday.
        (2017, 7, 5, late_open(17 * 3_600), T1, "cfe-holiday-calendar @2017-06-26T00:36:49Z"),
        // 2017-09-04 - T1 - cfe-holiday-calendar @2017-06-26T00:36:49Z -
        // Labor Day, 10:30 CT (the Monday-holiday chart).
        (2017, 9, 4, early_close(10 * 3_600 + 30 * 60), T1, "cfe-holiday-calendar @2017-06-26T00:36:49Z"),
        // 2017-09-05 - T1 - cfe-holiday-calendar @2017-06-26T00:36:49Z - the
        // chart's Tuesday column prints Extended `5:00 p.m. (Monday) to 8:30
        // a.m.`, so the day's first open moves from 15:30 CT to 17:00 CT on
        // the holiday.
        (2017, 9, 5, late_open(17 * 3_600), T1, "cfe-holiday-calendar @2017-06-26T00:36:49Z"),
        // 2017-11-23 - T1 - cfe-holiday-calendar @2017-11-13T01:40:35Z -
        // Thanksgiving Day, 10:30 CT (the Thanksgiving chart).
        (2017, 11, 23, early_close(10 * 3_600 + 30 * 60), T1, "cfe-holiday-calendar @2017-11-13T01:40:35Z"),
        // 2017-11-24 - T1 - cfe-holiday-calendar @2017-11-13T01:40:35Z -
        // Thanksgiving half day: the chart's Friday column prints Extended
        // `5:00 p.m. (Thursday) to 8:30 a.m.` and Regular `8:30 a.m. to 12:15
        // p.m.`, so the first open moves to 17:00 CT on the holiday and the
        // close to 12:15 CT.
        (2017, 11, 24, late_open_and_early_close(17 * 3_600, 12 * 3_600 + 15 * 60), T1, "cfe-holiday-calendar @2017-11-13T01:40:35Z"),
        // 2017-12-25 - T1 - cfe-holiday-calendar @2017-11-13T01:40:35Z -
        // Christmas on a Monday: the Monday-Thursday chart prints no
        // holiday-day session and a 5:00 p.m. reopen on the holiday itself.
        (2017, 12, 25, Closed, T1, "cfe-holiday-calendar @2017-11-13T01:40:35Z"),
        // 2017-12-26 - T1 - cfe-holiday-calendar @2017-11-13T01:40:35Z - the
        // chart's day-after column prints Extended `5:00 p.m. (on holiday) to
        // 8:30 a.m. (day after holiday)`, so the day's first open moves from
        // 15:30 CT to 17:00 CT on the holiday.
        (2017, 12, 26, late_open(17 * 3_600), T1, "cfe-holiday-calendar @2017-11-13T01:40:35Z"),
        // 2018-01-01 - T1 - cfe-holiday-calendar @2017-12-29T07:06:24Z - New
        // Year's Day on a Monday: the Monday-Thursday chart prints no
        // holiday-day session; the 2018 calendar names the date.
        (2018, 1, 1, Closed, T1, "cfe-holiday-calendar @2017-12-29T07:06:24Z"),
        // 2018-01-02 - T1 - cfe-holiday-calendar @2017-12-29T07:06:24Z - the
        // chart's day-after column prints Extended `5:00 p.m. (on holiday) to
        // 8:30 a.m. (day after holiday)`, so the day's first open moves from
        // 15:30 CT to 17:00 CT on the holiday.
        (2018, 1, 2, late_open(17 * 3_600), T1, "cfe-holiday-calendar @2017-12-29T07:06:24Z"),
        // 2018-01-15 - T1 - CBOE-SU-2018-MLK - MLK Jr. Day, 10:30 CT.
        (2018, 1, 15, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2018-MLK"),
        // 2018-01-16 - T1 - CBOE-SU-2018-MLK - the notice's Tuesday column
        // prints Extended `5:00 p.m. (Monday) to 8:30 a.m.`, so the day's
        // first open moves from 15:30 CT to 17:00 CT on the holiday.
        (2018, 1, 16, late_open(17 * 3_600), T1, "CBOE-SU-2018-MLK"),
        // 2018-02-19 - T1 - CBOE-SU-2018-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2018, 2, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2018-PRESIDENTS"),
        // 2018-02-20 - T1 - CBOE-SU-2018-PRESIDENTS - the notice's Tuesday
        // column prints Extended `5:00 p.m. (Monday) to 8:30 a.m.`, so the
        // day's first open moves from 15:30 CT to 17:00 CT on the holiday.
        (2018, 2, 20, late_open(17 * 3_600), T1, "CBOE-SU-2018-PRESIDENTS"),
        // 2018-03-30 - T1 - CBOE-SU-2018-GOOD-FRIDAY - Good Friday: the notice
        // states trading will be closed for all CFE products.
        (2018, 3, 30, Closed, T1, "CBOE-SU-2018-GOOD-FRIDAY"),
        // 2018-05-28 - T1 - CBOE-SU-2018-MEMORIAL - Memorial Day, 10:30 CT.
        (2018, 5, 28, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2018-MEMORIAL"),
        // 2018-07-04 - T1 - CBOE-SU-2018-INDEPENDENCE - Independence Day on a
        // Wednesday, 10:30 CT; the notice's holiday leg opens 5:00 p.m.
        // Tuesday, so 2018-07-03 traded normally.
        (2018, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2018-INDEPENDENCE"),
        // 2018-09-03 - T1 - CBOE-SU-2018-LABOR - Labor Day, 10:30 CT.
        (2018, 9, 3, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2018-LABOR"),
        // 2018-11-22 - T1 - CBOE-SU-2018-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2018, 11, 22, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2018-THANKSGIVING"),
        // 2018-11-23 - T1 - CBOE-SU-2018-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT; the Wednesday-evening leg belongs to the 2018-11-22 row.
        (2018, 11, 23, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2018-THANKSGIVING"),
        // 2018-12-24 - T1 - CBOE-SU-2018-CHRISTMAS - Christmas Eve: the notice
        // states trading in all CFE products will close at 12:15 p.m.
        (2018, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2018-CHRISTMAS"),
        // 2018-12-25 - T1 - CBOE-SU-2018-CHRISTMAS - Christmas Day: the notice
        // states the reopen at 5:00 p.m. on the holiday itself, so no session
        // belongs to the day.
        (2018, 12, 25, Closed, T1, "CBOE-SU-2018-CHRISTMAS"),
        // 2019-01-01 - T1 - CBOE-SU-2019-NEW-YEAR - New Year's Day: the notice
        // states the reopen at 5:00 p.m. on the holiday itself and normal hours
        // Monday 2018-12-31.
        (2019, 1, 1, Closed, T1, "CBOE-SU-2019-NEW-YEAR"),
        // 2019-01-21 - T1 - CBOE-SU-2019-MLK - MLK Jr. Day, 10:30 CT.
        (2019, 1, 21, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2019-MLK"),
        // 2019-02-18 - T1 - CBOE-SU-2019-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2019, 2, 18, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2019-PRESIDENTS"),
        // 2019-04-19 - T1 - CBOE-SU-2019-GOOD-FRIDAY - Good Friday: the notice
        // states trading will be closed for all CFE products.
        (2019, 4, 19, Closed, T1, "CBOE-SU-2019-GOOD-FRIDAY"),
        // 2019-05-27 - T1 - CBOE-SU-2019-MEMORIAL - Memorial Day, 10:30 CT.
        (2019, 5, 27, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2019-MEMORIAL"),
        // 2019-07-03 - T1 - CBOE-SU-2019-INDEPENDENCE - the eve before the
        // Thursday holiday: the notice states the 12:15 p.m. close outright.
        (2019, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2019-INDEPENDENCE"),
        // 2019-07-04 - T1 - CBOE-SU-2019-INDEPENDENCE - Independence Day on a
        // Thursday, 10:30 CT.
        (2019, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2019-INDEPENDENCE"),
        // 2019-09-02 - T1 - CBOE-SU-2019-LABOR - Labor Day, 10:30 CT.
        (2019, 9, 2, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2019-LABOR"),
        // 2019-11-28 - T1 - CBOE-SU-2019-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2019, 11, 28, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2019-THANKSGIVING"),
        // 2019-11-29 - T1 - CBOE-SU-2019-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT.
        (2019, 11, 29, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2019-THANKSGIVING"),
        // 2019-12-24 - T1 - CBOE-SU-2019-CHRISTMAS - Christmas Eve: the
        // notice's per-product table closes VX at 12:15 p.m.
        (2019, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2019-CHRISTMAS"),
        // 2019-12-25 - T1 - CBOE-SU-2019-CHRISTMAS - Christmas Day: the notice
        // states the reopen at 5:00 p.m. on the holiday itself.
        (2019, 12, 25, Closed, T1, "CBOE-SU-2019-CHRISTMAS"),
        // 2020-01-01 - T1 - CBOE-SU-2020-NEW-YEAR - New Year's Day: the notice
        // states normal hours Tuesday 2019-12-31 and the 5:00 p.m. reopen on
        // the holiday itself.
        (2020, 1, 1, Closed, T1, "CBOE-SU-2020-NEW-YEAR"),
        // 2020-01-20 - T1 - CBOE-SU-2020-MLK - MLK Jr. Day, 10:30 CT.
        (2020, 1, 20, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2020-MLK"),
        // 2020-02-17 - T1 - CBOE-SU-2020-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2020, 2, 17, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2020-PRESIDENTS"),
        // 2020-04-10 - T1 - CBOE-SU-2020-GOOD-FRIDAY - Good Friday: the notice
        // states trading will be closed for all CFE products.
        (2020, 4, 10, Closed, T1, "CBOE-SU-2020-GOOD-FRIDAY"),
        // 2020-05-25 - T1 - CBOE-SU-2020-MEMORIAL - Memorial Day, 10:30 CT.
        (2020, 5, 25, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2020-MEMORIAL"),
        // 2020-07-03 - T1 - CBOE-SU-2020-INDEPENDENCE - Independence Day
        // observed Friday, 10:30 CT; the notice states normal hours Thursday.
        (2020, 7, 3, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2020-INDEPENDENCE"),
        // 2020-09-07 - T1 - CBOE-SU-2020-LABOR - Labor Day, 10:30 CT.
        (2020, 9, 7, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2020-LABOR"),
        // 2020-11-26 - T1 - CBOE-SU-2020-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2020, 11, 26, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2020-THANKSGIVING"),
        // 2020-11-27 - T1 - CBOE-SU-2020-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT.
        (2020, 11, 27, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2020-THANKSGIVING"),
        // 2020-12-24 - T1 - CBOE-SU-2020-CHRISTMAS - Christmas Eve: the
        // notice's per-product table closes VX at 12:15 p.m.
        (2020, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2020-CHRISTMAS"),
        // 2020-12-25 - T1 - CBOE-SU-2020-CHRISTMAS - Christmas Day: the notice
        // states the closure and no Thursday-evening hours.
        (2020, 12, 25, Closed, T1, "CBOE-SU-2020-CHRISTMAS"),
        // 2021-01-01 - T1 - CBOE-SU-2021-NEW-YEAR - New Year's Day: the notice
        // states the closure and no Thursday-evening hours.
        (2021, 1, 1, Closed, T1, "CBOE-SU-2021-NEW-YEAR"),
        // 2021-01-18 - T1 - CBOE-SU-2021-MLK - MLK Jr. Day, 10:30 CT.
        (2021, 1, 18, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-MLK"),
        // 2021-02-15 - T1 - CBOE-SU-2021-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2021, 2, 15, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-PRESIDENTS"),
        // 2021-04-02 - T1 - CBOE-SU-2021-GOOD-FRIDAY - Good Friday, 08:30 CT:
        // the notice's table stops the Thursday-evening leg at the regular
        // open and prints no regular session.
        (2021, 4, 2, early_close(8 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-GOOD-FRIDAY"),
        // 2021-05-31 - T1 - CBOE-SU-2021-MEMORIAL - Memorial Day, 10:30 CT.
        (2021, 5, 31, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-MEMORIAL"),
        // 2021-07-05 - T1 - CBOE-SU-2021-INDEPENDENCE - Independence Day
        // observed Monday, 10:30 CT; the notice's table runs the Sunday-evening
        // leg to 10:30 a.m. (Juneteenth 2021 traded unadjusted: C2021061701).
        (2021, 7, 5, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-INDEPENDENCE"),
        // 2021-09-06 - T1 - CBOE-SU-2021-LABOR - Labor Day, 10:30 CT.
        (2021, 9, 6, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-LABOR"),
        // 2021-11-25 - T1 - CBOE-SU-2021-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2021, 11, 25, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2021-THANKSGIVING"),
        // 2021-11-26 - T1 - CBOE-SU-2021-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT.
        (2021, 11, 26, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2021-THANKSGIVING"),
        // 2021-12-24 - T1 - CBOE-SU-2021-CHRISTMAS - Christmas observed Friday:
        // the notice states the closure and no Thursday-evening hours.
        (2021, 12, 24, Closed, T1, "CBOE-SU-2021-CHRISTMAS"),
        // 2022-01-17 - T1 - CBOE-SU-2022-MLK - MLK Jr. Day, 10:30 CT.
        (2022, 1, 17, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-MLK"),
        // 2022-02-21 - T1 - CBOE-SU-2022-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2022, 2, 21, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-PRESIDENTS"),
        // 2022-04-15 - T1 - CBOE-SU-2022-GOOD-FRIDAY - Good Friday: the notice
        // states trading will be closed for all CFE products.
        (2022, 4, 15, Closed, T1, "CBOE-SU-2022-GOOD-FRIDAY"),
        // 2022-05-30 - T1 - CBOE-SU-2022-MEMORIAL - Memorial Day, 10:30 CT.
        (2022, 5, 30, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-MEMORIAL"),
        // 2022-06-20 - T1 - CBOE-SU-2022-JUNETEENTH - Juneteenth observed
        // Monday, 10:30 CT.
        (2022, 6, 20, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-JUNETEENTH"),
        // 2022-07-04 - T1 - CBOE-SU-2022-INDEPENDENCE - Independence Day, 10:30 CT.
        (2022, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-INDEPENDENCE"),
        // 2022-09-05 - T1 - CBOE-SU-2022-LABOR - Labor Day, 10:30 CT.
        (2022, 9, 5, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-LABOR"),
        // 2022-11-24 - T1 - CBOE-SU-2022-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2022, 11, 24, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2022-THANKSGIVING"),
        // 2022-11-25 - T1 - CBOE-SU-2022-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT.
        (2022, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2022-THANKSGIVING"),
        // 2022-12-26 - T1 - CBOE-SU-2022-CHRISTMAS - Christmas observed Monday:
        // the notice's table prints no Sunday-evening or Monday session, and
        // Friday 2022-12-23's normal 3:00/4:00 PM closes are printed beside it.
        (2022, 12, 26, Closed, T1, "CBOE-SU-2022-CHRISTMAS"),
        // 2023-01-02 - T1 - CBOE-SU-2023-NEW-YEAR - New Year's Day observed
        // Monday: the notice's table prints no Sunday-evening or Monday
        // session, and Friday 2022-12-30's normal closes are printed beside it.
        (2023, 1, 2, Closed, T1, "CBOE-SU-2023-NEW-YEAR"),
        // 2023-01-16 - T1 - CBOE-SU-2023-MLK - MLK Jr. Day, 10:30 CT.
        (2023, 1, 16, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-MLK"),
        // 2023-02-20 - T1 - CBOE-SU-2023-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2023, 2, 20, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-PRESIDENTS"),
        // 2023-04-07 - T1 - CBOE-SU-2023-GOOD-FRIDAY - Good Friday, 08:30 CT:
        // the notice's table stops the Thursday-evening leg at the regular
        // open and prints no regular session.
        (2023, 4, 7, early_close(8 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-GOOD-FRIDAY"),
        // 2023-05-29 - T1 - CBOE-SU-2023-MEMORIAL - Memorial Day, 10:30 CT.
        (2023, 5, 29, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-MEMORIAL"),
        // 2023-06-19 - T1 - CBOE-SU-2023-JUNETEENTH - Juneteenth, 10:30 CT.
        (2023, 6, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-JUNETEENTH"),
        // 2023-07-03 - T1 - CBOE-SU-2023-INDEPENDENCE - the eve before the
        // Tuesday holiday: the notice's table closes the Monday RTH at 12:15.
        (2023, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2023-INDEPENDENCE"),
        // 2023-07-04 - T1 - CBOE-SU-2023-INDEPENDENCE - Independence Day on a
        // Tuesday, 10:30 CT.
        (2023, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-INDEPENDENCE"),
        // 2023-09-04 - T1 - CBOE-SU-2023-LABOR - Labor Day, 10:30 CT.
        (2023, 9, 4, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-LABOR"),
        // 2023-11-23 - T1 - CBOE-SU-2023-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2023, 11, 23, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2023-THANKSGIVING"),
        // 2023-11-24 - T1 - CBOE-SU-2023-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT.
        (2023, 11, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2023-THANKSGIVING"),
        // 2023-12-25 - T1 - CBOE-SU-2023-CHRISTMAS - Christmas on a Monday: the
        // notice's table prints no Sunday-evening or Monday session.
        (2023, 12, 25, Closed, T1, "CBOE-SU-2023-CHRISTMAS"),
        // 2024-01-01 - T1 - CBOE-SU-2024-NEW-YEAR - New Year's Day on a Monday:
        // the notice's table prints no Sunday-evening or Monday session.
        (2024, 1, 1, Closed, T1, "CBOE-SU-2024-NEW-YEAR"),
        // 2024-01-15 - T1 - CBOE-SU-2024-MLK - MLK Jr. Day, 10:30 CT.
        (2024, 1, 15, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-MLK"),
        // 2024-02-19 - T1 - CBOE-SU-2024-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2024, 2, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-PRESIDENTS"),
        // 2024-03-29 - T1 - CBOE-SU-2024-GOOD-FRIDAY - Good Friday: the
        // notice's table prints no Friday session and its Trade Date row skips
        // the day, so no session belongs to it.
        (2024, 3, 29, Closed, T1, "CBOE-SU-2024-GOOD-FRIDAY"),
        // 2024-05-27 - T1 - CBOE-SU-2024-MEMORIAL - Memorial Day, 10:30 CT.
        (2024, 5, 27, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-MEMORIAL"),
        // 2024-06-19 - T1 - CBOE-SU-2024-JUNETEENTH - Juneteenth, 10:30 CT.
        (2024, 6, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-JUNETEENTH"),
        // 2024-07-03 - T1 - CBOE-SU-2024-INDEPENDENCE - the eve before the
        // Thursday holiday: the notice's table closes the Wednesday RTH at
        // 12:15 (trade date Wednesday, July 3, 2024).
        (2024, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2024-INDEPENDENCE"),
        // 2024-07-04 - T1 - CBOE-SU-2024-INDEPENDENCE - Independence Day on a
        // Thursday, 10:30 CT.
        (2024, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-INDEPENDENCE"),
        // 2024-09-02 - T1 - CBOE-SU-2024-LABOR - Labor Day, 10:30 CT.
        (2024, 9, 2, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-LABOR"),
        // 2024-11-28 - T1 - CBOE-SU-2024-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2024, 11, 28, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2024-THANKSGIVING"),
        // 2024-11-29 - T1 - CBOE-SU-2024-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT.
        (2024, 11, 29, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2024-THANKSGIVING"),
        // 2024-12-24 - T1 - CBOE-SU-2024-CHRISTMAS - Christmas Eve: the
        // notice's table closes the Tuesday RTH at 12:15 (trade date Tuesday,
        // December 24, 2024).
        (2024, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2024-CHRISTMAS"),
        // 2024-12-25 - T1 - CBOE-SU-2024-CHRISTMAS - Christmas Day on a
        // Wednesday: the notice's table prints no Tuesday-evening or Wednesday
        // session.
        (2024, 12, 25, Closed, T1, "CBOE-SU-2024-CHRISTMAS"),
        // 2025-01-01 - T1 - CBOE-SU-2025-NEW-YEAR - New Year's Day: the notice
        // prints the Tuesday 2024-12-31 close and the Thursday 2025-01-02
        // evening reopen, and no block at all for the holiday's own day.
        (2025, 1, 1, Closed, T1, "CBOE-SU-2025-NEW-YEAR"),
        // 2025-01-09 - T1 - CBOE-SU-2025-MOURNING - National Day of Mourning:
        // one extended block, 17:00 CT on 2025-01-08 to 08:30 CT, and no regular
        // session at all.
        (2025, 1, 9, ReplacementBlocks(&MOURNING_2025_01_09_BLOCKS), T1, "CBOE-SU-2025-MOURNING"),
        // 2025-01-20 - T1 - CBOE-SU-2025-MLK - MLK Jr. Day, 10:30 CT.
        (2025, 1, 20, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-MLK"),
        // 2025-02-17 - T1 - CBOE-SU-2025-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2025, 2, 17, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-PRESIDENTS"),
        // 2025-04-18 - T1 - CBOE-SU-2025-GOOD-FRIDAY - Good Friday: the notice
        // prints no Friday close and no Friday trade date, so the day is deleted
        // outright rather than clipped at the regular open.
        (2025, 4, 18, Closed, T1, "CBOE-SU-2025-GOOD-FRIDAY"),
        // 2025-05-26 - T1 - CBOE-SU-2025-MEMORIAL - Memorial Day, 10:30 CT.
        (2025, 5, 26, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-MEMORIAL"),
        // 2025-06-19 - T1 - CBOE-SU-2025-JUNETEENTH - Juneteenth, 10:30 CT.
        (2025, 6, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-JUNETEENTH"),
        // 2025-07-03 - T1 - CBOE-SU-2025-INDEPENDENCE - Independence Day
        // observed Friday 2025-07-04, so its own day is deleted and the half day
        // falls on the Thursday trade date.
        (2025, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2025-INDEPENDENCE"),
        // 2025-07-04 - T1 - CBOE-SU-2025-INDEPENDENCE - Independence Day: the
        // notice prints no block for that column and no trade date for it.
        (2025, 7, 4, Closed, T1, "CBOE-SU-2025-INDEPENDENCE"),
        // 2025-09-01 - T1 - CBOE-SU-2025-LABOR - Labor Day, 10:30 CT.
        (2025, 9, 1, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-LABOR"),
        // 2025-11-27 - T1 - CBOE-SU-2025-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2025, 11, 27, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-THANKSGIVING"),
        // 2025-11-28 - T1 - CBOE-SU-2025-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT; the Thursday-evening leg belongs to the 2025-11-27 row.
        (2025, 11, 28, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2025-THANKSGIVING"),
        // 2025-12-24 - T1 - CBOE-SU-2025-CHRISTMAS - Christmas Eve, 12:15 CT;
        // the missing Thursday-evening leg belongs to the 2025-12-25 row.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2025-CHRISTMAS"),
        // 2025-12-25 - T1 - CBOE-SU-2025-CHRISTMAS - Christmas Day: the notice
        // prints no RTH start or close and no ETH start for that column.
        (2025, 12, 25, Closed, T1, "CBOE-SU-2025-CHRISTMAS"),
        // 2026-01-01 - T1 - CBOE-HOURS-USFUT-2026 - New Year's Day: regular
        // `None`, and the only extended block printed is the Thursday-evening
        // leg of trade date 2026-01-02.
        (2026, 1, 1, Closed, T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-01-19 - T1 - CBOE-HOURS-USFUT-2026 - MLK Jr. Day, 10:30 CT.
        (2026, 1, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-02-16 - T1 - CBOE-HOURS-USFUT-2026 - Presidents' Day, 10:30 CT.
        (2026, 2, 16, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-04-03 - T1 - CBOE-HOURS-USFUT-2026 - Good Friday, 08:30 CT: the
        // overnight leg ends at the regular open and no regular session runs.
        (2026, 4, 3, early_close(8 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-05-25 - T1 - CBOE-HOURS-USFUT-2026 - Memorial Day, 10:30 CT.
        (2026, 5, 25, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-06-19 - T1 - CBOE-HOURS-USFUT-2026 - Juneteenth, 10:30 CT.
        (2026, 6, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-07-03 - T1 - CBOE-HOURS-USFUT-2026 - Independence Day observed, 10:30 CT.
        (2026, 7, 3, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-09-07 - T1 - CBOE-HOURS-USFUT-2026 - Labor Day, 10:30 CT.
        (2026, 9, 7, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-11-26 - T1 - CBOE-HOURS-USFUT-2026 - Thanksgiving Day, 10:30 CT.
        (2026, 11, 26, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-11-27 - T1 - CBOE-HOURS-USFUT-2026 - Thanksgiving half day, 12:15 CT.
        (2026, 11, 27, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-12-24 - T1 - CBOE-HOURS-USFUT-2026 - Christmas Eve, 12:15 CT;
        // the missing Thursday-evening leg belongs to the 2026-12-25 row.
        (2026, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-12-25 - T1 - CBOE-HOURS-USFUT-2026 - Christmas Day, both cells `None`.
        (2026, 12, 25, Closed, T1, "CBOE-HOURS-USFUT-2026"),
    ],
};
