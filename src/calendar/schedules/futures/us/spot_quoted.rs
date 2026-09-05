// SPDX-License-Identifier: MIT-0

//! CME/CBOT Spot-Quoted Futures ("SQF") schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// CME/CBOT Spot-Quoted Futures in America/Chicago: eight tradeable Globex
// roots, all of them Rulebook Chapter 24 — `QSPX`, `QNDX`, `QRTY` and `QDOW`
// on the equity indices, `QBTC`, `QETH`, `QSOL` and `QXRP` on the
// cryptocurrency reference rates. SQF quote at the spot-equivalent level and
// append a daily financing adjustment: CME Rulebook 2400 defines the product
// as "Spot Equivalent Quoted Price + Financing Adjustment", and CBOT 24100
// carries the identical sentence. `QDOW` is the CBOT listing; the other seven
// are CME.
// https://www.cmegroup.com/rulebook/CME/I/24.pdf
// https://www.cmegroup.com/rulebook/CBOT/III/24.pdf
//
// SCOPE — EIGHT EXECUTABLE ROOTS, NOT THE SIXTEEN CODES AROUND THEM. Each SQF
// contract also carries a non-trade clearing leg (`QSF`, `QNF`, `QRF`, `QDF`,
// `QTF`, `QEF`, `QOF`, `QXF`) and a financing-adjustment marker code (`QSM`,
// `QNM`, `QDM`, `QRM`, `QTM`, `QEM`, `QOM`, `QXM`). Those are settlement and
// reference identifiers, not order books, and no session is modeled for them.
// The rulebook's own product table — the "Spot-Quoted Futures Listing and
// Procedures Table", incorporated by reference from inside both Chapter 24
// PDFs — lists exactly the eight tradeable roots across its Equity Index and
// Cryptocurrency sheets and no ninth.
// https://www.cmegroup.com/rulebook/files/spot-quoted-futures-listing-and-procedures-table.xlsx
//
// ONE KEY FOR ALL EIGHT, AND THE REASON IS NOT "THE GRIDS NEVER DIFFER". They
// do differ, on half-days. CME's own trading-hours service, read 2026-09-05,
// publishes a 4/4 split along the equity/cryptocurrency line: on Thanksgiving
// Friday 2026-11-27 the four equity roots (Globex security groups `D1`-`D4`)
// carry `closed@12:00` while the four cryptocurrency roots (`D5`, `D6`, `D7`,
// `D9`) carry `closed@13:45`; on Christmas Eve 2026-12-24 the same partition
// reads 12:15 against 12:45. Two holidays, two split magnitudes, the same
// partition. This is recorded here so nobody later "discovers" it and
// concludes the single key is wrong.
//
// One key is nevertheless correct, on the ground that actually holds in this
// crate: a `MarketHoursKey` encodes the normal week plus its dated revisions,
// and holiday and early-close data is caller-owned — `src/calendar/
// exceptions.rs` ships none, and LAW-HOLIDAY-SCOPE keeps single-trade-date
// closes out of profile tables entirely. On the *normal* week all eight are
// identical: the same service returns byte-identical event schedules for all
// eight product ids over 2026-09-13..2026-09-19 (11086 `QSPX`/`D1`, 11088
// `QNDX`/`D2`, 11090 `QRTY`/`D3`, 11092 `QDOW`/`D4`, 11094 `QBTC`/`D5`, 11096
// `QETH`/`D6`, 11415 `QSOL`/`D7`, 11417 `QXRP`/`D9`), and CME's spec API
// returns one identical CME Globex hours string for all eight. Their dated
// history is identical too: one listing notice states one hours cell for both
// exhibit tables. CME administers them as one thing — the rulebook's own
// listing table files every root, cryptocurrency included, under Product Group
// "Equity" and Product Subgroup "U.S. Index".
//
// CALLERS ATTACHING EXCEPTION DATA TO THIS KEY MUST BUILD IT PER SUBGROUP:
// one early-close set applied to all eight roots will be wrong for four of
// them on at least two dates a year.
// https://www.cmegroup.com/services/trading-hours-by-product?id=11086
// https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/11086
//
// NOT `globex_equity_index`. That key carries a published 08:30-15:15 CT RTH
// (`CME_REGULAR` in `cme_group.rs`); no CME document classifies any part of
// the SQF session as regular, so pointing the four equity SQF roots at it
// would assert an RTH split out of nothing. Its timeline also runs from the
// January-2010 floor with revisions in 2012, 2015 and 2021, none of which can
// apply to a family that did not exist until 2025. The envelope argument runs
// the other way as well: CME's trading-hours service shows the two envelopes
// separating on U.S. Monday holidays, where `ES` opens Sunday evening and
// trades a shortened Monday session while every SQF root is shut. Labor Day
// 2026 is the worked example — `ES` (id 133) returns `preopen@16:00` on Sunday
// 2026-09-05 and `preopen@12:00` on Monday 2026-09-07, while all eight SQF ids
// return *no* Sunday events and nothing on Monday before that evening's
// `preopen@16:45` for trade date 2026-09-08. That divergence is holiday data
// and is therefore not encoded here; it is recorded because it refutes the
// fold on the envelope, not merely on the unsourced RTH question.
//
// NOT `globex_cryptocurrency` either. CME expanded the cryptocurrency futures
// and options families to 24/7 trading effective Friday 2026-05-29, and left
// spot-quoted out by name: CME Submission 26-114 certifies the expansion "for
// all cryptocurrency futures and options on futures contracts noted in Table
// 1. below ... effective Friday, May 29, 2026", and the strings "Spot-Quoted",
// "Spot Quoted", "Chapter 24", "QBTC", "QETH", "QSOL" and "QXRP" do not occur
// anywhere in that filing. CME's own client-systems wiki for the migration
// says it directly — "Cryptocurrency Spot Quoted Futures (SQF) will remain on
// a 5 day schedule." — and its "Products Not In Scope" table lists the four
// crypto SQF roots with their security groups. The live grid agrees: on a
// normal week `BTC` (id 8478, group `BF`) carries Saturday events and a
// 16:01/16:02 daily turn, while `QBTC` carries none and the five-day
// 16:00/16:45/17:00 grid. Reusing that key would report this family open all
// weekend when CME publishes it closed from Friday 16:00 CT.
// https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf
// https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1283194884/Cryptocurrency+Futures+and+Options+Migration+to+24-7+Trading
//
// NO REGULAR SESSION, AND THAT IS A SOURCED ABSENCE RATHER THAN A GUESS. Four
// CME channels publish SQF hours and not one splits the session into RTH and
// ETH: SER-9506R and SER-9630RR print exactly one "CME Globex" line per
// product; all eight live contract specifications print exactly one; CME
// Rulebook 2402.A and CBOT 24102.A both say only "Futures contracts shall be
// scheduled for trading during such hours and for delivery in such months as
// may be determined by the Exchange."; and the rulebook's Listing and
// Procedures Table has no hours column at all. The whole 17:00-16:00 CT window
// is therefore `extended` with `regular` empty, exactly as `cryptocurrency.rs`
// treats its own five-day era. Assigning the equity-index RTH to `QSPX` and
// its three neighbours would be an invention.
//
// ENVELOPE COINCIDENCE, AGAIN. This grid is identical to `globex_weather`'s
// current envelope — 17:00->16:00 CT with the same Sunday 16:00 and
// Monday-Thursday 16:45 queues — while the two histories share nothing at all:
// weather ran to a 15:15 CT close from the January-2010 floor until
// 2025-04-13, and spot-quoted did not exist until 2025-06-29, ten weeks later.
// The same point `weather.rs` documents against `globex_fx` and
// `globex_energy` applies here with a third pair.
//
// CME CLEARPORT IS NOT MODELLED. The same notices and specification pages
// publish "CME ClearPort: Sunday 5:00 p.m. - Friday 5:45 p.m. CT with no
// reporting Monday - Thursday from 5:45 p.m. - 6:00 p.m. CT". ClearPort
// accepts already-negotiated trades for clearing; it is neither a matching
// venue nor a queue on the central limit order book, so it belongs in neither
// `extended` nor `order_entry`.
//
// TIMEZONE. CME states SQF hours in Eastern time in the notices and in Central
// time on the specification pages and the trading-hours service. Both rulebook
// chapters fix the notice convention — "Unless otherwise specified, times
// referenced herein shall refer to and indicate Eastern Prevailing Time
// (EPT)." — and ET and CT shift together, so the two publications agree
// year-round and the Central grid is the stable one to encode.

