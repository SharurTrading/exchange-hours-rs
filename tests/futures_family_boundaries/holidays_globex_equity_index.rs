// SPDX-License-Identifier: MIT-0

//! `globex_equity_index`'s built-in holiday rows, over the public surface.
//!
//! Every probe below is stated in America/Chicago wall clock — the zone CME
//! prints its trading hours in — and converted to UTC by `ct`, so a reader can
//! compare a line here against the operator's published instant without doing
//! the arithmetic. The family's normal trading day for venue-local trade date
//! `D` opens 17:00 CT on the preceding business evening, runs the 08:30-15:15
//! CT regular session and the 15:15-16:00 CT extended leg, and ends at its
//! 16:00 CT final close on `D`; the rows move that close or delete the day.
//!
//! The cases are the design memo's §4.1 seven: a closed day, both sides of an
//! early close, the late-open branch (absent in this window, and fenced as
//! absent), the wrap a closure removes, the trade-date consequence, both edges
//! of the coverage window, and `without_holidays` restoring the normal answer.
//!
//! The file holds two kinds of claim, and Stage 2B's coverage contract
//! (LAW-COVERAGE) separates them. Every case stated **at or above** the
//! permanent 2025-01-01 support floor still fences the schedule itself: the
//! session bounds, the trade date, the end-exclusive close and the candle. The
//! `era_*` and `wave2_*` cases state the **audited rows** the family has shipped
//! since 2010 — their dates, kinds, instants, tiers, order and each era's
//! totals — and no schedule answer for them survives, because every date they
//! audit precedes the floor. Those cases therefore state the refusal itself, on
//! the same dates and through the same entry points the schedule probes used,
//! and their row assertions carry the rest of the fence. This family declares
//! the Sunday 16:00-16:15 CT order-entry queue withheld (#79), which is why a
//! pre-floor probe that falls outside every session is refused as
//! `OutsideCoveredRange` rather than as the floor: see `assert_declared_refusal`.

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, DateCoverage, EvidenceTier,
    ExceptionBlock, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    SessionState, calendar_for_exchange, calendar_for_market_hours_key, hours_for_market_hours_key,
};

/// The family calendar under test, with its built-in table attached.
fn equity_index() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
}

/// A venue-local America/Chicago wall clock, converted to the UTC instant the
/// public surface takes.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// The identity-backed calendar a coverage refusal names.
///
/// `CalendarSource` is `#[non_exhaustive]`, so a match outside the crate needs
/// a wildcard arm even though the two variants below are the whole enum. The
/// `None` it produces is unreachable from anything this file can build.
fn calendar_of(source: CalendarSource) -> ExchangeCalendar {
    match source {
        CalendarSource::Exchange(exchange) => Some(calendar_for_exchange(exchange)),
        CalendarSource::MarketHoursKey(key) => Some(calendar_for_market_hours_key(key)),
        _ => None,
    }
    .expect("CalendarSource has exactly these two variants")
}

/// Asserts that an identity-backed date-aware query refused, with the error the
/// identity's own coverage declares for the venue-local date it could not
/// establish (LAW-COVERAGE).
///
/// Stage 2B fixes the support floor at 2025-01-01, so every audited row the
/// pre-2025 eras below ship is still the crate's claim while no schedule
/// question about those dates has an answer. Each probe therefore states the
/// refusal where it used to state an open, a session bound or a trade date, and
/// the verdict is read from `CalendarCoverage::coverage_on` instead of being
/// copied beside the probe: `BeforeSupportFloor` below the floor,
/// `OutsideCoveredRange` on a date the family has no sourced answer for — every
/// date before the knowledge-bound 2026-08-22 era is one, because this family
/// declares the Sunday 16:00-16:15 CT order-entry queue withheld (#79) — and
/// `UnresolvedGap` on a date the identity withholds.
///
/// The row, kind, tier, window and count assertions beside each probe are the
/// part of the old claim the fixed snapshot still states.
fn assert_declared_refusal<T: std::fmt::Debug>(result: Result<T, CalendarQueryError>) {
    let error = result.expect_err("the query must state its coverage refusal, not an answer");
    let coverage = calendar_of(error.source()).coverage();
    let day = error.date();
    let verdict = coverage.coverage_on(day);
    let states_verdict = matches!(
        (verdict, error),
        (
            DateCoverage::BeforeSupportFloor,
            CalendarQueryError::BeforeSupportFloor { .. }
        ) | (
            DateCoverage::OutsideCoveredRange,
            CalendarQueryError::OutsideCoveredRange { .. }
        ) | (
            DateCoverage::UnresolvedGap,
            CalendarQueryError::UnresolvedGap { .. }
        )
    );
    // One documented exception: `trade_date` resolves a containing session
    // before it consults the floor, so an instant that lies in no session falls
    // through to the order-entry scan, and that scan reports the declared
    // phase-level gap ahead of the floor. The exception reaches exactly as far
    // as a pre-floor date on an identity that declares such a gap.
    let declared_phase_gap = matches!(
        (verdict, error),
        (
            DateCoverage::BeforeSupportFloor,
            CalendarQueryError::OutsideCoveredRange { .. }
        )
    ) && !coverage.phase_gaps().is_empty();
    assert!(
        states_verdict || declared_phase_gap,
        "{day} carries the verdict {verdict:?}: {error:?} does not state it"
    );
}

/// 12:00 CT, the Monday/Thursday-holiday final close.
const NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Christmas-Eve and day-after-Thanksgiving final close.
const QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;

// ---------------------------------------------------------------------------
// 1. A closed day.
// ---------------------------------------------------------------------------

/// Christmas 2025 falls on a Thursday and CME publishes no equity-index
/// session for it: the trade date is gone, so nothing the crate would have
/// assigned to it survives — including the Wednesday-evening leg.
#[test]
fn christmas_2025_is_a_closed_trade_date_with_no_session_of_its_own() {
    let calendar = equity_index();

    let holiday = calendar
        .holiday_on(day(2025, 12, 25))
        .expect("2025-12-25 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::Closed);
    assert_eq!(holiday.tier(), EvidenceTier::T2);
    assert_eq!(holiday.document_id(), "CME-SVC-2025-12-24");

    assert!(
        calendar
            .is_closed_trade_date(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    // Three probes inside the civil day, before the evening leg that belongs
    // to the *next* trade date opens at 17:00 CT.
    for time in [(3, 0, 0), (10, 0, 0), (16, 0, 0)] {
        assert!(
            !calendar
                .is_open(ct((2025, 12, 25), time))
                .expect("the coverage contract must answer a covered date"),
            "2025-12-25 {time:?} CT must be closed"
        );
    }

    // The civil day is not wholly closed: 17:00 CT opens trade date 12-26.
    assert!(
        !calendar
            .is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on each side of the cutoff.
// ---------------------------------------------------------------------------

/// The day after Thanksgiving 2025 closes 12:15 CT. The trading day opened
/// 17:00 CT on Thursday, so the clip has to land on the Friday and take the
/// 15:15-16:00 CT leg — which opens after the cutoff — with it.
#[test]
fn the_day_after_thanksgiving_2025_closes_at_1215_central() {
    // Thanksgiving Day publishes no final close of its own, so 2025-11-28 now
    // owns the span from Wednesday evening and its own regular session is cut at
    // the same 12:15 the earlier `EarlyClose` row stated.
    static EXPECTED: [ExceptionBlock; 6] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 8 * 3_600 + 30 * 60),
        ExceptionBlock::regular(-1, 8 * 3_600 + 30 * 60, 12 * 3_600),
        ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60),
        ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, QUARTER_PAST_NOON),
    ];
    let calendar = equity_index();
    let holiday = calendar
        .holiday_on(day(2025, 11, 28))
        .expect("2025-11-28 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::ReplacementBlocks(&EXPECTED));
    assert_eq!(holiday.document_id(), "CME-SVC-2025-11-26");

    // One second before the cutoff, and at it: closes are end-exclusive.
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The rest of the regular session is gone, and so is the extended leg
    // that would have opened at 15:15 CT.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 11, 28), (8, 30, 0)),
            ct((2025, 11, 28), (12, 15, 0))
        ))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
}

