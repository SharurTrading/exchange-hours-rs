// SPDX-License-Identifier: MIT-0

//! The single declarative source for exchange variants and canonical names.

/// Generates `Exchange`, `Exchange::ALL`, and `Exchange::as_str` from one
/// variant table, so the three cannot disagree: every row contributes its
/// variant, its `ALL` entry, and its canonical name together.
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
    };
}

pub(super) use exchanges;