/// The executable CME Globex leg SER-9506R established at launch and CME still
/// publishes: Sunday and Monday-Thursday 17:00 CT, wrapping local midnight to
/// a 16:00 CT close, with the operator's 60-minute maintenance period in the
/// 16:00-17:00 CT gap. Friday is not an opening day, which is what produces
/// the weekend close and the absent Friday-evening reopen.
pub(crate) static SPOT_QUOTED_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

/// The CME Globex Pre-Open queues SER-9506R states at launch and CME's session
/// service still publishes: Sunday 16:00-17:00 CT and Monday-Thursday
/// 16:45-17:00 CT. Orders queue, amend and cancel; nothing matches until
/// 17:00, so the first 45 minutes of the daily break accept nothing at all.
pub(crate) static SPOT_QUOTED_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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

// CLOSED BEFORE LAUNCH. The family did not exist before its listing day, so
// the pre-launch baseline is an explicit sessionless profile rather than an
// absence — the same treatment `cryptocurrency.rs`, `ice_us.rs` and `sgx.rs`
// give their own launch-dated families. An instant below the January-2010
// audit floor therefore resolves to this closure, which is the correct oldest
// state on record for a venue that launched in 2025.
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// The one grid this family has ever published, from its launch day onward.
static FROM_2025_06_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: SPOT_QUOTED_EXTENDED_CURRENT,
    order_entry: SPOT_QUOTED_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2025-06-29 — THE LAUNCH, AND THE FAMILY'S ONLY REVISION ROW. CME SER-9506R
