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

Below the window, the span 2010-01-01..2024-12-31 answers its **session** questions as the one-flank bridged residual since the 2026-10-07 ruling (AGENTS.md "Modeling conventions", #296 — the metadata reports `HolidayWindowsBridged`, the complete-calendar claim stays withheld, and the dates below this family's carried horizon keep refusing) while the **holiday-table classification** (`is_closed_trade_date`) refuses a typed `UnresolvedGap` on every span date: nothing witnesses the pre-2025 holiday layer, and no closure is asserted that no operator statement witnesses.

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
| `IFUS-NOTICE-2025-INDEPENDENCE` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_IndDayHoliday_20250508.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `74b89e65fd51405d0df0fc4809785e470f8ed2823c18bd119fe6fd21d637cbe2` |
| `IFUS-NOTICE-2025-CHRISTMAS` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `b5e92b1abf47947bef85e290f2efc16c9fcd9288ce40830d293c71aca1007809` |
| `IFUS-CAL-2026` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice_2026_Holiday_Calendar_20260609.pdf> | retrieved 2026-09-12 07:47 UTC | T1 | `b9723d9afa1c20a0792b628de36c7d3726add58ea3d94972a1e7cefd3e50bf4e` |
| `IFUS-CAL-2027` | 2027-01-01 .. 2028-01-03 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice-2027_Holiday_Calendar_20260604.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d2e39a2db2a26e0578ad0d09b36f51dfdad8502635020fe79d66303c09c1a040` |
| `IFUS-NOTICE-2026-GOODFRIDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_GoodFridayHoliday_20260202.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `0a38abd0c9c8b0f904e9701999ac56466610c46437bdfe0109f88e67da35d181` |
| `IFUS-NOTICE-2026-MEMORIAL` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Memorial_DayHoliday_20260310.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `3dfe2a49babece024ce3cd91948e4fb27740abe31282fd9121097bd8433298f9` |
| `IFUS-NOTICE-2026-JUNETEENTH` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_JuneteenthHoliday_20260320.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `34a21ef6facf3dafd72d444ea352fc60e9b2dbf4fc674c1f4f8c199936169edd` |
| `IFUS-NOTICE-2026-INDEPENDENCE` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US__IndDayHoliday_.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `9de2852ed397d403cf8061358d98839954dbe861bf349fb91f57f95910983ea6` |
| `IFUS-NOTICE-2026-LABORDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_LaborDayHoliday_20260708.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d383fe9ae20065f4de6825bff68f13d9cdd8233ea57de0f40644af92439b88f4` |
| `IFUS-NOTICE-2026-THANKSGIVING` | trade dates 2026-11-26 .. 2026-11-27 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Thanksgiving_Holiday_20260923.pdf> | retrieved live from the operator, 2026-09-27 19:39 UTC | T1 | `f153d1066d398ee1a65a598ed8fada26f95e7ad24b26e43b1ba720e073e6d28c` |
| `IFUS-NOTICE-2026-DST-END` | trade dates 2026-10-26 .. 2026-10-30 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_DST_End2026_20260925.pdf> | retrieved live from the operator, 2026-09-27 19:40 UTC | T1 | `2fe4ac40c6bf93ddc2bcee8e15356e86b12cb7e001d60345ca9bd52070632a5e` |
| `IFUS-NOTICE-2026-MLK` | trade dates 2026-01-16 .. 2026-01-20 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_MLKDay_Holiday_20251203.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `1cab5d8d23084cce002098769fa472aa8795bd410fc30a2e43850ec319030780` |
| `IFUS-NOTICE-2026-PRESIDENTS` | trade dates 2026-02-13 .. 2026-02-17 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Presidents_Day_Holiday_20251223.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `961d3a35f54f13477ab927610e75e445cc9acefa2c3dd2d7b5340f0a9080e76b` |

**Notes on the documents.**
- `IFUS-CAL-2026` resolves to the Exchange-Notice copy, whose labels the rows follow; the Holiday-Hours-hub copy <https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> (retrieved 2026-09-12 04:19 UTC, sha256 `da97503545a3fb7607948367d30896685e220b36abed24aa02bbe1a21acb2817`) carries the earlier product-group headers with every `open` / `closed` / `open1` status cell identical.
- `IFUS-CAL-2027` is an unconditional published future, which LAW-NO-FABRICATED-DATES permits encoding ahead of its effective days.
- `IFUS-NOTICE-2025-MOMENT-OF-SILENCE` is the operator's one-page notice of 2024-12-30 for the National Day of Mourning; the research store holds it at `holidays/raw/iceus-2025-2027/extra/ICE_Futures_US_ExNot2024MomentOfSilence20241230.pdf`, whose sha256 and URL its own `INDEX.md` records, and the `pdftotext -layout` twin beside it reproduces every quotation below. It is the artifact #168 recorded as unread; it is read now.
- The 2026 equity-index companion notice of the same date (sha256 `6f07e49c36c70ef0c81cf028c3c7f539339b03092c81b22cf9c583f9f0c6b9e2`) is last-trade-day and final-settlement content only and keys nothing here (LAW-SESSION-NOT-EXPIRY).
- The four notices `IFUS-NOTICE-2025-INDEPENDENCE`, `IFUS-NOTICE-2025-CHRISTMAS`, `IFUS-NOTICE-2026-MLK` and `IFUS-NOTICE-2026-PRESIDENTS` were retrieved live on 2026-10-09 UTC through the operator's own notices listing service — POST `https://www.ice.com/api/sitesearchservice/v1/search/websitenotices?searchCollections=futures_us_exchange_notice&year=<YYYY>`, the machine channel behind the client-side notices page — which names every notice with its live URL; the T2 listing captures and the four PDFs are under `holidays/raw/iceus-notices-service-2026-10-09/`. The earlier records that no 2026 MLK or Presidents Day notice exists were wrong: both notices issued in December 2025 and the archive-only channels never reached them. The 2026 Christmas / Boxing Day notice had still not issued at the same re-check (the listing's latest notice was 2026-10-05), so the 2026-12-28 `open1` cell keeps withholding.
- The research store holds the 2025 artifacts under `holidays/raw/iceus-2025-2027/`, with each file's URL, UTC retrieval time, sha256 and byte count in its `INDEX.md` and `INVENTORY.tsv`; the 2026-2027 artifacts are under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/` and `…-fix/`.

