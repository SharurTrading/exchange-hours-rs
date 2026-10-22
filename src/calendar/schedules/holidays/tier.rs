// SPDX-License-Identifier: MIT-0

//! The evidence tier a holiday row keys on.

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
