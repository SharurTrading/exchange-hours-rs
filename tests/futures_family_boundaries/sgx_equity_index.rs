// SPDX-License-Identifier: MIT-0

//! SGX equity-index families: the grids stay distinct, the sourced window
//! precedes the current grid, and both dated boundaries hold on each side.

use super::prelude::*;

/// The five SGX equity-index grids must disagree. If any two collapsed onto one
/// profile, the crate would be substituting one market's hours for another's -
/// the exact failure the split exists to prevent.
#[test]
fn sgx_equity_index_grids_do_not_collapse_onto_each_other() {
    let keys = [
        MarketHoursKey::SgxEquityIndexJapan,
        MarketHoursKey::SgxEquityIndexChina,
        MarketHoursKey::SgxEquityIndexSingapore,
        MarketHoursKey::SgxEquityIndexTaiwan,
        MarketHoursKey::SgxEquityIndexNtrUsd,
    ];

    // 08:00 Singapore on a Wednesday = 00:00Z. Japan (07:30) and NTR (07:25)
    // have opened; China (09:00), Singapore (08:30) and Taiwan (08:45) have not.
    let probe = utc(2026, 6, 17, 0, 0);
    let open_count = keys.iter().filter(|&&key| open_at(key, probe)).count();

    assert_eq!(
        open_count, 2,
        "at 08:00 Singapore exactly the Japan and NTR (USD) grids are open; \
         a different count means two grids have collapsed together"
    );
}

/// SGX equity-index history: sessionless before the first sourced calendar
/// edition, then one sourced window, then the current grid from the dated
/// 2025-04-07 cutover.
///
/// Nine editions of SGX's Derivatives Trading Calendar disagree in two places.
/// The later disagreement is dated — SGX-DT Circular DT/AM 15 of 2025 puts it
/// at Monday 7 April 2025 — but Japan's earlier T-session extension is not, so
/// the first era serves the intersection of every state sourced inside it
/// rather than keying a revision to an edition's year. Japan's T closes at
/// 14:25 and its T+1 opens at 15:25 in that window, the narrowest bounds any
/// edition gives.
///
/// Singapore has no DST, so 06:30Z is 14:30 SGT, 07:15Z is 15:15 SGT, 07:40Z is
/// 15:40 SGT and 08:50Z is 16:50 SGT. 2026-09-16 falls after the cutover; the
/// 2022 and 2025 probes below fall before it.
#[test]
fn sgx_equity_index_serves_the_sourced_window_then_the_verified_grid() {
    let japan = MarketHoursKey::SgxEquityIndexJapan;

    assert!(
        !open_at(japan, utc(2015, 6, 17, 6, 30)),
        "pre-2020 SGX dates are sessionless: that era is unmodelled, not unsourceable"
    );

    // Inside the sourced window the narrowest bounds apply, so both the T close
    // beyond 14:25 and the T+1 open before 15:25 stay closed. 2022-06-15 and
    // 2025-03-19 are both Wednesdays inside that era.
    for (year, month, day) in [(2022, 6, 15), (2025, 3, 19)] {
        for (h, m, what) in [
            (6u32, 30u32, "14:30 SGT, past the 14:25 T close"),
            (7, 15, "15:15 SGT, before the 15:25 T+1 open"),
        ] {
            let t = utc(year, month, day, h, m);
            assert!(
                !open_at(japan, t),
                "{year}: {what} is outside the sourced window every edition agrees on"
            );
        }
        let inside = utc(year, month, day, 7, 40);
        assert!(
            open_at(japan, inside),
            "{year}: 15:40 SGT is inside the T+1 session in every sourced edition"
        );
    }

    // After the cutover the circular's grid applies: T runs to 14:55 and T+1
    // opens at 15:10.
    for (h, m) in [(6u32, 30u32), (7, 15)] {
        let t = utc(2026, 9, 16, h, m);
        assert!(
            open_at(japan, t),
            "the verified-current grid opens Japan at both 14:30 and 15:15 SGT"
        );
    }

    // China's T+1 opens at 17:00 in the sourced window and 16:45 currently.
    let china = MarketHoursKey::SgxEquityIndexChina;
    assert!(
        !open_at(china, utc(2022, 6, 15, 8, 50)),
        "16:50 SGT precedes China's 17:00 T+1 open in the sourced window"
    );
    assert!(
        open_at(china, utc(2026, 9, 16, 8, 50)),
        "the verified-current grid opens China's T+1 at 16:45 SGT"
    );

    // The remaining three keys each pull their T+1 open fifteen minutes earlier
    // at the same 2025 boundary, so each gets both sides of it. 09:40Z is 17:40
    // SGT, 06:05Z is 14:05 SGT and 10:50Z is 18:50 SGT — each five minutes
    // inside the current T+1 and outside the sourced window's.
    for (key, hour, minute, window_open, current_open) in [
        (
            MarketHoursKey::SgxEquityIndexSingapore,
            9u32,
            40u32,
            "17:50",
            "17:35",
        ),
        (MarketHoursKey::SgxEquityIndexTaiwan, 6, 5, "14:15", "14:00"),
        (
            MarketHoursKey::SgxEquityIndexNtrUsd,
            10,
            50,
            "19:00",
            "18:45",
        ),
    ] {
        assert!(
            !open_at(key, utc(2022, 6, 15, hour, minute)),
            "{key:?}: the sourced window opens T+1 at {window_open} SGT, so this probe is closed"
        );
        assert!(
            open_at(key, utc(2026, 9, 16, hour, minute)),
            "{key:?}: the verified-current grid opens T+1 at {current_open} SGT"
        );
        assert!(
            !open_at(key, utc(2015, 6, 17, 6, 30)),
            "{key:?} must be sessionless before the first sourced edition"
        );
    }
}

