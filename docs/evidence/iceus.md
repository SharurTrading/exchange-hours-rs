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
| `IFUS-NOTICE-2025-LBMA-MAY05` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250429040409id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_ExNotDelayedOpensMay5_20250424.pdf> | Wayback `id_` replay of capture `20250429040409`, 2026-09-26 07:28:02 UTC | T1 | `a737611d7dcc47d5dc3e06e70011e13695a63150b454cc1d23006ded64ab4c30` |
| `IFUS-NOTICE-2025-LBMA-AUG25` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250816054649id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_ExNotDelayedOpensAug25_20250728.pdf> | Wayback `id_` replay of capture `20250816054649`, 2026-09-26 07:28:04 UTC | T1 | `2efe0b39ad99c1a0aa6abfcfa9db2f44bd1157564e73ec7cf6476d5ae65f3cc8` |
| `IFUS-NOTICE-2025-MOMENT-OF-SILENCE` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250207231515id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_ExNot2024MomentOfSilence20241230.pdf> | Wayback `id_` replay of capture `20250207231515`, 2026-09-26 UTC | T1 | `a4dca19c06c46d62886e3379b8db8f95319e94681bd477ac43cac03ce3e002de` |
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
- `IFUS-NOTICE-2025-MOMENT-OF-SILENCE` is the operator's one-page notice of 2024-12-30 for the National Day of Mourning; the research store holds it at `holidays/raw/iceus-2025-2027/extra/ICE_Futures_US_ExNot2024MomentOfSilence20241230.pdf`, whose sha256 and URL its own `INDEX.md` records, and the `pdftotext -layout` twin beside it reproduces every quotation below. It is the artifact #168 recorded as unread; it is read now.
- The 2026 equity-index companion notice of the same date (sha256 `6f07e49c36c70ef0c81cf028c3c7f539339b03092c81b22cf9c583f9f0c6b9e2`) is last-trade-day and final-settlement content only and keys nothing here (LAW-SESSION-NOT-EXPIRY).
- The research store holds the 2025 artifacts under `holidays/raw/iceus-2025-2027/`, with each file's URL, UTC retrieval time, sha256 and byte count in its `INDEX.md` and `INVENTORY.tsv`; the 2026-2027 artifacts are under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/` and `…-fix/`.

### 2025

The 2025 block is the intersection of the five family tables over the `July 5, 2024` — `2025 Trading Holiday Calendar` and the 2025 Exchange Notices, and it is the first year the venue answers for. Three dates ship `Closed`; the other fifteen ship `Unsourced`, because on each of them at least one modelled family states something no other agrees with. Every row below was read from the cited artifact's own bytes, and the `Derived from` cell names each disagreement.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-01-01; every modelled family prints `closed`, so all five tables agree |
| 2025-01-09 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-MOMENT-OF-SILENCE` | T1 | notice date Thu, Jan 9: `Micro NYSE FANG+` (`FNG`) ends at 09:30 NY, the softs and the dollar index keep regular hours — the notice's own “All other contracts will follow regular trading hours and daily settlement window times” |
| 2025-01-20 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-MLK` | T1 | notice date Mon, Jan 20: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-02-17 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | notice date Mon, Feb 17: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-04-18 | closed | `closed` — no instant printed | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Fri, Apr 18: every modelled family prints `Closed` |
| 2025-04-21 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Mon, Apr 21: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2025-05-05 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-LBMA-MAY05` | T1 | London bank holiday: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2025-05-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-MEMORIAL` | T1 | notice date Mon, May 26: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-06-19 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | notice date Thu, June 19: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-07-03 | unsourced | no single cell — the Independence Day notice was not retrieved | `IFUS-CAL-2025` | T1 | no family states the eve, and the only document that would is unretrieved |
| 2025-07-04 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-07-04: softs `closed`, index families `open1`, which defers to the unretrieved notice |
| 2025-08-25 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-LBMA-AUG25` | T1 | London bank holiday: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2025-09-01 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-LABORDAY` | T1 | notice date Mon, Sep 1: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-11-27 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Thu, Nov 27: softs `Closed`, FANG+ 13:00, dollar index 13:15 |
| 2025-11-28 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Fri, Nov 28: Cotton late 08:00 and early 13:30, FCOJ 13:30, FANG+ 13:15, dollar index 13:15, Sugar/Coffee/Cocoa regular |
| 2025-12-24 | unsourced | no single cell — the Christmas notice was not retrieved | `IFUS-CAL-2025` | T1 | no family states the eve, and the only document that would is unretrieved |
| 2025-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-25; every modelled family prints `closed` |
| 2025-12-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-26: softs `open`, index families `open1` |

