// SPDX-License-Identifier: MIT-0

//! Caller-supplied trade-date policy and state contracts over the public API.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::{America, Asia, US};
use exchange_hours::{
    CalendarResolution, DayPolicy, Exchange, MarketHoursKey, NoPolicy, SessionKind, SessionState,
    calendar_for_exchange, calendar_for_market_hours_key,
};

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid CT instant")
        .with_timezone(&Utc)
}

fn et(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    America::New_York
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid ET instant")
        .with_timezone(&Utc)
}

fn sgt(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Asia::Singapore
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid SGT instant")
        .with_timezone(&Utc)
}

fn bangkok(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Asia::Bangkok
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid Bangkok instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

struct TestPolicy<'a> {
    closed: &'a [NaiveDate],
    early: Option<(NaiveDate, u32)>,
    late: Option<(NaiveDate, u32)>,
}

impl DayPolicy for TestPolicy<'_> {
    fn is_closed(&self, trade_date: NaiveDate) -> bool {
        self.closed.contains(&trade_date)
    }

    fn early_close_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
        self.early
            .filter(|(date, _ssm)| *date == trade_date)
            .map(|(_date, ssm)| ssm)
    }

    fn late_open_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
        self.late
            .filter(|(date, _ssm)| *date == trade_date)
            .map(|(_date, ssm)| ssm)
    }
}

struct Closed;

impl DayPolicy for Closed {
    fn is_closed(&self, _trade_date: NaiveDate) -> bool {
        true
    }

    fn early_close_ssm(&self, _trade_date: NaiveDate) -> Option<u32> {
        None
    }
}

#[test]
fn calendar_identity_distinguishes_exchanges_from_product_families() {
    let exchange = calendar_for_exchange(Exchange::Cme);
    assert_eq!(exchange.exchange(), Some(Exchange::Cme));
    assert_eq!(exchange.market_hours_key(), None);

    let key = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    assert_eq!(key.exchange(), None);
    assert_eq!(
        key.market_hours_key(),
        Some(MarketHoursKey::GlobexEquityIndex)
    );
}

#[test]
fn no_policy_preserves_the_complete_calendar_surface() {
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let policy = base.with_day_policy(&NoPolicy);

    for instant in [
        ct((2026, 4, 19), (18, 0, 0)),
        ct((2026, 4, 20), (10, 0, 0)),
        ct((2026, 4, 20), (15, 20, 0)),
        ct((2026, 4, 20), (16, 30, 0)),
    ] {
        assert_eq!(policy.hours_at(instant), base.hours_at(instant));
        assert_eq!(policy.is_open(instant), base.is_open(instant));
        assert_eq!(policy.session_bounds(instant), base.session_bounds(instant));
        assert_eq!(policy.session_state(instant), base.session_state(instant));
        assert_eq!(policy.trade_date(instant), base.trade_date(instant));
        assert_eq!(
            policy.candle_end(instant, CalendarResolution::Daily),
            base.candle_end(instant, CalendarResolution::Daily)
        );
    }
}

#[test]
fn closed_trade_date_removes_the_prior_evening_but_not_the_next_trade_date() {
    let monday = day(2026, 4, 20);
    let closed = [monday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let calendar = base.with_day_policy(&policy);

    assert!(!calendar.is_open(ct((2026, 4, 19), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2026, 4, 20), (10, 0, 0))));
    assert!(calendar.is_open(ct((2026, 4, 20), (17, 0, 0))));
    assert!(calendar.is_closed_trade_date(monday, SessionKind::Both));
    assert!(
        !calendar.is_closed_all_day_on(monday, SessionKind::Both),
        "Tuesday's trade date opens during civil Monday"
    );
    assert_eq!(
        calendar.session_bounds(ct((2026, 4, 19), (18, 0, 0))),
        // Sessions now begin at the matching open. Previously this returned the
        // 16:45-17:00 pre-open queue as if it were a fifteen-minute session.
        Some((ct((2026, 4, 20), (17, 0, 0)), ct((2026, 4, 21), (8, 30, 0)),))
    );
}

#[test]
fn a_closed_friday_scan_falls_forward_to_sunday() {
    let friday = day(2026, 4, 24);
    let closed = [friday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);
    assert_eq!(
        calendar.next_session_open_after(ct((2026, 4, 23), (16, 30, 0))),
        Some(ct((2026, 4, 26), (17, 0, 0)))
    );
}

