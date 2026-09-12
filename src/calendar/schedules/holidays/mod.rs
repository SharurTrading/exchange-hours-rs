// SPDX-License-Identifier: MIT-0

//! Built-in per-family holiday and early-close tables (LAW-HOLIDAY-SCOPE).
//!
//! A holiday is a change confined to one venue-local trade date or a bounded
//! run of them: a full closure, an early final close, a late first open, or
//! both. It is never a normal-week template edit and never a revision row. The
//! data lives here as static date tables keyed by the crate's own
//! **venue-local trade date**, never by the operator's event date, so a Globex
//! session that opened the previous evening is clipped on the correct civil day.
//!
//! One module per identity owns its rows; `table_for` resolves an identity to
//! its table through a match with no catch-all arm, so a new `Exchange` or
//! `MarketHoursKey` cannot silently acquire or lose a table.
//!
//! **Tables ship for the served CME families and the 2026 venue block.**
//! `table_for` answers with a table for the eight CME product families, for
//! CFE, Eurex, ICE Futures U.S. and Coinbase Derivatives, and `None` for every
//! other identity; where it answers `None` the caller's `DayPolicy` overlay
//! remains the only holiday layer. Each table declares the trade-date window
//! it audited, so a date outside that window is "no answer", not "normal".
//!
//! # Layering
//!
//! A resolved table is the **innermost** of three layers and is resolved once
//! per query, when the query context is built:
//!
//! 1. the caller's `SessionExceptionSource` — an explicit `Closed` or
//!    `ReplaceSessions` record wins outright and suppresses the built-in row
//!    for that trade date;
//! 2. this built-in table;
//! 3. the caller's `DayPolicy`, which clips whatever survives.
//!
//! Layers 2 and 3 compose by **tightening**: closures are `OR`, early closes
//! take the `min`, late opens take the `max`. A caller can always make a
//! trading day shorter; a caller can never widen the crate's answer with a
//! `DayPolicy`.

mod cfe;
mod coinbase_derivatives;
mod eurex;
pub(crate) mod fences;
mod globex_cryptocurrency;
mod globex_energy;
mod globex_equity_index;
mod globex_fx;
mod globex_grains;
mod globex_interest_rates;
mod globex_livestock;
mod globex_nikkei_225_dollar;
mod ice_us;
mod routing;

pub(crate) use routing::table_for;

use chrono::NaiveDate;

use super::timeline::SourceRef;

/// The evidence tier behind one recorded fact (LAW-PRIMARY-SOURCES).
///
/// A holiday row keys on [`Self::T1`] or [`Self::T2`] only; the two lower
/// tiers exist because the law names four, and a row that claims one of them
/// fails the build rather than shipping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceTier {
    /// The operator's own statement — rulebook, notice, circular,
    /// specification page, product change log, or holiday calendar.
    T1,
    /// The operator's own machine channel — a session-schedule feed, a
    /// trading-hours service, or a reference-data API, read as bytes and saved.
    T2,
    /// A restatement by a member firm, vendor, data provider, or index
    /// publisher. Never keys a holiday row.
    T3,
    /// Press and everything else. Never keys a holiday row.
    T4,
}

/// What a built-in table records about one venue-local trade date.
///
/// The vocabulary is deliberately the scalar vocabulary of
/// [`DayPolicy`](crate::DayPolicy): a special day whose *internal* phase
/// topology changes is not representable here and is recorded as a gap in the
/// owner's evidence file rather than approximated.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HolidayKind {
    /// No session belongs to this trade date.
    ///
    /// This normally removes the complete trading day, including a session that
    /// opened the previous evening. A sourced following-business-day family —
    /// CME cryptocurrency — instead rolls its continuous trading past the date,
    /// so the row means "there is no such trade date", not "trading stopped".
    Closed,
    /// The trade date's final close moves earlier.
    EarlyClose {
        /// The replacement final close, in venue-local seconds since midnight
        /// on the trade date, in `0..=86_400`.
        close_ssm: u32,
    },
    /// The trade date's first open moves later.
    LateOpen {
        /// The replacement first open, in venue-local seconds since midnight,
        /// in `0..86_400`. For a wrapped trading day a value at or after the
        /// day's normal first open is interpreted on the **preceding** local
        /// date, exactly as
        /// [`DayPolicy::late_open_ssm`](crate::DayPolicy::late_open_ssm)
        /// interprets a caller's.
        open_ssm: u32,
    },
    /// Both boundaries move.
    LateOpenAndEarlyClose {
        /// The replacement first open; see [`Self::LateOpen`].
        open_ssm: u32,
        /// The replacement final close; see [`Self::EarlyClose`].
        close_ssm: u32,
    },
    /// The date is inside the table's coverage window and is **not** audited
    /// normal: the operator published nothing this crate could read.
    ///
    /// This changes no answer — it clips nothing, exactly like
    /// [`DateException::OutOfCoverage`](crate::DateException::OutOfCoverage) —
    /// and exists so a contiguous coverage window does not have to choose
    /// between claiming an unknown date is normal and halving the window.
    Unsourced,
}

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

