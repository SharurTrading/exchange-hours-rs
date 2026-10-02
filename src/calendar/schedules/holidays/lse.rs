// SPDX-License-Identifier: MIT-0

//! London Stock Exchange (SETS) holiday rows, 2010-2027 in one audited
//! window.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/London`. LSE runs
//! no overnight session, so an event date and its trade date are one civil day
//! and the conversion is the identity.
//!
//! The whole block is **T1**: the operator's own holiday table. It has run on
//! four generations of the operator's own channels — "The Public and Bank
//! Holidays in England & Wales" on the `.htm` Business days page of 2010-2014;
//! the same page on the operator group's own site
//! (`lseg.com/areas-expertise/our-markets/london-stock-exchange/.../business-days`,
//! to which `lseg.com/businessdays` redirects and which the operator's own
//! service announcements cite, captured continuously 2013-10..2020-07 — the
//! operator's December 2015 announcement `live001-03122015` names it as the
//! place where "Full details of London Stock Exchange trading and EUI
//! settlement days can be found"); then the server-rendered "Bank holidays and
//! their impact on our trading services" table of the 2020 SPA page; then the
//! same table delivered by the content API behind `londonstockexchange.com`.
//! The table is **rolling** — it lists business days from the present forward
//! to the next New Year — so every audited year is pinned by captures taken
//! inside it or just before it; the thirty-six archived states are listed in
//! the evidence file's `### Documents` table. The 2015-2019 and January-August
//! 2020 spans that the `.htm` captures miss are printed by the `lseg.com`
//! channel's captures (found 2026-10-02 UTC; the closing capture of the
//! 2020-05-25 window is `20200317072237`), so coverage is one continuous
//! window.
//!
//! Half days are **early closes at the operator's own instant**: the sheet
//! states `Markets closing process commences from 12:30 London time.` for the
//! Christmas-period half days (2010-2012 in the words `all Exchange markets
//! will close from 12:30 London time onwards`), so the day's final close is
//! 12:30 and the closing-auction phase that would follow disappears rather
//! than being invented. The half days are not always 24/31 December: 2011's
//! fell on Friday 23 and Friday 30 December, 2016's on Friday 23 and Friday
//! 30, 2017's on Friday 22 and Friday 29 and 2022's on Friday 23 and Friday
//! 30 December, as printed. No other date carries an early close in any
//! audited year.
//!
//! Five 2025 dates — 2025-04-18, 2025-04-21, 2025-05-05, 2025-05-26 and
//! 2025-08-25 — are inside the second window but **not audited**: the rolling
//! table had already moved past them when the first 2025-era capture was taken
//! (2025-12-18 starts at 2025-12-24) and the Wayback index holds no capture of
//! the gap, so the rows ship as `Unsourced` and claim no closure.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// CFE-style alias: LSE's stated half-day close, 12:30 London time.
///
/// The sheet's own words are `Markets closing process commences from 12:30
/// London time.` — the same boundary its normal week prints as 16:30 for the
/// closing-auction start, so the scalar is the day's final close.
const HALF_DAY_12_30: u32 = 12 * 3_600 + 30 * 60;

