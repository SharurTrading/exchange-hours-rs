// SPDX-License-Identifier: MIT-0

//! CME weather temperature-index futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// CME weather temperature-index FUTURES in America/Chicago: the HDD, CDD and
// CAT monthly, seasonal-strip and quarterly-strip contracts listed on CME
// (XCME) under CME Globex security group `HW` — US HDD `H0`-`H7`/`HQ`/`HR`/
// `HS`/`HW`/`LP`, US CDD `K0`-`K7`/`KQ`/`KR`/`KS`/`KW`/`KP`, European HDD
// `D0`/`D1`/`D2`/`D4`, European CAT `G0`/`G1`/`G2`/`G4`, Pacific Rim CAT `G6`,
// and their `X`/`Z`/`K`/`N` seasonal and current quarterly strips. CME
// publishes every one of these in Chicago time whatever city the index
// measures: "Trading hours are in U.S. Central Time unless otherwise stated."
//
// EXCLUDES OPTIONS. Options on weather futures carry the same roots as their
// futures and today publish the same Globex line, but they are a different
// Globex security group (`W7`) and, for most of the audited history, a
// different venue: they were an open-outcry product on the CME trading floor.
// CME's US Monthly Weather HDD OPTIONS specification reads "Trading Hours /
// (All times listed are Central Time) / Open Outcry / (Trading Floor) /
// MON-FRI: 8:30 a.m.-3:15 p.m." on captures from 2012-08-20 through
// 2022-06-30, and SER-7606R states it flatly in 2016: "The weather futures
// contracts will continue to be listed for trading on the CME Globex
// electronic trading platform, and the weather options contracts will continue
// to be listed for trading on the CME trading floor." So for roughly
// 2010-01..2023 the options' executable window was a same-day 08:30-15:15 CT
// floor session, not the Globex wrap this profile serves.
//
// NO `globex_weather_options` KEY IS CREATED, and that is a deliberate
// omission rather than an oversight. Such a key could not be encoded within
// LAW-NO-FABRICATED-DATES: its floor era begins before the January-2010 audit
// floor, and the day the legacy options left the floor for CME Globex is
// bracketed only to (2023-03-08, 2023-09-29] — the options block of the US
// Monthly Weather HDD specification still reads "MON-FRI: 8:30 a.m.-3:15 p.m."
// on the 2023-03-08 capture and reads "Sunday - Friday 5:00 p.m. - 3:15 p.m.
// CT / Daily trading halts 3:15 p.m. - 5:00 p.m." on the 2023-09-29 capture,
// with no capture between them and no retrieved CME notice naming the day.
// The 2023 Globex notices that list weather options (2023-07-31, for August
// 2023) list *new city* contracts, not a venue migration of the pre-existing
// ones, so they do not date it either. Encoding either bracket endpoint would
// invent a cutover; encoding neither would leave an undated venue change
// inside a shipped key. Weather options are therefore caller-catalog data
// until a CME document states the migration day.
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7606R.pdf
// https://web.archive.org/web/20140827025018id_/http://www.cmegroup.com/trading/weather/temperature/us-monthly-weather-heating_contractSpecs_options.html
// https://web.archive.org/web/20230308085113id_/https://www.cmegroup.com/markets/weather/temperature/us-monthly-weather-heating.contractSpecs.html
// https://web.archive.org/web/20230929024322id_/https://www.cmegroup.com/markets/weather/temperature/us-monthly-weather-heating.contractSpecs.html
//
// ENVELOPE MATCH IS NOT FAMILY IDENTITY, and this family is the crate's
// clearest worked example of it. Weather's *current* envelope is byte-for-byte
// the `globex_fx` and `globex_energy` envelope: extended SUN+MON-THU
// 17:00->16:00 CT wrapping midnight, order entry Sunday 16:00-17:00 and
// Monday-Thursday 16:45-17:00, no regular session, daily and weekend close.
// Folding was tested against all three histories and refuted by every one of
// them. `globex_fx` has carried a 17:00->16:00 matching grid since the
// January-2010 floor with no extended revision at all; `globex_energy` closed
// 16:15 at the floor and moved to 16:00 on 2015-09-20; weather closed **15:15**
// from the floor until 2025-04-13. Three keys, one envelope, three histories.
// Reusing either neighbour would report weather's daily close 60 to 75 minutes
// late on every trading day of roughly ninety-two per cent of the audited
// timeline, and reusing `globex_energy` would additionally fabricate a 2015
// cutover weather never had: every capture of CME's own weather specification
// *after* the 2015-09-20 NYMEX/COMEX move still reads "3:15 p.m." — 2016-12-01,
// 2018-06-01, 2019-11-18, 2021-04-22, 2021-07-28 and 2022-06-30, and the
// `.contractSpecs.html` chain beyond them — with the 2015-09-04 capture
// establishing the same value seventeen days before the move. The grids
// converged on 2025-04-13 by arithmetic, not by a shared notice — SER-9519
// names weather and nothing else.
//
// The product-family separation is equally sharp on CME's own taxonomy:
// weather is CME (XCME), financially settled off a published temperature
// index, with its own Globex security groups (`HW` futures, `W7` options), its
// own rulebook chapters (CME 403/403A, 405/405A, 407/407A, 408/408A, 409/409A
// — European CAT seasonal-strip options sit in 409A), and its own Daily
// Bulletin section 24, which carries every weather sub-group at once: WEATHER
// INDEX FUTURES, WEATHER STRIPS INDEX FUTURES, EUROPEAN WEATHER INDEX FUTURES,
// CAT EUROPEAN SEASONAL STRIP FUTURES and ADDITIONAL WEATHER FUTURES beside
// the matching options sections. `globex_energy` is NYMEX/COMEX
// physically-settled energy and metals on two other DCMs.
//
// ONE FAMILY, NO SUB-SPLIT. At the January-2010 floor the identical "SUN 5:00
// p.m. - FRI 3:15 p.m. / Daily trading halts 3:15 p.m. - 5:00 p.m." block
// appears on the US HDD, European CAT, Asia-Pacific Seasonal and Canadian CAT
// specifications; today CME's own spec API returns a byte-identical CME Globex
// string for Chicago HDD Monthly (productId 1186), Atlanta CDD Jul Seasonal
// Strip (1035), Amsterdam CAT Jul Seasonal Strip (599), Tokyo CAT Monthly
// (928), London CAT Monthly (182) and Paris HDD Monthly (10485); and SER-9519
// amends "all weather futures and options on weather contracts" in a single
// table row. CME's dedicated weather trading-hours page carries the same
// values for every US / Canadian / European / Asia-Pacific / Australian /
// weekly row apart from one persistent typo — the "European Monthly Weather
// HDD Futures & Options" cell reads 17:00-15:00 on the 2009-07-27, 2015-04-06
// and platform-wide 2012 captures while all twenty other rows read 17:00-15:15
// — and that product's own specification reads "SUN 5:00 p.m. - FRI 3:15 p.m."
// on the 2013-09-10 and 2015-03-26 captures, so the odd cell is a page error,
// not a divergence. Now-delisted CME weather families that shared this grid
// (Canadian HDD/CDD/CAT, Australian, Asia-Pacific monthly/seasonal, Weekly,
// Rainfall, Snowfall, Frost and the Hurricane Index products) rode the same
// clock. CME's separate taxonomy child "Wind" is not weather-index temperature
// and is not covered here.
// https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/1186
// https://www.cmegroup.com/daily_bulletin/current/Section24_Weather_Futures_And_Options.pdf
// https://web.archive.org/web/20120503104742id_/http://www.cmegroup.com/trading_hours/weather-hours.html
//
// NO REGULAR SESSION. Weather futures have never had a pit: CME's Daily
// Bulletin section 24 states the operator's own convention — "FOR PRODUCTS
// THAT ARE TRADED IN BOTH Open Outcry(RTH) AND on CME Globex (ETH) ... ETH
// REPRESENTS GLOBEX VOLUME TRANSACTIONS FROM THE GLOBEX(R) ELECTRONIC SESSION
// ONLY" — and every weather futures product record carries `pitEligible=0`.
// The 2007 launch release says the same thing prospectively: "The futures
// contracts will trade only on the CME Globex(R) electronic trading platform
// from 5 p.m. to 3:15 p.m. Chicago time the following day." `regular` is
// therefore empty in every era, exactly as in `energy_metals.rs`.
// https://investor.cmegroup.com/news-releases/news-release-details/cme-launch-weekly-weather-futures-and-options-contracts
//
// THE JANUARY-2010 FLOOR: Sunday 17:00 CT to Friday 15:15 CT, with a daily
// 15:15-17:00 CT halt. CME's US Monthly Weather HDD specification captured
// 2010-02-06 — the nearest primary capture above the audit floor — reads
// verbatim "Trading Hours / (All times listed are Central Time) / CME Globex
// (Electronic Platform) / SUN 5:00 p.m. - FRI 3:15 p.m. / Daily trading halts
// 3:15 p.m. - 5:00 p.m." The same two hours lines are on the same URL at
// 2010-07-25, 2012-05-11, 2013-09-10, 2015-09-04, 2016-12-01, 2018-06-01,
// 2019-11-18 and 2021-04-22, and server-rendered inside the replacement
// product page at 2021-07-28 and 2022-06-30. The chain continues on the
// `.contractSpecs.html` variants — which kept serving the table server-side
// long after the product pages became client-rendered shells — through
// 2022-10-31, 2023-03-08, 2023-09-29, 2024-12-03 and 2025-02-09, with a
// seasonal-strip options capture of 2025-02-18 carrying the same Globex line
// three weeks before SER-9519's notice date. SER-9519's own "Current CME
// Globex Hours" column then states the outgoing grid directly, so nothing is
// interpolated across the gap. Under the carry-back convention this state is
// extended to the January-2010 floor: no primary source names a cutover
// between the floor and the 2010-02-06 capture, so declining to invent one is
// the correct answer rather than modelling a demonstrably trading venue as
// sessionless.
// https://web.archive.org/web/20100206082420id_/http://www.cmegroup.com/trading/weather/temperature/us-monthly-weather-heating_contract_specifications.html
// https://web.archive.org/web/20220630073135id_/https://www.cmegroup.com/markets/weather/temperature/us-monthly-weather-heating.contractSpecs.options.html
// https://web.archive.org/web/20250209021116id_/https://www.cmegroup.com/markets/weather/temperature/european-monthly-weather-heating.contractSpecs.html
// https://web.archive.org/web/20250218001555id_/https://www.cmegroup.com/markets/weather/temperature/us-seasonal-strip-weather-cooling.contractSpecs.options.html
//
// 2025-04-13 — THE ONE DATED REVISION. CME SER-9519 (notice date 10 March
// 2025), subject "Expansion of the Trading Hours on the CME Globex Electronic
// Trading Platform for all Weather Futures and Options on Weather Futures
// Contracts", states: "Effective Sunday, April 13, 2025, for trade date
// Monday, April 14, 2025, and pending all relevant CFTC regulatory review
// periods, Chicago Mercantile Exchange Inc. (\"CME\" or \"Exchange\") will
// expand the CME Globex electronic trading (\"CME Globex\") hours of all
// weather futures and options on weather contracts (the \"Contracts\") as
// detailed in the table below." Its single table row moves "Current CME Globex
// Hours" = "Sunday 5:00 p.m. - Friday 3:15 p.m. / Daily trading halts 3:15
// p.m. - 5:00 p.m." to "Expanded CME Globex Hours" = "Sunday 5:00 p.m. -
// Friday 4:00 p.m. / Daily trading halts 4:00 p.m. - 5:00 p.m.", and closes:
// "Note that the CME Globex Pre-Open hours and the hours for trades submitted
// for clearing via CME ClearPort of the Contracts shall remain unchanged."
//
// DATE KEYING. CME states both days and they mean different things. The SER
// landing page stamps an effective date of 2025-04-14 (`data-advisory-
// effective-date="2025-04-14"`), which is the trade date; the notice body
// states the venue-local opening day, Sunday 2025-04-13. The first session
// that behaves differently is the one opening Sunday 2025-04-13 at 17:00 CT
// and closing Monday 2025-04-14 at 16:00 CT instead of 15:15, so a
// wrapped rule must be keyed to the opening day — the same convention
// `energy_metals.rs` records for its 2015-09-20 row. Both days are
// primary-sourced in one sentence; neither is inferred. CME Globex Notice of
// 2025-04-07 corroborates: "Effective this Sunday, April 13 (trade date
// Monday, April 14), The trading hours and daily trading halt for all weather
// products will be updated to Sunday 5:00 p.m. - Friday 4:00 p.m. Central Time
// (CT) with daily trading halts 4:00 p.m. - 5:00 p.m. CT." The same item runs
// in the Globex Notices of 2025-03-17 and 2025-03-24.
// https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2025/03/ser-9519.pdf
// https://www.cmegroup.com/notices/ser/2025/03/ser-9519.html
// https://www.cmegroup.com/notices/electronic-trading/2025/04/20250407.html
//
// CURRENT GRID. CME's live contract specifications, read 2026-09-05, state
// under TRADING HOURS: "CME Globex: Sunday - Friday 5:00 p.m. - 4:00 p.m. CT
// with a 60-minute break each day beginning at 4:00 p.m. CT", with PRODUCT
// CODE "CME Globex: H2" for Chicago HDD Monthly. CME's live session-event
// service independently returns, for the same product across a normal week:
// Sunday `preopen` 16:00 then `open` 17:00; Tuesday, Wednesday and Thursday
// `closed` 16:00, `preopen` 16:45, `open` 17:00; and Friday `closed` 16:00
// with no reopen. That confirms every field this profile serves, including the
// absent Friday evening session.
// https://www.cmegroup.com/markets/weather/hdd/chicago-hdd-monthly/specs
// https://www.cmegroup.com/services/trading-hours-by-product?id=1186
//
// CME CLEARPORT IS DELIBERATELY NOT MODELLED, and its hours are not static.
// The same specification pages publish "CME ClearPort: Sunday 5:00 p.m. -
// Friday 5:45 p.m. CT with no reporting Monday - Thursday from 5:45 p.m. -
// 6:00 p.m. CT". ClearPort accepts already-negotiated trades for clearing; it
// is neither a matching venue nor a queue on the central limit order book, so
// it belongs in neither `extended` nor `order_entry`. SER-7606R put weather on
// ClearPort effective Sunday 2016-03-20 for trade date Monday 2016-03-21 and
// stated a *different* window then — "CME ClearPort hours are: Sunday - Friday
// 6:00 p.m. - 5:00 p.m. (5:00 p.m. - 4:00 p.m. Chicago time CT) with a
// 60-minute break each day beginning at 5:00 p.m. (4:00 p.m. CT)" — so nothing
// here should be read as claiming ClearPort hours have been constant.

