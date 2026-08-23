// SPDX-License-Identifier: MIT-0

//! LTSE, 24X, and TXSE public-surface schedule contracts.

use super::prelude::*;

#[test]
fn independent_us_exchanges_publish_their_current_envelopes() {
    for (exchange, open, close) in [
        (Exchange::Ltse, (8, 0, 0), (17, 0, 0)),
        (Exchange::TwentyFourX, (4, 0, 0), (20, 0, 0)),
        (Exchange::Txse, (8, 0, 0), (17, 0, 0)),
    ] {
        let hours = hours_for_exchange(exchange);
        let open = et((2026, 8, 17), open);
        let close = et((2026, 8, 17), close);

        assert!(
            !hours.is_open(open - chrono::Duration::seconds(1)),
            "{exchange:?}"
        );
        assert!(hours.is_open_extended(open), "{exchange:?}");
        assert!(
            hours.is_open_extended(et((2026, 8, 17), (9, 29, 59))),
            "{exchange:?}"
        );
        assert!(
            hours.is_open_regular(et((2026, 8, 17), (9, 30, 0))),
            "{exchange:?}"
        );
        assert!(
            hours.is_open_extended(et((2026, 8, 17), (16, 0, 0))),
            "{exchange:?}"
        );
        assert!(
            hours.is_open(close - chrono::Duration::seconds(1)),
            "{exchange:?}"
        );
        assert!(!hours.is_open(close), "{exchange:?}");
        assert!(
            !hours.is_open(et((2026, 8, 22), (12, 0, 0))),
            "{exchange:?}"
        );
    }
}

#[test]
fn independent_us_exchange_launches_are_not_backfilled() {
    for (exchange, date, open) in [
        (Exchange::Ltse, (2020, 8, 28), (8, 0, 0)),
        (Exchange::TwentyFourX, (2025, 10, 14), (4, 0, 0)),
        (Exchange::Txse, (2026, 7, 10), (8, 0, 0)),
    ] {
        let boundary = et(date, (0, 0, 0));
        let before = hours_for_exchange_as_of(exchange, boundary - chrono::Duration::seconds(1));
        let launched = hours_for_exchange_as_of(exchange, boundary);

        assert!(before.regular.is_empty(), "{exchange:?}");
        assert!(before.extended.is_empty(), "{exchange:?}");
        assert!(launched.is_open_extended(et(date, open)), "{exchange:?}");
        assert!(
            launched.is_open_regular(et(date, (9, 30, 0))),
            "{exchange:?}"
        );
    }

    // TXSE's July 6–9 connectivity/test-symbol period was not the production
    // NMS-stock exchange schedule represented by this identity.
    assert!(
        !hours_for_exchange_as_of(Exchange::Txse, et((2026, 7, 9), (12, 0, 0)))
            .is_open(et((2026, 7, 9), (12, 0, 0)))
    );
}

#[test]
fn twenty_four_x_conditional_overnight_session_is_not_encoded() {
    // The SEC order leaves 21:00–04:00 commencement dependent on the SIP and a
    // later filing. Fixed, future-as-of, and date-aware queries retain 04:00–20:00.
    let sunday_night = et((2026, 12, 6), (21, 0, 0));
    let fixed = hours_for_exchange(Exchange::TwentyFourX);
    let future = hours_for_exchange_as_of(Exchange::TwentyFourX, et((2026, 12, 7), (12, 0, 0)));

    assert!(!fixed.is_open(sunday_night));
    assert!(!future.is_open(sunday_night));
    assert!(!calendar_for_exchange(Exchange::TwentyFourX).is_open(sunday_night));
}

#[test]
fn independent_us_exchange_wires_and_bulk_order_are_stable() {
    for (exchange, wire) in [
        (Exchange::Ltse, "ltse"),
        (Exchange::TwentyFourX, "24x"),
        (Exchange::Txse, "txse"),
    ] {
        assert_eq!(exchange.as_str(), wire);
        assert_eq!(
            serde_json::to_string(&exchange).expect("exchange serializes"),
            format!("\"{wire}\"")
        );
        assert_eq!(
            serde_json::from_str::<Exchange>(&format!("\"{wire}\""))
                .expect("canonical exchange wire deserializes"),
            exchange
        );
    }

    let exchanges: Vec<_> = hours_for_us_equities()
        .into_iter()
        .filter_map(|hours| hours.exchange())
        .collect();
    let expected = [
        Exchange::Nasdaq,
        Exchange::NasdaqBx,
        Exchange::NasdaqPsx,
        Exchange::CboeBzx,
        Exchange::CboeByx,
        Exchange::CboeEdga,
        Exchange::CboeEdgx,
        Exchange::Nyse,
        Exchange::NyseArca,
        Exchange::NyseAmerican,
        Exchange::NyseNational,
        Exchange::NyseTexas,
        Exchange::MemxEq,
        Exchange::MiaxPearlEq,
        Exchange::Iex,
        Exchange::Ltse,
        Exchange::TwentyFourX,
        Exchange::Txse,
        Exchange::BlueOceanAts,
    ];
    assert_eq!(exchanges.as_slice(), expected.as_slice());
}
