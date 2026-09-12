<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_event_contracts` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`event_contracts.rs`](../../src/calendar/schedules/futures/us/event_contracts.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the family's one grid is sourced, but its daily close is carried back from 2025 to the 2022 launch, and a close instant governs a window where trades print. CME Group Event Contracts on futures: the Chapter 23 roots `ECES`, `ECNQ`, `ECRTY`, `ECYM`, `EC6E`, `ECCL`, `ECNG`, `ECGC`, `ECSI` and `ECHG`, plus `ECBTC` from 2023-03-12 to 2026-05-28, together with the hourly Chapter 23A roots (`ECS*`, `ECN*`, `ECR*`, `ECD*`, `ECC*`, `ECH*`, `ECG*`) listed from 2025-12-08. Excludes the Chapter 22 swap-based economic and cryptocurrency event contracts, whose SER-9586RR grid is 24/7 with Saturday and Tuesday 03:00–05:00 ET maintenance and a one-minute 17:00 ET daily halt, and the sports and political contracts on the CME FutureSports Performance Indexes. **A Termination of Trading time is not this family's session close, and that is the whole modelling question.** CME writes every hours cell as "Sunday 5:00 p.m. - `T`" with six different `T` values across the ten Chapter 23 roots (15:00 CT for the equity indices down to 12:00 CT for `ECHG`); read as daily closes they would force six grids. SER-9624 refutes that directly by printing five contracts with five *different* daily Termination times — 10:00/11:00/13:00/15:00/16:00 ET — against **one** Trading Hours cell, "CME Globex: Sunday 6:00 p.m. - Friday 5:00 p.m. ET with a daily maintenance period from 5:00 p.m. - 6:00 p.m. ET", under the footnote "Termination of Trading is contingent on the Contract's Stated Expiration Time". Clearing advisory 25-326 glosses the same cell shape itself — "Sunday 5:00 p.m.- stated expiration time of Event Contract (i.e., Monday's ECS10: Sunday 5:00 p.m.- Monday 10:00 a.m.)" — rulebook 2302.A defers the schedule to the Exchange while 2302.E puts termination in its own per-contract rule, and SER-9586RR carries the identical five termination times against a wholly different session. So one key, not six. No regular session in any era: no CME document about this family uses RTH or ETH or splits the window, so the whole 17:00→16:00 CT wrap is `extended`. **Partial because the close is carried back.** SER-8968R states "Effective Sunday, September 18, 2022, for trade date Monday, September 19, 2022" with the Pre-Opens (Sunday 16:00–17:00, Monday–Thursday 16:45–17:00 CT), the 17:00 CT open and the daily relist, but no daily close; 16:00 CT comes from SER-9624 (2025-10-15) and from SER-9740R (2026-05-28), whose Table 2 states the same grid in Central time for a table that includes `ECBTC`. No primary source names a cutover between them, so the grid is carried back to the launch day under the carry-back convention rather than split at an invented date. The row is keyed to the venue-local opening day, since the first session opens Sunday evening. SER-8968 (2022-03-30) is preliminary, superseded and dateless, and must not be used for the launch. 2023-03-12 (SER-9092, `ECBTC`) and 2025-12-08 (rule filing 25-521, the hourly contracts) are member catalog data: both reprint the family's hours cell unchanged. SER-9587RR amends contract size only. The "ECGE" → "EC6E" correction visible between the 2023-12-07 and 2024-03-30 captures of CME's specification page is a typo fix on a marketing page, not a dated revision. **`ECBTC` leaves this key on 2026-05-29 for its own.** SER-9740R expands that root alone to 24/7 "Effective Friday, May 29, 2026", and CME's client-systems wiki scopes it — "only the below event contracts on Bitcoin on channel 329 will migrate to weekend trading. Other event contracts will continue on the current schedule." That is a session change, not another expiry artifact, so the root genuinely departs; `globex_event_contracts_btc` carries it in every era and records the hour-wide conflict between CME's two statements of its new close. **Knowledge bound, stated rather than hidden.** CME has published no hours cell for the ten Chapter 23 daily roots since clearing advisory 24-011 (2024-01-10): the specification page now redirects to a prediction-markets page carrying none, and the current specification PDF lists every root while containing the string "Trading Hours" zero times. The 2026-09-05 evidence pass also paged CME's product-slate and trading-hours services to exhaustion and found no event-contract product in either — a negative recorded from that pass rather than re-derived, since cmegroup.com refuses this machine at IP level. The most recent affirmative statement is the wiki's continuity sentence above. **Not foldable.** The envelope is identical to `globex_spot_quoted` and `globex_weather` at the 2026-09-05 review and no history overlaps — weather closed 15:15 CT until 2025-04-13, spot-quoted did not exist until 2025-06-29, and a single midweek afternoon in 2023 separates all three. `globex_cryptocurrency` carries Bitcoin futures history from 2017 with no sourced Pre-Open in its five-day era and runs the weekend from 2026-05-29, which this family does not.

## Revision rows

- 2022-09-18 — T1 — CME SER-8968R — the launch grid: "Effective Sunday, September 18, 2022, for trade date Monday, September 19, 2022", the 17:00 CT open, both Pre-Opens and the daily relist; keyed to the venue-local opening day.

## Sources

Row review: 2026-09-05 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/notices/ser/2022/08/SER-8968R.pdf> — CME SER-8968R — the 2022-09-18 launch, with the Pre-Opens and the 17:00 CT open but no daily close.
- <https://www.cmegroup.com/notices/ser/2025/10/ser-9624.pdf> — CME SER-9624 (2025-10-15) — five Termination times against one Trading Hours cell, under the footnote "Termination of Trading is contingent on the Contract's Stated Expiration Time".
- <https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf> — CME SER-9740R (2026-05-28) — Table 2 states the same grid in Central time for a table that includes `ECBTC`.
- <https://www.cmegroup.com/content/dam/cmegroup/notices/clearing/2025/10/chadv25-326.pdf> — CME clearing advisory 25-326 — glosses the hours cell shape: "Sunday 5:00 p.m.- stated expiration time of Event Contract".
- <https://www.cmegroup.com/notices/ser/2025/11/ser-9586rr.pdf> — CME SER-9586RR — the Chapter 22 swap-based contracts, carrying the identical five Termination times against a wholly different session.
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2025/12/25-521.pdf> — CME rule filing 25-521 — the hourly Chapter 23A contracts listed 2025-12-08, reprinting the hours cell unchanged.
- <https://www.cmegroup.com/content/dam/cmegroup/notices/clearing/2022/09/Chadv22-355.pdf> — CME clearing advisory 22-355.
- <https://www.cmegroup.com/notices/clearing/2024/01/Chadv24-011.pdf> — CME clearing advisory 24-011 (2024-01-10) — the last CME hours cell published for the ten Chapter 23 daily roots.
- <https://www.cmegroup.com/markets/prediction-markets/files/summary-of-rulebook-chapters.pdf> — CME prediction markets, summary of rulebook chapters — Rule 2302.A defers the schedule to the Exchange while 2302.E puts termination in its own per-contract rule.
- <https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1394343937/Event-Based+Contracts+Expansion+to+24-7+Trading> — CME client-systems wiki — "only the below event contracts on Bitcoin on channel 329 will migrate to weekend trading. Other event contracts will continue on the current schedule.".

## Gaps and residual risks

- **executable** — the daily close is carried back from 2025 to the 2022 launch. SER-8968R states the launch day, the Pre-Opens, the 17:00 CT open and the daily relist but no daily close; 16:00 CT comes from SER-9624 and SER-9740R, and no primary source names a cutover between them, so the grid is carried back under the carry-back convention rather than split at an invented date. A close instant governs a window where trades print. Closing condition: a CME artifact stating this family's daily close between 2022-09-18 and 2025-10-15. Dormant identity, so recorded here rather than opened as an issue.
- **knowledge bound, stated rather than hidden** — CME has published no hours cell for the ten Chapter 23 daily roots since clearing advisory 24-011 (2024-01-10): the specification page now redirects to a prediction-markets page carrying none, and the current specification PDF lists every root while containing the string "Trading Hours" zero times. The 2026-09-05 evidence pass also paged CME's product-slate and trading-hours services to exhaustion and found no event-contract product in either — a negative recorded from that pass rather than re-derived, since cmegroup.com refuses this machine at IP level. The most recent affirmative statement is the wiki's continuity sentence.
- **do not use** — SER-8968 (2022-03-30) is preliminary, superseded and dateless, and must not be used for the launch.
- **catalog, not schedule** — 2023-03-12 (SER-9092, `ECBTC`) and 2025-12-08 (rule filing 25-521, the hourly contracts) both reprint the family's hours cell unchanged. SER-9587RR amends contract size only. The "ECGE" to "EC6E" correction visible between the 2023-12-07 and 2024-03-30 captures of CME's specification page is a typo fix on a marketing page, not a dated revision.
- **root departure** — `ECBTC` leaves this key on 2026-05-29 for its own. SER-9740R expands that root alone to 24/7 "Effective Friday, May 29, 2026" and CME's client-systems wiki scopes it, so the root genuinely departs; `globex_event_contracts_btc` carries it in every era.
- **excluded** — the Chapter 22 swap-based economic and cryptocurrency event contracts, whose SER-9586RR grid is 24/7 with Saturday and Tuesday 03:00-05:00 ET maintenance and a one-minute 17:00 ET daily halt, and the sports and political contracts on the CME FutureSports Performance Indexes.
- **no regular session** — no CME document about this family uses RTH or ETH or splits the window, so the whole 17:00 to 16:00 CT wrap is `extended`.

## Module narrative (moved from src/calendar/schedules/futures/us/event_contracts.rs on 2026-09-12 UTC)

CME Group Event Contracts on futures in America/Chicago: daily-expiring,
cash-settled, European-style options on futures, listed under CME, CBOT,
NYMEX and COMEX Rulebook Chapter 23 and — for the hourly contracts — Chapter
23A. The Chapter 23 roots are `ECES`, `ECNQ`, `ECRTY` and `ECYM` on the
equity indices, `EC6E` on EUR/USD, `ECCL` and `ECNG` on NYMEX energy, and
`ECGC`, `ECSI` and `ECHG` on COMEX metals, plus `ECBTC` on Bitcoin futures
until 2026-05-29 (see below; `bitcoin_event_contracts.rs` carries that root). The Chapter 23A roots are the hourly
contracts `ECS10`-`ECS15`, `ECN10`-`ECN15`, `ECR10`-`ECR15`,
`ECD10`-`ECD15`, `ECC10`-`ECC16`, `ECH10`-`ECH16` and `ECG10`-`ECG16`.
https://www.cmegroup.com/notices/ser/2022/08/SER-8968R.pdf
https://www.cmegroup.com/notices/ser/2025/10/ser-9624.pdf

TERMINATION OF TRADING IS NOT A SESSION CLOSE, AND THAT IS THE WHOLE
MODELLING QUESTION FOR THIS FAMILY. Every CME hours cell for these products
is written in the shape "Sunday 5:00 p.m. - <time>", one line per root, with
six different `<time>` values across the ten Chapter 23 roots: 15:00 CT for
the equity indices, 14:00 for `EC6E`, 13:30 for `ECCL`/`ECNG`, 12:30 for
`ECGC`, 12:25 for `ECSI` and 12:00 for `ECHG`. Read as daily session closes
those six numbers would force six separate grids. They are not session
closes; they are the contracts' stated expiration times, and CME says so in
three independent ways.

First, SER-9624 (15 October 2025) prints **five** event contracts with five
*different* daily Termination-of-Trading times — "10:00AM ET daily",
"11:00AM ET daily", "1:00PM ET daily", "3:00PM ET daily", "4:00PM ET daily"
— against **one** Trading Hours cell: "CME Globex Pre-Open: Sunday 5:00 p.m.
- 6:00 p.m. Eastern Time/ET / Monday - Thursday 5:45 p.m. - 6:00 p.m. ET //
CME Globex: Sunday 6:00 p.m. - Friday 5:00 p.m. ET with a daily maintenance
period from 5:00 p.m. - 6:00 p.m. ET // The next day's Event Contract will
be listed at 5:00p.m. ET", under the footnote "Termination of Trading is
contingent on the Contract's Stated Expiration Time". If termination were
the session close that table would need five hours cells; it has one. When
CME means a daily session close it writes it as a named clause — "a daily
maintenance period from 5:00 p.m. - 6:00 p.m. ET" — whose close, 17:00 ET,
is not any of the five termination times.

Second, CME Clearing advisory 25-326 writes the same "Sunday 5:00 p.m. -
<time>" shape and glosses the end of the range itself: "Sunday 5:00 p.m.-
stated expiration time of Event Contract (i.e., Monday's ECS10: Sunday 5:00
p.m.- Monday 10:00 a.m.)." That is a contract lifetime, not a weekly
session.

Third, the rulebook separates the two concepts. Chapter 23 rule 2302.A
Trading Schedule says only "Event Contracts shall be listed for Expiration
on such dates and shall be scheduled for trading during such hours as may be
determined by the Exchange", while 2302.E Termination of Trading is its own
rule tied to one contract's Expiration Date: "Trading shall terminate at the
end of the daily settlement period for the Underlying Futures, on the
Expiration Date that the Event Contract was listed for."

SER-9586RR settles it from the other side. The Chapter 22 swap-based event
contracts carry the *same* five termination times — "10:00AM ET", "11:00AM
ET", "01:00PM ET", "03:00PM ET", "04:00PM ET" — against a completely
different session: "24-hour trading, 7 days a week inclusive of up to two
2-hour maintenance periods per week from 3:00 A.M. Eastern Time/ET - 5:00
A.M. ET on Saturday and Tuesday. In addition, there will be a one (1) minute
trading halt each day from 5:00 P.M. ET to 5:01 P.M. ET". Identical
termination times, unrelated grids: the two axes are independent.
https://www.cmegroup.com/content/dam/cmegroup/notices/clearing/2025/10/chadv25-326.pdf
https://www.cmegroup.com/markets/prediction-markets/files/summary-of-rulebook-chapters.pdf
https://www.cmegroup.com/notices/ser/2025/11/ser-9586rr.pdf

SCOPE — NOT EVERY PRODUCT CME SELLS AS AN "EVENT CONTRACT". Three neighbours
share the name and not the clock, and none is covered here. The Chapter 22
swap-based economic and crypto event contracts (`ECCPI`, `ECGDP`, `ECNFP`,
`ECUNE`, `ECPCE`, `ECFD`, and the hourly `CCB10`/`CCB11`/`CCB13`/`CCB15`/
`CCB16`) run the 24/7 grid quoted above. The sports and political event
contracts on the CME FutureSports Performance Indexes are a separate listing
entirely. And `ECBTC` leaves this family on 2026-05-29, below.

ONE KEY, NOT SIX OR SEVEN. Once the six per-root numbers are read as
expiries, every element CME publishes as a *session* is identical across all
eleven Chapter 23 roots and the Chapter 23A roots too, and has never been
amended: the Sunday 16:00-17:00 CT queue, the Monday-Thursday 16:45-17:00 CT
queue, the 17:00 CT open, the 60-minute daily maintenance period from 16:00
CT, the daily relist, and the absent Friday-evening reopen. Four
independent CME publications print that cell with no per-root variation in
any of it — SER-8968R (2022-08-25), Clearing advisory 22-355 (2022-09-20),
SER-9092 (2023-02-14) and Clearing advisory 24-011 (2024-01-10) — and the
last of them lists all eleven roots including `ECGC`, which CME's own
specification web page never named.
https://www.cmegroup.com/content/dam/cmegroup/notices/clearing/2022/09/Chadv22-355.pdf
https://www.cmegroup.com/notices/clearing/2024/01/Chadv24-011.pdf

THE DAILY CLOSE IS SOURCED IN SESSION IDIOM TWICE, AND CARRIED BACK. The
2022 launch document states the queues, the Sunday open and the relist but
no daily close: 16:00 CT comes from SER-9624's maintenance clause (October
2025, quoted above, Chapter 23A) and from SER-9740R (28 May 2026), whose
Table 1 includes "Event Contracts on Bitcoin Futures | ECBTC | 23" and whose
Table 2 gives that table's current CME Globex hours in Central time: "CME
Globex Pre-open: Sunday 4:00 p.m. - 5:00 p.m. CT / Monday - Thursday 4:45
p.m. - 5:00 p.m. CT // CME Globex: Sunday 5:00 p.m. - Friday - 4:00 p.m. CT
with a daily maintenance period from 4:00 p.m. - 5:00 p.m. CT". The two
agree exactly, one stated in Eastern time and one in Central. No primary
source names a cutover between the 2022 launch and either of them, so the
grid is carried back to the launch day under the crate's carry-back
convention rather than split at an invented date — which is also why the
ledger row is **Partial** and says the gap is executable: the close instant
governs a window where trades print, and for 2022-09-18..2025-10-15 it rests
on carry-back rather than on a launch-day statement. The 16:45 CT weekday
Pre-Open in the launch document is independent corroboration that a daily
close exists at or before 16:45 from day one; only its exact instant is
carried.
https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf

`ECBTC` — CATALOG DATA IN 2023, OUT OF SCOPE FROM 2026-05-29. CME listed
Event Contracts on Bitcoin Futures with SER-9092, "Effective Sunday, March
12, 2023, for trade date Monday, March 13, 2023", whose Exhibit 1 reprints
the family's hours cell unchanged ("CME Globex Pre-Open: Sunday 4:00 - 5:00
p.m. Monday - Thursday 4:45 - 5:00 p.m. / CME Globex: Sunday 5:00 p.m.-
Friday 3:00 p.m. - Next day's Event Contract will list at 5:00 p.m."). The
family clock does not move, so that listing is member catalog data and not a
revision row — the same call `mini_grains.rs` makes for `MKC` and
`spot_quoted.rs` for `QSOL`/`QXRP`.

Its 2026 departure is the opposite case, and it is a genuine session change
rather than another expiry artifact: SER-9740R expands "the trading hours on
the CME Globex electronic trading platform ... for all cryptocurrency
futures and options on futures contracts noted in Table 1", ECBTC included,
"Effective Friday, May 29, 2026", to "24/7 with the exception of the
following maintenance windows: Saturday 2:00 a.m. to 4:00 a.m. CT.
Monday-Friday 4:00p.m. to 4:02 p.m. CT". CME's own client-systems wiki scopes
it to that one root — "only the below event contracts on Bitcoin on channel
329 will migrate to weekend trading. Other event contracts will continue on
the current schedule." — with a Product Scope table whose single row is
"Event contracts on Bitcoin Futures | ECBTC | VB | 329 | 74". So `ECBTC`
rides this key from 2023-03-12 to 2026-05-28 and a caller must stop using it
for that root on 2026-05-29.

`ECBTC` AFTER THAT DAY HAS ITS OWN KEY, AND ITS CLOSE IS AN INTERSECTION.
Two CME primary sources give the root's new daily maintenance window
different start times — SER-9740R says 16:00 to 16:02 CT, the client-systems
wiki says "Close: 3:00:00 p.m. to 4:01:00 p.m. CT" — and agree on everything
else. `bitcoin_event_contracts.rs` records the conflict, establishes from the
wiki's own version history that its figure is the earlier and never-revised
statement, and serves the window both documents support: open 16:02→15:00
CT, the disputed hour withheld. That the disputed hour coincides with this
root's old expiry instant is not evidence either way: LAW-SESSION-NOT-EXPIRY
puts instrument lifecycle outside this crate, so an expiry can neither
corroborate nor refute a close.
https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1394343937/Event-Based+Contracts+Expansion+to+24-7+Trading

THE HOURLY CONTRACTS ARE IN SCOPE AND CHANGE NO CLOCK. CME rule filing
25-521 reproduces SER-9624's exhibit with an "Initial Listing Schedule" of
"December 8, 2025" for Rulebook Chapter 23A and the identical Trading Hours
cell. Their arrival is therefore member catalog data on the same test as
`ECBTC`'s 2023 listing. They are in scope deliberately rather than
incidentally: SER-9624 is the document that states this family's daily close
in session idiom, and a key that excluded the products that document lists
would be sourcing its own grid from a document about something else — the
error `sgx.rs` records for the FTSE Taiwan suite.
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2025/12/25-521.pdf

NO REGULAR SESSION, AND IT IS A SOURCED ABSENCE. No CME document retrieved
for this family — SER-8968R, SER-9092, SER-9624, SER-9586RR, advisories
22-355, 24-011 and 25-326, rule filings 25-521 and 25-522, or any archived
capture of CME's own event-contract specification page from 2022-12-01 to
2025-09-16 — splits the session into Regular and Extended, or uses the words
RTH or ETH about it. Each publishes one undifferentiated "CME Globex"
window plus a Pre-Open, and rule 2302.A defers the schedule to the Exchange
entirely. The whole 17:00-16:00 CT window is therefore `extended` with
`regular` empty, exactly as `spot_quoted.rs` and `cryptocurrency.rs` treat
their own families.

NOT `globex_spot_quoted` AND NOT `globex_weather`, THOUGH ALL THREE SHARE
ONE ENVELOPE TODAY. This is now the third pair the crate records, and the
histories separate cleanly: weather closed 15:15 CT from the January-2010
floor until 2025-04-13, spot-quoted did not exist before 2025-06-29, and
this family has run its one grid since 2022-09-18. A midweek afternoon in
2023 tells all three apart at a single instant.

NOT `globex_cryptocurrency` either, even for `ECBTC`'s own era: that key
carries Bitcoin *futures* history from 2017-12-17 with no sourced Pre-Open
in its five-day era, where this family's queues are stated on its launch
day, and it runs the weekend from 2026-05-29 while these contracts do not.

TIMEZONE. CME states this family's hours in Central time in the 2022 and
2023 SERs — SER-8968R and SER-9092 both head the cell "All times are in
Central Prevailing Time (CPT)", and advisory 24-011 writes "All times are in
Central Time (CT)" — and in Eastern time in the 2025 hourly documents, which
head it "All times are in Eastern Time (ET)". ET and CT shift together, so
the two publications agree year-round and the Central grid is the stable one
to encode.

---

2022-09-18 — THE LAUNCH, AND THE FAMILY'S ONLY REVISION ROW. CME SER-8968R
(notice date 25 August 2022), subject "Initial Listing of Event Contracts on
Certain CME Group Futures Contracts", states: "Effective Sunday, September
18, 2022, for trade date Monday, September 19, 2022, and pending all
relevant CFTC regulatory review periods, Chicago Mercantile Exchange Inc.
(\"CME\"), The Board of Trade of the City of Chicago, Inc. (\"CBOT\"), New
York Mercantile Exchange, Inc. (\"NYMEX\") and Commodity Exchange, Inc.
(\"COMEX\") (collectively the \"Exchanges\") will list Event Contracts
(\"Event Contracts\") on certain CME Group futures contracts as listed in
Table 1. for trading on the CME Globex electronic trading platform (\"CME
Globex\")." Table 1 lists the ten Chapter 23 roots; Exhibit 1's Trading
Hours row opens "All Contracts: CME Globex Pre Open: Sunday 4:00 - 5:00 p.m.
Monday - Thursday 4:45 - 5:00 p.m." and closes each per-root line with "Next
day's Event Contract will list at 5:00 p.m."

DATE KEYING. CME states both days and they mean different things. The row is
keyed to the venue-local **opening** day, Sunday 2022-09-18, not the stated
trade date of Monday 2022-09-19: the first session that exists at all is the
one opening Sunday at 17:00 CT (queueing from 16:00 CT) and closing on the
Monday trade date, and a wrapped rule keyed a day late would render that
Sunday evening closed. This is the same convention `weather.rs`,
`rough_rice.rs`, `mini_grains.rs` and `spot_quoted.rs` record for their own
rows.

SER-8968 (30 March 2022) must not be used to date this. It is expressly
preliminary and superseded — "SER 8968R supersedes SER 8968 dated March 30,
2022, which provided preliminary information on the upcoming listing" — it
states no effective day, and it omits the Pre-Open row entirely.
https://www.cmegroup.com/notices/ser/2022/08/SER-8968R.pdf

NON-EVENTS DELIBERATELY NOT RECORDED AS REVISIONS. SER-9587RR (26 November
2025, effective Sunday 2025-12-07 for trade date Monday 2025-12-08) amends
contract size and minimum price increment only and carries no hours change.
CME's own event-contract specification web page shows a garbled "ECGE |
Sunday 5:00 p.m. - Friday 3:00 p.m." row in its 2022-2023 captures replaced
by "EC6E | Sunday 5:00 p.m. - Friday 2:00 p.m." by 2024-03-30; that is a
typo correction on a marketing page, since SER-8968R already stated the
EC6E figure at launch, and per LAW-NO-FABRICATED-DATES a capture date may
not date a cutover. That page never named `ECGC` at all, which is why the
clearing advisories rather than the web page are cited for the full root
list.

KNOWLEDGE BOUND, STATED RATHER THAN HIDDEN. CME has published no hours cell
for the ten Chapter 23 daily roots since advisory 24-011 (2024-01-10): the
specification page that carried one now redirects to a prediction-markets
landing page with no hours, and the current event-contract specification PDF
lists every root but contains the string "Trading Hours" zero times. The
2026-09-05 evidence pass additionally paged CME's product-slate and
trading-hours services to exhaustion and found no event-contract product in
either; that negative is recorded rather than re-derived here, because
cmegroup.com now refuses this machine at IP level and the services cannot be
re-queried from it. The most recent affirmative statement about these roots
is the client-systems wiki's "Other event contracts will continue on the
current schedule." (revised 22 April 2026), which asserts continuity without
restating the grid. Under the carry-back convention no cutover is asserted,
which declines to invent one.

Revision evidence — the day-level effective date and the primary source that
states it:
  2022-09-18 "CME SER-8968R"
    https://www.cmegroup.com/notices/ser/2022/08/SER-8968R.pdf
Evidence: docs/evidence/globex_event_contracts.md
