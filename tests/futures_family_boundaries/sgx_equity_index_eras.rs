// SPDX-License-Identifier: MIT-0

//! The SGX equity-index eras before the 2020 calendar edition, the dated 2024
//! Japan move, the Taiwan launch boundary and the NTR first-listing boundary.
//!
//! Singapore is UTC+8 with no DST, so every probe is stated in Singapore
//! wall-clock and converted once. Each boundary is probed on both sides at an
//! instant that only one side serves, so a row keyed a day early or late — or
//! to the capture's weekday instead of the following Monday — fails here.

use chrono::{DateTime, TimeZone as _, Utc};
use chrono_tz::Asia;
use exchange_hours::{MarketHoursKey, SessionState, hours_for_market_hours_key};

const JAPAN: MarketHoursKey = MarketHoursKey::SgxEquityIndexJapan;
const CHINA: MarketHoursKey = MarketHoursKey::SgxEquityIndexChina;
const SINGAPORE: MarketHoursKey = MarketHoursKey::SgxEquityIndexSingapore;
const TAIWAN: MarketHoursKey = MarketHoursKey::SgxEquityIndexTaiwan;
const NTR: MarketHoursKey = MarketHoursKey::SgxEquityIndexNtrUsd;

/// A probe instant stated in Singapore wall-clock.
fn sgt(date: (i32, u32, u32), time: (u32, u32)) -> DateTime<Utc> {
    Asia::Singapore
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, 0)
        .single()
        .expect("Singapore keeps no DST, so every wall-clock instant is unambiguous")
        .with_timezone(&Utc)
}

fn open(key: MarketHoursKey, at: DateTime<Utc>) -> bool {
    hours_for_market_hours_key(key, at).is_open(at)
}

fn state(key: MarketHoursKey, at: DateTime<Utc>) -> SessionState {
    hours_for_market_hours_key(key, at).session_state(at)
}

/// The floor grids: the intersection of SGX's 2009 specification pages and
/// the 2013-08-20 portal table, carried to the January-2010 floor for Japan,
/// China and Singapore and — because the floor row is the timeline baseline
/// rather than a dated row — below it too. The T+1 leg closes 22:55, the
/// narrowest sourced close across the undated span; Japan's T+1 opens 15:30
/// behind the 15:15-15:30 queue; the A50's 2009 lunch break is withheld and
/// its T+1 open held at 17:00; the 2009 pre-open queues are served where their
/// anchor did not move.
#[test]
fn the_floor_grids_are_the_2009_and_2013_intersection_carried_to_the_floor_and_below() {
    for date in [(2009, 6, 17), (2012, 6, 20)] {
        // Japan.
        assert!(
            !open(JAPAN, sgt(date, (7, 40))),
            "{date:?}: Nikkei opened 07:45"
        );
        assert_eq!(
            state(JAPAN, sgt(date, (7, 35))),
            SessionState::OrderEntry,
            "{date:?}"
        );
        assert!(
            open(JAPAN, sgt(date, (10, 0))),
            "{date:?}: Nikkei T session"
        );
        assert_eq!(
            state(JAPAN, sgt(date, (14, 27))),
            SessionState::OpenExtended
        );
        assert_eq!(
            state(JAPAN, sgt(date, (15, 20))),
            SessionState::OrderEntry,
            "{date:?}"
        );
        assert!(
            !open(JAPAN, sgt(date, (15, 20))),
            "{date:?}: T+1 opens 15:30 at the floor"
        );
        assert!(open(JAPAN, sgt(date, (15, 35))), "{date:?}: T+1 open");
        assert!(open(JAPAN, sgt(date, (22, 50))), "{date:?}: T+1 to 22:55");
        assert!(
            !open(JAPAN, sgt(date, (23, 30))),
            "{date:?}: 22:55 close at the floor"
        );
        // China.
        assert_eq!(
            state(CHINA, sgt(date, (9, 5))),
            SessionState::OrderEntry,
            "{date:?}"
        );
        assert!(open(CHINA, sgt(date, (10, 0))), "{date:?}: A50 T session");
        assert!(
            !open(CHINA, sgt(date, (12, 0))),
            "{date:?}: the 2009 lunch break is withheld"
        );
        assert!(open(CHINA, sgt(date, (15, 3))), "{date:?}: A50 T to 15:05");
        assert!(
            !open(CHINA, sgt(date, (15, 10))),
            "{date:?}: A50 floor T close 15:05"
        );
        assert!(
            !open(CHINA, sgt(date, (16, 50))),
            "{date:?}: A50 T+1 held at 17:00"
        );
        assert!(open(CHINA, sgt(date, (17, 5))), "{date:?}: A50 T+1 open");
        assert!(!open(CHINA, sgt(date, (23, 30))), "{date:?}: 22:55 close");
        // Singapore.
        assert_eq!(
            state(SINGAPORE, sgt(date, (8, 20))),
            SessionState::OrderEntry,
            "{date:?}"
        );
        assert!(
            open(SINGAPORE, sgt(date, (17, 0))),
            "{date:?}: SiMSCI T to 17:10"
        );
        assert_eq!(
            state(SINGAPORE, sgt(date, (17, 12))),
            SessionState::OpenExtended
        );
        assert_eq!(
            state(SINGAPORE, sgt(date, (18, 5))),
            SessionState::OrderEntry,
            "{date:?}"
        );
        assert!(
            open(SINGAPORE, sgt(date, (18, 20))),
            "{date:?}: SiMSCI T+1 from 18:15"
        );
        assert!(
            !open(SINGAPORE, sgt(date, (23, 30))),
            "{date:?}: 22:55 close"
        );
        // Not yet listed.
        assert!(
            !open(TAIWAN, sgt(date, (10, 0))),
            "{date:?}: no FTSE Taiwan yet"
        );
        assert!(!open(NTR, sgt(date, (10, 0))), "{date:?}: no NTR suite yet");
    }
}

