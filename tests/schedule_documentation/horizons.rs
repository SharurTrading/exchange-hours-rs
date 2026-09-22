// SPDX-License-Identifier: MIT-0

//! Fence for the verification ledger's `Horizon` column.
//!
//! `schedules/sourcing.rs` restates, per identity, the venue-local date below
//! which that identity's normal-week rows are **carried** rather than
//! **sourced** — the ledger's `Horizon` cell, or an em dash when nothing is
//! carried. The ledger is the authority: this fence re-reads it and compares
//! every cell against the shipped metadata, so a horizon cannot be invented,
//! dropped or silently re-dated. `docs/schedules/coverage-2025.md` repeats the
//! same value for every served scope, and the third test holds the two
//! documents together.
//!
//! The comparison is against the ledger's own bytes, never against a list
//! copied out of the module, so a module that drifts fails here.

use super::VERIFICATION;
use chrono::NaiveDate;
use exchange_hours::{
    Exchange, MarketHoursKey, calendar_for_exchange, calendar_for_market_hours_key,
};

/// One ledger table row: its wire-name, `Horizon` cell and service tier.
struct LedgerRow {
    name: String,
    horizon: String,
    served: bool,
}

/// The rows of one ledger section, in document order.
fn ledger_rows(heading: &str) -> Vec<LedgerRow> {
    let mut rows = Vec::new();
    let mut inside = false;
    for line in VERIFICATION.lines() {
        if line.starts_with("## ") {
            inside = line.starts_with(heading);
            continue;
        }
        if !inside || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if cells.len() != 12 || cells[0].starts_with("---") || cells[0] == "Identity" {
            continue;
        }
        rows.push(LedgerRow {
            name: cells[0].trim_matches('`').to_owned(),
            horizon: cells[6].to_owned(),
            served: cells[5] == "served",
        });
    }
    assert!(
        !rows.is_empty(),
        "the ledger section {heading:?} must carry one row per identity"
    );
    rows
}

/// Reads a `Horizon` cell: an em dash is `None`, a date is itself, and a cell
/// that is neither comes back as `Err` so the caller can name it.
fn horizon(cell: &str) -> Result<Option<NaiveDate>, &str> {
    if cell.starts_with('\u{2014}') {
        return Ok(None);
    }
    NaiveDate::parse_from_str(cell, "%Y-%m-%d")
        .map(Some)
        .map_err(|_| cell)
}

/// The wire names of one section, in document order.
fn names(rows: &[LedgerRow]) -> Vec<&str> {
    rows.iter().map(|row| row.name.as_str()).collect()
}

#[test]
fn exchange_horizons_match_the_declared_sourcing() {
    let rows = ledger_rows("## Exchanges");
    let expected: Vec<&str> = Exchange::ALL
        .iter()
        .map(|exchange| exchange.as_str())
        .collect();
    assert_eq!(
        names(&rows),
        expected,
        "the ledger must carry exactly one row per Exchange variant, in ALL order"
    );
    for row in rows {
        let exchange: Exchange = row.name.parse().expect("a canonical exchange wire name");
        let expected = horizon(&row.horizon)
            .unwrap_or_else(|cell| panic!("the Horizon cell for {} reads {cell:?}", row.name));
        assert_eq!(
            calendar_for_exchange(exchange)
                .coverage()
                .normal_week_sourced_from(),
            expected,
            "the Horizon cell for {} must be what the crate declares",
            row.name
        );
    }
}

#[test]
fn market_hours_key_horizons_match_the_declared_sourcing() {
    let rows = ledger_rows("## `MarketHoursKey` profiles");
    let expected: Vec<&str> = MarketHoursKey::ALL.iter().map(|key| key.as_str()).collect();
    assert_eq!(
        names(&rows),
        expected,
        "the ledger must carry exactly one row per MarketHoursKey variant, in ALL order"
    );
    for row in rows {
        let key: MarketHoursKey = row.name.parse().expect("a canonical key wire name");
        let expected = horizon(&row.horizon)
            .unwrap_or_else(|cell| panic!("the Horizon cell for {} reads {cell:?}", row.name));
        assert_eq!(
            calendar_for_market_hours_key(key)
                .coverage()
                .normal_week_sourced_from(),
            expected,
            "the Horizon cell for {} must be what the crate declares",
            row.name
        );
    }
}

#[test]
fn the_coverage_inventory_repeats_the_ledger_horizons() {
    // `docs/schedules/coverage-2025.md` states one row per served scope with
    // the same horizon value. Its own fence compares membership, windows and
    // counts against the shipped tables; this one adds the horizon, so the two
    // documents cannot disagree about where a served scope's sourcing starts.
    let mut served: Vec<(String, Option<NaiveDate>)> = Vec::new();
    for row in ledger_rows("## Exchanges")
        .into_iter()
        .chain(ledger_rows("## `MarketHoursKey` profiles"))
    {
        let cell = horizon(&row.horizon)
            .unwrap_or_else(|cell| panic!("the Horizon cell for {} reads {cell:?}", row.name));
        if let Some((_, previous)) = served.iter().find(|(name, _)| *name == row.name) {
            assert_eq!(
                *previous, cell,
                "{} carries two ledger rows with different horizons",
                row.name
            );
        } else {
            served.push((row.name, cell));
        }
    }
    let served_count = ledger_rows("## Exchanges")
        .into_iter()
        .chain(ledger_rows("## `MarketHoursKey` profiles"))
        .filter(|row| row.served)
        .count();

    let inventory = include_str!("../../docs/schedules/coverage-2025.md");
    let mut checked = 0_usize;
    let mut inside = false;
    for line in inventory.lines() {
        if line.starts_with("## ") {
            inside = line.starts_with("## Inventory");
            continue;
        }
        if !inside || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if cells.len() != 10 || cells[0].starts_with("---") || cells[0] == "Identity" {
            continue;
        }
        let name = cells[0].trim_matches('`');
        let cell = cells[3];
        let (_, expected) = served
            .iter()
            .find(|(listed, _)| listed == name)
            .unwrap_or_else(|| panic!("the inventory names {name}, which the ledger lists"));
        let listed = horizon(cell).unwrap_or_else(|cell| {
            panic!("the inventory's Horizon cell for {name} reads {cell:?}")
        });
        assert_eq!(
            *expected, listed,
            "the inventory's Horizon cell for {name} must be the ledger's"
        );
        checked += 1;
    }
    assert_eq!(
        checked, served_count,
        "the inventory carries one row per served ledger scope"
    );
}
