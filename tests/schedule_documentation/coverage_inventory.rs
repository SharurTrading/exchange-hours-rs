// SPDX-License-Identifier: MIT-0

//! Fence for `docs/schedules/coverage-2025.md`, the Stage 1 coverage inventory.
//!
//! The inventory's holiday columns are re-derived here from the shipped tables
//! through the **public** query surface, so a row that lands, a window that
//! moves or a kind that changes fails the inventory until its cells are
//! corrected. It is the contract the ledger's `Holidays` column already
//! carries, extended to the inventory's two count columns.
//!
//! The count columns are **trade dates** this scope answers for, which is what
//! completeness is a claim about. One row per date is the shape of every table
//! shipping today, so the date count is also the row count; the fence does not
//! assume that, it counts whatever the surface returns.
//!
//! Only a **routed** table is counted. A module may hold several — `ice_us.rs`
//! carries the served venue table beside five dormant family tables — and a
//! count taken over the whole file reports those dormant rows under the served
//! identity. Going through the identity's own calendar is what keeps the two
//! apart, and it is why this fence queries rather than reading module text.

use super::VERIFICATION;
use chrono::{Datelike as _, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US::Central;
use exchange_hours::{
    CalendarQueryError, CoverageGap, CoverageGapReason, DateCoverage, Exchange, ExchangeCalendar,
    Holiday, HolidayKind, MarketHoursKey, calendar_for_exchange, calendar_for_market_hours_key,
};

const INVENTORY: &str = include_str!("../../docs/schedules/coverage-2025.md");

/// The one venue-local date the completeness verdicts are compared on.
///
/// It has to be a date the inventory's own columns say is *inside* every scope's
/// audit — not a withheld `Unsourced` date and not before a window opens — or the
/// comparison would be measuring something other than the verdict. 2025-06-10 is
/// a **Tuesday** inside every shipped audited window (`2025-01-01..2027-12-31`
/// reaches it for the fourteen scopes that answer 2025, and the two that open
/// later are asserted below to answer it as incomplete anyway), and it is not a
/// date any served table withholds:
/// `inventory_sample_date_is_inside_every_scopes_audit` re-derives both claims
/// from the shipped tables rather than resting on this comment.
const SAMPLE: (i32, u32, u32) = (2025, 6, 10);

/// The support floor LAW-COVERAGE fixes at 2025-01-01.
fn floor() -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 1, 1).expect("2025-01-01 is a valid date")
}

/// The sample date the `Complete?` cells are compared against.
fn sample() -> NaiveDate {
    NaiveDate::from_ymd_opt(SAMPLE.0, SAMPLE.1, SAMPLE.2).expect("the sample is a valid date")
}

