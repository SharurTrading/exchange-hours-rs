// SPDX-License-Identifier: MIT-0

//! SIX Swiss Exchange holiday rows, 2010-2011, 2012-2017, 2018-2019,
//! 2020-2024 and 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Zurich`. For
//! 2012-2027 the rows key the operator's own `Trading Calendar` documents —
//! one per year, each a Trading Guide page whose twelve month grids mark every
//! non-trading day: light shades for Saturday and Sunday and a dark cell for
//! `Market Holiday — Market Closed`. The whole block is **T1**; each year's
//! rows cite that year's PDF (`SIX-TC-<year>`, retrieved through Wayback `id_`
//! replays for 2012-2017 and 2020-2024), except 2018-2019: their grids ride in
//! the operator's own Trading Guide of 28 May 2018, whose "Trading Calendar
//! 2018" and "Trading Calendar 2019" sections print both years' twelve-month
//! grids under one cover (`SIX-TG-2018`, the live education-path compilation).
//!
//! **The 2010-2011 rows key the operator's own market-holiday marks** on the
//! era's Trading-and-Settlement-Calendar pages (`SIX-TSC-2010` and
//! `SIX-TSC-2011`, archived 2010-01-31 and 2010-04-11, and 2010-11-15). Each
//! page's calendar grid shades a whole date row `#ccddff` for the legend entry
//! `SIX Swiss Exchange Market holiday`, a category distinct from the
//! per-currency `Currency holiday` flags, and on 24 and 31 December 2010 it
//! shades while no bank flag flies — the operator's own prose calls those
//! dates normal settlement days, so the layer states the market's holidays,
//! not bank closures. The same `#ccddff` fill in the operator's Currency
//! Holiday Calendar 2019 shades exactly the twelve 2019 closures the
//! `SIX-TG-2018` grids print, so the layer's semantics is calibrated against
//! a T1 document the crate already holds; #212 closed as data on 2026-10-05
//! UTC and the quotations and derivations live in
//! [`docs/evidence/six.md`](../../../../../docs/evidence/six.md).
//!
//! The calendars print closures only: no half day, no late open and no
//! intraday instant anywhere in the audited years, so `Closed` is the only
//! kind the operator's own statement supports and none other is invented.
//! Holidays that fall on a weekend are not marked (the Saturday/Sunday shading
//! already deletes them) and key no weekday row: 2010-01-02, 2010-05-01,
//! 2010-08-01, 2010-12-25, 2010-12-26, 2011-01-01, 2011-01-02, 2011-05-01,
//! 2011-12-24, 2011-12-25 and 2011-12-31, then 2015-08-01, 2016-12-25,
//! 2021-01-02, 2021-08-01, 2022-01-01 and 2022-12-24/25/31, 2023-12-24/31 and
//! the 2012-2017 weekend falls are those cases, while no holiday of 2018 or
//! 2019 falls on a weekend, so those two grids arise no such case. The
//! per-cell derivation is
//! recorded in [`docs/evidence/six.md`](../../../../../docs/evidence/six.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// SIX's built-in holiday rows and the windows they were audited over.
///
/// Every row is one dark `Market Holiday — Market Closed` cell of the year's
/// own `Trading Calendar` PDF. A date inside a window with no row is audited
/// normal; a date inside no window is an unaudited span and the identity-backed
/// queries refuse it.
// Evidence: docs/evidence/six.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2011, 12, 31), (2012, 1, 1) ..= (2017, 12, 31), (2018, 1, 1) ..= (2019, 12, 31), (2020, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // SIX-TSC-2010: the operator's own Trading-and-Settlement-Calendar page,
        // 2010 grid, `SIX Swiss Exchange Market holiday` (`#ccddff`) shading
        // 2010-01-01 - T1 - SIX-TSC-2010 - New Year's Day.
        (2010, 1, 1, Closed, T1, "SIX-TSC-2010"),
        // 2010-04-02 - T1 - SIX-TSC-2010 - Good Friday.
        (2010, 4, 2, Closed, T1, "SIX-TSC-2010"),
        // 2010-04-05 - T1 - SIX-TSC-2010 - Easter Monday.
        (2010, 4, 5, Closed, T1, "SIX-TSC-2010"),
        // 2010-05-13 - T1 - SIX-TSC-2010 - Ascension Day.
        (2010, 5, 13, Closed, T1, "SIX-TSC-2010"),
        // 2010-05-24 - T1 - SIX-TSC-2010 - Whit Monday.
        (2010, 5, 24, Closed, T1, "SIX-TSC-2010"),
        // 2010-12-24 - T1 - SIX-TSC-2010 - Christmas Eve.
        (2010, 12, 24, Closed, T1, "SIX-TSC-2010"),
        // 2010-12-31 - T1 - SIX-TSC-2010 - New Year's Eve.
        (2010, 12, 31, Closed, T1, "SIX-TSC-2010"),
        // SIX-TSC-2011: the same page's 2011 grid, published 15 November 2010,
        // same `SIX Swiss Exchange Market holiday` (`#ccddff`) shading
        // 2011-04-22 - T1 - SIX-TSC-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "SIX-TSC-2011"),
        // 2011-04-25 - T1 - SIX-TSC-2011 - Easter Monday.
        (2011, 4, 25, Closed, T1, "SIX-TSC-2011"),
        // 2011-06-02 - T1 - SIX-TSC-2011 - Ascension Day.
        (2011, 6, 2, Closed, T1, "SIX-TSC-2011"),
        // 2011-06-13 - T1 - SIX-TSC-2011 - Whit Monday.
        (2011, 6, 13, Closed, T1, "SIX-TSC-2011"),
        // 2011-08-01 - T1 - SIX-TSC-2011 - Swiss National Day.
        (2011, 8, 1, Closed, T1, "SIX-TSC-2011"),
        // 2011-12-26 - T1 - SIX-TSC-2011 - St. Stephen's Day.
        (2011, 12, 26, Closed, T1, "SIX-TSC-2011"),
        // SIX-TC-2012: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2012-01-02 - T1 - SIX-TC-2012 - St. Berchtold Day.
        (2012, 1, 2, Closed, T1, "SIX-TC-2012"),
        // 2012-04-06 - T1 - SIX-TC-2012 - Good Friday.
        (2012, 4, 6, Closed, T1, "SIX-TC-2012"),
        // 2012-04-09 - T1 - SIX-TC-2012 - Easter Monday.
        (2012, 4, 9, Closed, T1, "SIX-TC-2012"),
        // 2012-05-01 - T1 - SIX-TC-2012 - Labour Day.
        (2012, 5, 1, Closed, T1, "SIX-TC-2012"),
        // 2012-05-17 - T1 - SIX-TC-2012 - Ascension Day.
        (2012, 5, 17, Closed, T1, "SIX-TC-2012"),
        // 2012-05-28 - T1 - SIX-TC-2012 - Whit Monday.
        (2012, 5, 28, Closed, T1, "SIX-TC-2012"),
        // 2012-08-01 - T1 - SIX-TC-2012 - Swiss National Day.
        (2012, 8, 1, Closed, T1, "SIX-TC-2012"),
        // 2012-12-24 - T1 - SIX-TC-2012 - Christmas Eve.
        (2012, 12, 24, Closed, T1, "SIX-TC-2012"),
        // 2012-12-25 - T1 - SIX-TC-2012 - Christmas Day.
        (2012, 12, 25, Closed, T1, "SIX-TC-2012"),
        // 2012-12-26 - T1 - SIX-TC-2012 - St. Stephen's Day.
        (2012, 12, 26, Closed, T1, "SIX-TC-2012"),
        // 2012-12-31 - T1 - SIX-TC-2012 - New Year's Eve.
        (2012, 12, 31, Closed, T1, "SIX-TC-2012"),
        // SIX-TC-2013: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2013-01-01 - T1 - SIX-TC-2013 - New Year's Day.
        (2013, 1, 1, Closed, T1, "SIX-TC-2013"),
        // 2013-01-02 - T1 - SIX-TC-2013 - St. Berchtold Day.
        (2013, 1, 2, Closed, T1, "SIX-TC-2013"),
        // 2013-03-29 - T1 - SIX-TC-2013 - Good Friday.
        (2013, 3, 29, Closed, T1, "SIX-TC-2013"),
        // 2013-04-01 - T1 - SIX-TC-2013 - Easter Monday.
        (2013, 4, 1, Closed, T1, "SIX-TC-2013"),
        // 2013-05-01 - T1 - SIX-TC-2013 - Labour Day.
        (2013, 5, 1, Closed, T1, "SIX-TC-2013"),
        // 2013-05-09 - T1 - SIX-TC-2013 - Ascension Day.
        (2013, 5, 9, Closed, T1, "SIX-TC-2013"),
        // 2013-05-20 - T1 - SIX-TC-2013 - Whit Monday.
        (2013, 5, 20, Closed, T1, "SIX-TC-2013"),
        // 2013-08-01 - T1 - SIX-TC-2013 - Swiss National Day.
        (2013, 8, 1, Closed, T1, "SIX-TC-2013"),
        // 2013-12-24 - T1 - SIX-TC-2013 - Christmas Eve.
        (2013, 12, 24, Closed, T1, "SIX-TC-2013"),
        // 2013-12-25 - T1 - SIX-TC-2013 - Christmas Day.
        (2013, 12, 25, Closed, T1, "SIX-TC-2013"),
        // 2013-12-26 - T1 - SIX-TC-2013 - St. Stephen's Day.
        (2013, 12, 26, Closed, T1, "SIX-TC-2013"),
        // 2013-12-31 - T1 - SIX-TC-2013 - New Year's Eve.
        (2013, 12, 31, Closed, T1, "SIX-TC-2013"),
        // SIX-TC-2014: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2014-01-01 - T1 - SIX-TC-2014 - New Year's Day.
        (2014, 1, 1, Closed, T1, "SIX-TC-2014"),
        // 2014-01-02 - T1 - SIX-TC-2014 - St. Berchtold Day.
        (2014, 1, 2, Closed, T1, "SIX-TC-2014"),
        // 2014-04-18 - T1 - SIX-TC-2014 - Good Friday.
        (2014, 4, 18, Closed, T1, "SIX-TC-2014"),
        // 2014-04-21 - T1 - SIX-TC-2014 - Easter Monday.
        (2014, 4, 21, Closed, T1, "SIX-TC-2014"),
        // 2014-05-01 - T1 - SIX-TC-2014 - Labour Day.
        (2014, 5, 1, Closed, T1, "SIX-TC-2014"),
        // 2014-05-29 - T1 - SIX-TC-2014 - Ascension Day.
        (2014, 5, 29, Closed, T1, "SIX-TC-2014"),
        // 2014-06-09 - T1 - SIX-TC-2014 - Whit Monday.
        (2014, 6, 9, Closed, T1, "SIX-TC-2014"),
        // 2014-08-01 - T1 - SIX-TC-2014 - Swiss National Day.
        (2014, 8, 1, Closed, T1, "SIX-TC-2014"),
        // 2014-12-24 - T1 - SIX-TC-2014 - Christmas Eve.
        (2014, 12, 24, Closed, T1, "SIX-TC-2014"),
        // 2014-12-25 - T1 - SIX-TC-2014 - Christmas Day.
        (2014, 12, 25, Closed, T1, "SIX-TC-2014"),
        // 2014-12-26 - T1 - SIX-TC-2014 - St. Stephen's Day.
        (2014, 12, 26, Closed, T1, "SIX-TC-2014"),
        // 2014-12-31 - T1 - SIX-TC-2014 - New Year's Eve.
        (2014, 12, 31, Closed, T1, "SIX-TC-2014"),
        // SIX-TC-2015: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2015-01-01 - T1 - SIX-TC-2015 - New Year's Day.
        (2015, 1, 1, Closed, T1, "SIX-TC-2015"),
        // 2015-01-02 - T1 - SIX-TC-2015 - St. Berchtold Day.
        (2015, 1, 2, Closed, T1, "SIX-TC-2015"),
        // 2015-04-03 - T1 - SIX-TC-2015 - Good Friday.
        (2015, 4, 3, Closed, T1, "SIX-TC-2015"),
        // 2015-04-06 - T1 - SIX-TC-2015 - Easter Monday.
        (2015, 4, 6, Closed, T1, "SIX-TC-2015"),
        // 2015-05-01 - T1 - SIX-TC-2015 - Labour Day.
        (2015, 5, 1, Closed, T1, "SIX-TC-2015"),
        // 2015-05-14 - T1 - SIX-TC-2015 - Ascension Day.
        (2015, 5, 14, Closed, T1, "SIX-TC-2015"),
        // 2015-05-25 - T1 - SIX-TC-2015 - Whit Monday.
        (2015, 5, 25, Closed, T1, "SIX-TC-2015"),
        // 2015-12-24 - T1 - SIX-TC-2015 - Christmas Eve.
        (2015, 12, 24, Closed, T1, "SIX-TC-2015"),
        // 2015-12-25 - T1 - SIX-TC-2015 - Christmas Day.
        (2015, 12, 25, Closed, T1, "SIX-TC-2015"),
        // 2015-12-31 - T1 - SIX-TC-2015 - New Year's Eve.
        (2015, 12, 31, Closed, T1, "SIX-TC-2015"),
        // SIX-TC-2016: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2016-01-01 - T1 - SIX-TC-2016 - New Year's Day.
        (2016, 1, 1, Closed, T1, "SIX-TC-2016"),
        // 2016-03-25 - T1 - SIX-TC-2016 - Good Friday.
        (2016, 3, 25, Closed, T1, "SIX-TC-2016"),
        // 2016-03-28 - T1 - SIX-TC-2016 - Easter Monday.
        (2016, 3, 28, Closed, T1, "SIX-TC-2016"),
        // 2016-05-05 - T1 - SIX-TC-2016 - Ascension Day.
        (2016, 5, 5, Closed, T1, "SIX-TC-2016"),
        // 2016-05-16 - T1 - SIX-TC-2016 - Whit Monday.
        (2016, 5, 16, Closed, T1, "SIX-TC-2016"),
        // 2016-08-01 - T1 - SIX-TC-2016 - Swiss National Day.
        (2016, 8, 1, Closed, T1, "SIX-TC-2016"),
        // 2016-12-26 - T1 - SIX-TC-2016 - St. Stephen's Day.
        (2016, 12, 26, Closed, T1, "SIX-TC-2016"),
        // SIX-TC-2017: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2017-01-02 - T1 - SIX-TC-2017 - St. Berchtold Day.
        (2017, 1, 2, Closed, T1, "SIX-TC-2017"),
        // 2017-04-14 - T1 - SIX-TC-2017 - Good Friday.
        (2017, 4, 14, Closed, T1, "SIX-TC-2017"),
        // 2017-04-17 - T1 - SIX-TC-2017 - Easter Monday.
        (2017, 4, 17, Closed, T1, "SIX-TC-2017"),
        // 2017-05-01 - T1 - SIX-TC-2017 - Labour Day.
        (2017, 5, 1, Closed, T1, "SIX-TC-2017"),
        // 2017-05-25 - T1 - SIX-TC-2017 - Ascension Day.
        (2017, 5, 25, Closed, T1, "SIX-TC-2017"),
        // 2017-06-05 - T1 - SIX-TC-2017 - Whit Monday.
        (2017, 6, 5, Closed, T1, "SIX-TC-2017"),
        // 2017-08-01 - T1 - SIX-TC-2017 - Swiss National Day.
        (2017, 8, 1, Closed, T1, "SIX-TC-2017"),
        // 2017-12-25 - T1 - SIX-TC-2017 - Christmas Day.
        (2017, 12, 25, Closed, T1, "SIX-TC-2017"),
        // 2017-12-26 - T1 - SIX-TC-2017 - St. Stephen's Day.
        (2017, 12, 26, Closed, T1, "SIX-TC-2017"),
        // SIX-TG-2018: the operator's Trading Guide of 28 May 2018 (the live
        // education-path compilation), whose "Trading Calendar 2018" section
        // prints the year's twelve month grids with the legend `Saturday —
        // Market Closed / Sunday — Market Closed / Market Holiday — Market
        // Closed`
        // 2018-01-01 - T1 - SIX-TG-2018 - New Year's Day.
        (2018, 1, 1, Closed, T1, "SIX-TG-2018"),
        // 2018-01-02 - T1 - SIX-TG-2018 - St. Berchtold Day.
        (2018, 1, 2, Closed, T1, "SIX-TG-2018"),
        // 2018-03-30 - T1 - SIX-TG-2018 - Good Friday.
        (2018, 3, 30, Closed, T1, "SIX-TG-2018"),
        // 2018-04-02 - T1 - SIX-TG-2018 - Easter Monday.
        (2018, 4, 2, Closed, T1, "SIX-TG-2018"),
        // 2018-05-01 - T1 - SIX-TG-2018 - Labour Day.
        (2018, 5, 1, Closed, T1, "SIX-TG-2018"),
        // 2018-05-10 - T1 - SIX-TG-2018 - Ascension Day.
        (2018, 5, 10, Closed, T1, "SIX-TG-2018"),
        // 2018-05-21 - T1 - SIX-TG-2018 - Whit Monday.
        (2018, 5, 21, Closed, T1, "SIX-TG-2018"),
        // 2018-08-01 - T1 - SIX-TG-2018 - Swiss National Day.
        (2018, 8, 1, Closed, T1, "SIX-TG-2018"),
        // 2018-12-24 - T1 - SIX-TG-2018 - Christmas Eve.
        (2018, 12, 24, Closed, T1, "SIX-TG-2018"),
        // 2018-12-25 - T1 - SIX-TG-2018 - Christmas Day.
        (2018, 12, 25, Closed, T1, "SIX-TG-2018"),
        // 2018-12-26 - T1 - SIX-TG-2018 - St. Stephen's Day.
        (2018, 12, 26, Closed, T1, "SIX-TG-2018"),
        // 2018-12-31 - T1 - SIX-TG-2018 - New Year's Eve.
        (2018, 12, 31, Closed, T1, "SIX-TG-2018"),
        // SIX-TG-2018: the same guide's "Trading Calendar 2019" section, same
        // legend
        // 2019-01-01 - T1 - SIX-TG-2018 - New Year's Day.
        (2019, 1, 1, Closed, T1, "SIX-TG-2018"),
        // 2019-01-02 - T1 - SIX-TG-2018 - St. Berchtold Day.
        (2019, 1, 2, Closed, T1, "SIX-TG-2018"),
        // 2019-04-19 - T1 - SIX-TG-2018 - Good Friday.
        (2019, 4, 19, Closed, T1, "SIX-TG-2018"),
        // 2019-04-22 - T1 - SIX-TG-2018 - Easter Monday.
        (2019, 4, 22, Closed, T1, "SIX-TG-2018"),
        // 2019-05-01 - T1 - SIX-TG-2018 - Labour Day.
        (2019, 5, 1, Closed, T1, "SIX-TG-2018"),
        // 2019-05-30 - T1 - SIX-TG-2018 - Ascension Day.
        (2019, 5, 30, Closed, T1, "SIX-TG-2018"),
        // 2019-06-10 - T1 - SIX-TG-2018 - Whit Monday.
        (2019, 6, 10, Closed, T1, "SIX-TG-2018"),
        // 2019-08-01 - T1 - SIX-TG-2018 - Swiss National Day.
        (2019, 8, 1, Closed, T1, "SIX-TG-2018"),
        // 2019-12-24 - T1 - SIX-TG-2018 - Christmas Eve.
        (2019, 12, 24, Closed, T1, "SIX-TG-2018"),
        // 2019-12-25 - T1 - SIX-TG-2018 - Christmas Day.
        (2019, 12, 25, Closed, T1, "SIX-TG-2018"),
        // 2019-12-26 - T1 - SIX-TG-2018 - St. Stephen's Day.
        (2019, 12, 26, Closed, T1, "SIX-TG-2018"),
        // 2019-12-31 - T1 - SIX-TG-2018 - New Year's Eve. Every one of the
        // guide's 24 dark cells across 2018-2019 falls on a weekday and no
        // SIX holiday of those two years falls on a weekend, so no weekend
        // rule arises in these two grids.
        (2019, 12, 31, Closed, T1, "SIX-TG-2018"),
        // SIX-TC-2020: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2020-01-01 - T1 - SIX-TC-2020 - New Year's Day.
        (2020, 1, 1, Closed, T1, "SIX-TC-2020"),
        // 2020-01-02 - T1 - SIX-TC-2020 - St. Berchtold Day.
        (2020, 1, 2, Closed, T1, "SIX-TC-2020"),
        // 2020-04-10 - T1 - SIX-TC-2020 - Good Friday.
        (2020, 4, 10, Closed, T1, "SIX-TC-2020"),
        // 2020-04-13 - T1 - SIX-TC-2020 - Easter Monday.
        (2020, 4, 13, Closed, T1, "SIX-TC-2020"),
        // 2020-05-01 - T1 - SIX-TC-2020 - Labour Day.
        (2020, 5, 1, Closed, T1, "SIX-TC-2020"),
        // 2020-05-21 - T1 - SIX-TC-2020 - Ascension Day.
        (2020, 5, 21, Closed, T1, "SIX-TC-2020"),
        // 2020-06-01 - T1 - SIX-TC-2020 - Whit Monday.
        (2020, 6, 1, Closed, T1, "SIX-TC-2020"),
        // 2020-12-24 - T1 - SIX-TC-2020 - Christmas Eve.
        (2020, 12, 24, Closed, T1, "SIX-TC-2020"),
        // 2020-12-25 - T1 - SIX-TC-2020 - Christmas Day.
        (2020, 12, 25, Closed, T1, "SIX-TC-2020"),
        // 2020-12-31 - T1 - SIX-TC-2020 - New Year's Eve.
        (2020, 12, 31, Closed, T1, "SIX-TC-2020"),
        // SIX-TC-2021: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2021-01-01 - T1 - SIX-TC-2021 - New Year's Day.
        (2021, 1, 1, Closed, T1, "SIX-TC-2021"),
        // 2021-04-02 - T1 - SIX-TC-2021 - Good Friday.
        (2021, 4, 2, Closed, T1, "SIX-TC-2021"),
        // 2021-04-05 - T1 - SIX-TC-2021 - Easter Monday.
        (2021, 4, 5, Closed, T1, "SIX-TC-2021"),
        // 2021-05-13 - T1 - SIX-TC-2021 - Ascension Day.
        (2021, 5, 13, Closed, T1, "SIX-TC-2021"),
        // 2021-05-24 - T1 - SIX-TC-2021 - Whit Monday.
        (2021, 5, 24, Closed, T1, "SIX-TC-2021"),
        // 2021-12-24 - T1 - SIX-TC-2021 - Christmas Eve.
        (2021, 12, 24, Closed, T1, "SIX-TC-2021"),
        // 2021-12-31 - T1 - SIX-TC-2021 - New Year's Eve.
        (2021, 12, 31, Closed, T1, "SIX-TC-2021"),
        // SIX-TC-2022: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2022-04-15 - T1 - SIX-TC-2022 - Good Friday.
        (2022, 4, 15, Closed, T1, "SIX-TC-2022"),
        // 2022-04-18 - T1 - SIX-TC-2022 - Easter Monday.
        (2022, 4, 18, Closed, T1, "SIX-TC-2022"),
        // 2022-05-26 - T1 - SIX-TC-2022 - Ascension Day.
        (2022, 5, 26, Closed, T1, "SIX-TC-2022"),
        // 2022-06-06 - T1 - SIX-TC-2022 - Whit Monday.
        (2022, 6, 6, Closed, T1, "SIX-TC-2022"),
        // 2022-08-01 - T1 - SIX-TC-2022 - Swiss National Day.
        (2022, 8, 1, Closed, T1, "SIX-TC-2022"),
        // 2022-12-26 - T1 - SIX-TC-2022 - St. Stephen's Day.
        (2022, 12, 26, Closed, T1, "SIX-TC-2022"),
        // SIX-TC-2023: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2023-01-02 - T1 - SIX-TC-2023 - St. Berchtold Day.
        (2023, 1, 2, Closed, T1, "SIX-TC-2023"),
        // 2023-04-07 - T1 - SIX-TC-2023 - Good Friday.
        (2023, 4, 7, Closed, T1, "SIX-TC-2023"),
        // 2023-04-10 - T1 - SIX-TC-2023 - Easter Monday.
        (2023, 4, 10, Closed, T1, "SIX-TC-2023"),
        // 2023-05-01 - T1 - SIX-TC-2023 - Labour Day.
        (2023, 5, 1, Closed, T1, "SIX-TC-2023"),
        // 2023-05-18 - T1 - SIX-TC-2023 - Ascension Day.
        (2023, 5, 18, Closed, T1, "SIX-TC-2023"),
        // 2023-05-29 - T1 - SIX-TC-2023 - Whit Monday.
        (2023, 5, 29, Closed, T1, "SIX-TC-2023"),
        // 2023-08-01 - T1 - SIX-TC-2023 - Swiss National Day.
        (2023, 8, 1, Closed, T1, "SIX-TC-2023"),
        // 2023-12-25 - T1 - SIX-TC-2023 - Christmas Day.
        (2023, 12, 25, Closed, T1, "SIX-TC-2023"),
        // 2023-12-26 - T1 - SIX-TC-2023 - St. Stephen's Day.
        (2023, 12, 26, Closed, T1, "SIX-TC-2023"),
        // SIX-TC-2024: the year's own Trading Calendar PDF, dark `Market Holiday —
        // Market Closed` cells
        // 2024-01-01 - T1 - SIX-TC-2024 - New Year's Day.
        (2024, 1, 1, Closed, T1, "SIX-TC-2024"),
        // 2024-01-02 - T1 - SIX-TC-2024 - St. Berchtold Day.
        (2024, 1, 2, Closed, T1, "SIX-TC-2024"),
        // 2024-03-29 - T1 - SIX-TC-2024 - Good Friday.
        (2024, 3, 29, Closed, T1, "SIX-TC-2024"),
        // 2024-04-01 - T1 - SIX-TC-2024 - Easter Monday.
        (2024, 4, 1, Closed, T1, "SIX-TC-2024"),
        // 2024-05-01 - T1 - SIX-TC-2024 - Labour Day.
        (2024, 5, 1, Closed, T1, "SIX-TC-2024"),
        // 2024-05-09 - T1 - SIX-TC-2024 - Ascension Day.
        (2024, 5, 9, Closed, T1, "SIX-TC-2024"),
        // 2024-05-20 - T1 - SIX-TC-2024 - Whit Monday.
        (2024, 5, 20, Closed, T1, "SIX-TC-2024"),
        // 2024-08-01 - T1 - SIX-TC-2024 - Swiss National Day.
        (2024, 8, 1, Closed, T1, "SIX-TC-2024"),
        // 2024-12-24 - T1 - SIX-TC-2024 - Christmas Eve.
        (2024, 12, 24, Closed, T1, "SIX-TC-2024"),
        // 2024-12-25 - T1 - SIX-TC-2024 - Christmas Day.
        (2024, 12, 25, Closed, T1, "SIX-TC-2024"),
        // 2024-12-26 - T1 - SIX-TC-2024 - St. Stephen's Day.
        (2024, 12, 26, Closed, T1, "SIX-TC-2024"),
        // 2024-12-31 - T1 - SIX-TC-2024 - New Year's Eve.
        (2024, 12, 31, Closed, T1, "SIX-TC-2024"),

        // 2025-01-01 - T1 - SIX-TC-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "SIX-TC-2025"),
        // 2025-01-02 - T1 - SIX-TC-2025 - St. Berchtold Day.
        (2025, 1, 2, Closed, T1, "SIX-TC-2025"),
        // 2025-04-18 - T1 - SIX-TC-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "SIX-TC-2025"),
        // 2025-04-21 - T1 - SIX-TC-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "SIX-TC-2025"),
        // 2025-05-01 - T1 - SIX-TC-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "SIX-TC-2025"),
        // 2025-05-29 - T1 - SIX-TC-2025 - Ascension Day.
        (2025, 5, 29, Closed, T1, "SIX-TC-2025"),
        // 2025-06-09 - T1 - SIX-TC-2025 - Whit Monday.
        (2025, 6, 9, Closed, T1, "SIX-TC-2025"),
        // 2025-08-01 - T1 - SIX-TC-2025 - Swiss National Day.
        (2025, 8, 1, Closed, T1, "SIX-TC-2025"),
        // 2025-12-24 - T1 - SIX-TC-2025 - Christmas Eve.
        (2025, 12, 24, Closed, T1, "SIX-TC-2025"),
        // 2025-12-25 - T1 - SIX-TC-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "SIX-TC-2025"),
        // 2025-12-26 - T1 - SIX-TC-2025 - St. Stephen's Day.
        (2025, 12, 26, Closed, T1, "SIX-TC-2025"),
        // 2025-12-31 - T1 - SIX-TC-2025 - New Year's Eve.
        (2025, 12, 31, Closed, T1, "SIX-TC-2025"),
        // 2026-01-01 - T1 - SIX-TC-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "SIX-TC-2026"),
        // 2026-01-02 - T1 - SIX-TC-2026 - St. Berchtold Day.
        (2026, 1, 2, Closed, T1, "SIX-TC-2026"),
        // 2026-04-03 - T1 - SIX-TC-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "SIX-TC-2026"),
        // 2026-04-06 - T1 - SIX-TC-2026 - Easter Monday.
        (2026, 4, 6, Closed, T1, "SIX-TC-2026"),
        // 2026-05-01 - T1 - SIX-TC-2026 - Labour Day.
        (2026, 5, 1, Closed, T1, "SIX-TC-2026"),
        // 2026-05-14 - T1 - SIX-TC-2026 - Ascension Day.
        (2026, 5, 14, Closed, T1, "SIX-TC-2026"),
        // 2026-05-25 - T1 - SIX-TC-2026 - Whit Monday.
        (2026, 5, 25, Closed, T1, "SIX-TC-2026"),
        // 2026-12-24 - T1 - SIX-TC-2026 - Christmas Eve.
        (2026, 12, 24, Closed, T1, "SIX-TC-2026"),
        // 2026-12-25 - T1 - SIX-TC-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "SIX-TC-2026"),
        // 2026-12-31 - T1 - SIX-TC-2026 - New Year's Eve. The calendar marks
        // no weekday holiday for Swiss National Day 2026 (Saturday) or St.
        // Stephen's Day 2026 (Saturday).
        (2026, 12, 31, Closed, T1, "SIX-TC-2026"),
        // 2027-01-01 - T1 - SIX-TC-2027 - New Year's Day. St. Berchtold Day
        // 2027 falls on Saturday and is marked as a weekend.
        (2027, 1, 1, Closed, T1, "SIX-TC-2027"),
        // 2027-03-26 - T1 - SIX-TC-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "SIX-TC-2027"),
        // 2027-03-29 - T1 - SIX-TC-2027 - Easter Monday.
        (2027, 3, 29, Closed, T1, "SIX-TC-2027"),
        // 2027-05-06 - T1 - SIX-TC-2027 - Ascension Day.
        (2027, 5, 6, Closed, T1, "SIX-TC-2027"),
        // 2027-05-17 - T1 - SIX-TC-2027 - Whit Monday. Labour Day 2027 falls
        // on Saturday and is marked as a weekend.
        (2027, 5, 17, Closed, T1, "SIX-TC-2027"),
        // 2027-12-24 - T1 - SIX-TC-2027 - Christmas Eve. Christmas Day 2027
        // (Saturday) and St. Stephen's Day 2027 (Sunday) are marked as a
        // weekend, and Swiss National Day 2027 (Sunday) likewise.
        (2027, 12, 24, Closed, T1, "SIX-TC-2027"),
        // 2027-12-31 - T1 - SIX-TC-2027 - New Year's Eve.
        (2027, 12, 31, Closed, T1, "SIX-TC-2027"),
    ],
};
