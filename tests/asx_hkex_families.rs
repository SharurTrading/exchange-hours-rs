// SPDX-License-Identifier: MIT-0

//! Public-surface baselines for the two wave-2 product-family keys:
//! `asx_spi_200` and `hkex_index_futures`.
//!
//! Every test runs through `calendar_for_market_hours_key` (TEST-LAYOUT: the
//! tests get nothing callers do not get). Per AGENTS.md step 8: the published
//! open and the instant before it, the phase classification, the end-exclusive
//! close, the weekend boundary, holidays and early closes both sides, the
//! serde form — and, because the ASX selector is seasonal, both sides of each
//! US daylight-saving transition.

#![expect(
    clippy::expect_used,
    reason = "fixture instants are constructed from literal civil times and must \
              fail the test if unrepresentable"
)]

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Asia::Hong_Kong, Australia};
use exchange_hours::{
    Holiday, HolidayKind, MarketHoursKey, SessionKind, SessionState, calendar_for_market_hours_key,
};

fn sydney(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
    Australia::Sydney
        .with_ymd_and_hms(year, month, day, hour, min, 0)
        .single()
        .expect("fixture civil time must be unambiguous")
        .with_timezone(&Utc)
}

fn hongkong(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
    Hong_Kong
        .with_ymd_and_hms(year, month, day, hour, min, 0)
        .single()
        .expect("fixture civil time must be unambiguous")
        .with_timezone(&Utc)
}

// --- asx_spi_200 -------------------------------------------------------------

#[test]
fn asx_serde_round_trips_the_canonical_name() {
    let key = MarketHoursKey::AsxSpi200;
    assert_eq!(key.as_str(), "asx_spi_200");
    assert_eq!(key.to_string(), "asx_spi_200");
    assert_eq!("asx_spi_200".parse::<MarketHoursKey>(), Ok(key));
    assert!("asx_spi_200!".parse::<MarketHoursKey>().is_err());
    let json = serde_json::to_string(&key).expect("serde");
    assert_eq!(json, "\"asx_spi_200\"");
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>(&json).expect("serde"),
        key
    );
}

#[test]
fn asx_baseline_normal_week_in_the_us_standard_season() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::AsxSpi200);
    // Tuesday 2024-07-02 (US daylight, Sydney standard): the night session
    // opened Monday 17:10 and closes Tuesday 07:00; the day session runs
    // 09:50-16:30.
    assert!(calendar.is_open(sydney(2024, 7, 2, 6, 0)).expect("open"));
    assert!(!calendar.is_open(sydney(2024, 7, 2, 7, 0)).expect("closed"));
    assert!(
        !calendar
            .is_open(sydney(2024, 7, 2, 9, 45))
            .expect("pre-open")
    );
    assert!(calendar.is_open(sydney(2024, 7, 2, 10, 0)).expect("day"));
    assert!(
        !calendar
            .is_open(sydney(2024, 7, 2, 16, 30))
            .expect("end-exclusive")
    );
    // Order entry only before each open; the queue matches nothing.
    assert_eq!(
        calendar
            .session_state(sydney(2024, 7, 2, 9, 45))
            .expect("state"),
        SessionState::OrderEntry
    );
    assert_eq!(
        calendar
            .session_state(sydney(2024, 7, 1, 17, 5))
            .expect("state"),
        SessionState::OrderEntry
    );
    assert_eq!(
        calendar
            .session_state(sydney(2024, 7, 2, 12, 0))
            .expect("state"),
        SessionState::OpenRegular
    );
    // The night session is extended, not regular.
    assert!(
        calendar
            .is_open_with(sydney(2024, 7, 2, 6, 0), SessionKind::Extended)
            .expect("extended"),
    );
    assert!(
        !calendar
            .is_open_with(sydney(2024, 7, 2, 6, 0), SessionKind::Regular)
            .expect("not regular"),
    );
    // Trade date of the night session that opened Monday is Tuesday.
    assert_eq!(
        calendar
            .trade_date(sydney(2024, 7, 2, 6, 0))
            .expect("trade date"),
        Some(chrono::NaiveDate::from_ymd_opt(2024, 7, 2).expect("fixture"))
    );
    // Weekend: the Friday-evening leg ends Saturday 07:00 (July is the US
    // daylight column), and Saturday daytime is closed.
    assert!(
        calendar
            .is_open(sydney(2024, 7, 6, 6, 30))
            .expect("fri leg")
    );
    assert!(!calendar.is_open(sydney(2024, 7, 6, 7, 0)).expect("closed"));
    assert!(!calendar.is_open(sydney(2024, 7, 7, 12, 0)).expect("sunday"));
    // The session bounds are end-exclusive at the day close.
    let (open, close) = calendar
        .session_bounds(sydney(2024, 7, 2, 12, 0))
        .expect("bounds")
        .expect("a session contains noon");
    assert_eq!(open, sydney(2024, 7, 2, 9, 50));
    assert_eq!(close, sydney(2024, 7, 2, 16, 30));
}