/// Martin Luther King Jr. Day 2025 is the memo's §1.1 worked example: CME
/// prints `12:00 preopen`, matching stops, and the trading day that opened
/// 17:00 CT on Sunday ends there instead of at 16:00 CT.
#[test]
fn martin_luther_king_day_2025_clips_the_sunday_evening_trading_day() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2025, 1, 20))
            .expect("2025-01-20 ships a row")
            .kind(),
        HolidayKind::EarlyClose { close_ssm: NOON }
    );

    // The Sunday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(
        calendar
            .is_open(ct((2025, 1, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 1, 20), (11, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 1, 20), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 1, 20), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // 17:00 CT that evening opens trade date 2025-01-21, which is normal.
    assert!(
        calendar
            .is_open(ct((2025, 1, 20), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 1, 20), (12, 5, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 1, 20), (17, 0, 0)))
    );
}

/// Good Friday 2026 is the one date CME itself flags as an exception, for the
/// U.S. employment release: equity index closes 08:15 CT, before its own
/// regular session would have opened, so the whole day session disappears and
/// only the Thursday-evening leg survives.
#[test]
fn good_friday_2026_closes_equity_index_at_0815_central() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2026, 4, 3))
            .expect("2026-04-03 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60
        }
    );

    assert!(
        calendar
            .is_open(ct((2026, 4, 3), (8, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 3), (8, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 3), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 4, 3), (2, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 4, 2), (17, 0, 0)), ct((2026, 4, 3), (8, 15, 0))))
    );
}

// ---------------------------------------------------------------------------
// 4. The late-open branch.
// ---------------------------------------------------------------------------

/// No `globex_equity_index` row in this coverage window moves a first open.
///
/// CME reopens every one of these holidays at the family's ordinary 17:00 CT,
/// and the one sourced *later* first open the corpus carries for this family —
/// the 05:00 CT Saturday sessions of 2026-06-20, 2026-07-04 and 2027-06-19 —
/// would have to **create** a session the normal week does not have, which the
/// scalar vocabulary cannot state and which ships as a declared gap instead
/// (design memo D7). This fence keeps that a recorded fact rather than an
/// accident: a late-open row appearing here without its evidence fails.
#[test]
fn no_late_open_row_ships_in_the_2025_2027_window() {
    let calendar = equity_index();
    let mut date = day(2025, 1, 1);
    let last = day(2027, 12, 31);

    while date <= last {
        if let Some(holiday) = calendar.holiday_on(date) {
            // `ReplacementBlocks` joined the vocabulary in Stage 4: it states a
            // complete trading day for a date a scalar row cannot describe, and
            // it is not a late open. The fence this test draws is unchanged.
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed
                        | HolidayKind::EarlyClose { .. }
                        | HolidayKind::ReplacementBlocks(_)
                ),
                "{date}: this window ships closures, early closes and complete-day \
                 replacements, not {:?}",
                holiday.kind()
            );
        }
        date = date
            .succ_opt()
            .expect("the window ends well before the epoch bound");
    }

    // The Saturday sessions are rows now, on the trade date each carries: the
    // session itself is stated by the following Monday's replacement row, and
    // the Saturday holds no holiday row of its own.
    for (saturday, trade_date) in [
        ((2026, 6, 20), (2026, 6, 22)),
        ((2026, 7, 4), (2026, 7, 6)),
        ((2027, 6, 19), (2027, 6, 21)),
    ] {
        assert!(
            calendar
                .is_open(ct(saturday, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{saturday:?}: the Saturday session must be open"
        );
        assert_eq!(
            calendar
                .trade_date(ct(saturday, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(day(trade_date.0, trade_date.1, trade_date.2)),
            "{saturday:?}: the session carries the following Monday"
        );
        assert_eq!(
            calendar.holiday_on(day(saturday.0, saturday.1, saturday.2)),
            None,
            "{saturday:?}: the session is stated on its trade date, not the Saturday"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. A wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes 12:15 CT and does not reopen, because the
/// Wednesday 17:00 CT leg would have carried trade date 2025-12-25 and that
/// date is closed. The next open is the Thursday-evening leg, on the holiday
/// itself, for trade date 2025-12-26.
#[test]
fn the_christmas_2025_closure_removes_the_previous_evenings_wrap() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2025, 12, 24))
            .expect("2025-12-24 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON
        }
    );

    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The normal 17:00 CT open of Wednesday evening is gone with its trade date.
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 12, 24), (12, 20, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 12, 25), (17, 0, 0))),
        "the next open is the holiday evening's leg, not a same-day remainder"
    );
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date; the leg that opens on a closed
/// date carries the date after it.
///
/// The two probes that land **outside** every session do not survive this
/// family's declared phase-level gap. `trade_date` resolves a containing
/// session before it consults the floor, so an instant in no session falls
/// through to the order-entry scan, and that scan is withheld for every date
/// before the knowledge-bound 2026-08-22 era (#79: the Sunday 16:00-16:15 CT
/// queue this family does not serve). Both therefore state the refusal their
/// own coverage declares rather than the `None` they used to answer. The three
/// probes inside a session are answered normally and are unchanged.
#[test]
fn holiday_rows_move_the_trade_date_only_where_the_operator_does() {
    let calendar = equity_index();

    // Inside the shortened day after Thanksgiving 2025.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // The shortened Martin Luther King Jr. Day 2025 carries the *next* trade
    // date: CME publishes no final close for the holiday, so the operator labels
    // the whole span from Sunday evening with 2025-01-21.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 21))
    );
    // The evening open on closed Christmas Day 2025 belongs to the Friday.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
    // Nothing is assigned to the closed trade date itself, and the evening
    // before that closure has no session at all to assign: both are refused.
    assert_declared_refusal(calendar.trade_date(ct((2025, 12, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2025, 12, 24), (18, 0, 0))));
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// The window is a claim about every date inside it, so both its edges are
/// fenced: one day out, the table has no answer.
///
/// The schedule half of the old fence is gone with the floor and the audit
/// edge: the identity-backed calendar refuses both 2009-12-25 (below the
/// 2025-01-01 support floor) and 2028-01-01 (above the audited window), so the
/// "the crate and the pre-table answer agree" comparison is not available. The
/// detached calendar states the normal week on its own, as it always did.
#[test]
fn the_coverage_window_is_exactly_2010_through_2027() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2010, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));
    // The table audits six eras from 2010 through 2027 with no gap between
    // them, so `contains` is true on every date from 2010-01-01 to 2027-12-31
    // and false only outside the first and last window. The 2013-2015 and
    // 2019-2021 intervals joined the table when those waves shipped, and the
    // 2024-12-31 edge moved in with the 2022-2024 wave: a date with no row
    // inside a window answers `None` as an audited normal date rather than as
    // silence.
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert!(coverage.contains(day(2013, 6, 14)));
    assert!(coverage.contains(day(2020, 12, 25)));
    assert_eq!(
        calendar.holiday_on(day(2020, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(coverage.contains(day(2024, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2013, 6, 14)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);

    // Christmas Day 2009 is a CME Globex closure one year below the window, and
    // New Year's Day 2028 one day above it. The identity-backed calendar
    // refuses both; the detached calendar claims no complete calendar and so
    // still states the normal week it serves, where 2028-01-01 is a Saturday
    // the family never trades.
    let bare = calendar.without_holidays();
    assert_declared_refusal(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2028, 1, 1), (10, 0, 0))));
    assert!(
        !bare
            .is_open(ct((2028, 1, 1), (10, 0, 0)))
            .expect("a detached calendar states the normal week it serves")
    );
}

