// SPDX-License-Identifier: MIT-0

//! Parsing and formatting for [`Exchange`]'s canonical names.
//!
//! One venue has exactly one `snake_case` name, shared by serde,
//! [`Exchange::as_str`], `Display`, and `FromStr` — callers holding a string
//! (a config key, a wire message, a CLI argument) parse it directly instead of
//! deserializing through a format crate or pattern-matching by hand.
//!
//! The names themselves live in `exchange/mod.rs`, where the `exchanges!` macro
//! generates the enum, [`Exchange::ALL`], and [`Exchange::as_str`] from one
//! table — a variant cannot be missing a name or missing from `ALL`, because
//! all three are the same rows. `FromStr` here searches `ALL` by `as_str`, so
//! it inherits that completeness; the test suite pins the remaining
//! hand-checkable facts (each name equals the serde wire form, `ALL` matches
//! an independently maintained list, in order).

use super::Exchange;

impl core::fmt::Display for Exchange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::str::FromStr for Exchange {
    type Err = ParseExchangeError;

    /// Parses a canonical `snake_case` venue name (`"nyse_arca"`,
    /// `"binance_futures"`, …) — exactly the strings [`Exchange::as_str`]
    /// returns and serde writes, matched case-sensitively.
    ///
    /// An unrecognized name is an error, **not** [`Exchange::Unknown`]:
    /// `Unknown` is a value a caller chooses deliberately (its name
    /// `"unknown"` parses like any other), never a silent default for a typo.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Exchange::ALL
            .iter()
            .copied()
            .find(|exchange| exchange.as_str() == s)
            .ok_or_else(|| ParseExchangeError { input: s.into() })
    }
}

/// The error [`Exchange`]'s [`FromStr`](core::str::FromStr) returns for a
/// string that is not a canonical venue name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseExchangeError {
    input: Box<str>,
}

impl ParseExchangeError {
    /// The string that failed to parse.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl core::fmt::Display for ParseExchangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?} is not a known exchange name; expected a snake_case name such as \"cme\" or \"nyse_arca\"",
            self.input
        )
    }
}

impl std::error::Error for ParseExchangeError {}