/// The executable CME Globex leg from the January-2010 floor until
/// SER-9519's cutover: Sunday and Monday-Thursday 17:00 CT, wrapping local
/// midnight to a 15:15 CT close, with the operator's 15:15-17:00 CT daily
/// trading halt in the gap.
static WEATHER_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 15 * 3600 + 15 * 60,
}];

/// The executable CME Globex leg SER-9519 established for 2025-04-13: the
/// same 17:00 CT opens, wrapping to a 16:00 CT close, with the daily halt
/// moved to 16:00-17:00 CT. Friday's close is therefore 16:00 CT and there is
/// no Friday-evening reopen.
pub(crate) static WEATHER_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

/// The CME Globex Pre-Open queues SER-9519 tabulates as "(unchanged)" and
/// CME's live session service still publishes: Sunday 16:00-17:00 CT and
/// Monday-Thursday 16:45-17:00 CT. Orders queue; nothing matches until 17:00.
pub(crate) static WEATHER_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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

// DATED QUEUES: THE SOURCED INTERSECTION, CARRIED BACK, WITH NO CUTOVER
// ASSERTED. Two Sunday queue states are sourced for weather and no primary
// source dates the move between them. CME's weather trading-hours page
// publishes Sunday Pre-Open 16:15 and weekday 16:45 on the captures of
// 2012-05-03, 2012-05-04 and 2012-05-05, and Sunday 16:00 with weekday 16:45
// on 2012-06-16 and every later capture through 2015-04-06. CME's
// platform-wide trading-hours pages narrow that gap on weather's own rows:
// all thirty weather/hurricane/frost/snowfall/rainfall rows read Sunday
// Pre-Open 16:15 on the 2012-05-28 capture and 16:00 on the 2012-06-07
// capture, with weekday 16:45 and the electronic session 17:00-15:15 on both.
// The bracket is therefore 2012-05-28..2012-06-07 — the identical
// platform-wide, unannounced Sunday-queue widening already bracketed to the
// same two captures in `cme_group.rs`, `energy_metals.rs`, `fx.rs` and
// `interest_rates.rs`, whose reviews read both CME notice channels in full
// across that window and found no announcement. No day may be encoded, so the
// dated profiles carry the window that holds under *both* sourced states —
// Sunday 16:15-17:00 — from the January-2010 floor, and only the disputed
// 16:00-16:15 quarter-hour waits on the knowledge-bound row below.
//
// The floor's own queue is a further step back: CME's weather trading-hours
// captures of 2009-07-27 and 2011-09-27 have no Pre-Open columns at all
// (their columns are Product Name | Open Outcry | Electronic Trading (Weekday)
// | Electronic Trading (Sunday) | CME ClearPort), so the first sourced weather
// queue of any kind is the 2012-05-03 capture. Carrying it back asserts no
// revision; it declines to invent one, and it under-reports rather than
// over-reports queueing, which is the safe direction for a phase in which no
// trade can print.
// https://web.archive.org/web/20110927011404id_/http://www.cmegroup.com/trading_hours/weather-hours.html
// https://web.archive.org/web/20120616193925id_/http://www.cmegroup.com/trading_hours/weather-hours.html
// https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
// https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
static WEATHER_ORDER_ENTRY_DATED: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
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

