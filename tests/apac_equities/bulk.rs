// SPDX-License-Identifier: MIT-0

//! APAC bulk-builder and canonical-name contracts.

use super::prelude::*;

#[test]
fn apac_bulk_and_canonical_names_cover_every_new_venue() {
    let expected = [
        (Exchange::Asx, "asx"),
        (Exchange::TmxAustralia, "tmx_australia"),
        (Exchange::Nzx, "nzx"),
        (Exchange::Tse, "tse"),
        (Exchange::NseIndia, "nse_india"),
        (Exchange::BseIndia, "bse_india"),
        (Exchange::Hkex, "hkex"),
        (Exchange::SgxSecurities, "sgx_securities"),
        (Exchange::BursaMalaysia, "bursa_malaysia"),
        (Exchange::SetThailand, "set_thailand"),
        (Exchange::Idx, "idx"),
        (Exchange::Pse, "pse"),
        (Exchange::Hose, "hose"),
        (Exchange::Sse, "sse"),
        (Exchange::Szse, "szse"),
        (Exchange::Krx, "krx"),
        (Exchange::Twse, "twse"),
    ];
    let bulk = hours_for_apac_equities();
    let map = hours_map_apac_equities();
    assert_eq!(bulk.len(), expected.len());
    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.keys().copied().collect::<Vec<_>>(),
        expected
            .iter()
            .map(|&(exchange, _)| exchange)
            .collect::<Vec<_>>()
    );
    for ((exchange, name), hours) in expected.into_iter().zip(bulk) {
        assert_eq!(hours.exchange(), Some(exchange));
        assert_eq!(exchange.as_str(), name);
        assert_eq!(
            serde_json::to_string(&exchange).expect("serializes"),
            format!("\"{name}\"")
        );
        assert_eq!(
            serde_json::from_str::<Exchange>(&format!("\"{name}\"")).expect("parses"),
            exchange
        );
    }
}
