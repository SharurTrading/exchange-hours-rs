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
//! A declared phase-level gap has no interior edge at all: the identity
//! answers no date completely, so [`CoverageGaps`] reports one whole-domain
//! record per declaration instead of walking — the declarations are the answer,
//! and repeating them through the run walk could only ever report the first.

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
/// Produced by [`CalendarCoverage::gaps`]. An identity that declares
/// phase-level gaps reports one whole-domain span per declaration, each with its
/// own reason and [`closing_condition`](CoverageGap::closing_condition); the
/// declaration list itself is [`CalendarCoverage::phase_gaps`].
#[derive(Debug)]
pub struct CoverageGaps {
    coverage: CalendarCoverage,
    /// How many of the identity's declared phase-level gaps have been reported.
    declared: usize,
    runs: Runs,
}

impl CoverageGaps {
    /// Starts the walk this iterator reports.
    pub(super) const fn new(coverage: CalendarCoverage) -> Self {
        Self {
            coverage,
            declared: 0,
            runs: Runs::new(coverage),
        }
    }
}

impl Iterator for CoverageGaps {
    type Item = CoverageGap;

    fn next(&mut self) -> Option<CoverageGap> {
        // A declared phase-level gap applies to every date the claim covers, so
        // one identity answers nothing completely: its declarations are the whole
        // answer, reported one record each in declaration order. A date walk here
        // could only repeat the first declaration's reason over the whole domain
        // and would never surface the rest.
        if let Some(gap) = self.coverage.phase_gaps().get(self.declared) {
            self.declared = self.declared.saturating_add(1);
            return Some(CoverageGap {
                range: DateRange {
                    first: SUPPORT_FLOOR,
                    last: NaiveDate::MAX,
                },
                reason: gap.reason(),
                phase_gap: Some(*gap),
            });
        }
        if !self.coverage.phase_gaps().is_empty() {
            return None;
        }
        loop {
            let (range, reason) = self.runs.next_run()?;
            if let Some(reason) = reason {
                return Some(super::gap_of(range, reason));
            }
        }
    }
}
