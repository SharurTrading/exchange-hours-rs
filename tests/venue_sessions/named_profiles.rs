// SPDX-License-Identifier: MIT-0

//! Named futures-session profile contracts.

use super::prelude::*;
use serde_test::{Configure, Token, assert_de_tokens_error, assert_tokens};

const EXPECTED_MARKET_HOURS_KEYS: &[(MarketHoursKey, &str)] = &[
    (MarketHoursKey::GlobexEquityIndex, "globex_equity_index"),
    (MarketHoursKey::GlobexEnergy, "globex_energy"),
    (MarketHoursKey::GlobexGrains, "globex_grains"),
    (MarketHoursKey::GlobexFx, "globex_fx"),
    (MarketHoursKey::GlobexInterestRates, "globex_interest_rates"),
    (MarketHoursKey::GlobexLivestock, "globex_livestock"),
    (
        MarketHoursKey::GlobexCryptocurrency,
        "globex_cryptocurrency",
    ),
    (MarketHoursKey::CfeVix, "cfe_vix"),
    (MarketHoursKey::Eurex, "eurex"),
    (MarketHoursKey::IceUs, "ice_us"),
    (MarketHoursKey::IceUsSugar, "ice_us_sugar"),
    (MarketHoursKey::IceUsCoffee, "ice_us_coffee"),
    (MarketHoursKey::IceUsCocoa, "ice_us_cocoa"),
    (MarketHoursKey::IceUsCotton, "ice_us_cotton"),
    (MarketHoursKey::IceUsOrangeJuice, "ice_us_orange_juice"),
    (MarketHoursKey::IceUsDollarIndex, "ice_us_dollar_index"),
    (
        MarketHoursKey::GlobexNikkei225Dollar,
        "globex_nikkei_225_dollar",
    ),
    (MarketHoursKey::EurexFixedIncome, "eurex_fixed_income"),
    (
        MarketHoursKey::SgxEquityIndexJapan,
        "sgx_equity_index_japan",
    ),
    (
        MarketHoursKey::SgxEquityIndexChina,
        "sgx_equity_index_china",
    ),
    (
        MarketHoursKey::SgxEquityIndexSingapore,
        "sgx_equity_index_singapore",
    ),
    (
        MarketHoursKey::SgxEquityIndexTaiwan,
        "sgx_equity_index_taiwan",
    ),
    (
        MarketHoursKey::SgxEquityIndexNtrUsd,
        "sgx_equity_index_ntr_usd",
    ),
    (MarketHoursKey::Sgx, "sgx"),
    (MarketHoursKey::AlwaysOpen, "always_open"),
];

// ---------------------------------------------------------------------------
// Named futures session profiles.
// ---------------------------------------------------------------------------

#[test]
fn market_hours_key_all_matches_the_independent_expected_list() {
    let expected_keys = EXPECTED_MARKET_HOURS_KEYS
        .iter()
        .map(|&(key, _name)| key)
        .collect::<Vec<_>>();
    assert_eq!(MarketHoursKey::ALL, expected_keys);
    assert!(
        MarketHoursKey::ALL.windows(2).all(|pair| pair[0] < pair[1]),
        "MarketHoursKey::ALL must stay in declaration/Ord order"
    );
}

#[test]
fn all_market_hours_keys_return_profiles() {
    for &key in MarketHoursKey::ALL {
        let profile = session_profile(key);
        let total_rules = profile.regular.len() + profile.extended.len();
        assert!(
            total_rules > 0,
            "{key:?} profile must have at least one session rule"
        );
    }
}

#[test]
fn every_market_hours_key_uses_its_canonical_string_in_every_serde_format() {
    for &(key, name) in EXPECTED_MARKET_HOURS_KEYS {
        assert_tokens(&key.readable(), &[Token::Str(name)]);
        assert_tokens(&key.compact(), &[Token::Str(name)]);
        assert_eq!(key.as_str(), name, "{key:?}: canonical name drifted");
        assert_eq!(key.to_string(), name, "{key:?}: Display drifted");
        assert_eq!(name.parse::<MarketHoursKey>(), Ok(key));
    }
}

