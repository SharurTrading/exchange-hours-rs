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

/// The floor grids: the 2013-08-20 portal table carried to the January-2010
/// floor for Japan, China and Singapore, and — because the floor row is the
/// timeline baseline rather than a dated row — below it too. The 02:00 T+1
/// close, the 07:45 Nikkei open and the Notes-stated closing routine are all
/// probed; the A50's T+1 open is held at 17:00, the narrowest value sourced
/// anywhere in the undated span, not the 16:10 the older fragment prints.
#[test]
fn the_floor_grids_are_carried_to_the_audit_floor_and_below() {
    for date in [(2009, 6, 17), (2012, 6, 20), (2015, 6, 17)] {
        assert!(
            open(JAPAN, sgt(date, (10, 0))),
            "{date:?}: Nikkei T session"
        );
        assert!(
            !open(JAPAN, sgt(date, (7, 40))),
            "{date:?}: Nikkei opened 07:45 then"
        );
        assert!(
            open(JAPAN, sgt(date, (20, 0))),
            "{date:?}: Nikkei T+1 session"
        );
        assert_eq!(
            state(JAPAN, sgt(date, (14, 27))),
            SessionState::OpenExtended
        );
        assert!(
            open(CHINA, sgt(date, (15, 20))),
            "{date:?}: A50 T to at least 15:25"
        );
        assert!(
            !open(CHINA, sgt(date, (16, 50))),
            "{date:?}: A50 T+1 held at 17:00"
        );
        assert_eq!(state(CHINA, sgt(date, (8, 50))), SessionState::OrderEntry);
        assert!(
            open(SINGAPORE, sgt(date, (17, 0))),
            "{date:?}: SiMSCI T to 17:10"
        );
        assert!(
            !open(SINGAPORE, sgt(date, (18, 0))),
            "{date:?}: SiMSCI T+1 opened 18:15"
        );
        assert_eq!(
            state(SINGAPORE, sgt(date, (17, 12))),
            SessionState::OpenExtended
        );
        // The floor eras serve no pre-open queue: no artifact of that era that
        // has been read states its length (#65).
        for (key, at) in [(JAPAN, (7, 20)), (SINGAPORE, (8, 20))] {
            let phase = state(key, sgt(date, at));
            assert_ne!(
                phase,
                SessionState::OrderEntry,
                "{key:?} {date:?}: no queue is served"
            );
            assert!(
                !open(key, sgt(date, at)),
                "{key:?} {date:?}: and nothing matches"
            );
        }
        assert!(
            !open(TAIWAN, sgt(date, (10, 0))),
            "{date:?}: no FTSE Taiwan yet"
        );
        assert!(!open(NTR, sgt(date, (10, 0))), "{date:?}: no NTR suite yet");
    }
    // The A50's floor T close is 15:25 until the 2013-08-20 row widens it to
    // 15:55; the T+1 open is held at 17:00 throughout.
    for date in [(2009, 6, 17), (2012, 6, 20)] {
        assert!(
            !open(CHINA, sgt(date, (15, 40))),
            "{date:?}: A50 floor T close 15:25"
        );
        assert_eq!(
            state(CHINA, sgt(date, (15, 27))),
            SessionState::OpenExtended,
            "{date:?}: the Notes-stated closing routine follows the 15:25 close"
        );
    }
    assert!(
        open(CHINA, sgt((2015, 6, 17), (15, 40))),
        "2015: A50 T runs to 15:55"
    );
    assert!(
        !open(CHINA, sgt((2015, 6, 17), (16, 10))),
        "2015: closed after the 15:55-16:00 routine"
    );
    // The 02:00 close: a Wednesday leg ends on Thursday morning.
    assert!(open(JAPAN, sgt((2015, 6, 18), (1, 59))));
    assert!(!open(JAPAN, sgt((2015, 6, 18), (2, 0))));
    assert!(!open(CHINA, sgt((2015, 6, 18), (2, 30))));
    assert!(!open(SINGAPORE, sgt((2015, 6, 18), (3, 0))));
}

/// The A50's T close widens on 2013-08-20 — the one boundary that is China's
/// alone — while its T+1 open stays held.
#[test]
fn the_a50_t_close_widens_on_2013_08_20_and_its_t_plus_one_open_does_not_narrow() {
    assert!(
        !open(CHINA, sgt((2013, 8, 19), (15, 40))),
        "Monday: 15:25 close"
    );
    assert!(
        open(CHINA, sgt((2013, 8, 20), (15, 40))),
        "Tuesday: 15:55 close"
    );
    assert_eq!(
        state(CHINA, sgt((2013, 8, 20), (15, 57))),
        SessionState::OpenExtended
    );
    assert!(
        !open(CHINA, sgt((2013, 8, 20), (16, 50))),
        "T+1 still held at 17:00"
    );
    assert!(open(CHINA, sgt((2013, 8, 20), (17, 5))));
    assert!(
        !open(CHINA, sgt((2013, 8, 21), (2, 30))),
        "02:00 close persists"
    );
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
fn the_2020_rows_carry_the_content_api_routines_on_all_five_keys() {
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
