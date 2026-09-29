// SPDX-License-Identifier: MIT-0

//! Tokyo Stock Exchange holiday rows, 2010-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Tokyo` (design
//! memo D1). TSE runs no overnight wrap and no Saturday sessions, so a closed
//! trade date is one whose daytime sessions are absent outright.
//!
//! The whole block is **T1**: the operator's own holiday page, which prints the
//! current and next year. The 2010-2014 tables are the TSE-era page
//! `tse.or.jp/english/about/calendar.html` ("Tokyo Stock Exchange is open five
//! days a week from Monday to Friday. The market will be closed on the
//! following national and observed holidays, and the market holidays of Jan. 2,
//! 3, and Dec. 31"), read from Wayback `id_` replays; the 2015-2017 tables are
//! the JPX page's `Exchange is closed on every Saturday, Sunday and the
//! following holidays:` editions, and the 2018-2027 tables the JPX page's
//! current form, whose scope sentence is "JPX markets are closed on Saturdays,
//! Sundays, national holidays, and on the dates indicated below". The rows
//! below are exactly the printed dates that fall on a weekday — a printed
//! holiday that lands on a Saturday or Sunday removes no session beyond the
//! normal week and ships no row, which keeps every shipped row an answer that
//! differs from the normal week. The 2010-2014 pages state Jan. 2, Jan. 3 and
//! Dec. 31 as market holidays in the scope sentence while the year's table
//! omits the weekday ones, so those two prose-stated weekday rows (2010-12-31
//! and 2011-01-03) ship from the same sentence. The derivation, per-row
//! quotations, the national-holiday observance notes and the edition lineages
//! are recorded in
//! [`docs/evidence/tse.md`](../../../../../docs/evidence/tse.md).
//!
//! JPX publishes no holiday-time early closes or late opens for the cash
//! market in any retrieved edition: every printed date is a full closure, and
//! the page's only conditional note ("Exchange holidays are subject to change
//! due to changes to national holidays under Japan's Act on National
//! Holidays") governs a legislative change that has not occurred inside this
//! window.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// TSE's built-in holiday rows and the window they were audited over.
///
/// Every row is one printed date of the operator's own holiday page: the
/// TSE-era editions `TSE-CAL-2010`..`TSE-CAL-2014` and the JPX editions
/// `JPX-HOL-2015`..`JPX-HOL-2024`, then `JPX-HOL-2025` for the 2025 table
/// (Wayback replay of 2025-09-23, page state "Update : Mar. 07, 2025") and
/// `JPX-HOL-2026-2027` for the 2026 and 2027 tables (live page, "Update :
/// Feb. 06, 2026"). A date inside the window with no row is audited normal.
// Evidence: docs/evidence/tse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - TSE-CAL-2010 - New Year's Day.
        (2010, 1, 1, Closed, T1, "TSE-CAL-2010"),
        // 2010-01-11 - T1 - TSE-CAL-2010 - Coming of Age Day.
        (2010, 1, 11, Closed, T1, "TSE-CAL-2010"),
        // 2010-02-11 - T1 - TSE-CAL-2010 - National Foundation Day.
        (2010, 2, 11, Closed, T1, "TSE-CAL-2010"),
        // 2010-03-22 - T1 - TSE-CAL-2010 - Holiday: printed observance day.
        (2010, 3, 22, Closed, T1, "TSE-CAL-2010"),
        // 2010-04-29 - T1 - TSE-CAL-2010 - Showa Day.
        (2010, 4, 29, Closed, T1, "TSE-CAL-2010"),
        // 2010-05-03 - T1 - TSE-CAL-2010 - Constitution Memorial Day.
        (2010, 5, 3, Closed, T1, "TSE-CAL-2010"),
        // 2010-05-04 - T1 - TSE-CAL-2010 - Greenery Day.
        (2010, 5, 4, Closed, T1, "TSE-CAL-2010"),
        // 2010-05-05 - T1 - TSE-CAL-2010 - Children's Day.
        (2010, 5, 5, Closed, T1, "TSE-CAL-2010"),
        // 2010-07-19 - T1 - TSE-CAL-2010 - Marine Day.
        (2010, 7, 19, Closed, T1, "TSE-CAL-2010"),
        // 2010-09-20 - T1 - TSE-CAL-2010 - Respect for the Aged Day.
        (2010, 9, 20, Closed, T1, "TSE-CAL-2010"),
        // 2010-09-23 - T1 - TSE-CAL-2010 - Autumnal equinox.
        (2010, 9, 23, Closed, T1, "TSE-CAL-2010"),
        // 2010-10-11 - T1 - TSE-CAL-2010 - Health and Sports Day.
        (2010, 10, 11, Closed, T1, "TSE-CAL-2010"),
        // 2010-11-03 - T1 - TSE-CAL-2010 - Culture Day.
        (2010, 11, 3, Closed, T1, "TSE-CAL-2010"),
        // 2010-11-23 - T1 - TSE-CAL-2010 - Labor Thanksgiving Day.
        (2010, 11, 23, Closed, T1, "TSE-CAL-2010"),
        // 2010-12-23 - T1 - TSE-CAL-2010 - Emperor's Birthday.
        (2010, 12, 23, Closed, T1, "TSE-CAL-2010"),
        // 2010-12-31 - T1 - TSE-CAL-2010 - Market Holiday (prose-stated).
        (2010, 12, 31, Closed, T1, "TSE-CAL-2010"),
        // 2011-01-03 - T1 - TSE-CAL-2011 - Market Holiday (prose-stated).
        (2011, 1, 3, Closed, T1, "TSE-CAL-2011"),
        // 2011-01-10 - T1 - TSE-CAL-2011 - Coming of Age Day.
        (2011, 1, 10, Closed, T1, "TSE-CAL-2011"),
        // 2011-02-11 - T1 - TSE-CAL-2011 - National Foundation Day.
        (2011, 2, 11, Closed, T1, "TSE-CAL-2011"),
        // 2011-03-21 - T1 - TSE-CAL-2011 - Vernal Equinox.
        (2011, 3, 21, Closed, T1, "TSE-CAL-2011"),
        // 2011-04-29 - T1 - TSE-CAL-2011 - Showa Day.
        (2011, 4, 29, Closed, T1, "TSE-CAL-2011"),
        // 2011-05-03 - T1 - TSE-CAL-2011 - Constitution Memorial Day.
        (2011, 5, 3, Closed, T1, "TSE-CAL-2011"),
        // 2011-05-04 - T1 - TSE-CAL-2011 - Greenery Day.
        (2011, 5, 4, Closed, T1, "TSE-CAL-2011"),
        // 2011-05-05 - T1 - TSE-CAL-2011 - Children's Day.
        (2011, 5, 5, Closed, T1, "TSE-CAL-2011"),
        // 2011-07-18 - T1 - TSE-CAL-2011 - Marine Day.
        (2011, 7, 18, Closed, T1, "TSE-CAL-2011"),
        // 2011-09-19 - T1 - TSE-CAL-2011 - Respect for the Aged Day.
        (2011, 9, 19, Closed, T1, "TSE-CAL-2011"),
        // 2011-09-23 - T1 - TSE-CAL-2011 - Autumnal equinox.
        (2011, 9, 23, Closed, T1, "TSE-CAL-2011"),
        // 2011-10-10 - T1 - TSE-CAL-2011 - Health and Sports Day.
        (2011, 10, 10, Closed, T1, "TSE-CAL-2011"),
        // 2011-11-03 - T1 - TSE-CAL-2011 - Culture Day.
        (2011, 11, 3, Closed, T1, "TSE-CAL-2011"),
        // 2011-11-23 - T1 - TSE-CAL-2011 - Labor Thanksgiving Day.
        (2011, 11, 23, Closed, T1, "TSE-CAL-2011"),
        // 2011-12-23 - T1 - TSE-CAL-2011 - Emperor's Birthday.
        (2011, 12, 23, Closed, T1, "TSE-CAL-2011"),
        // 2012-01-02 - T1 - TSE-CAL-2012 - Holiday: printed observance day.
        (2012, 1, 2, Closed, T1, "TSE-CAL-2012"),
        // 2012-01-03 - T1 - TSE-CAL-2012 - Exchange Holiday.
        (2012, 1, 3, Closed, T1, "TSE-CAL-2012"),
        // 2012-01-09 - T1 - TSE-CAL-2012 - Coming of Age Day.
        (2012, 1, 9, Closed, T1, "TSE-CAL-2012"),
        // 2012-03-20 - T1 - TSE-CAL-2012 - Vernal Equinox.
        (2012, 3, 20, Closed, T1, "TSE-CAL-2012"),
        // 2012-04-30 - T1 - TSE-CAL-2012 - Holiday: printed observance day.
        (2012, 4, 30, Closed, T1, "TSE-CAL-2012"),
        // 2012-05-03 - T1 - TSE-CAL-2012 - Constitution Memorial Day.
        (2012, 5, 3, Closed, T1, "TSE-CAL-2012"),
        // 2012-05-04 - T1 - TSE-CAL-2012 - Greenery Day.
        (2012, 5, 4, Closed, T1, "TSE-CAL-2012"),
        // 2012-07-16 - T1 - TSE-CAL-2012 - Marine Day.
        (2012, 7, 16, Closed, T1, "TSE-CAL-2012"),
        // 2012-09-17 - T1 - TSE-CAL-2012 - Respect for the Aged Day.
        (2012, 9, 17, Closed, T1, "TSE-CAL-2012"),
        // 2012-10-08 - T1 - TSE-CAL-2012 - Health and Sports Day.
        (2012, 10, 8, Closed, T1, "TSE-CAL-2012"),
        // 2012-11-23 - T1 - TSE-CAL-2012 - Labor Thanksgiving Day.
        (2012, 11, 23, Closed, T1, "TSE-CAL-2012"),
        // 2012-12-24 - T1 - TSE-CAL-2012 - Holiday: printed observance day.
        (2012, 12, 24, Closed, T1, "TSE-CAL-2012"),
        // 2012-12-31 - T1 - TSE-CAL-2012 - Exchange Holiday.
        (2012, 12, 31, Closed, T1, "TSE-CAL-2012"),
        // 2013-01-01 - T1 - TSE-CAL-2013 - New Year's Day.
        (2013, 1, 1, Closed, T1, "TSE-CAL-2013"),
        // 2013-01-02 - T1 - TSE-CAL-2013 - Exchange Holiday.
        (2013, 1, 2, Closed, T1, "TSE-CAL-2013"),
        // 2013-01-03 - T1 - TSE-CAL-2013 - Exchange Holiday.
        (2013, 1, 3, Closed, T1, "TSE-CAL-2013"),
        // 2013-01-14 - T1 - TSE-CAL-2013 - Coming of Age Day.
        (2013, 1, 14, Closed, T1, "TSE-CAL-2013"),
        // 2013-02-11 - T1 - TSE-CAL-2013 - National Foundation Day.
        (2013, 2, 11, Closed, T1, "TSE-CAL-2013"),
        // 2013-03-20 - T1 - TSE-CAL-2013 - Vernal Equinox.
        (2013, 3, 20, Closed, T1, "TSE-CAL-2013"),
        // 2013-04-29 - T1 - TSE-CAL-2013 - Showa Day.
        (2013, 4, 29, Closed, T1, "TSE-CAL-2013"),
        // 2013-05-03 - T1 - TSE-CAL-2013 - Constitution Memorial Day.
        (2013, 5, 3, Closed, T1, "TSE-CAL-2013"),
        // 2013-05-06 - T1 - TSE-CAL-2013 - Holiday: printed observance day.
        (2013, 5, 6, Closed, T1, "TSE-CAL-2013"),
        // 2013-07-15 - T1 - TSE-CAL-2013 - Marine Day.
        (2013, 7, 15, Closed, T1, "TSE-CAL-2013"),
        // 2013-09-16 - T1 - TSE-CAL-2013 - Respect for the Aged Day.
        (2013, 9, 16, Closed, T1, "TSE-CAL-2013"),
        // 2013-09-23 - T1 - TSE-CAL-2013 - Autumnal equinox.
        (2013, 9, 23, Closed, T1, "TSE-CAL-2013"),
        // 2013-10-14 - T1 - TSE-CAL-2013 - Health and Sports Day.
        (2013, 10, 14, Closed, T1, "TSE-CAL-2013"),
        // 2013-11-04 - T1 - TSE-CAL-2013 - Holiday: printed observance day.
        (2013, 11, 4, Closed, T1, "TSE-CAL-2013"),
        // 2013-12-23 - T1 - TSE-CAL-2013 - Emperor's Birthday.
        (2013, 12, 23, Closed, T1, "TSE-CAL-2013"),
        // 2013-12-31 - T1 - TSE-CAL-2013 - Exchange Holiday.
        (2013, 12, 31, Closed, T1, "TSE-CAL-2013"),
        // 2014-01-01 - T1 - TSE-CAL-2014 - New Year's Day.
        (2014, 1, 1, Closed, T1, "TSE-CAL-2014"),
        // 2014-01-02 - T1 - TSE-CAL-2014 - Exchange Holiday.
        (2014, 1, 2, Closed, T1, "TSE-CAL-2014"),
        // 2014-01-03 - T1 - TSE-CAL-2014 - Exchange Holiday.
        (2014, 1, 3, Closed, T1, "TSE-CAL-2014"),
        // 2014-01-13 - T1 - TSE-CAL-2014 - Coming of Age Day.
        (2014, 1, 13, Closed, T1, "TSE-CAL-2014"),
        // 2014-02-11 - T1 - TSE-CAL-2014 - National Foundation Day.
        (2014, 2, 11, Closed, T1, "TSE-CAL-2014"),
        // 2014-03-21 - T1 - TSE-CAL-2014 - Vernal Equinox.
        (2014, 3, 21, Closed, T1, "TSE-CAL-2014"),
        // 2014-04-29 - T1 - TSE-CAL-2014 - Showa Day.
        (2014, 4, 29, Closed, T1, "TSE-CAL-2014"),
        // 2014-05-05 - T1 - TSE-CAL-2014 - Children's Day.
        (2014, 5, 5, Closed, T1, "TSE-CAL-2014"),
        // 2014-05-06 - T1 - TSE-CAL-2014 - Holiday: printed observance day.
        (2014, 5, 6, Closed, T1, "TSE-CAL-2014"),
        // 2014-07-21 - T1 - TSE-CAL-2014 - Marine Day.
        (2014, 7, 21, Closed, T1, "TSE-CAL-2014"),
        // 2014-09-15 - T1 - TSE-CAL-2014 - Respect for the Aged Day.
        (2014, 9, 15, Closed, T1, "TSE-CAL-2014"),
        // 2014-09-23 - T1 - TSE-CAL-2014 - Autumnal equinox.
        (2014, 9, 23, Closed, T1, "TSE-CAL-2014"),
        // 2014-10-13 - T1 - TSE-CAL-2014 - Health and Sports Day.
        (2014, 10, 13, Closed, T1, "TSE-CAL-2014"),
        // 2014-11-03 - T1 - TSE-CAL-2014 - Culture Day.
        (2014, 11, 3, Closed, T1, "TSE-CAL-2014"),
        // 2014-11-24 - T1 - TSE-CAL-2014 - Holiday: printed observance day.
        (2014, 11, 24, Closed, T1, "TSE-CAL-2014"),
        // 2014-12-23 - T1 - TSE-CAL-2014 - Emperor's Birthday.
        (2014, 12, 23, Closed, T1, "TSE-CAL-2014"),
        // 2014-12-31 - T1 - TSE-CAL-2014 - Exchange Holiday.
        (2014, 12, 31, Closed, T1, "TSE-CAL-2014"),
        // 2015-01-01 - T1 - JPX-HOL-2015 - New Year's Day.
        (2015, 1, 1, Closed, T1, "JPX-HOL-2015"),
        // 2015-01-02 - T1 - JPX-HOL-2015 - Exchange Holiday.
        (2015, 1, 2, Closed, T1, "JPX-HOL-2015"),
        // 2015-01-12 - T1 - JPX-HOL-2015 - Coming of Age Day.
        (2015, 1, 12, Closed, T1, "JPX-HOL-2015"),
        // 2015-02-11 - T1 - JPX-HOL-2015 - National Foundation Day.
        (2015, 2, 11, Closed, T1, "JPX-HOL-2015"),
        // 2015-04-29 - T1 - JPX-HOL-2015 - Showa Day.
        (2015, 4, 29, Closed, T1, "JPX-HOL-2015"),
        // 2015-05-04 - T1 - JPX-HOL-2015 - Greenery Day.
        (2015, 5, 4, Closed, T1, "JPX-HOL-2015"),
        // 2015-05-05 - T1 - JPX-HOL-2015 - Children's Day.
        (2015, 5, 5, Closed, T1, "JPX-HOL-2015"),
        // 2015-05-06 - T1 - JPX-HOL-2015 - Holiday: printed observance day.
        (2015, 5, 6, Closed, T1, "JPX-HOL-2015"),
        // 2015-07-20 - T1 - JPX-HOL-2015 - Marine Day.
        (2015, 7, 20, Closed, T1, "JPX-HOL-2015"),
        // 2015-09-21 - T1 - JPX-HOL-2015 - Respect for the Aged Day.
        (2015, 9, 21, Closed, T1, "JPX-HOL-2015"),
        // 2015-09-22 - T1 - JPX-HOL-2015 - Holiday: the citizen's holiday between Respect for the Aged Day (Sep. 21) and the Autumnal Equinox (Sep. 23).
        (2015, 9, 22, Closed, T1, "JPX-HOL-2015"),
        // 2015-09-23 - T1 - JPX-HOL-2015 - Autumnal equinox.
        (2015, 9, 23, Closed, T1, "JPX-HOL-2015"),
        // 2015-10-12 - T1 - JPX-HOL-2015 - Health and Sports Day.
        (2015, 10, 12, Closed, T1, "JPX-HOL-2015"),
        // 2015-11-03 - T1 - JPX-HOL-2015 - Culture Day.
        (2015, 11, 3, Closed, T1, "JPX-HOL-2015"),
        // 2015-11-23 - T1 - JPX-HOL-2015 - Labor Thanksgiving Day.
        (2015, 11, 23, Closed, T1, "JPX-HOL-2015"),
        // 2015-12-23 - T1 - JPX-HOL-2015 - Emperor's Birthday.
        (2015, 12, 23, Closed, T1, "JPX-HOL-2015"),
        // 2015-12-31 - T1 - JPX-HOL-2015 - Exchange Holiday.
        (2015, 12, 31, Closed, T1, "JPX-HOL-2015"),
        // 2016-01-01 - T1 - JPX-HOL-2016 - New Year's Day.
        (2016, 1, 1, Closed, T1, "JPX-HOL-2016"),
        // 2016-01-11 - T1 - JPX-HOL-2016 - Coming of Age Day.
        (2016, 1, 11, Closed, T1, "JPX-HOL-2016"),
        // 2016-02-11 - T1 - JPX-HOL-2016 - National Foundation Day.
        (2016, 2, 11, Closed, T1, "JPX-HOL-2016"),
        // 2016-03-21 - T1 - JPX-HOL-2016 - Holiday: printed observance day.
        (2016, 3, 21, Closed, T1, "JPX-HOL-2016"),
        // 2016-04-29 - T1 - JPX-HOL-2016 - Showa Day.
        (2016, 4, 29, Closed, T1, "JPX-HOL-2016"),
        // 2016-05-03 - T1 - JPX-HOL-2016 - Constitution Memorial Day.
        (2016, 5, 3, Closed, T1, "JPX-HOL-2016"),
        // 2016-05-04 - T1 - JPX-HOL-2016 - Greenery Day.
        (2016, 5, 4, Closed, T1, "JPX-HOL-2016"),
        // 2016-05-05 - T1 - JPX-HOL-2016 - Children's Day.
        (2016, 5, 5, Closed, T1, "JPX-HOL-2016"),
        // 2016-07-18 - T1 - JPX-HOL-2016 - Marine Day.
        (2016, 7, 18, Closed, T1, "JPX-HOL-2016"),
        // 2016-08-11 - T1 - JPX-HOL-2016 - Mountain Day.
        (2016, 8, 11, Closed, T1, "JPX-HOL-2016"),
        // 2016-09-19 - T1 - JPX-HOL-2016 - Respect for the Aged Day.
        (2016, 9, 19, Closed, T1, "JPX-HOL-2016"),
        // 2016-09-22 - T1 - JPX-HOL-2016 - Autumnal equinox.
        (2016, 9, 22, Closed, T1, "JPX-HOL-2016"),
        // 2016-10-10 - T1 - JPX-HOL-2016 - Health and Sports Day.
        (2016, 10, 10, Closed, T1, "JPX-HOL-2016"),
        // 2016-11-03 - T1 - JPX-HOL-2016 - Culture Day.
        (2016, 11, 3, Closed, T1, "JPX-HOL-2016"),
        // 2016-11-23 - T1 - JPX-HOL-2016 - Labor Thanksgiving Day.
        (2016, 11, 23, Closed, T1, "JPX-HOL-2016"),
        // 2016-12-23 - T1 - JPX-HOL-2016 - Emperor's Birthday.
        (2016, 12, 23, Closed, T1, "JPX-HOL-2016"),
        // 2017-01-02 - T1 - JPX-HOL-2017 - Holiday: printed observance day.
        (2017, 1, 2, Closed, T1, "JPX-HOL-2017"),
        // 2017-01-03 - T1 - JPX-HOL-2017 - Exchange Holiday.
        (2017, 1, 3, Closed, T1, "JPX-HOL-2017"),
        // 2017-01-09 - T1 - JPX-HOL-2017 - Coming of Age Day.
        (2017, 1, 9, Closed, T1, "JPX-HOL-2017"),
        // 2017-03-20 - T1 - JPX-HOL-2017 - Vernal Equinox.
        (2017, 3, 20, Closed, T1, "JPX-HOL-2017"),
        // 2017-05-03 - T1 - JPX-HOL-2017 - Constitution Memorial Day.
        (2017, 5, 3, Closed, T1, "JPX-HOL-2017"),
        // 2017-05-04 - T1 - JPX-HOL-2017 - Greenery Day.
        (2017, 5, 4, Closed, T1, "JPX-HOL-2017"),
        // 2017-05-05 - T1 - JPX-HOL-2017 - Children's Day.
        (2017, 5, 5, Closed, T1, "JPX-HOL-2017"),
        // 2017-07-17 - T1 - JPX-HOL-2017 - Marine Day.
        (2017, 7, 17, Closed, T1, "JPX-HOL-2017"),
        // 2017-08-11 - T1 - JPX-HOL-2017 - Mountain Day.
        (2017, 8, 11, Closed, T1, "JPX-HOL-2017"),
        // 2017-09-18 - T1 - JPX-HOL-2017 - Respect for the Aged Day.
        (2017, 9, 18, Closed, T1, "JPX-HOL-2017"),
        // 2017-10-09 - T1 - JPX-HOL-2017 - Health and Sports Day.
        (2017, 10, 9, Closed, T1, "JPX-HOL-2017"),
        // 2017-11-03 - T1 - JPX-HOL-2017 - Culture Day.
        (2017, 11, 3, Closed, T1, "JPX-HOL-2017"),
        // 2017-11-23 - T1 - JPX-HOL-2017 - Labor Thanksgiving Day.
        (2017, 11, 23, Closed, T1, "JPX-HOL-2017"),
        // 2018-01-01 - T1 - JPX-HOL-2018 - New Year's Day.
        (2018, 1, 1, Closed, T1, "JPX-HOL-2018"),
        // 2018-01-02 - T1 - JPX-HOL-2018 - Market Holiday.
        (2018, 1, 2, Closed, T1, "JPX-HOL-2018"),
        // 2018-01-03 - T1 - JPX-HOL-2018 - Market Holiday.
        (2018, 1, 3, Closed, T1, "JPX-HOL-2018"),
        // 2018-01-08 - T1 - JPX-HOL-2018 - Coming of Age Day.
        (2018, 1, 8, Closed, T1, "JPX-HOL-2018"),
        // 2018-02-12 - T1 - JPX-HOL-2018 - National Foundation Day (Feb. 11) observed: Feb. 11 is a Sunday, so the observance is the row and Feb. 11 ships none.
        (2018, 2, 12, Closed, T1, "JPX-HOL-2018"),
        // 2018-03-21 - T1 - JPX-HOL-2018 - Vernal Equinox.
        (2018, 3, 21, Closed, T1, "JPX-HOL-2018"),
        // 2018-04-30 - T1 - JPX-HOL-2018 - Showa Day (Apr. 29) observed: Apr. 29 is a Sunday, so the observance is the row and Apr. 29 ships none.
        (2018, 4, 30, Closed, T1, "JPX-HOL-2018"),
        // 2018-05-03 - T1 - JPX-HOL-2018 - Constitution Memorial Day.
        (2018, 5, 3, Closed, T1, "JPX-HOL-2018"),
        // 2018-05-04 - T1 - JPX-HOL-2018 - Greenery Day.
        (2018, 5, 4, Closed, T1, "JPX-HOL-2018"),
        // 2018-07-16 - T1 - JPX-HOL-2018 - Marine Day.
        (2018, 7, 16, Closed, T1, "JPX-HOL-2018"),
        // 2018-09-17 - T1 - JPX-HOL-2018 - Respect for the Aged Day.
        (2018, 9, 17, Closed, T1, "JPX-HOL-2018"),
        // 2018-09-24 - T1 - JPX-HOL-2018 - Autumnal Equinox (Sep. 23) observed: Sep. 23 is a Sunday, so the observance is the row and Sep. 23 ships none.
        (2018, 9, 24, Closed, T1, "JPX-HOL-2018"),
        // 2018-10-08 - T1 - JPX-HOL-2018 - Health and Sports Day.
        (2018, 10, 8, Closed, T1, "JPX-HOL-2018"),
        // 2018-11-23 - T1 - JPX-HOL-2018 - Labor Thanksgiving Day.
        (2018, 11, 23, Closed, T1, "JPX-HOL-2018"),
        // 2018-12-24 - T1 - JPX-HOL-2018 - Emperor's Birthday (Dec. 23) observed: Dec. 23 is a Sunday, so the observance is the row and Dec. 23 ships none.
        (2018, 12, 24, Closed, T1, "JPX-HOL-2018"),
        // 2018-12-31 - T1 - JPX-HOL-2018 - Market Holiday.
        (2018, 12, 31, Closed, T1, "JPX-HOL-2018"),
        // 2019-01-01 - T1 - JPX-HOL-2019 - New Year's Day.
        (2019, 1, 1, Closed, T1, "JPX-HOL-2019"),
        // 2019-01-02 - T1 - JPX-HOL-2019 - Market Holiday.
        (2019, 1, 2, Closed, T1, "JPX-HOL-2019"),
        // 2019-01-03 - T1 - JPX-HOL-2019 - Market Holiday.
        (2019, 1, 3, Closed, T1, "JPX-HOL-2019"),
        // 2019-01-14 - T1 - JPX-HOL-2019 - Coming of Age Day.
        (2019, 1, 14, Closed, T1, "JPX-HOL-2019"),
        // 2019-02-11 - T1 - JPX-HOL-2019 - National Foundation Day.
        (2019, 2, 11, Closed, T1, "JPX-HOL-2019"),
        // 2019-03-21 - T1 - JPX-HOL-2019 - Vernal Equinox.
        (2019, 3, 21, Closed, T1, "JPX-HOL-2019"),
        // 2019-04-29 - T1 - JPX-HOL-2019 - Showa Day.
        (2019, 4, 29, Closed, T1, "JPX-HOL-2019"),
        // 2019-04-30 - T1 - JPX-HOL-2019 - Abdication Day.
        (2019, 4, 30, Closed, T1, "JPX-HOL-2019"),
        // 2019-05-01 - T1 - JPX-HOL-2019 - Accession Day.
        (2019, 5, 1, Closed, T1, "JPX-HOL-2019"),
        // 2019-05-02 - T1 - JPX-HOL-2019 - National Holiday.
        (2019, 5, 2, Closed, T1, "JPX-HOL-2019"),
        // 2019-05-03 - T1 - JPX-HOL-2019 - Constitution Memorial Day.
        (2019, 5, 3, Closed, T1, "JPX-HOL-2019"),
        // 2019-05-06 - T1 - JPX-HOL-2019 - Children's Day (May 5) observed: May 5 is a Sunday, so the observance is the row and May 5 ships none.
        (2019, 5, 6, Closed, T1, "JPX-HOL-2019"),
        // 2019-07-15 - T1 - JPX-HOL-2019 - Marine Day.
        (2019, 7, 15, Closed, T1, "JPX-HOL-2019"),
        // 2019-08-12 - T1 - JPX-HOL-2019 - Mountain Day (Aug. 11) observed: Aug. 11 is a Sunday, so the observance is the row and Aug. 11 ships none.
        (2019, 8, 12, Closed, T1, "JPX-HOL-2019"),
        // 2019-09-16 - T1 - JPX-HOL-2019 - Respect for the Aged Day.
        (2019, 9, 16, Closed, T1, "JPX-HOL-2019"),
        // 2019-09-23 - T1 - JPX-HOL-2019 - Autumnal Equinox.
        (2019, 9, 23, Closed, T1, "JPX-HOL-2019"),
        // 2019-10-14 - T1 - JPX-HOL-2019 - Health and Sports Day.
        (2019, 10, 14, Closed, T1, "JPX-HOL-2019"),
        // 2019-10-22 - T1 - JPX-HOL-2019 - Enthronement Ceremony Day.
        (2019, 10, 22, Closed, T1, "JPX-HOL-2019"),
        // 2019-11-04 - T1 - JPX-HOL-2019 - Culture Day (Nov. 3) observed: Nov. 3 is a Sunday, so the observance is the row and Nov. 3 ships none.
        (2019, 11, 4, Closed, T1, "JPX-HOL-2019"),
        // 2019-12-31 - T1 - JPX-HOL-2019 - Market Holiday.
        (2019, 12, 31, Closed, T1, "JPX-HOL-2019"),
        // 2020-01-01 - T1 - JPX-HOL-2020 - New Year's Day.
        (2020, 1, 1, Closed, T1, "JPX-HOL-2020"),
        // 2020-01-02 - T1 - JPX-HOL-2020 - Market Holiday.
        (2020, 1, 2, Closed, T1, "JPX-HOL-2020"),
        // 2020-01-03 - T1 - JPX-HOL-2020 - Market Holiday.
        (2020, 1, 3, Closed, T1, "JPX-HOL-2020"),
        // 2020-01-13 - T1 - JPX-HOL-2020 - Coming of Age Day.
        (2020, 1, 13, Closed, T1, "JPX-HOL-2020"),
        // 2020-02-11 - T1 - JPX-HOL-2020 - National Foundation Day.
        (2020, 2, 11, Closed, T1, "JPX-HOL-2020"),
        // 2020-02-24 - T1 - JPX-HOL-2020 - Emperor's Birthday (Feb. 23) observed: Feb. 23 is a Sunday, so the observance is the row and Feb. 23 ships none.
        (2020, 2, 24, Closed, T1, "JPX-HOL-2020"),
        // 2020-03-20 - T1 - JPX-HOL-2020 - Vernal Equinox.
        (2020, 3, 20, Closed, T1, "JPX-HOL-2020"),
        // 2020-04-29 - T1 - JPX-HOL-2020 - Showa Day.
        (2020, 4, 29, Closed, T1, "JPX-HOL-2020"),
        // 2020-05-04 - T1 - JPX-HOL-2020 - Greenery Day.
        (2020, 5, 4, Closed, T1, "JPX-HOL-2020"),
        // 2020-05-05 - T1 - JPX-HOL-2020 - Children's Day.
        (2020, 5, 5, Closed, T1, "JPX-HOL-2020"),
        // 2020-05-06 - T1 - JPX-HOL-2020 - Constitution Memorial Day (May 3) observed: May 3 is a Sunday, so the observance is the row and May 3 ships none.
        (2020, 5, 6, Closed, T1, "JPX-HOL-2020"),
        // 2020-07-23 - T1 - JPX-HOL-2020 - Marine Day.
        (2020, 7, 23, Closed, T1, "JPX-HOL-2020"),
        // 2020-07-24 - T1 - JPX-HOL-2020 - Sports Day.
        (2020, 7, 24, Closed, T1, "JPX-HOL-2020"),
        // 2020-08-10 - T1 - JPX-HOL-2020 - Mountain Day.
        (2020, 8, 10, Closed, T1, "JPX-HOL-2020"),
        // 2020-09-21 - T1 - JPX-HOL-2020 - Respect for the Aged Day.
        (2020, 9, 21, Closed, T1, "JPX-HOL-2020"),
        // 2020-09-22 - T1 - JPX-HOL-2020 - Autumnal Equinox.
        (2020, 9, 22, Closed, T1, "JPX-HOL-2020"),
        // 2020-11-03 - T1 - JPX-HOL-2020 - Culture Day.
        (2020, 11, 3, Closed, T1, "JPX-HOL-2020"),
        // 2020-11-23 - T1 - JPX-HOL-2020 - Labor Thanksgiving Day.
        (2020, 11, 23, Closed, T1, "JPX-HOL-2020"),
        // 2020-12-31 - T1 - JPX-HOL-2020 - Market Holiday.
        (2020, 12, 31, Closed, T1, "JPX-HOL-2020"),
        // 2021-01-01 - T1 - JPX-HOL-2021 - New Year's Day.
        (2021, 1, 1, Closed, T1, "JPX-HOL-2021"),
        // 2021-01-11 - T1 - JPX-HOL-2021 - Coming of Age Day.
        (2021, 1, 11, Closed, T1, "JPX-HOL-2021"),
        // 2021-02-11 - T1 - JPX-HOL-2021 - National Foundation Day.
        (2021, 2, 11, Closed, T1, "JPX-HOL-2021"),
        // 2021-02-23 - T1 - JPX-HOL-2021 - Emperor's Birthday.
        (2021, 2, 23, Closed, T1, "JPX-HOL-2021"),
        // 2021-04-29 - T1 - JPX-HOL-2021 - Showa Day.
        (2021, 4, 29, Closed, T1, "JPX-HOL-2021"),
        // 2021-05-03 - T1 - JPX-HOL-2021 - Constitution Memorial Day.
        (2021, 5, 3, Closed, T1, "JPX-HOL-2021"),
        // 2021-05-04 - T1 - JPX-HOL-2021 - Greenery Day.
        (2021, 5, 4, Closed, T1, "JPX-HOL-2021"),
        // 2021-05-05 - T1 - JPX-HOL-2021 - Children's Day.
        (2021, 5, 5, Closed, T1, "JPX-HOL-2021"),
        // 2021-07-22 - T1 - JPX-HOL-2021 - Marine Day.
        (2021, 7, 22, Closed, T1, "JPX-HOL-2021"),
        // 2021-07-23 - T1 - JPX-HOL-2021 - Sports Day.
        (2021, 7, 23, Closed, T1, "JPX-HOL-2021"),
        // 2021-08-09 - T1 - JPX-HOL-2021 - Mountain Day (Aug. 8) observed: Aug. 8 is a Sunday, so the observance is the row and Aug. 8 ships none.
        (2021, 8, 9, Closed, T1, "JPX-HOL-2021"),
        // 2021-09-20 - T1 - JPX-HOL-2021 - Respect for the Aged Day.
        (2021, 9, 20, Closed, T1, "JPX-HOL-2021"),
        // 2021-09-23 - T1 - JPX-HOL-2021 - Autumnal Equinox.
        (2021, 9, 23, Closed, T1, "JPX-HOL-2021"),
        // 2021-11-03 - T1 - JPX-HOL-2021 - Culture Day.
        (2021, 11, 3, Closed, T1, "JPX-HOL-2021"),
        // 2021-11-23 - T1 - JPX-HOL-2021 - Labor Thanksgiving Day.
        (2021, 11, 23, Closed, T1, "JPX-HOL-2021"),
        // 2021-12-31 - T1 - JPX-HOL-2021 - Market Holiday.
        (2021, 12, 31, Closed, T1, "JPX-HOL-2021"),
        // 2022-01-03 - T1 - JPX-HOL-2022 - Market Holiday.
        (2022, 1, 3, Closed, T1, "JPX-HOL-2022"),
        // 2022-01-10 - T1 - JPX-HOL-2022 - Coming of Age Day.
        (2022, 1, 10, Closed, T1, "JPX-HOL-2022"),
        // 2022-02-11 - T1 - JPX-HOL-2022 - National Foundation Day.
        (2022, 2, 11, Closed, T1, "JPX-HOL-2022"),
        // 2022-02-23 - T1 - JPX-HOL-2022 - Emperor's Birthday.
        (2022, 2, 23, Closed, T1, "JPX-HOL-2022"),
        // 2022-03-21 - T1 - JPX-HOL-2022 - Vernal Equinox.
        (2022, 3, 21, Closed, T1, "JPX-HOL-2022"),
        // 2022-04-29 - T1 - JPX-HOL-2022 - Showa Day.
        (2022, 4, 29, Closed, T1, "JPX-HOL-2022"),
        // 2022-05-03 - T1 - JPX-HOL-2022 - Constitution Memorial Day.
        (2022, 5, 3, Closed, T1, "JPX-HOL-2022"),
        // 2022-05-04 - T1 - JPX-HOL-2022 - Greenery Day.
        (2022, 5, 4, Closed, T1, "JPX-HOL-2022"),
        // 2022-05-05 - T1 - JPX-HOL-2022 - Children's Day.
        (2022, 5, 5, Closed, T1, "JPX-HOL-2022"),
        // 2022-07-18 - T1 - JPX-HOL-2022 - Marine Day.
        (2022, 7, 18, Closed, T1, "JPX-HOL-2022"),
        // 2022-08-11 - T1 - JPX-HOL-2022 - Mountain Day.
        (2022, 8, 11, Closed, T1, "JPX-HOL-2022"),
        // 2022-09-19 - T1 - JPX-HOL-2022 - Respect for the Aged Day.
        (2022, 9, 19, Closed, T1, "JPX-HOL-2022"),
        // 2022-09-23 - T1 - JPX-HOL-2022 - Autumnal Equinox.
        (2022, 9, 23, Closed, T1, "JPX-HOL-2022"),
        // 2022-10-10 - T1 - JPX-HOL-2022 - Sports Day.
        (2022, 10, 10, Closed, T1, "JPX-HOL-2022"),
        // 2022-11-03 - T1 - JPX-HOL-2022 - Culture Day.
        (2022, 11, 3, Closed, T1, "JPX-HOL-2022"),
        // 2022-11-23 - T1 - JPX-HOL-2022 - Labor Thanksgiving Day.
        (2022, 11, 23, Closed, T1, "JPX-HOL-2022"),
        // 2023-01-02 - T1 - JPX-HOL-2023 - New Year's Day (Jan. 1) observed: Jan. 1 is a Sunday, so the observance is the row and Jan. 1 ships none.
        (2023, 1, 2, Closed, T1, "JPX-HOL-2023"),
        // 2023-01-03 - T1 - JPX-HOL-2023 - Market Holiday.
        (2023, 1, 3, Closed, T1, "JPX-HOL-2023"),
        // 2023-01-09 - T1 - JPX-HOL-2023 - Coming of Age Day.
        (2023, 1, 9, Closed, T1, "JPX-HOL-2023"),
        // 2023-02-23 - T1 - JPX-HOL-2023 - Emperor's Birthday.
        (2023, 2, 23, Closed, T1, "JPX-HOL-2023"),
        // 2023-03-21 - T1 - JPX-HOL-2023 - Vernal Equinox.
        (2023, 3, 21, Closed, T1, "JPX-HOL-2023"),
        // 2023-05-03 - T1 - JPX-HOL-2023 - Constitution Memorial Day.
        (2023, 5, 3, Closed, T1, "JPX-HOL-2023"),
        // 2023-05-04 - T1 - JPX-HOL-2023 - Greenery Day.
        (2023, 5, 4, Closed, T1, "JPX-HOL-2023"),
        // 2023-05-05 - T1 - JPX-HOL-2023 - Children's Day.
        (2023, 5, 5, Closed, T1, "JPX-HOL-2023"),
        // 2023-07-17 - T1 - JPX-HOL-2023 - Marine Day.
        (2023, 7, 17, Closed, T1, "JPX-HOL-2023"),
        // 2023-08-11 - T1 - JPX-HOL-2023 - Mountain Day.
        (2023, 8, 11, Closed, T1, "JPX-HOL-2023"),
        // 2023-09-18 - T1 - JPX-HOL-2023 - Respect for the Aged Day.
        (2023, 9, 18, Closed, T1, "JPX-HOL-2023"),
        // 2023-10-09 - T1 - JPX-HOL-2023 - Sports Day.
        (2023, 10, 9, Closed, T1, "JPX-HOL-2023"),
        // 2023-11-03 - T1 - JPX-HOL-2023 - Culture Day.
        (2023, 11, 3, Closed, T1, "JPX-HOL-2023"),
        // 2023-11-23 - T1 - JPX-HOL-2023 - Labor Thanksgiving Day.
        (2023, 11, 23, Closed, T1, "JPX-HOL-2023"),
        // 2024-01-01 - T1 - JPX-HOL-2024 - New Year's Day.
        (2024, 1, 1, Closed, T1, "JPX-HOL-2024"),
        // 2024-01-02 - T1 - JPX-HOL-2024 - Market Holiday.
        (2024, 1, 2, Closed, T1, "JPX-HOL-2024"),
        // 2024-01-03 - T1 - JPX-HOL-2024 - Market Holiday.
        (2024, 1, 3, Closed, T1, "JPX-HOL-2024"),
        // 2024-01-08 - T1 - JPX-HOL-2024 - Coming of Age Day.
        (2024, 1, 8, Closed, T1, "JPX-HOL-2024"),
        // 2024-02-12 - T1 - JPX-HOL-2024 - National Foundation Day (Feb. 11) observed: Feb. 11 is a Sunday, so the observance is the row and Feb. 11 ships none.
        (2024, 2, 12, Closed, T1, "JPX-HOL-2024"),
        // 2024-02-23 - T1 - JPX-HOL-2024 - Emperor's Birthday.
        (2024, 2, 23, Closed, T1, "JPX-HOL-2024"),
        // 2024-03-20 - T1 - JPX-HOL-2024 - Vernal Equinox.
        (2024, 3, 20, Closed, T1, "JPX-HOL-2024"),
        // 2024-04-29 - T1 - JPX-HOL-2024 - Showa Day.
        (2024, 4, 29, Closed, T1, "JPX-HOL-2024"),
        // 2024-05-03 - T1 - JPX-HOL-2024 - Constitution Memorial Day.
        (2024, 5, 3, Closed, T1, "JPX-HOL-2024"),
        // 2024-05-06 - T1 - JPX-HOL-2024 - Children's Day (May 5) observed: May 5 is a Sunday, so the observance is the row and May 5 ships none.
        (2024, 5, 6, Closed, T1, "JPX-HOL-2024"),
        // 2024-07-15 - T1 - JPX-HOL-2024 - Marine Day.
        (2024, 7, 15, Closed, T1, "JPX-HOL-2024"),
        // 2024-08-12 - T1 - JPX-HOL-2024 - Mountain Day (Aug. 11) observed: Aug. 11 is a Sunday, so the observance is the row and Aug. 11 ships none.
        (2024, 8, 12, Closed, T1, "JPX-HOL-2024"),
        // 2024-09-16 - T1 - JPX-HOL-2024 - Respect for the Aged Day.
        (2024, 9, 16, Closed, T1, "JPX-HOL-2024"),
        // 2024-09-23 - T1 - JPX-HOL-2024 - Autumnal Equinox (Sep. 22) observed: Sep. 22 is a Sunday, so the observance is the row and Sep. 22 ships none.
        (2024, 9, 23, Closed, T1, "JPX-HOL-2024"),
        // 2024-10-14 - T1 - JPX-HOL-2024 - Sports Day.
        (2024, 10, 14, Closed, T1, "JPX-HOL-2024"),
        // 2024-11-04 - T1 - JPX-HOL-2024 - Culture Day (Nov. 3) observed: Nov. 3 is a Sunday, so the observance is the row and Nov. 3 ships none.
        (2024, 11, 4, Closed, T1, "JPX-HOL-2024"),
        // 2024-12-31 - T1 - JPX-HOL-2024 - Market Holiday.
        (2024, 12, 31, Closed, T1, "JPX-HOL-2024"),
        // 2025-01-01 - T1 - JPX-HOL-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "JPX-HOL-2025"),
        // 2025-01-02 - T1 - JPX-HOL-2025 - Market Holiday.
        (2025, 1, 2, Closed, T1, "JPX-HOL-2025"),
        // 2025-01-03 - T1 - JPX-HOL-2025 - Market Holiday.
        (2025, 1, 3, Closed, T1, "JPX-HOL-2025"),
        // 2025-01-13 - T1 - JPX-HOL-2025 - Coming of Age Day.
        (2025, 1, 13, Closed, T1, "JPX-HOL-2025"),
        // 2025-02-11 - T1 - JPX-HOL-2025 - National Foundation Day.
        (2025, 2, 11, Closed, T1, "JPX-HOL-2025"),
        // 2025-02-24 - T1 - JPX-HOL-2025 - Emperor's Birthday (Feb. 23) observed:
        // Feb. 23 is a Sunday, so the observance is the row and Feb. 23 ships none.
        (2025, 2, 24, Closed, T1, "JPX-HOL-2025"),
        // 2025-03-20 - T1 - JPX-HOL-2025 - Vernal Equinox.
        (2025, 3, 20, Closed, T1, "JPX-HOL-2025"),
        // 2025-04-29 - T1 - JPX-HOL-2025 - Showa Day.
        (2025, 4, 29, Closed, T1, "JPX-HOL-2025"),
        // 2025-05-05 - T1 - JPX-HOL-2025 - Children's Day. May 3 (Constitution
        // Memorial Day, Saturday) and May 4 (Greenery Day, Sunday) ship no rows.
        (2025, 5, 5, Closed, T1, "JPX-HOL-2025"),
        // 2025-05-06 - T1 - JPX-HOL-2025 - Greenery Day (May 4) observed.
        (2025, 5, 6, Closed, T1, "JPX-HOL-2025"),
        // 2025-07-21 - T1 - JPX-HOL-2025 - Marine Day.
        (2025, 7, 21, Closed, T1, "JPX-HOL-2025"),
        // 2025-08-11 - T1 - JPX-HOL-2025 - Mountain Day.
        (2025, 8, 11, Closed, T1, "JPX-HOL-2025"),
        // 2025-09-15 - T1 - JPX-HOL-2025 - Respect for the Aged Day.
        (2025, 9, 15, Closed, T1, "JPX-HOL-2025"),
        // 2025-09-23 - T1 - JPX-HOL-2025 - Autumnal Equinox.
        (2025, 9, 23, Closed, T1, "JPX-HOL-2025"),
        // 2025-10-13 - T1 - JPX-HOL-2025 - Sports Day.
        (2025, 10, 13, Closed, T1, "JPX-HOL-2025"),
        // 2025-11-03 - T1 - JPX-HOL-2025 - Culture Day.
        (2025, 11, 3, Closed, T1, "JPX-HOL-2025"),
        // 2025-11-24 - T1 - JPX-HOL-2025 - Labor Thanksgiving Day (Nov. 23) observed:
        // Nov. 23 is a Sunday, so the observance is the row and Nov. 23 ships none.
        (2025, 11, 24, Closed, T1, "JPX-HOL-2025"),
        // 2025-12-31 - T1 - JPX-HOL-2025 - Market Holiday.
        (2025, 12, 31, Closed, T1, "JPX-HOL-2025"),
        // 2026-01-01 - T1 - JPX-HOL-2026-2027 - New Year's Day.
        (2026, 1, 1, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-01-02 - T1 - JPX-HOL-2026-2027 - Market Holiday. Jan. 3 is a
        // Saturday Market Holiday and ships no row.
        (2026, 1, 2, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-01-12 - T1 - JPX-HOL-2026-2027 - Coming of Age Day.
        (2026, 1, 12, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-02-11 - T1 - JPX-HOL-2026-2027 - National Foundation Day.
        (2026, 2, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-02-23 - T1 - JPX-HOL-2026-2027 - Emperor's Birthday.
        (2026, 2, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-03-20 - T1 - JPX-HOL-2026-2027 - Vernal Equinox.
        (2026, 3, 20, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-04-29 - T1 - JPX-HOL-2026-2027 - Showa Day.
        (2026, 4, 29, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-05-04 - T1 - JPX-HOL-2026-2027 - Greenery Day. May 3 is a Sunday
        // Constitution Memorial Day and ships no row.
        (2026, 5, 4, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-05-05 - T1 - JPX-HOL-2026-2027 - Children's Day.
        (2026, 5, 5, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-05-06 - T1 - JPX-HOL-2026-2027 - Constitution Memorial Day (May 3)
        // observed.
        (2026, 5, 6, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-07-20 - T1 - JPX-HOL-2026-2027 - Marine Day.
        (2026, 7, 20, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-08-11 - T1 - JPX-HOL-2026-2027 - Mountain Day.
        (2026, 8, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-09-21 - T1 - JPX-HOL-2026-2027 - Respect for the Aged Day.
        (2026, 9, 21, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-09-22 - T1 - JPX-HOL-2026-2027 - Holiday: the page states
        // "September 22, 2026, is a holiday in accordance with Rule 3,
        // Paragraph 3 of Act on National Holidays" (the citizen's holiday
        // between two national holidays).
        (2026, 9, 22, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-09-23 - T1 - JPX-HOL-2026-2027 - Autumnal Equinox.
        (2026, 9, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-10-12 - T1 - JPX-HOL-2026-2027 - Sports Day.
        (2026, 10, 12, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-11-03 - T1 - JPX-HOL-2026-2027 - Culture Day.
        (2026, 11, 3, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-11-23 - T1 - JPX-HOL-2026-2027 - Labor Thanksgiving Day.
        (2026, 11, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-12-31 - T1 - JPX-HOL-2026-2027 - Market Holiday.
        (2026, 12, 31, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-01-01 - T1 - JPX-HOL-2026-2027 - New Year's Day. Jan. 2 (Saturday)
        // and Jan. 3 (Sunday) are printed Market Holidays on weekend days and
        // ship no rows.
        (2027, 1, 1, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-01-11 - T1 - JPX-HOL-2026-2027 - Coming of Age Day.
        (2027, 1, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-02-11 - T1 - JPX-HOL-2026-2027 - National Foundation Day.
        (2027, 2, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-02-23 - T1 - JPX-HOL-2026-2027 - Emperor's Birthday.
        (2027, 2, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-03-22 - T1 - JPX-HOL-2026-2027 - Vernal Equinox (Mar. 21) observed:
        // Mar. 21 is a Sunday, so the observance is the row and Mar. 21 ships none.
        (2027, 3, 22, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-04-29 - T1 - JPX-HOL-2026-2027 - Showa Day.
        (2027, 4, 29, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-05-03 - T1 - JPX-HOL-2026-2027 - Constitution Memorial Day.
        (2027, 5, 3, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-05-04 - T1 - JPX-HOL-2026-2027 - Greenery Day.
        (2027, 5, 4, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-05-05 - T1 - JPX-HOL-2026-2027 - Children's Day.
        (2027, 5, 5, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-07-19 - T1 - JPX-HOL-2026-2027 - Marine Day.
        (2027, 7, 19, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-08-11 - T1 - JPX-HOL-2026-2027 - Mountain Day.
        (2027, 8, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-09-20 - T1 - JPX-HOL-2026-2027 - Respect for the Aged Day.
        (2027, 9, 20, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-09-23 - T1 - JPX-HOL-2026-2027 - Autumnal Equinox.
        (2027, 9, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-10-11 - T1 - JPX-HOL-2026-2027 - Sports Day.
        (2027, 10, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-11-03 - T1 - JPX-HOL-2026-2027 - Culture Day.
        (2027, 11, 3, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-11-23 - T1 - JPX-HOL-2026-2027 - Labor Thanksgiving Day.
        (2027, 11, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-12-31 - T1 - JPX-HOL-2026-2027 - Market Holiday.
        (2027, 12, 31, Closed, T1, "JPX-HOL-2026-2027"),
    ],
};
