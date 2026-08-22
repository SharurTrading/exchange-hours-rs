// SPDX-License-Identifier: MIT-0

//! Exchange enumeration and stable-name identity contracts.

use super::identity_expectations::*;
use super::prelude::*;

#[test]
fn all_exchanges_matches_the_crates_own_list() {
    // `Exchange` is `#[non_exhaustive]`, so this crate cannot hold an
    // exhaustive match as a coverage fence. It does not need one: the
    // library's `exchanges!` macro generates the enum, `Exchange::ALL`, and
    // `as_str` from one table, so a variant cannot exist while missing from
    // `ALL`. What this test pins is the expectation side: the hand-maintained
    // list above agrees element-by-element (and in order) with the generated
    // `ALL`, and both carry the declared count — catching a row dropped from
    // or reordered in the library's table.
    assert_eq!(
        ALL_EXCHANGES,
        Exchange::ALL,
        "the test's independent list and Exchange::ALL disagree; a new \
         variant was added to one but not the other"
    );
    assert_eq!(
        ALL_EXCHANGES.len(),
        EXCHANGE_VARIANT_COUNT,
        "ALL_EXCHANGES lost or gained an entry; update it together with \
         Exchange::ALL, Exchange::as_str, and EXCHANGE_VARIANT_COUNT or the \
         grids skip a venue"
    );
    assert!(
        Exchange::ALL.windows(2).all(|pair| pair[0] < pair[1]),
        "Exchange::ALL must stay in declaration (Ord) order with no duplicates"
    );
}

#[test]
fn every_exchange_name_round_trips_through_serde_display_and_from_str() {
    // One venue has exactly one snake_case name, shared by serde, `as_str`,
    // `Display`, and `FromStr`. `as_str` is a hand-written second copy of the
    // serde rename table, so every variant is checked both ways; `FromStr`
    // searches `Exchange::ALL` by `as_str`, so its agreement is implied but
    // pinned anyway.
    for &exchange in Exchange::ALL {
        let name = exchange.as_str();
        let serde_form = serde_json::to_value(exchange).expect("serializes");
        assert_eq!(
            serde_form,
            serde_json::Value::String(name.to_owned()),
            "{exchange:?}: as_str and the serde wire form disagree"
        );
        assert_eq!(
            exchange.to_string(),
            name,
            "{exchange:?}: Display and as_str disagree"
        );
        assert_eq!(
            name.parse::<Exchange>(),
            Ok(exchange),
            "{exchange:?}: FromStr does not round-trip its own name"
        );
    }
}

#[test]
fn from_str_rejects_unrecognized_names_instead_of_defaulting() {
    // A typo must surface as an error a caller can see — never silently
    // become `Exchange::Unknown`. `"unknown"` itself parses, because that is
    // the canonical name a caller uses to *choose* the fallback explicitly.
    for bad in ["", "CME", "nyse-arca", "cme ", "totally_made_up"] {
        let parsed = bad.parse::<Exchange>();
        assert!(parsed.is_err(), "{bad:?} must not parse, got {parsed:?}");
        let error = parsed.expect_err("checked above");
        assert_eq!(error.input(), bad, "the error must carry the bad input");
        assert!(
            !error.to_string().is_empty(),
            "the error must render a message"
        );
    }
    assert_eq!("unknown".parse::<Exchange>(), Ok(Exchange::Unknown));
}
