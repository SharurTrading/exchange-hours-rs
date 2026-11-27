// SPDX-License-Identifier: MIT-0

//! Named venue and reference clocks. Both clock sources use this same registry.

use jiff::{Timestamp, tz::TimeZone};

static UTC_ZONE: TimeZone = TimeZone::UTC;

/// A profile selector uses the same zone rules for every civil date and offset.
#[derive(Clone, Copy)]
pub(crate) struct SelectionInstant<'a> {
    pub(crate) instant: Timestamp,
    pub(crate) zones: ZoneSelection<'a>,
}

/// Embedded data occupies no calendar storage in a default build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ZoneSelection<'a> {
    Embedded(std::marker::PhantomData<&'a ()>),
    #[cfg(feature = "runtime-tzdb")]
    Runtime(SnapshotRef<'a>),
}

impl<'a> ZoneSelection<'a> {
    pub(crate) const EMBEDDED: Self = Self::Embedded(std::marker::PhantomData);

    pub(crate) fn resolve(self, zone: ZoneId) -> &'a TimeZone {
        match self {
            Self::Embedded(_) => zone.embedded(),
            #[cfg(feature = "runtime-tzdb")]
            Self::Runtime(snapshot) => snapshot.0.zone(zone),
        }
    }
}

#[cfg(feature = "runtime-tzdb")]
#[derive(Clone, Copy, Debug)]
pub(crate) struct SnapshotRef<'a>(pub(crate) &'a TimeZoneSnapshot);

#[cfg(feature = "runtime-tzdb")]
impl PartialEq for SnapshotRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

#[cfg(feature = "runtime-tzdb")]
impl Eq for SnapshotRef<'_> {}

#[cfg(feature = "runtime-tzdb")]
impl std::hash::Hash for SnapshotRef<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

macro_rules! time_zones {
    ($($region:ident { $($city:ident => $name:literal),+ $(,)? })+) => {
        #[expect(non_camel_case_types, reason = "Zone variants retain the city spelling in the IANA registry")]
        #[derive(Clone, Copy)]
        pub(crate) enum ZoneId {
            Utc,
            $( $($city,)+ )+
        }

        pub(crate) const UTC: ZoneId = ZoneId::Utc;
        $(
            #[expect(non_snake_case, reason = "Region modules retain the spelling in IANA identifiers")]
            pub(crate) mod $region {
                $(pub(crate) use super::ZoneId::$city;)+
            }
        )+

        impl ZoneId {
            pub(crate) const fn embedded(self) -> &'static TimeZone {
                match self {
                    Self::Utc => &UTC_ZONE,
                    $( $(Self::$city => {
                        static ZONE: TimeZone = jiff::tz::get!($name);
                        &ZONE
                    },)+ )+
                }
            }
        }

        /// An immutable set of venue and reference clocks from one `TZif` release.
        ///
        /// Construct it once and borrow it from any number of calendars using
        /// [`crate::ExchangeCalendar::with_time_zone_snapshot`]. UTC is intrinsic.
        #[cfg(feature = "runtime-tzdb")]
        #[expect(non_snake_case, reason = "Fields are generated directly from IANA city names")]
        pub struct TimeZoneSnapshot {
            release: String,
            $( $($city: TimeZone,)+ )+
        }

        #[cfg(feature = "runtime-tzdb")]
        impl TimeZoneSnapshot {
            /// Parses the compiled `TZif` bytes for every IANA name used by the crate.
            /// The caller supplies the release label and all files from that release.
            ///
            /// # Errors
            ///
            /// Returns the first loading or parsing error, including the requested
            /// name. A partially loaded snapshot cannot be selected.
            pub fn from_tzif<B: AsRef<[u8]>, E>(
                release: impl Into<String>,
                mut load: impl FnMut(&'static str) -> Result<B, E>,
            ) -> Result<Self, TimeZoneSnapshotError<E>> {
                let mut parse = |name: &'static str| {
                    let bytes = load(name).map_err(|source| TimeZoneSnapshotError::Load { name, source })?;
                    TimeZone::tzif(name, bytes.as_ref()).map_err(|source| TimeZoneSnapshotError::Parse { name, source })
                };
                Ok(Self {
                    release: release.into(),
                    $( $($city: parse($name)?,)+ )+
                })
            }

            /// Returns the release label supplied at construction.
            #[must_use]
            pub fn release(&self) -> &str {
                &self.release
            }

            fn zone(&self, id: ZoneId) -> &TimeZone {
                match id {
                    ZoneId::Utc => &UTC_ZONE,
                    $( $(ZoneId::$city => &self.$city,)+ )+
                }
            }
        }
    };
}

