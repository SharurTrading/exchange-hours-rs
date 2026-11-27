<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_cotton` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_cotton.rs`](../../src/calendar/schedules/futures/us/ice_cotton.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable.** The normal-week horizon remains August 2011. ICE's March 2009 notice witnesses the same opening clock earlier, but the intervening amendments through the floor have not been established. The holiday audit starts in 2025.

## Revision rows

- 2014-02-02 — T1 — ICE ExNot 012714 hours — the Sunday opening starts the first session ending at 14:20 NY on trade date Monday 2014-02-03. The previous close was 14:30.
- 2018-10-05 — T1 — ICE PCPO notice 20180920 — Friday 14:50–18:00 NY is the first post-close pre-open, for Monday 2018-10-08.

## Holidays

**Coverage:** 2025-01-01..2028-01-03 (inclusive trade dates). Tier: T1 throughout.

Below the window, the span the sourced week covers answers its **session** questions as the one-flank bridged residual since the 2026-10-07 ruling (AGENTS.md "Modeling conventions", #296 — the metadata reports `HolidayWindowsBridged`, the complete-calendar claim stays withheld, and the dates below this family's carried horizon keep refusing) while the **holiday-table classification** (`is_closed_trade_date`) refuses a typed `UnresolvedGap` on every span date: nothing witnesses the pre-2025 holiday layer, and no closure is asserted that no operator statement witnesses.

### Documents

Every artifact below is saved in the research store, and each row's own section quotes the bytes it keys.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `IFUS-COTTON-2009` | 2009-03-29 opening | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot031909aghours.pdf> | retrieved live from the operator, 2026-11-27 01:34:50 UTC | T1 | `ee9455593273f98046e9254b3d9215029b1a4221468c7e6d7505034444739d5e` |
| `ICE ExNot 012714 hours` | 2014-02-02 opening | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf> | retrieved live from the operator, 2026-11-27 01:36:03 UTC | T1 | `95c48deb127cb55cd1c60734c1a545c9a25289dc963d91bac90c60b92552a299` |
| `ICE PCPO notice 20180920` | 2018-10-05 opening | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf> | retrieved live from the operator, 2026-10-31 22:23:50 UTC | T1 | `42a4e0ec45f59fae316924a8421bbdd779a3ee6e8b47aea08ad5944caa27b21d` |
| `IFUS-REGULAR-HOURS-2026-09` | September 2026 edition | <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> | retrieved live from the operator, 2026-10-31 22:23:50 UTC | T1 | `381c04fcb4d5b4093f72463246652ac8e24c3ca7b89283dfe86f736497c13ed2` |
| `IFUS-COTTON-PRODUCT` | current normal week | <https://www.ice.com/products/254/cotton-no-2-futures> | retrieved live from the operator, 2026-11-27 01:35:42 UTC | T1 | `3d214889bb0f37f1376d259b982dd3b2b0143443c5b8a1b7150f551cec41639c` |
| `IFUS-NOTICE-2025-NEWYEAR` | 2024-12-31 .. 2025-01-02 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025NewYearHoliday_20241015.pdf> | retrieved live from the operator, 2026-11-27 01:48:20 UTC | T1 | `83df4bd0d66c474fc58dedc5200c2ad281bc7baaee5abf3dadab8759ee6057b5` |
| `IFUS-NOTICE-2026-NEWYEAR` | 2025-12-31 .. 2026-01-02 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_NewYearHoliday_20251107.pdf> | retrieved live from the operator, 2026-09-26 07:21:21 UTC | T1 | `dbe530fd8933783b9779d237b2f7118fe3c6517d7cfddfd76197f12788ca2b98` |
| `IFUS-CAL-2025` | 2025-01-01 .. 2026-01-01 | <https://web.archive.org/web/20250523135551id_/https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> | Wayback `id_` replay of capture `20250523135551`, 2026-09-26 07:16:50 UTC | T1 | `0add2b10e7d6cb2a35b727db654e4ea87ed30970ec637f53e62dd044f553d049` |
| `IFUS-NOTICE-2025-MLK` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250213182737id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025MLKDay_Holiday_20241203.pdf> | Wayback `id_` replay of capture `20250213182737`, 2026-09-26 07:16:29 UTC | T1 | `d6e7a3c0672fee4364cd610c88e861f5175de1494a7bebd1d8df43a790ef0364` |
| `IFUS-NOTICE-2025-PRESIDENTS` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250225071405id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025PresidentsDay_Holiday.pdf> | Wayback `id_` replay of capture `20250225071405`, 2026-09-26 07:16:30 UTC | T1 | `9c5946b286419b3670147e35c8dd752634b050cccd26fc564bccbf3f3fdf57b1` |
| `IFUS-NOTICE-2025-GOODFRIDAY` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250409083118id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_GoodFridayHoliday20250210.pdf> | Wayback `id_` replay of capture `20250409083118`, 2026-09-26 07:16:32 UTC | T1 | `18342cf005b428e18d15348d660126de3c9decb6b2ff93ffbc2fedefa116596c` |
| `IFUS-NOTICE-2025-MEMORIAL` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Memorial_DayHoliday_20250303.pdf> | retrieved live from the operator, 2026-09-26 07:21:21 UTC | T1 | `b06fbcb489331e7b9a2e4f7d2b092d63459771b1fb939e1c7ed6cd4a1b782fc7` |
| `IFUS-NOTICE-2025-JUNETEENTH` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250508170956id_/https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_JuneteenthHoliday_20250424.pdf> | Wayback `id_` replay of capture `20250508170956`, 2026-09-26 07:16:34 UTC | T1 | `851ba17addf37f89007be95f8f3943ca0017204873bcf94522ad96cca68478d7` |
| `IFUS-NOTICE-2025-LABORDAY` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_LaborDayHoliday_20250715.pdf> | retrieved live from the operator, 2026-09-26 07:21:20 UTC | T1 | `9768ce292b4c55c8171285444059a445a088e9ebc29058354ffbbf252373e036` |
| `IFUS-NOTICE-2025-THANKSGIVING` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Thanksgiving_Holiday_updated.pdf> | retrieved live from the operator, 2026-09-26 07:21:20 UTC | T1 | `aa643597896431140300de04f60ea68c725dd1ad90e66f22d2cd5a9fe410d1db` |
| `IFUS-NOTICE-2025-INDEPENDENCE` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_IndDayHoliday_20250508.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `74b89e65fd51405d0df0fc4809785e470f8ed2823c18bd119fe6fd21d637cbe2` |
| `IFUS-NOTICE-2025-CHRISTMAS` | 2025-01-01 .. 2025-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `b5e92b1abf47947bef85e290f2efc16c9fcd9288ce40830d293c71aca1007809` |
| `IFUS-CAL-2026` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice_2026_Holiday_Calendar_20260609.pdf> | retrieved 2026-09-12 07:47 UTC | T1 | `b9723d9afa1c20a0792b628de36c7d3726add58ea3d94972a1e7cefd3e50bf4e` |
| `IFUS-NOTICE-2026-MLK` | trade dates 2026-01-16 .. 2026-01-20 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_MLKDay_Holiday_20251203.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `1cab5d8d23084cce002098769fa472aa8795bd410fc30a2e43850ec319030780` |
| `IFUS-NOTICE-2026-PRESIDENTS` | trade dates 2026-02-13 .. 2026-02-17 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Presidents_Day_Holiday_20251223.pdf> | retrieved live from the operator, 2026-10-09 23:49 UTC | T1 | `961d3a35f54f13477ab927610e75e445cc9acefa2c3dd2d7b5340f0a9080e76b` |
| `IFUS-CAL-2027` | 2027-01-01 .. 2028-01-03 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice-2027_Holiday_Calendar_20260604.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d2e39a2db2a26e0578ad0d09b36f51dfdad8502635020fe79d66303c09c1a040` |
| `IFUS-NOTICE-2026-GOODFRIDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_GoodFridayHoliday_20260202.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `0a38abd0c9c8b0f904e9701999ac56466610c46437bdfe0109f88e67da35d181` |
| `IFUS-NOTICE-2026-MEMORIAL` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Memorial_DayHoliday_20260310.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `3dfe2a49babece024ce3cd91948e4fb27740abe31282fd9121097bd8433298f9` |
| `IFUS-NOTICE-2026-JUNETEENTH` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_JuneteenthHoliday_20260320.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `34a21ef6facf3dafd72d444ea352fc60e9b2dbf4fc674c1f4f8c199936169edd` |
| `IFUS-NOTICE-2026-INDEPENDENCE` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US__IndDayHoliday_.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `9de2852ed397d403cf8061358d98839954dbe861bf349fb91f57f95910983ea6` |
| `IFUS-NOTICE-2026-LABORDAY` | 2026-01-01 .. 2026-12-31 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_LaborDayHoliday_20260708.pdf> | retrieved 2026-09-12 04:20 UTC | T1 | `d383fe9ae20065f4de6825bff68f13d9cdd8233ea57de0f40644af92439b88f4` |
| `IFUS-NOTICE-2026-THANKSGIVING` | trade dates 2026-11-26 .. 2026-11-27 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Thanksgiving_Holiday_20260923.pdf> | retrieved live from the operator, 2026-09-27 19:39 UTC | T1 | `f153d1066d398ee1a65a598ed8fada26f95e7ad24b26e43b1ba720e073e6d28c` |
| `IFUS-NOTICE-2026-CHRISTMAS` | trade dates 2026-12-24 .. 2026-12-28 | <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2026_Christmas_Holiday_20261022.pdf> | retrieved live from the operator, 2026-10-23 23:21 UTC | T1 | `a7658155b57c51085cd3b36abc9ff6b91f3ca353cda88365a06459ebb40f0801` |

**Notes on the documents.**
- The New Year timetables are saved as `normal-weeks/ice-us-cotton-2026-11-27/ICE_Futures_US_2025NewYearHoliday_20241015.pdf` and `holidays/raw/iceus-2025-2027/IFUS_2026_NewYearHoliday_20251107.pdf`. Both state regular Cotton trading on December 31 and January 2 and closure on January 1. The first supplies the pre-window queue for January 2 without extending pre-2025 holiday classification.
- The 2009 notice, 2014 reminder and product specification are saved under `normal-weeks/ice-us-cotton-2026-11-27/` as `ICE-2009-03-24-COTTON.pdf`, `ICE-2014-01-27.pdf` and `product.html`. The 2018 PCPO notice and September 2026 master are `ICE_PCPO_2018.pdf` and `ICE_Regular_Hours.pdf` in `reviews/2026-10-31-future-update/independent-final-us-review/`.
- `IFUS-CAL-2026` resolves to the Exchange-Notice copy, whose labels the rows follow; the Holiday-Hours-hub copy <https://www.ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf> (retrieved 2026-09-12 04:19 UTC, sha256 `da97503545a3fb7607948367d30896685e220b36abed24aa02bbe1a21acb2817`) carries the earlier product-group headers with every `open` / `closed` / `open1` status cell identical.
- `IFUS-CAL-2027` is an unconditional published future, which LAW-NO-FABRICATED-DATES permits encoding ahead of its effective days.
- The 2026 equity-index companion notice of the same date (sha256 `6f07e49c36c70ef0c81cf028c3c7f539339b03092c81b22cf9c583f9f0c6b9e2`) is last-trade-day and final-settlement content only and keys nothing here (LAW-SESSION-NOT-EXPIRY).
- The four notices `IFUS-NOTICE-2025-INDEPENDENCE`, `IFUS-NOTICE-2025-CHRISTMAS`, `IFUS-NOTICE-2026-MLK` and `IFUS-NOTICE-2026-PRESIDENTS` were retrieved live on 2026-10-09 UTC through the operator's own notices listing service — POST `https://www.ice.com/api/sitesearchservice/v1/search/websitenotices?searchCollections=futures_us_exchange_notice&year=<YYYY>`, the machine channel behind the client-side notices page — which names every notice with its live URL; the T2 listing captures are under `holidays/raw/iceus-notices-service-2026-10-09/` with the PDFs beside them.
- The research store holds the 2025 artifacts under `holidays/raw/iceus-2025-2027/`, with each file's URL, UTC retrieval time, sha256 and byte count in its `INDEX.md` and `INVENTORY.tsv`; the 2026-2027 artifacts are under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/` and `…-fix/`.

### 2025

The 2025 block is the `July 5, 2024` — `2025 Trading Holiday Calendar`'s own day-level spine, ICE's per-holiday Exchange Notices for the holidays it issued one for, the **revised** Thanksgiving notice that adds Friday 2025-11-28, and the two London-bank-holiday “Delayed Opens” notices that carry dates the calendar does not list. The dated rows use each notice or calendar together with the standing PCPO rule to derive the receiving trade date.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-01-01, “New Year’s Day”; the softs column prints `closed` |
| 2025-01-02 | replacement blocks | 2024-12-31 queue 14:50–18:00, prior evening 19:30–21:00, matching 21:00–14:20 NY | `IFUS-NOTICE-2025-NEWYEAR` | T1 | the prior Cotton business day is two dates back because the intervening day is closed. |
| 2025-01-20 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-MLK` | T1 | notice date Mon, Jan 20, group line “Sugar No. 11® and 16, Coffee “C” ®, Cotton No. 2®, Cocoa and FCOJ Contracts” |
| 2025-01-21 | replacement blocks | 2025-01-17 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2025-MLK` | T1 | Cotton is closed on 2025-01-20. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2025-02-17 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | notice date Mon, Feb 17; the same notice's group line also names Canola, which no crate identity models |
| 2025-02-18 | replacement blocks | 2025-02-14 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | Cotton is closed on 2025-02-17. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2025-04-18 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Fri, Apr 18, the shared softs/Canola group line |
| 2025-04-21 | replacement blocks | 2025-04-17 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | Cotton is closed on 2025-04-18. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2025-05-26 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-MEMORIAL` | T1 | notice date Mon, May 26, the softs group line |
| 2025-05-27 | replacement blocks | 2025-05-23 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2025-MEMORIAL` | T1 | Cotton is closed on 2025-05-26. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2025-06-19 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | notice date Thu, June 19 |
| 2025-06-20 | replacement blocks | 2025-06-18 queue 14:50–18:00, prior evening 19:30–21:00, matching 21:00–14:20 NY | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | the prior Cotton business day is two dates back because the intervening day is closed. |
| 2025-07-04 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-07-04, “Independence Day”; the softs column prints `closed` |
| 2025-07-07 | replacement blocks | 2025-07-03 post-close queue 14:50–18:00 NY; 08:00–14:20 matching NY | `IFUS-NOTICE-2025-INDEPENDENCE` | T1 | Cotton is closed on 2025-07-04. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2025-09-01 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-LABORDAY` | T1 | notice date Mon, Sep 1 |
| 2025-09-02 | replacement blocks | 2025-08-29 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2025-LABORDAY` | T1 | Cotton is closed on 2025-09-01. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2025-11-27 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Thu, Nov 27 |
| 2025-11-28 | replacement blocks | Wednesday 14:50–18:00 NY; Friday 08:00–13:30 NY | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | Wednesday regular, Thursday closed, Friday 08:00 late open and 13:30 early close; Wednesday’s post-close queue feeds Friday. |
| 2025-12-01 | replacement blocks | Friday 14:00–15:30, Sunday 19:30–21:00, matching Sunday 21:00 to Monday 14:20 NY | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | Friday post-close queue runs from 30 minutes after the 13:30 close to 15:30; the standing rule assigns it to Monday. |
| 2025-12-24 | early close | `Early Close at 1:05 pm for Cotton, Coffee “C”, Cocoa and FCOJ.` (TAS trading for Cotton ends 1:00 pm) — 13:05 NY | `IFUS-NOTICE-2025-CHRISTMAS` | T1 | notice date Wed, Dec 24; the notice also starts that eve's Post-Close Pre-Open order entry 30 minutes after each product's close and ends it at 3:30 pm — an order-entry instant this paragraph records beside the row, and not a session boundary |
| 2025-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-25, “Christmas Day”; the softs column prints `closed` |
| 2025-12-26 | replacement blocks | Wednesday 13:35–15:30; Friday 07:30–14:20 NY | `IFUS-NOTICE-2025-CHRISTMAS` | T1 | The Christmas Eve note begins the queue 30 minutes after the 13:05 close, and ends it at 15:30. Friday has a 07:30 late open and regular close. |

**Gaps, 2025:** **The 2025 Independence Day and Christmas notices were retrieved on 2026-10-09 UTC**, live from the operator through its own notices listing service (the machine channel behind the client-side notices page), closing the #168 channel — and both moved Cotton rows, exactly where the earlier record said they would. The Independence Day notice gives Cotton its own Monday late open: `Late Open for Cotton: 8:00 am` on Mon, July 7, with `Regular open times for all other contracts` — the 2025 twin of 2026-07-06 — and that row is above. The Christmas notice gives Cotton the 1:05 pm Christmas-eve early close (with the Post-Close Pre-Open ending at 3:30 pm, recorded beside the row) and names Cotton No. 2 in Boxing Day's 07:30 late open; both rows are above. **Canola** and **`Daily Gold and Silver`** are product groups on ICE's calendar with no crate identity in this block. **2025-12-31 is normal, not unsourced:** the 2026 New Year's notice's `Wed, Dec 31` column prints `Regular Hours` for the group that names Cotton No. 2.

**Interpretive steps, 2025:** Cotton shares the softs group line on every 2025 notice and is the only family the revised Thanksgiving notice gives a late open to: `Late Open for Cotton - 8:00 am`. Its normal matching open applies on Easter Monday — the Good Friday notice names “Sugar No. 11, Coffee “C” and Cocoa” and adds “Regular Hours for all other contracts” — and the family takes no London-bank-holiday row because those notices name the same three softs and state that “All other Exchange products will follow their regular trading schedules”. Cotton is a wrapping contract: the session for trade date D commences 21:00 NY on D-1, so the 2025-11-28 row's late open is the observable deletion of the Thanksgiving-evening leg and its early close ends the Friday session at 13:30.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-01, “New Year's Day” |
| 2026-01-02 | replacement blocks | 2025-12-31 queue 14:50–18:00, prior evening 19:30–21:00, matching 21:00–14:20 NY | `IFUS-NOTICE-2026-NEWYEAR` | T1 | the prior Cotton business day is two dates back because the intervening day is closed. |
| 2026-01-19 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-19, “Martin Luther King Day” |
| 2026-01-20 | replacement blocks | 2026-01-16 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2026-MLK` | T1 | Cotton is closed on 2026-01-19. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-02-16 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-02-16, “Presidents Day” |
| 2026-02-17 | replacement blocks | 2026-02-13 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2026-PRESIDENTS` | T1 | Cotton is closed on 2026-02-16. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-04-03 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Fri, Apr 3 |
| 2026-04-06 | replacement blocks | 2026-04-02 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | Cotton is closed on 2026-04-03. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-05-25 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25 |
| 2026-05-26 | replacement blocks | 2026-05-22 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2026-MEMORIAL` | T1 | Cotton is closed on 2026-05-25. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-06-19 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19 |
| 2026-06-22 | replacement blocks | 2026-06-18 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | Cotton is closed on 2026-06-19. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-07-03 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3 |
| 2026-07-06 | replacement blocks | 2026-07-02 post-close queue 14:50–18:00 NY; 08:00–14:20 matching NY | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | Cotton is closed on 2026-07-03. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-09-07 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7 |
| 2026-09-08 | replacement blocks | 2026-09-04 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-NOTICE-2026-LABORDAY` | T1 | Cotton is closed on 2026-09-07. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2026-11-26 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26, “Thanksgiving Day” |
| 2026-11-27 | replacement blocks | Wednesday 14:50–18:00 NY; Friday 08:00–13:30 NY | `IFUS-NOTICE-2026-THANKSGIVING` | T1 | Wednesday regular, Thursday closed, Friday 08:00 late open and 13:30 early close; Wednesday’s post-close queue feeds Friday. |
| 2026-11-30 | replacement blocks | Friday 14:00–15:30, Sunday 19:30–21:00, matching Sunday 21:00 to Monday 14:20 NY | `IFUS-NOTICE-2026-THANKSGIVING` | T1 | Friday post-close queue runs from 30 minutes after the 13:30 close to 15:30; the standing rule assigns it to Monday. |
| 2026-12-24 | early close | `Early Close for Coffee “C”, Coffee “C”® Metric, Cotton, Cocoa and FCOJ - 1:05 pm` — 13:05 NY; the Post-Close Pre-Open starts 30 minutes after close and ends at 3:30 pm | `IFUS-NOTICE-2026-CHRISTMAS` | T1 | notice dated October 22, 2026, Thu, Dec 24 column of the softs group line plus its NOTE |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25, “Christmas Day” |
| 2026-12-28 | replacement blocks | matching 08:00–14:20 NY; prior Thursday queue 13:35–15:30 NY | `IFUS-NOTICE-2026-CHRISTMAS` | T1 | the 2026-12-24 post-close queue is carried at offset -4 to the session it feeds; FCOJ's Monday open is regular |

**Gaps, 2026:** **The 2026 MLK and Presidents Day notices exist** — the earlier record that none did was an archive-channel failure, corrected on 2026-10-09 UTC when the operator's own notices listing service named both (issued 2025-12-03 and 2025-12-23, retrieved live the same day). Both print `Closed` for the softs group on the holiday, which the calendar rows above already stated; the following Tuesday rows carry the preceding Friday’s PCPO. **The 2026 Christmas notice issued on October 22, 2026 and was retrieved live 2026-10-23 UTC** (`IFUS-NOTICE-2026-CHRISTMAS`): its Christmas-eve column gives Cotton the 1:05 pm early close — the Post-Close Pre-Open starts 30 minutes after the close and ends at 3:30 pm, an order-entry instant recorded beside the row — and its Boxing Day column names Cotton No. 2 in the 8:00 am late open; both rows are above. The 2026-12-28 replacement combines the Thursday queue with the Monday 08:00–14:20 session. The 2026 Thanksgiving notice issued on 2026-09-23 and is read below; the DST-end notice of 2026-09-25 is read above. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice.

**Interpretive steps, 2026:** ICE's calendars key one line per product group, not per contract, and the group “Cocoa, Coffee “C”®, Coffee "C"® Metric, Cotton No 2®, FCOJ, Sugar No. 11 and No. 16 Contracts” is the line this family sits on; the per-holiday notices restate the same set as “Sugar No. 11® and 16, Coffee “C”®, Coffee "C"® Metric, Cotton No. 2®, Cocoa and FCOJ”. Where a notice names a subset — Easter Monday's “Sugar No. 11, Coffee “C” and Cocoa”, and 2026-07-06's “Cotton” — only the named families take the row and the notice's own “Regular Hours for all other contracts” is the positive statement of normality for the rest. 2026-10-12 is normal for the same reason. Cotton's session does **not** run inside one New York civil day — it opens at 21:00 NY on D-1 and closes at 14:20 NY on D — but ICE names the civil date on which the affected session closes, and that closing date is the crate's trade date, so the conversion from ICE's civil date to the crate's trade date is still the identity. Cotton is **not** part of the 2026-04-06 late open. The July notices move Monday matching to 08:00 NY. Both Thanksgiving notices leave Wednesday regular, close Thursday and give Friday matching at 08:00–13:30 NY. Their Friday post-close order-entry window begins 30 minutes after matching ends and ends at 15:30. The following Monday's replacement owns Friday 14:00–15:30, the ordinary Sunday 19:30–21:00 pre-open and the overnight session ending Monday 14:20.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-01 |
| 2027-01-04 | replacement blocks | 2026-12-31 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-01-01. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-01-18 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-01-18 |
| 2027-01-19 | replacement blocks | 2027-01-15 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-01-18. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-02-15 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-02-15 |
| 2027-02-16 | replacement blocks | 2027-02-12 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-02-15. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-03-26 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-03-26 |
| 2027-03-29 | replacement blocks | 2027-03-25 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-03-26. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-05-31 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-05-31 |
| 2027-06-01 | replacement blocks | 2027-05-28 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-05-31. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-06-18 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-06-18 |
| 2027-06-21 | replacement blocks | 2027-06-17 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-06-18. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-07-05 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-07-05 |
| 2027-07-06 | replacement blocks | 2027-07-02 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-07-05. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-09-06 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-09-06 |
| 2027-09-07 | replacement blocks | 2027-09-03 post-close queue 14:50–18:00 NY; 19:30–21:00 pre-open and 21:00–14:20 matching NY | `IFUS-CAL-2027` | T1 | Cotton is closed on 2027-09-06. Standing PCPO rule dates the prior business day’s queue to this session. |
| 2027-11-24 | unsourced | holiday timetable unpublished | `IFUS-CAL-2027` | T1 | the annual gives the closure date; the operator’s holiday timetable is needed to determine the post-close queue and its receiving session |
| 2027-11-25 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-11-25 |
| 2027-11-26 | unsourced | holiday timetable unpublished | `IFUS-CAL-2027` | T1 | the annual gives the closure date; the operator’s holiday timetable is needed to determine the post-close queue and its receiving session |
| 2027-11-29 | unsourced | prior Cotton post-close queue timetable unpublished | `IFUS-CAL-2027` | T1 | the queue feeding this trade date depends on the preceding holiday hours; the annual does not state its boundaries. |
| 2027-12-23 | unsourced | holiday timetable unpublished | `IFUS-CAL-2027` | T1 | the annual gives the closure date; the operator’s holiday timetable is needed to determine the post-close queue and its receiving session |
| 2027-12-24 | closed | `closed` — no instant printed | `IFUS-CAL-2027` | T1 | ICE calendar date 2027-12-24 |
| 2027-12-27 | unsourced | holiday timetable unpublished | `IFUS-CAL-2027` | T1 | the annual gives the closure date; the operator’s holiday timetable is needed to determine the post-close queue and its receiving session |
| 2027-12-28 | unsourced | prior Cotton post-close queue timetable unpublished | `IFUS-CAL-2027` | T1 | the queue feeding this trade date depends on the preceding holiday hours; the annual does not state its boundaries. |

**Gaps, 2027:** **Every 2027 `open1` date carries no instant by construction.** ICE announces those hours “in advance of the respective holiday via Exchange Notices”, and none had issued at retrieval, so the annual calendar establishes the printed statuses. The standing PCPO rule determines the receiving trade date wherever the adjacent Cotton clocks are known. Closed by the 2027 per-holiday Exchange Notices as they are issued. Coverage ends 2028-01-03, the last trade date the 2027 calendar names. Its last two dates are audited normal on the calendar itself: 2027-12-31 is not on ICE's 2027 calendar at all, and on 2028-01-03 — New Year's Day observed, since 2028-01-01 is a Saturday — every product group a crate identity routes to prints `open` and only Canola, which no crate identity models, prints `closed`; so neither date carries a row.

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

## Normal week and holiday queues

ICE’s 24 March 2009 notice, page 1, applies its Cotton electronic session from Sunday 29 March for Monday 30 March, at 21:00–14:30 New York time. Page 2 explicitly sets the Cotton pre-open at 19:30 on that Sunday. The August 2011 and January 2013 regular-hours tables repeat 21:00–14:30. The 2014 reminder keeps the open and advances the close to 14:20 starting with the Monday trade date. The September 2026 master retains 21:00–14:20. The master’s single asterisk identifies the previous business day. The double and triple asterisks specify *different Sunday start times* for other products. Cotton’s own effective notice gives its Sunday time directly.

The current product specification gives 19:30 pre-open, 21:00 open, 14:20 close and `closeNextDay: true` for New York. The 2025 and 2026 Good Friday notices both close Cotton on Friday and assign regular hours to Cotton on Easter Monday, which corroborates Sunday evening as the opening day of the Monday session. The executable week has five opens, Sunday through Thursday. Rule 4.22 limits the regular pre-trading phase to order entry before the opening match. Cotton publishes no separate extended matching hours. The trade date follows the final matching close; settlement and TAS do not supply its boundary.

ICE’s 20 September 2018 PCPO notice puts the post-close queue on the prior Exchange business day, beginning 30 minutes after the contract ends trading and ending at 18:00 NY. Its example places Monday 8 October’s queue on Friday 5 October. The normal Cotton close therefore gives 14:50–18:00. The queue accepts GTD, GTC and GTD&T orders and amendments. A Day order entered then is killed at its end. Sunday’s 19:30 regular pre-open is a separate order-entry window.

For the holiday rows, the trade-date assignment is derived from that standing rule and the cited Cotton holiday schedule. After a normal Thursday followed by a closed Friday, Thursday’s queue feeds Monday. Before a closed Monday, the ordinary Friday queue feeds Tuesday; that Monday evening can still open Tuesday’s session. Source times for a shortened post-close queue take precedence. Each replacement states the complete matching session and its queues. The 2027 annual calendar explicitly closes the named Cotton trade dates, and its softs column does not carry the financial groups’ `open1` qualification. Unlisted adjacent trade dates use the published ordinary clock within the audited window, subject to later date-specific notices. The 2027 Thanksgiving Friday and observed Boxing Day arrangements need those notices. The annual does not give the changed matching or queue clocks. The receiving trade dates 2027-11-26 and 2027-12-27, and the 2027-11-24 and 2027-12-23 lead-in dates whose post-close queues cannot yet be attached to complete sessions, are withheld as whole dates until ICE publishes the holiday hours. The following trade dates, 2027-11-29 and 2027-12-28, also own a queue with unpublished boundaries. The holiday table withholds the complete trade date even when its overnight matching window would otherwise follow the ordinary grid. The explicit last-known-week policy can provide that grid. The intervening holiday closures remain in the table.

## Gaps and residual risks

The August 2011 master is month-dated, so 2011-08-01 identifies the month of the earlier audited normal week. No exchange cutover is inferred on that day. The 2009 notice also witnesses this Sunday opening clock, but complete continuity from the 2010 floor to August 2011 remains unverified. A period-specific schedule or the intervening amendments would close that historical gap. ICE sets hours administratively; the contemporary Cotton rulebook and Chapter 4 do not supply a trading-hours table. Holiday classification before 2025 remains unaudited.

## 2026-10-31 UTC post-close queue correction

The already-cited `IFUS-NOTICE-2026-CHRISTMAS` was retrieved again at
2026-10-31 22:05:03 UTC. Its bytes are identical to the 2026-10-23 capture
in the Documents table (SHA-256
`a7658155b57c51085cd3b36abc9ff6b91f3ca353cda88365a06459ebb40f0801`).
The second PDF capture and layout text are under
`reviews/2026-10-31-future-update/ice-americas/` in the research store, with
URL and retrieval metadata in `ice-christmas-2026.FETCH.json`. The annual
window remains 2025-01-01..2028-01-03.

Matching on 2026-12-24 ends at 13:05 NY; Sugar's own hours are
regular while the four other modelled softs have the notice's 13:05 early
close. The notice separately states the Post-Close Pre-Open starts thirty
minutes after each contract closes and ends at 15:30 NY. Thus this family's
queue is 13:35–15:30 NY on Thursday 2026-12-24. It is order entry only.

The next matching session is Monday 2026-12-28, 08:00–14:20 NY. The
notice expressly gives FCOJ its regular open and the other four modelled
softs an 08:00 late open, and keeps every regular close. The complete replacement states the
Thursday queue at offset -4 and the Monday matching block. The queue carries
2026-12-28, the trade date of the session it feeds, under the charter's
Post-Close convention; no matching on the closed 2026-12-25 is invented.
Monday's own post-close queue still feeds 2026-12-29 and remains unchanged.
TAS and settlement windows are excluded by LAW-SESSION-NOT-EXPIRY.

The two new exception dates are forward publications confirmed against the
operator on 2026-10-31 UTC. The source is sufficiently specified for the
blocks above; the published 2027 `open1` dates remain withheld.

**2026-11-27 UTC review.** With the Sunday opening restored, Christmas Friday’s raw queue belongs to the 28 December replacement and is suppressed there. Tuesday 2026-12-29 follows the ordinary week, including both Monday queues. It needs no holiday row.

**Forward confirmation completed — 2026-10-31 23:21:52 UTC:** The publication check
tracked in [#374](https://github.com/SharurTrading/exchange-hours-rs/issues/374)
was completed before its 2026-12-23 confirm-by date. A fresh live retrieval of
`IFUS-NOTICE-2026-CHRISTMAS` at 2026-10-31 23:19:44 UTC has SHA-256
`a7658155b57c51085cd3b36abc9ff6b91f3ca353cda88365a06459ebb40f0801`,
identical to the original Documents-table artifact. Original retrieval history
is retained. Cotton keeps the 13:05 NY Christmas-eve close; its 13:35–15:30 queue feeds the 28 December 08:00–14:20 session. The ordinary week supplies the 29 December session and both Monday queues.

The current ICE notices listing was also retrieved at 2026-10-31 23:19:44 UTC;
no superseding Christmas-hours notice was identified among its latest ten
entries, dated 2–29 October. Exact URLs, retrieval instants, the PDF and
supporting ordinary-hours/PCPO documents, and the checks against merged data
are retained in `reviews/2026-10-31-confirm-374/ice/` in the research store
(`MANIFEST.json`, `REPORT.md` and `MERGED_DATA_CHECK.json`). This confirms the
published arrangements as checked; it does not witness future production
trading or establish that later notices cannot change them.
