// SPDX-License-Identifier: MIT-0

//! Date-aware behavior at Chrono's representational edges.

use super::prelude::*;

#[test]
fn date_aware_queries_are_total_at_chrono_bounds() {
    for &exchange in Exchange::ALL {
        let calendar = calendar_for_exchange(exchange);
        for instant in [DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC] {
            let result = std::panic::catch_unwind(|| {
                let snapshot = hours_for_exchange(exchange, instant);
                let fixed_open = snapshot.is_open(instant);
                let fixed_bounds = session_bounds(&snapshot, instant);
                let _fixed_next = next_session_after(&snapshot, instant);
                let _fixed_daily = candle_end(&snapshot, instant, CalendarResolution::Daily);
                let _fixed_start = candle_start(&snapshot, instant, CalendarResolution::Monthly);
                let _fixed_seconds = candle_end(&snapshot, instant, CalendarResolution::Seconds(1));
                let _fixed_maintenance = snapshot.is_maintenance(instant);
                let _fixed_closed =
                    snapshot.is_closed_all_day_at(instant, chrono_tz::UTC, SessionKind::Both);
                let open = calendar.is_open(instant);
                let bounds = calendar.session_bounds(instant);
                let _next = calendar.next_session_after(instant);
                let _daily = calendar.candle_end(instant, CalendarResolution::Daily);
                let _weekly = calendar.candle_end(instant, CalendarResolution::Weekly);
                let _monthly = calendar.candle_end(instant, CalendarResolution::Monthly);
                let _start = calendar.candle_start(instant, CalendarResolution::Monthly);
                let _seconds = calendar.candle_end(instant, CalendarResolution::Seconds(1));
                let _maintenance = calendar.is_maintenance(instant);
                let _closed =
                    calendar.is_closed_all_day_at(instant, chrono_tz::UTC, SessionKind::Both);
                let _week = calendar.normal_week_open_seconds_containing(instant);

                (fixed_open, fixed_bounds, open, bounds)
            });
            let (fixed_open, fixed_bounds, open, bounds) = result
                .unwrap_or_else(|_| panic!("{exchange:?} panicked at chrono bound {instant}"));
            assert_eq!(
                fixed_open,
                fixed_bounds.is_some_and(|(start, end)| start <= instant && instant < end),
                "{exchange:?} fixed query fence failed at {instant}"
            );
            assert_eq!(
                open,
                bounds.is_some_and(|(start, end)| start <= instant && instant < end),
                "{exchange:?} calendar query fence failed at {instant}"
            );
        }
    }
}

