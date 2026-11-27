// SPDX-License-Identifier: MIT-0

//! Runtime venue, selector, and reference-clock routing through public APIs.

#![cfg(feature = "runtime-tzdb")]

use std::convert::Infallible;
use std::io;

use exchange_hours::{
    Exchange, ExchangeCalendar, MarketHoursKey, NoPolicy, SessionKind, TimeZoneSnapshot,
    TimeZoneSnapshotError,
};
use jiff::Timestamp;

// A TZif v1 file with one fixed UTC offset and no transitions.
fn fixed_tzif(seconds: i32) -> Vec<u8> {
    let mut file = b"TZif".to_vec();
    file.resize(20, 0);
    for count in [0_u32, 0, 0, 0, 1, 4] {
        file.extend_from_slice(&count.to_be_bytes());
    }
    file.extend_from_slice(&seconds.to_be_bytes());
    file.extend_from_slice(b"\0\0FIX\0");
    file
}

fn snapshot(new_york_hours: i32) -> TimeZoneSnapshot {
    TimeZoneSnapshot::from_tzif("test", |name| {
        let hours = match name {
            "America/New_York" => new_york_hours,
            "America/Sao_Paulo" => -3,
            _ => 0,
        };
        Ok::<_, Infallible>(fixed_tzif(hours * 3_600))
    })
    .unwrap()
}

fn at(text: &str) -> Timestamp {
    text.parse().unwrap()
}

#[test]
fn incomplete_or_corrupt_snapshots_report_the_requested_zone() {
    let missing = TimeZoneSnapshot::from_tzif("missing", |name| {
        if name == "US/Central" {
            Err(io::Error::from(io::ErrorKind::NotFound))
        } else {
            Ok(fixed_tzif(0))
        }
    });
    assert!(matches!(
        missing,
        Err(TimeZoneSnapshotError::Load {
            name: "US/Central",
            ..
        })
    ));

    let invalid = TimeZoneSnapshot::from_tzif("invalid", |name| {
        Ok::<_, Infallible>(if name == "Europe/London" {
            vec![42]
        } else {
            fixed_tzif(0)
        })
    });
    assert!(matches!(
        invalid,
        Err(TimeZoneSnapshotError::Parse {
            name: "Europe/London",
            ..
        })
    ));
}

#[test]
fn venue_revision_uses_the_snapshots_opening_day() {
    let zones = snapshot(2);
    let calendar = ExchangeCalendar::new(Exchange::Ltse);
    // 00:30 on the launch date with the supplied rules, still the preceding
    // evening in the precompiled America/New_York.
    let instant = at("2020-08-27T22:30Z");
    assert!(calendar.hours_at(instant).regular.is_empty());
    assert!(
        !zones
            .hours_for_exchange(Exchange::Ltse, instant)
            .regular
            .is_empty()
    );
    assert_eq!(zones.release(), "test");
}

#[test]
fn sessions_holiday_clips_overlays_and_bulk_results_use_the_selected_clock() {
    let zones = snapshot(-2);
    let instant = at("2025-11-28T14:59:59Z");
    let close = at("2025-11-28T15:00Z");
    let embedded = ExchangeCalendar::new(Exchange::Nyse);
    let runtime = embedded.with_time_zone_snapshot(&zones);
    assert_eq!(runtime.is_open_regular(instant), Ok(true));
    assert_eq!(
        runtime.session_bounds_with(instant, SessionKind::Regular),
        Ok(Some((at("2025-11-28T11:30Z"), close)))
    );
    assert_eq!(runtime.is_open_regular(close), Ok(false));
    assert_eq!(
        runtime.with_day_policy(&NoPolicy).is_open_regular(close),
        Ok(false)
    );
    assert_eq!(embedded.is_open_regular(close), Ok(true));

    let one = runtime.hours_at(instant);
    assert_eq!(
        zones.hours_for_all(&[Exchange::Nyse], instant),
        std::slice::from_ref(&one)
    );
    assert_eq!(
        zones.hours_map_us_equities(instant).get(&Exchange::Nyse),
        Some(&one)
    );
    let key = MarketHoursKey::OandaFx;
    assert_eq!(
        zones.session_profile(key).tz.to_offset(instant),
        zones
            .hours_for_market_hours_key(key, instant)
            .tz
            .to_offset(instant)
    );
    assert_eq!(
        zones.session_profile(key).tz.to_offset(instant).seconds(),
        -7_200
    );
}

