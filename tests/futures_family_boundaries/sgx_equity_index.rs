// SPDX-License-Identifier: MIT-0

//! SGX equity-index families: the grids stay distinct, the pre-2020 floor
//! grids are carried, the dated 2024 Japan and 2025 boundaries hold on each
//! side, and Taiwan starts on its launch day. The pre-2020 boundaries
//! themselves are fenced in the sibling `sgx_equity_index_eras` module.

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

/// SGX equity-index history: the 2013 portal grid carried to the floor, the
/// 04:45 grid from Monday 2017-07-10, the 05:15 close from Monday 2020-01-06,
/// then — for Japan — the dated 2024-11-04 T-session extension, and the
/// current grid from the dated 2025-04-07 cutover on every key.
///
/// Both dated moves come from SGX-DT circulars: DT/AM 50 of 2024 lengthens
/// Japan's T session to 14:55 and moves its T+1 open to 15:25 from Monday
/// 4 November 2024; DT/AM 15 of 2025 pulls every family's T+1 open fifteen
/// minutes earlier from Monday 7 April 2025. Between the 2020 row and DT/AM 50
/// Japan's T closes at 14:25 and its T+1 opens at 14:55.
///
/// Singapore has no DST, so 06:30Z is 14:30 SGT, 07:15Z is 15:15 SGT, 07:40Z is
/// 15:40 SGT and 08:50Z is 16:50 SGT. 2026-09-16 falls after the cutover; the
/// 2022 and 2025 probes below fall before it.
#[test]
fn sgx_equity_index_serves_the_sourced_window_then_the_verified_grid() {
    let japan = MarketHoursKey::SgxEquityIndexJapan;

    // Pre-2020 dates serve the carried floor grid, not a sessionless profile:
    // 10:00 SGT is inside the floor T session and 07:40 SGT precedes its 07:45
    // open. The era suite fences every pre-2020 boundary in detail.
    assert!(
        open_at(japan, utc(2015, 6, 17, 2, 0)),
        "the 2013 portal grid is carried to the January-2010 floor"
    );
    assert!(
        !open_at(japan, utc(2015, 6, 16, 23, 40)),
        "the floor grid opens the Nikkei T session at 07:45 SGT"
    );

    // Two dated eras sit between the 2020 row and the 2025 cutover. Through
    // 2024-11-03 the T session closes 14:25 and the T+1 opens 14:55; from
    // 2024-11-04 (DT/AM 50) the T session runs to 14:55 and the T+1 opens
    // 15:25 behind a 15:15-15:25 queue. 2022-06-15 and 2025-03-19 are
    // Wednesdays on either side.
    let t_2022 = |h, m| utc(2022, 6, 15, h, m);
    assert!(
        !open_at(japan, t_2022(6, 30)),
        "2022: 14:30 SGT, the closing routine has ended"
    );
    assert!(
        open_at(japan, t_2022(7, 15)),
        "2022: 15:15 SGT is inside the 14:55 T+1 session"
    );
    let t_2025 = |h, m| utc(2025, 3, 19, h, m);
    assert!(
        open_at(japan, t_2025(6, 30)),
        "2025: 14:30 SGT is inside the lengthened T session"
    );
    assert!(
        !open_at(japan, t_2025(7, 15)),
        "2025: 15:15 SGT is the T+1 queue, not the session"
    );
    for t in [t_2022(7, 40), t_2025(7, 40)] {
        assert!(
            open_at(japan, t),
            "15:40 SGT is inside the T+1 session in both eras"
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
        // Singapore's floor grid is carried to the January-2010 floor (14:30
        // SGT is inside its 08:30-17:10 T session); the FTSE Taiwan and NTR
        // suites did not exist in 2015 and stay sessionless.
        let carried = key == MarketHoursKey::SgxEquityIndexSingapore;
        assert_eq!(
            open_at(key, utc(2015, 6, 17, 6, 30)),
            carried,
            "{key:?}: pre-2020 history is carried only where the family was listed"
        );
    }
}

/// The FTSE Taiwan suite starts on its own launch day, not at a calendar
/// edition.
///
/// SGX's 2020 Derivatives Trading Calendar contains no FTSE Taiwan contract at
/// all — it lists only the MSCI Taiwan predecessors (`TW`, `TWO`, `NTW`) — and
/// the 2021 edition is the first calendar to list "SGX FTSE Taiwan Index
/// Futures" under `TWN`. But SGX's own release states the launch, 20 July
/// 2020, and SGX's content API lists the contract with its grid from
/// 2020-07-15, so the family's history starts on 2020-07-20. The predecessor's
/// hours were identical, so this boundary changes no served time on the day —
/// it stops the crate asserting that a contract SGX had not yet listed was
/// open. 10:00 SGT is 02:00Z and sits inside the Taiwan T session (08:45–13:45)
/// in every artifact that lists it.
#[test]
fn sgx_taiwan_history_starts_on_its_stated_launch_day() {
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

    // The other four families were already trading on that 2020 Wednesday —
    // Japan, China and Singapore on their 2017-07-10 grids and NTR on its
    // 2018-04-16 one — so only Taiwan is sessionless there. Each probe is
    // inside that family's own T session.
    for (key, hour, minute) in [
        (MarketHoursKey::SgxEquityIndexJapan, 2u32, 0u32),
        (MarketHoursKey::SgxEquityIndexChina, 2, 0),
        (MarketHoursKey::SgxEquityIndexSingapore, 2, 0),
        (MarketHoursKey::SgxEquityIndexNtrUsd, 2, 0),
    ] {
        assert!(
            open_at(key, utc(2020, 6, 17, hour, minute)),
            "{key:?} was already trading in June 2020 and must serve its era"
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
    // That 14:55 came in on Monday 2024-11-04 under DT/AM 50 of 2024, so the
    // profile in force on 2025-04-06 already serves it — 06:30Z is 14:30 SGT.
    let japan = MarketHoursKey::SgxEquityIndexJapan;
    let japan_t_probe = utc(2025, 4, 7, 6, 30);
    assert!(
        hours_for_market_hours_key(japan, last_earlier_day).is_open(japan_t_probe),
        "DT/AM 50 of 2024 lengthened Japan's T session to 14:55 before this cutover"
    );
    assert!(
        hours_for_market_hours_key(japan, first_revised_day).is_open(japan_t_probe),
        "the circular's appendices keep Japan's T close at 14:55 from 2025-04-07"
    );
    // And the lengthening itself is keyed to the Monday DT/AM 50 states:
    // 23:59 SGT on 2024-11-03 is 15:59Z, midnight 2024-11-04 is 16:00Z.
    let last_short_day = utc(2024, 11, 3, 15, 59);
    let first_long_day = utc(2024, 11, 3, 16, 0);
    let monday_afternoon = utc(2024, 11, 4, 6, 40);
    assert!(!hours_for_market_hours_key(japan, last_short_day).is_open(monday_afternoon));
    assert!(hours_for_market_hours_key(japan, first_long_day).is_open(monday_afternoon));
}
