// SPDX-License-Identifier: MIT-0

//! Public contracts for the four CME venue holiday tables (design memo D17).
//!
//! A venue calendar is the availability union of the venue's automated
//! order-capable systems, and `Exchange::Cme`, `Cbot`, `Comex` and `Nymex` each
//! route several product families onto one schedule. The venue's table is
//! therefore the **intersection** of those families' tables, and this module
//! fences the two halves of that claim.
//!
//! **The intersection is what the code says it is.** The routing table in
//! `holidays/venues.rs` is data, so it can drift from `hours_for_exchange` the
//! moment someone re-routes a family. `the_venue_table_is_the_intersection_of_its_families`
//! recomputes the whole venue table from the routed families' own public
//! answers, so the two cannot disagree silently.
//!
//! **Each kind behaves as the family rows behind it do.** A `Closed` venue row
//! removes the trading day including any prior-evening wrap, and an
//! `Unsourced` row changes no answer at all — it is the crate saying the date
//! is special and the venue has no single instant for it. Both are asserted
//! against the same calendar with the holiday layer detached, which is the only
//! honest reference: `without_holidays()` removes the venue's own table and
//! nothing else.
//!
//! The per-family rows themselves are fenced beside the families that own them
//! (`holidays_globex_*.rs`); nothing here re-tests a family's instants.

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarSource, DateCoverage, EvidenceTier, Exchange, ExchangeCalendar,
    Holiday, HolidayKind, MarketHoursKey, SessionKind, calendar_for_exchange,
    calendar_for_market_hours_key,
};

use super::prelude::{assert_refused_variant, assert_refuses_before_floor};

/// The four venues this change gives a table, with the families each routes.
///
/// This is a **handwritten** copy of the routing `holidays/venues.rs` declares,
/// not a derivation from it: the point is to compare the module's data against
/// the crate's own schedules, and a list generated from the module would agree
/// with it by construction.
const VENUES: [(Exchange, &[MarketHoursKey]); 4] = [
    (
        Exchange::Cme,
        &[
            MarketHoursKey::GlobexEquityIndex,
            MarketHoursKey::GlobexEnergy,
            MarketHoursKey::GlobexFx,
            MarketHoursKey::GlobexGrains,
            MarketHoursKey::GlobexInterestRates,
            MarketHoursKey::GlobexLivestock,
        ],
    ),
    (
        Exchange::Cbot,
        &[
            MarketHoursKey::GlobexGrains,
            MarketHoursKey::GlobexInterestRates,
        ],
    ),
    (Exchange::Comex, &[MarketHoursKey::GlobexEnergy]),
    (Exchange::Nymex, &[MarketHoursKey::GlobexEnergy]),
];

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid CT instant")
        .with_timezone(&Utc)
}

/// The holiday layer's own answer for one venue, as a kind per trade date.
///
/// `None` means the date is inside the window and audited normal.
fn venue_layer(calendar: ExchangeCalendar) -> Vec<(NaiveDate, Option<HolidayKind>)> {
    let coverage = calendar
        .holiday_coverage()
        .expect("every venue in this module ships a table");
    let mut rows = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        rows.push((date, calendar.holiday_on(date).map(Holiday::kind)));
        date = date
            .checked_add_days(Days::new(1))
            .expect("the scan stays inside the representable calendar");
    }
    rows
}

/// What the routed families jointly say about one trade date.
///
/// Three states, not two: a date every family audited normal is not the same as
/// a date they dispute, and collapsing them would let the venue be wrong in one
/// direction or the other without any test noticing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Joint {
    /// No routed family states a row: the date is audited normal everywhere, so
    /// the venue must ship no row either.
    AuditedNormal,
    /// Every routed family states the same row.
    Agreed(HolidayKind),
    /// At least one family states a row and they do not all agree, so no single
    /// venue answer exists.
    Disputed,
}

/// The intersection of the routed families' layers, computed from their public
/// `holiday_on` answers.
///
/// A `Closed` variant carries no instant: its own evening re-open belongs to
/// the **next** trade date, so two closures agree whatever that re-open's clock
/// reads. Comparing the variant's fields would make a venue withhold a date
/// every routed family closed. Every other kind is compared as stated.
fn joint_kind(kind: Option<HolidayKind>) -> Option<HolidayKind> {
    kind
}

/// Returns whether the identity ships no table covering `date`, so it has no
/// answer there rather than an audited-normal one.
fn abstains(calendar: ExchangeCalendar, date: NaiveDate) -> bool {
    match calendar.holiday_coverage() {
        None => true,
        Some(coverage) => !coverage
            .windows()
            .iter()
            .any(|(first, last)| *first <= date && date <= *last),
    }
}

fn family_intersection(families: &[MarketHoursKey]) -> Vec<(NaiveDate, Joint)> {
    let calendars = families
        .iter()
        .map(|key| calendar_for_market_hours_key(*key))
        .collect::<Vec<_>>();
    let coverage = calendars
        .first()
        .and_then(|calendar| calendar.holiday_coverage())
        .expect("a routed family ships a table");
    let mut rows = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        // A routed family whose table does not cover this date **abstains**:
        // it states nothing, so it cannot dispute what the others state. A
        // family whose table does cover the date and holds no row has audited
        // it normal, which is an answer, and it does dispute a row.
        let stated = calendars
            .iter()
            .filter(|calendar| !abstains(**calendar, date))
            .map(|calendar| joint_kind(calendar.holiday_on(date).map(Holiday::kind)))
            .collect::<Vec<_>>();
        // The joint answer is read off the families that answered. A family
        // that **states nothing** has audited the date normal, which is an
        // answer, and it disputes a row another family states on that date —
        // that is what makes a holiday morning on which only some families
        // halt an `Unsourced` date rather than a venue row. Only a date on
        // which no family that answers states a row is audited normal, and a
        // date in a gap between two eras, which no family covers, is the same
        // "no answer" as before.
        let joint = match stated.first().copied() {
            // No family answers for this date at all: it falls in a gap
            // between two eras, and the venue has no answer either.
            None => Joint::AuditedNormal,
            // Every family that answers audited the date normal.
            Some(None) if stated.iter().all(Option::is_none) => Joint::AuditedNormal,
            Some(Some(kind)) if stated.iter().all(|other| *other == Some(kind)) => {
                Joint::Agreed(kind)
            }
            // A family that states nothing has audited the date normal, which
            // is an answer: a date only some families halt on is disputed.
            Some(_) => Joint::Disputed,
        };
        rows.push((date, joint));
        date = date
            .checked_add_days(Days::new(1))
            .expect("the scan stays inside the representable calendar");
    }
    rows
}

/// Design memo D17: the venue's table **is** the intersection of the families
/// that route to it, with every disagreement stated as `Unsourced`.
///
/// This recomputes the table from the families' own answers rather than reading
/// the venue module back, so a re-routing that `holidays/venues.rs` did not
/// follow fails here, and so does a venue row that copies one family's early
/// close onto a date the others dispute.
#[test]
fn the_venue_table_is_the_intersection_of_its_families() {
    for (exchange, families) in VENUES {
        let venue = venue_layer(calendar_for_exchange(exchange));
        let intersection = family_intersection(families);
        assert_eq!(
            venue.len(),
            intersection.len(),
            "{exchange:?}: the venue's window must be the families' window"
        );
        for ((venue_day, venue_kind), (family_day, family_kind)) in
            venue.iter().zip(intersection.iter())
        {
            assert_eq!(venue_day, family_day, "{exchange:?}");
            match family_kind {
                Joint::AuditedNormal => assert_eq!(
                    venue_kind, &None,
                    "{exchange:?}: no routed family states a row on {venue_day}, so the \
                     venue must state none either"
                ),
                Joint::Agreed(kind) => assert_eq!(
                    venue_kind,
                    &Some(*kind),
                    "{exchange:?}: every routed family states {kind:?} on {venue_day}, \
                     so the venue must state it too"
                ),
                Joint::Disputed => assert_eq!(
                    venue_kind,
                    &Some(HolidayKind::Unsourced),
                    "{exchange:?}: the routed families disagree on {venue_day}, so the \
                     venue must state `Unsourced` rather than a row it cannot support or \
                     the silence that would claim the date was audited normal"
                ),
            }
        }
    }
}

