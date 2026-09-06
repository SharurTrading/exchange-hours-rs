<!-- SPDX-License-Identifier: MIT-0 -->

# Ambiguous futures families

`MarketHoursKey` identifies an exact product-family schedule, not an
approximation selected from the same venue. Every family deferred from an
earlier draft now ships with a primary-sourced profile. One prospective name
remains rejected, on evidence rather than on effort:

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

The crate performs no symbol-to-family mapping; refusing an unsupported product
belongs in the caller's instrument catalog.
