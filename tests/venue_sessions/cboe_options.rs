// SPDX-License-Identifier: MIT-0

//! US listed-equity-options product-family boundaries and launch history.

use super::prelude::*;

const ALL_LISTED_EQUITY_OPTIONS: &[Exchange] = &[
    Exchange::CboeOptionsC1,
    Exchange::CboeC2Options,
    Exchange::CboeBzxOptions,
    Exchange::CboeEdgxOptions,
    Exchange::NyseArcaOptions,
    Exchange::NyseAmericanOptions,
    Exchange::NasdaqPhlx,
    Exchange::NasdaqIse,
    Exchange::NasdaqNom,
    Exchange::NasdaqMrx,
    Exchange::NasdaqGemx,
    Exchange::NasdaqBxOptions,
    Exchange::MiaxOptions,
    Exchange::MiaxEmeraldOptions,
    Exchange::MiaxPearlOptions,
    Exchange::MiaxSapphireOptions,
    Exchange::BoxOptions,
    Exchange::MemxOptions,
];

const OPEN_0600: &[Exchange] = &[
    Exchange::NyseArcaOptions,
    Exchange::NyseAmericanOptions,
    Exchange::NasdaqIse,
    Exchange::NasdaqMrx,
    Exchange::NasdaqGemx,
];

const OPEN_0700: &[Exchange] = &[Exchange::BoxOptions];

const OPEN_0730: &[Exchange] = &[
    Exchange::CboeOptionsC1,
    Exchange::CboeC2Options,
    Exchange::CboeBzxOptions,
    Exchange::CboeEdgxOptions,
    Exchange::NasdaqPhlx,
    Exchange::NasdaqNom,
    Exchange::NasdaqBxOptions,
    Exchange::MiaxOptions,
    Exchange::MiaxEmeraldOptions,
    Exchange::MiaxPearlOptions,
    Exchange::MiaxSapphireOptions,
];

// Generic pre-open order acceptance is Extended even though execution begins
// at 09:30. Product-specific executable sessions remain outside this ordinary
// individual-stock-options family.
#[test]
fn listed_equity_options_include_each_current_order_acceptance_edge() {
    for (exchanges, open) in [
        (OPEN_0600, (6, 0, 0)),
        (OPEN_0700, (7, 0, 0)),
        (OPEN_0730, (7, 30, 0)),
    ] {
        for &exchange in exchanges {
            let hours = hours_for_exchange(
                exchange,
                chrono::DateTime::<chrono::Utc>::UNIX_EPOCH
                    + chrono::Duration::seconds(1_787_400_000),
            );
            let instant = et((2026, 4, 20), open);

            assert!(
                !hours.is_open(instant - chrono::Duration::seconds(1)),
                "{exchange:?}"
            );
            // Each venue's order-acceptance edge: orders only, no matching.
            assert!(hours.is_order_entry_only(instant), "{exchange:?}");
            assert!(
                hours.is_order_entry_only(et((2026, 4, 20), (9, 29, 59))),
                "{exchange:?}"
            );
            assert!(
                hours.is_open_regular(et((2026, 4, 20), (9, 30, 0))),
                "{exchange:?}"
            );
            assert!(
                !hours.is_open(et((2026, 4, 20), (16, 0, 0))),
                "{exchange:?}"
            );
            assert!(
                !hours.is_open(et((2026, 4, 18), (12, 0, 0))),
                "{exchange:?}"
            );
        }
    }

    let memx = hours_for_exchange(
        Exchange::MemxOptions,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!memx.is_open(et((2026, 4, 20), (9, 29, 59))));
    assert!(memx.is_open_regular(et((2026, 4, 20), (9, 30, 0))));
    assert!(memx.extended.is_empty());
}

#[test]
fn every_listed_options_identity_is_covered_by_a_current_group() {
    for &exchange in ALL_LISTED_EQUITY_OPTIONS {
        let hours = hours_for_exchange(
            exchange,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );

        assert!(
            hours.is_open_regular(et((2026, 4, 20), (15, 59, 59))),
            "{exchange:?}"
        );
    }
}

