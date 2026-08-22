// SPDX-License-Identifier: MIT-0

//! Exchange enumeration and stable-name identity contracts.

use super::identity_expectations::*;
use super::prelude::*;
use serde_test::{Configure, Token, assert_de_tokens_error, assert_tokens};

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
    // `Display`, and `FromStr`. The token assertion is format-neutral: it pins
    // serde to its string data model instead of a derive-generated enum
    // variant or ordinal. The macro generates the serde mapping and `as_str`
    // from the same table; `FromStr` searches `Exchange::ALL` by `as_str`.
    for &exchange in Exchange::ALL {
        let name = exchange.as_str();
        assert_tokens(&exchange.readable(), &[Token::Str(name)]);
        assert_tokens(&exchange.compact(), &[Token::Str(name)]);
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
fn serde_rejects_the_retired_intelligentcross_wire_name() {
    // This removal was deliberately authorized at the 1.0 boundary and is
    // documented under CHANGELOG [1.0.0] / Removed. There is no replacement Exchange
    // identity; rejection prevents persisted IQX values from being silently
    // reinterpreted as another venue or as Exchange::Unknown.
    const RETIRED: &str = "intelligentcross_iqx";

    let expected_names = Exchange::ALL
        .iter()
        .map(|exchange| format!("`{}`", exchange.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    let expected_error = format!("unknown variant `{RETIRED}`, expected one of {expected_names}");

    assert_de_tokens_error::<Exchange>(&[Token::Str(RETIRED)], &expected_error);
    assert!(
        RETIRED.parse::<Exchange>().is_err(),
        "the retired wire name must not parse through FromStr"
    );
}

#[test]
fn serde_rejects_ordinal_exchange_indices() {
    assert_de_tokens_error::<Exchange>(
        &[Token::U32(0)],
        "invalid type: integer `0`, expected a canonical exchange name",
    );
}

#[test]
fn from_str_rejects_unrecognized_names_instead_of_defaulting() {
    // A typo must surface as an error a caller can see — never silently
    // become `Exchange::Unknown`. `"unknown"` itself parses, because that is
    // the canonical name a caller uses to *choose* the fallback explicitly.
    for bad in [
        "",
        "CME",
        "nyse-arca",
        "cme ",
        "totally_made_up",
        "intelligentcross_iqx",
    ] {
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
