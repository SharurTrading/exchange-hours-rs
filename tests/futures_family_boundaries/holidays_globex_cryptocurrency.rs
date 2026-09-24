// SPDX-License-Identifier: MIT-0

//! `globex_cryptocurrency` holiday-table contracts, over the public surface.
//!
//! Every probe below is stated in America/Chicago wall clock — the zone CME
//! publishes in and the zone the rows are keyed in — and converted here, so a
//! daylight-saving slip cannot hide behind a hard-coded UTC literal.
//!
//! The family spans both holiday regimes the design memo names, and the two
//! answer differently on purpose:
//!
//! - the five-day 17:00-16:00 CT era closes over the weekend, so a `Closed`
//!   trade date deletes its whole trading day, wrap included;
//! - the 24/7 era from trade date 2026-05-30 assigns a block to the following
//!   open business date, so a `Closed` trade date is *skipped by that roll* and
//!   trading continues through the holiday under the next business date.
//!
//! Both are asserted. If the table and the business-date roll were wired up in
//! the wrong order the 24/7 family would go dark on every holiday, which is the
//! single highest-risk interaction in the whole feature.

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind,
    MarketHoursKey, SUPPORT_FLOOR, SessionKind, calendar_for_market_hours_key,
};

/// The family under test, as a date-aware calendar.
fn crypto() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
}

/// Asserts a date-aware query refused with `BeforeSupportFloor`, the verdict the
/// permanent 2025 floor gives a date below it (LAW-COVERAGE). The error must
/// name a venue-local day that really is below the floor, so a refusal for an
/// unrelated day or reason fails here, and an `Ok` answer fails the
/// `expect_err` above rather than being tolerated.
fn assert_before_floor<T: core::fmt::Debug>(result: Result<T, CalendarQueryError>, claim: &str) {
    let error = result.expect_err(claim);
    assert!(
        matches!(
            error,
            CalendarQueryError::BeforeSupportFloor { date, .. } if date < SUPPORT_FLOOR
        ),
        "{claim}: a date below the 2025 floor must refuse with BeforeSupportFloor; got {error}"
    );
}

/// Asserts a date-aware query refused with `OutsideCoveredRange`: the date's own
/// unsourced span, its missing holiday-layer answer, or a declared phase-level
/// gap (LAW-COVERAGE), on a date **at or above** the support floor.
///
/// The floor is checked first by every entry point, including the order-entry
/// scans: a pre-floor date reports `BeforeSupportFloor` and never the phase
/// verdict, because no range has been claimed below the floor for a phase gap to
/// be outside of. Use `assert_before_floor` for those probes.
fn assert_outside_range<T: core::fmt::Debug>(result: Result<T, CalendarQueryError>, claim: &str) {
    let error = result.expect_err(claim);
    assert!(
        matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
        "{claim}: the date is outside this identity's covered ranges, so the query must \
         refuse it with OutsideCoveredRange; got {error}"
    );
}