### 2025

The 2025 block is the intersection of the seven family tables over the `July 5, 2024` — `2025 Trading Holiday Calendar` and the 2025 Exchange Notices, and it is the first year the venue answers for. Three dates ship `Closed`; the other fifteen ship `Unsourced`, because on each of them at least one modelled family states something no other agrees with. Every row below was read from the cited artifact's own bytes, and the `Derived from` cell names each disagreement.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-01-01; every modelled family prints `closed`, so all seven tables agree |
| 2025-01-09 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-MOMENT-OF-SILENCE` | T1 | notice date Thu, Jan 9: `Micro NYSE FANG+` (`FNG`) ends at 09:30 NY, the softs and the dollar index keep regular hours — the notice's own “All other contracts will follow regular trading hours and daily settlement window times” |
| 2025-01-20 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-MLK` | T1 | notice date Mon, Jan 20: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-02-17 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | notice date Mon, Feb 17: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-04-18 | closed | `closed` — no instant printed | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Fri, Apr 18: every modelled family prints `Closed` |
| 2025-04-21 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Mon, Apr 21: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2025-05-05 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-LBMA-MAY05` | T1 | London bank holiday: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2025-05-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-MEMORIAL` | T1 | notice date Mon, May 26: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-06-19 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | notice date Thu, June 19: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-07-03 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-INDEPENDENCE` | T1 | notice date Thu, July 3: FANG+ early at 13:15 in its own bullet, every other family printing `Regular Hours` |
| 2025-07-04 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-INDEPENDENCE` | T1 | notice date Fri, July 4: the softs group and FCOJ `Closed`, FANG+ early at 13:00, the dollar index early at 13:00 |
| 2025-07-07 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-INDEPENDENCE` | T1 | notice date Mon, July 7: `Late Open for Cotton: 8:00 am`, regular open times for every other family |
| 2025-08-25 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-LBMA-AUG25` | T1 | London bank holiday: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2025-09-01 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-LABORDAY` | T1 | notice date Mon, Sep 1: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2025-11-27 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Thu, Nov 27: softs `Closed`, FANG+ 13:00, dollar index 13:15 |
| 2025-11-28 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Fri, Nov 28: Cotton late 08:00 and early 13:30, FCOJ 13:30, FANG+ 13:15, dollar index 13:15, Sugar/Coffee/Cocoa regular |
| 2025-12-24 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-CHRISTMAS` | T1 | notice date Wed, Dec 24: Cotton, Coffee, Cocoa and FCOJ early at 13:05, Sugar 11/16 regular close, FANG+ early at 13:15, the dollar index early at 13:45 |
| 2025-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-25; every modelled family prints `closed` |
| 2025-12-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2025-CHRISTMAS` | T1 | notice date Fri, Dec 26: Coffee, Cocoa, Cotton and Sugar 11 late at 07:30, FCOJ and Sugar 16 and the index families printing `Regular Hours` |

**Gaps, 2025: closed — the two notices were retrieved on 2026-10-09 UTC.** The channel the old closing condition named was the operator's own notices listing service, and it answered: `POST https://www.ice.com/api/sitesearchservice/v1/search/websitenotices?searchCollections=futures_us_exchange_notice&year=<YYYY>` — the machine channel (T2) behind the client-side notices page, found by reading the page's JS bundle — returns the whole back catalogue the page's default view never showed (188 exchange notices for 2025 alone), naming each notice with its live URL. The **2025 Independence Day notice** (May 8, 2025) and the **2025 Christmas notice** (October 31, 2025) were retrieved live from the operator minutes later, and their bytes state every date the table used to withhold: 2025-07-03 is FANG+ 13:15 against every other family's `Regular Hours`, 2025-07-04 is the softs group and FCOJ `Closed` against the index families' 13:00, 2025-12-24 is the softs' 13:05 (Sugar 11/16 regular) against FANG+ 13:15 and the dollar index 13:45, and 2025-12-26 is the softs' 07:30 late open against the index families' `Regular Hours`. The Independence Day notice also gives **Cotton a Monday late open at 8:00 am on 2025-07-07** — the 2025 twin of 2026-07-06 — which adds this table's one new row that year, exactly the move the old paragraph said such a notice would make. All four dates stay `Unsourced` here on the intersection rule: the notices resolved them into family rows that disagree, which is what the vocabulary is for, and no venue row moves from withheld to stated on a date its families still dispute. The 2025-01-09 row and its scope step are unchanged, as are the two London-bank-holiday rows. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **2025-12-31 is normal, not unsourced:** the 2026 New Year's notice's `Wed, Dec 31` column prints `Regular Hours` for every modelled group, which is what lets 2026-01-01's trade-date deletion answer.

