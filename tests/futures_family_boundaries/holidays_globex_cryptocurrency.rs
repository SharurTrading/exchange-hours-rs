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

use chrono::{DateTime, Duration, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key,
};

/// The family under test, as a date-aware calendar.
fn crypto() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
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
#[test]
fn a_closed_trade_date_has_no_session_and_no_trade_date() {
    let calendar = crypto();

    assert_eq!(kind_on(day(2025, 12, 25)), Some(HolidayKind::Closed));
    assert!(calendar.is_closed_trade_date(day(2025, 12, 25), SessionKind::Both));

    for probe in [
        ct(2025, 12, 25, 0, 30),
        ct(2025, 12, 25, 9, 0),
        ct(2025, 12, 25, 15, 0),
    ] {
        assert!(
            !calendar.is_open(probe),
            "2025-12-25 must be shut at {probe}"
        );
        assert_eq!(
            calendar.trade_date(probe),
            None,
            "a closed instant carries no trade date"
        );
    }

    // The eve's own 17:00 CT open would carry trade date 2025-12-25, so the
    // closure deletes it rather than leaving an orphaned evening leg.
    assert!(!calendar.is_open(ct(2025, 12, 24, 17, 30)));
}

/// The Monday holidays of the five-day era behave the same way: the block that
/// opened Sunday at 17:00 CT carried the Monday trade date and goes with it.
#[test]
fn a_closed_monday_removes_the_sunday_evening_block() {
    let calendar = crypto();

    assert_eq!(kind_on(day(2025, 1, 20)), Some(HolidayKind::Closed));
    assert!(!calendar.is_open(ct(2025, 1, 19, 18, 0)));
    assert!(!calendar.is_open(ct(2025, 1, 20, 9, 0)));
    assert_eq!(calendar.trade_date(ct(2025, 1, 20, 9, 0)), None);

    // Monday's own 17:00 CT open feeds Tuesday and is untouched.
    assert!(calendar.is_open(ct(2025, 1, 20, 17, 30)));
    assert_eq!(
        calendar.trade_date(ct(2025, 1, 20, 17, 30)),
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

    assert!(calendar.is_open(cutoff - Duration::seconds(1)));
    assert!(!calendar.is_open(cutoff));

    let (open, close) = calendar
        .session_bounds(ct(2025, 7, 4, 9, 0))
        .expect("the shortened day is still a session");
    assert_eq!(open, ct(2025, 7, 3, 17, 0));
    assert_eq!(close, cutoff);
    assert_eq!(
        calendar.candle_end(ct(2025, 7, 4, 9, 0), CalendarResolution::Daily),
        Some(cutoff)
    );

    // Nothing re-opens behind the cutoff on that civil day.
    assert!(!calendar.is_open(ct(2025, 7, 4, 13, 0)));
    assert!(!calendar.is_open(ct(2025, 7, 4, 18, 0)));
}

/// The day after Thanksgiving 2025 stops at 13:45 CT, and its own trade date
/// survives even though the Thursday before it is closed.
#[test]
fn an_early_close_survives_a_closed_neighbour() {
    let calendar = crypto();
    let cutoff = ct(2025, 11, 28, 13, 45);

    assert_eq!(kind_on(day(2025, 11, 27)), Some(HolidayKind::Closed));
    assert_eq!(
        kind_on(day(2025, 11, 28)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600 + 45 * 60
        })
    );

    assert!(calendar.is_open(cutoff - Duration::seconds(1)));
    assert!(!calendar.is_open(cutoff));
    assert_eq!(
        calendar.trade_date(cutoff - Duration::seconds(1)),
        Some(day(2025, 11, 28))
    );
    // Thursday's 17:00 CT open feeds Friday's trade date, so the Thanksgiving
    // closure must not take it.
    assert!(calendar.is_open(ct(2025, 11, 27, 18, 0)));
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
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed | HolidayKind::EarlyClose { .. }
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

    assert!(calendar.is_open(ct(2025, 12, 24, 12, 44)));
    assert!(!calendar.is_open(ct(2025, 12, 24, 12, 45)));
    assert!(!calendar.is_open(ct(2025, 12, 24, 17, 30)));

    let (open, _close) = calendar
        .next_session_after(ct(2025, 12, 24, 12, 20))
        .expect("trading resumes inside the bounded search");
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
        calendar.trade_date(ct(2025, 7, 4, 9, 0)),
        Some(day(2025, 7, 4))
    );
    assert_eq!(
        calendar.trade_date(ct(2025, 12, 23, 18, 0)),
        Some(day(2025, 12, 24))
    );
    assert_eq!(
        calendar.trade_date(ct(2025, 12, 25, 17, 30)),
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
            calendar.is_open(probe),
            "24/7 cryptocurrency must keep trading at {probe}"
        );
        assert_eq!(
            calendar.trade_date(probe),
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
        assert!(calendar.is_closed_trade_date(closed, SessionKind::Both));
        // The charter's own surprise: a full closure is not a closed civil day
        // for this family, because trading never stops.
        assert!(!calendar.is_closed_all_day_on(closed, SessionKind::Both));
    }

    // The Thursday before the closed 2027-12-24 settles its own trade date.
    assert_eq!(
        calendar.trade_date(ct(2027, 12, 23, 9, 0)),
        Some(day(2027, 12, 23))
    );
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer at all, and must not silently extend to a neighbouring year's
/// holidays.
#[test]
fn the_coverage_window_has_two_hard_edges() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a holiday table");

    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2025, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(kind_on(day(2025, 1, 1)), Some(HolidayKind::Closed));
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);

    // Christmas 2024 is a CME closure the crate has not audited. Below the
    // window the answer is the pure normal-week answer, not a guess.
    let bare = calendar.without_holidays();
    for probe in [ct(2024, 12, 25, 9, 0), ct(2024, 12, 25, 18, 0)] {
        assert_eq!(calendar.is_open(probe), bare.is_open(probe));
        assert_eq!(calendar.trade_date(probe), bare.trade_date(probe));
    }

    // One day past the window the table is equally silent.
    for probe in [ct(2028, 1, 1, 9, 0), ct(2028, 1, 1, 20, 0)] {
        assert_eq!(calendar.is_open(probe), bare.is_open(probe));
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
    assert!(!crypto().is_open(ct(2025, 12, 25, 9, 0)));
    assert!(bare.is_open(ct(2025, 12, 25, 9, 0)));
    assert_eq!(
        bare.trade_date(ct(2025, 12, 25, 9, 0)),
        Some(day(2025, 12, 25))
    );

    // Five-day era: the clipped Independence Day runs to its normal 16:00 CT.
    assert!(!crypto().is_open(ct(2025, 7, 4, 13, 0)));
    assert!(bare.is_open(ct(2025, 7, 4, 13, 0)));
    assert_eq!(
        bare.session_bounds(ct(2025, 7, 4, 13, 0))
            .map(|(_open, close)| close),
        Some(ct(2025, 7, 4, 16, 0))
    );

    // 24/7 era: the business-date roll stops rolling.
    assert_eq!(
        crypto().trade_date(ct(2026, 12, 25, 9, 0)),
        Some(day(2026, 12, 28))
    );
    assert_eq!(
        bare.trade_date(ct(2026, 12, 25, 9, 0)),
        Some(day(2026, 12, 25))
    );
}