time_zones! {
    Africa {
        Johannesburg => "Africa/Johannesburg",
    }
    America {
        Chicago => "America/Chicago",
        Mexico_City => "America/Mexico_City",
        New_York => "America/New_York",
        Sao_Paulo => "America/Sao_Paulo",
        Toronto => "America/Toronto",
        Winnipeg => "America/Winnipeg",
    }
    Asia {
        Bangkok => "Asia/Bangkok",
        Dubai => "Asia/Dubai",
        Ho_Chi_Minh => "Asia/Ho_Chi_Minh",
        Hong_Kong => "Asia/Hong_Kong",
        Jakarta => "Asia/Jakarta",
        Kolkata => "Asia/Kolkata",
        Kuala_Lumpur => "Asia/Kuala_Lumpur",
        Manila => "Asia/Manila",
        Riyadh => "Asia/Riyadh",
        Seoul => "Asia/Seoul",
        Shanghai => "Asia/Shanghai",
        Singapore => "Asia/Singapore",
        Taipei => "Asia/Taipei",
        Tokyo => "Asia/Tokyo",
    }
    Australia {
        Sydney => "Australia/Sydney",
    }
    Europe {
        Amsterdam => "Europe/Amsterdam",
        Berlin => "Europe/Berlin",
        Brussels => "Europe/Brussels",
        Copenhagen => "Europe/Copenhagen",
        Dublin => "Europe/Dublin",
        Helsinki => "Europe/Helsinki",
        Istanbul => "Europe/Istanbul",
        Lisbon => "Europe/Lisbon",
        London => "Europe/London",
        Madrid => "Europe/Madrid",
        Paris => "Europe/Paris",
        Rome => "Europe/Rome",
        Stockholm => "Europe/Stockholm",
        Vienna => "Europe/Vienna",
        Zurich => "Europe/Zurich",
    }
    Pacific {
        Auckland => "Pacific/Auckland",
    }
    US {
        Central => "US/Central",
    }
}

#[cfg(feature = "runtime-tzdb")]
impl std::fmt::Debug for TimeZoneSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeZoneSnapshot")
            .field("release", &self.release)
            .finish_non_exhaustive()
    }
}

/// The zone that prevented construction of a complete runtime snapshot.
#[cfg(feature = "runtime-tzdb")]
#[derive(Debug)]
#[non_exhaustive]
pub enum TimeZoneSnapshotError<E> {
    /// The caller's source could not supply this named file.
    Load {
        /// The requested IANA name.
        name: &'static str,
        /// The error from the supplied loader.
        source: E,
    },
    /// Jiff could not parse the supplied `TZif` file.
    Parse {
        /// The requested IANA name.
        name: &'static str,
        /// The parser's error.
        source: jiff::Error,
    },
}

#[cfg(feature = "runtime-tzdb")]
impl<E: std::fmt::Display> std::fmt::Display for TimeZoneSnapshotError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load { name, source } => write!(f, "could not load {name}: {source}"),
            Self::Parse { name, source } => write!(f, "invalid TZif for {name}: {source}"),
        }
    }
}

#[cfg(feature = "runtime-tzdb")]
impl<E: std::error::Error + 'static> std::error::Error for TimeZoneSnapshotError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Load { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
        }
    }
}

#[cfg(feature = "runtime-tzdb")]
impl TimeZoneSnapshot {
    pub(crate) const fn selection(&self) -> ZoneSelection<'_> {
        ZoneSelection::Runtime(SnapshotRef(self))
    }

    /// Resolves an exchange's fixed profile at the given instant.
    #[must_use]
    pub fn hours_for_exchange(
        &self,
        exchange: crate::Exchange,
        at: Timestamp,
    ) -> crate::MarketHours {
        crate::ExchangeCalendar::new(exchange)
            .with_time_zone_snapshot(self)
            .hours_at(at)
    }

    /// Resolves a product family's fixed profile at the given instant.
    #[must_use]
    pub fn hours_for_market_hours_key(
        &self,
        key: crate::MarketHoursKey,
        at: Timestamp,
    ) -> crate::MarketHours {
        crate::ExchangeCalendar::for_market_hours_key(key)
            .with_time_zone_snapshot(self)
            .hours_at(at)
    }

    /// Returns the undated normal-week table from [`crate::session_profile`]
    /// with its venue zone resolved from this snapshot. For era and seasonal
    /// selection use [`Self::hours_for_market_hours_key`].
    #[must_use]
    pub fn session_profile(
        &self,
        key: crate::MarketHoursKey,
    ) -> crate::ZonedFuturesSessionProfile<'_> {
        let calendar =
            crate::ExchangeCalendar::for_market_hours_key(key).with_time_zone_snapshot(self);
        crate::ZonedFuturesSessionProfile {
            tz: calendar.zone(),
            ..*crate::session_profile(key)
        }
    }
}
