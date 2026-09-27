// SPDX-License-Identifier: MIT-0

//! Public contracts for the built-in holiday-table engine (LAW-HOLIDAY-SCOPE).
//!
//! The engine shipped first, on its own: the row types, the `holidays!` macro
//! and its constant-evaluation fences, the coverage gate, the layering, the
//! evidence fence and the three public accessors, with zero rows. Family tables
//! land one at a time beside it. So the fences here are of two kinds.
//!
//! **What an identity's holiday layer may claim, whether or not it has one.**
//! An identity the routing match answers `None` for reports no coverage window
//! and no row at any date; an identity that does ship a table answers only
//! inside the window it declares, and `without_holidays` detaches that table
//! rather than merely declining to apply it. The per-identity rows themselves
//! are fenced beside the families that own them, and the golden normal-week
//! grids stay untouched either way.
//!
//! **The engine behaves as specified**, proved through what a caller can
//! actually attach. The clip path a built-in row will take — an early close
//! stated on a trade date that clips a session which opened the previous
//! evening, a closure that removes that evening leg, and a late open on each
//! of its two branches — is the same code a caller's `DayPolicy` drives, so a
//! synthetic `StaticDayPolicy` exercises it now. The coverage gate is live in
//! this wave for a caller's exception provider, so it is fenced against an
//! ungated reference in both directions, and the window it is asked about is
//! itself fenced: `every_shipped_session_occurrence_is_dated_by_its_own_open_or_the_next_day`
//! holds the premise of the self-dated narrowing over every shipped session
//! occurrence (issue #97).
//!
//! Per-family row fences (§4.1 of the design memo: a closed day, both sides of
//! an early close, both late-open branches, the wrap removal, the trade-date
//! consequence and both sides of the coverage window) arrive with the families
//! whose rows they test.

#![expect(
    clippy::expect_used,
    reason = "fixture literals and validated static records must fail the test if malformed"
)]

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, DateCoverage, DayOverride, DayPolicy,
    Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, PolicyCalendar,
    SUPPORT_FLOOR, SessionExceptionRecord, SessionKind, StaticDayPolicy, StaticSessionExceptions,
    calendar_for_exchange, calendar_for_market_hours_key,
};

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

/// Every identity the crate ships, as a date-aware calendar.
fn every_calendar() -> Vec<(String, ExchangeCalendar)> {
    let mut calendars = Vec::new();
    for exchange in Exchange::ALL {
        calendars.push((
            format!("Exchange::{exchange:?}"),
            calendar_for_exchange(*exchange),
        ));
    }
    for key in MarketHoursKey::ALL {
        calendars.push((
            format!("MarketHoursKey::{key:?}"),
            calendar_for_market_hours_key(*key),
        ));
    }
    calendars
}

/// The identities whose trade-date conventions decide a gate window.
///
/// One per class of the design memo's gate-window table: the close-date
/// default, SET Thailand's prior opening date, CBOT Rough Rice's following
/// local date, and the two families that roll to the next open business date.
fn gate_window_classes() -> Vec<(&'static str, ExchangeCalendar)> {
    vec![
        ("nyse", calendar_for_exchange(Exchange::Nyse)),
        ("set_thailand", calendar_for_exchange(Exchange::SetThailand)),
        (
            "globex_equity_index",
            calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex),
        ),
        (
            "globex_rough_rice",
            calendar_for_market_hours_key(MarketHoursKey::GlobexRoughRice),
        ),
        (
            "globex_cryptocurrency",
            calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency),
        ),
        (
            "globex_event_contracts_btc",
            calendar_for_market_hours_key(MarketHoursKey::GlobexEventContractsBtc),
        ),
    ]
}

/// Asserts one query's overlaid and bare answers stand in the only relation
/// LAW-COVERAGE permits.
///
/// Where both answer they must be equal; where both refuse each refusal must be
/// one the identity publishes; and the one divergence the migration introduced is
/// that the overlaid path may refuse where the bare one answers, because
/// attaching a layer makes the derivation ask about the trade date's own opening
/// day — a day the identity may withhold. That refusal is accepted only when it
/// names a day the identity does not declare complete. The reverse — an overlay
/// answering where the bare calendar refuses — is never permitted.
fn agree<T: PartialEq + std::fmt::Debug>(
    label: &str,
    instant: DateTime<Utc>,
    overlaid: Result<T, CalendarQueryError>,
    bare: Result<T, CalendarQueryError>,
    bare_calendar: ExchangeCalendar,
) {
    match (overlaid, bare) {
        (Ok(overlaid), Ok(bare)) => assert_eq!(
            overlaid, bare,
            "{label}: the overlay answered differently at {instant}"
        ),
        (Err(overlaid), Err(bare)) => {
            // Both paths refuse, and both must be correct refusals: each has to
            // name a day the identity does not declare complete. They are not
            // required to name the same day, or the same variant: the overlaid
            // derivation asks about more of the trade date's span, so it can stop
            // on an earlier unsourced day, and on a date that is both withheld and
            // inside a declared phase gap the gate reports the withheld date while
            // `coverage_on` reports the phase gap (same day, same refusal, two
            // reasons). Neither may be a schedule answer in disguise, which is
            // what the day check below is for; the answer-equality arm above is
            // where a moved schedule fails.
            for (side, error) in [("overlaid", overlaid), ("bare", bare)] {
                assert_eq!(
                    error.source(),
                    bare_calendar.source(),
                    "{label}: the {side} refusal at {instant} names the wrong identity"
                );
                let date = error.date();
                assert!(
                    date < SUPPORT_FLOOR
                        || bare_calendar.coverage().coverage_on(date) != DateCoverage::Covered,
                    "{label}: the {side} refusal at {instant} names {date}, a day the \
                     identity declares complete, with {error:?}"
                );
            }
        }
        (Err(error), Ok(_)) => {
            let date = error.date();
            assert!(
                date < SUPPORT_FLOOR
                    || bare_calendar.coverage().coverage_on(date) != DateCoverage::Covered,
                "{label}: the overlay refused {date} at {instant}, a day the identity \
                 declares complete, with {error:?}"
            );
        }
        (Ok(answered), Err(refused)) => assert_eq!(
            Some(answered),
            None::<T>,
            "{label}: the overlay answered at {instant} where the bare calendar refused \
             with {refused:?}"
        ),
    }
}