/// The FTSE Taiwan suite starts a year later than the other four SGX grids.
///
/// SGX's 2020 Derivatives Trading Calendar contains no FTSE Taiwan contract at
/// all — it lists only the MSCI Taiwan predecessors (`TW`, `TWO`, `NTW`), and
/// "TWN" appears in that edition solely as the holiday country code for Taiwan
/// (TWSE). The 2021 edition is the first to list "SGX FTSE Taiwan Index
/// Futures" under the code `TWN`, so this family's sourced history starts
/// there while the other four start at the 2020 edition.
///
/// The predecessor's hours were identical, so this boundary changes no served
/// time — it stops the crate asserting that a contract the cited edition does
/// not contain was open. 10:00 SGT is 02:00Z and sits inside the Taiwan T
/// session (08:45–13:45) in every edition that lists it.
#[test]
fn sgx_taiwan_history_starts_at_the_first_edition_that_lists_the_ftse_suite() {
    let taiwan = MarketHoursKey::SgxEquityIndexTaiwan;
    let inside_t_session_2020 = utc(2020, 6, 17, 2, 0);
    let inside_t_session_2021 = utc(2021, 1, 6, 2, 0);

    assert!(
        !open_at(taiwan, inside_t_session_2020),
        "the 2020 edition lists no FTSE Taiwan contract, so 2020 must be sessionless"
    );
    assert!(
        open_at(taiwan, inside_t_session_2021),
        "the 2021 edition lists SGX FTSE Taiwan Index Futures at 08:45-13:45 SGT"
    );

    // The other four families are sourced from the 2020 edition, which does
    // list them, so only Taiwan moves. Each probe is inside that family's own
    // T session on the same 2020 Wednesday.
    for (key, hour, minute) in [
        (MarketHoursKey::SgxEquityIndexJapan, 2u32, 0u32),
        (MarketHoursKey::SgxEquityIndexChina, 2, 0),
        (MarketHoursKey::SgxEquityIndexSingapore, 2, 0),
        (MarketHoursKey::SgxEquityIndexNtrUsd, 2, 0),
    ] {
        assert!(
            open_at(key, utc(2020, 6, 17, hour, minute)),
            "{key:?} is listed in the 2020 edition and must keep its 2020 history"
        );
    }
}

