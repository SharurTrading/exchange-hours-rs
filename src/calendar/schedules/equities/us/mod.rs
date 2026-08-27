// SPDX-License-Identifier: MIT-0

//! US cash-equity and options schedules, all in `America/New_York`.
//!
//! Venue profiles are deliberately distinct even when they borrow the same
//! rule slices. That keeps a future venue-specific amendment local.
//! Point-in-time selection is owned by each venue module's selector; the
//! preset router dispatches every venue to one.

mod ats;
mod cboe;
mod equities;
mod history;
mod independent;
mod nyse;
mod options;
mod trfs;

pub(crate) use ats::{blue_ocean_profile_at, iex_profile_at};
pub(crate) use cboe::{byx_profile_at, bzx_profile_at, edga_profile_at, edgx_profile_at};
pub(crate) use history::{
    memx_profile_at, miax_pearl_profile_at, nasdaq_bx_profile_at, nasdaq_profile_at,
    nasdaq_psx_profile_at,
};
pub(crate) use independent::{ltse_profile_at, twenty_four_x_profile_at, txse_profile_at};
pub(crate) use nyse::{
    nyse_american_profile_at, nyse_arca_profile_at, nyse_national_profile_at, nyse_profile_at,
    nyse_texas_profile_at,
};
pub(crate) use options::{
    box_options_profile_at, bzx_options_profile_at, c1_profile_at, c2_options_profile_at,
    edgx_options_profile_at, memx_options_profile_at, miax_emerald_options_profile_at,
    miax_options_profile_at, miax_pearl_options_profile_at, miax_sapphire_options_profile_at,
    nasdaq_bx_options_profile_at, nasdaq_gemx_profile_at, nasdaq_ise_profile_at,
    nasdaq_mrx_profile_at, nasdaq_nom_profile_at, nasdaq_phlx_profile_at,
    nyse_american_options_profile_at, nyse_arca_options_profile_at,
};
pub(crate) use trfs::{
    carteret_profile_at as finra_trf_carteret_profile_at,
    chicago_profile_at as finra_trf_chicago_profile_at,
    nyse_profile_at as finra_trf_nyse_profile_at,
};

pub(crate) use super::StaticHoursProfile;