#[test]
fn closed_crypto_monday_rolls_weekend_into_the_following_business_day() {
    let monday = day(2026, 6, 8);
    let tuesday = day(2026, 6, 9);
    let closed = [monday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
        .with_day_policy(&policy);

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (1, 0, 0)),
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert!(
            calendar.is_open(instant),
            "weekend trading vanished at {instant}"
        );
        assert_eq!(calendar.trade_date(instant), Some(tuesday));
    }
    assert!(calendar.is_closed_trade_date(monday, SessionKind::Both));
    assert!(calendar.is_open(ct((2026, 6, 8), (16, 1, 0))));
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 8), (16, 1, 0))),
        Some(tuesday)
    );
    assert_eq!(
        calendar.candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2026, 6, 5), (16, 1, 0)))
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2026, 6, 9), (16, 0, 0)))
    );
    assert_eq!(
        calendar.candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Weekly,),
        Some(ct((2026, 6, 5), (16, 1, 0)))
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Weekly,),
        Some(ct((2026, 6, 12), (16, 0, 0)))
    );

    let closed = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
        .with_day_policy(&Closed);
    assert_eq!(closed.session_bounds(ct((2026, 6, 5), (17, 0, 0))), None);
    assert_eq!(closed.trade_date(ct((2026, 6, 5), (17, 0, 0))), None);
}

#[test]
fn closed_set_trade_date_removes_its_day_and_after_midnight_tail() {
    let wednesday = day(2025, 5, 7);
    let closed = [wednesday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar = calendar_for_exchange(Exchange::SetThailand).with_day_policy(&policy);

    assert!(calendar.is_open_extended(bangkok((2025, 5, 7), (2, 50, 0))));
    assert_eq!(
        calendar.trade_date(bangkok((2025, 5, 7), (2, 50, 0))),
        Some(day(2025, 5, 6))
    );
    assert!(!calendar.is_open(bangkok((2025, 5, 7), (12, 0, 0))));
    assert!(!calendar.is_open(bangkok((2025, 5, 7), (19, 0, 0))));
    assert!(!calendar.is_open(bangkok((2025, 5, 8), (2, 50, 0))));
    assert!(calendar.is_open(bangkok((2025, 5, 8), (10, 0, 0))));
    assert!(calendar.is_closed_trade_date(wednesday, SessionKind::Both));
}

#[test]
fn early_close_clamps_sessions_candles_and_state() {
    let monday = day(2026, 4, 20);
    let policy = TestPolicy {
        closed: &[],
        early: Some((monday, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(calendar.is_open_regular(ct((2026, 4, 20), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2026, 4, 20), (12, 15, 0))));
    assert_eq!(
        calendar.candle_end(ct((2026, 4, 19), (18, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2026, 4, 20), (12, 15, 0)))
    );
    assert_eq!(
        calendar.candle_end(
            ct((2026, 4, 20), (12, 14, 0)),
            CalendarResolution::Minutes(5),
        ),
        Some(ct((2026, 4, 20), (12, 15, 0)))
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (13, 0, 0))),
        SessionState::Closed
    );
}

#[test]
fn early_close_on_the_last_trade_date_clamps_weekly_and_monthly_bars() {
    let friday = day(2026, 5, 29);
    let policy = TestPolicy {
        closed: &[],
        early: Some((friday, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);
    let expected = ct((2026, 5, 29), (12, 15, 0));

    assert_eq!(
        calendar.candle_end(ct((2026, 5, 24), (18, 0, 0)), CalendarResolution::Weekly,),
        Some(expected)
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 5, 1), (10, 0, 0)), CalendarResolution::Monthly,),
        Some(expected)
    );
}

