// SPDX-License-Identifier: MIT-0

//! NYSE-family US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile, equity_profile_with_entry};
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Pillar order-entry edges. NYSE's hours table lists "Order Entry" starting at
// 06:30 (02:30 on Arca) with the first executable phase — the Early Trading
// Session — beginning only at 07:00 (04:00 on Arca). Nothing can print inside
// these windows: they exist so orders can be entered, amended and cancelled
// ahead of the first matching session, so they are `order_entry`, not
// `extended`.
// https://www.nyse.com/markets/hours-calendars
static ENTRY_0630_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 30 * 60,
    close_ssm: 7 * 3600,
}];

static ENTRY_0230_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

/// NYSE Arca's pre-2021 acceptance edge: 30 minutes before the 04:00 open.
static ENTRY_0330_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

/// NYSE's executable Early Trading Session, 07:00–09:30 (Tapes B and C).
static EXTENDED_0700_0930: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static EXTENDED_0700_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static NATIONAL_0800_1830: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 18 * 3600 + 30 * 60,
    },
];

static NATIONAL_0800_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static NATIONAL_0800_1700: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

static NYSE_CHICAGO_EXTENDED_PRE_PILLAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

// NYSE accepts orders from 06:30 ET. Tape A queues them for the core opening;
// Tapes B/C also enter an active early session at 07:00. The 06:30–07:00 leg is
// therefore acceptance only — no trade can print before the Early Trading
// Session opens — and is classified `order_entry`; 07:00–09:30 stays Extended
// because Tape B/C trades execute there.
//
// The acceptance edge is a **rulebook** provision, not an operator system
// setting: NYSE Rule 7.34(a)(1), adopted with the UTP Pillar filing
// (SR-NYSE-2017-36), sets the Early Trading Session at 07:00 and provides that
// "the Exchange would begin accepting orders 30 minutes before the Early
// Trading Session begins, which means order entry acceptance would begin at
// 6:30 a.m. Eastern Time". Two later NYSE filings state the production day
// unconditionally — "On April 9, 2018, the Exchange began trading UTP
// Securities on the Exchange on the Pillar trading platform" — and the
// exchange's own Trader Update of 2018-03-27 announced the same day. That is
// the day the venue envelope first carried both phases, so it is the onset row
// here; NYSE-listed (Tape A) symbols migrated to Pillar in tranches from
// 2019-08-05, which extends the same phases to more symbols without moving the
// venue-level onset (see the envelope convention in `AGENTS.md`).
//
// Before 2018-04-09 the modelled grid stays the sourced 09:30–16:00 core
// session. Two pre-Pillar phases remain unmodelled and are recorded as the
// row's residual gap in `docs/schedules/verification.md`: the pre-Pillar Tape A
// order-acceptance edge, and Crossing Session II (16:00–18:30), the surviving
// leg of the Off-Hours Trading Facility, which ran from before the audit floor
// until the exchange decommissioned it effective 18:30 on 2024-01-31. Crossing
// Session I was eliminated in 2009, below the floor. Modelling the crossing
// leg would widen the executable envelope, so it waits on its full amendment
// chain rather than being added from the endpoints.
// https://www.federalregister.gov/documents/2017/08/09/2017-16742/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-proposed-rule-change
// https://www.federalregister.gov/documents/2018/03/29/2018-06339/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-amendment-no-1-and
// https://www.federalregister.gov/documents/2018/05/18/2018-10606/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
// https://www.federalregister.gov/documents/2019/08/01/2019-16365/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
// https://www.federalregister.gov/documents/2024/02/29/2024-04168/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
// https://www.nyse.com/trade/hours-calendars?os=.
// https://www.nyse.com/markets/hours-calendars
static NYSE_HISTORICAL_PROFILE: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_0930, ENTRY_0630_0700);