/// Splits one Markdown table row into trimmed cells.
fn cells(row: &str) -> Vec<&str> {
    row.trim()
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

/// The inventory table's data rows as `(wire name, cells)`, in document order.
fn inventory_rows() -> Vec<(String, Vec<String>)> {
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in INVENTORY.lines() {
        if line.starts_with("| Identity | Owner |") {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        if !line.starts_with('|') {
            break;
        }
        let parsed = cells(line);
        if parsed.first().is_some_and(|cell| cell.starts_with("---")) {
            continue;
        }
        let name = parsed[0].trim_matches('`').to_owned();
        rows.push((name, parsed.into_iter().map(str::to_owned).collect()));
    }
    assert!(
        rows.len() > 1,
        "the coverage inventory must carry one row per served scope"
    );
    rows
}

/// The served wire names, in ledger order (LAW-SERVICE-TIERS).
fn served_names() -> Vec<String> {
    VERIFICATION
        .lines()
        .filter(|line| line.starts_with('|'))
        .filter_map(|line| {
            let parsed = cells(line);
            (parsed.len() >= 12 && parsed[5] == "served")
                .then(|| parsed[0].trim_matches('`').to_owned())
        })
        .collect()
}

/// The public calendar for a wire name, whichever identity enum owns it.
fn calendar_for(name: &str) -> Option<ExchangeCalendar> {
    if let Ok(exchange) = name.parse::<Exchange>() {
        return Some(calendar_for_exchange(exchange));
    }
    name.parse::<MarketHoursKey>()
        .ok()
        .map(calendar_for_market_hours_key)
}

/// Counts, from the floor through `last`, the trade dates carrying a row and
/// the trade dates whose row withholds the answer as `Unsourced`.
///
/// A date outside every coverage window and a date inside one that was audited
/// normal both answer `None`, so the walk counts rows without needing to know
/// where the windows fall.
fn count_from_floor(calendar: ExchangeCalendar, last: NaiveDate) -> (usize, usize) {
    let (mut dated, mut unsourced) = (0_usize, 0_usize);
    let mut date = floor();
    while date <= last {
        if let Some(holiday) = calendar.holiday_on(date) {
            dated += 1;
            if holiday.kind() == HolidayKind::Unsourced {
                unsourced += 1;
            }
        }
        match date.succ_opt() {
            Some(next) => date = next,
            None => break,
        }
    }
    (dated, unsourced)
}

/// Reads the count from an inventory count cell: `41`, or the em dash that
/// means none. Every served table states one row per date, so no cell carries
/// a bracketed secondary count.
fn leading(cell: &str) -> Option<usize> {
    if cell.starts_with('\u{2014}') {
        return Some(0);
    }
    cell.split_whitespace()
        .next()
        .and_then(|count| count.parse().ok())
}

#[test]
fn inventory_membership_is_exactly_the_served_ledger_rows() {
    let expected = served_names();
    let actual: Vec<String> = inventory_rows().into_iter().map(|(name, _)| name).collect();
    assert_eq!(
        actual, expected,
        "the coverage inventory must carry exactly the ledger's served identities, in its order"
    );
}

#[test]
fn inventory_windows_and_date_counts_match_the_shipped_tables() {
    for (name, row) in inventory_rows() {
        let calendar = calendar_for(&name).expect("the inventory names a known identity");
        let coverage = calendar
            .holiday_coverage()
            .expect("a served identity ships a holiday table");
        let windows = coverage
            .windows()
            .iter()
            .map(|(first, last)| format!("{first}..{last}"))
            .collect::<Vec<_>>()
            .join(", ");
        assert_eq!(row[4], windows, "Holidays cell for {name}");
        let (dated, unsourced) = count_from_floor(calendar, coverage.last());
        assert_eq!(
            leading(&row[5]).expect("a 2025+ dates cell"),
            dated,
            "2025+ trade dates carrying a holiday row, for {name}"
        );
        assert_eq!(
            leading(&row[6]).expect("an Unsourced cell"),
            unsourced,
            "2025+ trade dates withheld as Unsourced, for {name}"
        );
    }
}

/// A row that names a tracked issue in its `Missing / disputed` cell has an
/// unresolved gap, and LAW-COVERAGE does not let that row read as complete.
///
/// The inventory's own definition is "no unresolved normal-week, required-phase,
/// holiday or special-session gap in the claimed interval", so a scope carrying
/// the #79 quarter-hour or the #93 special sessions cannot say `complete to ...`
/// however far its coverage window reaches. A horizon that simply stops before
/// the inspection date is not a tracked gap and is left alone.
#[test]
fn a_row_recording_a_tracked_gap_does_not_claim_completeness() {
    for (name, row) in inventory_rows() {
        let (missing, verdict) = (&row[7], &row[8]);
        if !missing.contains('#') {
            continue;
        }
        assert!(
            verdict.contains("incomplete"),
            "{name} records a tracked gap ({missing}) but its verdict reads {verdict}"
        );
    }
}

/// The `Complete?` cell the metadata's per-date verdict must agree with.
///
/// The inventory states the verdict in prose, so it is read in prose: a cell
/// beginning `complete to` claims completeness over the interval it names, and a
/// cell reading `**incomplete**` or `**no 2025 coverage**` denies it. A cell in
/// neither shape is a third verdict this fence does not know how to check, and
/// it must be added deliberately rather than skipped.
enum Verdict {
    /// `complete to <date>`: this scope answers the sample date completely.
    Complete,
    /// `**no 2025 coverage**`: the scope's audit opens after the sample date, so
    /// it cannot answer it at all.
    NoCoverage,
    /// `**incomplete**`: the scope denies the interval verdict, and this fence
    /// re-derives which of the two statements about the *sample date* that
    /// implies — see the test that uses this.
    Incomplete,
}

/// Reads the `Complete?` cell, naming any cell this fence does not understand.
fn verdict_of(name: &str, cell: &str) -> Verdict {
    if cell.starts_with("complete to") {
        return Verdict::Complete;
    }
    if cell.starts_with("**no 2025 coverage**") {
        return Verdict::NoCoverage;
    }
    if cell.starts_with("**incomplete**") {
        return Verdict::Incomplete;
    }
    panic!("{name}'s Complete? cell reads {cell:?}, which is neither shape this fence knows")
}

/// Whether this scope's built-in table withholds `date` as `Unsourced`.
fn withheld(calendar: ExchangeCalendar, date: NaiveDate) -> bool {
    calendar
        .holiday_on(date)
        .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
}

/// The scopes whose `**incomplete**` verdict is entirely **date-level**: a set of
/// `Unsourced` trade dates the routed families dispute, with no phase-level gap
/// behind it.
///
/// The map is keyed by scope and carries the inventory's own withheld count, so
/// the claim "this scope's incompleteness is date-shaped" is compared against the
/// page rather than asserted by this list. It is a fence only while the
/// `Unsrc 2025+ dates` column it is compared against is re-derived from the
/// shipped tables — which `inventory_windows_and_date_counts_match_the_shipped_tables`
/// does in the same file. `cme` is deliberately absent: it withholds 48 dates
/// **and** the Sunday quarter-hour (#79), so its denial of completeness is no
/// longer date-shaped. `iceus` is the second entry: from 2026-09-26 UTC it audits
/// from the 2025 floor and withholds 34 dates its routed families dispute, with
/// no phase-level gap behind the denial.
fn date_level_incompleteness() -> &'static [(&'static str, usize)] {
    &[("cbot", 61), ("iceus", 34)]
}

