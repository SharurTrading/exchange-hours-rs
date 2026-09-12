<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_event_contracts_btc` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`bitcoin_event_contracts.rs`](../../src/calendar/schedules/futures/us/bitcoin_event_contracts.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the 24/7 weekday close is the sourced intersection of two CME primaries that disagree by an hour, and the withheld hour is one in which trades may print. CME Event Contracts on Bitcoin Futures, `ECBTC`, Rulebook Chapter 23 — the one event-contract root CME moved to 24/7 trading on 2026-05-29. The key carries the root's whole life so a caller never switches keys on a date: sessionless before SER-9092's listing ("Effective Sunday, March 12, 2023, for trade date Monday, March 13, 2023", keyed to the Sunday opening day), the `globex_event_contracts` grid by reference to 2026-05-28, the transition day, then 24/7. **The conflict.** SER-9740R (28 May 2026) states "CME Globex: 24/7 with the exception of the following maintenance windows: Saturday 2:00 a.m. to 4:00 a.m. CT. Monday-Friday 4:00p.m. to 4:02 p.m. CT" with Pre-Opens "Monday - Friday: 4:01 p.m. to 4:02 p.m. CT Saturday: 3:45 a.m. to 4:00 a.m. CT"; CME's client-systems wiki page for this root states "Monday through Friday / Daily Maintenance Window (with Trade Date roll) / Close: 3:00:00 p.m. to 4:01:00 p.m. CT / Pre-open: 4:01:00 p.m. to 4:01:30 p.m. CT / No cancel: 4:01:30 p.m. to 4:02:00 p.m. CT / Open: 4:02:00 p.m. CT" and the same Saturday window with a 03:45 Pre-Open. One document closes weekdays at 16:00 CT, the other at 15:00; everything else matches. **The two statements are ordered and each has a weakness.** Confluence's version API shows the wiki table entered at version 4 — absent from versions 1–3 — under the page's own Revision History row "April 22, 2026 / Added Event-Based Contract Maintenance Windows and Market Hours schedule impacts", and shows no later version (the history has no `nextVersion`; version 5 answers 404), so the 15:00 is the earlier statement, never revised, still live. SER-9740R is later but its Table 2 was not written for this root: the SER says "No other changes have been made to the original SER" beyond adding it to Table 1, and its "Current" column — "Sunday 5:00 p.m. - Friday 4:00 p.m. CT with a daily maintenance period from 4:00 p.m. - 5:00 p.m. CT" — misstates the root's own prior cell, which SER-9092 and rule filing 23-014 both print as "Sunday 5:00 p.m.- Friday 3:00 p.m." under "CME Globex Trading Hours", separate from their Termination of Trading row. Rule filing 26-266 (4 June 2026, a Regulation 40.6(d) notification) reproduces SER-9740R verbatim as Exhibit A and is the same statement, not a second one. **Both channels are exhausted**: no SER, Globex notice, clearing advisory, product page, Daily Bulletin section, product-slate or trading-hours service entry, or the event-contract specification PDF (which has no "Trading Hours" field) restates this root's weekday window after the cutover, so waiting has no endpoint. **The key serves the intersection**: open 16:02→15:00 CT Monday–Friday with the 16:01–16:02 Pre-Open as order entry, Saturday 02:00–04:00 closed with the 03:45–04:00 Pre-Open, Sunday open — the window true under both documents — and withholds 15:00–16:00 as maintenance. Under SER-9740R's reading that under-reports an hour a day; under the wiki's it is exact; it never over-reports. **Independently confirmed after the cutover**: CME Globex notices 20260727, 20260824 and 20260831 each list "Event-Based Contracts / 329" among the 24/7 markets whose Saturday window reverts "to its 2:00 a.m. – 4:00 a.m. CT standard schedule", and date three one-day extensions — 2026-08-01 to 09:00, 2026-08-29 to 06:00 and 2026-09-19 to 08:00 CT — each modelled as a one-day row without a replacement Pre-Open, the September one forward-dated on the operator's statement. The same tables list the cryptocurrency channels, and `globex_cryptocurrency` models the same three extensions (#61). The key-backed calendar joins the Saturday-04:00-to-Monday-15:00 pieces into one block and carries the following open business date, as both documents' trade-date-roll language states. **Not evidence, recorded so it is not rediscovered**: CME's 8 July 2026 gold/crude deck prints the wiki's row template with "4:00:00 p.m." for a product migrating onto "329 (shared with events contracts)" — a statement about gold; cmegroup.com/trading-hours.html's "Event-Based Contracts / 16:01 CT / 16:00 CT" row is byte-identical before and after the cutover and still advertises a Tuesday window CME cancelled on 2026-03-03 for the neighbouring Event Contracts II channel; and the coincidence of 15:00 CT with the root's former daily settlement instant can neither corroborate nor refute a close under `LAW-SESSION-NOT-EXPIRY`. **Not `globex_cryptocurrency`**: that key has no sourced five-day Pre-Open and a 16:00 close; this root's Sunday 16:00–17:00 and Monday–Thursday 16:45–17:00 CT queues are stated on its listing day and its close under the intersection is 15:00. Partial because the disputed hour is withheld rather than sourced; one CME statement of the window after 2026-05-29 would settle it either way.

## Revision rows

- 2023-03-12 — T1 — CME SER-9092 — the listing day: "Effective Sunday, March 12, 2023, for trade date Monday, March 13, 2023"; keyed to the venue-local opening day.
- 2026-05-29 — T1 — CME SER-9740R — the transition day into 24/7 trading, "Effective Friday, May 29, 2026".
- 2026-05-30 — T1 — CME SER-9740R — the permanent sourced intersection of SER-9740R and CME's client-systems wiki.
- 2026-08-01 — T1 — CME Globex notice 20260727 — one-day Saturday maintenance extension to 09:00 CT, table row "Event-based contracts | 329", with no replacement Pre-Open.
- 2026-08-02 — T1 — CME Globex notice 20260727 — revert to the standard 02:00-04:00 CT Saturday window.
- 2026-08-29 — T1 — CME Globex notice 20260824 — one-day Saturday maintenance extension to 06:00 CT.
- 2026-08-30 — T1 — CME Globex notice 20260824 — revert to the standard Saturday window.
- 2026-09-19 — T1 — CME Globex notice 20260824 — one-day Saturday maintenance extension to 08:00 CT; forward-dated on the operator's statement and restated by notice 20260831.
- 2026-09-20 — T1 — CME Globex notice 20260824 — revert to the standard Saturday window.

## Sources

Row review: 2026-09-06 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/notices/ser/2023/02/SER-9092.pdf> — CME SER-9092 — the listing day and the root's own "Sunday 5:00 p.m.- Friday 3:00 p.m." CME Globex Trading Hours cell.
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2023/2/23-014.pdf> — CME rule filing 23-014 — prints the same prior cell, separate from its Termination of Trading row.
- <https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf> — CME SER-9740R (28 May 2026) — "CME Globex: 24/7 with the exception of the following maintenance windows: Saturday 2:00 a.m. to 4:00 a.m. CT. Monday-Friday 4:00p.m. to 4:02 p.m. CT".
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/6/26-266.pdf> — CME rule filing 26-266 (4 June 2026) — reproduces SER-9740R verbatim as Exhibit A; the same statement, not a second one.
- <https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1394343937/Event-Based+Contracts+Expansion+to+24-7+Trading> — CME client-systems wiki — "Close: 3:00:00 p.m. to 4:01:00 p.m. CT / Pre-open: 4:01:00 p.m. to 4:01:30 p.m. CT / No cancel: 4:01:30 p.m. to 4:02:00 p.m. CT / Open: 4:02:00 p.m. CT".
- <https://cmegroupclientsite.atlassian.net/wiki/rest/api/content/1394343937/version> — Confluence version API for that page — the table entered at version 4, with no later version.
- <https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html> — CME Globex notice 20260727 — the 2026-08-01 Saturday extension.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html> — CME Globex notice 20260824 — the 2026-08-29 and 2026-09-19 Saturday extensions.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html> — CME Globex notice 20260831 — restates the September date.

## Gaps and residual risks

- **executable** — the 24/7 weekday close is the sourced intersection of two CME primaries that disagree by an hour, and the withheld 15:00-16:00 CT hour is one in which trades may print. Under SER-9740R's reading the key under-reports an hour a day; under the wiki's it is exact; it never over-reports. Closing condition: one CME statement of this root's weekday window after 2026-05-29. Dormant identity, so recorded here rather than opened as an issue.
- **ordering the two statements, each with a weakness** — Confluence's version API shows the wiki table entered at version 4 (absent from versions 1-3) under the page's own Revision History row "April 22, 2026 / Added Event-Based Contract Maintenance Windows and Market Hours schedule impacts", and shows no later version (the history has no `nextVersion`; version 5 answers 404), so the 15:00 is the earlier statement, never revised, still live. SER-9740R is later but its Table 2 was not written for this root: the SER says "No other changes have been made to the original SER" beyond adding it to Table 1, and its "Current" column misstates the root's own prior cell, which SER-9092 and rule filing 23-014 both print as "Sunday 5:00 p.m.- Friday 3:00 p.m.".
- **both channels are exhausted** — no SER, Globex notice, clearing advisory, product page, Daily Bulletin section, product-slate or trading-hours service entry, or the event-contract specification PDF (which has no "Trading Hours" field) restates this root's weekday window after the cutover, so waiting has no endpoint.
- **independently confirmed after the cutover** — CME Globex notices 20260727, 20260824 and 20260831 each list "Event-Based Contracts / 329" among the 24/7 markets whose Saturday window reverts "to its 2:00 a.m. - 4:00 a.m. CT standard schedule". The same tables list the cryptocurrency channels, and `globex_cryptocurrency` models the same three extensions (#61).
- **not evidence, recorded so it is not rediscovered** — CME's 8 July 2026 gold/crude deck prints the wiki's row template with "4:00:00 p.m." for a product migrating onto "329 (shared with events contracts)", which is a statement about gold; cmegroup.com/trading-hours.html's "Event-Based Contracts / 16:01 CT / 16:00 CT" row is byte-identical before and after the cutover and still advertises a Tuesday window CME cancelled on 2026-03-03 for the neighbouring Event Contracts II channel; and the coincidence of 15:00 CT with the root's former daily settlement instant can neither corroborate nor refute a close under LAW-SESSION-NOT-EXPIRY.
- **not `globex_cryptocurrency`** — that key has no sourced five-day Pre-Open and a 16:00 close; this root's Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 CT queues are stated on its listing day and its close under the intersection is 15:00.
- **trade-date roll** — the key-backed calendar joins the Saturday-04:00-to-Monday-15:00 pieces into one block and carries the following open business date, as both documents' trade-date-roll language states.

## Module narrative (moved from src/calendar/schedules/futures/us/bitcoin_event_contracts.rs on 2026-09-12 UTC)

CME Event Contracts on Bitcoin Futures in America/Chicago: the one Chapter 23
event-contract root CME moved to 24/7 trading on 2026-05-29, leaving the
other ten on the five-day grid `event_contracts.rs` models. This key carries
`ECBTC`'s whole life so a caller never switches keys on a date: sessionless
before its 2023-03-12 listing, the shared event-contract grid until
2026-05-28, and the 24/7 grid from 2026-05-29 on.

WHY A SECOND KEY. CME SER-9740R (28 May 2026) expands "the trading hours on
the CME Globex electronic trading platform ... for all cryptocurrency futures
and options on futures contracts noted in Table 1", and its Table 1 now reads
"Event Contracts on Bitcoin Futures | ECBTC | 23", "Effective Friday, May 29,
2026". CME's own client-systems wiki scopes the expansion to that one root —
"only the below event contracts on Bitcoin on channel 329 will migrate to
weekend trading. Other event contracts will continue on the current
schedule." — with a Product Scope table whose single row is "Event contracts
on Bitcoin Futures | ECBTC | VB | 329 | 74". A session change on one root and
not its siblings is a divergence, so the root gets its own timeline, exactly
as `mini_grains.rs` and `rough_rice.rs` split from the grain grid.
https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf
https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1394343937/Event-Based+Contracts+Expansion+to+24-7+Trading

THE CONFLICT, STATED EXACTLY. Two CME primaries describe the 24/7 grid and
agree on everything except where the weekday maintenance window begins.
SER-9740R's Table 2: "CME Globex: 24/7 with the exception of the following
maintenance windows: Saturday 2:00 a.m. to 4:00 a.m. CT. Monday-Friday
4:00p.m. to 4:02 p.m. CT" with "CME Globex Pre-open: Monday - Friday: 4:01
p.m. to 4:02 p.m. CT Saturday: 3:45 a.m. to 4:00 a.m. CT". The wiki's
"Event-Based Contract Maintenance Windows and Market Hours" table: "Monday
through Friday | Daily Maintenance Window (with Trade Date roll) | Close:
3:00:00 p.m. to 4:01:00 p.m. CT | Pre-open: 4:01:00 p.m. to 4:01:30 p.m. CT /
No cancel: 4:01:30 p.m. to 4:02:00 p.m. CT / Open: 4:02:00 p.m. CT" and
"Saturday | Extended Maintenance Window | Close: 2:00 a.m. to 3:45 a.m. CT |
Pre-open: 3:45:00 a.m. to 4:00:00 a.m. CT / ... Open: 4:00 a.m. CT". So the
weekday executable close is 16:00 CT in one document and 15:00 CT in the
other, and everything else — the 16:01 queue, the 16:02 open, the Saturday
window and its 03:45 queue — is identical.

THE TWO STATEMENTS ARE ORDERED, AND EACH HAS A WEAKNESS. Confluence's own
version history shows the wiki table entered at v4 — absent from v1-v3 (all
2026-03-25) — and the page's Revision History row says "April 22, 2026 |
Added Event-Based Contract Maintenance Windows and Market Hours schedule
impacts." There is no later revision: the version list ends at v4, the
history endpoint has no `nextVersion`, and version 5 answers HTTP 404. So the
15:00 figure is the earlier statement, written five weeks before the SER and
never revisited, and it is still the live page. SER-9740R is the later one,
but its Table 2 was not written for `ECBTC`: the SER says "No other changes
have been made to the original SER" beyond adding the root to Table 1, and
the "Current" column it prints for `ECBTC` — "Sunday 5:00 p.m. - Friday 4:00
p.m. CT with a daily maintenance period from 4:00 p.m. - 5:00 p.m. CT" — is
the cryptocurrency-complex cell, whereas the root's own listing cell in
SER-9092 and rule filing 23-014 read "Sunday 5:00 p.m.- Friday 3:00 p.m." So
the later document misdescribes this root's prior hours in the very table
that states its new ones. Rule filing 26-266 (4 June 2026, a Regulation
40.6(d) notification) reproduces SER-9740R verbatim as its Exhibit A and is
the same statement, not a second one.
https://cmegroupclientsite.atlassian.net/wiki/rest/api/content/1394343937/version
https://www.cmegroup.com/notices/ser/2023/02/SER-9092.pdf
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2023/2/23-014.pdf
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/6/26-266.pdf

BOTH CHANNELS ARE EXHAUSTED. Nothing CME has published since the cutover
restates this root's weekday close: not the wiki (frozen at v4), not any SER,
Globex notice, clearing advisory, product page, the Daily Bulletin, the
product-slate or trading-hours services (neither lists an event contract) or
the event-contract specification PDF (which contains no "Trading Hours"
field at all). Waiting for a restatement has no endpoint.

SO THE KEY SERVES THE SOURCED INTERSECTION. The window true under both
documents is open from 16:02 CT to 15:00 CT the next day, Monday through
Friday; the disputed 15:00-16:00 hour is served closed. Under SER-9740R's
reading that under-reports one executable hour a day; under the wiki's it is
exact; it never over-reports, which is the direction every Partial row in
this crate errs. `AGENTS.md`'s sourced-intersection convention is written
for an undated changeover between two states, and this is its second shape
— two documents describing one period — but the rationale is identical:
withholding the whole key over a one-hour dispute reports ~23 agreed hours a
day, six days a week, as unmodelled, and absence is a claim too.

WHAT IS AGREED IS ALSO INDEPENDENTLY CONFIRMED AFTER THE CUTOVER. CME's
Globex notices of 27 July, 24 August and 31 August 2026 each list
"Event-Based Contracts | 329" among the "24/7 markets" whose Saturday
maintenance window they temporarily extend, and each says the window will
"revert to its 2:00 a.m. – 4:00 a.m. CT standard schedule". Those notices
are also this key's three temporary rows, below.

NOT EVIDENCE, RECORDED SO IT IS NOT REDISCOVERED. (1) CME's "24/7 Trading
for 1-Ounce Gold and 10-Barrel Crude" deck (8 July 2026) prints the wiki's
exact row template with "Close: 4:00:00 p.m." for a product migrating onto
"329 (shared with events contracts)"; it is a statement about gold, not
about this root. (2) cmegroup.com/trading-hours.html's "Event-Based
Contracts | 16:01 CT | 16:00 CT" row is byte-identical in captures before
and after the cutover and still advertises a Tuesday maintenance window
that CME cancelled on 2026-03-03 for the neighbouring Event Contracts II
channel; it describes that channel, stale. (3) That 15:00 CT coincides with
this root's former daily settlement instant is a hypothesis about why the
documents diverge, and under LAW-SESSION-NOT-EXPIRY it can neither
corroborate nor refute a close. The 15:00 in SER-9092 is sourced here only
because it sits in that document's "CME Globex Trading Hours" row, in
session language, separate from its "Termination of Trading" row.

NOT `globex_cryptocurrency`, THOUGH THE 24/7 SHAPE IS THE SAME. That key
carries Bitcoin futures from 2017 with no sourced five-day Pre-Open and a
16:00 CT weekday close; this root ran the event-contract grid with stated
queues from 2023 and closes at 15:00 CT under the intersection. Its queues
stay `order_entry` here, as `event_contracts.rs` models them, rather than
folded into `extended` as the cryptocurrency module chose.

TIMEZONE. Every document above states this root's hours in Central time
("CT"); the wiki adds seconds. The Central grid is the one encoded.

---

Revision evidence — the day-level effective date and the primary source that
states it:
  2023-03-12 "CME SER-9092" — "Effective Sunday, March 12, 2023, for trade
    date Monday, March 13, 2023"; keyed to the venue-local opening day, as
    `event_contracts.rs` keys its own launch.
    https://www.cmegroup.com/notices/ser/2023/02/SER-9092.pdf
  2026-05-29 "CME SER-9740R" — "Effective Friday, May 29, 2026"; the
    transition day above, then the intersection from the Saturday.
    https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2026/05/ser-9740r.pdf
  2026-08-01 "CME Globex notice 20260727" — "Effective this Saturday, August
    1, CME Group will temporarily extend the Saturday maintenance window
    schedule for 24/7 markets to 2:00 – 9:00 a.m. Central Time (CT)", table
    row "Event-based contracts | 329"; "Following this one-day extension,
    the Saturday window will revert to its standard schedule."
    https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html
  2026-08-29 and 2026-09-19 "CME Globex notice 20260824" — "Saturday, August
    29: 2:00 a.m. – 6:00 a.m. Central Time (CT)" and "Saturday, September
    19: 2:00 a.m. – 8:00 a.m. CT", table row "Event-Based Contracts | 329";
    "Following each extension, the Saturday maintenance window will revert
    to its 2:00 a.m. – 4:00 a.m. CT standard schedule." The 31 August notice
    restates the September date. The September row is forward-dated on the
    operator's statement.
    https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html
    https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html
Evidence: docs/evidence/globex_event_contracts_btc.md