// Row evidence:
//   2018-04-09 "SEC 34-83230 (NYSE UTP Pillar production)"
//     83 FR 23313: "On April 9, 2018, the Exchange began trading UTP
//     Securities on the Exchange on the Pillar trading platform."
//     Restated in 84 FR 37702 and announced in the NYSE Trader Update of
//     2018-03-27, "NYSE will begin trading Tape B and C securities on
//     April 9, 2018".
static NYSE_REVISIONS: &[Revision] = revisions![(
    2018,
    4,
    9,
    &NYSE_PROFILE,
    "SEC 34-83230 (NYSE UTP Pillar production)"
),];

pub(crate) fn nyse_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_HISTORICAL_PROFILE,
        NYSE_REVISIONS,
    )
}

// NYSE Arca accepts and queues orders at 02:30 before its 04:00 active early
// session. That queue matches nothing, so it is `order_entry`; the 04:00–20:00
// execution grid stays Extended and predates the audit floor.
//
// The acceptance edge is a **rulebook** provision throughout. Pre-Pillar Rule
// 7.35(a)(1) had the Corporation begin accepting orders 30 minutes before the
// 04:00 Opening Session, i.e. at 03:30; the Pillar I filing carried that text
// into Rule 7.34-E(a)(1) "without any substantive differences". No reviewed
// primary source names a change to that 30-minute edge between the January-2010
// floor and the 2015 Pillar filing, so 03:30 is carried back to the floor and
// no revision row is asserted for it. SR-NYSEArca-2021-71 then amended Rule
// 7.34-E(a)(1) from 30 to 90 minutes (02:30); the filing itself deferred
// production to a Trader Update, and that Trader Update — issued 2021-08-11 and
// repeated as a reminder on 2021-09-09 — states the day unconditionally:
// "Beginning on Monday, September 13, 2021, NYSE Arca will change the time for
// order entry to 2:30 a.m. ET. Currently, NYSE Arca opens for order entry at
// 3:30 a.m. ET." Archived captures of the exchange's own hours page agree,
// showing 3:30 on 2021-09-12 and 2:30 on 2021-09-27.
//
// The announced 23-hour Overnight Session (from Sunday 2026-12-06) is a future
// change tracked in `docs/schedules/updating.md`, not encoded here.
// https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf
// https://www.federalregister.gov/documents/2015/05/19/2015-12028/self-regulatory-organizations-nyse-arca-inc-notice-of-filing-of-proposed-rule-change-adopting-new
// https://www.federalregister.gov/documents/2021/08/18/2021-17673/self-regulatory-organizations-nysearca-inc-notice-of-filing-and-immediate-effectiveness-of-proposed
// https://www.nyse.com/trader-update/history#110000372318
// https://www.nyse.com/trade/hours-calendars?os=.
// https://www.nyse.com/publicdocs/nyse/data/ArcaBook_Client_Specification.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-arca/rule-filings/filings/2021/SR-NYSEArca-2021-71.pdf
static NYSE_ARCA_PRE_2021_09_13: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0330_0400);
pub(crate) static NYSE_ARCA_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

// Row evidence:
//   2021-09-13 "NYSE Trader Update 2021-08-11"
//     https://www.nyse.com/trader-update/history#110000372318
//     Reminder of 2021-09-09: https://www.nyse.com/trader-update/history#110000381060
static NYSE_ARCA_REVISIONS: &[Revision] = revisions![(
    2021,
    9,
    13,
    &NYSE_ARCA_PROFILE,
    "NYSE Trader Update 2021-08-11"
),];

pub(crate) fn nyse_arca_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_ARCA_PRE_2021_09_13,
        NYSE_ARCA_REVISIONS,
    )
}

