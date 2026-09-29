// SPDX-License-Identifier: MIT-0

//! ASX cash-market holiday rows, 2010-2024 backfilled alongside the
//! operator's published 2025-2027 sheets.
//!
//! Keyed by the crate's venue-local trade date in `Australia/Sydney` (design
//! memo D1). ASX sessions do not wrap past local midnight, so every holiday
//! row lands on its own civil date. The operator has published one annual
//! `Trading calendar` sheet per year across the whole window, so every row is
//! read from that year's own operator sheet, per era:
//!
//! - **2010-2011**, the operator's `trading_calendar/asx/<year>` pages;
//! - **2012**, the operator's `trading_services/asx-trading-calendar-2012`
//!   page (the operator's only surviving capture of the 2012 sheet, published
//!   in-year in advance);
//! - **2013-2019**, the operator's `about/asx-trading-calendar-<year>` pages;
//! - **2020-2022**, the operator's `www2.asx.com.au` cash-market
//!   `trading-calendar` page, whose year selector server-renders each sheet;
//! - **2023-2027**, the same page on `www.asx.com.au` (the 2023 and 2024
//!   rows) and the live page (2025-2027).
//!
//! Every pre-2025 artifact is a Wayback `id_` replay of the operator's own
//! page — verbatim bytes of an operator statement, so T1
//! (LAW-PUBLIC-SOURCES). The per-row derivation is recorded in
//! [`docs/evidence/asx.md`](../../../../../docs/evidence/asx.md).
//!
//! The sheets' `Trading Day` column is the venue's own answer, and only its
//! `CLOSED` and `CLOSE EARLY` rows ship: a row that closes individual states
//! while printing `OPEN` (the Tasmanian cups, the WA and QLD March and
//! October days, the Melbourne Cup) is a settlement fact, not a market
//! closure. `CLOSE EARLY` states its own instant through the sheet's
//! footnote, `Normal trading ceases at 14:10 (Sydney time)`, so the row
//! clips the envelope at 14:10 and the 16:10-16:21:30 Post Close block is
//! gone with it. Three sheets print no early close at all — 2017 has no
//! `Last Business Day` rows, and the 2022 and 2023 sheets print both
//! year-end rows `OPEN` — so those years ship none, rather than carrying the
//! neighbouring years' pattern. Weekend `ANZAC Day` rows (2015 Saturday,
//! 2020 Saturday, 2021 Sunday) restate closures the Mon-Fri normal week has
//! already made and are shipped as printed, like the 2026 Saturday row.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// ASX's built-in holiday rows and the window they were audited over.
///
/// Every date inside the window with no row is audited normal: each sheet
/// names every closure and half day it observes for its year, the `OPEN`
/// rows it prints ship no row, and every printed date ships. 2010-2024 rows
/// cite that year's own operator sheet; 2025 rows cite the operator's page
/// as replayed 2025-04-16 (`ASX-CAL-2025`); 2026 and 2027 rows cite the live
/// page (`ASX-CAL-LIVE`), which renders both years' sheets.
// Evidence: docs/evidence/asx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - ASX-CAL-2010 - New Year's Day, `CLOSED`.
        (2010, 1, 1, Closed, T1, "ASX-CAL-2010"),
        // 2010-01-26 - T1 - ASX-CAL-2010 - Australia Day, `CLOSED`.
        (2010, 1, 26, Closed, T1, "ASX-CAL-2010"),
        // 2010-04-02 - T1 - ASX-CAL-2010 - Good Friday, `CLOSED`.
        (2010, 4, 2, Closed, T1, "ASX-CAL-2010"),
        // 2010-04-05 - T1 - ASX-CAL-2010 - Easter Monday, `CLOSED`.
        (2010, 4, 5, Closed, T1, "ASX-CAL-2010"),
        // 2010-04-26 - T1 - ASX-CAL-2010 - ANZAC Day Holiday (Monday, the
        // sheet's own observed date for the Sunday 25 April holiday),
        // `CLOSED`.
        (2010, 4, 26, Closed, T1, "ASX-CAL-2010"),
        // 2010-06-14 - T1 - ASX-CAL-2010 - Queen's Birthday, `CLOSED`.
        (2010, 6, 14, Closed, T1, "ASX-CAL-2010"),
        // 2010-12-24 - T1 - ASX-CAL-2010 - Last Business Day before Christmas
        // Day, `CLOSE EARLY`: `Normal trading ceases at 14:10 (Sydney time)`.
        (2010, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2010"),
        // 2010-12-27 - T1 - ASX-CAL-2010 - Christmas Day / Boxing Day
        // (Monday, the sheet's own observed date for the Saturday 25 December
        // holiday), `CLOSED`.
        (2010, 12, 27, Closed, T1, "ASX-CAL-2010"),
        // 2010-12-28 - T1 - ASX-CAL-2010 - Boxing Day / Christmas Day /
        // Proclamation Day (Tuesday, the sheet's own observed date),
        // `CLOSED`.
        (2010, 12, 28, Closed, T1, "ASX-CAL-2010"),
        // 2010-12-31 - T1 - ASX-CAL-2010 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2010, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2010"),
        // 2011-01-03 - T1 - ASX-CAL-2011 - New Year's Day (Monday, the
        // sheet's own observed date for the Saturday 1 January holiday),
        // `CLOSED`.
        (2011, 1, 3, Closed, T1, "ASX-CAL-2011"),
        // 2011-01-26 - T1 - ASX-CAL-2011 - Australia Day, `CLOSED`.
        (2011, 1, 26, Closed, T1, "ASX-CAL-2011"),
        // 2011-04-22 - T1 - ASX-CAL-2011 - Good Friday, `CLOSED`.
        (2011, 4, 22, Closed, T1, "ASX-CAL-2011"),
        // 2011-04-25 - T1 - ASX-CAL-2011 - Easter Monday / ANZAC Day Holiday,
        // printed as one date, `CLOSED`.
        (2011, 4, 25, Closed, T1, "ASX-CAL-2011"),
        // 2011-04-26 - T1 - ASX-CAL-2011 - Easter Tuesday / Public Holiday,
        // `CLOSED`: the sheet's own one-off congruence holiday, whose
        // footnote reads "In recognition of the congruence of Anzac Day and
        // Easter Monday".
        (2011, 4, 26, Closed, T1, "ASX-CAL-2011"),
        // 2011-06-13 - T1 - ASX-CAL-2011 - Queen's Birthday, `CLOSED`.
        (2011, 6, 13, Closed, T1, "ASX-CAL-2011"),
        // 2011-12-23 - T1 - ASX-CAL-2011 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2011, 12, 23, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2011"),
        // 2011-12-26 - T1 - ASX-CAL-2011 - Christmas Day / Boxing Day
        // (Monday, the sheet's own observed date for the Saturday 25 December
        // holiday), `CLOSED`.
        (2011, 12, 26, Closed, T1, "ASX-CAL-2011"),
        // 2011-12-27 - T1 - ASX-CAL-2011 - Boxing Day / Proclamation Day /
        // Christmas Day (Tuesday, the sheet's own observed date), `CLOSED`.
        (2011, 12, 27, Closed, T1, "ASX-CAL-2011"),
        // 2011-12-30 - T1 - ASX-CAL-2011 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2011, 12, 30, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2011"),
        // 2012-01-02 - T1 - ASX-CAL-2012 - New Year's Day (Monday, the
        // sheet's own observed date for the Sunday 1 January holiday),
        // `CLOSED`.
        (2012, 1, 2, Closed, T1, "ASX-CAL-2012"),
        // 2012-01-26 - T1 - ASX-CAL-2012 - Australia Day, `CLOSED`.
        (2012, 1, 26, Closed, T1, "ASX-CAL-2012"),
        // 2012-04-06 - T1 - ASX-CAL-2012 - Good Friday, `CLOSED`.
        (2012, 4, 6, Closed, T1, "ASX-CAL-2012"),
        // 2012-04-09 - T1 - ASX-CAL-2012 - Easter Monday, `CLOSED`.
        (2012, 4, 9, Closed, T1, "ASX-CAL-2012"),
        // 2012-04-25 - T1 - ASX-CAL-2012 - ANZAC Day, `CLOSED`.
        (2012, 4, 25, Closed, T1, "ASX-CAL-2012"),
        // 2012-06-11 - T1 - ASX-CAL-2012 - Queen's Birthday, `CLOSED`.
        (2012, 6, 11, Closed, T1, "ASX-CAL-2012"),
        // 2012-12-24 - T1 - ASX-CAL-2012 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2012, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2012"),
        // 2012-12-25 - T1 - ASX-CAL-2012 - Christmas Day, `CLOSED`.
        (2012, 12, 25, Closed, T1, "ASX-CAL-2012"),
        // 2012-12-26 - T1 - ASX-CAL-2012 - Boxing Day / Proclamation Day,
        // `CLOSED`.
        (2012, 12, 26, Closed, T1, "ASX-CAL-2012"),
        // 2012-12-31 - T1 - ASX-CAL-2012 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2012, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2012"),
        // 2013-01-01 - T1 - ASX-CAL-2013 - New Year's Day, `CLOSED`.
        (2013, 1, 1, Closed, T1, "ASX-CAL-2013"),
        // 2013-01-28 - T1 - ASX-CAL-2013 - Australia Day (Monday, the
        // sheet's own observed date for the Saturday 26 January holiday),
        // `CLOSED`.
        (2013, 1, 28, Closed, T1, "ASX-CAL-2013"),
        // 2013-03-29 - T1 - ASX-CAL-2013 - Good Friday, `CLOSED`.
        (2013, 3, 29, Closed, T1, "ASX-CAL-2013"),
        // 2013-04-01 - T1 - ASX-CAL-2013 - Easter Monday, `CLOSED`.
        (2013, 4, 1, Closed, T1, "ASX-CAL-2013"),
        // 2013-04-25 - T1 - ASX-CAL-2013 - ANZAC Day, `CLOSED`.
        (2013, 4, 25, Closed, T1, "ASX-CAL-2013"),
        // 2013-06-10 - T1 - ASX-CAL-2013 - Queen's Birthday, `CLOSED`.
        (2013, 6, 10, Closed, T1, "ASX-CAL-2013"),
        // 2013-12-24 - T1 - ASX-CAL-2013 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2013, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2013"),
        // 2013-12-25 - T1 - ASX-CAL-2013 - Christmas Day, `CLOSED`.
        (2013, 12, 25, Closed, T1, "ASX-CAL-2013"),
        // 2013-12-26 - T1 - ASX-CAL-2013 - Boxing Day / Proclamation Day,
        // `CLOSED`.
        (2013, 12, 26, Closed, T1, "ASX-CAL-2013"),
        // 2013-12-31 - T1 - ASX-CAL-2013 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2013, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2013"),
        // 2014-01-01 - T1 - ASX-CAL-2014 - New Year's Day, `CLOSED`.
        (2014, 1, 1, Closed, T1, "ASX-CAL-2014"),
        // 2014-01-27 - T1 - ASX-CAL-2014 - Australia Day (Monday, the
        // sheet's own observed date for the Sunday 26 January holiday),
        // `CLOSED`.
        (2014, 1, 27, Closed, T1, "ASX-CAL-2014"),
        // 2014-04-18 - T1 - ASX-CAL-2014 - Good Friday, `CLOSED`.
        (2014, 4, 18, Closed, T1, "ASX-CAL-2014"),
        // 2014-04-21 - T1 - ASX-CAL-2014 - Easter Monday, `CLOSED`.
        (2014, 4, 21, Closed, T1, "ASX-CAL-2014"),
        // 2014-04-25 - T1 - ASX-CAL-2014 - ANZAC Day, `CLOSED`.
        (2014, 4, 25, Closed, T1, "ASX-CAL-2014"),
        // 2014-06-09 - T1 - ASX-CAL-2014 - Queen's Birthday, `CLOSED`.
        (2014, 6, 9, Closed, T1, "ASX-CAL-2014"),
        // 2014-12-24 - T1 - ASX-CAL-2014 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2014, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2014"),
        // 2014-12-25 - T1 - ASX-CAL-2014 - Christmas Day, `CLOSED`.
        (2014, 12, 25, Closed, T1, "ASX-CAL-2014"),
        // 2014-12-26 - T1 - ASX-CAL-2014 - Boxing Day / Proclamation Day,
        // `CLOSED`.
        (2014, 12, 26, Closed, T1, "ASX-CAL-2014"),
        // 2014-12-31 - T1 - ASX-CAL-2014 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2014, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2014"),
        // 2015-01-01 - T1 - ASX-CAL-2015 - New Year's Day, `CLOSED`.
        (2015, 1, 1, Closed, T1, "ASX-CAL-2015"),
        // 2015-01-26 - T1 - ASX-CAL-2015 - Australia Day, `CLOSED`.
        (2015, 1, 26, Closed, T1, "ASX-CAL-2015"),
        // 2015-04-03 - T1 - ASX-CAL-2015 - Good Friday, `CLOSED`.
        (2015, 4, 3, Closed, T1, "ASX-CAL-2015"),
        // 2015-04-06 - T1 - ASX-CAL-2015 - Easter Monday, `CLOSED`.
        (2015, 4, 6, Closed, T1, "ASX-CAL-2015"),
        // 2015-04-25 - T1 - ASX-CAL-2015 - ANZAC Day (Saturday, as printed),
        // `CLOSED`: the row restates the closure the Mon-Fri normal week has
        // already made.
        (2015, 4, 25, Closed, T1, "ASX-CAL-2015"),
        // 2015-06-08 - T1 - ASX-CAL-2015 - Queen's Birthday, `CLOSED`.
        (2015, 6, 8, Closed, T1, "ASX-CAL-2015"),
        // 2015-12-24 - T1 - ASX-CAL-2015 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2015, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2015"),
        // 2015-12-25 - T1 - ASX-CAL-2015 - Christmas Day, `CLOSED`.
        (2015, 12, 25, Closed, T1, "ASX-CAL-2015"),
        // 2015-12-28 - T1 - ASX-CAL-2015 - Boxing Day / Proclamation Day
        // (Monday, the sheet's own observed date for the Saturday 26 December
        // holiday), `CLOSED`.
        (2015, 12, 28, Closed, T1, "ASX-CAL-2015"),
        // 2015-12-31 - T1 - ASX-CAL-2015 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2015, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2015"),
        // 2016-01-01 - T1 - ASX-CAL-2016 - New Year's Day, `CLOSED`.
        (2016, 1, 1, Closed, T1, "ASX-CAL-2016"),
        // 2016-01-26 - T1 - ASX-CAL-2016 - Australia Day, `CLOSED`.
        (2016, 1, 26, Closed, T1, "ASX-CAL-2016"),
        // 2016-03-25 - T1 - ASX-CAL-2016 - Good Friday, `CLOSED`.
        (2016, 3, 25, Closed, T1, "ASX-CAL-2016"),
        // 2016-03-28 - T1 - ASX-CAL-2016 - Easter Monday, `CLOSED`.
        (2016, 3, 28, Closed, T1, "ASX-CAL-2016"),
        // 2016-04-25 - T1 - ASX-CAL-2016 - ANZAC Day, `CLOSED`.
        (2016, 4, 25, Closed, T1, "ASX-CAL-2016"),
        // 2016-06-13 - T1 - ASX-CAL-2016 - Queen's Birthday, `CLOSED`.
        (2016, 6, 13, Closed, T1, "ASX-CAL-2016"),
        // 2016-12-23 - T1 - ASX-CAL-2016 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2016, 12, 23, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2016"),
        // 2016-12-26 - T1 - ASX-CAL-2016 - Christmas Day (Monday, the
        // sheet's own observed date for the Sunday 25 December holiday),
        // `CLOSED`.
        (2016, 12, 26, Closed, T1, "ASX-CAL-2016"),
        // 2016-12-27 - T1 - ASX-CAL-2016 - Boxing Day / Proclamation Day
        // (Tuesday, the sheet's own observed date), `CLOSED`.
        (2016, 12, 27, Closed, T1, "ASX-CAL-2016"),
        // 2016-12-30 - T1 - ASX-CAL-2016 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2016, 12, 30, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2016"),
        // 2017-01-02 - T1 - ASX-CAL-2017 - New Year's Day (Monday, the
        // sheet's own substitute for the Sunday holiday; footnote "When New
        // Year's Day falls on a Sunday"), `CLOSED`.
        (2017, 1, 2, Closed, T1, "ASX-CAL-2017"),
        // 2017-01-26 - T1 - ASX-CAL-2017 - Australia Day, `CLOSED`.
        (2017, 1, 26, Closed, T1, "ASX-CAL-2017"),
        // 2017-04-14 - T1 - ASX-CAL-2017 - Good Friday, `CLOSED`.
        (2017, 4, 14, Closed, T1, "ASX-CAL-2017"),
        // 2017-04-17 - T1 - ASX-CAL-2017 - Easter Monday, `CLOSED`.
        (2017, 4, 17, Closed, T1, "ASX-CAL-2017"),
        // 2017-04-25 - T1 - ASX-CAL-2017 - ANZAC Day, `CLOSED`.
        (2017, 4, 25, Closed, T1, "ASX-CAL-2017"),
        // 2017-06-12 - T1 - ASX-CAL-2017 - Queen's Birthday, `CLOSED`.
        (2017, 6, 12, Closed, T1, "ASX-CAL-2017"),
        // 2017-12-25 - T1 - ASX-CAL-2017 - Christmas Day, `CLOSED`.
        (2017, 12, 25, Closed, T1, "ASX-CAL-2017"),
        // 2017-12-26 - T1 - ASX-CAL-2017 - Boxing Day / Proclamation Day /
        // Christmas Day (Tuesday, the sheet's own observed date for the
        // Monday 25 December holiday in the states that observe it then),
        // `CLOSED`. The 2017 sheet prints no `Last Business Day` rows, so no
        // early close ships.
        (2017, 12, 26, Closed, T1, "ASX-CAL-2017"),
        // 2018-01-01 - T1 - ASX-CAL-2018 - New Year's Day, `CLOSED`.
        (2018, 1, 1, Closed, T1, "ASX-CAL-2018"),
        // 2018-01-26 - T1 - ASX-CAL-2018 - Australia Day, `CLOSED`.
        (2018, 1, 26, Closed, T1, "ASX-CAL-2018"),
        // 2018-03-30 - T1 - ASX-CAL-2018 - Good Friday, `CLOSED`.
        (2018, 3, 30, Closed, T1, "ASX-CAL-2018"),
        // 2018-04-02 - T1 - ASX-CAL-2018 - Easter Monday, `CLOSED`.
        (2018, 4, 2, Closed, T1, "ASX-CAL-2018"),
        // 2018-04-25 - T1 - ASX-CAL-2018 - ANZAC Day, `CLOSED`.
        (2018, 4, 25, Closed, T1, "ASX-CAL-2018"),
        // 2018-06-11 - T1 - ASX-CAL-2018 - Queen's Birthday, `CLOSED`.
        (2018, 6, 11, Closed, T1, "ASX-CAL-2018"),
        // 2018-12-24 - T1 - ASX-CAL-2018 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2018, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2018"),
        // 2018-12-25 - T1 - ASX-CAL-2018 - Christmas Day, `CLOSED`.
        (2018, 12, 25, Closed, T1, "ASX-CAL-2018"),
        // 2018-12-26 - T1 - ASX-CAL-2018 - Boxing Day / Proclamation Day,
        // `CLOSED`.
        (2018, 12, 26, Closed, T1, "ASX-CAL-2018"),
        // 2018-12-31 - T1 - ASX-CAL-2018 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2018, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2018"),
        // 2019-01-01 - T1 - ASX-CAL-2019 - New Year's Day, `CLOSED`.
        (2019, 1, 1, Closed, T1, "ASX-CAL-2019"),
        // 2019-01-28 - T1 - ASX-CAL-2019 - Australia Day (Monday, the
        // sheet's own observed date for the Saturday 26 January holiday),
        // `CLOSED`.
        (2019, 1, 28, Closed, T1, "ASX-CAL-2019"),
        // 2019-04-19 - T1 - ASX-CAL-2019 - Good Friday, `CLOSED`.
        (2019, 4, 19, Closed, T1, "ASX-CAL-2019"),
        // 2019-04-22 - T1 - ASX-CAL-2019 - Easter Monday, `CLOSED`.
        (2019, 4, 22, Closed, T1, "ASX-CAL-2019"),
        // 2019-04-25 - T1 - ASX-CAL-2019 - ANZAC Day, `CLOSED`.
        (2019, 4, 25, Closed, T1, "ASX-CAL-2019"),
        // 2019-06-10 - T1 - ASX-CAL-2019 - Queen's Birthday, `CLOSED`.
        (2019, 6, 10, Closed, T1, "ASX-CAL-2019"),
        // 2019-12-24 - T1 - ASX-CAL-2019 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2019, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2019"),
        // 2019-12-25 - T1 - ASX-CAL-2019 - Christmas Day, `CLOSED`.
        (2019, 12, 25, Closed, T1, "ASX-CAL-2019"),
        // 2019-12-26 - T1 - ASX-CAL-2019 - Boxing Day / Proclamation Day,
        // `CLOSED`.
        (2019, 12, 26, Closed, T1, "ASX-CAL-2019"),
        // 2019-12-31 - T1 - ASX-CAL-2019 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2019, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2019"),
        // 2020-01-01 - T1 - ASX-CAL-2020 - New Year's Day, `CLOSED`.
        (2020, 1, 1, Closed, T1, "ASX-CAL-2020"),
        // 2020-01-27 - T1 - ASX-CAL-2020 - Australia Day (Monday, the
        // sheet's own observed date for the Sunday 26 January holiday),
        // `CLOSED`.
        (2020, 1, 27, Closed, T1, "ASX-CAL-2020"),
        // 2020-04-10 - T1 - ASX-CAL-2020 - Good Friday, `CLOSED`.
        (2020, 4, 10, Closed, T1, "ASX-CAL-2020"),
        // 2020-04-13 - T1 - ASX-CAL-2020 - Easter Monday, `CLOSED`.
        (2020, 4, 13, Closed, T1, "ASX-CAL-2020"),
        // 2020-04-25 - T1 - ASX-CAL-2020 - ANZAC Day (Saturday, as printed),
        // `CLOSED`, with the sheet's own footnote "No substitute holiday on
        // Monday 27th April".
        (2020, 4, 25, Closed, T1, "ASX-CAL-2020"),
        // 2020-06-08 - T1 - ASX-CAL-2020 - Queen's Birthday, `CLOSED`.
        (2020, 6, 8, Closed, T1, "ASX-CAL-2020"),
        // 2020-12-24 - T1 - ASX-CAL-2020 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2020, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2020"),
        // 2020-12-25 - T1 - ASX-CAL-2020 - Christmas Day, `CLOSED`.
        (2020, 12, 25, Closed, T1, "ASX-CAL-2020"),
        // 2020-12-28 - T1 - ASX-CAL-2020 - Boxing Day (Monday, the sheet's
        // own observed date for the Saturday 26 December holiday), `CLOSED`.
        (2020, 12, 28, Closed, T1, "ASX-CAL-2020"),
        // 2020-12-31 - T1 - ASX-CAL-2020 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2020, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2020"),
        // 2021-01-01 - T1 - ASX-CAL-2021 - New Year's Day, `CLOSED`.
        (2021, 1, 1, Closed, T1, "ASX-CAL-2021"),
        // 2021-01-26 - T1 - ASX-CAL-2021 - Australia Day, `CLOSED`.
        (2021, 1, 26, Closed, T1, "ASX-CAL-2021"),
        // 2021-04-02 - T1 - ASX-CAL-2021 - Good Friday, `CLOSED`.
        (2021, 4, 2, Closed, T1, "ASX-CAL-2021"),
        // 2021-04-05 - T1 - ASX-CAL-2021 - Easter Monday, `CLOSED`.
        (2021, 4, 5, Closed, T1, "ASX-CAL-2021"),
        // 2021-04-25 - T1 - ASX-CAL-2021 - ANZAC Day (Sunday, as printed),
        // `CLOSED`: the row restates the closure the Mon-Fri normal week has
        // already made.
        (2021, 4, 25, Closed, T1, "ASX-CAL-2021"),
        // 2021-06-14 - T1 - ASX-CAL-2021 - Queen's Birthday, `CLOSED`.
        (2021, 6, 14, Closed, T1, "ASX-CAL-2021"),
        // 2021-12-24 - T1 - ASX-CAL-2021 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2021, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2021"),
        // 2021-12-27 - T1 - ASX-CAL-2021 - Christmas Day (Monday, the
        // sheet's own substitute for the Saturday 25 December holiday),
        // `CLOSED`.
        (2021, 12, 27, Closed, T1, "ASX-CAL-2021"),
        // 2021-12-28 - T1 - ASX-CAL-2021 - Boxing Day (Tuesday, the sheet's
        // own substitute for the Sunday 26 December holiday), `CLOSED`.
        (2021, 12, 28, Closed, T1, "ASX-CAL-2021"),
        // 2021-12-31 - T1 - ASX-CAL-2021 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2021, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2021"),
        // 2022-01-03 - T1 - ASX-CAL-2022 - New Year's Day (Monday, the
        // sheet's own substitute for the Saturday 1 January holiday),
        // `CLOSED`.
        (2022, 1, 3, Closed, T1, "ASX-CAL-2022"),
        // 2022-01-26 - T1 - ASX-CAL-2022 - Australia Day, `CLOSED`.
        (2022, 1, 26, Closed, T1, "ASX-CAL-2022"),
        // 2022-04-15 - T1 - ASX-CAL-2022 - Good Friday, `CLOSED`.
        (2022, 4, 15, Closed, T1, "ASX-CAL-2022"),
        // 2022-04-18 - T1 - ASX-CAL-2022 - Easter Monday, `CLOSED`.
        (2022, 4, 18, Closed, T1, "ASX-CAL-2022"),
        // 2022-04-25 - T1 - ASX-CAL-2022 - ANZAC Day, `CLOSED`.
        (2022, 4, 25, Closed, T1, "ASX-CAL-2022"),
        // 2022-06-13 - T1 - ASX-CAL-2022 - Queen's Birthday, `CLOSED`.
        (2022, 6, 13, Closed, T1, "ASX-CAL-2022"),
        // 2022-09-22 - T1 - ASX-CAL-2022 - National Day of Mourning for Her
        // Majesty the Queen, `CLOSED`: the sheet's own unscheduled closure,
        // added to the sheet between its July and October 2022 replays.
        (2022, 9, 22, Closed, T1, "ASX-CAL-2022"),
        // 2022-12-26 - T1 - ASX-CAL-2022 - Boxing Day, `CLOSED`.
        (2022, 12, 26, Closed, T1, "ASX-CAL-2022"),
        // 2022-12-27 - T1 - ASX-CAL-2022 - Christmas Day (Tuesday, the
        // sheet's own substitute for the Sunday 25 December holiday),
        // `CLOSED`. The 2022 sheet prints both year-end rows `OPEN`, so no
        // early close ships.
        (2022, 12, 27, Closed, T1, "ASX-CAL-2022"),
        // 2023-01-02 - T1 - ASX-CAL-2023 - New Year's Day (Monday, the
        // sheet's own substitute for the Sunday 1 January holiday),
        // `CLOSED`.
        (2023, 1, 2, Closed, T1, "ASX-CAL-2023"),
        // 2023-01-26 - T1 - ASX-CAL-2023 - Australia Day, `CLOSED`.
        (2023, 1, 26, Closed, T1, "ASX-CAL-2023"),
        // 2023-04-07 - T1 - ASX-CAL-2023 - Good Friday, `CLOSED`.
        (2023, 4, 7, Closed, T1, "ASX-CAL-2023"),
        // 2023-04-10 - T1 - ASX-CAL-2023 - Easter Monday, `CLOSED`.
        (2023, 4, 10, Closed, T1, "ASX-CAL-2023"),
        // 2023-04-25 - T1 - ASX-CAL-2023 - ANZAC Day, `CLOSED`.
        (2023, 4, 25, Closed, T1, "ASX-CAL-2023"),
        // 2023-06-12 - T1 - ASX-CAL-2023 - King's Birthday, `CLOSED`.
        (2023, 6, 12, Closed, T1, "ASX-CAL-2023"),
        // 2023-12-25 - T1 - ASX-CAL-2023 - Christmas Day, `CLOSED`. The 2023
        // sheet prints both year-end rows `OPEN`, so no early close ships.
        (2023, 12, 25, Closed, T1, "ASX-CAL-2023"),
        // 2023-12-26 - T1 - ASX-CAL-2023 - Boxing Day, `CLOSED`.
        (2023, 12, 26, Closed, T1, "ASX-CAL-2023"),
        // 2024-01-01 - T1 - ASX-CAL-2024 - New Year's Day, `CLOSED`.
        (2024, 1, 1, Closed, T1, "ASX-CAL-2024"),
        // 2024-01-26 - T1 - ASX-CAL-2024 - Australia Day, `CLOSED`.
        (2024, 1, 26, Closed, T1, "ASX-CAL-2024"),
        // 2024-03-29 - T1 - ASX-CAL-2024 - Good Friday, `CLOSED`.
        (2024, 3, 29, Closed, T1, "ASX-CAL-2024"),
        // 2024-04-01 - T1 - ASX-CAL-2024 - Easter Monday, `CLOSED`.
        (2024, 4, 1, Closed, T1, "ASX-CAL-2024"),
        // 2024-04-25 - T1 - ASX-CAL-2024 - ANZAC Day, `CLOSED`.
        (2024, 4, 25, Closed, T1, "ASX-CAL-2024"),
        // 2024-06-10 - T1 - ASX-CAL-2024 - King's Birthday, `CLOSED`.
        (2024, 6, 10, Closed, T1, "ASX-CAL-2024"),
        // 2024-12-24 - T1 - ASX-CAL-2024 - Last Business Day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2024, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2024"),
        // 2024-12-25 - T1 - ASX-CAL-2024 - Christmas Day, `CLOSED`.
        (2024, 12, 25, Closed, T1, "ASX-CAL-2024"),
        // 2024-12-26 - T1 - ASX-CAL-2024 - Boxing Day, `CLOSED`.
        (2024, 12, 26, Closed, T1, "ASX-CAL-2024"),
        // 2024-12-31 - T1 - ASX-CAL-2024 - Last Business Day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2024, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2024"),
        // 2025-01-01 - T1 - ASX-CAL-2025 - New Year's Day, `CLOSED`.
        (2025, 1, 1, Closed, T1, "ASX-CAL-2025"),
        // 2025-01-27 - T1 - ASX-CAL-2025 - Australia Day, `CLOSED` (the
        // Monday after the Sunday 26 January holiday).
        (2025, 1, 27, Closed, T1, "ASX-CAL-2025"),
        // 2025-04-18 - T1 - ASX-CAL-2025 - Good Friday, `CLOSED`.
        (2025, 4, 18, Closed, T1, "ASX-CAL-2025"),
        // 2025-04-21 - T1 - ASX-CAL-2025 - Easter Monday, `CLOSED`.
        (2025, 4, 21, Closed, T1, "ASX-CAL-2025"),
        // 2025-04-25 - T1 - ASX-CAL-2025 - ANZAC Day, `CLOSED`.
        (2025, 4, 25, Closed, T1, "ASX-CAL-2025"),
        // 2025-06-09 - T1 - ASX-CAL-2025 - King's Birthday, `CLOSED`.
        (2025, 6, 9, Closed, T1, "ASX-CAL-2025"),
        // 2025-12-24 - T1 - ASX-CAL-2025 - Last Business day before Christmas
        // Day, `CLOSE EARLY`: `Normal trading ceases at 14:10 (Sydney time)`.
        (2025, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2025"),
        // 2025-12-25 - T1 - ASX-CAL-2025 - Christmas Day, `CLOSED`.
        (2025, 12, 25, Closed, T1, "ASX-CAL-2025"),
        // 2025-12-26 - T1 - ASX-CAL-2025 - Boxing Day, `CLOSED`.
        (2025, 12, 26, Closed, T1, "ASX-CAL-2025"),
        // 2025-12-31 - T1 - ASX-CAL-2025 - Last Business day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2025, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2025"),
        // 2026-01-01 - T1 - ASX-CAL-LIVE - New Year's Day, `CLOSED`.
        (2026, 1, 1, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-01-26 - T1 - ASX-CAL-LIVE - Australia Day, `CLOSED`.
        (2026, 1, 26, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-04-03 - T1 - ASX-CAL-LIVE - Good Friday, `CLOSED`.
        (2026, 4, 3, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-04-06 - T1 - ASX-CAL-LIVE - Easter Monday, `CLOSED`.
        (2026, 4, 6, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-04-25 - T1 - ASX-CAL-LIVE - ANZAC Day, `CLOSED`: the sheet
        // prints the Saturday with no substitute, so it restates the normal
        // week's closure.
        (2026, 4, 25, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-06-08 - T1 - ASX-CAL-LIVE - King's Birthday, `CLOSED`.
        (2026, 6, 8, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-12-24 - T1 - ASX-CAL-LIVE - Last Business day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2026, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
        // 2026-12-25 - T1 - ASX-CAL-LIVE - Christmas Day, `CLOSED`.
        (2026, 12, 25, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-12-28 - T1 - ASX-CAL-LIVE - Boxing Day, `CLOSED` (Monday, the
        // sheet's own date for the Saturday 26 December holiday).
        (2026, 12, 28, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-12-31 - T1 - ASX-CAL-LIVE - Last Business day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2026, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
        // 2027-01-01 - T1 - ASX-CAL-LIVE - New Year's Day, `CLOSED`.
        (2027, 1, 1, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-01-26 - T1 - ASX-CAL-LIVE - Australia Day, `CLOSED`.
        (2027, 1, 26, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-03-26 - T1 - ASX-CAL-LIVE - Good Friday, `CLOSED`.
        (2027, 3, 26, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-03-29 - T1 - ASX-CAL-LIVE - Easter Monday, `CLOSED`.
        (2027, 3, 29, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-06-14 - T1 - ASX-CAL-LIVE - King's Birthday, `CLOSED`.
        (2027, 6, 14, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-12-24 - T1 - ASX-CAL-LIVE - Last Business day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2027, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
        // 2027-12-27 - T1 - ASX-CAL-LIVE - Christmas Day, `CLOSED`: the
        // sheet's substitute for Saturday 25 December.
        (2027, 12, 27, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-12-28 - T1 - ASX-CAL-LIVE - Boxing Day, `CLOSED`: the sheet's
        // substitute for Sunday 26 December.
        (2027, 12, 28, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-12-31 - T1 - ASX-CAL-LIVE - Last Business day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2027, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
    ],
};
