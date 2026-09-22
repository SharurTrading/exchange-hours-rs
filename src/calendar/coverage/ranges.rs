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
//! A declared phase-level gap has no interior edge at all: the walk then reports
//! a single run from the support floor to the end of the domain, carrying the
//! identity's own declaration with it.

use super::{CalendarCoverage, CoverageGap, CoverageGapReason, DateRange, SUPPORT_FLOOR};
use chrono::NaiveDate;

/// Walks the supported domain once in maximal runs of one verdict.
///
/// Every run boundary is a static table edge, so the walk is bounded by the
/// identity's window and row counts and allocates nothing.
#[derive(Debug, Clone, Copy)]
pub(super) struct Runs {
    pub(super) coverage: CalendarCoverage,
    cursor: NaiveDate,
    done: bool,
}

impl Runs {
    /// Starts a walk at the support floor.
    const fn new(coverage: CalendarCoverage) -> Self {
        Self {
            coverage,
            cursor: SUPPORT_FLOOR,
            done: false,
        }
    }

    /// Returns the next run, with the reason that applies inside it, or `None`
    /// once the domain is walked.
    fn next_run(&mut self) -> Option<(DateRange, Option<CoverageGapReason>)> {
        if self.done {
            return None;
        }
        let first = self.cursor;
        let reason = self.coverage.gap_reason_on(first);
        let last = self.coverage.run_end(first);
        match last.succ_opt() {
            Some(next) => self.cursor = next,
            None => self.done = true,
        }
        Some((DateRange { first, last }, reason))
    }
}

/// Ascending iterator over the spans an identity answers completely.
///
/// Produced by [`CalendarCoverage::complete_ranges`]. An identity that declares
/// a phase-level gap reports no span at all.
#[derive(Debug)]
pub struct CompleteRanges {
    runs: Runs,
}

impl CompleteRanges {
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
            let (range, reason) = self.runs.next_run()?;
            if reason.is_none() {
                return Some(range);
            }
        }
    }
}

/// Ascending iterator over the spans an identity cannot answer completely, each
/// with its reason.
///
/// Produced by [`CalendarCoverage::gaps`]. An identity that declares a
/// phase-level gap reports one span covering the whole supported domain, and its
/// [`closing_condition`](CoverageGap::closing_condition) is the issue that would
/// discharge it.
#[derive(Debug)]
pub struct CoverageGaps {
    runs: Runs,
}

impl CoverageGaps {
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
            let (range, reason) = self.runs.next_run()?;
            if let Some(reason) = reason {
                return Some(super::gap_of(self.runs.coverage, range, reason));
            }
        }
    }
}