// NYSE American's Pillar launch added its current 06:30 order-acceptance edge
// around the 07:00–20:00 execution grid on 2017-07-24. The acceptance edge is
// `order_entry` — 06:30–07:00 matches nothing — and the execution grid is
// Extended. Before Pillar, the sourced continuous session was 09:30–16:00.
//
// Like its NYSE and NYSE Arca siblings, the acceptance edge is a **rulebook**
// provision: NYSE American Rule 7.34E(a)(1), adopted by SR-NYSEMKT-2017-01,
// puts the Early Trading Session at 07:00 and provides that the exchange "would
// begin accepting orders 30 minutes before the Early Trading Session begins,
// which means order entry acceptance would begin at 6:30 a.m." The Commission
// separately records the production day without condition: "NYSE American's
// cash equities market transitioned to Pillar on July 24, 2017."
//
// Legacy off-hours crosses are still not backfilled. That residue is an
// **executable** gap, not an order-entry one: at the audit floor NYSE Amex
// participated in the NYSE Off-Hours Trading Facility crossing sessions, and
// the 2024 NYSE price-list filing records that Crossing Session I was
// eliminated in 2009 while Crossing Session II ran until 18:30 on 2024-01-31.
// The exact January-2010 NYSE Amex phase table and its complete amendment chain
// have not been established, and widening the executable envelope from two
// endpoints is exactly the inference this crate refuses; this history therefore
// remains explicitly partial.
// https://www.sec.gov/rules/sro/nyseamex/2010/34-61890.pdf
// https://www.federalregister.gov/documents/2017/02/15/2017-02990/self-regulatory-organizations-nyse-mkt-llc-notice-of-filing-of-proposed-rule-change-to-adopt-new
// https://www.federalregister.gov/documents/2018/03/29/2018-06339/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-amendment-no-1-and
// https://www.federalregister.gov/documents/2024/02/29/2024-04168/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
// https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_March_2017.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf
static NYSE_AMERICAN_PRE_PILLAR: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_AMERICAN_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);

static AMERICAN_REVISIONS: &[Revision] = revisions![(
    2017,
    7,
    24,
    &NYSE_AMERICAN_PROFILE,
    "NYSE American Pillar update 2017-07-21"
),];

pub(crate) fn nyse_american_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_AMERICAN_PRE_PILLAR,
        AMERICAN_REVISIONS,
    )
}

static NATIONAL_PRE_2010_08_02: StaticHoursProfile = equity_profile(NATIONAL_0800_1830);
static NATIONAL_2010_08_02: StaticHoursProfile = equity_profile(NATIONAL_0800_2000);
static NATIONAL_2015_12_22: StaticHoursProfile = equity_profile(NATIONAL_0800_1700);
pub(crate) static NYSE_NATIONAL_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);

