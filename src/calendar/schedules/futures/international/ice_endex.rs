// SPDX-License-Identifier: MIT-0

//! ICE Endex Dutch TTF Natural Gas Futures.

use chrono_tz::{America, Europe};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_ONLY, TUE_FRI};
use crate::calendar::schedules::timeline::{effective_date, local_date, reference_delta_seconds};

// ICE Endex is scoped to the post-combination Dutch TTF Natural Gas Futures
// contract. ICE's 2013-08-31 WebICE hours table shows the transferred contract's
// 07:45 pre-open / 08:00-18:00 CET grid immediately before the handoff. Circular
// 13/107 moves the equivalent contract to ICE Endex on 2013-10-07 and explicitly
// keeps it on the same ICE platform; the completion release confirms combined
// WebICE trading from that date. The first archived Endex product page and March
// 2014 schedule retain the 08:00 open, 18:00 close, and pre-market phase. The
// original predecessor-venue onset of 07:45 is outside this identity's modeled
// interval; the transfer selector does not claim that the phase began in 2013.
// E26/004 proves the immediately preceding grid and changes it on 2026-04-13 to
// a 21-hour day, including the exact one-hour-earlier profile used while US and
// Central-European daylight time differ.
// https://web.archive.org/web/20130831104114id_/https://www.theice.com/productguide/Search.shtml?tradingHours=
// https://www.ice.com/publicdocs/circulars/10010%20attach%201%20-%20TTF%20Nat%20Gas%20Contract%20Spec.pdf
// https://www.ice.com/publicdocs/circulars/13107.pdf
// https://www.ice.com/publicdocs/circulars/13134.pdf
// https://ir.theice.com/press/news-details/2013/IntercontinentalExchange-Completes-Trading-and-Clearing-Transition-for-ICE-Endex-Futures-Markets/default.aspx
// https://web.archive.org/web/20140215045503id_/https://www.theice.com/productguide/ProductSpec.shtml?specId=27996665
// https://www.ice.com/publicdocs/endex/circulars/ICE-Endex-Derivatives-Rules-V21-2-201403-Appendix-B-1-Operating-Time-Schedule.pdf
// https://www.ice.com/publicdocs/endex/ICE_Endex_Rules.pdf
// https://www.ice.com/publicdocs/endex/circulars/E19003_attach_2.pdf
// https://www.ice.com/publicdocs/endex/circulars/E21013_attach_2.pdf
// https://www.ice.com/publicdocs/endex/circulars/E26004.pdf
// https://www.ice.com/products/27996665/Dutch-TTF-Gas-Futures
static PRE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 18 * 3600,
}];
static PRE_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 45 * 60,
    close_ssm: 8 * 3600,
}];
static ALIGNED_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 50 * 60,
        close_ssm: 23 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 3600 + 50 * 60,
        close_ssm: 23 * 3600,
    },
];
static ALIGNED_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 40 * 60,
        close_ssm: 23 * 3600 + 50 * 60,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 3600 + 40 * 60,
        close_ssm: 3600 + 50 * 60,
    },
];
static MISMATCH_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 22 * 3600 + 50 * 60,
        close_ssm: 22 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 50 * 60,
        close_ssm: 22 * 3600,
    },
];
static MISMATCH_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 22 * 3600 + 40 * 60,
        close_ssm: 22 * 3600 + 50 * 60,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 40 * 60,
        close_ssm: 50 * 60,
    },
];

static PRE: StaticHoursProfile = amsterdam_profile(PRE_REGULAR, PRE_EXTENDED);
static CLOSED: StaticHoursProfile = amsterdam_profile(&[], &[]);
static EXTENSION_EVE: StaticHoursProfile = amsterdam_profile(
    &[SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 50 * 60,
        close_ssm: 23 * 3600,
    }],
    &[SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 40 * 60,
        close_ssm: 23 * 3600 + 50 * 60,
    }],
);
pub(crate) static CURRENT: StaticHoursProfile =
    amsterdam_profile(ALIGNED_REGULAR, ALIGNED_EXTENDED);
static MISMATCH: StaticHoursProfile = amsterdam_profile(MISMATCH_REGULAR, MISMATCH_EXTENDED);

const fn amsterdam_profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: Europe::Amsterdam,
        regular,
        extended,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

const TRANSFER: chrono::NaiveDate = effective_date(2013, 10, 7);
const EXTENSION_OPENING_DAY: chrono::NaiveDate = effective_date(2026, 4, 12);
const EXTENSION: chrono::NaiveDate = effective_date(2026, 4, 13);

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Europe::Amsterdam);
    if day < TRANSFER {
        return &CLOSED;
    }
    if day < EXTENSION_OPENING_DAY {
        return &PRE;
    }
    if day < EXTENSION {
        return &EXTENSION_EVE;
    }
    if reference_delta_seconds(as_of, Europe::Amsterdam, America::New_York) == -6 * 3600 {
        &CURRENT
    } else {
        &MISMATCH
    }
}
