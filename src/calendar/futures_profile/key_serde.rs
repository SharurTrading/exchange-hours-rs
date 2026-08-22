// SPDX-License-Identifier: MIT-0

//! The single declarative source for market-hours keys and canonical names.

/// Generates `MarketHoursKey`, `MarketHoursKey::ALL`, `MarketHoursKey::as_str`,
/// and its serde implementations from one variant table, so none can drift.
macro_rules! market_hours_keys {
    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $canonical:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Every key, in declaration (and therefore [`Ord`]) order.
            ///
            /// This enumerates the variants in the crate version a caller
            /// compiled against. The enum is `#[non_exhaustive]`, so later
            /// minor releases may add entries.
            $vis const ALL: &'static [$name] = &[$($name::$variant,)+];

            /// The key's stable canonical `snake_case` name.
            ///
            /// This is the same string serde writes, [`Display`](core::fmt::Display)
            /// renders, and [`FromStr`](core::str::FromStr) accepts.
            #[must_use]
            $vis const fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $canonical,)+
                }
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl core::str::FromStr for $name {
            type Err = ParseMarketHoursKeyError;

            /// Parses the exact canonical `snake_case` name returned by
            /// `as_str`.
            ///
            /// An unrecognized name is an error; matching is case-sensitive.
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input {
                    $($canonical => Ok($name::$variant),)+
                    _ => Err(ParseMarketHoursKeyError::new(input)),
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct CanonicalNameVisitor;

                impl<'de> serde::de::Visitor<'de> for CanonicalNameVisitor {
                    type Value = $name;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        formatter.write_str("a canonical market-hours key")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        const EXPECTED: &[&str] = &[$($canonical,)+];

                        match value {
                            $($canonical => Ok($name::$variant),)+
                            _ => Err(E::unknown_variant(value, EXPECTED)),
                        }
                    }
                }

                deserializer.deserialize_str(CanonicalNameVisitor)
            }
        }
    };
}

pub(super) use market_hours_keys;

/// The error [`MarketHoursKey`](super::MarketHoursKey)'s
/// [`FromStr`](core::str::FromStr) implementation returns for an unknown name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMarketHoursKeyError {
    input: Box<str>,
}

impl ParseMarketHoursKeyError {
    pub(super) fn new(input: &str) -> Self {
        Self {
            input: input.into(),
        }
    }

    /// The string that failed to parse.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl core::fmt::Display for ParseMarketHoursKeyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?} is not a known market-hours key; expected a canonical snake_case name",
            self.input
        )
    }
}

impl std::error::Error for ParseMarketHoursKeyError {}