#[test]
fn asx_night_close_follows_the_us_daylight_column_across_both_transitions() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::AsxSpi200);
    // March: US daylight starts on the second Sunday (2024-03-10). The night
    // session opening that Sunday closes 07:00 Monday; the session that opened
    // the prior Friday closed 08:00 Saturday.
    let (_, close) = calendar
        .session_bounds(sydney(2024, 3, 10, 18, 0))
        .expect("bounds")
        .expect("the Sunday night session contains 18:00");
    assert_eq!(close, sydney(2024, 3, 11, 7, 0));
    let (_, close) = calendar
        .session_bounds(sydney(2024, 3, 8, 18, 0))
        .expect("bounds")
        .expect("the Friday night session contains 18:00");
    assert_eq!(close, sydney(2024, 3, 9, 8, 0));
    // November: US daylight ends on the first Sunday (2024-11-03). The session
    // opening that Sunday closes 08:00 Monday; the prior Friday's leg closed
    // 07:00 Saturday.
    let (_, close) = calendar
        .session_bounds(sydney(2024, 11, 3, 18, 0))
        .expect("bounds")
        .expect("the Sunday night session contains 18:00");
    assert_eq!(close, sydney(2024, 11, 4, 8, 0));
    let (_, close) = calendar
        .session_bounds(sydney(2024, 11, 1, 18, 0))
        .expect("bounds")
        .expect("the Friday night session contains 18:00");
    assert_eq!(close, sydney(2024, 11, 2, 7, 0));
}

