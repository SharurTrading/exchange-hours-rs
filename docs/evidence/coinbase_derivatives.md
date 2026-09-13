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

## Holidays

**Coverage:** 2026-01-01..2026-09-07 (inclusive trade dates). Tier: T1 throughout.

**Documents.**

- `CDE-MN-25-42` — Coinbase Derivatives Market Notice 25-42, 2025-11-25, “Coinbase Derivatives 2026 New Year's Day” <https://assets.ctfassets.net/k3n74unfin40/46Q4yxPzeFtSLYsDm4hz6C/f586118d66500c05dddcacceac04be97/Market_Notice_25-42_New_Years_Day.pdf> (retrieved 2026-09-12 04:51 UTC, sha256 `ac81272ecfc006a634cf855723f8a6d6875476555427d8612d90761172c12b0e`) — **T1**.
- `CDE-MN-26-01` — Market Notice 26-01, 2026-01-08, “Coinbase Derivatives 2026 Martin Luther King Jr. Day Schedule” <https://images.ctfassets.net/k3n74unfin40/jJEoZRkAZ0JlPC3UcJ2CL/d71ac89c5cd10783b9a7e11f449d1424/Market_Notice_26-01_MLK_Holiday.pdf> (retrieved 2026-09-12 07:41 UTC, sha256 `d4b206391f70a015002af78c04c46bdce0ed08f87005b5b8e383cc7c5cd01f38`) — **T1**. The href is recovered from the operator's own Market Notices page bytes (sha256 `523705cd5fa828ba035ec789a637a5c5d71298d1db4ab39e554b05e2b2f15a87`), whose table lists “26-01 | Market | 01/08/2026 | 2026 MLK | Holiday”.
- `CDE-MN-26-05` — Market Notice 26-05, 2026-02-02, 2026 Presidents' Day (`Market_Notice_26-05_Presidents-_Day_Holiday.pdf`, retrieved 2026-09-12 04:51 UTC, sha256 `9cc8ffb0b88a2a007d31243cd27ffc5c696585ddba8ec06ca562e63c1a448c34`) — **T1**.
- `CDE-MN-26-12` — Market Notice 26-12, 2026-03-19, 2026 Good Friday (`CDE_Market_Notice_26-12_Good_Friday_Holiday.pdf`, retrieved 2026-09-12 04:51 UTC, sha256 `3226ba73e92d91f252e8c163d49f9b329567c77f701dfba6a7f85413a25de150`) — **T1**.
- `CDE-MN-26-23` — Market Notice 26-23, 2026-05-15, 2026 Memorial Day (`CDE_Market_Notice_26-23_2026_Memorial_Day.pdf`, retrieved 2026-09-12 04:51 UTC, sha256 `b41bc8ee48084084fe5dbe8ac77be1f98688fad1b4b5a134d4fa6cd79f5c0869`) — **T1**.
- `CDE-MN-26-27.1` — Market Notice 26-27.1, 2026-06-15, an amendment that **supersedes** Notice 26-27 of 2026-06-10 (`CDE_Market_Notice_26-27.1_Amendment_to_26.27_2026_Juneteenth_Schedule_for_Gold_Silver_24x7.pdf`, retrieved 2026-09-12 04:51 UTC, sha256 `c141366e2f50eeacfd26def14e0ad6a9de448b0f319d79a5007933769881c10f`) — **T1, controlling**. It moved Gold (GOL) and Silver (SLR) into the 24x7 tier on trade date 2026-06-15, which is why they are no longer closed for Juneteenth; the superseded 26-27 (sha256 `e13cb644cebef3c851e83c14132860c8b6abc9b026a0dd23b36c5b55ae072b79`) is kept for lineage.
- `CDE-MN-26-29` — Market Notice 26-29, 2026-06-24, 2026 Independence Day observed (`CDE_Market_Notice_26-29__2026_Independence_Day_Schedule.pdf`, retrieved 2026-09-12 04:51 UTC, sha256 `481e0f35da824519f4046129cc66142ae004b90fe19b2749b2073f8b89dea736`) — **T1**.
- `CDE-MN-26-36` — Market Notice 26-36, 2026-08-25, 2026 Labor Day (`Market_Notice_26-36__2026_Labor_Day.pdf`, retrieved 2026-09-12 04:51 UTC, sha256 `c21dc783a6e6c560540126a30572e932654d8531f4e7225f0b3221514116616d`) — **T1**.