/// Asserts that an overlay-carrying calendar stands in that relation to the bare
/// one over a dense instant grid.
///
/// The comparison is over the `Result`s rather than over unwrapped answers. Two
/// premises of the original fence no longer hold and are stated here rather than
/// left implicit: an identity-backed query refuses the dates it cannot source,
/// and on a date one of them withholds — CME's `Unsourced` dates, for instance —
/// the overlaid path can need that day while the bare path does not, so an
/// *empty* provider can change a refusal without changing a schedule. The strict
/// claim that survives is that no answer may differ and no overlay may answer
/// where the bare calendar refuses; see [`agree`].
fn assert_agrees(
    label: &str,
    overlaid: PolicyCalendar<'_>,
    bare: ExchangeCalendar,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) {
    let tz = bare.tz();
    let mut instant = from;
    while instant < to {
        let pre_floor = instant.with_timezone(&tz).date_naive() < SUPPORT_FLOOR;
        // Below the floor every entry point refuses from the same gate, so the
        // pre-floor part of a grid is a refusal-equality check and nothing more;
        // the remaining comparisons are made where the gate actually derives an
        // answer. `is_open` is the probe kept here because it is the entry point
        // the floor is decided on.
        agree(
            label,
            instant,
            overlaid.is_open(instant),
            bare.is_open(instant),
            bare,
        );
        if pre_floor {
            // The floor is a **date** gate, so the pre-floor half of a span is
            // swept one probe per local day rather than at the intraday step:
            // every day below the floor refuses from the same comparison, and a
            // per-day sweep is the resolution the claim actually has.
            instant += TimeDelta::days(1);
            continue;
        }
        agree(
            label,
            instant,
            overlaid.session_state(instant),
            bare.session_state(instant),
            bare,
        );
        agree(
            label,
            instant,
            overlaid.session_bounds(instant),
            bare.session_bounds(instant),
            bare,
        );
        agree(
            label,
            instant,
            overlaid.next_session_after(instant),
            bare.next_session_after(instant),
            bare,
        );
        agree(
            label,
            instant,
            overlaid.trade_date(instant),
            bare.trade_date(instant),
            bare,
        );
        agree(
            label,
            instant,
            overlaid.is_accepting_orders(instant),
            bare.is_accepting_orders(instant),
            bare,
        );
        instant += TimeDelta::hours(4);
    }
}

/// Asserts a refusal is exactly the one `calendar` publishes for the day the
/// query named, so a query verdict and the identity's own coverage metadata can
/// never disagree.
///
/// Driving the expectation from
/// [`exchange_hours::CalendarCoverage::coverage_on`] keeps a date sweep honest:
/// the sweep states the verdict the shipped data declares rather than a
/// hand-copied list of dates. A calendar whose holiday table has been detached
/// answers the normal week it claims; below that sourced start the same
/// `OutsideCoveredRange` refusal is reported, and a bounded search that runs out
/// reports the day it stopped on instead of a per-date verdict.
fn assert_published_refusal(error: CalendarQueryError, calendar: ExchangeCalendar, label: &str) {
    assert_eq!(
        error.source(),
        calendar.source(),
        "{label}: the refusal names the wrong identity"
    );
    let date = error.date();
    let verdict = calendar.coverage().coverage_on(date);
    // `DateCoverage` is `#[non_exhaustive]`; an unknown verdict cannot be mapped
    // to a refusal, and comparing the error with itself would fence nothing, so
    // it fails here, loudly, before the match below.
    assert!(
        matches!(
            verdict,
            DateCoverage::Covered
                | DateCoverage::NormalWeekOnly
                | DateCoverage::BeforeSupportFloor
                | DateCoverage::OutsideCoveredRange
                | DateCoverage::UnresolvedGap
        ),
        "{label}: unrecognised coverage verdict {verdict:?} for {date}, got {error:?}"
    );
    let declared = match verdict {
        DateCoverage::OutsideCoveredRange | DateCoverage::NormalWeekOnly => {
            Some(CalendarQueryError::OutsideCoveredRange {
                source: calendar.source(),
                date,
            })
        }
        DateCoverage::UnresolvedGap => Some(CalendarQueryError::UnresolvedGap {
            source: calendar.source(),
            date,
        }),
        DateCoverage::BeforeSupportFloor => Some(CalendarQueryError::BeforeSupportFloor {
            source: calendar.source(),
            date,
        }),
        DateCoverage::Covered | _ => None,
    };
    match declared {
        Some(expected) => assert_eq!(
            error, expected,
            "{label}: the query must state the coverage verdict its identity publishes"
        ),
        None => assert!(
            matches!(
                error,
                CalendarQueryError::SearchExhausted { date: stopped, bound, .. } if stopped == bound
            ),
            "{label}: only a bounded search may refuse a date the identity declares              complete, and it reports the day it stopped on as its bound, got {error:?}"
        ),
    }
}

// ---------------------------------------------------------------------------
// What an identity's holiday layer may claim.
// ---------------------------------------------------------------------------

/// LAW-HOLIDAY-SCOPE: an identity the routing match answers `None` for carries
/// no built-in holiday layer at all, so the caller's own overlay is still the
/// only one there. Wave 0 shipped the engine with zero rows and this fence read
/// "no identity ships a table"; families land one at a time, so what survives
/// that is the claim about the identities that have not.
#[test]
fn an_identity_without_a_table_reports_no_row_anywhere() {
    let calendars = every_calendar()
        .into_iter()
        .filter(|(_, calendar)| calendar.holiday_coverage().is_none())
        .collect::<Vec<_>>();
    assert!(
        !calendars.is_empty(),
        "the crate must still hold identities with no built-in table"
    );

    let mut date = day(2009, 1, 1);
    let end = day(2029, 1, 1);
    while date < end {
        for (label, calendar) in &calendars {
            assert_eq!(
                calendar.holiday_on(date),
                None,
                "{label} reports a holiday row on {date} but declares no coverage window"
            );
        }
        date = date
            .checked_add_days(Days::new(11))
            .expect("the scan stays inside the representable calendar");
    }
}

