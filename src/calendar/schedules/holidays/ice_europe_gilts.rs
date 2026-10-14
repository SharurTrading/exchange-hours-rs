// SPDX-License-Identifier: MIT-0

//! ICE Futures Europe gilt-futures holiday and early-close table: audited
//! windows 2010-12-24..2014-12-31, 2021, 2022-12-23..2023-01-02 and
//! 2024..2026.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, EarlyClose, Unsourced};
use super::{HolidayTable, holidays};

// The Christmas-eve and New-Year-eve half days: the NYSE Liffe notices'
// attachments print the gilt rows closing 12:15 GMT (2010-2013), and the IFEU
// Christmas documents print "Trading ceases 12:15 (GMT)" (2022-12-30 and both
// 2025 eves). 2022-12-23 printed a later 13:00 GMT cessation.

// The announced but time-unpublished eves of 2014, 2021, 2024 and 2026 stay
// `Unsourced`: the operator said "partial trading day" without stating an
// instant.
// Evidence: docs/evidence/ice_europe_gilts.md
const CLOSE_1215_SSM: u32 = 12 * 3_600 + 15 * 60;
const CLOSE_1300_SSM: u32 = 13 * 3_600;

// Evidence: docs/evidence/ice_europe_gilts.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [
        (2010, 12, 24) ..= (2014, 12, 31),
        (2021, 1, 1) ..= (2021, 12, 31),
        (2022, 12, 23) ..= (2023, 1, 2),
        (2024, 1, 1) ..= (2026, 12, 31),
    ],
    rows: [
        (2010, 12, 24, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2010"),
        (2010, 12, 27, Closed, T1, "LIFFE-ZIP-2010"),
        (2010, 12, 28, Closed, T1, "LIFFE-ZIP-2010"),
        (2010, 12, 31, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2010"),
        (2011, 1, 3, Closed, T1, "LIFFE-ZIP-2010"),
        (2011, 4, 22, Closed, T1, "LIFFE-ZIP-2010"),
        (2011, 4, 25, Closed, T1, "LIFFE-ZIP-2010"),
        (2011, 5, 2, Closed, T1, "LIFFE-ZIP-2010"),
        (2011, 5, 30, Closed, T1, "LIFFE-ZIP-2010"),
        (2011, 8, 29, Closed, T1, "LIFFE-ZIP-2010"),
        (2011, 12, 23, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2011"),
        (2011, 12, 26, Closed, T1, "LIFFE-ZIP-2011"),
        (2011, 12, 27, Closed, T1, "LIFFE-ZIP-2011"),
        (2011, 12, 30, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2011"),
        (2012, 1, 2, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 4, 6, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 4, 9, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 5, 7, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 6, 4, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 6, 5, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 8, 27, Closed, T1, "LIFFE-ZIP-2011"),
        (2012, 12, 24, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2012"),
        (2012, 12, 25, Closed, T1, "LIFFE-ZIP-2012"),
        (2012, 12, 26, Closed, T1, "LIFFE-ZIP-2012"),
        (2012, 12, 31, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2012"),
        (2013, 1, 1, Closed, T1, "LIFFE-ZIP-2012"),
        (2013, 3, 29, Closed, T1, "LIFFE-ZIP-2012"),
        (2013, 4, 1, Closed, T1, "LIFFE-ZIP-2012"),
        (2013, 5, 6, Closed, T1, "LIFFE-ZIP-2012"),
        (2013, 5, 27, Closed, T1, "LIFFE-ZIP-2012"),
        (2013, 8, 26, Closed, T1, "LIFFE-ZIP-2012"),
        (2013, 12, 24, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2013"),
        (2013, 12, 25, Closed, T1, "LIFFE-ZIP-2013"),
        (2013, 12, 26, Closed, T1, "LIFFE-ZIP-2013"),
        (2013, 12, 31, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "LIFFE-ZIP-2013"),
        (2014, 1, 1, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 4, 18, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 4, 21, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 5, 5, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 5, 26, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 8, 25, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 12, 24, Unsourced, T1, "LIFFE-ZIP-2013"),
        (2014, 12, 25, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 12, 26, Closed, T1, "LIFFE-ZIP-2013"),
        (2014, 12, 31, Unsourced, T1, "LIFFE-ZIP-2013"),
        (2021, 1, 1, Closed, T1, "IFEU-FI-2021"),
        (2021, 4, 2, Closed, T1, "IFEU-FI-2021"),
        (2021, 4, 5, Closed, T1, "IFEU-FI-2021"),
        (2021, 5, 3, Closed, T1, "IFEU-FI-2021"),
        (2021, 5, 31, Closed, T1, "IFEU-FI-2021"),
        (2021, 8, 30, Closed, T1, "IFEU-FI-2021"),
        (2021, 12, 24, Unsourced, T1, "IFEU-FI-2021"),
        (2021, 12, 27, Closed, T1, "IFEU-FI-2021"),
        (2021, 12, 28, Closed, T1, "IFEU-FI-2021"),
        (2021, 12, 31, Unsourced, T1, "IFEU-FI-2021"),
        (2022, 12, 23, EarlyClose { close_ssm: CLOSE_1300_SSM }, T1, "IFEU-XMAS-2022"),
        (2022, 12, 26, Closed, T1, "IFEU-XMAS-2022"),
        (2022, 12, 27, Closed, T1, "IFEU-XMAS-2022"),
        (2022, 12, 30, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "IFEU-XMAS-2022"),
        (2023, 1, 2, Closed, T1, "IFEU-XMAS-2022"),
        (2024, 1, 1, Closed, T1, "IFEU-FI-2024"),
        (2024, 3, 29, Closed, T1, "IFEU-FI-2024"),
        (2024, 4, 1, Closed, T1, "IFEU-FI-2024"),
        (2024, 5, 6, Closed, T1, "IFEU-FI-2024"),
        (2024, 5, 27, Closed, T1, "IFEU-FI-2024"),
        (2024, 8, 26, Closed, T1, "IFEU-FI-2024"),
        (2024, 12, 24, Unsourced, T1, "IFEU-FI-2024"),
        (2024, 12, 25, Closed, T1, "IFEU-FI-2024"),
        (2024, 12, 26, Closed, T1, "IFEU-FI-2024"),
        (2024, 12, 31, Unsourced, T1, "IFEU-FI-2024"),
        (2025, 1, 1, Closed, T1, "IFEU-FI-2025"),
        (2025, 4, 18, Closed, T1, "IFEU-FI-2025"),
        (2025, 4, 21, Closed, T1, "IFEU-FI-2025"),
        (2025, 5, 26, Closed, T1, "IFEU-FI-2025"),
        (2025, 8, 25, Closed, T1, "IFEU-FI-2025"),
        (2025, 12, 24, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "IFEU-XMAS-2025"),
        (2025, 12, 25, Closed, T1, "IFEU-FI-2025"),
        (2025, 12, 26, Closed, T1, "IFEU-XMAS-2025"),
        (2025, 12, 31, EarlyClose { close_ssm: CLOSE_1215_SSM }, T1, "IFEU-XMAS-2025"),
        (2026, 1, 1, Closed, T1, "IFEU-FI-2026"),
        (2026, 4, 3, Closed, T1, "IFEU-FI-2026"),
        (2026, 4, 6, Closed, T1, "IFEU-FI-2026"),
        (2026, 5, 4, Closed, T1, "IFEU-FI-2026"),
        (2026, 5, 25, Closed, T1, "IFEU-FI-2026"),
        (2026, 8, 31, Closed, T1, "IFEU-FI-2026"),
        (2026, 12, 24, Unsourced, T1, "IFEU-FI-2026"),
        (2026, 12, 25, Closed, T1, "IFEU-FI-2026"),
        (2026, 12, 28, Closed, T1, "IFEU-FI-2026"),
        (2026, 12, 31, Unsourced, T1, "IFEU-FI-2026"),
    ],
};
