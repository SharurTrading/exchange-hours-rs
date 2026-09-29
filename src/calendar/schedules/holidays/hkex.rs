//! Hong Kong Exchanges and Clearing holiday rows, 2010-2024 and 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Hong_Kong`. HKEX's
//! securities market trades Monday to Friday excluding public holidays, and
//! the operator's own holiday schedule names every such day: the per-year
//! `Trading Calendar` PDFs of 2010-2017 (whose holiday list plus the footer
//! sentence `Markets are closed on Saturdays, Sundays and Public Holidays`
//! states each year's closures and half-day trading days) and, from 2018, the
//! `Trading Calendar and Holiday Schedule` page's `Holiday Schedule` table
//! with its eve `Notes`. The whole block is **T1**; 2010-2017 rows cite that
//! year's PDF (`HKEX-TC-<year>`) and 2018-2024 rows cite the edition of the
//! page that prints the year (`HKEX-TC-PAGE-<year>`).
//!
//! An eve row is an **early close**, not a closure. The instant is era-bound:
//! 2010-2011 half days ran `9:30am to 12:30pm` with `no afternoon trading
//! session` (`HKEX-TN-2010`), so their close is 12:30; the 2017-2024 eves
//! delete the afternoon and close at the half-day CAS edge 12:10
//! (`HKEX-HOURS-SEC`, the arrangement in force since the 2016-07-25 CAS
//! launch). The 2012-2015 calendars likewise name their eves half-day trading
//! days, but no retrieved artifact states that era's half-day close, so those
//! ten dates ship [`HolidayKind::Unsourced`] rather than an invented instant;
//! the gap and its closing condition are recorded in
//! [`docs/evidence/hkex.md`](../../../../../docs/evidence/hkex.md).
//!
//! Severe-weather arrangements (typhoon signals) are conditional and key no
//! row (LAW-NO-FABRICATED-DATES); the many typhoon and black-rain halts of
//! 2010-2024 are halts, not closures. Holidays that fall on a weekend are
//! subsumed by the calendar's own Saturday/Sunday closure statement and key no
//! weekday row. The derivatives-only footnotes (after-hours suspensions on
//! UK/US bank holidays and, from 2022, Holiday Trading Exchange Contracts)
//! name no securities session and key no row either.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::HolidayKind::Unsourced;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// The half-day final close, `12:10` venue-local, in seconds since midnight.
///
/// The securities hours page prints the half-day Closing Auction Session as
/// `12:00 noon to a random closing between 12:08 p.m. and 12:10 p.m.`; the
/// profile states the latest scheduled CAS edge, exactly as the normal day's
/// `4:08 p.m. - 4:10 p.m.` close is stated at 16:10.
const HALF_DAY_CLOSE_SSM: u32 = 12 * 3_600 + 10 * 60;

/// The 2010-2011-era half-day final close, `12:30` venue-local, in seconds
/// since midnight.
///
/// The operator's own trading-news page states the era's half-day shape: `a
/// half trading day from 9:30am to 12:30pm` with `no afternoon trading
/// session` (`HKEX-TN-2010`, updated 24 Dec 2010). The same session grid was
/// in force through 4 March 2011 (`HKEX-NEWS-PHASE1`'s `Current` column:
/// Morning Session 10:00-12:30).
const ERA_HALF_DAY_CLOSE_SSM: u32 = 12 * 3_600 + 30 * 60;