#[test]
fn cme_good_friday_closed_reference_case_uses_caller_data() {
    // CME's 2014 advisory states that Globex had its regular Thursday close,
    // was closed Friday April 18, and resumed normal hours Sunday April 20.
    // This fixture proves the overlay; the crate intentionally ships no
    // holiday calendar.
    // https://www.cmegroup.com/tools-information/lookups/advisories/clearing/files/Chadv14-136.pdf
    let good_friday = day(2014, 4, 18);
    let closed = [good_friday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(!calendar.is_open(ct((2014, 4, 18), (10, 0, 0))));
    assert!(calendar.is_closed_trade_date(good_friday, SessionKind::Both));
    assert!(calendar.is_closed_all_day_on(good_friday, SessionKind::Both));
    assert_eq!(
        calendar.next_session_open_after(ct((2014, 4, 17), (16, 15, 0))),
        Some(ct((2014, 4, 20), (17, 0, 0)))
    );
}

#[test]
fn cme_christmas_eve_and_post_thanksgiving_close_at_12_15() {
    // These are operator-published Globex fixtures. The Thanksgiving source
    // makes the important distinction explicit: Wednesday was normal and the
    // 12:15 CT equity close was Friday, the day after Thanksgiving.
    // https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf
    // https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-christmas-holiday-schedule.pdf
    let post_thanksgiving = day(2015, 11, 27);
    let thanksgiving_policy = TestPolicy {
        closed: &[],
        early: Some((post_thanksgiving, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let thanksgiving = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_day_policy(&thanksgiving_policy);
    assert_eq!(
        thanksgiving.candle_end(ct((2015, 11, 26), (18, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2015, 11, 27), (12, 15, 0)))
    );
    assert!(!thanksgiving.is_open(ct((2015, 11, 27), (12, 15, 0))));

    let christmas_eve = day(2015, 12, 24);
    let christmas_day = day(2015, 12, 25);
    let christmas_closed = [christmas_day];
    let christmas_policy = TestPolicy {
        closed: &christmas_closed,
        early: Some((christmas_eve, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let christmas = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_day_policy(&christmas_policy);
    assert_eq!(
        christmas.candle_end(ct((2015, 12, 23), (18, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2015, 12, 24), (12, 15, 0)))
    );
    assert_eq!(
        christmas.next_session_open_after(ct((2015, 12, 24), (12, 15, 0))),
        Some(ct((2015, 12, 27), (17, 0, 0)))
    );
}

#[test]
fn cme_mlk_and_presidents_day_close_at_noon_then_reopen_at_five() {
    // CME's live 2026 product table gives equities a 12:00 CT holiday close
    // and 17:00 CT reopen on both dates. These are caller-policy fixtures, not
    // a built-in or prospective holiday dataset.
    // https://www.cmegroup.com/trading-hours.html#tradeDate=2026-01-19
    // https://www.cmegroup.com/trading-hours.html#tradeDate=2026-02-16
    for holiday in [day(2026, 1, 19), day(2026, 2, 16)] {
        let policy = TestPolicy {
            closed: &[],
            early: Some((holiday, 12 * 3_600)),
            late: None,
        };
        let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
            .with_day_policy(&policy);
        let previous_day = holiday
            .pred_opt()
            .expect("holiday fixture has a predecessor");

        assert_eq!(
            calendar.candle_end(
                ct(
                    (
                        previous_day.year(),
                        previous_day.month(),
                        previous_day.day()
                    ),
                    (18, 0, 0),
                ),
                CalendarResolution::Daily,
            ),
            Some(ct(
                (holiday.year(), holiday.month(), holiday.day()),
                (12, 0, 0),
            ))
        );
        assert!(!calendar.is_open(ct(
            (holiday.year(), holiday.month(), holiday.day()),
            (12, 0, 0),
        )));
        assert_eq!(
            calendar.next_session_open_after(ct(
                (holiday.year(), holiday.month(), holiday.day()),
                (12, 0, 0),
            )),
            // 16:45 is the pre-open queue, not a session; matching resumes at
            // 17:00 and that is where the next session opens.
            Some(ct(
                (holiday.year(), holiday.month(), holiday.day()),
                (17, 0, 0),
            ))
        );
        assert!(calendar.is_open_extended(ct(
            (holiday.year(), holiday.month(), holiday.day()),
            (17, 0, 0),
        )));
    }
}

#[test]
fn late_open_clips_every_earlier_phase_of_the_trade_date() {
    let monday = day(2026, 4, 20);
    let policy = TestPolicy {
        closed: &[],
        early: None,
        late: Some((monday, 9 * 3_600 + 30 * 60)),
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(!calendar.is_open(ct((2026, 4, 19), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2026, 4, 20), (9, 29, 59))));
    assert!(calendar.is_open_regular(ct((2026, 4, 20), (9, 30, 0))));
    assert_eq!(
        calendar.trade_date(ct((2026, 4, 20), (9, 30, 0))),
        Some(monday)
    );
}

#[test]
fn policy_scans_are_bounded_and_hours_at_is_unmodified() {
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let instant = ct((2026, 4, 20), (10, 0, 0));
    let calendar = base.with_day_policy(&Closed);
    assert_eq!(calendar.session_bounds(instant), None);
    assert_eq!(calendar.next_session_after(instant), None);
    assert_eq!(calendar.hours_at(instant), base.hours_at(instant));
}

#[test]
fn policy_does_not_invent_trade_dates_for_an_always_open_profile() {
    let monday = day(2026, 4, 20);
    let closed = [monday];
    let policy = TestPolicy {
        closed: &closed,
        early: Some((monday, 12 * 3_600)),
        late: Some((monday, 9 * 3_600)),
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::AlwaysOpen).with_day_policy(&policy);

    for instant in [
        ct((2026, 4, 19), (12, 0, 0)),
        ct((2026, 4, 20), (0, 0, 0)),
        ct((2026, 4, 20), (12, 0, 0)),
    ] {
        assert!(calendar.is_open(instant));
        assert_eq!(calendar.trade_date(instant), None);
    }
    assert!(calendar.is_closed_trade_date(monday, SessionKind::Both));
    assert!(!calendar.is_closed_all_day_on(monday, SessionKind::Both));
}

#[test]
fn invalid_policy_seconds_fail_closed_for_that_trade_date() {
    let monday = day(2026, 4, 20);
    let invalid_early = TestPolicy {
        closed: &[],
        early: Some((monday, 86_401)),
        late: None,
    };
    let invalid_late = TestPolicy {
        closed: &[],
        early: None,
        late: Some((monday, 86_400)),
    };
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    for policy in [&invalid_early, &invalid_late] {
        let calendar = base.with_day_policy(policy);
        assert!(!calendar.is_open(ct((2026, 4, 19), (18, 0, 0))));
        assert!(!calendar.is_open(ct((2026, 4, 20), (10, 0, 0))));
        assert!(calendar.is_closed_trade_date(monday, SessionKind::Both));
    }
}

#[test]
fn next_session_scan_includes_day_fourteen_and_excludes_day_fifteen() {
    let calendar = calendar_for_exchange(Exchange::NyseNational);
    assert_eq!(
        calendar.next_session_open_after(et((2018, 5, 7), (7, 0, 0))),
        // NYSE National's 06:30 order-acceptance edge is not a session; the
        // first session opens with matching at 07:00.
        Some(et((2018, 5, 21), (7, 0, 0)))
    );
    assert_eq!(
        calendar.next_session_open_after(et((2018, 5, 6), (7, 0, 0))),
        None,
        "the fifteenth local day is outside the bounded forward scan"
    );
}

#[test]
fn trade_dates_and_states_cover_the_globex_day() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let monday = day(2026, 4, 20);

    assert_eq!(
        calendar.trade_date(ct((2026, 4, 19), (18, 0, 0))),
        Some(monday)
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 4, 20), (10, 0, 0))),
        Some(monday)
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 4, 20), (15, 20, 0))),
        Some(monday)
    );
    assert_eq!(calendar.trade_date(ct((2026, 4, 20), (16, 30, 0))), None);
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (10, 0, 0))),
        SessionState::OpenRegular
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 19), (18, 0, 0))),
        SessionState::OpenExtended
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (15, 20, 0))),
        SessionState::OpenExtended
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (16, 30, 0))),
        SessionState::Maintenance
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 25), (12, 0, 0))),
        SessionState::Closed
    );

    let livestock = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    assert_eq!(
        livestock.trade_date(ct((2026, 4, 20), (9, 0, 0))),
        Some(monday)
    );
    assert_eq!(livestock.trade_date(ct((2026, 4, 25), (9, 0, 0))), None);

    assert_eq!(
        calendar.trade_date(ct((2026, 3, 8), (18, 0, 0))),
        Some(day(2026, 3, 9)),
        "the Central DST transition does not change the Globex trade date"
    );

    let cfe = calendar_for_market_hours_key(MarketHoursKey::CfeVix);
    assert_eq!(
        cfe.trade_date(ct((2026, 4, 19), (16, 30, 0))),
        Some(monday),
        "the Sunday CFE order-entry queue belongs to Monday"
    );
    assert_eq!(
        cfe.trade_date(ct((2026, 4, 20), (16, 50, 0))),
        Some(day(2026, 4, 21)),
        "the Monday-evening CFE queue belongs to Tuesday"
    );

    let sgx = calendar_for_market_hours_key(MarketHoursKey::Sgx);
    assert_eq!(sgx.trade_date(sgt((2026, 4, 20), (10, 0, 0))), Some(monday));
    assert_eq!(
        sgx.trade_date(sgt((2026, 4, 20), (22, 0, 0))),
        Some(day(2026, 4, 21)),
        "the SGX T+1 phase belongs to the following trade date"
    );
}

