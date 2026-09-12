<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_cotton` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_cotton.rs`](../../src/calendar/schedules/futures/us/ice_cotton.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Cotton No. 2 (`CT`) only. Current 21:00→14:20 NY wrap sourced, with dated 2014-02-03 and 2018-10-08 revisions. Partial because no ICE document names Sunday for Cotton. The 2026-08-31 review strengthened the basis for that omission from silence to contrast: both the AUGUST 2011 and JANUARY 2, 2013 master tables mark Cotton "21:00*" where `*` is "Trading commences on previous business day", while the Grains, Russell, USDX and currency rows carry `**` ("…and on Sunday evenings only trading commences at 18:00") and the energy rows `***`. ICE drew the Sunday distinction explicitly at two dated points and did not extend it to Cotton, so the Sunday evening open and its pre-open stay omitted. The tradeable week runs Monday 21:00 through Friday 14:20; the sourced Friday 14:50–18:00 post-close pre-open stays as the week's final order-entry window, feeding Monday's session. Its January 2010 to August 2011 baseline carries the same document-availability bound as the other ICE Futures U.S. rows: the 2026-09-01 review found no hours provision in any product rulebook chapter, so these hours are set administratively and the master hours table — earliest surviving edition August 2011 — is the only source.

## Revision rows

- 2014-02-03 — T1 — ICE ExNot 012714 hours — daily close moves from 14:30 to 14:20 NY; the 21:00 open on the previous day is unchanged.
- 2018-10-08 — T1 — ICE PCPO notice 20180920 — post-close pre-open added, 14:50-18:00 NY.

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
| 2026-05-25 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25 |
| 2026-06-19 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19 |
| 2026-07-03 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3 |
| 2026-07-06 | late open | `Late Open for Cotton: 8:00 am / Regular open times for all other contracts, and Regular close times for all contracts` — 08:00 ET | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Mon, July 6 |
| 2026-09-07 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7 |
| 2026-11-26 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26, “Thanksgiving Day” |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25, “Christmas Day” |

**Gaps, 2026:** **No 2026 MLK or Presidents Day Exchange Notice exists.** The ICE notices listing reaches back only to February 2026, and a Wayback CDX prefix enumeration of `ice.com/publicdocs/futures_us/exchange_notices*` for 2025-2027 (113 distinct URLs, saved as `ifus_notices_cdx_2025-2027.json`) holds neither; it holds the 2025 equivalents, which must not be read across years. So the gap is tested, not assumed. **The late-2026 notices had not issued at retrieval:** Thanksgiving 2026-11-26, Christmas Eve 2026-12-24, Boxing Day 2026-12-28 — and note that 2026-11-27 and 2026-12-24 are not holidays on the calendar at all, so their customary early closes exist only in notices that did not yet exist and this table therefore carries no row for either. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice.

**Interpretive steps, 2026:** ICE's calendars key one line per product group, not per contract, and the group “Cocoa, Coffee “C”®, Coffee "C"® Metric, Cotton No 2®, FCOJ, Sugar No. 11 and No. 16 Contracts” is the line this family sits on; the per-holiday notices restate the same set as “Sugar No. 11® and 16, Coffee “C”®, Coffee "C"® Metric, Cotton No. 2®, Cocoa and FCOJ”. Where a notice names a subset — Easter Monday's “Sugar No. 11, Coffee “C” and Cocoa”, and 2026-07-06's “Cotton” — only the named families take the row and the notice's own “Regular Hours for all other contracts” is the positive statement of normality for the rest. 2026-10-12 is normal for the same reason. Cotton's session does **not** run inside one New York civil day — it opens at 21:00 NY on D-1 and closes at 14:20 NY on D — but ICE names the civil date on which the affected session closes, and that closing date is the crate's trade date, so the conversion from ICE's civil date to the crate's trade date is still the identity. Cotton is **not** part of the 2026-04-06 late open. Its own 2026-07-06 late open is recorded because ICE states it, but it changes no answer: the crate's Cotton grid opens for trade date D at 21:00 NY on D-1 and names no Sunday, so there is no modelled trade date 2026-07-06 for the row to clip. That limitation is the profile's, recorded under **Gaps and residual risks**, not the holiday table's.

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

**Gaps, 2027:** **Every 2027 `open1` date carries no instant by construction.** ICE announces those hours “in advance of the respective holiday via Exchange Notices”, and none had issued at retrieval, so the 2027 rows are the calendar's `open`/`closed`/`open1` statuses only. Closed by the 2027 per-holiday Exchange Notices as they are issued. Coverage ends 2028-01-03, the last trade date the 2027 calendar names.

## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
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
- <https://www.ice.com/products/254/cotton-no-2-futures> — ICE Cotton No. 2 product page — 21:00 open, 14:20 close, `closeNextDay` true, 19:30 pre-open.
- <https://www.ice.com/publicdocs/rulebooks/futures_us/10_Cotton.pdf> — ICE Futures U.S. Rulebook chapter 10 (Cotton) — live edition.
- <https://web.archive.org/web/20111120214154id_/https://www.theice.com/publicdocs/rulebooks/futures_us/10_Cotton.pdf> — ICE Futures U.S. Rulebook chapter 10 (Cotton), captured November 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2021LaborDay_Holiday_20210713.pdf> — ICE 2021 Labor Day holiday notice — "Mon, Sep 6 Closed / Tue, Sep 7 Regular Hours".
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_GoodFridayHoliday20250210.pdf> — ICE 2025 Good Friday holiday notice — Cotton "Regular Hours" on the Monday.
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf> — ICE 2025 Christmas holiday notice.

## Gaps and residual risks

- **executable** — the Sunday evening open and its pre-open are omitted, so the documented weekend boundary is Friday 14:20 to Monday 19:30 NY. A Sunday 21:00 open is the only reading consistent with the holiday notices, but no ICE sentence states it, and under LAW-PRIMARY-SOURCES an unasserted phase is omitted. The omission is positive evidence rather than mere silence: ICE marks Cotton "21:00*" at two dated points while the grains, Russell, USDX and currency rows carry "**" ("...and on Sunday evenings only trading commences at 18:00") and the energy rows "***". Closing condition: an ICE document naming Sunday for Cotton No. 2. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **horizon precision** — the AUGUST 2011 edition of the master table is month-dated by ICE and carries no day, so the ledger horizon is written as `2011-08-01`, the first day of the edition's own month. No ICE document states a day-level date for that edition.
- **order-entry** — no primary ICE document inside the modelled window states a Cotton pre-open time earlier than the 19:30 on the product page and in the 2018 notice table, so the 19:30-21:00 pre-open is carried back unchanged rather than given an invented cutover.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_cotton.rs on 2026-09-12 UTC)