#[test]
fn market_hours_key_from_str_rejects_noncanonical_names() {
    for input in ["", "EUREX", "globex-energy", "sgx ", "unknown"] {
        let error: ParseMarketHoursKeyError = input
            .parse::<MarketHoursKey>()
            .expect_err("noncanonical key must be rejected");
        assert_eq!(error.input(), input);
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn market_hours_key_serde_rejects_ordinal_indices() {
    assert_de_tokens_error::<MarketHoursKey>(
        &[Token::U32(0)],
        "invalid type: integer `0`, expected a canonical market-hours key",
    );
}

#[test]
fn futures_session_profile_globex_equity_index_respects_daily_break() {
    let profile = session_profile(MarketHoursKey::GlobexEquityIndex);

    assert!(
        profile.is_open(ct((2026, 4, 20), (15, 45, 0))),
        "Globex equity-index profile is open during the 15:30-16:00 CT window"
    );
    assert!(
        !profile.is_open(ct((2026, 4, 20), (16, 30, 0))),
        "Globex equity-index profile is closed during the 16:00-16:45 CT maintenance gap"
    );
    assert!(
        profile.is_open(ct((2026, 4, 20), (17, 0, 0))),
        "Globex equity-index matching resumes at 17:00 CT"
    );
}

#[test]
fn futures_session_profile_globex_energy_uses_wrap_session() {
    let profile = session_profile(MarketHoursKey::GlobexEnergy);

    assert!(
        profile.is_open(ct((2026, 4, 20), (15, 0, 0))),
        "Globex energy profile is open before the 16:00 CT daily close"
    );
    assert!(
        !profile.is_open(ct((2026, 4, 20), (16, 30, 0))),
        "Globex energy profile is closed during the 16:00-16:45 CT maintenance gap"
    );
}

#[test]
fn futures_session_profile_globex_fx_matches_current_major_cme_fx_grid() {
    let profile = session_profile(MarketHoursKey::GlobexFx);

    assert!(!profile.is_open(ct((2026, 4, 19), (15, 59, 59))));
    assert!(
        profile.is_order_entry_only(ct((2026, 4, 19), (16, 0, 0))),
        "CME FX enters its Sunday Pre-Open at 16:00 CT - order entry, not matching"
    );
    assert!(
        profile.is_open(ct((2026, 4, 19), (17, 0, 0))),
        "CME FX matching begins Sunday at 17:00 CT"
    );
    assert!(
        profile.is_open(ct((2026, 4, 20), (15, 59, 59))),
        "CME FX remains open until the 16:00 CT daily break"
    );
    assert!(
        !profile.is_open(ct((2026, 4, 20), (16, 0, 0))),
        "CME FX close is end-exclusive at 16:00 CT"
    );
    assert!(
        profile.is_open(ct((2026, 4, 20), (17, 0, 0))),
        "CME FX matching resumes after Pre-Open"
    );
    assert!(
        !profile.is_open(ct((2026, 4, 25), (12, 0, 0))),
        "CME FX has a true weekend close"
    );
    assert_eq!(
        serde_json::to_string(&MarketHoursKey::GlobexFx).expect("key serializes"),
        r#""globex_fx""#
    );
}

#[test]
fn futures_session_profile_always_open_has_one_all_days_rule() {
    let profile = session_profile(MarketHoursKey::AlwaysOpen);
    let total_rules = profile.regular.len() + profile.extended.len();

    assert_eq!(total_rules, 1, "AlwaysOpen should have exactly one rule");
    assert!(
        profile.extended.is_empty(),
        "AlwaysOpen has no extended rules"
    );
    assert!(
        !profile.has_daily_close,
        "AlwaysOpen must not have a daily close"
    );
    assert!(
        !profile.has_weekend_close,
        "AlwaysOpen must not have a weekend close"
    );

    let rule = &profile.regular[0];
    assert_eq!(
        rule.days, [true; 7],
        "AlwaysOpen rule must activate all days"
    );
    assert_eq!(rule.open_ssm, 0);
    assert_eq!(rule.close_ssm, 24 * 3600);
}

#[test]
fn dated_market_hours_keys_reuse_cme_group_and_cfe_histories() {
    let equity_before = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexEquityIndex,
        ct((2012, 11, 17), (12, 0, 0)),
    );
    let equity_after = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexEquityIndex,
        ct((2012, 11, 18), (0, 0, 0)),
    );
    assert!(equity_before.is_open_extended(ct((2012, 11, 19), (16, 29, 59))));
    assert!(!equity_before.is_open(ct((2012, 11, 19), (16, 30, 0))));
    assert!(equity_after.is_open_extended(ct((2012, 11, 19), (15, 45, 0))));
    assert!(!equity_after.is_open(ct((2012, 11, 19), (16, 15, 0))));

    let equity_close_before = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexEquityIndex,
        ct((2015, 9, 19), (12, 0, 0)),
    );
    let equity_close_after = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexEquityIndex,
        ct((2015, 9, 20), (0, 0, 0)),
    );
    assert!(equity_close_before.is_open(ct((2015, 9, 21), (16, 14, 59))));
    assert!(!equity_close_after.is_open(ct((2015, 9, 21), (16, 0, 0))));

    let energy_before = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexEnergy,
        ct((2015, 9, 19), (12, 0, 0)),
    );
    let energy_after = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexEnergy,
        ct((2015, 9, 20), (0, 0, 0)),
    );
    assert!(energy_before.is_open(ct((2015, 9, 21), (16, 14, 59))));
    assert!(!energy_after.is_open(ct((2015, 9, 21), (16, 0, 0))));
    assert!(
        !hours_for_market_hours_key(MarketHoursKey::GlobexEnergy)
            .is_open(ct((2015, 9, 21), (16, 5, 0))),
        "the pre-existing date-free function remains a current snapshot"
    );

    let grains_before = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexGrains,
        ct((2012, 5, 19), (12, 0, 0)),
    );
    let grains_after = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexGrains,
        ct((2012, 5, 20), (0, 0, 0)),
    );
    assert!(grains_before.is_order_entry_only(ct((2012, 5, 20), (17, 0, 0))));
    assert!(grains_after.is_open_extended(ct((2012, 5, 20), (17, 0, 0))));
    assert!(grains_before.is_order_entry_only(ct((2012, 5, 21), (14, 30, 0))));
    assert!(!grains_after.is_open(ct((2012, 5, 21), (14, 30, 0))));

    let cfe_before =
        hours_for_market_hours_key_as_of(MarketHoursKey::CfeVix, ct((2010, 12, 9), (12, 0, 0)));
    let cfe_after =
        hours_for_market_hours_key_as_of(MarketHoursKey::CfeVix, ct((2010, 12, 10), (0, 0, 0)));
    assert!(!cfe_before.is_open(ct((2010, 12, 10), (7, 20, 0))));
    assert!(cfe_after.is_open_extended(ct((2010, 12, 10), (7, 20, 0))));
    assert_eq!(cfe_after.exchange(), Some(Exchange::Unknown));

    let cfe_weekday_queue_before =
        hours_for_market_hours_key_as_of(MarketHoursKey::CfeVix, ct((2013, 10, 27), (12, 0, 0)));
    let cfe_weekday_queue_after =
        hours_for_market_hours_key_as_of(MarketHoursKey::CfeVix, ct((2013, 10, 28), (0, 0, 0)));
    assert!(!cfe_weekday_queue_before.is_open(ct((2013, 10, 28), (15, 29, 0))));
    assert!(cfe_weekday_queue_after.is_order_entry_only(ct((2013, 10, 28), (15, 29, 0))));

    let cfe_sunday_queue_before =
        hours_for_market_hours_key_as_of(MarketHoursKey::CfeVix, ct((2014, 6, 21), (12, 0, 0)));
    let cfe_sunday_queue_after =
        hours_for_market_hours_key_as_of(MarketHoursKey::CfeVix, ct((2014, 6, 22), (0, 0, 0)));
    assert!(!cfe_sunday_queue_before.is_open(ct((2014, 6, 22), (16, 15, 0))));
    assert!(!cfe_sunday_queue_after.is_open(ct((2014, 6, 22), (16, 14, 59))));
    assert!(cfe_sunday_queue_after.is_order_entry_only(ct((2014, 6, 22), (16, 15, 0))));
}

