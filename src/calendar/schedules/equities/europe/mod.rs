// SPDX-License-Identifier: MIT-0

//! European cash-equity schedules by operator or venue.
//!
//! FESE's consolidated 2025 trading-hours table is a secondary corroborating
//! source for continuous and auction windows represented in these venue-owned
//! modules:
//! <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf>.

pub(crate) mod bist;
pub(crate) mod bme;
pub(crate) mod euronext;
pub(crate) mod lse;
pub(crate) mod nasdaq_nordics;
pub(crate) mod six;
pub(crate) mod vienna;
pub(crate) mod xetra;
