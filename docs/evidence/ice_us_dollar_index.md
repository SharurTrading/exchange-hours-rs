<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_dollar_index` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_usdx.rs`](../../src/calendar/schedules/futures/us/ice_usdx.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. U.S. Dollar Index (`DX`) only. Current 20:00→17:00 NY grid with the Sunday 18:00 open confirmed by three independent primary ICE sources; dated 2011-02-14 close revision cited from an archived ICE notice. Partial because no primary source dates the pre-2011 grid. Confirmed 2026-08-31: ICE's 2007 currencies release states only the 20:00–18:00 ET currency-futures grid and never prints a USDX grid, and the earliest surviving master-table edition (August 2011) postdates the 2011-02-14 revision. The 2026-09-01 review established why that interval cannot be sourced: **ICE Futures U.S. sets these hours administratively, not by rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision at all, and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is August 2011. The gap is therefore bounded by document availability rather than by an unfinished search, and the August-2011 carry-back is the terminal answer unless an earlier edition surfaces.

## Revision rows

- 2011-02-14 — T1 — ICE ExNot 020311 DX hours — the electronic trading day ends at 17:00 NY; the notice states the opens are unchanged.

## Holidays

**Coverage:** 2026-01-01..2028-01-03 (inclusive trade dates). Tier: T1 throughout.

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
| 2026-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-01, “New Year's Day”, `closed` for the Currency / Digital Asset / Stock / SOFR / MSCI Bond / Mortgage Index group |
| 2026-01-19 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-19; the archive holds no 2026 MLK Exchange Notice |
| 2026-02-16 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-02-16; the archive holds no 2026 Presidents Day Exchange Notice |
| 2026-04-03 | late open and early close | `Late Open - 5:00 am / Early Close - 11:15 am, No TAS Trading` — 05:00 and 11:15 ET | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Fri, Apr 3, “U.S. Dollar Index®, Currency Pair, ICE Mortgage, SOFR Index and Digital Asset” bullet |
| 2026-05-25 | early close | `Early Close – 2:30 pm, TAS trading will not be held` — 14:30 ET | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25, “U.S. Dollar Index®, Currency Pair and Digital Asset Contracts” bullet |
| 2026-06-19 | early close | `Early Close – 2:30 pm, TAS trading will not be held` — 14:30 ET | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19, “U.S. Dollar Index®, Currency Pair and Digital Asset Contracts” bullet |
| 2026-07-03 | early close | `Early Close – 2:30 pm, TAS trading will not be held` — 14:30 ET | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3, “U.S. Dollar Index®, Currency Pair and Digital Asset Contracts” bullet |
| 2026-11-26 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26; the 2026 Thanksgiving notice had not issued at retrieval |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25 |
| 2026-12-28 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-28, “Boxing Day”; the 2026 Christmas/New-Year notice had not issued at retrieval |

**Gaps, 2026:** **No 2026 MLK or Presidents Day Exchange Notice exists.** The ICE notices listing reaches back only to February 2026, and a Wayback CDX prefix enumeration of `ice.com/publicdocs/futures_us/exchange_notices*` for 2025-2027 (113 distinct URLs, saved as `ifus_notices_cdx_2025-2027.json`) holds neither; it holds the 2025 equivalents, which must not be read across years. So the gap is tested, not assumed. **The late-2026 notices had not issued at retrieval:** Thanksgiving 2026-11-26, Christmas Eve 2026-12-24, Boxing Day 2026-12-28 — and note that 2026-11-27 and 2026-12-24 are not holidays on the calendar at all, so their customary early closes exist only in notices that did not yet exist and this table therefore carries no row for either. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice.

**Interpretive steps, 2026:** ICE names “U.S. Dollar Index®” in its own bullet on every 2026 notice that gives instants, so no interpretive step carries these rows. The calendars key the family on the group “Currency, Digital Asset, Stock, SOFR, MSCI Bond, and Mortgage Index Contracts”, which is why the `open1` dates are `unsourced` here as well. **2026-09-07 ships no row**: the Labor Day notice prints “Regular Hours, TAS trading will not be held” for this family, and a TAS suspension is a trade-type restriction, not a session boundary (LAW-SESSION-NOT-EXPIRY), so the date is audited normal. **What is not a late open.** Good Friday's 05:00 is a time of day below the family's normal 20:00 first-open time, so the crate reads the late open as 05:00 ET on the trade date itself rather than on the preceding local date; the Thursday-evening leg, which opens at 20:00 ET on Thursday and belongs to the same trade date, falls before that instant and is deleted, which is what ICE states.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-01 |
| 2027-01-18 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-18 |
| 2027-02-15 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-02-15 |
| 2027-03-26 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-03-26; the softs and energy groups print `closed`, this group prints `open1` |
| 2027-05-31 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-05-31 |
| 2027-06-18 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-06-18 |
| 2027-07-05 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-07-05 |
| 2027-09-06 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-09-06 |
| 2027-11-25 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-11-25 |
| 2027-12-24 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-12-24 |
| 2027-12-27 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-12-27 |

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
- <https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures> — ICE USDX product page 194 — "Market opens at 6:00 pm on Sunday for Monday's trade date. Pre-open at 5:30 pm Sunday, 7:30 pm Monday through Thursday.".
- <https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf> — ICE US Dollar Index FAQ — the Friday 17:00 wrap and the 30-minute order-entry window.
- <https://web.archive.org/web/20110222135802/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot020311DXhours.pdf> — ICE Futures U.S. Exchange Notice, "Change To Electronic Trading Hours For IFUS Currency Contracts", 7 February 2011 — retired from ice.com, cited from the archive capture.
- <https://web.archive.org/web/20101224164411id_/https://www.theice.com/publicdocs/futures_us/exchange_notices/Currencies_Electronic_Trade.pdf> — ICE 2007 currencies release — states the currency-futures grid and never prints a USDX grid.
- <https://web.archive.org/web/20111213003348id_/https://www.theice.com/publicdocs/rulebooks/futures_us/15_USDX.pdf> — ICE Futures U.S. Rulebook chapter 15 (USDX), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.

## Gaps and residual risks

- **executable** — no primary source dates the pre-2011 USDX grid. The baseline's 18:00 NY close and its unchanged opens are known only because ICE's 7 February 2011 notice states the state it replaced; ICE's 2007 currencies release mentions the Dollar Index solely as having "began trading electronically in June" and never prints a USDX grid, and the earliest surviving master-table edition (AUGUST 2011) postdates the 2011-02-14 revision. Closing condition: an ICE document dating the pre-2011 grid. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **wording conflict, resolved** — the 2011 notice's parenthetical lists the unchanged weekday opens as "8:00 pm NY time Tuesday through Thursday evenings", which would leave Tuesday's trade date with no open at all. The master-table footnote applies the previous-business-day rule to every trade date, so the weekday evening mask stays Monday through Thursday; no primary source dates any change to which evenings carry an open, so none is encoded.
- **order-entry** — the 30-minute pre-open is carried back unchanged. ICE states it as a standing platform property rather than a dated one: the April 2011 edition of the USDX FAQ already reads "The ICE trading platform is available for order entry thirty minutes before the opening of trading", and the current product page repeats it.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_usdx.rs on 2026-09-12 UTC)

