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
use chrono::NaiveDate;
use exchange_hours::{
    CoverageGapReason, Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    calendar_for_exchange, calendar_for_market_hours_key,
};

const INVENTORY: &str = include_str!("../../docs/schedules/coverage-2025.md");

/// The one venue-local date the completeness verdicts are compared on.
///
/// It has to be a date the inventory's own columns say is *inside* every scope's
/// audit — not a withheld `Unsourced` date and not before a window opens — or the
/// comparison would be measuring something other than the verdict. 2025-06-10 is
/// a **Tuesday** inside every shipped audited window (`2025-01-01..2027-12-31`
/// reaches it for the fourteen scopes that answer 2025, and the three that open
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
/// does in the same file.
fn date_level_incompleteness() -> &'static [(&'static str, usize)] {
    &[("cme", 32), ("cbot", 31)]
}

/// `is_complete_on(SAMPLE)` agrees with the inventory's `Complete?` cell for all
/// sixteen served scopes.
///
/// The page and the API are one record, and this is the test that stops them
/// disagreeing. The defect it was added for is why it is not a loop over the cell
/// text: `is_complete_on(2025-06-10)` returned `true` for `globex_equity_index`,
/// `globex_fx` and `globex_cryptocurrency` — three scopes the same page calls
/// incomplete — because a **phase-level** gap is invisible to a date walk over
/// the identity's tables.
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
/// Between them the three phase-gap scopes fail on the page's own words before
/// their declarations exist, and the metadata must carry a cause that accounts
/// for every denial.
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
            let declared = coverage.phase_gap().is_some();
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
        (8, 5, 3),
        "the inventory's verdict shapes: eight complete, five incomplete, three with no 2025 \
         coverage"
    );
}

/// The three scopes the defect was reproduced on, pinned by name.
///
/// `inventory_completeness_verdicts_match_the_metadata` compares the metadata
/// against the page, so it would pass if the page were edited to agree with a
/// wrong API. This names the scopes and the reason each carries, so neither side
/// can move silently: the required-phase shape is `#79` and the special-session
/// shape is `#93`, and each scope reports the closing condition its own
/// `Missing / disputed` cell cites.
#[test]
fn the_three_phase_level_gaps_are_declared_with_their_closing_issues() {
    let day = sample();
    let fixtures = [
        (
            "globex_equity_index",
            CoverageGapReason::NormalWeekPhaseWithheld,
            "#79",
        ),
        (
            "globex_fx",
            CoverageGapReason::SpecialSessionUnrepresentable,
            "#93",
        ),
        (
            "globex_cryptocurrency",
            CoverageGapReason::SpecialSessionUnrepresentable,
            "#93",
        ),
    ];
    for (name, reason, closing) in fixtures {
        let coverage = calendar_for(name)
            .expect("the fixture names a served identity")
            .coverage();
        assert!(
            !coverage.is_complete_on(day),
            "{name} carries a phase-level gap, so no date is complete"
        );
        let declared = coverage
            .phase_gap()
            .unwrap_or_else(|| panic!("{name} must declare its phase-level gap"));
        assert_eq!(declared.reason(), reason, "{name}");
        assert_eq!(declared.closing_condition(), closing, "{name}");

        // The gap is reportable, and it spans the whole supported domain rather
        // than a span of dates inside it.
        let gaps: Vec<exchange_hours::CoverageGap> = coverage.gaps().collect();
        assert_eq!(gaps.len(), 1, "{name} reports exactly its declared gap");
        assert_eq!(gaps[0].range().first(), floor(), "{name}");
        assert!(gaps[0].range().is_open_ended(), "{name}");
        assert_eq!(gaps[0].reason(), reason, "{name}");
        assert_eq!(gaps[0].closing_condition(), Some(closing), "{name}");
        assert_eq!(coverage.complete_ranges().count(), 0, "{name}");
    }

    // The seven scopes the same page calls complete are untouched: a declaration
    // must not leak to a scope that carries no gap.
    for name in [
        "comex",
        "nymex",
        "globex_energy",
        "globex_grains",
        "globex_interest_rates",
        "globex_livestock",
        "globex_nikkei_225_dollar",
    ] {
        let coverage = calendar_for(name)
            .expect("the fixture names a served identity")
            .coverage();
        assert_eq!(coverage.phase_gap(), None, "{name}");
        assert!(
            coverage.is_complete_on(day),
            "{name} is complete on the sample"
        );
    }
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
