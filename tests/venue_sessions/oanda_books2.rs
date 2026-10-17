// SPDX-License-Identifier: MIT-0

//! OANDA's own book, second wave: the metals grid, the two US-anchored
//! continuous grids, and the Brent grid.

use super::prelude::*;

// ---------------------------------------------------------------------------
// oanda_metals — Sunday 18:05 through Friday 16:59 New York time.
// ---------------------------------------------------------------------------

#[test]
fn oanda_metals_runs_the_printed_grid_in_both_dst_regimes() {
    // The row prints no *** marker, so the zone carries DST: 18:05 ET is
    // 23:05 UTC in winter and 22:05 UTC in summer.
    let winter =
        hours_for_market_hours_key(MarketHoursKey::OandaMetals, et((2026, 1, 12), (12, 0, 0)));
    assert!(
        !winter.is_open(et((2026, 1, 11), (18, 4, 59))),
        "one second before the winter open"
    );
    assert!(
        winter.is_open(et((2026, 1, 11), (18, 5, 0))),
        "the winter Sunday open"
    );
    let summer =
        hours_for_market_hours_key(MarketHoursKey::OandaMetals, et((2026, 7, 12), (12, 0, 0)));
    assert!(
        summer.is_open(et((2026, 7, 12), (18, 5, 0))),
        "the summer Sunday open"
    );
    assert_eq!(summer.tz, America::New_York);
}

#[test]
fn oanda_metals_daily_gap_runs_1659_to_1805() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaMetals, et((2026, 6, 10), (9, 0, 0)));
    assert!(
        h.is_open(et((2026, 6, 10), (16, 58, 59))),
        "the last minute before the close"
    );
    assert!(
        !h.is_open(et((2026, 6, 10), (16, 59, 0))),
        "the close instant is closed"
    );
    assert!(
        !h.is_open(et((2026, 6, 10), (18, 4, 59))),
        "inside the printed daily gap"
    );
    assert!(
        h.is_open(et((2026, 6, 10), (18, 5, 0))),
        "the evening reopen"
    );
    assert!(!h.is_open(et((2026, 6, 13), (12, 0, 0))), "Saturday closed");
    assert!(
        !h.is_open(et((2026, 6, 12), (16, 59, 0))),
        "Friday 16:59 ends the week"
    );
}

#[test]
fn oanda_metals_answers_the_witnessed_eras_on_their_own_opening_days() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaMetals);
    // The legacy USD-pair grid `18:03 - 16:58`, witnessed
    // 2021-12-08..2023-06-05.
    assert_eq!(
        calendar.is_open(et((2022, 6, 7), (18, 2, 59))),
        Ok(false),
        "one second before the legacy open"
    );
    assert_eq!(
        calendar.is_open(et((2022, 6, 7), (18, 3, 0))),
        Ok(true),
        "the legacy open instant"
    );
    assert_eq!(
        calendar.is_open(et((2022, 6, 8), (16, 57, 59))),
        Ok(true),
        "the legacy week's last minute"
    );
    assert_eq!(
        calendar.is_open(et((2022, 6, 8), (16, 58, 0))),
        Ok(false),
        "the legacy close is end-exclusive"
    );
    // The per-day machine-channel witnessing (an 18:03-minute tick on every
    // classified opening day — a minute inside the current grid's break)
    // serves the legacy grid exactly through the 2024-02-08 opening day.
    assert_eq!(
        calendar.is_open(et((2023, 8, 8), (18, 2, 59))),
        Ok(false),
        "one second before the legacy open on a per-day-witnessed date"
    );
    assert_eq!(
        calendar.is_open(et((2023, 8, 8), (18, 3, 0))),
        Ok(true),
        "the legacy open instant on a per-day-witnessed date"
    );
    assert_eq!(
        calendar.is_open(et((2023, 8, 9), (16, 58, 0))),
        Ok(false),
        "the legacy close is end-exclusive on a per-day-witnessed date"
    );
    // The dated boundary: opening day 2024-02-08 (Thursday) is the legacy
    // grid's last positively witnessed day; opening day 2024-02-11 (Sunday)
    // is the first current-grid day: its Monday closing leg contains S5
    // starts at 16:58:45, :50 and :55, inside the earlier grid's break.
    // Friday and Saturday open no session, so no intersection days remain.
    let last_legacy = calendar
        .session_bounds(et((2024, 2, 9), (12, 0, 0)))
        .expect("a witnessed date answers")
        .expect("noon Friday sits in the session opened Thursday");
    assert_eq!(
        last_legacy,
        (et((2024, 2, 8), (18, 3, 0)), et((2024, 2, 9), (16, 58, 0))),
        "the legacy era's final session"
    );
    let first_current = calendar
        .session_bounds(et((2024, 2, 12), (12, 0, 0)))
        .expect("a witnessed date answers")
        .expect("noon Monday sits in the session opened Sunday");
    assert_eq!(
        first_current,
        (
            et((2024, 2, 11), (18, 5, 0)),
            et((2024, 2, 12), (16, 59, 0))
        ),
        "the current era's first session"
    );
    assert_refused_variant(
        &calendar.is_closed_trade_date(
            chrono::NaiveDate::from_ymd_opt(2022, 6, 8).expect("valid date"),
            SessionKind::Both,
        ),
        DateCoverage::UnresolvedGap,
        "the witnessed span below the first audited window has no holiday bracket",
    );
}