USDX is a near-24-hour contract whose session commences on the evening before
its trade date, with a Sunday evening that opens two hours earlier than the
weekday evenings. ICE's June 2026 master hours table gives the row as
"USDX(R)   20:00** - 17:00", where footnote ** reads "Trading commences on
previous business day and on Sunday evenings only trading commences at
18:00".

The 18:00 Sunday open is stated by three independent primary ICE documents,
so it is modelled as a distinct rule rather than folded into the weekday
evening grid:
  1. the master hours table footnote ** quoted above;
  2. the ICE US Dollar Index FAQ - "Trading ends at 5:00 p.m. ET on Friday
     afternoon. On Sunday evening, trading in the contracts begins at 6:00
     p.m. ET; the trading session that begins on Sunday evening ends at 5:00
     p.m. ET on the following Monday";
  3. product page 194 - "Market opens at 6:00 pm on Sunday for Monday's trade
     date. Pre-open at 5:30 pm Sunday, 7:30 pm Monday through Thursday."

The weekday evening rule stops at Thursday: a Friday 20:00 open would belong
to a Saturday trade date, which does not exist, and the FAQ fixes the weekly
wrap at "5:00 p.m. ET on Friday afternoon".

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures
https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf

---

Order entry only, 30 minutes ahead of each open, and nothing else: USDX has
no post-close pre-open ("PCPO") window. The 2018 PCPO notice that added one
to the ICE softs names only Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar
No. 11 and Sugar No. 16, and neither the USDX product page nor the FAQ
publishes a post-close order-entry phase; the product page's only extended
phase is the Pre-Open column.

