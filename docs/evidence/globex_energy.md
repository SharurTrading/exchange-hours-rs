<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_energy` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Named NYMEX energy/PGM and COMEX metals roots only; TAS/TAM/BTIC, options, and different product clocks are excluded. Current matching and queues are exact and the 2015 close revision is dated; historical selectors omit the queues because the Sunday 16:15→16:00 move is bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld.

## Revision rows

- 2015-09-20 — T1 — CME Globex notice 20150907 — every COMEX and NYMEX close moves to 16:00 CT for trade date Monday 2015-09-21.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html> — CME Globex notice 20090130, the pre-floor 17:00–16:15 CT energy and metals grid.
- <https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf> — CME `MT-027` gold futures cheat sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html> — CME SER-5391.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, which observes the weekday 16:45 queue already in effect without dating its onset.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html> — CME Globex notice 20150907, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html> — CME gold futures contract specification, current grid.
- <https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html> — CME light sweet crude contract specification, current grid.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-05-01, "17:15 ET (16:15 CT)". **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-06-16, "17:00 ET (16:00 CT)". **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the earliest artifact in the Sunday-queue intersection. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — named NYMEX energy/PGM and COMEX metals roots only; TAS/TAM/BTIC, options and different product clocks are excluded.

## Holidays

**Coverage:** 2025-01-01 .. 2027-12-31 (inclusive venue-local trade dates,
`America/Chicago`). Tier: **T2** throughout — CME's own trading-hours service,
the endpoint `cmegroup.com/trading-hours.html` itself calls, read as bytes and
saved. No T1 per-asset-class rendering exists for these years; that is a named
gap below, not a reason to withhold the rows.

**Zone.** CME prints no Eastern column in this channel. The operator states the
zone once, on the page the service backs, quoted verbatim: "Trading hours are
subject to change and are in U.S. Central Time unless otherwise stated." The
calibration is independent of that sentence as well: the same channel prints
`16:00 closed`, `16:45 preopen` and `17:00 open` for an ordinary Monday-Thursday
of the reference week 2026-10-18..2026-10-24, which is this family's sourced
17:00-16:00 CT grid event for event. Every instant below is therefore CT as
printed, and **no ET value is asserted**.

**Energy and metals are one key, and CME prints them as one row.** On every
date in this window the service returns `CL` and `GC` with identical event
lists, and the payload records them under one combined product-group label, so
the memo's D17 intersection rule is never reached and no date is withheld for
a disagreement between the two halves.

**How an event date becomes a trade date.** The family's trading day wraps: one
occurrence opens 17:00 CT on the previous local day and closes 16:00 CT on the
trade date, Sunday through Thursday. Three shapes appear in this window:

1. a final `closed` event on the date at an instant earlier than 16:00 CT — an
   `EarlyClose` on that trade date, clipping the occurrence that opened the
   previous evening;
2. no final close on the date, only a pre-open and a 17:00 CT `open` carrying
   the **next** business day's trade date, or no events at all — no session
   belongs to the date, so the row is `Closed` and the previous evening's leg
   goes with it;
3. only a `13:30 preopen` on the date, followed by the ordinary `17:00 open`.
   CME's own legend for the event types reads, verbatim, PREOPEN — "Order
   Entry, modification, and cancel are allowed. No order matching." — and OPEN —
   "Start of continuous trading phase. Order matching begins." A pre-open
   therefore ends matching, so 13:30 CT is that trade date's final close and the
   row is an `EarlyClose` at 13:30 CT. This is the `[N16]` shape of the
   retrieval and the same reading the design memo applies to the Equity Index
   `12:00 preopen` row of 2025-01-20.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — both 2025-01-02, so no session settles on 01-01 | `CME-SVC-2024-12-31` | T2 | eventDate 2024-12-31 `16:00 closed` CME trade date 2024-12-31 with no evening re-open, and eventDate 2025-01-01 CME trade date 2025-01-02 |
| 2025-01-20 | early close | `13:30 preopen` — 13:30 CT; the 17:00 CT open that follows carries 2025-01-21 | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME trade date printed as 2025-01-21 |
| 2025-02-17 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-02-18 | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME trade date printed as 2025-02-18 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18, and eventDate 2025-04-17 `16:00 closed` CME trade date 2025-04-17 with no evening re-open |
| 2025-05-26 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-05-27 | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME trade date printed as 2025-05-27 |
| 2025-06-19 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-06-20 | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME trade date printed as 2025-06-20 |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-09-02 | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME trade date printed as 2025-09-02 |
| 2025-11-27 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-11-28 | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-27, CME trade date printed as 2025-11-28 |
| 2025-11-28 | early close | `07:00 preopen; 07:30 open; 13:45 closed` — final close 13:45 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28 |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29, the Thanksgiving window extended to the Saturday |
| 2025-12-24 | early close | `12:45 closed` — 12:45 CT, and no evening re-open | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — both 2025-12-26 | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26; the 2025-12-24 record carries no evening leg for 12-25 |

**Interpretive steps, 2025.** The 2025-01-01 and 2025-12-25 rows are the
shape-2 conversion: CME publishes a pre-open and a 17:00 CT open on the holiday
itself, both already carrying the next business day, so the holiday has no
trade date and the row is `Closed` rather than a late open. The 2025-01-20,
2025-02-17, 2025-05-26, 2025-06-19, 2025-09-01 and 2025-11-27 rows are the
shape-3 conversion described above. Saturday 2025-11-29 changes no answer for
this family — the normal week has no Saturday trade date — and ships because
CME's own 2025 Globex table states the Thanksgiving period as "27 - 29 November
2025" and the operator's channel was asked and answered for the Saturday.

**Gaps, 2025.**

- **Intraday topology, 2025-11-28** — the finalised publication prints
  `07:00 preopen; 07:30 open` ahead of the 13:45 CT close, on a trading day
  that opened 17:00 CT the previous evening. That implies a halt CME does not
  print, and the scalar vocabulary of a holiday row cannot state it. Not a late
  open: the trading day's own first open is 17:00 CT on 11-27, and clipping to
  07:30 CT would delete fourteen hours CME publishes as open. Recorded, not
  modelled. Closing condition: a block-row holiday kind (#93), or an operator
  statement of the halt.
- **Order-entry deviations** — on the shape-3 dates the pre-open runs
  13:30-17:00 CT rather than the normal 16:45-17:00 CT, and on 2025-01-01 and
  2025-12-25 it starts 16:00 CT rather than 16:45 CT. A holiday row has no
  order-entry boundary, so this is not representable. It changes no `is_open`
  answer, only `is_accepting_orders` and `is_order_entry_only`.
- **Pre-finalisation publications** — the 2025-01-01 through 2025-09-01 rows
  rest on the single archive capture 2024-12-20T15:53:40Z, on a page that
  states "This schedule is subject to change. Trading hours are usually
  finalized approximately two weeks prior to the holiday." The live service was
  re-probed on 2026-09-12 for all eight of those windows and returns the
  products with empty schedules — a retention edge, not a contradiction — so no
  post-finalisation statement is reachable. Thanksgiving 2025 and Christmas
  2025 are sourced from post-holiday captures instead.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — both 2026-01-02 | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02; eventDate 2025-12-31 `16:00 closed` with no evening re-open |
| 2026-01-19 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-01-20 | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-19, CME trade date printed as 2026-01-20 |
| 2026-02-16 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-02-17 | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-16, CME trade date printed as 2026-02-17 |
| 2026-04-03 | closed | `no events published` | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, and eventDate 2026-04-02 `16:00 closed` CME trade date 2026-04-02 with no evening re-open |
| 2026-05-25 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-05-26 | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-25, CME trade date printed as 2026-05-26 |
| 2026-06-19 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date printed as 2026-06-22 |
| 2026-07-03 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date printed as 2026-07-06 |
| 2026-09-07 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-09-08 | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date printed as 2026-09-08 |
| 2026-11-26 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-11-27 | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date printed as 2026-11-27 |
| 2026-11-27 | early close | `13:45 closed` — 13:45 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:45 closed` — 12:45 CT, and no evening re-open | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25; the 2026-12-24 record carries no evening leg for 12-25 |