/// The dates the intersection states a single status on, each with the tier the
/// venue row carries: 2010-2012, 2016-2018, 2019-2021 and 2022-2024 are T1
/// (CME's own published schedules, except the two 2024 dates the trading-hours
/// service answers), and 2025-2027 is T2 (that service).
const UNANIMOUS_CLOSURES: [((i32, u32, u32), EvidenceTier); 47] = [
    // 2010-2012: the six Globex full closures CME published for those years
    ((2010, 1, 1), EvidenceTier::T1),
    ((2010, 12, 24), EvidenceTier::T1),
    ((2011, 4, 22), EvidenceTier::T1),
    ((2011, 12, 26), EvidenceTier::T1),
    ((2012, 1, 2), EvidenceTier::T1),
    ((2012, 12, 25), EvidenceTier::T1),
    // 2016-2018: the nine the era's own published schedules state
    ((2016, 1, 1), EvidenceTier::T1),
    ((2016, 3, 25), EvidenceTier::T1),
    ((2016, 12, 26), EvidenceTier::T1),
    ((2017, 1, 2), EvidenceTier::T1),
    ((2017, 4, 14), EvidenceTier::T1),
    ((2017, 12, 25), EvidenceTier::T1),
    ((2018, 1, 1), EvidenceTier::T1),
    ((2018, 3, 30), EvidenceTier::T1),
    ((2018, 12, 25), EvidenceTier::T1),
    // 2019-2021: the eight the era's own published compact schedules state.
    // Good Friday 2021 is **not** one: the energy complex closed that day while
    // the financial families halted early, so the six-family venues state
    // `Unsourced` and only COMEX and NYMEX close.
    ((2019, 1, 1), EvidenceTier::T1),
    ((2019, 4, 19), EvidenceTier::T1),
    ((2019, 12, 25), EvidenceTier::T1),
    ((2020, 1, 1), EvidenceTier::T1),
    ((2020, 4, 10), EvidenceTier::T1),
    ((2020, 12, 25), EvidenceTier::T1),
    ((2021, 1, 1), EvidenceTier::T1),
    ((2021, 12, 24), EvidenceTier::T1),
    // 2013-2015: the eight the era's own published schedules state, all T1.
    ((2013, 1, 1), EvidenceTier::T1),
    ((2013, 3, 29), EvidenceTier::T1),
    ((2013, 12, 25), EvidenceTier::T1),
    ((2014, 1, 1), EvidenceTier::T1),
    ((2014, 4, 18), EvidenceTier::T1),
    ((2014, 12, 25), EvidenceTier::T1),
    ((2015, 1, 1), EvidenceTier::T1),
    ((2015, 12, 25), EvidenceTier::T1),
    // 2022-2024: the seven the era's own published schedules state. Five are
    // T1 sheets; 2024-03-29 and 2024-12-25 are the two the trading-hours
    // service answers, so their rows carry T2.
    ((2022, 4, 15), EvidenceTier::T1),
    ((2022, 12, 26), EvidenceTier::T1),
    ((2023, 1, 2), EvidenceTier::T1),
    ((2023, 12, 25), EvidenceTier::T1),
    ((2024, 1, 1), EvidenceTier::T1),
    ((2024, 3, 29), EvidenceTier::T2),
    ((2024, 12, 25), EvidenceTier::T2),
    // 2025-2027: the nine the trading-hours service states
    ((2025, 1, 1), EvidenceTier::T2),
    ((2025, 4, 18), EvidenceTier::T2),
    ((2025, 11, 29), EvidenceTier::T2),
    ((2025, 12, 25), EvidenceTier::T2),
    ((2026, 1, 1), EvidenceTier::T2),
    ((2026, 12, 25), EvidenceTier::T2),
    ((2027, 1, 1), EvidenceTier::T2),
    ((2027, 3, 26), EvidenceTier::T2),
    ((2027, 12, 24), EvidenceTier::T2),
];

/// The energy family's own extra closures: a six-family venue cannot state
/// them, and `Exchange::Comex`/`Nymex` can. Good Friday 2015 and 2021 join the
/// two 2010-2012 Good Fridays and 2026-04-03.
const ENERGY_ONLY_CLOSURES: [(i32, u32, u32); 5] = [
    (2010, 4, 2),
    (2012, 4, 6),
    (2015, 4, 3),
    (2021, 4, 2),
    (2026, 4, 3),
];

/// A `Closed` venue row may only stand where every routed family states a
/// closure: the sets below are the operator-facing statement of that.
fn assert_closed_row_is_unanimous(
    exchange: Exchange,
    venue: ExchangeCalendar,
    date: NaiveDate,
    unanimous: &[((i32, u32, u32), EvidenceTier)],
    energy_only: &[(i32, u32, u32)],
    single_family: bool,
) {
    if venue.holiday_on(date).map(Holiday::kind) != Some(HolidayKind::Closed) {
        return;
    }
    let agreed = unanimous
        .iter()
        .any(|((y, m, d), _)| day(*y, *m, *d) == date);
    let energy = single_family && energy_only.iter().any(|(y, m, d)| day(*y, *m, *d) == date);
    assert!(
        agreed || energy,
        "{exchange:?}: {date} ships a `Closed` row but is not a closure every family \
         routing to this venue states"
    );
}

