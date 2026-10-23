// SPDX-License-Identifier: MIT-0

//! Eurex SMI® index futures holiday rows, 2010 through 2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Zurich` (design
//! memo D1). The conversion is the identity here: every FSMI phase the crate
//! models runs inside one Zurich civil day, so no session wraps a midnight and
//! an event date is its own trade date.
//!
//! The block is **T1** throughout, keyed by the same operator documents as the
//! sibling `eurex` tables: the annual **Trading Calendar** editions 2010-2024
//! (`EUREX-CAL-2010` through `EUREX-CAL-2024`, each printing an `Overview of
//! holidays by countries`) and the day-by-day **Holiday regulations** page for
//! 2025-2026 (`EUREX-HOLREG-2025`, `EUREX-HOLREG-2026`), each corroborated by
//! the matching edition PDF. Two of the editions' scope lines reach this
//! family and only these two:
//!
//! - the `in all derivatives` lists close every product and key their rows
//!   here exactly as they key the sibling tables; and
//! - the recurring Swiss clause — `Eurex is closed for trading and clearing
//!   (exercise and settlement) in Swiss fixed income as well as equity and
//!   equity index derivatives; no cash payment in CHF: …` — names Swiss equity
//!   index derivatives, which FSMI is, so each date it lists keys a row here.
//!   The clause's `no cash payment in CHF` tail is a clearing-cash annotation;
//!   the 2015 edition's separate `No cash payment in CHF, GBP and KRW:
//!   25 May` line is the same annotation on that year's all-derivatives Whit
//!   Monday closure and keys nothing of its own.
//!
//! The German-scope notes — `Eurex is closed for trading and exercise in
//! German equity and equity index derivatives as well as ETF and ETC
//! derivatives, which are based on Xetra® listings: …` — never name this
//! family, so the five German-only rows the sibling `TABLE` carries
//! (2014-10-03, 2016-10-03, 2017-10-03, 2017-10-31 and 2018-10-03) do **not**
//! appear here and FSMI keeps trading through them. The three Whit Mondays the
//! German note also closes (2016-05-16, 2017-06-05 and 2018-05-21) are Swiss
//! clause dates as well, so they close here on the Swiss line regardless. The
//! panel's grammar prints a scope explicitly when a closure reaches it, as the
//! Swiss clause does, and the family's own test fences the split.
//!
//! Eurex publishes no early closes — 24 and 31 December are full trading
//! closures with clearing open — so every row is `Closed`. Coverage stops at
//! 2026-12-31: the operator's 2027-2036 calendars exist only "on a preliminary
//! and indicative basis … and are subject to change", which is not an
//! unconditional dated future, so LAW-NO-FABRICATED-DATES keeps them out.
//! [`TABLE`](super::eurex::TABLE) serves `Exchange::Eurex` and the `eurex`
//! key; [`FIXED_INCOME`](super::eurex::FIXED_INCOME)
//! serves the `eurex_fixed_income` key; this table serves the `eurex_smi` key.
//! All three resolve through
//! [`docs/evidence/eurex_smi.md`](../../../../../docs/evidence/eurex_smi.md)
//! beside the sibling files.

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// Eurex SMI index futures rows: the all-derivatives closures plus the
/// Swiss-scope closures, and the window they were audited over.
///
/// Serves the `eurex_smi` key. The 57 Swiss-scope rows (the clause dates the
/// editions list per year, two to four each) sit beside the 108 all-derivatives
/// rows the sibling tables ship; the German-scope rows the sibling `TABLE`
/// alone carries appear here on no date.
// Evidence: docs/evidence/eurex_smi.md
pub(crate) static SMI: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - EUREX-CAL-2010 - New Year's Day.
        (2010, 1, 1, Closed, T1, "EUREX-CAL-2010"),
        // 2010-04-02 - T1 - EUREX-CAL-2010 - Good Friday.
        (2010, 4, 2, Closed, T1, "EUREX-CAL-2010"),
        // 2010-04-05 - T1 - EUREX-CAL-2010 - Easter Monday.
        (2010, 4, 5, Closed, T1, "EUREX-CAL-2010"),
        // 2010-05-13 - T1 - EUREX-CAL-2010 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2010, 5, 13, Closed, T1, "EUREX-CAL-2010"),
        // 2010-05-24 - T1 - EUREX-CAL-2010 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2010, 5, 24, Closed, T1, "EUREX-CAL-2010"),
        // 2010-12-24 - T1 - EUREX-CAL-2010 - Christmas Eve, a full trading closure; clearing stays open.
        (2010, 12, 24, Closed, T1, "EUREX-CAL-2010"),
        // 2010-12-31 - T1 - EUREX-CAL-2010 - New Year's Eve, a full trading closure; clearing stays open.
        (2010, 12, 31, Closed, T1, "EUREX-CAL-2010"),
        // 2011-04-22 - T1 - EUREX-CAL-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "EUREX-CAL-2011"),
        // 2011-04-25 - T1 - EUREX-CAL-2011 - Easter Monday.
        (2011, 4, 25, Closed, T1, "EUREX-CAL-2011"),
        // 2011-06-02 - T1 - EUREX-CAL-2011 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2011, 6, 2, Closed, T1, "EUREX-CAL-2011"),
        // 2011-06-13 - T1 - EUREX-CAL-2011 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2011, 6, 13, Closed, T1, "EUREX-CAL-2011"),
        // 2011-08-01 - T1 - EUREX-CAL-2011 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2011, 8, 1, Closed, T1, "EUREX-CAL-2011"),
        // 2011-12-26 - T1 - EUREX-CAL-2011 - Boxing Day.
        (2011, 12, 26, Closed, T1, "EUREX-CAL-2011"),
        // 2012-01-02 - T1 - EUREX-CAL-2012 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2012, 1, 2, Closed, T1, "EUREX-CAL-2012"),
        // 2012-04-06 - T1 - EUREX-CAL-2012 - Good Friday.
        (2012, 4, 6, Closed, T1, "EUREX-CAL-2012"),
        // 2012-04-09 - T1 - EUREX-CAL-2012 - Easter Monday.
        (2012, 4, 9, Closed, T1, "EUREX-CAL-2012"),
        // 2012-05-01 - T1 - EUREX-CAL-2012 - Labour Day.
        (2012, 5, 1, Closed, T1, "EUREX-CAL-2012"),
        // 2012-05-17 - T1 - EUREX-CAL-2012 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2012, 5, 17, Closed, T1, "EUREX-CAL-2012"),
        // 2012-05-28 - T1 - EUREX-CAL-2012 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2012, 5, 28, Closed, T1, "EUREX-CAL-2012"),
        // 2012-08-01 - T1 - EUREX-CAL-2012 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2012, 8, 1, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-24 - T1 - EUREX-CAL-2012 - Christmas Eve, a full trading closure; clearing stays open.
        (2012, 12, 24, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-25 - T1 - EUREX-CAL-2012 - Christmas Day.
        (2012, 12, 25, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-26 - T1 - EUREX-CAL-2012 - Boxing Day.
        (2012, 12, 26, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-31 - T1 - EUREX-CAL-2012 - New Year's Eve, a full trading closure; clearing stays open.
        (2012, 12, 31, Closed, T1, "EUREX-CAL-2012"),
        // 2013-01-01 - T1 - EUREX-CAL-2013 - New Year's Day.
        (2013, 1, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-01-02 - T1 - EUREX-CAL-2013 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2013, 1, 2, Closed, T1, "EUREX-CAL-2013"),
        // 2013-03-29 - T1 - EUREX-CAL-2013 - Good Friday.
        (2013, 3, 29, Closed, T1, "EUREX-CAL-2013"),
        // 2013-04-01 - T1 - EUREX-CAL-2013 - Easter Monday.
        (2013, 4, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-05-01 - T1 - EUREX-CAL-2013 - Labour Day.
        (2013, 5, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-05-09 - T1 - EUREX-CAL-2013 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2013, 5, 9, Closed, T1, "EUREX-CAL-2013"),
        // 2013-05-20 - T1 - EUREX-CAL-2013 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2013, 5, 20, Closed, T1, "EUREX-CAL-2013"),
        // 2013-08-01 - T1 - EUREX-CAL-2013 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2013, 8, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-24 - T1 - EUREX-CAL-2013 - Christmas Eve, a full trading closure; clearing stays open.
        (2013, 12, 24, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-25 - T1 - EUREX-CAL-2013 - Christmas Day.
        (2013, 12, 25, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-26 - T1 - EUREX-CAL-2013 - Boxing Day.
        (2013, 12, 26, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-31 - T1 - EUREX-CAL-2013 - New Year's Eve, a full trading closure; clearing stays open.
        (2013, 12, 31, Closed, T1, "EUREX-CAL-2013"),
        // 2014-01-01 - T1 - EUREX-CAL-2014 - New Year's Day.
        (2014, 1, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-01-02 - T1 - EUREX-CAL-2014 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2014, 1, 2, Closed, T1, "EUREX-CAL-2014"),
        // 2014-04-18 - T1 - EUREX-CAL-2014 - Good Friday.
        (2014, 4, 18, Closed, T1, "EUREX-CAL-2014"),
        // 2014-04-21 - T1 - EUREX-CAL-2014 - Easter Monday.
        (2014, 4, 21, Closed, T1, "EUREX-CAL-2014"),
        // 2014-05-01 - T1 - EUREX-CAL-2014 - Labour Day.
        (2014, 5, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-05-29 - T1 - EUREX-CAL-2014 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2014, 5, 29, Closed, T1, "EUREX-CAL-2014"),
        // 2014-06-09 - T1 - EUREX-CAL-2014 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2014, 6, 9, Closed, T1, "EUREX-CAL-2014"),
        // 2014-08-01 - T1 - EUREX-CAL-2014 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2014, 8, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-24 - T1 - EUREX-CAL-2014 - Christmas Eve, a full trading closure; clearing stays open.
        (2014, 12, 24, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-25 - T1 - EUREX-CAL-2014 - Christmas Day.
        (2014, 12, 25, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-26 - T1 - EUREX-CAL-2014 - Boxing Day.
        (2014, 12, 26, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-31 - T1 - EUREX-CAL-2014 - New Year's Eve, a full trading closure; clearing stays open.
        (2014, 12, 31, Closed, T1, "EUREX-CAL-2014"),
        // 2015-01-01 - T1 - EUREX-CAL-2015 - New Year's Day.
        (2015, 1, 1, Closed, T1, "EUREX-CAL-2015"),
        // 2015-01-02 - T1 - EUREX-CAL-2015 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2015, 1, 2, Closed, T1, "EUREX-CAL-2015"),
        // 2015-04-03 - T1 - EUREX-CAL-2015 - Good Friday.
        (2015, 4, 3, Closed, T1, "EUREX-CAL-2015"),
        // 2015-04-06 - T1 - EUREX-CAL-2015 - Easter Monday.
        (2015, 4, 6, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-01 - T1 - EUREX-CAL-2015 - Labour Day.
        (2015, 5, 1, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-14 - T1 - EUREX-CAL-2015 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2015, 5, 14, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-25 - T1 - EUREX-CAL-2015 - Whit Monday, a full trading closure; clearing stays open.
        (2015, 5, 25, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-24 - T1 - EUREX-CAL-2015 - Christmas Eve, a full trading closure; clearing stays open.
        (2015, 12, 24, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-25 - T1 - EUREX-CAL-2015 - Christmas Day.
        (2015, 12, 25, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-26 - T1 - EUREX-CAL-2015 - Boxing Day.
        (2015, 12, 26, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-31 - T1 - EUREX-CAL-2015 - New Year's Eve, a full trading closure; clearing stays open.
        (2015, 12, 31, Closed, T1, "EUREX-CAL-2015"),
        // 2016-01-01 - T1 - EUREX-CAL-2016 - New Year's Day.
        (2016, 1, 1, Closed, T1, "EUREX-CAL-2016"),
        // 2016-03-25 - T1 - EUREX-CAL-2016 - Good Friday.
        (2016, 3, 25, Closed, T1, "EUREX-CAL-2016"),
        // 2016-03-28 - T1 - EUREX-CAL-2016 - Easter Monday.
        (2016, 3, 28, Closed, T1, "EUREX-CAL-2016"),
        // 2016-05-05 - T1 - EUREX-CAL-2016 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2016, 5, 5, Closed, T1, "EUREX-CAL-2016"),
        // 2016-05-16 - T1 - EUREX-CAL-2016 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2016, 5, 16, Closed, T1, "EUREX-CAL-2016"),
        // 2016-08-01 - T1 - EUREX-CAL-2016 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2016, 8, 1, Closed, T1, "EUREX-CAL-2016"),
        // 2016-12-26 - T1 - EUREX-CAL-2016 - Boxing Day.
        (2016, 12, 26, Closed, T1, "EUREX-CAL-2016"),
        // 2017-01-02 - T1 - EUREX-CAL-2017 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2017, 1, 2, Closed, T1, "EUREX-CAL-2017"),
        // 2017-04-14 - T1 - EUREX-CAL-2017 - Good Friday.
        (2017, 4, 14, Closed, T1, "EUREX-CAL-2017"),
        // 2017-04-17 - T1 - EUREX-CAL-2017 - Easter Monday.
        (2017, 4, 17, Closed, T1, "EUREX-CAL-2017"),
        // 2017-05-01 - T1 - EUREX-CAL-2017 - Labour Day.
        (2017, 5, 1, Closed, T1, "EUREX-CAL-2017"),
        // 2017-05-25 - T1 - EUREX-CAL-2017 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2017, 5, 25, Closed, T1, "EUREX-CAL-2017"),
        // 2017-06-05 - T1 - EUREX-CAL-2017 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2017, 6, 5, Closed, T1, "EUREX-CAL-2017"),
        // 2017-08-01 - T1 - EUREX-CAL-2017 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2017, 8, 1, Closed, T1, "EUREX-CAL-2017"),
        // 2017-12-25 - T1 - EUREX-CAL-2017 - Christmas Day.
        (2017, 12, 25, Closed, T1, "EUREX-CAL-2017"),
        // 2017-12-26 - T1 - EUREX-CAL-2017 - Boxing Day.
        (2017, 12, 26, Closed, T1, "EUREX-CAL-2017"),
        // 2018-01-01 - T1 - EUREX-CAL-2018 - New Year's Day.
        (2018, 1, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-01-02 - T1 - EUREX-CAL-2018 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2018, 1, 2, Closed, T1, "EUREX-CAL-2018"),
        // 2018-03-30 - T1 - EUREX-CAL-2018 - Good Friday.
        (2018, 3, 30, Closed, T1, "EUREX-CAL-2018"),
        // 2018-04-02 - T1 - EUREX-CAL-2018 - Easter Monday.
        (2018, 4, 2, Closed, T1, "EUREX-CAL-2018"),
        // 2018-05-01 - T1 - EUREX-CAL-2018 - Labour Day.
        (2018, 5, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-05-10 - T1 - EUREX-CAL-2018 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2018, 5, 10, Closed, T1, "EUREX-CAL-2018"),
        // 2018-05-21 - T1 - EUREX-CAL-2018 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2018, 5, 21, Closed, T1, "EUREX-CAL-2018"),
        // 2018-08-01 - T1 - EUREX-CAL-2018 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2018, 8, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-24 - T1 - EUREX-CAL-2018 - Christmas Eve, a full trading closure; clearing stays open.
        (2018, 12, 24, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-25 - T1 - EUREX-CAL-2018 - Christmas Day.
        (2018, 12, 25, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-26 - T1 - EUREX-CAL-2018 - Boxing Day.
        (2018, 12, 26, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-31 - T1 - EUREX-CAL-2018 - New Year's Eve, a full trading closure; clearing stays open.
        (2018, 12, 31, Closed, T1, "EUREX-CAL-2018"),
        // 2019-01-01 - T1 - EUREX-CAL-2019 - New Year's Day.
        (2019, 1, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-01-02 - T1 - EUREX-CAL-2019 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2019, 1, 2, Closed, T1, "EUREX-CAL-2019"),
        // 2019-04-19 - T1 - EUREX-CAL-2019 - Good Friday.
        (2019, 4, 19, Closed, T1, "EUREX-CAL-2019"),
        // 2019-04-22 - T1 - EUREX-CAL-2019 - Easter Monday.
        (2019, 4, 22, Closed, T1, "EUREX-CAL-2019"),
        // 2019-05-01 - T1 - EUREX-CAL-2019 - Labour Day.
        (2019, 5, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-05-30 - T1 - EUREX-CAL-2019 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2019, 5, 30, Closed, T1, "EUREX-CAL-2019"),
        // 2019-06-10 - T1 - EUREX-CAL-2019 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2019, 6, 10, Closed, T1, "EUREX-CAL-2019"),
        // 2019-08-01 - T1 - EUREX-CAL-2019 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2019, 8, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-24 - T1 - EUREX-CAL-2019 - Christmas Eve, a full trading closure; clearing stays open.
        (2019, 12, 24, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-25 - T1 - EUREX-CAL-2019 - Christmas Day.
        (2019, 12, 25, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-26 - T1 - EUREX-CAL-2019 - Boxing Day.
        (2019, 12, 26, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-31 - T1 - EUREX-CAL-2019 - New Year's Eve, a full trading closure; clearing stays open.
        (2019, 12, 31, Closed, T1, "EUREX-CAL-2019"),
        // 2020-01-01 - T1 - EUREX-CAL-2020 - New Year's Day.
        (2020, 1, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-01-02 - T1 - EUREX-CAL-2020 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2020, 1, 2, Closed, T1, "EUREX-CAL-2020"),
        // 2020-04-10 - T1 - EUREX-CAL-2020 - Good Friday.
        (2020, 4, 10, Closed, T1, "EUREX-CAL-2020"),
        // 2020-04-13 - T1 - EUREX-CAL-2020 - Easter Monday.
        (2020, 4, 13, Closed, T1, "EUREX-CAL-2020"),
        // 2020-05-01 - T1 - EUREX-CAL-2020 - Labour Day.
        (2020, 5, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-05-21 - T1 - EUREX-CAL-2020 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2020, 5, 21, Closed, T1, "EUREX-CAL-2020"),
        // 2020-06-01 - T1 - EUREX-CAL-2020 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2020, 6, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-24 - T1 - EUREX-CAL-2020 - Christmas Eve, a full trading closure; clearing stays open.
        (2020, 12, 24, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-25 - T1 - EUREX-CAL-2020 - Christmas Day.
        (2020, 12, 25, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-31 - T1 - EUREX-CAL-2020 - New Year's Eve, a full trading closure; clearing stays open.
        (2020, 12, 31, Closed, T1, "EUREX-CAL-2020"),
        // 2021-01-01 - T1 - EUREX-CAL-2021 - New Year's Day.
        (2021, 1, 1, Closed, T1, "EUREX-CAL-2021"),
        // 2021-04-02 - T1 - EUREX-CAL-2021 - Good Friday.
        (2021, 4, 2, Closed, T1, "EUREX-CAL-2021"),
        // 2021-04-05 - T1 - EUREX-CAL-2021 - Easter Monday.
        (2021, 4, 5, Closed, T1, "EUREX-CAL-2021"),
        // 2021-05-13 - T1 - EUREX-CAL-2021 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2021, 5, 13, Closed, T1, "EUREX-CAL-2021"),
        // 2021-05-24 - T1 - EUREX-CAL-2021 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2021, 5, 24, Closed, T1, "EUREX-CAL-2021"),
        // 2021-12-24 - T1 - EUREX-CAL-2021 - Christmas Eve, a full trading closure; clearing stays open.
        (2021, 12, 24, Closed, T1, "EUREX-CAL-2021"),
        // 2021-12-31 - T1 - EUREX-CAL-2021 - New Year's Eve, a full trading closure; clearing stays open.
        (2021, 12, 31, Closed, T1, "EUREX-CAL-2021"),
        // 2022-04-15 - T1 - EUREX-CAL-2022 - Good Friday.
        (2022, 4, 15, Closed, T1, "EUREX-CAL-2022"),
        // 2022-04-18 - T1 - EUREX-CAL-2022 - Easter Monday.
        (2022, 4, 18, Closed, T1, "EUREX-CAL-2022"),
        // 2022-05-26 - T1 - EUREX-CAL-2022 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2022, 5, 26, Closed, T1, "EUREX-CAL-2022"),
        // 2022-06-06 - T1 - EUREX-CAL-2022 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2022, 6, 6, Closed, T1, "EUREX-CAL-2022"),
        // 2022-08-01 - T1 - EUREX-CAL-2022 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2022, 8, 1, Closed, T1, "EUREX-CAL-2022"),
        // 2022-12-26 - T1 - EUREX-CAL-2022 - Boxing Day.
        (2022, 12, 26, Closed, T1, "EUREX-CAL-2022"),
        // 2023-01-02 - T1 - EUREX-CAL-2023 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2023, 1, 2, Closed, T1, "EUREX-CAL-2023"),
        // 2023-04-07 - T1 - EUREX-CAL-2023 - Good Friday.
        (2023, 4, 7, Closed, T1, "EUREX-CAL-2023"),
        // 2023-04-10 - T1 - EUREX-CAL-2023 - Easter Monday.
        (2023, 4, 10, Closed, T1, "EUREX-CAL-2023"),
        // 2023-05-01 - T1 - EUREX-CAL-2023 - Labour Day.
        (2023, 5, 1, Closed, T1, "EUREX-CAL-2023"),
        // 2023-05-18 - T1 - EUREX-CAL-2023 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2023, 5, 18, Closed, T1, "EUREX-CAL-2023"),
        // 2023-05-29 - T1 - EUREX-CAL-2023 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2023, 5, 29, Closed, T1, "EUREX-CAL-2023"),
        // 2023-08-01 - T1 - EUREX-CAL-2023 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2023, 8, 1, Closed, T1, "EUREX-CAL-2023"),
        // 2023-12-25 - T1 - EUREX-CAL-2023 - Christmas Day.
        (2023, 12, 25, Closed, T1, "EUREX-CAL-2023"),
        // 2023-12-26 - T1 - EUREX-CAL-2023 - Boxing Day.
        (2023, 12, 26, Closed, T1, "EUREX-CAL-2023"),
        // 2024-01-01 - T1 - EUREX-CAL-2024 - New Year's Day.
        (2024, 1, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-01-02 - T1 - EUREX-CAL-2024 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2024, 1, 2, Closed, T1, "EUREX-CAL-2024"),
        // 2024-03-29 - T1 - EUREX-CAL-2024 - Good Friday.
        (2024, 3, 29, Closed, T1, "EUREX-CAL-2024"),
        // 2024-04-01 - T1 - EUREX-CAL-2024 - Easter Monday.
        (2024, 4, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-05-01 - T1 - EUREX-CAL-2024 - Labour Day.
        (2024, 5, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-05-09 - T1 - EUREX-CAL-2024 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2024, 5, 9, Closed, T1, "EUREX-CAL-2024"),
        // 2024-05-20 - T1 - EUREX-CAL-2024 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2024, 5, 20, Closed, T1, "EUREX-CAL-2024"),
        // 2024-08-01 - T1 - EUREX-CAL-2024 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2024, 8, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-24 - T1 - EUREX-CAL-2024 - Christmas Eve, a full trading closure; clearing stays open.
        (2024, 12, 24, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-25 - T1 - EUREX-CAL-2024 - Christmas Day.
        (2024, 12, 25, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-26 - T1 - EUREX-CAL-2024 - Boxing Day.
        (2024, 12, 26, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-31 - T1 - EUREX-CAL-2024 - New Year's Eve, a full trading closure; clearing stays open.
        (2024, 12, 31, Closed, T1, "EUREX-CAL-2024"),
        // 2025-01-01 - T1 - EUREX-HOLREG-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-01-02 - T1 - EUREX-HOLREG-2025 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2025, 1, 2, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-18 - T1 - EUREX-HOLREG-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-21 - T1 - EUREX-HOLREG-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-05-01 - T1 - EUREX-HOLREG-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-05-29 - T1 - EUREX-HOLREG-2025 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2025, 5, 29, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-06-09 - T1 - EUREX-HOLREG-2025 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2025, 6, 9, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-08-01 - T1 - EUREX-HOLREG-2025 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2025, 8, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-24 - T1 - EUREX-HOLREG-2025 - Christmas Eve, a full trading closure.
        (2025, 12, 24, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-25 - T1 - EUREX-HOLREG-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-26 - T1 - EUREX-HOLREG-2025 - Boxing Day.
        (2025, 12, 26, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-31 - T1 - EUREX-HOLREG-2025 - New Year's Eve, a full trading closure.
        (2025, 12, 31, Closed, T1, "EUREX-HOLREG-2025"),
        // 2026-01-01 - T1 - EUREX-HOLREG-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-01-02 - T1 - EUREX-HOLREG-2026 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2026, 1, 2, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-03 - T1 - EUREX-HOLREG-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-06 - T1 - EUREX-HOLREG-2026 - Easter Monday.
        (2026, 4, 6, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-05-01 - T1 - EUREX-HOLREG-2026 - Labour Day.
        (2026, 5, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-05-14 - T1 - EUREX-HOLREG-2026 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2026, 5, 14, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-05-25 - T1 - EUREX-HOLREG-2026 - Swiss scope: the edition's `in Swiss fixed income as well as equity and equity index derivatives` clause closes FSMI.
        (2026, 5, 25, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-24 - T1 - EUREX-HOLREG-2026 - Christmas Eve, a full trading closure.
        (2026, 12, 24, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-25 - T1 - EUREX-HOLREG-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-31 - T1 - EUREX-HOLREG-2026 - New Year's Eve, a full trading closure.
        (2026, 12, 31, Closed, T1, "EUREX-HOLREG-2026"),
    ],
};
