// SPDX-License-Identifier: MIT-0

//! CME Event Contracts on Bitcoin Futures (`ECBTC`) schedule.

use chrono_tz::US;

use super::event_contracts::{
    EVENT_CONTRACTS_EXTENDED_CURRENT, EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
};
use crate::calendar::SessionRule;
use crate::calendar::rule::{FRI, MON_FRI, SUN_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

const SAT_ONLY: [bool; 7] = [false, false, false, false, false, true, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];

// CME Event Contracts on Bitcoin Futures in America/Chicago: the one Chapter 23
// event-contract root CME moved to 24/7 trading on 2026-05-29, leaving the
// other ten on the five-day grid `event_contracts.rs` models. This key carries
// `ECBTC`'s whole life so a caller never switches keys on a date: sessionless
// before its 2023-03-12 listing, the shared event-contract grid until
// 2026-05-28, and the 24/7 grid from 2026-05-29 on.
//
// WHY A SECOND KEY. CME SER-9740R (28 May 2026) expands "the trading hours on
// the CME Globex electronic trading platform ... for all cryptocurrency futures
// and options on futures contracts noted in Table 1", and its Table 1 now reads
// "Event Contracts on Bitcoin Futures | ECBTC | 23", "Effective Friday, May 29,
// 2026". CME's own client-systems wiki scopes the expansion to that one root —
// "only the below event contracts on Bitcoin on channel 329 will migrate to
// weekend trading. Other event contracts will continue on the current
// schedule." — with a Product Scope table whose single row is "Event contracts
// on Bitcoin Futures | ECBTC | VB | 329 | 74". A session change on one root and
// not its siblings is a divergence, so the root gets its own timeline, exactly
// as `mini_grains.rs` and `rough_rice.rs` split from the grain grid.
// https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf
// https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1394343937/Event-Based+Contracts+Expansion+to+24-7+Trading
//
// THE CONFLICT, STATED EXACTLY. Two CME primaries describe the 24/7 grid and
// agree on everything except where the weekday maintenance window begins.
// SER-9740R's Table 2: "CME Globex: 24/7 with the exception of the following
// maintenance windows: Saturday 2:00 a.m. to 4:00 a.m. CT. Monday-Friday
// 4:00p.m. to 4:02 p.m. CT" with "CME Globex Pre-open: Monday - Friday: 4:01
// p.m. to 4:02 p.m. CT Saturday: 3:45 a.m. to 4:00 a.m. CT". The wiki's
// "Event-Based Contract Maintenance Windows and Market Hours" table: "Monday
// through Friday | Daily Maintenance Window (with Trade Date roll) | Close:
// 3:00:00 p.m. to 4:01:00 p.m. CT | Pre-open: 4:01:00 p.m. to 4:01:30 p.m. CT /
// No cancel: 4:01:30 p.m. to 4:02:00 p.m. CT / Open: 4:02:00 p.m. CT" and
// "Saturday | Extended Maintenance Window | Close: 2:00 a.m. to 3:45 a.m. CT |
// Pre-open: 3:45:00 a.m. to 4:00:00 a.m. CT / ... Open: 4:00 a.m. CT". So the
// weekday executable close is 16:00 CT in one document and 15:00 CT in the
// other, and everything else — the 16:01 queue, the 16:02 open, the Saturday
// window and its 03:45 queue — is identical.
//
// THE TWO STATEMENTS ARE ORDERED, AND EACH HAS A WEAKNESS. Confluence's own
// version history shows the wiki table entered at v4 — absent from v1-v3 (all
// 2026-03-25) — and the page's Revision History row says "April 22, 2026 |
// Added Event-Based Contract Maintenance Windows and Market Hours schedule
// impacts." There is no later revision: the version list ends at v4, the
// history endpoint has no `nextVersion`, and version 5 answers HTTP 404. So the
// 15:00 figure is the earlier statement, written five weeks before the SER and
// never revisited, and it is still the live page. SER-9740R is the later one,
// but its Table 2 was not written for `ECBTC`: the SER says "No other changes
// have been made to the original SER" beyond adding the root to Table 1, and
// the "Current" column it prints for `ECBTC` — "Sunday 5:00 p.m. - Friday 4:00
// p.m. CT with a daily maintenance period from 4:00 p.m. - 5:00 p.m. CT" — is
// the cryptocurrency-complex cell, whereas the root's own listing cell in
// SER-9092 and rule filing 23-014 read "Sunday 5:00 p.m.- Friday 3:00 p.m." So
// the later document misdescribes this root's prior hours in the very table
// that states its new ones. Rule filing 26-266 (4 June 2026, a Regulation
// 40.6(d) notification) reproduces SER-9740R verbatim as its Exhibit A and is
// the same statement, not a second one.
// https://cmegroupclientsite.atlassian.net/wiki/rest/api/content/1394343937/version
// https://www.cmegroup.com/notices/ser/2023/02/SER-9092.pdf
// https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2023/2/23-014.pdf
// https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/6/26-266.pdf
//
// BOTH CHANNELS ARE EXHAUSTED. Nothing CME has published since the cutover
// restates this root's weekday close: not the wiki (frozen at v4), not any SER,
// Globex notice, clearing advisory, product page, the Daily Bulletin, the
// product-slate or trading-hours services (neither lists an event contract) or
// the event-contract specification PDF (which contains no "Trading Hours"
// field at all). Waiting for a restatement has no endpoint.
//
// SO THE KEY SERVES THE SOURCED INTERSECTION. The window true under both
// documents is open from 16:02 CT to 15:00 CT the next day, Monday through
// Friday; the disputed 15:00-16:00 hour is served closed. Under SER-9740R's
// reading that under-reports one executable hour a day; under the wiki's it is
// exact; it never over-reports, which is the direction every Partial row in
// this crate errs. `AGENTS.md`'s sourced-intersection convention is written
// for an undated changeover between two states, and this is its second shape
// — two documents describing one period — but the rationale is identical:
// withholding the whole key over a one-hour dispute reports ~23 agreed hours a
// day, six days a week, as unmodelled, and absence is a claim too.
//
// WHAT IS AGREED IS ALSO INDEPENDENTLY CONFIRMED AFTER THE CUTOVER. CME's
// Globex notices of 27 July, 24 August and 31 August 2026 each list
// "Event-Based Contracts | 329" among the "24/7 markets" whose Saturday
// maintenance window they temporarily extend, and each says the window will
// "revert to its 2:00 a.m. – 4:00 a.m. CT standard schedule". Those notices
// are also this key's three temporary rows, below.
//
// NOT EVIDENCE, RECORDED SO IT IS NOT REDISCOVERED. (1) CME's "24/7 Trading
// for 1-Ounce Gold and 10-Barrel Crude" deck (8 July 2026) prints the wiki's
// exact row template with "Close: 4:00:00 p.m." for a product migrating onto
// "329 (shared with events contracts)"; it is a statement about gold, not
// about this root. (2) cmegroup.com/trading-hours.html's "Event-Based
// Contracts | 16:01 CT | 16:00 CT" row is byte-identical in captures before
// and after the cutover and still advertises a Tuesday maintenance window
// that CME cancelled on 2026-03-03 for the neighbouring Event Contracts II
// channel; it describes that channel, stale. (3) That 15:00 CT coincides with
// this root's former daily settlement instant is a hypothesis about why the
// documents diverge, and under LAW-SESSION-NOT-EXPIRY it can neither
// corroborate nor refute a close. The 15:00 in SER-9092 is sourced here only
// because it sits in that document's "CME Globex Trading Hours" row, in
// session language, separate from its "Termination of Trading" row.
//
// NOT `globex_cryptocurrency`, THOUGH THE 24/7 SHAPE IS THE SAME. That key
// carries Bitcoin futures from 2017 with no sourced five-day Pre-Open and a
// 16:00 CT weekday close; this root ran the event-contract grid with stated
// queues from 2023 and closes at 15:00 CT under the intersection. Its queues
// stay `order_entry` here, as `event_contracts.rs` models them, rather than
// folded into `extended` as the cryptocurrency module chose.
//
// TIMEZONE. Every document above states this root's hours in Central time
// ("CT"); the wiki adds seconds. The Central grid is the one encoded.

/// The 24/7 executable leg under the sourced intersection: Monday-Friday
/// 00:00-15:00 and 16:02-24:00 CT, Saturday 00:00-02:00 and 04:00-24:00 CT,
/// and all of Sunday. `SessionRule` spans at most one local midnight, so the
/// continuous Saturday-04:00-to-Monday-15:00 block is stored in day pieces
/// that the key-backed calendar joins at query time.
pub(crate) static BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 0,
        close_ssm: 15 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 2 * 60,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 0,
        close_ssm: 2 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 4 * 3600,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 0,
        close_ssm: 24 * 3600,
    },
];