All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Closed for holiday`; “In observance of New Year's Day, the January 1, 2026 trading session will be closed for select Coinbase Derivatives, LLC products.” | `CDE-MN-25-42` | T1 | CDE trade date Thursday 1/1; the notice prints Friday 1/2 as OPEN/START 1/1 17:00 CT, CLOSE/ROLL 1/2 16:00 CT |
| 2026-01-19 | closed | `Closed for holiday`; “For select Coinbase Derivatives products, the January 19, 2026 trading session will be closed in observance of Martin Luther King Jr. Day.” | `CDE-MN-26-01` | T1 | CDE trade date Monday 1/19; Tuesday 1/20 prints OPEN/START 1/19 17:00 CT, CLOSE/ROLL 1/20 16:00 CT |
| 2026-02-16 | closed | `Closed for holiday`; “For select Coinbase Derivatives products, the February 16, 2026 trading session will be closed in observance of Presidents' Day.” | `CDE-MN-26-05` | T1 | CDE trade date Monday 2/16; Tuesday 2/17 prints OPEN/START 2/16 17:00 CT, ROLL 2/17 16:00 CT |
| 2026-04-03 | closed | `Closed for holiday` | `CDE-MN-26-12` | T1 | CDE trade date Friday 4/03; Monday 4/06 prints OPEN/START 4/05 17:00 CT, ROLL 4/06 16:00 CT |
| 2026-05-25 | closed | `Closed for holiday` | `CDE-MN-26-23` | T1 | CDE trade date Monday 5/25; Tuesday 5/26 prints OPEN/START 05/25 17:00 CT, ROLL 05/26 16:00 CT |
| 2026-06-19 | closed | `Closed for holiday` in both the OPEN/START and CLOSE/ROLL cells | `CDE-MN-26-27.1` | T1 | CDE trade date Friday 06/19 for the 23x5 and 24x5 tiers; Monday 06/22 prints OPEN/START 06/21 17:00 CT, CLOSE/ROLL 06/22 16:00 CT |
| 2026-07-03 | closed | `Closed for holiday`; “In observance of Independence Day (observed), the Friday, July 3, 2026 trading session will be closed for select Coinbase Derivatives products.” | `CDE-MN-26-29` | T1 | CDE trade date Friday 7/03; Monday 7/06 prints OPEN/START 7/05 17:00 CT, ROLL 7/06 16:00 CT |
| 2026-09-07 | closed | `Closed for holiday`; “In observance of Labor Day, the Monday, September 7, 2026 trading session will be closed for select Coinbase Derivatives products.” | `CDE-MN-26-36` | T1 | CDE trade date Monday 9/7; Tuesday 9/8 prints OPEN/START 09/07 17:00 CT, ROLL 09/08 16:00 CT |

**Gaps, 2026:** **Coverage stops at 2026-09-07.** CDE publishes no standing annual holiday calendar — `coinbase.com/derivatives/holiday-calendar` is a 404 — only one Market Notice per holiday, and the Thanksgiving 2026-11-26 and Christmas 2026-12-25 notices had not issued as of 2026-09-12 (their 2025 equivalents, 25-37 and 25-41, issued in November and December 2025). No 2027 CDE holiday document of any kind exists. Both are closed by the CDE Market Notices listing as those notices appear. **The baseline grid is a moving target.** Notices 26-25 and 26-33/26-33.1/26-33.2 (“24x7 Transition … Final Friday Maintenance Windows”, from 2026-09-11) change the recurring schedule the holiday notices deviate from; they belong to the regular-schedule review of this venue, not to the holiday table, and were not retrieved. **No 24x7 rows exist here.** Crypto trades through every one of these dates, and from 2026-06-15 so do Gold and Silver; the 24x7 tier is out of scope for this venue profile and claims no key, so nothing about it is encoded.

**Interpretive steps, 2026:** CDE's notices are already keyed by **trade date** and print the neighbouring trade dates' open, close and roll instants beside each one, so the conversion to the crate's trade-date key is the identity and the operator's own table corroborates every row. Each row is a full closure of the 23x5 tier the venue profile models (Sunday to Friday, 17:00-16:00 CT with the daily 16:00-17:00 break): the notice's own “OPEN/START `D` 17:00 CT” for the following trade date is what shows that the evening leg the closure deletes is the holiday's, not the next day's. The tier labels move across the window — 2026-01-01 prints “Energy & Metal” and “Equity”, 2026-01-19 through 2026-05-25 print “Energy, Metal & Equity”, and from 2026-06-19 the notices print “23x5 Products: Energies, Copper & Platinum, Equities” beside a separate “24x5 Products: Equity Indexes PSF” line — but every one of those labels is closed on its date, so the venue row is the same either way.

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
