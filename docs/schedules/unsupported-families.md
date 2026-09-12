<!-- SPDX-License-Identifier: MIT-0 -->

# Ambiguous futures families

`MarketHoursKey` identifies an exact product-family schedule, not an
approximation selected from the same venue. This file is the register of names
that deliberately do not resolve. It has three parts: one **ambiguous** name,
whose products do not share a grid; the **rejections on evidence**, where a
family exists but no primary source states its hours; and the **blocked** keys,
where the family and its hours are known and an unconditional effective day is
not. A name that is in none of the three, and is not a shipped key, has been
dropped rather than decided — which is what
`handoff_keys_are_registered_or_rejected` in `tests/schedule_documentation/`
exists to prevent.

Every family deferred from an earlier draft now ships with a primary-sourced
profile. One prospective name remains **ambiguous**, on evidence rather than on
effort:

| Prospective identifier | Why it does not resolve |
|---|---|
| `sgx_equity_index` | SGX equity-index products do not share a session grid |

SGX runs five materially different grids, all `Asia/Singapore`:

| Key | T session |
|---|---|
| `sgx_equity_index_japan` | 07:30–14:55 |
| `sgx_equity_index_china` | 09:00–16:30 |
| `sgx_equity_index_singapore` | 08:30–17:20 |
| `sgx_equity_index_taiwan` | 08:45–13:45 |
| `sgx_equity_index_ntr_usd` | 07:25–18:30 |

A single `sgx_equity_index` key would have to pick one of these and answer with
it for all five. A Taiwan contract would report Singapore's 17:25 close — a
wrong answer delivered with full confidence, which is the exact failure this
crate exists to prevent. Select the specific grid instead.

Nifty is also absent. It is no longer an SGX-listed product: SGX's own
[derivatives trading-hours page, captured 11 July 2018](https://web.archive.org/web/20180711020353id_/http://www.sgx.com/wps/wcm/connect/mp_en/site/trading_on_sgx/derivatives_market/derivatives_trading_hours_and_calendar/Trading+Hours?%20noCache=1531274630984.837727.133108399), lists
`S&P CNX Nifty Index Futures`, while its 2025 and 2026 calendars
list only NSE IFSC contracts under `GIN`/`GINB`/`GINF`/`GINI`. SGX's own 2026
calendar and its GIFT Connect product page state different T+1 start times
(18:35 versus 19:05), so no profile is modelled from contradicting primary
sources.

## `ECBTC` after 2026-05-29 — resolved on 2026-09-06

This file listed `globex_event_contracts_btc` as rejected from 2026-09-05 to
2026-09-06 because two CME primary sources — SER-9740R (28 May 2026) and the
client-systems wiki page
[Event-Based Contracts Expansion to 24-7 Trading](https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/1394343937/Event-Based+Contracts+Expansion+to+24-7+Trading)
(revised 22 April 2026) — disagree by an hour on the root's new weekday
maintenance start, 16:00 versus 15:00 CT, while agreeing on everything else.

The name now resolves. `MarketHoursKey::GlobexEventContractsBtc` serves the
**sourced intersection** of the two statements — open 16:02→15:00 CT
Monday–Friday, Saturday 02:00–04:00 closed, Sunday open — and withholds the
disputed hour, on the `AGENTS.md` convention that a source conflict is served
the way an undated changeover is: what both documents support, and no more.
What changed the call was establishing, from Confluence's own version API,
that the wiki's figure is the *earlier* statement (its table entered at
version 4 under the page's "April 22, 2026" revision row, and no later version
exists), that SER-9740R's Table 2 is the cryptocurrency cell carried over
unchanged and misstates this root's own prior hours, and that no CME channel
has restated the window since — so there is no restatement to wait for.
The ledger row and `bitcoin_event_contracts.rs` carry the full record.

Two near-identically titled wiki pages give different windows; the figure
above is from the page named in this section, not from
[Swap-Based Event Contracts and 24-7 Trading](https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/988020743/Swap-Based+Event+Contracts+and+24-7+Trading),
which covers different products and reads `4:00 to 4:01 p.m. CT`. Reading the
wrong page makes the conflict appear not to exist; that happened once during
review and was corrected.