#[test]
fn oanda_metals_group_closes_at_the_printed_instants() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaMetals);
    // The gold/silver book closes with the US group at its printed instant:
    // 14:29 on 2026-05-25 (Memorial Day), 12:59 on 2026-07-03, 13:44 ET on
    // 2025-12-24, and the 2025-04-18 Good Friday full closure.
    for (day, ssm) in [
        ((2026, 5, 25), 14 * 3_600 + 29 * 60),
        ((2026, 7, 3), 12 * 3_600 + 59 * 60),
        ((2025, 12, 24), 13 * 3_600 + 44 * 60),
    ] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        let instant = et((day.0, day.1, day.2), (0, 0, 0)) + chrono::Duration::seconds(ssm - 1);
        assert!(
            calendar.is_open(instant).expect("an audited date answers"),
            "{date} still trades one second before its printed early close",
        );
        let at = instant + chrono::Duration::seconds(1);
        assert!(
            !calendar.is_open(at).expect("an audited date answers"),
            "{date} is closed from its printed early close instant",
        );
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .is_ok(),
            "{date} answers its holiday classification inside the window",
        );
    }
    let good_friday = chrono::NaiveDate::from_ymd_opt(2025, 4, 18).expect("valid date");
    assert!(
        calendar
            .is_closed_trade_date(good_friday, SessionKind::Both)
            .expect("inside the audited window the holiday layer answers"),
        "2025-04-18 is the stated full closure",
    );
}

// ---------------------------------------------------------------------------
// oanda_us_index_cfd / oanda_us_commodity_cfd — one clock, two timezones.
// ---------------------------------------------------------------------------

#[test]
fn oanda_us_books_print_one_absolute_clock_in_two_zones() {
    // Chicago 17:01-15:59 (indices/bonds) and New York 18:01-16:59 (energies)
    // are the same instants in both DST regimes.
    for key in [
        MarketHoursKey::OandaUsIndexCfd,
        MarketHoursKey::OandaUsCommodityCfd,
    ] {
        let h = hours_for_market_hours_key(key, et((2026, 6, 10), (9, 0, 0)));
        let winter_open = et((2026, 1, 11), (18, 1, 0)); // 18:01 EST = 17:01 CST
        assert!(
            h.is_open(winter_open),
            "{key:?} opens at New York 18:01 / Chicago 17:01 in winter",
        );
        let summer_open = et((2026, 7, 12), (18, 1, 0));
        assert!(
            h.is_open(summer_open),
            "{key:?} opens at New York 18:01 / Chicago 17:01 in summer",
        );
        assert!(
            !h.is_open(et((2026, 6, 10), (18, 0, 59))),
            "the daily gap's last second"
        );
        assert!(
            !h.is_open(et((2026, 6, 12), (16, 59, 0))),
            "Friday 16:59 ends the week"
        );
        assert!(!h.is_open(et((2026, 6, 13), (12, 0, 0))), "Saturday closed");
        assert_eq!(h.tz, America::New_York);
    }
}

