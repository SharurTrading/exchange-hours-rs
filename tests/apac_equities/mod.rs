// SPDX-License-Identifier: MIT-0

//! APAC cash-equity integration suite, split by current data, history, and wiring.

mod bulk;
mod current;
mod history_east_asia;
mod history_india;
mod history_oceania_japan;
mod history_singapore_korea_taiwan;
mod history_southeast_asia;

mod prelude {
    pub(super) use chrono::Duration;
    pub(super) use chrono_tz::{Asia, Australia, Pacific, Tz};
    pub(super) use exchange_hours::{
        CalendarResolution, Exchange, MarketHours, SessionKind, SessionState,
        calendar_for_exchange, hours_for_apac_equities, hours_for_exchange,
        hours_for_exchange_as_of, hours_map_apac_equities,
    };

    pub(super) use crate::support::local;

    pub(super) fn cutover_sides(
        exchange: Exchange,
        tz: Tz,
        date: (i32, u32, u32),
    ) -> (MarketHours, MarketHours) {
        let midnight = local(tz, date, (0, 0, 0));
        (
            hours_for_exchange_as_of(exchange, midnight - Duration::nanoseconds(1)),
            hours_for_exchange_as_of(exchange, midnight),
        )
    }

    pub(super) fn assert_weekend_closed(exchange: Exchange, tz: Tz) {
        let saturday = local(tz, (2026, 8, 22), (11, 0, 0));
        assert!(!hours_for_exchange(exchange).is_open(saturday));
    }
}
