// SPDX-License-Identifier: MIT-0

//! Stable regional bulk membership and exchange-name wiring.

use super::prelude::*;

#[test]
fn global_bulk_and_names_are_stable() {
    let expected = [
        (Exchange::BorsaIstanbul, "borsa_istanbul"),
        (Exchange::Tsx, "tsx"),
        (Exchange::Jse, "jse"),
        (Exchange::Tadawul, "tadawul"),
        (Exchange::B3, "b3"),
        (Exchange::Bmv, "bmv"),
    ];
    let bulk = hours_for_global_equities(
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let map = hours_map_global_equities(
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
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
