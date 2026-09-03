// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S.: sourced open revisions.

use super::prelude::*;

/// ICE Sugar No. 11 moved its open across 2012 and its whole grid on
/// 2014-02-03. 08:00Z is 03:00 NY in winter, before the current 03:30 open but
/// after the 02:30 open in force between 2012-11-05 and 2014-01-31.
#[test]
fn ice_sugar_open_tracks_its_sourced_revisions() {
    let key = MarketHoursKey::IceUsSugar;

    assert!(
        open_regular_at(key, utc(2013, 1, 16, 8, 0)),
        "2012-11-05 through 2014-01-31 opens 02:30 NY, so 03:00 NY is executable"
    );
    assert!(
        !open_regular_at(key, utc(2026, 1, 14, 8, 0)),
        "from 2014-02-03 the open is 03:30 NY, so 03:00 NY is not yet executable"
    );
}
