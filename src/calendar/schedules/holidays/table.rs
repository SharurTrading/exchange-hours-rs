// SPDX-License-Identifier: MIT-0

//! One family's static holiday table, its rows, and the caller-facing row and
//! coverage reports the table answers with.

use chrono::NaiveDate;

use super::kind::HolidayKind;
use super::tier::EvidenceTier;
use crate::calendar::schedules::timeline::SourceRef;

/// One built-in holiday row, as reported to a caller.
///
/// The tier and document id travel **in** the row rather than in an adjacent
/// comment, so LAW-EVIDENCE-FILES' "a row's tier and document id live beside
/// the row" is mechanically checkable and a T3 holiday is unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Holiday {
    kind: HolidayKind,
    tier: EvidenceTier,
    document: &'static str,
}

impl Holiday {
    /// Returns what this row does to the trade date.
    #[must_use]
    pub const fn kind(self) -> HolidayKind {
        self.kind
    }

    /// Returns the evidence tier of the artifact behind this row.
    #[must_use]
    pub const fn tier(self) -> EvidenceTier {
        self.tier
    }

    /// Returns the stable document id of the artifact behind this row.
    ///
    /// The id resolves to a URL, a capture or retrieval time in UTC, a sha256
    /// and a tier in the owner's `docs/evidence/<owner>.md`, under a
    /// `## Holidays` section keyed by year.
    #[must_use]
    pub const fn document_id(self) -> &'static str {
        self.document
    }
}

/// The venue-local trade-date windows a built-in table audited.
///
/// Mirrors [`ExceptionCoverage`](crate::ExceptionCoverage), and for the same
/// reason: inside an audited window a date with no row is **audited normal**,
/// while outside one the table has no answer at all. That is the distinction
/// [`ExchangeCalendar::holiday_on`](crate::ExchangeCalendar::holiday_on) alone
/// cannot express, which is why both accessors exist.
///
/// A table may hold **more than one window**. The built-in holiday tables are
/// built one operator-era wave at a time, and a wave's years are audited from
/// that wave's documents alone; a decade between two waves is not audited by
/// either, so claiming one contiguous span across it would report every date in
/// the gap as audited normal. [`Self::windows`] names the audited spans,
/// [`Self::first`] and [`Self::last`] bound them, and [`Self::contains`] is the
/// authority on whether one date was audited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HolidayCoverage {
    first: NaiveDate,
    last: NaiveDate,
    windows: &'static [(i32, u32, u32, i32, u32, u32)],
}

impl HolidayCoverage {
    /// Returns the earliest audited trade date.
    #[must_use]
    pub const fn first(self) -> NaiveDate {
        self.first
    }

    /// Returns the latest audited trade date.
    #[must_use]
    pub const fn last(self) -> NaiveDate {
        self.last
    }

    /// Returns every audited span as `(first, last)` date pairs, ascending and
    /// non-overlapping.
    ///
    /// A caller that needs to know whether a *particular* date was audited
    /// should ask [`Self::contains`]; this is for a caller that reports the
    /// coverage itself, or that wants to size a scan. An invalid window literal
    /// has already failed the build, so the conversion is total.
    #[must_use]
    pub fn windows(self) -> Vec<(NaiveDate, NaiveDate)> {
        self.windows
            .iter()
            .map(
                |(first_year, first_month, first_day, last_year, last_month, last_day)| {
                    (
                        chrono::NaiveDate::from_ymd_opt(*first_year, *first_month, *first_day)
                            .unwrap_or(self.first),
                        chrono::NaiveDate::from_ymd_opt(*last_year, *last_month, *last_day)
                            .unwrap_or(self.last),
                    )
                },
            )
            .collect()
    }

    /// Returns whether `trade_date` falls inside an audited window.
    #[must_use]
    pub fn contains(self, trade_date: NaiveDate) -> bool {
        use chrono::Datelike as _;

        let key = trade_date.num_days_from_ce();
        self.windows.iter().any(
            |(first_year, first_month, first_day, last_year, last_month, last_day)| {
                let first = chrono::NaiveDate::from_ymd_opt(*first_year, *first_month, *first_day);
                let last = chrono::NaiveDate::from_ymd_opt(*last_year, *last_month, *last_day);
                match (first, last) {
                    (Some(first), Some(last)) => {
                        first.num_days_from_ce() <= key && key <= last.num_days_from_ce()
                    }
                    _ => false,
                }
            },
        )
    }
}