/// The Pre-Open queues both documents state: Monday-Friday 16:01-16:02 CT and
/// Saturday 03:45-04:00 CT. Orders queue and nothing matches until the open.
pub(crate) static BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 16 * 3600 + 2 * 60,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 3 * 3600 + 45 * 60,
        close_ssm: 4 * 3600,
    },
];

// The cutover day itself. The wiki states the instant — "Starting at 4 p.m.
// Central Time on Friday, May 29, 2026, CME Group will expand event contracts
// to 24/7 trading" — so Thursday's 17:00 open still closes at Friday 16:00 CT
// under the old grid, and the first 24/7 queue opens at 16:01 that afternoon.
static EXTENDED_2026_05_29: &[SessionRule] = &[
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 16 * 3600 + 2 * 60,
        close_ssm: 24 * 3600,
    },
];
static ORDER_ENTRY_2026_05_29: &[SessionRule] = &[SessionRule {
    days: FRI,
    open_ssm: 16 * 3600 + 60,
    close_ssm: 16 * 3600 + 2 * 60,
}];

// Three one-day Saturday extensions CME announced for channel 329, each
// without a replacement Pre-Open, each followed by the standard window. The
// weekday and Sunday pieces are unchanged; only the Saturday reopen moves.
static ORDER_ENTRY_WEEKDAYS_ONLY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 60,
    close_ssm: 16 * 3600 + 2 * 60,
}];
macro_rules! saturday_extended_to {
    ($name:ident, $reopen_hour:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: MON_FRI,
                open_ssm: 0,
                close_ssm: 15 * 3600,
            },
            SessionRule {
                days: MON_FRI,
                open_ssm: 16 * 3600 + 2 * 60,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: 0,
                close_ssm: 2 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: $reopen_hour * 3600,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SUN_ONLY,
                open_ssm: 0,
                close_ssm: 24 * 3600,
            },
        ];
    };
}
saturday_extended_to!(EXTENDED_2026_08_01, 9);
saturday_extended_to!(EXTENDED_2026_08_29, 6);
saturday_extended_to!(EXTENDED_2026_09_19, 8);

