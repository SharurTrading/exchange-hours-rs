// SPDX-License-Identifier: MIT-0

//! Toronto Stock Exchange (TSX) holiday rows, 2010-2026 across two audited
//! windows.
//!
//! Keyed by the crate's own venue-local trade date in `America/Toronto`. TSX
//! runs no overnight session, so an event date and its trade date are one
//! civil day and the conversion is the identity.
//!
//! The whole block is **T1**: the operator's own statements. 2017-2026 is read
//! from TMX Group's "Stock Market Holidays - Stock Markets Closed" calendar at
//! `tsx.com/en/trading/calendars-and-trading-hours/calendar` — one server-
//! rendered page carrying the current year's list in full and the next year's
//! from Q4, pinned by Wayback `id_` captures of its two paths (the 2018
//! relaunch path's first capture 2018-09-11 restates the complete 2017 list
//! in its archive section; the `/en/` path carries 2024-2025) plus the live
//! retrieval of 2026-09-28. **2010-2014** is read from the operator's own
//! per-holiday closure news releases and Holiday (Operating) Schedule
//! releases on `tmx.com` (found 2026-09-30 UTC by domain-wide CDX sweeps of
//! `tsx.com` and `tmx.com`; the all-time release sweep enumerated every
//! holiday release the archive holds): each release states Toronto Stock
//! Exchange closed, or open until 1:00 p.m. EST on a Christmas Eve, for an
//! unconditional named date. **2011-12-23..2012-01-02** keys on the 2011-12-12
//! *TMX Group Holiday Operating Schedule* release itself, recovered
//! 2026-10-03 UTC from the release series' own Mondo Visione verbatim public
//! mirror (the same series whose 2016 edition reads word-for-word against the
//! operator's CNW wire mirror). **2013-09-02..2016-12-31** keys on the same
//! release series' verbatim public wire mirrors (CNW/PR Newswire, retrieved
//! live 2026-10-02 UTC) and on two further operator pages: TMX Money's own
//! "Market Hours & Holiday" page as served 2014-01-09 printing the complete
//! 2014 list, and the `tsx.com` "Calendar & Events" page as served 2015-03-15
//! printing the complete 2015 and 2016 lists. All of this is recorded in
//! [`docs/evidence/tsx.md`](../../../../../docs/evidence/tsx.md).
//!
//! The Christmas Eve early closes are the operator's own sentences: the
//! 2018-2021 and 2024-2026 rows ride the calendar page's `1:00 PM (TSX/TSXV)`
//! footnote and sentence (the 1:30 PM half applies to the ALPHA/ALPHA X/DRK
//! book systems and is outside this venue's scope; the 2021 sentence was
//! printed `subject to Board Approval` in January 2021 and the 2022-01-28
//! capture witnesses the discharged state), the 2010 and 2012 rows ride
//! the news releases' `Open until 1:00 p.m. (EST)` — the same 13:00 Toronto
//! instant — and the 2013, 2014 and 2015 rows ride the wire mirrors' and
//! trading notice's identical prints. The U.S. holidays the calendar page
//! lists under a separate heading are footnoted as **special-settlement**
//! days for USD issues, not trading closures, so none of them is encoded
//! (LAW-SESSION-NOT-EXPIRY).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// TSX's stated Christmas Eve close, 1:00 PM Toronto time.
///
/// The calendar's footnote reads `* Closing at 1:00 PM (TSX/TSXV) and 1:30
/// (ALPHA/ALPHA X/DRK)`; this identity's scope is the Toronto Stock Exchange
/// cash-equity market, whose close is 13:00.
const HALF_DAY_13_00: u32 = 13 * 3_600;

