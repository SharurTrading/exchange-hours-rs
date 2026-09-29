// SPDX-License-Identifier: MIT-0

//! Euronext Paris holiday rows, 2010-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Paris`. Euronext
//! Paris runs no overnight session, so an event date and its trade date are one
//! civil day and the conversion is the identity.
//!
//! The whole block is **T1**: the operator's own holiday calendar, read across
//! four generations — the per-year NYSE Euronext press releases and cash-
//! market notice of 2010-2012 (the operator's `Trading Calendar Archives`
//! page lists each year's document with its own URL), the 2012 Info-Flash for
//! 2013, the NYSE Euronext `trading-hours-and-holidays` and `nyse-euronext-
//! trading-calendar` pages (2014-2015, one calendar for every Euronext cash
//! market), the `en/trading-calendars-hours` page (2016-2019, one calendar
//! per year) and the `live.euronext.com` `trading-hours-holidays` page
//! (2019-2026, per-market tables whose Paris is the last column), with the
//! per-year end-of-year appendix XLSX beside the newest two. Twenty archived
//! page states and five operator documents pin the years 2010-2025 (the
//! evidence file's `### Documents` table); the live retrieval corroborates
//! the newest rows.
//!
//! December half days are **early closes at the operator's stated instant**,
//! which moved across the eras: the 2010 appendix's grid prints Trading to
//! 13:55, a 14:00 closing uncross and TAL to 14:05 (the same shape the 2025
//! appendix prints), so the 2010 eves end at 14:05; the 2011 press release
//! states its eves `close at 5.35 pm CET` (17:35, the closing-auction end);
//! the 2012 notice states 14:00; the 2013 Info-Flash states 14:00 while the
//! operator's 2014-01-12 page restates 14:05, so 2013 holds the narrowest
//! sourced value, 14:00; 2014, 2015 and 2018-2021 print `all instruments
//! closing by 14:05 CET`; and the 2024 and 2025 end-of-year appendices print
//! the same 14:05 grid as 2010. 2016 and 2017 print `close at the usual
//! times` for their substitute December Fridays — no half day. 2022 and 2023
//! move to the per-market table, where the 23/30 December 2022 and 22/29
//! December 2023 half days are **Dublin's** substitutes and the Paris column
//! prints `Full Day Trading`; Paris ships no row for them. The 2026 eves
//! print `**Half Trading Day` with the hours "To be announced" —
//! `Unsourced`. This table encodes **Paris only**; the other markets' columns
//! are out of this identity's scope, and the follow-up for them is recorded
//! in [`docs/evidence/euronext_paris.md`](../../../../../docs/evidence/euronext_paris.md).

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// Euronext Paris's stated half-day envelope close, 14:05 CET.
///
/// For the 2024 and 2025 end-of-year appendices and the 2010 December
/// appendix, the printed `14:00  -  14:05` TAL phase of every Paris equity
/// segment ends the availability envelope; for 2014, 2015 and 2018-2021 the
/// operator's own calendar pages state `all instruments closing by 14:05 CET`
/// for the Amsterdam, Brussels, Lisbon and Paris cash markets. In both
/// wordings 14:05 is the day's final close of the availability envelope.
const HALF_DAY_14_05: u32 = 14 * 3_600 + 5 * 60;

/// Euronext Paris's stated half-day close, 14:00 CET.
///
/// The 2012 notice states `the markets will close at 14:00 CET` for its
/// December eves, and the 2013 Info-Flash states the same instant while the
/// operator's 2014-01-12 page restates 14:05 for 2013 — the narrowest sourced
/// value across that undated span is 14:00.
const HALF_DAY_14_00: u32 = 14 * 3_600;

/// Euronext Paris's stated 2011 half-day close, 17:35 CET.
///
/// The 2011 press release states `trading on the Cash markets will close at
/// 5.35 pm CET` for its December eves — the closing-auction end of the legacy
/// grid, five minutes ahead of the 17:40 Trading-at-Last envelope end.
const HALF_DAY_17_35: u32 = 17 * 3_600 + 35 * 60;