// ---------------------------------------------------------------------------
// 8. `without_holidays` restores the normal answer.
// ---------------------------------------------------------------------------

/// The escape hatch is exact: detaching the table reproduces the pre-table
/// answer over a dense grid across the Christmas 2025 week, and the two
/// calendars must actually disagree inside it, or the grid proves nothing.
#[test]
fn without_holidays_restores_the_normal_week_across_the_christmas_2025_week() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2025, 12, 25)), None);

    // The normal week has a session at each of these instants; the table
    // removes or shortens all three.
    assert!(
        bare.is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2025, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // Over the whole week, at five-minute resolution, the detached calendar
    // reproduces the pre-table engine exactly — a detached `MarketHours`
    // snapshot carries no identity and therefore no table, so it is the
    // control — while the attached one differs on at least one probe.
    let mut instant = ct((2025, 12, 21), (0, 0, 0));
    let end = ct((2025, 12, 28), (0, 0, 0));
    let mut differed = false;
    while instant < end {
        let pre_table =
            hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, instant).is_open(instant);
        assert_eq!(
            bare.is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            pre_table,
            "without_holidays diverged from the pre-table answer at {instant}"
        );
        differed |= calendar
            .is_open(instant)
            .expect("the coverage contract must answer a covered date")
            != pre_table;
        instant += Duration::minutes(5);
    }
    assert!(
        differed,
        "the table must change an answer inside the Christmas 2025 week"
    );
}

// ---------------------------------------------------------------------------
// 9. The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 10:30 CT, the era's Monday/Thursday-holiday equity-index final close.
const ERA_MONDAY_HOLIDAY_CLOSE: u32 = 10 * 3_600 + 30 * 60;
/// 08:15 CT, the era's Good Friday equity-index final close.
const ERA_GOOD_FRIDAY_CLOSE: u32 = 8 * 3_600 + 15 * 60;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening, so the cutoff has to delete the whole of the remaining 08:30-15:15
/// CT day session while leaving the opening wrap in place.
///
/// Martin Luther King Jr. Day 2010 is the 10:30 CT shape; Good Friday 2010 is
/// the 08:15 CT shape, stated before the era's own 08:30 CT day open, so only
/// the Thursday-evening leg survives to it.
///
/// Both dates precede the 2025-01-01 floor, so the clip the paragraph above
/// describes is no longer observable. What survives is the row itself — each
/// date ships the `EarlyClose` its stated instant — and the refusal at every
/// probe the clip used to be read through, including both sides of each cutoff
/// and the trade date the evening leg carried.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2010, 1, 18))
            .expect("2010-01-18 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_MONDAY_HOLIDAY_CLOSE
        }
    );
    // The Sunday-evening leg that feeds this trade date, one second before the
    // cutoff, at it, after it, and the bounds it used to answer.
    assert_declared_refusal(calendar.is_open(ct((2010, 1, 17), (17, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2010, 1, 18), (10, 29, 59))));
    assert_declared_refusal(calendar.is_open(ct((2010, 1, 18), (10, 30, 0))));
    assert_declared_refusal(calendar.is_open(ct((2010, 1, 18), (15, 0, 0))));
    assert_declared_refusal(calendar.session_bounds(ct((2010, 1, 18), (9, 0, 0))));
    // And the 17:00 CT evening leg that would have opened 2010-01-19.
    assert_declared_refusal(calendar.trade_date(ct((2010, 1, 18), (18, 0, 0))));

    assert_eq!(
        calendar
            .holiday_on(day(2010, 4, 2))
            .expect("2010-04-02 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_GOOD_FRIDAY_CLOSE
        }
    );
    assert_declared_refusal(calendar.is_open(ct((2010, 4, 2), (8, 14, 59))));
    assert_declared_refusal(calendar.is_open(ct((2010, 4, 2), (8, 15, 0))));
    assert_declared_refusal(calendar.session_bounds(ct((2010, 4, 2), (2, 0, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2010, 4, 2), (8, 15, 0))));
}

/// Every late open the era ships states exactly 05:00 CT, which is *earlier* than
/// the family's normal 17:00 CT first open, so the cutoff lands on the trade date
/// itself: the Monday-evening leg that would have opened trade date 2011-12-27
/// did not run, and matching starts at 05:00 CT on the Tuesday.
///
/// The two Good Friday eves the retrieval also states a `1530 CT - Regular CME
/// Globex open` for — 2010-04-01 and 2012-04-05 — ship **no** row: the crate's
/// own era grid already ends those eves' day sessions at 15:15 CT and reopens
/// at the ordinary 17:00 CT, and the operator's 15:30 instant is the trade
/// date's *own* evening open rather than a delayed one. A `late_open` row at
/// 15:30 would land after its own trade date's close and delete the session;
/// the dates are named as gaps in the family's evidence file instead.
///
/// Where the 05:00 CT cutoff lands is a schedule claim about a pre-2025 date, so
/// only the rows and the absence of the two 15:30 dates survive; each probe
/// states the refusal in place of the open it used to fence.
#[test]
fn era_late_opens_land_on_the_trade_date_itself() {
    let calendar = equity_index();
    let era_late_opens = [(2011, 12, 27), (2012, 1, 3), (2012, 12, 26)];
    for (year, month, date) in era_late_opens {
        let row = calendar
            .holiday_on(day(year, month, date))
            .unwrap_or_else(|| panic!("{year}-{month:02}-{date:02} ships a row"));
        assert_eq!(
            row.kind(),
            HolidayKind::LateOpen {
                open_ssm: 5 * 3_600
            },
            "{year}-{month:02}-{date:02} must state the operator's 05:00 CT first open"
        );
        // Both sides of the printed first open, and the trade date the day
        // session carried.
        assert_declared_refusal(calendar.is_open(ct((year, month, date), (4, 59, 59))));
        assert_declared_refusal(calendar.is_open(ct((year, month, date), (5, 0, 0))));
        assert_declared_refusal(calendar.trade_date(ct((year, month, date), (9, 0, 0))));
    }
    for (year, month, date) in [(2010, 4, 1), (2012, 4, 5)] {
        assert_eq!(
            calendar.holiday_on(day(year, month, date)),
            None,
            "{year}-{month:02}-{date:02} ships no row: the operator's 1530 CT open is its own evening open"
        );
    }
}

