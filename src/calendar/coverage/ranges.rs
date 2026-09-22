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
//! case beside it. Both a declaration's own edges — its `PhaseGap::until` day and
//! the day before it — are run boundaries, so every run lies wholly inside the
//! declaration's era or wholly outside it, and one precedence
//! ([`CalendarCoverage::phase_gap_on`], the first declaration applying to a date)
//! settles each run exactly as the per-date accessor settles its dates. A run a
//! declaration answers for is reported by [`CoverageGaps`] with that declaration
//! and is **not** yielded by [`CompleteRanges`]; a run no declaration answers for is
//! decided by the ordinary date-level facts.
//!
//! The two iterators are a partition of the supported domain, and a declaration's
//! record spans its whole era however many static edges fall inside it: the era
//! before its bound, the whole domain when it carries none and no bounded
//! declaration precedes it, or what a bounded predecessor left when one does —
//! `globex_fx` is the shipped case, whose special-session gap applies everywhere
//! but reports from the quarter-hour gap's bound on. A declaration an earlier one
//! already answers for on every date has no record, because no date has it as its
//! answer.

use super::{CalendarCoverage, CoverageGap, CoverageGapReason, DateRange, SUPPORT_FLOOR};
use chrono::NaiveDate;

/// The most gap records one identity may report before the walk falls back to
/// reporting its remaining runs a run at a time.
///
/// The walk's own edges are bounded by the identity's window and row counts, and a
/// declaration adds two more, so this is generous headroom rather than a limit
/// anything ships near: the largest shipped identity reports **32** — `cbot`, which
/// withholds 31 dates and adds the window's trailing gap — so the capacity is eight
/// times the worst case today. The effect of exceeding it is only that a
/// declaration's record is not merged across its date-level edges, never a missing
/// or a wrong span, and `tests/coverage_metadata.rs` holds every identity to it.
pub(super) const GAP_RECORD_CAPACITY: usize = 256;
/// Walks the supported domain once in maximal runs of one verdict.
///
/// Every run boundary is a static table edge, so the walk is bounded by the
/// identity's window and row counts and allocates nothing.
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

    /// Starts a walk at `cursor`, restoring a walk another value parked.
    ///
    /// The walk carries no state beyond its cursor — every run is derived from the
    /// identity's static edges — so a scope may run several walks over the same
    /// domain without them disagreeing, which is what lets each declaration be
    /// walked on its own.
    const fn resume(coverage: CalendarCoverage, cursor: NaiveDate) -> Self {
        Self {
            coverage,
            cursor,
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
/// Produced by [`CalendarCoverage::complete_ranges`]. An identity whose
/// declarations are all unbounded reports no span at all; one that bounds a
/// declaration to an era reports the spans from that era on.
#[derive(Debug)]
pub struct CompleteRanges {
    runs: Runs,
}

impl CompleteRanges {
    /// Returns the most records this iterator can report before it falls back to
    /// walking the remaining runs one at a time.
    ///
    /// A caller sizing a buffer for [`CalendarCoverage::gaps`] needs the bound, and
    /// a fence needs it to notice when the shipped tables outgrow it. Exceeding it
    /// truncates no record: the walk continues past the capacity.
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
            let (range, reason) = self.runs.next_run()?;
            // A declared gap can cover a run the date level would have called
            // ordinary — the bounded era is the shipped case — and that run is
            // reported by `gaps` with its closing condition. Yielding it here too
            // would report one date as both complete and incomplete, so the two
            // iterators never overlap.
            let declared = self.runs.coverage.phase_gap_on(range.first()).is_some();
            if reason.is_none() && !declared {
                return Some(range);
            }
        }
    }
}

