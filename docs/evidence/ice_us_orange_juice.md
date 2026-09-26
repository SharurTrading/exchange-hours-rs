<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_orange_juice` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_fcoj.rs`](../../src/calendar/schedules/futures/us/ice_fcoj.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. FCOJ-A (`OJ`) only. Current 08:00–14:00 NY grid and both order-entry windows sourced, with the dated 2018-10-08 PCPO revision. Partial because no primary source dates any earlier change inside the modelled window; the 2026-08-31 review made that positive rather than inferential by finding "FCOJ-A   8:00 - 14:00" unchanged in both the AUGUST 2011 and JANUARY 2, 2013 master-table editions. January 2010 to August 2011 stays unsourced. The 2026-09-01 review established why that interval cannot be sourced: **ICE Futures U.S. sets these hours administratively, not by rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision at all, and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is August 2011. The gap is therefore bounded by document availability rather than by an unfinished search, and the August-2011 carry-back is the terminal answer unless an earlier edition surfaces.

## Revision rows

- 2018-10-08 — T1 — ICE PCPO notice 20180920 — post-close pre-open added, 14:30-18:00 NY.

## Holidays

**Coverage:** 2025-01-01..2028-01-03 (inclusive trade dates). Tier: T1 throughout.

### Documents

Every artifact below is saved in the research store, and each row's own section quotes the bytes it keys.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `IFUS-CAL-2025` | 2025-01-01 .. 2026-01-01 | <https://web.archive.org/web/20250523135551id_/https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> | Wayback `id_` replay of capture `20250523135551`, 2026-09-26 07:16:50 UTC | T1 | `0add2b10e7d6cb2a35b727db654e4ea87ed30970ec637f53e62dd044f553d049` |
| `IFUS-NOTICE-2025-MLK` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250213182737id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025MLKDay_Holiday_20241203.pdf> | Wayback `id_` replay of capture `20250213182737`, 2026-09-26 07:16:29 UTC | T1 | `d6e7a3c0672fee4364cd610c88e861f5175de1494a7bebd1d8df43a790ef0364` |
| `IFUS-NOTICE-2025-PRESIDENTS` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250225071405id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025PresidentsDay_Holiday.pdf> | Wayback `id_` replay of capture `20250225071405`, 2026-09-26 07:16:30 UTC | T1 | `9c5946b286419b3670147e35c8dd752634b050cccd26fc564bccbf3f3fdf57b1` |
| `IFUS-NOTICE-2025-GOODFRIDAY` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250409083118id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_GoodFridayHoliday20250210.pdf> | Wayback `id_` replay of capture `20250409083118`, 2026-09-26 07:16:32 UTC | T1 | `18342cf005b428e18d15348d660126de3c9decb6b2ff93ffbc2fedefa116596c` |
| `IFUS-NOTICE-2025-MEMORIAL` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Memorial_DayHoliday_20250303.pdf> | retrieved live from the operator, 2026-09-26 07:21:21 UTC | T1 | `b06fbcb489331e7b9a2e4f7d2b092d63459771b1fb939e1c7ed6cd4a1b782fc7` |
| `IFUS-NOTICE-2025-JUNETEENTH` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250508170956id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_JuneteenthHoliday_20250424.pdf> | Wayback `id_` replay of capture `20250508170956`, 2026-09-26 07:16:34 UTC | T1 | `851ba17addf37f89007be95f8f3943ca0017204873bcf94522ad96cca68478d7` |
| `IFUS-NOTICE-2025-LABORDAY` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_LaborDayHoliday_20250715.pdf> | retrieved live from the operator, 2026-09-26 07:21:20 UTC | T1 | `9768ce292b4c55c8171285444059a445a088e9ebc29058354ffbbf252373e036` |
| `IFUS-NOTICE-2025-THANKSGIVING` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Thanksgiving_Holiday_updated.pdf> | retrieved live from the operator, 2026-09-26 07:21:20 UTC | T1 | `aa643597896431140300de04f60ea68c725dd1ad90e66f22d2cd5a9fe410d1db` |
| `IFUS-CAL-2026` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice_2026_Holiday_Calendar_20260609.pdf> | retrieved 2026-09-12 07:47 UTC | T1 | `b9723d9afa1c20a0792b628de36c7d3726add58ea3d94972a1e7cefd3e50bf4e` |
| `IFUS-CAL-2027` | 2027-01-01 .. 2028-01-03 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice-2027_Holiday_Calendar_20260604.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d2e39a2db2a26e0578ad0d09b36f51dfdad8502635020fe79d66303c09c1a040` |
| `IFUS-NOTICE-2026-GOODFRIDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_GoodFridayHoliday_20260202.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `0a38abd0c9c8b0f904e9701999ac56466610c46437bdfe0109f88e67da35d181` |
| `IFUS-NOTICE-2026-MEMORIAL` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Memorial_DayHoliday_20260310.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `3dfe2a49babece024ce3cd91948e4fb27740abe31282fd9121097bd8433298f9` |
| `IFUS-NOTICE-2026-JUNETEENTH` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_JuneteenthHoliday_20260320.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `34a21ef6facf3dafd72d444ea352fc60e9b2dbf4fc674c1f4f8c199936169edd` |
| `IFUS-NOTICE-2026-INDEPENDENCE` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US__IndDayHoliday_.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `9de2852ed397d403cf8061358d98839954dbe861bf349fb91f57f95910983ea6` |
| `IFUS-NOTICE-2026-LABORDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_LaborDayHoliday_20260708.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d383fe9ae20065f4de6825bff68f13d9cdd8233ea57de0f40644af92439b88f4` |

**Notes on the documents.**
- `IFUS-CAL-2026` resolves to the Exchange-Notice copy, whose labels the rows follow; the Holiday-Hours-hub copy <https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> (retrieved 2026-09-12 04:19 UTC, sha256 `da97503545a3fb7607948367d30896685e220b36abed24aa02bbe1a21acb2817`) carries the earlier product-group headers with every `open` / `closed` / `open1` status cell identical.
- `IFUS-CAL-2027` is an unconditional published future, which LAW-NO-FABRICATED-DATES permits encoding ahead of its effective days.
- The 2026 equity-index companion notice of the same date (sha256 `6f07e49c36c70ef0c81cf028c3c7f539339b03092c81b22cf9c583f9f0c6b9e2`) is last-trade-day and final-settlement content only and keys nothing here (LAW-SESSION-NOT-EXPIRY).
- The research store holds the 2025 artifacts under `holidays/raw/iceus-2025-2027/`, with each file's URL, UTC retrieval time, sha256 and byte count in its `INDEX.md` and `INVENTORY.tsv`; the 2026-2027 artifacts are under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/` and `…-fix/`.

### 2025

The 2025 block is the `July 5, 2024` — `2025 Trading Holiday Calendar`'s own day-level spine, ICE's per-holiday Exchange Notices for the holidays it issued one for, the **revised** Thanksgiving notice that adds Friday 2025-11-28, and the two London-bank-holiday “Delayed Opens” notices that carry dates the calendar does not list. Every row below was read from the cited artifact's own bytes.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-01-01, “New Year’s Day”; the softs column prints `closed` |
| 2025-01-20 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-MLK` | T1 | notice date Mon, Jan 20, group line “Sugar No. 11® and 16, Coffee “C” ®, Cotton No. 2®, Cocoa and FCOJ Contracts” |
| 2025-02-17 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | notice date Mon, Feb 17; the same notice's group line also names Canola, which no crate identity models |
| 2025-04-18 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Fri, Apr 18, the shared softs/Canola group line |
| 2025-05-26 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-MEMORIAL` | T1 | notice date Mon, May 26, the softs group line |
| 2025-06-19 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | notice date Thu, June 19 |
| 2025-07-04 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-07-04, “Independence Day”; the softs column prints `closed` |
| 2025-09-01 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-LABORDAY` | T1 | notice date Mon, Sep 1 |
| 2025-11-27 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Thu, Nov 27 |
| 2025-11-28 | early close | `Early Close for Cotton and FCOJ - 1:30 pm` — 13:30 ET | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Fri, Nov 28; the same cell prints `Regular Hours for Sugar, Coffee and Cocoa` |
| 2025-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-25, “Christmas Day”; the softs column prints `closed` |

**Gaps, 2025:** **The 2025 Independence Day and Christmas / Boxing Day notices were not retrieved.** Neither is in the Wayback Machine or Common Crawl, 44 live filename candidates returned 404 before the operator's host began rate-limiting, and the notices listing renders client-side so its archived copies name no notice. Their closing condition is page 2+ of the paginated report at <https://www.ice.com/futures-us/notices> (report id `futures_us_exchange_notice`), or the exact PDF URL; tracked as #168. FCOJ's calendar column is unaffected — `closed` on 2025-07-04 and 2025-12-25, plain `open` on 2025-12-26 — but those two notices are the only documents that could state an FCOJ arrangement on 2025-07-03 or 2025-12-24, so no claim of a *complete* 2025 audit is made for them here. **Canola** and **`Daily Gold and Silver`** are product groups on ICE's calendar with no crate identity in this block. **2025-12-31 is normal, not unsourced:** the 2026 New Year's notice's `Wed, Dec 31` column prints `Regular Hours` for the group that names FCOJ.

**Interpretive steps, 2025:** FCOJ shares the softs group line on every 2025 notice, and it is one of the two families the revised Thanksgiving notice names on Friday 2025-11-28: `Early Close for Cotton and FCOJ - 1:30 pm`. It takes no Easter Monday row, because the Good Friday notice gives that late open to “Sugar No. 11, Coffee “C” and Cocoa” alone and adds “Regular Hours for all other contracts”. FCOJ runs one same-day 08:00-14:00 New York session, so the conversion from ICE's civil date to the crate's trade date is the identity.

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
| 2026-09-07 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7 |
| 2026-11-26 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26, “Thanksgiving Day” |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25, “Christmas Day” |

**Gaps, 2026:** **No 2026 MLK or Presidents Day Exchange Notice exists.** The ICE notices listing reaches back only to February 2026, and a Wayback CDX prefix enumeration of `ice.com/publicdocs/futures_us/exchange_notices*` for 2025-2027 (113 distinct URLs, saved as `ifus_notices_cdx_2025-2027.json`) holds neither; it holds the 2025 equivalents, which must not be read across years. So the gap is tested, not assumed. **The late-2026 notices had not issued at retrieval:** Thanksgiving 2026-11-26, Christmas Eve 2026-12-24, Boxing Day 2026-12-28 — and note that 2026-11-27 and 2026-12-24 are not holidays on the calendar at all, so their customary early closes exist only in notices that did not yet exist and this table therefore carries no row for either. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice.

**Interpretive steps, 2026:** ICE's calendars key one line per product group, not per contract, and the group “Cocoa, Coffee “C”®, Coffee "C"® Metric, Cotton No 2®, FCOJ, Sugar No. 11 and No. 16 Contracts” is the line this family sits on; the per-holiday notices restate the same set as “Sugar No. 11® and 16, Coffee “C”®, Coffee "C"® Metric, Cotton No. 2®, Cocoa and FCOJ”. Where a notice names a subset — Easter Monday's “Sugar No. 11, Coffee “C” and Cocoa”, and 2026-07-06's “Cotton” — only the named families take the row and the notice's own “Regular Hours for all other contracts” is the positive statement of normality for the rest. 2026-10-12 is normal for the same reason. Each family's session runs inside one New York civil day, so the conversion from ICE's civil date to the crate's trade date is the identity. FCOJ is therefore **not** part of the 2026-04-06 late open and ships no row for it.

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
- <https://www.ice.com/products/30/FCOJ-A-Futures> — ICE FCOJ-A product page — "NEW YORK  8:00 AM - 2:00 PM" and the PCPO footnote.
- <https://web.archive.org/web/20111213010113id_/https://www.theice.com/publicdocs/rulebooks/futures_us/13_FCOJ.pdf> — ICE Futures U.S. Rulebook chapter 13 (FCOJ), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.

## Gaps and residual risks

- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **horizon precision** — the AUGUST 2011 edition of the master table is month-dated by ICE and carries no day, so the ledger horizon is written as `2011-08-01`, the first day of the edition's own month. No ICE document states a day-level date for that edition.
- **order-entry** — the Friday-evening leg of the 20:00 pre-open is omitted, for the reason the sugar row records. The pre-open start itself is only ever stated in undated product-page form ("Pre-Open ... 8:00 PM 20:00"); ICE publishes no notice establishing or moving it, so it is carried in the baseline rather than given a cutover.
- **corroboration** — ICE's 2014 softs hours notice re-tabulated Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and Sugar No. 16 and then stated "Daily trading hours for all other products are unchanged", which excludes FCOJ-A from that change.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_fcoj.rs on 2026-09-12 UTC)

FCOJ-A runs one same-day executable session. The ICE master hours table
carries no footnote marker on the FCOJ-A row - neither the single asterisk
("Trading commences on previous business day") nor the double asterisk that
additionally moves the Sunday open - so nothing commences on the previous
calendar evening and there is no Sunday session. The executable week runs
Monday 08:00 through Friday 14:00 New York.

Master table, verbatim: "FCOJ-A   8:00 - 14:00". Corroborated by the ICE
product page: "NEW YORK  8:00 AM - 2:00 PM  08:00 - 14:00".


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
https://web.archive.org/web/20111213010113id_/https://www.theice.com/publicdocs/rulebooks/futures_us/13_FCOJ.pdf
2026-08-31: the same "FCOJ-A   8:00 - 14:00" row appears in the AUGUST 2011
and JANUARY 2, 2013 editions of the master table, so "no earlier change
inside the modelled window" is now supported by two dated ICE documents
rather than by absence of evidence. The residual gap is January 2010 to
August 2011, for which no edition survives in the archive.
Dated editions of ICE's own master table, official origin
https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
delivered via
https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/30/FCOJ-A-Futures

---

Two order-entry-only phases, neither of which matches: the post-close pre-open
("PCPO") beginning 30 minutes after the 14:00 close and ending at 18:00, and
the regular pre-open from 20:00 running to the next morning's 08:00 open. Both
attach to the Exchange business day prior to the trade date, so the windows
feeding a Tuesday session are Monday 14:30-18:00 and Monday 20:00 onward.

The 20:00 pre-open start is modelled Monday-Thursday only. ICE places it "on
the prior Exchange business day", which for a Monday trade date is the
preceding Friday - a 60-hour order-entry window spanning the weekend. No ICE
document states that order entry stays live across the weekend, so the
Friday-evening leg is deliberately omitted rather than asserted. The PCPO is
modelled Monday-Friday because Friday 14:30-18:00 is an ordinary same-week
window that ICE's own worked example confirms (the PCPO for trade date Monday
8 October 2018 fell on Friday 5 October 2018).

Product page footnote, verbatim: "**In addition to the Pre-Open start time
shown above, there will be a Post-Close Pre-Open order entry session from 2:30
pm to 6:00 pm NY time on the prior Exchange business day."

Both windows are order_entry, not extended: the product page names the PCPO
an "order entry session", and Rule 4.22(a) confines the Pre-Trading Session
to Limit order entry with the Opening Match at the open. FCOJ-A publishes no
tradeable phase outside its 08:00-14:00 executable session, so the extended
slice is empty.

https://www.ice.com/products/30/FCOJ-A-Futures
https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf

---

Baseline before 2018-10-08: the executable session is already the 2026-08-23 review's
08:00-14:00 grid, but the PCPO order-entry window does not exist yet, leaving
the 20:00 pre-open as the only non-executable phase - order entry, like the
current one, so this era carries no extended phase either.

The executable grid is carried back unchanged rather than dated to an earlier
cutover. ICE's 2014 softs hours notice re-tabulated Sugar No. 11, Coffee "C",
Cocoa, Cotton No. 2 and Sugar No. 16 and then stated "Daily trading hours for
all other products are unchanged", which excludes FCOJ-A; no primary ICE
document inside the modelled window states a different FCOJ-A open or close,
so no earlier revision is invented here.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf

---

2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
  order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
  and Sugar No. 16 futures contracts will be enhanced by the addition of a new
  post-close pre-open ("PCPO") session that will start at 30 minutes after the
  end of trading for the contract and end at 6:00 pm on the Exchange business
  day prior to each trading day."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf

FCOJ is named in the notice by contract, so the 14:00 close puts the FCOJ-A
PCPO at 14:30-18:00, matching the 2:30 pm figure the product page footnote
carries at the 2026-08-23 review. The executable session is untouched by this notice.

No other FCOJ-A schedule change inside the modelled window is dated by a
primary ICE source. The pre-open start itself is only ever stated in undated
product-page form ("Pre-Open ... 8:00 PM 20:00"); ICE publishes no notice
establishing or moving it, so it is carried in the baseline rather than given
a cutover.
Evidence: docs/evidence/ice_us_orange_juice.md
