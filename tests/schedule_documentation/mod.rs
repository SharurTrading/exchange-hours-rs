// SPDX-License-Identifier: MIT-0

//! Documentation fences for the schedule-review ledger.

mod coverage_inventory;
mod databento;
mod evidence_files;
mod family_aggregates;
mod fences;
mod fixtures;
mod horizons;
mod source_registry;
mod trade_type_keys;

use fixtures::{
    AUDIT, DATABENTO_VENUES, DATE_EXCEPTIONS, EVIDENCE_DIR, EXPECTED_MARKET_HOURS_KEY_NAMES,
    LEDGER_CELLS, README, SOURCES, UNSUPPORTED_FAMILIES, UPDATING, VERIFICATION, assert_row_shape,
    assert_source_links_resolve, basis_of, evidence_target, exchange_rows, is_partial,
    market_hours_key_rows, number_words, owner_targets, repository_cutoff, row_cells, today_utc,
    validated_source_link_count, wire_name,
};
