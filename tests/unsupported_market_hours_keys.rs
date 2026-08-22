// SPDX-License-Identifier: MIT-0

//! Explicit rejection fence for futures families deferred from version 1.0.

use exchange_hours::MarketHoursKey;

const DEFERRED_FAMILY_NAMES: &[&str] = &[
    "globex_nikkei_225_dollar",
    "ice_us_dollar_index",
    "ice_us_sugar",
    "ice_us_coffee",
    "ice_us_cocoa",
    "ice_us_cotton",
    "ice_us_orange_juice",
    "eurex_fixed_income",
    "sgx_equity_index",
];

#[test]
fn deferred_family_names_are_rejected_instead_of_substituted() {
    for &name in DEFERRED_FAMILY_NAMES {
        let parse_error = name
            .parse::<MarketHoursKey>()
            .expect_err("a deferred family must not parse as a supported key");
        assert_eq!(parse_error.input(), name);

        let encoded_name = format!("\"{name}\"");
        assert!(
            serde_json::from_str::<MarketHoursKey>(&encoded_name).is_err(),
            "{name}: Serde must reject the deferred wire name"
        );
    }
}
