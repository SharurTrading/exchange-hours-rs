// SPDX-License-Identifier: MIT-0

//! The single declarative source for exchange variants and canonical names.

/// Generates `Exchange`, `Exchange::ALL`, `Exchange::as_str`, and its serde
/// implementations from one variant table, so they cannot disagree: every row
/// contributes its variant, its `ALL` entry, and its canonical name together.
macro_rules! exchanges {
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
            /// Every variant, in declaration (and therefore `Ord`) order.
            ///
            /// This is the version-you-compiled-against enumeration: the enum
            /// is `#[non_exhaustive]`, so new venues appear here in later
            /// releases. Generated from the same table as the enum itself, so
            /// it is complete by construction.
            $vis const ALL: &'static [$name] = &[$($name::$variant,)+];

            /// The variant's canonical `snake_case` name — the same string
            /// serde writes and [`FromStr`](std::str::FromStr) accepts.
            ///
            /// Generated from the same table as the enum, so every variant has
            /// exactly one name; the test suite asserts each one equals the
            /// serde wire form.
            #[must_use]
            $vis const fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $canonical,)+
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
                        formatter.write_str("a canonical exchange name")
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

pub(super) use exchanges;
