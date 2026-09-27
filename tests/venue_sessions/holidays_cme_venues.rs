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

use chrono::{DateTime, Days, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarSource, DateCoverage, EvidenceTier, Exchange, ExchangeCalendar,
    Holiday, HolidayKind, MarketHoursKey, SessionKind, calendar_for_exchange,
    calendar_for_market_hours_key,
};

use super::prelude::assert_refused_variant;

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

fn family_intersection(
    families: &[MarketHoursKey],
    first: NaiveDate,
    last: NaiveDate,
) -> Vec<(NaiveDate, Joint)> {
    let calendars = families
        .iter()
        .map(|key| calendar_for_market_hours_key(*key))
        .collect::<Vec<_>>();
    let mut rows = Vec::new();
    let mut date = first;
    while date <= last {
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
        let calendar = calendar_for_exchange(exchange);
        let venue = venue_layer(calendar);
        // The walk runs over the venue's own retained window: Stage 5 (#117)
        // pruned the venue's pre-floor eras ahead of the routed families, whose
        // tables still declare earlier eras beside it until their own pruning
        // PR lands. Inside this window every routed family answers, so the
        // intersection is well-defined on every date it walks.
        let coverage = calendar
            .holiday_coverage()
            .expect("the venue ships a table");
        let intersection = family_intersection(families, coverage.first(), coverage.last());
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
/// venue row carries: 2025-2027 is T2 throughout (CME's own trading-hours
/// service), and the eras before the 2025 support floor were removed from the
/// venue tables on 2026-09-27, so no earlier closure ships a row to fence.
const UNANIMOUS_CLOSURES: [((i32, u32, u32), EvidenceTier); 9] = [
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

/// The energy family's own extra closure: a six-family venue cannot state it,
/// and `Exchange::Comex`/`Nymex` can. The pre-floor Good Fridays left with the
/// eras they keyed.
const ENERGY_ONLY_CLOSURES: [(i32, u32, u32); 1] = [(2026, 4, 3)];

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
    for (exchange, _families) in VENUES {
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
                    assert!(
                        matches!(
                            error,
                            CalendarQueryError::BeforeSupportFloor { .. }
                                | CalendarQueryError::UnresolvedGap { .. }
                        ),
                        "{exchange:?}: {closure} may refuse only as below-floor or \
                         withheld; got {error:?}"
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
        // Parsed from the shipped tables over the single retained window, both
        // multi-family venues withhold 61 disputed dates: 2025-01-02, 2025-12-26
        // and 2026-01-02 where only `globex_grains` states its whole-day blocks,
        // the fourteen closure eves where it is likewise the only family with a
        // row, 2025-07-03 where equity index and grains both state, the sixteen
        // Monday and Thursday holidays with `globex_fx` silent, the thirteen
        // `globex_fx` merged trade dates, the four holidays whose four financial
        // families close at 12:00 CT against grains and livestock shut, the three
        // Thanksgiving Fridays, the two Christmas Eves, the three Saturday-session
        // trade dates (2026-06-22, 2026-07-06 and 2027-06-21) and 2026-04-03,
        // 2027-07-05 and 2027-07-06. For CBOT (grains ∩ interest rates) the same
        // count decomposes into its own shapes; the single-family energy venues
        // state no `Unsourced` row at all in the retained window.
        let expected = if single_family { 0 } else { 61 };
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
    let family_coverage = energy.holiday_coverage().expect("the family ships a table");

    for exchange in [Exchange::Comex, Exchange::Nymex] {
        let venue = calendar_for_exchange(exchange);
        // Stage 5 (#117) pruned the venue's pre-floor eras first: the venue now
        // declares the single floor-onward window, and the family's own table
        // still declares its earlier eras beside it until its own pruning PR
        // lands. The claim under test is that the venue's window is covered by
        // the family's and that the rows agree on every date the venue answers.
        let coverage = venue.holiday_coverage().expect("the venue ships a table");
        assert_eq!(
            coverage.windows(),
            &[(day(2025, 1, 1), day(2027, 12, 31))][..],
            "{exchange:?}: the retained window is the floor-onward window"
        );
        let (venue_first, venue_last) = (coverage.first(), coverage.last());
        assert!(
            family_coverage
                .windows()
                .iter()
                .any(|(first, last)| *first <= venue_first && venue_last <= *last),
            "{exchange:?}: the family covers the venue's retained window"
        );
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
            // document id and tier here. The retained window holds no
            // `Unsourced` row at all — the eras whose dates the family marked
            // not worked up were below the floor and left with them — but the
            // comparison is written against the family's answer either way.
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
        // Stage 5 (#117) moved the window's open to the permanent 2025 floor.
        assert_eq!(coverage.first(), day(2025, 1, 1), "{exchange:?}");
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

/// The venue's window is the part of its families' shared answer that reaches
/// the floor.
///
/// Stage 5 (#117) prunes owners in separate PRs: the four venue tables dropped
/// their pre-floor eras first, and the routed families' own tables still declare
/// those earlier eras beside them until their own pruning PR lands. What holds
/// already — and is asserted here — is that the venue declares exactly one
/// floor-onward window, that it sits inside every routed family's covered
/// dates, and that the day before the floor ships no row rather than an
/// audited-normal one.
#[test]
fn the_venue_window_is_the_floor_onward_part_of_its_families_answer() {
    for (exchange, families) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let coverage = venue.holiday_coverage().expect("the venue ships a table");
        assert_eq!(
            coverage.windows(),
            &[(day(2025, 1, 1), day(2027, 12, 31))][..],
            "{exchange:?}: one window, from the floor"
        );

        // Every routed family covers every date the venue answers, so the
        // intersection fence above never walks a date a family abstains on.
        for key in families {
            let family = calendar_for_market_hours_key(*key)
                .holiday_coverage()
                .expect("a routed family ships a table");
            let (venue_first, venue_last) = (coverage.first(), coverage.last());
            assert!(
                family
                    .windows()
                    .iter()
                    .any(|(first, last)| *first <= venue_first && venue_last <= *last),
                "{exchange:?}: {key:?} covers the venue's retained window"
            );
        }

        // The day before the floor is outside the window: no row, not silence
        // read as an audited normal.
        assert_eq!(
            venue.holiday_on(day(2024, 12, 31)),
            None,
            "{exchange:?}: 2024-12-31 is below the floor and ships no row"
        );
    }
}

/// A pre-floor holiday request is refused, never answered from removed history.
///
/// Stage 5 (#117) removed the eras before the support floor, so 2024-12-25 — a
/// Globex closure the venue tables used to ship — is now outside every window:
/// `holiday_on` returns `None` (the table has no answer) and the date-aware
/// query returns the explicit `BeforeSupportFloor` error (LAW-COVERAGE), never a
/// closure read from the removed rows.
#[test]
fn pre_floor_closures_refuse_instead_of_answering() {
    let christmas_2024 = day(2024, 12, 25);
    for (exchange, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        assert_eq!(
            venue.holiday_on(christmas_2024),
            None,
            "{exchange:?}: the removed 2024 row ships no holiday_on answer"
        );
        assert!(matches!(
            venue.is_closed_trade_date(christmas_2024, SessionKind::Both),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
    }
}
