// SPDX-License-Identifier: MIT-0

//! OANDA CFD holidays, 2026-10-10..2026-10-30. Evidence: `docs/evidence/oanda_singapore30_cfd.md`.
use super::{HolidayTable, holidays};

/// The current broker publication window, with no exceptions for this scope.
// Evidence: docs/evidence/oanda_singapore30_cfd.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 10, 10) ..= (2026, 10, 30)],
    rows: [],
};
