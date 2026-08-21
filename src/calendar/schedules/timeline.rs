// SPDX-License-Identifier: MIT-0

//! Shared selection primitives for venue-local schedule histories.

use chrono::{DateTime, NaiveDate, Offset, Utc};
use chrono_tz::Tz;

use super::StaticHoursProfile;
use crate::calendar::local_time::mk_local_open;

/// A profile that takes effect at venue-local midnight on `effective`.
#[derive(Clone, Copy)]
pub(crate) struct Revision {
    pub(crate) effective: NaiveDate,
    pub(crate) profile: &'static StaticHoursProfile,
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
#[inline]
pub(crate) fn local_date(as_of: DateTime<Utc>, tz: Tz) -> NaiveDate {
    as_of.with_timezone(&tz).date_naive()
}

/// Selects from an ascending, venue-local revision timeline.
///
/// Dates before the first revision retain `baseline`. Temporary regimes are
/// represented by an ordinary start revision followed by a restoration row.
pub(crate) fn select_revision(
    day: NaiveDate,
    baseline: &'static StaticHoursProfile,
    revisions: &[Revision],
) -> &'static StaticHoursProfile {
    let mut selected = baseline;
    for revision in revisions {
        if day < revision.effective {
            break;
        }
        selected = revision.profile;
    }
    selected
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
