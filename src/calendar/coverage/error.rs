// SPDX-License-Identifier: MIT-0

//! The explicit coverage-error vocabulary for identity-backed queries
//! (LAW-COVERAGE).

use chrono::NaiveDate;

use super::SUPPORT_FLOOR;
use crate::calendar::exchange_calendar::CalendarSource;

use super::baseline::NormalWeekBaseline;

/// Why an identity-backed date-aware query cannot answer.
///
/// LAW-COVERAGE requires an unsupported date to be an explicit error rather
/// than a claim that a market is open or closed, and it requires an unsupported
/// date to stay distinguishable from a bounded search that simply ran out of
/// room to look. The four variants are that distinction:
///
/// - [`Self::BeforeSupportFloor`] — the instant or date precedes the permanent
///   2010-01-01 local-date floor;
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
///
/// **The mapping from [`DateCoverage`](super::DateCoverage) to these variants
/// is exact in one direction only.** A date the metadata calls
/// [`DateCoverage::Covered`](super::DateCoverage::Covered) promises that every
/// query addressed to an instant of it answers, or refuses naming a date the
/// metadata itself does not call covered (#151) — a wrapped session can open on
/// the previous civil day, and the next session's trade date can lie beyond the
/// day, so the refusal a caller sees on a covered date names the *neighbour*
/// the answer depends on, never a market state for the covered date itself.
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
    /// withholds as `Unsourced`, or the date sits on a one-flank bridged span
    /// **below the identity's first audited window**, whose holiday-table
    /// classification refuses (issue #296, the 2026-10-07 ruling).
    ///
    /// The withheld case is the one LAW-HOLIDAY-SCOPE means by "a date without
    /// a row is audited normal, while `Unsourced` expressly withholds that
    /// claim". The one-flank case is the ruling's asymmetry with two-flank
    /// bridged spans: a date whose session questions answer from the sourced
    /// normal week still refuses `is_closed_trade_date`, because no audited
    /// window brackets it from below and nothing witnesses the holiday layer —
    /// two flanks bracket a span, one does not. Either way the crate has no
    /// answer and will not guess one, and
    /// [`Self::normal_week_baseline`] names the sourced week the refusal sits
    /// inside.
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

    /// Returns the sourced normal-week baseline the refused date sits inside,
    /// or `None` when the crate states none (issue #296, Tier 3).
    ///
    /// This is the context a consumer needs to make its own call beside a
    /// refusal: the weekday the refused date falls on and the normal-week
    /// windows the identity's sourced timeline serves there — "Wednesday;
    /// normal week open 09:30-16:30" — rendered by
    /// [`NormalWeekBaseline`]'s `Display`. The enrichment is additive: the
    /// refusal is unchanged, and the baseline says nothing about the holiday
    /// arrangement the refusal withholds.
    ///
    /// The baseline is absent where claiming one would state a sourced fact
    /// the ledger does not record: the date precedes the support floor, or the
    /// identity's normal week is carried backwards below its recorded horizon
    /// at the date.
    #[must_use]
    pub fn normal_week_baseline(self) -> Option<NormalWeekBaseline> {
        super::baseline::for_error(self)
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
                )?;
                // Tier 3 (issue #296): the refusal carries the normal-week
                // baseline it sits inside, so a consumer can make its own call.
                // The holiday arrangement stays unsourced — the baseline is
                // context, never an answer.
                if let Some(baseline) = self.normal_week_baseline() {
                    write!(f, " ({baseline}; the holiday arrangement is unsourced)")?;
                }
                Ok(())
            }
            Self::SearchExhausted { date, bound, .. } => write!(
                f,
                "{name}: no answer for {date}; the bounded search ended at {bound}"
            ),
        }
    }
}

impl std::error::Error for CalendarQueryError {}