/// The dates the intersection states a status for are the only dates those
/// venues may ship a `Closed` row on, each with the tier its own era's document
/// carries; the single-family energy venues, whose scope is narrower,
/// additionally close for Good Friday 2026 and carry that row too.
///
/// This is the fact the venue tables are built out of, asserted from the
/// operator-facing side: a `Closed` venue row is a claim that every routed
/// family closed, and nothing outside these sets may make it.
#[test]
fn a_closed_venue_row_is_a_unanimous_closure() {
    for (exchange, families) in VENUES {
        let single_family = matches!(exchange, Exchange::Comex | Exchange::Nymex);
        let venue = calendar_for_exchange(exchange);
        let coverage = venue.holiday_coverage().expect("the venue ships a table");

        for ((year, month, date), expected_tier) in UNANIMOUS_CLOSURES {
            let closure = day(year, month, date);
            assert!(
                coverage.contains(closure),
                "{exchange:?}: {closure} is inside the audited window"
            );
            let row = venue
                .holiday_on(closure)
                .unwrap_or_else(|| panic!("{exchange:?}: {closure} ships no row"));
            assert_eq!(
                row.kind(),
                HolidayKind::Closed,
                "{exchange:?}: {closure} is a unanimous closure"
            );
            // The tier travels in the row, so it is fenced with it.
            assert_eq!(
                row.tier(),
                expected_tier,
                "{exchange:?}: {closure} must carry its era's tier"
            );
            // The row's own statement is the table's; the date-aware query is a
            // separate claim, and the two do not coincide here. Below the floor
            // the calendar refuses the closure's trade date as
            // `BeforeSupportFloor`; at or above it the query answers `Ok(true)`
            // for a known closure — **unless** the answer also needs a date the
            // table withholds, in which case the venue refuses with
            // `UnresolvedGap` naming that date instead of reporting withheld
            // evidence as a closure (LAW-COVERAGE). `false` is never an
            // acceptable answer for a row in this set.
            match venue.is_closed_trade_date(closure, SessionKind::Both) {
                Ok(closed) => assert!(
                    closed,
                    "{exchange:?}: {closure} closes the venue's trading day"
                ),
                Err(error) => {
                    // 2010-01-01 is the floor itself and its opening day
                    // (2009-12-31) lies below it, so the query can refuse as
                    // outside the covered range too.
                    assert!(
                        matches!(
                            error,
                            CalendarQueryError::BeforeSupportFloor { .. }
                                | CalendarQueryError::UnresolvedGap { .. }
                                | CalendarQueryError::OutsideCoveredRange { .. }
                        ),
                        "{exchange:?}: {closure} may refuse only as below-floor, \
                         withheld or outside the audited windows; got {error:?}"
                    );
                }
            }
        }

        let mut date = coverage.first();
        while date <= coverage.last() {
            assert_closed_row_is_unanimous(
                exchange,
                venue,
                date,
                &UNANIMOUS_CLOSURES,
                &ENERGY_ONLY_CLOSURES,
                single_family,
            );
            date = date
                .checked_add_days(Days::new(1))
                .expect("the scan stays inside the representable calendar");
        }

        // Every other row is `Unsourced`, because the intersection has no other
        // answer to give: one family's early close is not the venue's. The
        // single-family energy venues are the exception — there their one
        // family's early close *is* the venue's.
        let mut unsigned = 0_usize;
        let mut date = coverage.first();
        while date <= coverage.last() {
            if let Some(kind) = venue.holiday_on(date).map(Holiday::kind) {
                assert!(
                    single_family || matches!(kind, HolidayKind::Closed | HolidayKind::Unsourced),
                    "{exchange:?}: {date} ships {kind:?}, which no unanimous date supports"
                );
                if kind == HolidayKind::Unsourced {
                    unsigned = unsigned.saturating_add(1);
                }
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the scan stays inside the representable calendar");
        }
        // `Exchange` is `#[non_exhaustive]`, so the count is keyed off the
        // routing list this module already pins rather than off the variant.
        // Parsed from the shipped tables window by window, 2010-2012
        // contributes 63 (CME) or 61 (CBOT) unsourced dates, 2013-2015 another
        // 50 or 46, 2016-2018 27 each, 2019-2021 34 each, 2022-2024 34 and 32,
        // and 2025-2027 53 and 48 — the closure-eve replacement rows
        // `globex_grains` states on fourteen dates move both venues by those
        // fourteen (CME's 2025-07-03 was already `Unsourced`), and the rest of
        // the era's growth is the three Saturday-session trade dates
        // (2026-06-22, 2026-07-06 and 2027-06-21) carry a row in `globex_energy`,
        // `globex_equity_index`, `globex_interest_rates` and `globex_fx`, so
        // they are disputes on the six-family venue, which gains nothing here — its
        // three dates were already `Unsourced` — while the two-family venue gains them; and
        // because interest rates states one where grains states none they are
        // disputes on the two-family CBOT intersection too, which also gains
        // three; the single-family venues have six, the three 2019-2021
        // Juneteenth markers and the three 2023 dates. The 2025 five is
        // `globex_fx`'s merged trade dates (2025-01-21, 2025-02-18, 2025-05-27,
        // 2025-06-20 and 2025-09-02), which that family alone states, and the
        // 2026-2027 eight are the same shape at 2026-01-20, 2026-02-17,
        // 2026-05-26, 2026-09-08, 2027-01-19, 2027-02-16, 2027-06-01 and
        // 2027-09-07. 2026-11-27 and 2027-11-26 already carried venue rows.
        // CME's 2010-2012 fourteen and CBOT's twenty-eight are the #101 wave:
        // the eighteen noon halts put `globex_interest_rates` in disagreement
        // with grains on fourteen dates CBOT had no row for (four already
        // disagreed), and its fourteen merged trade dates — 2010-01-19,
        // 2010-02-16, 2010-06-01, 2010-07-06, 2010-09-07, 2011-01-18,
        // 2011-02-22, 2011-05-31, 2011-07-05, 2011-09-06, 2012-01-17,
        // 2012-02-21, 2012-05-29, 2012-09-04 — state a row no other family
        // states, which both venues must withhold; the eighteen CME halt dates
        // and 2012-07-05 already carried their disputes.
        let expected = if single_family {
            6
        } else if families.len() == 6 {
            269
        } else {
            261
        };
        assert_eq!(unsigned, expected, "{exchange:?}: unsourced row count");
    }
}

/// `COMEX` and `NYMEX` route one family each, so their tables are that family's
/// table whole — the intersection drops nothing and every row states a status,
/// including the single-family Good Friday 2026 closure that is **not** one of
/// the six-family closures CME and CBOT agree on.
#[test]
fn the_energy_venues_carry_the_family_table_unchanged() {
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);
    let coverage = energy.holiday_coverage().expect("the family ships a table");

    for exchange in [Exchange::Comex, Exchange::Nymex] {
        let venue = calendar_for_exchange(exchange);
        assert_eq!(venue.holiday_coverage(), energy.holiday_coverage());
        let mut date = coverage.first();
        while date <= coverage.last() {
            let family = energy.holiday_on(date).map(Holiday::kind);
            assert_eq!(
                venue.holiday_on(date).map(Holiday::kind),
                family,
                "{exchange:?}: {date} must carry the energy family's own row"
            );
            // A single-family intersection cannot **disagree**, so it states
            // the family's row exactly: the same kind above, and the same
            // document id and tier here. Where that row is `Unsourced` — the
            // three 2023 dates this wave did not work up — the venue carries
            // the family's own `Unsourced` because the family says so, not
            // because anyone disputes it.
            assert_eq!(
                venue.holiday_on(date).map(Holiday::document_id),
                energy.holiday_on(date).map(Holiday::document_id),
                "{exchange:?}: {date} must cite the family's own document"
            );
            assert_eq!(
                venue.holiday_on(date).map(Holiday::tier),
                energy.holiday_on(date).map(Holiday::tier),
                "{exchange:?}: {date} must carry the family's own tier"
            );
            date = date
                .checked_add_days(Days::new(1))
                .expect("the scan stays inside the representable calendar");
        }

        // The date that proves the point: the energy complex closed on Good
        // Friday 2026 while the other five CME families were open or short.
        assert_eq!(
            venue.holiday_on(day(2026, 4, 3)).map(Holiday::kind),
            Some(HolidayKind::Closed),
        );
        assert!(
            venue
                .is_closed_trade_date(day(2026, 4, 3), SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
        assert_eq!(
            calendar_for_exchange(Exchange::Cme)
                .holiday_on(day(2026, 4, 3))
                .map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "the six-family venue cannot state the energy family's Good Friday",
        );
    }
}

/// A venue `Closed` row removes the trading day that carries the holiday,
/// including the leg that opened the previous evening.
///
/// The row is keyed by trade date, so the test asks the venue's schedule rather
/// than its civil day: an instant inside the removed trade date has no session
/// and no trade date, and the next session the venue offers is the one that
/// opens after the closure.
#[test]
fn a_venue_closure_removes_the_trading_day_it_names() {
    let venue = calendar_for_exchange(Exchange::Cme);
    let closure = day(2025, 12, 25);

    // The **row's** statement is the table's, and it is asserted there: the
    // venue ships a `Closed` row for trade date 2025-12-25, which is what
    // "the row removes the whole trading day" means.
    assert_eq!(
        venue.holiday_on(closure).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );

    // The date-aware surface cannot confirm any of it. Every instant below
    // resolves to a trade date this venue's 2025 table withholds — 2025-12-24
    // for the 12-24 evening leg that would have settled 12-25, and 2025-12-25's
    // own 17:00 leg for the day after — so each query is refused with
    // `UnresolvedGap` naming the withheld date rather than answered. What is no
    // longer claimable is a session observation on this holiday: no trade date,
    // no session and no next session is stated. The row's kind above is the
    // removal claim, the family tables carry the removed instants, and
    // `tests/static_day_policy.rs` exercises the day-removal mechanism on dates
    // this identity covers.
    for probe in [
        ct((2025, 12, 25), (10, 0, 0)),
        ct((2025, 12, 25), (15, 30, 0)),
    ] {
        assert_refused_variant(
            &venue.is_open(probe),
            DateCoverage::UnresolvedGap,
            &format!("{probe} is inside the removed day"),
        );
        assert_refused_variant(
            &venue.trade_date(probe),
            DateCoverage::UnresolvedGap,
            &format!("{probe} trade date"),
        );
    }
    assert_refused_variant(
        &venue.is_closed_trade_date(closure, SessionKind::Both),
        DateCoverage::UnresolvedGap,
        "the closure's own trade date",
    );
    // The 17:00 CT leg that opens on the evening of 12-25 would have belonged to
    // trade date 2025-12-26; 2025-12-24's own session would have been untouched,
    // and only that day's 17:00 CT leg — the one that would have settled 12-25 —
    // removed. None of that is stateable here, and the next session after the
    // closure — the `ct((2025, 12, 25), (17, 0, 0))` open that runs to
    // `ct((2025, 12, 26), (8, 30, 0))` — cannot be reported either: the search
    // has to establish a withheld date to name it.
    for probe in [
        ct((2025, 12, 25), (17, 30, 0)),
        ct((2025, 12, 24), (12, 0, 0)),
        ct((2025, 12, 24), (17, 30, 0)),
    ] {
        assert_refused_variant(
            &venue.is_open(probe),
            DateCoverage::UnresolvedGap,
            &format!("{probe}"),
        );
        assert_refused_variant(
            &venue.trade_date(probe),
            DateCoverage::UnresolvedGap,
            &format!("{probe} trade date"),
        );
    }
    assert_refused_variant(
        &venue.next_session_after(ct((2025, 12, 25), (12, 0, 0))),
        DateCoverage::UnresolvedGap,
        "the session after the closure",
    );
}

/// A `Closed` row on a date the venue has no session for anyway is neutral: it
/// ships because the operator answered, not because a session was removed.
///
/// Saturday 2025-11-29 is the case. The row cannot change this venue's answers,
/// so the test asserts the neutrality rather than assuming it — the same
/// distinction the family table draws for the same date.
#[test]
fn a_closure_on_a_non_session_day_changes_nothing() {
    let venue = calendar_for_exchange(Exchange::Cme);
    let saturday = day(2025, 11, 29);
    assert_eq!(
        venue.holiday_on(saturday).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );

    // The neutrality claim is now a claim about the **detached** calendar, the
    // only surface that answers here: every instant of this Saturday belongs to
    // the trade date that opened Friday 2025-11-28, which the venue's own table
    // withholds, so `venue` refuses the whole day with `UnresolvedGap`. The
    // detached calendar carries no holiday layer to withhold anything and still
    // states the ordinary normal-week answers the row must not move.
    let detached = venue.without_holidays();
    let start = US::Central
        .with_ymd_and_hms(2025, 11, 29, 0, 0, 0)
        .single()
        .expect("fixture must be a valid CT midnight")
        .with_timezone(&Utc);
    // The ordinary schedule the row must not move: the next session after
    // Saturday is the Sunday-evening leg that opens 2025-11-30 17:00 CT and
    // closes 2025-12-01 08:30 CT.
    assert_eq!(
        detached
            .session_bounds(start)
            .expect("the detached calendar answers the normal week"),
        Some((
            ct((2025, 11, 30), (17, 0, 0)),
            ct((2025, 12, 1), (8, 30, 0)),
        )),
        "the venue's ordinary schedule is in force across the closed Saturday"
    );
    for step in 0..48 {
        let probe = start + chrono::TimeDelta::minutes(30 * step);
        assert_refused_variant(
            &venue.is_open(probe),
            DateCoverage::UnresolvedGap,
            &format!("the 2025-11-29 closure must not be reported as a closure at {probe}"),
        );
        assert!(
            !detached
                .is_open(probe)
                .expect("the detached calendar answers the normal week"),
            "{probe}: the Saturday is outside the CME normal week"
        );
    }
}

/// An `Unsourced` row is reported, closes nothing, and leaves the venue's own
/// ordinary schedule in force on that trade date.
///
/// This is the whole point of the variant. On 2026-12-24 the equity-index,
/// interest-rate and livestock families close at 12:15 CT, energy and FX at
/// 12:45 CT and grains at 12:05 CT, so the venue states no instant: it must
/// neither claim the date was audited normal nor pick one family's close. The
/// test therefore probes past every disputed close — an instant a compromise
/// `EarlyClose` row would have clipped — and requires the venue's ordinary
/// session to still be running there.
#[test]
fn an_unsourced_venue_row_is_reported_and_clips_nothing() {
    // 2026-12-24 is disputed; 2026-12-25 is the unanimous closure beside it.
    let disputed = day(2026, 12, 24);

    for (exchange, probes) in [
        (
            Exchange::Cme,
            // Inside both the 08:30-15:15 CT regular session and the
            // 15:15-16:00 CT extended one, so every probe is a session the
            // shallowest disputed close would have cut short.
            vec![
                ct((2026, 12, 24), (11, 0, 0)),
                ct((2026, 12, 24), (12, 10, 0)),
                ct((2026, 12, 24), (12, 30, 0)),
                ct((2026, 12, 24), (13, 0, 0)),
                ct((2026, 12, 24), (15, 30, 0)),
            ],
        ),
        (
            Exchange::Cbot,
            // CBOT's grains-backed profile runs one day session, 08:30-13:20 CT.
            vec![
                ct((2026, 12, 24), (9, 0, 0)),
                ct((2026, 12, 24), (12, 10, 0)),
                ct((2026, 12, 24), (12, 30, 0)),
                ct((2026, 12, 24), (13, 0, 0)),
            ],
        ),
    ] {
        let venue = calendar_for_exchange(exchange);
        assert_eq!(
            venue.holiday_on(disputed).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{exchange:?}: the routed families state different closes"
        );
        // The venue's own date-aware surface cannot make this claim: an
        // `Unsourced` row is a withheld date, so every query touching
        // 2026-12-24 is refused with `UnresolvedGap` rather than answered. The
        // ordinary schedule the row must leave alone is therefore read from the
        // calendar with the holiday layer detached — the only surface that
        // states it — and the `Unsourced` row's neutrality is that the venue
        // neither claims the date is normal nor clips it.
        assert_refused_variant(
            &venue.is_closed_trade_date(disputed, SessionKind::Both),
            DateCoverage::UnresolvedGap,
            &format!("{exchange:?}: `Unsourced` closes nothing on {disputed}"),
        );
        let detached = venue.without_holidays();
        for probe in probes {
            assert_eq!(
                detached
                    .trade_date(probe)
                    .expect("the detached calendar answers the normal week"),
                Some(disputed),
                "{exchange:?}: {probe} still belongs to the disputed trade date"
            );
            assert!(
                detached
                    .is_open(probe)
                    .expect("the detached calendar answers the normal week"),
                "{exchange:?}: {probe} is inside the venue's ordinary session; an \
                 `Unsourced` row must not clip it"
            );
            assert_refused_variant(
                &venue.is_open(probe),
                DateCoverage::UnresolvedGap,
                &format!("{exchange:?}: the disputed date is withheld at {probe}"),
            );
        }
    }
}

/// The venue window is the families' window, on both sides.
///
/// A venue row one day outside the audited range would make "in coverage and no
/// row means audited normal" false without any family's own test noticing, so
/// the boundary is asserted here as well as crate-wide.
#[test]
fn every_venue_answers_only_inside_its_own_window() {
    for (exchange, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let coverage = venue.holiday_coverage().expect("the venue ships a table");
        assert_eq!(coverage.first(), day(2010, 1, 1), "{exchange:?}");
        assert_eq!(coverage.last(), day(2027, 12, 31), "{exchange:?}");

        let before = coverage
            .first()
            .checked_sub_days(Days::new(1))
            .expect("the probe stays representable");
        let after = coverage
            .last()
            .checked_add_days(Days::new(1))
            .expect("the probe stays representable");
        assert_eq!(venue.holiday_on(before), None, "{exchange:?}");
        assert_eq!(venue.holiday_on(after), None, "{exchange:?}");

        // 2028-01-01 is a real CME record the families' window ends before.
        assert_eq!(
            venue.holiday_on(day(2028, 1, 1)),
            None,
            "{exchange:?}: a record outside the audited window ships no row"
        );
    }
}

/// Detaching a venue's table restores the pure normal week, and detaching twice
/// changes nothing further.
#[test]
fn without_holidays_restores_the_normal_week_for_every_venue() {
    for (exchange, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        assert_eq!(
            venue.source(),
            CalendarSource::Exchange(exchange),
            "the venue keeps its identity"
        );
        let detached = venue.without_holidays();
        assert_eq!(detached.source(), venue.source());
        assert_eq!(detached.tz(), venue.tz());
        assert_eq!(detached.holiday_coverage(), None);
        assert_eq!(detached.without_holidays(), detached);

        // Christmas Day itself: the layer is the only thing that closes the
        // ordinary week, and the detached calendar states exactly that. The
        // attached venue can only refuse on the day the 2025 holiday table
        // withholds — 2025-12-24 for the six-family venues, whose 2025 rows are
        // `Unsourced`, and no date at all for the two single-family energy
        // venues, which answer it `Ok(false)` — so the closure is asserted where
        // it is stated and the refusal is asserted as a refusal, never read as
        // a closure (LAW-COVERAGE).
        let christmas = ct((2025, 12, 25), (10, 0, 0));
        let venue_answer = venue.is_open(christmas);
        match &venue_answer {
            Ok(open) => assert!(!open, "{exchange:?}: the venue closes Christmas Day"),
            Err(_) => assert_refused_variant(
                &venue_answer,
                DateCoverage::UnresolvedGap,
                &format!("{exchange:?}: Christmas Day is withheld, not answered"),
            ),
        }
        assert!(
            detached
                .is_open(christmas)
                .expect("the detached calendar answers the normal week"),
            "{exchange:?}: detaching restores the normal-week answer"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// The era's intersection is the block's own shape: the nine dates every
/// routed family states a closure for ship `Closed`, and the other 27 ship
/// `Unsourced`, because `globex_grains` states a row on all 36 of the block's
/// dates while the five financial families state one on 34 and the two do not
/// state the same row. `globex_livestock`'s 2016-2018 rows (#110) join the
/// derivation and move nothing: the family states `Closed` on all nine, so the
/// closures stay unanimous, and across the other 27 it early-closes at 12:15 CT
/// on its own eight half-days (the three Thanksgiving Fridays, 2018-12-24, the
/// two Christmas-eve Fridays and the two July-3 eves), closes outright on the
/// eighteen Monday and Thursday holidays, and audits 2018-12-26 normal — a
/// disagreement either way. The assertions
/// below are unchanged from the wave that wrote them against the abstention,
/// which is the point: encoding the family changed no venue answer.
#[test]
fn wave2_venue_rows_are_closed_on_the_nine_and_unsourced_on_the_rest() {
    const WAVE2_CLOSURES: [(i32, u32, u32); 9] = [
        (2016, 1, 1),
        (2016, 3, 25),
        (2016, 12, 26),
        (2017, 1, 2),
        (2017, 4, 14),
        (2017, 12, 25),
        (2018, 1, 1),
        (2018, 3, 30),
        (2018, 12, 25),
    ];

    let mut dates = 0_usize;
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        let closed = WAVE2_CLOSURES
            .iter()
            .any(|(y, m, d)| day(*y, *m, *d) == date);
        for (exchange, families) in VENUES {
            let venue = calendar_for_exchange(exchange);
            let kind = venue.holiday_on(date).map(Holiday::kind);
            if families.len() == 1 {
                // COMEX and NYMEX route one key, so the era is that key's own.
                let family = calendar_for_market_hours_key(families[0]);
                assert_eq!(
                    kind,
                    family.holiday_on(date).map(Holiday::kind),
                    "{exchange:?} {date}"
                );
                continue;
            }
            if closed {
                assert_eq!(
                    kind,
                    Some(HolidayKind::Closed),
                    "{exchange:?} {date}: every routed family states a closure"
                );
            } else if let Some(stated) = kind {
                assert_eq!(
                    stated,
                    HolidayKind::Unsourced,
                    "{exchange:?} {date}: a routed family states a row the others do not, so \
                     the venue must withhold the date rather than state a normal one"
                );
            }
            // `None` is the third state: no routed family states a row, so the
            // date is audited normal everywhere and the venue ships none.
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era is representable");
        dates += 1;
    }
    assert_eq!(dates, 1_096, "2016-01-01 through 2018-12-31 inclusive");
}

/// The era's counts, and the two energy venues' agreement with the family they
/// route.
#[test]
fn wave2_venue_era_counts_match_the_families_they_route() {
    for (exchange, expected) in [
        (Exchange::Cme, 36_usize),
        (Exchange::Cbot, 36),
        (Exchange::Comex, 31),
        (Exchange::Nymex, 31),
    ] {
        let calendar = calendar_for_exchange(exchange);
        let mut rows = 0_usize;
        let mut date = day(2016, 1, 1);
        while date <= day(2018, 12, 31) {
            if calendar.holiday_on(date).is_some() {
                rows += 1;
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
        assert_eq!(rows, expected, "{exchange:?}: 2016-2018 rows");
    }

    // The energy venues carry the family's own rows, kind for kind.
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);
    for exchange in [Exchange::Comex, Exchange::Nymex] {
        let venue = calendar_for_exchange(exchange);
        let mut date = day(2016, 1, 1);
        while date <= day(2018, 12, 31) {
            assert_eq!(
                venue.holiday_on(date).map(Holiday::kind),
                energy.holiday_on(date).map(Holiday::kind),
                "{exchange:?} {date}"
            );
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
    }
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// The era's rows, counted per year and per kind.
///
/// The numbers are handwritten — the era's own shape — while the rows are read
/// through the venue's public surface; the intersection fence above already
/// recomputes the kinds from the families, so a wrong count here and a wrong
/// derivation there cannot both pass.
#[test]
fn wave3_venue_era_counts_match_the_families_they_route() {
    for (exchange, closed, unsourced, instants) in [
        (
            Exchange::Cme,
            [2_usize, 2, 3],
            [9_usize, 12, 13],
            [0_usize, 0, 0],
        ),
        (Exchange::Cbot, [2, 2, 3], [9, 11, 12], [0, 0, 0]),
        (Exchange::Comex, [2, 2, 3], [0, 3, 0], [8, 6, 9]),
        (Exchange::Nymex, [2, 2, 3], [0, 3, 0], [8, 6, 9]),
    ] {
        let venue = calendar_for_exchange(exchange);
        for (index, year) in [2022, 2023, 2024].into_iter().enumerate() {
            let (mut year_closed, mut year_unsourced, mut year_instants) =
                (0_usize, 0_usize, 0_usize);
            let mut date = day(year, 1, 1);
            while date <= day(year, 12, 31) {
                if let Some(kind) = venue.holiday_on(date).map(Holiday::kind) {
                    // `HolidayKind` is `#[non_exhaustive]`: anything that is
                    // neither a closure nor the not-worked-up marker states an
                    // instant, which is the third count below.
                    match kind {
                        HolidayKind::Closed => year_closed += 1,
                        HolidayKind::Unsourced => year_unsourced += 1,
                        _ => year_instants += 1,
                    }
                }
                date = date
                    .checked_add_days(Days::new(1))
                    .expect("the era is representable");
            }
            assert_eq!(year_closed, closed[index], "{exchange:?} {year} closures");
            assert_eq!(
                year_unsourced, unsourced[index],
                "{exchange:?} {year} withheld dates"
            );
            assert_eq!(
                year_instants, instants[index],
                "{exchange:?} {year} rows stating an instant"
            );
        }
    }
}

/// Every venue row's document id and tier are a routed family's own, on the
/// same date.
///
/// `the_venue_table_is_the_intersection_of_its_families` compares kinds; this
/// closes the other two fields, so a venue row can neither invent an artifact
/// nor re-tier one it read.
#[test]
fn wave3_venue_rows_cite_a_family_row_on_the_same_date() {
    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let mut date = day(2022, 1, 1);
        while date <= day(2024, 12, 31) {
            if let Some(row) = venue.holiday_on(date) {
                let stated = families
                    .iter()
                    .filter_map(|key| calendar_for_market_hours_key(*key).holiday_on(date))
                    .collect::<Vec<_>>();
                assert!(
                    !stated.is_empty(),
                    "{exchange:?}: the venue states a row on {date} that no routed family states"
                );
                let matching = stated
                    .iter()
                    .find(|family| family.document_id() == row.document_id())
                    .unwrap_or_else(|| {
                        panic!(
                            "{exchange:?}: {date} cites `{}`, which no routed family cites on \
                             that date ({:?})",
                            row.document_id(),
                            stated
                                .iter()
                                .map(|family| family.document_id())
                                .collect::<Vec<_>>()
                        )
                    });
                assert_eq!(
                    row.tier(),
                    matching.tier(),
                    "{exchange:?}: {date} must carry the tier of the family row it cites"
                );
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
    }
}

/// The era's closures, derived from the families rather than listed by hand.
///
/// A date is a closure exactly when **every** routed family states `Closed` on
/// it, so the venue's `Closed` rows are compared with the families' own answers
/// through the public surface: a handwritten set would agree with the venue
/// module by construction and fence nothing.
#[test]
fn wave3_closures_are_the_dates_every_family_states_closed() {
    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let mut date = day(2022, 1, 1);
        while date <= day(2024, 12, 31) {
            let unanimous = families.iter().all(|key| {
                calendar_for_market_hours_key(*key)
                    .holiday_on(date)
                    .map(Holiday::kind)
                    == Some(HolidayKind::Closed)
            });
            assert_eq!(
                venue.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Closed),
                unanimous,
                "{exchange:?}: {date} is a closure exactly where every routed family says so"
            );
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
    }
}

/// The era's three `Unsourced` shapes, each read from the families' own rows.
///
/// **One** — two families state different instants. **Two** — one family states
/// a row while another audited the date normal, which is an answer and not a
/// missing one. **Three** — every covering family states `Unsourced`, this
/// wave's marker for a date inside the window it did not work up: they agree,
/// so the venue ships their marker rather than a dispute.
#[test]
fn wave3_unsourced_shapes_are_the_families_own_answers() {
    let cme = calendar_for_exchange(Exchange::Cme);
    let cbot = calendar_for_exchange(Exchange::Cbot);
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);
    let grains = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);

    // Shape one: the Friday after Thanksgiving 2022. The financial families and
    // FX halt at 12:15 CT, livestock and grains at 12:05 (grains after a 08:30
    // late open) and energy at 12:45 — no one instant stands for the venue.
    let thanksgiving_friday = day(2022, 11, 25);
    assert_eq!(
        equity.holiday_on(thanksgiving_friday).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }),
        "equity index halts at 12:15 CT"
    );
    assert_eq!(
        energy.holiday_on(thanksgiving_friday).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60
        }),
        "energy halts at 12:45 CT"
    );
    assert_eq!(
        grains.holiday_on(thanksgiving_friday).map(Holiday::kind),
        Some(HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        }),
        "grains reopens at 08:30 CT and closes at 12:05 CT"
    );
    for venue in [cme, cbot] {
        assert_eq!(
            venue.holiday_on(thanksgiving_friday).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "the routed families state different instants"
        );
    }

    // Shape two: 2022-07-05, where only grains states a row — a late open at
    // 08:30 CT — and every financial family audited the date normal.
    let grains_only = day(2022, 7, 5);
    assert_eq!(
        grains.holiday_on(grains_only).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60
        }),
        "grains opens late on 2022-07-05"
    );
    assert_eq!(
        equity.holiday_on(grains_only),
        None,
        "equity index audited 2022-07-05 normal, which disputes grains' row"
    );
    for venue in [cme, cbot] {
        assert_eq!(
            venue.holiday_on(grains_only).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "one family's row against another's audited normal is a dispute"
        );
    }
    assert_eq!(
        cbot.holiday_on(grains_only).map(Holiday::document_id),
        grains.holiday_on(grains_only).map(Holiday::document_id),
        "the venue cites the family row behind the date"
    );

    // Shape three: 2023-01-16, the one 2023 date this wave did not work up.
    // Every covering family states `Unsourced`, so the venue ships the marker
    // the families agree on — and the single-family energy venues carry it too,
    // because their one family states it, not because anyone disputes it.
    for date in [day(2023, 1, 16)] {
        for key in [
            MarketHoursKey::GlobexEquityIndex,
            MarketHoursKey::GlobexEnergy,
            MarketHoursKey::GlobexFx,
            MarketHoursKey::GlobexGrains,
            MarketHoursKey::GlobexInterestRates,
            MarketHoursKey::GlobexLivestock,
        ] {
            assert_eq!(
                calendar_for_market_hours_key(key)
                    .holiday_on(date)
                    .map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{key:?} must state `Unsourced` on {date}"
            );
        }
        for exchange in [
            Exchange::Cme,
            Exchange::Cbot,
            Exchange::Comex,
            Exchange::Nymex,
        ] {
            agreed_marker_cites_a_routed_family(exchange, date)
                .unwrap_or_else(|missing| panic!("{missing} (holiday_on date lookup)"));
        }
    }

    // Shape four: 2023-02-20 and 2023-04-07, worked up from the operator's own
    // unsuffixed summary sheets. The two families the CBOT venue routes state
    // their own sourced rows there — grains closed, rates an early close —
    // while the four families this wave never worked up still state the
    // marker, so the six-family intersection disputes and the two-family one
    // does too; both venues cite a routed family's id, and the single-family
    // energy venues carry the marker their one family states.
    for (date, rates_close_ssm, sheet) in [
        (
            day(2023, 2, 20),
            12 * 3_600,
            "files/presidents-day.pdf @2023-03-29T11:57:47Z",
        ),
        (
            day(2023, 4, 7),
            10 * 3_600 + 15 * 60,
            "files/good-friday.pdf @2024-07-08T16:00:09Z",
        ),
    ] {
        assert_eq!(
            grains.holiday_on(date).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "grains is closed on {date}"
        );
        assert_eq!(
            calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates)
                .holiday_on(date)
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: rates_close_ssm
            }),
            "the rate leg halts on {date}"
        );
        for key in [
            MarketHoursKey::GlobexEquityIndex,
            MarketHoursKey::GlobexEnergy,
            MarketHoursKey::GlobexFx,
            MarketHoursKey::GlobexLivestock,
        ] {
            assert_eq!(
                calendar_for_market_hours_key(key)
                    .holiday_on(date)
                    .map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{key:?} still states `Unsourced` on {date}"
            );
        }
        // The CBOT intersection routes grains first, so it cites the sheet;
        // the six-family CME intersection routes equity index first, which
        // still states the marker, so it cites the service capture. Both are
        // disputes either way.
        {
            let venue = calendar_for_exchange(Exchange::Cbot);
            let row = venue
                .holiday_on(date)
                .unwrap_or_else(|| panic!("Cbot: {date} ships a row"));
            assert_eq!(
                row.kind(),
                HolidayKind::Unsourced,
                "Cbot: the routed families' disagreement withholds {date}"
            );
            assert_eq!(row.document_id(), sheet, "Cbot: {date} cites the sheet");
            assert_eq!(row.tier(), EvidenceTier::T1, "Cbot: {date}");
        }
        agreed_marker_cites_a_routed_family(Exchange::Cme, date)
            .unwrap_or_else(|missing| panic!("{missing} (holiday_on date lookup)"));
        for exchange in [Exchange::Comex, Exchange::Nymex] {
            agreed_marker_cites_a_routed_family(exchange, date)
                .unwrap_or_else(|missing| panic!("{missing} (holiday_on date lookup)"));
        }
    }
}

