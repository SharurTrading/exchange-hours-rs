// SPDX-License-Identifier: MIT-0

//! Point-in-time Southeast Asian cash-equity schedule revisions.

use super::prelude::*;

/// Asserts an identity-backed query returns exactly `expected`, the coverage
/// error the shipped data declares. The variant, identity and date are all part
/// of the contract: a refusal that named the wrong day would be as wrong as an
/// answer.
fn assert_refusal<T>(
    answer: Result<T, CalendarQueryError>,
    expected: CalendarQueryError,
    label: &str,
) {
    assert_eq!(
        answer.err(),
        Some(expected),
        "{label}: the query must state the refusal its identity declares"
    );
}

/// Asserts a query refuses `date` because the identity has no sourced answer for
/// it at or above the floor.
fn assert_outside_coverage<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    source: CalendarSource,
    date: NaiveDate,
    label: &str,
) {
    assert_refusal(
        answer,
        CalendarQueryError::OutsideCoveredRange { source, date },
        label,
    );
}

#[test]
fn bursa_january_2010_baseline_matches_current_grid() {
    let tz = Asia::Kuala_Lumpur;
    let as_of = local(tz, (2010, 1, 4), (12, 0, 0));
    let probe = (2026, 8, 19);
    let hours = hours_for_exchange(Exchange::BursaMalaysia, as_of);

    assert!(hours.is_open_regular(local(tz, probe, (12, 29, 59))));
    assert!(!hours.is_open(local(tz, probe, (12, 30, 0))));
}

#[test]
fn thailand_cutover() {
    let tz = Asia::Bangkok;
    let (pre, post) = cutover_sides(Exchange::SetThailand, tz, (2024, 3, 25));
    let at_1415 = local(tz, (2026, 8, 19), (14, 15, 0));
    assert!(!pre.is_open_regular(at_1415));
    // Before 2024 the afternoon session opened later, so 14:15 was still the
    // pre-open order-accumulation window rather than continuous trading.
    assert!(pre.is_order_entry_only(at_1415));
    assert!(post.is_open_regular(at_1415));
}

