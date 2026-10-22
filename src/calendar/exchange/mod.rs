// SPDX-License-Identifier: MIT-0

//! Venue identity. One table generates [`Exchange`], [`Exchange::ALL`], and
//! [`Exchange::as_str`]; exhaustive routing catches incomplete additions.

mod define;
mod name;
mod table;

pub use name::ParseExchangeError;
pub use table::Exchange;
