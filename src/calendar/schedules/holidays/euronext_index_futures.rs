// SPDX-License-Identifier: MIT-0

//! Holiday rows for the CAC 40 and AEX index futures, 2010–2027.
//! [Evidence](../../../../docs/evidence/euronext_index_futures.md).

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::{HolidayTable, holidays};

/// Euronext index-futures holiday table.
// Evidence: docs/evidence/euronext_index_futures.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - EURONEXT-PR-2010 - New Year's Day.
        (2010, 1, 1, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-04-02 - T1 - EURONEXT-PR-2010 - Good Friday.
        (2010, 4, 2, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-04-05 - T1 - EURONEXT-PR-2010 - Easter Monday.
        (2010, 4, 5, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-12-24 - T1 - EURONEXT-PR-2010 - December half day.
        (2010, 12, 24, Unsourced, T1, "EURONEXT-PR-2010"),
        // 2010-12-31 - T1 - EURONEXT-PR-2010 - December half day.
        (2010, 12, 31, Unsourced, T1, "EURONEXT-PR-2010"),
        // 2011-04-22 - T1 - EURONEXT-PR-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-04-25 - T1 - EURONEXT-PR-2011 - Easter Monday.
        (2011, 4, 25, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-12-26 - T1 - EURONEXT-PR-2011 - Boxing Day.
        (2011, 12, 26, Closed, T1, "EURONEXT-PR-2011"),
        // 2012-04-06 - T1 - EURONEXT-PR-2012 - Good Friday.
        (2012, 4, 6, Closed, T1, "EURONEXT-PR-2012"),
        // 2012-04-09 - T1 - EURONEXT-PR-2012 - Easter Monday.
        (2012, 4, 9, Closed, T1, "EURONEXT-PR-2012"),
        // 2012-05-01 - T1 - EURONEXT-PR-2012 - Labour Day.
        (2012, 5, 1, Closed, T1, "EURONEXT-PR-2012"),
        // 2012-12-24 - T1 - EURONEXT-PR-2012 - Christmas Eve half day.
        (2012, 12, 24, Unsourced, T1, "EURONEXT-PR-2012"),
        // 2012-12-25 - T1 - EURONEXT-PR-2012 - Christmas Day.
        (2012, 12, 25, Closed, T1, "EURONEXT-PR-2012"),
        // 2012-12-26 - T1 - EURONEXT-PR-2012 - Boxing Day.
        (2012, 12, 26, Closed, T1, "EURONEXT-PR-2012"),
        // 2012-12-31 - T1 - EURONEXT-PR-2012 - New Year's Eve half day.
        (2012, 12, 31, Unsourced, T1, "EURONEXT-PR-2012"),
        // 2013-01-01 - T1 - EURONEXT-PR-2013 - New Year's Day.
        (2013, 1, 1, Closed, T1, "EURONEXT-PR-2013"),
        // 2013-03-29 - T1 - EURONEXT-PR-2013 - Good Friday.
        (2013, 3, 29, Closed, T1, "EURONEXT-PR-2013"),
        // 2013-04-01 - T1 - EURONEXT-PR-2013 - Easter Monday.
        (2013, 4, 1, Closed, T1, "EURONEXT-PR-2013"),
        // 2013-05-01 - T1 - EURONEXT-PR-2013 - Labour Day.
        (2013, 5, 1, Closed, T1, "EURONEXT-PR-2013"),
        // 2013-12-24 - T1 - EURONEXT-PR-2013 - Christmas Eve half day.
        (2013, 12, 24, Unsourced, T1, "EURONEXT-PR-2013"),
        // 2013-12-25 - T1 - EURONEXT-PR-2013 - Christmas Day.
        (2013, 12, 25, Closed, T1, "EURONEXT-PR-2013"),
        // 2013-12-26 - T1 - EURONEXT-PR-2013 - Boxing Day.
        (2013, 12, 26, Closed, T1, "EURONEXT-PR-2013"),
        // 2013-12-31 - T1 - EURONEXT-PR-2013 - New Year's Eve half day.
        (2013, 12, 31, Unsourced, T1, "EURONEXT-PR-2013"),
        // 2014-01-01 - T1 - EURONEXT-HH-2014-01-12 - New Year's Day.
        (2014, 1, 1, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-04-18 - T1 - EURONEXT-HH-2014-01-12 - Good Friday.
        (2014, 4, 18, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-04-21 - T1 - EURONEXT-HH-2014-01-12 - Easter Monday.
        (2014, 4, 21, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-05-01 - T1 - EURONEXT-HH-2014-01-12 - Labour Day.
        (2014, 5, 1, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-24 - T1 - EURONEXT-HH-2014-01-12 - December half day.
        (2014, 12, 24, Unsourced, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-25 - T1 - EURONEXT-HH-2014-01-12 - Christmas Day.
        (2014, 12, 25, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-26 - T1 - EURONEXT-HH-2014-01-12 - Boxing Day.
        (2014, 12, 26, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-31 - T1 - EURONEXT-HH-2014-01-12 - December half day.
        (2014, 12, 31, Unsourced, T1, "EURONEXT-HH-2014-01-12"),
        // 2015-01-01 - T1 - EURONEXT-HH-2015-03-22 - New Year's Day.
        (2015, 1, 1, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-04-03 - T1 - EURONEXT-HH-2015-03-22 - Good Friday.
        (2015, 4, 3, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-04-06 - T1 - EURONEXT-HH-2015-03-22 - Easter Monday.
        (2015, 4, 6, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-05-01 - T1 - EURONEXT-HH-2015-03-22 - Labour Day.
        (2015, 5, 1, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-24 - T1 - EURONEXT-HH-2015-03-22 - December half day.
        (2015, 12, 24, Unsourced, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-25 - T1 - EURONEXT-HH-2015-03-22 - Christmas Day.
        (2015, 12, 25, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-31 - T1 - EURONEXT-HH-2015-03-22 - New Year's Eve.
        (2015, 12, 31, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2016-01-01 - T1 - EURONEXT-HH-2016-03-04 - New Year's Day.
        (2016, 1, 1, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-03-25 - T1 - EURONEXT-HH-2016-03-04 - Good Friday.
        (2016, 3, 25, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-03-28 - T1 - EURONEXT-HH-2016-03-04 - Easter Monday.
        (2016, 3, 28, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-12-26 - T1 - EURONEXT-HH-2016-03-04 - Boxing Day.
        (2016, 12, 26, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2017-04-14 - T1 - EURONEXT-HH-2017-08-05 - Good Friday.
        (2017, 4, 14, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-04-17 - T1 - EURONEXT-HH-2017-08-05 - Easter Monday.
        (2017, 4, 17, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-05-01 - T1 - EURONEXT-HH-2017-08-05 - Labour Day.
        (2017, 5, 1, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-12-25 - T1 - EURONEXT-HH-2017-08-05 - Christmas Day.
        (2017, 12, 25, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-12-26 - T1 - EURONEXT-HH-2017-08-05 - Boxing Day.
        (2017, 12, 26, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2018-01-01 - T1 - EURONEXT-HH-2018-08-26 - New Year's Day.
        (2018, 1, 1, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-03-30 - T1 - EURONEXT-HH-2018-08-26 - Good Friday.
        (2018, 3, 30, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-04-02 - T1 - EURONEXT-HH-2018-08-26 - Easter Monday.
        (2018, 4, 2, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-05-01 - T1 - EURONEXT-HH-2018-08-26 - Labour Day.
        (2018, 5, 1, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-24 - T1 - EURONEXT-HH-2018-08-26 - December half day.
        (2018, 12, 24, Unsourced, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-25 - T1 - EURONEXT-HH-2018-08-26 - Christmas Day.
        (2018, 12, 25, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-26 - T1 - EURONEXT-HH-2018-08-26 - Boxing Day.
        (2018, 12, 26, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-31 - T1 - EURONEXT-HH-2018-08-26 - December half day.
        (2018, 12, 31, Unsourced, T1, "EURONEXT-HH-2018-08-26"),
        // 2019-01-01 - T1 - EURONEXT-HH-2019-03-25 - New Year's Day.
        (2019, 1, 1, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-04-19 - T1 - EURONEXT-HH-2019-03-25 - Good Friday.
        (2019, 4, 19, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-04-22 - T1 - EURONEXT-HH-2019-03-25 - Easter Monday.
        (2019, 4, 22, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-05-01 - T1 - EURONEXT-HH-2019-03-25 - Labour Day.
        (2019, 5, 1, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-24 - T1 - EURONEXT-HH-2019-12-10 - December half day.
        (2019, 12, 24, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2019-12-25 - T1 - EURONEXT-HH-2019-03-25 - Christmas Day.
        (2019, 12, 25, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-26 - T1 - EURONEXT-HH-2019-03-25 - Boxing Day.
        (2019, 12, 26, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-31 - T1 - EURONEXT-HH-2019-12-10 - December half day.
        (2019, 12, 31, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-01-01 - T1 - EURONEXT-HH-2019-12-10 - New Year's Day.
        (2020, 1, 1, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-04-10 - T1 - EURONEXT-HH-2019-12-10 - Good Friday.
        (2020, 4, 10, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-04-13 - T1 - EURONEXT-HH-2019-12-10 - Easter Monday.
        (2020, 4, 13, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-05-01 - T1 - EURONEXT-HH-2019-12-10 - Labour Day.
        (2020, 5, 1, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-24 - T1 - EURONEXT-HH-2019-12-10 - December half day.
        (2020, 12, 24, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-25 - T1 - EURONEXT-HH-2019-12-10 - Christmas Day.
        (2020, 12, 25, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-31 - T1 - EURONEXT-HH-2019-12-10 - December half day.
        (2020, 12, 31, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2021-01-01 - T1 - EURONEXT-HH-2020-12-05 - New Year's Day.
        (2021, 1, 1, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-04-02 - T1 - EURONEXT-HH-2020-12-05 - Good Friday.
        (2021, 4, 2, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-04-05 - T1 - EURONEXT-HH-2020-12-05 - Easter Monday.
        (2021, 4, 5, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-12-24 - T1 - EURONEXT-HH-2020-12-05 - December half day.
        (2021, 12, 24, Unsourced, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-12-31 - T1 - EURONEXT-HH-2020-12-05 - December half day.
        (2021, 12, 31, Unsourced, T1, "EURONEXT-HH-2020-12-05"),
        // 2022-04-15 - T1 - EURONEXT-HH-2022-05-23 - Good Friday.
        (2022, 4, 15, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2022-04-18 - T1 - EURONEXT-HH-2022-05-23 - Easter Monday.
        (2022, 4, 18, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2022-12-26 - T1 - EURONEXT-HH-2022-05-23 - Boxing Day.
        (2022, 12, 26, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2023-04-07 - T1 - EURONEXT-HH-2023-11-27 - Good Friday.
        (2023, 4, 7, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-04-10 - T1 - EURONEXT-HH-2023-11-27 - Easter Monday.
        (2023, 4, 10, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-05-01 - T1 - EURONEXT-HH-2023-11-27 - Labour Day.
        (2023, 5, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas Day.
        (2023, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-12-26 - T1 - EURONEXT-HH-2023-11-27 - Boxing Day.
        (2023, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-01-01 - T1 - EURONEXT-HH-2023-11-27 - New Year's Day.
        (2024, 1, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-03-29 - T1 - EURONEXT-HH-2023-11-27 - Good Friday.
        (2024, 3, 29, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-04-01 - T1 - EURONEXT-HH-2023-11-27 - Easter Monday.
        (2024, 4, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-05-01 - T1 - EURONEXT-HH-2023-11-27 - Labour Day.
        (2024, 5, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-24 - T1 - EURONEXT-HH-2023-11-27 - December half day.
        (2024, 12, 24, Unsourced, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas Day.
        (2024, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-26 - T1 - EURONEXT-HH-2023-11-27 - Boxing Day.
        (2024, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-31 - T1 - EURONEXT-HH-2023-11-27 - December half day.
        (2024, 12, 31, Unsourced, T1, "EURONEXT-HH-2023-11-27"),
        // 2025-01-01 - T1 - EURONEXT-HH-2025-12-06 - New Year's Day.
        (2025, 1, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-18 - T1 - EURONEXT-HH-2025-12-06 - Good Friday.
        (2025, 4, 18, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-21 - T1 - EURONEXT-HH-2025-12-06 - Easter Monday.
        (2025, 4, 21, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-05-01 - T1 - EURONEXT-HH-2025-12-06 - Labour Day.
        (2025, 5, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-24 - T1 - EURONEXT-HH-2025-12-06 - December half day.
        (2025, 12, 24, Unsourced, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-25 - T1 - EURONEXT-HH-2025-12-06 - Christmas Day.
        (2025, 12, 25, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-26 - T1 - EURONEXT-HH-2025-12-06 - Boxing Day.
        (2025, 12, 26, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-31 - T1 - EURONEXT-HH-2025-12-06 - December half day.
        (2025, 12, 31, Unsourced, T1, "EURONEXT-HH-2025-12-06"),
        // 2026-01-01 - T1 - EURONEXT-IF-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-03 - T1 - EURONEXT-IF-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-06 - T1 - EURONEXT-IF-2026 - Easter Monday.
        (2026, 4, 6, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-05-01 - T1 - EURONEXT-IF-2026 - Labour Day.
        (2026, 5, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-24 - T1 - EURONEXT-IF-2026 - December half day.
        (2026, 12, 24, Unsourced, T1, "EURONEXT-IF-2026"),
        // 2026-12-25 - T1 - EURONEXT-IF-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-31 - T1 - EURONEXT-IF-2026 - December half day.
        (2026, 12, 31, Unsourced, T1, "EURONEXT-IF-2026"),
        // 2027-01-01 - T1 - EURONEXT-IF-2027 - New Year's Day.
        (2027, 1, 1, Closed, T1, "EURONEXT-IF-2027"),
        // 2027-03-26 - T1 - EURONEXT-IF-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "EURONEXT-IF-2027"),
        // 2027-03-29 - T1 - EURONEXT-IF-2027 - Easter Monday.
        (2027, 3, 29, Closed, T1, "EURONEXT-IF-2027"),
        // 2027-12-24 - T1 - EURONEXT-IF-2027 - December half day.
        (2027, 12, 24, Unsourced, T1, "EURONEXT-IF-2027"),
        // 2027-12-31 - T1 - EURONEXT-IF-2027 - December half day.
        (2027, 12, 31, Unsourced, T1, "EURONEXT-IF-2027"),
    ],
};
