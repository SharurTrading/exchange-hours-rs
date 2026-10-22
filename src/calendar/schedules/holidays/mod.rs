// SPDX-License-Identifier: MIT-0

//! Built-in per-family holiday and early-close tables (LAW-HOLIDAY-SCOPE).
//!
//! A holiday is a change confined to one venue-local trade date or a bounded
//! run of them: a full closure, an early final close, a late first open, or
//! both. It is never a normal-week template edit and never a revision row. The
//! data lives here as static date tables keyed by the crate's own
//! **venue-local trade date**, never by the operator's event date, so a Globex
//! session that opened the previous evening is clipped on the correct civil day.
//!
//! One module per identity owns its rows; `table_for` resolves an identity to
//! its table through a match with no catch-all arm, so a new `Exchange` or
//! `MarketHoursKey` cannot silently acquire or lose a table.
//!
//! `table_for` resolves every published table through the exhaustive routing
//! match, including the three Nordic principal-share calendars. Where it
//! answers `None` the caller's `DayPolicy`
//! overlay remains the only holiday layer. Each table declares the trade-date
//! window it audited, so a date outside that window is "no answer", not
//! "normal".
//!
//! # Layering
//!
//! A resolved table is the **innermost** of three layers and is resolved once
//! per query, when the query context is built:
//!
//! 1. the caller's `SessionExceptionSource` — an explicit `Closed` or
//!    `ReplaceSessions` record wins outright and suppresses the built-in row
//!    for that trade date;
//! 2. this built-in table;
//! 3. the caller's `DayPolicy`, which clips whatever survives.
//!
//! Layers 2 and 3 compose by **tightening**: closures are `OR`, early closes
//! take the `min`, late opens take the `max`. A caller can always make a
//! trading day shorter; a caller can never widen the crate's answer with a
//! `DayPolicy`.

mod asx;
mod asx_spi_200;
mod b3;
mod bme;
mod bme_ibex;
mod borsa_istanbul;
mod builder;
mod cfe;
mod coinbase_derivatives;
mod eurex;
mod eurex_smi;
mod euronext_amsterdam;
mod euronext_brussels;
mod euronext_index_futures;
mod euronext_lisbon;
mod euronext_paris;
pub(crate) mod fences;
mod globex_cryptocurrency;
mod globex_energy;
mod globex_equity_index;
mod globex_fx;
mod globex_grains;
mod globex_interest_rates;
mod globex_livestock;
mod globex_nikkei_225_dollar;
mod hkex;
mod hkex_index_futures;
mod ice_europe_gilts;
mod ice_us;
mod kind;
mod lse;
mod nasdaq;
mod nasdaq_copenhagen;
mod nasdaq_helsinki;
mod nasdaq_stockholm;
mod nse_india;
mod nyse;
mod nzx;
mod oanda_brent;
mod oanda_china50_cfd;
mod oanda_fx;
mod oanda_metals;
mod oanda_singapore30_cfd;
mod oanda_try_fx;
mod oanda_us_bond_cfd;
mod oanda_us_commodity_cfd;
mod oanda_us_index_cfd;
mod routing;
mod sgx_securities;
mod six;
mod sse;
mod table;
mod tadawul;
mod tier;
mod tse;
mod tsx;
mod venues;
mod xetra;

pub(crate) use builder::holidays;
pub use kind::HolidayKind;
pub(crate) use routing::table_for;
pub use table::{Holiday, HolidayCoverage};
pub(crate) use table::{HolidayRow, HolidayTable};
pub use tier::EvidenceTier;