/// The inclusive venue-local trade-date window a built-in table audited.
///
/// Mirrors [`ExceptionCoverage`](crate::ExceptionCoverage), and for the same
/// reason: inside the window a date with no row is **audited normal**, while
/// outside it the table has no answer at all. That is the distinction
/// [`ExchangeCalendar::holiday_on`](crate::ExchangeCalendar::holiday_on) alone
/// cannot express, which is why both accessors exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HolidayCoverage {
    first: NaiveDate,
    last: NaiveDate,
}

impl HolidayCoverage {
    /// Returns the first audited trade date.
    #[must_use]
    pub const fn first(self) -> NaiveDate {
        self.first
    }

    /// Returns the last audited trade date.
    #[must_use]
    pub const fn last(self) -> NaiveDate {
        self.last
    }

    /// Returns whether `trade_date` falls inside the audited window.
    #[must_use]
    pub fn contains(self, trade_date: NaiveDate) -> bool {
        self.first <= trade_date && trade_date <= self.last
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

/// One family's holiday rows plus the window they were audited over.
#[derive(Clone, Copy)]
pub(crate) struct HolidayTable {
    pub(crate) first: NaiveDate,
    pub(crate) last: NaiveDate,
    pub(crate) rows: &'static [HolidayRow],
}

impl HolidayTable {
    /// Returns the audited trade-date window.
    pub(crate) const fn coverage(&self) -> HolidayCoverage {
        HolidayCoverage {
            first: self.first,
            last: self.last,
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

/// Builds a `&'static HolidayTable` whose invariants hold by construction.
///
/// The invocation is
///
/// ```text
/// holidays! {
///     coverage: (2025, 1, 1) ..= (2027, 12, 31),
///     rows: [
///         (2025, 1, 20, early_close(12 * 3_600), T2, "CME-SVC-2025-01-20"),
///         (2025, 12, 25, HolidayKind::Closed, T2, "CME-HOL-2025-CHRISTMAS"),
///     ],
/// }
/// ```
///
/// where each row is `(year, month, day, kind, tier, document id)`. Constant
/// evaluation fails the build unless:
///
/// 1. the coverage window is ordered — `last` does not precede `first`;
/// 2. trade dates are strictly ascending, so `HolidayTable::may_affect`'s
///    partition-point search and `HolidayTable::holiday_on`'s binary search
///    see a total order and no row is shadowed by a duplicate;
/// 3. every row lies inside the coverage window, the rule
///    [`StaticSessionExceptions`](crate::StaticSessionExceptions) enforces for
///    a caller's records;
/// 4. every row carries a non-empty document id, so a holiday can never exist
///    without a named artifact; and
/// 5. every row's tier is [`EvidenceTier::T1`] or [`EvidenceTier::T2`], and
///    every instant is inside the `DayPolicy` ranges — a close in `0..=86_400`
///    and an open in `0..86_400`.
///
/// The quotations, URLs, capture times and interpretive steps stay in
/// `docs/evidence/<owner>.md` (LAW-EVIDENCE-FILES); the row carries only its
/// tier and document id, and a fence checks that both appear there.
macro_rules! holidays {
    (
        coverage: ($first_year:expr, $first_month:expr, $first_day:expr)
            ..= ($last_year:expr, $last_month:expr, $last_day:expr),
        rows: [
            $( ($year:expr, $month:expr, $day:expr, $kind:expr, $tier:expr, $document:literal) ),*
            $(,)?
        ] $(,)?
    ) => {{
        const ROWS: &[$crate::calendar::schedules::holidays::HolidayRow] = &[
            $(
                $crate::calendar::schedules::holidays::HolidayRow {
                    trade_date: $crate::calendar::schedules::holidays::fences::holiday_date(
                        $year, $month, $day,
                    ),
                    kind: $kind,
                    tier: $tier,
                    document: $crate::calendar::schedules::timeline::SourceRef::new($document),
                }
            ),*
        ];
        const TABLE: &$crate::calendar::schedules::holidays::HolidayTable =
            &$crate::calendar::schedules::holidays::HolidayTable {
                first: $crate::calendar::schedules::holidays::fences::holiday_date(
                    $first_year, $first_month, $first_day,
                ),
                last: $crate::calendar::schedules::holidays::fences::holiday_date(
                    $last_year, $last_month, $last_day,
                ),
                rows: ROWS,
            };
        const _: () = {
            const DATES: &[(i32, u32, u32)] = &[$(($year, $month, $day)),*];
            $crate::calendar::schedules::holidays::fences::assert_table(
                ($first_year, $first_month, $first_day),
                ($last_year, $last_month, $last_day),
                DATES,
                ROWS,
            );
        };
        TABLE
    }};
}

pub(crate) use holidays;