/// TSX's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of the operator's statement at the document its id
/// names — a per-holiday news release or Holiday (Operating) Schedule
/// release on `tmx.com` for 2010-2013 (the 2011 year-end edition recovered
/// from the series' Mondo Visione verbatim mirror), their verbatim CNW/PR
/// Newswire wire mirrors for 2013-2016's year-end and single-holiday
/// releases, TMX Money's own market-hours page for the 2014 list, an archived
/// `tsx.com` "Calendar & Events" state for the 2015-2016 lists, or one
/// archived/live state of the "Stock Markets Closed" calendar page for
/// 2017-2026. A date inside a window with no row is audited normal: the
/// release practice printed one notice per market closure and the archive
/// sweep enumerated every one, the year-end schedules print the year-end
/// arrangement in full (the 2011 edition's timetable closes Friday,
/// December 23, 2011 at the regular 4:00 p.m. EST, so no earlier row exists
/// that week), the 2014 and 2015-2016 pages print the complete year's list,
/// and the year-end schedules print `Open` on December 31.
// Evidence: docs/evidence/tsx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [
        (2010, 1, 1) ..= (2016, 12, 31),
        (2017, 1, 1) ..= (2026, 12, 31),
    ],
    rows: [
        // 2010-01-01 - T1 - TSX-REL-2009-12-02 - New Year's Day: the Holiday
        // Market Operating Schedule's TSX/TSXV table prints `Friday, January
        // 1, 2010 Closed`.
        (2010, 1, 1, Closed, T1, "TSX-REL-2009-12-02"),
        // 2010-02-15 - T1 - TSX-REL-2010-02-08 - Family Day: `Toronto Stock
        // Exchange and TSX Venture Exchange will be closed for Family Day on
        // Monday February 15, 2010`.
        (2010, 2, 15, Closed, T1, "TSX-REL-2010-02-08"),
        // 2010-04-02 - T1 - TSX-REL-2010-03-26 - Good Friday.
        (2010, 4, 2, Closed, T1, "TSX-REL-2010-03-26"),
        // 2010-05-24 - T1 - TSX-REL-2010-05-17 - Victoria Day.
        (2010, 5, 24, Closed, T1, "TSX-REL-2010-05-17"),
        // 2010-08-02 - T1 - TSX-REL-2010-07-27 - Civic Holiday.
        (2010, 8, 2, Closed, T1, "TSX-REL-2010-07-27"),
        // 2010-09-06 - T1 - TSX-REL-2010-09-01 - Labour Day.
        (2010, 9, 6, Closed, T1, "TSX-REL-2010-09-01"),
        // 2010-10-11 - T1 - TSX-REL-2010-10-01 - Thanksgiving.
        (2010, 10, 11, Closed, T1, "TSX-REL-2010-10-01"),
        // 2010-12-24 - T1 - TSX-REL-2010-11-08 - the Holiday Operating
        // Schedule prints TSX/TSXV `Open until 1:00 p.m. (EST)`.
        (2010, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-REL-2010-11-08"),
        // 2010-12-27 - T1 - TSX-REL-2010-11-08 - `(In lieu of Christmas Day)
        // Closed`.
        (2010, 12, 27, Closed, T1, "TSX-REL-2010-11-08"),
        // 2010-12-28 - T1 - TSX-REL-2010-11-08 - `(In lieu of Boxing Day)
        // Closed`.
        (2010, 12, 28, Closed, T1, "TSX-REL-2010-11-08"),
        // 2011-01-03 - T1 - TSX-REL-2010-11-08 - `(In lieu of New Year's Day)
        // Closed`.
        (2011, 1, 3, Closed, T1, "TSX-REL-2010-11-08"),
        // 2011-02-21 - T1 - TMX-REL-2011-02-16 - Family Day: `Toronto Stock
        // Exchange, TSX Venture Exchange and Montreal Exchange will be closed
        // for the Family Day holiday on Monday, February 21`.
        (2011, 2, 21, Closed, T1, "TMX-REL-2011-02-16"),
        // 2011-04-22 - T1 - TMX-REL-2011-04-14 - Good Friday.
        (2011, 4, 22, Closed, T1, "TMX-REL-2011-04-14"),
        // 2011-05-23 - T1 - TMX-REL-2011-05-18 - Victoria Day.
        (2011, 5, 23, Closed, T1, "TMX-REL-2011-05-18"),
        // 2011-07-01 - T1 - TMX-REL-2011-06-22 - Canada Day.
        (2011, 7, 1, Closed, T1, "TMX-REL-2011-06-22"),
        // 2011-08-01 - T1 - TMX-REL-2011-07-25 - Civic Holiday.
        (2011, 8, 1, Closed, T1, "TMX-REL-2011-07-25"),
        // 2011-09-05 - T1 - TMX-REL-2011-08-30 - Labour Day.
        (2011, 9, 5, Closed, T1, "TMX-REL-2011-08-30"),
        // 2011-10-10 - T1 - TMX-REL-2011-09-30 - Thanksgiving.
        (2011, 10, 10, Closed, T1, "TMX-REL-2011-09-30"),
        // 2011-12-26 - T1 - TMX-REL-2011-12-12 - the year-end Holiday
        // Operating Schedule (recovered 2026-10-03 UTC from the series'
        // Mondo Visione verbatim mirror): the TSX/TSXV/TMX Select table
        // prints `(In lieu of Christmas Day) Closed`.
        (2011, 12, 26, Closed, T1, "TMX-REL-2011-12-12"),
        // 2011-12-27 - T1 - TMX-REL-2011-12-12 - `(In lieu of Boxing Day)
        // Closed`.
        (2011, 12, 27, Closed, T1, "TMX-REL-2011-12-12"),
        // 2012-01-02 - T1 - TMX-REL-2011-12-12 - `(In lieu of New Year's
        // Day) Closed`; the timetable lists no other date, so the window
        // runs on into the 2012 releases.
        (2012, 1, 2, Closed, T1, "TMX-REL-2011-12-12"),
        // 2012-02-20 - T1 - TMX-REL-2012-02-13 - Family Day.
        (2012, 2, 20, Closed, T1, "TMX-REL-2012-02-13"),
        // 2012-04-06 - T1 - TMX-REL-2012-03-30 - Good Friday.
        (2012, 4, 6, Closed, T1, "TMX-REL-2012-03-30"),
        // 2012-05-21 - T1 - TMX-REL-2012-05-14 - Victoria Day.
        (2012, 5, 21, Closed, T1, "TMX-REL-2012-05-14"),
        // 2012-07-02 - T1 - TMX-REL-2012-06-22 - Canada Day: the release
        // states `closed on Monday, July 2, 2012, for the Canada Day holiday`
        // (1 July fell on a Sunday).
        (2012, 7, 2, Closed, T1, "TMX-REL-2012-06-22"),
        // 2012-08-06 - T1 - TMX-REL-2012-07-31 - Civic Holiday.
        (2012, 8, 6, Closed, T1, "TMX-REL-2012-07-31"),
        // 2012-09-03 - T1 - TMX-REL-2012-08-24 - Labour Day.
        (2012, 9, 3, Closed, T1, "TMX-REL-2012-08-24"),
        // 2012-10-08 - T1 - TMX-REL-2012-09-28 - Thanksgiving.
        (2012, 10, 8, Closed, T1, "TMX-REL-2012-09-28"),
        // 2012-12-24 - T1 - TMX-REL-2012-11-28 - the Holiday Operating
        // Schedule prints TSX/TSXV `Open until 1:00 p.m. (EST)`.
        (2012, 12, 24, early_close(HALF_DAY_13_00), T1, "TMX-REL-2012-11-28"),
        // 2012-12-25 - T1 - TMX-REL-2012-11-28 - Christmas Day.
        (2012, 12, 25, Closed, T1, "TMX-REL-2012-11-28"),
        // 2012-12-26 - T1 - TMX-REL-2012-11-28 - Boxing Day; the schedule's
        // TSX row also prints `Monday, December 31, 2012 Open`, so the window
        // runs to the year's end.
        (2012, 12, 26, Closed, T1, "TMX-REL-2012-11-28"),
        // 2013-01-01 - T1 - TMX-REL-2012-11-28 - New Year's Day.
        (2013, 1, 1, Closed, T1, "TMX-REL-2012-11-28"),
        // 2013-02-18 - T1 - TMX-REL-2013-01-30 - Family Day.
        (2013, 2, 18, Closed, T1, "TMX-REL-2013-01-30"),
        // 2013-03-29 - T1 - TMX-REL-2013-03-20 - Good Friday.
        (2013, 3, 29, Closed, T1, "TMX-REL-2013-03-20"),
        // 2013-05-20 - T1 - TMX-REL-2013-05-13 - Victoria Day.
        (2013, 5, 20, Closed, T1, "TMX-REL-2013-05-13"),
        // 2013-07-01 - T1 - TMX-REL-2013-06-24 - Canada Day.
        (2013, 7, 1, Closed, T1, "TMX-REL-2013-06-24"),
        // 2013-08-05 - T1 - TMX-REL-2013-07-26 - Civic Holiday. The archive's
        // last bulk sweep of the 2013 release tree is 2013-08-18 (it caught
        // every release through the Civic notice); the closures after it key
        // on the release series' wire mirrors.
        (2013, 8, 5, Closed, T1, "TMX-REL-2013-07-26"),
        // 2013-09-02 - T1 - TMX-REL-2013-08-23 - Labour Day: the CNW/PRN wire
        // mirror of the operator's release, `will be closed on Monday,
        // September 2, 2013 for the Labour Day holiday`.
        (2013, 9, 2, Closed, T1, "TMX-REL-2013-08-23"),
        // 2013-10-14 - T1 - TMX-REL-2013-10-07 - Thanksgiving: the CNW wire
        // mirror, `will be closed on Monday, October 14, 2013 for the
        // Thanksgiving holiday`.
        (2013, 10, 14, Closed, T1, "TMX-REL-2013-10-07"),
        // 2013-12-24 - T1 - TMX-REL-2013-12-03 - the year-end Holiday
        // Operating Schedule prints TSX `Open until 1:00 p.m. (EST)`; the
        // operator's trading notice 2013-03-07 prints the same 13:00 close
        // with the schedule table (`CCP Determination 13:00:00`).
        (2013, 12, 24, early_close(HALF_DAY_13_00), T1, "TMX-REL-2013-12-03"),
        // 2013-12-25 - T1 - TMX-REL-2013-12-03 - Christmas Day.
        (2013, 12, 25, Closed, T1, "TMX-REL-2013-12-03"),
        // 2013-12-26 - T1 - TMX-REL-2013-12-03 - Boxing Day; the schedule's
        // TSX row also prints `Tuesday, December 31, 2013 Open`, so the
        // window runs on.
        (2013, 12, 26, Closed, T1, "TMX-REL-2013-12-03"),
        // 2014-01-01 - T1 - TMX-REL-2013-12-03 - New Year's Day.
        (2014, 1, 1, Closed, T1, "TMX-REL-2013-12-03"),
        // 2014-02-17 - T1 - TMX-REL-2014-02-07 - Family Day.
        (2014, 2, 17, Closed, T1, "TMX-REL-2014-02-07"),
        // 2014-04-18 - T1 - TMX-REL-2014-04-09 - Good Friday: `will be closed
        // on Friday, April 18 for Good Friday`.
        (2014, 4, 18, Closed, T1, "TMX-REL-2014-04-09"),
        // 2014-05-19 - T1 - TMX-REL-2014-05-13 - Victoria Day.
        (2014, 5, 19, Closed, T1, "TMX-REL-2014-05-13"),
        // 2014-07-01 - T1 - TMX-REL-2014-06-23 - Canada Day: the last release
        // the tmx.com archive captured; the closures after it key on the
        // operator's TMX Money page and the wire mirrors.
        (2014, 7, 1, Closed, T1, "TMX-REL-2014-06-23"),
        // 2014-08-04 - T1 - TSX-MH-2014-01-09 - Civic Holiday: TMX Money's own
        // "Stock Market Hours & Holiday" page prints the complete 2014 list;
        // the operator's 2014-07-28 release (CNW mirror) prints the same
        // closure.
        (2014, 8, 4, Closed, T1, "TSX-MH-2014-01-09"),
        // 2014-09-01 - T1 - TSX-MH-2014-01-09 - Labour Day; the operator's
        // 2014-08-25 release (PRN mirror) prints the same closure.
        (2014, 9, 1, Closed, T1, "TSX-MH-2014-01-09"),
        // 2014-10-13 - T1 - TSX-MH-2014-01-09 - Thanksgiving Day; the
        // operator's 2014-09-23 release (CNW mirror) prints the same closure.
        (2014, 10, 13, Closed, T1, "TSX-MH-2014-01-09"),
        // 2014-12-24 - T1 - TMX-REL-2014-12-02 - the year-end Holiday
        // Operating Schedule prints TSX `Open until 1:00 p.m. (EST)` — the
        // eve line the January page state predates.
        (2014, 12, 24, early_close(HALF_DAY_13_00), T1, "TMX-REL-2014-12-02"),
        // 2014-12-25 - T1 - TMX-REL-2014-12-02 - Christmas Day.
        (2014, 12, 25, Closed, T1, "TMX-REL-2014-12-02"),
        // 2014-12-26 - T1 - TMX-REL-2014-12-02 - Boxing Day; the schedule's
        // TSX row also prints `Wednesday, December 31, 2014 Open`, so the
        // window runs on.
        (2014, 12, 26, Closed, T1, "TMX-REL-2014-12-02"),
        // 2015-01-01 - T1 - TSX-CAL-2015-03-15 - New Year's Day: the
        // `tsx.com` "Calendar & Events" page as served 2015-03-15 prints the
        // complete 2015 list; the 2014 year-end schedule keys the same date.
        (2015, 1, 1, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-02-16 - T1 - TSX-CAL-2015-03-15 - Family Day.
        (2015, 2, 16, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-04-03 - T1 - TSX-CAL-2015-03-15 - Good Friday.
        (2015, 4, 3, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-05-18 - T1 - TSX-CAL-2015-03-15 - Victoria Day.
        (2015, 5, 18, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-07-01 - T1 - TSX-CAL-2015-03-15 - Canada Day.
        (2015, 7, 1, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-08-03 - T1 - TSX-CAL-2015-03-15 - Civic Holiday.
        (2015, 8, 3, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-09-07 - T1 - TSX-CAL-2015-03-15 - Labour Day.
        (2015, 9, 7, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-10-12 - T1 - TSX-CAL-2015-03-15 - Thanksgiving Day.
        (2015, 10, 12, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2015-12-24 - T1 - TMX-REL-2015-12-01 - the year-end Holiday
        // Operating Schedule prints TSX `Open until 1:00 p.m. (EST)` — the
        // eve line the March page state predates.
        (2015, 12, 24, early_close(HALF_DAY_13_00), T1, "TMX-REL-2015-12-01"),
        // 2015-12-25 - T1 - TMX-REL-2015-12-01 - Christmas Day.
        (2015, 12, 25, Closed, T1, "TMX-REL-2015-12-01"),
        // 2015-12-28 - T1 - TMX-REL-2015-12-01 - In lieu of Boxing Day (the
        // Saturday); the schedule's TSX row also prints `Thursday, December
        // 31, 2015 Open`, so the window runs on.
        (2015, 12, 28, Closed, T1, "TMX-REL-2015-12-01"),
        // 2016-01-01 - T1 - TSX-CAL-2015-03-15 - New Year's Day: the same
        // page state prints the complete 2016 list beside the 2015 one.
        (2016, 1, 1, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-02-15 - T1 - TSX-CAL-2015-03-15 - Family Day.
        (2016, 2, 15, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-03-25 - T1 - TSX-CAL-2015-03-15 - Good Friday.
        (2016, 3, 25, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-05-23 - T1 - TSX-CAL-2015-03-15 - Victoria Day.
        (2016, 5, 23, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-07-01 - T1 - TSX-CAL-2015-03-15 - Canada Day.
        (2016, 7, 1, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-08-01 - T1 - TSX-CAL-2015-03-15 - Civic Holiday.
        (2016, 8, 1, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-09-05 - T1 - TSX-CAL-2015-03-15 - Labour Day.
        (2016, 9, 5, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-10-10 - T1 - TSX-CAL-2015-03-15 - Thanksgiving Day.
        (2016, 10, 10, Closed, T1, "TSX-CAL-2015-03-15"),
        // 2016-12-26 - T1 - TMX-REL-2016-11-30 - In lieu of Christmas Day
        // (the Sunday); the year-end schedule's TSX table prints no other
        // December date, so December 23 and 30 answer as ordinary trading
        // days.
        (2016, 12, 26, Closed, T1, "TMX-REL-2016-11-30"),
        // 2016-12-27 - T1 - TMX-REL-2016-11-30 - In lieu of Boxing Day.
        (2016, 12, 27, Closed, T1, "TMX-REL-2016-11-30"),
        // 2017-01-02 - T1 - TSX-CAL-2018-09-11 - New Year's Day (in lieu):
        // `Monday, January 2, 2017 * in lieu of New Years Day, Sunday January 1`.
        (2017, 1, 2, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-02-20 - T1 - TSX-CAL-2018-09-11 - Family Day.
        (2017, 2, 20, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-04-14 - T1 - TSX-CAL-2018-09-11 - Good Friday.
        (2017, 4, 14, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-05-22 - T1 - TSX-CAL-2018-09-11 - Victoria Day.
        (2017, 5, 22, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-07-03 - T1 - TSX-CAL-2018-09-11 - Canada Day (in lieu of the
        // Saturday).
        (2017, 7, 3, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-08-07 - T1 - TSX-CAL-2018-09-11 - Civic Holiday.
        (2017, 8, 7, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-09-04 - T1 - TSX-CAL-2018-09-11 - Labour Day.
        (2017, 9, 4, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-10-09 - T1 - TSX-CAL-2018-09-11 - Thanksgiving Day.
        (2017, 10, 9, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-12-25 - T1 - TSX-CAL-2018-09-11 - Christmas Day.
        (2017, 12, 25, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-12-26 - T1 - TSX-CAL-2018-09-11 - Boxing Day; the 2017 list
        // prints no Christmas Eve line.
        (2017, 12, 26, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-01-01 - T1 - TSX-CAL-2018-09-11 - New Year's Day.
        (2018, 1, 1, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-02-19 - T1 - TSX-CAL-2018-09-11 - Family Day.
        (2018, 2, 19, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-03-30 - T1 - TSX-CAL-2018-09-11 - Good Friday.
        (2018, 3, 30, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-05-21 - T1 - TSX-CAL-2018-09-11 - Victoria Day.
        (2018, 5, 21, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-07-02 - T1 - TSX-CAL-2018-09-11 - Canada Day (in lieu of the
        // Sunday).
        (2018, 7, 2, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-08-06 - T1 - TSX-CAL-2018-09-11 - Civic Holiday.
        (2018, 8, 6, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-09-03 - T1 - TSX-CAL-2018-09-11 - Labour Day.
        (2018, 9, 3, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-10-08 - T1 - TSX-CAL-2018-09-11 - Thanksgiving Day.
        (2018, 10, 8, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-12-24 - T1 - TSX-CAL-2018-09-11 - the page states `Markets
        // will close at 1:00 PM on December 24th, 2018.`; corroborated by the
        // 2019-08-20 capture's archive section.
        (2018, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2018-09-11"),
        // 2018-12-25 - T1 - TSX-CAL-2018-09-11 - Christmas Day.
        (2018, 12, 25, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-12-26 - T1 - TSX-CAL-2018-09-11 - Boxing Day.
        (2018, 12, 26, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2019-01-01 - T1 - TSX-CAL-2019-08-20 - New Year's Day.
        (2019, 1, 1, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-02-18 - T1 - TSX-CAL-2019-08-20 - Family Day.
        (2019, 2, 18, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-04-19 - T1 - TSX-CAL-2019-08-20 - Good Friday.
        (2019, 4, 19, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-05-20 - T1 - TSX-CAL-2019-08-20 - Victoria Day.
        (2019, 5, 20, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-07-01 - T1 - TSX-CAL-2019-08-20 - Canada Day.
        (2019, 7, 1, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-08-05 - T1 - TSX-CAL-2019-08-20 - Civic Holiday.
        (2019, 8, 5, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-09-02 - T1 - TSX-CAL-2019-08-20 - Labour Day.
        (2019, 9, 2, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-10-14 - T1 - TSX-CAL-2019-08-20 - Thanksgiving Day.
        (2019, 10, 14, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-12-24 - T1 - TSX-CAL-2019-08-20 - `Markets will close at
        // 1:00 PM on December 24th, 2019.`; corroborated by the 2020-03-29
        // capture.
        (2019, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2019-08-20"),
        // 2019-12-25 - T1 - TSX-CAL-2019-08-20 - Christmas Day.
        (2019, 12, 25, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-12-26 - T1 - TSX-CAL-2019-08-20 - Boxing Day.
        (2019, 12, 26, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2020-01-01 - T1 - TSX-CAL-2020-03-29 - New Year's Day.
        (2020, 1, 1, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-02-17 - T1 - TSX-CAL-2020-03-29 - Family Day.
        (2020, 2, 17, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-04-10 - T1 - TSX-CAL-2020-03-29 - Good Friday.
        (2020, 4, 10, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-05-18 - T1 - TSX-CAL-2020-03-29 - Victoria Day.
        (2020, 5, 18, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-07-01 - T1 - TSX-CAL-2020-03-29 - Canada Day.
        (2020, 7, 1, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-08-03 - T1 - TSX-CAL-2020-03-29 - Civic Holiday.
        (2020, 8, 3, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-09-07 - T1 - TSX-CAL-2020-03-29 - Labour Day.
        (2020, 9, 7, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-10-12 - T1 - TSX-CAL-2020-03-29 - Thanksgiving Day.
        (2020, 10, 12, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-12-24 - T1 - TSX-CAL-2020-03-29 - `Markets will close at
        // 1:00 PM on December 24th, 2020.`; restated by the 2021-01-25
        // capture.
        (2020, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2020-03-29"),
        // 2020-12-25 - T1 - TSX-CAL-2020-03-29 - Christmas Day.
        (2020, 12, 25, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-12-28 - T1 - TSX-CAL-2020-03-29 - In Lieu of Boxing Day.
        (2020, 12, 28, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2021-01-01 - T1 - TSX-CAL-2021-01-25 - New Year's Day.
        (2021, 1, 1, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-02-15 - T1 - TSX-CAL-2021-01-25 - Family Day.
        (2021, 2, 15, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-04-02 - T1 - TSX-CAL-2021-01-25 - Good Friday.
        (2021, 4, 2, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-05-24 - T1 - TSX-CAL-2021-01-25 - Victoria Day.
        (2021, 5, 24, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-07-01 - T1 - TSX-CAL-2021-01-25 - Canada Day.
        (2021, 7, 1, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-08-02 - T1 - TSX-CAL-2021-01-25 - Civic Holiday.
        (2021, 8, 2, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-09-06 - T1 - TSX-CAL-2021-01-25 - Labour Day.
        (2021, 9, 6, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-10-11 - T1 - TSX-CAL-2021-01-25 - Thanksgiving Day.
        (2021, 10, 11, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-12-24 - T1 - TSX-CAL-2022-01-28 - Christmas Eve: the January
        // capture printed the half day `subject to Board Approval`, and this
        // later state of the page witnesses the discharged condition —
        // `TSX and TSX Venture Exchange will close early at 1:00 p.m.`
        (2021, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2022-01-28"),
        // 2021-12-27 - T1 - TSX-CAL-2021-01-25 - In Lieu of Christmas Day.
        (2021, 12, 27, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-12-28 - T1 - TSX-CAL-2021-01-25 - In Lieu of Boxing Day.
        (2021, 12, 28, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2022-01-03 - T1 - TSX-CAL-2022-01-28 - In Lieu of New Year's Day.
        (2022, 1, 3, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-02-21 - T1 - TSX-CAL-2022-01-28 - Family Day.
        (2022, 2, 21, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-04-15 - T1 - TSX-CAL-2022-01-28 - Good Friday.
        (2022, 4, 15, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-05-23 - T1 - TSX-CAL-2022-01-28 - Victoria Day.
        (2022, 5, 23, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-07-01 - T1 - TSX-CAL-2022-01-28 - Canada Day.
        (2022, 7, 1, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-08-01 - T1 - TSX-CAL-2022-01-28 - Civic Holiday.
        (2022, 8, 1, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-09-05 - T1 - TSX-CAL-2022-01-28 - Labour Day.
        (2022, 9, 5, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-10-10 - T1 - TSX-CAL-2022-01-28 - Thanksgiving Day.
        (2022, 10, 10, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-12-26 - T1 - TSX-CAL-2022-01-28 - In Lieu of Christmas Day;
        // 24 December 2022 was a Saturday and the list prints no Christmas
        // Eve line.
        (2022, 12, 26, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-12-27 - T1 - TSX-CAL-2022-01-28 - In Lieu of Boxing Day.
        (2022, 12, 27, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2023-01-02 - T1 - TSX-CAL-2023-01-16 - In Lieu of New Year's Day.
        (2023, 1, 2, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-02-20 - T1 - TSX-CAL-2023-01-16 - Family Day.
        (2023, 2, 20, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-04-07 - T1 - TSX-CAL-2023-01-16 - Good Friday.
        (2023, 4, 7, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-05-22 - T1 - TSX-CAL-2023-01-16 - Victoria Day.
        (2023, 5, 22, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-07-03 - T1 - TSX-CAL-2023-01-16 - Canada Day.
        (2023, 7, 3, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-08-07 - T1 - TSX-CAL-2023-01-16 - Civic Holiday.
        (2023, 8, 7, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-09-04 - T1 - TSX-CAL-2023-01-16 - Labour Day.
        (2023, 9, 4, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-10-09 - T1 - TSX-CAL-2023-01-16 - Thanksgiving Day.
        (2023, 10, 9, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-12-25 - T1 - TSX-CAL-2023-01-16 - Christmas Day; 24 December
        // 2023 was a Sunday and the list prints no Christmas Eve line.
        (2023, 12, 25, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-12-26 - T1 - TSX-CAL-2023-01-16 - Boxing Day.
        (2023, 12, 26, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2024-01-01 - T1 - TSX-CAL-2024-12-17 - New Year's Day; the July 2024
        // state prints the same closure under the label "In Lieu of New Year's
        // Day".
        (2024, 1, 1, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-02-19 - T1 - TSX-CAL-2024-12-17 - Family Day.
        (2024, 2, 19, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-03-29 - T1 - TSX-CAL-2024-12-17 - Good Friday.
        (2024, 3, 29, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-05-20 - T1 - TSX-CAL-2024-12-17 - Victoria Day.
        (2024, 5, 20, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-07-01 - T1 - TSX-CAL-2024-12-17 - Canada Day.
        (2024, 7, 1, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-08-05 - T1 - TSX-CAL-2024-12-17 - Civic Holiday.
        (2024, 8, 5, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-09-02 - T1 - TSX-CAL-2024-12-17 - Labour Day.
        (2024, 9, 2, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-10-14 - T1 - TSX-CAL-2024-12-17 - Thanksgiving Day.
        (2024, 10, 14, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-12-24 - T1 - TSX-CAL-2024-12-17 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV); the operator added this line to the 2024 list
        // between the July and December states of the page.
        (2024, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2024-12-17"),
        // 2024-12-25 - T1 - TSX-CAL-2024-12-17 - Christmas Day.
        (2024, 12, 25, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-12-26 - T1 - TSX-CAL-2024-12-17 - Boxing Day.
        (2024, 12, 26, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2025-01-01 - T1 - TSX-CAL-2026-09-28 - New Year's Day.
        (2025, 1, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-02-17 - T1 - TSX-CAL-2026-09-28 - Family Day.
        (2025, 2, 17, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-04-18 - T1 - TSX-CAL-2026-09-28 - Good Friday.
        (2025, 4, 18, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-05-19 - T1 - TSX-CAL-2026-09-28 - Victoria Day.
        (2025, 5, 19, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-07-01 - T1 - TSX-CAL-2026-09-28 - Canada Day.
        (2025, 7, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-08-04 - T1 - TSX-CAL-2026-09-28 - Civic Holiday.
        (2025, 8, 4, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-09-01 - T1 - TSX-CAL-2026-09-28 - Labour Day.
        (2025, 9, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-10-13 - T1 - TSX-CAL-2026-09-28 - Thanksgiving Day.
        (2025, 10, 13, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-12-24 - T1 - TSX-CAL-2026-09-28 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV).
        (2025, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2026-09-28"),
        // 2025-12-25 - T1 - TSX-CAL-2026-09-28 - Christmas Day.
        (2025, 12, 25, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-12-26 - T1 - TSX-CAL-2026-09-28 - Boxing Day.
        (2025, 12, 26, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-01-01 - T1 - TSX-CAL-2026-09-28 - New Year's Day.
        (2026, 1, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-02-16 - T1 - TSX-CAL-2026-09-28 - Family Day.
        (2026, 2, 16, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-04-03 - T1 - TSX-CAL-2026-09-28 - Good Friday.
        (2026, 4, 3, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-05-18 - T1 - TSX-CAL-2026-09-28 - Victoria Day.
        (2026, 5, 18, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-07-01 - T1 - TSX-CAL-2026-09-28 - Canada Day.
        (2026, 7, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-08-03 - T1 - TSX-CAL-2026-09-28 - Civic Holiday.
        (2026, 8, 3, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-09-07 - T1 - TSX-CAL-2026-09-28 - Labour Day.
        (2026, 9, 7, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-10-12 - T1 - TSX-CAL-2026-09-28 - Thanksgiving Day.
        (2026, 10, 12, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-12-24 - T1 - TSX-CAL-2026-09-28 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV).
        (2026, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2026-09-28"),
        // 2026-12-25 - T1 - TSX-CAL-2026-09-28 - Christmas Day.
        (2026, 12, 25, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-12-28 - T1 - TSX-CAL-2026-09-28 - In Lieu of Boxing Day.
        (2026, 12, 28, Closed, T1, "TSX-CAL-2026-09-28"),
    ],
};