/// The 2012-07-03 equity row is an `EarlyClose` at 12:15 CT on its own trade
/// date. CME also states a `1530 CT - Regular CME Globex open for trade date
/// Thursday, July 5` on that sheet, but that instant belongs to the *next*
/// trade date: the crate's scalar vocabulary cannot state a same-day re-open
/// after a close, so the evening leg of 07-05 is not modelled and is recorded
/// as a gap in the family's evidence file. The row that ships states 12:15 CT.
///
/// That the session stopped there, and that 07-05 kept its ordinary 17:00 CT
/// leg, are pre-floor schedule claims; the row and the refusals at the same
/// probes are what this test can state now.
#[test]
fn era_2012_07_03_early_close_ends_the_trade_date_at_1215_central() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2012, 7, 3))
            .expect("2012-07-03 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }
    );

    // Both sides of the printed close, its day-session bounds, and the next
    // open and trade date the sheet pairs with it.
    assert_declared_refusal(calendar.is_open(ct((2012, 7, 3), (12, 14, 59))));
    assert_declared_refusal(calendar.is_open(ct((2012, 7, 3), (12, 15, 0))));
    assert_declared_refusal(calendar.session_bounds(ct((2012, 7, 3), (9, 0, 0))));
    assert_declared_refusal(calendar.next_session_open_after(ct((2012, 7, 3), (12, 15, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2012, 7, 3), (17, 0, 0))));
}

/// A date inside the widened window with no row is audited normal: the crate
/// serves the profile's own 2010 week, not a holiday-shaped guess.
///
/// With the window below the 2025-01-01 floor, "the attached and detached
/// calendars agree instant for instant" is no longer a claim either calendar
/// makes — both refuse every probe — so what remains is that each date ships no
/// row and that both calendars refuse it.
#[test]
fn era_dates_without_rows_are_audited_normal() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();

    for (date, time) in [
        ((2010, 6, 15), (10, 0, 0)),
        ((2010, 6, 14), (17, 0, 0)),
        ((2011, 3, 9), (9, 0, 0)),
        ((2012, 10, 10), (14, 0, 0)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?} must ship no row"
        );
        assert_declared_refusal(calendar.is_open(ct(date, time)));
        assert_declared_refusal(bare.is_open(ct(date, time)));
    }
}