Cotton No. 2 is a wrapping contract: the session for trade date D commences at
21:00 NY on calendar day D-1 and closes at 14:20 NY on D. The ICE master hours
table states the row verbatim as "Cotton No. 2(R)   21:00* - 14:20" with the
footnote "*Trading commences on previous business day."; the product page
carries the same 21:00 open, 14:20 close and `closeNextDay` true on
America/New_York. Rulebook Rule 4.25(c)(ii) puts the last-trading-day close at
the same clock ("for Cotton No. 2 Futures at 2:20 PM") and Rule 4.25(b)(iii)
puts the daily settlement window at 14:14 - 14:15, so 14:20 is the session end
on every trade date, not just ordinary ones.

SUNDAY EVENING — A KNOWN GAP, NOT A MODELLED PHASE. The master-table row
"*Trading commences on previous business day." pins Monday-Thursday
evening opens: each of those evenings is the previous business day of the
next trade date. No ICE primary document names Sunday in connection with
Cotton No. 2 — the product page carries no day names, and the master
table's explicit Sunday footnotes ("**... on Sunday evenings only trading
commences at 18:00", "*** ... 17:50") belong to other products. A Sunday
21:00 open is the only reading consistent with the holiday notices (the
2021 Labor Day notice runs "Mon, Sep 6 Closed / Tue, Sep 7 Regular Hours",
and the 2025 Good Friday notice gives Cotton "Regular Hours" on the Monday
while the morning-opening softs take a late open), but that reading is
assembled from indirect material rather than stated by any ICE sentence.

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
https://web.archive.org/web/20111120214154id_/https://www.theice.com/publicdocs/rulebooks/futures_us/10_Cotton.pdf
Confirmed independently on 2026-08-31: the JANUARY 2, 2013 edition of the
master table repeats the same contrast - "Cotton No. 2(R)  21:00* - 14:30"
against "Grains and Oilseeds", "Russell Index", "USDX(R)" and the currency
rows all carrying "**", and the energy rows "***". ICE therefore drew the
Sunday distinction explicitly at two independent dated points and did not
extend it to Cotton, which is positive evidence for the omission rather than
mere silence.
Dated editions of ICE's own master table, official origin
https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
delivered via
https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
Under LAW-PRIMARY-SOURCES an unasserted phase is omitted, so the Sunday
evening open and its pre-open are modelled as closed; the documented
weekend boundary is therefore Friday 14:20 to Monday 19:30 NY.