#[test]
fn session_state_and_trade_date_are_consistent_for_every_key() {
    let mut instant = Utc
        .with_ymd_and_hms(2026, 4, 19, 0, 0, 0)
        .single()
        .expect("fixture must have a valid start");
    let end = instant + Duration::days(7);

    while instant < end {
        for &key in MarketHoursKey::ALL {
            let calendar = calendar_for_market_hours_key(key);
            let state = calendar.session_state(instant);
            assert_eq!(
                calendar.is_maintenance(instant),
                state == SessionState::Maintenance,
                "{key} at {instant}"
            );
            if key == MarketHoursKey::AlwaysOpen {
                assert_eq!(calendar.trade_date(instant), None, "{key} at {instant}");
            } else {
                // A trade date exists wherever the venue is doing business -
                // including an order-entry phase, which carries the trade date
                // of the session it feeds.
                assert_eq!(
                    calendar.trade_date(instant).is_some(),
                    calendar.is_accepting_orders(instant),
                    "{key} at {instant}"
                );
            }
            match state {
                SessionState::OpenRegular => {
                    assert!(calendar.is_open_regular(instant), "{key} at {instant}");
                }
                SessionState::OpenExtended => {
                    assert!(!calendar.is_open_regular(instant), "{key} at {instant}");
                    assert!(calendar.is_open_extended(instant), "{key} at {instant}");
                }
                SessionState::OrderEntry => {
                    // Nothing matches, so the market is not open - but orders
                    // are accepted, which is the whole point of the phase.
                    assert!(!calendar.is_open(instant), "{key} at {instant}");
                    assert!(
                        calendar.is_accepting_orders(instant),
                        "{key} at {instant}: OrderEntry state must accept orders"
                    );
                }
                SessionState::Halt | SessionState::Maintenance | SessionState::Closed => {
                    assert!(!calendar.is_open(instant), "{key} at {instant}");
                }
                // SessionState is #[non_exhaustive]; a future state must not
                // silently satisfy this assertion.
                _ => panic!("unhandled SessionState variant at {instant} for {key}"),
            }
        }
        instant += Duration::hours(1);
    }
}