/// The widened window's edges as the module declares them: it opens on
/// 2010-01-01, closes on 2027-12-31, and Christmas Day 2009 — a real CME
/// closure one year below the window — carries no row.
///
/// The schedule half of this fence cannot survive the floor: 2009-12-25 is
/// below 2025-01-01, so neither the attached calendar's "the table does not
/// reach it, so the normal Thursday is served" nor the detached calendar's
/// agreement with it is observable any more. Both refuse.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(!coverage.contains(day(2009, 12, 25)));

    assert_eq!(calendar.holiday_on(day(2009, 12, 25)), None);
    assert_declared_refusal(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_declared_refusal(bare.is_open(ct((2009, 12, 25), (10, 0, 0))));
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// 15:30 CT, the open the 2018 Christmas sheet prints on 26 December.
const WAVE2_LATE_OPEN: u32 = 15 * 3_600 + 30 * 60;
/// 12:15 CT is the same instant the 2025-2027 window's half-days close at, so
/// the era reuses the constant above rather than restating it.
const WAVE2_HALF_DAY_CLOSE: u32 = QUARTER_PAST_NOON;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening: 12:00 CT on the nine Monday and Thursday holidays, 12:15 CT on the
/// three Thanksgiving Fridays, the three Independence Day eves and the two
/// Christmas Eves CME prints one.
///
/// The instants are the era's own row payload and are still asserted; the
/// clipped day around them is below the floor, so every probe that read the
/// clip states the refusal instead.
#[test]
fn wave2_early_closes_end_the_wrapped_trading_day_at_the_printed_instant() {
    let calendar = equity_index();

    for (date, previous_day, close_ssm) in [
        ((2016, 1, 18), (2016, 1, 17), NOON),
        ((2016, 11, 24), (2016, 11, 23), NOON),
        ((2016, 11, 25), (2016, 11, 24), WAVE2_HALF_DAY_CLOSE),
        ((2017, 7, 3), (2017, 7, 2), WAVE2_HALF_DAY_CLOSE),
        ((2018, 12, 24), (2018, 12, 23), WAVE2_HALF_DAY_CLOSE),
        ((2018, 12, 26), (2018, 12, 25), 16 * 3_600),
    ] {
        if close_ssm == 16 * 3_600 {
            // 2018-12-26 is the late-open case, asserted on its own below.
            continue;
        }
        let kind = calendar
            .holiday_on(day(date.0, date.1, date.2))
            .map(Holiday::kind);
        assert_eq!(
            kind,
            Some(HolidayKind::EarlyClose { close_ssm }),
            "{date:?}"
        );

        // The evening leg that feeds this trade date, both sides of the
        // printed close, the daily candle and the trade date it carried.
        assert_declared_refusal(calendar.is_open(ct(previous_day, (17, 0, 0))));
        let cutoff = ct(date, (close_ssm / 3_600, (close_ssm % 3_600) / 60, 0));
        assert_declared_refusal(calendar.is_open(cutoff - Duration::seconds(1)));
        assert_declared_refusal(calendar.is_open(cutoff));
        assert_declared_refusal(
            calendar.candle_end(ct(date, (9, 0, 0)), CalendarResolution::Daily),
        );
        assert_declared_refusal(calendar.trade_date(ct(date, (9, 0, 0))));
        assert_declared_refusal(calendar.trade_date(cutoff - Duration::seconds(1)));
    }

    // The holiday's own evening 17:00 CT leg, and the instant that belongs to
    // the neighbouring family rather than this one.
    assert_declared_refusal(calendar.trade_date(ct((2016, 1, 18), (18, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2016, 11, 25), (12, 5, 0))));
}

/// A closure removes the trade date and the leg that opened the previous
/// evening, and the operator's own stated re-open — always this family's
/// ordinary 17:00 CT — starts the next trade date.
///
/// Every date here is below the floor, so the removals and re-opens the
/// paragraph above describes are not observable. The era's closure date set,
/// read back from `holiday_on` below, is the part that survives.
#[test]
fn wave2_closures_remove_the_trade_date_and_the_prior_evening_leg() {
    let calendar = equity_index();

    // Good Friday 2016 is a full closure: the Thursday-evening leg that fed
    // the Friday trade date goes with it, and the Friday-evening leg CME
    // publishes starts the following Monday's trade date.
    assert_eq!(
        calendar.holiday_on(day(2016, 3, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(calendar.is_closed_trade_date(day(2016, 3, 25), SessionKind::Both));
    assert_declared_refusal(calendar.is_open(ct((2016, 3, 24), (18, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2016, 3, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.next_session_open_after(ct((2016, 3, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2016, 3, 27), (18, 0, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2016, 3, 27), (18, 0, 0))));

    // New Year's Day 2016 fell on a Friday, so the closure is the observed
    // Friday and the next open is the Sunday leg into Monday's trade date.
    assert_eq!(
        calendar.holiday_on(day(2016, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 31), (17, 0, 0))));
    assert_declared_refusal(calendar.next_session_open_after(ct((2016, 1, 1), (12, 0, 0))));

    // Every one of the era's nine closures is a closure, and nothing else in
    // the era is.
    let mut closures = Vec::new();
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        if calendar.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Closed) {
            closures.push(date);
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        closures,
        [
            day(2016, 1, 1),
            day(2016, 3, 25),
            day(2016, 12, 26),
            day(2017, 1, 2),
            day(2017, 4, 14),
            day(2017, 12, 25),
            day(2018, 1, 1),
            day(2018, 3, 30),
            day(2018, 12, 25),
        ]
    );
}

/// 2018-12-26 is the era's one late open: the Christmas sheet prints the
/// Equity line's `Pre-opening 15:15` and `Open 15:30` on a date the crate's
/// grid has open at 15:15 CT, so the first open moves half an hour later and
/// nothing else about the day moves.
///
/// The row is the claim that survives; the moved boundary, the bounds it
/// produced and the 16:00 CT close the row left alone are all pre-floor
/// schedule answers, so each probe states the refusal instead.
#[test]
fn wave2_2018_12_26_opens_late_at_1530_central() {
    let calendar = equity_index();
    let date = day(2018, 12, 26);
    let cutoff = ct((2018, 12, 26), (15, 30, 0));

    assert_eq!(
        calendar.holiday_on(date).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: WAVE2_LATE_OPEN
        })
    );
    assert_declared_refusal(calendar.is_open(cutoff - Duration::seconds(1)));
    assert_declared_refusal(calendar.is_open(cutoff));
    assert_declared_refusal(calendar.session_bounds(cutoff + Duration::seconds(1)));
    assert_declared_refusal(calendar.trade_date(cutoff));
    assert_declared_refusal(calendar.is_open(ct((2018, 12, 26), (15, 59, 0))));
    assert_declared_refusal(calendar.is_open(ct((2018, 12, 26), (16, 0, 0))));
}

/// The era's own window edges, and the one unaudited interval below it.
///
/// The two dates probed here are below the floor, so the closure each one
/// carries is the only claim left: the attached calendar refuses them, and so
/// does the detached one — the "the table changes this answer" contrast the
/// old fence drew is not observable.
#[test]
fn wave2_window_edges_and_unaudited_neighbours_answer_as_declared() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

    assert!(coverage.contains(day(2016, 1, 1)));
    assert!(coverage.contains(day(2018, 12, 31)));
    assert!(coverage.contains(day(2015, 12, 31)));
    // Every interval below the 2016-2018 wave is a window of its own: the
    // 2013-2015 and 2019-2021 waves both shipped after this test was written.
    assert!(coverage.contains(day(2019, 1, 1)));

    // 2015-12-25 is the 2013-2015 wave's own Christmas closure.
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 25), (10, 0, 0))));
    assert_declared_refusal(bare.is_open(ct((2015, 12, 25), (10, 0, 0))));
    // 2019-01-01 is one day above it and is now audited: the 2019-2021 wave
    // ships the New Year closure the crate previously served as an ordinary
    // Tuesday.
    assert_eq!(
        calendar.holiday_on(day(2019, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(calendar.is_open(ct((2019, 1, 1), (10, 0, 0))));
    assert_declared_refusal(bare.is_open(ct((2019, 1, 1), (10, 0, 0))));
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence. A sample would let a slipped close move
/// unnoticed on the dates nobody probed, so the sweep below walks the whole
/// window and compares against this list row for row: a dropped, added or
/// moved row fails as loudly as a wrong instant.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    (
        (2022, 1, 17),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 2, 21),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 5, 30),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 6, 20),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 7, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 9, 5),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 24),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    (
        (2023, 5, 29),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 6, 19),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 7, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 9, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 23),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2024, 1, 15),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 2, 19),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 5, 27),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 6, 19),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 7, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 9, 2),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 28),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    ((2024, 12, 25), HolidayKind::Closed, EvidenceTier::T2),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, time: (u32, u32, u32)) -> DateTime<Utc> {
    ct((date.year(), date.month(), date.day()), time)
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The ordinary 17:00 CT evening open that follows a closed trade date: this
/// civil date's own leg when the week has one, otherwise the Sunday evening
/// that opens the next week — the grid has no Friday-evening occurrence, which
/// is why a Good Friday closure's next session is Sunday's.
fn era_reopen_after_closure(date: NaiveDate) -> DateTime<Utc> {
    let reopen = if date.weekday() == Weekday::Fri {
        date.checked_add_days(Days::new(2))
            .expect("the era is far from the representable bound")
    } else {
        date
    };
    ct_on(reopen, (17, 0, 0))
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
///
/// The row half of the sweep is unchanged — order, kind, instant, tier and the
/// era's shape all still fail on a dropped, added or moved row. The boundaries
/// themselves are pre-floor schedule answers, so each of the instants the sweep
/// used to fence is now stated as the refusal the coverage contract returns.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = equity_index();
    let mut index = 0_usize;
    let (mut noons, mut quarters, mut closures, mut unsourced) = (0_usize, 0, 0, 0);
    let mut date = day(2022, 1, 1);
    while date <= day(2024, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = ERA_ROWS[index];
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the era's rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            match kind {
                HolidayKind::EarlyClose { close_ssm } => {
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    if close_ssm == NOON {
                        noons += 1;
                    } else {
                        quarters += 1;
                    }
                    // The wrap that opened this trade date, and the trade date
                    // it carried.
                    assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (17, 0, 0))));
                    assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (19, 30, 0))));
                    assert_declared_refusal(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                    );
                    // Both sides of the printed close, the day session's
                    // bounds, the daily candle and the trade date.
                    assert_declared_refusal(calendar.is_open(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(calendar.is_open(cutoff));
                    assert_declared_refusal(calendar.session_bounds(ct_on(date, (9, 0, 0))));
                    assert_declared_refusal(
                        calendar.candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily),
                    );
                    assert_declared_refusal(calendar.trade_date(cutoff - Duration::seconds(1)));
                }
                HolidayKind::Closed => closures += 1,
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: the era ships no {other:?}"),
            }
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_ROWS.len(), "every planned row ships");
    assert_eq!(
        (noons, quarters, closures, unsourced),
        (19, 6, 7, 3),
        "the era's shape"
    );
}