/// HKEX's built-in holiday rows and the windows they were audited over.
///
/// Every row is one line of the operator's own schedule: a holiday list line of
/// the 2010-2017 `Trading Calendar` PDF, a `Holiday (no trading)` row of the
/// 2018-2024 page table, or one line of its `Notes` block (an eve with `no
/// afternoon and after-hours trading session`). The 12:10 instant comes from
/// the operator's securities-market half-day schedule and the 2010-2011 12:30
/// instant from the era's own half-day statement. A date inside a window with
/// no row is audited normal.
// Evidence: docs/evidence/hkex.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // HKEX-TC-2010: the operator's own holiday schedule
        // 2010-01-01 - T1 - HKEX-TC-2010 - The first day of January.
        (2010, 1, 1, Closed, T1, "HKEX-TC-2010"),
        // 2010-02-15 - T1 - HKEX-TC-2010 - The second day of the Lunar New Year.
        (2010, 2, 15, Closed, T1, "HKEX-TC-2010"),
        // 2010-02-16 - T1 - HKEX-TC-2010 - The third day of the Lunar New Year.
        (2010, 2, 16, Closed, T1, "HKEX-TC-2010"),
        // 2010-04-02 - T1 - HKEX-TC-2010 - Good Friday.
        (2010, 4, 2, Closed, T1, "HKEX-TC-2010"),
        // 2010-04-05 - T1 - HKEX-TC-2010 - Easter Monday.
        (2010, 4, 5, Closed, T1, "HKEX-TC-2010"),
        // 2010-04-06 - T1 - HKEX-TC-2010 - The day following Ching Ming Festival.
        (2010, 4, 6, Closed, T1, "HKEX-TC-2010"),
        // 2010-05-21 - T1 - HKEX-TC-2010 - The Buddha's Birthday.
        (2010, 5, 21, Closed, T1, "HKEX-TC-2010"),
        // 2010-06-16 - T1 - HKEX-TC-2010 - Tuen Ng Festival.
        (2010, 6, 16, Closed, T1, "HKEX-TC-2010"),
        // 2010-07-01 - T1 - HKEX-TC-2010 - Hong Kong Special Administrative Region Establishment Day.
        (2010, 7, 1, Closed, T1, "HKEX-TC-2010"),
        // 2010-09-23 - T1 - HKEX-TC-2010 - The day following Chinese Mid-Autumn Festival.
        (2010, 9, 23, Closed, T1, "HKEX-TC-2010"),
        // 2010-10-01 - T1 - HKEX-TC-2010 - National Day.
        (2010, 10, 1, Closed, T1, "HKEX-TC-2010"),
        // 2010-12-24 - T1 - HKEX-TC-2010 - Eve of Christmas Day; no afternoon session; the half-day close is 12:30 (HKEX-TN-2010).
        (2010, 12, 24, early_close(ERA_HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2010"),
        // 2010-12-27 - T1 - HKEX-TC-2010 - The first weekday after Christmas Day.
        (2010, 12, 27, Closed, T1, "HKEX-TC-2010"),
        // 2010-12-31 - T1 - HKEX-TC-2010 - Eve of New Year; no afternoon session; the half-day close is 12:30 (HKEX-TN-2010).
        (2010, 12, 31, early_close(ERA_HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2010"),
        // HKEX-TC-2011: the operator's own holiday schedule
        // 2011-02-02 - T1 - HKEX-TC-2011 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:30 (HKEX-TN-2010).
        (2011, 2, 2, early_close(ERA_HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2011"),
        // 2011-02-03 - T1 - HKEX-TC-2011 - Lunar New Year's Day.
        (2011, 2, 3, Closed, T1, "HKEX-TC-2011"),
        // 2011-02-04 - T1 - HKEX-TC-2011 - The second day of the Lunar New Year.
        (2011, 2, 4, Closed, T1, "HKEX-TC-2011"),
        // 2011-04-05 - T1 - HKEX-TC-2011 - Ching Ming Festival.
        (2011, 4, 5, Closed, T1, "HKEX-TC-2011"),
        // 2011-04-22 - T1 - HKEX-TC-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "HKEX-TC-2011"),
        // 2011-04-25 - T1 - HKEX-TC-2011 - Easter Monday.
        (2011, 4, 25, Closed, T1, "HKEX-TC-2011"),
        // 2011-05-02 - T1 - HKEX-TC-2011 - The day following Labour Day.
        (2011, 5, 2, Closed, T1, "HKEX-TC-2011"),
        // 2011-05-10 - T1 - HKEX-TC-2011 - The Buddha's Birthday.
        (2011, 5, 10, Closed, T1, "HKEX-TC-2011"),
        // 2011-06-06 - T1 - HKEX-TC-2011 - Tuen Ng Festival.
        (2011, 6, 6, Closed, T1, "HKEX-TC-2011"),
        // 2011-07-01 - T1 - HKEX-TC-2011 - Hong Kong Special Administrative Region Establishment Day.
        (2011, 7, 1, Closed, T1, "HKEX-TC-2011"),
        // 2011-09-13 - T1 - HKEX-TC-2011 - The day following Chinese Mid-Autumn Festival.
        (2011, 9, 13, Closed, T1, "HKEX-TC-2011"),
        // 2011-10-05 - T1 - HKEX-TC-2011 - Chung Yeung Festival.
        (2011, 10, 5, Closed, T1, "HKEX-TC-2011"),
        // 2011-12-26 - T1 - HKEX-TC-2011 - The first weekday after Christmas Day.
        (2011, 12, 26, Closed, T1, "HKEX-TC-2011"),
        // 2011-12-27 - T1 - HKEX-TC-2011 - The second weekday after Christmas Day.
        (2011, 12, 27, Closed, T1, "HKEX-TC-2011"),
        // HKEX-TC-2012: the operator's own holiday schedule
        // 2012-01-02 - T1 - HKEX-TC-2012 - The day following the first day of January.
        (2012, 1, 2, Closed, T1, "HKEX-TC-2012"),
        // 2012-01-23 - T1 - HKEX-TC-2012 - Lunar New Year's Day.
        (2012, 1, 23, Closed, T1, "HKEX-TC-2012"),
        // 2012-01-24 - T1 - HKEX-TC-2012 - The second day of the Lunar New Year.
        (2012, 1, 24, Closed, T1, "HKEX-TC-2012"),
        // 2012-01-25 - T1 - HKEX-TC-2012 - The third day of the Lunar New Year.
        (2012, 1, 25, Closed, T1, "HKEX-TC-2012"),
        // 2012-04-04 - T1 - HKEX-TC-2012 - Ching Ming Festival.
        (2012, 4, 4, Closed, T1, "HKEX-TC-2012"),
        // 2012-04-06 - T1 - HKEX-TC-2012 - Good Friday.
        (2012, 4, 6, Closed, T1, "HKEX-TC-2012"),
        // 2012-04-09 - T1 - HKEX-TC-2012 - Easter Monday.
        (2012, 4, 9, Closed, T1, "HKEX-TC-2012"),
        // 2012-05-01 - T1 - HKEX-TC-2012 - Labour Day.
        (2012, 5, 1, Closed, T1, "HKEX-TC-2012"),
        // 2012-07-02 - T1 - HKEX-TC-2012 - The day following Hong Kong Special Administrative Region Establishment Day.
        (2012, 7, 2, Closed, T1, "HKEX-TC-2012"),
        // 2012-10-01 - T1 - HKEX-TC-2012 - The day following Chinese Mid-Autumn Festival.
        (2012, 10, 1, Closed, T1, "HKEX-TC-2012"),
        // 2012-10-02 - T1 - HKEX-TC-2012 - The day following National Day.
        (2012, 10, 2, Closed, T1, "HKEX-TC-2012"),
        // 2012-10-23 - T1 - HKEX-TC-2012 - Chung Yeung Festival.
        (2012, 10, 23, Closed, T1, "HKEX-TC-2012"),
        // 2012-12-24 - T1 - HKEX-TC-2012 - Christmas Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2012, 12, 24, Unsourced, T1, "HKEX-TC-2012"),
        // 2012-12-25 - T1 - HKEX-TC-2012 - Christmas Day.
        (2012, 12, 25, Closed, T1, "HKEX-TC-2012"),
        // 2012-12-26 - T1 - HKEX-TC-2012 - The first weekday after Christmas Day.
        (2012, 12, 26, Closed, T1, "HKEX-TC-2012"),
        // 2012-12-31 - T1 - HKEX-TC-2012 - New Year's Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2012, 12, 31, Unsourced, T1, "HKEX-TC-2012"),
        // HKEX-TC-2013: the operator's own holiday schedule
        // 2013-01-01 - T1 - HKEX-TC-2013 - The first day of January.
        (2013, 1, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-02-11 - T1 - HKEX-TC-2013 - The second day of Lunar New Year.
        (2013, 2, 11, Closed, T1, "HKEX-TC-2013"),
        // 2013-02-12 - T1 - HKEX-TC-2013 - The third day of Lunar New Year.
        (2013, 2, 12, Closed, T1, "HKEX-TC-2013"),
        // 2013-02-13 - T1 - HKEX-TC-2013 - The fourth day of Lunar New Year.
        (2013, 2, 13, Closed, T1, "HKEX-TC-2013"),
        // 2013-03-29 - T1 - HKEX-TC-2013 - Good Friday.
        (2013, 3, 29, Closed, T1, "HKEX-TC-2013"),
        // 2013-04-01 - T1 - HKEX-TC-2013 - Easter Monday.
        (2013, 4, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-04-04 - T1 - HKEX-TC-2013 - Ching Ming Festival.
        (2013, 4, 4, Closed, T1, "HKEX-TC-2013"),
        // 2013-05-01 - T1 - HKEX-TC-2013 - Labour Day.
        (2013, 5, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-05-17 - T1 - HKEX-TC-2013 - The Birthday of the Buddha.
        (2013, 5, 17, Closed, T1, "HKEX-TC-2013"),
        // 2013-06-12 - T1 - HKEX-TC-2013 - Tuen Ng Festival.
        (2013, 6, 12, Closed, T1, "HKEX-TC-2013"),
        // 2013-07-01 - T1 - HKEX-TC-2013 - Hong Kong Special Administrative Region Establishment Day.
        (2013, 7, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-09-20 - T1 - HKEX-TC-2013 - The day following the Chinese Mid-Autumn Festival.
        (2013, 9, 20, Closed, T1, "HKEX-TC-2013"),
        // 2013-10-01 - T1 - HKEX-TC-2013 - National Day.
        (2013, 10, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-10-14 - T1 - HKEX-TC-2013 - The day following Chung Yeung Festival.
        (2013, 10, 14, Closed, T1, "HKEX-TC-2013"),
        // 2013-12-24 - T1 - HKEX-TC-2013 - Christmas Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2013, 12, 24, Unsourced, T1, "HKEX-TC-2013"),
        // 2013-12-25 - T1 - HKEX-TC-2013 - Christmas Day.
        (2013, 12, 25, Closed, T1, "HKEX-TC-2013"),
        // 2013-12-26 - T1 - HKEX-TC-2013 - The first weekday after Christmas Day.
        (2013, 12, 26, Closed, T1, "HKEX-TC-2013"),
        // 2013-12-31 - T1 - HKEX-TC-2013 - New Year's Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2013, 12, 31, Unsourced, T1, "HKEX-TC-2013"),
        // HKEX-TC-2014: the operator's own holiday schedule
        // 2014-01-01 - T1 - HKEX-TC-2014 - The first day of January.
        (2014, 1, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-01-30 - T1 - HKEX-TC-2014 - Eve of Lunar New Year: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2014, 1, 30, Unsourced, T1, "HKEX-TC-2014"),
        // 2014-01-31 - T1 - HKEX-TC-2014 - Lunar New Year's Day.
        (2014, 1, 31, Closed, T1, "HKEX-TC-2014"),
        // 2014-02-03 - T1 - HKEX-TC-2014 - The fourth day of Lunar New Year.
        (2014, 2, 3, Closed, T1, "HKEX-TC-2014"),
        // 2014-04-18 - T1 - HKEX-TC-2014 - Good Friday.
        (2014, 4, 18, Closed, T1, "HKEX-TC-2014"),
        // 2014-04-21 - T1 - HKEX-TC-2014 - Easter Monday.
        (2014, 4, 21, Closed, T1, "HKEX-TC-2014"),
        // 2014-05-01 - T1 - HKEX-TC-2014 - Labour Day.
        (2014, 5, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-05-06 - T1 - HKEX-TC-2014 - The Birthday of the Buddha.
        (2014, 5, 6, Closed, T1, "HKEX-TC-2014"),
        // 2014-06-02 - T1 - HKEX-TC-2014 - Tuen Ng Festival.
        (2014, 6, 2, Closed, T1, "HKEX-TC-2014"),
        // 2014-07-01 - T1 - HKEX-TC-2014 - Hong Kong Special Administrative Region Establishment Day.
        (2014, 7, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-09-09 - T1 - HKEX-TC-2014 - The day following the Chinese Mid-Autumn Festival.
        (2014, 9, 9, Closed, T1, "HKEX-TC-2014"),
        // 2014-10-01 - T1 - HKEX-TC-2014 - National Day.
        (2014, 10, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-10-02 - T1 - HKEX-TC-2014 - Chung Yeung Festival.
        (2014, 10, 2, Closed, T1, "HKEX-TC-2014"),
        // 2014-12-24 - T1 - HKEX-TC-2014 - Christmas Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2014, 12, 24, Unsourced, T1, "HKEX-TC-2014"),
        // 2014-12-25 - T1 - HKEX-TC-2014 - Christmas Day.
        (2014, 12, 25, Closed, T1, "HKEX-TC-2014"),
        // 2014-12-26 - T1 - HKEX-TC-2014 - The first weekday after Christmas Day.
        (2014, 12, 26, Closed, T1, "HKEX-TC-2014"),
        // 2014-12-31 - T1 - HKEX-TC-2014 - New Year's Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2014, 12, 31, Unsourced, T1, "HKEX-TC-2014"),
        // HKEX-TC-2015: the operator's own holiday schedule
        // 2015-01-01 - T1 - HKEX-TC-2015 - The first day of January.
        (2015, 1, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-02-18 - T1 - HKEX-TC-2015 - Eve of Lunar New Year: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2015, 2, 18, Unsourced, T1, "HKEX-TC-2015"),
        // 2015-02-19 - T1 - HKEX-TC-2015 - Lunar New Year's Day.
        (2015, 2, 19, Closed, T1, "HKEX-TC-2015"),
        // 2015-02-20 - T1 - HKEX-TC-2015 - The second day of Lunar New Year.
        (2015, 2, 20, Closed, T1, "HKEX-TC-2015"),
        // 2015-04-03 - T1 - HKEX-TC-2015 - Good Friday.
        (2015, 4, 3, Closed, T1, "HKEX-TC-2015"),
        // 2015-04-06 - T1 - HKEX-TC-2015 - The day following Ching Ming Festival.
        (2015, 4, 6, Closed, T1, "HKEX-TC-2015"),
        // 2015-04-07 - T1 - HKEX-TC-2015 - The day following Easter Monday.
        (2015, 4, 7, Closed, T1, "HKEX-TC-2015"),
        // 2015-05-01 - T1 - HKEX-TC-2015 - Labour Day.
        (2015, 5, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-05-25 - T1 - HKEX-TC-2015 - The Birthday of the Buddha.
        (2015, 5, 25, Closed, T1, "HKEX-TC-2015"),
        // 2015-07-01 - T1 - HKEX-TC-2015 - Hong Kong Special Administrative Region Establishment Day.
        (2015, 7, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-09-28 - T1 - HKEX-TC-2015 - The day following the Chinese Mid-Autumn Festival.
        (2015, 9, 28, Closed, T1, "HKEX-TC-2015"),
        // 2015-10-01 - T1 - HKEX-TC-2015 - National Day.
        (2015, 10, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-10-21 - T1 - HKEX-TC-2015 - Chung Yeung Festival.
        (2015, 10, 21, Closed, T1, "HKEX-TC-2015"),
        // 2015-12-24 - T1 - HKEX-TC-2015 - Christmas Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2015, 12, 24, Unsourced, T1, "HKEX-TC-2015"),
        // 2015-12-25 - T1 - HKEX-TC-2015 - Christmas Day.
        (2015, 12, 25, Closed, T1, "HKEX-TC-2015"),
        // 2015-12-31 - T1 - HKEX-TC-2015 - New Year's Eve: named a half-day trading day; the pre-CAS half-day close instant is unsourced, so the date ships Unsourced.
        (2015, 12, 31, Unsourced, T1, "HKEX-TC-2015"),
        // HKEX-TC-2016: the operator's own holiday schedule
        // 2016-01-01 - T1 - HKEX-TC-2016 - The first day of January.
        (2016, 1, 1, Closed, T1, "HKEX-TC-2016"),
        // 2016-02-08 - T1 - HKEX-TC-2016 - Lunar New Year's Day.
        (2016, 2, 8, Closed, T1, "HKEX-TC-2016"),
        // 2016-02-09 - T1 - HKEX-TC-2016 - The second day of Lunar New Year.
        (2016, 2, 9, Closed, T1, "HKEX-TC-2016"),
        // 2016-02-10 - T1 - HKEX-TC-2016 - The third day of Lunar New Year.
        (2016, 2, 10, Closed, T1, "HKEX-TC-2016"),
        // 2016-03-25 - T1 - HKEX-TC-2016 - Good Friday.
        (2016, 3, 25, Closed, T1, "HKEX-TC-2016"),
        // 2016-03-28 - T1 - HKEX-TC-2016 - Easter Monday.
        (2016, 3, 28, Closed, T1, "HKEX-TC-2016"),
        // 2016-04-04 - T1 - HKEX-TC-2016 - Ching Ming Festival.
        (2016, 4, 4, Closed, T1, "HKEX-TC-2016"),
        // 2016-05-02 - T1 - HKEX-TC-2016 - The day following the Labour Day.
        (2016, 5, 2, Closed, T1, "HKEX-TC-2016"),
        // 2016-06-09 - T1 - HKEX-TC-2016 - Tuen Ng Festival.
        (2016, 6, 9, Closed, T1, "HKEX-TC-2016"),
        // 2016-07-01 - T1 - HKEX-TC-2016 - Hong Kong Special Administrative Region Establishment Day.
        (2016, 7, 1, Closed, T1, "HKEX-TC-2016"),
        // 2016-09-16 - T1 - HKEX-TC-2016 - The day following the Chinese Mid-Autumn Festival.
        (2016, 9, 16, Closed, T1, "HKEX-TC-2016"),
        // 2016-10-10 - T1 - HKEX-TC-2016 - The day following the Chung Yeung Festival.
        (2016, 10, 10, Closed, T1, "HKEX-TC-2016"),
        // 2016-12-26 - T1 - HKEX-TC-2016 - The first weekday after Christmas Day.
        (2016, 12, 26, Closed, T1, "HKEX-TC-2016"),
        // 2016-12-27 - T1 - HKEX-TC-2016 - The second weekday after Christmas Day.
        (2016, 12, 27, Closed, T1, "HKEX-TC-2016"),
        // HKEX-TC-2017: the operator's own holiday schedule
        // 2017-01-02 - T1 - HKEX-TC-2017 - The day following the first day of January.
        (2017, 1, 2, Closed, T1, "HKEX-TC-2017"),
        // 2017-01-27 - T1 - HKEX-TC-2017 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2017, 1, 27, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2017"),
        // 2017-01-30 - T1 - HKEX-TC-2017 - The third day of Lunar New Year.
        (2017, 1, 30, Closed, T1, "HKEX-TC-2017"),
        // 2017-01-31 - T1 - HKEX-TC-2017 - The fourth day of Lunar New Year.
        (2017, 1, 31, Closed, T1, "HKEX-TC-2017"),
        // 2017-04-04 - T1 - HKEX-TC-2017 - Ching Ming Festival.
        (2017, 4, 4, Closed, T1, "HKEX-TC-2017"),
        // 2017-04-14 - T1 - HKEX-TC-2017 - Good Friday.
        (2017, 4, 14, Closed, T1, "HKEX-TC-2017"),
        // 2017-04-17 - T1 - HKEX-TC-2017 - Easter Monday.
        (2017, 4, 17, Closed, T1, "HKEX-TC-2017"),
        // 2017-05-01 - T1 - HKEX-TC-2017 - Labour Day.
        (2017, 5, 1, Closed, T1, "HKEX-TC-2017"),
        // 2017-05-03 - T1 - HKEX-TC-2017 - The Birthday of the Buddha.
        (2017, 5, 3, Closed, T1, "HKEX-TC-2017"),
        // 2017-05-30 - T1 - HKEX-TC-2017 - Tuen Ng Festival.
        (2017, 5, 30, Closed, T1, "HKEX-TC-2017"),
        // 2017-10-02 - T1 - HKEX-TC-2017 - The day following National Day.
        (2017, 10, 2, Closed, T1, "HKEX-TC-2017"),
        // 2017-10-05 - T1 - HKEX-TC-2017 - The day following the Chinese Mid-Autumn Festival.
        (2017, 10, 5, Closed, T1, "HKEX-TC-2017"),
        // 2017-12-25 - T1 - HKEX-TC-2017 - Christmas Day.
        (2017, 12, 25, Closed, T1, "HKEX-TC-2017"),
        // 2017-12-26 - T1 - HKEX-TC-2017 - The first weekday after Christmas Day.
        (2017, 12, 26, Closed, T1, "HKEX-TC-2017"),
        // HKEX-TC-PAGE-2018: the operator's own holiday schedule
        // 2018-01-01 - T1 - HKEX-TC-PAGE-2018 - The first day of January.
        (2018, 1, 1, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-02-15 - T1 - HKEX-TC-PAGE-2018 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2018, 2, 15, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2018"),
        // 2018-02-16 - T1 - HKEX-TC-PAGE-2018 - Lunar New Year's Day.
        (2018, 2, 16, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-02-19 - T1 - HKEX-TC-PAGE-2018 - The fourth day of Lunar New Year.
        (2018, 2, 19, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-03-30 - T1 - HKEX-TC-PAGE-2018 - Good Friday.
        (2018, 3, 30, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-04-02 - T1 - HKEX-TC-PAGE-2018 - Easter Monday.
        (2018, 4, 2, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-04-05 - T1 - HKEX-TC-PAGE-2018 - Ching Ming Festival.
        (2018, 4, 5, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-05-01 - T1 - HKEX-TC-PAGE-2018 - Labour day.
        (2018, 5, 1, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-05-22 - T1 - HKEX-TC-PAGE-2018 - The Birthday of the Buddha.
        (2018, 5, 22, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-06-18 - T1 - HKEX-TC-PAGE-2018 - Tuen Ng Festival.
        (2018, 6, 18, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-07-02 - T1 - HKEX-TC-PAGE-2018 - The day following Hong Kong Special Administrative Region Establishment Day.
        (2018, 7, 2, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-09-25 - T1 - HKEX-TC-PAGE-2018 - The day following the Chinese Mid-Autumn Festival.
        (2018, 9, 25, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-10-01 - T1 - HKEX-TC-PAGE-2018 - National Day.
        (2018, 10, 1, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-10-17 - T1 - HKEX-TC-PAGE-2018 - Chung Yeung Festival.
        (2018, 10, 17, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-12-24 - T1 - HKEX-TC-PAGE-2018 - Eve of Christmas Day; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2018, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2018"),
        // 2018-12-25 - T1 - HKEX-TC-PAGE-2018 - Christmas Day.
        (2018, 12, 25, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-12-26 - T1 - HKEX-TC-PAGE-2018 - The first weekday after Christmas Day.
        (2018, 12, 26, Closed, T1, "HKEX-TC-PAGE-2018"),
        // 2018-12-31 - T1 - HKEX-TC-PAGE-2018 - Eve of New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2018, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2018"),
        // HKEX-TC-PAGE-2019: the operator's own holiday schedule
        // 2019-01-01 - T1 - HKEX-TC-PAGE-2019 - The first day of January.
        (2019, 1, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-04 - T1 - HKEX-TC-PAGE-2019 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2019, 2, 4, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-05 - T1 - HKEX-TC-PAGE-2019 - Lunar New Year's Day.
        (2019, 2, 5, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-06 - T1 - HKEX-TC-PAGE-2019 - The second day of Lunar New Year.
        (2019, 2, 6, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-07 - T1 - HKEX-TC-PAGE-2019 - The third day of Lunar New Year.
        (2019, 2, 7, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-04-05 - T1 - HKEX-TC-PAGE-2019 - Ching Ming Festival.
        (2019, 4, 5, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-04-19 - T1 - HKEX-TC-PAGE-2019 - Good Friday.
        (2019, 4, 19, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-04-22 - T1 - HKEX-TC-PAGE-2019 - Easter Monday.
        (2019, 4, 22, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-05-01 - T1 - HKEX-TC-PAGE-2019 - Labour day.
        (2019, 5, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-05-13 - T1 - HKEX-TC-PAGE-2019 - The day following the Birthday of the Buddha.
        (2019, 5, 13, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-06-07 - T1 - HKEX-TC-PAGE-2019 - Tuen Ng Festival.
        (2019, 6, 7, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-07-01 - T1 - HKEX-TC-PAGE-2019 - Hong Kong Special Administrative Region Establishment Day.
        (2019, 7, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-10-01 - T1 - HKEX-TC-PAGE-2019 - National Day.
        (2019, 10, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-10-07 - T1 - HKEX-TC-PAGE-2019 - Chung Yeung Festival.
        (2019, 10, 7, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-24 - T1 - HKEX-TC-PAGE-2019 - Eve of Christmas Day; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2019, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-25 - T1 - HKEX-TC-PAGE-2019 - Christmas Day.
        (2019, 12, 25, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-26 - T1 - HKEX-TC-PAGE-2019 - The first weekday after Christmas Day.
        (2019, 12, 26, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-31 - T1 - HKEX-TC-PAGE-2019 - Eve of New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2019, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2019"),
        // HKEX-TC-PAGE-2020: the operator's own holiday schedule
        // 2020-01-01 - T1 - HKEX-TC-PAGE-2020 - The first day of January.
        (2020, 1, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-01-24 - T1 - HKEX-TC-PAGE-2020 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2020, 1, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2020"),
        // 2020-01-27 - T1 - HKEX-TC-PAGE-2020 - The third day of Lunar New Year.
        (2020, 1, 27, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-01-28 - T1 - HKEX-TC-PAGE-2020 - The fourth day of Lunar New Year.
        (2020, 1, 28, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-04-10 - T1 - HKEX-TC-PAGE-2020 - Good Friday.
        (2020, 4, 10, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-04-13 - T1 - HKEX-TC-PAGE-2020 - Easter Monday.
        (2020, 4, 13, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-04-30 - T1 - HKEX-TC-PAGE-2020 - Birthday of the Buddha.
        (2020, 4, 30, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-05-01 - T1 - HKEX-TC-PAGE-2020 - Labour Day.
        (2020, 5, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-06-25 - T1 - HKEX-TC-PAGE-2020 - Tuen Ng Festival.
        (2020, 6, 25, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-07-01 - T1 - HKEX-TC-PAGE-2020 - Hong Kong Special Administrative Region Establishment Day.
        (2020, 7, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-10-01 - T1 - HKEX-TC-PAGE-2020 - National Day.
        (2020, 10, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-10-02 - T1 - HKEX-TC-PAGE-2020 - The day following the Chinese Mid-Autumn Festival.
        (2020, 10, 2, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-10-26 - T1 - HKEX-TC-PAGE-2020 - The day following Chung Yeung Festival.
        (2020, 10, 26, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-12-24 - T1 - HKEX-TC-PAGE-2020 - Eve of Christmas Day; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2020, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2020"),
        // 2020-12-25 - T1 - HKEX-TC-PAGE-2020 - Christmas Day.
        (2020, 12, 25, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2020-12-31 - T1 - HKEX-TC-PAGE-2020 - Eve of New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2020, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2020"),
        // HKEX-TC-PAGE-2021: the operator's own holiday schedule
        // 2021-01-01 - T1 - HKEX-TC-PAGE-2021 - The first day of January.
        (2021, 1, 1, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-02-11 - T1 - HKEX-TC-PAGE-2021 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2021, 2, 11, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2021"),
        // 2021-02-12 - T1 - HKEX-TC-PAGE-2021 - Lunar New Year's Day.
        (2021, 2, 12, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-02-15 - T1 - HKEX-TC-PAGE-2021 - The fourth day of Lunar New Year.
        (2021, 2, 15, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-04-02 - T1 - HKEX-TC-PAGE-2021 - Good Friday.
        (2021, 4, 2, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-04-05 - T1 - HKEX-TC-PAGE-2021 - The day following Ching Ming Festival.
        (2021, 4, 5, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-04-06 - T1 - HKEX-TC-PAGE-2021 - The day following Easter Monday.
        (2021, 4, 6, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-05-19 - T1 - HKEX-TC-PAGE-2021 - Birthday of the Buddha.
        (2021, 5, 19, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-06-14 - T1 - HKEX-TC-PAGE-2021 - Tuen Ng Festival.
        (2021, 6, 14, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-07-01 - T1 - HKEX-TC-PAGE-2021 - Hong Kong Special Administrative Region Establishment Day.
        (2021, 7, 1, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-09-22 - T1 - HKEX-TC-PAGE-2021 - The day following the Chinese Mid-Autumn Festival.
        (2021, 9, 22, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-10-01 - T1 - HKEX-TC-PAGE-2021 - National Day.
        (2021, 10, 1, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-10-14 - T1 - HKEX-TC-PAGE-2021 - Chung Yeung Festival.
        (2021, 10, 14, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-12-24 - T1 - HKEX-TC-PAGE-2021 - Eve of Christmas Day; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2021, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2021"),
        // 2021-12-27 - T1 - HKEX-TC-PAGE-2021 - The first weekday after Christmas Day.
        (2021, 12, 27, Closed, T1, "HKEX-TC-PAGE-2021"),
        // 2021-12-31 - T1 - HKEX-TC-PAGE-2021 - Eve of New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2021, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2021"),
        // HKEX-TC-PAGE-2022: the operator's own holiday schedule
        // 2022-01-31 - T1 - HKEX-TC-PAGE-2022 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2022, 1, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2022"),
        // 2022-02-01 - T1 - HKEX-TC-PAGE-2022 - Lunar New Year's Day.
        (2022, 2, 1, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-02-02 - T1 - HKEX-TC-PAGE-2022 - The Second day of Lunar New Year.
        (2022, 2, 2, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-02-03 - T1 - HKEX-TC-PAGE-2022 - The Third day of Lunar New Year.
        (2022, 2, 3, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-04-05 - T1 - HKEX-TC-PAGE-2022 - Ching Ming Festival.
        (2022, 4, 5, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-04-15 - T1 - HKEX-TC-PAGE-2022 - Good Friday.
        (2022, 4, 15, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-04-18 - T1 - HKEX-TC-PAGE-2022 - Easter Monday.
        (2022, 4, 18, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-05-02 - T1 - HKEX-TC-PAGE-2022 - The day following Labour Day.
        (2022, 5, 2, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-05-09 - T1 - HKEX-TC-PAGE-2022 - The day following the Birthday of the Buddha.
        (2022, 5, 9, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-06-03 - T1 - HKEX-TC-PAGE-2022 - Tuen Ng Festival.
        (2022, 6, 3, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-07-01 - T1 - HKEX-TC-PAGE-2022 - Hong Kong Special Administrative Region Establishment Day.
        (2022, 7, 1, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-09-12 - T1 - HKEX-TC-PAGE-2022 - The second day following the Chinese Mid-Autumn Festival.
        (2022, 9, 12, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-10-04 - T1 - HKEX-TC-PAGE-2022 - Chung Yeung Festival.
        (2022, 10, 4, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-12-26 - T1 - HKEX-TC-PAGE-2022 - The first weekday after Christmas Day.
        (2022, 12, 26, Closed, T1, "HKEX-TC-PAGE-2022"),
        // 2022-12-27 - T1 - HKEX-TC-PAGE-2022 - The second weekday after Christmas Day.
        (2022, 12, 27, Closed, T1, "HKEX-TC-PAGE-2022"),
        // HKEX-TC-PAGE-2023: the operator's own holiday schedule
        // 2023-01-02 - T1 - HKEX-TC-PAGE-2023 - The day following the first day of January.
        (2023, 1, 2, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-01-23 - T1 - HKEX-TC-PAGE-2023 - The second day of Lunar New Year.
        (2023, 1, 23, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-01-24 - T1 - HKEX-TC-PAGE-2023 - The third day of Lunar New Year.
        (2023, 1, 24, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-01-25 - T1 - HKEX-TC-PAGE-2023 - The fourth day of Lunar New Year.
        (2023, 1, 25, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-04-05 - T1 - HKEX-TC-PAGE-2023 - Ching Ming Festival.
        (2023, 4, 5, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-04-07 - T1 - HKEX-TC-PAGE-2023 - Good Friday.
        (2023, 4, 7, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-04-10 - T1 - HKEX-TC-PAGE-2023 - Easter Monday.
        (2023, 4, 10, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-05-01 - T1 - HKEX-TC-PAGE-2023 - Labour Day.
        (2023, 5, 1, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-05-26 - T1 - HKEX-TC-PAGE-2023 - The Birthday of the Buddha.
        (2023, 5, 26, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-06-22 - T1 - HKEX-TC-PAGE-2023 - Tuen Ng Festival.
        (2023, 6, 22, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-10-02 - T1 - HKEX-TC-PAGE-2023 - The day following of National Day.
        (2023, 10, 2, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-10-23 - T1 - HKEX-TC-PAGE-2023 - Chung Yeung Festival.
        (2023, 10, 23, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-12-25 - T1 - HKEX-TC-PAGE-2023 - Christmas Day.
        (2023, 12, 25, Closed, T1, "HKEX-TC-PAGE-2023"),
        // 2023-12-26 - T1 - HKEX-TC-PAGE-2023 - The first weekday after Christmas Day.
        (2023, 12, 26, Closed, T1, "HKEX-TC-PAGE-2023"),
        // HKEX-TC-PAGE-2024: the operator's own holiday schedule
        // 2024-01-01 - T1 - HKEX-TC-PAGE-2024 - The first day of January.
        (2024, 1, 1, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-02-09 - T1 - HKEX-TC-PAGE-2024 - Eve of Lunar New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2024, 2, 9, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2024"),
        // 2024-02-12 - T1 - HKEX-TC-PAGE-2024 - The third day of Lunar New Year.
        (2024, 2, 12, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-02-13 - T1 - HKEX-TC-PAGE-2024 - The fourth day of Lunar New Year.
        (2024, 2, 13, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-03-29 - T1 - HKEX-TC-PAGE-2024 - Good Friday.
        (2024, 3, 29, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-04-01 - T1 - HKEX-TC-PAGE-2024 - Easter Monday.
        (2024, 4, 1, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-04-04 - T1 - HKEX-TC-PAGE-2024 - Ching Ming Festival.
        (2024, 4, 4, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-05-01 - T1 - HKEX-TC-PAGE-2024 - Labour Day.
        (2024, 5, 1, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-05-15 - T1 - HKEX-TC-PAGE-2024 - The Birthday of the Buddha.
        (2024, 5, 15, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-06-10 - T1 - HKEX-TC-PAGE-2024 - Tuen Ng Festival.
        (2024, 6, 10, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-07-01 - T1 - HKEX-TC-PAGE-2024 - Hong Kong Special Administrative Region Establishment Day.
        (2024, 7, 1, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-09-18 - T1 - HKEX-TC-PAGE-2024 - The day following the Chinese Mid-Autumn Festival.
        (2024, 9, 18, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-10-01 - T1 - HKEX-TC-PAGE-2024 - National Day.
        (2024, 10, 1, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-10-11 - T1 - HKEX-TC-PAGE-2024 - Chung Yeung Festival.
        (2024, 10, 11, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-12-24 - T1 - HKEX-TC-PAGE-2024 - Eve of Christmas Day; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2024, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2024"),
        // 2024-12-25 - T1 - HKEX-TC-PAGE-2024 - Christmas Day.
        (2024, 12, 25, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-12-26 - T1 - HKEX-TC-PAGE-2024 - The first weekday after Christmas Day.
        (2024, 12, 26, Closed, T1, "HKEX-TC-PAGE-2024"),
        // 2024-12-31 - T1 - HKEX-TC-PAGE-2024 - Eve of New Year; no afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2024, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-PAGE-2024"),
        // 2025-01-01 - T1 - HKEX-TC-2025 - The first day of January.
        (2025, 1, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-01-28 - T1 - HKEX-TC-2025 - Eve of Lunar New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2025, 1, 28, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2025"),
        // 2025-01-29 - T1 - HKEX-TC-2025 - Lunar New Year's Day.
        (2025, 1, 29, Closed, T1, "HKEX-TC-2025"),
        // 2025-01-30 - T1 - HKEX-TC-2025 - The second day of Lunar New Year.
        (2025, 1, 30, Closed, T1, "HKEX-TC-2025"),
        // 2025-01-31 - T1 - HKEX-TC-2025 - The third day of Lunar New Year.
        (2025, 1, 31, Closed, T1, "HKEX-TC-2025"),
        // 2025-04-04 - T1 - HKEX-TC-2025 - Ching Ming Festival.
        (2025, 4, 4, Closed, T1, "HKEX-TC-2025"),
        // 2025-04-18 - T1 - HKEX-TC-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "HKEX-TC-2025"),
        // 2025-04-21 - T1 - HKEX-TC-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "HKEX-TC-2025"),
        // 2025-05-01 - T1 - HKEX-TC-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-05-05 - T1 - HKEX-TC-2025 - The Birthday of the Buddha.
        (2025, 5, 5, Closed, T1, "HKEX-TC-2025"),
        // 2025-07-01 - T1 - HKEX-TC-2025 - HKSAR Establishment Day.
        (2025, 7, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-10-01 - T1 - HKEX-TC-2025 - National Day.
        (2025, 10, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-10-07 - T1 - HKEX-TC-2025 - The day following the Chinese
        // Mid-Autumn Festival.
        (2025, 10, 7, Closed, T1, "HKEX-TC-2025"),
        // 2025-10-29 - T1 - HKEX-TC-2025 - Chung Yeung Festival.
        (2025, 10, 29, Closed, T1, "HKEX-TC-2025"),
        // 2025-12-24 - T1 - HKEX-TC-2025 - Eve of Christmas Day: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2025, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2025"),
        // 2025-12-25 - T1 - HKEX-TC-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "HKEX-TC-2025"),
        // 2025-12-26 - T1 - HKEX-TC-2025 - The first weekday after Christmas Day.
        (2025, 12, 26, Closed, T1, "HKEX-TC-2025"),
        // 2025-12-31 - T1 - HKEX-TC-2025 - Eve of New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2025, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2025"),
        // 2026-01-01 - T1 - HKEX-TC-2026-2027 - The first day of January.
        (2026, 1, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-02-16 - T1 - HKEX-TC-2026-2027 - Eve of Lunar New Year: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2026, 2, 16, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2026-02-17 - T1 - HKEX-TC-2026-2027 - Lunar New Year's Day.
        (2026, 2, 17, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-02-18 - T1 - HKEX-TC-2026-2027 - The second day of Lunar New Year.
        (2026, 2, 18, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-02-19 - T1 - HKEX-TC-2026-2027 - The third day of Lunar New Year.
        (2026, 2, 19, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-04-03 - T1 - HKEX-TC-2026-2027 - Good Friday.
        (2026, 4, 3, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-04-06 - T1 - HKEX-TC-2026-2027 - The day following Ching Ming
        // Festival.
        (2026, 4, 6, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-04-07 - T1 - HKEX-TC-2026-2027 - The day following Easter Monday.
        (2026, 4, 7, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-05-01 - T1 - HKEX-TC-2026-2027 - Labour Day.
        (2026, 5, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-05-25 - T1 - HKEX-TC-2026-2027 - The day following the Birthday
        // of the Buddha.
        (2026, 5, 25, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-06-19 - T1 - HKEX-TC-2026-2027 - Tuen Ng Festival.
        (2026, 6, 19, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-07-01 - T1 - HKEX-TC-2026-2027 - HKSAR Establishment Day.
        (2026, 7, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-10-01 - T1 - HKEX-TC-2026-2027 - National Day.
        (2026, 10, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-10-19 - T1 - HKEX-TC-2026-2027 - The day following Chung Yeung
        // Festival.
        (2026, 10, 19, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-12-24 - T1 - HKEX-TC-2026-2027 - Eve of Christmas Day: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2026, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2026-12-25 - T1 - HKEX-TC-2026-2027 - Christmas Day.
        (2026, 12, 25, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-12-31 - T1 - HKEX-TC-2026-2027 - Eve of New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2026, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2027-01-01 - T1 - HKEX-TC-2026-2027 - The first day of January.
        (2027, 1, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-02-05 - T1 - HKEX-TC-2026-2027 - Eve of Lunar New Year: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2027, 2, 5, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2027-02-08 - T1 - HKEX-TC-2026-2027 - The third day of Lunar New Year.
        (2027, 2, 8, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-02-09 - T1 - HKEX-TC-2026-2027 - The fourth day of Lunar New Year.
        (2027, 2, 9, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-03-26 - T1 - HKEX-TC-2026-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-03-29 - T1 - HKEX-TC-2026-2027 - Easter Monday.
        (2027, 3, 29, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-04-05 - T1 - HKEX-TC-2026-2027 - Ching Ming Festival.
        (2027, 4, 5, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-05-13 - T1 - HKEX-TC-2026-2027 - The Birthday of the Buddha.
        (2027, 5, 13, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-06-09 - T1 - HKEX-TC-2026-2027 - Tuen Ng Festival.
        (2027, 6, 9, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-07-01 - T1 - HKEX-TC-2026-2027 - HKSAR Establishment Day.
        (2027, 7, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-09-16 - T1 - HKEX-TC-2026-2027 - The day following the Chinese
        // Mid-Autumn Festival.
        (2027, 9, 16, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-10-01 - T1 - HKEX-TC-2026-2027 - National Day.
        (2027, 10, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-10-08 - T1 - HKEX-TC-2026-2027 - Chung Yeung Festival.
        (2027, 10, 8, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-12-24 - T1 - HKEX-TC-2026-2027 - Eve of Christmas Day: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2027, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2027-12-27 - T1 - HKEX-TC-2026-2027 - The first weekday after
        // Christmas Day (Christmas Eve-day falls on the 2027 weekend).
        (2027, 12, 27, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-12-31 - T1 - HKEX-TC-2026-2027 - Eve of New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2027, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
    ],
};