## CME trade-type rejections, recorded on evidence — 2026-09-12

Three CME trade-type groups surveyed for issue #58 are **rejected**, not
deferred. Each has a launch, a root list and a measurable feed boundary; none
has a CME document that states its hours in session language, and under
LAW-SESSION-NOT-EXPIRY a settlement, marker or fixing instant is never promoted
to a session close to fill the gap. Each entry names the document that would
close it.

### Treasury TAS — `TNT`, `UBT`, `ZBT`, `ZFT`, `ZNS`, `ZTT`

Both CME documents that date the launch — SER-8863 and CME Globex Notice
2021-10-18, each naming trade date 2021-11-15 — state that day
**conditionally**, so whether either anchors an effective day is the referred
question [#71](https://github.com/SharurTrading/exchange-hours-rs/issues/71).
It does not matter for this verdict: **no CME document states the hours**
either way. The 14:00 CT close is feed-only and equals the end of CME's stated
Treasuries settlement range, `13:59:30-14:00:00 CT`; under
LAW-SESSION-NOT-EXPIRY that coincidence proves nothing about the session.

SER-8863 **was retrieved** (294,279 bytes) and states no hours anywhere — do
not re-fetch it. Two further defects in it are recorded so the next reader does
not re-derive them: its body is conditional
(*"and pending all relevant CFTC regulatory review periods"*), so its header
Effective Date is not an unconditional anchor on its own, and the Globex Notice
does not supply one either — its own sentence reads *"Effective Sunday,
November 14 (trade date Monday, November 15), pending completion of all
regulatory review periods"*; and its summary table prints `ZNT` for the Ultra
10-Year where its own Exhibit 1 and the Globex Product Reference Sheet print
`TNT`.

These roots are **not** served by `globex_interest_rates`. That key's own doc
says only *"Excludes options and separately specified interest-rate product
families"* — it does **not** name TAS — so the non-reuse argument rests on the
two-hour close difference between the outright grid and the observed TAS
boundary, not on a citation borrowed from `globex_energy`.

**What closes it:** a CFTC 40.2 or 40.6 certification for the CBOT October–
November 2021 Treasury TAS listing whose appendix carries a Trading Hours row.

### Dutch TTF TAS — `TAS`, `TTS`

Absent from MRAN RA1005-4 and RA1006-4 and from every retrieved CME hours
statement. Beyond the missing hours, the **zone anchor is undecided**: one
summer week cannot separate a 10:05 CT close from 16:05 London or 17:05 CET,
because all three coincide while both zones are on summer time.

CME demonstrably *does* express settlement ranges in London time when a product
is London-anchored — its settlement-time table prints
*"Aluminum \| 16:25:00-16:30:00 London"* — while listing no row for Dutch gas
at all, so the absence is not evidence of a Chicago anchor. **Never infer this
anchor from the energy TAM note**: that note anchors crude and gasoil markers,
not Dutch gas, and borrowing it would put a sourced-looking zone on an unsourced
boundary.

**What closes it:** any CME statement of the TTF TAS trading hours that names
its zone, or a second observation from a week in which Chicago, London and
Amsterdam do not share an offset.

### Commodity-index BTIC — `AWT`, `BAT`, `BET`, `BGT`, `BLT`, `BMT`, `BPT`, `BST`, `CCT`

Measured on their own Globex security groups — B7, 8G, 8R, 8S, 8U, 8T, 8Q, 8X
and FX — at `open@08:15` and `closed@13:30` CT, with **no CME prose** stating
that window. The trading-hours service for AWT on group B7 publishes, verbatim:

> AWT [B7] 2026-09-13 preopen@16:00->2026-09-14; 2026-09-14 open@08:15->2026-09-14,
> closed@13:30->2026-09-14, preopen@16:45->2026-09-15. Identical shape on
> BAT [8G], BPT [8Q], BET [8R], BGT [8S], BMT [8T], BLT [8U], BST [8X],
> CCT [FX] and on the outrights AW [AW] and CCI [FW].

The one CME document that tabulates a BTIC window for this family — SER-9138
and CME/CBOT Submission 23-074 — is demonstrably defective twice over. Its AW
row reads *"Current CME Globex BTIC Trading Hours 5:00 pm – 1:30pm CT"* against
a feed that opens these roots at 08:15 CT; and the value its "Current" column
prints, *"5:00 pm - 4:00 pm CT"*, is exactly the **ClearPort** BTIC window
printed one line below the Globex one on the same specification pages.

They must **not** ride `globex_bloomberg_commodity_index`. Merging them would
import AW's history and report BTIC BCOM closed from 17:00 to 08:15 CT — a
**15h15m** executable-hours gap asserted with full confidence, the exact class
of wrong answer this register exists to prevent.

One further measured fact, so the next reader does not re-derive it: **no
`17:00` boundary of any kind exists on group B7.** The adversarial verifier
re-parsed the saved capture and found *"r2-svc-8539-AWT-sep.json contains no
17:00 event of any kind. The queue starts at 16:00 (Sunday preopen) and 16:45
(weekday preopen) and runs overnight to the 08:15 open."* So 23-074's "5:00 pm"
is not even a queue start on these groups.

**What closes it:** a CME document stating the BTIC trading hours for these nine
roots in session language, on their own groups rather than the outrights'.

## Blocked, not rejected

These six `MarketHoursKey` names are **not** rejections. Each family exists, its
grid is partly or wholly sourced, and what is missing is an unconditional,
day-level effective date or a representation decision — so LAW-NO-FABRICATED-
DATES, not the evidence, is what keeps them out. Each has an open issue, and
each issue states the single artifact or decision that closes it.

| Prospective identifier | What blocks it | Issue |
|---|---|---|
| `globex_cryptocurrency_btic_new_york` | The current 24/7 grid's cutover day is undated: CME 26-114 and SER-9740R scope "all cryptocurrency futures and options on futures contracts" and neither contains the string "BTIC". Era 1 alone would serve today's market wrongly; era 2 would invent a day | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_cryptocurrency_btic_london` | The same undated cutover, plus a 24/7 weekend block that spans the DST transition, which the adopted two-profile selector resolves on the **opening** day and so answers by up to an hour of false-closed executable window, plus an unrecorded CME FAQ-versus-feed conflict over a 30-minute against a 5-minute daily stop | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_cryptocurrency_btic_apac` | As London, measured on ABB across the US fall-back as well. Note SOL, XRP, ADA, LINK, AVAX, SUI and Stellar Lumens have no APAC variant at all | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_europe_index_btic` | No research gate ever covered it: neither the DVT/E3T root codes, the key name, nor productIds 8005/7955/7956 appear in the non-equity BTIC result. No launch certification retrieved; E3G's route change 68/EU → 68/EQ unresolved; and the retrieved evidence does not fix the close's zone, which one channel decoded as CEST and another as Europe/London | [#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) |
| `globex_ftse_china_50_btic` | Same gate gap. The zone is re-derived first-hand as 16:00 `Asia/Hong_Kong` in both DST states, but no launch day was retrieved, and FTC's own product page's flat "5:00 p.m. - 3:00 a.m. CT" is wrong for a boundary that moves with Hong Kong | [#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) |
| `globex_equity_index_btic_plus_taco_plus` | Two CME documents give two different conditional launch days — Globex Notice 2019-08-26's "Effective Sunday, September 8 (trade date Monday, September 9)" against the BTIC+/TACO+ FAQ's "October 7". That is a conflict, not a condition, so no discharge argument rescues it, and there is no window to intersect when the disputed thing is a launch day | [#75](https://github.com/SharurTrading/exchange-hours-rs/issues/75) |

A blocked name behaves exactly like any other unregistered name: it does not
parse and it is not substituted. The distinction is for the reader deciding
whether to spend a retrieval.

The crate performs no symbol-to-family mapping; refusing an unsupported product
belongs in the caller's instrument catalog.
