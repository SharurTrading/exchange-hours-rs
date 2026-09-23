// SPDX-License-Identifier: MIT-0

//! The explicit coverage-error vocabulary for identity-backed queries
//! (LAW-COVERAGE).

use chrono::NaiveDate;

use super::SUPPORT_FLOOR;
use crate::calendar::exchange_calendar::CalendarSource;

/// Why an identity-backed date-aware query cannot answer.
///
/// LAW-COVERAGE requires an unsupported date to be an explicit error rather
/// than a claim that a market is open or closed, and it requires an unsupported
/// date to stay distinguishable from a bounded search that simply ran out of
/// room to look. The four variants are that distinction:
///
/// - [`Self::BeforeSupportFloor`] — the instant or date precedes the permanent
///   2025-01-01 local-date floor;
/// - [`Self::OutsideCoveredRange`] — at or after the floor, but the identity has
///   no sourced answer there: its weekday profile is carried backwards, it ships
///   no holiday table, its holiday layer has no audited answer for the date, or it
///   declares a phase-level gap that applies on the date;
/// - [`Self::UnresolvedGap`] — inside a covered range on a date the identity
///   explicitly withholds as `Unsourced`;
/// - [`Self::SearchExhausted`] — a bounded forward or period search needed a
///   date it could not establish and could not continue past.
///
/// A **known closure** is never an error: a date whose sourced answer is
/// "closed" — a holiday row, a pre-launch closure, a weekend — returns a normal
/// `Ok` result, and so does a genuine absence inside the covered range.
///
/// The type carries no allocation and no `String`: the identity, the offending
/// venue-local date and, for an exhausted search, the bound it hit. It is
/// `Copy + Send + Sync + 'static` so a caller can log or return it without
/// ceremony.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalendarQueryError {
    /// The date precedes the venue-local support floor.
    ///
    /// The floor is a *local* date ([`SUPPORT_FLOOR`]), so two identities in
    /// opposing zones reach it at different UTC instants.
    BeforeSupportFloor {
        /// The identity that cannot answer.
        source: CalendarSource,
        /// The offending venue-local date.
        date: NaiveDate,
    },
    /// The date is at or after the floor but outside the ranges this identity
    /// has a sourced answer for.
    ///
    /// Either the weekday profile is carried backwards below the identity's
    /// recorded horizon, the date falls outside every holiday window the
    /// identity's built-in table audited, or a declared phase-level gap applies
    /// on the date. The reason is available from
    /// [`CalendarCoverage::gaps`](super::CalendarCoverage::gaps), which reports
    /// the same derivation as spans.
    OutsideCoveredRange {
        /// The identity that cannot answer.
        source: CalendarSource,
        /// The offending venue-local date.
        date: NaiveDate,
    },
    /// The date is inside an audited window on a date the identity explicitly
    /// withholds as `Unsourced`.
    ///
    /// The crate has no answer and will not guess one; this is the case
    /// LAW-HOLIDAY-SCOPE means by "a date without a row is audited normal,
    /// while `Unsourced` expressly withholds that claim".
    UnresolvedGap {
        /// The identity that cannot answer.
        source: CalendarSource,
        /// The withheld venue-local date.
        date: NaiveDate,
    },
    /// A bounded search ran out of room on a date it could not establish.
    ///
    /// A forward or period search inspects at most a documented bounded number
    /// of venue-local days. When the date it needs next is one it cannot
    /// establish, the search stops here rather than skipping it and claiming
    /// the next known session is the next session.
    SearchExhausted {
        /// The identity whose search was bounded.
        source: CalendarSource,
        /// The venue-local date the search needed to establish.
        date: NaiveDate,
        /// The last venue-local date the bounded search examined.
        bound: NaiveDate,
    },
}

impl CalendarQueryError {
    /// Returns the identity that cannot answer.
    #[must_use]
    pub const fn source(self) -> CalendarSource {
        match self {
            Self::BeforeSupportFloor { source, .. }
            | Self::OutsideCoveredRange { source, .. }
            | Self::UnresolvedGap { source, .. }
            | Self::SearchExhausted { source, .. } => source,
        }
    }

    /// Returns the venue-local date the query could not establish.
    #[must_use]
    pub const fn date(self) -> NaiveDate {
        match self {
            Self::BeforeSupportFloor { date, .. }
            | Self::OutsideCoveredRange { date, .. }
            | Self::UnresolvedGap { date, .. }
            | Self::SearchExhausted { date, .. } => date,
        }
    }
}

/// Renders an identity by its canonical `snake_case` wire name.
const fn identity_name(source: CalendarSource) -> &'static str {
    match source {
        CalendarSource::Exchange(exchange) => exchange.as_str(),
        CalendarSource::MarketHoursKey(key) => key.as_str(),
    }
}

impl core::fmt::Display for CalendarQueryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = identity_name(self.source());
        match *self {
            Self::BeforeSupportFloor { date, .. } => write!(
                f,
                "{name}: {date} precedes the {SUPPORT_FLOOR} venue-local support floor"
            ),
            Self::OutsideCoveredRange { date, .. } => {
                write!(
                    f,
                    "{name}: {date} is outside this identity's covered ranges"
                )
            }
            Self::UnresolvedGap { date, .. } => {
                write!(
                    f,
                    "{name}: {date} is an unresolved gap in the covered range"
                )
            }
            Self::SearchExhausted { date, bound, .. } => write!(
                f,
                "{name}: no answer for {date}; the bounded search ended at {bound}"
            ),
        }
    }
}

impl std::error::Error for CalendarQueryError {}