/// One row of a family's static holiday table.
///
/// Rows are strictly ascending by `trade_date` and every row lies inside its
/// table's coverage window; the `holidays!` macro makes both build failures.
#[derive(Clone, Copy)]
pub(crate) struct HolidayRow {
    pub(crate) trade_date: NaiveDate,
    pub(crate) kind: HolidayKind,
    pub(crate) tier: EvidenceTier,
    pub(crate) document: SourceRef,
}

/// One family's holiday rows plus the windows they were audited over.
#[derive(Clone, Copy)]
pub(crate) struct HolidayTable {
    pub(crate) windows: &'static [(i32, u32, u32, i32, u32, u32)],
    pub(crate) rows: &'static [HolidayRow],
    /// Whether any row carries a replacement block set.
    ///
    /// Decided during constant evaluation rather than by walking the rows per
    /// query: the replacement scan and the per-occurrence block gate read this
    /// bit first, so a table shipping no block row must not pay for either,
    /// and one that ships a row pays a bounded search over the rows of the
    /// asked-for window alone.
    pub(crate) has_blocks: bool,
}

impl HolidayTable {
    /// Returns whether any row of this table replaces a trade date with blocks.
    ///
    /// This is the one-bit gate in front of the replacement scan. It is `false`
    /// for every table the crate ships today, which is what keeps Stage 3's new
    /// kind off the hot path until Stage 4 ships a row that uses it.
    pub(crate) const fn carries_replacement_blocks(&self) -> bool {
        self.has_blocks
    }

    /// Returns whether any replacement block row is keyed inside the inclusive
    /// window `first..=last`.
    ///
    /// This is the block-row sibling of [`Self::may_affect`]: one partition
    /// point over the sorted rows plus a scan of the window's own rows,
    /// allocation-free and with no trading-day derivation behind it. A table
    /// with no block row answers `false` from the constant-evaluated bit
    /// without touching the rows; a table with rows pays a binary search and
    /// then only the handful of rows the window itself holds, so a date
    /// nowhere near a merged or Saturday arrangement costs no more than a
    /// scalar-row lookup.
    pub(crate) fn blocks_may_affect(&self, first: NaiveDate, last: NaiveDate) -> bool {
        if !self.has_blocks {
            return false;
        }
        let start = self.rows.partition_point(|row| row.trade_date < first);
        self.rows[start..]
            .iter()
            .take_while(|row| row.trade_date <= last)
            .any(|row| matches!(row.kind, HolidayKind::ReplacementBlocks(_)))
    }

    /// Returns the audited trade-date windows.
    ///
    /// `first` and `last` are the outermost dates the table answers for, which
    /// its own fenced rows bound: every window holds at least one row and every
    /// row lies inside a window.
    pub(crate) const fn coverage(&self) -> HolidayCoverage {
        HolidayCoverage {
            first: crate::calendar::schedules::holidays::fences::window_first(self.windows),
            last: crate::calendar::schedules::holidays::fences::window_last(self.windows),
            windows: self.windows,
        }
    }

    /// Returns the row for `trade_date`, if this table holds one.
    ///
    /// `None` means the date is outside the window *or* inside it and audited
    /// normal; [`Self::coverage`] separates the two.
    pub(crate) fn holiday_on(&self, trade_date: NaiveDate) -> Option<Holiday> {
        let index = self
            .rows
            .binary_search_by_key(&trade_date, |row| row.trade_date)
            .ok()?;
        self.rows.get(index).map(|row| Holiday {
            kind: row.kind,
            tier: row.tier,
            document: row.document.as_str(),
        })
    }

    /// Returns whether any row lands in the inclusive window `first..=last`.
    ///
    /// This is the coverage gate: one `partition_point` over the sorted rows,
    /// allocation-free and with no trading-day derivation behind it, so a day
    /// the table says nothing about costs one binary search and nothing else.
    /// The coverage window is not consulted because every row lies inside it
    /// by construction.
    pub(crate) fn may_affect(&self, first: NaiveDate, last: NaiveDate) -> bool {
        let start = self.rows.partition_point(|row| row.trade_date < first);
        self.rows
            .get(start)
            .is_some_and(|row| row.trade_date <= last)
    }
}
