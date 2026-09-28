// SPDX-License-Identifier: MIT-0

//! Per-identity holiday suites for the three served cash-equity venues.

mod hkex;
mod six;
mod xetra;

mod prelude {
    pub(super) use chrono::{DateTime, Datelike as _, NaiveDate, Utc};
    pub(super) use chrono_tz::{Asia, Europe, Tz};
    pub(super) use exchange_hours::{
        CalendarQueryError, EvidenceTier, Exchange, ExchangeCalendar, HolidayKind, SessionKind,
        calendar_for_exchange,
    };

    pub(super) use crate::support::local;

    /// Every closure row of one identity's table, asserted date by date.
    ///
    /// A closure whose session walk must read a date before the audited window
    /// refuses instead of answering (the shape the Eurex suite fences for
    /// 2025-01-01); the walk's reach differs per identity, so both verdicts —
    /// a closed answer and the window-boundary refusal naming the
    /// predecessor — are accepted, and anything else fails.
    pub(super) fn assert_closed(
        label: &str,
        calendar: ExchangeCalendar,
        tz: Tz,
        closures: &[NaiveDate],
    ) {
        for date in closures {
            let holiday = calendar
                .holiday_on(*date)
                .unwrap_or_else(|| panic!("{label}: {date} ships a closure row"));
            assert_eq!(
                holiday.kind(),
                HolidayKind::Closed,
                "{label} must be closed on {date}"
            );
            assert_eq!(holiday.tier(), EvidenceTier::T1, "{label} on {date}");
            let noon = local(tz, (date.year(), date.month(), date.day()), (12, 0, 0));
            match calendar.is_open(noon) {
                Ok(open) => {
                    assert!(
                        !open,
                        "{label} at {noon} must be closed: the row deletes the day"
                    );
                }
                Err(CalendarQueryError::OutsideCoveredRange { .. }) => {
                    // The session walk read the unaudited predecessor; the
                    // row itself is still fenced by `holiday_on` above.
                }
                Err(error) => {
                    panic!("{label} at {noon}: unexpected query error {error:?}");
                }
            }
            match calendar.is_closed_trade_date(*date, SessionKind::Both) {
                Ok(closed) => {
                    assert!(closed, "{label}: {date} must answer as a closed trade date");
                }
                Err(CalendarQueryError::OutsideCoveredRange { .. }) => {}
                Err(error) => {
                    panic!("{label} on {date}: unexpected trade-date error {error:?}");
                }
            }
        }
    }

    /// An early close's exact instant, fenced from both directions: the row
    /// carries the printed second and the envelope shuts there end-exclusive.
    pub(super) fn assert_early_close(
        label: &str,
        calendar: ExchangeCalendar,
        tz: Tz,
        date: (i32, u32, u32),
        close_ssm: u32,
        document: &str,
    ) {
        let holiday = calendar
            .holiday_on(NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("valid date"))
            .unwrap_or_else(|| panic!("{label}: {date:?} ships an early-close row"));
        assert_eq!(
            holiday.kind(),
            HolidayKind::EarlyClose { close_ssm },
            "{label} on {date:?}"
        );
        assert_eq!(holiday.document_id(), document, "{label} on {date:?}");
        let before = ssm_instant(tz, date, close_ssm - 1);
        let at = ssm_instant(tz, date, close_ssm);
        assert!(
            calendar
                .is_open(before)
                .expect("the coverage contract must answer a covered date"),
            "{label}: one second before the early close at {before} must be open"
        );
        assert!(
            !calendar
                .is_open(at)
                .expect("the coverage contract must answer a covered date"),
            "{label}: the early close at {at} is end-exclusive, so it must be closed"
        );
    }

    /// A venue-local instant from seconds since midnight.
    fn ssm_instant(tz: Tz, date: (i32, u32, u32), ssm: u32) -> DateTime<Utc> {
        local(tz, date, (ssm / 3_600, (ssm % 3_600) / 60, ssm % 60))
    }

    /// The refusal a pre-floor date must produce.
    pub(super) const PRE_FLOOR_REFUSAL: fn(&CalendarQueryError) -> bool =
        |error| matches!(error, CalendarQueryError::BeforeSupportFloor { .. });
}