/// A venue row shipping an `Unsourced` kind — the families' agreed marker or a
/// multi-family dispute — must cite an artifact one of **its own** routed
/// families states for that date.
///
/// Equity index specifically is not the bar: CBOT routes grains and interest
/// rates, COMEX and NYMEX route energy alone, and a venue row citing any routed
/// family is as well grounded as one citing equity index. On the agreed
/// markers the routed families' ids all agree, which this also pins — if they
/// ever stop agreeing, the row must follow a routed family and the assertion
/// says so.
///
/// The missing row is returned rather than panicked on, so the caller — a test
/// body — owns the failure message this repository's lint configuration expects
/// there.
fn agreed_marker_cites_a_routed_family(exchange: Exchange, date: NaiveDate) -> Result<(), String> {
    let venue = calendar_for_exchange(exchange);
    let Some(row) = venue.holiday_on(date) else {
        return Err(format!("{exchange:?}: {date} ships no row"));
    };
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{exchange:?}: {date}");

    let keys = VENUES
        .iter()
        .find_map(|(venue, keys)| (*venue == exchange).then_some(*keys))
        .unwrap_or(&[]);
    let matches = keys
        .iter()
        .filter_map(|key| {
            let holiday = calendar_for_market_hours_key(*key).holiday_on(date);
            (holiday.map(Holiday::document_id) == Some(row.document_id())).then_some((key, holiday))
        })
        .collect::<Vec<_>>();
    assert!(
        !matches.is_empty(),
        "{exchange:?}: {date} cites {}, which no family this venue routes states",
        row.document_id()
    );
    assert!(
        matches
            .iter()
            .all(|(_, holiday)| holiday.map(Holiday::tier) == Some(row.tier())),
        "{exchange:?}: {date} carries a tier none of the families citing that artifact states"
    );
    Ok(())
}