#[test]
fn thailand_dr_night_launch_and_trade_date() {
    let tz = Asia::Bangkok;
    let calendar = calendar_for_exchange(Exchange::SetThailand);
    let monday_lunch = local(tz, (2025, 5, 5), (12, 30, 0));
    let launch_lunch = local(tz, (2025, 5, 6), (12, 30, 0));
    let prelaunch_tail = local(tz, (2025, 5, 6), (2, 50, 0));
    let night_preopen = local(tz, (2025, 5, 6), (18, 45, 0));
    let night_regular = local(tz, (2025, 5, 6), (19, 0, 0));
    let night_close_call = local(tz, (2025, 5, 7), (2, 45, 0));
    let final_close = local(tz, (2025, 5, 7), (3, 0, 0));

    // The launched grid is a dated profile selection, so the **fixed snapshot**
    // states it: which phase each instant below falls in does not depend on a
    // holiday layer. Every grid assertion therefore reads `hours_for_exchange`,
    // which is the surface that still answers for this identity.
    let hours = |instant| hours_for_exchange(Exchange::SetThailand, instant);
    assert!(!hours(monday_lunch).is_open(monday_lunch));
    assert!(!hours(prelaunch_tail).is_open(prelaunch_tail));
    assert!(hours(launch_lunch).is_open_regular(launch_lunch));
    assert!(hours(night_preopen).is_order_entry_only(night_preopen));
    assert!(hours(night_regular).is_open_regular(night_regular));
    // Pre-close and off-hour: off-hour trades print here.
    assert!(hours(night_close_call).is_open_extended(night_close_call));
    assert!(!hours(final_close).is_open(final_close));
    // The pause between the day close and the night open: the identity-backed
    // calendar used to report `Halt` because the pause stays inside one trade
    // date, and that is identity-dependent topology. A detached snapshot carries
    // no identity, so it classifies the same gap by the crate's four-hour policy
    // and reports `Maintenance`; the identity's `Halt` is not observable while it
    // refuses the date, and this is the value the surface under test actually
    // states.
    let pause = local(tz, (2025, 5, 6), (17, 30, 0));
    assert_eq!(
        hours(pause).session_state(pause),
        SessionState::Maintenance,
        "a detached snapshot classifies the 2.5-hour gap by the four-hour policy"
    );
    // The final close by the same rule: the detached snapshot reports the
    // post-close gap through the crate's policy (`Halt`, because the next
    // session at 09:55 belongs to the snapshot's own trade-date default), while
    // the identity's `Closed` is not observable. The phase assertion above shows
    // the close itself; the state that follows it is stated as this surface
    // computes it.
    assert_eq!(
        hours(final_close).session_state(final_close),
        SessionState::Halt
    );

    // The trade-date consequences need the identity: which trade date a wrapped
    // session carries is identity-dependent topology, and a detached snapshot
    // must not guess it. `Exchange::SetThailand` ships no holiday layer, so its
    // date-aware surface has no sourced answer for any of these 2025 dates and
    // refuses each probe rather than reporting a phase or a `None`
    // (LAW-COVERAGE). "The night session carries the following trade date" is
    // therefore not claimable through this identity; a scope with a complete
    // holiday layer states it (`tests/order_entry_phase.rs`).
    let thailand = CalendarSource::Exchange(Exchange::SetThailand);
    for (instant, date) in [
        (monday_lunch, (2025, 5, 5)),
        (prelaunch_tail, (2025, 5, 6)),
        (launch_lunch, (2025, 5, 6)),
        (night_preopen, (2025, 5, 6)),
        (night_regular, (2025, 5, 6)),
        (night_close_call, (2025, 5, 7)),
        (final_close, (2025, 5, 7)),
    ] {
        assert_outside_coverage(
            calendar.is_open(instant),
            thailand,
            NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture date"),
            "the launched grid through the date-aware calendar",
        );
    }
    for (instant, date) in [
        (launch_lunch, (2025, 5, 6)),
        (night_preopen, (2025, 5, 6)),
        (night_regular, (2025, 5, 6)),
        (night_close_call, (2025, 5, 7)),
    ] {
        assert_outside_coverage(
            calendar.trade_date(instant),
            thailand,
            NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture date"),
            "the wrapped session's trade date",
        );
    }
    // The bar adapter resolves the opening day of the trade date the night
    // session belongs to, so its refusal names 2025-05-05 rather than the
    // evening of the 06th the probe was addressed to.
    assert_outside_coverage(
        calendar.candle_end(night_regular, CalendarResolution::Daily),
        thailand,
        NaiveDate::from_ymd_opt(2025, 5, 5).expect("fixture date"),
        "the daily bar's end",
    );
    assert_outside_coverage(
        calendar.session_state(local(tz, (2025, 5, 6), (17, 30, 0))),
        thailand,
        NaiveDate::from_ymd_opt(2025, 5, 6).expect("fixture date"),
        "the inter-phase pause",
    );
    assert_outside_coverage(
        calendar.session_state(final_close),
        thailand,
        NaiveDate::from_ymd_opt(2025, 5, 7).expect("fixture date"),
        "the final close",
    );
    // The closure question for the trade date itself refuses for the date it was
    // asked about, because the identity has no answered holiday layer for it.
    assert_outside_coverage(
        calendar.is_closed_trade_date(
            NaiveDate::from_ymd_opt(2025, 5, 6).expect("fixture date"),
            SessionKind::Both,
        ),
        thailand,
        NaiveDate::from_ymd_opt(2025, 5, 6).expect("fixture date"),
        "the trade-date closure question",
    );
}