Verbatim from the product page: "NEW YORK  8:00 PM - 5:00 PM*  20:00 - 17:00
7:30 PM  19:30" and "Pre-open at 5:30 pm Sunday, 7:30 pm Monday through
Thursday."; the specification block adds "*The trading platform is available
30 minutes before the opening for order entry. Open on Sunday night is 6:00
PM ET; Pre-Open at 5:30 PM ET".

ICE's own wording settles the classification: the platform "is available 30
minutes before the opening for order entry" and nothing matches until the
open, so the Pre-Open goes in order_entry. USDX publishes no tradeable phase
outside its near-24-hour executable session, so the extended slice is empty.

https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures

---

Baseline before 2011-02-14: the same evening opens, closing an hour later at
18:00 NY. The February 2011 notice changed only the close and said so
explicitly - "This change does not affect the start of the electronic trading
day for these products" - so the Sunday 18:00 and weekday-evening 20:00 opens
are carried back rather than reconstructed.

The Sunday rule has equal endpoints here. A SessionRule with `open_ssm ==
close_ssm` encodes exactly one complete local-day span, so Sunday 18:00 runs
continuously to Monday 18:00, which is what the pre-2011 grid described.


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
https://web.archive.org/web/20111213003348id_/https://www.theice.com/publicdocs/rulebooks/futures_us/15_USDX.pdf
2026-08-31: the pre-2011 dating question was re-worked and stays negative.
ICE's 2007 currencies release states only that "Electronic trading hours for
currency futures are from 8:00 p.m. ET through 6:00 p.m. ET each trading
day", and mentions the Dollar Index solely as having "began trading
electronically in June" - it never prints a USDX grid. The earliest surviving
edition of the master table is AUGUST 2011, after the 2011-02-14 close
revision. So no primary source dates the pre-2011 USDX grid, and the gap is
document availability rather than an unfinished search.
https://web.archive.org/web/20101224164411id_/https://www.theice.com/publicdocs/futures_us/exchange_notices/Currencies_Electronic_Trade.pdf

The notice's parenthetical lists the unchanged weekday opens as "8:00 pm NY
time Tuesday through Thursday evenings". That wording would leave Tuesday's
trade date with no open at all, and the master hours table footnote applies
the previous-business-day rule to every trade date, so the weekday evening
mask stays Monday through Thursday. No primary source dates any change to
which evenings carry an open, so none is encoded.

---

The 30-minute pre-open is carried back unchanged. ICE states it as a standing
platform property rather than a dated one - the April 2011 edition of the US
Dollar Index FAQ already reads "The ICE trading platform is available for
order entry thirty minutes before the opening of trading", and the current
product page repeats it. No primary source dates its introduction, so no
cutover is encoded for the order-entry phases; only the close moves in 2011.
The FAQ's wording - "available for order entry thirty minutes before the
opening of trading" - is also why the phase is order_entry in this era, not a
tradeable extended session.

https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf

---

2011-02-14: "Effective with the start of trading for trade date Monday,
  February 14, 2011, the Exchange is implementing a change to electronic
  trading hours for the USDX futures and options contracts, the Euro Index
  futures contract and for all currency pair futures contracts on the ICE
  electronic trading platform: the trading day for these contracts will now
  end at 5:00 pm NY time. This change does not affect the start of the
  electronic trading day for these products (which remains as 6:00 pm NY time
  on Sunday evenings, 8:00 pm NY time Tuesday through Thursday evenings) or
  the settlement window for these products (which remains 2:59 to 3:00 pm)."
  ICE Futures U.S. Exchange Notice, "Change To Electronic Trading Hours For
  IFUS Currency Contracts", February 7, 2011. The notice has been retired
  from ice.com; per the crate's established practice for superseded operator
  documents, the archive capture is cited as-is.
  https://web.archive.org/web/20110222135802/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot020311DXhours.pdf
Evidence: docs/evidence/ice_us_dollar_index.md