#[test]
fn oanda_us_index_cfd_group_closes_at_the_printed_instants() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaUsIndexCfd);
    // Good Friday 2026 ships the indices' 9:14 close. Memorial Day
    // queries refuse the adjacent incomplete reopen; see oanda_publication.
    for (day, ssm) in [
        ((2026, 4, 3), 9 * 3_600 + 14 * 60),
        ((2025, 12, 24), 13 * 3_600 + 14 * 60),
        ((2025, 7, 3), 13 * 3_600 + 15 * 60),
    ] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        let instant = et((day.0, day.1, day.2), (0, 0, 0)) + chrono::Duration::seconds(ssm - 1);
        assert!(
            calendar.is_open(instant).expect("an audited date answers"),
            "{date} still trades one second before its printed early close",
        );
        assert!(
            !calendar
                .is_open(instant + chrono::Duration::seconds(1))
                .expect("an audited date answers"),
            "{date} is closed from its printed early close instant",
        );
    }
}

#[test]
fn oanda_us_commodity_cfd_group_closes_at_the_printed_instants() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaUsCommodityCfd);
    for (day, ssm) in [
        ((2026, 5, 25), 14 * 3_600 + 29 * 60),
        ((2026, 7, 3), 12 * 3_600 + 59 * 60),
        ((2025, 12, 24), 13 * 3_600 + 44 * 60),
    ] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        let instant = et((day.0, day.1, day.2), (0, 0, 0)) + chrono::Duration::seconds(ssm - 1);
        assert!(
            calendar.is_open(instant).expect("an audited date answers"),
            "{date} still trades one second before its printed early close",
        );
        assert!(
            !calendar
                .is_open(instant + chrono::Duration::seconds(1))
                .expect("an audited date answers"),
            "{date} is closed from its printed early close instant",
        );
    }
    let good_friday = chrono::NaiveDate::from_ymd_opt(2026, 4, 3).expect("valid date");
    assert!(
        calendar
            .is_closed_trade_date(good_friday, SessionKind::Both)
            .expect("inside the audited window the holiday layer answers"),
        "2026-04-03 is the stated full closure",
    );
}

// ---------------------------------------------------------------------------
// oanda_brent — London-anchored per the page's own dagger note.
// ---------------------------------------------------------------------------

#[test]
fn oanda_brent_opens_sunday_2301_london_in_both_dst_regimes() {
    let winter =
        hours_for_market_hours_key(MarketHoursKey::OandaBrent, lon((2026, 1, 12), (9, 0, 0)));
    assert!(
        !winter.is_open(lon((2026, 1, 11), (23, 0, 59))),
        "one second before the winter Sunday open",
    );
    assert!(
        winter.is_open(lon((2026, 1, 11), (23, 1, 0))),
        "the winter Sunday open"
    );
    let summer =
        hours_for_market_hours_key(MarketHoursKey::OandaBrent, lon((2026, 7, 12), (9, 0, 0)));
    assert!(
        summer.is_open(lon((2026, 7, 12), (23, 1, 0))),
        "the summer Sunday open"
    );
    assert_eq!(summer.tz, Europe::London);
}

#[test]
fn oanda_brent_breaks_match_the_dagger_note() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaBrent, lon((2026, 6, 10), (9, 0, 0)));
    // Monday-Thursday: the six-minute 21:59-22:05 break...
    assert!(
        !h.is_open(lon((2026, 6, 10), (21, 59, 0))),
        "the evening break opens"
    );
    assert!(
        !h.is_open(lon((2026, 6, 10), (22, 4, 59))),
        "inside the six-minute break"
    );
    assert!(
        h.is_open(lon((2026, 6, 10), (22, 5, 0))),
        "the evening reopen"
    );
    // ...the two-hour 22:59-01:01 break...
    assert!(
        !h.is_open(lon((2026, 6, 10), (22, 59, 0))),
        "the overnight break opens"
    );
    assert!(
        !h.is_open(lon((2026, 6, 11), (1, 0, 59))),
        "inside the two-hour break"
    );
    assert!(
        h.is_open(lon((2026, 6, 11), (1, 1, 0))),
        "the overnight reopen"
    );
    // ...and Friday ends the week at 21:59.
    assert!(
        h.is_open(lon((2026, 6, 12), (21, 58, 59))),
        "the week's last minute"
    );
    assert!(
        !h.is_open(lon((2026, 6, 12), (21, 59, 0))),
        "Friday 21:59 ends the week"
    );
    assert!(
        !h.is_open(lon((2026, 6, 13), (12, 0, 0))),
        "Saturday closed"
    );
}

