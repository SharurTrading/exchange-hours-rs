// SPDX-License-Identifier: MIT-0

//! One mutually exclusive market-state classification.

/// The effective state of a schedule at one UTC instant.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionState {
    /// A primary or regular session is open.
    OpenRegular,
    /// A tradeable extended, electronic, or auction session is open.
    OpenExtended,
    /// An order-entry-only phase is active: orders may be entered, amended or
    /// cancelled, but **no trade can match**.
    ///
    /// Distinct from [`SessionState::OpenExtended`], which is a session in
    /// which trades genuinely print. A consumer building bars must not emit one
    /// for this state: there is no price.
    OrderEntry,
    /// Trading is paused between two phases of the same trade date.
    Halt,
    /// The schedule is in a short operational maintenance break.
    ///
    /// This normally separates trade dates. A continuously traded-week
    /// profile can explicitly retain an operator-designated short maintenance
    /// window inside one trade date.
    Maintenance,
    /// The schedule is otherwise closed.
    Closed,
}
