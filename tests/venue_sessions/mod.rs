// SPDX-License-Identifier: MIT-0

//! Normal-week venue fixtures organized by product family and query behavior.

mod always_open_and_cross_venue;
mod bounds_and_serde;
mod candle_starts_and_profile_adapters;
mod cboe_options;
mod cme_cbot;
mod cme_families;
mod commodities;
mod equity_corrections;
mod eurex_ice;
mod finra_trfs;
mod international_products;
mod intraday_and_monthly_candles;
mod kind_aware_candles;
mod named_profiles;
mod nasdaq;
mod sgx_cfe;
mod sunday_wraps;
mod us_equity_history;
mod verified_corrections;

mod prelude {
    pub(super) use chrono::{DateTime, Datelike, TimeZone, Utc};
    pub(super) use chrono_tz::{America, Asia, Europe, US};
    pub(super) use exchange_hours::*;

    pub(super) fn zoned(
        tz: chrono_tz::Tz,
        date: (i32, u32, u32),
        time: (u32, u32, u32),
    ) -> DateTime<Utc> {
        tz.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("valid zoned instant")
            .with_timezone(&Utc)
    }

    pub(super) fn utc(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
        zoned(chrono_tz::UTC, date, time)
    }

    pub(super) fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
        zoned(US::Central, date, time)
    }

    pub(super) fn et(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
        zoned(America::New_York, date, time)
    }

    pub(super) fn cet(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
        zoned(Europe::Berlin, date, time)
    }

    pub(super) fn lon(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
        zoned(Europe::London, date, time)
    }

    pub(super) fn sgt(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
        zoned(Asia::Singapore, date, time)
    }
}
