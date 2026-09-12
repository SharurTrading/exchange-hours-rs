// SPDX-License-Identifier: MIT-0

//! NYMEX platinum and palladium Trading at Settlement (TAS) schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// NYMEX platinum-group Trading at Settlement books in America/Chicago: TAS on
// Platinum Futures (CME Globex `PLT`, security group PE) and Palladium TAS
// (`PAT`, group PX). One era each, seven and eight years after the COMEX
// launches in `metals_tas.rs`, on a different exchange and in different
// security groups — which is why they are not folded into that module and, in
// particular, why palladium is not folded into copper.
//
// DO NOT MERGE COPPER AND PALLADIUM ON THEIR COINCIDENT 12:00 CT CLOSE.
// Different exchange (COMEX versus NYMEX), different security group (HT versus
// PX), launches seven years apart, and different settlement determination
// ranges. The `globex_mini_grains` precedent — three keys, one envelope, three
// histories — applies directly: matching today's envelope is not matching the
// family.
//
// NO REGULAR SESSION. The same four channels as the COMEX TAS keys carry the
// negative for the TAS shape: NYMEX Rule 524.A enumerates CME Globex, Rule 526
// block trades and Rule 538 EFP/EFR and names no other route; CME's
// ProductSlate carries forty-five TAS records, all `floor "-"`, `floorVol "0"`
// and venues "Globex ClearPort "; the ContractSpecs API's "Open Outcry:" venue
// label exists and is never attached to a TAS hours line; and the Daily
// Bulletin carries TAS only as an ex-pit volume column. The 2009-2015 NYMEX
// Rule 524.A.1 pit clause discussed in `metals_tas.rs` reached gold and silver
// only, never these two — RA1107-4 enumerates the pit-eligible TAS contracts
// and names neither — and in any case ties the pit route to the underlying's
// own pit hours rather than giving the TAS root a window. Encode no pit or
// open-outcry window here.
// https://www.cmegroup.com/rulebook/files/cme-group-Rule-524.pdf
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul101811nymexandcomex001.pdf
//
// THE PRE-LAUNCH ERAS ARE SOURCED CLOSURES. Platinum: CME's own platinum
// contract specification contains the string "TAS" zero times at its capture of
// 2017-03-03, eleven weeks before the notice, and carries "TAS: PLT" at
// 2017-05-23. Palladium: neither root appears in any TAS eligibility list this
// crate holds before its notice — RA0907-4's eleven codes, RA1001-4/RA1002-4's
// fifteen, and the RA1005-4/RA1006-4/RA1101-4 tables all predate them. Nothing
// is carried back to the January-2010 floor on either key.
// https://web.archive.org/web/20170303065057id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html
// https://web.archive.org/web/20170523204255id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html
//
// LAUNCH DAYS, BOTH UNCONDITIONAL AND KEYED TO THE LOCAL OPENING DAY. Platinum:
// CME Globex notice 20170508, "Listing Platinum Futures TAS - May 21. Effective
// Sunday, May 21 (trade date Monday, May 22), Platinum futures TAS will be
// listed for trading on CME Globex and for submission for clearing via CME
// ClearPort. … Platinum Futures TAS | PLT | PE". Palladium: CME Globex notice
// 20181112, "Palladium TAS - This Week. Effective this Sunday, November 18
// (trade date Monday, November 19), Palladium TAS will be listed for trading on
// CME Globex … Palladium TAS | PAT | PX | 384". Neither item carries a
// readiness clause, and CME writes one when it means it: the 20180903 notice
// pairs an unconditional TAS listing with "Effective Sunday, September 30
// (trade date Monday, October 1), pending completion of all regulatory review
// periods, CME SONIA futures …". The grids open 17:00 CT on the Sunday, so the
// rows are keyed to 2017-05-21 and 2018-11-18.
// https://web.archive.org/web/20260214165515id_/https://www.cmegroup.com/notices/electronic-trading/2017/05/20170508.html
// https://web.archive.org/web/20260218023215id_/https://www.cmegroup.com/notices/electronic-trading/2018/11/20181112.html
//
// THE WINDOWS, IN CME'S OWN SESSION LANGUAGE, AND CONSTANT SINCE LAUNCH.
// Platinum's close is stated two days after launch and holds for two years in
// the specification's Trade At Marker Or Trade At Settlement Rules field —
// "Trading ceases daily at 1:05 PM ET." at 2017-05-23, 2017-06-10, 2017-08-02
// and 2019-07-17 — and then in hours form, "TAS: Sunday - Friday 6:00 p.m. -
// 1:05 p.m. (5:00 p.m. - 12:05 p.m. CT)", at 2021-04-11, on the archived
// ContractSpecs API at 2026-03-10 and 2026-04-10, and live. Palladium reads
// "TAS: Sunday - Friday 6:00 p.m. - 1:00 p.m. (5:00 p.m. - Noon CT)" at
// 2020-11-26 and 2021-05-13, the same instants at 2019-07-17, and "TAS: Sunday
// - Friday 5:00 p.m. - Noon CT" on the API at 2026-05-27 and live. So the
// closes are 12:05 and 12:00 CT, the opens 17:00 CT, and there are zero
// executable revision rows on either key. The 2017-2019 platinum statements
// give only the close; the 17:00 open comes from the 2021 hours form carried
// back, with no cutover asserted.
//
// OBSERVATION GAPS TO RECORD, and they are archive limits rather than sampling
// choices: platinum has no capture of that API path between 2021-04-11 and
// 2026-03-10, and palladium none between 2021-05-13 and 2026-05-27. A CDX
// prefix query filtered to status 200 returns exactly two captures for platinum
// and one for palladium. Both ends state the same instants and no source names
// a change, so the state carries — and this is why both rows are `Partial` with
// an executable gap.
//
// PRODUCT IDS, RECORDED BECAUSE THE OBVIOUS GUESS IS BACKWARDS: on CME's
// ContractSpecs API productId **445 is palladium** and **446 is platinum**, not
// the other way round. Verified from `ProductName` and `ProductCode.TAS` on
// both live records.
// https://web.archive.org/web/20190717122952id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html
// https://web.archive.org/web/20210411151252id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html
// https://web.archive.org/web/20201126020132id_/http://www.cmegroup.com/trading/metals/precious/palladium_contract_specifications.html
// https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/446
// https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/445
//
// ENCODE NO 16:00-17:00 CT DAILY BREAK ON PALLADIUM TAS. The palladium
// specification's TAS line at 2019-07-17 ends "… with a 60-minute break each
// day beginning at 4:00 p.m.(CT) (5:00 p.m. ET)". That clause is inherited
// boilerplate from the outright row one line above, restated with the CT/ET
// pair swapped: a book that closes at 12:00 CT cannot break at 16:00 CT, and
// CME had deleted the clause from the TAS line by 2020-11-26 while leaving the
// instants untouched. The handoff calls that specification the strongest
// document in the group; it is not. The strongest are RA1006-4 and RA1107-4,
// which call it "a TAS trading session" with a stated end and pre-date the
// specification channel by nine years.
//
// EVERY INTER-TRADE-DATE GAP EXCEEDS FOUR HOURS, SO IT IS `Closed`, NOT
// `Maintenance`: platinum 12:05->17:00 is 4h55m and palladium 5h. The crate
// retains an inter-trade-date gap as `Maintenance` only up to four elapsed
// hours. Stated here so nobody "fixes" it later.
//
// THE QUEUE. Both launched after the undated 2012 move of the Sunday TAS pause
// from 16:15 to 16:00 CT (see `metals_tas.rs` and issue #79), so their earliest
// sourced Sunday onset is 16:00 and nothing is withheld: CME's client-systems
// wiki states "a market pause … at 16:00:00 on Sunday and 16:45:00 Monday
// through Thursday. TAS groups then go into pre-open on Sundays between
// 16:00:00 and 16:01:00 Central time (CT), and Mondays through Thursdays
// between 16:45:00 and 16:46:00 CT." The nominal onset is encoded, never the
// randomized second; RA2302-5 confirms the randomization is still the live
// construction and that "Rule 524 permits the initiation of TAS … orders into
// CME Globex only subsequent to the beginning of each group's pre-open state",
// which is why the phase is `order_entry` and not `extended`. The wiki's
// blanket statement is carried back to each launch day with no cutover
// asserted, and that carry-back is the residual on these two rows' queue.
// https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/457223974/Trade+at+Settlement+-+TAS