// CLOSED BEFORE LISTING. The root did not exist before SER-9092's listing day,
// so the pre-listing baseline is an explicit sessionless profile, as for every
// launch-dated family in this crate.
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// 2023-03-12 to 2026-05-28: the event-contract grid, by reference. SER-9092's
/// Exhibit 1 reprints the family's hours cell unchanged for this root, so the
/// tables are shared rather than copied.
static FROM_2023_03_12: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EVENT_CONTRACTS_EXTENDED_CURRENT,
    order_entry: EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static TRANSITION_2026_05_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_2026_05_29,
    order_entry: ORDER_ENTRY_2026_05_29,
    has_daily_close: true,
    has_weekend_close: false,
};

/// The sourced intersection of SER-9740R and the client-systems wiki.
static SOURCED_INTERSECTION: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT,
    order_entry: BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: false,
};

macro_rules! temporary_saturday {
    ($name:ident, $extended:ident) => {
        static $name: StaticHoursProfile = StaticHoursProfile {
            tz: US::Central,
            regular: &[],
            extended: $extended,
            order_entry: ORDER_ENTRY_WEEKDAYS_ONLY,
            has_daily_close: true,
            has_weekend_close: false,
        };
    };
}
temporary_saturday!(TEMPORARY_2026_08_01, EXTENDED_2026_08_01);
temporary_saturday!(TEMPORARY_2026_08_29, EXTENDED_2026_08_29);
temporary_saturday!(TEMPORARY_2026_09_19, EXTENDED_2026_09_19);

// Revision evidence — the day-level effective date and the primary source that
// states it:
//   2023-03-12 "CME SER-9092" — "Effective Sunday, March 12, 2023, for trade
//     date Monday, March 13, 2023"; keyed to the venue-local opening day, as
//     `event_contracts.rs` keys its own launch.
//     https://www.cmegroup.com/notices/ser/2023/02/SER-9092.pdf
//   2026-05-29 "CME SER-9740R" — "Effective Friday, May 29, 2026"; the
//     transition day above, then the intersection from the Saturday.
//     https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf
//   2026-08-01 "CME Globex notice 20260727" — "Effective this Saturday, August
//     1, CME Group will temporarily extend the Saturday maintenance window
//     schedule for 24/7 markets to 2:00 – 9:00 a.m. Central Time (CT)", table
//     row "Event-based contracts | 329"; "Following this one-day extension,
//     the Saturday window will revert to its standard schedule."
//     https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html
//   2026-08-29 and 2026-09-19 "CME Globex notice 20260824" — "Saturday, August
//     29: 2:00 a.m. – 6:00 a.m. Central Time (CT)" and "Saturday, September
//     19: 2:00 a.m. – 8:00 a.m. CT", table row "Event-Based Contracts | 329";
//     "Following each extension, the Saturday maintenance window will revert
//     to its 2:00 a.m. – 4:00 a.m. CT standard schedule." The 31 August notice
//     restates the September date. The September row is forward-dated on the
//     operator's statement.
//     https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html
//     https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html
static REVISIONS: &[Revision] = revisions![
    (2023, 3, 12, &FROM_2023_03_12, "CME SER-9092"),
    (2026, 5, 29, &TRANSITION_2026_05_29, "CME SER-9740R"),
    (2026, 5, 30, &SOURCED_INTERSECTION, "CME SER-9740R"),
    (
        2026,
        8,
        1,
        &TEMPORARY_2026_08_01,
        "CME Globex notice 20260727"
    ),
    (
        2026,
        8,
        2,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260727"
    ),
    (
        2026,
        8,
        29,
        &TEMPORARY_2026_08_29,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        8,
        30,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        9,
        19,
        &TEMPORARY_2026_09_19,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        9,
        20,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260824"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