#[test]
fn oanda_brent_group_closes_at_the_printed_instants_in_london_time() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaBrent);
    // Memorial Day 2025: 1:30 PM ET = 18:30 London. Christmas Eve 2025:
    // 13:59 ET = 18:59 London. Thanksgiving Friday: 3:00 PM ET = 20:00
    // London. All five-offset dates, so the conversion is flat.
    for (day, ssm) in [
        ((2025, 5, 26), 18 * 3_600 + 30 * 60),
        ((2025, 12, 24), 18 * 3_600 + 59 * 60),
        ((2025, 11, 28), 20 * 3_600),
        ((2026, 5, 25), 18 * 3_600 + 29 * 60),
    ] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        let instant = lon((day.0, day.1, day.2), (0, 0, 0)) + chrono::Duration::seconds(ssm - 1);
        assert!(
            calendar.is_open(instant).expect("an audited date answers"),
            "{date} still trades one second before its converted early close",
        );
        assert!(
            !calendar
                .is_open(instant + chrono::Duration::seconds(1))
                .expect("an audited date answers"),
            "{date} is closed from its converted early close instant",
        );
    }
    for day in [(2025, 1, 1), (2025, 12, 25), (2026, 1, 1), (2026, 4, 3)] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("inside the audited window the holiday layer answers"),
            "{date} is the stated full closure",
        );
    }
    // 2025-04-18 is a stated closure too, but its session opens on
    // 2025-04-17 at 23:01 London — and 04-17 is one of this book's withheld
    // dates — so answering the classification completely would consult a
    // gap: it refuses typed rather than answer from a half-known day.
    let shadowed = chrono::NaiveDate::from_ymd_opt(2025, 4, 18).expect("valid date");
    assert_refused_variant(
        &calendar.is_closed_trade_date(shadowed, SessionKind::Both),
        DateCoverage::UnresolvedGap,
        "Good Friday 2025's opening day is withheld, so the classification refuses",
    );
}

#[test]
fn oanda_brent_withholds_its_silent_dates() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaBrent);
    for day in [(2025, 1, 9), (2025, 4, 17), (2025, 7, 3), (2025, 11, 26)] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        assert_refused_variant(
            &calendar.is_closed_trade_date(date, SessionKind::Both),
            DateCoverage::UnresolvedGap,
            "the page's Brent silence on a page with a regular-hours vocabulary withholds",
        );
    }
}

// ---------------------------------------------------------------------------
// Wire forms.
// ---------------------------------------------------------------------------

#[test]
fn oanda_book_wire_forms_are_snake_case_and_stable() {
    for (key, name) in [
        (MarketHoursKey::OandaMetals, "oanda_metals"),
        (MarketHoursKey::OandaUsIndexCfd, "oanda_us_index_cfd"),
        (
            MarketHoursKey::OandaUsCommodityCfd,
            "oanda_us_commodity_cfd",
        ),
        (MarketHoursKey::OandaBrent, "oanda_brent"),
    ] {
        assert_eq!(key.as_str(), name);
        assert_eq!(name.parse::<MarketHoursKey>().unwrap(), key);
    }
}