/// The floor grid's edges, exactly: the last open minute and the end-exclusive
/// close of every bound the intersection sets, so a table off by a minute
/// on any of them fails here rather than passing between loose probes.
#[test]
fn the_floor_edges_are_exact_on_the_japan_key() {
    let date = (2011, 3, 16);
    for key in [JAPAN, CHINA, SINGAPORE] {
        assert!(
            open(key, sgt(date, (22, 54))),
            "{key:?}: last minute before 22:55"
        );
        assert!(
            !open(key, sgt(date, (22, 55))),
            "{key:?}: 22:55 closes end-exclusive"
        );
    }
    assert_eq!(
        state(JAPAN, sgt(date, (7, 29))),
        SessionState::Closed,
        "queue opens 07:30"
    );
    assert_eq!(state(JAPAN, sgt(date, (7, 30))), SessionState::OrderEntry);
    assert_eq!(state(JAPAN, sgt(date, (7, 44))), SessionState::OrderEntry);
    assert_eq!(state(JAPAN, sgt(date, (7, 45))), SessionState::OpenRegular);
    assert_eq!(state(JAPAN, sgt(date, (14, 24))), SessionState::OpenRegular);
    assert_eq!(
        state(JAPAN, sgt(date, (14, 25))),
        SessionState::OpenExtended
    );
    assert_eq!(
        state(JAPAN, sgt(date, (14, 29))),
        SessionState::OpenExtended
    );
    assert_eq!(state(JAPAN, sgt(date, (14, 30))), SessionState::Halt);
    assert_eq!(
        state(JAPAN, sgt(date, (15, 14))),
        SessionState::Halt,
        "T+1 queue opens 15:15"
    );
    assert_eq!(state(JAPAN, sgt(date, (15, 15))), SessionState::OrderEntry);
    assert_eq!(state(JAPAN, sgt(date, (15, 29))), SessionState::OrderEntry);
    assert_eq!(
        state(JAPAN, sgt(date, (15, 30))),
        SessionState::OpenRegular,
        "T+1 opens 15:30"
    );
}