/// A closure removes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
///
/// Every date this walk visits is pre-floor, so the removal and the reopen are
/// not claimable; each probe states the refusal, and `ERA_ROWS` above remains
/// the fence for which dates ship a closure and how many the era has.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = equity_index();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(date.0, date.1, date.2);
        let previous = day_before(date);
        assert_declared_refusal(calendar.is_closed_trade_date(date, SessionKind::Both));
        // The evening leg that would have carried this trade date.
        assert_declared_refusal(calendar.is_open(ct_on(previous, (17, 0, 0))));
        assert_declared_refusal(calendar.is_open(ct_on(previous, (19, 30, 0))));
        // The trade date's own civil day, its trade date, and the reopen.
        assert_declared_refusal(calendar.is_open(ct_on(date, (9, 0, 0))));
        assert_declared_refusal(calendar.is_open(ct_on(date, (15, 59, 0))));
        assert_declared_refusal(calendar.trade_date(ct_on(date, (10, 0, 0))));
        assert_declared_refusal(calendar.next_session_open_after(ct_on(date, (10, 0, 0))));
        // A mid-week closure left the same day's evening leg in place, and that
        // leg carried the next trade date.
        let reopen = era_reopen_after_closure(date);
        if reopen == ct_on(date, (17, 0, 0)) {
            assert_declared_refusal(calendar.trade_date(reopen));
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date is refused, and refused identically by
/// the detached calendar: the row states that the date was audited and makes no
/// scheduling claim, but below the 2025-01-01 floor neither calendar has an
/// answer to compare, so the "the row clips nothing" equality this helper used
/// to fence is no longer observable. What remains is the row's own claim —
/// kind and tier — plus the refusal at every probe the old comparison used,
/// which is the `OutsideCoveredRange` its declared Sunday order-entry gap (#79)
/// states wherever a probe falls outside every session.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = equity_index();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    assert_declared_refusal(calendar.is_closed_trade_date(date, SessionKind::Both));
    assert_declared_refusal(detached.is_closed_trade_date(date, SessionKind::Both));

    let previous = day_before(date);
    for probe in [
        ct_on(previous, (18, 0, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (15, 59, 0)),
        ct_on(date, (18, 0, 0)),
    ] {
        assert_declared_refusal(calendar.is_open(probe));
        assert_declared_refusal(detached.is_open(probe));
        assert_declared_refusal(calendar.trade_date(probe));
        assert_declared_refusal(detached.trade_date(probe));
        assert_declared_refusal(calendar.session_bounds(probe));
        assert_declared_refusal(detached.session_bounds(probe));
        assert_declared_refusal(calendar.next_session_open_after(probe));
        assert_declared_refusal(detached.next_session_open_after(probe));
        assert_declared_refusal(calendar.candle_end(probe, CalendarResolution::Daily));
        assert_declared_refusal(detached.candle_end(probe, CalendarResolution::Daily));
    }
}

/// Every `Unsourced` row the era ships changes no answer — or would, if the
/// date were answerable: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing, but no query about it has an answer to
/// compare (see `assert_unsourced_changes_nothing`).
#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = equity_index();
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date.0, date.1, date.2);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
    }
}

/// The 2022-2024 window sits fifth in the declared coverage, and the 2013-2015
/// interval below the 2016-2018 wave is a window of its own since this wave
/// shipped. (The 2019-2021 interval this test used to fence became a window of
/// its own when that wave shipped; the section below fences it.)
///
/// The window list and the shipped rows at each date are the claims that
/// survive; the sessions those rows shut are below the floor and are refused.
#[test]
fn era_2022_2024_window_sits_fifth_and_the_2013_2015_era_is_audited() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2013, 1, 1), day(2015, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(coverage.contains(day(2021, 12, 31)));
    // 2022-01-01 is a Saturday the operator audits normal, so a date with no
    // row inside the window answers `None`, not a closure.
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    // The 2013-2015 wave shipped after this test was written, so the dates it
    // used to probe for an unaudited gap now carry its shipped rows. Those rows
    // are the claim left here: 2015-12-25 is its Christmas closure and
    // 2015-12-24 the 12:15 CT early close it shipped beside it, and both dates
    // are below the floor, so the sessions they shut are refused rather than
    // answered.
    assert_eq!(
        calendar.holiday_on(day(2013, 6, 14)),
        None,
        "an ordinary Friday inside the new window is audited normal"
    );
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "the 2013-2015 wave's own Christmas closure"
    );
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 24)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        })
    );
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 24), (13, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 24), (18, 0, 0))));
}

/// This family's 2022-2024 grid keeps the trading day in two phases — the
/// 17:00 CT overnight leg into the 08:30 CT regular session, then the regular
/// session to the final close — and an early close clips only the phase it
/// lands in, leaving the overnight phase's 08:30 CT end untouched.
///
/// Neither date is answerable below the 2025-01-01 support floor, so the two
/// bounds the paragraph above describes are no longer claimable: both phases'
/// probes state the refusal, and `ERA_ROWS` remains the fence for the instants
/// the era ships.
#[test]
fn era_2022_2024_early_closes_clip_the_day_phase_not_the_overnight_one() {
    let calendar = equity_index();

    for date in [(2022, 1, 17), (2024, 12, 24)] {
        let previous = day_before(day(date.0, date.1, date.2));
        assert_declared_refusal(calendar.session_bounds(ct_on(previous, (18, 0, 0))));
        assert_declared_refusal(calendar.session_bounds(ct(date, (9, 0, 0))));
    }
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// 08:15 CT, the one Good Friday 2021 close this era prints.
const ERA_EIGHT_FIFTEEN: u32 = 8 * 3_600 + 15 * 60;

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. The
/// boundaries those counts fence are pre-floor schedule answers, so each probe
/// states the refusal instead.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = equity_index();
    let (mut noons, mut quarters, mut eight_fifteens) = (0_usize, 0, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut rows = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::EarlyClose { close_ssm } => {
                    match close_ssm {
                        NOON => noons += 1,
                        QUARTER_PAST_NOON => quarters += 1,
                        ERA_EIGHT_FIFTEEN => eight_fifteens += 1,
                        other => {
                            panic!("{date}: this family ships no {other}-second CT close here")
                        }
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date, and the trade date
                    // it carried.
                    assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (17, 0, 0))));
                    assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (19, 30, 0))));
                    assert_declared_refusal(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                    );
                    // Both sides of the printed close, and the phase bounds and
                    // daily candle the instant used to end: the 08:15 Good
                    // Friday close landed inside the overnight phase, and a
                    // 12:00/12:15 close inside the regular one.
                    assert_declared_refusal(calendar.is_open(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(calendar.is_open(cutoff));
                    let inside = cutoff - Duration::minutes(1);
                    assert_declared_refusal(calendar.session_bounds(inside));
                    if close_ssm != ERA_EIGHT_FIFTEEN {
                        assert_declared_refusal(
                            calendar.session_bounds(ct_on(day_before(date), (18, 0, 0))),
                        );
                    }
                    assert_declared_refusal(calendar.candle_end(inside, CalendarResolution::Daily));
                    assert_declared_refusal(calendar.trade_date(cutoff - Duration::seconds(1)));
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert_declared_refusal(calendar.is_closed_trade_date(date, SessionKind::Both));
                    // The evening leg that would have carried this trade date,
                    // and the trade date's own session.
                    assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (17, 0, 0))));
                    assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (19, 30, 0))));
                    assert_declared_refusal(calendar.is_open(ct_on(date, (9, 0, 0))));
                    assert_declared_refusal(calendar.is_open(ct_on(date, (15, 59, 0))));
                    assert_declared_refusal(calendar.trade_date(ct_on(date, (10, 0, 0))));
                }
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: this era ships no {other:?}"),
            }
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 36, "the era's rows");
    assert_eq!(
        (noons, quarters, eight_fifteens, closures, unsourced),
        (18, 6, 1, 8, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships changes no answer — or would, if the
/// date were answerable: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing, but no query about it has an answer to
/// compare (see `assert_unsourced_changes_nothing`).
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = equity_index();
    let mut checked = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date)
            && row.kind() == HolidayKind::Unsourced
        {
            assert_unsourced_changes_nothing(date, row, EvidenceTier::T1);
            checked += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(checked, 3, "the era's `Unsourced` rows");
}

/// The era is its own declared window: 2019-01-01 and 2021-12-31 are inside
/// it, while 2018-12-31 and 2022-01-01 belong to the waves either side and lie
/// outside it.
///
/// Both edges are below the floor, so their own answers are not claimable; the
/// shipped rows at each edge are, and the last day's probe states the refusal.
#[test]
fn era_2019_2021_window_edges_answer_as_the_module_declares() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
    let era = (day(2019, 1, 1), day(2021, 12, 31));

    assert!(
        coverage.windows().contains(&era),
        "the 2019-2021 window is declared as a window of its own"
    );
    let inside = |date: NaiveDate| era.0 <= date && date <= era.1;
    assert!(inside(day(2019, 1, 1)));
    assert!(inside(day(2021, 12, 31)));
    assert!(
        !inside(day(2018, 12, 31)),
        "2018-12-31 belongs to the 2016-2018 wave, not to this era"
    );
    assert!(
        !inside(day(2022, 1, 1)),
        "2022-01-01 belongs to the 2022-2024 wave, not to this era"
    );

    // The era's own edges carry their rows: its first day is the shipped New
    // Year closure, and its last is an ordinary Friday this table audited.
    assert_eq!(
        calendar.holiday_on(day(2019, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day(2021, 12, 31)), None);
    assert_declared_refusal(calendar.is_open(ct((2021, 12, 31), (9, 0, 0))));
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day(2018, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2013, 1, 1), day(2015, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );

    // Every shipped row of the era is inside the era's own window: the walk
    // reads the module, and the declared window is what must contain it.
    let mut rows = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if calendar.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 36, "the era's rows");
    for probe in [day(2018, 12, 31), day(2022, 1, 1)] {
        assert_eq!(calendar.holiday_on(probe), None, "{probe}");
    }
}