**Interpretive steps, 2026.** The 2026-06-19 and 2026-07-03 rows are the
Friday-holiday divergence. CME prints the final close on the Friday but dates
it to the following Monday, so on the operator's clearing convention the Friday
has no trade date at all. The crate assigns a session to the venue-local date
of its own final close, and this family has a weekend close and no
following-business-day roll, so a `Closed` row here would delete the whole span
from 17:00 CT Thursday to 12:00 CT Friday — trading CME itself publishes. The
row therefore stays on the Friday as an early close at the sourced instant, and
the trade-date divergence is recorded here rather than modelled. The
cryptocurrency family is treated differently for the same operator records
precisely because its business-date roll preserves the trading instead of
deleting it (design memo D9).

**Gaps, 2026.**

- **Saturday sessions, 2026-06-20 and 2026-07-04** — CME publishes
  `05:00 open; 17:00 closed`, both carrying the following Monday, on Saturdays
  the normal week has none. A holiday row can move an existing occurrence's
  boundaries; it cannot create an occurrence. Sourced and unrepresentable.
  Closing condition: a block-row holiday kind (design memo §7 follow-up 8, #93).
- **Order-entry deviations** — as 2025: the shape-3 pre-opens run 13:30-17:00
  CT and the 2026-01-01 pre-open starts 16:00 CT.
- **A T1 rendering** — the trading-hours page renders only the next upcoming
  holiday and its selector could not be driven from a URL, so no T1
  per-asset-class rendering was obtainable for any date in this block. One T1
  capture, for Thanksgiving 2026, was used to validate the service rows group
  by group and matched row for row.
- **Black Friday finalisation** — 2026-11-27 is sourced from a pre-holiday
  publication. The 2025 equivalent gained a `07:00 preopen; 07:30 open` pair
  when CME finalised it, so this row may gain the same intraday pair. The close
  instant was unchanged by finalisation in 2025.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, and eventDate 2026-12-31 `16:00 closed` CME trade date 2026-12-31 with no evening re-open |
| 2027-01-18 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-01-19 | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date printed as 2027-01-19 |
| 2027-02-15 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-02-16 | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date printed as 2027-02-16 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, and eventDate 2027-03-25 `16:00 closed` CME trade date 2027-03-25 with no evening re-open |
| 2027-05-31 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-06-01 | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date printed as 2027-06-01 |
| 2027-06-18 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date printed as 2027-06-21 |
| 2027-07-05 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-07-06 | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date printed as 2027-07-06 |
| 2027-09-06 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-09-07 | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date printed as 2027-09-07 |
| 2027-11-25 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-11-26 | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date printed as 2027-11-26 |
| 2027-11-26 | early close | `13:45 closed` — 13:45 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-24, and eventDate 2027-12-23 `16:00 closed` CME trade date 2027-12-23 with no evening re-open |

**Interpretive steps, 2027.** 2027-06-18 is the third instance of the
Friday-holiday divergence and is converted exactly as 2026-06-19 above.
CME's holiday date for Christmas 2027 is Thursday 2027-12-23, on which the
family closes at its normal 16:00 CT and therefore ships no row; what the
closure removes is the Friday, so the only row is `Closed(2027-12-24)`.

**Gaps, 2027.**

- **Saturday session, 2027-06-19** — `05:00 open; 17:00 closed` carrying
  2027-06-21, on a Saturday the normal week has none. As 2026 above.
- **Order-entry deviations** — as 2025 and 2026.
- **A T1 rendering** — as 2026.
- **Black Friday finalisation** — 2027-11-26 is sourced from a pre-holiday
  publication; see the 2026 note.

### Documents

Every id below resolves to one response of CME's trading-hours service for the
stated event-date window, product id set `316,133,425,300,58,437,22,8478,5201,10191`.
Expand `[THBP-A]` to
<https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true>
plus `&fromEventDate=…&toEventDate=…`, and `WA:<ts>` to
`https://web.archive.org/web/<ts>id_/` in front of it. Bytes, hashes and the
research-store code for each are in
`exchange-hours-research/holidays/raw/cme-2025-2027*/INDEX.md`.

| Document | Window | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | live retrieval 2026-09-12T04:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | live retrieval 2026-09-12T04:30Z | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | live retrieval 2026-09-12T04:30Z | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | live retrieval 2026-09-12T04:30Z | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | live retrieval 2026-09-12T04:30Z | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | live retrieval 2026-09-12T04:30Z | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |

### Holiday coverage notes

- **Dates with no row are audited normal.** Columbus Day and Veterans Day are
  the ones a reader will look for: they appear nowhere in CME's own Globex
  holiday list, CME publishes only clearing advisories and settlement-time PDFs
  for them, and Globex trades a normal session on both. They are therefore not
  holidays for this family, and their absence from the tables above is a
  positive answer rather than a silence.
- **2028-01-01 is outside coverage and ships no row**, although the retrieval
  holds it: coverage ends at the last day of the operator's published future
  for this family, 2027-12-31.
- **No row below 2025-01-01.** Waves 2 to 6 of the migration extend the window
  backwards to the January-2010 floor; until then `holiday_coverage` says so
  and `holiday_on` answers nothing below 2025.
- **Reviewed on** 2026-09-12 (UTC).


> Shared module. The narrative for
> [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs) lives in
> [`comex`](comex.md#module-narrative-moved-from-srccalendarschedulesfuturesusenergy_metalsrs-on-2026-09-12-utc).
> Sibling identities: [`comex`](comex.md), [`nymex`](nymex.md).
