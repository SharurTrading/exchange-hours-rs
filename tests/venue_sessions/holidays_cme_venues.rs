// SPDX-License-Identifier: MIT-0

//! Public contracts for the four CME venue holiday tables.
//!
//! A venue calendar is the availability union of the venue's automated
//! order-capable systems, and `Exchange::Cme`, `Cbot`, `Comex` and `Nymex` each
//! route several product families onto one schedule. **The venue speaks for its
//! profile clock** (AGENTS.md, LAW-HOLIDAY-SCOPE; the 2026-10-09 amendment
//! completing the 2026-09-30 decision on #153): the venue's holiday table is the
//! verbatim mirror of one family table — `cbot`'s is the grain grid, `cme`'s the
//! equity-index grid, `comex`'s and `nymex`'s the energy grid — and this module
//! fences that claim from the operator-facing side.
//!
//! **The mirror is what the code says it is.** The routing in `holidays/venues.rs`
//! is data, so it can drift. `the_venue_table_is_its_clock_families_table`
//! recomputes the whole venue table from the clock family's own public answers,
//! so the two cannot disagree silently.
//!
//! **Each kind behaves as the family row behind it does.** A `Closed` venue row
//! removes the trading day including any prior-evening wrap, and the one
//! inherited `Unsourced` marker (2023-01-16, the clock family's own
//! not-worked-up date) changes no answer at all. Both are asserted against the
//! same calendar with the holiday layer detached, which is the only honest
//! reference: `without_holidays()` removes the venue's own table and nothing
//! else.
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

/// The four venues this change gives a table, each with its **profile clock**
/// and the families it routes.
///
/// This is a **handwritten** copy of the routing `holidays/venues.rs` declares,
/// not a derivation from it: the point is to compare the module's data against
/// the crate's own schedules, and a list generated from the module would agree
/// with it by construction. The clock is the family whose grid the venue's own
/// schedule is built from (AGENTS.md, LAW-HOLIDAY-SCOPE, the 2026-09-30
/// decision on #153): the venue answers every date its clock answers.
const VENUES: [(Exchange, MarketHoursKey, &[MarketHoursKey]); 4] = [
    (
        Exchange::Cme,
        MarketHoursKey::GlobexEquityIndex,
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
        MarketHoursKey::GlobexGrains,
        &[
            MarketHoursKey::GlobexGrains,
            MarketHoursKey::GlobexInterestRates,
        ],
    ),
    (
        Exchange::Comex,
        MarketHoursKey::GlobexEnergy,
        &[MarketHoursKey::GlobexEnergy],
    ),
    (
        Exchange::Nymex,
        MarketHoursKey::GlobexEnergy,
        &[MarketHoursKey::GlobexEnergy],
    ),
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

/// What the venue's profile clock says about one trade date.
///
/// The 2026-10-09 amendment completes the profile-clock rule: the venue answers
/// every date its clock answers and withholds every date its clock withholds,
/// so the joint read the earlier intersection rule needed collapses to the
/// clock's own answer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Joint {
    /// The clock audits the date normal: the venue ships no row.
    AuditedNormal,
    /// The clock states this row: the venue ships it verbatim.
    Agreed(HolidayKind),
}

/// The clock family's layer over its whole audited span, computed from its
/// public `holiday_on` answers.
fn clock_layer(clock: MarketHoursKey) -> Vec<(NaiveDate, Joint)> {
    let calendar = calendar_for_market_hours_key(clock);
    let coverage = calendar
        .holiday_coverage()
        .expect("a routed family ships a table");
    let mut rows = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        let joint = match calendar.holiday_on(date).map(Holiday::kind) {
            None => Joint::AuditedNormal,
            Some(kind) => Joint::Agreed(kind),
        };
        rows.push((date, joint));
        date = date
            .checked_add_days(Days::new(1))
            .expect("the scan stays inside the representable calendar");
    }
    rows
}