// NSX's operative 2010 filing dates the 18:30→20:00 close extension to
// 2010-08-02. Its immediately operative 2014 filing shortened that close to
// 17:00 on 2014-05-16. Trading ceased after the 2014-05-30 close. The SEC's
// 2015 approval says the resumed marketplace would use the rules then in
// effect, and the exchange's SEC-filed Form 1 dates its phased relaunch to
// 2015-12-22. It ceased again before the 2017-02-01 open. NYSE National
// launched on Pillar on 2018-05-21 with its current 06:30 order-acceptance edge
// around the 07:00–20:00 execution grid; the acceptance edge is `order_entry`
// because no trade prints before the 07:00 Early Trading Session.
//
// The pre-2018 NSX grid is a **rulebook** provision too: NSX Rule 11.1 ("Hours
// of Trading") fixed the sessions, and the companion 2010 filing states the
// floor-era table outright — "the Exchange's Regular Trading Hours ... are from
// 9:30 a.m. until 4 p.m. Eastern Time. The pre-Regular Trading Hours trading
// session is from 8 a.m. until 9:30 a.m. ET, and the post-Regular Trading Hours
// trading session is from 4 p.m. until 6:30 p.m. ET." That is exactly the
// baseline modelled below, so it is carried back to the January-2010 floor.
// No reviewed primary source describes an NSX order-acceptance phase distinct
// from those three trading sessions, so none is modelled; because Rule 11.1(a)
// also let the Board set business hours by Regulatory Circular, and NSX's own
// circular archive is unreachable (nsx.com serves only index pages through the
// web archive, with the circular PDFs themselves gone), a Board-noticed
// acceptance edge could not be recovered even if one existed. The residual risk
// is therefore under-reporting an NSX queue, never over-reporting one.
// https://www.federalregister.gov/documents/2010/08/04/2010-19225/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate
// https://www.federalregister.gov/documents/2010/08/10/2010-19652/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate
// https://www.federalregister.gov/documents/2014/05/28/2014-12229/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate
// https://www.federalregister.gov/documents/2018/03/13/2018-04962/self-regulatory-organizations-nyse-national-inc-notice-of-filing-of-proposed-rule-change-to-support
// https://www.sec.gov/files/rules/sro/nsx/2010/34-62643.pdf
// https://www.sec.gov/files/rules/sro/nsx/2014/34-72215.pdf
// https://www.sec.gov/files/rules/sro/nsx/2014/34-72107.pdf
// https://www.sec.gov/files/rules/sro/nsx/2015/34-76640.pdf
// https://www.sec.gov/Archives/edgar/vprr/1601/16019238.pdf
// https://www.sec.gov/files/rules/sro/nsx/2017/34-80018.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-national/rule-filings/filings/2020/SR-NYSENat-2020-05.pdf
static NATIONAL_REVISIONS: &[Revision] = revisions![
    (2010, 8, 2, &NATIONAL_2010_08_02, "SEC 34-62643"),
    (2014, 5, 16, &NATIONAL_2015_12_22, "SEC 34-72215"),
    (2014, 5, 31, &CLOSED_NEW_YORK, "SEC 34-72107"),
    (
        2015,
        12,
        22,
        &NATIONAL_2015_12_22,
        "NSX SEC Form 1 relaunch filing"
    ),
    (2017, 2, 1, &CLOSED_NEW_YORK, "SEC 34-80018"),
    (2018, 5, 21, &NYSE_NATIONAL_PROFILE, "SR-NYSENat-2020-05"),
];

pub(crate) fn nyse_national_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NATIONAL_PRE_2010_08_02,
        NATIONAL_REVISIONS,
    )
}

// NYSE Texas is the same registered exchange formerly called NYSE Chicago and
// CHX; its 2025-03-28 conversion and rename were non-substantive. At the audit
// floor, CHX accepted orders from 07:00 through 17:00 ET: early trading to
// 09:30, core trading to 16:00, the late session to 16:15, and cross-only late
// crossing through 17:00. Its Pillar migration established the current 06:30
// order-acceptance edge around the 07:00–20:00 three-session grid on 2019-11-04.
// That 06:30–07:00 edge is `order_entry`; the pre-Pillar CHX slice stays wholly
// Extended because its 07:00 early session, 16:00–16:15 late session and
// 16:15–17:00 cross-only late crossing all print trades.
// https://www.sec.gov/rules/sro/chx/2009/34-60775.pdf
// https://www.sec.gov/files/rules/sro/nysechx/2019/34-86709.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-chicago/NYSE_Chicago_Migration.pdf
// https://www.sec.gov/files/rules/sro/nysechx/2019/34-87264.pdf
// https://www.sec.gov/files/rules/sro/nysechx/2025/34-102507.pdf
// https://www.nyse.com/trade/hours-calendars?os=.
pub(crate) static NYSE_TEXAS_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);
static NYSE_CHICAGO_PROFILE_PRE_PILLAR: StaticHoursProfile =
    equity_profile(NYSE_CHICAGO_EXTENDED_PRE_PILLAR);

static TEXAS_REVISIONS: &[Revision] = revisions![(
    2019,
    11,
    4,
    &NYSE_TEXAS_PROFILE,
    "NYSE Chicago migration notice"
),];

pub(crate) fn nyse_texas_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_CHICAGO_PROFILE_PRE_PILLAR,
        TEXAS_REVISIONS,
    )
}
