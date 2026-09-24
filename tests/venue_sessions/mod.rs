// SPDX-License-Identifier: MIT-0

//! Normal-week venue fixtures organized by product family and query behavior.

mod always_open_and_cross_venue;
mod bounds_and_serde;
mod candle_starts_and_profile_adapters;
mod cboe_options;
mod cde_smfe;
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
mod us_independent_exchanges;
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

    /// Asserts a date-aware query refused with the error variant `expected`
    /// maps onto, whatever date inside the error it names.
    ///
    /// This is the shape for a query whose answer depends on a run of days the
    /// caller did not name: `is_closed_trade_date(2025-01-01)` reads the
    /// holiday layer on 2025-01-01 but settles the trade date that opens on
    /// 2024-12-31, so it can refuse naming either day — and with the variant
    /// that applies to *that* day. The variant is the claim; the date inside it
    /// is the query's own business. The claim is always the refusal, never a
    /// `bool`, and never a date the identity would have had to guess.
    pub(super) fn assert_refused_variant(
        answer: &Result<impl core::fmt::Debug, CalendarQueryError>,
        expected: DateCoverage,
        label: &str,
    ) {
        let refused = matches!(
            (expected, answer),
            (
                DateCoverage::BeforeSupportFloor,
                Err(CalendarQueryError::BeforeSupportFloor { .. })
            ) | (
                DateCoverage::OutsideCoveredRange,
                Err(CalendarQueryError::OutsideCoveredRange { .. })
            ) | (
                DateCoverage::UnresolvedGap,
                Err(CalendarQueryError::UnresolvedGap { .. })
            )
        );
        assert!(
            refused,
            "{label} must be refused with the error variant {expected:?} maps onto; \
             got {answer:?}"
        );
    }

    /// Asserts a date-aware query refused the **venue-local day of `instant`**
    /// because that day precedes the permanent support floor (LAW-COVERAGE).
    ///
    /// The era sweeps in this suite probe historical cutovers whose dates lie
    /// before 2025-01-01. The published grid of those eras is still a sourced
    /// fact and the fixed snapshot states it — `hours_for_exchange` /
    /// `hours_for_market_hours_key` — but the date-aware calendar has no
    /// coverage there and must refuse rather than report the refusal as a
    /// closure. The floor is a *local* date, so it is checked against the
    /// identity's own zone, never the UTC day.
    ///
    /// The `Result` is taken by value so each call site stays one expression;
    /// the query helpers return owned payloads (`bool`, `Option<SessionWindow>`,
    /// `Option<NaiveDate>`), and tests are not exempt from
    /// `clippy::needless_pass_by_value`, so the exception is stated here.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "query helpers return owned Results; taking them by value keeps \
                  every call site a single expression"
    )]
    pub(super) fn assert_refuses_before_floor(
        answer: Result<impl core::fmt::Debug, CalendarQueryError>,
        calendar: ExchangeCalendar,
        instant: DateTime<Utc>,
    ) {
        let local_date = instant.with_timezone(&calendar.tz()).date_naive();
        assert!(
            matches!(
                answer,
                Err(CalendarQueryError::BeforeSupportFloor { source, date })
                    if source == calendar.source() && date == local_date
            ),
            "{:?}: {instant} is venue-local {local_date}, which precedes the support \
             floor, so the date-aware surface must refuse it with BeforeSupportFloor; \
             got {answer:?}",
            calendar.source(),
        );
    }

    /// Asserts a date-aware query refused the venue-local day of `instant` with
    /// `expected`, the verdict the identity's own `coverage()` reports for it.
    ///
    /// Post-floor refusals split in two: [`DateCoverage::OutsideCoveredRange`]
    /// (the identity ships no holiday table and claims none, its weekday profile
    /// is carried there, or it declares a phase-level gap) and
    /// [`DateCoverage::UnresolvedGap`] (an audited window withholds the date as
    /// `Unsourced`). Both map one-to-one onto a [`CalendarQueryError`] variant,
    /// so the caller states the verdict it expects and this asserts the query
    /// answered exactly that — never a `bool`, and never a different variant.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "query helpers return owned Results; taking them by value keeps \
                  every call site a single expression"
    )]
    pub(super) fn assert_refused(
        answer: Result<impl core::fmt::Debug, CalendarQueryError>,
        expected: DateCoverage,
        calendar: ExchangeCalendar,
        instant: DateTime<Utc>,
    ) {
        let local_date = instant.with_timezone(&calendar.tz()).date_naive();
        let reported = calendar.coverage().coverage_on(local_date);
        assert_eq!(
            reported,
            expected,
            "{:?}: the fixture's expected verdict must be the identity's own for \
             venue-local {local_date}",
            calendar.source(),
        );
        let refused = matches!(
            (expected, &answer),
            (
                DateCoverage::BeforeSupportFloor,
                Err(CalendarQueryError::BeforeSupportFloor { .. })
            ) | (
                DateCoverage::OutsideCoveredRange,
                Err(CalendarQueryError::OutsideCoveredRange { .. })
            ) | (
                DateCoverage::UnresolvedGap,
                Err(CalendarQueryError::UnresolvedGap { .. })
            )
        );
        assert!(
            refused,
            "{:?}: {instant} is venue-local {local_date}, which this identity covers \
             as {expected:?}, so the date-aware surface must refuse it with the \
             matching CalendarQueryError; got {answer:?}",
            calendar.source(),
        );
    }
}

mod holidays_cme_venues;
mod holidays_coinbase_derivatives;
