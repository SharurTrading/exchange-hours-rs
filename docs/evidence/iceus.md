<!-- SPDX-License-Identifier: MIT-0 -->

# `iceus` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_us.rs`](../../src/calendar/schedules/futures/us/ice_us.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

NYSE FANG+ Index Futures only: closed before the sourced 2017-11-07 launch-eve 19:30 Pre-Open / 20:00 matching start. Current Pre-Open is Sunday 17:30–18:00 and Monday–Thursday 19:30–20:00; regular matching then runs through 18:00 the next day.

## Revision rows

- 2017-11-07 — T1 — ICE FANG+ launch notice 20170926 — the launch-eve profile: a Tuesday 19:30–20:00 ET Pre-Open and the 20:00 matching start, and nothing earlier that day.
- 2017-11-08 — T1 — ICE FANG+ launch notice 20170926 — the full grid for trade date 2017-11-08: Sunday 18:00 open, Monday–Thursday 20:00 opens, matching through 18:00 the next day, with the 30-minute Pre-Open queues.

Everything below the first row is `CLOSED_NEW_YORK`, a sourced closure, so the
row's horizon is `—` and nothing is carried back to the January-2010 floor.

## Holidays

**Coverage:** 2026-01-01 .. 2028-01-03 (inclusive trade dates). Tier: T1 throughout.

**Documents.**

- `IFUS-CAL-2026` — “ICE Futures U.S. — June 9, 2025 — 2026 Trading Holiday Calendar”, the Exchange-Notice copy <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice_2026_Holiday_Calendar_20260609.pdf> (retrieved live 2026-09-12 07:47 UTC and by Wayback `id_` replay of capture `20260728113521` at 07:43 UTC, byte-identical; sha256 `b9723d9afa1c20a0792b628de36c7d3726add58ea3d94972a1e7cefd3e50bf4e`) — **T1**. The Holiday-Hours-hub copy <https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> (retrieved 2026-09-12 04:19 UTC, sha256 `da97503545a3fb7607948367d30896685e220b36abed24aa02bbe1a21acb2817`) carries the earlier product-group headers; every `open` / `closed` / `open1` status cell is identical in the two, so the rows here rest on either copy and the labels follow the Exchange-Notice copy, which is also the 2027 calendar's spelling.
- `IFUS-CAL-2027` — “ICE Futures U.S. — June 4, 2026 — 2027 Trading Holiday Calendar”, issued as an Exchange Notice <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice-2027_Holiday_Calendar_20260604.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `d2e39a2db2a26e0578ad0d09b36f51dfdad8502635020fe79d66303c09c1a040`) — **T1**, and an unconditional published future, which LAW-NO-FABRICATED-DATES permits encoding ahead of its effective days.
- `IFUS-NOTICE-2026-GOODFRIDAY` — ICE Futures U.S. 2026 Good Friday trading schedule, February 2, 2026, “all times shown in NY time” <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_GoodFridayHoliday_20260202.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `0a38abd0c9c8b0f904e9701999ac56466610c46437bdfe0109f88e67da35d181`) — **T1**. It carries the 2026-04-06 Easter Monday row as well.
- `IFUS-NOTICE-2026-MEMORIAL` — ICE Futures U.S. 2026 Memorial Day schedule, March 10, 2026 <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Memorial_DayHoliday_20260310.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `3dfe2a49babece024ce3cd91948e4fb27740abe31282fd9121097bd8433298f9`) — **T1**.
- `IFUS-NOTICE-2026-JUNETEENTH` — ICE Futures U.S. 2026 Juneteenth schedule, March 20, 2026 <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_JuneteenthHoliday_20260320.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `34a21ef6facf3dafd72d444ea352fc60e9b2dbf4fc674c1f4f8c199936169edd`) — **T1**. The equity-index companion notice of the same date (sha256 `6f07e49c36c70ef0c81cf028c3c7f539339b03092c81b22cf9c583f9f0c6b9e2`) is last-trade-day and final-settlement content only and keys nothing here (LAW-SESSION-NOT-EXPIRY).
- `IFUS-NOTICE-2026-INDEPENDENCE` — ICE Futures U.S. 2026 Independence Day schedule, June 26, 2026, marked “**Revised (MSCI Index, Stock Index, ICE Mortgage, and SOFR)” <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US__IndDayHoliday_.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `9de2852ed397d403cf8061358d98839954dbe861bf349fb91f57f95910983ea6`) — **T1**. It carries the 2026-07-06 Cotton row as well, and it supersedes an earlier version that wrongly closed the index and rates products early on Thursday 2026-07-02.
- `IFUS-NOTICE-2026-LABORDAY` — ICE Futures U.S. 2026 Labor Day schedule, July 8, 2026 <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_LaborDayHoliday_20260708.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `d383fe9ae20065f4de6825bff68f13d9cdd8233ea57de0f40644af92439b88f4`) — **T1**. This is the notice that names “NYSE FANG+™ Index” inside the NYSE-index early-close bullet.
- `IFUS-NOTICE-2026-COLUMBUS` — ICE Futures U.S. 2026 Columbus Day / Canadian Thanksgiving notice, August 28, 2026 <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Columbus_Day_Thanksgiving_Canada_8.28.26.pdf> (retrieved 2026-09-12 04:20 UTC, sha256 `ca04aca37e90bc961c4e766610bb93f74705afcd303b4617e1ec825e7dbfeff4`) — **T1**. Canola closed, “all ICE Futures U.S. contracts other than Canola futures and options will follow regular trading hours”, so it keys no row and is the positive statement of normality for 2026-10-12.

