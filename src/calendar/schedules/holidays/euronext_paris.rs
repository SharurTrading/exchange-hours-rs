// SPDX-License-Identifier: MIT-0

//! Euronext Paris holiday rows, 2014-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Paris`. Euronext
//! Paris runs no overnight session, so an event date and its trade date are one
//! civil day and the conversion is the identity.
//!
//! The whole block is **T1**: the operator's own holiday calendar, read across
//! three site generations — the NYSE Euronext `trading-hours-and-holidays`
//! page (2014-2015, one calendar for every Euronext cash market), the
//! `en/trading-calendars-hours` page (2016-2019, one calendar per year) and
//! the `live.euronext.com` `trading-hours-holidays` page (2019-2025, per-
//! market tables whose Paris is the last column). Twenty archived states pin
//! the years 2014-2025 (the evidence file's `### Documents` table); the
//! Wayback index holds no capture of any operator holiday page for
//! **2010-2013** (CDX sweep 2026-09-29 UTC), so coverage opens 2014-01-01 and
//! queries before it refuse.
//!
//! December half days are **early closes at the operator's stated instant**:
//! 2014, 2015 and 2018-2021 print `all instruments closing by 14:05 CET` for
//! the Amsterdam, Brussels, Lisbon and Paris cash markets, so the day's
//! availability envelope ends at 14:05. 2016 and 2017 print `close at the
//! usual times` for their substitute December Fridays — no half day. 2022 and
//! 2023 move to the per-market table, where the 23/30 December 2022 and
//! 22/29 December 2023 half days are **Dublin's** substitutes and the Paris
//! column prints `Full Day Trading`; Paris ships no row for them. For 2024
//! the Paris column prints `**Half Trading Day` on 24 and 31 December but the
//! instant lives in the operator's end-of-year appendix to the Euronext
//! Instructions 4-01/4-03, which no surviving capture holds, so those two
//! rows ship as `Unsourced` (the same shape as the 2026 eves below). The 2025
//! appendix is held and keys the 2025 half days at 14:05. This table encodes
//! **Paris only**; the other markets' columns are out of this identity's
//! scope, and the follow-up for them is recorded in
//! [`docs/evidence/euronext_paris.md`](../../../../../docs/evidence/euronext_paris.md).

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// Euronext Paris's stated half-day envelope close, 14:05 CET.
///
/// For 2025 the end-of-year appendix prints `14:00  -  14:05` for the TAL
/// phase of every Paris equity segment on `24th and 31st of December 2025`;
/// for 2014, 2015 and 2018-2021 the operator's own calendar pages state `all
/// instruments closing by 14:05 CET` for the Amsterdam, Brussels, Lisbon and
/// Paris cash markets. In both wordings 14:05 is the day's final close of the
/// availability envelope.
const HALF_DAY_14_05: u32 = 14 * 3_600 + 5 * 60;

/// Euronext Paris's built-in holiday rows and the window they were audited
/// over.
///
/// Every row is the Paris cell (or, before the per-market tables, the cash-
/// markets line) of one line of the operator's calendar at the capture its
/// document id names — the earliest archived state that prints the row.
/// A date inside the window with no row is audited normal; the two 2024 eves
/// and the two 2026 eves carry `Unsourced` rows because their half-day
/// instants are published only in the end-of-year appendix each year.
// Evidence: docs/evidence/euronext_paris.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2014, 1, 1) ..= (2026, 12, 31)],
    rows: [
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
        // 2024-12-24 - T1 - EURONEXT-HH-2023-11-27 - Christmas Eve: Paris
        // prints `**Half Trading Day` but the instant lives in the operator's
        // end-of-year appendix, which no surviving capture holds; no instant
        // is claimed.
        (2024, 12, 24, Unsourced, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas.
        (2024, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-26 - T1 - EURONEXT-HH-2023-11-27 - St Stephens Day / Boxing
        // Day.
        (2024, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-31 - T1 - EURONEXT-HH-2023-11-27 - New Year's Eve: Paris
        // prints `**Half Trading Day` but the end-of-year appendix is not
        // archived; no instant is claimed.
        (2024, 12, 31, Unsourced, T1, "EURONEXT-HH-2023-11-27"),
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