static AT_2010_FLOOR: StaticHoursProfile =
    profile(WEATHER_EXTENDED_AT_2010_FLOOR, WEATHER_ORDER_ENTRY_DATED);
static FROM_2025_04_13: StaticHoursProfile =
    profile(WEATHER_EXTENDED_CURRENT, WEATHER_ORDER_ENTRY_DATED);
/// The knowledge-bound current state: SER-9519's matching grid plus both
/// queues CME publishes today, including the Sunday quarter-hour whose onset
/// day no source states. `profiles.rs` builds the fixed-current
/// `FuturesSessionProfile` from the same two rule tables.
static DATED_CURRENT: StaticHoursProfile =
    profile(WEATHER_EXTENDED_CURRENT, WEATHER_ORDER_ENTRY_CURRENT);

// Revision evidence — the day-level effective date and the primary source that
// states it, plus the knowledge-bound row that carries the undated Sunday
// quarter-hour:
//   2025-04-13 "CME SER-9519"
//     https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2025/03/ser-9519.pdf
static REVISIONS: &[Revision] = revisions![
    (2025, 4, 13, &FROM_2025_04_13, "CME SER-9519"),
    // Knowledge-bound row, dated at the repository review that verified this
    // family's phases. The dated profiles above already carry the sourced
    // Sunday 16:15-17:00 and Monday-Thursday 16:45-17:00 queues from the
    // January-2010 floor; this row adds only the 16:00-16:15 quarter-hour,
    // the single part that depends on the undated 2012 widening. It makes no
    // onset claim, its date never moves forward, and a sourced onset day
    // replaces it. It is dated 2026-09-05 rather than the 2026-08-22 row the
    // four other CME families carry because 2026-09-05 is the day this
    // family's evidence was actually checked; one consequence is visible in
    // `tests/golden/normal_week_grids.txt`, whose 2026-08-22 fixture instant
    // renders this key's dated Sunday 16:15 queue while `session_profile`
    // serves the current 16:00 one.
    (
        2026,
        9,
        5,
        &DATED_CURRENT,
        "2026-09-05 review: verified current, onset undated"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
