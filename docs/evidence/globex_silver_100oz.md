<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_silver_100oz` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`silver_100oz.rs`](../../src/calendar/schedules/futures/us/silver_100oz.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-01 UTC)

**Gap: order-entry** — the trading session is sourced; what is withheld is a
queue or phase in which no trade can print. COMEX 100-Ounce Silver futures,
CME Globex root `SIL`, security group 4S. CME Globex notice 20260817 announced
the product's migration to MDP channel 329 / market segment 74 "Effective
Sunday, August 30 (trade date Monday, August 31)" together with its expansion
to 24/7 trading, and notice 20260907 restated the expansion's own effective
day in session language: "Effective this Friday, September 11, CME Group will
expand 100-Ounce Silver futures to 24/7 Trading. Production 100-Ounce Silver
futures weekend trading will begin at 4:30 p.m. Central Time." The key carries
the product's whole life so a caller never switches keys on a date: the shared
NYMEX/COMEX energy-and-metals grid by reference to that family's own dated
rows through 2026-09-10, the 2026-09-11 bridge day (Thursday's leg still opens
17:00 CT and closes Friday 16:00 under the old grid; the first weekend leg
opens 16:30 CT behind that one Friday's 30-minute maintenance extension), and
the 24/7 grid from 2026-09-12 — Monday-Friday 00:00-16:00 and 16:02-24:00 CT
behind the operator's two-minute daily maintenance ("All subsequent Friday
trading sessions will open at their regular time of 4:02 p.m. CT"), Saturday
00:00-02:00 and 04:00-24:00 CT around the two-hour weekly window whose
"2:00 a.m. – 4:00 a.m. CT standard schedule" notice 20260921 states for the
24/7 markets, and all of Sunday. The weekend block carries the following open
business date, as the notice states: "All holiday or weekend trading from
Friday evening through Sunday evening will have a trade date of the following
business day." Partial / order-entry because no artifact in hand states a
Pre-Open queue for the 24/7 era, so the maintenance windows are served as
closed halts rather than order entry; the queue's absence is a sourced absence
like Rough Rice's, recorded under residual risks below. Dormant: the consumer
routes `SIL` to the Comex venue / `globex_energy` today, whose documented scope
excludes differently specified products — the routing observation is issue
#193, and this key is what the consumer re-routes the root to.

## Revision rows

- 2015-09-20 — T1 — CME Globex notice 20150907 — the family row this product rode, restated by reference: every COMEX and NYMEX close moves to 16:00 CT for trade date Monday 2015-09-21; keyed to the Sunday opening day. Documented in [comex.md](comex.md), which resolves the notice.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — the family knowledge-bound row this product rode, restated by reference: the Sunday queue widens to the sourced current 16:00-17:00 CT Pre-Open.
- 2026-09-11 — T1 — CME Globex notice 20260907 — the 24/7 bridge day: "Effective this Friday, September 11 ... Production 100-Ounce Silver futures weekend trading will begin at 4:30 p.m. Central Time"; Thursday's leg closes Friday 16:00 under the old grid, that Friday's standard two-minute daily maintenance is extended "from two minutes to 30 minutes, from 4:00 p.m. to 4:30 p.m. CT", and the first weekend leg opens 16:30 CT.
- 2026-09-12 — T1 — CME Globex notice 20260907 — the permanent 24/7 grid from the Saturday: continuous trading behind the two-minute daily maintenance and the two-hour weekend window, weekend trade dates rolling to the following business day.
- 2026-09-19 — T1 — CME Globex notice 20260824 — one-day Saturday maintenance-window extension to 02:00-08:00 CT; reopen 08:00 CT. The notice's extension table scopes it to the channels including MDP channel 329 — the channel the same notice moves this product onto — so it governs a product that had been a 24/7 market for eight days. Confirmed effective by the operator's T2 service on 2026-09-27 UTC for the shared window (closed 02:00, preopen 07:45, open 08:00, trade date 2026-09-21; store `holidays/raw/cme-globex/release-inspection-20260928/thbp_sep2026_sat.raw`), discharging the confirm-by obligation.
- 2026-09-20 — T1 — CME Globex notice 20260824 — revert to the standard 02:00-04:00 CT Saturday window.
- 2026-10-03 — T1 — CME Globex notice 20260921 — one-day Saturday maintenance-window extension to 02:00-05:00 CT for the 24/7 markets; reopen 05:00 CT. Forward-dated on the operator's statement.
- 2026-10-04 — T1 — CME Globex notice 20260921 — revert to the standard 02:00-04:00 CT Saturday window.
- 2026-10-24 — T1 — CME Globex notice 20260921 — one-day Saturday maintenance-window extension to 02:00-15:30 CT for the FIA industry disaster-recovery exercise; reopen 15:30 CT. Forward-dated on the operator's statement.
- 2026-10-25 — T1 — CME Globex notice 20260921 — revert to the standard 02:00-04:00 CT Saturday window.

## Sources

Row review: 2026-10-01 (UTC) is the date the ledger row was created and the
rows above were verified against the captured bytes. The 2026 notices were
retrieved in the Stage 7 release-month re-inspection, one pass on 2026-09-27
19:16-19:44 UTC through the public reader `r.jina.ai` (`cmegroup.com` 403s the
retrieval machine, proved negative in prior waves); the artifacts are the
research store's `holidays/raw/cme-globex/release-inspection-20260928/`, whose
`INDEX.md` records the session and whose `SHA256SUMS.txt` hashes every file.

- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260817.html> — CME Globex notice 20260817 — "Update 100-Ounce Silver Futures Migration and Expansion to 24/7 Trading - August 30": the channel/segment migration effective Sunday, August 30 (trade date Monday, August 31), the 24/7 expansion, production weekend trading from Friday, September 11 at 4:30 p.m. CT, the September 11 maintenance extension 16:00-16:30 CT, subsequent Fridays opening "at their regular time of 4:02 p.m. CT", continuous trading with "at least a two-hour weekly maintenance period over the weekend", and the following-business-day trade-date roll.
- <https://www.cmegroup.com/notices/electronic-trading/2026/09/20260907.html> — CME Globex notice 20260907 — "100-Ounce Silver Futures Migration and Expansion to 24/7 Trading - This Week": "Effective this Friday, September 11", the new security group 4S, and the same maintenance and trade-date statements; the lineage-winner for the expansion's effective day.
- <https://www.cmegroup.com/notices/electronic-trading/2026/09/20260921.html> — CME Globex notice 20260921 — "24/7 Maintenance Window Schedule Extension - October 3" (02:00-05:00 CT) and "- October 24" (02:00-15:30 CT, the FIA industry disaster-recovery exercise), each for "24/7 markets" and each reverting "to its 2:00 a.m. – 4:00 a.m. CT standard schedule".
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html> — CME Globex notice 20260824 — "24/7 Maintenance Window Schedule Extension - Starting This Week": Saturday, August 29 (02:00-06:00 CT) and Saturday, September 19 (02:00-08:00 CT), with the extension table naming MDP channels 326, 327, 329 and 333 — channel 329 being the channel this product migrates onto in the same notice. Also restates the 100-oz Silver go-live for the week of Sunday, August 30.
- <https://www.cmegroup.com/services/trading-hours-by-product> — the operator's own trading-hours service (T2), window 2026-09-18..2026-09-22 read 2026-09-27 ~19:21 UTC (`thbp_sep2026_sat.raw`): the Saturday 2026-09-19 window closed 02:00, preopen 07:45, open 08:00 with trade date 2026-09-21, confirming the extension effective.
- The 2015-09-20 and 2026-08-22 rows restate the energy/metals family's own rows; notice 20150907 and the 2026-08-22 review are documented and resolved in [comex.md](comex.md) and [globex_energy.md](globex_energy.md), whose scope statements name `SI`/`SIL` explicitly.

### Documents

The three 2026 notices this module's own rows cite, all at **T1**, each read
through the public reader and saved in the research store directory above.
`CME-GLOBEX-NOTICE-20260921` is the id [globex_cryptocurrency.md](globex_cryptocurrency.md)
already minted for the same artifact; the same id resolves to the same bytes
repository-wide. The 2015-09-20 and 2026-08-22 rows cite no document of their
own here — they are the family rows restated by reference, resolved in
[comex.md](comex.md).

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `CME-GLOBEX-NOTICE-20260817` | the 100-oz Silver 4S migration and 24/7 expansion announcement | <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260817.html> read through the public reader `r.jina.ai` | retrieved 2026-09-27T19:25Z | T1 | `bf594642d8ee0671390e55c9c4eaecc8f65f34b50a9c1af57310fd06bd9f995b` |
| `CME-GLOBEX-NOTICE-20260907` | the 100-oz Silver go-live restatement, effective Friday 2026-09-11 | <https://www.cmegroup.com/notices/electronic-trading/2026/09/20260907.html> read through the public reader `r.jina.ai` | retrieved 2026-09-27T19:25Z | T1 | `d4fa16963386e57d4eafa7e1b32713d8636ab7c4d6d07a823b47740544692c9b` |
| `CME-GLOBEX-NOTICE-20260921` | the 2026-10-03 and 2026-10-24 Saturday maintenance-window extensions | <https://www.cmegroup.com/notices/electronic-trading/2026/09/20260921.html> read through the public reader `r.jina.ai` | retrieved 2026-09-27T19:31Z | T1 | `47387659655a80d7aac49b84ba98a860a7d5ca402a9d4d85c90f2fb570e7596c` |
| `CME-GLOBEX-NOTICE-20260824` | the 2026-08-29 and 2026-09-19 Saturday maintenance-window extensions, and the silver migration restatement | <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html> read through the public reader `r.jina.ai` | retrieved 2026-09-06 UTC | T1 | `619e7995d5dd6a72ca2acd18416b0497015a4b6bdc1cde20ed9e15f0896019b0` |

## Gaps and residual risks

- **order-entry** — no artifact in hand states a Pre-Open queue for the 24/7 era: the notices state the maintenance windows (the two-minute daily 16:00-16:02 CT halt and the Saturday 02:00-04:00 CT weekly window) and nothing between them, so the module models no `order_entry` rule, exactly as `rough_rice.rs` models the queues its specification does not publish. Closing condition: a CME artifact that states this product's 24/7-era Pre-Open in session language on a day-level effective date — most likely the client impact assessment the notice links ("100-Ounce Silver Futures Expansion to 24-7 Trading"), which sits behind the client wiki's login and is admissible only as T2 with the retrieved artifact saved. Dormant identity, so recorded here rather than opened as an issue.
- **the two announcement dates, ordered by lineage** — notice 20260817 heads its expansion "Effective Sunday, August 30 (trade date Monday, August 31)" but defers production weekend trading to Friday, September 11, and notice 20260907 — the later operator document, issued the week of the change — states the expansion itself "Effective this Friday, September 11". The module keys the only observable session change (the weekend leg and its maintenance windows) to the day both documents agree on, and treats the August 30/31 migration as market-data plumbing, which the store's INDEX records as "market data only, no session change" for the sibling livestock migration of the same shape. No row is keyed to a disputed reading.
- **forward-dated rows** — the 2026-10-03 and 2026-10-24 Saturday extensions are encoded ahead of their effective days on notice 20260921. Each must be confirmed against the operator before its effective day (LAW-WATCH); the 2026-09-19 extension of the same class was **confirmed effective** by the operator's T2 service on 2026-09-27 UTC, discharging its own confirm-by obligation. The October 24 window is a 13.5-hour halt: it exceeds the four-hour operator-designated bound the crate's maintenance policy keeps and falls inside one trade date — the weekend block still carries the following Monday's — so `session_state` classifies the gap `Halt`, not `Maintenance`, the same reading `globex_cryptocurrency` ships for the same window.
- **the October extensions' scope is the class, and one sibling module withholds them** — notice 20260921 extends the window "for 24/7 markets" without a channel table, so this module reads it as governing every 24/7 market, this product included from its 2026-09-11 cutover — the reading `globex_cryptocurrency` also ships. `globex_event_contracts_btc`, the other channel-329 family, models only the extensions whose notices carry a channel table and states no position on 20260921; the two modules' disagreement is disclosed rather than resolved here, and one operator statement naming channel 329 for the October windows would settle it.
- **no built-in holiday table routes here** — the pre-cutover era shares the energy/metals holiday arrangement (documented in [globex_energy.md](globex_energy.md)), but the routing layer attaches one table per key and the 24/7 era's holiday arrangement is not yet sourced: whether the product appears in the operator's T2 trading-hours service under its new group 4S from the cutover, and with which holiday rows, is exactly what that service would state. Activation condition (LAW-SERVICE-TIERS, dormant row): source the 24/7 era's holiday rows from the T2 service before a consumer maps `SIL` here; until then the identity answers no holiday claim rather than a mis-applied family row.
- **dormant until the consumer re-routes** — SharurPlatform maps `SIL` to the Comex venue / `globex_energy` today (issue #193); that mapped family's documented scope excludes differently specified products and does not state the weekend availability this product has had since 2026-09-11. This key is the crate-side fix; the consumer's map decides when the root moves.
- **why a separate key, though the envelope coincides with `globex_cryptocurrency` today** — the 24/7 shape is the operator's class-wide template and the weekday envelopes coincide, but the histories share nothing: this product rode the energy/metals grid from the January-2010 floor and carried the family's dated revisions, and the cryptocurrency family launched on its own five-day grid in 2017 and carries its own. A venue coinciding with another still gets its own named profile.

## Module narrative (moved from src/calendar/schedules/futures/us/silver_100oz.rs on 2026-10-01 UTC)

COMEX 100-Ounce Silver futures in America/Chicago. The product rode the shared
NYMEX/COMEX energy-and-metals grid — `energy_metals.rs`'s tables, whose scope
statements name `SI`/`SIL` — from the January-2010 floor, and its pre-cutover
rows restate that family's own dated rows by reference so one owner keeps the
family's evidence. CME Globex notice 20260907 expands the product to 24/7
trading "Effective this Friday, September 11", with production weekend trading
beginning 4:30 p.m. CT behind a one-day 30-minute maintenance extension and
every later Friday opening "at their regular time of 4:02 p.m. CT". The key
carries the product's whole life so a caller never switches keys on a date —
the same shape `bitcoin_event_contracts.rs` ships for the one event-contract
root CME moved to 24/7 — and `SessionRule` spans at most one local midnight,
so the continuous Friday-evening-to-Monday weekend block is stored in day
pieces the key-backed calendar joins at query time, carrying the following
open business date the notice states.

The Saturday extensions modelled here are the ones that govern the product
after its cutover: notice 20260824's table scopes 2026-08-29 and 2026-09-19 to
MDP channels 326, 327, 329 and 333 — channel 329 being the channel the same
notice moves `SIL` onto — so only the September date falls after the go-live
and only it is modelled from that notice; notice 20260921 extends the window
"for 24/7 markets" on 2026-10-03 (to 05:00 CT) and 2026-10-24 (to 15:30 CT,
the FIA disaster-recovery exercise). The August extensions predate the
go-live — on those days the product still traded the five-day energy/metals
grid — and never governed it.

No Pre-Open queue is modelled in the 24/7 era because no captured source
states one for this product; the residual-risk entry above records what would
close that. The trade-date roll follows the operator's statement — weekend
trading carries the following business date — through the same identity
convention `globex_cryptocurrency` and `globex_event_contracts_btc` ship.
