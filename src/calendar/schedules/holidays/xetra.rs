// SPDX-License-Identifier: MIT-0

//! Xetra (Deutsche Börse cash market) holiday rows, 2010-2024 and 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Berlin`. The
//! operator states each year's closure set in one sentence — "there will be
//! trading Mondays to Fridays in <year>, with the exception of ..." — printed
//! on the `Trading calendar` PDFs: the `Trading Calendar <year>` grids of
//! 2010-2014 (the `DB_HK_<year>` documents of the xetra.com and
//! deutsche-boerse.com archive) and the `xetra-trading-calendar-<year>` PDFs of
//! 2015-2024. The 2025-2027 rows key on the operator's per-year PDFs and the
//! `Trading calendar and trading hours` page's non-trading-days table. The
//! whole block is **T1**; every row cites the calendar that prints its year.
//!
//! Christmas Eve and New Year's Eve are full closures, not early closes, in
//! every era: the historical sentences place them in the exception list and add
//! "24 and 31 December are settlement days" — no trading, settlement only —
//! which is the same statement as the modern `**` footnote "No trading but
//! settlement is open". Early closes appear only where the operator names both
//! the day and the instant: the live page names 2026's trading holidays —
//! Ascension Day (14 May 2026), Whit Monday (25 May 2026), Corpus Christi
//! (4 June 2026) — and states that "Trading of shares and Exchange traded
//! products on Frankfurt and Xetra ends on public holidays ... at 20:00 CET".
//! The 2025 edition of the same page words the 20:00 note over Börse Frankfurt
//! only, so no 2025 Xetra early close is sourced and none ships; the 2027
//! trading holidays are named by no retrieved artifact, so 2027 ships closures
//! only. Both omissions are recorded as residual gaps in
//! [`docs/evidence/xetra.md`](../../../../../docs/evidence/xetra.md).
//!
//! The historical sentences read as they print, per year: 2011 names no 3
//! October (a Monday) and the 2013 sentence names no 3 October (a Thursday), so
//! the exchange traded those German Unity Days; from 2022 the sentences stop
//! naming Whit Monday and German Unity Day altogether, so those days trade as
//! ordinary days from 2022 on. Weekend-falling exception dates (2011-01-01,
//! 2012-01-01, 2015-10-03, 2016-12-24, 2016-12-25, 2016-12-31, 2021-05-01,
//! 2021-10-03, 2022-01-01, 2022-05-01, 2022-06-06, 2022-10-03, 2022-12-24,
//! 2022-12-31, 2023-05-29, 2023-10-03, 2023-12-24, 2023-12-31, 2024-05-20 and
//! the like) are deleted by the operator's Monday-to-Friday statement itself
//! and key no weekday row. The page's conditional note — "On December 30,
//! 2026, deviating trading hours may apply", echoed by the `*)` footnote on the
//! 2026 PDF — keys no row (LAW-NO-FABRICATED-DATES).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// The trading-holiday close on shares and exchange traded products, `20:00`
/// venue-local, in seconds since midnight.
const HOLIDAY_CLOSE_SSM: u32 = 20 * 3_600;

