// SPDX-License-Identifier: MIT-0

//! Euronext CAC 40 and AEX index futures (one family, Paris/Amsterdam clock).
//!
//! CAC 40 Index futures (`FCE`, Euronext Paris) and AEX Index futures (`FTI`,
//! Euronext Amsterdam) print one identical Central Order Book grid —
//! 08:00-22:00 CET/CEST, no lunch break, Monday to Friday — on the operator's
//! own contract-specification pages, and the two markets' clocks (Paris and
//! Amsterdam) are the same CET/CEST zone, so one family serves both. The
//! Large-in-Scale Facility beside that grid (07:30-22:00) is an off-book
//! wholesale facility, not an order-book phase, and is modelled no further —
//! the same scope the Eurex families draw over TES.

use chrono_tz::Europe;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;

// The operator's contract pages print one Trading hours row per product:
// "Central Order Book: 08:00 - 22:00 CET/CEST". Both endpoints are stated on
// the local clock (CET with CEST named beside it), so one table serves the
// whole year; no seasonal selector exists. No pre-open or post-close phase is
// published on either contract page, so `order_entry` is empty.
// Narrative: docs/evidence/euronext_index_futures.md
pub(crate) static EURONEXT_INDEX_FUTURES_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 22 * 3600,
}];

static EURONEXT_INDEX_FUTURES: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Paris,
    regular: EURONEXT_INDEX_FUTURES_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// Resolves the Euronext index-futures profile at any instant.
///
/// The family carries no recorded change and no seasonal grid: the two banked
/// contract pages are its only normal-week sources, and no banked artifact
/// dates any earlier state, so the ledger records the grid as carried below
/// the 2026-10-11 retrieval rather than inventing a cutover history. The same
/// one-grid-at-every-instant shape as a launch-dated family past its launch,
/// with the horizon disclosure living in the ledger row and evidence file.
pub(crate) fn euronext_index_futures_profile_at(
    _as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    &EURONEXT_INDEX_FUTURES
}