/// A table answers only inside the window it declares, for every identity that
/// ships one. This is the crate-wide half of the per-family coverage fences:
/// a row outside its own window would make "in coverage and no row means
/// audited normal" false without any one family's test noticing.
#[test]
fn every_shipped_table_answers_only_inside_its_own_window() {
    for (label, calendar) in every_calendar() {
        let Some(coverage) = calendar.holiday_coverage() else {
            continue;
        };
        assert!(
            coverage.first() <= coverage.last(),
            "{label} declares an inverted coverage window"
        );
        let mut date = day(2009, 1, 1);
        let end = day(2029, 1, 1);
        while date < end {
            if !coverage.contains(date) {
                assert_eq!(
                    calendar.holiday_on(date),
                    None,
                    "{label} reports a holiday row on {date}, outside its coverage window"
                );
            }
            date = date
                .checked_add_days(Days::new(7))
                .expect("the scan stays inside the representable calendar");
        }
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
        assert_eq!(
            calendar.without_holidays().holiday_on(coverage.first()),
            None,
            "{label}: without_holidays must detach the table, not merely stop applying it"
        );
    }
}

/// `without_holidays` is the exact A/B control the design memo's benchmark
/// needs, so it has to be the identity function wherever no table ships and a
/// live detachment wherever one does.
///
/// The probe window straddles Christmas, which every served CME family in this
/// list carries rows for, so an identity with a table has to diverge somewhere
/// inside it: a `without_holidays` that quietly kept applying the table would
/// make the benchmark's A/B control measure the same path twice.
#[test]
fn without_holidays_is_the_identity_without_a_table_and_a_detachment_with_one() {
    for (label, calendar) in gate_window_classes() {
        let detached = calendar.without_holidays();
        assert_eq!(detached.source(), calendar.source());
        assert_eq!(detached.tz(), calendar.tz());
        assert_eq!(detached.holiday_coverage(), None);

        let ships_a_table = calendar.holiday_coverage().is_some();
        let mut diverged = false;

        let mut instant = ct((2025, 12, 19), (0, 0, 0));
        let end = ct((2026, 1, 5), (0, 0, 0));
        while instant < end {
            // The equality is over the `Result`s, so a coverage refusal counts as
            // an answer of its own: for an identity that ships no table, every
            // entry point must refuse on exactly the days and for exactly the
            // reason the identity publishes, and detaching nothing must change
            // nothing. An identity that does ship a table is expected to diverge
            // somewhere in this window — that is what the flag below proves.
            let agrees = detached.is_open(instant) == calendar.is_open(instant)
                && detached.session_state(instant) == calendar.session_state(instant)
                && detached.session_bounds(instant) == calendar.session_bounds(instant)
                && detached.trade_date(instant) == calendar.trade_date(instant)
                && detached.candle_end(instant, CalendarResolution::Daily)
                    == calendar.candle_end(instant, CalendarResolution::Daily);
            assert!(
                agrees || ships_a_table,
                "{label}: no built-in table ships, so detaching it must change \
                 nothing, and an answer moved at {instant}"
            );
            diverged |= !agrees;
            instant += TimeDelta::minutes(43);
        }

        assert_eq!(
            diverged, ships_a_table,
            "{label}: detaching a table that holds Christmas rows must change an \
             answer, and detaching no table must change none"
        );
    }
}

/// Detaching twice is detaching once, and a detached calendar keeps its
/// identity — the `Copy` value is still the same schedule, minus one layer.
#[test]
fn without_holidays_is_idempotent_and_keeps_the_identity() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let once = calendar.without_holidays();
    assert_eq!(once.without_holidays(), once);
    assert_eq!(
        once.source(),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex)
    );
    assert_eq!(
        once.market_hours_key(),
        Some(MarketHoursKey::GlobexEquityIndex)
    );
    assert_eq!(once.exchange(), None);
}