/// Xetra's built-in holiday rows and the windows they were audited over.
///
/// Every closure row is one line of the operator's per-year calendar sentence
/// ("there will be trading Mondays to Fridays in <year>, with the exception
/// of ...") or the equivalent cell of the page's non-trading-days table; the
/// three 2026 early closes are the page's own named 2026 trading holidays
/// under its 20:00 close rule. A date inside a window with no row is audited
/// normal.
// Evidence: docs/evidence/xetra.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // DB-TC-PDF-2010: the operator's own calendar sentence for 2010
        // 2010-01-01 - T1 - DB-TC-PDF-2010 - New Year's Day.
        (2010, 1, 1, Closed, T1, "DB-TC-PDF-2010"),
        // 2010-04-02 - T1 - DB-TC-PDF-2010 - Good Friday.
        (2010, 4, 2, Closed, T1, "DB-TC-PDF-2010"),
        // 2010-04-05 - T1 - DB-TC-PDF-2010 - Easter Monday.
        (2010, 4, 5, Closed, T1, "DB-TC-PDF-2010"),
        // 2010-12-24 - T1 - DB-TC-PDF-2010 - Christmas Eve — the sentence states it a settlement day.
        (2010, 12, 24, Closed, T1, "DB-TC-PDF-2010"),
        // 2010-12-31 - T1 - DB-TC-PDF-2010 - New Year's Eve — the sentence states it a settlement day.
        (2010, 12, 31, Closed, T1, "DB-TC-PDF-2010"),
        // DB-TC-PDF-2011: the operator's own calendar sentence for 2011
        // 2011-04-22 - T1 - DB-TC-PDF-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "DB-TC-PDF-2011"),
        // 2011-04-25 - T1 - DB-TC-PDF-2011 - Easter Monday.
        (2011, 4, 25, Closed, T1, "DB-TC-PDF-2011"),
        // 2011-12-26 - T1 - DB-TC-PDF-2011 - Boxing Day.
        (2011, 12, 26, Closed, T1, "DB-TC-PDF-2011"),
        // DB-TC-PDF-2012: the operator's own calendar sentence for 2012
        // 2012-04-06 - T1 - DB-TC-PDF-2012 - Good Friday.
        (2012, 4, 6, Closed, T1, "DB-TC-PDF-2012"),
        // 2012-04-09 - T1 - DB-TC-PDF-2012 - Easter Monday.
        (2012, 4, 9, Closed, T1, "DB-TC-PDF-2012"),
        // 2012-05-01 - T1 - DB-TC-PDF-2012 - Labour Day.
        (2012, 5, 1, Closed, T1, "DB-TC-PDF-2012"),
        // 2012-12-24 - T1 - DB-TC-PDF-2012 - Christmas Eve — the sentence states it a settlement day.
        (2012, 12, 24, Closed, T1, "DB-TC-PDF-2012"),
        // 2012-12-25 - T1 - DB-TC-PDF-2012 - Christmas Day.
        (2012, 12, 25, Closed, T1, "DB-TC-PDF-2012"),
        // 2012-12-26 - T1 - DB-TC-PDF-2012 - Boxing Day.
        (2012, 12, 26, Closed, T1, "DB-TC-PDF-2012"),
        // 2012-12-31 - T1 - DB-TC-PDF-2012 - New Year's Eve — the sentence states it a settlement day.
        (2012, 12, 31, Closed, T1, "DB-TC-PDF-2012"),
        // DB-TC-PDF-2013: the operator's own calendar sentence for 2013
        // 2013-01-01 - T1 - DB-TC-PDF-2013 - New Year's Day.
        (2013, 1, 1, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-03-29 - T1 - DB-TC-PDF-2013 - Good Friday.
        (2013, 3, 29, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-04-01 - T1 - DB-TC-PDF-2013 - Easter Monday.
        (2013, 4, 1, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-05-01 - T1 - DB-TC-PDF-2013 - Labour Day.
        (2013, 5, 1, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-12-24 - T1 - DB-TC-PDF-2013 - Christmas Eve — the sentence states it a settlement day.
        (2013, 12, 24, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-12-25 - T1 - DB-TC-PDF-2013 - Christmas Day.
        (2013, 12, 25, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-12-26 - T1 - DB-TC-PDF-2013 - Boxing Day.
        (2013, 12, 26, Closed, T1, "DB-TC-PDF-2013"),
        // 2013-12-31 - T1 - DB-TC-PDF-2013 - New Year's Eve — the sentence states it a settlement day.
        (2013, 12, 31, Closed, T1, "DB-TC-PDF-2013"),
        // DB-TC-PDF-2014: the operator's own calendar sentence for 2014
        // 2014-01-01 - T1 - DB-TC-PDF-2014 - New Year's Day.
        (2014, 1, 1, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-04-18 - T1 - DB-TC-PDF-2014 - Good Friday.
        (2014, 4, 18, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-04-21 - T1 - DB-TC-PDF-2014 - Easter Monday.
        (2014, 4, 21, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-05-01 - T1 - DB-TC-PDF-2014 - Labour Day.
        (2014, 5, 1, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-10-03 - T1 - DB-TC-PDF-2014 - German Unity Day — the sentence states it a settlement day.
        (2014, 10, 3, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-12-24 - T1 - DB-TC-PDF-2014 - Christmas Eve — the sentence states it a settlement day.
        (2014, 12, 24, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-12-25 - T1 - DB-TC-PDF-2014 - Christmas Day.
        (2014, 12, 25, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-12-26 - T1 - DB-TC-PDF-2014 - Boxing Day.
        (2014, 12, 26, Closed, T1, "DB-TC-PDF-2014"),
        // 2014-12-31 - T1 - DB-TC-PDF-2014 - New Year's Eve — the sentence states it a settlement day.
        (2014, 12, 31, Closed, T1, "DB-TC-PDF-2014"),
        // DB-TC-PDF-2015: the operator's own calendar sentence for 2015
        // 2015-01-01 - T1 - DB-TC-PDF-2015 - New Year's Day.
        (2015, 1, 1, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-04-03 - T1 - DB-TC-PDF-2015 - Good Friday.
        (2015, 4, 3, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-04-06 - T1 - DB-TC-PDF-2015 - Easter Monday.
        (2015, 4, 6, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-05-01 - T1 - DB-TC-PDF-2015 - Labour Day.
        (2015, 5, 1, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-05-25 - T1 - DB-TC-PDF-2015 - Whit Monday — the sentence states it a settlement day.
        (2015, 5, 25, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-12-24 - T1 - DB-TC-PDF-2015 - Christmas Eve — the sentence states it a settlement day.
        (2015, 12, 24, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-12-25 - T1 - DB-TC-PDF-2015 - Christmas Day.
        (2015, 12, 25, Closed, T1, "DB-TC-PDF-2015"),
        // 2015-12-31 - T1 - DB-TC-PDF-2015 - New Year's Eve — the sentence states it a settlement day.
        (2015, 12, 31, Closed, T1, "DB-TC-PDF-2015"),
        // DB-TC-PDF-2016: the operator's own calendar sentence for 2016
        // 2016-01-01 - T1 - DB-TC-PDF-2016 - New Year's Day.
        (2016, 1, 1, Closed, T1, "DB-TC-PDF-2016"),
        // 2016-03-25 - T1 - DB-TC-PDF-2016 - Good Friday.
        (2016, 3, 25, Closed, T1, "DB-TC-PDF-2016"),
        // 2016-03-28 - T1 - DB-TC-PDF-2016 - Easter Monday.
        (2016, 3, 28, Closed, T1, "DB-TC-PDF-2016"),
        // 2016-05-16 - T1 - DB-TC-PDF-2016 - Whit Monday — the sentence states it a settlement day.
        (2016, 5, 16, Closed, T1, "DB-TC-PDF-2016"),
        // 2016-10-03 - T1 - DB-TC-PDF-2016 - German Unity Day — the sentence states it a settlement day.
        (2016, 10, 3, Closed, T1, "DB-TC-PDF-2016"),
        // 2016-12-26 - T1 - DB-TC-PDF-2016 - Boxing Day.
        (2016, 12, 26, Closed, T1, "DB-TC-PDF-2016"),
        // DB-TC-PDF-2017: the operator's own calendar sentence for 2017
        // 2017-04-14 - T1 - DB-TC-PDF-2017 - Good Friday.
        (2017, 4, 14, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-04-17 - T1 - DB-TC-PDF-2017 - Easter Monday.
        (2017, 4, 17, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-05-01 - T1 - DB-TC-PDF-2017 - Labour Day.
        (2017, 5, 1, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-06-05 - T1 - DB-TC-PDF-2017 - Whit Monday — the sentence states it a settlement day.
        (2017, 6, 5, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-10-03 - T1 - DB-TC-PDF-2017 - German Unity Day — the sentence states it a settlement day.
        (2017, 10, 3, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-10-31 - T1 - DB-TC-PDF-2017 - Reformation Day — the sentence states it a settlement day.
        (2017, 10, 31, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-12-25 - T1 - DB-TC-PDF-2017 - Christmas Day.
        (2017, 12, 25, Closed, T1, "DB-TC-PDF-2017"),
        // 2017-12-26 - T1 - DB-TC-PDF-2017 - Boxing Day.
        (2017, 12, 26, Closed, T1, "DB-TC-PDF-2017"),
        // DB-TC-PDF-2018: the operator's own calendar sentence for 2018
        // 2018-01-01 - T1 - DB-TC-PDF-2018 - New Year's Day.
        (2018, 1, 1, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-03-30 - T1 - DB-TC-PDF-2018 - Good Friday.
        (2018, 3, 30, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-04-02 - T1 - DB-TC-PDF-2018 - Easter Monday.
        (2018, 4, 2, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-05-01 - T1 - DB-TC-PDF-2018 - Labour Day.
        (2018, 5, 1, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-05-21 - T1 - DB-TC-PDF-2018 - Whit Monday — the sentence states it a settlement day.
        (2018, 5, 21, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-10-03 - T1 - DB-TC-PDF-2018 - German Unity Day — the sentence states it a settlement day.
        (2018, 10, 3, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-12-24 - T1 - DB-TC-PDF-2018 - Christmas Eve — the sentence states it a settlement day.
        (2018, 12, 24, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-12-25 - T1 - DB-TC-PDF-2018 - Christmas Day.
        (2018, 12, 25, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-12-26 - T1 - DB-TC-PDF-2018 - Boxing Day.
        (2018, 12, 26, Closed, T1, "DB-TC-PDF-2018"),
        // 2018-12-31 - T1 - DB-TC-PDF-2018 - New Year's Eve — the sentence states it a settlement day.
        (2018, 12, 31, Closed, T1, "DB-TC-PDF-2018"),
        // DB-TC-PDF-2019: the operator's own calendar sentence for 2019
        // 2019-01-01 - T1 - DB-TC-PDF-2019 - New Year's Day.
        (2019, 1, 1, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-04-19 - T1 - DB-TC-PDF-2019 - Good Friday.
        (2019, 4, 19, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-04-22 - T1 - DB-TC-PDF-2019 - Easter Monday.
        (2019, 4, 22, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-05-01 - T1 - DB-TC-PDF-2019 - Labour Day.
        (2019, 5, 1, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-06-10 - T1 - DB-TC-PDF-2019 - Whit Monday — the sentence states it a settlement day.
        (2019, 6, 10, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-10-03 - T1 - DB-TC-PDF-2019 - German Unity Day — the sentence states it a settlement day.
        (2019, 10, 3, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-12-24 - T1 - DB-TC-PDF-2019 - Christmas Eve — the sentence states it a settlement day.
        (2019, 12, 24, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-12-25 - T1 - DB-TC-PDF-2019 - Christmas Day.
        (2019, 12, 25, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-12-26 - T1 - DB-TC-PDF-2019 - Boxing Day.
        (2019, 12, 26, Closed, T1, "DB-TC-PDF-2019"),
        // 2019-12-31 - T1 - DB-TC-PDF-2019 - New Year's Eve — the sentence states it a settlement day.
        (2019, 12, 31, Closed, T1, "DB-TC-PDF-2019"),
        // DB-TC-PDF-2020: the operator's own calendar sentence for 2020
        // 2020-01-01 - T1 - DB-TC-PDF-2020 - New Year's Day.
        (2020, 1, 1, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-04-10 - T1 - DB-TC-PDF-2020 - Good Friday.
        (2020, 4, 10, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-04-13 - T1 - DB-TC-PDF-2020 - Easter Monday.
        (2020, 4, 13, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-05-01 - T1 - DB-TC-PDF-2020 - Labour Day.
        (2020, 5, 1, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-06-01 - T1 - DB-TC-PDF-2020 - Whit Monday — the sentence states it a settlement day.
        (2020, 6, 1, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-12-24 - T1 - DB-TC-PDF-2020 - Christmas Eve — the sentence states it a settlement day.
        (2020, 12, 24, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-12-25 - T1 - DB-TC-PDF-2020 - Christmas Day.
        (2020, 12, 25, Closed, T1, "DB-TC-PDF-2020"),
        // 2020-12-31 - T1 - DB-TC-PDF-2020 - New Year's Eve — the sentence states it a settlement day.
        (2020, 12, 31, Closed, T1, "DB-TC-PDF-2020"),
        // DB-TC-PDF-2021: the operator's own calendar sentence for 2021
        // 2021-01-01 - T1 - DB-TC-PDF-2021 - New Year's Day.
        (2021, 1, 1, Closed, T1, "DB-TC-PDF-2021"),
        // 2021-04-02 - T1 - DB-TC-PDF-2021 - Good Friday.
        (2021, 4, 2, Closed, T1, "DB-TC-PDF-2021"),
        // 2021-04-05 - T1 - DB-TC-PDF-2021 - Easter Monday.
        (2021, 4, 5, Closed, T1, "DB-TC-PDF-2021"),
        // 2021-05-24 - T1 - DB-TC-PDF-2021 - Whit Monday — the sentence states it a settlement day.
        (2021, 5, 24, Closed, T1, "DB-TC-PDF-2021"),
        // 2021-12-24 - T1 - DB-TC-PDF-2021 - Christmas Eve — the sentence states it a settlement day.
        (2021, 12, 24, Closed, T1, "DB-TC-PDF-2021"),
        // 2021-12-31 - T1 - DB-TC-PDF-2021 - New Year's Eve — the sentence states it a settlement day.
        (2021, 12, 31, Closed, T1, "DB-TC-PDF-2021"),
        // DB-TC-PDF-2022: the operator's own calendar sentence for 2022
        // 2022-04-15 - T1 - DB-TC-PDF-2022 - Good Friday.
        (2022, 4, 15, Closed, T1, "DB-TC-PDF-2022"),
        // 2022-04-18 - T1 - DB-TC-PDF-2022 - Easter Monday.
        (2022, 4, 18, Closed, T1, "DB-TC-PDF-2022"),
        // 2022-12-26 - T1 - DB-TC-PDF-2022 - Boxing Day.
        (2022, 12, 26, Closed, T1, "DB-TC-PDF-2022"),
        // DB-TC-PDF-2023: the operator's own calendar sentence for 2023
        // 2023-04-07 - T1 - DB-TC-PDF-2023 - Good Friday.
        (2023, 4, 7, Closed, T1, "DB-TC-PDF-2023"),
        // 2023-04-10 - T1 - DB-TC-PDF-2023 - Easter Monday.
        (2023, 4, 10, Closed, T1, "DB-TC-PDF-2023"),
        // 2023-05-01 - T1 - DB-TC-PDF-2023 - Labour Day.
        (2023, 5, 1, Closed, T1, "DB-TC-PDF-2023"),
        // 2023-12-25 - T1 - DB-TC-PDF-2023 - Christmas Day.
        (2023, 12, 25, Closed, T1, "DB-TC-PDF-2023"),
        // 2023-12-26 - T1 - DB-TC-PDF-2023 - Boxing Day.
        (2023, 12, 26, Closed, T1, "DB-TC-PDF-2023"),
        // DB-TC-PDF-2024: the operator's own calendar sentence for 2024
        // 2024-01-01 - T1 - DB-TC-PDF-2024 - New Year's Day.
        (2024, 1, 1, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-03-29 - T1 - DB-TC-PDF-2024 - Good Friday.
        (2024, 3, 29, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-04-01 - T1 - DB-TC-PDF-2024 - Easter Monday.
        (2024, 4, 1, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-05-01 - T1 - DB-TC-PDF-2024 - Labour Day.
        (2024, 5, 1, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-12-24 - T1 - DB-TC-PDF-2024 - Christmas Eve — the sentence states it a settlement day.
        (2024, 12, 24, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-12-25 - T1 - DB-TC-PDF-2024 - Christmas Day.
        (2024, 12, 25, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-12-26 - T1 - DB-TC-PDF-2024 - Boxing Day.
        (2024, 12, 26, Closed, T1, "DB-TC-PDF-2024"),
        // 2024-12-31 - T1 - DB-TC-PDF-2024 - New Year's Eve — the sentence states it a settlement day.
        (2024, 12, 31, Closed, T1, "DB-TC-PDF-2024"),

        // 2025-01-01 - T1 - DB-TC-PDF-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-04-18 - T1 - DB-TC-PDF-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-04-21 - T1 - DB-TC-PDF-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-05-01 - T1 - DB-TC-PDF-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-24 - T1 - DB-TC-PDF-2025 - Christmas Eve: "No trading but
        // settlement is open".
        (2025, 12, 24, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-25 - T1 - DB-TC-PDF-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-26 - T1 - DB-TC-PDF-2025 - Boxing Day.
        (2025, 12, 26, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-31 - T1 - DB-TC-PDF-2025 - New Year's Eve: "No trading but
        // settlement is open".
        (2025, 12, 31, Closed, T1, "DB-TC-PDF-2025"),
        // 2026-01-01 - T1 - DB-TC-PDF-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-04-03 - T1 - DB-TC-PDF-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-04-06 - T1 - DB-TC-PDF-2026 - Easter Monday.
        (2026, 4, 6, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-05-01 - T1 - DB-TC-PDF-2026 - Labour Day.
        (2026, 5, 1, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-05-14 - T1 - DB-TC-PAGE - Ascension Day, a named 2026 trading
        // holiday; shares and ETPs end at 20:00 CET.
        (2026, 5, 14, early_close(HOLIDAY_CLOSE_SSM), T1, "DB-TC-PAGE"),
        // 2026-05-25 - T1 - DB-TC-PAGE - Whit Monday, a named 2026 trading
        // holiday; shares and ETPs end at 20:00 CET.
        (2026, 5, 25, early_close(HOLIDAY_CLOSE_SSM), T1, "DB-TC-PAGE"),
        // 2026-06-04 - T1 - DB-TC-PAGE - Corpus Christi, a named 2026 trading
        // holiday; shares and ETPs end at 20:00 CET.
        (2026, 6, 4, early_close(HOLIDAY_CLOSE_SSM), T1, "DB-TC-PAGE"),
        // 2026-12-24 - T1 - DB-TC-PDF-2026 - Christmas Eve: "No trading but
        // settlement is open".
        (2026, 12, 24, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-12-25 - T1 - DB-TC-PDF-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-12-31 - T1 - DB-TC-PDF-2026 - New Year's Eve: "No trading but
        // settlement is open".
        (2026, 12, 31, Closed, T1, "DB-TC-PDF-2026"),
        // 2027-01-01 - T1 - DB-TC-PAGE - New Year's Day.
        (2027, 1, 1, Closed, T1, "DB-TC-PAGE"),
        // 2027-03-26 - T1 - DB-TC-PAGE - Good Friday.
        (2027, 3, 26, Closed, T1, "DB-TC-PAGE"),
        // 2027-03-29 - T1 - DB-TC-PAGE - Easter Monday.
        (2027, 3, 29, Closed, T1, "DB-TC-PAGE"),
        // 2027-12-24 - T1 - DB-TC-PAGE - Christmas Eve: "No trading but
        // settlement is open".
        (2027, 12, 24, Closed, T1, "DB-TC-PAGE"),
        // 2027-12-31 - T1 - DB-TC-PAGE - New Year's Eve: "No trading but
        // settlement is open".
        (2027, 12, 31, Closed, T1, "DB-TC-PAGE"),
    ],
};