#[test]
fn partial_options_history_does_not_invent_a_queue_onset() {
    // C1's current 07:30 queue is primary-supported, but its exact onset day is
    // not. The fixed current snapshot includes it; the as-of selector keeps the
    // sourced execution-only history instead of assigning an inferred date.
    let current = hours_for_exchange(
        Exchange::CboeOptionsC1,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let historical = hours_for_exchange(Exchange::CboeOptionsC1, et((2026, 4, 20), (12, 0, 0)));

    assert!(current.is_order_entry_only(et((2026, 4, 20), (7, 30, 0))));
    assert!(!historical.is_open(et((2026, 4, 20), (7, 30, 0))));
    assert!(historical.is_open_regular(et((2026, 4, 20), (9, 30, 0))));
}

// These venues all traded this product family before the January-2010 audit
// floor. Their exact execution grid is retained; no undated current queue is
// backfilled into this historical surface.
// https://www.sec.gov/rules/sro/cboe/2006/34-53246.pdf
// https://www.sec.gov/rules/sro/pcx/34-53249.pdf
// https://www.sec.gov/rules/sro/amex/2006/34-53244.pdf
// https://www.sec.gov/rules/sro/phlx/34-53247.pdf
// https://www.sec.gov/rules/sro/ise/2006/34-53248.pdf
// https://www.sec.gov/rules/sro/bse/2006/34-53245.pdf
// https://www.sec.gov/rules/sro/nasdaq/2008/34-57478.pdf
// https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2008-001
#[test]
fn pre_floor_options_venues_retain_the_sourced_2010_grid() {
    for exchange in [
        Exchange::CboeOptionsC1,
        Exchange::NyseArcaOptions,
        Exchange::NyseAmericanOptions,
        Exchange::NasdaqPhlx,
        Exchange::NasdaqIse,
        Exchange::NasdaqNom,
        Exchange::BoxOptions,
    ] {
        let hours = hours_for_exchange(exchange, et((2010, 1, 4), (12, 0, 0)));

        assert!(
            !hours.is_open(et((2010, 1, 4), (9, 29, 59))),
            "{exchange:?}"
        );
        assert!(
            hours.is_open_regular(et((2010, 1, 4), (9, 30, 0))),
            "{exchange:?}"
        );
        assert!(!hours.is_open(et((2010, 1, 4), (16, 0, 0))), "{exchange:?}");
    }
}

// Exact production launches are stated in the operator/regulator notices
// cited beside the production revisions in options.rs.
#[test]
fn post_floor_options_venues_are_closed_before_their_launches() {
    for (exchange, date) in [
        (Exchange::CboeBzxOptions, (2010, 2, 26)),
        (Exchange::CboeC2Options, (2010, 10, 29)),
        (Exchange::CboeEdgxOptions, (2015, 11, 2)),
        (Exchange::NasdaqBxOptions, (2012, 6, 29)),
        (Exchange::NasdaqGemx, (2013, 8, 5)),
        (Exchange::NasdaqMrx, (2016, 2, 16)),
        (Exchange::MiaxOptions, (2012, 12, 7)),
        (Exchange::MiaxPearlOptions, (2017, 2, 6)),
        (Exchange::MiaxEmeraldOptions, (2019, 3, 1)),
        (Exchange::MiaxSapphireOptions, (2024, 8, 12)),
        (Exchange::MemxOptions, (2023, 9, 27)),
    ] {
        let boundary = et(date, (0, 0, 0));
        let before = hours_for_exchange(exchange, boundary - chrono::Duration::seconds(1));
        let launched = hours_for_exchange(exchange, boundary);

        assert!(before.regular.is_empty(), "{exchange:?}");
        assert!(before.extended.is_empty(), "{exchange:?}");
        assert!(
            launched.is_open_regular(et(date, (9, 30, 0))),
            "{exchange:?}"
        );
        assert!(!launched.is_open(et(date, (16, 0, 0))), "{exchange:?}");
    }
}