#[test]
fn all_key_calendars_match_dated_snapshots_over_two_years() {
    let mut instant = Utc
        .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
        .single()
        .expect("fixture must have a valid start");
    let end = Utc
        .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
        .single()
        .expect("fixture must have a valid end");

    // Rough Rice is the one key whose daily bar is identity-dependent: CBOT
    // Submission 18-001 assigns its non-wrapping 19:00-21:00 CT evening leg to
    // the following trade date, which only an identified calendar can know. A
    // detached snapshot has no identity and falls back to the close-date
    // default, exactly as `docs/schedules/sources.md` states for fixed
    // snapshots. Open/closed state must still agree everywhere, and the flag
    // below fails the test if this carve-out ever goes stale.
    let mut rough_rice_daily_bars_diverged = false;

    while instant < end {
        for &key in MarketHoursKey::ALL {
            let calendar = calendar_for_market_hours_key(key);
            let snapshot = exchange_hours::hours_for_market_hours_key(key, instant);
            assert_eq!(
                calendar.is_open(instant),
                snapshot.is_open(instant),
                "{key}"
            );
            let calendar_bar = calendar.candle_end(instant, CalendarResolution::Daily);
            let snapshot_bar =
                exchange_hours::candle_end(&snapshot, instant, CalendarResolution::Daily);
            if calendar_bar == snapshot_bar {
                continue;
            }
            if key == MarketHoursKey::GlobexRoughRice {
                rough_rice_daily_bars_diverged = true;
                continue;
            }
            // Open/closed state agrees everywhere, but a daily bar can
            // legitimately diverge inside a revision's shadow: the fixed
            // snapshot projects its own era onto the containing-or-next trade
            // date, while the calendar re-selects the product-family profile
            // for every candidate opening day. That shadow is exactly where
            // the snapshot taken at the bar's opening session differs from the
            // snapshot taken at the instant, so require every disagreement to
            // be explained that way and nothing else. The revisions themselves
            // are fenced by each family's boundary tests.
            let bar_era = calendar
                .session_bounds(instant)
                .map(|(open, _)| exchange_hours::hours_for_market_hours_key(key, open));
            assert_ne!(
                bar_era.as_ref(),
                Some(&snapshot),
                "{key} at {instant}: the calendar's daily bar {calendar_bar:?} disagrees with \
                 the dated snapshot outside any revision shadow"
            );
        }
        instant += Duration::hours(1);
    }

    assert!(
        rough_rice_daily_bars_diverged,
        "the Rough Rice daily-bar carve-out is stale: the identified calendar and the \
         detached snapshot now agree, so the exemption belongs in neither this test nor \
         the documentation"
    );
}