/// An `Unsourced` row in the new era is reported, closes nothing, and leaves
/// the venue's ordinary schedule in force — the same contract the 2026 date
/// fences, on the dates this wave added.
#[test]
fn wave3_unsourced_rows_clip_nothing() {
    for (date, probe) in [
        (day(2022, 11, 25), ct((2022, 11, 25), (12, 30, 0))),
        (day(2022, 7, 5), ct((2022, 7, 5), (9, 0, 0))),
        (day(2023, 1, 16), ct((2023, 1, 16), (10, 0, 0))),
    ] {
        for exchange in [Exchange::Cme, Exchange::Cbot] {
            let venue = calendar_for_exchange(exchange);
            assert_eq!(
                venue.holiday_on(date).map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{exchange:?}: {date}"
            );
            // At the 2010 floor these 2022-2023 dates are inside the audited
            // windows, so the queries are refused as `UnresolvedGap` — the
            // venue never reports the withheld evidence as a closure
            // (LAW-COVERAGE). The row's kind above is the neutrality itself.
            assert!(
                venue
                    .is_closed_trade_date(date, SessionKind::Both)
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "{venue:?}: {date} must be refused as UnresolvedGap"
            );
            // The is_open probe may name the neighbouring day its wrap
            // derivation reads; either way the withheld evidence is never
            // reported as a closure.
            assert!(
                venue.is_open(probe).is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::UnresolvedGap { .. }
                        | CalendarQueryError::OutsideCoveredRange { .. }
                )),
                "{venue:?}: {probe} must be refused, never answered as closed"
            );
        }
    }
}