/// Euronext Paris's built-in holiday rows and the window they were audited
/// over.
///
/// Every row is the Paris cell (or, before the per-market tables, the cash-
/// markets line) of one line of the operator's calendar at the capture its
/// document id names — the earliest archived state that prints the row.
/// A date inside the window with no row is audited normal; the two 2026 eves
/// carry `Unsourced` rows because their half-day instants are announced but
/// not yet published ("To be announced" in the operator's own table).
// Evidence: docs/evidence/euronext_paris.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - EURONEXT-PR-2010 - New Year's Day: `Friday 1
        // January 2010 (New Year's Day)`.
        (2010, 1, 1, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-04-02 - T1 - EURONEXT-PR-2010 - Good Friday: `Friday 2 April
        // 2010 (Good Friday)`.
        (2010, 4, 2, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-04-05 - T1 - EURONEXT-PR-2010 - Easter Monday: `Monday 5 April
        // 2010 (Easter Monday)`.
        (2010, 4, 5, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-12-24 - T1 - EURONEXT-EOY-2010 - Christmas Eve half day: the
        // appendix's Paris grid prints Trading to 13:55, a 14:00 closing
        // uncross and TAL 14:00-14:05; the press release names the same
        // arrangement `1.00 pm GMT (2.00 pm CET)`.
        (2010, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2010"),
        // 2010-12-31 - T1 - EURONEXT-EOY-2010 - New Year's Eve half day: the
        // same appendix grid.
        (2010, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2010"),
        // 2011-04-22 - T1 - EURONEXT-PR-2011 - Good Friday: `Friday 22 April
        // 2011 (Good Friday)`.
        (2011, 4, 22, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-04-25 - T1 - EURONEXT-PR-2011 - Easter Monday: `Monday 25
        // April 2011 (Easter Monday)`.
        (2011, 4, 25, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-12-23 - T1 - EURONEXT-PR-2011 - Christmas Eve half day:
        // `trading on the Cash markets will close at 5.35 pm CET` — the
        // closing-auction end, five minutes ahead of the 17:40 envelope end.
        (2011, 12, 23, early_close(HALF_DAY_17_35), T1, "EURONEXT-PR-2011"),
        // 2011-12-26 - T1 - EURONEXT-PR-2011 - the list's unnamed `Monday 26
        // December 2011` closure (Boxing Day).
        (2011, 12, 26, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-12-30 - T1 - EURONEXT-PR-2011 - New Year's Eve half day: the
        // same 5.35 pm CET sentence.
        (2011, 12, 30, early_close(HALF_DAY_17_35), T1, "EURONEXT-PR-2011"),
        // 2012-04-06 - T1 - EURONEXT-TC-2012 - Good Friday.
        (2012, 4, 6, Closed, T1, "EURONEXT-TC-2012"),
        // 2012-04-09 - T1 - EURONEXT-TC-2012 - Easter Monday.
        (2012, 4, 9, Closed, T1, "EURONEXT-TC-2012"),
        // 2012-05-01 - T1 - EURONEXT-TC-2012 - Labour Day.
        (2012, 5, 1, Closed, T1, "EURONEXT-TC-2012"),
        // 2012-12-24 - T1 - EURONEXT-TC-2012 - Christmas Eve half day:
        // `the markets will close at 14:00 CET`.
        (2012, 12, 24, early_close(HALF_DAY_14_00), T1, "EURONEXT-TC-2012"),
        // 2012-12-25 - T1 - EURONEXT-TC-2012 - Christmas Day.
        (2012, 12, 25, Closed, T1, "EURONEXT-TC-2012"),
        // 2012-12-26 - T1 - EURONEXT-TC-2012 - Boxing Day.
        (2012, 12, 26, Closed, T1, "EURONEXT-TC-2012"),
        // 2012-12-31 - T1 - EURONEXT-TC-2012 - New Year's Eve half day: the
        // same 14:00 CET sentence.
        (2012, 12, 31, early_close(HALF_DAY_14_00), T1, "EURONEXT-TC-2012"),
        // 2013-01-01 - T1 - EURONEXT-IF-2013 - New Year's Day: `Tuesday 1
        // January 2013 (New Year's Day)`.
        (2013, 1, 1, Closed, T1, "EURONEXT-IF-2013"),
        // 2013-03-29 - T1 - EURONEXT-IF-2013 - Good Friday.
        (2013, 3, 29, Closed, T1, "EURONEXT-IF-2013"),
        // 2013-04-01 - T1 - EURONEXT-IF-2013 - Easter Monday.
        (2013, 4, 1, Closed, T1, "EURONEXT-IF-2013"),
        // 2013-05-01 - T1 - EURONEXT-IF-2013 - Labour Day.
        (2013, 5, 1, Closed, T1, "EURONEXT-IF-2013"),
        // 2013-12-24 - T1 - EURONEXT-IF-2013 - Christmas Eve half day: the
        // Info-Flash states `close at 14:00 CET` while the operator's
        // 2014-01-12 page restates 14:05 — held at the narrowest sourced
        // value, 14:00 (see the evidence file's conflict note).
        (2013, 12, 24, early_close(HALF_DAY_14_00), T1, "EURONEXT-IF-2013"),
        // 2013-12-25 - T1 - EURONEXT-IF-2013 - Christmas Day.
        (2013, 12, 25, Closed, T1, "EURONEXT-IF-2013"),
        // 2013-12-26 - T1 - EURONEXT-IF-2013 - Boxing Day.
        (2013, 12, 26, Closed, T1, "EURONEXT-IF-2013"),
        // 2013-12-31 - T1 - EURONEXT-IF-2013 - New Year's Eve half day: the
        // same 14:00-vs-14:05 conflict, held at 14:00.
        (2013, 12, 31, early_close(HALF_DAY_14_00), T1, "EURONEXT-IF-2013"),
        // 2014-01-01 - T1 - EURONEXT-HH-2014-01-12 - New Year's Day: the 2014
        // calendar of business days, cash markets closed.
        (2014, 1, 1, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-04-18 - T1 - EURONEXT-HH-2014-01-12 - Good Friday.
        (2014, 4, 18, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-04-21 - T1 - EURONEXT-HH-2014-01-12 - Easter Monday.
        (2014, 4, 21, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-05-01 - T1 - EURONEXT-HH-2014-01-12 - Labour Day.
        (2014, 5, 1, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-24 - T1 - EURONEXT-HH-2014-01-12 - Christmas Eve half day:
        // `all instruments closing by 14:05 CET`.
        (2014, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-25 - T1 - EURONEXT-HH-2014-01-12 - Christmas Day.
        (2014, 12, 25, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-26 - T1 - EURONEXT-HH-2014-01-12 - Boxing Day.
        (2014, 12, 26, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-31 - T1 - EURONEXT-HH-2014-01-12 - New Year's Eve half day:
        // `all instruments closing by 14:05 CET`.
        (2014, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2014-01-12"),
        // 2015-01-01 - T1 - EURONEXT-HH-2015-03-22 - New Year's Day.
        (2015, 1, 1, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-04-03 - T1 - EURONEXT-HH-2015-03-22 - Good Friday.
        (2015, 4, 3, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-04-06 - T1 - EURONEXT-HH-2015-03-22 - Easter Monday.
        (2015, 4, 6, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-05-01 - T1 - EURONEXT-HH-2015-03-22 - Labour Day.
        (2015, 5, 1, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-24 - T1 - EURONEXT-HH-2015-03-22 - Christmas Eve half day:
        // `all instruments closing by 14:05 CET`.
        (2015, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-25 - T1 - EURONEXT-HH-2015-03-22 - Christmas Day.
        (2015, 12, 25, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-31 - T1 - EURONEXT-HH-2015-03-22 - New Year's Eve: the
        // calendar's own closed-day list prints it as a full closure.
        (2015, 12, 31, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2016-01-01 - T1 - EURONEXT-HH-2016-03-04 - New Year's Day.
        (2016, 1, 1, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-03-25 - T1 - EURONEXT-HH-2016-03-04 - Good Friday.
        (2016, 3, 25, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-03-28 - T1 - EURONEXT-HH-2016-03-04 - Easter Monday.
        (2016, 3, 28, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-12-26 - T1 - EURONEXT-HH-2016-03-04 - Boxing Day; the page
        // states 23 and 30 December close `at the usual times`, so no half
        // days in 2016.
        (2016, 12, 26, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2017-04-14 - T1 - EURONEXT-HH-2017-08-05 - Good Friday.
        (2017, 4, 14, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-04-17 - T1 - EURONEXT-HH-2017-08-05 - Easter Monday.
        (2017, 4, 17, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-05-01 - T1 - EURONEXT-HH-2017-08-05 - Labour Day; 1 January
        // fell on a Sunday and the calendar prints no substitute.
        (2017, 5, 1, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-12-25 - T1 - EURONEXT-HH-2017-08-05 - Christmas Day.
        (2017, 12, 25, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-12-26 - T1 - EURONEXT-HH-2017-08-05 - Boxing Day; 22 and 29
        // December close `at the usual times`, so no half days in 2017.
        (2017, 12, 26, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2018-01-01 - T1 - EURONEXT-HH-2018-08-26 - New Year's Day.
        (2018, 1, 1, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-03-30 - T1 - EURONEXT-HH-2018-08-26 - Good Friday.
        (2018, 3, 30, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-04-02 - T1 - EURONEXT-HH-2018-08-26 - Easter Monday.
        (2018, 4, 2, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-05-01 - T1 - EURONEXT-HH-2018-08-26 - Labour Day.
        (2018, 5, 1, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-24 - T1 - EURONEXT-HH-2018-08-26 - Christmas Eve half day:
        // `all instruments closing by 14:05 CET`.
        (2018, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-25 - T1 - EURONEXT-HH-2018-08-26 - Christmas Day.
        (2018, 12, 25, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-26 - T1 - EURONEXT-HH-2018-08-26 - Boxing Day.
        (2018, 12, 26, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-31 - T1 - EURONEXT-HH-2018-08-26 - New Year's Eve half day:
        // `all instruments closing by 14:05 CET`.
        (2018, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2018-08-26"),
        // 2019-01-01 - T1 - EURONEXT-HH-2019-03-25 - New Year's Day.
        (2019, 1, 1, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-04-19 - T1 - EURONEXT-HH-2019-03-25 - Good Friday.
        (2019, 4, 19, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-04-22 - T1 - EURONEXT-HH-2019-03-25 - Easter Monday.
        (2019, 4, 22, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-05-01 - T1 - EURONEXT-HH-2019-03-25 - Labour Day.
        (2019, 5, 1, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-24 - T1 - EURONEXT-HH-2019-03-25 - Christmas Eve half day:
        // the Amsterdam, Brussels, Lisbon and Paris cash markets' instruments
        // close by 14:05 CET.
        (2019, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-25 - T1 - EURONEXT-HH-2019-03-25 - Christmas Day.
        (2019, 12, 25, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-26 - T1 - EURONEXT-HH-2019-03-25 - Boxing Day.
        (2019, 12, 26, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-31 - T1 - EURONEXT-HH-2019-03-25 - New Year's Eve half day:
        // the same 14:05 CET sentence.
        (2019, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2019-03-25"),
        // 2020-01-01 - T1 - EURONEXT-HH-2019-12-10 - New Year's Day.
        (2020, 1, 1, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-04-10 - T1 - EURONEXT-HH-2019-12-10 - Good Friday.
        (2020, 4, 10, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-04-13 - T1 - EURONEXT-HH-2019-12-10 - Easter Monday.
        (2020, 4, 13, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-05-01 - T1 - EURONEXT-HH-2019-12-10 - Labour Day.
        (2020, 5, 1, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-24 - T1 - EURONEXT-HH-2019-12-10 - Christmas Eve half day:
        // `On the Euronext Amsterdam, Brussels, Lisbon and Paris Cash Markets,
        // all instruments will close by 14:05 CET`; 12-28 is Dublin's
        // substitute only.
        (2020, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-25 - T1 - EURONEXT-HH-2019-12-10 - Christmas Day.
        (2020, 12, 25, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-31 - T1 - EURONEXT-HH-2019-12-10 - New Year's Eve half day:
        // the same 14:05 CET sentence.
        (2020, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2019-12-10"),
        // 2021-01-01 - T1 - EURONEXT-HH-2020-12-05 - New Year's Day; the
        // Dublin and Oslo rows (May Day, Ascension, Constitution, Whit) print
        // `Full Day Trading` for Paris.
        (2021, 1, 1, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-04-02 - T1 - EURONEXT-HH-2020-12-05 - Good Friday.
        (2021, 4, 2, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-04-05 - T1 - EURONEXT-HH-2020-12-05 - Easter Monday.
        (2021, 4, 5, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-12-24 - T1 - EURONEXT-HH-2020-12-05 - Christmas Eve half day:
        // the Amsterdam, Brussels, Lisbon and Paris cash markets close by
        // 14:05 CET (Oslo closed all day).
        (2021, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2020-12-05"),
        // 2021-12-31 - T1 - EURONEXT-HH-2020-12-05 - New Year's Eve half day:
        // the same 14:05 CET sentence.
        (2021, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-HH-2020-12-05"),
        // 2022-04-15 - T1 - EURONEXT-HH-2022-05-23 - Good Friday: the Paris
        // column of the per-market table.
        (2022, 4, 15, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2022-04-18 - T1 - EURONEXT-HH-2022-05-23 - Easter Monday.
        (2022, 4, 18, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2022-12-26 - T1 - EURONEXT-HH-2022-05-23 - Stephen's Day / Boxing
        // Day; 01-03, 12-23, 12-27 and 12-30 print `Full Day Trading` for
        // Paris (the substitutes are Dublin's).
        (2022, 12, 26, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2023-04-07 - T1 - EURONEXT-HH-2023-11-27 - Good Friday: the Paris
        // column of the per-market table.
        (2023, 4, 7, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-04-10 - T1 - EURONEXT-HH-2023-11-27 - Easter Monday.
        (2023, 4, 10, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-05-01 - T1 - EURONEXT-HH-2023-11-27 - May Day.
        (2023, 5, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas.
        (2023, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-12-26 - T1 - EURONEXT-HH-2023-11-27 - St Stephens Day / Boxing
        // Day; 01-02, 12-22 and 12-29 print `Full Day Trading` for Paris
        // (Dublin's substitutes).
        (2023, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-01-01 - T1 - EURONEXT-HH-2023-11-27 - New Year's Day.
        (2024, 1, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-03-29 - T1 - EURONEXT-HH-2023-11-27 - Good Friday; the 27/28
        // March half days are Oslo's and print `Full Day Trading` for Paris.
        (2024, 3, 29, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-04-01 - T1 - EURONEXT-HH-2023-11-27 - Easter Monday.
        (2024, 4, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-05-01 - T1 - EURONEXT-HH-2023-11-27 - Labour Day.
        (2024, 5, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-24 - T1 - EURONEXT-EOY-2024 - Christmas Eve half day: Paris
        // `**Half Trading Day`; the operator's 2024 end-of-year appendix
        // prints the Paris equity segments' TAL to 14:05 CET (retrieved live
        // 2026-09-29, its identity witnessed by the operator page capture of
        // 2025-01-02 that links this appendix by name).
        (2024, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2024"),
        // 2024-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas.
        (2024, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-26 - T1 - EURONEXT-HH-2023-11-27 - St Stephens Day / Boxing
        // Day.
        (2024, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-31 - T1 - EURONEXT-EOY-2024 - New Year's Eve half day: the
        // same appendix grid, TAL to 14:05 CET.
        (2024, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2024"),
        // 2025-01-01 - T1 - EURONEXT-HH-2025-12-06 - New Year's Day: Paris
        // `Closed`.
        (2025, 1, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-18 - T1 - EURONEXT-HH-2025-12-06 - Good Friday: Paris
        // `Closed`.
        (2025, 4, 18, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-21 - T1 - EURONEXT-HH-2025-12-06 - Easter Monday: Paris
        // `Closed`.
        (2025, 4, 21, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-05-01 - T1 - EURONEXT-HH-2025-12-06 - Labour Day: Paris
        // `Closed`.
        (2025, 5, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-24 - T1 - EURONEXT-EOY-2025 - Christmas Eve half day: Paris
        // `**Half Trading Day`; the appendix states TAL to 14:05 CET.
        (2025, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2025"),
        // 2025-12-25 - T1 - EURONEXT-HH-2025-12-06 - Christmas: Paris
        // `Closed`.
        (2025, 12, 25, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-26 - T1 - EURONEXT-HH-2025-12-06 - St Stephen's Day / Boxing
        // Day: Paris `Closed`.
        (2025, 12, 26, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-31 - T1 - EURONEXT-EOY-2025 - New Year's Eve half day: Paris
        // `**Half Trading Day`; the appendix states TAL to 14:05 CET.
        (2025, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2025"),
        // 2026-01-01 - T1 - EURONEXT-IF-2026 - New Year's Day: Paris `Closed`.
        (2026, 1, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-03 - T1 - EURONEXT-IF-2026 - Good Friday: Paris `Closed`.
        (2026, 4, 3, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-06 - T1 - EURONEXT-IF-2026 - Easter Monday: Paris `Closed`.
        (2026, 4, 6, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-05-01 - T1 - EURONEXT-IF-2026 - Labour Day: Paris `Closed`.
        (2026, 5, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-24 - T1 - EURONEXT-IF-2026 - Christmas Eve: Paris prints
        // `**Half Trading Day` but the 2026 end-of-year hours are announced
        // and unstated ("To be announced"), so no instant is claimed.
        (2026, 12, 24, Unsourced, T1, "EURONEXT-IF-2026"),
        // 2026-12-25 - T1 - EURONEXT-IF-2026 - Christmas: Paris `Closed`.
        (2026, 12, 25, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-31 - T1 - EURONEXT-IF-2026 - New Year's Eve: Paris prints
        // `**Half Trading Day` but the 2026 end-of-year hours are announced
        // and unstated ("To be announced"), so no instant is claimed.
        (2026, 12, 31, Unsourced, T1, "EURONEXT-IF-2026"),
    ],
};