/// `is_complete_on(SAMPLE)` agrees with the inventory's `Complete?` cell for all
/// sixteen served scopes.
///
/// The page and the API are one record, and this is the test that stops them
/// disagreeing. The defect it was added for is why it is not a loop over the cell
/// text: `is_complete_on(2025-06-10)` returned `true` for `globex_equity_index`,
/// `globex_fx` and `globex_cryptocurrency` — three scopes the same page calls
/// incomplete — because a **phase-level** gap is invisible to a date walk over
/// the identity's tables. `cme`, `comex`, `nymex`, `globex_energy` and
/// `globex_interest_rates` each withheld the Sunday 16:00-16:15 CT quarter-hour
/// their own ledger basis notes record, and this fence is what made the correction
/// to the page and the metadata land together — but only **four** of the five had
/// a cell reading complete: `cme`'s already denied it, for the 48 dates it
/// withholds as `Unsourced` in 2025+, so the quarter-hour corrected that scope's
/// cause of incompleteness rather than its verdict.
///
/// The cell is an interval verdict and the API is a per-date verdict, so the rule
/// is stated in two parts rather than assumed to be one-to-one:
///
/// 1. **Direction.** `complete to …` must be `true` and `**incomplete**` /
///    `**no 2025 coverage**` must be `false` on the sample date, except for the
///    scopes in [`date_level_incompleteness`], whose denial names dates other
///    than the sample. The sample is a date no served table withholds
///    (`inventory_sample_date_is_inside_every_scopes_audit`), so no other scope
///    has that excuse.
/// 2. **Cause.** Every direction that came out `false` must be explained by the
///    metadata: a declared phase-level gap, a gap span containing the sample, or
///    the table withholding it. A scope cannot read incomplete for a reason the
///    API does not report.
///
/// Between them the phase-gap scopes fail on the page's own words before their
/// declarations exist, and the metadata must carry a cause that accounts for
/// every denial.
#[test]
fn inventory_completeness_verdicts_match_the_metadata() {
    let day = sample();
    let mut complete = 0_usize;
    let mut incomplete = 0_usize;
    let mut no_coverage = 0_usize;
    for (name, row) in inventory_rows() {
        let calendar = calendar_for(&name).expect("the inventory names a known identity");
        let coverage = calendar.coverage();
        let cell = &row[8];
        let date_level = date_level_incompleteness()
            .iter()
            .find(|(scope, _)| *scope == name)
            .copied();

        let expected = match verdict_of(&name, cell) {
            Verdict::Complete => {
                complete += 1;
                true
            }
            Verdict::NoCoverage => {
                no_coverage += 1;
                false
            }
            Verdict::Incomplete => {
                incomplete += 1;
                if let Some((_, claimed)) = date_level {
                    // The page's own withheld count must be the one the shipped
                    // table produces, or this scope is in the wrong group.
                    assert_eq!(
                        leading(&row[6]).expect("an Unsourced cell"),
                        claimed,
                        "{name} is listed as date-level incomplete with {claimed} withheld dates"
                    );
                    !withheld(calendar, day)
                } else {
                    false
                }
            }
        };
        assert_eq!(
            coverage.is_complete_on(day),
            expected,
            "{name}: the inventory's Complete? cell reads {cell:?}, so is_complete_on({day}) must \
             be {expected}"
        );

        // Every "not complete" the inventory states is one the metadata reports a
        // cause for: a declared phase-level gap, a gap span covering the sample,
        // or the table withholding it.
        if !expected {
            let declared = !coverage.phase_gaps().is_empty();
            let contained = coverage.gaps().any(|gap| gap.range().contains(day));
            assert!(
                declared || contained || withheld(calendar, day),
                "{name} is called incomplete on {day}, but the metadata reports no declared \
                 phase-level gap, no gap span containing that date and no withheld row"
            );
        }
    }
    assert_eq!(
        (complete, incomplete, no_coverage),
        (3, 13, 0),
        "the inventory's verdict shapes: three complete, thirteen incomplete, none with no 2025 \
         coverage"
    );
}

/// Every declared phase-level gap, by served wire name: the reason it records
/// and the issue its own evidence names as the closing condition.
///
/// A served scope with no entry declares none. The table is compared against the
/// page's `Missing / disputed` and `Closing issues` cells below, so neither
/// record can move without the other, and the shipped profiles are checked
/// against it in `the_sunday_quarter_hour_is_declared_exactly_where_the_profiles_withhold_it`.
///
/// `globex_cryptocurrency`'s remaining gap is the one entry whose issue number the
/// evidence file did not supply: `docs/evidence/globex_cryptocurrency.md` records
/// the undated five-day-era Pre-Open onset and its closing condition ("a CME
/// artifact that states the Pre-Open in session language on a day-level effective
/// date"), but said only that the gap is "tracked as an issue" and named no
/// number. #123 was opened for it, and the declaration cites that, so
/// LAW-FOLLOW-UPS-ARE-ISSUES is discharged rather than waived. The
/// `SpecialSessionUnrepresentable` shape #93 named has no entry at all since
/// 2026-09-26 UTC: `globex_cryptocurrency`'s eight 24/7-era merged trade dates
/// shipped as `replacement blocks` rows, so no served scope declares it.
fn declared_phase_gaps() -> Vec<(&'static str, Vec<(CoverageGapReason, &'static str)>)> {
    let quarter_hour = (CoverageGapReason::NormalWeekPhaseWithheld, "#79");
    let pre_open_onset = (CoverageGapReason::NormalWeekPhaseWithheld, "#123");
    let post_close_label = (CoverageGapReason::PostCloseQueueTradeDateLabel, "#152");
    let undated_closures = (CoverageGapReason::UnpublishedClosureDates, "#157");
    vec![
        ("cme", vec![quarter_hour]),
        ("comex", vec![quarter_hour]),
        ("nymex", vec![quarter_hour]),
        ("globex_energy", vec![quarter_hour]),
        ("globex_equity_index", vec![quarter_hour]),
        // `globex_fx` carried the #93 special-session declaration until its merged
        // trade dates landed; every session CME publishes for it now ships as a row.
        ("globex_fx", vec![quarter_hour]),
        ("globex_interest_rates", vec![quarter_hour]),
        // `globex_cryptocurrency` carried the #93 special-session declaration
        // until its 24/7-era merged trade dates landed on 2026-09-26 UTC;
        // every session CME publishes for it now ships as a row, and the
        // declared Pre-Open onset is bounded at the 2026-05-29 bridge row.
        ("globex_cryptocurrency", vec![pre_open_onset]),
        // The two scopes whose declaration serves its phase: the post-close
        // queue is answered on every covered date, and only the trade date it
        // reads under is the crate's convention rather than the operator's
        // printing (#152).
        ("globex_grains", vec![post_close_label]),
        ("globex_livestock", vec![post_close_label]),
        // `eurex` withholds no *phase*: the operator declares German
        // equity/equity-index closures it has not dated, so the declaration is
        // a completeness fact the date walk cannot find and the order-entry
        // scans still answer through it.
        ("eurex", vec![undated_closures]),
    ]
}