#[test]
fn dated_market_hours_keys_reuse_international_product_launches() {
    let eurex_before =
        hours_for_market_hours_key_as_of(MarketHoursKey::Eurex, cet((2018, 12, 9), (12, 0, 0)));
    let eurex_after =
        hours_for_market_hours_key_as_of(MarketHoursKey::Eurex, cet((2018, 12, 10), (0, 0, 0)));
    assert!(!eurex_before.is_open(cet((2018, 12, 10), (5, 0, 0))));
    assert!(eurex_after.is_order_entry_only(cet((2018, 12, 10), (1, 0, 0))));
    assert!(eurex_after.is_open_regular(cet((2018, 12, 10), (5, 0, 0))));

    let ice_before =
        hours_for_market_hours_key_as_of(MarketHoursKey::IceUs, et((2017, 11, 6), (12, 0, 0)));
    let ice_launch =
        hours_for_market_hours_key_as_of(MarketHoursKey::IceUs, et((2017, 11, 7), (0, 0, 0)));
    assert!(!ice_before.is_open(et((2017, 11, 7), (20, 0, 0))));
    assert!(ice_launch.is_open_regular(et((2017, 11, 7), (20, 0, 0))));

    let sgx_before =
        hours_for_market_hours_key_as_of(MarketHoursKey::Sgx, sgt((2024, 7, 28), (12, 0, 0)));
    let sgx_after =
        hours_for_market_hours_key_as_of(MarketHoursKey::Sgx, sgt((2024, 7, 29), (0, 0, 0)));
    assert!(!sgx_before.is_open(sgt((2024, 7, 29), (7, 25, 0))));
    assert!(sgx_after.is_open_regular(sgt((2024, 7, 29), (7, 25, 0))));
}

#[test]
fn synthetic_key_without_an_in_scope_revision_returns_the_current_snapshot() {
    let continuous_2010 =
        hours_for_market_hours_key_as_of(MarketHoursKey::AlwaysOpen, utc((2010, 1, 4), (12, 0, 0)));
    assert_eq!(
        continuous_2010,
        hours_for_market_hours_key(MarketHoursKey::AlwaysOpen)
    );
}