**Interpretive steps, 2025:** **The venue table is the intersection of the families that route to it** (design memo D17): the seven `ice_us*` keys, which select seven tables — `SUGAR`, `COFFEE`, `COCOA`, `ORANGE_JUICE`, `COTTON`, `FANG` and `DOLLAR_INDEX`. A date ships a scheduling row only where all seven carry the same row, which in 2025 means the three dates every modelled family prints `closed`. **A disagreement date ships `Unsourced`, not silence:** inside a contiguous coverage window silence is the positive claim that the date was audited normal, which on these dates is false. The two London-bank-holiday dates are the clearest case: Sugar, Coffee and Cocoa open late while every other family is regular, so the venue withholds them even though the annual calendar does not list them at all. 2025-01-09 is the same rule read the other way round: the `FANG` family moves alone — a 09:30 NY close — while the softs and the dollar index trade a full session, so the venue withholds a date whose one moving family the notice names and whose six still families the notice itself leaves regular.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-01; all five calendar groups print `closed`, so every modelled family agrees |
| 2026-01-19 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-MLK` | T1 | notice date Mon, Jan 19: the softs group `Closed`, FANG+ early at 13:00 (its `NYSE Stock Index` bullet; interpretive step in the family file), the dollar index printing `Regular Hours, TAS trading will not be held` |
| 2026-02-16 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-PRESIDENTS` | T1 | notice date Mon, Feb 16: the softs group and Canola `Closed`, FANG+ early at 13:00 (the same bullet shape), the dollar index printing `Regular Hours, TAS trading will not be held` |
| 2026-04-03 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Fri, Apr 3: softs `Closed`, index families on a 05:00 late open with a 09:15 or 11:15 early close |
| 2026-04-06 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Mon, Apr 6: Sugar, Coffee and Cocoa late at 07:30, every other family regular |
| 2026-05-25 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25: softs `Closed`, FANG+ 13:00, dollar index 14:30 |
| 2026-06-19 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19: softs `Closed`, FANG+ 13:00, dollar index 14:30 |
| 2026-07-03 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3: softs `Closed`, FANG+ 13:00, dollar index 14:30 |
| 2026-07-06 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Mon, July 6: Cotton late at 08:00, every other family regular |
| 2026-09-07 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7: softs `Closed`, FANG+ 13:00, dollar index regular |
| 2026-10-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026, trade dates 2026-10-26 through 2026-10-30: Sugar opens 04:30 NY, Coffee 05:15 NY, Cocoa 05:45 NY, and “Regular Trading Hours” for every other group, so the seven families disagree and the venue states no instant |
| 2026-10-27 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026, trade dates 2026-10-26 through 2026-10-30: Sugar opens 04:30 NY, Coffee 05:15 NY, Cocoa 05:45 NY, and “Regular Trading Hours” for every other group, so the seven families disagree and the venue states no instant |
| 2026-10-28 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026, trade dates 2026-10-26 through 2026-10-30: Sugar opens 04:30 NY, Coffee 05:15 NY, Cocoa 05:45 NY, and “Regular Trading Hours” for every other group, so the seven families disagree and the venue states no instant |
| 2026-10-29 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026, trade dates 2026-10-26 through 2026-10-30: Sugar opens 04:30 NY, Coffee 05:15 NY, Cocoa 05:45 NY, and “Regular Trading Hours” for every other group, so the seven families disagree and the venue states no instant |
| 2026-10-30 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026, trade dates 2026-10-26 through 2026-10-30: Sugar opens 04:30 NY, Coffee 05:15 NY, Cocoa 05:45 NY, and “Regular Trading Hours” for every other group, so the seven families disagree and the venue states no instant |
| 2026-11-26 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-THANKSGIVING` | T1 | notice dated September 23, 2026: the softs group and Cotton and FCOJ `Closed`, the NYSE Stock Index group early at 13:00, the dollar index group early at 13:15 |
| 2026-11-27 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-NOTICE-2026-THANKSGIVING` | T1 | the same notice's Friday column: Cotton late open 08:00 and early close 13:30, FCOJ early close 13:30, “Regular Hours for Sugar, Coffee and Cocoa”, the index families early at 13:00/13:15, the dollar index early at 13:15 |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25; all five calendar groups print `closed` |
| 2026-12-28 | unsourced | no single cell — the modelled families disagree, so the venue states no instant | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-28: softs `open`, index families `open1` |

