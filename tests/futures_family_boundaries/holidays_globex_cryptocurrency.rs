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
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key,
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
        assert!(calendar.is_open(probe), "2025-01-20 matched at {probe}");
        assert_eq!(calendar.trade_date(probe), Some(day(2025, 1, 20)));
    }
    assert!(!calendar.is_open(ct(2025, 1, 20, 16, 0)));
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

    // The window opened at 2022-01-01 when the 2022-2024 wave shipped; its two
    // declared spans are contiguous, so the outer edges are what this fence
    // pins and `windows()` in the era section below pins the split.
    assert_eq!(coverage.first(), day(2022, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2021, 12, 31)));
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

    // Christmas 2021 is a CME closure the crate has not audited. Below the
    // window the answer is the pure normal-week answer, not a guess.
    let bare = calendar.without_holidays();
    for probe in [ct(2021, 12, 25, 9, 0), ct(2021, 12, 25, 18, 0)] {
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
        assert_eq!(
            calendar.trade_date(probe),
            bare.trade_date(probe),
            "{probe}"
        );
    }
    assert_eq!(calendar.holiday_on(day(2021, 12, 25)), None);

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

fn day_after(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The ordinary 17:00 CT evening open that follows a closed trade date: this
/// civil date's own leg when the week has one, otherwise the Sunday evening
/// that opens the next week — the five-day grid closes over the weekend and has
/// no Friday-evening occurrence.
fn era_reopen_after_closure(date: NaiveDate) -> DateTime<Utc> {
    let reopen = if date.weekday() == Weekday::Fri {
        date.checked_add_days(Days::new(2))
            .expect("the era is far from the representable bound")
    } else {
        date
    };
    ct_on(reopen, 17, 0)
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
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
                    assert_eq!(close_ssm, ERA_TWELVE_FORTY_FIVE, "{date}");
                    twelve_forty_five += 1;
                    let cutoff = ct_on(date, 12, 45);
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(calendar.is_open(ct_on(day_before(date), 17, 0)), "{date}");
                    assert!(calendar.is_open(ct_on(day_before(date), 19, 30)), "{date}");
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), 18, 0)),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant.
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, 9, 0)),
                        Some((ct_on(day_before(date), 17, 0), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(ct_on(date, 9, 0), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
                        Some(date),
                        "{date}"
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

/// The five-day weekend roll is intact on every closure: the trade date's whole
/// trading day goes, including the evening block that opened it, and the next
/// session the family offers is the ordinary 17:00 CT evening open — on a
/// Friday closure that is Sunday's leg into the next week's first trade date.
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
        assert!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
        // The evening block that would have carried this trade date is gone,
        // and so is the trade date's own civil day.
        assert!(!calendar.is_open(ct_on(day_before(date), 17, 0)), "{date}");
        assert!(!calendar.is_open(ct_on(day_before(date), 19, 30)), "{date}");
        assert!(!calendar.is_open(ct_on(date, 9, 0)), "{date}");
        assert!(!calendar.is_open(ct_on(date, 15, 59)), "{date}");
        assert_eq!(calendar.trade_date(ct_on(date, 10, 0)), None, "{date}");

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar.next_session_open_after(ct_on(date, 10, 0)),
            Some(reopen),
            "{date}: the next session is the ordinary evening open"
        );
        // A Friday closure takes the whole civil day; a Monday one leaves its
        // own evening leg, which opens the next trade date.
        assert_eq!(
            calendar.is_closed_all_day_on(date, SessionKind::Both),
            date.weekday() == Weekday::Fri,
            "{date}"
        );
        if reopen == ct_on(date, 17, 0) {
            assert_eq!(calendar.trade_date(reopen), Some(day_after(date)), "{date}");
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// The era's four early closes are 12:45 CT, all on days that opened the
/// previous evening at 17:00 CT, and each deletes the rest of the ordinary
/// 17:00-16:00 CT grid up to the next 17:00 open.
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
        // The ordinary 16:00 CT final close is not reached.
        assert!(
            !calendar.is_open(ct(date.0, date.1, date.2, 16, 0)),
            "{date:?}"
        );
        assert!(
            !calendar.is_open(ct(date.0, date.1, date.2, 13, 0)),
            "{date:?}"
        );
        assert!(
            calendar.is_open(ct(date.0, date.1, date.2, 12, 44)),
            "{date:?}"
        );
    }

    // The era's weekend roll on a plain Friday: the 16:00 CT close ends the
    // trade date, no evening block follows, and Sunday 17:00 CT opens Monday.
    assert!(calendar.is_open(ct(2022, 4, 14, 15, 59)));
    assert!(!calendar.is_open(ct(2022, 4, 14, 16, 0)));
    assert_eq!(
        calendar.trade_date(ct(2022, 4, 14, 15, 59)),
        Some(day(2022, 4, 14))
    );
    assert!(!calendar.is_open(ct(2022, 4, 15, 17, 0)));
    assert!(!calendar.is_open(ct(2022, 4, 16, 12, 0)));
    assert!(!calendar.is_open(ct(2022, 4, 17, 12, 0)));
    assert!(calendar.is_open(ct(2022, 4, 17, 17, 0)));
    assert_eq!(
        calendar.trade_date(ct(2022, 4, 17, 17, 0)),
        Some(day(2022, 4, 18))
    );
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday) {
    let calendar = crypto();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), EvidenceTier::T2, "{date}");
    assert!(
        !calendar.is_closed_trade_date(date, SessionKind::Both),
        "{date} must not be reported closed"
    );

    for probe in [
        ct_on(day_before(date), 18, 0),
        ct_on(date, 9, 0),
        ct_on(date, 15, 59),
        ct_on(date, 18, 0),
    ] {
        assert_eq!(calendar.is_open(probe), detached.is_open(probe), "{probe}");
        assert_eq!(
            calendar.trade_date(probe),
            detached.trade_date(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.session_bounds(probe),
            detached.session_bounds(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.next_session_open_after(probe),
            detached.next_session_open_after(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.candle_end(probe, CalendarResolution::Daily),
            detached.candle_end(probe, CalendarResolution::Daily),
            "{probe}"
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
        assert_unsourced_changes_nothing(date, row);
    }
}

/// The 2022-2024 span is the first of the two declared windows and the
/// 2019-2021 interval below it stays unaudited.
#[test]
fn era_2022_2024_window_is_the_first_declared_span() {
    let calendar = crypto();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2021, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    for date in [(2019, 1, 1), (2020, 12, 25), (2021, 7, 5)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    // Christmas 2020 is a real CME closure no wave audited; the ordinary
    // Friday answers, and the detached calendar answers it the same.
    for probe in [ct(2020, 12, 25, 9, 0), ct(2020, 12, 24, 18, 0)] {
        assert!(calendar.is_open(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
    }
}