#[test]
fn synthetic_always_open_utc_profile_has_exact_chrono_edge_sessions() {
    let calendar = calendar_for_exchange(Exchange::Unknown);
    let fixed = hours_for_exchange(
        Exchange::Unknown,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let minimum = DateTime::<Utc>::MIN_UTC;
    let next_midnight = minimum
        .checked_add_signed(Duration::days(1))
        .expect("one day after chrono minimum is representable");

    assert!(fixed.is_open(minimum));
    assert!(calendar.is_open(minimum));
    assert_eq!(
        session_bounds(&fixed, minimum),
        Some((minimum, next_midnight))
    );
    assert_eq!(
        calendar.session_bounds(minimum),
        Some((minimum, next_midnight))
    );
    for resolution in [
        CalendarResolution::Daily,
        CalendarResolution::Weekly,
        CalendarResolution::Monthly,
    ] {
        assert_eq!(candle_start(&fixed, minimum, resolution), None);
        assert_eq!(candle_end(&fixed, minimum, resolution), None);
        assert_eq!(calendar.candle_start(minimum, resolution), None);
        assert_eq!(calendar.candle_end(minimum, resolution), None);
    }

    let near_maximum = DateTime::<Utc>::MAX_UTC
        .checked_sub_signed(Duration::days(1))
        .expect("one day before chrono maximum is representable");
    assert!(fixed.is_open(near_maximum));
    assert!(calendar.is_open(near_maximum));
}

#[test]
fn negative_offset_scan_keeps_the_first_session_at_chrono_minimum() {
    let minimum = DateTime::<Utc>::MIN_UTC;
    let day = NaiveDate::MIN;
    let open = America::New_York
        .from_local_datetime(
            &day.and_hms_opt(9, 30, 0)
                .expect("NYSE open is representable on Chrono's minimum date"),
        )
        .single()
        .expect("NYSE minimum-date open has one local mapping")
        .with_timezone(&Utc);
    let close = America::New_York
        .from_local_datetime(
            &day.and_hms_opt(16, 0, 0)
                .expect("NYSE close is representable on Chrono's minimum date"),
        )
        .single()
        .expect("NYSE minimum-date close has one local mapping")
        .with_timezone(&Utc);
    let expected = Some((open, close));
    let fixed = hours_for_exchange(Exchange::Nyse, minimum);
    let calendar = calendar_for_exchange(Exchange::Nyse);

    assert_eq!(next_session_after(&fixed, minimum), expected);
    assert_eq!(session_bounds(&fixed, minimum), expected);
    assert_eq!(calendar.next_session_after(minimum), expected);
    assert_eq!(calendar.session_bounds(minimum), expected);
    assert_eq!(
        candle_end(&fixed, minimum, CalendarResolution::Daily),
        Some(close)
    );
    assert_eq!(
        calendar.candle_end(minimum, CalendarResolution::Daily),
        Some(close)
    );
}

#[test]
fn maximum_hour_resolution_clamps_without_losing_a_bar() {
    let tz = America::Toronto;
    let instant = local(tz, (2026, 8, 19), (10, 12, 0));
    let close = local(tz, (2026, 8, 19), (16, 0, 0));
    let resolution = CalendarResolution::Hours(u32::MAX);
    let fixed = hours_for_exchange(Exchange::Tsx, instant);
    let calendar = calendar_for_exchange(Exchange::Tsx);

    assert_eq!(candle_start(&fixed, instant, resolution), Some(instant));
    assert_eq!(candle_end(&fixed, instant, resolution), Some(close));
    assert_eq!(calendar.candle_start(instant, resolution), Some(instant));
    assert_eq!(calendar.candle_end(instant, resolution), Some(close));
}

#[test]
fn dynamic_period_walks_keep_the_last_close_near_chrono_maximum() {
    let calendar = calendar_for_exchange(Exchange::Nyse);
    let maximum = DateTime::<Utc>::MAX_UTC;
    let max_day = maximum.date_naive();
    let final_week_start = max_day
        .checked_sub_signed(Duration::days(i64::from(
            max_day.weekday().num_days_from_monday(),
        )))
        .expect("the final ISO week starts inside chrono's range");
    let instant = Utc.from_utc_datetime(
        &final_week_start
            .and_hms_opt(0, 0, 0)
            .expect("midnight is representable"),
    );

    let mut last_close = calendar
        .candle_end(instant, CalendarResolution::Daily)
        .expect("the final representable week has a daily close");
    for _ in 0..8 {
        let Some(probe) = last_close.checked_add_signed(Duration::nanoseconds(1)) else {
            break;
        };
        let Some(next) = calendar.candle_end(probe, CalendarResolution::Daily) else {
            break;
        };
        assert!(next > last_close);
        last_close = next;
    }
    assert!(
        last_close
            .checked_add_signed(Duration::nanoseconds(1))
            .is_none_or(|probe| calendar
                .candle_end(probe, CalendarResolution::Daily)
                .is_none())
    );

    assert_eq!(
        calendar.candle_end(instant, CalendarResolution::Weekly),
        Some(last_close)
    );
    assert_eq!(
        calendar.candle_end(instant, CalendarResolution::Monthly),
        Some(last_close)
    );
}
