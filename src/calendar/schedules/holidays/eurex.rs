// SPDX-License-Identifier: MIT-0

//! Eurex holiday rows, 2010 through 2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Berlin` (design
//! memo D1). The conversion is the identity here: every Eurex phase the crate
//! models runs inside one Berlin civil day, so no session wraps a midnight and
//! an event date is its own trade date.
//!
//! The block is **T1** throughout. Two document families key it: the
//! operator's annual **Trading Calendar** editions 2010-2024 — `Eurex Trading
//! Calendar 2010` through `Eurex trading calendar 2024`, each printing an
//! `Overview of holidays by countries` whose first entry states the closures
//! `in all derivatives` in session language — key the 2010-2024 rows, and the
//! `Holiday regulations` page (§ 2025 as captured on 2025-09-13 and § 2026,
//! each corroborated by the matching Trading Calendar PDF) keys 2025-2026.
//! Eurex publishes no early closes — 24 and 31 December are full trading
//! closures with clearing open — so every row is `Closed`.
//!
//! Coverage stops at 2026-12-31. Eurex's 2027-2036 calendars exist only "on a
//! preliminary and indicative basis … and are subject to change", which is not
//! an unconditional dated future, so LAW-NO-FABRICATED-DATES keeps them out.
//! The unresolved "to be announced" line the 2025 and 2026 editions print for
//! additional German closures in FDAX/FDXM — the only span the operator still
//! withholds — is declared in `schedules/sourcing.rs` and recorded in
//! [`docs/evidence/eurex.md`](../../../../../docs/evidence/eurex.md).
//!
//! **Two tables, split by the operator's own scope lines.** The editions print
//! two kinds of closure note. The `in all derivatives` lists close every
//! product and key both tables. The German-scope notes — `Eurex is closed for
//! trading and exercise in German equity and equity index derivatives as well
//! as ETF and ETC derivatives, which are based on Xetra® listings: …` — name
//! only the German equity and equity-index products and the Xetra-listed
//! ETF/ETC derivatives, so they key [`TABLE`] (the FESX/FDAX/FDXM
//! benchmark-index family, whose FDAX and FDXM members the note closes) and
//! never [`FIXED_INCOME`] (the fixed-income family, which the note does not
//! name; the panel's grammar prints fixed income explicitly when a closure
//! reaches it, as the Swiss line does). The German-scope rows are dated in the
//! 2014, 2016, 2017 and 2018 editions and encode as rows in [`TABLE`]; the
//! 2019-2021 editions date the same scope but print `(trading in German equity
//! index futures takes place!)`, so nothing closes in the futures scope those
//! years; the 2010-2013, 2015 and 2022-2024 editions print no German-scope
//! line at all in a panel that enumerates every other country's closures.
//!
//! [`TABLE`] serves `Exchange::Eurex` and the `eurex` key; [`FIXED_INCOME`]
//! serves the `eurex_fixed_income` key. The all-derivatives rows are the same
//! in both because the operator states them for every product; the German
//! rows are [`TABLE`]'s alone because the operator scopes them to the German
//! equity products.

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// Eurex's benchmark-index family rows — the all-derivatives closures plus the
/// dated German-scope closures — and the window they were audited over.
///
/// Serves `Exchange::Eurex` and the `eurex` key. The eight German-scope rows
/// (2014-10-03; 2016-05-16 and 2016-10-03; 2017-06-05, 2017-10-03 and
/// 2017-10-31; 2018-05-21 and 2018-10-03) key the FDAX/FDXM members the
/// operator's German-scope note closes; the fixed-income family, which the
/// note never names, ships [`FIXED_INCOME`] without them.
// Evidence: docs/evidence/eurex.md, docs/evidence/eurex_key.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - EUREX-CAL-2010 - New Year's Day, trading and clearing.
        (2010, 1, 1, Closed, T1, "EUREX-CAL-2010"),
        // 2010-04-02 - T1 - EUREX-CAL-2010 - Good Friday, trading and clearing.
        (2010, 4, 2, Closed, T1, "EUREX-CAL-2010"),
        // 2010-04-05 - T1 - EUREX-CAL-2010 - Easter Monday, trading and clearing.
        (2010, 4, 5, Closed, T1, "EUREX-CAL-2010"),
        // 2010-12-24 - T1 - EUREX-CAL-2010 - Christmas Eve, a full trading closure; clearing stays open.
        (2010, 12, 24, Closed, T1, "EUREX-CAL-2010"),
        // 2010-12-31 - T1 - EUREX-CAL-2010 - New Year's Eve, a full trading closure; clearing stays open.
        (2010, 12, 31, Closed, T1, "EUREX-CAL-2010"),
        // 2011-04-22 - T1 - EUREX-CAL-2011 - Good Friday, trading and clearing.
        (2011, 4, 22, Closed, T1, "EUREX-CAL-2011"),
        // 2011-04-25 - T1 - EUREX-CAL-2011 - Easter Monday, trading and clearing.
        (2011, 4, 25, Closed, T1, "EUREX-CAL-2011"),
        // 2011-12-26 - T1 - EUREX-CAL-2011 - Boxing Day, trading and clearing.
        (2011, 12, 26, Closed, T1, "EUREX-CAL-2011"),
        // 2012-04-06 - T1 - EUREX-CAL-2012 - Good Friday, trading and clearing.
        (2012, 4, 6, Closed, T1, "EUREX-CAL-2012"),
        // 2012-04-09 - T1 - EUREX-CAL-2012 - Easter Monday, trading and clearing.
        (2012, 4, 9, Closed, T1, "EUREX-CAL-2012"),
        // 2012-05-01 - T1 - EUREX-CAL-2012 - Labour Day, trading and clearing.
        (2012, 5, 1, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-24 - T1 - EUREX-CAL-2012 - Christmas Eve, a full trading closure; clearing stays open.
        (2012, 12, 24, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-25 - T1 - EUREX-CAL-2012 - Christmas Day, trading and clearing.
        (2012, 12, 25, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-26 - T1 - EUREX-CAL-2012 - Boxing Day, trading and clearing.
        (2012, 12, 26, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-31 - T1 - EUREX-CAL-2012 - New Year's Eve, a full trading closure; clearing stays open.
        (2012, 12, 31, Closed, T1, "EUREX-CAL-2012"),
        // 2013-01-01 - T1 - EUREX-CAL-2013 - New Year's Day, trading and clearing.
        (2013, 1, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-03-29 - T1 - EUREX-CAL-2013 - Good Friday, trading and clearing.
        (2013, 3, 29, Closed, T1, "EUREX-CAL-2013"),
        // 2013-04-01 - T1 - EUREX-CAL-2013 - Easter Monday, trading and clearing.
        (2013, 4, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-05-01 - T1 - EUREX-CAL-2013 - Labour Day, trading and clearing.
        (2013, 5, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-24 - T1 - EUREX-CAL-2013 - Christmas Eve, a full trading closure; clearing stays open.
        (2013, 12, 24, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-25 - T1 - EUREX-CAL-2013 - Christmas Day, trading and clearing.
        (2013, 12, 25, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-26 - T1 - EUREX-CAL-2013 - Boxing Day, trading and clearing.
        (2013, 12, 26, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-31 - T1 - EUREX-CAL-2013 - New Year's Eve, a full trading closure; clearing stays open.
        (2013, 12, 31, Closed, T1, "EUREX-CAL-2013"),
        // 2014-01-01 - T1 - EUREX-CAL-2014 - New Year's Day, trading and clearing.
        (2014, 1, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-04-18 - T1 - EUREX-CAL-2014 - Good Friday, trading and clearing.
        (2014, 4, 18, Closed, T1, "EUREX-CAL-2014"),
        // 2014-04-21 - T1 - EUREX-CAL-2014 - Easter Monday, trading and clearing.
        (2014, 4, 21, Closed, T1, "EUREX-CAL-2014"),
        // 2014-05-01 - T1 - EUREX-CAL-2014 - Labour Day, trading and clearing.
        (2014, 5, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-24 - T1 - EUREX-CAL-2014 - Christmas Eve, a full trading closure; clearing stays open.
        // 2014-10-03 - T1 - EUREX-CAL-2014 - German Unity Day: the edition closes German equity and
        // equity-index derivatives and the Xetra-based ETF/ETC derivatives, so FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 3 October`.
        (2014, 10, 3, Closed, T1, "EUREX-CAL-2014"),

        (2014, 12, 24, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-25 - T1 - EUREX-CAL-2014 - Christmas Day, trading and clearing.
        (2014, 12, 25, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-26 - T1 - EUREX-CAL-2014 - Boxing Day, trading and clearing.
        (2014, 12, 26, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-31 - T1 - EUREX-CAL-2014 - New Year's Eve, a full trading closure; clearing stays open.
        (2014, 12, 31, Closed, T1, "EUREX-CAL-2014"),
        // 2015-01-01 - T1 - EUREX-CAL-2015 - New Year's Day, trading and clearing.
        (2015, 1, 1, Closed, T1, "EUREX-CAL-2015"),
        // 2015-04-03 - T1 - EUREX-CAL-2015 - Good Friday, trading and clearing.
        (2015, 4, 3, Closed, T1, "EUREX-CAL-2015"),
        // 2015-04-06 - T1 - EUREX-CAL-2015 - Easter Monday, trading and clearing.
        (2015, 4, 6, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-01 - T1 - EUREX-CAL-2015 - Labour Day, trading and clearing.
        (2015, 5, 1, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-25 - T1 - EUREX-CAL-2015 - Whit Monday, a full trading closure; clearing stays open.
        (2015, 5, 25, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-24 - T1 - EUREX-CAL-2015 - Christmas Eve, a full trading closure; clearing stays open.
        (2015, 12, 24, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-25 - T1 - EUREX-CAL-2015 - Christmas Day, trading and clearing.
        (2015, 12, 25, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-26 - T1 - EUREX-CAL-2015 - Boxing Day, trading and clearing.
        (2015, 12, 26, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-31 - T1 - EUREX-CAL-2015 - New Year's Eve, a full trading closure; clearing stays open.
        (2015, 12, 31, Closed, T1, "EUREX-CAL-2015"),
        // 2016-01-01 - T1 - EUREX-CAL-2016 - New Year's Day, trading and clearing.
        (2016, 1, 1, Closed, T1, "EUREX-CAL-2016"),
        // 2016-03-25 - T1 - EUREX-CAL-2016 - Good Friday, trading and clearing.
        (2016, 3, 25, Closed, T1, "EUREX-CAL-2016"),
        // 2016-03-28 - T1 - EUREX-CAL-2016 - Easter Monday, trading and clearing.
        (2016, 3, 28, Closed, T1, "EUREX-CAL-2016"),
        // 2016-12-26 - T1 - EUREX-CAL-2016 - Boxing Day, trading and clearing.
        // 2016-05-16 - T1 - EUREX-CAL-2016 - Whit Monday, a German-scope closure: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 16 May, 3 October`.
        (2016, 5, 16, Closed, T1, "EUREX-CAL-2016"),

        // 2016-10-03 - T1 - EUREX-CAL-2016 - German Unity Day: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 16 May, 3 October`.
        (2016, 10, 3, Closed, T1, "EUREX-CAL-2016"),

        (2016, 12, 26, Closed, T1, "EUREX-CAL-2016"),
        // 2017-04-14 - T1 - EUREX-CAL-2017 - Good Friday, trading and clearing.
        (2017, 4, 14, Closed, T1, "EUREX-CAL-2017"),
        // 2017-04-17 - T1 - EUREX-CAL-2017 - Easter Monday, trading and clearing.
        (2017, 4, 17, Closed, T1, "EUREX-CAL-2017"),
        // 2017-05-01 - T1 - EUREX-CAL-2017 - Labour Day, trading and clearing.
        (2017, 5, 1, Closed, T1, "EUREX-CAL-2017"),
        // 2017-12-25 - T1 - EUREX-CAL-2017 - Christmas Day, trading and clearing.
        // 2017-06-05 - T1 - EUREX-CAL-2017 - Whit Monday, a German-scope closure: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 5 June, 3 October, 31 October`.
        (2017, 6, 5, Closed, T1, "EUREX-CAL-2017"),

        // 2017-10-03 - T1 - EUREX-CAL-2017 - German Unity Day: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 5 June, 3 October, 31 October`.
        (2017, 10, 3, Closed, T1, "EUREX-CAL-2017"),

        // 2017-10-31 - T1 - EUREX-CAL-2017 - Reformation Day, the one-off 500th-anniversary German
        // holiday: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 5 June, 3 October, 31 October`.
        (2017, 10, 31, Closed, T1, "EUREX-CAL-2017"),

        (2017, 12, 25, Closed, T1, "EUREX-CAL-2017"),
        // 2017-12-26 - T1 - EUREX-CAL-2017 - Boxing Day, trading and clearing.
        (2017, 12, 26, Closed, T1, "EUREX-CAL-2017"),
        // 2018-01-01 - T1 - EUREX-CAL-2018 - New Year's Day, trading and clearing.
        (2018, 1, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-03-30 - T1 - EUREX-CAL-2018 - Good Friday, trading and clearing.
        (2018, 3, 30, Closed, T1, "EUREX-CAL-2018"),
        // 2018-04-02 - T1 - EUREX-CAL-2018 - Easter Monday, trading and clearing.
        (2018, 4, 2, Closed, T1, "EUREX-CAL-2018"),
        // 2018-05-01 - T1 - EUREX-CAL-2018 - Labour Day, trading and clearing.
        (2018, 5, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-24 - T1 - EUREX-CAL-2018 - Christmas Eve, a full trading closure; clearing stays open.
        // 2018-05-21 - T1 - EUREX-CAL-2018 - Whit Monday, a German-scope closure: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 21 May, 3 October`.
        (2018, 5, 21, Closed, T1, "EUREX-CAL-2018"),

        // 2018-10-03 - T1 - EUREX-CAL-2018 - German Unity Day: FDAX and FDXM close.
        // Instant as printed: `Eurex is closed for trading and exercise in German equity and equity
        // index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings:
        // 21 May, 3 October`.
        (2018, 10, 3, Closed, T1, "EUREX-CAL-2018"),

        (2018, 12, 24, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-25 - T1 - EUREX-CAL-2018 - Christmas Day, trading and clearing.
        (2018, 12, 25, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-26 - T1 - EUREX-CAL-2018 - Boxing Day, trading and clearing.
        (2018, 12, 26, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-31 - T1 - EUREX-CAL-2018 - New Year's Eve, a full trading closure; clearing stays open.
        (2018, 12, 31, Closed, T1, "EUREX-CAL-2018"),
        // 2019-01-01 - T1 - EUREX-CAL-2019 - New Year's Day, trading and clearing.
        (2019, 1, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-04-19 - T1 - EUREX-CAL-2019 - Good Friday, trading and clearing.
        (2019, 4, 19, Closed, T1, "EUREX-CAL-2019"),
        // 2019-04-22 - T1 - EUREX-CAL-2019 - Easter Monday, trading and clearing.
        (2019, 4, 22, Closed, T1, "EUREX-CAL-2019"),
        // 2019-05-01 - T1 - EUREX-CAL-2019 - Labour Day, trading and clearing.
        (2019, 5, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-24 - T1 - EUREX-CAL-2019 - Christmas Eve, a full trading closure; clearing stays open.
        (2019, 12, 24, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-25 - T1 - EUREX-CAL-2019 - Christmas Day, trading and clearing.
        (2019, 12, 25, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-26 - T1 - EUREX-CAL-2019 - Boxing Day, trading and clearing.
        (2019, 12, 26, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-31 - T1 - EUREX-CAL-2019 - New Year's Eve, a full trading closure; clearing stays open.
        (2019, 12, 31, Closed, T1, "EUREX-CAL-2019"),
        // 2020-01-01 - T1 - EUREX-CAL-2020 - New Year's Day, trading and clearing.
        (2020, 1, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-04-10 - T1 - EUREX-CAL-2020 - Good Friday, trading and clearing.
        (2020, 4, 10, Closed, T1, "EUREX-CAL-2020"),
        // 2020-04-13 - T1 - EUREX-CAL-2020 - Easter Monday, trading and clearing.
        (2020, 4, 13, Closed, T1, "EUREX-CAL-2020"),
        // 2020-05-01 - T1 - EUREX-CAL-2020 - Labour Day, trading and clearing.
        (2020, 5, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-24 - T1 - EUREX-CAL-2020 - Christmas Eve, a full trading closure; clearing stays open.
        (2020, 12, 24, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-25 - T1 - EUREX-CAL-2020 - Christmas Day, trading and clearing.
        (2020, 12, 25, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-31 - T1 - EUREX-CAL-2020 - New Year's Eve, a full trading closure; clearing stays open.
        (2020, 12, 31, Closed, T1, "EUREX-CAL-2020"),
        // 2021-01-01 - T1 - EUREX-CAL-2021 - New Year's Day, trading and clearing.
        (2021, 1, 1, Closed, T1, "EUREX-CAL-2021"),
        // 2021-04-02 - T1 - EUREX-CAL-2021 - Good Friday, trading and clearing.
        (2021, 4, 2, Closed, T1, "EUREX-CAL-2021"),
        // 2021-04-05 - T1 - EUREX-CAL-2021 - Easter Monday, trading and clearing.
        (2021, 4, 5, Closed, T1, "EUREX-CAL-2021"),
        // 2021-12-24 - T1 - EUREX-CAL-2021 - Christmas Eve, a full trading closure; clearing stays open.
        (2021, 12, 24, Closed, T1, "EUREX-CAL-2021"),
        // 2021-12-31 - T1 - EUREX-CAL-2021 - New Year's Eve, a full trading closure; clearing stays open.
        (2021, 12, 31, Closed, T1, "EUREX-CAL-2021"),
        // 2022-04-15 - T1 - EUREX-CAL-2022 - Good Friday, trading and clearing.
        (2022, 4, 15, Closed, T1, "EUREX-CAL-2022"),
        // 2022-04-18 - T1 - EUREX-CAL-2022 - Easter Monday, trading and clearing.
        (2022, 4, 18, Closed, T1, "EUREX-CAL-2022"),
        // 2022-12-26 - T1 - EUREX-CAL-2022 - Boxing Day, trading and clearing.
        (2022, 12, 26, Closed, T1, "EUREX-CAL-2022"),
        // 2023-04-07 - T1 - EUREX-CAL-2023 - Good Friday, trading and clearing.
        (2023, 4, 7, Closed, T1, "EUREX-CAL-2023"),
        // 2023-04-10 - T1 - EUREX-CAL-2023 - Easter Monday, trading and clearing.
        (2023, 4, 10, Closed, T1, "EUREX-CAL-2023"),
        // 2023-05-01 - T1 - EUREX-CAL-2023 - Labour Day, trading and clearing.
        (2023, 5, 1, Closed, T1, "EUREX-CAL-2023"),
        // 2023-12-25 - T1 - EUREX-CAL-2023 - Christmas Day, trading and clearing.
        (2023, 12, 25, Closed, T1, "EUREX-CAL-2023"),
        // 2023-12-26 - T1 - EUREX-CAL-2023 - Boxing Day, trading and clearing.
        (2023, 12, 26, Closed, T1, "EUREX-CAL-2023"),
        // 2024-01-01 - T1 - EUREX-CAL-2024 - New Year's Day, trading and clearing.
        (2024, 1, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-03-29 - T1 - EUREX-CAL-2024 - Good Friday, trading and clearing.
        (2024, 3, 29, Closed, T1, "EUREX-CAL-2024"),
        // 2024-04-01 - T1 - EUREX-CAL-2024 - Easter Monday, trading and clearing.
        (2024, 4, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-05-01 - T1 - EUREX-CAL-2024 - Labour Day, trading and clearing.
        (2024, 5, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-24 - T1 - EUREX-CAL-2024 - Christmas Eve, a full trading closure; clearing stays open.
        (2024, 12, 24, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-25 - T1 - EUREX-CAL-2024 - Christmas Day, trading and clearing.
        (2024, 12, 25, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-26 - T1 - EUREX-CAL-2024 - Boxing Day, trading and clearing.
        (2024, 12, 26, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-31 - T1 - EUREX-CAL-2024 - New Year's Eve, a full trading closure; clearing stays open.
        (2024, 12, 31, Closed, T1, "EUREX-CAL-2024"),

        // 2025-01-01 - T1 - EUREX-HOLREG-2025 - New Year's Day, trading and clearing.
        (2025, 1, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-18 - T1 - EUREX-HOLREG-2025 - Good Friday, trading and clearing.
        (2025, 4, 18, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-21 - T1 - EUREX-HOLREG-2025 - Easter Monday, trading and clearing.
        (2025, 4, 21, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-05-01 - T1 - EUREX-HOLREG-2025 - Labour Day, trading and clearing.
        (2025, 5, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-24 - T1 - EUREX-HOLREG-2025 - Christmas Eve, a full trading closure.
        (2025, 12, 24, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-25 - T1 - EUREX-HOLREG-2025 - Christmas Day, trading and clearing.
        (2025, 12, 25, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-26 - T1 - EUREX-HOLREG-2025 - Boxing Day, trading and clearing.
        (2025, 12, 26, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-31 - T1 - EUREX-HOLREG-2025 - New Year's Eve, a full trading closure.
        (2025, 12, 31, Closed, T1, "EUREX-HOLREG-2025"),
        // 2026-01-01 - T1 - EUREX-HOLREG-2026 - New Year's Day, trading and clearing.
        (2026, 1, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-03 - T1 - EUREX-HOLREG-2026 - Good Friday, trading and clearing.
        (2026, 4, 3, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-06 - T1 - EUREX-HOLREG-2026 - Easter Monday, trading and clearing.
        (2026, 4, 6, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-05-01 - T1 - EUREX-HOLREG-2026 - Labour Day, trading and clearing.
        (2026, 5, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-24 - T1 - EUREX-HOLREG-2026 - Christmas Eve, a full trading closure.
        (2026, 12, 24, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-25 - T1 - EUREX-HOLREG-2026 - Christmas Day, trading and clearing.
        (2026, 12, 25, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-31 - T1 - EUREX-HOLREG-2026 - New Year's Eve, a full trading closure.
        (2026, 12, 31, Closed, T1, "EUREX-HOLREG-2026"),
    ],
};

/// Eurex's fixed-income family rows: the all-derivatives closures alone, and
/// the window they were audited over.
///
/// Serves the `eurex_fixed_income` key. The operator's German-scope closure
/// notes name German equity and equity-index derivatives and the Xetra-based
/// ETF/ETC derivatives and never fixed income — the panel prints fixed income
/// explicitly when a closure reaches it, as the Swiss line does — so FGBL,
/// FGBM, FGBS and FGBX keep trading on every German-scope date (2014-10-03,
/// 2016-05-16, 2016-10-03, 2017-06-05, 2017-10-03, 2017-10-31, 2018-05-21,
/// 2018-10-03) and this table carries none of those rows. The 2019-2021
/// editions' own parenthetical — trading in German equity index futures takes
/// place — is the same scope discipline in the operator's words.
// Evidence: docs/evidence/eurex_fixed_income.md
pub(crate) static FIXED_INCOME: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - EUREX-CAL-2010 - New Year's Day, trading and clearing.
        (2010, 1, 1, Closed, T1, "EUREX-CAL-2010"),
        // 2010-04-02 - T1 - EUREX-CAL-2010 - Good Friday, trading and clearing.
        (2010, 4, 2, Closed, T1, "EUREX-CAL-2010"),
        // 2010-04-05 - T1 - EUREX-CAL-2010 - Easter Monday, trading and clearing.
        (2010, 4, 5, Closed, T1, "EUREX-CAL-2010"),
        // 2010-12-24 - T1 - EUREX-CAL-2010 - Christmas Eve, a full trading closure; clearing stays open.
        (2010, 12, 24, Closed, T1, "EUREX-CAL-2010"),
        // 2010-12-31 - T1 - EUREX-CAL-2010 - New Year's Eve, a full trading closure; clearing stays open.
        (2010, 12, 31, Closed, T1, "EUREX-CAL-2010"),
        // 2011-04-22 - T1 - EUREX-CAL-2011 - Good Friday, trading and clearing.
        (2011, 4, 22, Closed, T1, "EUREX-CAL-2011"),
        // 2011-04-25 - T1 - EUREX-CAL-2011 - Easter Monday, trading and clearing.
        (2011, 4, 25, Closed, T1, "EUREX-CAL-2011"),
        // 2011-12-26 - T1 - EUREX-CAL-2011 - Boxing Day, trading and clearing.
        (2011, 12, 26, Closed, T1, "EUREX-CAL-2011"),
        // 2012-04-06 - T1 - EUREX-CAL-2012 - Good Friday, trading and clearing.
        (2012, 4, 6, Closed, T1, "EUREX-CAL-2012"),
        // 2012-04-09 - T1 - EUREX-CAL-2012 - Easter Monday, trading and clearing.
        (2012, 4, 9, Closed, T1, "EUREX-CAL-2012"),
        // 2012-05-01 - T1 - EUREX-CAL-2012 - Labour Day, trading and clearing.
        (2012, 5, 1, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-24 - T1 - EUREX-CAL-2012 - Christmas Eve, a full trading closure; clearing stays open.
        (2012, 12, 24, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-25 - T1 - EUREX-CAL-2012 - Christmas Day, trading and clearing.
        (2012, 12, 25, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-26 - T1 - EUREX-CAL-2012 - Boxing Day, trading and clearing.
        (2012, 12, 26, Closed, T1, "EUREX-CAL-2012"),
        // 2012-12-31 - T1 - EUREX-CAL-2012 - New Year's Eve, a full trading closure; clearing stays open.
        (2012, 12, 31, Closed, T1, "EUREX-CAL-2012"),
        // 2013-01-01 - T1 - EUREX-CAL-2013 - New Year's Day, trading and clearing.
        (2013, 1, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-03-29 - T1 - EUREX-CAL-2013 - Good Friday, trading and clearing.
        (2013, 3, 29, Closed, T1, "EUREX-CAL-2013"),
        // 2013-04-01 - T1 - EUREX-CAL-2013 - Easter Monday, trading and clearing.
        (2013, 4, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-05-01 - T1 - EUREX-CAL-2013 - Labour Day, trading and clearing.
        (2013, 5, 1, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-24 - T1 - EUREX-CAL-2013 - Christmas Eve, a full trading closure; clearing stays open.
        (2013, 12, 24, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-25 - T1 - EUREX-CAL-2013 - Christmas Day, trading and clearing.
        (2013, 12, 25, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-26 - T1 - EUREX-CAL-2013 - Boxing Day, trading and clearing.
        (2013, 12, 26, Closed, T1, "EUREX-CAL-2013"),
        // 2013-12-31 - T1 - EUREX-CAL-2013 - New Year's Eve, a full trading closure; clearing stays open.
        (2013, 12, 31, Closed, T1, "EUREX-CAL-2013"),
        // 2014-01-01 - T1 - EUREX-CAL-2014 - New Year's Day, trading and clearing.
        (2014, 1, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-04-18 - T1 - EUREX-CAL-2014 - Good Friday, trading and clearing.
        (2014, 4, 18, Closed, T1, "EUREX-CAL-2014"),
        // 2014-04-21 - T1 - EUREX-CAL-2014 - Easter Monday, trading and clearing.
        (2014, 4, 21, Closed, T1, "EUREX-CAL-2014"),
        // 2014-05-01 - T1 - EUREX-CAL-2014 - Labour Day, trading and clearing.
        (2014, 5, 1, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-24 - T1 - EUREX-CAL-2014 - Christmas Eve, a full trading closure; clearing stays open.
        (2014, 12, 24, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-25 - T1 - EUREX-CAL-2014 - Christmas Day, trading and clearing.
        (2014, 12, 25, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-26 - T1 - EUREX-CAL-2014 - Boxing Day, trading and clearing.
        (2014, 12, 26, Closed, T1, "EUREX-CAL-2014"),
        // 2014-12-31 - T1 - EUREX-CAL-2014 - New Year's Eve, a full trading closure; clearing stays open.
        (2014, 12, 31, Closed, T1, "EUREX-CAL-2014"),
        // 2015-01-01 - T1 - EUREX-CAL-2015 - New Year's Day, trading and clearing.
        (2015, 1, 1, Closed, T1, "EUREX-CAL-2015"),
        // 2015-04-03 - T1 - EUREX-CAL-2015 - Good Friday, trading and clearing.
        (2015, 4, 3, Closed, T1, "EUREX-CAL-2015"),
        // 2015-04-06 - T1 - EUREX-CAL-2015 - Easter Monday, trading and clearing.
        (2015, 4, 6, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-01 - T1 - EUREX-CAL-2015 - Labour Day, trading and clearing.
        (2015, 5, 1, Closed, T1, "EUREX-CAL-2015"),
        // 2015-05-25 - T1 - EUREX-CAL-2015 - Whit Monday, a full trading closure; clearing stays open.
        (2015, 5, 25, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-24 - T1 - EUREX-CAL-2015 - Christmas Eve, a full trading closure; clearing stays open.
        (2015, 12, 24, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-25 - T1 - EUREX-CAL-2015 - Christmas Day, trading and clearing.
        (2015, 12, 25, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-26 - T1 - EUREX-CAL-2015 - Boxing Day, trading and clearing.
        (2015, 12, 26, Closed, T1, "EUREX-CAL-2015"),
        // 2015-12-31 - T1 - EUREX-CAL-2015 - New Year's Eve, a full trading closure; clearing stays open.
        (2015, 12, 31, Closed, T1, "EUREX-CAL-2015"),
        // 2016-01-01 - T1 - EUREX-CAL-2016 - New Year's Day, trading and clearing.
        (2016, 1, 1, Closed, T1, "EUREX-CAL-2016"),
        // 2016-03-25 - T1 - EUREX-CAL-2016 - Good Friday, trading and clearing.
        (2016, 3, 25, Closed, T1, "EUREX-CAL-2016"),
        // 2016-03-28 - T1 - EUREX-CAL-2016 - Easter Monday, trading and clearing.
        (2016, 3, 28, Closed, T1, "EUREX-CAL-2016"),
        // 2016-12-26 - T1 - EUREX-CAL-2016 - Boxing Day, trading and clearing.
        (2016, 12, 26, Closed, T1, "EUREX-CAL-2016"),
        // 2017-04-14 - T1 - EUREX-CAL-2017 - Good Friday, trading and clearing.
        (2017, 4, 14, Closed, T1, "EUREX-CAL-2017"),
        // 2017-04-17 - T1 - EUREX-CAL-2017 - Easter Monday, trading and clearing.
        (2017, 4, 17, Closed, T1, "EUREX-CAL-2017"),
        // 2017-05-01 - T1 - EUREX-CAL-2017 - Labour Day, trading and clearing.
        (2017, 5, 1, Closed, T1, "EUREX-CAL-2017"),
        // 2017-12-25 - T1 - EUREX-CAL-2017 - Christmas Day, trading and clearing.
        (2017, 12, 25, Closed, T1, "EUREX-CAL-2017"),
        // 2017-12-26 - T1 - EUREX-CAL-2017 - Boxing Day, trading and clearing.
        (2017, 12, 26, Closed, T1, "EUREX-CAL-2017"),
        // 2018-01-01 - T1 - EUREX-CAL-2018 - New Year's Day, trading and clearing.
        (2018, 1, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-03-30 - T1 - EUREX-CAL-2018 - Good Friday, trading and clearing.
        (2018, 3, 30, Closed, T1, "EUREX-CAL-2018"),
        // 2018-04-02 - T1 - EUREX-CAL-2018 - Easter Monday, trading and clearing.
        (2018, 4, 2, Closed, T1, "EUREX-CAL-2018"),
        // 2018-05-01 - T1 - EUREX-CAL-2018 - Labour Day, trading and clearing.
        (2018, 5, 1, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-24 - T1 - EUREX-CAL-2018 - Christmas Eve, a full trading closure; clearing stays open.
        (2018, 12, 24, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-25 - T1 - EUREX-CAL-2018 - Christmas Day, trading and clearing.
        (2018, 12, 25, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-26 - T1 - EUREX-CAL-2018 - Boxing Day, trading and clearing.
        (2018, 12, 26, Closed, T1, "EUREX-CAL-2018"),
        // 2018-12-31 - T1 - EUREX-CAL-2018 - New Year's Eve, a full trading closure; clearing stays open.
        (2018, 12, 31, Closed, T1, "EUREX-CAL-2018"),
        // 2019-01-01 - T1 - EUREX-CAL-2019 - New Year's Day, trading and clearing.
        (2019, 1, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-04-19 - T1 - EUREX-CAL-2019 - Good Friday, trading and clearing.
        (2019, 4, 19, Closed, T1, "EUREX-CAL-2019"),
        // 2019-04-22 - T1 - EUREX-CAL-2019 - Easter Monday, trading and clearing.
        (2019, 4, 22, Closed, T1, "EUREX-CAL-2019"),
        // 2019-05-01 - T1 - EUREX-CAL-2019 - Labour Day, trading and clearing.
        (2019, 5, 1, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-24 - T1 - EUREX-CAL-2019 - Christmas Eve, a full trading closure; clearing stays open.
        (2019, 12, 24, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-25 - T1 - EUREX-CAL-2019 - Christmas Day, trading and clearing.
        (2019, 12, 25, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-26 - T1 - EUREX-CAL-2019 - Boxing Day, trading and clearing.
        (2019, 12, 26, Closed, T1, "EUREX-CAL-2019"),
        // 2019-12-31 - T1 - EUREX-CAL-2019 - New Year's Eve, a full trading closure; clearing stays open.
        (2019, 12, 31, Closed, T1, "EUREX-CAL-2019"),
        // 2020-01-01 - T1 - EUREX-CAL-2020 - New Year's Day, trading and clearing.
        (2020, 1, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-04-10 - T1 - EUREX-CAL-2020 - Good Friday, trading and clearing.
        (2020, 4, 10, Closed, T1, "EUREX-CAL-2020"),
        // 2020-04-13 - T1 - EUREX-CAL-2020 - Easter Monday, trading and clearing.
        (2020, 4, 13, Closed, T1, "EUREX-CAL-2020"),
        // 2020-05-01 - T1 - EUREX-CAL-2020 - Labour Day, trading and clearing.
        (2020, 5, 1, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-24 - T1 - EUREX-CAL-2020 - Christmas Eve, a full trading closure; clearing stays open.
        (2020, 12, 24, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-25 - T1 - EUREX-CAL-2020 - Christmas Day, trading and clearing.
        (2020, 12, 25, Closed, T1, "EUREX-CAL-2020"),
        // 2020-12-31 - T1 - EUREX-CAL-2020 - New Year's Eve, a full trading closure; clearing stays open.
        (2020, 12, 31, Closed, T1, "EUREX-CAL-2020"),
        // 2021-01-01 - T1 - EUREX-CAL-2021 - New Year's Day, trading and clearing.
        (2021, 1, 1, Closed, T1, "EUREX-CAL-2021"),
        // 2021-04-02 - T1 - EUREX-CAL-2021 - Good Friday, trading and clearing.
        (2021, 4, 2, Closed, T1, "EUREX-CAL-2021"),
        // 2021-04-05 - T1 - EUREX-CAL-2021 - Easter Monday, trading and clearing.
        (2021, 4, 5, Closed, T1, "EUREX-CAL-2021"),
        // 2021-12-24 - T1 - EUREX-CAL-2021 - Christmas Eve, a full trading closure; clearing stays open.
        (2021, 12, 24, Closed, T1, "EUREX-CAL-2021"),
        // 2021-12-31 - T1 - EUREX-CAL-2021 - New Year's Eve, a full trading closure; clearing stays open.
        (2021, 12, 31, Closed, T1, "EUREX-CAL-2021"),
        // 2022-04-15 - T1 - EUREX-CAL-2022 - Good Friday, trading and clearing.
        (2022, 4, 15, Closed, T1, "EUREX-CAL-2022"),
        // 2022-04-18 - T1 - EUREX-CAL-2022 - Easter Monday, trading and clearing.
        (2022, 4, 18, Closed, T1, "EUREX-CAL-2022"),
        // 2022-12-26 - T1 - EUREX-CAL-2022 - Boxing Day, trading and clearing.
        (2022, 12, 26, Closed, T1, "EUREX-CAL-2022"),
        // 2023-04-07 - T1 - EUREX-CAL-2023 - Good Friday, trading and clearing.
        (2023, 4, 7, Closed, T1, "EUREX-CAL-2023"),
        // 2023-04-10 - T1 - EUREX-CAL-2023 - Easter Monday, trading and clearing.
        (2023, 4, 10, Closed, T1, "EUREX-CAL-2023"),
        // 2023-05-01 - T1 - EUREX-CAL-2023 - Labour Day, trading and clearing.
        (2023, 5, 1, Closed, T1, "EUREX-CAL-2023"),
        // 2023-12-25 - T1 - EUREX-CAL-2023 - Christmas Day, trading and clearing.
        (2023, 12, 25, Closed, T1, "EUREX-CAL-2023"),
        // 2023-12-26 - T1 - EUREX-CAL-2023 - Boxing Day, trading and clearing.
        (2023, 12, 26, Closed, T1, "EUREX-CAL-2023"),
        // 2024-01-01 - T1 - EUREX-CAL-2024 - New Year's Day, trading and clearing.
        (2024, 1, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-03-29 - T1 - EUREX-CAL-2024 - Good Friday, trading and clearing.
        (2024, 3, 29, Closed, T1, "EUREX-CAL-2024"),
        // 2024-04-01 - T1 - EUREX-CAL-2024 - Easter Monday, trading and clearing.
        (2024, 4, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-05-01 - T1 - EUREX-CAL-2024 - Labour Day, trading and clearing.
        (2024, 5, 1, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-24 - T1 - EUREX-CAL-2024 - Christmas Eve, a full trading closure; clearing stays open.
        (2024, 12, 24, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-25 - T1 - EUREX-CAL-2024 - Christmas Day, trading and clearing.
        (2024, 12, 25, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-26 - T1 - EUREX-CAL-2024 - Boxing Day, trading and clearing.
        (2024, 12, 26, Closed, T1, "EUREX-CAL-2024"),
        // 2024-12-31 - T1 - EUREX-CAL-2024 - New Year's Eve, a full trading closure; clearing stays open.
        (2024, 12, 31, Closed, T1, "EUREX-CAL-2024"),

        // 2025-01-01 - T1 - EUREX-HOLREG-2025 - New Year's Day, trading and clearing.
        (2025, 1, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-18 - T1 - EUREX-HOLREG-2025 - Good Friday, trading and clearing.
        (2025, 4, 18, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-21 - T1 - EUREX-HOLREG-2025 - Easter Monday, trading and clearing.
        (2025, 4, 21, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-05-01 - T1 - EUREX-HOLREG-2025 - Labour Day, trading and clearing.
        (2025, 5, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-24 - T1 - EUREX-HOLREG-2025 - Christmas Eve, a full trading closure.
        (2025, 12, 24, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-25 - T1 - EUREX-HOLREG-2025 - Christmas Day, trading and clearing.
        (2025, 12, 25, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-26 - T1 - EUREX-HOLREG-2025 - Boxing Day, trading and clearing.
        (2025, 12, 26, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-31 - T1 - EUREX-HOLREG-2025 - New Year's Eve, a full trading closure.
        (2025, 12, 31, Closed, T1, "EUREX-HOLREG-2025"),
        // 2026-01-01 - T1 - EUREX-HOLREG-2026 - New Year's Day, trading and clearing.
        (2026, 1, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-03 - T1 - EUREX-HOLREG-2026 - Good Friday, trading and clearing.
        (2026, 4, 3, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-06 - T1 - EUREX-HOLREG-2026 - Easter Monday, trading and clearing.
        (2026, 4, 6, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-05-01 - T1 - EUREX-HOLREG-2026 - Labour Day, trading and clearing.
        (2026, 5, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-24 - T1 - EUREX-HOLREG-2026 - Christmas Eve, a full trading closure.
        (2026, 12, 24, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-25 - T1 - EUREX-HOLREG-2026 - Christmas Day, trading and clearing.
        (2026, 12, 25, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-31 - T1 - EUREX-HOLREG-2026 - New Year's Eve, a full trading closure.
        (2026, 12, 31, Closed, T1, "EUREX-HOLREG-2026"),
    ],
};