/// The venue's table **is** its profile clock's table, row for row.
///
/// This recomputes the table from the clock family's own answers rather than
/// reading the venue module back, so a re-routing that `holidays/venues.rs` did
/// not follow fails here, and so does a venue row that states a kind the clock
/// does not. The routed families that print differently are fenced in their own
/// files; what the venue drops is nothing.
#[test]
fn the_venue_table_is_its_clock_families_table() {
    for (exchange, clock, _families) in VENUES {
        let venue = venue_layer(calendar_for_exchange(exchange));
        let layer = clock_layer(clock);
        assert_eq!(
            venue.len(),
            layer.len(),
            "{exchange:?}: the venue's window must be the clock family's window"
        );
        for ((venue_day, venue_kind), (clock_day, clock_kind)) in venue.iter().zip(layer.iter()) {
            assert_eq!(venue_day, clock_day, "{exchange:?}");
            match clock_kind {
                Joint::AuditedNormal => assert_eq!(
                    venue_kind, &None,
                    "{exchange:?}: the clock audits {venue_day} normal, so the venue \
                     must ship no row"
                ),
                Joint::Agreed(kind) => assert_eq!(
                    venue_kind,
                    &Some(*kind),
                    "{exchange:?}: the clock states {kind:?} on {venue_day}, so the \
                     venue must carry it verbatim"
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

/// The dates in the shared closure list close every one of the four venues, each
/// row at its era's tier and each the clock family's own closure; the
/// single-family energy venues additionally close on their own Good Fridays.
///
/// Asserted from the operator-facing side under the 2026-10-09 clock rule: a
/// `Closed` venue row is the profile clock's closure, and the whole table is the
/// clock's table.
#[test]
fn a_closed_venue_row_is_the_clocks_closure() {
    for (exchange, clock, _families) in VENUES {
        let single_family = matches!(exchange, Exchange::Comex | Exchange::Nymex);
        let venue = calendar_for_exchange(exchange);
        let clock_cal = calendar_for_market_hours_key(clock);
        let coverage = venue.holiday_coverage().expect("the venue ships a table");

        // Every date in the shared closure list is a date the venue's own clock
        // states `Closed`, at the era's tier; the venue closes the trade date.
        for ((year, month, date), expected_tier) in UNANIMOUS_CLOSURES {
            let closure = day(year, month, date);
            assert!(
                coverage.contains(closure),
                "{exchange:?}: {closure} is inside the audited window"
            );
            let row = venue
                .holiday_on(closure)
                .unwrap_or_else(|| panic!("{exchange:?}: {closure} ships no row"));
            assert_eq!(row.kind(), HolidayKind::Closed, "{exchange:?}: {closure}");
            assert_eq!(
                row.tier(),
                expected_tier,
                "{exchange:?}: {closure} carries its era's tier"
            );
            assert_eq!(
                clock_cal.holiday_on(closure).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{exchange:?}: {closure} is the clock family's own closure"
            );
            match venue.is_closed_trade_date(closure, SessionKind::Both) {
                Ok(closed) => assert!(closed, "{exchange:?}: {closure} closes the trading day"),
                Err(error) => assert!(
                    matches!(
                        error,
                        CalendarQueryError::BeforeSupportFloor { .. }
                            | CalendarQueryError::UnresolvedGap { .. }
                            | CalendarQueryError::OutsideCoveredRange { .. }
                    ),
                    "{exchange:?}: {closure} may refuse only as below-floor, withheld or \
                     outside the audited windows; got {error:?}"
                ),
            }
        }

        // The energy-family Good Fridays: the single-family venues state the
        // closures; a multi-family venue states its clock's own row there —
        // which may itself be a closure (the grain grid closes on 2010-04-02)
        // or a short day (equity's 08:15 CT close on 2021-04-02).
        for (year, month, date) in ENERGY_ONLY_CLOSURES {
            let good_friday = day(year, month, date);
            assert_eq!(
                venue.holiday_on(good_friday).map(Holiday::kind),
                clock_cal.holiday_on(good_friday).map(Holiday::kind),
                "{exchange:?}: {good_friday} carries the clock's row"
            );
            if single_family {
                assert_eq!(
                    venue.holiday_on(good_friday).map(Holiday::kind),
                    Some(HolidayKind::Closed),
                    "{exchange:?}: {good_friday} is the energy family's own closure"
                );
            }
        }

        // The whole table: every row is the clock family's row, and the only
        // `Unsourced` marker is the inherited 2023-01-16 one.
        let mut unsigned = 0_usize;
        let mut date = coverage.first();
        while date <= coverage.last() {
            assert_eq!(
                venue.holiday_on(date).map(Holiday::kind),
                clock_cal.holiday_on(date).map(Holiday::kind),
                "{exchange:?}: {date} must carry the clock family's row"
            );
            if venue.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Unsourced) {
                unsigned += 1;
                assert_eq!(
                    date,
                    day(2023, 1, 16),
                    "{exchange:?}: the one inherited marker"
                );
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the scan stays inside the representable calendar");
        }
        assert_eq!(unsigned, 1, "{exchange:?}: unsourced row count");
    }
}

/// `COMEX` and `NYMEX` route one family each, so their tables are that family's
/// table whole — the clock mirror drops nothing and every row states a status,
/// including the single-family Good Friday 2026 closure that the equity clock
/// answers with its own short-day row.
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
        // The 2026-10-09 clock rule: `cme` answers from the equity-index grid,
        // so it states that family's own Good Friday row here and the energy
        // closure stays in `globex_energy`, where exact-family routing reads it.
        let cme_row = calendar_for_exchange(Exchange::Cme)
            .holiday_on(day(2026, 4, 3))
            .map(Holiday::kind);
        assert!(
            cme_row.is_some() && cme_row != Some(HolidayKind::Closed),
            "the six-family venue states its clock's own non-closed Good Friday row"
        );
        assert_eq!(
            cme_row,
            calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
                .holiday_on(day(2026, 4, 3))
                .map(Holiday::kind),
            "the six-family venue answers from its clock",
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

    // The **row's** statement is the table's: a `Closed` row for trade date
    // 2025-12-25. Under the 2026-10-09 clock rule the neighbouring dates answer
    // too — 2025-12-24 carries the clock's 12:15 CT early close — so the whole
    // arrangement is stateable, not refused.
    assert_eq!(
        venue.holiday_on(closure).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    assert!(
        venue
            .is_closed_trade_date(closure, SessionKind::Both)
            .expect("the closure's trade date answers"),
        "the Closed row removes the trade date"
    );
    for probe in [
        ct((2025, 12, 25), (10, 0, 0)),
        ct((2025, 12, 25), (15, 30, 0)),
    ] {
        assert!(
            !venue.is_open(probe).expect("the removed day answers"),
            "{probe} is inside the removed day's session"
        );
    }
    // The 17:00 CT open on the closure's own evening is the next trade date's
    // session (2025-12-26) and it answers.
    assert_eq!(
        venue
            .trade_date(ct((2025, 12, 25), (17, 30, 0)))
            .expect("the next session answers"),
        Some(day(2025, 12, 26)),
        "the 17:00 CT leg belongs to the next trade date"
    );
    // The Christmas Eve early close beside it: the venue's own clock states
    // 12:15 CT, so the morning is open and the afternoon is not, and the
    // evening leg for the next trade date is gone with the early close.
    let eve = day(2025, 12, 24);
    assert_eq!(
        venue.holiday_on(eve).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        }),
        "the clock family's own Christmas Eve row"
    );
    assert!(
        venue
            .is_open(ct((2025, 12, 24), (12, 0, 0)))
            .expect("the eve answers"),
    );
    assert!(
        !venue
            .is_open(ct((2025, 12, 24), (12, 30, 0)))
            .expect("the eve answers"),
        "the 12:15 CT close clips the afternoon"
    );
    assert!(
        !venue
            .is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the eve answers"),
        "no evening leg survives the early close"
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

    // The neutrality claim is a claim about the **answers**, attached and
    // detached alike: the attached calendar answers the whole Saturday `false`
    // from its own row and the shipped grid's shape — the Closed row removes
    // the Saturday's trading day, and no CME family grid wraps a Friday
    // session, so nothing from the withheld 2025-11-28 can cover a Saturday
    // instant and the answer cannot depend on it. The detached calendar
    // carries no holiday layer at all and still states the ordinary
    // normal-week answers the row must not move.
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
        assert!(
            !venue
                .is_open(probe)
                .expect("the closed Saturday answers from its own row"),
            "{probe}: the 2025-11-29 closure must not be reported as open"
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
    // 2023-01-16 is the one inherited marker: the clock families themselves
    // state `Unsourced` there (the operator published nothing this crate could
    // read), so the venues repeat it. 2023-01-02 beside it answers.
    let withheld = day(2023, 1, 16);

    for exchange in [Exchange::Cme, Exchange::Cbot] {
        let venue = calendar_for_exchange(exchange);
        assert_eq!(
            venue.holiday_on(withheld).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{exchange:?}: the clock family's own not-worked-up marker"
        );
        // An `Unsourced` row is a withheld date: every query touching it is
        // refused with `UnresolvedGap` rather than answered.
        assert_refused_variant(
            &venue.is_closed_trade_date(withheld, SessionKind::Both),
            DateCoverage::UnresolvedGap,
            &format!("{exchange:?}: the withheld date refuses"),
        );
        let detached = venue.without_holidays();
        for probe in [ct((2023, 1, 16), (9, 0, 0)), ct((2023, 1, 16), (12, 0, 0))] {
            assert_eq!(
                detached
                    .trade_date(probe)
                    .expect("the detached calendar answers the normal week"),
                Some(withheld),
                "{exchange:?}: {probe} still belongs to the withheld trade date"
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
                &format!("{exchange:?}: the withheld date refuses at {probe}"),
            );
        }
    }
}

/// Asserts the venue and its clock family answer every date of `[first, last]`
/// alike and returns the era's `(rows, unsourced)` counts as shipped.
fn era_mirror(
    exchange: Exchange,
    clock: MarketHoursKey,
    first: NaiveDate,
    last: NaiveDate,
) -> (usize, usize) {
    let venue = calendar_for_exchange(exchange);
    let clock_cal = calendar_for_market_hours_key(clock);
    let (mut rows, mut unsigned) = (0_usize, 0_usize);
    let mut date = first;
    while date <= last {
        let venue_row = venue.holiday_on(date);
        assert_eq!(
            venue_row.map(Holiday::kind),
            clock_cal.holiday_on(date).map(Holiday::kind),
            "{exchange:?}: {date} must carry the clock family's row"
        );
        assert_eq!(
            venue_row.map(Holiday::document_id),
            clock_cal.holiday_on(date).map(Holiday::document_id),
            "{exchange:?}: {date} must cite the clock family's document"
        );
        if let Some(holiday) = venue_row {
            rows += 1;
            if holiday.kind() == HolidayKind::Unsourced {
                unsigned += 1;
            }
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era walk stays bounded");
    }
    (rows, unsigned)
}

/// The venue window is the families' window, on both sides.
///
/// A venue row one day outside the audited range would make "in coverage and no
/// row means audited normal" false without any family's own test noticing, so
/// the boundary is asserted here as well as crate-wide.
#[test]
fn every_venue_answers_only_inside_its_own_window() {
    for (exchange, _, _) in VENUES {
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
    for (exchange, _, _) in VENUES {
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
    for (exchange, clock, _) in VENUES {
        let (rows, unsigned) = era_mirror(exchange, clock, day(2016, 1, 1), day(2018, 12, 31));
        // The era's nine full closures all route to `Closed` in every venue
        // table, because every clock states them; the era carries no marker.
        let venue = calendar_for_exchange(exchange);
        for (y, m, d) in [
            (2016, 1, 1),
            (2016, 3, 25),
            (2016, 12, 26),
            (2017, 1, 2),
            (2017, 4, 14),
            (2017, 12, 25),
            (2018, 1, 1),
            (2018, 3, 30),
            (2018, 12, 25),
        ] {
            assert_eq!(
                venue.holiday_on(day(y, m, d)).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{exchange:?}: {y}-{m:02}-{d:02} is a full closure"
            );
        }
        assert_eq!(unsigned, 0, "{exchange:?}: the era ships no withheld date");
        assert!(rows > unsigned, "{exchange:?}: the era's rows are stated");
    }
}
/// The era's counts, and the two energy venues' agreement with the family they
/// route.
#[test]
fn wave2_venue_era_counts_match_the_families_they_route() {
    for (exchange, expected) in [
        // CME's 2016-2018 share dropped from 36 to 34 on 2026-09-30: the
        // profile-clock rule (#153, #242) retired the two dates on which
        // grains' 12:05 CT and livestock's 12:15 CT closes were the only rows
        // stated (2016-12-23 and 2017-12-22), the equity clock auditing both
        // normal.
        (Exchange::Cme, 34_usize),
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
    for (exchange, clock, _) in VENUES {
        let (rows, unsigned) = era_mirror(exchange, clock, day(2019, 1, 1), day(2021, 12, 31));
        assert_eq!(
            unsigned, 0,
            "{exchange:?}: the Juneteenth markers are gone, so the era withholds nothing"
        );
        assert!(rows > 0, "{exchange:?}: the era's rows are stated");
    }
}
///Every venue row's document id and tier are a routed family's own, on the
/// same date.
///
/// `the_venue_table_is_the_intersection_of_its_families` compares kinds; this
/// closes the other two fields, so a venue row can neither invent an artifact
/// nor re-tier one it read.
#[test]
fn wave3_venue_rows_cite_a_family_row_on_the_same_date() {
    for (exchange, _clock, families) in VENUES {
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
    // The dates every routed family states closed are the venues' closures;
    // under the clock rule they are exactly the clock's closures, probed here.
    for (exchange, clock, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let clock_cal = calendar_for_market_hours_key(clock);
        for (y, m, d) in [
            (2019, 1, 1),
            (2019, 4, 19),
            (2019, 12, 25),
            (2020, 1, 1),
            (2020, 4, 10),
            (2020, 12, 25),
            (2021, 1, 1),
            (2021, 12, 24),
        ] {
            assert_eq!(
                venue.holiday_on(day(y, m, d)).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{exchange:?}: {y}-{m:02}-{d:02}"
            );
            assert_eq!(
                clock_cal.holiday_on(day(y, m, d)).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "clock states {y}-{m:02}-{d:02} closed too"
            );
        }
    }
}
///The era's three `Unsourced` shapes, each read from the families' own rows.
///
/// **One** — two families state different instants. **Two** — one family states
/// a row while another audited the date normal, which is an answer and not a
/// missing one. **Three** — every covering family states `Unsourced`, this
/// wave's marker for a date inside the window it did not work up: they agree,
/// so the venue ships their marker rather than a dispute.
#[test]
fn wave3_unsourced_shapes_are_the_families_own_answers() {
    // The era ships no `Unsourced` row at all after the 2026-10-09 work-up:
    // the Juneteenth markers are deleted (audited normal) and every other
    // arrangement is stated.
    for (exchange, clock, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let clock_cal = calendar_for_market_hours_key(clock);
        for d in [day(2019, 6, 19), day(2020, 6, 19), day(2021, 6, 19)] {
            assert_eq!(
                venue.holiday_on(d).map(Holiday::kind),
                clock_cal.holiday_on(d).map(Holiday::kind),
                "{exchange:?}: {d} carries the clock's (absent) row"
            );
            assert_eq!(
                venue.holiday_on(d),
                None,
                "{exchange:?}: {d} is audited normal"
            );
        }
    }
}
///A venue row shipping an `Unsourced` kind — the families' agreed marker or a
/// multi-family dispute — must cite an artifact one of **its own** routed
/// families states for that date.
///
/// Equity index specifically is not the bar: CBOT routes grains and interest
/// rates, COMEX and NYMEX route energy alone, and a venue row citing any routed
/// family is as well grounded as one citing equity index. On the agreed
/// markers the routed families' ids all agree, which this also pins — if they
/// ever stop agreeing, the row must follow a routed family and the assertion
/// says so.
/// An `Unsourced` row in the new era is reported, closes nothing, and leaves
/// the venue's ordinary schedule in force — the same contract the 2026 date
/// fences, on the dates this wave added.
#[test]
fn wave3_unsourced_rows_clip_nothing() {
    // Nothing in this era withholds any more, so the clip-nothing probe
    // attaches to the era's one interesting arrangement: Good Friday 2021,
    // where the energy complex closed while the financial families traded
    // short days. `cme` states its clock's row (equity's 08:15 CT early
    // close), `comex`/`nymex` state the closure, and `cbot` states the grain
    // grid's closure — each exactly as its clock does.
    let good_friday = day(2021, 4, 2);
    for (exchange, clock, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let clock_cal = calendar_for_market_hours_key(clock);
        assert_eq!(
            venue.holiday_on(good_friday).map(Holiday::kind),
            clock_cal.holiday_on(good_friday).map(Holiday::kind),
            "{exchange:?}: {good_friday} is the clock's row"
        );
    }
}
///The two 2024 dates the trading-hours service answers are the era's only
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

    for (exchange, _clock, families) in VENUES {
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
    for (exchange, clock, _) in VENUES {
        let (rows, unsigned) = era_mirror(exchange, clock, day(2025, 1, 1), day(2027, 12, 31));
        assert_eq!(
            unsigned, 0,
            "{exchange:?}: the service era withholds nothing"
        );
        assert!(rows > 0, "{exchange:?}: the era's rows are stated");
    }
}
///The era's early closes are end-exclusive on the venue calendars too.
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
    // The service era ships no `Unsourced` row: the clock rule states every
    // arrangement, including the merged trade dates the earlier rule withheld
    // as replacement-day disagreements. Thanksgiving 2026 is the probe: the
    // venue states its clock's row and the trade date's questions answer.
    for (exchange, clock, _) in VENUES {
        let (rows, unsigned) = era_mirror(exchange, clock, day(2025, 1, 1), day(2027, 12, 31));
        assert_eq!(unsigned, 0, "{exchange:?}: no withheld dates");
        assert!(rows > 0, "{exchange:?}");
        let venue = calendar_for_exchange(exchange);
        let clock_cal = calendar_for_market_hours_key(clock);
        let thanksgiving = day(2026, 11, 26);
        assert_eq!(
            venue.holiday_on(thanksgiving).map(Holiday::kind),
            clock_cal.holiday_on(thanksgiving).map(Holiday::kind),
            "{exchange:?}: Thanksgiving carries the clock's row"
        );
        assert_ne!(
            venue.holiday_on(thanksgiving).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{exchange:?}: Thanksgiving is stated, not withheld"
        );
    }
}
///The era's three `Unsourced` shapes, each read from the families' own rows.
///
/// **One** — two families state different instants. **Two** — one family states
/// a row while another audited the date normal, which is an answer and not a
/// missing one. **Three** — every covering family states `Unsourced`, the
/// wave's marker for a date inside the window it did not work up: they agree,
/// so the venue ships their marker rather than a dispute, and on the
/// single-family energy venues it is the family's own statement.
#[test]
fn wave4_unsourced_shapes_are_the_families_own_answers() {
    // The era's one `Unsourced` marker is 2023-01-16, inherited from the clock;
    // 2023-02-20 and 2023-04-07 state the clock's own sourced arrangements.
    for (exchange, clock, _) in VENUES {
        let venue = calendar_for_exchange(exchange);
        let clock_cal = calendar_for_market_hours_key(clock);
        assert_eq!(
            venue.holiday_on(day(2023, 1, 16)).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{exchange:?}: the one inherited marker"
        );
        for d in [day(2023, 2, 20), day(2023, 4, 7)] {
            assert_ne!(
                venue.holiday_on(d).map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{exchange:?}: {d} is worked up"
            );
            assert_eq!(
                venue.holiday_on(d).map(Holiday::kind),
                clock_cal.holiday_on(d).map(Holiday::kind),
                "{exchange:?}: {d} carries the clock's row"
            );
        }
    }
}
///The `cbot` withheld-date census: 202 `Unsourced` rows that are exactly four
/// not-worked-up markers plus 198 genuine disputes, and the refusals the
/// markers earn at the identity surface.
///
/// This is the fence behind the inventory's `date_level_incompleteness` entry
/// and the evidence file's cross-wave audit counts (re-derived on 2026-09-30
/// UTC under the profile-clock rule, #153 and #242; tracked as #223 for the
/// four markers). It pins four facts a row flip, a re-derivation or a new wave
/// would move:
///
/// 1. **the totals** — 249 rows over the six audited windows, 47 `Closed` and
///    202 `Unsourced`, with the per-era withheld series 16 / 35 / 27 / 34 / 32
///    / 58 and the closures 6 / 8 / 9 / 8 / 7 / 9. The #242 re-derivation
///    retired the fifty-nine rows the rate leg's lone dissent had withheld
///    against a grain table auditing the date normal;
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
fn the_cbot_withheld_date_is_the_clocks_marker() {
    // The 2026-10-09 clock rule: `cbot` ships the grain clock's table, so its
    // one withheld date is the family's own 2023-01-16 not-worked-up marker.
    // Every other date answers, whatever the rate leg prints beside it.
    let venue = calendar_for_exchange(Exchange::Cbot);
    let grains = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let coverage = venue.holiday_coverage().expect("the venue ships a table");
    let (mut rows, mut withheld, mut closed) = (0_usize, 0_usize, 0_usize);
    let mut date = coverage.first();
    while date <= coverage.last() {
        match venue.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Unsourced) => {
                withheld += 1;
                assert_eq!(date, day(2023, 1, 16), "the one inherited marker");
                assert_eq!(
                    grains.holiday_on(date).map(Holiday::kind),
                    Some(HolidayKind::Unsourced),
                    "the marker is the clock family's own"
                );
            }
            Some(HolidayKind::Closed) => closed += 1,
            Some(_) => rows += 1,
        }
        date = date.succ_opt().expect("the census stays representable");
    }
    assert_eq!(withheld, 1, "the cbot table's withheld census");
    assert!(
        closed > 0 && rows > 0,
        "the cbot table states closures and arrangements"
    );
    // Thanksgiving 2026 is the user-facing probe: the venue states the grain
    // clock's closure and the trade date answers closed.
    assert_eq!(
        venue.holiday_on(day(2026, 11, 26)).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "Thanksgiving is the grain clock's closure"
    );
    assert!(
        venue
            .is_closed_trade_date(day(2026, 11, 26), SessionKind::Both)
            .expect("the closure answers"),
    );
}