/// LSE's built-in holiday rows and the windows they were audited over.
///
/// Every row is one line of the operator's holiday table at the capture its
/// document id names (the earliest archived state that prints the row) or in
/// the live retrieval (2026-09-28). A date inside the window with no row is
/// audited normal; the five 2025 dates above carry `Unsourced` rows because no
/// surviving artifact states them.
// Evidence: docs/evidence/lse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - LSE-BUSDAYS-2010-01-30 - New Year:
        // `Friday 1 January`.
        (2010, 1, 1, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-04-02 - T1 - LSE-BUSDAYS-2010-01-30 - Easter (Good Friday):
        // `Friday 2 April`.
        (2010, 4, 2, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-04-05 - T1 - LSE-BUSDAYS-2010-01-30 - Easter (Easter Monday):
        // `Monday 5 April`.
        (2010, 4, 5, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-05-03 - T1 - LSE-BUSDAYS-2010-01-30 - May: `Monday 3 May*`.
        (2010, 5, 3, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-05-31 - T1 - LSE-BUSDAYS-2010-01-30 - Spring:
        // `Monday 31 May*`.
        (2010, 5, 31, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-08-30 - T1 - LSE-BUSDAYS-2010-01-30 - Summer:
        // `Monday 30 August*`.
        (2010, 8, 30, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-12-24 - T1 - LSE-BUSDAYS-2010-01-30 - the page states all
        // Exchange markets close from 12:30 London time onwards.
        (2010, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-12-27 - T1 - LSE-BUSDAYS-2010-01-30 - Christmas:
        // `Monday 27 December*`.
        (2010, 12, 27, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-12-28 - T1 - LSE-BUSDAYS-2010-01-30 - Christmas:
        // `Tuesday 28 December*`.
        (2010, 12, 28, Closed, T1, "LSE-BUSDAYS-2010-01-30"),
        // 2010-12-31 - T1 - LSE-BUSDAYS-2010-01-30 - the page states all
        // Exchange markets close from 12:30 London time onwards.
        (2010, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2010-01-30"),
        // 2011-01-03 - T1 - LSE-BUSDAYS-2010-10-07 - New Year:
        // `Monday 3 January*`.
        (2011, 1, 3, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-04-22 - T1 - LSE-BUSDAYS-2010-10-07 - Easter (Good Friday):
        // `Friday 22 April`.
        (2011, 4, 22, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-04-25 - T1 - LSE-BUSDAYS-2010-10-07 - Easter (Easter Monday):
        // `Monday 25 April`.
        (2011, 4, 25, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-04-29 - T1 - LSE-BUSDAYS-2010-12-03 - Royal Wedding:
        // `Friday 29 April*`; first stated by the December capture (the
        // wedding was announced 2010-11-16).
        (2011, 4, 29, Closed, T1, "LSE-BUSDAYS-2010-12-03"),
        // 2011-05-02 - T1 - LSE-BUSDAYS-2010-10-07 - May: `Monday 2 May*`.
        (2011, 5, 2, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-05-30 - T1 - LSE-BUSDAYS-2010-10-07 - Spring:
        // `Monday 30 May*`.
        (2011, 5, 30, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-08-29 - T1 - LSE-BUSDAYS-2010-10-07 - Summer:
        // `Monday 29 August*`.
        (2011, 8, 29, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-12-23 - T1 - LSE-BUSDAYS-2010-10-07 - the page states all
        // Exchange markets close from 12:30 London time onwards on Friday 23
        // December.
        (2011, 12, 23, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-12-26 - T1 - LSE-BUSDAYS-2010-10-07 - Christmas:
        // `Monday 26 December`.
        (2011, 12, 26, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-12-27 - T1 - LSE-BUSDAYS-2010-10-07 - Christmas:
        // `Tuesday 27 December*`.
        (2011, 12, 27, Closed, T1, "LSE-BUSDAYS-2010-10-07"),
        // 2011-12-30 - T1 - LSE-BUSDAYS-2010-10-07 - the page states all
        // Exchange markets close from 12:30 London time onwards on Friday 30
        // December.
        (2011, 12, 30, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2010-10-07"),
        // 2012-01-02 - T1 - LSE-BUSDAYS-2011-12-27 - New Year:
        // `Monday 2 January*`.
        (2012, 1, 2, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-04-06 - T1 - LSE-BUSDAYS-2011-12-27 - Good Friday:
        // `Friday 6 April`.
        (2012, 4, 6, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-04-09 - T1 - LSE-BUSDAYS-2011-12-27 - Easter Monday:
        // `Monday 9 April`.
        (2012, 4, 9, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-05-07 - T1 - LSE-BUSDAYS-2011-12-27 - Early May:
        // `Monday 7 May*`.
        (2012, 5, 7, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-06-04 - T1 - LSE-BUSDAYS-2011-12-27 - Spring:
        // `Monday 4 June*`.
        (2012, 6, 4, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-06-05 - T1 - LSE-BUSDAYS-2011-12-27 - Queen's Diamond Jubilee:
        // `Tuesday 5 June*`.
        (2012, 6, 5, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-08-27 - T1 - LSE-BUSDAYS-2011-12-27 - Summer:
        // `Monday 27 August*`.
        (2012, 8, 27, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-12-24 - T1 - LSE-BUSDAYS-2011-12-27 - the page states all
        // Exchange markets close from 12.30 London time onwards on Monday 24
        // December.
        (2012, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-12-25 - T1 - LSE-BUSDAYS-2011-12-27 - Christmas Day:
        // `Tuesday 25 December`.
        (2012, 12, 25, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-12-26 - T1 - LSE-BUSDAYS-2011-12-27 - Boxing Day:
        // `Wednesday 26 December*`.
        (2012, 12, 26, Closed, T1, "LSE-BUSDAYS-2011-12-27"),
        // 2012-12-31 - T1 - LSE-BUSDAYS-2011-12-27 - the page states all
        // Exchange markets close from 12.30 London time onwards on Monday 31
        // December.
        (2012, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2011-12-27"),
        // 2013-01-01 - T1 - LSE-BUSDAYS-2013-01-01 - New Year's Day:
        // `NON-trading day & NON-settlement day in EUI`.
        (2013, 1, 1, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-03-29 - T1 - LSE-BUSDAYS-2013-01-01 - Good Friday.
        (2013, 3, 29, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-04-01 - T1 - LSE-BUSDAYS-2013-01-01 - Easter Monday.
        (2013, 4, 1, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-05-06 - T1 - LSE-BUSDAYS-2013-01-01 - Early May Bank Holiday.
        (2013, 5, 6, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-05-27 - T1 - LSE-BUSDAYS-2013-01-01 - Spring Bank Holiday.
        (2013, 5, 27, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-08-26 - T1 - LSE-BUSDAYS-2013-01-01 - Summer Bank Holiday.
        (2013, 8, 26, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-12-24 - T1 - LSE-BUSDAYS-2013-01-01 - Christmas Eve:
        // closing process from 12:30 London time.
        (2013, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-12-25 - T1 - LSE-BUSDAYS-2013-01-01 - Christmas Day.
        (2013, 12, 25, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-12-26 - T1 - LSE-BUSDAYS-2013-01-01 - Boxing Day.
        (2013, 12, 26, Closed, T1, "LSE-BUSDAYS-2013-01-01"),
        // 2013-12-31 - T1 - LSE-BUSDAYS-2013-01-01 - New Years Eve: closing
        // process from 12:30 London time.
        (2013, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2013-01-01"),
        // 2014-01-01 - T1 - LSE-BUSDAYS-2014-01-15 - New Year's Day.
        (2014, 1, 1, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-04-18 - T1 - LSE-BUSDAYS-2014-01-15 - Good Friday.
        (2014, 4, 18, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-04-21 - T1 - LSE-BUSDAYS-2014-01-15 - Easter Monday.
        (2014, 4, 21, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-05-05 - T1 - LSE-BUSDAYS-2014-01-15 - Early May Bank Holiday.
        (2014, 5, 5, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-05-26 - T1 - LSE-BUSDAYS-2014-01-15 - Spring Bank Holiday.
        (2014, 5, 26, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-08-25 - T1 - LSE-BUSDAYS-2014-01-15 - Summer Bank Holiday.
        (2014, 8, 25, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-12-24 - T1 - LSE-BUSDAYS-2014-01-15 - Christmas Eve: closing
        // process from 12:30 London time.
        (2014, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-12-25 - T1 - LSE-BUSDAYS-2014-01-15 - Christmas Day.
        (2014, 12, 25, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-12-26 - T1 - LSE-BUSDAYS-2014-01-15 - Boxing Day.
        (2014, 12, 26, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2014-12-31 - T1 - LSE-BUSDAYS-2014-01-15 - New Year's: closing
        // process from 12:30 London time.
        (2014, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2014-01-15"),
        // 2015-01-01 - T1 - LSE-BUSDAYS-2014-01-15 - New Year's Day.
        (2015, 1, 1, Closed, T1, "LSE-BUSDAYS-2014-01-15"),
        // 2015-04-03 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Good Friday:
        // `NON-trading day & NON-settlement day in EUI`.
        (2015, 4, 3, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-04-06 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Easter Monday.
        (2015, 4, 6, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-05-04 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Early May Bank
        // Holiday.
        (2015, 5, 4, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-05-25 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Spring Bank Holiday.
        (2015, 5, 25, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-08-31 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Summer Bank holiday.
        (2015, 8, 31, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-12-24 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Christmas Eve:
        // `Markets closing process commences from 12:30 London time.`
        (2015, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-12-25 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Christmas Day.
        (2015, 12, 25, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-12-28 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - Boxing Day (holiday):
        // the Saturday substitute.
        (2015, 12, 28, Closed, T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2015-12-31 - T1 - LSE-LSEGBUSDAYS-2015-01-03 - New Year's Eve:
        // `Markets closing process commences from 12:30 London time.`
        (2015, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2015-01-03"),
        // 2016-01-01 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - New Year's Day.
        (2016, 1, 1, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-03-25 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Good Friday.
        (2016, 3, 25, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-03-28 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Easter Monday.
        (2016, 3, 28, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-05-02 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Early May Bank
        // Holiday.
        (2016, 5, 2, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-05-30 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Spring Bank Holiday.
        (2016, 5, 30, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-08-29 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Summer Bank Holiday.
        (2016, 8, 29, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-12-23 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Christmas: the half
        // day before the weekend Christmas pair, closing process from 12:30
        // London time.
        (2016, 12, 23, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-12-26 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Boxing Day.
        (2016, 12, 26, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-12-27 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - Christmas Holiday:
        // the Christmas-Day substitute.
        (2016, 12, 27, Closed, T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2016-12-30 - T1 - LSE-LSEGBUSDAYS-2015-12-21 - New Year: closing
        // process from 12:30 London time.
        (2016, 12, 30, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2015-12-21"),
        // 2017-01-02 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - New Year Holiday:
        // the New-Year-Day substitute.
        (2017, 1, 2, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-04-14 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Good Friday.
        (2017, 4, 14, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-04-17 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Easter Monday.
        (2017, 4, 17, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-05-01 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Early May Bank
        // Holiday.
        (2017, 5, 1, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-05-29 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Spring Bank Holiday.
        (2017, 5, 29, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-08-28 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Summer Bank Holiday.
        (2017, 8, 28, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-12-22 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Christmas: closing
        // process from 12:30 London time.
        (2017, 12, 22, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-12-25 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Christmas Day.
        (2017, 12, 25, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-12-26 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - Boxing Day.
        (2017, 12, 26, Closed, T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2017-12-29 - T1 - LSE-LSEGBUSDAYS-2016-11-22 - New Year: closing
        // process from 12:30 London time.
        (2017, 12, 29, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2016-11-22"),
        // 2018-01-01 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - New Year's Day.
        (2018, 1, 1, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-03-30 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Good Friday.
        (2018, 3, 30, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-04-02 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Easter Monday.
        (2018, 4, 2, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-05-07 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Early May Bank
        // Holiday.
        (2018, 5, 7, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-05-28 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Spring Bank Holiday.
        (2018, 5, 28, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-08-27 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Summer bank Holiday.
        (2018, 8, 27, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-12-24 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Christmas Eve:
        // closing process from 12:30 London time.
        (2018, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-12-25 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Christmas Day.
        (2018, 12, 25, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-12-26 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - Boxing Day.
        (2018, 12, 26, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2018-12-31 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - New Year's Eve:
        // closing process from 12:30 London time.
        (2018, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2019-01-01 - T1 - LSE-LSEGBUSDAYS-2017-12-02 - New Year's Day;
        // first stated by the December 2017 capture.
        (2019, 1, 1, Closed, T1, "LSE-LSEGBUSDAYS-2017-12-02"),
        // 2019-04-19 - T1 - LSE-LSEGBUSDAYS-2018-10-01 - Good Friday.
        (2019, 4, 19, Closed, T1, "LSE-LSEGBUSDAYS-2018-10-01"),
        // 2019-04-22 - T1 - LSE-LSEGBUSDAYS-2018-10-01 - Easter Monday.
        (2019, 4, 22, Closed, T1, "LSE-LSEGBUSDAYS-2018-10-01"),
        // 2019-05-06 - T1 - LSE-LSEGBUSDAYS-2018-10-01 - Early May Bank
        // Holiday.
        (2019, 5, 6, Closed, T1, "LSE-LSEGBUSDAYS-2018-10-01"),
        // 2019-05-27 - T1 - LSE-LSEGBUSDAYS-2018-10-01 - Spring Bank Holiday.
        (2019, 5, 27, Closed, T1, "LSE-LSEGBUSDAYS-2018-10-01"),
        // 2019-08-26 - T1 - LSE-LSEGBUSDAYS-2019-03-26 - Summer Bank Holiday.
        (2019, 8, 26, Closed, T1, "LSE-LSEGBUSDAYS-2019-03-26"),
        // 2019-12-24 - T1 - LSE-LSEGBUSDAYS-2019-03-26 - Christmas Eve:
        // closing process from 12:30 London time.
        (2019, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2019-03-26"),
        // 2019-12-25 - T1 - LSE-LSEGBUSDAYS-2019-03-26 - Christmas Day.
        (2019, 12, 25, Closed, T1, "LSE-LSEGBUSDAYS-2019-03-26"),
        // 2019-12-26 - T1 - LSE-LSEGBUSDAYS-2019-03-26 - Boxing Day.
        (2019, 12, 26, Closed, T1, "LSE-LSEGBUSDAYS-2019-03-26"),
        // 2019-12-31 - T1 - LSE-LSEGBUSDAYS-2019-03-26 - New Year's Eve:
        // closing process from 12:30 London time.
        (2019, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2019-03-26"),
        // 2020-01-01 - T1 - LSE-LSEGBUSDAYS-2019-03-26 - New Year's Day.
        (2020, 1, 1, Closed, T1, "LSE-LSEGBUSDAYS-2019-03-26"),
        // 2020-04-10 - T1 - LSE-LSEGBUSDAYS-2019-06-14 - Good Friday.
        (2020, 4, 10, Closed, T1, "LSE-LSEGBUSDAYS-2019-06-14"),
        // 2020-04-13 - T1 - LSE-LSEGBUSDAYS-2019-06-14 - Easter Monday.
        (2020, 4, 13, Closed, T1, "LSE-LSEGBUSDAYS-2019-06-14"),
        // 2020-05-08 - T1 - LSE-LSEGBUSDAYS-2019-06-14 - Early May Bank
        // Holiday (VE Day): the Early May holiday moved from Monday 4 May for
        // the 75th anniversary, as printed.
        (2020, 5, 8, Closed, T1, "LSE-LSEGBUSDAYS-2019-06-14"),
        // 2020-05-25 - T1 - LSE-LSEGBUSDAYS-2019-06-14 - Spring Bank Holiday.
        (2020, 5, 25, Closed, T1, "LSE-LSEGBUSDAYS-2019-06-14"),
        // 2020-08-31 - T1 - LSE-LSEGBUSDAYS-2019-12-19 - Summer Bank Holiday:
        // first stated by the December 2019 capture of the lseg.com channel;
        // the 2020-07-31 SPA capture corroborates.
        (2020, 8, 31, Closed, T1, "LSE-LSEGBUSDAYS-2019-12-19"),
        // 2020-12-24 - T1 - LSE-LSEGBUSDAYS-2019-12-19 - Christmas Eve: closing
        // process from 12:30 London time.
        (2020, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2019-12-19"),
        // 2020-12-25 - T1 - LSE-LSEGBUSDAYS-2019-12-19 - Christmas Day:
        // `NON-trading day.`
        (2020, 12, 25, Closed, T1, "LSE-LSEGBUSDAYS-2019-12-19"),
        // 2020-12-28 - T1 - LSE-LSEGBUSDAYS-2019-12-19 - Boxing Day (substitute):
        // `NON-trading day.`
        (2020, 12, 28, Closed, T1, "LSE-LSEGBUSDAYS-2019-12-19"),
        // 2020-12-31 - T1 - LSE-LSEGBUSDAYS-2019-12-19 - New Year's Eve: closing
        // process from 12:30 London time.
        (2020, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-LSEGBUSDAYS-2019-12-19"),
        // 2021-01-01 - T1 - LSE-LSEGBUSDAYS-2019-12-19 - New Year's Day:
        // `NON-trading day.`
        (2021, 1, 1, Closed, T1, "LSE-LSEGBUSDAYS-2019-12-19"),
        // 2021-04-02 - T1 - LSE-BUSDAYS-2020-12-21 - Good Friday.
        (2021, 4, 2, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-04-05 - T1 - LSE-BUSDAYS-2020-12-21 - Easter Monday.
        (2021, 4, 5, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-05-03 - T1 - LSE-BUSDAYS-2020-12-21 - Early May Bank Holiday.
        (2021, 5, 3, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-05-31 - T1 - LSE-BUSDAYS-2020-12-21 - Spring Bank Holiday.
        (2021, 5, 31, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-08-30 - T1 - LSE-BUSDAYS-2020-12-21 - Summer Bank Holiday.
        (2021, 8, 30, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-12-24 - T1 - LSE-BUSDAYS-2020-12-21 - Christmas Eve: closing
        // process from 12:30 London time.
        (2021, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-12-27 - T1 - LSE-BUSDAYS-2020-12-21 - Christmas Day
        // (substitute): `NON-trading day.`
        (2021, 12, 27, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-12-28 - T1 - LSE-BUSDAYS-2020-12-21 - Boxing Day (Substitute):
        // `NON-trading day.`
        (2021, 12, 28, Closed, T1, "LSE-BUSDAYS-2020-12-21"),
        // 2021-12-31 - T1 - LSE-BUSDAYS-2020-12-21 - New Year's Eve: closing
        // process from 12:30 London time.
        (2021, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2020-12-21"),
        // 2022-01-03 - T1 - LSE-BUSDAYS-2022-01-24 - New Year's Day
        // (substitute): `NON-trading day.`
        (2022, 1, 3, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-04-15 - T1 - LSE-BUSDAYS-2022-01-24 - Good Friday.
        (2022, 4, 15, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-04-18 - T1 - LSE-BUSDAYS-2022-01-24 - Easter Monday.
        (2022, 4, 18, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-05-02 - T1 - LSE-BUSDAYS-2022-01-24 - Early May Bank Holiday.
        (2022, 5, 2, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-06-02 - T1 - LSE-BUSDAYS-2022-01-24 - Spring Bank Holiday
        // (moved for the Platinum Jubilee).
        (2022, 6, 2, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-06-03 - T1 - LSE-BUSDAYS-2022-01-24 - Platinum Jubilee Bank
        // Holiday.
        (2022, 6, 3, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-08-29 - T1 - LSE-BUSDAYS-2022-01-24 - Summer Bank Holiday.
        (2022, 8, 29, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-12-23 - T1 - LSE-BUSDAYS-2022-01-24 - Christmas Holiday half
        // day: closing process from 12:30 London time.
        (2022, 12, 23, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-12-26 - T1 - LSE-BUSDAYS-2022-01-24 - Boxing Day.
        (2022, 12, 26, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-12-27 - T1 - LSE-BUSDAYS-2022-01-24 - Christmas Day (substitute
        // day).
        (2022, 12, 27, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2022-12-30 - T1 - LSE-BUSDAYS-2022-01-24 - New Year Holiday half
        // day: closing process from 12:30 London time.
        (2022, 12, 30, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2022-01-24"),
        // 2023-01-02 - T1 - LSE-BUSDAYS-2022-01-24 - New Year Day (substitute
        // day).
        (2023, 1, 2, Closed, T1, "LSE-BUSDAYS-2022-01-24"),
        // 2023-04-07 - T1 - LSE-BUSDAYS-API-2023-04-10 - Good Friday:
        // `NON-trading day.`
        (2023, 4, 7, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-04-10 - T1 - LSE-BUSDAYS-API-2023-04-10 - Easter Monday.
        (2023, 4, 10, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-05-01 - T1 - LSE-BUSDAYS-API-2023-04-10 - Early May Bank
        // Holiday.
        (2023, 5, 1, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-05-08 - T1 - LSE-BUSDAYS-API-2023-04-10 - Bank Holiday for the
        // coronation of King Charles III.
        (2023, 5, 8, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-05-29 - T1 - LSE-BUSDAYS-API-2023-04-10 - Spring Bank Holiday.
        (2023, 5, 29, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-08-28 - T1 - LSE-BUSDAYS-API-2023-04-10 - Summer Bank Holiday.
        (2023, 8, 28, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-12-22 - T1 - LSE-BUSDAYS-API-2023-04-10 - Christmas Holiday
        // half day: closing process from 12:30 London time.
        (2023, 12, 22, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-12-25 - T1 - LSE-BUSDAYS-API-2023-04-10 - Christmas Day.
        (2023, 12, 25, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-12-26 - T1 - LSE-BUSDAYS-API-2023-04-10 - Boxing Day.
        (2023, 12, 26, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2023-12-29 - T1 - LSE-BUSDAYS-API-2023-04-10 - New Year Holiday half
        // day: closing process from 12:30 London time.
        (2023, 12, 29, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2024-01-01 - T1 - LSE-BUSDAYS-API-2023-04-10 - New Year Day:
        // `NON-trading day.`
        (2024, 1, 1, Closed, T1, "LSE-BUSDAYS-API-2023-04-10"),
        // 2024-03-29 - T1 - LSE-BUSDAYS-2024-02-07 - Good Friday:
        // `NON-trading day.`
        (2024, 3, 29, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-04-01 - T1 - LSE-BUSDAYS-2024-02-07 - Easter Monday.
        (2024, 4, 1, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-05-06 - T1 - LSE-BUSDAYS-2024-02-07 - Early May Bank Holiday.
        (2024, 5, 6, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-05-27 - T1 - LSE-BUSDAYS-2024-02-07 - Spring Bank Holiday.
        (2024, 5, 27, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-08-26 - T1 - LSE-BUSDAYS-2024-02-07 - Summer Bank Holiday.
        (2024, 8, 26, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-12-24 - T1 - LSE-BUSDAYS-2024-02-07 - Christmas Holiday half
        // day: closing process from 12:30 London time.
        (2024, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-12-25 - T1 - LSE-BUSDAYS-2024-02-07 - Christmas Day.
        (2024, 12, 25, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-12-26 - T1 - LSE-BUSDAYS-2024-02-07 - Boxing Day.
        (2024, 12, 26, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2024-12-31 - T1 - LSE-BUSDAYS-2024-02-07 - New Year Holiday half
        // day: closing process from 12:30 London time.
        (2024, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2024-02-07"),
        // 2025-01-01 - T1 - LSE-BUSDAYS-2024-02-07 - New Year Day:
        // `NON-trading day.`
        (2025, 1, 1, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2025-04-18 - T1 - LSE-BUSDAYS-2025-12-18 - Good Friday: the rolling
        // table had moved past this date before any surviving capture; no
        // status claimed.
        (2025, 4, 18, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-04-21 - T1 - LSE-BUSDAYS-2025-12-18 - Easter Monday: not stated
        // by any surviving artifact; no status claimed.
        (2025, 4, 21, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-05-05 - T1 - LSE-BUSDAYS-2025-12-18 - Early May Bank Holiday:
        // not stated by any surviving artifact; no status claimed.
        (2025, 5, 5, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-05-26 - T1 - LSE-BUSDAYS-2025-12-18 - Spring Bank Holiday: not
        // stated by any surviving artifact; no status claimed.
        (2025, 5, 26, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-08-25 - T1 - LSE-BUSDAYS-2025-12-18 - Summer Bank Holiday: not
        // stated by any surviving artifact; no status claimed.
        (2025, 8, 25, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-24 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Holiday half
        // day, closing process from 12:30 London time.
        (2025, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-25 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Day:
        // `NON-trading day.`
        (2025, 12, 25, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-26 - T1 - LSE-BUSDAYS-2025-12-18 - Boxing Day:
        // `NON-trading day.`
        (2025, 12, 26, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-31 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Holiday half
        // day, closing process from 12:30 London time.
        (2025, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-01-01 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Day:
        // `NON-trading day.`
        (2026, 1, 1, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-04-03 - T1 - LSE-BUSDAYS-2025-12-18 - Good Friday:
        // `NON-trading day.`
        (2026, 4, 3, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-04-06 - T1 - LSE-BUSDAYS-2025-12-18 - Easter Monday:
        // `NON-trading day.`
        (2026, 4, 6, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-05-04 - T1 - LSE-BUSDAYS-2025-12-18 - Early May Bank Holiday:
        // `NON-trading day.`
        (2026, 5, 4, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-05-25 - T1 - LSE-BUSDAYS-2025-12-18 - Spring Bank Holiday:
        // `NON-trading day.`
        (2026, 5, 25, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-08-31 - T1 - LSE-BUSDAYS-2025-12-18 - Summer Bank Holiday:
        // `NON-trading day.`
        (2026, 8, 31, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-24 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Holiday half
        // day, closing process from 12:30 London time.
        (2026, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-25 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Day:
        // `NON-trading day.`
        (2026, 12, 25, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-28 - T1 - LSE-BUSDAYS-2025-12-18 - Boxing Day (substitute):
        // `NON-trading day.`
        (2026, 12, 28, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-31 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Holiday half
        // day, closing process from 12:30 London time.
        (2026, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-01-01 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Day:
        // `NON-trading day.`
        (2027, 1, 1, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-03-26 - T1 - LSE-BUSDAYS-2025-12-18 - Good Friday:
        // `NON-trading day.`
        (2027, 3, 26, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-03-29 - T1 - LSE-BUSDAYS-2025-12-18 - Easter Monday:
        // `NON-trading day.`
        (2027, 3, 29, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-05-03 - T1 - LSE-BUSDAYS-2025-12-18 - Early May Bank Holiday:
        // `NON-trading day.`
        (2027, 5, 3, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-05-31 - T1 - LSE-BUSDAYS-2025-12-18 - Spring Bank Holiday:
        // `NON-trading day.`
        (2027, 5, 31, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-08-30 - T1 - LSE-BUSDAYS-2025-12-18 - Summer Bank Holiday:
        // `NON-trading day.`
        (2027, 8, 30, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-24 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Holiday half
        // day, closing process from 12:30 London time.
        (2027, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-27 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Day
        // (substitute): `NON-trading day.`
        (2027, 12, 27, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-28 - T1 - LSE-BUSDAYS-2025-12-18 - Boxing Day (substitute):
        // `NON-trading day.`
        (2027, 12, 28, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-31 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Holiday half
        // day, closing process from 12:30 London time.
        (2027, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
    ],
};
