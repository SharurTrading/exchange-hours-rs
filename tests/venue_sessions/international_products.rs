// SPDX-License-Identifier: MIT-0

//! Product-scoped international derivatives histories.

use super::prelude::*;

#[test]
fn eex_nordic_zonal_power_is_closed_before_launch() {
    let cutover = cet((2024, 3, 25), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Eex, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Eex, cutover);

    assert!(before.regular.is_empty());
    assert!(!before.is_open(cet((2024, 3, 22), (12, 0, 0))));
    assert!(!after.is_open(cet((2024, 3, 25), (7, 59, 59))));
    assert!(after.is_open_regular(cet((2024, 3, 25), (8, 0, 0))));
    assert!(!after.is_open(cet((2024, 3, 25), (18, 0, 0))));
}

#[test]
fn ice_ftse_100_migration_and_open_extensions() {
    let launch = lon((2014, 11, 17), (0, 0, 0));
    let closed = hours_for_exchange_as_of(
        Exchange::IceEuropeFinancials,
        launch - chrono::Duration::seconds(1),
    );
    let migrated = hours_for_exchange_as_of(Exchange::IceEuropeFinancials, launch);
    assert!(closed.regular.is_empty());
    assert!(closed.extended.is_empty());
    assert!(!migrated.is_open(lon((2014, 11, 17), (6, 2, 59))));
    assert!(migrated.is_open_extended(lon((2014, 11, 17), (6, 3, 0))));
    assert!(migrated.is_open_regular(lon((2014, 11, 17), (8, 0, 0))));

    let seven = hours_for_exchange_as_of(
        Exchange::IceEuropeFinancials,
        lon((2015, 2, 16), (12, 0, 0)),
    );
    assert!(seven.is_open_extended(lon((2015, 2, 16), (6, 3, 0))));
    assert!(seven.is_open_regular(lon((2015, 2, 16), (7, 0, 0))));

    let one = hours_for_exchange_as_of(
        Exchange::IceEuropeFinancials,
        lon((2015, 10, 1), (12, 0, 0)),
    );
    assert!(!one.is_open(lon((2015, 10, 1), (0, 44, 59))));
    assert!(one.is_open_extended(lon((2015, 10, 1), (0, 45, 0))));
    assert!(one.is_open_regular(lon((2015, 10, 1), (1, 0, 0))));
    assert!(!one.is_open(lon((2015, 10, 1), (21, 0, 0))));
}

#[test]
fn ice_endex_ttf_extension_and_reference_clock() {
    let transfer = zoned(Europe::Amsterdam, (2013, 10, 7), (0, 0, 0));
    let closed =
        hours_for_exchange_as_of(Exchange::IceEndex, transfer - chrono::Duration::seconds(1));
    let transferred = hours_for_exchange_as_of(Exchange::IceEndex, transfer);
    assert!(closed.regular.is_empty());
    assert!(closed.extended.is_empty());
    assert!(transferred.is_open_extended(zoned(Europe::Amsterdam, (2013, 10, 7), (7, 45, 0))));
    assert!(transferred.is_open_regular(zoned(Europe::Amsterdam, (2013, 10, 7), (8, 0, 0))));

    let before = hours_for_exchange_as_of(
        Exchange::IceEndex,
        zoned(Europe::Amsterdam, (2026, 4, 10), (12, 0, 0)),
    );
    assert!(!before.is_open(zoned(Europe::Amsterdam, (2026, 4, 10), (7, 44, 59))));
    assert!(before.is_open_extended(zoned(Europe::Amsterdam, (2026, 4, 10), (7, 45, 0))));
    assert!(before.is_open_regular(zoned(Europe::Amsterdam, (2026, 4, 10), (8, 0, 0))));
    assert!(!before.is_open(zoned(Europe::Amsterdam, (2026, 4, 10), (18, 0, 0))));

    let opening_eve = hours_for_exchange_as_of(
        Exchange::IceEndex,
        zoned(Europe::Amsterdam, (2026, 4, 12), (23, 45, 0)),
    );
    assert!(!opening_eve.is_open(zoned(Europe::Amsterdam, (2026, 4, 12), (23, 39, 59))));
    assert!(opening_eve.is_open_extended(zoned(Europe::Amsterdam, (2026, 4, 12), (23, 40, 0))));
    assert!(opening_eve.is_open_regular(zoned(Europe::Amsterdam, (2026, 4, 12), (23, 50, 0))));

    let aligned = hours_for_exchange_as_of(
        Exchange::IceEndex,
        zoned(Europe::Amsterdam, (2026, 4, 14), (12, 0, 0)),
    );
    assert!(!aligned.is_open(zoned(Europe::Amsterdam, (2026, 4, 14), (1, 39, 59))));
    assert!(aligned.is_open_extended(zoned(Europe::Amsterdam, (2026, 4, 14), (1, 40, 0))));
    assert!(aligned.is_open_regular(zoned(Europe::Amsterdam, (2026, 4, 14), (1, 50, 0))));
    assert!(!aligned.is_open(zoned(Europe::Amsterdam, (2026, 4, 14), (23, 0, 0))));

    let mismatch = hours_for_exchange_as_of(
        Exchange::IceEndex,
        zoned(Europe::Amsterdam, (2026, 10, 27), (12, 0, 0)),
    );
    assert!(mismatch.is_open_extended(zoned(Europe::Amsterdam, (2026, 10, 27), (0, 40, 0))));
    assert!(mismatch.is_open_regular(zoned(Europe::Amsterdam, (2026, 10, 27), (0, 50, 0))));
    assert!(!mismatch.is_open(zoned(Europe::Amsterdam, (2026, 10, 27), (22, 0, 0))));

    let spring_mismatch = hours_for_exchange_as_of(
        Exchange::IceEndex,
        zoned(Europe::Amsterdam, (2027, 3, 16), (12, 0, 0)),
    );
    assert!(spring_mismatch.is_open_extended(zoned(Europe::Amsterdam, (2027, 3, 16), (0, 40, 0))));
    assert!(spring_mismatch.is_open_regular(zoned(Europe::Amsterdam, (2027, 3, 16), (0, 50, 0))));
    assert!(!spring_mismatch.is_open(zoned(Europe::Amsterdam, (2027, 3, 16), (22, 0, 0))));
}

#[test]
fn ice_abu_dhabi_murban_launch_and_us_reference_clock() {
    let launch = zoned(Asia::Dubai, (2021, 3, 29), (0, 0, 0));
    let before =
        hours_for_exchange_as_of(Exchange::IceAbuDhabi, launch - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::IceAbuDhabi, launch);
    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(!after.is_open(zoned(Asia::Dubai, (2021, 3, 29), (0, 59, 59))));
    assert!(after.is_open_extended(zoned(Asia::Dubai, (2021, 3, 29), (1, 0, 0))));
    assert!(after.is_open_regular(zoned(Asia::Dubai, (2021, 3, 29), (2, 0, 0))));
    assert!(!after.is_open(zoned(Asia::Dubai, (2021, 3, 30), (2, 0, 0))));

    let winter = hours_for_exchange_as_of(
        Exchange::IceAbuDhabi,
        zoned(Asia::Dubai, (2026, 1, 19), (12, 0, 0)),
    );
    assert!(winter.is_open_extended(zoned(Asia::Dubai, (2026, 1, 19), (2, 0, 0))));
    assert!(winter.is_open_regular(zoned(Asia::Dubai, (2026, 1, 19), (3, 0, 0))));
    assert!(!winter.is_open(zoned(Asia::Dubai, (2026, 1, 20), (3, 0, 0))));
}
