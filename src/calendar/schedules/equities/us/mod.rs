// SPDX-License-Identifier: MIT-0

//! US cash-equity and options schedules, all in `America/New_York`.
//!
//! Venue profiles are deliberately distinct even when they borrow the same
//! rule slices. That keeps a future venue-specific amendment local.
//! Point-in-time pairs are selected by
//! [`hours_for_exchange_as_of`](crate::hours_for_exchange_as_of); the current
//! preset always names the current venue-owned profile.

mod ats;
mod cboe;
mod equities;
mod history;
mod independent;
mod nyse;
mod options;
mod trfs;

pub(crate) use ats::{BLUE_OCEAN_PROFILE, IEX_PROFILE, blue_ocean_profile_at, iex_profile_at};
pub(crate) use cboe::{
    CBOE_BYX_PROFILE, CBOE_BZX_PROFILE, CBOE_EDGA_PROFILE, CBOE_EDGX_PROFILE, byx_profile_at,
    bzx_profile_at, edga_profile_at, edgx_profile_at,
};
pub(crate) use equities::{
    MEMX_EQ_PROFILE, MIAX_PEARL_EQ_PROFILE, NASDAQ_BX_PROFILE, NASDAQ_PROFILE, NASDAQ_PSX_PROFILE,
};
pub(crate) use history::{
    memx_profile_at, miax_pearl_profile_at, nasdaq_bx_profile_at, nasdaq_profile_at,
    nasdaq_psx_profile_at,
};
pub(crate) use independent::{
    LTSE_PROFILE, TWENTY_FOUR_X_PROFILE, TXSE_PROFILE, ltse_profile_at, twenty_four_x_profile_at,
    txse_profile_at,
};
pub(crate) use nyse::{
    NYSE_AMERICAN_PROFILE, NYSE_ARCA_PROFILE, NYSE_NATIONAL_PROFILE, NYSE_PROFILE,
    NYSE_TEXAS_PROFILE, nyse_american_profile_at, nyse_arca_profile_at, nyse_national_profile_at,
    nyse_profile_at, nyse_texas_profile_at,
};
pub(crate) use options::{
    BOX_OPTIONS_PROFILE, CBOE_BZX_OPTIONS_PROFILE, CBOE_C2_OPTIONS_PROFILE,
    CBOE_EDGX_OPTIONS_PROFILE, CBOE_OPTIONS_C1_PROFILE, MEMX_OPTIONS_PROFILE,
    MIAX_EMERALD_OPTIONS_PROFILE, MIAX_OPTIONS_PROFILE, MIAX_PEARL_OPTIONS_PROFILE,
    MIAX_SAPPHIRE_OPTIONS_PROFILE, NASDAQ_BX_OPTIONS_PROFILE, NASDAQ_GEMX_OPTIONS_PROFILE,
    NASDAQ_ISE_OPTIONS_PROFILE, NASDAQ_MRX_OPTIONS_PROFILE, NASDAQ_NOM_OPTIONS_PROFILE,
    NASDAQ_PHLX_OPTIONS_PROFILE, NYSE_AMERICAN_OPTIONS_PROFILE, NYSE_ARCA_OPTIONS_PROFILE,
    box_options_profile_at, bzx_options_profile_at, c1_profile_at, c2_options_profile_at,
    edgx_options_profile_at, memx_options_profile_at, miax_emerald_options_profile_at,
    miax_options_profile_at, miax_pearl_options_profile_at, miax_sapphire_options_profile_at,
    nasdaq_bx_options_profile_at, nasdaq_gemx_profile_at, nasdaq_ise_profile_at,
    nasdaq_mrx_profile_at, nasdaq_nom_profile_at, nasdaq_phlx_profile_at,
    nyse_american_options_profile_at, nyse_arca_options_profile_at,
};
pub(crate) use trfs::{
    FINRA_TRF_CARTERET_PROFILE, FINRA_TRF_CHICAGO_PROFILE, FINRA_TRF_NYSE_PROFILE,
    carteret_profile_at as finra_trf_carteret_profile_at,
    chicago_profile_at as finra_trf_chicago_profile_at,
    nyse_profile_at as finra_trf_nyse_profile_at,
};

pub(crate) use super::StaticHoursProfile;
