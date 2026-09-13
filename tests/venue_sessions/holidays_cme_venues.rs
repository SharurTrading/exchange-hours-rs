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
    CalendarSource, Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_exchange, calendar_for_market_hours_key,
};

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
        let stated = calendars
            .iter()
            .map(|calendar| calendar.holiday_on(date).map(Holiday::kind))
            .collect::<Vec<_>>();
        let first = stated.first().copied().expect("at least one family routes");
        let agreed = stated.iter().all(|kind| *kind == first);
        rows.push((
            date,
            match (agreed, first) {
                (true, None) => Joint::AuditedNormal,
                (true, Some(kind)) => Joint::Agreed(kind),
                (false, _) => Joint::Disputed,
            },
        ));
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

/// The nine dates the six-family (or two-family) intersection states a status
/// for are the only dates those venues may ship a `Closed` row on; the
/// single-family energy venues, whose scope is narrower, additionally close for
/// Good Friday 2026 and carry that row too.
///
/// This is the fact the venue tables are built out of, asserted from the
/// operator-facing side: a `Closed` venue row is a claim that every routed
/// family closed, and nothing outside these sets may make it.
#[test]
fn a_closed_venue_row_is_a_unanimous_closure() {
    const UNANIMOUS: [(i32, u32, u32); 9] = [
        (2025, 1, 1),
        (2025, 4, 18),
        (2025, 11, 29),
        (2025, 12, 25),
        (2026, 1, 1),
        (2026, 12, 25),
        (2027, 1, 1),
        (2027, 3, 26),
        (2027, 12, 24),
    ];
    /// The energy family's own extra closure: a six-family venue cannot state
    /// it, and `Exchange::Comex`/`Nymex` can.
    const ENERGY_ONLY: (i32, u32, u32) = (2026, 4, 3);

    for (exchange, families) in VENUES {
        let single_family = matches!(exchange, Exchange::Comex | Exchange::Nymex);
        let venue = calendar_for_exchange(exchange);
        let coverage = venue.holiday_coverage().expect("the venue ships a table");

        for (year, month, date) in UNANIMOUS {
            let closure = day(year, month, date);
            assert!(
                coverage.contains(closure),
                "{exchange:?}: {closure} is inside the audited window"
            );
            assert_eq!(
                venue.holiday_on(closure).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{exchange:?}: {closure} is a unanimous closure"
            );
            assert!(
                venue.is_closed_trade_date(closure, SessionKind::Both),
                "{exchange:?}: {closure} closes the venue's trading day"
            );
        }

        let mut date = coverage.first();
        while date <= coverage.last() {
            if venue.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Closed) {
                let unanimous = UNANIMOUS.iter().any(|(y, m, d)| day(*y, *m, *d) == date);
                let energy_only =
                    single_family && date == day(ENERGY_ONLY.0, ENERGY_ONLY.1, ENERGY_ONLY.2);
                assert!(
                    unanimous || energy_only,
                    "{exchange:?}: {date} ships a `Closed` row but is not a closure every \
                     family routing to this venue states"
                );
            }
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
        let expected = if single_family {
            0
        } else if families.len() == 6 {
            32
        } else {
            31
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
            assert_ne!(
                family,
                Some(HolidayKind::Unsourced),
                "{exchange:?}: a single-family intersection can never disagree"
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
        assert!(venue.is_closed_trade_date(day(2026, 4, 3), SessionKind::Both));
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

    assert!(venue.is_closed_trade_date(closure, SessionKind::Both));
    // No *trade date* 2025-12-25 exists: the 12-25 day session is gone, and so
    // is the 12-25 17:00 CT leg that would have settled trade date 12-26. Both
    // instants fall inside the trade date the row names.
    for probe in [
        ct((2025, 12, 25), (10, 0, 0)),
        ct((2025, 12, 25), (15, 30, 0)),
    ] {
        assert!(!venue.is_open(probe), "{probe} is inside the removed day");
        assert_eq!(venue.trade_date(probe), None, "{probe}");
    }
    // The evening of the *following* civil day belongs to trade date 12-26,
    // which no row names, so it survives.
    let post_holiday_eve = ct((2025, 12, 25), (17, 30, 0));
    assert!(venue.is_open(post_holiday_eve), "{post_holiday_eve}");
    assert_eq!(venue.trade_date(post_holiday_eve), Some(day(2025, 12, 26)));
    // 2025-12-24's own session is untouched: the row names the *next* trade
    // date, so it cannot clip the one before it.
    assert!(venue.is_open(ct((2025, 12, 24), (12, 0, 0))));
    assert_eq!(
        venue.trade_date(ct((2025, 12, 24), (12, 0, 0))),
        Some(day(2025, 12, 24))
    );
    // The next session after the closure opens at 2025-12-25 17:00 CT.
    assert_eq!(
        venue.next_session_after(ct((2025, 12, 25), (12, 0, 0))),
        Some((
            ct((2025, 12, 25), (17, 0, 0)),
            ct((2025, 12, 26), (8, 30, 0)),
        )),
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

    let detached = venue.without_holidays();
    let start = US::Central
        .with_ymd_and_hms(2025, 11, 29, 0, 0, 0)
        .single()
        .expect("fixture must be a valid CT midnight")
        .with_timezone(&Utc);
    for step in 0..48 {
        let probe = start + chrono::TimeDelta::minutes(30 * step);
        assert_eq!(venue.is_open(probe), detached.is_open(probe), "{probe}");
        assert_eq!(
            venue.session_bounds(probe),
            detached.session_bounds(probe),
            "{probe}"
        );
        assert_eq!(
            venue.trade_date(probe),
            detached.trade_date(probe),
            "{probe}"
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
        assert!(
            !venue.is_closed_trade_date(disputed, SessionKind::Both),
            "{exchange:?}: `Unsourced` closes nothing"
        );
        for probe in probes {
            assert_eq!(
                venue.trade_date(probe),
                Some(disputed),
                "{exchange:?}: {probe} still belongs to the disputed trade date"
            );
            assert!(
                venue.is_open(probe),
                "{exchange:?}: {probe} is inside the venue's ordinary session; an \
                 `Unsourced` row must not clip it"
            );
            assert_eq!(
                venue.is_open(probe),
                venue.without_holidays().is_open(probe),
                "{exchange:?}: `Unsourced` moved is_open at {probe}"
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

        // Christmas Day itself: the layer is the only thing that closes it.
        let christmas = ct((2025, 12, 25), (10, 0, 0));
        assert!(!venue.is_open(christmas), "{exchange:?}");
        assert!(
            detached.is_open(christmas) || detached.session_bounds(christmas).is_none(),
            "{exchange:?}: detaching must restore the normal-week answer"
        );
    }
}