// (notice date 13 May 2025), subject "Initial Listing of Six (6) CME and CBOT
// Spot-Quoted Futures Contracts", states: "Effective Sunday, June 29, 2025 for
// trade date Monday, June 30, 2025, and pending applicable regulatory review
// periods, Chicago Mercantile Exchange Inc. (\"CME\") and The Board of Trade of
// the City of Chicago, Inc. (\"CBOT\") (collectively, the \"Exchanges\") will
// list the six (6) Spot-Quoted Futures (SQF) contracts noted in Table 1. below
// (collectively, the \"SQF Contracts\") for trading on the CME Globex
// electronic trading platform (\"CME Globex\") and for submission for clearing
// via CME ClearPort." Table 1 lists `QSF`, `QNF`, `QRF`, `QTF` and `QEF` under
// "CME 24" and `QDF` under "CBOT 24"; Exhibit 1 gives the tradeable codes
// `QSPX`, `QNDX`, `QDOW`, `QRTY`, `QBTC` and `QETH`.
//
// Exhibit 1 prints the identical "Trading and Clearing Hours" cell twice, once
// under "CME and CBOT Spot-Quoted Equity Index Futures" and once under "CME
// Spot-Quoted Cryptocurrency Index Futures": "CME Globex PreOpen: Sunday 5:00
// p.m. - 6:00 p.m. Eastern Time/ET, Monday - Thursday 5:45 p.m. - 6:00 p.m. ET
// / CME Globex: Sunday 6:00 p.m. - Friday - 5:00 p.m. ET with a daily
// maintenance period from 5:00 p.m. - 6:00 p.m.ET / CME ClearPort: Sunday 6:00
// p.m. - Friday 6:45 p.m. ET with no reporting Monday - Thursday from 6:45
// p.m. - 7:00 p.m. ET". In Central time that is the 17:00->16:00 CT wrap and
// the 16:00-17:00 and 16:45-17:00 CT queues encoded above. Both phases are
// sourced on this one day, so this family has no undated queue onset.
//
// DATE KEYING. CME states both days and they mean different things. The row is
// keyed to the venue-local **opening** day, Sunday 2025-06-29, not the stated
// trade date of Monday 2025-06-30: the first session that exists at all is the
// one opening Sunday at 17:00 CT (queueing from 16:00 CT) and closing on the
// Monday trade date, and a wrapped rule keyed a day late would render that
// Sunday evening closed. This is the same convention `weather.rs`,
// `rough_rice.rs` and `mini_grains.rs` record for their own rows. Both days
// are primary-sourced in one sentence; neither is inferred. CME Clearing
// advisory 25-201 corroborates the launch from the trade-date side: "Launch of
// Spot-Quoted Futures - Effective June 30, 2025 ... Pending regulatory review,
// CME Group will be launching Spot-Quoted Futures on six markets on June 30,
// 2025".
// https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2025/05/ser-9506r.pdf
// https://web.archive.org/web/20260218091422id_/https://www.cmegroup.com/notices/clearing/2025/06/25-201.html
//
// 2025-12-14 — CATALOG DATA, NOT A REVISION ROW. CME SER-9630RR states
// "Effective Sunday, December 14, 2025, for trade date Monday, December 15,
// 2025 ... will list Spot-Quoted SOL Futures and Spot-Quoted XRP Futures",
// adding `QSOL` (`QOF`) and `QXRP` (`QXF`) to the live family. Its Exhibit 1
// reprints the launch hours cell character for character, so the *family
// clock* does not change: only the membership does. Under the crate's
// member-listing convention that is instrument-catalog data — the same call
// `mini_grains.rs` makes for `MKC`'s 2014-03-23 listing and
// `cryptocurrency.rs` makes for ETH/MBT/MET in 2021 — so no revision row
// exists for it and a caller that trades `QSOL` before 2025-12-14 must gate
// that in its own catalog, exactly as it must for `MKC` before 2014.
//
// The opening day is the one to record even so, for the same reason as the
// launch: SQF sessions open on Sunday evening. CME's own notice-search index
// lists SER-9630RR's effective date as **Dec 17, 2025**, which is its posting
// date — the SER's own header reads "DATE: December 17, 2025" and it
// supersedes SER-9630R of November 14, 2025 to add commodity codes. The
// document body is the authority; a future reader who checks the index and
// finds Dec 17 is looking at a publication date, not an effective one.
// https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2025/12/ser-9630rr.pdf
//
// 2026-06-01 — NOT A CLOCK CHANGE. CME SER-9739 amends "the Listing Schedule
// of all CME and CBOT Spot-Quoted Futures" effective Sunday 2026-05-31 for
// trade date Monday 2026-06-01, adding a June 2027 expiry beside the June 2026
// one. The whole notice is one page, it carries no Trading and Clearing Hours
// table, and the word "hour" does not occur in it; its only mention of the
// platform is the boilerplate "for trading on the CME Globex electronic
// trading platform". It is noted here because its Table 1 is the most recent
// primary confirmation that all eight roots are one family — `QSPX`, `QNDX`,
// `QRTY`, `QBTC`, `QETH`, `QSOL` and `QXRP` under "CME 24" and `QDOW` under
// "CBOT 24".
// https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9739.pdf
//
// NO KNOWLEDGE-BOUND ROW, BECAUSE THERE IS NO UNDATED PHASE. Both the matching
// leg and both queues are stated by SER-9506R on the launch day and returned
// unchanged by CME's live specification API and session service on 2026-09-05;
// the four CME families with a knowledge-bound review row have it because
// their Sunday queue's onset day is genuinely unsourced, which is not the case
// here. The residual risk is stated rather than hidden: no single document
// affirms that the Pre-Open never moved and moved back between those two
// endpoints. CME's session service is forward-only — a query for the launch
// week 2025-06-28..2025-07-04 returns empty days — so the interior cannot be
// sampled. What can be done was done: a sweep of CME's notice-search index
// from 2025-04-01 to the 2026-09-05 review date (about 1,800 notices) returns
// nine notices
// mentioning spot-quoted futures — SER-9506R, SER-9630RR, SER-9739, clearing
// advisories 25-154, 25-201 and 25-348, and market-regulation notices
// MSN06-25-25, MSN11-14-25 and MSN02-11-26 — and none of them amends trading
// hours. Under the carry-back convention no cutover is asserted, which
// declines to invent one rather than fabricating a date.
//
// Revision evidence — the day-level effective date and the primary source that
// states it:
//   2025-06-29 "CME SER-9506R"
//     https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2025/05/ser-9506r.pdf
static REVISIONS: &[Revision] = revisions![(2025, 6, 29, &FROM_2025_06_29, "CME SER-9506R")];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