All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-01; all five calendar groups print `closed`, so every modelled family agrees |
| 2026-01-19 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-19: softs `closed`, index families `open1` |
| 2026-02-16 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-02-16: softs `closed`, index families `open1` |
| 2026-04-03 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Fri, Apr 3: softs `Closed`, index families on a 05:00 late open with a 09:15 or 11:15 early close |
| 2026-04-06 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Mon, Apr 6: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2026-05-25 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25: softs `Closed`, FANG+ 13:00, dollar index 14:30 |
| 2026-06-19 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19: softs `Closed`, FANG+ 13:00, dollar index 14:30 |
| 2026-07-03 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3: softs `Closed`, FANG+ 13:00, dollar index 14:30 |
| 2026-07-06 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Mon, July 6: Cotton late at 08:00, every other family regular |
| 2026-09-07 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2026-11-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26: softs `closed`, index families `open1` |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25; all five calendar groups print `closed` |
| 2026-12-28 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-28: softs `open`, index families `open1` |

**Gaps, 2026:** **No 2026 MLK or Presidents Day Exchange Notice exists.** The ICE notices listing reaches back only to February 2026, and a Wayback CDX prefix enumeration of `ice.com/publicdocs/futures_us/exchange_notices*` for 2025-2027 (113 distinct URLs, saved as `ifus_notices_cdx_2025-2027.json`) holds neither; it holds the 2025 equivalents, which must not be read across years. So the gap is tested, not assumed. **The late-2026 notices had not issued at retrieval:** Thanksgiving 2026-11-26, Christmas Eve 2026-12-24, Boxing Day 2026-12-28 — and note that 2026-11-27 and 2026-12-24 are not holidays on the calendar at all, so their customary early closes exist only in notices that did not yet exist and this table therefore carries no row for either. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice. **Scope of the intersection.** `Canola`, `Energy* and Environmental Contracts` and `Daily Gold and Silver Contracts` are product groups on ICE's own calendar that no crate identity routes to, so they are outside this table's intersection; a consumer that maps such a product to the venue calendar would be using a clock the crate never claimed for it. On all four dates that do ship a `closed` row those three groups are closed as well, so including them would not change a row.

**Interpretive steps, 2026:** **The venue table is the intersection of the families that route to it** (design memo D17): the seven `ice_us*` keys. A date ships a scheduling row only where all seven carry the same row, which in this window means the four dates on which every ICE product group prints `closed`. **A disagreement date ships `Unsourced`, not silence.** D17 says such a date ships no scheduling row and is a declared gap; inside a contiguous coverage window, silence is the positive claim that the date was audited normal, which on these dates is false. `Unsourced` is the third thing the vocabulary exists to say: it changes no answer, it clips nothing, and it tells a consumer that the crate knows the date is special and cannot state one answer for the venue. Each such date's own disagreement is printed in the `Derived from` column above, and the per-family rows are in the seven key evidence files.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-01; all five calendar groups print `closed` |
| 2027-01-18 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-18: softs `closed`, index families `open1` |
| 2027-02-15 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-02-15: softs `closed`, index families `open1` |
| 2027-03-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-03-26: softs `closed`, index families `open1` |
| 2027-05-31 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-05-31: softs `closed`, index families `open1` |
| 2027-06-18 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-06-18: softs `closed`, index families `open1` |
| 2027-07-05 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-07-05: softs `closed`, index families `open1` |
| 2027-09-06 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-09-06: softs `closed`, index families `open1` |
| 2027-11-25 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-11-25: softs `closed`, index families `open1` |
| 2027-12-24 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-12-24; all five calendar groups print `closed` |
| 2027-12-27 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-12-27: softs `open`, index families `open1` |

