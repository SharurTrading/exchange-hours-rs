// SPDX-License-Identifier: MIT-0

//! Global cash-equity integration suite, split by current data, history, and wiring.

mod bulk;
mod current;
mod history;

mod prelude {
    pub(super) use chrono::Duration;
    pub(super) use chrono_tz::{Africa, America, Asia, Europe, Tz};
    pub(super) use exchange_hours::{
        Exchange, MarketHours, hours_for_exchange, hours_for_global_equities,
        hours_map_global_equities,
    };

    pub(super) use crate::support::local;
}