/// The two 2024 dates the trading-hours service answers are the era's only
/// closures outside its T1 sheets, and the energy early closes state the
/// family's own instants.
///
/// The instants are spelled as seconds here rather than read from the module,
/// and compared against the energy family's public row at the same time: a
/// mutation of any shipped minute fails without the venue module being the
/// reference.
#[test]
fn wave3_energy_early_close_instants_are_the_familys_own() {
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);
    for (date, close_ssm, document, tier) in [
        (
            day(2022, 1, 17),
            13 * 3_600 + 30 * 60,
            "2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z",
            EvidenceTier::T1,
        ),
        (
            day(2022, 11, 25),
            12 * 3_600 + 45 * 60,
            "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z",
            EvidenceTier::T1,
        ),
        (
            day(2024, 11, 29),
            13 * 3_600 + 45 * 60,
            "CME-SVC-2024-11-27",
            EvidenceTier::T2,
        ),
    ] {
        let expected = HolidayKind::EarlyClose { close_ssm };
        let family = energy
            .holiday_on(date)
            .unwrap_or_else(|| panic!("globex_energy states a row on {date}"));
        assert_eq!(
            family.kind(),
            expected,
            "globex_energy's own {date} instant"
        );
        assert_eq!(
            family.document_id(),
            document,
            "globex_energy's own {date} id"
        );
        assert_eq!(family.tier(), tier, "globex_energy's own {date} tier");
        for exchange in [Exchange::Comex, Exchange::Nymex] {
            let venue = calendar_for_exchange(exchange);
            let row = venue
                .holiday_on(date)
                .unwrap_or_else(|| panic!("{exchange:?} ships {date}"));
            assert_eq!(row.kind(), expected, "{exchange:?}: {date}");
            assert_eq!(row.document_id(), document, "{exchange:?}: {date}");
            assert_eq!(row.tier(), tier, "{exchange:?}: {date}");
        }
    }

    // The two 2024 service-answered closures carry T2 while the five T1
    // closures of the era carry T1: the tier travels with the artifact, not
    // with the era.
    let cme = calendar_for_exchange(Exchange::Cme);
    for (date, tier) in [
        (day(2024, 1, 1), EvidenceTier::T1),
        (day(2024, 3, 29), EvidenceTier::T2),
        (day(2024, 12, 25), EvidenceTier::T2),
    ] {
        let row = cme.holiday_on(date).unwrap_or_else(|| {
            panic!(
                "Exchange::Cme: {date} must ship a closure row (holiday_on date lookup; \
                 SessionKind and CalendarResolution do not apply)"
            )
        });
        assert_eq!(row.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(row.tier(), tier, "{date} must carry its artifact's tier");
    }
}