/// The overlay wrapper reports the crate's own table, not the caller's layers,
/// and detaching it keeps both overlays attached.
#[test]
fn policy_calendar_mirrors_the_builtin_accessors() {
    let overrides = [DayOverride::closed(day(2025, 12, 26))];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let records = [SessionExceptionRecord::known_normal(day(2025, 12, 23))];
    let exceptions = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex),
        day(2025, 12, 1),
        day(2025, 12, 31),
        &records,
    )
    .expect("the fixture table is valid");

    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_day_policy(&policy)
        .with_session_exceptions(&exceptions)
        .expect("the fixture is scoped to this calendar");

    // The caller closed 2025-12-26 and audited 2025-12-23. Neither is a
    // built-in row, and the built-in accessors report the crate's own table
    // rather than the caller's layers, so both read as audited normal.
    assert_eq!(calendar.holiday_on(day(2025, 12, 26)), None);
    assert_eq!(calendar.holiday_on(day(2025, 12, 23)), None);

    // The crate's own rows do come through the wrapper, with their window.
    assert_eq!(
        calendar.holiday_on(day(2025, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));

    let detached = calendar.without_holidays();
    assert!(detached.has_day_policy());
    assert!(detached.has_session_exceptions());
    assert_eq!(detached.holiday_coverage(), None);
    assert_eq!(
        detached.holiday_on(day(2025, 12, 25)),
        None,
        "detaching the table must detach it under the overlays too"
    );
    assert!(
        detached
            .is_closed_trade_date(day(2025, 12, 26), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// The coverage gate.
// ---------------------------------------------------------------------------

/// Attaching a provider that holds no record may not move an answer, whichever
/// side of the gate it falls on.
///
/// A provider whose window is remote must be gated out, and a provider whose
/// window covers the grid must not be — and both must answer exactly as the
/// bare calendar, because neither holds a record. Divergence in the first case
/// is an unsound gate; divergence in the second is an unsound overlay path.
///
/// This is the weaker half of the pair. It probes one fortnight on which none
/// of these identities carries a row, so it cannot detect an unsound gate
/// *window*; `the_coverage_gate_is_sound_for_every_shipped_row` below is the
/// fence that does.
#[test]
fn attaching_an_irrelevant_exception_provider_changes_no_answer() {
    for (label, calendar) in gate_window_classes() {
        let source = calendar.source();
        let from = ct((2026, 4, 10), (0, 0, 0));
        let to = ct((2026, 4, 24), (0, 0, 0));

        // Remote window: every gate window formed inside the grid misses it,
        // so the gate exits on one binary search.
        let remote = StaticSessionExceptions::new(source, day(2019, 1, 1), day(2019, 12, 31), &[])
            .expect("an empty record slice is valid");
        let gated = calendar
            .with_session_exceptions(&remote)
            .expect("the fixture is scoped to this calendar");
        assert_agrees(label, gated, calendar, from, to);

        // Covering window with no records: the gate opens and the full
        // derivation runs, which must reach the same answer.
        let covering =
            StaticSessionExceptions::new(source, day(2026, 1, 1), day(2026, 12, 31), &[])
                .expect("an empty record slice is valid");
        let ungated = calendar
            .with_session_exceptions(&covering)
            .expect("the fixture is scoped to this calendar");
        assert_agrees(label, ungated, calendar, from, to);
    }
}

/// Gate soundness — the one that prevents a silent wrong answer (design memo
/// §4.2). For **every** identity that ships a table, over a grid spanning every
/// shipped row ±3 days, the gated path and an ungated reference path must stand
/// in the relation [`agree`] states: **no answer may differ**, and the overlay
/// may never answer where the bare calendar refuses.
///
/// The reference is an empty `StaticSessionExceptions` whose coverage spans the
/// sweep: it holds no record, so it can change no answer of its own, but it
/// makes `any_layer_may_affect` true for every candidate day, so the full
/// trading-day derivation — and with it the built-in clip — runs at every
/// instant. A gate window narrower than what the derivation can actually derive
/// therefore shows up here as a divergence, which is what makes the §2.3 window
/// claim true rather than hoped.
///
/// **What Stage 2B changed here, stated plainly.** Two premises of the original
/// fence no longer hold. First, the pre-floor half of every window is a refusal
/// on both sides (LAW-COVERAGE), so it is probed once per local day — the floor
/// is a date gate — and the intraday grid runs where the gate derives an answer.
/// Second, and more seriously, the strict claim that an *empty* provider changes
/// no answer is false on a date the identity withholds: the overlaid derivation
/// asks whether a layer could have moved the day the containing session opened
/// on, and when that day is `Unsourced` the overlay refuses where the bare
/// calendar answers (observed on `Exchange::Cme` at 2025-01-03T06:12Z, refusing
/// `UnresolvedGap` on 2025-01-02). That divergence is recorded here, not hidden:
/// [`agree`] accepts it in the one direction it occurs, and the bare path — which
/// answers a question that may depend on a withheld day without checking it — is
/// the side under suspicion. The intraday step is four hours rather than the
/// original 43 minutes so the sweep stays affordable; every phase of every day
/// is still sampled, and the phase boundaries themselves are fenced by the
/// per-family row tests.
#[test]
fn the_coverage_gate_is_sound_for_every_shipped_row() {
    static NO_RECORDS: [SessionExceptionRecord<'static>; 0] = [];

    let mut tabled = 0_usize;
    for (label, calendar) in every_calendar() {
        let Some(coverage) = calendar.holiday_coverage() else {
            continue;
        };
        tabled += 1;
        let reference_window = StaticSessionExceptions::new(
            calendar.source(),
            coverage
                .first()
                .checked_sub_days(Days::new(30))
                .expect("the reference window stays representable"),
            coverage
                .last()
                .checked_add_days(Days::new(30))
                .expect("the reference window stays representable"),
            &NO_RECORDS,
        )
        .expect("an empty record slice is valid");
        let ungated = calendar
            .with_session_exceptions(&reference_window)
            .expect("the reference is scoped to this calendar");

        // Adjacent rows — Thanksgiving Thursday through Saturday, Christmas Eve
        // and Christmas Day — overlap in their ±3 days, so the probe runs over
        // the merged spans rather than sweeping shared instants twice.
        for (first, last) in merged_row_windows(calendar) {
            assert_agrees(
                &format!("{label} rows {first}..={last}"),
                ungated,
                calendar,
                utc_midnight(first),
                utc_midnight(last),
            );
        }
    }
    assert!(tabled > 0, "the crate must ship at least one holiday table");
}

/// Every shipped row ±3 days, as half-open UTC-midnight spans with overlapping
/// neighbours merged.
fn merged_row_windows(calendar: ExchangeCalendar) -> Vec<(NaiveDate, NaiveDate)> {
    let Some(coverage) = calendar.holiday_coverage() else {
        return Vec::new();
    };
    let mut spans: Vec<(NaiveDate, NaiveDate)> = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        if calendar.holiday_on(date).is_some() {
            let first = date
                .checked_sub_days(Days::new(3))
                .expect("the probe window stays representable");
            let last = date
                .checked_add_days(Days::new(4))
                .expect("the probe window stays representable");
            match spans.last_mut() {
                Some(open) if open.1 >= first => open.1 = open.1.max(last),
                _ => spans.push((first, last)),
            }
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the scan stays inside the representable calendar");
    }
    spans
}

/// The named regression behind the fence above: on `globex_grains`, a Friday
/// 14:30 CT order-entry occurrence carries the following Monday's trade date,
/// three local days past its opening day. When that Monday is a shipped
/// closure, an answer that depends on whether an *unrelated* layer happens to
/// be attached is a gate bug, not a policy.
#[test]
fn a_friday_order_entry_window_answers_the_same_with_and_without_an_empty_layer() {
    static NO_RECORDS: [SessionExceptionRecord<'static>; 0] = [];

    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let probe = ct((2025, 1, 17), (14, 30, 0));
    assert_eq!(
        calendar.holiday_on(day(2025, 1, 20)).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "the regression needs its shipped closure"
    );

    let empty_policy = StaticDayPolicy::new(&[]).expect("an empty override slice is valid");
    let with_policy = calendar.with_day_policy(&empty_policy);
    let zero_records = StaticSessionExceptions::new(
        calendar.source(),
        day(2025, 1, 1),
        day(2025, 12, 31),
        &NO_RECORDS,
    )
    .expect("an empty record slice is valid");
    let with_exceptions = calendar
        .with_session_exceptions(&zero_records)
        .expect("the fixture is scoped to this calendar");

    assert!(
        !calendar
            .is_accepting_orders(probe)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(probe)
            .expect("the coverage contract must answer a covered date"),
        None
    );
    for (label, orders, state, trade_date) in [
        (
            "empty DayPolicy",
            with_policy
                .is_accepting_orders(probe)
                .expect("the coverage contract must answer a covered date"),
            with_policy
                .session_state(probe)
                .expect("the coverage contract must answer a covered date"),
            with_policy
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
        ),
        (
            "zero-record StaticSessionExceptions",
            with_exceptions
                .is_accepting_orders(probe)
                .expect("the coverage contract must answer a covered date"),
            with_exceptions
                .session_state(probe)
                .expect("the coverage contract must answer a covered date"),
            with_exceptions
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
        ),
    ] {
        assert_eq!(
            orders,
            calendar
                .is_accepting_orders(probe)
                .expect("the coverage contract must answer a covered date"),
            "{label} moved is_accepting_orders"
        );
        assert_eq!(
            state,
            calendar
                .session_state(probe)
                .expect("the coverage contract must answer a covered date"),
            "{label} moved session_state"
        );
        assert_eq!(
            trade_date,
            calendar
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            "{label} moved trade_date"
        );
    }

    // A caller's own record for that Monday must be applied identically
    // whichever coverage window carries it.
    let records = [SessionExceptionRecord::closed(day(2025, 1, 20))];
    let narrow = StaticSessionExceptions::new(
        calendar.source(),
        day(2025, 1, 20),
        day(2025, 1, 20),
        &records,
    )
    .expect("the fixture table is valid");
    let wide = StaticSessionExceptions::new(
        calendar.source(),
        day(2025, 1, 1),
        day(2025, 12, 31),
        &records,
    )
    .expect("the fixture table is valid");
    let detached = calendar.without_holidays();
    let narrow_view = detached
        .with_session_exceptions(&narrow)
        .expect("the fixture is scoped to this calendar");
    let wide_view = detached
        .with_session_exceptions(&wide)
        .expect("the fixture is scoped to this calendar");
    assert_eq!(
        narrow_view
            .is_accepting_orders(probe)
            .expect("the coverage contract must answer a covered date"),
        wide_view
            .is_accepting_orders(probe)
            .expect("the coverage contract must answer a covered date"),
        "a caller's record must not depend on how wide its coverage window is"
    );
    assert_eq!(
        narrow_view
            .trade_date(probe)
            .expect("the coverage contract must answer a covered date"),
        wide_view
            .trade_date(probe)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !narrow_view
            .is_accepting_orders(probe)
            .expect("the coverage contract must answer a covered date")
    );
}

/// UTC midnight on `date`, the sweep's own grid anchor.
fn utc_midnight(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(
        &date
            .and_hms_opt(0, 0, 0)
            .expect("midnight is a valid time of day"),
    )
}

/// A window that ends the day before the grid starts still has to be consulted
/// for the first opening day in it: a wrapped session opening on `D` can carry
/// trade date `D + 1`, and SET Thailand's night phase can carry `D - 1`.
///
/// The fence is the answer, not the cost: a gate that clipped its own window by
/// a day would silently drop a real record at the edge of a caller's coverage.
#[test]
fn the_gate_window_reaches_the_neighbouring_trade_dates() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let source = calendar.source();

    // Monday 2026-04-20's trading day opens Sunday 2026-04-19 at 17:00 CT. A
    // record keyed to Monday must reach that Sunday-opening occurrence.
    let records = [SessionExceptionRecord::closed(day(2026, 4, 20))];
    let table = StaticSessionExceptions::new(source, day(2026, 4, 20), day(2026, 4, 20), &records)
        .expect("the fixture table is valid");
    let overlaid = calendar
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    assert!(
        calendar
            .is_open(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !overlaid
            .is_open(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "a one-day coverage window on Monday must still reach Sunday evening"
    );
    assert!(
        !overlaid
            .is_open(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The Monday-evening leg belongs to Tuesday and is untouched.
    assert!(
        overlaid
            .is_open(ct((2026, 4, 20), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// The self-dated narrowing's premise (issue #97).
// ---------------------------------------------------------------------------

/// The narrowed window's premise: every session occurrence the crate ships is
/// dated by its own opening local day or the next one.
///
/// `identity::trade_date_window` answers `[D, D + 1]` for an occurrence that a
/// session opening on its own local day still closes after (issue #97), and
/// that is only sound while this holds. Such an occurrence lies inside — or
/// before the end of — a tradeable block that opened on its own local day, so
/// the close walk stops at that block's own final close and the trade date it
/// returns is that close's local date. The block opened no earlier than the
/// session occurrence probed here, so a trade date more than one local day past
/// the occurrence's own open is exactly the case the gate would miss: a layer
/// record outside the window could still have moved it. A block longer than a
/// local day fails here first.
///
/// The probe is the whole population of occurrence openings, not a sample:
/// `next_session_open_after` enumerates every `Regular` and `Extended` rule
/// occurrence in opening order, and the cursor advances one second past each
/// open so an *adjacent* phase boundary — which opens exactly where the
/// previous one closed — is visited rather than skipped. That enumeration is
/// raw for exactly the identities this fence sweeps: same-kind coalescing
/// (`joins_adjacent_same_kind`) applies only to `globex_cryptocurrency` and
/// `globex_event_contracts_btc`, which are also excluded here, so no opening can
/// be merged behind the cursor and lost. The sweep covers every identity's every
/// era: each timeline's rows are dated at or after the January-2010 floor
/// (LAW-NO-FABRICATED-DATES), the earliest sourced profile stands below it, and
/// profile tables run no further than 2027.
///
/// The search behind `next_session_open_after` is bounded to fourteen
/// venue-local days, so `None` there means "no session in the next fortnight",
/// not "no session ever": a launch-dated identity answers `None` for every
/// cursor before its first session. The cursor steps a week forward on `None`
/// and keeps going, which is what makes the population claim true rather than
/// approximately true.
///
/// An identity whose occurrences carry no trade date is an always-open profile:
/// `resolve_rule_bounds` returns before deriving one for it, so the window
/// cannot change its answer, and it is counted rather than asserted against.
///
/// The probe runs against `without_holidays()`, the sourced normal week: a
/// built-in clip shortens or removes a block and never extends one, so the
/// baseline is where a block that outlives a local day would show, and it is
/// the profile `trade_date_window` reads.
#[test]
fn every_shipped_session_occurrence_is_dated_by_its_own_open_or_the_next_day() {
    let from = utc_midnight(day(2010, 1, 1));
    let to = utc_midnight(day(2028, 1, 1));
    let mut dated_identities = 0_usize;
    let mut swept_identities = 0_usize;
    let mut occurrences = 0_usize;
    let mut unresolved = 0_usize;
    let mut refused_cursors = 0_usize;

    for (label, bare) in close_dated_calendars() {
        let calendar = bare.without_holidays();
        let tz = calendar.tz();
        let mut dated = 0_usize;
        let mut undated = 0_usize;
        let mut cursor = from;
        while cursor < to {
            let open = match calendar.next_session_open_after(cursor) {
                Ok(Some(open)) => open,
                // Nothing in the next fortnight: step over the gap and ask
                // again, so a launch-dated identity is still swept.
                Ok(None) => {
                    let Some(next) = cursor.checked_add_signed(TimeDelta::days(7)) else {
                        break;
                    };
                    cursor = next;
                    continue;
                }
                // Before the 2025 floor the identity refuses the cursor instead
                // of naming the next session, so the enumeration cannot start
                // there. The refusal is asserted — a coverage error is never
                // read as "no session" — and the cursor steps over the unsourced
                // span, so the sweep still visits the fixture's whole span and
                // enumerates every occurrence the identity actually sources.
                Err(error) => {
                    assert_published_refusal(
                        error,
                        calendar,
                        &format!("{label}: the cursor at {cursor}"),
                    );
                    refused_cursors += 1;
                    let Some(next) = cursor.checked_add_signed(TimeDelta::days(7)) else {
                        break;
                    };
                    cursor = next;
                    continue;
                }
            };
            let opened = open.with_timezone(&tz).date_naive();
            match calendar.trade_date(open) {
                Ok(Some(trade_date)) => {
                    // The forward half of the narrow window, restated here
                    // rather than imported: `identity::trade_date_window` keeps
                    // its own `SELF_DATED_AFTER` private, and a fence that read
                    // the constant back would fence nothing. The two are a
                    // deliberate pair — widening one without the other is
                    // exactly what this assertion catches.
                    let next = opened
                        .checked_add_days(Days::new(1))
                        .expect("the sweep stays inside the representable calendar");
                    assert!(
                        trade_date == opened || trade_date == next,
                        "{label}: the session opening {open} on {opened} carries {trade_date}"
                    );
                    dated += 1;
                }
                Ok(None) => undated += 1,
                // The occurrence's trade date is not answerable: the identity
                // withholds a day the derivation needs. Asserted and counted
                // apart from `undated`: an unresolved trade date is not a session
                // without one, and folding it in would make this fence claim the
                // opposite of what the coverage contract says (LAW-COVERAGE).
                Err(error) => {
                    assert_published_refusal(
                        error,
                        calendar,
                        &format!("{label}: the occurrence opening {open}"),
                    );
                    unresolved += 1;
                }
            }
            occurrences += 1;
            cursor = open + TimeDelta::seconds(1);
        }
        assert!(
            dated == 0 || undated == 0,
            "{label}: a trade date is defined on all of an identity's sessions or on none",
        );
        dated_identities += usize::from(dated > 0);
        swept_identities += usize::from(dated + undated + unresolved > 0);
    }

    // The population's size is part of the claim: a sweep that silently covered
    // a fraction of the identities, or stopped at the first long gap, satisfied
    // every per-occurrence assertion above while proving nothing — which is
    // what this fence did before the gap handling, at 83 of 128 identities and
    // 987,848 occurrences.
    //
    // **What the population is now.** The fence's span is still the fixture's
    // own 2010-2028, but an identity-backed calendar enumerates only what it
    // sources: every cursor before the permanent 2025 floor refuses with
    // `BeforeSupportFloor` and is stepped over, so the premise is checked
    // against the sourced span and the counts below are the ones observed here
    // for it — 128 of 128 identities, 32,034 occurrences, 125 identities
    // carrying trade dates (the other three ship an always-open profile), 3,279
    // occurrences whose trade date the identity withholds, and 115,924 refused
    // cursors over the unsourced span. The pre-floor population this fence once
    // enumerated (1,195,680 occurrences) is not reachable through any
    // identity-backed query, and the counts state that rather than hiding it.
    // The premise itself is unchanged and still asserted per occurrence: what
    // shrank is the population it can be asserted over.
    //
    // The population itself is pinned off the two enums, so a filter that
    // quietly starts dropping identities fails here as well as in the ledger:
    // `Exchange::ALL` is compared element-by-element with the handwritten
    // `ALL_EXCHANGES` / `EXCHANGE_VARIANT_COUNT`
    // (`tests/contract/session_invariants/identity_expectations.rs`), every
    // `MarketHoursKey` the ledger names must resolve, and the verification
    // ledger states one row per identity. This count is that ledger's 132 rows
    // less the four sourced trade-date conventions the sweep excludes.
    assert_eq!(
        close_dated_calendars().len(),
        128,
        "132 ledger identities less this fence's four excluded conventions"
    );
    assert_eq!(
        swept_identities,
        close_dated_calendars().len(),
        "every close-dated identity must ship a session inside the span"
    );
    assert!(
        occurrences > 30_000,
        "the sweep must cover every occurrence the identities source, saw {occurrences}"
    );
    assert!(
        refused_cursors > 100_000,
        "the sweep must step over the unsourced span rather than stopping at it, \
         saw {refused_cursors} refused cursors"
    );
    assert!(
        unresolved > 3_000,
        "a session whose trade date the identity withholds must be counted as a \
         refusal, never as a session without a trade date, saw {unresolved}"
    );
    // `dated_identities` is the observable part of the premise at this head: an
    // identity whose holiday layer has no answer for a day the derivation needs
    // refuses the trade-date probe, and those occurrences are counted as
    // `unresolved` refusals above rather than as sessions without a trade date.
    // Only an always-open profile answers `None`, which the per-identity
    // `dated == 0 || undated == 0` assertion above is what forbids.
    assert!(
        dated_identities >= 25,
        "the identities whose sourced occurrences carry trade dates are the \
         premise's observable population; the bound is the count observed at \
         this head, saw {dated_identities}"
    );
}

/// The identities whose trade date is the close-date default — every identity
/// the self-dated narrowing applies to.
///
/// The three sourced conventions — SET Thailand's prior opening date, CBOT
/// Rough Rice's following local date, and the cryptocurrency and `ECBTC`
/// business-date roll — keep the close walk's full window, so they are the
/// calendars this fence does not speak for.
fn close_dated_calendars() -> Vec<(String, ExchangeCalendar)> {
    every_calendar()
        .into_iter()
        .filter(|(_, calendar)| {
            !matches!(
                calendar.source(),
                CalendarSource::Exchange(Exchange::SetThailand)
                    | CalendarSource::MarketHoursKey(
                        MarketHoursKey::GlobexRoughRice
                            | MarketHoursKey::GlobexCryptocurrency
                            | MarketHoursKey::GlobexEventContractsBtc
                    )
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The clip path a built-in row will take, driven by a caller's DayPolicy.
// ---------------------------------------------------------------------------

/// An early close is stated on the **trade date**, so it lands on the correct
/// civil day for a session that opened the previous evening, and an occurrence
/// that would begin after the cutoff disappears rather than inverting.
///
/// This is the design memo's §1.3 table, on the family it was written for:
/// `globex_equity_index`'s Sunday-to-Thursday 17:00 CT leg wraps to 08:30 CT,
/// the regular session runs 08:30-15:15 CT, and a 15:15-16:00 CT extended leg
/// closes the trading day.
#[test]
fn an_early_close_clips_a_trading_day_that_opened_the_previous_evening() {
    let overrides = [DayOverride::early_close(
        day(2025, 11, 28),
        12 * 3_600 + 15 * 60,
    )];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    // Before the cutoff, inside the regular session.
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    // At the cutoff: closes are end-exclusive.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The remainder of the regular session is gone.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // So is the 15:15-16:00 CT leg, which opens after the cutoff: it is
    // dropped rather than inverted.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The trading day now ends at the cutoff.
    assert_eq!(
        calendar
            .candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // The Thursday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(
        calendar
            .is_open(ct((2025, 11, 27), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The next session is Sunday's reopen, not a same-day remainder.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 11, 28), (12, 20, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 30), (17, 0, 0)))
    );
}

/// A closed trade date removes its complete trading day, including the leg that
/// opened the previous evening — and leaves the next trade date's leg, which
/// opens on the holiday itself, alone.
#[test]
fn a_closed_trade_date_removes_the_previous_evenings_wrap() {
    let overrides = [DayOverride::closed(day(2025, 12, 25))];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(
        calendar
            .is_closed_trade_date(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    // Wednesday evening fed trade date Thursday; it is gone.
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
    // Thursday evening feeds trade date Friday; it is untouched.
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 12, 24), (16, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 12, 25), (17, 0, 0)))
    );
    // The civil day is not wholly closed, because the next trade date's
    // session opens inside it. `is_closed_trade_date` is the holiday question.
    assert!(
        !calendar
            .is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

/// A late open's wall clock is disambiguated against the trading day's own
/// first open, and both branches must be exercised: the branch choice is
/// data-dependent and a flip is a silent 24-hour error.
#[test]
fn a_late_open_resolves_on_both_of_its_branches() {
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    // Branch one: 19:00 CT is at or after the trading day's normal 17:00 CT
    // first open, so the cutoff lands on the **preceding** local date.
    let evening = [DayOverride::late_open(day(2025, 12, 26), 19 * 3_600)];
    let policy = StaticDayPolicy::new(&evening).expect("the fixture records are valid");
    let calendar = base.with_day_policy(&policy);
    assert!(
        base.is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (19, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), (19, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );

    // Branch two: 09:00 CT is before that first open, so the cutoff lands on
    // the trade date itself and the whole evening leg goes.
    let morning = [DayOverride::late_open(day(2025, 12, 26), 9 * 3_600)];
    let policy = StaticDayPolicy::new(&morning).expect("the fixture records are valid");
    let calendar = base.with_day_policy(&policy);
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (20, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 26), (8, 45, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 26), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The regular session now opens at the cutoff. It is its own session --
    // this family does not join adjacent phases -- so the 15:15 CT regular
    // close still bounds it.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 12, 26), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 12, 26), (9, 0, 0)),
            ct((2025, 12, 26), (15, 15, 0))
        ))
    );
}

/// One trade date can carry both a late open and an early close, and their
/// numeric order is deliberately unconstrained: a wrapped trading day opens on
/// the preceding local date at a numerically later wall clock — 19:00 — than
/// its final close on the trade date — 12:00.
///
/// This is why the `holidays!` instant fence checks each boundary's range and
/// **not** `open < close`: a fence on their order would reject the sourced
/// shape this test encodes.
#[test]
fn a_late_open_and_an_early_close_compose_on_one_trade_date() {
    let overrides = [DayOverride::late_open_and_early_close(
        day(2025, 12, 26),
        19 * 3_600,
        12 * 3_600,
    )];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (19, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 26), (11, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 26), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The evening leg opens at the late-open cutoff on the preceding local
    // date and still hands over at 08:30 CT.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 12, 25), (20, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 12, 25), (19, 0, 0)),
            ct((2025, 12, 26), (8, 30, 0))
        ))
    );
    // The regular session is clipped by the early close on the trade date.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 12, 26), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 12, 26), (8, 30, 0)),
            ct((2025, 12, 26), (12, 0, 0))
        ))
    );
}

// ---------------------------------------------------------------------------
// Layering and precedence.
// ---------------------------------------------------------------------------

/// A caller's replacement record resolves the trading day before anything
/// clips it, and the caller's own `DayPolicy` then overlays the replacement
/// exactly as it overlays a normal week.
#[test]
fn a_caller_replacement_resolves_the_day_before_a_clip_applies() {
    let source = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    let blocks = [exchange_hours::ExceptionBlock::regular(
        0,
        9 * 3_600,
        14 * 3_600,
    )];
    let records = [SessionExceptionRecord::replace_sessions(
        day(2026, 4, 20),
        &blocks,
    )];
    let table = StaticSessionExceptions::new(source, day(2026, 4, 1), day(2026, 4, 30), &records)
        .expect("the fixture table is valid");
    let overrides = [DayOverride::early_close(day(2026, 4, 20), 11 * 3_600)];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");

    let replaced = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    assert_eq!(
        replaced
            .session_bounds(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 4, 20), (9, 0, 0)), ct((2026, 4, 20), (14, 0, 0))))
    );

    let clipped = replaced.with_day_policy(&policy);
    assert_eq!(
        clipped
            .session_bounds(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 4, 20), (9, 0, 0)), ct((2026, 4, 20), (11, 0, 0)))),
        "the policy clips the replacement, and never widens it"
    );
    assert!(
        !clipped
            .is_open(ct((2026, 4, 20), (11, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

/// `KnownNormal` is not an assertion that the layers below are wrong.
///
/// A provider returns `KnownNormal` both for a date it audited and found normal
/// and for a covered date it holds no record for, so the two are
/// indistinguishable at the trait level. Treating it as suppression would
/// silently disable every layer below for any caller who attaches a provider.
/// Suppression is `ReplaceSessions`; the coarse undo is `without_holidays`.
#[test]
fn a_known_normal_record_suppresses_nothing_below_it() {
    let source = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    let records = [SessionExceptionRecord::known_normal(day(2026, 4, 20))];
    let table = StaticSessionExceptions::new(source, day(2026, 4, 1), day(2026, 4, 30), &records)
        .expect("the fixture table is valid");
    let overrides = [DayOverride::closed(day(2026, 4, 20))];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");

    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar")
        .with_day_policy(&policy);

    assert!(
        calendar
            .is_closed_trade_date(day(2026, 4, 20), SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "an audited-normal record must not veto the layer below it"
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

/// An out-of-range boundary makes a trade date **unavailable**, which is not
/// the same as closing it.
///
/// An invalid record is not evidence that the operator was shut, so it must not
/// feed the following-business-day roll: CME cryptocurrency keeps assigning its
/// continuous trading to that date rather than rolling past it. A closed record
/// on the same date does roll.
#[test]
fn an_out_of_range_boundary_is_unavailable_and_never_rolls_a_trade_date() {
    struct OutOfRange(NaiveDate);

    impl DayPolicy for OutOfRange {
        fn is_closed(&self, _trade_date: NaiveDate) -> bool {
            false
        }

        fn early_close_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
            (trade_date == self.0).then_some(86_401)
        }
    }

    // 2026-08-21 is a Friday inside the permanent 24/7 cryptocurrency era, and
    // its trading is continuous: the Thursday-evening block's trade date is
    // what moves when a layer closes the Friday. The date carries no built-in
    // row, so the only layer under test is the caller's.
    let friday = day(2026, 8, 21);
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    let probe = ct((2026, 8, 20), (20, 0, 0));
    assert_eq!(base.holiday_on(friday), None);
    assert_eq!(
        base.trade_date(probe)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 8, 21))
    );

    let invalid = OutOfRange(friday);
    let unavailable = base.with_day_policy(&invalid);
    assert!(
        !unavailable
            .is_open(probe)
            .expect("the coverage contract must answer a covered date"),
        "an unavailable trade date takes its sessions with it"
    );
    // An invalid record must not roll a trade date. The identity covers the
    // day, and the boundary the policy states is outside `u32` seconds-in-day,
    // so the date is **unavailable**: its sessions are removed and there is no
    // trade date left to name. What the query must not do is name the rolled
    // date — that is the closed-record leg below, which reports Monday
    // 2026-08-24 for the same probe, so the roll the invalid record must not
    // perform is one this engine does perform when the record is a closure.
    assert_eq!(
        unavailable
            .trade_date(probe)
            .expect("the coverage contract must answer a covered date"),
        None,
        "an unavailable trade date rolls nowhere, so it names no date",
    );

    let overrides = [DayOverride::closed(friday)];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let closed = base.with_day_policy(&policy);
    assert!(
        closed
            .is_open(probe)
            .expect("the coverage contract must answer a covered date"),
        "the roll exists so a closure deletes a trade date, never a day of trading"
    );
    assert_eq!(
        closed
            .trade_date(probe)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 8, 24)),
        "a closed record rolls the continuous week to the next open business date"
    );
}
