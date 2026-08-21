// SPDX-License-Identifier: MIT-0

//! Intraday and monthly candle-boundary behavior.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Monthly candle boundaries + MarketHoursKey -> MarketHours resolver.
// ---------------------------------------------------------------------------

#[test]
fn intraday_bar_ends_at_the_daily_close_not_the_reopen() {
    // CME (Both): the electronic day closes 16:00 CT with a maintenance break
    // to 17:00. The day's last intraday bar clamps to the 16:00 close itself —
    // closes are end-exclusive boundaries, and a bar whose end sat at the
    // 17:00 reopen would claim the closed hour as bar time. (V1 snapped this
    // end to the reopen; removed in 0.2.0.)
    let h = hours_for_exchange(Exchange::Cme);
    assert_eq!(
        candle_end(
            &h,
            ct((2026, 4, 20), (15, 58, 0)),
            CalendarResolution::Minutes(5)
        ),
        Some(ct((2026, 4, 20), (16, 0, 0))),
        "last bar of the day ends at the 16:00 CT close"
    );
    // A large interval clamps the same way: at 14:00 CT the enclosing session
    // (for Both) is RTH, so a 4h bar truncates at its 15:15 CT close rather
    // than running to 18:00.
    assert_eq!(
        candle_end(
            &h,
            ct((2026, 4, 20), (14, 0, 0)),
            CalendarResolution::Hours(4)
        ),
        Some(ct((2026, 4, 20), (15, 15, 0))),
        "a 4h bar is truncated at the enclosing session close"
    );
    // From inside the 15:30-16:00 short window the same 4h bar clamps at the
    // 16:00 daily close — never at the 17:00 reopen.
    assert_eq!(
        candle_end(
            &h,
            ct((2026, 4, 20), (15, 40, 0)),
            CalendarResolution::Hours(4)
        ),
        Some(ct((2026, 4, 20), (16, 0, 0))),
        "a bar in the short window ends at the daily close, not the reopen"
    );
}

#[test]
fn intraday_bar_queried_inside_maintenance_anchors_at_the_reopen() {
    // From inside the 16:00-17:00 CT break the containing session is the next
    // one, so the bar anchors at the 17:00 reopen and steps from there.
    let h = hours_for_exchange(Exchange::Cme);
    assert_eq!(
        candle_end(
            &h,
            ct((2026, 4, 20), (16, 30, 0)),
            CalendarResolution::Minutes(5)
        ),
        Some(ct((2026, 4, 20), (17, 5, 0))),
        "a query inside the break belongs to the next session's first bar"
    );
}

#[test]
fn daily_candles_ignore_phase_handoffs_that_remain_open() {
    let dubai = |date: (i32, u32, u32), time: (u32, u32, u32)| {
        Asia::Dubai
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("valid Dubai instant")
            .with_timezone(&Utc)
    };
    let cases = [
        (
            Exchange::Cfe,
            ct((2026, 8, 17), (12, 0, 0)),
            ct((2026, 8, 17), (16, 0, 0)),
        ),
        (
            Exchange::Iceeu,
            et((2026, 8, 17), (12, 0, 0)),
            et((2026, 8, 17), (18, 0, 0)),
        ),
        (
            Exchange::IceAbuDhabi,
            dubai((2026, 8, 17), (12, 0, 0)),
            dubai((2026, 8, 18), (2, 0, 0)),
        ),
        (
            Exchange::Sgx,
            sgt((2026, 8, 17), (12, 0, 0)),
            sgt((2026, 8, 17), (18, 0, 0)),
        ),
    ];

    for (exchange, instant, expected_close) in cases {
        let hours = hours_for_exchange(exchange);
        let close = candle_end(&hours, instant, CalendarResolution::Daily);
        assert_eq!(close, Some(expected_close), "{exchange:?}");
        assert!(!hours.is_open(expected_close), "{exchange:?}");
    }
}

#[test]
fn candle_end_monthly_returns_last_close_of_month() {
    // A mid-January instant resolves to January's final daily close: the
    // returned instant is in January (exchange-local) and the very next daily
    // close after it falls in February.
    let h = hours_for_exchange(Exchange::Cme);
    let mid_jan = ct((2026, 1, 15), (12, 0, 0));

    let close =
        candle_end(&h, mid_jan, CalendarResolution::Monthly).expect("January has daily closes");
    assert!(
        close > mid_jan,
        "monthly close must be after the input instant"
    );
    assert_eq!(
        close.with_timezone(&US::Central).month(),
        1,
        "monthly close stays in January (exchange-local)"
    );

    let next =
        candle_end(&h, close, CalendarResolution::Daily).expect("trading continues in February");
    assert_eq!(
        next.with_timezone(&US::Central).month(),
        2,
        "the close after the monthly boundary is in February"
    );
}

#[test]
fn candle_end_monthly_on_final_trading_day_returns_that_close() {
    // On a month's last trading day, the monthly boundary coincides with that
    // day's daily close. Friday 2026-01-30 is January's last trading day
    // (2026-01-31 is a Saturday).
    let h = hours_for_exchange(Exchange::Cme);
    let on_last_day = ct((2026, 1, 30), (9, 0, 0));
    assert_eq!(
        candle_end(&h, on_last_day, CalendarResolution::Monthly),
        candle_end(&h, on_last_day, CalendarResolution::Daily),
        "monthly close equals the daily close of the month's final trading day"
    );
}

#[test]
fn calendar_resolution_monthly_serde_round_trip() {
    let json = serde_json::to_string(&CalendarResolution::Monthly).unwrap();
    let rt: CalendarResolution = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, CalendarResolution::Monthly);
}

#[test]
fn candle_end_monthly_year_boundary() {
    // A late-December instant resolves to December's final close, not January:
    // the boundary stays in December 2026 and the next close rolls to 2027.
    let h = hours_for_exchange(Exchange::Cme);
    let late_dec = ct((2026, 12, 20), (12, 0, 0));

    let close =
        candle_end(&h, late_dec, CalendarResolution::Monthly).expect("December has daily closes");
    let local = close.with_timezone(&US::Central);
    assert_eq!(local.month(), 12, "boundary stays in December");
    assert_eq!(local.year(), 2026, "boundary stays in 2026");

    let next = candle_end(&h, close, CalendarResolution::Daily)
        .expect("trading continues in January 2027");
    let next_local = next.with_timezone(&US::Central);
    assert_eq!(next_local.month(), 1, "next close rolls into January");
    assert_eq!(next_local.year(), 2027, "next close rolls into 2027");
}

#[test]
fn candle_end_monthly_mid_month_is_idempotent_within_month() {
    // Two instants in the same month resolve to the same monthly boundary.
    let h = hours_for_exchange(Exchange::Cme);
    let early = ct((2026, 3, 3), (9, 0, 0));
    let later = ct((2026, 3, 25), (14, 0, 0));
    assert_eq!(
        candle_end(&h, early, CalendarResolution::Monthly),
        candle_end(&h, later, CalendarResolution::Monthly),
        "any two instants in March share one monthly boundary"
    );
}
