<!-- SPDX-License-Identifier: MIT-0 -->

# Ambiguous futures families

`MarketHoursKey` identifies an exact product-family schedule, not an
approximation selected from the same venue. This file is the register of names
that deliberately do not resolve. It has three parts: one **ambiguous** name,
whose products do not share a grid; the **rejections on evidence**, where a
family exists but no operator document states its hours at the tier
LAW-PRIMARY-SOURCES requires; and the **trade-type variants**, which are out of
scope until a consumer maps one (LAW-SERVICE-TIERS). A name that is in none of
the three, and is not a shipped key, has been dropped rather than decided.

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

These three are now doubly out: LAW-SERVICE-TIERS does not model a trade-type
variant as its own key until a consumer maps one, so nothing here is a work
item. The evidence stands as the record, so that a consumer arriving later does
not re-derive it.

### Treasury TAS — `TNT`, `UBT`, `ZBT`, `ZFT`, `ZNS`, `ZTT`

Both CME documents that date the launch — SER-8863 and CME Globex Notice
2021-10-18, each naming trade date 2021-11-15 — state that day
**conditionally**. LAW-PRIMARY-SOURCES now answers the question that was
referred as [#71](https://github.com/SharurTrading/exchange-hours-rs/issues/71)
(closed 2026-09-12): a conditional clause on a day now past is discharged when
a later operator artifact witnesses the new state, and the discharge is
recorded beside the row. It does not matter for this verdict: **no CME document
states the hours** either way. The 14:00 CT close is feed-only and equals the
end of CME's stated Treasuries settlement range, `13:59:30-14:00:00 CT`; under
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

## Trade-type variants — out of scope until a consumer maps one

These six `MarketHoursKey` names are **not** rejections: each family exists and
its grid is partly or wholly sourced. They are also not work items. Under
LAW-SERVICE-TIERS a trade-type variant (TAS, TAM, BTIC, TACO, TMAC) is not
modelled as a key until a consumer maps one; until then the consumer maps the
variant to its underlying family with a disclosed variant flag, and a variant
carrying more structure than its underlying — a second window, a London break,
an extra stop — stays on that flagged fallback rather than earning a key. Issues
[#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73),
[#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) and
[#75](https://github.com/SharurTrading/exchange-hours-rs/issues/75) are closed
on that basis, and this table is now the record they pointed at.

What each name would still need, if a consumer ever maps it, is unchanged: a
named evidence gap — an undated cutover, a missing launch certification, an
unresolved source conflict, a zone anchor the retrieved source does not state,
or a weekend shape the adopted selector cannot yet express. The gaps differ per
key, so each row lists every artifact or decision its key would need. Research
already done is kept here and in the research store rather than discarded.

| Prospective identifier | What it would still need | What closes it | Issue (closed 2026-09-12) |
|---|---|---|---|
| `globex_cryptocurrency_btic_new_york` | The date question is settled (D-8, 2026-09-12 UTC): CME's client-systems wiki states "Starting at 4 p.m. Central Time on Friday, May 29, 2026" at page scope, above all seven schedule tables, with the three "24/7 Crypto BTIC" tables as siblings of the TAS table and the page's only carve-out naming twelve spot-quoted futures. What remains is citation, not retrieval: the statement is an instant, so the boundary is an exact-instant cutover; the page's day-one 30-minute maintenance row covers the futures and options books only; the page names no BTIC product code; and the wiki's 16:01–16:02 CT daily stop is absent from BTC's ContractSpecs | (1) the one-day bridge row at 2026-05-29 and the steady state from 2026-05-30, as `globex_cryptocurrency_tas` is written, with the day-one reopen withheld; (2) product membership cited per root from ContractSpecs and the trading-hours service files in the research store; (3) the 16:01–16:02 CT conflict served as the sourced intersection and recorded beside the table, as `globex_event_contracts_btc` does | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_cryptocurrency_btic_london` | Everything the New York row lists, plus two more: the 24/7 weekend block runs from Saturday about 04:00 CT to Monday and so spans the mismatched US and London DST transitions, and the adopted two-profile selector resolves on the **opening** day and would serve up to an hour of false-closed executable window in the transition weeks (U-8 verdict N1); and CME's BTIC-on-Cryptocurrency FAQ states a 30-minute daily stop at 16:00 London against the feed's 5-minute restart | (1) to (3) as New York; (4) a representation for the weekend block — a close-day anchor, a split of the block at the transition, or the key stays unmapped; (5) the 25 disputed minutes withheld and the FAQ-versus-feed conflict recorded | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_cryptocurrency_btic_apac` | As London, measured on ABB across the US fall-back as well; and the root list is narrower than the handoff's — only the Bitcoin and Ether complexes and Bitcoin Friday futures carry an APAC variant (SOL, XRP, ADA, LINK, AVAX, SUI and Stellar Lumens do not) | (1) to (5) as London, with the root list restricted to the variants CME publishes | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_europe_index_btic` | No research gate covered it: neither the DVT/E3T root codes, the key name, nor productIds 8005/7955/7956 appear in the non-equity BTIC result. No launch certification was retrieved; E3G's route change 68/EU → 68/EQ is unresolved; and the shape queue decoded the 15:30 UTC close as CEST although ContractSpecs 8005 states it as "4:30pm London time" | (1) the CFTC 40.2/40.6 certification for the DVT and E3G BTIC listings, stating the launch day and hours in session language; (2) a time-aligned schedule week resolving E3G's route change; (3) the zone anchor written as the source states it (Europe/London), with the CEST decoding recorded as the error it was | [#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) |
| `globex_ftse_china_50_btic` | Same gate gap. The zone is re-derived first-hand as 16:00 `Asia/Hong_Kong` in both DST states, but no launch day was retrieved, and FTC's own product page's flat "5:00 p.m. - 3:00 a.m. CT" contradicts a boundary that moves with Hong Kong | (1) the FT5 BTIC launch certification; (2) the FTC page's flat CT statement recorded as a source conflict beside the table, with the ContractSpecs 7955 Hong Kong statement controlling | [#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) |
| `globex_equity_index_btic_plus_taco_plus` | Two CME documents give two different conditional launch days — Globex Notice 2019-08-26's "Effective Sunday, September 8 (trade date Monday, September 9)" against the BTIC+/TACO+ FAQ's "October 7". That is a conflict, not a condition, so no discharge argument rescues it, and there is no window to intersect when the disputed thing is a launch day. Rule 524 is not a channel for it ("BTIC+ and TACO+ are not governed by Rule 524") | (1) the SER confirming which day took effect; (2) when written, an empty `regular` resting on the ProductSlate (ES1 8689, ES2 8690, EQ1 8691), the ContractSpecs venue label and the Daily Bulletin only, with Rule 524 named as the channel that does not reach it | [#75](https://github.com/SharurTrading/exchange-hours-rs/issues/75) |

An out-of-scope name behaves exactly like any other unregistered name: it does
not parse and it is not substituted. The distinction is for the reader deciding
whether to spend a retrieval — and, now, for the consumer deciding whether to
ask for the key at all.

The crate performs no symbol-to-family mapping; refusing an unsupported product
belongs in the caller's instrument catalog.
