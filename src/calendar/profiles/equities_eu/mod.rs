// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! EU equities venue tables, grouped by operator family.
//!
//! The continental default is a 09:00–17:30 local continuous session
//! ([`REG_0900_1730`]), but the venues genuinely diverge around it: London
//! runs 08:00–16:30, SIX ends continuous trading at 17:20, Euronext Dublin at
//! 17:28, Nasdaq Stockholm at 17:25, Copenhagen at 16:55, and Helsinki trades
//! 10:00–18:25 in its own EET zone (the Nordic books synchronise on CET).
//! Post-close price-formation windows differ too: Euronext, BME, Xetra,
//! Vienna, and SIX run a trading-at-last/trade-at-close session after the
//! closing auction, LSE runs its Closing Price Crossing, and the Nordics do
//! not (except Copenhagen's optional trade-at-close).
//!
//! Auction windows are modeled as **extended** sessions rather than as part of
//! the regular session: they are periods when the venue is doing something but
//! continuous trading is not open, which is exactly the regular/extended split.
//!
//! Where a venue is not individually cited, hours are verified against FESE,
//! "Regular Trading Hours CET in 2025"
//! (<https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf>), which
//! tabulates continuous-trading and auction windows for every European
//! exchange in local time. Submodules carry the per-venue citations next to
//! their tables.

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

mod euronext;
mod german_swiss;
mod lse;
mod nordics;

pub(crate) use euronext::{
    EURONEXT_AMS_PROFILE, EURONEXT_BRU_PROFILE, EURONEXT_DUB_PROFILE, EURONEXT_LIS_PROFILE,
    EURONEXT_MIL_PROFILE, EURONEXT_PARIS_PROFILE,
};
pub(crate) use german_swiss::{BME_PROFILE, SIX_PROFILE, VIENNA_PROFILE, XETRA_PROFILE};
pub(crate) use lse::LSE_PROFILE;
pub(crate) use nordics::{NASDAQ_CPH_PROFILE, NASDAQ_HEL_PROFILE, NASDAQ_STO_PROFILE};

/// The shared continental continuous session, 09:00–17:30 local, used by the
/// Euronext family (except Dublin) and the Deutsche Börse / SIX Group venues
/// (except SIX itself).
static REG_0900_1730: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 30 * 60,
}];