#[test]
fn oanda_us_books_answer_the_witnessed_eras_on_their_own_opening_days() {
    // All three US books share the clock: whole-hour `18:00 - 17:00` New York
    // through the witnessed legacy era, then the current `18:01 - 16:59`
    // grid — which nests inside the legacy print, so it is also the undated
    // changeover's sourced intersection.
    for key in [
        MarketHoursKey::OandaUsIndexCfd,
        MarketHoursKey::OandaUsBondCfd,
        MarketHoursKey::OandaUsCommodityCfd,
    ] {
        let calendar = calendar_for_market_hours_key(key);
        // Legacy era, witnessed 2021-08-04 (index) and 2021-03-03 (bond,
        // commodity) through 2023-06-05.
        assert_eq!(
            calendar.is_open(et((2022, 6, 7), (17, 59, 59))),
            Ok(false),
            "{key:?}: one second before the legacy open"
        );
        assert_eq!(
            calendar.is_open(et((2022, 6, 7), (18, 0, 0))),
            Ok(true),
            "{key:?}: the legacy open instant"
        );
        assert_eq!(
            calendar.is_open(et((2022, 6, 8), (16, 59, 59))),
            Ok(true),
            "{key:?}: the legacy week's last minute"
        );
        assert_eq!(
            calendar.is_open(et((2022, 6, 8), (17, 0, 0))),
            Ok(false),
            "{key:?}: the legacy close is end-exclusive"
        );
        // The undated span serves the intersection (the current grid): the
        // nested grids admit no positive candle witness for the current
        // state, so the archived page-print bounds stand.
        assert_eq!(
            calendar.is_open(et((2023, 8, 8), (18, 0, 59))),
            Ok(false),
            "{key:?}: one second before the intersection open"
        );
        assert_eq!(
            calendar.is_open(et((2023, 8, 8), (18, 1, 0))),
            Ok(true),
            "{key:?}: the intersection open instant"
        );
        assert_eq!(
            calendar.is_open(et((2023, 8, 9), (16, 58, 59))),
            Ok(true),
            "{key:?}: the intersection's last minute"
        );
        assert_eq!(
            calendar.is_open(et((2023, 8, 9), (16, 59, 0))),
            Ok(false),
            "{key:?}: the intersection close is end-exclusive"
        );
        // The witnessed span sits below the first audited window, so the
        // holiday classification refuses typed while sessions answer.
        assert_refused_variant(
            &calendar.is_closed_trade_date(
                chrono::NaiveDate::from_ymd_opt(2022, 6, 8).expect("valid date"),
                SessionKind::Both,
            ),
            DateCoverage::UnresolvedGap,
            "{key:?}: the witnessed span below the first audited window has no holiday bracket",
        );
    }
}

#[test]
fn oanda_books_refuse_below_their_own_witnesses() {
    // Below each book's first witness the grid is carried and every
    // date-aware query refuses. The witnesses differ per book: metals'
    // exact rows are first printed 2021-12-08, the index clock 2021-08-04,
    // the bond/commodity clocks 2021-03-03, and Brent's London dagger grid
    // 2024-02-21 — leaving its whole earlier history carried.
    for (key, carried) in [
        (MarketHoursKey::OandaMetals, (2021, 6, 16)),
        (MarketHoursKey::OandaUsIndexCfd, (2021, 6, 16)),
        (MarketHoursKey::OandaUsBondCfd, (2021, 2, 15)),
        (MarketHoursKey::OandaUsCommodityCfd, (2021, 2, 15)),
    ] {
        let calendar = calendar_for_market_hours_key(key);
        assert_refused(
            calendar.is_closed_trade_date(
                chrono::NaiveDate::from_ymd_opt(carried.0, carried.1, carried.2)
                    .expect("valid date"),
                SessionKind::Both,
            ),
            DateCoverage::OutsideCoveredRange,
            calendar,
            et((carried.0, carried.1, carried.2), (12, 0, 0)),
        );
    }
    let brent = calendar_for_market_hours_key(MarketHoursKey::OandaBrent);
    assert_refused(
        brent.is_closed_trade_date(
            chrono::NaiveDate::from_ymd_opt(2023, 6, 7).expect("valid date"),
            SessionKind::Both,
        ),
        DateCoverage::OutsideCoveredRange,
        brent,
        et((2023, 6, 7), (12, 0, 0)),
    );
}
