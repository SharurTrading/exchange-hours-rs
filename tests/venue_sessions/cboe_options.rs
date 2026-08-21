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

// The venue rules consistently distinguish ordinary options on individual
// stocks from ETF, ETN, index, FLEX, and designated extended-hours products.
// The scoped family's published regular-session envelope is 09:30–16:00 ET;
// venue-specific primary sources are cited beside the production table.
#[test]
fn listed_equity_options_have_one_exact_product_family_scope() {
    for &exchange in ALL_LISTED_EQUITY_OPTIONS {
        let hours = hours_for_exchange(exchange);

        assert!(
            !hours.is_open(et((2026, 4, 20), (9, 29, 59))),
            "{exchange:?}"
        );
        assert!(
            hours.is_open_regular(et((2026, 4, 20), (9, 30, 0))),
            "{exchange:?}"
        );
        assert!(
            hours.is_open_regular(et((2026, 4, 20), (15, 59, 59))),
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
        assert!(hours.extended.is_empty(), "{exchange:?}");
    }
}

// These venues all traded this product family before the January-2010 audit
// floor. Each identity has its own regulator/operator baseline rather than
// inheriting another venue's coincident hours.
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
        let hours = hours_for_exchange_as_of(exchange, et((2010, 1, 4), (12, 0, 0)));

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
        let before = hours_for_exchange_as_of(exchange, boundary - chrono::Duration::seconds(1));
        let launched = hours_for_exchange_as_of(exchange, boundary);

        assert!(before.regular.is_empty(), "{exchange:?}");
        assert!(before.extended.is_empty(), "{exchange:?}");
        assert!(
            launched.is_open_regular(et(date, (9, 30, 0))),
            "{exchange:?}"
        );
        assert!(!launched.is_open(et(date, (16, 0, 0))), "{exchange:?}");
    }
}