WEEKEND WRAP: the tradeable week runs Monday 21:00 open through Friday
14:20 close, with Monday-Thursday 19:30 pre-opens. There is no
Friday-evening open, because a Friday 21:00 open would belong to a
Saturday trade date, which does not exist. The Friday 14:50-18:00 PCPO is
retained: the product-page footnote and the 2018 notice's worked example
state it on the prior Exchange business day, so it remains the week's
final order-entry window, feeding Monday's session — orders accepted
there wait for the modelled Monday 21:00 open instead of the unsourced
Sunday one.

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/254/cotton-no-2-futures
https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf
https://www.ice.com/publicdocs/rulebooks/futures_us/10_Cotton.pdf
https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2021LaborDay_Holiday_20210713.pdf
https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_GoodFridayHoliday20250210.pdf
https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf

---

Two order-entry-only phases, neither of them executable.

The regular Pre-Open / Pre-Trading Session runs 19:30 - 21:00 NY immediately
before each open, so it carries the same opening-day mask as the executable
session. The product page gives Pre-Open "7:30 PM" / "19:30",
and the 2018 PCPO notice's own table lists Cotton No. 2 with Pre-Open Start
7:30 PM and End 9:00 PM. Only Limit orders are accepted (Rule 4.22(a):
"Prior to the opening of a trading session for an Exchange Commodity
Contract, there will be a Pre-Trading Session designated by the Exchange
during which time only Limit orders may be entered."), with an Opening Match
uncrossing between the Pre-Open and the open (Rules 4.22(b), 4.23).

The Post-Close Pre-Open ("PCPO") runs 14:50 - 18:00 NY, 30 minutes after the
14:20 close, "on the prior Exchange business day" - product page footnote
verbatim: "**In addition to the Pre-Open start time shown above, there will be
a Post-Close Pre-Open order entry session from 2:50 pm to 6:00 pm NY time on
the prior Exchange business day." Every Monday-Friday business day is the
prior business day of some trading day, so the mask is MON_FRI. The Friday leg
is stated rather than inferred: the 2018 notice's own worked example places
the PCPO for trade date Monday 8 October 2018 on Friday 5 October, making the
Friday PCPO the last order-entry window of the week. The PCPO accepts GTC /
GTD / GTD&T entry and amendment only; Day orders entered in it are killed at
its end.

https://www.ice.com/products/254/cotton-no-2-futures
https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
CLASSIFICATION. Both phases are order_entry, not extended. Rule 4.22(a)
restricts the Pre-Trading Session to Limit order entry and puts the Opening
Match at the open, so no trade prints between 19:30 and 21:00; the PCPO is an
order entry/amendment window whose own Day orders are killed at its end, so
no trade prints between 14:50 and 18:00 either. Cotton No. 2 publishes no
tradeable phase outside its executable session, which leaves the extended
slice empty.

https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf

---

Baseline before 2014-02-03. The January 2014 notice moved only the close, from
14:30 to 14:20 NY, and left the 21:00 open on the previous day untouched, so
the baseline regular grid is the current one with the older close.

The notice does not address order entry at all, and no primary ICE document
inside the modelled window states a pre-open time for Cotton earlier than the
19:30 on the product page and in the 2018 notice table. The 19:30 - 21:00
Pre-Open is therefore carried back unchanged rather than inventing an earlier
cutover for it; if ICE moved it at some point before 2014, no primary source
dates that move.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf

---

2014-02-03: ICE Futures U.S. Exchange Notice, originally issued 6 January
  2014, "Changes to Daily Trading Hours": "Effective with the start of trading
  for trade date Monday, February 3, 2014, the Exchange will implement changes
  to daily trading hours for Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and
  Sugar No. 16 futures and options contracts." Cotton's row moves the close
  from 14:30 to 14:20; the 21:00 open on the previous day is unchanged.
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
2018-10-08: ICE Futures U.S. Notice, 20 September 2018, "EXTENSION OF THE
  PRE-OPEN ORDER ENTRY SESSION FOR COFFEE "C", COTTON NO. 2, COCOA, FCOJ,
  SUGAR NO. 11 AND SUGAR NO. 16 FUTURES CONTRACTS": "Commencing for trade date
  Monday, October 8, 2018, the pre-open order entry session for Coffee "C",
  Cotton No. 2, Cocoa, FCOJ, Sugar No. 11 and Sugar No. 16 futures contracts
  will be enhanced by the addition of a new post-close pre-open ("PCPO")
  session that will start at 30 minutes after the end of trading for the
  contract and end at 6:00 pm on the Exchange business day prior to each
  trading day." For Cotton the 14:20 close puts that start at 14:50.
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
Evidence: docs/evidence/ice_us_cotton.md
