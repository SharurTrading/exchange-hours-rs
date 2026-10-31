<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_sugar` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_sugar.rs`](../../src/calendar/schedules/futures/us/ice_sugar.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Sugar No. 11 (`SB`) only. Current 03:30–13:00 NY grid and both order-entry phases are primary-sourced, with dated 2012-01-30, 2012-11-05, 2014-02-03 and 2018-10-08 revisions. Partial because no primary source gives a January 2010 close; the August 2011 master table is carried back as the baseline — and the carried grid is no longer carried blind: the operator's own "Sugar No. 11 / No. 16" brochure, document date 22 March 2010, prints the Sugar No. 11 spec row `HOURS  0330 EST TO 1400EST`, and the operator's Chinese Sugar No. 11 introduction printed 12 December 2011 re-witnesses the same 03:30–14:00 ET electronic session, so the grid is witnessed by agreeing operator statements at both ends of the carried span. What remains unwitnessed is January to mid-March 2010, the pre-audited-window holiday and order-entry layer, and a document that states the January 2010 close as of that day. The 2026-08-31 review enumerated every archived master-table edition and confirmed August 2011 (captured 2011-12-12) is the earliest of that class; the earlier conclusion that no in-gap document could exist rested on the master table being the only source class, and the 2026-10-13 wave (store `holidays/raw/ice-history-2026-10-13/sugar-pre2011/`, STATUS dated 2026-10-13 UTC) superseded it by finding the brochure outside that class. The Friday-evening pre-open leg is not expressible as a normal-week wrap and is omitted.

## Revision rows

- 2012-01-30 — T1 — ICE ExNot 121911 S11 hours — daily electronic session begins at 01:30 NY.
- 2012-11-05 — T1 — ICE ExNot 1018912 S11 hours — daily session begins at 02:30 NY.
- 2014-02-03 — T1 — ICE ExNot 012714 hours — grid becomes 03:30-13:00 NY.
- 2018-10-08 — T1 — ICE PCPO notice 20180920 — post-close pre-open added, 13:30-18:00 NY.

## Holidays

**Coverage:** 2025-01-01..2028-01-03 (inclusive trade dates). Tier: T1 throughout.

Below the window, the span the sourced week covers answers its **session** questions as the one-flank bridged residual since the 2026-10-07 ruling (AGENTS.md "Modeling conventions", #296 — the metadata reports `HolidayWindowsBridged`, the complete-calendar claim stays withheld, and the dates below this family's carried horizon keep refusing) while the **holiday-table classification** (`is_closed_trade_date`) refuses a typed `UnresolvedGap` on every span date: the pre-2025 holiday layer is not in the audited window, and no closure is asserted that no operator statement witnesses.

**Witnessed pre-window data, not encoded.** The 2026-10-13 wave (store `holidays/raw/ice-history-2026-10-13/sugar-pre2011/`) brought the first holiday-layer operator statement from inside the carried span: Exchange Notice **ExNot 041111**, dated April 11, 2011 (T1, artifact `sugar-pre2011/ice_ExNot041111_SoftsOpen.wayback-20111213005119.pdf`, sha256 `7d56ecc883f767c71a306f2217c9da9f384c0cb756c2bbb0aa559038a79e37aa`, Wayback replay of `theice.com/publicdocs/futures_us/exchange_notices/ExNot041111SoftsOpen.pdf` capture `20111213005119`, retrieved 2026-10-13 23:56:33 UTC), states: "Due to UK banking holiday on Monday, May 2, 2011, ICE Futures U.S. will delay the start of electronic trading in Sugar No. 11, Coffee "C" and Cocoa futures and options contacts until 7:30 am NY time." — a dated, unconditional **2011-05-02 late open at 07:30 NY**, the same shape this family ships inside the audited window (2025-04-21, 2025-05-05, 2025-08-25). The notice adds "This change to the start time for these products does not affect floor trading hours, end of day times or settlement times for these products", witnessing that fixed end-of-day times existed in April 2011 without stating them. **Why no row ships for it:** the family's audited window is 2025-01-01..2028-01-03, and LAW-HOLIDAY-SCOPE makes a date without a row inside a window a claim of audited normal; encoding a lone 2011 row would require extending the window down to 2011, which would assert audited-normal for every other 2011-2019 date — a claim nothing backs. Outside the window the table has no answer by construction, so the notice stays recorded here as witnessed pre-window data, and it becomes a row only when the family's pre-2025 holiday layer is worked up to the same audited depth (the #332 historical programme).

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
| `IFUS-NOTICE-2025-CHRISTMAS` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `b5e92b1abf47947bef85e290f2efc16c9fcd9288ce40830d293c71aca1007809` |
| `IFUS-NOTICE-2025-LBMA-MAY05` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250429040409id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_ExNotDelayedOpensMay5_20250424.pdf> | Wayback `id_` replay of capture `20250429040409`, 2026-09-26 07:28:02 UTC | T1 | `a737611d7dcc47d5dc3e06e70011e13695a63150b454cc1d23006ded64ab4c30` |
| `IFUS-NOTICE-2025-LBMA-AUG25` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250816054649id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_ExNotDelayedOpensAug25_20250728.pdf> | Wayback `id_` replay of capture `20250816054649`, 2026-09-26 07:28:04 UTC | T1 | `2efe0b39ad99c1a0aa6abfcfa9db2f44bd1157564e73ec7cf6476d5ae65f3cc8` |
| `IFUS-CAL-2026` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice_2026_Holiday_Calendar_20260609.pdf> | retrieved 2026-09-12 07:47 UTC | T1 | `b9723d9afa1c20a0792b628de36c7d3726add58ea3d94972a1e7cefd3e50bf4e` |
| `IFUS-CAL-2027` | 2027-01-01 .. 2028-01-03 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice-2027_Holiday_Calendar_20260604.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d2e39a2db2a26e0578ad0d09b36f51dfdad8502635020fe79d66303c09c1a040` |
| `IFUS-NOTICE-2026-GOODFRIDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_GoodFridayHoliday_20260202.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `0a38abd0c9c8b0f904e9701999ac56466610c46437bdfe0109f88e67da35d181` |
| `IFUS-NOTICE-2026-MEMORIAL` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Memorial_DayHoliday_20260310.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `3dfe2a49babece024ce3cd91948e4fb27740abe31282fd9121097bd8433298f9` |
| `IFUS-NOTICE-2026-JUNETEENTH` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_JuneteenthHoliday_20260320.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `34a21ef6facf3dafd72d444ea352fc60e9b2dbf4fc674c1f4f8c199936169edd` |
| `IFUS-NOTICE-2026-INDEPENDENCE` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US__IndDayHoliday_.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `9de2852ed397d403cf8061358d98839954dbe861bf349fb91f57f95910983ea6` |
| `IFUS-NOTICE-2026-LABORDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_LaborDayHoliday_20260708.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d383fe9ae20065f4de6825bff68f13d9cdd8233ea57de0f40644af92439b88f4` |
| `IFUS-NOTICE-2026-DST-END` | trade dates 2026-10-26 .. 2026-10-30 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_DST_End2026_20260925.pdf> | retrieved live from the operator, 2026-09-27 19:40 UTC | T1 | `2fe4ac40c6bf93ddc2bcee8e15356e86b12cb7e001d60345ca9bd52070632a5e` |

**Notes on the documents.**
- `IFUS-CAL-2026` resolves to the Exchange-Notice copy, whose labels the rows follow; the Holiday-Hours-hub copy <https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> (retrieved 2026-09-12 04:19 UTC, sha256 `da97503545a3fb7607948367d30896685e220b36abed24aa02bbe1a21acb2817`) carries the earlier product-group headers with every `open` / `closed` / `open1` status cell identical.
- `IFUS-CAL-2027` is an unconditional published future, which LAW-NO-FABRICATED-DATES permits encoding ahead of its effective days.
- The 2026 equity-index companion notice of the same date (sha256 `6f07e49c36c70ef0c81cf028c3c7f539339b03092c81b22cf9c583f9f0c6b9e2`) is last-trade-day and final-settlement content only and keys nothing here (LAW-SESSION-NOT-EXPIRY).
- The four notices `IFUS-NOTICE-2025-INDEPENDENCE`, `IFUS-NOTICE-2025-CHRISTMAS`, `IFUS-NOTICE-2026-MLK` and `IFUS-NOTICE-2026-PRESIDENTS` were retrieved live on 2026-10-09 UTC through the operator's own notices listing service — POST `https://www.ice.com/api/sitesearchservice/v1/search/websitenotices?searchCollections=futures_us_exchange_notice&year=<YYYY>`, the machine channel behind the client-side notices page — which names every notice with its live URL; the T2 listing captures are under `holidays/raw/iceus-notices-service-2026-10-09/` with the PDFs beside them.
- The research store holds the 2025 artifacts under `holidays/raw/iceus-2025-2027/`, with each file's URL, UTC retrieval time, sha256 and byte count in its `INDEX.md` and `INVENTORY.tsv`; the 2026-2027 artifacts are under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/` and `…-fix/`.

### Documents, pre-window witnesses (2010-2011)

The 2026-10-13 wave (store `holidays/raw/ice-history-2026-10-13/sugar-pre2011/`, its `INDEX.md` section B) added the first operator statements dated inside the carried span. None of these keys a shipped row: the first two witness the normal-week baseline the module carries, the third re-witnesses it at the span's end, and the fourth is the pre-window exception recorded above. Each is quoted from its own bytes.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `IFUS-SUGAR-BROCHURE-2010` | witness of the carried baseline in-gap; document dated 2010-03-22 (PDF `CreationDate`) | <https://web.archive.org/web/20111121015738id_/https://www.theice.com/publicdocs/ICE_Sugar_Brochure.pdf> | Wayback `id_` replay of capture `20111121015738`, retrieved 2026-10-13 23:46:28 UTC | T1 | `fb028bcaaa2473cc45c892263ca57331b70958a87e95ef64e9902b4fa727b5d8` |
| `IFUS-SUGAR-BROCHURE-2012` | corroborates the dated 2012-01-30 revision; PDF dated 2012-05-14 | <https://web.archive.org/web/20120610134716id_/https://www.theice.com/publicdocs/ICE_Sugar_Brochure.pdf> | Wayback `id_` replay of capture `20120610134716`, retrieved 2026-10-13 23:53:23 UTC | T1 | `124b5f10b67bb7c5d2ab1f4f08a92d1bfdc99c48a60afb5bd9dc3a084e3d51b8` |
| `IFUS-SUGAR-INTRO-ZH-2011` | re-witnesses the carried grid; printed in-document 2011-12-12 | <https://web.archive.org/web/20120610022724id_/https://www.theice.com/publicdocs/Sugar_Introduction_Chinese.pdf> | Wayback `id_` replay of capture `20120610022724`, retrieved 2026-10-13 23:46:32 UTC | T1 | `5bfe0b72181874bfdffdc300e9d332441e84c8c55236878ac6a2c3efac057b70` |
| `IFUS-EXNOT-041111-SOFTS-OPEN` | in-gap exception witness, trade date 2011-05-02, recorded above and not encoded | <https://web.archive.org/web/20111213005119id_/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot041111SoftsOpen.pdf> | Wayback `id_` replay of capture `20111213005119`, retrieved 2026-10-13 23:56:33 UTC | T1 | `7d56ecc883f767c71a306f2217c9da9f384c0cb756c2bbb0aa559038a79e37aa` |
| `IFUS-NOTICE-2026-CHRISTMAS` | trade dates 2026-12-24 .. 2026-12-28 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Christmas_Holiday_20261022.pdf> | retrieved live from the operator, 2026-10-23 23:21 UTC | T1 | `a7658155b57c51085cd3b36abc9ff6b91f3ca353cda88365a06459ebb40f0801` |

**What the bytes state.**

- `IFUS-SUGAR-BROCHURE-2010` — the operator's own "**SUGAR NO. 11® AND SUGAR NO. 16**" brochure, PDF `CreationDate D:20100322164205-04'00'` (22 March 2010; `ModDate D:20100322164326-04'00'`; no printed issue date inside). Under "The ICE Futures U.S. Sugar No. 11 sugar futures contract … The key specifications are:" it prints the spec row **`HOURS  0330 EST TO 1400EST`** — a `TRADING HOURS`-row class statement in session language (LAW-SESSION-NOT-EXPIRY), naming the exact grid the module carries as `SUGAR_REGULAR_BASELINE`, observed from inside the carried span. The Sugar No. 16 table in the same document prints **`HOURS  0900 EST to 1400 EST`**. The brochure states no holiday data and no order-entry phases, so it witnesses the normal-week grid only.
- `IFUS-SUGAR-BROCHURE-2012` — the same brochure's next generation, PDF created 14 May 2012: the Sugar No. 11 spec now prints **`0130 Eastern Standard Time (0230 Eastern Daylight when applicable) to 1400 Eastern Standard Time.`** and Sugar No. 16 **`0900 Eastern Standard Time to 1400 Eastern Standard Time.`** — the post-2012-01-30 grid the module already carries, corroborated from a second T1 document five weeks after the revision.
- `IFUS-SUGAR-INTRO-ZH-2011` — the operator's Chinese-language Sugar No. 11 introduction, printed in-document 2011年12月12日 (12 December 2011): **`交易时段: 3:30 a.m. – 2:00 p.m. 美国东部时间`** (electronic session 03:30–14:00 US Eastern for futures and options) and **`场内交易时段: 8:00 a.m. 至 1:30 p.m. 美国东部时间`** (floor session 08:00–13:30 ET, context the electronic-session crate does not model). It re-witnesses the carried grid at the end of the span, the last observation before the dated 2012-01-30 change.
- `IFUS-EXNOT-041111-SOFTS-OPEN` — quoted in full in the witnessed-pre-window paragraph above.
- The wave's other artifacts key nothing: a 19 March 2010 floor-rule notice (Rule 4.09 options post-close, effective 2010-03-22) states no electronic-session bound; a 2009 swap-contract specification and a January 2011 clearing notice reference the session without stating it; a September 2011 Sugar No. 16 stevedoring notice is not session data; a Chinese rulebook translation (PDF created 2011-10-10) carries no hours provision, consistent with the 2026-09-01 rulebook finding; and the "Monthly Softs Fast Facts" newsletter is a J. Ganes Consulting vendor restatement distributed "compliments of ICE Futures U.S." — T3/T4 that mirrors no operator document verbatim, so it dates nothing and keys nothing (LAW-PRIMARY-SOURCES), kept as residual-risk context only.

### 2025

The 2025 block is the `July 5, 2024` — `2025 Trading Holiday Calendar`'s own day-level spine, ICE's per-holiday Exchange Notices for the holidays it issued one for, the **revised** Thanksgiving notice that adds Friday 2025-11-28, and the two London-bank-holiday “Delayed Opens” notices that carry dates the calendar does not list. Every row below was read from the cited artifact's own bytes.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-01-01, “New Year’s Day”; the softs column prints `closed` |
| 2025-01-20 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-MLK` | T1 | notice date Mon, Jan 20, group line “Sugar No. 11® and 16, Coffee “C” ®, Cotton No. 2®, Cocoa and FCOJ Contracts” |
| 2025-02-17 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | notice date Mon, Feb 17; the same notice's group line also names Canola, which no crate identity models |
| 2025-04-18 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Fri, Apr 18, the shared softs/Canola group line |
| 2025-04-21 | late open | `Sugar No. 11, Coffee “C” and Cocoa - late open at 7:30 am. Regular Hours for all other contracts.` — 07:30 ET | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Mon, Apr 21; only Sugar, Coffee and Cocoa are named, so FCOJ and Cotton carry no row |
| 2025-05-05 | late open | `ICE Futures U.S. will delay the start of trading in Sugar No. 11, Coffee “C” and Cocoa futures and options contracts until 7:30 am NY time on that day.` — 07:30 NY | `IFUS-NOTICE-2025-LBMA-MAY05` | T1 | notice titled “ON MAY 5, 2025”; a London bank holiday the annual calendar does not list |
| 2025-05-26 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-MEMORIAL` | T1 | notice date Mon, May 26, the softs group line |
| 2025-06-19 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | notice date Thu, June 19 |
| 2025-07-04 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-07-04, “Independence Day”; the softs column prints `closed` |
| 2025-08-25 | late open | `ICE Futures U.S. will delay the start of trading in Sugar No. 11, Coffee “C” and Cocoa futures and options contracts until 7:30 am ET on that day.` — 07:30 ET | `IFUS-NOTICE-2025-LBMA-AUG25` | T1 | notice titled “ON AUGUST 25, 2025”; a London bank holiday the annual calendar does not list |
| 2025-09-01 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-LABORDAY` | T1 | notice date Mon, Sep 1 |
| 2025-11-27 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Thu, Nov 27 |
| 2025-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-25, “Christmas Day”; the softs column prints `closed` |
| 2025-12-26 | late open | `Late Open for Coffee “C”, Cocoa, Cotton No. 2 and Sugar 11 - 7:30 am` — 07:30 NY | `IFUS-NOTICE-2025-CHRISTMAS` | T1 | notice date Fri, Dec 26; the notice's own line names Sugar 11 and prints `Regular Open times for FCOJ and Sugar 16, Regular Close Times for all products` |

**Gaps, 2025:** **The 2025 Independence Day and Christmas notices were retrieved on 2026-10-09 UTC**, live from the operator through its own notices listing service (the machine channel behind the client-side notices page), closing the #168 channel: the listing's back catalogue named every notice the archive-only sweeps had missed. The Independence Day notice prints `Regular Hours` for the softs on Thu, July 3 and `Regular open times for all other contracts` beside Cotton's Monday late open on July 7, and the Christmas notice prints `Regular Close times for Sugar 11 and 16` on Wed, Dec 24, so 2025-07-03, 2025-07-07 and 2025-12-24 are audited normal for this family — the notice's own words are the positive witness, and the family carries no row on any of them. The Christmas notice's Boxing Day column names Sugar 11 in the 07:30 late open, and the family's 2025-12-26 row above is that line. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. **2025-12-31 is normal, not unsourced:** the 2026 New Year's notice's `Wed, Dec 31` column prints `Regular Hours` for “Sugar No. 11® and 16, Coffee “C” ®, Coffee “C”® Metric, Cotton No. 2®, Cocoa, Canola and FCOJ Contracts”.

**Interpretive steps, 2025:** The softs group is one line on every 2025 notice — “Sugar No. 11® and 16, Coffee “C” ®, Cotton No. 2®, Cocoa and FCOJ Contracts” (Canola, which no crate identity models, joins the line on some notices) — and the calendar's own column reads “Cocoa, Coffee “C”®, Cotton No 2®, FCOJ, Sugar No. 11 and No. 16 Contracts”, so the three Sugar/Coffee/Cocoa keys, FCOJ and Cotton would share every row but for the two dates ICE names a subset on. The Good Friday notice gives the Easter Monday late open to “Sugar No. 11, Coffee “C” and Cocoa” and adds “Regular Hours for all other contracts”; the revised Thanksgiving notice gives Friday 2025-11-28's late open and early close to “Cotton and FCOJ” and prints “Regular Hours for Sugar, Coffee and Cocoa”. The London-bank-holiday notices name the same three softs and add “All other Exchange products will follow their regular trading schedules and are not affected by these changes”. Each family's session runs inside one New York civil day, so the conversion from ICE's civil date to the crate's trade date is the identity.

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
| 2026-10-26 | late open | `Sugar No. 11: 4:30 am NY time` | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026: “Effective with the start of trading for trade date Monday, October 26, 2026, and through and including trade date Friday, October 30, 2026”; the family's session runs inside one New York civil day, so the trade date is the identity |
| 2026-10-27 | late open | `4:30 am NY time` — the notice's “on each day in this period” | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026: “Effective with the start of trading for trade date Monday, October 26, 2026, and through and including trade date Friday, October 30, 2026”; the family's session runs inside one New York civil day, so the trade date is the identity |
| 2026-10-28 | late open | `4:30 am NY time` — the notice's “on each day in this period” | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026: “Effective with the start of trading for trade date Monday, October 26, 2026, and through and including trade date Friday, October 30, 2026”; the family's session runs inside one New York civil day, so the trade date is the identity |
| 2026-10-29 | late open | `4:30 am NY time` — the notice's “on each day in this period” | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026: “Effective with the start of trading for trade date Monday, October 26, 2026, and through and including trade date Friday, October 30, 2026”; the family's session runs inside one New York civil day, so the trade date is the identity |
| 2026-10-30 | late open | `4:30 am NY time` — the notice's “on each day in this period” | `IFUS-NOTICE-2026-DST-END` | T1 | notice dated September 25, 2026: “Effective with the start of trading for trade date Monday, October 26, 2026, and through and including trade date Friday, October 30, 2026”; the family's session runs inside one New York civil day, so the trade date is the identity |
| 2026-11-26 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26, “Thanksgiving Day” |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25, “Christmas Day” |
| 2026-12-28 | replacement blocks | matching 08:00–13:00 NY; prior Thursday queue 13:30–15:30 NY | `IFUS-NOTICE-2026-CHRISTMAS` | T1 | the 2026-12-24 post-close queue is carried at offset -4 to the session it feeds; FCOJ's Monday open is regular |

**Gaps, 2026: the 2026 MLK and Presidents Day notices exist** — the earlier record that none did was an archive-channel failure, corrected on 2026-10-09 UTC when the operator's own notices listing service named both (issued 2025-12-03 and 2025-12-23, retrieved live the same day). Their softs group lines print `Closed` for the holidays, which is what the calendar rows above already state, so no row moves; the notices corroborate, they do not re-key. **The 2026 Christmas notice issued on October 22, 2026 and was retrieved live 2026-10-23 UTC** (`IFUS-NOTICE-2026-CHRISTMAS`): its Christmas-eve column prints `Regular hours for Sugar 11 and 16`, so 2026-12-24 is audited normal and carries no row, and its Boxing Day column names Sugar 11 in the 8:00 am late open beside a `Regular Open for FCOJ and Sugar 16` and `Regular Close Times for all products` — the row above. The 2026 Thanksgiving notice issued on 2026-09-23 and is read in the interpretive steps below. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice.

**Interpretive steps, 2026:** ICE's calendars key one line per product group, not per contract, and the group “Cocoa, Coffee “C”®, Coffee "C"® Metric, Cotton No 2®, FCOJ, Sugar No. 11 and No. 16 Contracts” is the line this family sits on; the per-holiday notices restate the same set as “Sugar No. 11® and 16, Coffee “C”®, Coffee "C"® Metric, Cotton No. 2®, Cocoa and FCOJ”. Where a notice names a subset — Easter Monday's “Sugar No. 11, Coffee “C” and Cocoa”, and 2026-07-06's “Cotton” — only the named families take the row and the notice's own “Regular Hours for all other contracts” is the positive statement of normality for the rest. 2026-10-12 is normal for the same reason. Each family's session runs inside one New York civil day, so the conversion from ICE's civil date to the crate's trade date is the identity. **The 2026 DST-end week.** Notice IFUS-NOTICE-2026-DST-END (September 25, 2026) states unconditional, day-level temporary opens for trade dates 2026-10-26 through 2026-10-30 — the week between the end of British Summer Time (October 25) and the end of US Daylight Saving Time (November 1) — and “At the end of this period the settlement windows for these contracts will revert to the normal times”, so 2026-11-02 is normal. The notice states the opens in session language and prints everything else unchanged “as expressed in NY local times”, so the row is a late open and nothing else. The notice leaves Sugar's own daily settlement window unchanged, and the temporary settlement-window changes it gives Coffee and Cocoa (1:23–1:25 pm and 12:48–12:50 pm) are calculation windows, not session boundaries (LAW-SESSION-NOT-EXPIRY); no row states any settlement window. **The 2026 Thanksgiving notice.** The September 23, 2026 notice (`IFUS-NOTICE-2026-THANKSGIVING`) prints `Closed` for Thu, Nov 26 — the calendar row above — and “Regular Hours for Sugar, Coffee and Cocoa” for Fri, Nov 27, so the family gains no row on either date from it: the notice's late open and early close are Cotton's and FCOJ's, recorded in their own evidence files. Its settlement-window lines for the group print “Regular Times for Sugar, Coffee and Cocoa”, and settlement windows are calculation windows in any case (LAW-SESSION-NOT-EXPIRY).

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
- <https://web.archive.org/web/20111121015738id_/https://www.theice.com/publicdocs/ICE_Sugar_Brochure.pdf> — the operator's "SUGAR NO. 11® AND SUGAR NO. 16" brochure, document date 22 March 2010 (PDF metadata), captured 2011-11-21 — the in-gap witness of the carried baseline (`IFUS-SUGAR-BROCHURE-2010`). **Read at the 2026-10-13 wave** (store `holidays/raw/ice-history-2026-10-13/sugar-pre2011/`), which governs for this source.
- <https://web.archive.org/web/20120610134716id_/https://www.theice.com/publicdocs/ICE_Sugar_Brochure.pdf> — the same brochure, 14 May 2012 generation, captured 2012-06-10 — corroborates the dated 2012-01-30 revision (`IFUS-SUGAR-BROCHURE-2012`). **Read at the 2026-10-13 wave.**
- <https://web.archive.org/web/20120610022724id_/https://www.theice.com/publicdocs/Sugar_Introduction_Chinese.pdf> — the operator's Chinese Sugar No. 11 introduction, printed 12 December 2011, captured 2012-06-10 — re-witnesses the carried grid at the end of the span (`IFUS-SUGAR-INTRO-ZH-2011`). **Read at the 2026-10-13 wave.**
- <https://web.archive.org/web/20111213005119id_/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot041111SoftsOpen.pdf> — ICE Exchange Notice ExNot 041111, April 11, 2011, captured 2011-12-13 — the dated 2011-05-02 late open, witnessed pre-window data (`IFUS-EXNOT-041111-SOFTS-OPEN`). **Read at the 2026-10-13 wave.**
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf> — ICE Futures U.S. notice, 20 September 2018, extension of the pre-open order entry session (PCPO).
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf> — ICE Futures U.S. Exchange Notice, Changes to Daily Trading Hours, effective trade date 2014-02-03.
- <https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf> — ICE Futures U.S. Rulebook chapter 4 (Trading) — Rule 4.22(a) confines the Pre-Trading Session to Limit order entry. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf> — ICE Futures U.S. Rulebook chapter 11 (Sugar No. 11), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf> — ICE Futures U.S. Rulebook chapter 8 (Coffee), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf> — ICE Futures U.S. Rulebook chapter 9 (Cocoa), captured December 2011 — no hours provision. **Read at the 2026-09-01 targeted review** that established these hours are set administratively rather than by rule; that date is later than this row's review date and governs for this source.
- <https://www.ice.com/products/23/Sugar-No-11-Futures> — ICE Sugar No. 11 product page — the current grid and the pre-open footnote.
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot121911S11_Hours_12312.pdf> — ICE Exchange Notice — "Effective with the start of trading on Monday, January 30, 2012 ... will begin at 1:30 am NY time".
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/1018912ExNotS11Hours.pdf> — ICE Exchange Notice — "Effective with the start of trading on Monday, November 5, 2012 ... will begin at 2:30 am NY time".
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/exnot03XX10DST.pdf> — ICE March 2010 DST notice — names "the normal 3:30 am NY time" Sugar open.

## Gaps and residual risks

- **executable** — the carried baseline is now **witnessed in-gap**; what remains unwitnessed is the head of the span and the holiday/phase layer. The operator's own Sugar No. 11/No. 16 brochure, document date **22 March 2010** (`IFUS-SUGAR-BROCHURE-2010`), prints the Sugar No. 11 spec row `HOURS  0330 EST TO 1400EST` — the exact grid the module carries — and the operator's Chinese introduction printed **12 December 2011** (`IFUS-SUGAR-INTRO-ZH-2011`) re-witnesses it, so the 03:30–14:00 NY grid is stated by agreeing operator documents at both ends of the carried span and the module's carried baseline equals the witnessed state (no served hour moves in this correction). Remaining gaps, each with its closer: **2010-01-01..2010-03-21** — the span between the support floor and the brochure's document date, closed only by an operator document dated inside it (a desk answer or a surfaced artifact); **the holiday and order-entry layer of the whole carried span** — the brochure states the grid only, and outside the one witnessed exception (the 2011-05-02 late open above) no pre-2025 holiday arrangement is witnessed, closed family-by-family in the #332 historical programme; **the January 2010 close as of that day** — no source states it, and a closure stays distinct from missing evidence. The earlier record that the August-2011 carry-back was "the terminal answer" (2026-08-31 and 2026-09-01 reviews) is **superseded as of 2026-10-13 UTC** (store `holidays/raw/ice-history-2026-10-13/sugar-pre2011/`, STATUS entry dated 2026-10-13): it rested on the master hours table being the only source class, and the brochure is an operator document outside that class. What survives of those reviews: August 2011 remains the earliest surviving **master-table** edition, the rulebook chapters still carry no hours provision, so no filing fixes these hours — the finds came from the name-level artifact sweep, not from a filing. **Domain-lineage pass, 2026-10-06 UTC (store: `holidays/raw/iceus/domain-lineage-2026-10-06/`):** the untried lineage names closed negative — **nybot.com** holds zero 200-status captures in 2009-2013 (the whole domain is redirects and images; the 2007 `marketInfo/tradingSchedule` pages are pre-floor observations), and **theice.com**'s productguide contract-spec family (`DelayedData.shtml?specId=N`) first appears 2011-11-20, its `products/*.jhtml` pages 2012-06-10 (the 2026-09-30 finding), with the only pre-August-2011 productguide captures being literal `specId={0}` template crawls; the operator's own marketdata Holiday/Special-Trading-Hours calendar pages (captures 2011-10-16, 2011-12-08 and 2011-12-13) render client-side and carry no event data in their bytes; and the CFTC rule submission 12-1 of 2012-01-10 (`12-1_Sugar_trading_Hours.pdf`, retrieved) is the 2012-01-30 hours change the module already carries, post-dating the master table. Dormant-to-served under the OANDA programme, so the remaining gaps are tracked in the maintainer's consolidated historical-research issue [#332](https://github.com/SharurTrading/exchange-hours-rs/issues/332).
- **horizon precision** — the AUGUST 2011 edition of the master table is month-dated by ICE and carries no day, so the ledger horizon is written as `2011-08-01`, the first day of the edition's own month. No ICE document states a day-level date for that edition.
- **order-entry** — the Friday-evening leg of the 20:00 pre-open is omitted. ICE runs it "on the prior Exchange business day", so a Monday trade date is fed by the preceding Friday evening; a wrapping `SessionRule` always wraps into the next local day, so a Friday rule would assert order entry on Saturday morning instead. The omission is a limit of the normal-week rule model, not a claim that ICE closes order entry over the weekend.
- **not modelled** — the January 2012 notice also announced a standing summer adjustment to 02:30 NY, which ICE itself called a "temporary change to the opening time". Seasonal opening shifts are exceptional-day changes under LAW-HOLIDAY-SCOPE and are not normal-week revisions.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_sugar.rs on 2026-09-12 UTC)

Sugar No. 11 runs one same-day executable session; the ICE master hours table
carries no footnote marker on its row, so nothing commences on the previous
calendar evening. Order entry is a separate, non-matching phase.

The 20:00 pre-open is modelled Monday-Thursday. ICE runs it "on the prior
Exchange business day", so a Monday trade date is fed by the preceding Friday
evening. That leg is not expressible here: a wrapping SessionRule always
wraps into the NEXT local day, so a Friday rule would assert order entry on
Saturday morning rather than carrying through to Monday. The Friday PCPO is
unaffected and is modelled MON_FRI, because it opens and closes inside one
local day. The omission is a limit of the normal-week rule model, not a claim
that ICE closes order entry over the weekend.

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/23/Sugar-No-11-Futures

---

Two order-entry-only phases: the post-close pre-open ("PCPO") beginning 30
minutes after the 13:00 close, and the regular pre-open from 20:00 running to
the next morning's open.

Both are classified order_entry, not extended: neither matches. The 2018
notice that created the PCPO calls it an extension of the "pre-open order
entry session" and kills Day orders entered in it at its end, and the
pre-open only accepts orders ahead of the Opening Match that occurs at the
open itself. Sugar No. 11 publishes no tradeable phase outside its executable
session, so the extended slice is empty.

---

Baseline before 2012-01-30. The earliest primary statements of the Sugar grid inside the modelled window are both March 2010: ICE's DST notice (day unstated, `exnot03XX10DST.pdf`) names "the normal 3:30 am NY time" open, and the operator's Sugar brochure (`IFUS-SUGAR-BROCHURE-2010`, document date 22 March 2010) prints the first full grid, `HOURS  0330 EST TO 1400EST` — the same electronic 03:30-14:00 the August 2011 master table shows, so the carried baseline is witnessed, not merely carried. No primary source gives a January 2010 close, so the grid is still keyed from the August 2011 edition and carried back over the unwitnessed head of the span rather than inventing an earlier cutover.

2026-09-30: the theice.com product-page channel was swept for pre-August-2011
editions (domain-wide CDX 2009-2012, 12 503 collapsed url keys; saved beside the
other raw directories in the research store's
`holidays/raw/equities/NORMAL-WEEK-RETRIEVALS-2026-09-30.md`). The product
pages (`sugar.jhtml`, `coffee.jhtml`, `cocoa.jhtml`, `cotton.jhtml`) first
appear in the archive at 2012-06-10 — after the master table's earliest edition
— and no 2009-2011 product or specification page printing a softs grid
survives on that channel. **Superseded 2026-10-13 UTC:** that sweep's
terminal-answer conclusion ("no in-gap document can exist") was scoped to the
product-page channel and the master-table class; the 2026-10-13 wave's
name-level sweep of every sugar-named theice.com artifact 2008-2012 (13 rows)
found the operator brochure outside both, and the conclusion no longer holds.

2026-09-01: WHY THE 2010-2011 INTERVAL WAS TAKEN TO BE UNSOURCABLE. ICE Futures U.S.
sets these hours administratively, not by rule. Its product rulebook chapters
- Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured
December 2011 - contain no hours provision at all, and chapter 4 is
trade-practice rules. No SEC or CFTC filing therefore fixes an ICE Futures
U.S. trading hour, and the master hours table was taken to be the only source;
its earliest surviving edition is August 2011. **Superseded 2026-10-13 UTC:**
the reasoning about filings and rulebooks stands, but the master table is no
longer the only source class — the operator's own Sugar brochure of 22 March
2010 (`IFUS-SUGAR-BROCHURE-2010`) states the grid inside the interval, so the
interval is witnessed from March 2010 onward and only January to mid-March
2010, the holiday layer and the January-2010 close remain unwitnessed.
https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf
https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf
https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf
2026-08-31: confirmed negative. Every archived edition of the master table
was enumerated and the AUGUST 2011 one (captured 2011-12-12) is the earliest
in existence; there is no 2010 edition to consult. The JANUARY 2, 2013
edition reads "Sugar No. 11(R)  2:30 - 14:00", consistent with the dated
2012-01-30 revision already modelled. The January-2010 close therefore
remains unsourced by document availability, not by an unfinished search.
Dated editions of ICE's own master table, official origin
https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
delivered via
https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf

https://www.ice.com/publicdocs/futures_us/exchange_notices/exnot03XX10DST.pdf
https://web.archive.org/web/20111212140120/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf

---

2012-01-30: "Effective with the start of trading on Monday, January 30, 2012
  the daily electronic trading session for Sugar No. 11 futures and options
  contracts will begin at 1:30 am NY time."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot121911S11_Hours_12312.pdf
2012-11-05: "Effective with the start of trading on Monday, November 5, 2012
  the daily trading session for Sugar No. 11 futures and options contracts
  will begin at 2:30 am NY time."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/1018912ExNotS11Hours.pdf
2014-02-03: "Effective with the start of trading for trade date Monday,
  February 3, 2014, the Exchange will implement changes to daily trading hours
  for Sugar No. 11 ... Sugar No. 11 3:30 13:00"
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
  order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
  and Sugar No. 16 futures contracts will be enhanced by the addition of a new
  post-close pre-open ("PCPO") session that will start at 30 minutes after the
  end of trading for the contract and end at 6:00 pm on the Exchange business
  day prior to each trading day."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
Evidence: docs/evidence/ice_us_sugar.md

## 2026-10-31 UTC post-close queue correction

The already-cited `IFUS-NOTICE-2026-CHRISTMAS` was retrieved again at
2026-10-31 22:05:03 UTC. Its bytes are identical to the 2026-10-23 capture
in the Documents table (SHA-256
`a7658155b57c51085cd3b36abc9ff6b91f3ca353cda88365a06459ebb40f0801`).
The second PDF capture and layout text are under
`reviews/2026-10-31-future-update/ice-americas/` in the research store, with
URL and retrieval metadata in `ice-christmas-2026.FETCH.json`. The annual
window remains 2025-01-01..2028-01-03.

Matching on 2026-12-24 ends at 13:00 NY; Sugar's own hours are
regular while the four other modelled softs have the notice's 13:05 early
close. The notice separately states the Post-Close Pre-Open starts thirty
minutes after each contract closes and ends at 15:30 NY. Thus this family's
queue is 13:30–15:30 NY on Thursday 2026-12-24. It is order entry only.

The next matching session is Monday 2026-12-28, 08:00–13:00 NY. The
notice expressly gives FCOJ its regular open and the other four modelled
softs an 08:00 late open, and keeps every regular close. For Cotton this
creates a Monday daytime session even though its normal grid has no Sunday
evening leg. A scalar late-open clip cannot create that session or preserve
the Thursday queue. A complete `ReplacementBlocks` row therefore states the
Thursday queue at offset -4 and the Monday matching block. The queue carries
2026-12-28, the trade date of the session it feeds, under the charter's
Post-Close convention; no matching on the closed 2026-12-25 is invented.
Monday's own post-close queue still feeds 2026-12-29 and remains unchanged.
TAS and settlement windows are excluded by LAW-SESSION-NOT-EXPIRY.

The two new exception dates are forward publications confirmed against the
operator on 2026-10-31 UTC. The source is sufficiently specified for the
blocks above; the published 2027 `open1` dates remain withheld.

**Forward confirmation:** re-confirm the published 2026-12-24 and
2026-12-28 arrangements, plus Cotton's derived 2026-12-29 replacement,
against the operator by 2026-12-23 UTC under LAW-WATCH; tracked in
[#374](https://github.com/SharurTrading/exchange-hours-rs/issues/374).