/// The twelve scopes that declare a gap declare exactly the ones
/// advertised, each reportable with its own reason and closing issue, and each
/// issue is one the scope's own row names.
///
/// `inventory_completeness_verdicts_match_the_metadata` compares the metadata
/// against the page, so it would pass if the page were edited to agree with a
/// wrong API. This names the scopes and what each declares, so neither side can
/// move silently.
/// Asserts one scope's declaration records match what it declares.
///
/// A declaration the per-date accessor names on some date has a record, carrying
/// its own reason and closing condition over the span it is the answer for: the
/// era before its bound for a bounded declaration, the whole domain for one that
/// carries no bound. Every served scope declares one gap today, so the records
/// and the declarations correspond one to one; the helper still reads the
/// `phase_gaps` stack rather than assuming that, because a scope that stacks two
/// would report only the first through `gaps`.
fn check_declaration_records(
    name: &str,
    coverage: exchange_hours::CalendarCoverage,
    expected: &[(CoverageGapReason, &str)],
    row: &[String],
) {
    let gaps: Vec<CoverageGap> = coverage.gaps().collect();
    let declared: Vec<CoverageGap> = gaps
        .iter()
        .filter(|gap| gap.phase_gap().is_some())
        .copied()
        .collect();
    assert!(
        declared.len() <= expected.len(),
        "{name} reports no more declaration records than it declares"
    );
    for (reason, closing) in expected {
        let Some(gap) = declared
            .iter()
            .find(|gap| gap.closing_condition() == Some(*closing))
        else {
            let shadowed = coverage.phase_gaps().iter().any(|candidate| {
                candidate.closing_condition() == *closing && candidate.applies_until().is_none()
            }) && coverage.phase_gaps()[0].closing_condition() != *closing;
            assert!(
                shadowed,
                "{name} must report a record for {closing} unless an earlier declaration \
                 applies to every date it does"
            );
            continue;
        };
        assert_eq!(gap.reason(), *reason, "{name}'s {closing} record");
        let declaration = coverage
            .phase_gaps()
            .iter()
            .find(|candidate| candidate.closing_condition() == *closing)
            .expect("the fixture names a declared gap");
        assert!(
            gap.range().first() >= floor(),
            "{name}'s {closing} record starts at or after the floor"
        );
        assert_eq!(
            gap.range().is_open_ended(),
            declaration.applies_until().is_none(),
            "{name}'s {closing} record must be open-ended exactly while its declaration is"
        );
        assert!(
            declaration.applies_on(gap.range().first()),
            "{name}'s {closing} record must start where its declaration applies"
        );
        assert!(
            row[7].contains(closing) || row[9].contains(closing),
            "{name}'s Missing / disputed and Closing issues cells must name {closing}: {:?} / {:?}",
            row[7],
            row[9]
        );
    }
    assert!(
        gaps.iter().all(|gap| gap.range().first() >= floor()),
        "{name}: no record starts before the floor"
    );
}

#[test]
fn the_declared_phase_level_gaps_match_the_inventory() {
    let day = sample();
    let expected_table = declared_phase_gaps();
    let rows = inventory_rows();
    let mut declaring = 0_usize;
    let mut declarations = 0_usize;
    for (name, row) in &rows {
        let coverage = calendar_for(name)
            .expect("the inventory names a known identity")
            .coverage();
        let expected = expected_table
            .iter()
            .find(|(scope, _)| scope == name)
            .map_or_else(Vec::new, |(_, gaps)| gaps.clone());
        let actual: Vec<(CoverageGapReason, &str)> = coverage
            .phase_gaps()
            .iter()
            .map(|gap| (gap.reason(), gap.closing_condition()))
            .collect();
        assert_eq!(actual, expected, "{name}");
        if expected.is_empty() {
            continue;
        }
        declaring += 1;
        declarations += expected.len();

        // The horizon the ledger declares is untouched: a phase gap is additional
        // information, not a re-dating. Whether the identity answers any date
        // completely follows from the declarations: one that carries a
        // whole-domain gap answers none, and one whose gaps are all bounded by an
        // era answers from the latest bound on.
        let whole_domain = coverage
            .phase_gaps()
            .iter()
            .any(|gap| gap.applies_until().is_none());
        assert_eq!(
            coverage.complete_ranges().count() == 0,
            whole_domain,
            "{name}: complete_ranges() is empty exactly while a whole-domain gap applies"
        );
        if whole_domain {
            assert!(
                !coverage.is_complete_on(day),
                "{name} carries a whole-domain phase-level gap, so no date is complete"
            );
        }

        check_declaration_records(name, coverage, &expected, row);
    }
    assert_eq!(
        (declaring, declarations),
        (11, 11),
        "eleven served scopes declare a gap today, one declaration each: seven \
         quarter-hour scopes, `globex_cryptocurrency`'s undated five-day-era Pre-Open \
         onset, `eurex`'s undated closure scope, and the post-close queue label \
         `globex_grains` and `globex_livestock` declare"
    );

    // The scopes the quarter-hour probe cleared of the disputed window declare
    // nothing at all: a declaration must not leak onto a profile whose grid has
    // no session there. Whether each is complete is the `Complete?` cell's
    // question, answered by the test above.
    for name in [
        "cbot",
        "cfe",
        "coinbase_derivatives",
        "iceus",
        "globex_nikkei_225_dollar",
    ] {
        assert!(
            calendar_for(name)
                .expect("the fixture names a served scope")
                .coverage()
                .phase_gaps()
                .is_empty(),
            "{name} declares no phase-level gap"
        );
    }
}

