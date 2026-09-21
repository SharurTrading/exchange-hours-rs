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
    Exchange, ExchangeCalendar, HolidayKind, MarketHoursKey, calendar_for_exchange,
    calendar_for_market_hours_key,
};

const INVENTORY: &str = include_str!("../../docs/schedules/coverage-2025.md");

/// The support floor LAW-COVERAGE fixes at 2025-01-01.
fn floor() -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 1, 1).expect("2025-01-01 is a valid date")
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

/// Reads the leading count from an inventory count cell: `41`,
/// `24 (129 rows)`, or the em dash that means none.
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