**Gaps, 2025:** **The 2025 Independence Day and Christmas / Boxing Day notices were not retrieved.** Neither is in the Wayback Machine or Common Crawl, 44 live filename candidates returned 404 before the operator's host began rate-limiting, and the notices listing renders client-side so its archived copies name no notice. Their closing condition is page 2+ of the paginated report at <https://www.ice.com/futures-us/notices> (report id `futures_us_exchange_notice`), or the exact PDF URL; tracked as #168. 2025-07-03 and 2025-12-24 ship `Unsourced` on that account alone, and 2025-07-04 and 2025-12-26 because the index families' `open1` cells defer to the same notices. **What the two notices would change here:** a Cotton late open on 2025-07-07 (the shape the 2026 notice gives for 2026-07-06) or a softs early close on either eve would add or move family rows, and no such row is invented. **The third 2025 artifact #168 named is read, and it moved the table.** `ICE_Futures_US_ExNot2024MomentOfSilence20241230.pdf` (Wayback capture 2025-02-07) had been recorded as unretrieved because the annual calendar does not list 2025-01-09 and its state therefore rested on that calendar's day-level spine. It states, verbatim, that “Trading in each of the following futures contracts will end at 9:30 am NY time on that day:” for `Micro NYSE FANG+™ Index futures contracts (contract symbol FNG)`, `NYSE Biotechnology GTR Index futures contracts (contract symbol IUT)` and `NYSE Semiconductor GTR Index futures contracts (contract symbol IUS)`, that “Trading in each of the following futures contracts will end at 1:15 pm NY time on that day:” for `ICE One-Month SOFR Index Futures (SR1)`, `ICE Three-Month SOFR Index Futures (SR3)`, `ICE Conforming 30-year Fixed Mortgage Rate Lock Weighted APR Index Futures (30C)` and `ICE Jumbo 30-year Fixed Mortgage Rate Lock APR Index Futures (30J)`, and that “All other contracts will follow regular trading hours and daily settlement window times.” So 2025-01-09 is not an ordinary day for this venue: the `FANG` family ships an early close at 09:30 NY and this table ships `Unsourced`, because the four tables that back the softs and the dollar index state a full session on a date the notice leaves regular. **Every other part of #168 is untouched** — the two unretrieved notices and the four withheld dates stand — and the 2025-01-09 part of it is resolved by this row. **The twenty 2026-2027 `Unsourced` dates are unchanged** — 2025 material does not resolve them: the missing 2026 MLK and Presidents Day notices and the late-2026 notices are separate gaps, and the 2027 per-holiday notices are unpublished by construction. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **2025-12-31 is normal, not unsourced:** the 2026 New Year's notice's `Wed, Dec 31` column prints `Regular Hours` for every modelled group, which is what lets 2026-01-01's trade-date deletion answer.

**Interpretive steps, 2025:** **The venue table is the intersection of the families that route to it** (design memo D17): the seven `ice_us*` keys, which select five tables — `SUGAR_COFFEE_COCOA`, `ORANGE_JUICE`, `COTTON`, `FANG` and `DOLLAR_INDEX`. A date ships a scheduling row only where all five carry the same row, which in 2025 means the three dates every modelled family prints `closed`. **A disagreement date ships `Unsourced`, not silence:** inside a contiguous coverage window silence is the positive claim that the date was audited normal, which on these dates is false. The two London-bank-holiday dates are the clearest case: Sugar, Coffee and Cocoa open late while every other family is regular, so the venue withholds them even though the annual calendar does not list them at all. 2025-01-09 is the same rule read the other way round: the `FANG` family moves alone — a 09:30 NY close — while the softs and the dollar index trade a full session, so the venue withholds a date whose one moving family the notice names and whose four still families the notice itself leaves regular.

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

**Gaps, 2027:** **Every 2027 `open1` date carries no instant by construction.** ICE announces those hours “in advance of the respective holiday via Exchange Notices”, and none had issued at retrieval, so the 2027 rows are the calendar's `open`/`closed`/`open1` statuses only. Closed by the 2027 per-holiday Exchange Notices as they are issued. Coverage ends 2028-01-03, the last trade date the 2027 calendar names. Its last two dates are audited normal on the calendar itself: 2027-12-31 is not on ICE's 2027 calendar at all, and on 2028-01-03 — New Year's Day observed, since 2028-01-01 is a Saturday — every product group a crate identity routes to prints `open` and only Canola, which no crate identity models, prints `closed`; so neither date carries a row.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below.

- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf> — the ICE Futures U.S. FANG+ launch notice of 2017-09-26, which states trading begins at the start of trade date 2017-11-08 with 20:00–18:00 ET hours, the exceptional Sunday 18:00 open, and a Pre-Open 30 minutes before each executable session — T1.
- <https://www.ice.com/products/66380320/NYSE-FANG-Index-Future> — the current NYSE FANG+ Index Future product page, which retains the grid and separately publishes the 17:30 Sunday and 19:30 weekday queue starts; read 2026-09-26 UTC it is titled `MICRO NYSE FANG+™ Index Futures` and gives `Contract Symbol` `FNG`, which is how the 2025-01-09 notice's `Micro NYSE FANG+` line is tied to this identity's family — T1.
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
