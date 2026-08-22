// SPDX-License-Identifier: MIT-0

//! One mutually exclusive market-state classification.

/// The effective state of a schedule at one UTC instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionState {
    /// A primary or regular session is open.
    OpenRegular,
    /// An extended, electronic, auction, or order-entry session is open.
    OpenExtended,
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
