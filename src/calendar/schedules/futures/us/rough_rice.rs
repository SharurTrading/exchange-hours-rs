// SPDX-License-Identifier: MIT-0

//! CBOT Rough Rice futures and options schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

use super::grains::{
    CBOT_EXTENDED_2012_05_20, CBOT_EXTENDED_AT_2010_FLOOR, CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2010_04_19, CBOT_ORDER_ENTRY_2011_12_27, CBOT_ORDER_ENTRY_2013_04_07,
    CBOT_ORDER_ENTRY_AT_2010_FLOOR, CBOT_ORDER_ENTRY_CURRENT, CBOT_REGULAR_0830_1315,
    CBOT_REGULAR_0930_1315, CBOT_REGULAR_CURRENT,
};

// Rough Rice futures (CME Globex `ZR`) and Rough Rice options (`OZR`), CBOT
// Rulebook chapters 17 and 17A, in America/Chicago.
//
// INHERITED PRE-2018 ERAS — NOT INDEPENDENTLY SOURCED FOR ROUGH RICE.
// Rough Rice ran on the standard CBOT grain and oilseed grid from the
// January-2010 audit floor until it diverged permanently on 2018-01-21, so the
// six eras before that day reuse the `CBOT_*` rule tables in `grains.rs` and
// the primary sources quoted there (2010-04-19, 2011-12-27, 2012-05-20,
// 2013-04-07, 2013-08-18, 2015-07-05). The CBOT grain and oilseed
// reorganisations name Rough Rice in their product lists, and 18-001 itself
// confirms the outgoing state by quoting ZR's then-current Sunday-Friday
// 19:00-07:45 CT extended leg — the grid `grains.rs` already serves for that
// day. What this crate has *not* done is source Rough Rice's own queue and
// RTH/ETH split separately for the 2010, 2012, 2013 and 2015 eras: those are
// inherited from the reviewed grains encoding, including its own undated
// 2012-05-20..2013-04-06 queue gap and its 08:00-08:30 CT morning Pre-Open.
// A later Rough Rice-specific finding repoints one era below; it must not
// edit the grains tables.
//
// 2018-01-21 — THE DIVERGENCE. CBOT Submission 18-001 certifies "the reduction
// of the extended trading hours on the CME Globex electronic trading platform
// for the Rough Rice Futures and Rough Rice Options contracts (the
// \"Contracts\") effective on Sunday, January 21, 2018 for trade date Monday,
// January 22, 2018", naming "Rough Rice Futures ZR 17" and "Rough Rice Options
// OZR 17A". Its table, headed "Current Extended Trading Hours" against
// "Extended Trading Hours Effective on Trade Date January 22, 2018", moves the
// leg from "Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT" to "Sunday - Thursday,
// 7:00 p.m. - 9:00 p.m. CT". This is an unconditional day-level effective date
// keyed to the venue-local opening Sunday, and the operator's own column
// heading classifies the evening leg as extended. Regular trading hours are
// untouched, so the 08:30-13:20 CT session carries through from SER-7395R.
// From this row Rough Rice has no midnight-wrapping session at all.
// https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf
// Archived, CME serves an anti-scraping block to automated clients:
// https://web.archive.org/web/20240314032026id_/https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf
//
// CURRENT GRID. CME's Rough Rice contract specification, read 2026-09-05,
// states under TRADING HOURS: "CME Globex: Sunday - Thursday, 7:00 p.m. - 9:00
// p.m. CT / Pre-Open Sunday: 4:00 p.m. - 7:00 p.m. CT // Monday - Friday: 8:30
// a.m. - 1:20 p.m. CT / Pre-Open Monday - Thursday: 4:45 p.m. - 7:00 p.m. CT",
// with PRODUCT CODE "CME Globex: ZR" and EXCHANGE RULEBOOK "CBOT 17". It
// therefore sources every session time this profile serves today, and confirms
// 18-001's grid is still the live one.
// https://www.cmegroup.com/markets/agriculture/grains/rough-rice/specs
//
// NO MORNING PRE-OPEN AND NO POST-CLOSE PRE-OPEN ARE MODELLED FROM 2018-01-21.
// Standard grains carry an 08:00-08:30 CT morning Pre-Open and a 14:30-16:00 CT
// post-close Pre-Open; CME's Rough Rice specification publishes neither, and no
// other primary source retrieved states either one for `ZR`. This profile
// therefore claims only the two evening queues the specification does publish,
// and drops the inherited grains morning Pre-Open and PCP at the divergence.
// Two things about that are worth stating plainly rather than hiding behind the
// basis label. First, omitting an order-entry window under-reports queueing,
// which is the safe direction: no window in which a trade can print is
// affected, so under the executable-windows-first rule this is the cheap error
// to make. Second, 18-001 is silent on the queues, so the day on which the
// morning Pre-Open and PCP stopped applying to Rough Rice is *not* sourced;
// attaching their withdrawal to the one sourced day in the interval is a
// modelling choice, not evidence, and it is why the ledger row is `Partial`
// rather than `Primary`.