#[test]
fn asx_holiday_rows_clip_and_delete_across_the_arrangements() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::AsxSpi200);
    // Australia Day 2024 (Friday 26 January, day_closed): the morning leg ran
    // to 08:00, the day session did not, and the evening reopened.
    assert!(
        calendar
            .is_open(sydney(2024, 1, 26, 6, 0))
            .expect("morning leg")
    );
    assert!(
        !calendar
            .is_open(sydney(2024, 1, 26, 12, 0))
            .expect("day closed")
    );
    assert!(
        calendar
            .is_open(sydney(2024, 1, 26, 18, 0))
            .expect("night reopen")
    );
    let holiday =
        calendar.holiday_on(chrono::NaiveDate::from_ymd_opt(2024, 1, 26).expect("fixture"));
    assert!(matches!(
        holiday.map(Holiday::kind),
        Some(HolidayKind::EarlyClose { close_ssm })
            if close_ssm == 8 * 3_600
    ));
    // Good Friday 2019 is the Day-before-Good-Friday trade date: the
    // Thursday-evening night leg ran to its midnight cease and the day itself
    // was closed; the Saturday leg was closed with it and Easter Monday
    // reopened with the day session alone (the closed Sunday-evening session
    // is the replacement's removal).
    assert!(
        calendar
            .is_open(sydney(2019, 4, 18, 20, 0))
            .expect("thursday evening leg")
    );
    assert_eq!(
        calendar
            .session_state(sydney(2019, 4, 18, 17, 5))
            .expect("state"),
        SessionState::OrderEntry
    );
    assert_eq!(
        calendar
            .trade_date(sydney(2019, 4, 18, 20, 0))
            .expect("trade date"),
        Some(chrono::NaiveDate::from_ymd_opt(2019, 4, 19).expect("fixture"))
    );
    assert!(
        !calendar
            .is_open(sydney(2019, 4, 19, 0, 0))
            .expect("midnight cease, end-exclusive")
    );
    assert!(
        !calendar
            .is_open(sydney(2019, 4, 19, 12, 0))
            .expect("good friday day closed")
    );
    assert!(
        !calendar
            .is_open(sydney(2019, 4, 20, 6, 0))
            .expect("saturday leg removed")
    );
    assert!(
        !calendar
            .is_open(sydney(2019, 4, 22, 6, 0))
            .expect("monday leg removed")
    );
    assert!(
        !calendar
            .is_open(sydney(2019, 4, 22, 10, 0))
            .expect("easter monday closed")
    );
    assert!(
        calendar
            .is_open(sydney(2019, 4, 23, 10, 0))
            .expect("tuesday day")
    );
    // The trade date after a closed single holiday (2019-01-01) runs its day
    // session but no wrapped leg.
    assert!(
        !calendar
            .is_open(sydney(2019, 1, 2, 6, 0))
            .expect("new-year evening leg removed")
    );
    assert!(
        calendar
            .is_open(sydney(2019, 1, 2, 10, 0))
            .expect("jan 2 day")
    );
    // Christmas Eve 2019: day session closes 14:30 and the evening leg is
    // removed from Christmas Day's trade date; Boxing Day is closed outright.
    let (open, close) = calendar
        .session_bounds(sydney(2019, 12, 24, 10, 0))
        .expect("bounds")
        .expect("the eve day session contains 10:00");
    assert_eq!(close, sydney(2019, 12, 24, 14, 30));
    assert_eq!(open, sydney(2019, 12, 24, 9, 50));
    assert!(
        !calendar
            .is_open(sydney(2019, 12, 25, 6, 0))
            .expect("christmas leg removed")
    );
    assert!(
        !calendar
            .is_open(sydney(2019, 12, 26, 12, 0))
            .expect("boxing day closed")
    );
    assert!(
        calendar
            .is_open(sydney(2019, 12, 27, 10, 0))
            .expect("resumed")
    );
    // US policy era: the night session opening on Memorial Day 2012 ceased at
    // midnight, and the Tuesday ran that shortened leg plus its own day. The
    // leg opens 17:10 with the ten-minute queue before it.
    assert!(
        !calendar
            .is_open(sydney(2012, 5, 28, 17, 5))
            .expect("queue before the ceased leg")
    );
    let (open, close) = calendar
        .session_bounds(sydney(2012, 5, 28, 20, 0))
        .expect("bounds")
        .expect("the ceased night leg contains 20:00");
    assert_eq!(open, sydney(2012, 5, 28, 17, 10));
    assert_eq!(close, sydney(2012, 5, 29, 0, 0));
    assert!(
        calendar
            .is_open(sydney(2012, 5, 29, 10, 0))
            .expect("tuesday day")
    );
    assert!(
        !calendar
            .is_open(sydney(2012, 5, 29, 3, 0))
            .expect("gap after")
    );
    // The 2016 Australia Day night session opened 17:00 (the capture's own
    // instant) and the next trade date ran its normal day.
    assert!(
        calendar
            .is_open(sydney(2016, 1, 26, 17, 5))
            .expect("early open")
    );
    assert!(
        !calendar
            .is_open(sydney(2016, 1, 26, 16, 55))
            .expect("still closed")
    );
    assert!(
        calendar
            .is_open(sydney(2016, 1, 27, 10, 0))
            .expect("next day")
    );
}

// --- hkex_index_futures ------------------------------------------------------

#[test]
fn hkex_serde_round_trips_the_canonical_name() {
    let key = MarketHoursKey::HkexIndexFutures;
    assert_eq!(key.as_str(), "hkex_index_futures");
    assert_eq!(key.to_string(), "hkex_index_futures");
    assert_eq!("hkex_index_futures".parse::<MarketHoursKey>(), Ok(key));
    let json = serde_json::to_string(&key).expect("serde");
    assert_eq!(json, "\"hkex_index_futures\"");
}

