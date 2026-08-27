// SPDX-License-Identifier: MIT-0

//! Anti-substitution fence for ambiguous product-family names.
//!
//! Every family deferred from an earlier draft now ships as a sourced key, with
//! one deliberate exception: SGX equity-index products do not share a grid, so
//! there is no single `sgx_equity_index` family to select. Modelling one would
//! answer a Taiwan contract with Singapore's close.

use exchange_hours::MarketHoursKey;

/// Names that must resolve, because their profiles are now sourced.
const SUPPORTED_FAMILY_NAMES: &[&str] = &[
    "globex_nikkei_225_dollar",
    "ice_us_dollar_index",
    "ice_us_sugar",
    "ice_us_coffee",
    "ice_us_cocoa",
    "ice_us_cotton",
    "ice_us_orange_juice",
    "eurex_fixed_income",
    "sgx_equity_index_japan",
    "sgx_equity_index_china",
    "sgx_equity_index_singapore",
    "sgx_equity_index_taiwan",
    "sgx_equity_index_ntr_usd",
];

/// Names that must never resolve, because no single venue-wide grid exists.
const AMBIGUOUS_FAMILY_NAMES: &[&str] = &["sgx_equity_index"];

#[test]
fn sourced_family_names_resolve_through_parse_and_serde() {
    for &name in SUPPORTED_FAMILY_NAMES {
        let key = name
            .parse::<MarketHoursKey>()
            .expect("a sourced family must parse");
        assert_eq!(key.as_str(), name, "{name}: canonical name must round-trip");

        let encoded_name = format!("\"{name}\"");
        let decoded = serde_json::from_str::<MarketHoursKey>(&encoded_name)
            .expect("a sourced family must deserialize");
        assert_eq!(decoded, key, "{name}: serde and FromStr must agree");
    }
}

#[test]
fn ambiguous_family_names_are_rejected_instead_of_substituted() {
    for &name in AMBIGUOUS_FAMILY_NAMES {
        let parse_error = name
            .parse::<MarketHoursKey>()
            .expect_err("an ambiguous family must not parse as a supported key");
        assert_eq!(parse_error.input(), name);

        let encoded_name = format!("\"{name}\"");
        assert!(
            serde_json::from_str::<MarketHoursKey>(&encoded_name).is_err(),
            "{name}: Serde must reject the ambiguous wire name"
        );
    }
}

#[test]
fn sgx_equity_index_grids_are_genuinely_distinct() {
    // The justification for refusing a single `sgx_equity_index` key: the five
    // grids disagree, so any substitution would return another market's hours.
    let opens: Vec<_> = SUPPORTED_FAMILY_NAMES
        .iter()
        .filter(|name| name.starts_with("sgx_equity_index_"))
        .map(|name| {
            let key = name.parse::<MarketHoursKey>().expect("sgx key parses");
            exchange_hours::hours_for_market_hours_key(
                key,
                chrono::DateTime::<chrono::Utc>::UNIX_EPOCH
                    + chrono::Duration::seconds(1_787_400_000),
            )
        })
        .collect();

    assert_eq!(opens.len(), 5, "all five SGX grids must be present");
}
