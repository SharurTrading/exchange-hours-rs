// SPDX-License-Identifier: MIT-0

//! Shared selection primitives for venue-local schedule histories.

use chrono::{DateTime, NaiveDate, Offset, Utc};
use chrono_tz::Tz;

use super::StaticHoursProfile;
use crate::calendar::local_time::{bounded_utc, mk_local_open};

/// The primary source that states a revision's day-level effective date.
///
/// A short label naming the dated primary artifact — notice, circular, or
/// rulebook filing — behind one [`Revision`] row. The full quotation and URL
/// stay in the comment beside the table; this carries the identity of the
/// evidence in the type itself, and [`revisions!`] rejects a row whose label
/// is empty.
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
    pub(crate) effective: NaiveDate,
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
/// The comment beside the invocation keeps the full quotations and URLs.
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
            const DATES: &[(i32, u32, u32)] = &[$(($year, $month, $day)),+];
            $crate::calendar::schedules::timeline::assert_ascending(DATES);
            $crate::calendar::schedules::timeline::assert_cited(TIMELINE);
        };
        TIMELINE
    }};
}

pub(crate) use revisions;

/// Fails the build when a timeline's effective dates are not strictly
/// ascending.
///
/// A duplicate date would be silently shadowed by `select_revision`'s
/// partition-point search, and an out-of-order row would misroute every
/// earlier lookup, so both are build failures rather than runtime hazards.
pub(crate) const fn assert_ascending(rows: &[(i32, u32, u32)]) {
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
const fn day_key(year: i32, month: u32, day: u32) -> i64 {
    year as i64 * 10_000 + month as i64 * 100 + day as i64
}

/// Builds a hard-coded effective date during constant evaluation.
#[expect(
    clippy::panic,
    reason = "const-eval only: an invalid sourced date must fail the build"
)]
pub(crate) const fn effective_date(year: i32, month: u32, day: u32) -> NaiveDate {
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => date,
        None => panic!("invalid hard-coded effective date"),
    }
}

/// Converts an instant to the calendar date used by a venue's rule table.
///
/// The instant is bounded to the venue zone first (see [`bounded_utc`]), so an
/// otherwise-unrepresentable chrono edge degrades inward instead of
/// misrouting the civil day. This is the one shared place every day-level
/// selector's instant is normalized.
#[inline]
pub(crate) fn local_date(as_of: DateTime<Utc>, tz: Tz) -> NaiveDate {
    bounded_utc(as_of, tz).with_timezone(&tz).date_naive()
}

/// Selects from an ascending, venue-local revision timeline.
///
/// Dates before the first revision retain `baseline`. Temporary regimes are
/// represented by an ordinary start revision followed by a restoration row.
/// Timelines built by [`revisions!`] guarantee the ascending-order
/// precondition this partition-point search relies on.
pub(crate) fn select_revision(
    day: NaiveDate,
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
/// Noon avoids civil-day boundary ambiguity. This is used only by published
/// cross-zone regimes such as B3 and BMV; it does not read a clock.
pub(crate) fn reference_delta_seconds(as_of: DateTime<Utc>, venue_tz: Tz, reference_tz: Tz) -> i32 {
    let venue_noon = mk_local_open(venue_tz, local_date(as_of, venue_tz), 12 * 3600);
    let reference_noon = venue_noon.with_timezone(&reference_tz);
    reference_noon.offset().fix().local_minus_utc() - venue_noon.offset().fix().local_minus_utc()
}