**Gaps, 2026: the two “missing” notices exist — the earlier record was an archive-channel failure, corrected 2026-10-09 UTC.** The notices listing service named both the **2026 MLK notice** (issued 2025-12-03) and the **2026 Presidents Day notice** (issued 2025-12-23), and both were retrieved live the same day; the prior sweeps' archive-only channels (the CDX prefix enumeration, the 150 live filename probes) had simply never reached December 2025. Their bytes state per-family arrangements that leave the intersection exactly where it was — softs `Closed`, FANG+ early at 13:00, the dollar index printing `Regular Hours, TAS trading will not be held` — so the two dates stay `Unsourced` on the rows above, now cited to the notices instead of the calendar's `open1` cells. **The late-2026 Christmas notices had still not issued at the re-check:** Christmas Eve 2026-12-24 and Boxing Day 2026-12-28 — dates not on the calendar at all, whose customary hours exist only in a notice that had not issued (the listing's latest 2026 notice was 2026-10-05), so this table carries no row for either. The 2026 Thanksgiving notice issued on 2026-09-23 and its rows are read above. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice. **Scope of the intersection.** `Canola`, `Energy* and Environmental Contracts` and `Daily Gold and Silver Contracts` are product groups on ICE's own calendar that no crate identity routes to, so they are outside this table's intersection; a consumer that maps such a product to the venue calendar would be using a clock the crate never claimed for it. On all four dates that do ship a `closed` row those three groups are closed as well, so including them would not change a row. **The 2026 DST-end week (2026-10-26..30) is the same disagreement shape as the London-bank-holiday late opens:** the notice names the three softs only and prints “Regular Trading Hours” for every other group, so the families disagree on the open instant and the venue ships `unsourced` for each of the five dates. The notice's settlement-window lines are calculation windows, not session boundaries (LAW-SESSION-NOT-EXPIRY), and state nothing this table could carry anyway.

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
- <https://www.ice.com/holiday-hours> — ICE holiday hours, the current-schedule monitoring entry point (the former `ice.com/trading-hours` page moved here; verified live 2026-10-10 UTC, its IFUS entry re-fetching byte-identical to the held calendar) — T1.
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