**Gaps, 2027:** **Every 2027 `open1` date carries no instant by construction.** ICE announces those hours “in advance of the respective holiday via Exchange Notices”, and none had issued at retrieval, so the 2027 rows are the calendar's `open`/`closed`/`open1` statuses only. Closed by the 2027 per-holiday Exchange Notices as they are issued. Coverage ends 2028-01-03, the last trade date the 2027 calendar names.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below.

- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf> — the ICE Futures U.S. FANG+ launch notice of 2017-09-26, which states trading begins at the start of trade date 2017-11-08 with 20:00–18:00 ET hours, the exceptional Sunday 18:00 open, and a Pre-Open 30 minutes before each executable session — T1.
- <https://www.ice.com/products/66380320/NYSE-FANG-Index-Future> — the current NYSE FANG+ Index Future product page, which retains the grid and separately publishes the 17:30 Sunday and 19:30 weekday queue starts — T1.
- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — the ICE Futures U.S. *Regular Trading Hours* master table, June-2026 edition, which retains the same grid — T1.
- <https://www.ice.com/trading-hours> — ICE trading hours, the current-schedule monitoring entry point — T1.
- <https://www.ice.com/products> — the ICE product directory — T1.
- <https://www.ice.com/holiday-hours> — ICE holiday hours, the watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The pre-launch era is a sourced closure and both
  revision rows rest on the operator's own launch notice, so the row is
  `Primary` with no carried interval.
- **Scope.** The `iceus` default is the NYSE FANG+ Index Futures family, not a
  venue-wide ICE Futures U.S. clock. ICE has no venue-wide clock at all: the six
  ICE Futures U.S. soft-commodity and index keys are separate identities with
  their own modules, their own dated revisions and their own January-2010 to
  August-2011 baseline gap, which this row does not share because FANG+ did not
  exist before 2017.
- **Queue classification.** The 17:30 Sunday and 19:30 weekday phases are the
  Pre-Open queues the launch notice and product page describe: the platform
  accepts, amends and cancels orders and nothing matches until the 18:00 or
  20:00 open, so they are `order_entry` rather than tradeable extended sessions.
  FANG+ publishes no tradeable phase outside its executable session, so the
  `extended` slice is deliberately empty.
- **The exceptional Sunday session is encoded as a full local-day span.** Equal
  `SessionRule` endpoints encode one complete local-day span, so the Sunday
  session remains continuous through Monday 18:00 rather than closing at
  midnight. A reader changing that rule must preserve the continuity.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_us.rs on 2026-09-12 UTC)

The `iceus` default is the NYSE FANG+ Index futures family, not a venue-wide
clock. ICE launched it for trade date 2017-11-08 with 20:00-18:00 ET hours
and an exceptional Sunday 18:00 open; the current product page and ICE's
June-2026 master table retain that grid. The launch notice starts Pre-Open
30 minutes before each executable session, and the current product page
separately publishes the 17:30 Sunday and 19:30 weekday queue starts.

Equal `SessionRule` endpoints encode one complete local-day span, so the
exceptional Sunday session remains continuous through Monday 18:00.
<https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf>
<https://www.ice.com/products/66380320/NYSE-FANG-Index-Future>
<https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf>

ORDER ENTRY, NOT TRADING. The 17:30 Sunday and 19:30 weekday phases are the
Pre-Open queues the launch notice and product page describe: the platform
accepts, amends and cancels orders for the coming session, and nothing
matches until the 18:00 / 20:00 open. They are therefore classified as
order-entry phases rather than tradeable extended sessions. FANG publishes no
tradeable phase outside its executable session, so the extended slice is
empty.

The launch notice says trading began at the start of trade date 2017-11-08;
its 20:00 prior-day trading rule and 30-minute Pre-Open therefore pin the
first order-entry phase to Tuesday 2017-11-07 at 19:30 ET. This one-evening
profile avoids pretending the product accepted orders earlier that day.

Same Pre-Open queue as the current profile, so the same classification: this
is the launch evening's order-entry phase, not a tradeable session.