/// An America/Chicago wall clock, converted to the UTC the surface takes.
fn ct(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must name an unambiguous Central wall clock")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

fn kind_on(date: NaiveDate) -> Option<HolidayKind> {
    crypto().holiday_on(date).map(Holiday::kind)
}

// ---------------------------------------------------------------------------
// 1. A closed day.
// ---------------------------------------------------------------------------

/// Christmas 2025 falls in the five-day era, where a closed trade date removes
/// the complete trading day — including the Wednesday-evening block that would
/// have fed it.
///
/// The civil-day half of that claim still answers (`is_open` is false on the
/// whole day, and on the eve's leg). The trade-date half does not: resolving an
/// instant on the closed day reads a phase the family declares it cannot state,
/// so Stage 2B refuses it as `OutsideCoveredRange` rather than answering "no
/// trade date".
#[test]
fn a_closed_trade_date_has_no_session_and_no_trade_date() {
    let calendar = crypto();

    assert_eq!(kind_on(day(2025, 12, 25)), Some(HolidayKind::Closed));
    assert!(
        calendar
            .is_closed_trade_date(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    for probe in [
        ct(2025, 12, 25, 0, 30),
        ct(2025, 12, 25, 9, 0),
        ct(2025, 12, 25, 15, 0),
    ] {
        assert!(
            !calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "2025-12-25 must be shut at {probe}"
        );
        // The trade-date half of this test's name is no longer claimable:
        // resolving an instant on this closed day reads an order-entry phase
        // the family declares it cannot state (#93/#123), so the query refuses
        // the date rather than reporting "no trade date".
        assert_outside_range(
            calendar.trade_date(probe),
            "2025-12-25's trade date is refused, not stated as absent",
        );
    }

    // The eve's own 17:00 CT open would carry trade date 2025-12-25, so the
    // closure deletes it rather than leaving an orphaned evening leg. This
    // one still answers — the deletion is a fact about the shipped row, not
    // about the phase-gapped order-entry window.
    assert!(
        !calendar
            .is_open(ct(2025, 12, 24, 17, 30))
            .expect("the coverage contract must answer a covered date"),
        "the eve's evening leg belongs to the closed trade date"
    );
}

/// The five-day era's `[N3]` holidays are a trade-date merge, not a closure:
/// CME published a 16:00 CT pre-open in place of the 16:00 CT final close, with
/// no `[N6]` predecessor on the preceding evening, so matching ran from Sunday
/// 17:00 CT through to 16:00 CT as on a normal Monday and only the trade-date
/// label moved. The table carries no row, and the normal week answers.
#[test]
fn a_trade_date_merge_keeps_the_sunday_evening_block() {
    let calendar = crypto();

    assert_eq!(kind_on(day(2025, 1, 20)), None);
    for probe in [
        ct(2025, 1, 19, 18, 0),
        ct(2025, 1, 20, 9, 0),
        ct(2025, 1, 20, 15, 0),
    ] {
        assert!(
            calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "2025-01-20 matched at {probe}"
        );
        assert_eq!(
            calendar
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            Some(day(2025, 1, 20))
        );
    }
    assert!(
        !calendar
            .is_open(ct(2025, 1, 20, 16, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2025, 1, 20, 17, 30))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 1, 20, 17, 30))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 21))
    );
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on each side of the cutoff.
// ---------------------------------------------------------------------------