#[test]
fn hkex_current_era_baseline_with_the_wrapped_overnight_session() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::HkexIndexFutures);
    // Tuesday 2026-11-03: the T+1 session opened Monday 17:00 and runs to
    // 03:00; the day sessions are 09:15-12:00 and 13:00-16:30.
    assert!(calendar.is_open(hongkong(2026, 11, 3, 2, 0)).expect("aht"));
    assert!(
        calendar
            .is_open(hongkong(2026, 11, 3, 10, 0))
            .expect("morning")
    );
    assert!(
        !calendar
            .is_open(hongkong(2026, 11, 3, 12, 0))
            .expect("end-exclusive")
    );
    assert!(
        !calendar
            .is_open(hongkong(2026, 11, 3, 12, 30))
            .expect("lunch")
    );
    assert!(
        calendar
            .is_open(hongkong(2026, 11, 3, 14, 0))
            .expect("afternoon")
    );
    assert!(
        !calendar
            .is_open(hongkong(2026, 11, 3, 16, 30))
            .expect("end-exclusive")
    );
    assert!(
        !calendar
            .is_open(hongkong(2026, 11, 3, 16, 45))
            .expect("break")
    );
    assert!(
        calendar
            .is_open(hongkong(2026, 11, 3, 17, 0))
            .expect("aht open")
    );
    // The pre-opening periods are order entry only.
    assert_eq!(
        calendar
            .session_state(hongkong(2026, 11, 3, 9, 0))
            .expect("state"),
        SessionState::OrderEntry
    );
    assert_eq!(
        calendar
            .session_state(hongkong(2026, 11, 3, 12, 45))
            .expect("state"),
        SessionState::OrderEntry
    );
    // The wrapped T+1 leg that opened Monday belongs to Tuesday's trade date.
    assert_eq!(
        calendar
            .trade_date(hongkong(2026, 11, 3, 2, 0))
            .expect("trade date"),
        Some(chrono::NaiveDate::from_ymd_opt(2026, 11, 3).expect("fixture"))
    );
    // Weekend: the Friday T+1 leg ends Saturday 03:00 and nothing runs after.
    assert!(
        calendar
            .is_open(hongkong(2026, 11, 7, 2, 0))
            .expect("friday leg")
    );
    assert!(
        !calendar
            .is_open(hongkong(2026, 11, 7, 3, 0))
            .expect("end-exclusive")
    );
    assert!(
        !calendar
            .is_open(hongkong(2026, 11, 8, 12, 0))
            .expect("sunday")
    );
}

#[test]
fn hkex_dated_and_bracketed_cutovers_answer_on_both_sides() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::HkexIndexFutures);
    // Floor: 9:45 open on Friday 2011-03-04; Phase 1 opens 9:15 from Monday
    // 2011-03-07.
    assert!(
        !calendar
            .is_open(hongkong(2011, 3, 4, 9, 30))
            .expect("floor pre-open")
    );
    assert!(
        calendar
            .is_open(hongkong(2011, 3, 4, 10, 0))
            .expect("floor morning")
    );
    assert!(
        calendar
            .is_open(hongkong(2011, 3, 7, 9, 30))
            .expect("phase 1 morning")
    );
    assert!(
        !calendar
            .is_open(hongkong(2011, 3, 4, 13, 30))
            .expect("floor lunch gap")
    );
    assert!(
        calendar
            .is_open(hongkong(2011, 3, 7, 14, 0))
            .expect("phase 1 afternoon")
    );
    // Phase 2: the afternoon opens 13:00 from 2012-03-05 (was 13:30).
    assert!(
        !calendar
            .is_open(hongkong(2012, 3, 2, 13, 15))
            .expect("phase 1 lunch")
    );
    assert!(
        calendar
            .is_open(hongkong(2012, 3, 5, 13, 15))
            .expect("phase 2 afternoon")
    );
    // T+1 launch 2013-04-08: no evening session before it, 17:00-23:00 from it.
    assert!(
        !calendar
            .is_open(hongkong(2013, 4, 5, 18, 0))
            .expect("no aht")
    );
    assert!(calendar.is_open(hongkong(2013, 4, 8, 18, 0)).expect("aht"));
    // The 11:45pm close is first served from the 03/11/2014 witness; the
    // Monday before still closes 23:00.
    assert!(
        !calendar
            .is_open(hongkong(2014, 10, 31, 23, 0))
            .expect("23:00 close")
    );
    assert!(
        calendar
            .is_open(hongkong(2014, 11, 3, 23, 30))
            .expect("23:45 era")
    );
    assert!(
        !calendar
            .is_open(hongkong(2014, 11, 3, 23, 45))
            .expect("end-exclusive")
    );
    // Day close 16:30 and T+1 start 17:15 from the 22/07/2016 witness: the
    // Thursday before still closes at 16:15 and runs the served intersection's
    // 17:15 T+1 open, while the Friday runs to 16:30 with the same T+1.
    assert!(
        !calendar
            .is_open(hongkong(2016, 7, 21, 16, 20))
            .expect("16:15 close")
    );
    assert!(
        calendar
            .is_open(hongkong(2016, 7, 22, 16, 20))
            .expect("16:30 era")
    );
    assert!(
        !calendar
            .is_open(hongkong(2016, 7, 21, 17, 5))
            .expect("intersection: 17:15 open")
    );
    assert!(
        !calendar
            .is_open(hongkong(2016, 7, 22, 17, 5))
            .expect("17:15 era gap")
    );
    assert!(
        !calendar
            .is_open(hongkong(2016, 7, 21, 16, 45))
            .expect("17:00 era gap")
    );
    assert!(
        calendar
            .is_open(hongkong(2016, 7, 22, 17, 20))
            .expect("17:15 era")
    );
    // The intersection holds from the 2014 witness: mid-2015 runs the 17:15
    // T+1 open and the 16:15 day close that hold under both sourced states.
    assert!(
        !calendar
            .is_open(hongkong(2015, 6, 1, 17, 5))
            .expect("intersection: 17:15 open")
    );
    assert!(
        calendar
            .is_open(hongkong(2015, 6, 1, 17, 20))
            .expect("intersection: leg runs")
    );
    assert!(
        !calendar
            .is_open(hongkong(2015, 6, 1, 16, 20))
            .expect("intersection: 16:15 close")
    );
    // The overnight shape is first served from 2026-10-12, the opening day of
    // the first session it governs: the prior Friday leg still ends 23:45.
    assert!(
        !calendar
            .is_open(hongkong(2026, 10, 11 - 1, 23, 50))
            .expect("23:45 era")
    );
    assert!(
        calendar
            .is_open(hongkong(2026, 10, 12, 23, 50))
            .expect("03:00 era")
    );
}

