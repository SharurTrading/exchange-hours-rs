// SPDX-License-Identifier: MIT-0

//! Stable canonical-string serde for [`MarketHoursKey`].

use core::fmt;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::MarketHoursKey;

macro_rules! market_hours_key_names {
    ($($variant:ident => $canonical:literal),+ $(,)?) => {
        const EXPECTED: &[&str] = &[$($canonical,)+];

        const fn canonical_name(key: MarketHoursKey) -> &'static str {
            match key {
                $(MarketHoursKey::$variant => $canonical,)+
            }
        }

        fn from_canonical_name<E>(value: &str) -> Result<MarketHoursKey, E>
        where
            E: Error,
        {
            match value {
                $($canonical => Ok(MarketHoursKey::$variant),)+
                _ => Err(E::unknown_variant(value, EXPECTED)),
            }
        }
    };
}

market_hours_key_names! {
    GlobexEquityIndex => "globex_equity_index",
    GlobexEnergy => "globex_energy",
    GlobexGrains => "globex_grains",
    GlobexFx => "globex_fx",
    CfeVix => "cfe_vix",
    Eurex => "eurex",
    IceUs => "ice_us",
    Sgx => "sgx",
    AlwaysOpen => "always_open",
}

impl Serialize for MarketHoursKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(canonical_name(*self))
    }
}

impl<'de> Deserialize<'de> for MarketHoursKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CanonicalNameVisitor;

        impl Visitor<'_> for CanonicalNameVisitor {
            type Value = MarketHoursKey;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a canonical market-hours key")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                from_canonical_name(value)
            }
        }

        deserializer.deserialize_str(CanonicalNameVisitor)
    }
}