static PLATINUM_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600 + 5 * 60,
}];
static PALLADIUM_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600,
}];

/// The platinum-group TAS queue: Sunday 16:00-17:00 and Monday-Thursday
/// 16:45-17:00 CT, as CME's client-systems wiki states it.
///
/// Nothing is withheld on the Sunday leg, unlike the COMEX TAS keys: both roots
/// launched after the undated 2012 move, so 16:00 is their earliest sourced
/// onset rather than a later state carried back over a wider one.
pub(crate) static PGM_TAS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

const fn profile(
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular: &[],
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

/// Before the listing notice the root was not TAS-eligible, and CME's own
/// eligibility lists and its pre-launch specification capture say so.
static CLOSED: StaticHoursProfile = profile(&[], &[]);

static PLATINUM_AT_LAUNCH: StaticHoursProfile =
    profile(PLATINUM_EXTENDED, PGM_TAS_ORDER_ENTRY_CURRENT);
static PALLADIUM_AT_LAUNCH: StaticHoursProfile =
    profile(PALLADIUM_EXTENDED, PGM_TAS_ORDER_ENTRY_CURRENT);

// Revision evidence — the day-level effective date, keyed to the local Sunday
// opening day of the first session it governs, and the primary source that
// states it (full quotations sit in the blocks above):
//   2017-05-21 "CME Globex notice 20170508"
//     https://www.cmegroup.com/notices/electronic-trading/2017/05/20170508.html
//   2018-11-18 "CME Globex notice 20181112"
//     https://www.cmegroup.com/notices/electronic-trading/2018/11/20181112.html
static PLATINUM_REVISIONS: &[Revision] = revisions![(
    2017,
    5,
    21,
    &PLATINUM_AT_LAUNCH,
    "CME Globex notice 20170508"
)];
static PALLADIUM_REVISIONS: &[Revision] = revisions![(
    2018,
    11,
    18,
    &PALLADIUM_AT_LAUNCH,
    "CME Globex notice 20181112"
)];

/// The current platinum TAS matching leg: Sunday-Thursday 17:00 CT wrapping to
/// 12:05 CT, with no Friday-evening reopen.
pub(crate) static PLATINUM_TAS_EXTENDED_CURRENT: &[SessionRule] = PLATINUM_EXTENDED;
/// The current palladium TAS matching leg, wrapping to 12:00 CT, and with no
/// daily break: the specification's "60-minute break" clause is inherited
/// boilerplate from the outright row and CME deleted it in 2020.
pub(crate) static PALLADIUM_TAS_EXTENDED_CURRENT: &[SessionRule] = PALLADIUM_EXTENDED;

pub(crate) fn platinum_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, PLATINUM_REVISIONS)
}

pub(crate) fn palladium_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, PALLADIUM_REVISIONS)
}
