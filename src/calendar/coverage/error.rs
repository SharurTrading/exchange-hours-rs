// SPDX-License-Identifier: MIT-0

//! The explicit coverage-error vocabulary for identity-backed queries
//! (LAW-COVERAGE).

use jiff::civil::Date;

use super::SUPPORT_FLOOR;
use crate::calendar::exchange_calendar::CalendarSource;

use super::baseline::NormalWeekBaseline;
use crate::calendar::time_zones::ZoneSelection;

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
pub enum CalendarQueryErrorKind {
    /// The date precedes the venue-local support floor.
    ///
    /// The floor is a *local* date ([`SUPPORT_FLOOR`]), so two identities in
    /// opposing zones reach it at different UTC instants.
    BeforeSupportFloor {
        /// The identity that cannot answer.
        source: CalendarSource,
        /// The offending venue-local date.
        date: Date,
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
        date: Date,
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
    /// [`CalendarQueryError::normal_week_baseline`] names the sourced week the refusal sits
    /// inside.
    UnresolvedGap {
        /// The identity that cannot answer.
        source: CalendarSource,
        /// The withheld venue-local date.
        date: Date,
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
        date: Date,
        /// The last venue-local date the bounded search examined.
        bound: Date,
    },
}

impl CalendarQueryErrorKind {
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
    pub const fn date(self) -> Date {
        match self {
            Self::BeforeSupportFloor { date, .. }
            | Self::OutsideCoveredRange { date, .. }
            | Self::UnresolvedGap { date, .. }
            | Self::SearchExhausted { date, .. } => date,
        }
    }
}
/// An identity-backed query refusal with the clock rules that produced it.
///
/// Match [`Self::kind`] to distinguish a coverage refusal from a bounded search
/// that ran out of room. [`Self::normal_week_baseline`] derives the available
/// context from those same rules for the lifetime of the error.
/// Embedded refusals carry the precompiled clock. A runtime refusal retains a
/// shared handle to the immutable snapshot only when the query fails.
///
/// Equality and hashing include both the structured kind and the selected clock.
/// To compare the refusal independently of clock provenance, compare its kind.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalendarQueryError {
    kind: CalendarQueryErrorKind,
    clock: ErrorClock,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ErrorClock {
    Embedded,
    #[cfg(feature = "runtime-tzdb")]
    Snapshot(crate::TimeZoneSnapshot),
}

impl CalendarQueryError {
    pub(in crate::calendar) fn new(kind: CalendarQueryErrorKind, zones: ZoneSelection<'_>) -> Self {
        let clock = match zones {
            ZoneSelection::Embedded(_) => ErrorClock::Embedded,
            #[cfg(feature = "runtime-tzdb")]
            ZoneSelection::Runtime(snapshot) => ErrorClock::Snapshot(snapshot.clone()),
        };
        Self { kind, clock }
    }

    /// Returns the category and identifying fields of this refusal.
    #[must_use]
    pub const fn kind(&self) -> CalendarQueryErrorKind {
        self.kind
    }

    /// Returns the identity that cannot answer.
    #[must_use]
    pub const fn source(&self) -> CalendarSource {
        self.kind.source()
    }

    /// Returns the venue-local date the query could not establish.
    #[must_use]
    pub const fn date(&self) -> Date {
        self.kind.date()
    }

    /// Returns the release label of the time zone rules used for the query.
    #[must_use]
    pub fn tzdb_release(&self) -> &str {
        match &self.clock {
            ErrorClock::Embedded => crate::EMBEDDED_TZDB_VERSION,
            #[cfg(feature = "runtime-tzdb")]
            ErrorClock::Snapshot(snapshot) => snapshot.release(),
        }
    }

    /// True if the refusal used the crate's precompiled time zone rules.
    #[must_use]
    pub const fn uses_embedded_tzdb(&self) -> bool {
        matches!(self.clock, ErrorClock::Embedded)
    }

    /// Returns the sourced normal-week baseline for the refused date, resolved
    /// with this query's clocks. A date before the support floor or the identity's
    /// sourced normal week has no baseline. A baseline states the ordinary
    /// weekday sessions and leaves the holiday arrangement unresolved.
    #[must_use]
    pub fn normal_week_baseline(&self) -> Option<NormalWeekBaseline> {
        let zones = match &self.clock {
            ErrorClock::Embedded => ZoneSelection::EMBEDDED,
            #[cfg(feature = "runtime-tzdb")]
            ErrorClock::Snapshot(snapshot) => snapshot.selection(),
        };
        super::baseline::for_error(self.kind, zones)
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
        match self.kind {
            CalendarQueryErrorKind::BeforeSupportFloor { date, .. } => write!(
                f,
                "{name}: {date} precedes the {SUPPORT_FLOOR} venue-local support floor"
            ),
            CalendarQueryErrorKind::OutsideCoveredRange { date, .. } => {
                write!(
                    f,
                    "{name}: {date} is outside this identity's covered ranges"
                )
            }
            CalendarQueryErrorKind::UnresolvedGap { date, .. } => {
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
            CalendarQueryErrorKind::SearchExhausted { date, bound, .. } => write!(
                f,
                "{name}: no answer for {date}; the bounded search ended at {bound}"
            ),
        }?;
        #[cfg(feature = "runtime-tzdb")]
        {
            let origin = if self.uses_embedded_tzdb() {
                "embedded tzdb"
            } else {
                "tzdb"
            };
            write!(f, " [{origin} {}]", self.tzdb_release())?;
        }
        Ok(())
    }
}

impl std::error::Error for CalendarQueryError {}