/// The same exact edges on the China and Singapore keys.
#[test]
fn the_floor_edges_are_exact_on_the_china_and_singapore_keys() {
    let date = (2011, 3, 16);
    assert_eq!(
        state(CHINA, sgt(date, (8, 59))),
        SessionState::Closed,
        "queue opens 09:00"
    );
    assert_eq!(state(CHINA, sgt(date, (9, 0))), SessionState::OrderEntry);
    assert_eq!(state(CHINA, sgt(date, (9, 14))), SessionState::OrderEntry);
    assert_eq!(
        state(CHINA, sgt(date, (9, 15))),
        SessionState::OpenRegular,
        "T opens 09:15"
    );
    assert_eq!(state(CHINA, sgt(date, (15, 4))), SessionState::OpenRegular);
    assert_eq!(
        state(CHINA, sgt(date, (15, 5))),
        SessionState::Halt,
        "15:05 closes end-exclusive"
    );
    assert_eq!(
        state(CHINA, sgt(date, (16, 59))),
        SessionState::Halt,
        "T+1 held at 17:00"
    );
    assert_eq!(state(CHINA, sgt(date, (17, 0))), SessionState::OpenRegular);
    assert_eq!(
        state(SINGAPORE, sgt(date, (8, 14))),
        SessionState::Closed,
        "queue opens 08:15"
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (8, 15))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (8, 30))),
        SessionState::OpenRegular
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (17, 9))),
        SessionState::OpenRegular
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (17, 10))),
        SessionState::OpenExtended
    );
    assert_eq!(state(SINGAPORE, sgt(date, (17, 15))), SessionState::Halt);
    assert_eq!(
        state(SINGAPORE, sgt(date, (17, 59))),
        SessionState::Halt,
        "queue opens 18:00"
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (18, 0))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (18, 14))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(SINGAPORE, sgt(date, (18, 15))),
        SessionState::OpenRegular,
        "T+1 opens 18:15"
    );
}

/// The 2013-08-20 table widens the A50's T close and lengthens every carried
/// family's T+1 close from 22:55 to 02:00; because the row lengthens a
/// wrapping overnight close it is keyed to Monday 2013-08-26, so the Friday
/// leg still ends 22:55 and Monday's runs to Tuesday 02:00.
#[test]
fn the_2013_table_takes_effect_on_the_monday_and_lengthens_the_overnight_close() {
    for key in [JAPAN, CHINA, SINGAPORE] {
        assert!(
            !open(key, sgt((2013, 8, 23), (23, 30))),
            "{key:?}: Friday leg ends 22:55"
        );
        assert!(
            open(key, sgt((2013, 8, 26), (23, 30))),
            "{key:?}: Monday's leg runs on"
        );
        assert!(open(key, sgt((2013, 8, 27), (1, 59))), "{key:?}: to 02:00");
        assert!(
            !open(key, sgt((2013, 8, 27), (2, 0))),
            "{key:?}: 02:00 closes end-exclusive"
        );
    }
    assert!(
        !open(CHINA, sgt((2013, 8, 23), (15, 40))),
        "Friday: 15:05 close"
    );
    assert!(
        !open(CHINA, sgt((2013, 8, 23), (12, 0))),
        "Friday: lunch withheld"
    );
    assert!(
        open(CHINA, sgt((2013, 8, 26), (12, 0))),
        "Monday: no lunch break"
    );
    assert!(
        open(CHINA, sgt((2013, 8, 26), (15, 40))),
        "Monday: 15:55 close"
    );
    assert_eq!(
        state(CHINA, sgt((2013, 8, 26), (15, 57))),
        SessionState::OpenExtended
    );
    assert_eq!(
        state(CHINA, sgt((2013, 8, 26), (8, 50))),
        SessionState::OrderEntry
    );
    assert!(
        !open(CHINA, sgt((2013, 8, 26), (16, 50))),
        "T+1 still held at 17:00"
    );
    assert!(
        !open(JAPAN, sgt((2013, 8, 23), (15, 20))),
        "Friday: T+1 opens 15:30"
    );
    assert!(
        open(JAPAN, sgt((2013, 8, 26), (15, 20))),
        "Monday: T+1 opens 15:15"
    );
    assert_eq!(
        state(JAPAN, sgt((2013, 8, 26), (7, 35))),
        SessionState::OrderEntry
    );
    assert_ne!(
        state(JAPAN, sgt((2013, 8, 26), (15, 5))),
        SessionState::OrderEntry,
        "T+1 queue withheld"
    );
    assert_eq!(
        state(SINGAPORE, sgt((2013, 8, 26), (18, 5))),
        SessionState::OrderEntry
    );
    // The intermediate state is served through 2015 on every key.
    assert!(
        open(CHINA, sgt((2015, 6, 17), (15, 40))),
        "2015: A50 T runs to 15:55"
    );
    assert!(!open(CHINA, sgt((2015, 6, 17), (16, 10))));
    assert!(open(JAPAN, sgt((2015, 6, 18), (1, 59))));
    assert!(!open(JAPAN, sgt((2015, 6, 18), (2, 0))));
}

