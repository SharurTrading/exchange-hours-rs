// SPDX-License-Identifier: MIT-0

//! Domain validation for every shipped public rule table.

use super::historical_expectations::*;
use super::identity_expectations::*;
use super::prelude::*;

#[test]
fn every_shipped_rule_table_satisfies_the_session_rule_domain() {
    use chrono::TimeZone;
    use exchange_hours::{MarketHoursKey, hours_for_exchange_as_of, session_profile};

    // Epochs on both sides of every recorded cutover, so the historical
    // profiles (including pre-go-live empties) are validated too.
    let epochs = [
        Utc.with_ymd_and_hms(2012, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2015, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2020, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2026, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
    ];

    let validate_all = |hours: &MarketHours, source: &str| {
        for rule in hours.regular.iter().chain(hours.extended.iter()) {
            rule.validate().unwrap_or_else(|violation| {
                panic!("{source}: shipped rule {rule:?} violates the domain: {violation}")
            });
        }
    };

    for &exchange in ALL_EXCHANGES {
        validate_all(&hours_for_exchange(exchange), &format!("{exchange:?}"));
        for epoch in epochs {
            validate_all(
                &hours_for_exchange_as_of(exchange, epoch),
                &format!("{exchange:?} as of {epoch}"),
            );
        }
    }

    for &(exchange, (year, month, day), tz) in HISTORICAL_CUTOVERS {
        let at = tz
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .earliest()
            .expect("cutover midnight resolves in the venue zone")
            .with_timezone(&Utc);
        for epoch in [at - Duration::seconds(1), at] {
            validate_all(
                &hours_for_exchange_as_of(exchange, epoch),
                &format!("{exchange:?} as of {epoch}"),
            );
        }
    }

    for &(exchange, (year, month, day, hour, minute, second)) in HISTORICAL_INSTANT_CUTOVERS {
        let at = Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .single()
            .expect("sourced UTC cutover resolves");
        for epoch in [at - Duration::nanoseconds(1), at] {
            validate_all(
                &hours_for_exchange_as_of(exchange, epoch),
                &format!("{exchange:?} as of {epoch}"),
            );
        }
    }

    // The generated enumeration ensures a newly added key cannot skip this
    // shipped-rule validation. A separate test owns the independent identity
    // and order expectation.
    for &key in MarketHoursKey::ALL {
        let profile = session_profile(key);
        for rule in profile.regular.iter().chain(profile.extended.iter()) {
            rule.validate().unwrap_or_else(|violation| {
                panic!("{key:?}: shipped rule {rule:?} violates the domain: {violation}")
            });
        }
    }
}

#[test]
fn every_equal_endpoint_rule_is_an_intentional_complete_day_session() {
    let sunday = [false, false, false, false, false, false, true];
    let monday = [true, false, false, false, false, false, false];
    let mut exchange_rules = 0_u8;

    for &exchange in ALL_EXCHANGES {
        let hours = hours_for_exchange(exchange);
        for rule in hours
            .regular
            .iter()
            .chain(hours.extended.iter())
            .filter(|rule| rule.open_ssm == rule.close_ssm)
        {
            assert!(rule.wraps_to_next_day());
            match exchange {
                Exchange::Iceus | Exchange::Iceeu | Exchange::IceEuropeCommodities => {
                    assert_eq!(rule.days, sunday);
                    assert_eq!(rule.open_ssm, 18 * 3600);
                }
                Exchange::IceAbuDhabi => {
                    assert_eq!(rule.days, monday);
                    assert_eq!(rule.open_ssm, 2 * 3600);
                }
                _ => panic!("unexpected equal-endpoint rule for {exchange:?}: {rule:?}"),
            }
            exchange_rules += 1;
        }
    }
    assert_eq!(exchange_rules, 4, "the scoped ICE full-local-day rules");

    let profile = exchange_hours::session_profile(exchange_hours::MarketHoursKey::IceUs);
    let equal_rules: Vec<_> = profile
        .regular
        .iter()
        .chain(profile.extended.iter())
        .filter(|rule| rule.open_ssm == rule.close_ssm)
        .collect();
    assert_eq!(equal_rules.len(), 1);
    assert_eq!(equal_rules[0].days, sunday);
    assert_eq!(equal_rules[0].open_ssm, 18 * 3600);
}
