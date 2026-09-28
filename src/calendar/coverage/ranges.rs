// SPDX-License-Identifier: MIT-0

//! The ascending span walks behind [`CalendarCoverage::complete_ranges`] and
//! [`CalendarCoverage::gaps`].
//!
//! One walk answers both: it visits the supported domain in maximal runs of a
//! single verdict, and the two iterators keep the runs they were asked for. The
//! walks are **derived, not duplicated** — a run ends at the next static edge
//! ([`CalendarCoverage::next_boundary_after`]) — so reading the metadata
//! allocates nothing, never sorts, and cannot disagree with the per-date
//! accessor beside it.
//!
//! A declared phase-level gap is part of the same partition rather than a special
//! case beside it. Where every declaration an identity carries is whole-domain or
//! era-bounded, the declaration is itself static across its era: every run lies
//! wholly inside the declaration's era or wholly outside it, and one precedence
//! ([`CalendarCoverage::phase_gap_on`], the first declaration applying to a date)
//! settles each run exactly as the per-date accessor settles its dates.
//!
//! A **date-scoped** declaration — one carrying an
//! [`PhaseGapShape::OrderEntryWindow`] shape — can flip the verdict from date to
//! date, because the served occurrence it is keyed to resolves on a bracket-era
//! Sunday and not on the Tuesday beside it. Inside such a declaration's span the
//! walk therefore merges verdicts **day by day**, which keeps every reported run
//! faithful to [`CalendarCoverage::coverage_on`] one date at a time: #79's
//! quarter-hour comes back as the bracket-era Sundays, one record each, with the
//! Tuesdays between them answered. The day-merged region is bounded by the
//! declaration's own era and the identity's audited windows — outside both, the
//! ordinary facts refuse every date uniformly and the static-edge walk resumes —
//! so the walk's cost stays bounded by the identity's own tables.
//!
//! The two iterators are a partition of the supported domain: the runs are
//! disjoint, ascending, and together they tile `[SUPPORT_FLOOR, NaiveDate::MAX]`,
//! and each run's verdict is exactly what the per-date accessor answers on every
//! date inside it.

use super::{CalendarCoverage, CoverageGap, CoverageGapReason, DateRange, SUPPORT_FLOOR};
use chrono::NaiveDate;

/// The most declarations one identity may carry.
///
/// A declaration is a `PhaseGap` on the identity's `phase_gaps` list, and this
/// bounds the list — not the records the walk reports, which a date-scoped
/// declaration expands to one per maximal run (hundreds for the bracket-era
/// Sundays) and which are streamed a run at a time and never stored. The bound
/// is generous headroom rather than a limit anything ships near: the most
/// declarations any shipped identity carries is **2** (`globex_grains`), against
/// a bound of 256, and `tests/coverage_metadata.rs` holds every identity to it.
pub(super) const GAP_RECORD_CAPACITY: usize = 256;

/// One walked run: a maximal span of one verdict, with the declaration the
/// verdict's gap carries when a declaration is the answer for the run.
pub(super) type Run = (
    DateRange,
    Option<CoverageGapReason>,
    Option<super::PhaseGap>,
);

/// Walks the supported domain once in maximal runs of one verdict.
///
/// Run boundaries are static table edges everywhere except inside the span of a
/// date-scoped declaration, where the run is merged from per-date verdicts; both
/// regimes are bounded by the identity's own tables and allocate nothing.
#[derive(Debug, Clone, Copy)]
pub(super) struct Runs {
    coverage: CalendarCoverage,
    cursor: NaiveDate,
    done: bool,
}

impl Runs {
    /// Starts a walk at the support floor.
    pub(super) const fn new(coverage: CalendarCoverage) -> Self {
        Self {
            coverage,
            cursor: SUPPORT_FLOOR,
            done: false,
        }
    }

