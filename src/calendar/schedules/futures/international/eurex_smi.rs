// SPDX-License-Identifier: MIT-0

//! Eurex SMI® index futures (`FSMI`) schedules.
//!
//! The Swiss benchmark-index future on Eurex trades one Zurich-clock grid whose
//! only recorded change is the 2010-07-05 extension of continuous trading to
//! 22:00. The operator's annual Trading Calendar editions print the product's
//! row from 2010 (07:50-17:27) and 2011 onward (07:50-22:00), and the
//! operator's archived Contract Specifications amendment of 2010-07-05 is the
//! dated change between them, so the family carries a dated baseline and one
//! revision rather than an undated changeover.

use chrono_tz::Europe;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The Swiss grid's phase machine matches the other Eurex families: continuous
// trading is the only executable phase and alone is `regular`; Pre-Trading and
// Post-Trading accept order entry without matching, so both are `order_entry`
// and `extended` is empty.

// The off-book OTC Block Trading column the same amendment tables is a
// wholesale facility, not an order-book phase, and is modelled no further
// (the same scope the fixed-income family draws over TES).
// Narrative: docs/evidence/eurex_smi.md
static EUREX_SMI_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 50 * 60,
    close_ssm: 17 * 3600 + 27 * 60,
}];

// Pre-Trading 07:30-07:50 CET and Post-Trading 17:27-20:30 CET: order entry
// only in both windows. The values are the 2009-10-26 amendment's Annex C row,
// which predates the January-2010 floor, so the baseline is sourced through the
// floor rather than carried back to it.
static EUREX_SMI_ORDER_ENTRY_BASELINE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 7 * 3600 + 50 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 27 * 60,
        close_ssm: 20 * 3600 + 30 * 60,
    },
];

static EUREX_SMI_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: EUREX_SMI_REGULAR_BASELINE,
    extended: &[],
    order_entry: EUREX_SMI_ORDER_ENTRY_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// From 2010-07-05: continuous trading 07:50-22:00 CET and Post-Trading
// 22:00-22:30 CET, the amendment's new values. The Trading Calendar editions
// 2011-2026 reprint the 07:50-22:00 row every year, so the regular phase is
// re-witnessed through the current table; the two order-entry windows are
// keyed by the 2010 amendment and carried since (see the evidence file).
pub(crate) static EUREX_SMI_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 50 * 60,
    close_ssm: 22 * 3600,
}];

pub(crate) static EUREX_SMI_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 7 * 3600 + 50 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

static EUREX_SMI_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: EUREX_SMI_REGULAR_CURRENT,
    extended: &[],
    order_entry: EUREX_SMI_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// The one dated change: the Contract Specifications amendment "As of
// 05.07.2010" extends FSMI's Continuous Trading from 07:50-17:27 to 07:50-22:00
// and its Post-Trading Full-Period from 17:27-20:30 to 22:00-22:30. Effective
// day and citation literal are the row's own fields; the redline and its
// archive row are in the evidence file.
// Evidence: docs/evidence/eurex_smi.md
static EUREX_SMI_REVISIONS: &[Revision] = revisions![(
    2010,
    7,
    5,
    &EUREX_SMI_CURRENT,
    "Eurex CS amendment 2010-07-05"
)];

/// Selects the Eurex SMI profile in force on `as_of`'s Zurich day.
///
/// The grid's endpoints are stated on the local clock in every era (the
/// Trading Calendar editions print no seasonal footnote for this row), so one
/// timeline serves the whole history; no CET/CEST selector exists.
pub(crate) fn eurex_smi_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::Zurich),
        &EUREX_SMI_BASELINE,
        EUREX_SMI_REVISIONS,
    )
}
