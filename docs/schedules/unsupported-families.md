<!-- SPDX-License-Identifier: MIT-0 -->

# Ambiguous futures families

`MarketHoursKey` identifies an exact product-family schedule, not an
approximation selected from the same venue. Every family deferred from an
earlier draft now ships with a primary-sourced profile. Two prospective names
remain rejected, and both are rejected on evidence rather than on effort:

| Prospective identifier | Why it does not resolve |
|---|---|
| `sgx_equity_index` | SGX equity-index products do not share a session grid |
| `globex_event_contracts_btc` | CME's own sources disagree on the post-2026-05-29 daily close |

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

## `ECBTC` after 2026-05-29

Event Contracts on Bitcoin Futures rode `globex_event_contracts` from their
2023-03-12 listing to 2026-05-28. CME then moved that one root — and no other
event contract — to 24/7 trading, so it genuinely leaves that key's grid on
2026-05-29. Two CME primary sources describe the replacement grid and they
agree everywhere except the boundary that matters most:

| Source | Saturday maintenance | Weekday maintenance | Reopen |
|---|---|---|---|
| SER-9740R, 28 May 2026 | 02:00–04:00 CT | **16:00**–16:02 CT | 16:02 CT |
| CME client-systems wiki, revised 22 April 2026 | 02:00–04:00 CT | **15:00**–16:01 CT | 16:02 CT |

The disagreement is a full hour of executable time, five days a week, and the
disputed hour is exactly `ECBTC`'s former Termination of Trading instant — the
expiry-for-session confusion that `globex_event_contracts` exists to avoid.
Encoding either number would assert a resolution neither document supports, and
serving the intersection would ship a key whose entire daily close is disputed.
So no key models this root after 2026-05-29; it is caller catalog data until CME
republishes the cell.

The crate performs no symbol-to-family mapping; refusing an unsupported product
belongs in the caller's instrument catalog.
