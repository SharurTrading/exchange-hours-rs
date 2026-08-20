// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The static venue profile tables and the one adapter that turns them into a
//! [`MarketHours`] value.
//!
//! These are `static` rather than constructed per call so that
//! [`MarketHours::regular`] / [`MarketHours::extended`] can borrow them through
//! `Cow::Borrowed` — building a profile allocates nothing. Several tables are
//! shared by many venues on purpose (all Reg NMS equities venues quote the same
//! 09:30–16:00 ET regular session); a venue that merely *coincides* with another
//! today still gets its own named profile, so a future divergence is a one-line
//! edit rather than an untangling.
//!
//! Tables are grouped by product family, one submodule each, because that is the
//! axis along which venue hours actually change together.

use std::borrow::Cow;

use chrono_tz::Tz;

use super::{Exchange, MarketHours, SessionRule};

mod equities_eu;
mod equities_us;
mod futures_intl;
mod futures_us;

pub use equities_us::{
    BLUE_OCEAN_EXTENDED, NYSE_TEXAS_EXTENDED, US_EQUITY_EXTENDED, US_EQUITY_REGULAR,
};

pub(crate) use equities_eu::{
    BME_PROFILE, EURONEXT_AMS_PROFILE, EURONEXT_BRU_PROFILE, EURONEXT_DUB_PROFILE,
    EURONEXT_LIS_PROFILE, EURONEXT_MIL_PROFILE, EURONEXT_PARIS_PROFILE, LSE_PROFILE,
    NASDAQ_CPH_PROFILE, NASDAQ_HEL_PROFILE, NASDAQ_STO_PROFILE, SIX_PROFILE, VIENNA_PROFILE,
    XETRA_PROFILE,
};
pub(crate) use equities_us::{
    BLUE_OCEAN_PROFILE, C1_PROFILE_POST_2024_08_26, C1_PROFILE_PRE_2024_08_26, FINRA_TRF_PROFILE,
    IEX_PROFILE_POST2015, IEX_PROFILE_PRE2015, IQX_PROFILE, NYSE_PROFILE, NYSE_TEXAS_PROFILE,
    US_EQUITIES_PROFILE, US_EQUITY_EARLY_0700_PROFILE, US_OPTIONS_DEFAULT_PROFILE,
};
pub(crate) use futures_intl::{
    ABU_DHABI_01_23_PROFILE, EEX_PROFILE, ENDEX_01_23_PROFILE, EUREX_ASIAN, EUREX_PROFILE_NO_ASIAN,
    EUREX_PROFILE_WITH_ASIAN, EUREX_REGULAR, ICE_CANADA_PROFILE, ICE_EU_LONDON_01_23_PROFILE,
    ICE_WRAP_20_18_EXT, SGX_EXTENDED, SGX_REGULAR,
};
pub(crate) use futures_us::{
    CBOT_EXT_POST2013, CBOT_PROFILE_POST2013, CBOT_PROFILE_PRE2013, CBOT_REGULAR_POST2013,
    CFE_EXTENDED, CFE_PROFILE, CFE_PROFILE_PRE_2021_12_06, CFE_PROFILE_PRE2014, CFE_REGULAR,
    CME_EXT_POST2016, CME_PROFILE_POST2016, CME_PROFILE_PRE2016, CME_REGULAR, MAINT_17_16_EXT,
};

/// A venue's normal-week schedule in fully static form.
///
/// Field-identical to the public [`super::FuturesSessionProfile`] by design:
/// this one is the crate-private table type for venues addressed by
/// [`Exchange`], while the public one is addressed by
/// [`super::MarketHoursKey`] and is re-exported to downstream crates.
#[derive(Clone, Copy)]
pub(crate) struct StaticHoursProfile {
    pub(crate) tz: Tz,
    pub(crate) regular: &'static [SessionRule],
    pub(crate) extended: &'static [SessionRule],
    pub(crate) has_daily_close: bool,
    pub(crate) has_weekend_close: bool,
}

/// Copies a static profile into an owned [`MarketHours`] tagged with `exchange`.
///
/// The rule slices are borrowed, not cloned, so this allocates nothing. The
/// `exchange` tag is passed separately because one table backs many venues.
#[inline]
pub(crate) fn from_profile(exchange: Exchange, p: &'static StaticHoursProfile) -> MarketHours {
    MarketHours {
        exchange,
        tz: p.tz,
        regular: Cow::Borrowed(p.regular),
        extended: Cow::Borrowed(p.extended),
        has_daily_close: p.has_daily_close,
        has_weekend_close: p.has_weekend_close,
    }
}