/// The State-A boundary is keyed to Monday 2017-07-10, not the Wednesday
/// capture, so the Friday leg that opened under the 02:00 regime closes at
/// 02:00 and the first leg to run to 04:45 opens on the Monday.
#[test]
fn state_a_begins_on_the_monday_after_the_capture_for_the_three_carried_families() {
    for (key, t_probe, t1_probe) in [
        (JAPAN, (7, 40), (3, 0)),
        (CHINA, (16, 10), (3, 0)),
        (SINGAPORE, (17, 45), (3, 0)),
    ] {
        assert!(
            !open(key, sgt((2017, 7, 7), t_probe)),
            "{key:?}: Friday still on the old T bounds"
        );
        assert!(
            !open(key, sgt((2017, 7, 8), t1_probe)),
            "{key:?}: Friday's leg closed at 02:00"
        );
        assert!(
            open(key, sgt((2017, 7, 10), t_probe)),
            "{key:?}: Monday on the new T bounds"
        );
        assert!(
            open(key, sgt((2017, 7, 11), t1_probe)),
            "{key:?}: Monday's leg runs to 04:45"
        );
        assert!(
            !open(key, sgt((2017, 7, 11), (4, 45))),
            "{key:?}: 04:45 closes end-exclusive"
        );
    }
    assert_eq!(
        state(JAPAN, sgt((2017, 7, 10), (7, 20))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(JAPAN, sgt((2017, 7, 10), (14, 50))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(CHINA, sgt((2017, 7, 10), (16, 55))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(SINGAPORE, sgt((2017, 7, 10), (17, 35))),
        SessionState::OrderEntry
    );
}

/// SGX's change log dates the `SiMSCI` move to Monday 2019-06-10 ("Amended
/// trading hours for SGP, SGPO and ST eff 10 Jun", entry issued 2019-05-21):
/// the T close moves 17:10 -> 17:20 and the T+1 open 17:40 -> 17:50 on that
/// day, and the content API's 2019-06-11 payload is the first witness of the
/// new grid.
#[test]
fn the_simsci_move_is_dated_to_2019_06_10_by_the_change_log() {
    assert!(
        !open(SINGAPORE, sgt((2019, 6, 7), (17, 15))),
        "Friday: 17:10 close"
    );
    assert!(
        open(SINGAPORE, sgt((2019, 6, 7), (17, 45))),
        "Friday: 17:40 T+1 open"
    );
    assert!(
        open(SINGAPORE, sgt((2019, 6, 10), (17, 15))),
        "Monday: 17:20 close"
    );
    assert_eq!(
        state(SINGAPORE, sgt((2019, 6, 10), (17, 22))),
        SessionState::OpenExtended
    );
    assert_eq!(
        state(SINGAPORE, sgt((2019, 6, 10), (17, 45))),
        SessionState::OrderEntry
    );
    assert!(
        open(SINGAPORE, sgt((2019, 6, 10), (17, 52))),
        "Monday: 17:50 T+1 open"
    );
}

/// DT/AM 50 of 2024 dates the Japan T-session extension to Monday 2024-11-04
/// and states the routines on both sides.
#[test]
fn the_japan_t_session_lengthens_on_2024_11_04() {
    assert!(
        !open(JAPAN, sgt((2024, 11, 1), (14, 40))),
        "Friday: 14:25 close"
    );
    assert_eq!(
        state(JAPAN, sgt((2024, 11, 1), (14, 50))),
        SessionState::OrderEntry
    );
    assert!(
        open(JAPAN, sgt((2024, 11, 4), (14, 40))),
        "Monday: 14:55 close"
    );
    assert_eq!(
        state(JAPAN, sgt((2024, 11, 4), (14, 57))),
        SessionState::OpenExtended
    );
    assert_eq!(
        state(JAPAN, sgt((2024, 11, 4), (15, 20))),
        SessionState::OrderEntry
    );
    assert!(open(JAPAN, sgt((2024, 11, 4), (15, 30))), "T+1 opens 15:25");
    assert!(
        open(JAPAN, sgt((2025, 4, 4), (14, 40))),
        "the era runs to 2025-04-06"
    );
    assert!(
        open(JAPAN, sgt((2025, 4, 7), (15, 12))),
        "then DT/AM 15's 15:10 open"
    );
}

/// Taiwan starts on its stated launch day and NTR on the Monday after its
/// first calendar listing.
#[test]
fn taiwan_starts_on_its_launch_day_and_ntr_on_its_first_listing() {
    assert!(
        !open(TAIWAN, sgt((2020, 7, 17), (10, 0))),
        "Friday before launch"
    );
    assert!(
        open(TAIWAN, sgt((2020, 7, 20), (10, 0))),
        "launch day, 20 July 2020"
    );
    assert_eq!(
        state(TAIWAN, sgt((2020, 7, 20), (8, 35))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state(TAIWAN, sgt((2020, 7, 20), (13, 47))),
        SessionState::OpenExtended
    );
    assert!(
        open(TAIWAN, sgt((2020, 12, 16), (10, 0))),
        "late 2020 is no longer sessionless"
    );
    assert!(
        !open(NTR, sgt((2018, 4, 13), (10, 0))),
        "Friday before the first listing"
    );
    assert!(open(NTR, sgt((2018, 4, 16), (10, 0))), "Monday 2018-04-16");
    assert!(open(NTR, sgt((2018, 4, 17), (3, 0))), "T+1 runs to 04:45");
    assert!(!open(NTR, sgt((2018, 4, 17), (4, 50))));
    assert_eq!(
        state(NTR, sgt((2018, 4, 16), (18, 32))),
        SessionState::OpenExtended
    );
    assert_eq!(
        state(NTR, sgt((2018, 4, 16), (18, 55))),
        SessionState::OrderEntry
    );
}

/// SGX's change log dates the T+1 close extension to Monday 2019-11-11
/// ("Effective 11 Nov: ... (T+1) session Closing hours to 5:15am all T+1 traded
/// contracts", entry issued 2019-10-07), so the Thursday leg that opened under
/// 04:45 still closes at 04:45 on Friday 2019-11-08 and the first leg to run
/// to 05:15 opens on the Monday, on all four keys trading then.
#[test]
fn the_t_plus_one_close_extends_to_05_15_on_2019_11_11_on_four_keys() {
    for key in [JAPAN, CHINA, SINGAPORE, NTR] {
        assert!(
            !open(key, sgt((2019, 11, 8), (5, 0))),
            "{key:?}: Friday leg closed at 04:45"
        );
        assert!(
            open(key, sgt((2019, 11, 12), (5, 0))),
            "{key:?}: Monday's leg runs to 05:15"
        );
        assert!(
            !open(key, sgt((2019, 11, 12), (5, 15))),
            "{key:?}: 05:15 closes end-exclusive"
        );
    }
}

/// The 2019-11-11 rows (and Taiwan's launch row) carry the routines SGX's
/// content API states on 2020-01-09 for every family on that grid.
#[test]
fn the_2019_11_11_rows_carry_the_content_api_routines_on_all_five_keys() {
    let day = (2022, 6, 15);
    for (key, order_entry, extended) in [
        (JAPAN, (7, 20), (14, 27)),
        (CHINA, (8, 50), (16, 32)),
        (SINGAPORE, (8, 20), (17, 22)),
        (TAIWAN, (8, 35), (13, 47)),
        (NTR, (7, 15), (18, 32)),
    ] {
        assert_eq!(
            state(key, sgt(day, order_entry)),
            SessionState::OrderEntry,
            "{key:?}"
        );
        assert_eq!(
            state(key, sgt(day, extended)),
            SessionState::OpenExtended,
            "{key:?}"
        );
    }
}
