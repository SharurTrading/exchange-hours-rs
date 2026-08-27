// SPDX-License-Identifier: MIT-0

//! Bursa Malaysia cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// Bursa Malaysia: two continuous sessions bounded by order-entry/call and
// afternoon closing-auction/trade-at-last phases. The true lunch closure is
// 12:30–14:00. Trading Manual v2.0, dated 2009-10-26 and therefore already in
// force at the January-2010 audit floor, removed the v1.0 morning pre-close and
// prints continuous trading through 12:30. Dated v3.0 (2011), v5.0 (2012),
// v29.0 (2021), and current v36.0 manuals retain the same modeled grid; the
// intervening amendment register contains no in-scope clock change.
// https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb55ac75f36ca0c3028d8e7/Amended_Participating_Organisations__Trading_Manual.pdf
// https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5cda944139fba22dab508ab1/rules_bms_cir_rr2_110411.pdf
// https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb55ab65f36ca0c3028d8c2/1._Amendments_to_the_Rules_of_Bursa_Malaysia_Securities_Berhad_in_relation_to_Market_Making_and_Margin_Financing.pdf
// https://www.bursamalaysia.com/sites/5d809dcf39fba22790cad230/assets/60b1b8e85b711a63ee7f1395/POs_Trading_Manual_v28n_29.pdf
// https://www.bursamalaysia.com/sites/5d809dcf39fba22790cad230/assets/65ead6cbe6414a1e16de8b8e/POs_Trading_Manual_v36_4_March_2024.pdf
static BURSA_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 45 * 60,
    },
];
// No rule is reclassified as order entry here. Each of these three windows is a
// combined order-entry/call phase whose call leg matches and prints: the
// morning and afternoon pre-open windows end in the Theoretical Opening Price
// match, and the 16:45–17:00 window carries the closing call plus trade-at-last.
// The trading manuals cited above give the window bounds but the crate has no
// reachable primary source for the sub-phase boundary at which the call begins,
// so splitting would be a guess and the whole window stays tradeable.
static BURSA_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 14 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static BURSA_MALAYSIA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kuala_Lumpur,
    regular: BURSA_REGULAR,
    extended: BURSA_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) const CURRENT: &StaticHoursProfile = &BURSA_MALAYSIA_PROFILE;

/// No dated revision is recorded: the reviewed grid holds for the whole audit
/// window, so every instant resolves to the one profile. A sourced revision
/// later replaces this with a real timeline row and needs no routing change.
pub(crate) fn profile_at(_as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    CURRENT
}