/// 16:05 or 16:20 CT on Sunday 2025-06-08, the instants the quarter-hour probe
/// uses: 16:05 falls inside CME's disputed 16:00-16:15 CT window and 16:20
/// inside the sourced 16:15-17:00 intersection it is served from.
fn chicago(hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Central
        .with_ymd_and_hms(2025, 6, 8, hour, minute, 0)
        .single()
        .expect("2025-06-08 is a Sunday with one 16:05/16:20 CT instant")
        .with_timezone(&Utc)
}

/// 16:05 CT on one venue-local date, for the era probe below.
fn chicago_on(date: NaiveDate) -> chrono::DateTime<Utc> {
    Central
        .from_local_datetime(
            &date
                .and_hms_opt(16, 5, 0)
                .expect("16:05 is a valid local time"),
        )
        .single()
        .expect("the era probe uses Sundays with one 16:05 CT instant")
        .with_timezone(&Utc)
}

/// Asserts the identity-backed order-acceptance answer is exactly the verdict
/// the identity publishes for `date` (LAW-COVERAGE).
///
/// The served quarter-hour answers as an acceptance where the scope declares no
/// remaining gap; a scope still withholding the date refuses with the error its
/// own `coverage_on` names. This keeps the era fence honest without assuming
/// that #79 is every scope's only declaration: `globex_cryptocurrency` carries an
/// unbounded `#93` too, so the same 16:05 CT instant answers for one scope and
/// refuses for the other, and both are correct.
fn assert_acceptance_matches_coverage(
    calendar: ExchangeCalendar,
    instant: chrono::DateTime<Utc>,
    date: NaiveDate,
    label: &str,
) {
    let answer = calendar.is_accepting_orders(instant);
    let source = calendar.source();
    let verdict = calendar.coverage().coverage_on(date);
    // `DateCoverage` is `#[non_exhaustive]`; a verdict this fence does not know
    // cannot be mapped to an expected answer, so it fails here rather than
    // letting the comparison below pass by accident.
    assert!(
        matches!(
            verdict,
            DateCoverage::Covered
                | DateCoverage::BeforeSupportFloor
                | DateCoverage::UnresolvedGap
                | DateCoverage::OutsideCoveredRange
        ),
        "{label}: unrecognised coverage verdict {verdict:?}, got {answer:?}"
    );
    let expected = match verdict {
        DateCoverage::Covered => Ok(true),
        DateCoverage::BeforeSupportFloor => {
            Err(CalendarQueryError::BeforeSupportFloor { source, date })
        }
        DateCoverage::UnresolvedGap => Err(CalendarQueryError::UnresolvedGap { source, date }),
        DateCoverage::OutsideCoveredRange | _ => {
            Err(CalendarQueryError::OutsideCoveredRange { source, date })
        }
    };
    assert_eq!(
        answer, expected,
        "{label}: the query must state the verdict its identity publishes"
    );
}

/// A venue-local date for the era probe.
fn day(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("the era probe uses valid dates")
}

/// The Sundays from `first` through `last`, inclusive.
fn sundays_between(first: NaiveDate, last: NaiveDate) -> Vec<NaiveDate> {
    let mut sundays = Vec::new();
    let mut date = first;
    while date <= last {
        if date.weekday() == Weekday::Sun {
            sundays.push(date);
        }
        date = date.succ_opt().expect("the probe window is bounded");
    }
    assert_eq!(
        sundays.first(),
        Some(&first),
        "the probe window starts on its first Sunday"
    );
    sundays
}