/// The extended leg CBOT Submission 18-001 established: Sunday through Thursday
/// evenings, 19:00-21:00 CT. `close_ssm` exceeds `open_ssm`, so this rule does
/// not wrap — from 2018-01-21 Rough Rice has no session crossing local
/// midnight.
pub(crate) static ROUGH_RICE_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 21 * 3600,
}];

/// The only order-entry phases this profile claims after the divergence: the
/// Sunday 16:00-19:00 CT and Monday-Thursday 16:45-19:00 CT queues CME's Rough
/// Rice specification publishes, each running up to the electronic open.
/// Neither can match a trade.
pub(crate) static ROUGH_RICE_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 19 * 3600,
    },
];

/// Rough Rice's regular session is the standard CBOT 08:30-13:20 CT grid that
/// SER-7395R set on 2015-07-05; 18-001 leaves it untouched and the contract
/// specification restates it as "Monday - Friday: 8:30 a.m. - 1:20 p.m. CT".
/// The table is shared with `grains.rs` because the two genuinely coincide.
pub(crate) use super::grains::CBOT_REGULAR_CURRENT as ROUGH_RICE_REGULAR_CURRENT;

const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

// The six pre-divergence eras. Each is Rough Rice's own named profile built
// from the grain and oilseed tables it shared at the time; see `grains.rs` for
// the quotations and URLs behind every value.
static AT_2010_FLOOR: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_AT_2010_FLOOR,
);
static FROM_2010_04_19: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_2010_04_19,
);
static FROM_2011_12_27: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_2011_12_27,
);
static FROM_2012_05_20: StaticHoursProfile =
    profile(CBOT_REGULAR_0930_1315, CBOT_EXTENDED_2012_05_20, &[]);
static FROM_2013_04_07: StaticHoursProfile = profile(
    CBOT_REGULAR_0830_1315,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2013_04_07,
);
static FROM_2013_08_18: StaticHoursProfile = profile(
    CBOT_REGULAR_0830_1315,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
static FROM_2015_07_05: StaticHoursProfile = profile(
    CBOT_REGULAR_CURRENT,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
// The divergence: the 19:00-21:00 CT evening leg replaces the wrapping
// 19:00-07:45 one, and the queue set narrows to the two evening pre-opens the
// contract specification publishes.
static DATED_CURRENT: StaticHoursProfile = profile(
    ROUGH_RICE_REGULAR_CURRENT,
    ROUGH_RICE_EXTENDED_CURRENT,
    ROUGH_RICE_ORDER_ENTRY_CURRENT,
);

// Revision evidence — each row's day-level effective date and the primary
// source that states it. The first six are the CBOT grain and oilseed chain
// Rough Rice shared; their full quotations sit in `grains.rs`.
//   2010-04-19 "CME Globex notice 20100405"
//     https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
//   2011-12-27 "CFTC filing rul120711cbot001"
//     https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
//   2012-05-20 "CME market-data advisory 20120518"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
//   2013-04-07 "CME SER-6617 and GCC notice 2013-03-22"
//     https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
//   2013-08-18 "CME market-data advisory 20130812"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
//   2015-07-05 "CME SER-7395R"
//     https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
//   2018-01-21 "CBOT Submission 18-001"
//     https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf
static REVISIONS: &[Revision] = revisions![
    (2010, 4, 19, &FROM_2010_04_19, "CME Globex notice 20100405"),
    (
        2011,
        12,
        27,
        &FROM_2011_12_27,
        "CFTC filing rul120711cbot001"
    ),
    (
        2012,
        5,
        20,
        &FROM_2012_05_20,
        "CME market-data advisory 20120518"
    ),
    (
        2013,
        4,
        7,
        &FROM_2013_04_07,
        "CME SER-6617 and GCC notice 2013-03-22"
    ),
    (
        2013,
        8,
        18,
        &FROM_2013_08_18,
        "CME market-data advisory 20130812"
    ),
    (2015, 7, 5, &FROM_2015_07_05, "CME SER-7395R"),
    (2018, 1, 21, &DATED_CURRENT, "CBOT Submission 18-001"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