#[test]
fn thailand_monthly_candles_group_the_after_midnight_close_by_trade_date() {
    let tz = Asia::Bangkok;
    let calendar = calendar_for_exchange(Exchange::SetThailand);
    let march_31 = local(tz, (2026, 3, 31), (12, 0, 0));
    let march_close = local(tz, (2026, 4, 1), (3, 0, 0));

    // What the identity-erased fixed snapshot states: the monthly bar opens at
    // the randomised opening auction on the month's first trading day.
    let hours = hours_for_exchange(Exchange::SetThailand, march_31);
    assert_eq!(
        candle_start(&hours, march_31, CalendarResolution::Monthly),
        // The monthly bar opens at the randomised opening auction, the first
        // instant a trade can print, rather than at the 09:30 pre-open.
        Some(local(tz, (2026, 3, 2), (9, 55, 0)))
    );
    // The grouping this fence is named for — the after-midnight 03:00 close
    // belonging to the *opening* day's bar, so the daily and monthly bars end at
    // `march_close` — is identity-dependent topology: only an identified
    // calendar knows that the night session rolls into the next trade date. A
    // detached snapshot carries no identity and, by documented design, falls
    // back to the close-date default, so it ends the bar at the 17:00 day close
    // instead. The claim is therefore not observable on either surface while
    // `Exchange::SetThailand` refuses the date; both values are stated so the
    // divergence is fenced rather than hidden.
    assert_eq!(
        candle_end(&hours, march_31, CalendarResolution::Daily),
        Some(local(tz, (2026, 3, 31), (17, 0, 0))),
        "the identity-erased default groups the bar by its own close date"
    );
    assert_eq!(
        candle_end(&hours, march_31, CalendarResolution::Monthly),
        Some(local(tz, (2026, 3, 31), (17, 0, 0)))
    );
    assert_ne!(
        candle_end(&hours, march_31, CalendarResolution::Monthly),
        Some(march_close),
        "the identity's grouping is not what a detached snapshot reports"
    );

    // The date-aware calendar does not answer these dates at all: the identity
    // ships no holiday layer, so every 2026 date is unsourced for it and each
    // probe refuses rather than reporting a bar boundary no evidence supports
    // (LAW-COVERAGE). The grouping claim survives on the surface above, which is
    // where the rules live.
    let thailand = CalendarSource::Exchange(Exchange::SetThailand);
    for resolution in [CalendarResolution::Daily, CalendarResolution::Monthly] {
        // Both bar adapters resolve the **opening day** of the trade date the
        // instant belongs to, so the refusal names 2026-03-30 — the day the
        // session opened on — rather than the 03-31 the probe was addressed to.
        assert_outside_coverage(
            calendar.candle_end(march_31, resolution),
            thailand,
            NaiveDate::from_ymd_opt(2026, 3, 30).expect("fixture date"),
            "the date-aware bar end",
        );
        assert_outside_coverage(
            calendar.candle_start(march_31, resolution),
            thailand,
            NaiveDate::from_ymd_opt(2026, 3, 30).expect("fixture date"),
            "the date-aware bar start",
        );
    }
}

#[test]
fn indonesia_cutovers() {
    let tz = Asia::Jakarta;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Idx, tz, (2013, 1, 2));
    let at_0910 = local(tz, probe, (9, 10, 0));
    assert!(!pre.is_open(local(tz, probe, (9, 9, 59))));
    assert!(pre.is_open_extended(at_0910));
    assert!(pre.is_open_extended(local(tz, probe, (9, 29, 59))));
    assert!(post.is_open_regular(at_0910));

    let (pre, post) = cutover_sides(Exchange::Idx, tz, (2020, 3, 30));
    let at_1530 = local(tz, probe, (15, 30, 0));
    assert!(pre.is_open_regular(at_1530));
    assert!(!post.is_open(at_1530));

    let (pre, post) = cutover_sides(Exchange::Idx, tz, (2023, 4, 3));
    assert!(!pre.is_open(at_1530));
    assert!(post.is_open_regular(at_1530));
    let at_1620 = local(tz, probe, (16, 20, 0));
    assert!(!pre.is_open(at_1620));
    assert!(post.is_open_extended(at_1620));
}