#[test]
fn hkex_holiday_half_day_and_aht_suspension_rows() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::HkexIndexFutures);
    // National Day 2024 (Tuesday 1 October): closed outright, including the
    // Monday-evening T+1 leg.
    assert!(
        !calendar
            .is_open(hongkong(2024, 10, 1, 10, 0))
            .expect("national day closed")
    );
    assert!(
        !calendar
            .is_open(hongkong(2024, 10, 1, 20, 0))
            .expect("no aht on the holiday")
    );
    assert!(
        calendar
            .is_open(hongkong(2024, 9, 30, 20, 0))
            .expect("monday evening aht ran")
    );
    // Christmas Eve 2024 (Tuesday): half day, no afternoon, no AHT; the
    // morning runs to its own noon close under the 12:30 clip.
    let (_, close) = calendar
        .session_bounds(hongkong(2024, 12, 24, 10, 0))
        .expect("bounds")
        .expect("the eve morning contains 10:00");
    assert_eq!(close, hongkong(2024, 12, 24, 12, 0));
    assert!(
        !calendar
            .is_open(hongkong(2024, 12, 24, 14, 0))
            .expect("no afternoon")
    );
    assert!(
        !calendar
            .is_open(hongkong(2024, 12, 24, 20, 0))
            .expect("no aht on the eve")
    );
    // UK/US bank holiday 2019-05-27 (Monday): the day ran; the evening T+1
    // session was suspended.
    assert!(
        calendar
            .is_open(hongkong(2019, 5, 27, 10, 0))
            .expect("day ran")
    );
    assert!(
        !calendar
            .is_open(hongkong(2019, 5, 27, 20, 0))
            .expect("aht suspended")
    );
    assert!(
        calendar
            .is_open(hongkong(2019, 5, 28, 20, 0))
            .expect("tuesday evening aht ran")
    );
    // The operator's 2026 table lists no 28 December holiday: the Monday runs
    // normally and the absence is fenced.
    assert!(
        calendar
            .is_open(hongkong(2026, 12, 28, 10, 0))
            .expect("trading day")
    );
    // The wrapped era's holiday evenings: after the 2027 Ching Ming Monday the
    // wrapped leg into Tuesday is removed while Tuesday's day runs.
    assert!(
        !calendar
            .is_open(hongkong(2027, 4, 6, 2, 0))
            .expect("no leg into tuesday")
    );
    assert!(
        calendar
            .is_open(hongkong(2027, 4, 6, 10, 0))
            .expect("tuesday day")
    );
    // And the Saturday after a Friday holiday holds no session at all.
    assert!(
        !calendar
            .is_open(hongkong(2027, 1, 2, 2, 0))
            .expect("no saturday leg after new year")
    );
    // The 2027 Good Friday Saturday closure removes the Friday-evening leg.
    assert!(
        !calendar
            .is_open(hongkong(2027, 3, 27, 2, 0))
            .expect("no saturday leg after good friday")
    );
}
