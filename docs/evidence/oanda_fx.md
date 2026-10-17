<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_fx` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's FX book, all FX products except TRY pairs: Sunday 17:05 through Friday
16:59 New York time, with a six-minute 16:59–17:05 daily break and no other
maintenance. The book trades straight through the daily maintenance halts of
every futures clock an FX hedge might route to (CME FX futures halt 16:00–17:00
CT daily), which is why this is a book key rather than a futures mapping — the
maintainer's 2026-10-10 decision recorded in SharurPlatform#1046.

## Revision rows

None. No `revisions!` block exists: the module's dated era machinery is
the selector rather than a revision list. Its positively dated bound — the
2024-02-11 opening day of the first current-grid session — is encoded as
the selector's boundary constant with its `MARKET-DATA` citation in the
module comment; the section below carries the tier and document id
(LAW-EVIDENCE-FILES). Historical completion remains tracked in #332 under
the dated current OANDA scope exception.

## The normal week

The hours-of-operation page (UK entity, OANDA Europe Limited, FCA 542574)
prints one row for the whole group:

> `All FX products (except TRY Pairs) | New York | 100,000 | Sun-Fri: 17:05 - 16:59*`

with the footnote:

> `*Forex instruments open on Sunday at 17:05 and close for the week on Friday 16:59 (New York time). There will be a six minute break between 16:59 - 17:05.`

and the note:

> `Note: There is an additional 6-minute break for NZD pairs starting at 06:59 Auckland time.`

Interpretation, each step from the operator's own words:

- The row prints **no `***` daylight-saving marker**, so both endpoints track
  New York local time: the anchor zone is `America/New_York`, DST-observing,
  and the zone carries the transitions — no seasonal selector and no
  fixed-offset zone is owed (the maintainer's zone-anchoring rule of
  2026-10-10). The single `*` is a footnote pointer, not a DST marker.
- The encoding is Sunday–Thursday 17:05 wrapping to the next local day 16:59
  (end-exclusive), Friday with no rule, `has_daily_close`/`has_weekend_close`
  both true. The six-minute gap between the close and the next 17:05 open is
  the stated break. Consecutive sessions have different trade dates; the
  crate classifies the six-minute inter-trade-date gap as `Maintenance`
  under its four-hour/ISO-week policy. That classification does not add
  an operator-published maintenance halt.
- The NZD-pair break (06:59–07:05 `Pacific/Auckland`) is stated **per
  instrument** (`XAU/NZD`, `XAG/NZD` and NZD FX pairs), not for the book: it
  is caller catalog data, and no rule in the grid states it.
- TRY pairs are carved out of this row by name and carry their own key,
  `oanda_try_fx`.

## Holidays

**Coverage:** 2024-02-21..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The current page prints the FX group on five dated sections — three reopen
statements and two no-reopen statements:

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-12-25 | closed | `FX open late 17:05` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas trade date closed; following session reopens at its normal 17:05 |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `FX open late 17:05` | `OANDA-UK-HOLIDAYS` | T1 | the holiday trade date's session does not trade; the reopen instant is the next session's normal open, so the trade date itself is a full closure |
| 2025-12-25 | closed | `Forex: open at 17:05` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day, the same derivation |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Forex: open at 17:05` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day, the same derivation; the page's 2025-12-31 section states `Forex: close at 16:59`, the no-reopen at the normal daily close instant that needs no row of its own |

The raw quotations, in the page's own layout:

> `Wednesday 1st January 2025` — `FX open late 17:05`
> `Wednesday, 24 December 2025 - Christmas Eve` — `Forex: close at 16:59`
> `Thursday, 25 December 2025 - Christmas Day` — `Forex: open at 17:05`
> `Wednesday, 31 December 2025 - New Year’s Eve` — `Forex: close at 16:59`
> `Thursday, 1 January 2026 - New Year’s Day` — `Forex: open at 17:05`