/// Both sides of the SGX equity-index cutover at Singapore local midnight.
///
/// SGX-DT Circular DT/AM 15 of 2025 (24 February 2025) states that "with effect
/// from Monday, [7] April 2025" the T+1 pre-open routine moves ten minutes
/// earlier and shortens from ten minutes to five, which opens every equity-index
/// T+1 session fifteen minutes earlier. Singapore is UTC+8 and keeps no DST, so
/// 15:59Z on 2025-04-06 is 23:59 on the last local day the earlier profile
/// governs and 16:00Z is 00:00 on the first local day the revised one does. Each
/// probe is a Monday-afternoon instant inside the revised T+1 session and
/// outside the earlier one, evaluated against the profile each boundary instant
/// selects — so a revision keyed a day early or late fails here.
#[test]
fn sgx_equity_index_t_plus_one_opens_fifteen_minutes_earlier_from_2025_04_07() {
    let last_earlier_day = utc(2025, 4, 6, 15, 59);
    let first_revised_day = utc(2025, 4, 6, 16, 0);

    // Each pair is that family's T+1 open on the earlier and the revised
    // profile, in UTC: Singapore is UTC+8 and keeps no DST, so 07:10Z is 15:10
    // SGT and so on. Both are pinned to the second, because a profile whose
    // open drifted a minute either way would still pass a probe taken five
    // minutes inside the session.
    for (key, earlier_open, revised_open, moved) in [
        (
            MarketHoursKey::SgxEquityIndexJapan,
            (7, 25),
            (7, 10),
            "15:25 -> 15:10",
        ),
        (
            MarketHoursKey::SgxEquityIndexChina,
            (9, 0),
            (8, 45),
            "17:00 -> 16:45",
        ),
        (
            MarketHoursKey::SgxEquityIndexSingapore,
            (9, 50),
            (9, 35),
            "17:50 -> 17:35",
        ),
        (
            MarketHoursKey::SgxEquityIndexTaiwan,
            (6, 15),
            (6, 0),
            "14:15 -> 14:00",
        ),
        (
            MarketHoursKey::SgxEquityIndexNtrUsd,
            (11, 0),
            (10, 45),
            "19:00 -> 18:45",
        ),
    ] {
        let earlier = hours_for_market_hours_key(key, last_earlier_day);
        let revised = hours_for_market_hours_key(key, first_revised_day);
        let earlier_open = utc(2025, 4, 7, earlier_open.0, earlier_open.1);
        let revised_open = utc(2025, 4, 7, revised_open.0, revised_open.1);

        // The revised profile opens exactly at the circular's time.
        assert!(
            !revised.is_open(just_before(revised_open)),
            "{key:?}: the revised T+1 opens a second too early ({moved})"
        );
        assert!(
            revised.is_open(revised_open),
            "{key:?}: the revised T+1 does not open at its stated time ({moved})"
        );

        // The earlier profile is still shut then, and opens at its own later
        // time — which is what makes this a fifteen-minute move rather than a
        // profile that merely differs somewhere.
        assert!(
            !earlier.is_open(revised_open),
            "{key:?}: the profile in force through 2025-04-06 already opened T+1 \
             at the revised time ({moved})"
        );
        assert!(
            !earlier.is_open(just_before(earlier_open)),
            "{key:?}: the earlier T+1 opens a second too early ({moved})"
        );
        assert!(
            earlier.is_open(earlier_open),
            "{key:?}: the earlier T+1 does not open at its stated time ({moved})"
        );
    }

    // The circular states "no change to the T session trading hours", and its
    // appendices confirm the Japan T close at 14:55 on both sides of the move.
    // The dated surface still lengthens Japan's T session here, because the
    // conservative intersection that hides the undated 2024 extension ends at
    // this same boundary — 06:30Z is 14:30 SGT.
    let japan = MarketHoursKey::SgxEquityIndexJapan;
    let japan_t_close = utc(2025, 4, 7, 6, 30);
    assert!(
        !hours_for_market_hours_key(japan, last_earlier_day).is_open(japan_t_close),
        "the intersection era closes Japan's T session at 14:25, its narrowest sourced bound"
    );
    assert!(
        hours_for_market_hours_key(japan, first_revised_day).is_open(japan_t_close),
        "the circular's appendices put Japan's T close at 14:55 from 2025-04-07"
    );
}