/// The 2019-2021 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2019_2021_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2019, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2019, 4, 19), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2019, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2019, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2020, 4, 10), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 5, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2020, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2020, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 9, 7),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2021, 1, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 2, 15),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 4, 2),
        HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 5, 31),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2021, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2021, 7, 5),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 9, 6),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 12, 24), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2019_2021_rows_are_the_audited_date_kind_and_tier_set() {
    let calendar = equity_index();
    let mut index = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = *ERA_2019_2021_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2019-2021 window that the block does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the 2019-2021 rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2019_2021_ROWS.len(), "every recorded row ships");
}

// ---------------------------------------------------------------------------
// The 2013-2015 rows.
// ---------------------------------------------------------------------------

/// The era-wide sweep: every row the 2013-2015 window ships, read from the
/// module, with both sides of every instant it states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the side of the instant it moved from,
/// and a kind this family does not ship fails outright. The count tuple is the
/// era's shape as the block records it; the handwritten table below pins the
/// date set the counts cannot see. Every instant the sweep probes belongs to a
/// pre-2025 date, so those probes state the refusal.
#[test]
fn era_2013_2015_sweeps_every_shipped_row_kind_and_instant() {
    let venue = equity_index();
    let (mut closed, mut early, mut late, mut both) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
        if let Some(row) = venue.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::Closed => {
                    closed += 1;
                    assert_declared_refusal(venue.is_closed_trade_date(date, SessionKind::Both));
                    // The prior evening's leg, which the closure removed.
                    assert_declared_refusal(venue.is_open(ct_on(day_before(date), (17, 0, 0))));
                }
                HolidayKind::EarlyClose { close_ssm } => {
                    early += 1;
                    let (h, m, s) = (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let cutoff = ct_on(date, (h, m, s));
                    // Both sides of the printed close, the trade date it ended
                    // and the daily candle.
                    assert_declared_refusal(venue.is_open(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(cutoff));
                    assert_declared_refusal(venue.trade_date(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(
                        venue.candle_end(cutoff - Duration::minutes(1), CalendarResolution::Daily),
                    );
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late += 1;
                    let (h, m, s) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let open = ct_on(date, (h, m, s));
                    // Both sides of the printed first open, and the trade date
                    // the day session carried.
                    assert_declared_refusal(venue.is_open(open - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(open));
                    assert_declared_refusal(venue.trade_date(open + Duration::hours(1)));
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    both += 1;
                    let (oh, om, os) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let (ch, cm, cs) =
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let open = ct_on(date, (oh, om, os));
                    let cutoff = ct_on(date, (ch, cm, cs));
                    assert_declared_refusal(venue.is_open(open - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(open));
                    assert_declared_refusal(venue.is_open(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(cutoff));
                }
                other => panic!("{date}: this era ships no {other:?}"),
            }
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        (closed, early, late, both),
        (8, 27, 3, 0),
        "the era's shape"
    );
}

/// The era is its own declared window: 2013-01-01 is inside it and 2012-12-31
/// and 2016-01-01 belong to the waves either side and lie outside it.
#[test]
fn era_2013_2015_window_edges_answer_as_the_module_declares() {
    let venue = equity_index();
    let coverage = venue
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
    let era = (day(2013, 1, 1), day(2015, 12, 31));

    assert!(
        coverage.windows().contains(&era),
        "the 2013-2015 window is declared as a window of its own"
    );
    assert!(coverage.contains(era.0));
    assert!(coverage.contains(era.1));

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is the last date the window declares.
    assert_eq!(
        venue.holiday_on(day(2013, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        coverage.contains(day(2015, 12, 31)),
        "the era's last day is inside the declared window"
    );
    // The neighbouring dates are their own eras' business: 2012-12-31 is
    // audited normal by the wave below, 2016-01-01 opens the next declared
    // window, and neither is this era.
    assert_eq!(venue.holiday_on(day(2012, 12, 31)), None);
    assert!(
        coverage.contains(day(2016, 1, 1)),
        "2016-01-01 is the next declared window's first day"
    );
    // A date below the January-2010 floor is outside every window, so this
    // table has no answer for it at all.
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert_eq!(venue.holiday_on(day(2009, 12, 31)), None);
}

/// The family's coverage names its windows in order, and the 2013-2015 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2013_2015_window_is_declared_in_order_and_bounds_every_row() {
    let venue = equity_index();
    let coverage = venue
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2013, 1, 1), day(2015, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );

    let mut rows = 0_usize;
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
        if venue.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 38, "the era's rows");
    assert_eq!(venue.holiday_on(day(2012, 12, 31)), None);
    assert!(!coverage.contains(day(2028, 1, 1)));
    assert_eq!(venue.holiday_on(day(2028, 1, 1)), None);
}

/// The 2013-2015 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2013_2015_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2013, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 3, 29), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2014, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 4, 18), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 5, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 9, 1),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 1, 19),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 2, 16),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 4, 3),
        HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 5, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 9, 7),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2015, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2013_2015_rows_are_the_audited_date_kind_and_tier_set() {
    let venue = equity_index();
    let mut index = 0_usize;
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
        if let Some(row) = venue.holiday_on(date) {
            let (expected, kind, tier) = *ERA_2013_2015_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2013-2015 window that the block does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the 2013-2015 rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2013_2015_ROWS.len(), "every recorded row ships");
}

// ---------------------------------------------------------------------------
// The Saturday-session trade dates (Stage 4, #116)
//
// CME states a Saturday session on three trade dates in this window, each
// carrying the following Monday. Every expectation below is read from the
// service windows the rows cite: two of the three trade dates are printed by
// two windows — the Saturday session by one, the Sunday legs by the window that
// starts on that Sunday — and
// `a_saturday_rows_sunday_legs_are_sourced_from_a_window_that_prints_them` pins
// that the evidence rows name both. The family's envelope is the 2021-06-27
// removal of the 15:15-15:30 halt, so the overnight leg, the regular session
// and the post-regular slice are one continuous matching span; the row splits
// that span into ordered blocks at the regular boundaries so the regular phase
// survives the replacement, and the assertions below pin the phase as well as
// the span.
// ---------------------------------------------------------------------------

/// A venue-local calendar date stated as `(year, month, day)`.
type Ymd = (i32, u32, u32);

/// The three trade dates CME states a Saturday session for, with the Saturday
/// each session opens on and the Friday whose early close precedes it.
const SATURDAY_SESSION_DATES: [(Ymd, Ymd, Ymd); 3] = [
    ((2026, 6, 22), (2026, 6, 20), (2026, 6, 19)),
    ((2026, 7, 6), (2026, 7, 4), (2026, 7, 3)),
    ((2027, 6, 21), (2027, 6, 19), (2027, 6, 18)),
];

#[test]
fn the_equity_index_states_its_saturday_sessions_on_the_trade_date() {
    let calendar = equity_index();
    for (trade_date, saturday, friday) in SATURDAY_SESSION_DATES {
        assert_eq!(
            day(saturday.0, saturday.1, saturday.2).weekday(),
            Weekday::Sat
        );
        assert_eq!(
            day(trade_date.0, trade_date.1, trade_date.2).weekday(),
            Weekday::Mon
        );
        assert!(
            matches!(
                calendar
                    .holiday_on(day(trade_date.0, trade_date.1, trade_date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::ReplacementBlocks(_))
            ),
            "{trade_date:?} must carry a replacement row"
        );
        assert_eq!(
            calendar
                .holiday_on(day(trade_date.0, trade_date.1, trade_date.2))
                .map(Holiday::document_id),
            Some(match trade_date {
                (2026, 6, 22) => "CME-SVC-2026-06-18",
                (2026, 7, 6) => "CME-SVC-2026-07-03",
                _ => "CME-SVC-2027-06-17",
            }),
            "{trade_date:?}: the row cites the window its instants were read from"
        );

        // The Saturday session: 05:00 open, 17:00 end-exclusive close, and the
        // following Monday as its trade date.
        let saturday_open = ct(saturday, (5, 0, 0));
        let saturday_close = ct(saturday, (17, 0, 0));
        assert!(
            calendar
                .is_open(saturday_open)
                .expect("2026 and 2027 are covered dates"),
            "{saturday:?}: the Saturday session is not open"
        );
        assert_eq!(
            calendar
                .session_bounds(ct(saturday, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((saturday_open, saturday_close)),
            "{saturday:?}: the Saturday session's own bounds"
        );
        assert!(
            !calendar
                .is_open(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            "{saturday:?}: the close is end-exclusive"
        );
        assert_eq!(
            calendar
                .trade_date(ct(saturday, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2))
        );

        // The Friday before it closes early at 12:00, ending the session that
        // opened Thursday evening. The operator dates that session to this row's
        // trade date, so the Friday morning is the Monday's, not the holiday's.
        assert!(
            calendar
                .is_open(ct(friday, (11, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{friday:?}: the eve must be open before the early close"
        );
        assert!(
            !calendar
                .is_open(ct(friday, (12, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{friday:?}: the early close is end-exclusive"
        );
        assert_eq!(
            calendar
                .trade_date(ct(friday, (11, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2)),
            "{friday:?}: the early-close morning belongs to the trade date the \
             operator prints on it, which is the Saturday session's"
        );
        // And no Friday-evening session is claimed: CME publishes none.
        assert!(
            !calendar
                .is_open(ct(friday, (18, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{friday:?}: no Friday-evening open is published for these dates"
        );

        // The Sunday-Monday part survives the row, and its regular session does
        // too: the envelope is one continuous span whose 08:30-15:15 slice is
        // also the family's `regular` session, and the block must not have
        // deleted it.
        let sunday = (saturday.0, saturday.1, saturday.2 + 1);
        assert!(
            calendar
                .is_open(ct(sunday, (18, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{sunday:?}: the Sunday-evening open was deleted by the row"
        );
        assert!(
            calendar
                .is_open(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the regular session was deleted by the row"
        );
        // The phase is the row's, not the ordinary week's: a replacement
        // replaces the complete trade date and the block scan selects by kind,
        // so a set that stated only the extended envelope would answer
        // `OpenExtended` here and move the regular bounds to the next day.
        assert!(
            calendar
                .is_open_regular(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: 10:00 CT must be inside the regular session the row states"
        );
        assert_eq!(
            calendar
                .session_state(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            SessionState::OpenRegular,
            "{trade_date:?}: the row must keep the regular phase on the trade date"
        );
        assert_eq!(
            calendar
                .session_bounds_with(ct(trade_date, (10, 0, 0)), SessionKind::Regular)
                .expect("2026 and 2027 are covered dates"),
            Some((ct(trade_date, (8, 30, 0)), ct(trade_date, (15, 15, 0)))),
            "{trade_date:?}: the regular bounds must be the trade date's own 08:30-15:15 CT"
        );
        assert!(
            calendar
                .is_open(ct(trade_date, (15, 45, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the post-regular slice was deleted by the row"
        );
        assert!(
            !calendar
                .is_open(ct(trade_date, (16, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the final close is end-exclusive"
        );
        assert_eq!(
            calendar
                .trade_date(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2))
        );

        // The gap between the Saturday session and the Sunday open is closed.
        for hour in [17_u32, 20, 23] {
            assert!(
                !calendar
                    .is_open(ct(saturday, (hour, 0, 0)))
                    .expect("2026 and 2027 are covered dates"),
                "{saturday:?}: {hour}:00 falls between the blocks and must not be open"
            );
        }
    }
}

/// A Saturday-session row's Sunday legs must be sourced from a window that
/// actually prints them.
///
/// Two of these rows span **two** windows: the window that carries the Saturday
/// session stops at that Saturday and prints no Sunday entry at all, so the
/// Sunday Pre-Open, the Sunday-17:00 open and the trade date's `16:00 closed`
/// come from the window that starts on the Sunday. The third row's window
/// happens to run through its Sunday and does print the legs, so it needs only
/// the one.
///
/// This fence exists because an independent review found the two-window rows
/// claiming a Sunday pair that their cited artifact does not contain. It pins
/// the distinction that the per-row citation check cannot see.
#[test]
fn a_saturday_rows_sunday_legs_are_sourced_from_a_window_that_prints_them() {
    // (trade date, the window that prints the Saturday session, the window that
    // prints the Sunday legs)
    for (trade_date, saturday_window, sunday_window) in [
        ("2026-06-22", "CME-SVC-2026-06-18", "CME-SVC-2026-06-21"),
        ("2026-07-06", "CME-SVC-2026-07-03", "CME-SVC-2026-07-03"),
        ("2027-06-21", "CME-SVC-2027-06-17", "CME-SVC-2027-06-20"),
    ] {
        let evidence = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("docs/evidence/globex_equity_index.md"),
        )
        .expect("the evidence file must be readable");
        for window in [saturday_window, sunday_window] {
            assert!(
                evidence.contains(&format!("| `{window}` |")),
                "{trade_date}: {window} must be a recorded document"
            );
        }
        let row = evidence
            .lines()
            .find(|line| line.starts_with(&format!("| {trade_date} |")))
            .unwrap_or_else(|| panic!("{trade_date} must have an evidence row"));
        assert!(
            row.contains(saturday_window),
            "{trade_date}: the row must cite the Saturday session's window"
        );
        // Where the two differ, the row must name the second window too: the
        // legs the first does not print are not in it.
        if sunday_window != saturday_window {
            assert!(
                row.contains(sunday_window),
                "{trade_date}: the row must name {sunday_window}, which is the window \
                 that prints the Sunday legs"
            );
        }
    }
}