/// Independence Day 2025: the session opened Thursday at 17:00 CT and the
/// early close is stated on Friday's trade date, so the clip lands on the
/// correct civil day.
#[test]
fn an_early_close_ends_a_day_that_opened_the_previous_evening() {
    let calendar = crypto();
    let cutoff = ct(2025, 7, 4, 12, 0);

    assert_eq!(
        kind_on(day(2025, 7, 4)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600
        })
    );

    assert!(
        calendar
            .is_open(cutoff - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );

    let (open, close) = calendar
        .session_bounds(ct(2025, 7, 4, 9, 0))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(open, ct(2025, 7, 3, 17, 0));
    assert_eq!(close, cutoff);
    assert_eq!(
        calendar
            .candle_end(ct(2025, 7, 4, 9, 0), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff)
    );

    // Nothing re-opens behind the cutoff on that civil day.
    assert!(
        !calendar
            .is_open(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2025, 7, 4, 18, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

/// The day after Thanksgiving 2025 stops at 13:45 CT. The Thursday before it is
/// a trade-date merge and carries no row, so the early close stands on its own.
#[test]
fn an_early_close_stands_without_a_closed_neighbour() {
    let calendar = crypto();
    let cutoff = ct(2025, 11, 28, 13, 45);

    assert_eq!(kind_on(day(2025, 11, 27)), None);
    assert_eq!(
        kind_on(day(2025, 11, 28)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600 + 45 * 60
        })
    );

    assert!(
        calendar
            .is_open(cutoff - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(cutoff - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // Thursday's 17:00 CT open feeds Friday's trade date, so the Thanksgiving
    // closure must not take it.
    assert!(
        calendar
            .is_open(ct(2025, 11, 27, 18, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// 4. A late open.
// ---------------------------------------------------------------------------

/// This family ships no late open, in either era, and that is a claim rather
/// than an omission.
///
/// CME never re-opens cryptocurrency later than its normal first open on a
/// holiday: what a holiday removes is the trade date, not the start of trading.
/// Asserting the absence keeps the claim honest — the moment a later wave keys
/// a `LateOpen` row here this fails, which is what forces the two-branch
/// cutoff test the design memo requires for a late open.
#[test]
fn the_family_keys_no_late_open_row() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a holiday table");

    let mut date = coverage.first();
    while date <= coverage.last() {
        if let Some(holiday) = calendar.holiday_on(date) {
            // `Unsourced` joined the vocabulary with the 2022-2024 wave: it
            // states that a date inside a window was audited and clips nothing.
            // The claim this test fences — no `LateOpen` anywhere — is
            // unchanged and is asserted again explicitly below.
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed | HolidayKind::EarlyClose { .. } | HolidayKind::Unsourced
                ),
                "{date} ships {:?}; a late open needs its own two-branch cutoff test",
                holiday.kind()
            );
            assert!(
                !matches!(
                    holiday.kind(),
                    HolidayKind::LateOpen { .. } | HolidayKind::LateOpenAndEarlyClose { .. }
                ),
                "{date} ships {:?}; a late open needs its own two-branch cutoff test",
                holiday.kind()
            );
        }
        date = date
            .succ_opt()
            .expect("the coverage window is representable");
    }
}

// ---------------------------------------------------------------------------
// 5. A wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes at 12:45 CT and does not re-open, because the
/// block that would have opened at 17:00 CT carries the closed Christmas trade
/// date. The next session is Christmas evening, feeding Boxing Day.
#[test]
fn a_closure_removes_the_previous_evenings_wrap() {
    let calendar = crypto();

    assert_eq!(
        kind_on(day(2025, 12, 24)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60
        })
    );

    assert!(
        calendar
            .is_open(ct(2025, 12, 24, 12, 44))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2025, 12, 24, 12, 45))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2025, 12, 24, 17, 30))
            .expect("the coverage contract must answer a covered date")
    );

    let (open, _close) = calendar
        .next_session_after(ct(2025, 12, 24, 12, 20))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(open, ct(2025, 12, 25, 17, 0));
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence, in both eras.
// ---------------------------------------------------------------------------

/// Five-day era: a shortened day keeps its own trade date, and the holiday
/// evening's block already belongs to the following date.
#[test]
fn the_five_day_era_keeps_the_holidays_own_trade_dates() {
    let calendar = crypto();

    assert_eq!(
        calendar
            .trade_date(ct(2025, 7, 4, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 7, 4))
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 12, 23, 18, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 24))
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 12, 25, 17, 30))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
}