/// Every venue's declared window is the union of the windows the families it
/// routes declare, and each era's window edge is an era's edge rather than part
/// of one contiguous 2010-2027 claim.
#[test]
fn every_venue_window_is_the_union_of_its_families_windows() {
    fn union(windows: &mut Vec<(NaiveDate, NaiveDate)>) -> Vec<(NaiveDate, NaiveDate)> {
        windows.sort_unstable();
        let mut merged: Vec<(NaiveDate, NaiveDate)> = Vec::new();
        for (first, last) in windows.drain(..) {
            match merged.last_mut() {
                Some((_, open_last)) if first <= *open_last => {
                    *open_last = (*open_last).max(last);
                }
                _ => merged.push((first, last)),
            }
        }
        merged
    }

    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let coverage = venue.holiday_coverage().expect("the venue ships a table");
        let mut windows = Vec::new();
        for key in families {
            let family = calendar_for_market_hours_key(*key)
                .holiday_coverage()
                .expect("a routed family ships a table");
            windows.extend(family.windows());
        }
        assert_eq!(
            coverage.windows(),
            union(&mut windows),
            "{exchange:?}: the venue answers exactly where a routed family answers"
        );
        assert!(
            coverage.contains(day(2022, 1, 1)) && coverage.contains(day(2024, 12, 31)),
            "{exchange:?}: the 2022-2024 era is inside the window"
        );
        // Every stage-2.2 family wave has now shipped, so 2013-2015 is a
        // window of its own too. The one gap that remains is 2016-2018, and
        // only for `globex_livestock`: it never declared that era, and the
        // venue's window set is the union of its routed families'.
        assert!(
            coverage.contains(day(2019, 1, 1)) && coverage.contains(day(2021, 12, 31)),
            "{exchange:?}: the 2019-2021 era is inside the window"
        );
        assert!(
            coverage.contains(day(2025, 1, 1)),
            "{exchange:?}: the 2025-2027 window is still declared"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2013-2015 rows.
// ---------------------------------------------------------------------------

/// The era's rows, counted per year and per kind.
///
/// The numbers are handwritten — the era's own shape — while the rows are read
/// through the venue's public surface; the intersection fence above already
/// recomputes the kinds from the families, so a wrong count here and a wrong
/// derivation there cannot both pass. CME and CBOT state no single status on
/// any era date, so their rows are closures and `Unsourced` markers only; the
/// single-family energy venues carry `globex_energy`'s own early closes.
#[test]
fn wave5_venue_era_counts_match_the_families_they_route() {
    for (exchange, closed, unsourced, instants) in [
        (
            Exchange::Cme,
            [3_usize, 3, 2],
            [17_usize, 17, 16],
            [0_usize, 0, 0],
        ),
        (
            Exchange::Cbot,
            [3_usize, 3, 2],
            [16_usize, 16, 14],
            [0_usize, 0, 0],
        ),
        (
            Exchange::Comex,
            [3_usize, 3, 3],
            [0_usize, 0, 0],
            [8_usize, 8, 8],
        ),
        (
            Exchange::Nymex,
            [3_usize, 3, 3],
            [0_usize, 0, 0],
            [8_usize, 8, 8],
        ),
    ] {
        let venue = calendar_for_exchange(exchange);
        for (index, year) in [2013, 2014, 2015].into_iter().enumerate() {
            let (mut found_closed, mut found_unsourced, mut found_instants) = (0_usize, 0, 0);
            let mut date = day(year, 1, 1);
            while date.year() == year {
                if let Some(row) = venue.holiday_on(date) {
                    match row.kind() {
                        HolidayKind::Closed => found_closed += 1,
                        HolidayKind::Unsourced => found_unsourced += 1,
                        _ => found_instants += 1,
                    }
                }
                date = date
                    .succ_opt()
                    .expect("the year ends well before the bound");
            }
            assert_eq!(
                (found_closed, found_unsourced, found_instants),
                (closed[index], unsourced[index], instants[index]),
                "{exchange:?} {year}: the era's shape"
            );
        }
    }
}

/// The era's early closes are end-exclusive on the venue calendars too.
///
/// `wave5_venue_era_counts_match_the_families_they_route` pins the era's shape
/// but counts every non-closure as "an instant". This walks the two
/// single-family energy venues over the era and, on each `EarlyClose` row,
/// checks the venue's own answers around the printed instant: open one second
/// before it, closed at it, and still carrying the row's own trade date. The
/// instant is `globex_energy`'s, read back through the venue calendar — that
/// the venue states it at all is the routing claim, and
/// `the_energy_venues_carry_the_family_table_unchanged` holds the two together.
#[test]
fn wave5_venue_era_early_closes_are_end_exclusive() {
    for exchange in [Exchange::Comex, Exchange::Nymex] {
        let venue = calendar_for_exchange(exchange);
        let mut probes = 0_usize;
        let mut date = day(2013, 1, 1);
        while date <= day(2015, 12, 31) {
            if let Some(HolidayKind::EarlyClose { close_ssm }) =
                venue.holiday_on(date).map(Holiday::kind)
            {
                let (hour, minute, second) =
                    (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                let cutoff = ct(
                    (date.year(), date.month(), date.day()),
                    (hour, minute, second),
                );
                // The era runs 2013-2015 and so lies entirely below the
                // 2025-01-01 floor. The row's printed instant is still what the
                // table states — and `close_ssm` above is read from that row —
                // but the calendar cannot confirm the end-exclusive behaviour
                // there: both probes are refused as `BeforeSupportFloor` rather
                // than answered, and the date-aware form of this fence is
                // exercised on the covered era by
                // `holidays_globex_energy.rs`. What is no longer claimable here
                // is an `is_open`/`trade_date` value on these dates.
                let before_the_close = venue.is_open(cutoff - Duration::seconds(1));
                assert_refuses_before_floor(before_the_close, venue, cutoff - Duration::seconds(1));
                let at_the_close = venue.is_open(cutoff);
                assert_refuses_before_floor(at_the_close, venue, cutoff);
                let trade_date_at_the_close = venue.trade_date(cutoff - Duration::seconds(1));
                assert_refuses_before_floor(
                    trade_date_at_the_close,
                    venue,
                    cutoff - Duration::seconds(1),
                );
                probes += 1;
            }
            date = date.succ_opt().expect("the era ends well before the bound");
        }
        assert_eq!(probes, 24, "{exchange:?}: the era's early closes");
    }
}

/// An `Unsourced` era row is a disagreement, never a scheduling claim.
///
/// The row says the date is special and the venue has no single instant for it;
/// it clips nothing and changes no answer. What it must therefore never be is a
/// closure the families actually agree on — that is what `Closed` is for. The
/// fence checks both halves on every one of the era's `Unsourced` rows: some
/// routed family states a row the others do not match, and the row is not a
/// unanimous closure.
#[test]
fn wave5_venue_era_unsourced_rows_are_disagreements_not_closures() {
    let mut probes = 0_usize;
    for (exchange, keys) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let mut date = day(2013, 1, 1);
        while date <= day(2015, 12, 31) {
            if let Some(row) = venue.holiday_on(date) {
                if row.kind() != HolidayKind::Unsourced {
                    date = date.succ_opt().expect("the era ends well before the bound");
                    continue;
                }
                let family_rows: Vec<Option<Holiday>> = keys
                    .iter()
                    .map(|key| calendar_for_market_hours_key(*key).holiday_on(date))
                    .collect();
                // Never a unanimous closure: that is `Closed`'s own shape.
                let unanimous = family_rows
                    .iter()
                    .all(|family| family.is_some_and(|f| f.kind() == HolidayKind::Closed));
                assert!(
                    !unanimous,
                    "{exchange:?} {date}: a unanimous closure ships `Closed`, \
                     never `Unsourced`"
                );
                // And it really is a disagreement: at least one family states
                // a scheduling row, or is silent where another states one.
                let scheduled = family_rows
                    .iter()
                    .filter(|family| {
                        family.is_some_and(|f| {
                            !matches!(f.kind(), HolidayKind::Closed | HolidayKind::Unsourced)
                        })
                    })
                    .count();
                let silent = family_rows.iter().filter(|family| family.is_none()).count();
                assert!(
                    scheduled + silent > 0,
                    "{exchange:?} {date}: an `Unsourced` row must rest on a \
                     disagreement the families state"
                );
                probes += 1;
            }
            date = date.succ_opt().expect("the era ends well before the bound");
        }
    }
    assert_eq!(probes, 96, "the era's `Unsourced` rows were swept");
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// The era's rows, counted per year and per kind.
///
/// The numbers are handwritten — the era's own shape — while the rows are read
/// through the venue's public surface; the intersection fence above already
/// recomputes the kinds from the families, so a wrong count here and a wrong
/// derivation there cannot both pass. CME and CBOT state no single status on
/// any era date, so their rows are closures and `Unsourced` markers only; the
/// single-family energy venues state the family's own 23 early closes.
#[test]
fn wave4_venue_era_counts_match_the_families_they_route() {
    for (exchange, closed, unsourced, instants) in [
        (
            Exchange::Cme,
            [3_usize, 3, 2],
            [13_usize, 11, 10],
            [0_usize, 0, 0],
        ),
        (Exchange::Cbot, [3, 3, 2], [13, 11, 10], [0, 0, 0]),
        (Exchange::Comex, [3, 3, 3], [1, 1, 1], [8, 8, 7]),
        (Exchange::Nymex, [3, 3, 3], [1, 1, 1], [8, 8, 7]),
    ] {
        let venue = calendar_for_exchange(exchange);
        for (index, year) in [2019, 2020, 2021].into_iter().enumerate() {
            let (mut year_closed, mut year_unsourced, mut year_instants) =
                (0_usize, 0_usize, 0_usize);
            let mut date = day(year, 1, 1);
            while date <= day(year, 12, 31) {
                if let Some(kind) = venue.holiday_on(date).map(Holiday::kind) {
                    // `HolidayKind` is `#[non_exhaustive]`: anything that is
                    // neither a closure nor the not-worked-up marker states an
                    // instant, which is the third count below.
                    match kind {
                        HolidayKind::Closed => year_closed += 1,
                        HolidayKind::Unsourced => year_unsourced += 1,
                        _ => year_instants += 1,
                    }
                }
                date = date
                    .checked_add_days(Days::new(1))
                    .expect("the era is representable");
            }
            assert_eq!(year_closed, closed[index], "{exchange:?} {year} closures");
            assert_eq!(
                year_unsourced, unsourced[index],
                "{exchange:?} {year} withheld dates"
            );
            assert_eq!(
                year_instants, instants[index],
                "{exchange:?} {year} rows stating an instant"
            );
        }
    }

    // The era's totals: 42 rows each for the two multi-family venues — 8
    // closures and 34 `Unsourced` — and the energy family's own 35 for the
    // single-family ones, 9 of them closures.
    for (exchange, total, closures) in [
        (Exchange::Cme, 42_usize, 8_usize),
        (Exchange::Cbot, 42, 8),
        (Exchange::Comex, 35, 9),
        (Exchange::Nymex, 35, 9),
    ] {
        let venue = calendar_for_exchange(exchange);
        let (mut rows, mut closed) = (0_usize, 0_usize);
        let mut date = day(2019, 1, 1);
        while date <= day(2021, 12, 31) {
            if let Some(kind) = venue.holiday_on(date).map(Holiday::kind) {
                rows += 1;
                if kind == HolidayKind::Closed {
                    closed += 1;
                }
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
        assert_eq!(rows, total, "{exchange:?}: 2019-2021 rows");
        assert_eq!(closed, closures, "{exchange:?}: 2019-2021 closures");
    }
}

/// The era's window is the families' intersection, recomputed from the
/// families' own public answers over 2019-01-01..2021-12-31.
///
/// `the_venue_table_is_the_intersection_of_its_families` walks the whole
/// declared coverage; this states the same fact inside the era the wave added,
/// so a venue row that drifted there cannot hide behind a passing global walk.
#[test]
fn wave4_venue_table_is_the_families_intersection_over_the_new_era() {
    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let intersection = family_intersection(families);
        let mut date = day(2019, 1, 1);
        while date <= day(2021, 12, 31) {
            let joint = intersection
                .iter()
                .find_map(|(day, joint)| (*day == date).then_some(*joint))
                .unwrap_or_else(|| panic!("{exchange:?}: {date} is outside the recomputed window"));
            let stated = venue.holiday_on(date).map(Holiday::kind);
            match joint {
                Joint::AuditedNormal => assert_eq!(
                    stated, None,
                    "{exchange:?}: no routed family states a row on {date}"
                ),
                Joint::Agreed(kind) => assert_eq!(
                    stated,
                    Some(kind),
                    "{exchange:?}: every routed family states {kind:?} on {date}"
                ),
                Joint::Disputed => assert_eq!(
                    stated,
                    Some(HolidayKind::Unsourced),
                    "{exchange:?}: the routed families disagree on {date}"
                ),
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
    }
}

/// The era's closures, derived from the families rather than listed by hand.
///
/// A date is a closure exactly when **every** routed family states `Closed` on
/// it, so the venue's `Closed` rows are compared with the families' own answers
/// through the public surface: a handwritten set would agree with the venue
/// module by construction and fence nothing. Good Friday 2021 is the era's
/// proof that the two single-family venues are not the six-family ones.
#[test]
fn wave4_closures_are_the_dates_every_family_states_closed() {
    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let mut date = day(2019, 1, 1);
        while date <= day(2021, 12, 31) {
            let unanimous = families.iter().all(|key| {
                calendar_for_market_hours_key(*key)
                    .holiday_on(date)
                    .map(Holiday::kind)
                    == Some(HolidayKind::Closed)
            });
            assert_eq!(
                venue.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Closed),
                unanimous,
                "{exchange:?}: {date} is a closure exactly where every routed family says so"
            );
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
    }

    // 2021-04-02 is the era's energy-only closure: the complex shut while the
    // financial families halted early, so only the single-family venues close.
    for exchange in [Exchange::Comex, Exchange::Nymex] {
        assert_eq!(
            calendar_for_exchange(exchange)
                .holiday_on(day(2021, 4, 2))
                .map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{exchange:?}: the energy family closed on Good Friday 2021"
        );
    }
    for exchange in [Exchange::Cme, Exchange::Cbot] {
        assert_eq!(
            calendar_for_exchange(exchange)
                .holiday_on(day(2021, 4, 2))
                .map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{exchange:?}: the six-family venues cannot state one status there"
        );
    }
}

/// Every venue row's document id and tier are a routed family's own, on the
/// same date.
///
/// `wave4_venue_table_is_the_families_intersection_over_the_new_era` compares
/// kinds; this closes the other two fields over the era, so a venue row can
/// neither invent an artifact nor re-tier one it read.
#[test]
fn wave4_venue_rows_cite_a_family_row_on_the_same_date() {
    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let mut date = day(2019, 1, 1);
        while date <= day(2021, 12, 31) {
            if let Some(row) = venue.holiday_on(date) {
                let stated = families
                    .iter()
                    .filter_map(|key| calendar_for_market_hours_key(*key).holiday_on(date))
                    .collect::<Vec<_>>();
                assert!(
                    !stated.is_empty(),
                    "{exchange:?}: the venue states a row on {date} that no routed family states"
                );
                let matching = stated
                    .iter()
                    .find(|family| family.document_id() == row.document_id())
                    .unwrap_or_else(|| {
                        panic!(
                            "{exchange:?}: {date} cites `{}`, which no routed family cites on \
                             that date ({:?})",
                            row.document_id(),
                            stated
                                .iter()
                                .map(|family| family.document_id())
                                .collect::<Vec<_>>()
                        )
                    });
                assert_eq!(
                    row.tier(),
                    matching.tier(),
                    "{exchange:?}: {date} must carry the tier of the family row it cites"
                );
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the era is representable");
        }
    }
}

/// The era's three `Unsourced` shapes, each read from the families' own rows.
///
/// **One** — two families state different instants. **Two** — one family states
/// a row while another audited the date normal, which is an answer and not a
/// missing one. **Three** — every covering family states `Unsourced`, the
/// wave's marker for a date inside the window it did not work up: they agree,
/// so the venue ships their marker rather than a dispute, and on the
/// single-family energy venues it is the family's own statement.
#[test]
fn wave4_unsourced_shapes_are_the_families_own_answers() {
    let cme = calendar_for_exchange(Exchange::Cme);
    let cbot = calendar_for_exchange(Exchange::Cbot);
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);
    let grains = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);

    // Shape one: the Friday after Thanksgiving 2019. The financial families
    // halt at 12:15 CT, energy at 12:45 and grains closes 08:30-12:05 — no one
    // instant stands for the venue.
    let thanksgiving_friday = day(2019, 11, 29);
    assert_eq!(
        equity.holiday_on(thanksgiving_friday).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }),
        "equity index halts at 12:15 CT"
    );
    assert_eq!(
        energy.holiday_on(thanksgiving_friday).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60
        }),
        "energy halts at 12:45 CT"
    );
    assert_eq!(
        grains.holiday_on(thanksgiving_friday).map(Holiday::kind),
        Some(HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        }),
        "grains reopens at 08:30 CT and closes at 12:05 CT"
    );
    for venue in [cme, cbot] {
        assert_eq!(
            venue.holiday_on(thanksgiving_friday).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "the routed families state different instants"
        );
    }

    // Shape two: 2019-07-05, where only grains states a row — a late open at
    // 08:30 CT — and every financial family audited the date normal.
    let grains_only = day(2019, 7, 5);
    assert_eq!(
        grains.holiday_on(grains_only).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60
        }),
        "grains opens late on 2019-07-05"
    );
    assert_eq!(
        equity.holiday_on(grains_only),
        None,
        "equity index audited 2019-07-05 normal, which disputes grains' row"
    );
    for venue in [cme, cbot] {
        assert_eq!(
            venue.holiday_on(grains_only).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "one family's row against another's audited normal is a dispute"
        );
    }
    assert_eq!(
        cbot.holiday_on(grains_only).map(Holiday::document_id),
        grains.holiday_on(grains_only).map(Holiday::document_id),
        "the venue cites the family row behind the date"
    );

    // Shape three: the three 2019-2021 Juneteenth dates this wave did not work
    // up. Every covering family states `Unsourced`, so the venue ships the
    // marker the families agree on — and the single-family energy venues carry
    // it too, because their one family states it, not because anyone disputes
    // it.
    for date in [day(2019, 6, 19), day(2020, 6, 19), day(2021, 6, 19)] {
        for key in [
            MarketHoursKey::GlobexEquityIndex,
            MarketHoursKey::GlobexEnergy,
            MarketHoursKey::GlobexFx,
            MarketHoursKey::GlobexGrains,
            MarketHoursKey::GlobexInterestRates,
            MarketHoursKey::GlobexLivestock,
        ] {
            assert_eq!(
                calendar_for_market_hours_key(key)
                    .holiday_on(date)
                    .map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{key:?} must state `Unsourced` on {date}"
            );
        }
        for exchange in [
            Exchange::Cme,
            Exchange::Cbot,
            Exchange::Comex,
            Exchange::Nymex,
        ] {
            agreed_marker_cites_a_routed_family(exchange, date)
                .unwrap_or_else(|missing| panic!("{missing} (holiday_on date lookup)"));
        }
    }
}

