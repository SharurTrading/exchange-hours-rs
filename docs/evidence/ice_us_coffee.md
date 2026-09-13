<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_coffee` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_coffee.rs`](../../src/calendar/schedules/futures/us/ice_coffee.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Coffee "C" (`KC`) only. Current 04:15–13:30 NY grid sourced, with dated 2014-02-03 and 2018-10-08 revisions. Partial: the 2026-08-31 review superseded the earlier caveat — the AUGUST 2011 and JANUARY 2, 2013 editions of ICE's own Regular Trading Hours master table both print the pre-2014 grid outright, so it is stated by dated primary documents rather than corroborated. The residual gap is January 2010 to August 2011, for which no edition of the master table survives in the archive. The 2026-09-01 review established why that interval cannot be sourced: **ICE Futures U.S. sets these hours administratively, not by rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision at all, and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is August 2011. The gap is therefore bounded by document availability rather than by an unfinished search, and the August-2011 carry-back is the terminal answer unless an earlier edition surfaces.

## Revision rows

- 2014-02-03 — T1 — ICE ExNot 012714 hours — grid becomes 04:15-13:30 NY.
- 2018-10-08 — T1 — ICE PCPO notice 20180920 — post-close pre-open added, 14:00-18:00 NY.

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
| 2026-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-01, “New Year's Day” |
| 2026-01-19 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-19, “Martin Luther King Day” |
| 2026-02-16 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-02-16, “Presidents Day” |
| 2026-04-03 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Fri, Apr 3 |
| 2026-04-06 | late open | `Sugar No. 11, Coffee “C” and Cocoa - late open at 7:30 am. Regular Hours for all other contracts.` — 07:30 ET | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Mon, Apr 6; the family's normal open is earlier that morning, so the delay lands on the trade date itself |
| 2026-05-25 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25 |
| 2026-06-19 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19 |
| 2026-07-03 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3 |
| 2026-09-07 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7 |
| 2026-11-26 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26, “Thanksgiving Day” |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25, “Christmas Day” |

**Gaps, 2026:** **No 2026 MLK or Presidents Day Exchange Notice exists.** The ICE notices listing reaches back only to February 2026, and a Wayback CDX prefix enumeration of `ice.com/publicdocs/futures_us/exchange_notices*` for 2025-2027 (113 distinct URLs, saved as `ifus_notices_cdx_2025-2027.json`) holds neither; it holds the 2025 equivalents, which must not be read across years. So the gap is tested, not assumed. **The late-2026 notices had not issued at retrieval:** Thanksgiving 2026-11-26, Christmas Eve 2026-12-24, Boxing Day 2026-12-28 — and note that 2026-11-27 and 2026-12-24 are not holidays on the calendar at all, so their customary early closes exist only in notices that did not yet exist and this table therefore carries no row for either. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice.

**Interpretive steps, 2026:** ICE's calendars key one line per product group, not per contract, and the group “Cocoa, Coffee “C”®, Coffee "C"® Metric, Cotton No 2®, FCOJ, Sugar No. 11 and No. 16 Contracts” is the line this family sits on; the per-holiday notices restate the same set as “Sugar No. 11® and 16, Coffee “C”®, Coffee "C"® Metric, Cotton No. 2®, Cocoa and FCOJ”. Where a notice names a subset — Easter Monday's “Sugar No. 11, Coffee “C” and Cocoa”, and 2026-07-06's “Cotton” — only the named families take the row and the notice's own “Regular Hours for all other contracts” is the positive statement of normality for the rest. 2026-10-12 is normal for the same reason. Each family's session runs inside one New York civil day, so the conversion from ICE's civil date to the crate's trade date is the identity.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-01 |
| 2027-01-18 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-18 |
| 2027-02-15 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-02-15 |
| 2027-03-26 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-03-26 |
| 2027-05-31 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-05-31 |
| 2027-06-18 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-06-18 |
| 2027-07-05 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-07-05 |
| 2027-09-06 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-09-06 |
| 2027-11-25 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-11-25 |
| 2027-12-24 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-12-24 |

**Gaps, 2027:** **Every 2027 `open1` date carries no instant by construction.** ICE announces those hours “in advance of the respective holiday via Exchange Notices”, and none had issued at retrieval, so the 2027 rows are the calendar's `open`/`closed`/`open1` statuses only. Closed by the 2027 per-holiday Exchange Notices as they are issued. Coverage ends 2028-01-03, the last trade date the 2027 calendar names. Its last two dates are audited normal on the calendar itself: 2027-12-31 is not on ICE's 2027 calendar at all, and on 2028-01-03 — New Year's Day observed, since 2028-01-01 is a Saturday — every product group a crate identity routes to prints `open` and only Canola, which no crate identity models, prints `closed`; so neither date carries a row.

## Sources

Row review: 2026-08-23 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Futures U.S. Regular Trading Hours master table, live edition — the current grid.
- <https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Regular Trading Hours master table, AUGUST 2011 edition — captured 2011-12-12, the earliest surviving edition. **Re-read and enumerated exhaustively at the 2026-08-31 targeted review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Regular Trading Hours master table, JANUARY 2, 2013 edition — captured 2013-01-22. **Re-read and enumerated exhaustively at the 2026-08-31 targeted review**, which is later than this row's review date and governs for this source.
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf> — ICE Futures U.S. notice, 20 September 2018, extension of the pre-open order entry session (PCPO).
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf> — ICE Futures U.S. Exchange Notice, Changes to Daily Trading Hours, effective trade date 2014-02-03.
- <https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf> — ICE Futures U.S. Rulebook chapter 4 (Trading) — Rule 4.22(a) confines the Pre-Trading Session to Limit order entry. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf> — ICE Futures U.S. Rulebook chapter 11 (Sugar No. 11), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf> — ICE Futures U.S. Rulebook chapter 8 (Coffee), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf> — ICE Futures U.S. Rulebook chapter 9 (Cocoa), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://www.ice.com/products/15/Coffee-C-Futures> — ICE Coffee "C" product page — the current grid and the pre-open footnote.

## Gaps and residual risks

- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **horizon precision** — the AUGUST 2011 edition of the master table is month-dated by ICE and carries no day, so the ledger horizon is written as `2011-08-01`, the first day of the edition's own month. No ICE document states a day-level date for that edition.
- **order-entry** — the Friday-evening leg of the 20:00 pre-open is omitted, for the reason the sugar row records: a wrapping rule cannot carry a Friday evening through the weekend to Monday's session.
- **superseded caveat, kept for the audit trail** — the module's original caveat said the January 2014 notice prints only the new grid and never the times it replaced. The 2026-08-31 review superseded that: both dated master-table editions read "Coffee \"C\"(R)   3:30 - 14:00", so the pre-2014 grid is stated by primary ICE documents at two dated points.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_coffee.rs on 2026-09-12 UTC)

Coffee "C" runs one same-day executable session; the ICE master hours table
carries no footnote marker on its row, so nothing commences on the previous
calendar evening. Order entry is a separate, non-matching phase.

The master table lists the row as "Coffee "C" and Coffee "C" Metric" on a
single line, so both instruments share this grid.

The 20:00 pre-open is modelled Monday-Thursday. ICE runs it "on the prior
Exchange business day", so a Monday trade date is fed by the preceding Friday
evening. That leg is not expressible here: a wrapping SessionRule always
wraps into the NEXT local day, so a Friday rule would assert order entry on
Saturday morning rather than carrying through to Monday. The Friday PCPO is
unaffected and is modelled MON_FRI, because it opens and closes inside one
local day. The omission is a limit of the normal-week rule model, not a claim
that ICE closes order entry over the weekend.

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/15/Coffee-C-Futures

---

Two order-entry-only phases: the post-close pre-open ("PCPO") beginning 30
minutes after the 13:30 close, and the regular pre-open from 20:00 running to
the next morning's open.

Both are classified order_entry, not extended: nothing matches in either. The
2018 notice creating the PCPO calls it an extension of the "pre-open order
entry session" and kills Day orders entered in it at its end, and the
pre-open only accepts orders ahead of the Opening Match at the open itself.
Coffee "C" publishes no tradeable phase outside its executable session, so
the extended slice is empty.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf

---

Baseline before 2014-02-03: open 03:30 NY, close 14:00 NY.

Sourcing caveat, stated plainly: the January 2014 notice prints only the NEW
grid, marking in bold which of those figures moved. It never prints the times
it replaced.


2026-09-01: WHY THE 2010-2011 INTERVAL CANNOT BE SOURCED. ICE Futures U.S.
sets these hours administratively, not by rule. Its product rulebook chapters
- Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured
December 2011 - contain no hours provision at all, and chapter 4 is
trade-practice rules. No SEC or CFTC filing therefore fixes an ICE Futures
U.S. trading hour, and the master hours table is the only source; its earliest
surviving edition is August 2011. This interval is bounded by document
availability, not by an unfinished search, so the carry-back above is the
terminal answer unless an earlier edition surfaces.
https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf
https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf
https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf
SOURCING CAVEAT SUPERSEDED 2026-08-31. The pre-2014 grid is no longer merely
corroborated: two dated editions of ICE's own "ICE Futures U.S. Regular
Trading Hours" master table print it outright. The AUGUST 2011 edition and
the JANUARY 2, 2013 edition both read "Coffee "C"(R)   3:30 - 14:00", so the
03:30 open and 14:00 close are stated by primary ICE documents at two dated
points spanning up to the 2014-02-03 change. Because no primary document
dates a cutover earlier than 2014-02-03 inside the modelled window, this grid
is still carried back as the baseline rather than inventing an earlier
revision; the residual gap is January 2010 to August 2011, for which no
edition of the master table survives in the archive.
Dated editions of ICE's own master table, official origin
https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
delivered via
https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf

The pre-open likewise runs from 20:00 on the prior Exchange business day; only
its end differs, tracking the 03:30 open of this era. It is order entry, not
trading, so it sits in the order_entry slice.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf

---

2014-02-03: "Effective with the start of trading for trade date Monday,
  February 3, 2014, the Exchange will implement changes to daily trading hours
  for Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and Sugar No. 16 futures
  and option contracts. ... Coffee "C" 4:15 13:30"
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
  order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
  and Sugar No. 16 futures contracts will be enhanced by the addition of a new
  post-close pre-open ("PCPO") session that will start at 30 minutes after the
  end of trading for the contract and end at 6:00 pm on the Exchange business
  day prior to each trading day."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
Evidence: docs/evidence/ice_us_coffee.md