/// 24/7 era: the row deletes no trading. It makes the business-date roll skip
/// the holiday, so `is_open` stays true across the whole civil day while the
/// trade date moves to the next open business date — a Monday holiday to the
/// Tuesday, a Thursday holiday to the Friday, and a Friday holiday across the
/// weekend to the Monday.
#[test]
fn the_twenty_four_seven_era_rolls_the_trade_date_without_deleting_a_day() {
    let calendar = crypto();

    for (probe, rolled_to) in [
        // Labor Day 2026, a Monday.
        (ct(2026, 9, 7, 9, 0), day(2026, 9, 8)),
        // Thanksgiving 2026, a Thursday.
        (ct(2026, 11, 26, 9, 0), day(2026, 11, 27)),
        // Christmas 2026, a Friday: the roll crosses the weekend.
        (ct(2026, 12, 25, 9, 0), day(2026, 12, 28)),
        // Globex's closed Friday before Christmas 2027.
        (ct(2027, 12, 24, 9, 0), day(2027, 12, 27)),
    ] {
        assert!(
            calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "24/7 cryptocurrency must keep trading at {probe}"
        );
        assert_eq!(
            calendar
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            Some(rolled_to),
            "the business-date roll must skip the closed trade date at {probe}"
        );
    }

    for closed in [
        day(2026, 9, 7),
        day(2026, 11, 26),
        day(2026, 12, 25),
        day(2027, 12, 24),
    ] {
        assert_eq!(kind_on(closed), Some(HolidayKind::Closed));
        assert!(
            calendar
                .is_closed_trade_date(closed, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
        // The charter's own surprise: a full closure is not a closed civil day
        // for this family, because trading never stops.
        assert!(
            !calendar
                .is_closed_all_day_on(closed, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
    }

    // The Thursday before the closed 2027-12-24 settles its own trade date.
    assert_eq!(
        calendar
            .trade_date(ct(2027, 12, 23, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2027, 12, 23))
    );
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer at all, and must not silently extend to a neighbouring year's
/// holidays.
///
/// The date-aware side of that edge is a refusal, not an answer: 2018-12-25
/// precedes the 2025 floor and 2028-01-01 lies outside every audited window, so
/// each earns its own variant — and the detached snapshot answers the normal
/// week only for the post-floor one.
#[test]
fn the_coverage_window_has_two_hard_edges() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a holiday table");

    // The window opened at 2019-01-01 when the 2019-2021 wave shipped; its
    // three declared spans are contiguous, so the outer edges are what this
    // fence pins and `windows()` in the era sections below pins the split.
    assert_eq!(coverage.first(), day(2019, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2019, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2018, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(kind_on(day(2025, 1, 1)), Some(HolidayKind::Closed));
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    // 2024-12-31 is inside the 2022-2024 span, so its `None` is an audited
    // normal date rather than silence — and Christmas 2024 is now a row.
    assert!(coverage.contains(day(2024, 12, 31)));
    assert_eq!(
        kind_on(day(2024, 12, 25)),
        Some(HolidayKind::Closed),
        "the 2022-2024 wave audits Christmas 2024"
    );

    // Christmas 2018 is a CME closure below the window and no wave has audited
    // it. What is gone is the old reading that the identity then answers the
    // pure normal-week result: the date precedes the 2025 floor, so the
    // identity and the detached snapshot both refuse it, and the identity
    // refuses the trade-date question with the same verdict.
    let bare = calendar.without_holidays();
    for probe in [ct(2018, 12, 25, 9, 0), ct(2018, 12, 25, 18, 0)] {
        assert_before_floor(
            calendar.is_open(probe),
            "a pre-floor date is refused, not answered from the normal week",
        );
        assert_before_floor(
            calendar.trade_date(probe),
            "a pre-floor trade date is refused",
        );
        assert_before_floor(
            bare.is_open(probe),
            "detaching the table does not lift the floor",
        );
    }
    assert_eq!(calendar.holiday_on(day(2018, 12, 25)), None);

    // One day past the window the table is equally silent, but 2028-01-01 is
    // above the floor: the identity refuses the date its holiday layer has no
    // answer for, while the detached snapshot — which claims no coverage —
    // still answers the normal week.
    for probe in [ct(2028, 1, 1, 9, 0), ct(2028, 1, 1, 20, 0)] {
        assert_outside_range(
            calendar.is_open(probe),
            "2028-01-01 is outside every audited window",
        );
        assert!(
            bare.is_open(probe)
                .expect("a detached snapshot claims no coverage"),
            "{probe}: the normal week trades once the table is detached"
        );
    }
}

// ---------------------------------------------------------------------------
// `without_holidays` restores the pre-table answer.
// ---------------------------------------------------------------------------

/// The consumer's escape hatch detaches the table outright: no rows, no
/// coverage, and every query answers as it did before the family shipped one.
#[test]
fn without_holidays_restores_the_normal_week_answer() {
    let bare = crypto().without_holidays();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2025, 12, 25)), None);
    assert_eq!(bare.holiday_on(day(2026, 12, 25)), None);

    // Five-day era: the deleted Christmas day comes back.
    assert!(
        !crypto()
            .is_open(ct(2025, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct(2025, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        bare.trade_date(ct(2025, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 25))
    );

    // Five-day era: the clipped Independence Day runs to its normal 16:00 CT.
    assert!(
        !crypto()
            .is_open(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        bare.session_bounds(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
            .map(|(_open, close)| close),
        Some(ct(2025, 7, 4, 16, 0))
    );

    // 24/7 era: the business-date roll stops rolling.
    assert_eq!(
        crypto()
            .trade_date(ct(2026, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 28))
    );
    assert_eq!(
        bare.trade_date(ct(2026, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 25))
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows, all inside the five-day era.
// ---------------------------------------------------------------------------

/// 12:45 CT, every early close this era ships.
const ERA_TWELVE_FORTY_FIVE: u32 = 12 * 3_600 + 45 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T2,
    ),
    ((2024, 12, 25), HolidayKind::Closed, EvidenceTier::T2),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, hour: u32, minute: u32) -> DateTime<Utc> {
    ct(date.year(), date.month(), date.day(), hour, minute)
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end derived from
/// the row itself.
///
/// The era precedes the 2025 floor, so each derived instant is fenced by the
/// refusal it earns — `BeforeSupportFloor` — instead of by the open state,
/// bounds or candle the crate used to state for it.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = crypto();
    let mut index = 0_usize;
    let (mut twelve_forty_five, mut closures, mut unsourced) = (0_usize, 0_usize, 0_usize);
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
                    // The row's own payload is intact and still asserted: the
                    // kind, the printed instant and the tier are the table's
                    // claim, not the calendar's answer.
                    assert_eq!(close_ssm, ERA_TWELVE_FORTY_FIVE, "{date}");
                    twelve_forty_five += 1;
                    let cutoff = ct_on(date, 12, 45);
                    // Every date-aware answer this branch used to state —
                    // the clipped wrap and its trade date, the end-exclusive
                    // boundary, the trading day's bounds and its daily candle
                    // — is refused: the era precedes the 2025 floor, so the
                    // identity has no sourced answer to give. The refusal, not
                    // the answer, is what this fence can now hold.
                    for (probe, what) in [
                        (ct_on(day_before(date), 17, 0), "the wrap's open"),
                        (ct_on(day_before(date), 19, 30), "the wrap two hours in"),
                        (cutoff - Duration::seconds(1), "one second before the close"),
                        (cutoff, "the printed close itself"),
                        (ct_on(date, 9, 0), "the trade date's own morning"),
                    ] {
                        assert_before_floor(calendar.is_open(probe), what);
                    }
                    assert_before_floor(
                        calendar.trade_date(ct_on(day_before(date), 18, 0)),
                        "the wrap's trade date",
                    );
                    assert_before_floor(
                        calendar.session_bounds(ct_on(date, 9, 0)),
                        "the trading day's bounds",
                    );
                    assert_before_floor(
                        calendar.candle_end(ct_on(date, 9, 0), CalendarResolution::Daily),
                        "the trading day's daily candle",
                    );
                    assert_before_floor(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
                        "the trade date one second before the close",
                    );
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
        (twelve_forty_five, closures, unsourced),
        (4, 7, 3),
        "the era's shape"
    );
}

/// The five-day weekend roll on every closure: the trade date's whole trading
/// day goes, including the evening block that opened it, and the next session
/// the family offers is the ordinary 17:00 CT evening open — on a Friday closure
/// that is Sunday's leg into the next week's first trade date.
///
/// Every closure this era ships precedes the 2025 floor, so each of those
/// answers is refused now. The fence holds the trade-date refusal
/// (`OutsideCoveredRange`, from the withheld Pre-Open phase) and the floor
/// refusal beside it.
#[test]
fn era_2022_2024_closures_keep_the_five_day_weekend_roll() {
    let calendar = crypto();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(date.0, date.1, date.2);
        // A closure's trade-date answer rests on the trading day that opened
        // the previous evening; both dates precede the 2025 floor, so every
        // answer this test used to state — the closed trade date, the deleted
        // evening block and civil day, the absent trade date, the reopening
        // session and the Friday/Monday asymmetry — is now the same refusal.
        assert_before_floor(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "a pre-floor closure is refused, not reported closed",
        );
        for probe in [
            ct_on(day_before(date), 17, 0),
            ct_on(day_before(date), 19, 30),
            ct_on(date, 9, 0),
            ct_on(date, 15, 59),
        ] {
            assert_before_floor(
                calendar.is_open(probe),
                "the closed trading day's own instants",
            );
        }
        // Resolving the trade date reads the family's Pre-Open phase, and this
        // identity's whole-domain phase gap (#93/#123) refuses that before the
        // floor is ever reached — so this one refusal is `OutsideCoveredRange`
        // even though the date itself is pre-floor.
        assert_before_floor(
            calendar.trade_date(ct_on(date, 10, 0)),
            "the closed day's trade date",
        );
        assert_before_floor(
            calendar.next_session_open_after(ct_on(date, 10, 0)),
            "the reopening session after the closure",
        );
        assert_before_floor(
            calendar.is_closed_all_day_on(date, SessionKind::Both),
            "the civil-day question about the closure",
        );
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// The era's four early closes are 12:45 CT, all on days that opened the
/// previous evening at 17:00 CT, and each deletes the rest of the ordinary
/// 17:00-16:00 CT grid up to the next 17:00 open.
///
/// The rows' payloads are fenced as before; the grid consequences are refused,
/// because the era precedes the 2025 floor. The plain-Friday tail of the test
/// is refused the same way.
#[test]
fn era_2022_2024_early_closes_are_twelve_forty_five_central() {
    let calendar = crypto();

    for date in [
        (2022, 11, 25),
        (2023, 11, 24),
        (2024, 11, 29),
        (2024, 12, 24),
    ] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: ERA_TWELVE_FORTY_FIVE
            }),
            "{date:?}"
        );
        // The row's printed instant is what survives: the calendar cannot
        // state the 16:00 CT final close that is not reached, the 13:00 CT
        // probe that is past the cut, or the 12:44 CT probe that is before it,
        // because all four 2022-2024 dates precede the 2025 floor.
        for time in [(16, 0), (13, 0), (12, 44)] {
            assert_before_floor(
                calendar.is_open(ct(date.0, date.1, date.2, time.0, time.1)),
                "a pre-floor early-close probe",
            );
        }
    }

    // The era's weekend roll on a plain Friday — the 16:00 CT close ending the
    // trade date, the Friday/Saturday/Sunday shutdown, and Sunday 17:00 CT
    // opening Monday — is refused with the rest of the era: every instant here
    // is a pre-floor date.
    for (probe, what) in [
        (
            ct(2022, 4, 14, 15, 59),
            "the last second of Thursday's session",
        ),
        (ct(2022, 4, 14, 16, 0), "Thursday's final close"),
        (ct(2022, 4, 15, 17, 0), "Good Friday's evening"),
        (ct(2022, 4, 16, 12, 0), "Saturday"),
        (ct(2022, 4, 17, 12, 0), "Sunday midday"),
        (ct(2022, 4, 17, 17, 0), "Sunday's reopening leg"),
    ] {
        assert_before_floor(calendar.is_open(probe), what);
    }
    assert_before_floor(
        calendar.trade_date(ct(2022, 4, 14, 15, 59)),
        "Thursday's trade date",
    );
    assert_before_floor(
        calendar.trade_date(ct(2022, 4, 17, 17, 0)),
        "the trade date Sunday's reopening leg carries",
    );
}

/// Asserts what an `Unsourced` row still states and what it can no longer
/// state.
///
/// The row itself is unchanged: the date ships as `Unsourced`, at its recorded
/// tier, so the table expressly withholds a scheduling claim. The observable
/// neutrality this helper used to fence — "every query answers exactly as the
/// detached calendar does" — is gone, because the date precedes the 2025 floor
/// and every date-aware query about it is refused, on the identity and on the
/// detached snapshot alike.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = crypto();
    let bare = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    assert_before_floor(
        calendar.is_closed_trade_date(date, SessionKind::Both),
        "an `Unsourced` trade date is withheld below the floor",
    );
    // Every query but one reports the floor on every probe. The trade-date
    // question splits by the path that resolves the instant, and the fence
    // states both refusals rather than blurring them:
    //   - an instant the five-day grid places on its own civil date reports
    //     the 2025 floor;
    //   - one that falls outside every session and has to search forward —
    //     every Saturday and Sunday instant, and the evening leg that would
    //     open the next trade date on a Friday — resolves through the family's
    //     Pre-Open phase, which the whole-domain declaration withholds
    //     (#93/#123), so its refusal is `OutsideCoveredRange`.
    let weekend = matches!(date.weekday(), Weekday::Sat | Weekday::Sun);
    for (probe, searches_forward) in [
        (ct_on(day_before(date), 18, 0), weekend),
        (ct_on(date, 9, 0), weekend),
        (ct_on(date, 15, 59), weekend),
        (
            ct_on(date, 18, 0),
            weekend || date.weekday() == Weekday::Fri,
        ),
    ] {
        assert_before_floor(calendar.is_open(probe), "an `Unsourced` date's probe");
        assert_before_floor(
            calendar.session_bounds(probe),
            "an `Unsourced` date's session bounds",
        );
        assert_before_floor(
            calendar.next_session_open_after(probe),
            "an `Unsourced` date's next session",
        );
        assert_before_floor(
            calendar.candle_end(probe, CalendarResolution::Daily),
            "an `Unsourced` date's daily candle",
        );
        if searches_forward {
            assert_before_floor(
                calendar.trade_date(probe),
                "an `Unsourced` date resolved across the weekend",
            );
        } else {
            assert_before_floor(
                calendar.trade_date(probe),
                "an `Unsourced` date resolved on the grid",
            );
        }
        assert_before_floor(
            bare.is_open(probe),
            "detaching the table does not lift the floor",
        );
    }
}

#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = crypto();
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date.0, date.1, date.2);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
    }
}