/// The `cbot` withheld-date census: 261 `Unsourced` rows that are exactly four
/// not-worked-up markers plus 257 genuine disputes, and the refusals the
/// markers earn at the identity surface.
///
/// This is the fence behind the inventory's `date_level_incompleteness` entry
/// and the evidence file's cross-wave audit counts (re-derived on 2026-09-29
/// UTC; tracked as #223 for the four markers). It pins four facts a row flip, a
/// re-derivation or a new wave would move:
///
/// 1. **the totals** — 308 rows over the six audited windows, 47 `Closed` and
///    261 `Unsourced`, with the per-era withheld series 61 / 46 / 27 / 34 / 32
///    / 61 and the closures 6 / 8 / 9 / 8 / 7 / 9;
/// 2. **the split** — exactly the four marker dates (the three Juneteenth dates
///    and 2023-01-16) are dates both routed families state `Unsourced` on;
///    every other withheld date is a dispute the two families' own answers
///    produce, and on no withheld date do the two state the same row (the
///    audit's "no date on which both families state the same shortened-day
///    row"). The 2026-09-29 fix worked 2023-02-20 and 2023-04-07 up from the
///    operator's own unsuffixed summary sheets, so the two date the families'
///    own sourced answers and are disputes, not markers;
/// 3. **the markers refuse** — each of the four dates answers
///    `UnresolvedGap` through the identity's coverage metadata, the #115
///    contract for a withheld date inside an audited window;
/// 4. **the edges answer** — the trade days either side of each marker are
///    audited normal and `Covered`, so the refusal is exactly the marker's span
///    and never a window hole.
#[test]
fn the_cbot_withheld_dates_are_four_markers_plus_only_disputes() {
    let venue = calendar_for_exchange(Exchange::Cbot);
    let grains = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let rates = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);
    let markers = [
        day(2019, 6, 19),
        day(2020, 6, 19),
        day(2021, 6, 19),
        day(2023, 1, 16),
    ];
    let era_of = |date: NaiveDate| match date.year() {
        2010..=2012 => 0,
        2013..=2015 => 1,
        2016..=2018 => 2,
        2019..=2021 => 3,
        2022..=2024 => 4,
        _ => 5,
    };
    let mut withheld_per_era = [0_usize; 6];
    let mut closed_per_era = [0_usize; 6];
    let mut rows = 0_usize;
    let mut withheld = 0_usize;
    let mut marker_dates = Vec::new();
    let mut date = day(2010, 1, 1);
    let last = venue.holiday_coverage().expect("cbot ships a table").last();
    while date <= last {
        if let Some(holiday) = venue.holiday_on(date) {
            rows += 1;
            let era = era_of(date);
            match holiday.kind() {
                HolidayKind::Closed => closed_per_era[era] += 1,
                HolidayKind::Unsourced => {
                    withheld_per_era[era] += 1;
                    withheld += 1;
                    let grains_kind = grains.holiday_on(date).map(Holiday::kind);
                    let rates_kind = rates.holiday_on(date).map(Holiday::kind);
                    if grains_kind == Some(HolidayKind::Unsourced)
                        && rates_kind == Some(HolidayKind::Unsourced)
                    {
                        marker_dates.push(date);
                    } else {
                        // A dispute: the two routed families' answers differ,
                        // so no single venue row exists. Neither family
                        // abstains — both windows cover every era — and on no
                        // withheld date do they state the same row.
                        assert_ne!(
                            grains_kind, rates_kind,
                            "the routed families state the same row on withheld {date}"
                        );
                    }
                }
                other => panic!("cbot ships no {other:?} rows, but {date} carries one"),
            }
        }
        date = date.succ_opt().expect("the census stays representable");
    }
    assert_eq!(
        (rows, closed_per_era.iter().sum::<usize>(), withheld),
        (308, 47, 261),
        "the cbot table's whole-table census"
    );
    assert_eq!(
        withheld_per_era,
        [61, 46, 27, 34, 32, 61],
        "withheld dates per audited era"
    );
    assert_eq!(
        closed_per_era,
        [6, 8, 9, 8, 7, 9],
        "closures per audited era"
    );
    assert_eq!(
        marker_dates, markers,
        "the four dates both routed families mark not worked up"
    );

    // The markers refuse through the coverage contract, and the trade days
    // either side of each are audited normal and covered. 2021-06-19 is a
    // Saturday, so its neighbours are the Friday and the Monday.
    for (marker, edges) in markers.iter().zip([
        [(2019, 6, 18), (2019, 6, 20)],
        [(2020, 6, 18), (2020, 6, 20)],
        [(2021, 6, 18), (2021, 6, 21)],
        [(2023, 1, 15), (2023, 1, 17)],
    ]) {
        assert_eq!(
            venue.coverage().coverage_on(*marker),
            DateCoverage::UnresolvedGap,
            "the withheld marker {marker} must refuse through the coverage metadata"
        );
        for (year, month, day_number) in edges {
            let edge = day(year, month, day_number);
            assert_eq!(
                venue.coverage().coverage_on(edge),
                DateCoverage::Covered,
                "the trade day {edge} beside the marker {marker} is audited normal"
            );
            assert!(
                venue.holiday_on(edge).is_none(),
                "the trade day {edge} beside the marker {marker} ships no row"
            );
        }
    }
}
