// SPDX-License-Identifier: MIT-0

//! Shared selection primitives for venue-local schedule histories.

use crate::calendar::time_zones::{SelectionInstant, ZoneId};
use jiff::civil::Date;

use super::StaticHoursProfile;
use crate::calendar::local_time::{self, mk_local_open};

/// The primary source that states a revision's day-level effective date.
///
/// A short label naming the dated primary artifact — notice, circular, or
/// rulebook filing — behind one [`Revision`] row. The full quotation and URL
/// live in the owner's evidence file. [`revisions!`] rejects a row whose
/// citation label is empty.
#[derive(Clone, Copy)]
pub(crate) struct SourceRef(&'static str);

impl SourceRef {
    /// Wraps a citation label naming the primary source of an effective date.
    pub(crate) const fn new(citation: &'static str) -> Self {
        Self(citation)
    }

    /// The citation label.
    pub(crate) const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A profile that takes effect at venue-local midnight on `effective`, on the
/// authority of `source`.
#[derive(Clone, Copy)]
pub(crate) struct Revision {
    pub(crate) effective: Date,
    pub(crate) profile: &'static StaticHoursProfile,
    pub(crate) source: SourceRef,
}

/// Builds a `&'static [Revision]` timeline whose invariants hold by
/// construction.
///
/// Each row is `(year, month, day, profile, citation)`, where `citation` is a
/// short label naming the primary source that states the row's unconditional
/// day-level effective date. Constant evaluation fails the build unless:
///
/// - the effective dates are strictly ascending, so [`select_revision`]'s
///   partition-point search sees a total order and no row is silently
///   shadowed by a duplicate or later-dated predecessor; and
/// - every row carries a non-empty [`SourceRef`] citation, so a revision can
///   never exist without a named primary source.
///
/// The comment beside the invocation points to the owner's evidence file.
macro_rules! revisions {
    (
        $( ($year:expr, $month:expr, $day:expr, $profile:expr, $citation:literal $(,)?) ),+ $(,)?
    ) => {{
        const TIMELINE: &[$crate::calendar::schedules::timeline::Revision] = &[
            $(
                $crate::calendar::schedules::timeline::Revision {
                    effective: $crate::calendar::schedules::timeline::effective_date(
                        $year, $month, $day,
                    ),
                    profile: $profile,
                    source: $crate::calendar::schedules::timeline::SourceRef::new($citation),
                }
            ),+
        ];
        const _: () = {
            const DATES: &[(i16, i8, i8)] = &[$(($year, $month, $day)),+];
            $crate::calendar::schedules::timeline::assert_ascending(DATES);
            $crate::calendar::schedules::timeline::assert_cited(TIMELINE);
        };
        TIMELINE
    }};
}

pub(crate) use revisions;

/// Declares one identity's **carried-below horizon**, the sibling of
/// [`revisions!`].
///
/// The verification ledger's `Horizon` column records, per identity, the
/// venue-local date below which that identity's normal-week rows are *carried*
/// rather than *sourced*, or an em dash when nothing is carried. The ledger is
/// prose; this macro is that column in the type system, so a reader of the code
/// sees the same boundary the reviewer recorded:
///
/// - `horizon!(2012, 5, 3)` — the ledger's `2012-05-03`: rows below that day
///   are carried backwards, not sourced.
/// - `horizon!(none)` — the ledger's `—`: nothing is carried below the
///   identity's own first row, so its weekday profile is sourced wherever its
///   own timeline governs.
///
/// A malformed date fails constant evaluation through [`effective_date`],
/// exactly as a [`revisions!`] row does. The values live beside their identity
/// in `schedules/sourcing.rs`; that module's header names the ledger cell each
/// one restates.
macro_rules! horizon {
    (none) => {
        None
    };
    ($year:expr, $month:expr, $day:expr $(,)?) => {
        Some($crate::calendar::schedules::timeline::effective_date(
            $year, $month, $day,
        ))
    };
}

pub(crate) use horizon;

/// Fails the build when a timeline's effective dates are not strictly
/// ascending.
///
/// A duplicate date would be silently shadowed by `select_revision`'s
/// partition-point search, and an out-of-order row would misroute every
/// earlier lookup, so both are build failures rather than runtime hazards.
pub(crate) const fn assert_ascending(rows: &[(i16, i8, i8)]) {
    let mut index = 1;
    while index < rows.len() {
        assert!(
            day_key(rows[index].0, rows[index].1, rows[index].2)
                > day_key(rows[index - 1].0, rows[index - 1].1, rows[index - 1].2),
            "revision timeline is not strictly ascending; a row is out of order or shadowed"
        );
        index += 1;
    }
}

/// Fails the build when a revision row carries no primary-source citation.
pub(crate) const fn assert_cited(rows: &[Revision]) {
    let mut index = 0;
    while index < rows.len() {
        assert!(
            !rows[index].source.as_str().is_empty(),
            "revision row carries no primary-source citation"
        );
        index += 1;
    }
}

/// Orders `(year, month, day)` as one comparable scalar for const evaluation;
/// tuple `PartialOrd` is not const-callable.
const fn day_key(year: i16, month: i8, day: i8) -> i64 {
    year as i64 * 10_000 + month as i64 * 100 + day as i64
}

/// Builds a hard-coded effective date during constant evaluation.
pub(crate) const fn effective_date(year: i16, month: i8, day: i8) -> Date {
    Date::constant(year, month, day)
}

/// Converts an instant to the calendar date used by a venue's rule table.
#[inline]
pub(crate) fn local_date(as_of: SelectionInstant<'_>, tz: ZoneId) -> Date {
    local_time::local_date(as_of.zones.resolve(tz), as_of.instant)
}

/// Selects from an ascending, venue-local revision timeline.
///
/// Dates before the first revision retain `baseline`. Temporary regimes are
/// represented by an ordinary start revision followed by a restoration row.
/// Timelines built by [`revisions!`] guarantee the ascending-order
/// precondition this partition-point search relies on.
pub(crate) fn select_revision(
    day: Date,
    baseline: &'static StaticHoursProfile,
    revisions: &[Revision],
) -> &'static StaticHoursProfile {
    let insertion = revisions.partition_point(|revision| revision.effective <= day);
    insertion
        .checked_sub(1)
        .and_then(|index| revisions.get(index))
        .map_or(baseline, |revision| revision.profile)
}

/// Returns `reference - venue` UTC-offset seconds on the venue's local day.
///
/// Sample at venue-local noon. If Jiff's timestamp range excludes that noon,
/// sample at the caller's representable instant on the same venue-local date.
pub(crate) fn reference_delta_seconds(
    as_of: SelectionInstant<'_>,
    venue_tz: ZoneId,
    reference_tz: ZoneId,
) -> i32 {
    let day = local_date(as_of, venue_tz);
    let venue_tz = as_of.zones.resolve(venue_tz);
    let reference_tz = as_of.zones.resolve(reference_tz);
    let sample = mk_local_open(venue_tz, day, 12 * 3600).unwrap_or(as_of.instant);
    reference_tz.to_offset(sample).seconds() - venue_tz.to_offset(sample).seconds()
}