    /// Returns the next run, with the reason that applies inside it and the
    /// declaration that reason carries, or `None` once the domain is walked.
    fn next_run(&mut self) -> Option<Run> {
        if self.done {
            return None;
        }
        let first = self.cursor;
        let verdict = self.verdict_at(first);
        let segment_last = self.coverage.run_end(first);
        // Outside a date-scoped declaration's reach the verdict is constant
        // across the whole static segment. Inside it the verdict can flip from
        // date to date, so the run is merged from per-date verdicts; the region
        // ends at the segment's own static edge at the latest.
        let last = if self
            .coverage
            .has_date_scoped_shapes_between(first, segment_last)
        {
            let mut day = first;
            while let Some(next) = day.succ_opt() {
                if next > segment_last || self.verdict_at(next) != verdict {
                    break;
                }
                day = next;
            }
            day
        } else {
            segment_last
        };
        match last.succ_opt() {
            Some(next) => self.cursor = next,
            None => self.done = true,
        }
        Some((DateRange { first, last }, verdict.0, verdict.1))
    }

    /// The verdict one date walks to: the reason the date refuses with, and the
    /// declaration that reason belongs to when a declaration supplies it.
    fn verdict_at(&self, date: NaiveDate) -> (Option<CoverageGapReason>, Option<super::PhaseGap>) {
        match self.coverage.phase_gap_on(date) {
            Some(declaration) => (Some(declaration.reason()), Some(declaration)),
            None => (self.coverage.gap_reason_on(date), None),
        }
    }
}

/// Ascending iterator over the spans an identity answers completely.
///
/// Produced by [`CalendarCoverage::complete_ranges`]. An identity with any
/// unbounded whole-domain declaration reports no span at all; one whose
/// declarations are bounded or date-scoped reports the spans its ordinary facts
/// answer between them.
#[derive(Debug)]
pub struct CompleteRanges {
    runs: Runs,
}

impl CompleteRanges {
    /// Returns the walk's declaration bound.
    ///
    /// This iterator reports complete spans, never declaration records, and holds
    /// no precomputed store; the constant bounds how many declarations one
    /// identity may carry. Exposed so a caller can size against the same number
    /// the fence uses, and `tests/coverage_metadata.rs` holds every identity to
    /// it.
    #[must_use]
    pub const fn capacity() -> usize {
        GAP_RECORD_CAPACITY
    }

    /// Starts the walk this iterator reports.
    pub(super) const fn new(coverage: CalendarCoverage) -> Self {
        Self {
            runs: Runs::new(coverage),
        }
    }
}

impl Iterator for CompleteRanges {
    type Item = DateRange;

    fn next(&mut self) -> Option<DateRange> {
        loop {
            let (range, reason, _declaration) = self.runs.next_run()?;
            // A run a declaration answers for is reported by `gaps` with its
            // closing condition. Yielding it here too would report one date as
            // both complete and incomplete, so the two iterators never overlap —
            // a declaration always refuses, so its runs carry a reason.
            if reason.is_none() {
                return Some(range);
            }
        }
    }
}

/// Ascending iterator over the spans an identity cannot answer completely, each
/// with its reason.
///
/// Produced by [`CalendarCoverage::gaps`]. The records are **streamed in
/// ascending date order** by one walk over the supported domain: a record whose
/// reason is a declared phase-level gap carries that declaration
/// ([`CoverageGap::phase_gap`]) over one maximal span the declaration answers
/// for, and every other record is a date-level run with the reason the walk
/// derived. No span is reported twice and none is withheld, so the records and
/// [`CompleteRanges`] tile the supported domain.
#[derive(Debug)]
pub struct CoverageGaps {
    runs: Runs,
}

impl CoverageGaps {
    /// Returns the walk's declaration bound.
    ///
    /// It bounds the **declarations** one identity may carry, not the records
    /// this iterator reports: a date-scoped declaration reports one record per
    /// maximal run its shape resolves, and the records are streamed a run at a
    /// time and never stored, so no record count can overflow anything.
    #[must_use]
    pub const fn capacity() -> usize {
        GAP_RECORD_CAPACITY
    }

    /// Starts the walk this iterator reports.
    pub(super) const fn new(coverage: CalendarCoverage) -> Self {
        Self {
            runs: Runs::new(coverage),
        }
    }
}

impl Iterator for CoverageGaps {
    type Item = CoverageGap;

    fn next(&mut self) -> Option<CoverageGap> {
        loop {
            let (range, reason, declaration) = self.runs.next_run()?;
            if let Some(reason) = reason {
                return Some(match declaration {
                    Some(declaration) => CoverageGap {
                        range,
                        reason,
                        phase_gap: Some(declaration),
                    },
                    None => super::gap_of(range, reason),
                });
            }
        }
    }
}