#[test]
fn philippines_cutovers() {
    let tz = Asia::Manila;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2011, 10, 1));
    let at_1230 = local(tz, probe, (12, 30, 0));
    assert!(!pre.is_open(at_1230));
    assert!(post.is_open_regular(at_1230));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2012, 1, 2));
    let at_1400 = local(tz, probe, (14, 0, 0));
    assert!(!pre.is_open(at_1400));
    assert!(post.is_open_regular(at_1400));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2013, 11, 4));
    let at_1516 = local(tz, probe, (15, 16, 0));
    assert!(pre.is_open_regular(at_1516));
    assert!(!post.is_open_regular(at_1516));
    assert!(post.is_open_extended(at_1516));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2020, 3, 16));
    assert!(pre.is_open_regular(at_1400));
    assert!(!post.is_open(at_1400));

    let at_1000_probe = local(tz, probe, (10, 0, 0));
    let (pre, closed) = cutover_sides(Exchange::Pse, tz, (2020, 3, 17));
    assert!(pre.is_open_regular(at_1000_probe));
    assert!(!closed.is_open(at_1000_probe));
    let (closed, resumed) = cutover_sides(Exchange::Pse, tz, (2020, 3, 19));
    assert!(!closed.is_open(at_1000_probe));
    assert!(resumed.is_open_regular(at_1000_probe));

    let at_1000 = |date| local(tz, date, (10, 0, 0));
    for date in [(2020, 3, 17), (2020, 3, 18)] {
        let hours = hours_for_exchange(Exchange::Pse, at_1000(date));
        assert!(!hours.is_open(at_1000(date)));
    }
    let resumed = hours_for_exchange(Exchange::Pse, at_1000((2020, 3, 19)));
    assert!(resumed.is_open_regular(at_1000((2020, 3, 19))));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2021, 12, 6));
    let at_1330 = local(tz, probe, (13, 30, 0));
    assert!(!pre.is_open(at_1330));
    assert!(post.is_open_regular(at_1330));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2022, 1, 14));
    assert!(pre.is_open_regular(at_1330));
    assert!(!post.is_open(at_1330));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2022, 2, 2));
    assert!(!pre.is_open(at_1330));
    assert!(post.is_open_regular(at_1330));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2024, 3, 1));
    let at_1505 = local(tz, probe, (15, 5, 0));
    assert!(!pre.is_open(at_1505));
    assert!(post.is_open_extended(at_1505));
}

#[test]
fn vietnam_cutover_and_oldest_profile() {
    let tz = Asia::Ho_Chi_Minh;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2010, 9, 13));
    let at_0845 = local(tz, probe, (8, 45, 0));
    assert!(pre.is_open_extended(at_0845));
    assert!(post.is_open_regular(at_0845));
    let at_1030 = local(tz, probe, (10, 30, 0));
    assert!(pre.is_open_extended(at_1030));
    assert!(post.is_open_extended(at_1030));
    assert!(post.is_open_extended(local(tz, probe, (10, 50, 0))));

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2012, 3, 5));
    let at_1045 = local(tz, probe, (10, 45, 0));
    assert!(!pre.is_open_regular(at_1045));
    assert!(pre.is_open_extended(at_1045));
    assert!(post.is_open_regular(at_1045));
    assert!(post.is_open_extended(local(tz, probe, (14, 10, 0))));

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2013, 7, 22));
    let at_1415 = local(tz, probe, (14, 15, 0));
    assert!(!pre.is_open(at_1415));
    assert!(post.is_open_regular(at_1415));
    assert!(post.is_open_extended(local(tz, probe, (14, 50, 0))));

    // The archived operator PDF supplies the exact January-2010 baseline.
    let oldest = hours_for_exchange(Exchange::Hose, local(tz, (2010, 1, 4), (10, 0, 0)));
    assert!(oldest.is_open_regular(local(tz, probe, (10, 0, 0))));
    assert!(oldest.is_open_extended(local(tz, probe, (10, 15, 0))));
    assert!(oldest.is_open_extended(local(tz, probe, (10, 30, 0))));
    assert!(!oldest.is_open(local(tz, probe, (11, 0, 0))));
    assert!(!oldest.is_open(local(tz, probe, (14, 15, 0))));
}
