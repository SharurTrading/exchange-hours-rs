<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_us.rs`](../../src/calendar/schedules/futures/us/ice_us.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

NYSE FANG+ Index Futures only, including the sourced 2017-11-07 launch-eve opening through the dated key API.

## Revision rows

The key shares one timeline with the `iceus` exchange row, so its days are the
same two:

- 2017-11-07 — T1 — ICE FANG+ launch notice 20170926 — the launch-eve profile: a Tuesday 19:30–20:00 ET Pre-Open and the 20:00 matching start, and nothing earlier that day.
- 2017-11-08 — T1 — ICE FANG+ launch notice 20170926 — the full grid for trade date 2017-11-08: Sunday 18:00 open, Monday–Thursday 20:00 opens, matching through 18:00 the next day, with the 30-minute Pre-Open queues.

Everything below the first row is `CLOSED_NEW_YORK`, a sourced closure, so the
row's horizon is `—`.

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

The 2025 block is the `July 5, 2024` — `2025 Trading Holiday Calendar`'s day-level spine and the per-holiday Exchange Notices for the holidays ICE issued one for. The two notices it did not issue are the reason this family's 2025 rows include four `Unsourced` dates: those are the `open1` cells and the eve hours that only such a notice resolves. Every row below was read from the cited artifact's own bytes.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-01-01, “New Year’s Day”; every group column prints `closed` |
| 2025-01-20 | early close | `Early Close – 1:00 pm, TAS trading will not be held` (SETTLEMENT WINDOWS `12:59-1:00 pm2`) — 13:00 NY | `IFUS-NOTICE-2025-MLK` | T1 | notice date Mon, Jan 20, bullet “NYSE Stock Index, MSCI Bond Index, and ICE Mortgage, Bond Index and SOFR Index Contracts”; see the interpretive step below |
| 2025-02-17 | early close | `Early Close – 1:00 pm, TAS trading will not be held` — 13:00 NY | `IFUS-NOTICE-2025-PRESIDENTS` | T1 | notice date Mon, Feb 17, the same bullet; the notice's separate `MSCI Stock Index` bullet prints 13:00 as well |
| 2025-04-18 | closed | `Closed`; SETTLEMENT WINDOWS `None` | `IFUS-NOTICE-2025-GOODFRIDAY` | T1 | notice date Fri, Apr 18, bullet “U.S. Dollar Index®, Currency Pair, ICE Mortgage, Bond and SOFR Index Contracts, MSCI Stock and Bond Index, and NYSE Stock Index Contracts” |
| 2025-05-26 | early close | `Early Close – 1:00 pm, TAS trading will not be held` (SETTLEMENT WINDOWS `12:59-1:00 pm1`) — 13:00 NY | `IFUS-NOTICE-2025-MEMORIAL` | T1 | notice date Mon, May 26, bullet “NYSE Stock Index, MSCI Bond Index and ICE Mortgage, Bond Index and SOFR Index Contracts” |
| 2025-06-19 | early close | `Early Close – 1:00 pm, TAS trading will not be held` (SETTLEMENT WINDOWS `12:59-1:00 pm1`) — 13:00 NY | `IFUS-NOTICE-2025-JUNETEENTH` | T1 | notice date Thu, June 19, bullet “NYSE FANG+TM Index and ICE Stock, Mortgage and SOFR Index Contracts”, which names this family outright |
| 2025-07-03 | unsourced | no single cell — the Independence Day notice was not retrieved | `IFUS-CAL-2025` | T1 | the eve's hours are announced only in the per-holiday notice; the calendar does not list the date |
| 2025-07-04 | unsourced | `open1` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-07-04, “Independence Day”; the `Currency, Stock, SOFR, MSCI Bond and Mortgage Index` column prints `open1`, footnote 1 of which defers the hours to the notice |
| 2025-09-01 | early close | `Early Close – 1:00 pm, TAS trading will not be held` (SETTLEMENT WINDOWS `12:59-1:00 pm2`) — 13:00 NY | `IFUS-NOTICE-2025-LABORDAY` | T1 | notice date Mon, Sep 1, bullet “NYSE FANG+TM Index and ICE Stock, Mortgage and SOFR Index Contracts” |
| 2025-11-27 | early close | `Early Close - 1:00 pm, TAS trading will not be held` (SETTLEMENT WINDOWS `12:59-1:00 pm1`) — 13:00 NY | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Thu, Nov 27, bullet “NYSE FANG+TM Index and ICE Stock, Mortgage and SOFR Index Contracts” |
| 2025-11-28 | early close | `Early Close - 1:15 pm, TAS ends at 1:00 pm` — 13:15 NY | `IFUS-NOTICE-2025-THANKSGIVING` | T1 | revised notice, Fri, Nov 28, the same bullet |
| 2025-12-24 | unsourced | no single cell — the Christmas notice was not retrieved | `IFUS-CAL-2025` | T1 | the eve's hours are announced only in the per-holiday notice; the calendar does not list the date |
| 2025-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-25, “Christmas Day”; every group column prints `closed` |
| 2025-12-26 | unsourced | `open1` — no instant printed | `IFUS-CAL-2025` | T1 | ICE calendar date 2025-12-26, “Boxing Day”; the index column prints `open1`, whose hours the Christmas / Boxing Day notice would state |

**Gaps, 2025:** **The 2025 Independence Day and Christmas / Boxing Day notices were not retrieved.** Neither is in the Wayback Machine or Common Crawl, 44 live filename candidates returned 404 before the operator's host began rate-limiting, and the notices listing renders client-side so its archived copies name no notice. Their closing condition is page 2+ of the paginated report at <https://www.ice.com/futures-us/notices> (report id `futures_us_exchange_notice`), or the exact PDF URL; tracked as #168. This family is where the gap bites: 2025-07-04 and 2025-12-26 are `open1` on the calendar, and 2025-07-03 and 2025-12-24 are the customary early-close eves, so all four dates ship `Unsourced` rather than an invented instant or the silence that would claim they were audited normal. 2025-12-25 is `closed` on the calendar's own column and ships as a closure, but its trade-date deletion still reads the withheld 2025-12-24, so that query refuses too. **2026's MLK and Presidents Day notices do not exist**, and the late-2026 notices had not issued at retrieval; those dates are withheld above for the same reason and must not be filled from 2025's. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **2025-12-31 is normal, not unsourced:** the 2026 New Year's notice's `Wed, Dec 31` column prints `Regular Hours` for “MSCI Stock Index, NYSE FANG+TM Index, and ICE Stock Index Contracts”.

**Interpretive steps, 2025:** ICE prints FANG+ inside an `NYSE ... Index` bullet at the same instant as NYSE Stock Index, and three 2025 notices name it outright: Juneteenth and Labor Day, both “NYSE FANG+TM Index and ICE Stock, Mortgage and SOFR Index Contracts” with `Early Close – 1:00 pm`, and the revised Thanksgiving notice, which names it on Thu, Nov 27 at 13:00 and Fri, Nov 28 at 13:15. The other 2025 notices print the same 13:00 early close in a bullet headed “NYSE Stock Index, MSCI Bond Index, and ICE Mortgage, Bond Index and SOFR Index Contracts”, and the Good Friday notice closes that whole bullet outright; carrying those instants to FANG+ is the same interpretive step the 2026 rows already record, and it is corroborated inside each notice by the `MSCI Stock Index` bullet, which prints the same 13:00 close. FANG+ is a wrapping contract — the session for trade date D opens 20:00 NY on D-1 — so an early close clips the session that opened the previous evening, and the 2025-12-25 `Closed` row deletes the 2025-12-24 evening leg.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-01, “New Year's Day”, `closed` for the Currency / Digital Asset / Stock / SOFR / MSCI Bond / Mortgage Index group |
| 2026-01-19 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-01-19; the archive holds no 2026 MLK Exchange Notice |
| 2026-02-16 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-02-16; the archive holds no 2026 Presidents Day Exchange Notice |
| 2026-04-03 | late open and early close | `Late Open - 5:00 am / Early Close – 9:15 am, No TAS trading` — 05:00 and 09:15 ET | `IFUS-NOTICE-2026-GOODFRIDAY` | T1 | notice date Fri, Apr 3; ICE names “NYSE FANG+™” in this bullet. 05:00 is later than the family's normal Thursday 20:00 first open, so the Thursday-evening leg of this trade date does not run |
| 2026-05-25 | early close | `Early Close – 1:00 pm, TAS trading will not be held` — 13:00 ET | `IFUS-NOTICE-2026-MEMORIAL` | T1 | notice date Mon, May 25, “NYSE Stock Index, FTSE Stock Index, MSCI Bond Index and ICE Mortgage, Bond Index and SOFR Index Contracts” bullet (interpretive step below) |
| 2026-06-19 | early close | `Early Close – 1:00 pm, TAS trading will not be held` — 13:00 ET | `IFUS-NOTICE-2026-JUNETEENTH` | T1 | notice date Fri, June 19, same bullet (interpretive step below) |
| 2026-07-03 | early close | `Early Close – 1:00 pm, TAS trading will not be held` — 13:00 ET | `IFUS-NOTICE-2026-INDEPENDENCE` | T1 | notice date Fri, July 3, same bullet (interpretive step below) |
| 2026-09-07 | early close | `Early Close – 1:00 pm, TAS trading will not be held` — 13:00 ET | `IFUS-NOTICE-2026-LABORDAY` | T1 | notice date Mon, Sep 7, “NYSE FANG+™ Index, NYSE Stock Index, FTSE Stock Index, MSCI Bond Index and ICE Mortgage, Bond Index and SOFR Index” bullet — named, no interpretive step |
| 2026-11-26 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-11-26; the 2026 Thanksgiving notice had not issued at retrieval |
| 2026-12-25 | closed | `closed` — no instant printed | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-25 |
| 2026-12-28 | unsourced | `open1` — footnote 1: “Trading Hours for these contracts will be announced in advance of the respective holiday via Exchange Notices”; no such notice exists | `IFUS-CAL-2026` | T1 | ICE calendar date 2026-12-28, “Boxing Day”; the 2026 Christmas/New-Year notice had not issued at retrieval |

**Gaps, 2026:** **No 2026 MLK or Presidents Day Exchange Notice exists.** The ICE notices listing reaches back only to February 2026, and a Wayback CDX prefix enumeration of `ice.com/publicdocs/futures_us/exchange_notices*` for 2025-2027 (113 distinct URLs, saved as `ifus_notices_cdx_2025-2027.json`) holds neither; it holds the 2025 equivalents, which must not be read across years. So the gap is tested, not assumed. **The late-2026 notices had not issued at retrieval:** Thanksgiving 2026-11-26, Christmas Eve 2026-12-24, Boxing Day 2026-12-28 — and note that 2026-11-27 and 2026-12-24 are not holidays on the calendar at all, so their customary early closes exist only in notices that did not yet exist and this table therefore carries no row for either. **`Daily Gold and Silver` may close on further days** the calendar does not name, by the operator's own admission, but that product group has no crate key. **Canola** is on ICE's calendar and has no crate identity in this block. Each gap is closed by the corresponding Exchange Notice. The three 2026 notices that do not name NYSE FANG+ in their index bullet are an interpretive step, recorded below, not a gap.

**Interpretive steps, 2026:** **Which ICE bullet NYSE FANG+ sits on.** The 2026 and 2027 calendars key this family on the group “Currency, Digital Asset, Stock, SOFR, MSCI Bond, and Mortgage Index Contracts”, and the per-holiday notices split that group into an index bullet and a currency bullet. Two of the four 2026 notices that give instants name FANG+ explicitly and both put it with NYSE Stock Index at the same instant: Good Friday's “MSCI Stock and Bond Index, NYSE FANG+™, NYSE Stock Index and FTSE Stock Index” at 09:15, and Labor Day's “NYSE FANG+™ Index, NYSE Stock Index, …” at 13:00. The Memorial Day, Juneteenth and Independence Day notices print the same bullet without spelling FANG+ out, at 13:00. The rows for those three dates take 13:00 on that reading, calibrated twice by the operator's own spelling; the alternative would be to claim a date the calendar marks `open1` was audited normal, which is false. **What is not a late open.** Good Friday's 05:00 is a time of day below the family's normal 20:00 first-open time, so the crate reads the late open as 05:00 ET on the trade date itself rather than on the preceding local date; the Thursday-evening leg, which opens at 20:00 ET on Thursday and belongs to the same trade date, falls before that instant and is deleted, which is what ICE states. **TAS suspension keys nothing** — “No TAS trading” and “TAS trading will not be held” are trade-type restrictions, not session boundaries.

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

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The row rests on the same
`ICE-DERIVATIVES` documents as the `iceus` exchange row; the full annotated
list is in [`iceus.md`](iceus.md#sources). The documents that key the rows
above are:

- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf> — the ICE Futures U.S. FANG+ launch notice of 2017-09-26 — T1.
- <https://www.ice.com/products/66380320/NYSE-FANG-Index-Future> — the current NYSE FANG+ Index Future product page, which publishes the 17:30 Sunday and 19:30 weekday queue starts — T1.
- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — the ICE Futures U.S. *Regular Trading Hours* master table, June-2026 edition — T1.
- <https://www.ice.com/trading-hours> — ICE trading hours, the current-schedule monitoring entry point — T1.

## Gaps and residual risks

- **No dated-history gap.** The pre-launch era is a sourced closure and both
  rows rest on the operator's own launch notice.
- **Scope.** The key names the NYSE FANG+ Index Futures family only. It is not
  an ICE Futures U.S. venue clock, and it must not be reused for the six ICE
  Futures U.S. soft-commodity and index keys, which carry their own dated
  histories and their own January-2010 to August-2011 baseline gap.
- **Queue classification.** The Sunday 17:30–18:00 and Monday–Thursday
  19:30–20:00 phases are `order_entry`; the `extended` slice is empty because
  FANG+ publishes no tradeable phase outside its executable session. See
  [`iceus.md`](iceus.md#gaps-and-residual-risks).
- **Dormant identity (LAW-SERVICE-TIERS).** A venue namespace reaches an
  `Exchange`, never a key, so no SharurPlatform adapter can reach this row and
  no family-map root points at it. It is reviewed on demand, and any future gap
  is recorded here rather than opened as an issue.

> Shared module. The narrative for
> [`ice_us.rs`](../../src/calendar/schedules/futures/us/ice_us.rs)
> lives in [`iceus`](iceus.md#module-narrative-moved-from-srccalendarschedulesfuturesusice_usrs-on-2026-09-12-utc).
