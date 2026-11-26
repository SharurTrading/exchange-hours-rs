// SPDX-License-Identifier: MIT-0

//! Built-in venue clocks embedded from Jiff's bundled IANA data at compile time.

use jiff::tz::TimeZone;

pub(crate) const UTC: &TimeZone = {
    static ZONE: TimeZone = TimeZone::UTC;
    &ZONE
};

macro_rules! time_zones {
    ($($region:ident { $($city:ident => $name:literal),+ $(,)? })+) => {
        $(
            #[allow(non_snake_case)]
            pub(crate) mod $region {
                use jiff::tz::TimeZone;

                $(
                    #[allow(non_upper_case_globals)]
                    pub(crate) const $city: &TimeZone = {
                        static ZONE: TimeZone = jiff::tz::get!($name);
                        &ZONE
                    };
                )+
            }
        )+
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