#[test]
fn b3_uses_each_snapshots_new_york_rules_for_queries_and_refusal_context() {
    let eastern_summer = snapshot(-4);
    let eastern_winter = snapshot(-5);
    let b3 = ExchangeCalendar::new(Exchange::B3);
    let short = b3.with_time_zone_snapshot(&eastern_summer);
    let long = b3.with_time_zone_snapshot(&eastern_winter);
    let in_long_regular = at("2026-06-16T20:15Z");
    assert_eq!(short.is_open_regular(in_long_regular), Ok(false));
    assert_eq!(long.is_open_regular(in_long_regular), Ok(true));
    assert_eq!(
        short.tz().to_offset(in_long_regular),
        long.tz().to_offset(in_long_regular)
    );

    let refused = at("2029-03-07T15:00Z");
    let error = short.is_open(refused).unwrap_err();
    assert_eq!(
        error
            .normal_week_baseline_in(&eastern_summer)
            .unwrap()
            .profile(),
        &short.hours_at(refused)
    );
    assert_eq!(
        error
            .normal_week_baseline_in(&eastern_winter)
            .unwrap()
            .profile(),
        &long.hours_at(refused)
    );
    let gap = exchange_hours::CalendarQueryError::UnresolvedGap {
        source: b3.source(),
        date: jiff::civil::date(2029, 3, 7),
    };
    let short_context = gap.display_in(&eastern_summer).to_string();
    let long_context = gap.display_in(&eastern_winter).to_string();
    assert!(short_context.contains("[tzdb test]"));
    assert_ne!(short_context, long_context);
    assert!(gap.to_string().contains("[embedded tzdb "));
    assert_ne!(
        short.hours_at(refused).regular,
        long.hours_at(refused).regular
    );
}

#[test]
fn bundled_tzif_and_precompiled_rules_agree_across_every_identity() {
    let zones = TimeZoneSnapshot::from_tzif(exchange_hours::EMBEDDED_TZDB_VERSION, |name| {
        jiff_tzdb::get(name).map(|(_, data)| data).ok_or(name)
    })
    .unwrap();
    let identities = Exchange::ALL
        .iter()
        .copied()
        .map(ExchangeCalendar::new)
        .chain(
            MarketHoursKey::ALL
                .iter()
                .copied()
                .map(ExchangeCalendar::for_market_hours_key),
        );
    for embedded in identities {
        let loaded = embedded.with_time_zone_snapshot(&zones);
        for instant in [
            at("2011-02-22T15:00Z"),
            at("2018-11-04T06:30Z"),
            at("2025-03-11T12:30Z"),
            at("2026-06-16T20:15Z"),
        ] {
            let original = embedded.hours_at(instant);
            let selected = loaded.hours_at(instant);
            assert_eq!(
                original.tz.to_offset(instant),
                selected.tz.to_offset(instant)
            );
            assert_eq!(original.regular, selected.regular);
            assert_eq!(original.extended, selected.extended);
            assert_eq!(original.order_entry, selected.order_entry);
            assert_eq!(
                embedded.session_bounds(instant),
                loaded.session_bounds(instant),
                "{:?} at {instant}",
                embedded.source()
            );
            assert_eq!(
                embedded
                    .coverage()
                    .coverage_on(original.tz.to_datetime(instant).date()),
                loaded
                    .coverage()
                    .coverage_on(selected.tz.to_datetime(instant).date())
            );
        }
    }
}
