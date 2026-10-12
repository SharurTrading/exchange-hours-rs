// SPDX-License-Identifier: MIT-0

//! OANDA TRY-pair holiday audit, 2026-10-10..2026-10-30.
//! Evidence: `docs/evidence/oanda_try_fx.md`.

use super::{HolidayTable, holidays};

/// The Forex category's published changes within the witnessed TRY grid.
// Evidence: docs/evidence/oanda_try_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 10, 10) ..= (2026, 10, 30)],
    rows: [],
};
