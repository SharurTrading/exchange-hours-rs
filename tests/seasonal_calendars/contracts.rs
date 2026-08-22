// SPDX-License-Identifier: MIT-0

//! Fixed/date-aware compatibility and cross-query contracts.

use super::prelude::*;

#[test]
fn fixed_venue_calendars_match_market_hours_queries() {
    assert_fixed_calendar_parity(
        Exchange::Tsx,
        local(America::Toronto, (2026, 8, 19), (10, 12, 0)),
    );
    assert_fixed_calendar_parity(
        Exchange::Cme,
        local(US::Central, (2026, 4, 20), (16, 30, 0)),
    );
}

#[test]
fn every_fixed_venue_calendar_matches_the_current_market_hours_surface() {
    // A future reference day is deliberately beyond every recorded cutover,
    // so fixed venues must have converged on their default current snapshot.
    let instants = [
        Utc.with_ymd_and_hms(2100, 8, 19, 2, 0, 0)
            .single()
            .expect("valid fixture"),
        Utc.with_ymd_and_hms(2100, 8, 19, 12, 0, 0)
            .single()
            .expect("valid fixture"),
        Utc.with_ymd_and_hms(2100, 8, 19, 18, 0, 0)
            .single()
            .expect("valid fixture"),
    ];

    for &exchange in Exchange::ALL {
        // B3/BMV/Vienna and the international reference-clock selectors are
        // intentionally date-dependent.
        if matches!(
            exchange,
            Exchange::B3
                | Exchange::Bmv
                | Exchange::Vienna
                | Exchange::Eurex
                | Exchange::IceEndex
                | Exchange::IceAbuDhabi
        ) {
            continue;
        }
        for instant in instants {
            assert_eq!(
                hours_for_exchange_as_of(exchange, instant),
                hours_for_exchange(exchange),
                "{exchange:?}: current and as-of snapshots diverge at {instant}"
            );
            assert_fixed_calendar_parity(exchange, instant);
        }
    }
}

#[test]
fn historical_profiles_preserve_each_venue_timezone() {
    let mut date = day((2010, 1, 1));
    let end = day((2027, 1, 1));

    let expected_zones: Vec<_> = Exchange::ALL
        .iter()
        .map(|&exchange| {
            let expected = hours_for_exchange(exchange).tz;
            assert_eq!(
                calendar_for_exchange(exchange).tz(),
                expected,
                "{exchange:?}: ExchangeCalendar exposes a different venue zone"
            );
            (exchange, expected)
        })
        .collect();

    while date < end {
        let instant = Utc.from_utc_datetime(
            &date
                .and_hms_opt(12, 0, 0)
                .expect("daily UTC-noon fixture is representable"),
        );
        for &(exchange, expected) in &expected_zones {
            assert_eq!(
                hours_for_exchange_as_of(exchange, instant).tz,
                expected,
                "{exchange:?}: historical profile changed venue zone at {instant}"
            );
        }
        date = date.succ_opt().expect("the bounded history scan advances");
    }
}

#[test]
fn seasonal_exchange_names_have_stable_serde_forms() {
    for (exchange, name) in [
        (Exchange::B3, "b3"),
        (Exchange::Bmv, "bmv"),
        (Exchange::Vienna, "vienna"),
    ] {
        assert_eq!(exchange.as_str(), name);
        assert_eq!(exchange.to_string(), name);
        assert_eq!(
            serde_json::to_string(&exchange).expect("serializes exchange"),
            format!("\"{name}\"")
        );
        assert_eq!(
            serde_json::from_str::<Exchange>(&format!("\"{name}\""))
                .expect("deserializes exchange"),
            exchange
        );
    }
}

#[test]
fn date_aware_cross_query_fence_holds_for_every_exchange() {
    let instants = [
        Utc.with_ymd_and_hms(2016, 3, 14, 12, 0, 0)
            .single()
            .expect("valid fixture"),
        Utc.with_ymd_and_hms(2023, 3, 13, 18, 0, 0)
            .single()
            .expect("valid fixture"),
        Utc.with_ymd_and_hms(2026, 11, 2, 18, 0, 0)
            .single()
            .expect("valid fixture"),
    ];
    for &exchange in Exchange::ALL {
        let calendar = calendar_for_exchange(exchange);
        for instant in instants {
            for kind in [
                SessionKind::Regular,
                SessionKind::Extended,
                SessionKind::Both,
            ] {
                let bounds = calendar.session_bounds_with(instant, kind);
                let contained =
                    bounds.is_some_and(|(open, close)| open <= instant && instant < close);
                assert_eq!(
                    calendar.is_open_with(instant, kind),
                    contained,
                    "{exchange:?}/{kind:?} disagrees at {instant}"
                );
                if !contained {
                    assert_eq!(
                        bounds,
                        calendar.next_session_after_with(instant, kind),
                        "{exchange:?}/{kind:?} next-session mismatch at {instant}"
                    );
                }
            }
        }
    }
}