/// The withheld Sunday quarter-hour, fenced on the shipped profiles themselves.
///
/// `is_complete_on` is derived from the declarations, so on its own it cannot
/// tell a scope that withholds a required phase from one whose grid simply has no
/// session at 16:05 CT. This fence observes the profiles instead: a scope that
/// withholds the quarter-hour is closed at 16:05 CT and **accepting orders** at
/// 16:20 CT, and exactly the served scopes that show that signature declare #79 - four dormant identities show it too and declare nothing, so the invariant is scoped to the inventory's sixteen rows —
/// `cme`, `comex`, `nymex`, `globex_energy`, `globex_equity_index`, `globex_fx`
/// and `globex_interest_rates`. The three that accept at 16:05 CT are genuinely
/// fine, and the six closed at both instants have a different grid rather than a
/// withheld quarter-hour, so neither group may carry the declaration.
///
/// This is the independent half of the fence the defect needed: it reads the
/// profiles rather than the prose, so a scope silently dropped from the
/// declaration list fails here even if the inventory is edited to match.
#[test]
fn the_sunday_quarter_hour_is_declared_exactly_where_the_profiles_withhold_it() {
    let inside = chicago(16, 5);
    let after = chicago(16, 20);
    let mut withholding = Vec::new();
    let mut accepts_inside = Vec::new();
    let mut closed_at_both = Vec::new();
    for (name, _) in inventory_rows() {
        let calendar = calendar_for(&name).expect("the inventory names a known identity");
        let declares = calendar
            .coverage()
            .phase_gaps()
            .iter()
            .any(|gap| gap.closing_condition() == "#79");
        // The grid this fence reads is the **fixed snapshot's**, because that is
        // the surface which still states it: the identity-backed query refuses
        // the very quarter-hour the declaring scopes withhold (LAW-COVERAGE), so
        // asking it here would measure the refusal rather than the profile. The
        // snapshot carries no identity and no coverage verdict, and
        // `ExchangeCalendar::hours_at` is documented as the identity's sourced
        // profile at the instant.
        let closed_inside = !calendar.hours_at(inside).is_accepting_orders(inside);
        let open_after = calendar.hours_at(after).is_accepting_orders(after);
        assert_eq!(
            declares,
            closed_inside && open_after,
            "{name} is {} at 16:05 CT and {} at 16:20 CT on {inside} ({}), so it must {}declare the \
             withheld Sunday quarter-hour (#79)",
            if closed_inside {
                "closed"
            } else {
                "accepting orders"
            },
            if open_after {
                "accepting orders"
            } else {
                "closed"
            },
            inside.with_timezone(&Central),
            if declares { "" } else { "not " }
        );
        // And the identity-backed answer is asserted for what it now says: a
        // declaring scope refuses the withheld quarter-hour outright, so a
        // caller can never read the withholding as a grid.
        if declares {
            assert_eq!(
                calendar.is_accepting_orders(inside),
                Err(CalendarQueryError::OutsideCoveredRange {
                    source: calendar.source(),
                    date: inside.with_timezone(&Central).date_naive(),
                }),
                "{name} must refuse the withheld Sunday quarter-hour, not answer it"
            );
        }
        if declares {
            withholding.push(name);
        } else if !closed_inside {
            accepts_inside.push(name);
        } else {
            closed_at_both.push(name);
        }
    }
    assert_eq!(
        withholding,
        [
            "cme",
            "comex",
            "nymex",
            "globex_equity_index",
            "globex_energy",
            "globex_fx",
            "globex_interest_rates"
        ],
        "the seven scopes whose Sunday queue withholds the 16:00-16:15 CT quarter-hour"
    );
    assert_eq!(
        accepts_inside,
        ["cbot", "cfe", "globex_grains"],
        "these accept orders inside the disputed window, so no gap is declared for them"
    );
    assert_eq!(
        closed_at_both,
        [
            "coinbase_derivatives",
            "eurex",
            "iceus",
            "globex_livestock",
            "globex_cryptocurrency",
            "globex_nikkei_225_dollar"
        ],
        "these are closed at both instants: a different grid, not a withheld quarter-hour"
    );
}

/// The sample date is a date every scope's audit actually reaches.
///
/// The comparison above is only meaningful if the sample is not itself a
/// withheld `Unsourced` date and falls inside a window the scope audited: either
/// would make a scope read incomplete for a reason that has nothing to do with
/// its `Complete?` cell. Both claims are re-derived here from the shipped
/// tables — the holiday row (if any) and the audited windows — for all sixteen
/// scopes, and the scope count is asserted so a scope cannot drop out.
#[test]
fn inventory_sample_date_is_inside_every_scopes_audit() {
    let day = sample();
    assert_eq!(
        day.format("%A").to_string(),
        "Tuesday",
        "the sample must not be a weekend"
    );
    assert!(
        day > floor(),
        "the sample must be at or after the support floor"
    );

    let mut checked = 0_usize;
    for (name, _) in inventory_rows() {
        let calendar = calendar_for(&name).expect("the inventory names a known identity");
        if calendar
            .holiday_coverage()
            .is_some_and(|coverage| coverage.contains(day))
        {
            assert_ne!(
                calendar.holiday_on(day).map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{name} withholds the sample date as Unsourced"
            );
        }
        checked += 1;
    }
    assert_eq!(
        checked, 16,
        "the inventory carries one row per served scope, and all sixteen are checked"
    );
}

/// One quarter-hour scope's era: the module's own knowledge-bound row, the last
/// Sunday its dated era covers, and the first Sunday its current era serves.
///
/// The bound is not assumed to be one date for all seven: each module was read and
/// each ends its timeline in its own `2026-08-22` knowledge-bound row, recorded
/// here as a cell so a module that moves is a failure rather than a silent
/// disagreement with the declarations in `schedules/sourcing.rs`.
struct QuarterHourEra {
    /// The served wire name.
    name: &'static str,
    /// The first date the module's current profile governs — the last date its
    /// dated profile does not.
    bound: (i32, u32, u32),
    /// The last Sunday before `bound`, when the dated profile withholds the
    /// quarter-hour.
    dated_sunday: (i32, u32, u32),
    /// The first Sunday at or after `bound`, when the current profile serves it.
    current_sunday: (i32, u32, u32),
}

