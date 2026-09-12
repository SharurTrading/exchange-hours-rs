<!-- SPDX-License-Identifier: MIT-0 -->

# `coinbase_derivatives` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`coinbase_derivatives.rs`](../../src/calendar/schedules/futures/us/coinbase_derivatives.rs)
- **Source sets:** [`US-COINBASE-DERIVATIVES`](../schedules/sources.md#us-coinbase-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Closed before FairX's exact 2021-06-28 08:00 CT launch, which the operator's homepage banner states; the first session opens at the launch, with no Sunday-evening session before it. The 23x5 grid, Sunday–Friday 17:00–16:00 CT with a daily 16:00–17:00 break, is stated by the four 2021-06-04 launch certifications and restated by every later dated filing through #2026-24, so no revision separates the launch from the 2026-09-11 review date. The Pre-Open phase is documented from 2021 without a time and its 16:50 CT start is first witnessed in a 2025 capture, so it enters at a knowledge-bound 2026-09-11 review row (verified current, onset undated); the dated profiles before it carry no Pre-Open. Since #2026-24 (on or after trade date 2026-05-04) most CDE futures trade 24x7; this default is the grid that copper, platinum, nano crude oil, natural gas and Mag7 + Crypto equity index futures retain. The 24x7 family, with its weekly Friday, quarterly weekend and ad-hoc maintenance windows, is out of scope; no product-family key is claimed.

## Revision rows

- 2026-09-11 — T1 — 2026-09-11 review: verified current, onset undated — the knowledge-bound row that adds the 16:50–17:00 CT Pre-Open queue to the sourced 23x5 grid.

The row's own tier is the tier of the artifact it rests on: the operator's own
market-hours documentation page, read in a 2025 capture. The row makes no onset
claim and never moves forward; a Coinbase Derivatives artifact that states the
Pre-Open on a day-level effective date replaces it.

Two further boundaries are in `profile_at` rather than in the `revisions!`
block, because neither is a venue-local-midnight revision and neither is a
revision row. The launch is an exact instant, 2021-06-28 13:00:00 UTC
(08:00 CDT), stated at T1 by FairX's own homepage banner; everything before it
is a sourced closure. The one-off launch-day profile then hands over to the
recurring 23x5 grid on 2021-06-29, the first full day, and the Monday-evening
session that runs into the next day is identical in both, so the handover never
splits a running session.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2021-06-28 — T1 — FairX homepage banner, 2021-06-22 capture — an exact-instant boundary at 2021-06-28 13:00:00 UTC (08:00 CDT), selected in `profile_at` rather than as a tuple; `HISTORICAL_INSTANT_CUTOVERS` records the same instant.
- 2021-06-29 — T1 — the same four 2021-06-04 launch certifications — the one-off launch-day profile hands over to the recurring 23x5 grid on the first full day; the Monday-evening session is identical in both, so the handover splits no running session.

## Sources

Row review: 2026-09-11 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below.

- <https://www.cftc.gov/filings/ptc/ptc060421lmxdcm001.pdf> — FairX launch certification #2021-06E, filed 2021-06-04, one of the four that state the 23x5 grid at launch — T1.
- <https://www.cftc.gov/filings/orgrules/rules0416261571.pdf> — Coinbase Derivatives filing #2026-24, which states which products keep the 23x5 grid after the 24x7 move — T1.
- <https://docs.cdp.coinbase.com/derivatives/introduction/market-hours> — the operator's market-hours documentation, the current-schedule entry point and the page that witnesses the 16:50 CT Pre-Open — T1. The page is CDE's "Derivatives Market Hours & 24x7" document, whose own opening sentence dates its content ("Starting May 9, 2025, Coinbase Derivatives, LLC (CDE) will enable 24x7 trading for select cryptocurrency futures products"); under **Regular Market Hours for 23x5 Crypto Products** and again under **Regular Market Hours for Energy and Metals Products** it states "Pre-open quoting begins daily at 4:50 PM CT, 10 minutes before the market opens."
- Research-store artifact `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/cde_market_hours.md` — the saved bytes of that page, **captured 2026-09-12 04:23 UTC** (sha256 `ff8cace80f2f6bb9e463c6ac762c8f50f79fa76ce7b205c14e42d29e847d13a5`), carrying both Pre-Open sentences quoted above — T1. This is the earliest capture of the page held in the research store; the "2025 capture" the basis note and the gap bullet refer to is the 2025-dated *content* of this document, not a separately saved 2025 retrieval. No archived 2025 capture is linked, because web.archive.org was unreachable for the whole 2026-09-12 retrieval session (recorded in that task's `INDEX.md`).
- <https://www.coinbase.com/derivatives> — the exchange product directory — T1.
- <https://help.coinbase.com/derivatives/general/market-notices> — the exchange's market notices, the watch channel — T1.
- <https://web.archive.org/web/20210622222253/https://www.fairx.com/> — the 2021-06-22 capture of FairX's homepage banner, naming Monday 2021-06-28 at 09:00 ET and fixing the year — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20210802230534/https://www.fairx.com/> — the 2021-08-02 capture, reporting the venue open — T1 through a verbatim public mirror.
- <https://www.cftc.gov/IndustryOversight/IndustryFilings/TradingOrganizations/43304> — the CFTC DCM record tying FairX, Coinbase Derivatives and the 2023 legal-name change to one contract market — T1.

## Gaps and residual risks

- **order-entry** — the Pre-Open queue's onset is undated. Coinbase Derivatives
  documents a Pre-Open phase from 2021 without a time, and the 16:50 CT start is
  first witnessed in a 2025 capture of the market-hours page, so no artifact
  states when it began. The crate therefore withholds the queue from every dated
  profile and admits it only at the knowledge-bound 2026-09-11 review row. No
  trade can print in the queue, so the gap never touches an executable window.
  Closing condition: a Coinbase Derivatives or FairX artifact that states the
  Pre-Open in session language on a day-level effective date. Served identity,
  so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **The 24x7 family is deliberately out of scope.** Since #2026-24, on or after
  trade date 2026-05-04, most CDE futures trade 24x7 with weekly Friday,
  quarterly weekend and ad-hoc maintenance windows. The venue default stays the
  23x5 grid that copper, platinum, nano crude oil, natural gas and the
  Mag7 + Crypto equity index futures retain. A 24x7 product family needs its own
  `MarketHoursKey` and its own sourced windows; none is claimed here, and the
  caller must not read the venue default as covering those roots.
- **A golden-fixture consequence, recorded so it is not rediscovered.** The
  2026-08-22 fixture instant in `tests/golden/normal_week_grids.txt` predates the
  knowledge-bound row, so it renders the dated grid without the Pre-Open.

## Module narrative (moved from src/calendar/schedules/futures/us/coinbase_derivatives.rs on 2026-09-12 UTC)

The venue default is CDE's recurring 23x5 futures grid: Sunday through
Friday, 17:00-16:00 CT, with the daily 16:00-17:00 break. The four launch
certifications filed 2021-06-04 state that grid, and every later dated
filing through #2026-24 restates it, so no revision separates launch from
the 2026-09-11 review date. Since #2026-24 (on or after trade date 2026-05-04) most CDE futures
trade 24x7; this grid is the one copper, platinum, nano crude oil, natural
gas and Mag7 + Crypto equity index futures retain. The 24x7 family needs its
own product-family key at the caller.
<https://www.cftc.gov/filings/ptc/ptc060421lmxdcm001.pdf>
<https://www.cftc.gov/filings/orgrules/rules0416261571.pdf>
<https://docs.cdp.coinbase.com/derivatives/introduction/market-hours>

ORDER ENTRY, NOT TRADING. Pre-Open quoting accepts orders for the coming
session and nothing matches until the 17:00 open. The Pre-Open phase is
documented from 2021 without a time, and its 16:50 start is first witnessed
in a 2025 capture of the market-hours page, so no source dates its onset: it
enters only at the knowledge-bound row below, and the dated profiles before
it carry no Pre-Open.

The dated grid from launch: the sourced trading session, no Pre-Open.

FairX, as CDE then traded, opened for trading on Monday 2021-06-28 at 09:00
ET with no Sunday-evening session before it. The operator's homepage banner
names the day and time; its captures from 2021-06-22 fix the year, and the
2021-08-02 capture reports the venue open. The launch-day profile starts
that first session at the launch instant, so its bounds never reach back to
a Sunday 17:00 that did not trade. The Monday-evening session that runs into
the next day is identical in both profiles, so the day-level switch to the
full grid never splits it.
<https://web.archive.org/web/20210622222253/https://www.fairx.com/>
<https://web.archive.org/web/20210802230534/https://www.fairx.com/>

2021-06-28 13:00:00 UTC, 08:00 CDT. An exact instant is required; this
launch is not a venue-local-midnight revision.

Knowledge-bound row, dated at the UTC date of the review that verified the
16:50 Pre-Open (LAW-UTC-DATES). It adds only that queue, makes no onset
claim, never moves forward, and a sourced onset day replaces it. One
consequence is visible in `tests/golden/normal_week_grids.txt`, whose
2026-08-22 fixture instant renders the dated grid without the Pre-Open.
