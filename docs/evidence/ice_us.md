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

**Interpretive steps, 2026:** **Which ICE bullet NYSE FANG+ sits on.** The 2026 and 2027 calendars key this family on the group “Currency, Digital Asset, Stock, SOFR, MSCI Bond, and Mortgage Index Contracts”, and the per-holiday notices split that group into an index bullet and a currency bullet. Two of the four 2026 notices that give instants name FANG+ explicitly and both put it with NYSE Stock Index at the same instant: Good Friday's “MSCI Stock and Bond Index, NYSE FANG+™, NYSE Stock Index and FTSE Stock Index” at 09:15, and Labor Day's “NYSE FANG+™ Index, NYSE Stock Index, …” at 13:00. The Memorial Day, Juneteenth and Independence Day notices print the same bullet without spelling FANG+ out, at 13:00. The rows for those three dates take 13:00 on that reading, calibrated twice by the operator's own spelling; the alternative would be to claim a date the calendar marks `open1` was audited normal, which is false. **What is not a late open.** Good Friday's 05:00 is earlier than the family's normal Thursday 20:00 first open on the preceding local date, so the crate's `late_open_ssm` cutoff lands on the trade date itself and the Thursday-evening leg is deleted, which is what ICE states. **TAS suspension keys nothing** — “No TAS trading” and “TAS trading will not be held” are trade-type restrictions, not session boundaries.

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

**Gaps, 2027:** **Every 2027 `open1` date carries no instant by construction.** ICE announces those hours “in advance of the respective holiday via Exchange Notices”, and none had issued at retrieval, so the 2027 rows are the calendar's `open`/`closed`/`open1` statuses only. Closed by the 2027 per-holiday Exchange Notices as they are issued. Coverage ends 2028-01-03, the last trade date the 2027 calendar names.

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