/// Ascending iterator over the spans an identity cannot answer completely, each
/// with its reason.
///
/// Produced by [`CalendarCoverage::gaps`]. The records are computed by one walk
/// when the iterator is first asked, then handed out: a **declared phase-level
/// gap** first, one record per declaration that answers for any date, then the
/// date-level runs in ascending order with the reason the walk derived. The runs a
/// declaration answers for are not repeated among the date-level ones, so every
/// record spans dates no other record claims — the declaration records themselves
/// are ascending, the date-level ones follow them, and each record reports exactly
/// one declaration ([`CoverageGap::phase_gap`]) whose own bound is on
/// [`CalendarCoverage::phase_gaps`].
#[derive(Debug)]
pub struct CoverageGaps {
    coverage: CalendarCoverage,
    records: [Option<CoverageGap>; GAP_RECORD_CAPACITY],
    /// How many records the walk produced, saturating at the capacity.
    len: usize,
    /// How many records have been handed out.
    next: usize,
    /// The date-level walk, used once the precomputed records are exhausted.
    runs: Runs,
    /// Whether the precomputed records have been built.
    built: bool,
}

impl CoverageGaps {
    /// Returns the most records this iterator can report before it falls back to
    /// walking the remaining runs one at a time.
    ///
    /// A caller sizing a buffer for [`CalendarCoverage::gaps`] needs the bound, and
    /// a fence needs it to notice when the shipped tables outgrow it. Exceeding it
    /// truncates no record: the walk continues past the capacity.
    #[must_use]
    pub const fn capacity() -> usize {
        GAP_RECORD_CAPACITY
    }

    /// Starts the walk this iterator reports.
    pub(super) const fn new(coverage: CalendarCoverage) -> Self {
        Self {
            coverage,
            records: [None; GAP_RECORD_CAPACITY],
            len: 0,
            next: 0,
            runs: Runs::new(coverage),
            built: false,
        }
    }

    /// Records one phase-level gap per declaration, over the whole span that
    /// declaration answers for.
    ///
    /// The span is the union of the runs the declaration answers for, which is
    /// contiguous because its own bound is a run edge: the era before it for the
    /// seven scopes withholding the Sunday quarter-hour, the whole supported domain
    /// for a declaration that carries no bound and has none before it, and what a
    /// bounded predecessor left for one that follows it — `globex_fx`'s shape, whose
    /// special-session gap applies everywhere and reports from the quarter-hour
    /// gap's bound on so the two records stay disjoint.
    ///
    /// A declaration answers for a run when it applies and no earlier declaration
    /// does; that is exactly the set the per-date accessor names it on, so each
    /// record's span is the span its own declaration is the answer for. The
    /// date-level half of the iterator resumes past those runs, so no span is
    /// reported twice.
    fn build(&mut self) {
        let mut claimed = 0_usize;
        for declaration in self.coverage.phase_gaps() {
            let mut first = None;
            let mut last = None;
            let mut walk = Runs::new(self.coverage);
            while let Some((range, _)) = walk.next_run() {
                let answers = self.coverage.phase_gap_on(range.first()) == Some(*declaration);
                if !answers {
                    continue;
                }
                claimed += 1;
                first.get_or_insert(range.first());
                last = Some(range.last());
            }
            if let (Some(first), Some(last)) = (first, last) {
                self.push(CoverageGap {
                    range: DateRange { first, last },
                    reason: declaration.reason(),
                    phase_gap: Some(*declaration),
                });
            }
        }
        self.runs = Runs::resume(self.coverage, SUPPORT_FLOOR);
        for _ in 0..claimed {
            if self.runs.next_run().is_none() {
                break;
            }
        }
        self.built = true;
    }

    /// Records one gap, saturating at the capacity.
    fn push(&mut self, gap: CoverageGap) {
        if let Some(slot) = self.records.get_mut(self.len) {
            *slot = Some(gap);
            self.len += 1;
        }
    }
}

impl Iterator for CoverageGaps {
    type Item = CoverageGap;

    fn next(&mut self) -> Option<CoverageGap> {
        if !self.built {
            self.build();
        }
        if let Some(gap) = self.records.get(self.next).copied().flatten() {
            self.next += 1;
            return Some(gap);
        }
        // Anything past the capacity is still walked here, so a record is never
        // dropped — only the declaration records, which are computed first and are
        // the fewest, could be truncated, and the capacity leaves room for many
        // times the three bounds the shipped tables declare.
        loop {
            let (range, reason) = self.runs.next_run()?;
            if let Some(reason) = reason {
                return Some(super::gap_of(range, reason));
            }
        }
    }
}