/// The seven scopes that withhold CME's Sunday 16:00-16:15 CT quarter-hour.
///
/// The probe days are Sundays so that 16:05 CT falls inside the Sunday Pre-Open
/// queue the #79 declaration is about: the bound is a Saturday, so `dated_sunday`
/// is the last Sunday of the dated era and `current_sunday` the first the current
/// era serves. Neither Sunday is a US market holiday, so nothing but the era bound
/// can move the verdict between them.
const QUARTER_HOUR_ERAS: [QuarterHourEra; 7] = [
    QuarterHourEra {
        name: "cme",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
    QuarterHourEra {
        name: "comex",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
    QuarterHourEra {
        name: "nymex",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
    QuarterHourEra {
        name: "globex_equity_index",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
    QuarterHourEra {
        name: "globex_energy",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
    QuarterHourEra {
        name: "globex_fx",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
    QuarterHourEra {
        name: "globex_interest_rates",
        bound: (2026, 8, 22),
        dated_sunday: (2026, 8, 16),
        current_sunday: (2026, 8, 23),
    },
];

/// The quarter-hour declaration's era, fenced on **both sides** of the bound.
///
/// `the_sunday_quarter_hour_is_declared_exactly_where_the_profiles_withhold_it`
/// probes one Sunday inside the dated era, so on its own it cannot see the era
/// boundary: a declaration that withheld the quarter-hour across the whole
/// supported domain passes it while denying the current era coverage the profiles
/// serve. This fence is the check that would have caught that. For each of the
/// seven scopes it requires, at 16:05 CT on the last Sunday of the dated era and on
/// the first Sunday of the current one:
///
/// 1. **the declaration's own bound** is its module's knowledge-bound row, read
///    back through [`exchange_hours::PhaseGap::applies_until`], and it applies on
///    the earlier Sunday and not on the later;
/// 2. **the profile agrees**: orders are refused inside the dated era and accepted
///    after it — read from the fixed snapshot, which is the surface that still
///    states a grid the identity withholds, with the identity's own answer
///    asserted beside it (a coverage refusal inside the era, and whatever its
///    published verdict says after the bound);
/// 3. **the metadata agrees with the profile**: the earlier Sunday is outside the
///    covered range for every declaring scope, and after the bound each scope's
///    verdict is the one its own declarations imply - `Covered` where `#79` was
///    the only gap, still outside it for `globex_cryptocurrency`, whose `#93` and
///    `#123` are unbounded - compared date by date across the boundary rather
///    than only at its ends.
///
/// The bound each module carries is a table cell here rather than an assumption, so
/// a module whose row moves fails this fence instead of silently disagreeing with
/// `schedules/sourcing.rs`.
/// The bound day itself, as a metadata statement (issue #124).
///
/// The behavioural walk above covers two Sundays, because 16:05 CT is the phase
/// under test only on a Sunday: on `2026-08-22` — the knowledge-bound row's own
/// day, and a Saturday — and on the six weekday dates after it, a 16:05 CT read
/// measures that date's own sourced closure or the weekend rather than the phase
/// bound. So the bound day was previously unprobed in either direction.
///
/// This asserts the *metadata* instead, which is what the declarations actually
/// change: each scope answers for every date from the floor through its bound,
/// and the day after the bound is the first the crate does not answer. It is a
/// separate statement from the behavioural probe on purpose — a weekend date can
/// never carry a phase reading, so the two questions are genuinely different.
#[test]
fn the_knowledge_bound_day_is_covered_and_the_day_before_it_is_not() {
    let mut checked = 0_usize;
    for era in QUARTER_HOUR_ERAS {
        let QuarterHourEra { name, bound, .. } = era;
        let bound = day(bound.0, bound.1, bound.2);
        let calendar = calendar_for(name).expect("the fixture names a served scope");
        let coverage = calendar.coverage();

        assert_eq!(
            coverage.coverage_on(bound),
            DateCoverage::Covered,
            "{name}: the knowledge-bound row's own day, {bound}, is inside the answered window"
        );
        // The declaration ends *at* the bound, so the bound is the first day the
        // quarter-hour is served; nothing before it resolves through the
        // declaration either, which is what makes the bound the era's edge.
        let declaration = coverage
            .phase_gaps()
            .iter()
            .find(|gap| gap.closing_condition() == "#79")
            .unwrap_or_else(|| {
                panic!("{name} must declare the withheld Sunday quarter-hour (#79)")
            });
        assert!(
            !declaration.applies_on(bound),
            "{name}: the #79 declaration must not apply on its own bound day, {bound}"
        );
        assert!(
            declaration.applies_on(bound - chrono::Duration::days(1)),
            "{name}: the #79 declaration applies on the day before {bound}"
        );
        checked += 1;
    }
    assert_eq!(
        checked,
        QUARTER_HOUR_ERAS.len(),
        "every declaring scope must be checked"
    );
}

#[test]
fn the_sunday_quarter_hour_gap_ends_at_the_knowledge_bound_row() {
    let mut checked = 0_usize;
    for era in QUARTER_HOUR_ERAS {
        let QuarterHourEra {
            name,
            bound,
            dated_sunday,
            current_sunday,
        } = era;
        let bound = day(bound.0, bound.1, bound.2);
        let dated = day(dated_sunday.0, dated_sunday.1, dated_sunday.2);
        let after = day(current_sunday.0, current_sunday.1, current_sunday.2);
        let calendar = calendar_for(name).expect("the fixture names a served scope");
        let coverage = calendar.coverage();

        // 1. The declaration exists, is the quarter-hour one, and carries this
        //    bound — the day its own module's knowledge-bound row begins.
        let declaration = coverage
            .phase_gaps()
            .iter()
            .find(|gap| gap.closing_condition() == "#79")
            .unwrap_or_else(|| {
                panic!("{name} must declare the withheld Sunday quarter-hour (#79)")
            });
        assert_eq!(
            declaration.reason(),
            CoverageGapReason::NormalWeekPhaseWithheld,
            "{name}'s #79 declaration"
        );
        assert_eq!(
            declaration.applies_until(),
            Some(bound),
            "{name}'s #79 declaration must end at its own module's knowledge-bound row, {bound}"
        );
        assert!(
            declaration.applies_on(dated) && !declaration.applies_on(after),
            "{name}: the declaration applies on {dated} and not on {after}"
        );

        // 2. The profiles: shut at 16:05 CT inside the dated era, open after it.
        //
        //    The grid comes from the fixed snapshot, which still states it; the
        //    identity-backed answer on the dated Sunday is the coverage refusal
        //    the withheld quarter-hour now earns, and that refusal is asserted
        //    rather than assumed. After the bound the identity answers, and its
        //    answer is an acceptance.
        let dated_instant = chicago_on(dated);
        assert!(
            !calendar
                .hours_at(dated_instant)
                .is_accepting_orders(dated_instant),
            "{name} must not accept orders at 16:05 CT on {dated}: its dated profile withholds \
             the 16:00-16:15 CT quarter-hour"
        );
        assert_eq!(
            calendar.is_accepting_orders(dated_instant),
            Err(CalendarQueryError::OutsideCoveredRange {
                source: calendar.source(),
                date: dated,
            }),
            "{name} withholds the quarter-hour on {dated}, so its own answer is the coverage \
             refusal and never a closed grid"
        );
        assert!(
            calendar
                .hours_at(chicago_on(after))
                .is_accepting_orders(chicago_on(after)),
            "{name} accepts orders at 16:05 CT on {after}: its {bound} knowledge-bound row \
             widened the Sunday queue to 16:00-17:00 CT"
        );
        // The identity states the widened grid only where #79 was its only
        // declared gap; a scope carrying another declared phase gap still
        // withholds the date, and the answer is then that refusal. The expected
        // answer is driven from the scope's own published coverage rather than
        // assumed, so both cases are stated exactly.
        assert_acceptance_matches_coverage(
            calendar,
            chicago_on(after),
            after,
            &format!("{name} after its {bound} knowledge-bound row"),
        );

        // 3. The metadata follows the profiles on every Sunday across the
        //    boundary, not only at its ends. The comparison is made on Sundays
        //    because 16:05 CT is inside the Sunday Pre-Open queue alone: on any
        //    other weekday the CRITICAL window is long closed, so `is_accepting_orders`
        //    would say nothing about the quarter-hour.
        for sunday in sundays_between(dated, after) {
            let instant = chicago_on(sunday);
            // The profile's own answer, from the surface that states it.
            let served = calendar.hours_at(instant).is_accepting_orders(instant);
            // The identity's answer, driven from its own published verdict for
            // the date: the served quarter-hour answers (a scope with no other
            // declared gap accepts orders; `globex_cryptocurrency`'s unbounded
            // `#93` still withholds the date), and the withheld quarter-hour
            // before the bound refuses. A refusal is never read as a closed
            // grid.
            assert_acceptance_matches_coverage(
                calendar,
                instant,
                sunday,
                &format!("{name} at 16:05 CT on {sunday}"),
            );
            let verdict = coverage.coverage_on(sunday);
            // The profile must serve the quarter-hour exactly from the bound on,
            // and the #79 declaration must be the reason the metadata withholds
            // the date before it — never after it.
            assert_eq!(
                served,
                sunday >= bound,
                "{name} on {sunday}: the profile must serve the quarter-hour exactly from {bound}"
            );
            assert_eq!(
                declaration.applies_on(sunday),
                !served,
                "{name} on {sunday}: the #79 declaration must apply exactly where the profile \
                 withholds the quarter-hour"
            );
            if !served {
                assert_eq!(
                    verdict,
                    DateCoverage::OutsideCoveredRange,
                    "{name} on {sunday}: {verdict:?} while the quarter-hour is withheld"
                );
            }
        }
        assert_eq!(
            coverage.coverage_on(dated),
            DateCoverage::OutsideCoveredRange,
            "{name}: {dated} is inside the era its #79 declaration covers"
        );
        // A scope whose *only* declaration is the quarter-hour answers the later
        // Sunday completely; `globex_cryptocurrency` does not, because the
        // whole-domain #93 special-session gap it also declares still applies
        // there. Both outcomes are read off the declaration list rather than
        // assumed.
        assert_eq!(
            coverage.is_complete_on(after),
            coverage.phase_gaps().len() == 1,
            "{name} on {after}: complete unless a second, still-applying declaration covers it"
        );
        checked += 1;
    }
    assert_eq!(
        checked, 7,
        "all seven scopes withholding the Sunday quarter-hour are probed on both sides of their \
         own bound"
    );
}