Interpretation: "open at 17:05" on the holiday names the reopen instant,
which is the **next** session's normal open — the holiday trade date's own
session (normally the prior evening 17:05 to that day's 16:59) does not
trade, so each of the three open dates ships a `Closed` row. The two
close-at-16:59 statements name the normal daily close — the no-reopen — and
need no row of their own. Every other date in the 2025–2026 portion carries no FX row:
the five stated rows establish the page's FX voice, and its silence
elsewhere — US holidays, Thanksgiving and all APAC/European holidays
included — is the operator's no-change on a page titled "the changes in
trading hours for upcoming holidays".

## Dated selectors

The module serves the witnessed normal-week eras; `profile_at` keys each
opening day to its era. `sourcing.rs` declares the normal week witnessed from
**2021-12-08**, the operator's own archived page print of the earlier grid.

The spans, and what each rests on:

- **Opening days 2021-12-08 through 2024-02-08 — the earlier grid.** The
  2021-12-08 capture prints `Sun-Fri: 17:03 - 16:58*` with the five-minute
  `16:58 - 17:03` break (`OANDA-UK-HOURS-2021-DEC-WAYBACK`), and the
  2023-06-05 capture — taken 05:35 UTC, inside the session opened Sunday
  2023-06-04 — still prints those endpoints
  (`OANDA-UK-HOURS-2023-JUN-WAYBACK`). The audit's intervening replays
  (2022-05-18, 2022-07-24, 2022-10-02, 2022-11-27, 2023-02-01, 2023-03-16,
  2023-04-25) all print the identical grid.
  The page prints witness the grid through 2023-06-05, and the operator's own
  machine channel witnesses it positively per day to the 2024-02-08
  opening day: every classified opening day in 2023-06-06..2024-02-08
  carries a tick in the 17:03 New York minute, which the current grid's
  16:59–17:05 break forbids (the section below). The span serves
  `Sun-Fri: 17:03 - 16:58`.
- **Opening days from 2024-02-11 — the current grid.** The changeover is
  positively dated by the operator's machine channel: the session opened
  Sunday 2024-02-11 closes on Monday at `16:59:00`, and its close minute
  contains S5 buckets at `16:58:20`, `:25` and `:30` New York — inside the
  earlier published grid's break — so the
  current `17:05 - 16:59` grid governs from that opening day (the section
  below). The intervening Friday and Saturday open no session, so no
  unwitnessed opening day remains between the eras. The 2024-02-21 page
  capture — the first current-grid print, taken 00:55 UTC inside the
  session opened 2024-02-20 — corroborates
  (`OANDA-UK-HOURS-2024-WAYBACK`), as do the 2024-05-07, 2024-06-20 and
  2024-09-19 replays and the 2026-10-10 capture.
- **Holiday layer, 2024-02-21 through 2024-12-31**: the 2025-03-26 holiday
  replay supplies the 2024 arrangements. New Year's Eve is explicitly normal
  FX trading through 16:59 and Christmas Day is closed until the 17:05
  reopen.
- **Below the 2021-12-08 witness**: carried. The 2021-03-03 capture prints
  no Forex table, and earlier complete FX evidence remains absent (#332).

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page for the UK entity, captured 2026-10-10 22:36 UTC (HTML) and
  rendered by the maintainer to PDF the same UTC date. The PDF's
  `11/10/2026, 09:22` header is the maintainer's local clock (UTC+11),
  i.e. 2026-10-10 22:22 UTC.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-STATIC-HOURS-252` | undated earlier grids; no effective day asserted | <https://www.oanda.com/assets/documents/252/Hours_of_Operation.pdf> | retrieved 2026-10-11 UTC | T1 | `778ea39e0f71005015a111963061cd86f8d7fce7b589af0279316992239244ba` |
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOURS-HTML` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | captured 2026-10-10 22:36 | T1 | `60f6143dd5d678b1a34a9937aa938d2884380a33bf3542a4c72f8f03fa3607b7` |
| `OANDA-UK-HOLIDAYS` | 2025-01-01 .. 2026-10-30 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours> | captured 2026-10-10 22:44 | T1 | `c7137ce3da4cc19d0431955feedc10fb6b9731214ad3cfc7966160e72f8559da` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |
| `OANDA-UK-HOURS-2021-WAYBACK` | 2021-03-03 | <https://web.archive.org/web/20210303210448id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20210303210448`, retrieved 2026-10-10 UTC; prints no Forex table | T1 | `ec5987239f2c79dd29fcfc40abe5a0d58c6e4ed446fa7dea653f1791b186cbed` |
| `OANDA-UK-HOURS-2021-DEC-WAYBACK` | 2021-12-08 observation; not a cutover | <https://web.archive.org/web/20211208123011id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2021-12-08; retrieved 2026-10-11 UTC | T1 | `63e6d54029b72e0ef1ec32f92b8c3412bd24edf05df9fda9fa2b4dce98cb78fb` |
| `OANDA-UK-HOURS-2023-JUN-WAYBACK` | 2023-06-05 observation; not a cutover | <https://web.archive.org/web/20230605093523id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2023-06-05; retrieved 2026-10-11 UTC | T1 | `98ddc07cf38727b2a861fd1f48c503b1b597054cbec9d1dde4b3dde1801ffb56` |
| `OANDA-STATIC-HOURS-402` | undated older grid; no effective day | <https://www.oanda.com/assets/documents/402/Hours_of_Operation_bT4prdz.pdf> | retrieved 2026-10-11 UTC | T1 | `2c36c16feefea3687390f972ac7b73e5778161900511b65ec570e0ee8b91e33c` |
| `OANDA-OEL-EXECUTION-2023` | 2023-10-23 execution policy; no schedule boundary | <https://docs.oanda.com/cd3a6fc6-6583-4a44-ad53-c866f886ad2a/OEL_Order_Execution_Policy_23.10.2023.pdf> | retrieved 2026-10-11 UTC | T1 | `2291d3cc6ea6ad38aa03cd76233e1d295002e43863c4aabad580d7f7ca210d60` |
| `OANDA-WEEKEND-2012-ORIGINAL` | 2012-07-06 announced change; preserved initial notice | <https://web.archive.org/web/20121019131147id_/http://www.oanda.com/corp/news/pr/oanda070412> | capture 2012-10-19; retrieved 2026-10-11 UTC | T1 | `1e692187c57d512ee1b68f10d32ab3185e121093a8efdd14f9bc25a2919e9b12` |
| `OANDA-UK-DEMO-2011` | 2011-06-21 observation; demo account scope | <https://web.archive.org/web/20110621021538id_/http://fxtrade.oanda.co.uk:80/trade-forex/demo/> | capture 2011-06-21; retrieved 2026-10-11 UTC | T1 | `b5a128e01d6e10fc9daf9bb9c32a6f7125d79e2fd5ff6f84b72d659af331f16f` |
| `OANDA-UK-DEMO-2012-JUN` | 2012-06-19 observation; demo account scope | <https://web.archive.org/web/20120619063323id_/http://fxtrade.oanda.co.uk:80/trade-forex/demo/> | capture 2012-06-19; retrieved 2026-10-11 UTC | T1 | `35c6a0ce5f811ce2e58460e316cf33a244c79d27177b2700e1ebcbd9b3b43d2e` |
| `OANDA-UK-DEMO-2012-AUG` | 2012-08-05 observation; demo account scope | <https://web.archive.org/web/20120805035159id_/http://fxtrade.oanda.co.uk:80/trade-forex/demo/> | capture 2012-08-05; retrieved 2026-10-11 UTC | T1 | `0799b916d8a31dd139bf14798341a3c61c8ff91815fb63672bea7dc3a6e13f5e` |
| `OANDA-UK-LIVE-2011` | 2011-06-21 observation; live fxTrade scope | <https://web.archive.org/web/20110621021554id_/http://fxtrade.oanda.co.uk:80/trade-forex/fxtrade/> | capture 2011-06-21; retrieved 2026-10-11 UTC | T1 | `29642bc231b31f744dd9a9e371cecc16ab96a71db06da6656ec1486865d6b211` |
| `OANDA-UK-LIVE-2012-JUN` | 2012-06-19 observation; live fxTrade scope | <https://web.archive.org/web/20120619063328id_/http://fxtrade.oanda.co.uk:80/trade-forex/fxtrade/> | capture 2012-06-19; retrieved 2026-10-11 UTC | T1 | `b4ae2f92bd290bc0f5c9c9ce343e2365032b89c517ea3bf5fc2b2f754496b4b6` |
| `OANDA-UK-LIVE-2012-AUG` | 2012-08-05 observation; live fxTrade scope | <https://web.archive.org/web/20120805032236id_/http://fxtrade.oanda.co.uk:80/trade-forex/fxtrade/> | capture 2012-08-05; retrieved 2026-10-11 UTC | T1 | `3cf4867c0e012c0758de4ac9c013499540110906c46be02a6e2126a9b72ba775` |
| `OANDA-WEEKEND-2012-MIRROR` | 2012-07-04 article; calibrated release reproduction | <https://www.forexcrunch.com/blog/2012/07/04/oanda-closes-weekend-trading/> | retrieved 2026-10-11 UTC | T3 | `e66a5c3106fbdcfde1786548458b0dd53eb2b25c8e46ec335bb6372b3805d736` |
| `OANDA-UK-CARTER-2025-WAYBACK` | 2025-01-09 | <https://web.archive.org/web/20250326184653id_/https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | capture 2025-03-26; retrieved 2026-10-11 UTC | T1 | `467313685d4d64a4b95e5112a58d0fa04583f0abe988b1478d41fddf8b04cf1c` |
| `OANDA-UK-HELP-FAQ-2024-JUN-WAYBACK` | undated FAQ; no schedule statement | <https://web.archive.org/web/20240619014415id_/https://help.oanda.com/uk/en/faqs/hours-of-operation.htm> | capture 2024-06-19; retrieved 2026-10-12 UTC | T1 | `1bd08fc0d32ec55698e69d6e37bcf805c633b29b261724c58ac16e40383e1fbe` |
| `OANDA-UK-HELP-FAQ-2024-NOV-WAYBACK` | undated FAQ; no schedule statement | <https://web.archive.org/web/20241107102830id_/https://help.oanda.com/uk/en/faqs/hours-of-operation.htm> | capture 2024-11-07; retrieved 2026-10-12 UTC | T1 | `b01b0bd93258460c79c761beb14fca664e3cab5723efcc176b0b4a5f514c3d47` |
| `OANDA-UK-HELP-FAQ` | live FAQ; no schedule statement | <https://help.oanda.com/uk/en/faqs/hours-of-operation.htm> | retrieved 2026-10-12 UTC | T1 | `3a42f46d4a4c3f6f165c3815fe122cc0e47faed305b293c93a8c0eca9a985332` |
| `OANDA-PRESS-2010-FX` | 2010 operator press release; corporate platform scope | <https://www.prnewswire.com/news-releases/oanda-deems-cftc-leverage-proposal-unfair-and-non-competitive-88598687.html> | retrieved 2026-10-11 UTC | T1 | `3e8a805537a3ddf2a54f7b47d326c04b562f1f1223a6ea1da6edcd652e9ef695` |
| `OANDA-PRESS-2011-FX` | 2011-07-16 operator press release; corporate platform scope | <https://www.prnewswire.com/news-releases/oanda-listens-to-growing-client-base-expands-cross-pair-coverage-125675218.html> | retrieved 2026-10-11 UTC | T1 | `080ea8716db2e93948b0d6a1ed6e591c600b405aa4a2ab3d207cacfed9f74247` |
| `OANDA-EUROPE-2011-MIRROR` | 2011-06-22 launch release reproduction; uncalibrated original | <https://www.financemagnates.com/forex/brokers/oanda-officially-announces-the-establishment-of-european-headquarters-in-london/> | retrieved 2026-10-11 UTC | T3 | `cce0a5398e17377ce24f98a0d6c63718bf35f42bef332ad716784f0ff5420ec2` |

Store paths (research store, wave-1 capture):
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation.pdf`,
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation.html`,
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation@2024-02-21-wayback.html`,
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation@2022-05-18-wayback.html`,
`normal-weeks/oanda-uk-hours-2026-10-10/oanda-uk-hours-of-operation@2021-03-03-wayback.html`,
`holidays/raw/oanda-uk-holiday-hours-2026-10-10/oanda-uk-holiday-trading-hours.html`.

## Gaps and residual risks

- **RESOLVED 2026-10-17 (UTC): the changeover is positively dated to the
  2024-02-11 opening day.** The residual recorded here — the disputed
  2023-06-05..2024-02-20 span served as the sourced intersection — is
  closed: the operator's own machine channel witnesses the earlier grid
  positively on classified opening days through 2024-02-08, and the Sunday
  2024-02-11-opened session's Monday close minute quotes inside the
  earlier grid's break, positively dating the current era from 2024-02-11
  (section below, `MARKET-DATA`). Two earlier claims were retracted in
  review and the retractions stand recorded: the 2024-02-11
  lead-bucket-only boundary of the first pass, and a 16:58:40 New York
  close-minute tick that no captured response contains. No unwitnessed
  opening day remains between the eras.
- **The pre-2021-12-08 normal week is carried, not sourced.** The
  2021-03-03 capture prints no Forex table, so the 2021-12-08 capture is the
  book's first complete witness; earlier complete FX evidence remains absent
  (see the 2026-10-11 audit below).
- **The holiday window begins at the current week's knowledge boundary.**
  The archived holiday page supplies 2024 arrangements; dates before
  2024-02-21 still require normal-week evidence, and 2026-10-31 onward is
  unpublished. Earlier broker holiday coverage remains tracked under #332.
- The page's overall note — "these times are subject to change during
  daylight saving time and certain public holidays. There may be occasions
  within market hours when trading may be halted" — states in-market halts
  exist operationally; those are not scheduled hours and are not modelled.
- Intraday-quote holidays for specific FX pairs beyond the page's rows (for
  example reduced liquidity on US holidays) are not closures and ship no
  row; the page states none.

## Changeover sweep closed negative — 2026-10-12 UTC

The 2026-10-11 historical audit recorded that no UK-scoped unconditional day
for the minute-endpoint change was found, while noting it had not claimed
every operator channel closed negative. This sweep closes the enumeration:

- **The help-centre FAQ** (`help.oanda.com/uk/en/faqs/hours-of-operation.htm`,
  "Hours of operation in OANDA Europe Limited") was the one indexed channel
  whose captures the audit had not retrieved. Both Wayback replays
  (2024-06-19 and 2024-11-07) and the live page, retrieved 2026-10-12, print
  no session grid at all — only the boilerplate that trading hours "are
  subject to change during daylight saving time and certain public
  holidays". Two captures with differing digests are layout drift, not a
  schedule statement (`OANDA-UK-HELP-FAQ-2024-JUN-WAYBACK`,
  `OANDA-UK-HELP-FAQ-2024-NOV-WAYBACK`, `OANDA-UK-HELP-FAQ`).
- **The hours page's Wayback index is complete inside the bracket**: the
  uncollapsed CDX query of 2026-10-12 returns zero captures between
  2023-06-06 and 2024-02-20, and the audit's eight twice-refused replays all
  postdate the 2024-02-21 current-grid witness, so none can narrow the
  changeover.
- The 2026-10-11 audit's other channels — the static hours guides 252 and
  402 (both undated), the OEL execution policy (no schedule boundary), the
  corporate news and press releases (the 2012 weekend-close era), and the
  demo/live fxTrade pages (marketing statements, removed 2012-08) — closed
  negative there and are unchanged by this sweep.

No operator channel enumerated by either pass names a date for the
minute-endpoint change; the hunts stood closed negative, and the undated
span served the sourced intersection — until the machine-channel witnessing
of 2026-10-16 narrowed it (corrected 2026-10-17 in review, below). Contact
was not attempted, per the maintainer's standing instruction recorded in
issue #332.

## Machine-channel witnessing and the dated boundary — 2026-10-16 UTC, corrected 2026-10-17 in review

The 2026-10-12 sweep above closed every operator publication channel
negative, and the undated span served the sourced intersection. On
2026-10-16 (UTC) the maintainer directed dating the changeover from the
operator's own machine channel instead — the practice-environment candle
service (`api-fxpractice.oanda.com/v3/accounts/<id>/instruments/EUR_USD/candles`, read
through the `oanda-rs` client), the same T2 class the MLK-2023 closure used
under the MARKET-DATA convention. The feed is read as bytes, paced through the
client's own admission control, and the confirming raw responses are retained
privately; the `MARKET-DATA` id below deliberately does not resolve in the
research store.

Bar stamps are bucket-open times: an M1 candle stamped `T` represents
`[T, T+1min)`, an S5 candle `[T, T+5s)`. The operator's
[API definition](https://developer.oanda.com/rest-live-v20/instrument-df/#Candlestick)
explicitly defines the stamp as the start time (retrieved 2026-10-17 UTC;
retained privately as `raw/validation-2026-10-17/instrument-df.html`). An
independent cross-granularity check reproduced all 65 S10/S15/S30/M1
midpoint OHLC and summed price-count aggregates from start-aligned S5
windows; none of the 61 comparable end-aligned aggregates matched. The
first end-aligned bucket of each granularity lacks preceding S5 input and
is excluded. Live forming-candle polling was not attempted during the
weekend closure.

Quoting and executable sessions are different observations. Ten sampled
current FX reopens first stamp at `17:04:55` New York, five seconds before
the published `17:05` open. Neither such a lead bucket nor absence of an
earlier quote identifies a state. Exact-open stamps alone also cannot
establish timestamp semantics if pre-open quoting is allowed. No universal
five-second quote envelope is asserted: the Singapore night census records
an S5 start stamp ten seconds beyond that book's candidate close. Endpoint
instants remain the T1 page values; the machine-channel comparison selects
between the published candidate grids and does not infer execution times
from midpoint updates.

The FX candidates are offset: earlier `17:03 - 16:58`, current
`17:05 - 16:59`. The retained positive observations are:

- **Earlier-grid observations:** the per-day M1 sweep of
  2023-06-06..2024-02-19 contains 176 opening days through 2024-02-08
  with a `17:03` New York minute bucket, inside the current candidate's
  break. The holiday Sundays 2023-12-24 and 2023-12-31 are quiet and do
  not supply positive witnesses. The February 8 S5 reopen includes
  `17:03:00` and `17:03:30`; the February 9 closing leg's final bucket
  starts at `16:58:00`.
- **Current-grid observations:** the Monday 2024-02-12 closing leg
  includes S5 starts at `16:58:15`, `:20`, `:25` and `:30`, and at
  `16:59:00`. Later retained close-minute starts are listed below. These
  observations support the current candidate in the two-grid comparison;
  they do not assert continuous quotes in every intervening bucket.

| Closing date, New York | Retained S5 starts after 16:58:00 and before 16:59:00 | 16:59:00 bucket |
|---|---|---|
| 2024-02-12 | :15, :20, :25, :30 | present |
| 2024-02-13 | :50 | present |
| 2024-02-14 | :05, :10, :25, :30, :35, :40, :45, :50, :55 | present |
| 2024-02-15 | :05, :10, :15, :20, :30, :35 | present |
| 2024-02-19 | :30, :35, :55 | present |

A Monday close belongs to the session opened the prior Sunday, so the
first current opening day is **2024-02-11**. The last earlier opening day
is Thursday February 8; Friday and Saturday open no sessions. No
unwitnessed opening day remains between the two eras, and the intersection
profile is retired. Session boundaries remain keyed to their opening day
and never split a running session.

The first M1 pass's current-state inference from silent/lead buckets and
its alleged February 8 `16:58:40` FX tick were retracted: that tick occurs
in no retained response. The subsequent February 12 opening-day key was
also corrected to February 11. These are corrections to the interpretation,
not different operator endpoints.

`MARKET-DATA` — T2. Retention root:
`~/Developer/oanda-feed-captures-2026-10-16/`, with `MANIFEST.md` and
`SHA256SUMS.txt`. The original per-day record is `witness.jsonl` (1,813
records across seven books); the original S5 inputs are the timestamped
`EUR_USD_S5_*.json` files under `raw/`. The independent 2026-10-17 reads
retain full typed responses and request metadata as
`raw/validation-2026-10-17/era-EUR_USD-2024-02-{08,09,12}.json`;
the aggregation inputs are `E3-{S5,S10,S15,S30,M1}.json` there.
`validation-tools-2026-10-17/market-data-analysis.json` records the
recomputed counts. The overlapping original FX/metals windows reproduce
exactly. Initial retrieval 2026-10-16 UTC; independent reads and corrections
2026-10-17 UTC. The practice host does not confirm live-host equivalence;
that remaining check is tracked in #332.

## Pending orders and session scope — 2026-10-11 UTC

Page 2 of `OANDA-UK-HOURS-PDF`, reproduced in the operator HTML, distinguishes
closed-market position execution from placement and modification of pending
entry, protective and contingent instructions. Page 3 explains that those
instructions may trigger when trading resumes. This proves a broker-held
pending-order facility, not a separately timed executable or session-queue
phase; the source gives it no session boundaries or trade-date assignment.
The book keys model the printed trading windows. `is_accepting_orders` asks
about those executable and published order-entry phases, not whether OANDA's
order-placement API or account instruction store is available. Off-hours
pending instructions therefore do not become fabricated 24/7 session rules.
This scope applies to the venue default and all seven OANDA keys (#333).

## Starred seasonal selectors — 2026-10-11 UTC

The current UK PDF `OANDA-UK-HOURS-PDF` labels the columns
`Timezone of exchange` and `Trading hours (local exchange time)`. Its `***`
footnote states that the marked opening or closing time moves forward by one
hour during daylight saving. Read that instruction in each row's named local
zone; the current table states no US-season override for the Sydney row.
This contextual reading establishes the current selectors:

- Australia 200 names Sydney. Its marked night close is 06:59 in Sydney
  standard time and 07:59 in Sydney daylight time, selected by
  `Australia/Sydney`. Its unmarked 09:51–16:29 day session and 17:11 night
  opening retain those local times.
- Euro 50, Germany 40 and Germany 10Y Bund name Frankfurt. Their marked
  opening is 01:16 in standard time and 02:16 in daylight time, selected by
  `Europe/Berlin`. Their unmarked 21:59 close retains that local time.

Every local endpoint also receives its named zone's ordinary conversion to
UTC. The extra one-hour local-bound shift applies only to the `***`-marked
endpoint. FX's single `*` describes its six-minute break; metals' `**`
describes the Auckland break. The Shares/ETF prose says their times *may*
move, which does not establish an unconditional extra one-hour shift of both
endpoints. Neither the ordinary zone conversion nor the marked-bound rule
adds an underlying futures maintenance halt.

The undated operator guide `OANDA-STATIC-HOURS-252` prints a different,
earlier Australia 200 grid: 17:10–07:00 Sydney during US DST and
17:10–08:00 otherwise. That explicit historical US-season selector belongs
to that older grid. It does not override the current row-scoped instruction.
The effective day of the older/current grid and selector transition is still
unsourced; historical exactness, the six absent derivatives families and exact
broker holiday scopes remain under #332. Captures are observations, not
fabricated operator cutovers.

Captured guide bytes:
`normal-weeks/oanda-star-selector-audit-2026-10-11/oanda-static-hours-252.pdf`
in the research store. No date or new library runtime row is keyed from this
artifact; exact CFD overlays remain the consumer's responsibility.

## Historical normal-week audit — 2026-10-11 UTC

The existing 19 usable UK hours replays were inspected as operator bytes,
including all Forex tables. The earliest replay with the explicit Forex grid
is `OANDA-UK-HOURS-2021-DEC-WAYBACK`: `Sun-Fri: 17:03 - 16:58*` and
`*FX trading will be closed from 16:58 until to 17:03 (New York time) daily.`
`OANDA-UK-HOURS-2023-JUN-WAYBACK` still prints those endpoints on
2023-06-05. The 2024-02-21 capture prints the current endpoints. These are
observations, not operator-stated changeover dates. They narrow the open
changeover search to the interval after 2023-06-05 and by 2024-02-21; neither
capture establishes complete coverage before or between the observations.

A distinct operator guide, `OANDA-STATIC-HOURS-402`, states Sunday
approximately 5 p.m. to Friday 5 p.m. New York time and prints older
instrument grids. It is undated and its approximate opening cannot establish
an exact week or an effective day. `OANDA-WEEKEND-2012-MIRROR` reproduces
what it labels an OANDA release dated 2012-07-04, announcing a Friday
2012-07-06 weekly close at 4 p.m. EDT and a Sunday reopening at
approximately 3 p.m. EDT, ending the broker's weekend-trading service. The
operator original, `OANDA-WEEKEND-2012-ORIGINAL`, has now been recovered and
calibrates the reproduction. Both retain the initial 16:00 notice. A dated
2012-07-05 post by the OANDA representative on Forex Factory, post 3091,
corrects that initial notification to 17:00 New York; post 3093 preserves
the approximately 15:00 Sunday opening. That correction is visible through
web retrieval but a direct capture returned HTTP 403, so no correction-byte
artifact is claimed here. These statements establish a real dated change
while leaving an approximate opening, conflicting preserved close times,
and regional applicability to resolve before a complete UK row can ship. It is a specific warning that a
modern five-day FX week must not be silently carried back to the 2010 floor.

Distinct searches checked operator hours guides, regional hours pages,
operator announcements and the dated OEL execution policy, alongside the
existing replays. The eight already twice-refused UK replays were not
retried. No UK-scoped unconditional day for the minute-endpoint change was
found. No contact request is a prerequisite of this audit. The 2010 baseline,
any intervening weekday/weekend changes, and the exact 2023–2024 endpoint
change therefore remain tracked under #332; the normal-week coverage
declaration is unchanged. Closing condition: admissible UK-scoped artifacts
establishing those states and their day-level boundaries, or a separately
reviewed application of the charter's sourced-intersection rule after its
negative-hunt conditions are met. This audit does not claim every operator
channel has closed negative. (2026-10-12: the help-centre sweep above closed
the last indexed channel negative and the intersection now ships — see
"Changeover sweep closed negative".)

`OANDA-OEL-EXECUTION-2023`, section 5.4, expressly says OANDA's trading hours
for some products may differ from the instrument exchanges' operating hours.
An inherited exchange holiday calendar chosen by product timezone can be an
explicit caller policy; its exchange T1 evidence does not become broker T1
proof. That distinction also applies to older missing broker holiday years
and to TRY applicability (#332).

Audit artifact paths in the research store:
`normal-weeks/oanda-history-audit-2026-10-11/` with filenames
`oanda-uk-hours-20211208123011.html`,
`oanda-uk-hours-20230605093523.html`, `oanda-static-hours-402.pdf`,
`oanda-oel-execution-policy-2023-10-23.pdf`, and
`weekend-2012-mirror.html`.

## 2024 FX holiday extension — 2026-10-11 UTC

The complete 2024 portion of `OANDA-UK-CARTER-2025-WAYBACK` prints
`Wednesday 25th December 2024` with `FX open late 17:05`,
`Tuesday 24th December 2024` with `FX closes 16:59`, and
`Tuesday 31st December 2024` with `FX close 16:59`. The same next-trade-date
derivation used for 2025 supplies one new `Closed` row on 2024-12-25; the
17:05 reopen is normal. The two 16:59 closes are normal boundaries and add
no rows. The page identifies its content as changes for holidays, so the
remaining dates from 2024-02-21 through year end are audited normal FX.
Four FX closure rows now ship across 2024-02-21..2026-10-30.

The replay also prints a 2024-01-01 FX reopening at 17:00. That earlier
minute value is preserved in research, but the normal week's historical
state is not established there, so this extension stops at the sourced
2024-02-21 boundary instead of inventing a complete January session.
Artifact: `normal-weeks/oanda-history-audit-2026-10-11/oanda-uk-holidays-20250326184653.html`.

## Earlier normal-week context — 2026-10-11 UTC

The historical audit also retrieved two operator-issued PRNewswire
releases. `OANDA-PRESS-2010-FX` and `OANDA-PRESS-2011-FX` describe fxTrade as
enabling `true 24/7 trading`; this is the operator's corporate platform
statement, not a fully specified UK session table. The latter release
mentions the recent OANDA Europe launch. `OANDA-EUROPE-2011-MIRROR`
reproduces a release headed 2011-06-22 saying the London office and OANDA
Europe Limited have opened, while the article says an earlier announcement
already existed. A retrospective launch announcement is not an exact first
trading day. These artifacts improve the historical state evidence but do
not justify a UK 2010 24/7 baseline, erase maintenance uncertainty, or date
the subsequent change from an approximately 15:00 Sunday reopening to the
minute-specific modern week (#332).

These additional captures live in
`normal-weeks/oanda-history-audit-2026-10-11/` as
`oanda-press-2010-leverage.html`, `oanda-press-2011-crosspairs.html`, and
`oanda-europe-launch-2011-mirror.html`.

The UK operator's own demo-account pages now supply regional historical
context: `OANDA-UK-DEMO-2011` and `OANDA-UK-DEMO-2012-JUN` state
`Trade any time, 24/7. Only OANDA is open for weekend forex trading.`
The page identifies OANDA Europe Limited and its FSA registration 542574.
The 2012-08-05 capture removes that advertisement. These
account-demo statements are corroborated by the separate live fxTrade page:
`OANDA-UK-LIVE-2011` and `OANDA-UK-LIVE-2012-JUN` state
`Trade any time, 24/7. Only OANDA is open for weekend trading.` They list
real forex and gold/silver products and link live-account registration; the
2012-08-05 live capture removes the statement too. The regional operator
thus affirmatively advertised a real weekend service before July 2012,
rather than merely platform availability. The captures still do not establish
the live account's exact launch day or reconcile every maintenance
arrangement; they cannot automatically extend a complete UK baseline to the
2010 support floor.
The initial release expressly says regular weekday trading is unaffected.
The July 5 correction is retained as a lineage lead at
<https://www.forexfactory.com/thread/10892-oanda-discussion?page=155>,
not as a retrieved research-store artifact.

Additional artifact paths beneath the same audit directory:
`corp-weekend-20121019131147.html`, `uk-demo-20110621021538.html`,
`uk-demo-20120619063323.html`, `uk-demo-20120805035159.html`, and
`corp-news-20120802041322.html` (operator news index). The original source
path was discovered from the UK page's corporate-news link; the earlier
failed modern-hours captures were not retried.

The live-account captures are saved as `uk-live-20110621021554.html`,
`uk-live-20120619063328.html`, and `uk-live-20120805032236.html` beneath
the same audit directory.