/// The 2022-2024 span is the second of the three declared windows, and the
/// pre-2019 interval below it stays unaudited. (The 2019-2021 span this test
/// used to fence became a window of its own when that wave shipped; the
/// section below fences it.)
///
/// The unaudited 2018 dates used to answer from the pure normal week; they
/// precede the 2025 floor, so the identity and the detached snapshot both
/// refuse them and the equality that once held is replaced by the shared
/// refusal.
#[test]
fn era_2022_2024_window_sits_second_and_the_pre_2019_interval_is_unaudited() {
    let calendar = crypto();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(coverage.contains(day(2021, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    for date in [(2018, 1, 1), (2018, 12, 25)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    // Christmas 2018 is a real CME closure no wave audited. Both calendars
    // used to answer it from the pure normal week; the date precedes the 2025
    // floor, so both now refuse it, and the equality that once held between
    // them is replaced by that shared refusal.
    for probe in [ct(2018, 12, 25, 9, 0), ct(2018, 12, 24, 18, 0)] {
        assert_before_floor(calendar.is_open(probe), "a pre-floor unaudited date");
        assert_before_floor(
            bare.is_open(probe),
            "detaching the table does not lift the floor",
        );
    }
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows, all inside the five-day era.
// ---------------------------------------------------------------------------

/// 12:00 CT, the Monday and Thursday holiday close this era prints.
const ERA_NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Independence Day eve and Christmas Eve close, and the
/// Thanksgiving Friday close of 2019 and 2020, that this era prints.
const ERA_QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;
/// 12:45 CT, the Thanksgiving Friday close of **2021-11-26**: the one date CME
/// moves this family half an hour later than the rest of the complex's 12:15 CT.
/// The wave-3 constant of the same name and value is the 2022-2024 era's own.
const ERA_TWELVE_FORTY_FIVE_2021: u32 = 12 * 3_600 + 45 * 60;
/// 08:15 CT, the one Good Friday 2021 close this era prints.
const ERA_EIGHT_FIFTEEN: u32 = 8 * 3_600 + 15 * 60;

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. Both sides
/// of an early close's instant, and the trade date and opening leg of a
/// closure, are still derived from the row and fenced — but the era precedes
/// the 2025 floor, so what they assert is the refusal, except for the trade
/// date, which the withheld Pre-Open phase refuses as `OutsideCoveredRange`.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = crypto();
    let (mut noons, mut quarters, mut eight_fifteens, mut twelve_forty_fives) = (0_usize, 0, 0, 0);
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
                        ERA_NOON => noons += 1,
                        ERA_QUARTER_PAST_NOON => quarters += 1,
                        ERA_EIGHT_FIFTEEN => eight_fifteens += 1,
                        ERA_TWELVE_FORTY_FIVE_2021 => twelve_forty_fives += 1,
                        other => {
                            panic!("{date}: this family ships no {other}-second CT close here")
                        }
                    }
                    let cutoff = ct_on(date, close_ssm / 3_600, (close_ssm % 3_600) / 60);
                    // The row's own payload survives — the kind, the printed CT
                    // instant and the tier are the table's claim. Every
                    // date-aware answer it used to state is refused: the era
                    // precedes the 2025 floor, so the clipped wrap and its
                    // trade date, the end-exclusive boundary, the trading day's
                    // bounds and its daily candle are no longer answerable.
                    //
                    // The probe sits just inside the session: on the 08:15 Good
                    // Friday close an ordinary 09:00 CT probe would already be
                    // past the cutoff, which is why this one is derived.
                    let inside = cutoff - Duration::minutes(1);
                    for (probe, what) in [
                        (ct_on(day_before(date), 17, 0), "the wrap's open"),
                        (ct_on(day_before(date), 19, 30), "the wrap two hours in"),
                        (cutoff - Duration::seconds(1), "one second before the close"),
                        (cutoff, "the printed close itself"),
                        (inside, "one minute inside the session"),
                    ] {
                        assert_before_floor(calendar.is_open(probe), what);
                    }
                    assert_before_floor(
                        calendar.trade_date(ct_on(day_before(date), 18, 0)),
                        "the wrap's trade date",
                    );
                    assert_before_floor(
                        calendar.session_bounds(inside),
                        "the trading day's bounds",
                    );
                    assert_before_floor(
                        calendar.candle_end(inside, CalendarResolution::Daily),
                        "the trading day's daily candle",
                    );
                    assert_before_floor(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
                        "the trade date one second before the close",
                    );
                }
                HolidayKind::Closed => {
                    closures += 1;
                    // Every answer this branch used to state about the closure
                    // — the closed trade date, the deleted evening block and
                    // civil day, the absent trade date — is refused below the
                    // 2025 floor.
                    assert_before_floor(
                        calendar.is_closed_trade_date(date, SessionKind::Both),
                        "a pre-floor closure is refused, not reported closed",
                    );
                    for probe in [
                        ct_on(day_before(date), 17, 0),
                        ct_on(day_before(date), 19, 30),
                        ct_on(date, 9, 0),
                        ct_on(date, 15, 59),
                    ] {
                        assert_before_floor(
                            calendar.is_open(probe),
                            "the closed trading day's own instants",
                        );
                    }
                    // The trade date reads the family's Pre-Open phase, which
                    // the whole-domain declaration withholds (#93/#123): that
                    // refusal is `OutsideCoveredRange` even below the floor.
                    assert_before_floor(
                        calendar.trade_date(ct_on(date, 10, 0)),
                        "the closed day's trade date",
                    );
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
        (
            noons,
            quarters,
            eight_fifteens,
            twelve_forty_fives,
            closures,
            unsourced
        ),
        (18, 5, 1, 1, 8, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships: the row states that the date was
/// audited, makes no scheduling claim, and clips nothing.
///
/// Its dates precede the 2025 floor, so the queries that used to demonstrate
/// that neutrality now refuse them instead; `assert_unsourced_changes_nothing`
/// states what survives and why.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = crypto();
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
/// it, while 2018-12-31 and 2022-01-01 lie outside it and belong to the
/// unaudited interval below the table and to the 2022-2024 wave above it.
///
/// The window and its rows are unchanged. Its last day is no longer answered,
/// though: 2021-12-31 precedes the 2025 floor, so the date-aware question takes
/// the explicit refusal rather than a normal-week answer.
#[test]
fn era_2019_2021_window_edges_answer_as_the_module_declares() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_cryptocurrency ships a table");
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
        "2018-12-31 lies below the table's first window"
    );
    assert!(
        !inside(day(2022, 1, 1)),
        "2022-01-01 belongs to the 2022-2024 wave, not to this era"
    );

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is an ordinary Friday this table audited.
    assert_eq!(
        calendar.holiday_on(day(2019, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day(2021, 12, 31)), None);
    // The era's last day is audited normal in the table, but the question is
    // refused: 2021-12-31 precedes the 2025 floor, so "answers as the module
    // declares" now means the explicit error rather than a normal-week answer.
    assert_before_floor(
        calendar.is_open(ct(2021, 12, 31, 9, 0)),
        "the era's last day is below the floor",
    );
    // The neighbouring dates carry no row of this era.
    assert_eq!(calendar.holiday_on(day(2018, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_cryptocurrency ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
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
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 12, 24), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2019_2021_rows_are_the_audited_date_kind_and_tier_set() {
    let calendar = crypto();
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
