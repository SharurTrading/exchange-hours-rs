<!-- SPDX-License-Identifier: MIT-0 -->

# `tadawul` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tadawul.rs`](../../src/calendar/schedules/equities/africa_middle_east/tadawul.rs)
- **Source sets:** [`MIDEAST-TADAWUL`](../schedules/sources.md#mideast-tadawul)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Main Market including sourced temporary 2020 regime. The pre-2016 opening-auction order window is omitted: no dated primary source states it for the 11:00-open eras.

## Revision rows

- 2013-06-29 — T3 — SPA news 7e453de27d — the trading week moves from Saturday–Wednesday to Sunday–Thursday, hours unchanged at 11:00–15:30.
- 2016-04-03 — T3 — SPA news 1484000 — hours move to 10:00–15:00.
- 2018-05-27 — T1 — Tadawul Statistical Report H1 2018 — a closing auction is added, 15:00–15:10.
- 2019-05-12 — T1 — Tadawul Statistical Report 2019 — trade at last is added, extending the close-side envelope to 15:20.
- 2020-03-26 — T1 — Saudi Exchange issuer news 6262 — temporary shortened hours: continuous 10:00–13:00 with the close-side envelope to 13:20.
- 2020-05-31 — T1 — Saudi Exchange resumption notice — normal trading hours resume.

## Normal week

**The pre-2013 grid is the operator's own Trading Times page, attested from
2010-01-12.** Tadawul's `static/pages/ar/TradingTimes/tradingtimes.html` page as
served 2010-01-12 (`TADAWUL-TT-2010-01-12`) states: the Saudi Stock Market
(Tadawul) trades "من السبت إلى الأربعاء فترة واحدة فقط من الساعة 11:00 صباحاً
إلى الساعة 03:30 عصراً" — Saturday to Wednesday, one session, 11:00 a.m. to
3:30 p.m. The English page as served 2011-04-29 (`TADAWUL-TT-2011-04-29`)
restates it: "Trading Days: One session, Saturday through Wednesday except
official holidays. Trading in Equities and ETFs: 11:00 am - 03:30 pm. Trading in
Sukuk & Bonds: 11:30 am - 03:30 pm." That is exactly the pre-2013 baseline the
module encodes — Saturday-Wednesday, 11:00-15:30 continuous — and the English
page's capture chain (`20110429234359`, `20110703101441`, `20110903170945`, then
the next surviving capture `20140603012846` after the operator's own dated
2013-06-29 change) brackets the era at both ends. The ledger horizon is
therefore 2010-01-12, the earliest capture day; the carried region below it runs
2010-01-01..2010-01-11.

**What the page settles.** The page is the operator's own standing statement of
the session grid (T1 through the Wayback verbatim mirror), and it also states
the single-session shape (فترة واحدة فقط — no lunch gap) and the Sukuk & Bonds
11:30 open, a segment outside this identity's equity envelope. It states no
pre-opening queue, consistent with the executable-gap note below: the omission
of the pre-2016 opening-auction window stands.

## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.saudiexchange.sa/wps/portal/saudiexchange/rules-guidance/capital-market-overview/trading-cycle-and-times?locale=en> — Saudi Exchange trading cycle and times: current Main Market phases — opening-auction orders 09:30–10:00, continuous trading 10:00–15:00, closing auction 15:00–15:10, trade at last 15:10–15:20. The table starts "Trading in Equities" at 10:00 and notes only that the market opens on a variable basis within 30 seconds after 10:00. Auction uncrosses can be randomized by up to 30 seconds; the static profile uses the published nominal boundaries.
- <https://www.spa.gov.sa/7e453de27d> — Saudi Press Agency release, the 2013-06-29 workweek change.
- <https://www.spa.gov.sa/1484000?lang=en&newsid=1484000> — Saudi Press Agency release, the 2016-04-03 hours change.
- <https://www.saudiexchange.sa/wps/wcm/connect/24ca438e-86a0-47d0-b8f4-65b4cbfebcdd/Saudi%2BStock%2BExchange%2B-Tadawul-%2CStatistical%2BReport%2B%E2%80%93%2BFirst%2BHalf%2B2018%2B-%2BUpdated.pdf> — Tadawul Statistical Report, first half 2018: the closing auction from 2018-05-27.
- <https://www.saudiexchange.sa/wps/wcm/connect/4657c15f-ef37-45c8-8423-09e2a5055ab7/Saudi%2BStock%2BExchange%2B%28Tadawul%29%2CStatistical%2BReport%2B%E2%80%93%2B%2B2019-%2BEn.pdf> — Tadawul Statistical Report 2019: trade at last from 2019-05-12.
- <https://www.saudiexchange.sa/wps/portal/saudiexchange/newsandreports/issuer-news/news-detail-wcm/?locale=en&newsId=6262> — Saudi Exchange issuer news 6262: temporary shortened hours applied 2020-03-26.
- <https://www.saudiexchange.sa/wps/portal/saudiexchange/newsandreports/issuer-news/news-detail-wcm/saudiexchangecontent/issuernews/issuernewsdetails/saudiexchange-announces-resumption-of-normal-trading-hours?locale=en> — Saudi Exchange resumption notice: normal trading hours resume from 2020-05-31.
- <https://www.saudiexchange.sa/wps/portal/saudiexchange/about-saudi-exchange/exchange-media-centre/saudi-exchange-holiday-calendar?locale=en> — `Saudi Exchange Holiday Calendar`, the operator's own holiday calendar page and the holiday watch entry point; its entries 2020-2029 print the trading-discontinue and trading-resume days. The site refuses plain HTTP clients (403) and answers a complete browser-grade header set.

## Holidays

**Coverage:** 2021-01-01..2027-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves the `tadawul` venue: the operator publishes one holiday arrangement per year and the crate routes the Main Market venue to it. Every row keys to the single artifact `TADAWUL-HOLCAL-2026-09-28` — the operator's `Saudi Exchange Holiday Calendar` page as retrieved on 2026-09-28, whose entries 2020-2029 are server-rendered in one table. Most Eid entries state the arrangement in session language: `Trading will discontinue at the end of trading day <date>. Trading will resume after the holiday on <date>`, several annotated `* According to the UMM AL-QURA calendar`; the 2021 and 2022 Eid Al Fiter entries instead print the holiday's own bounds — `First day of Eid Al Fiter is <date>. Last day of Eid Al Fiter is <date>.` — and each Founding Day and National Day entry states the single day observed (2023's National Day entry carries the discontinue/resume statement). The dates — Carnival-equivalent Islamic dates included — are read from the operator's printed calendar, never computed. The 2021-2024 rows were added on 2026-09-29 (UTC) from the same artifact the 2025-2027 rows already keyed: the page is the operator's own statement of its own past arrangements, so no new retrieval was needed. Re-checked 2026-09-29 UTC with a fresh live read of the same page (`TADAWUL-HOLCAL-2026-09-29` below): the entries 2020-2029 are intact and the 2027 legs are unchanged (Founding Day 22/02/2027, Eid Al Fiter 07-11/03/2027 with 04/03 discontinue and 14/03 resume, Eid Al Adha 16-20/05/2027 with 13/05 discontinue and 23/05 resume, National Day 23/09/2027), so the coverage window stands at 2021-01-01..2027-12-31 with no row moved.

The Main Market trades Sunday-Thursday, so every row below is a Sunday-Thursday trade date the operator's entry removes: an Eid's printed legs that fall on Friday-Saturday change no trade date and ship no row.

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-05-13 | closed | `First day of Eid Al Fiter is 13/5/2021. Last day of Eid Al Fiter is 16/5/2021.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-05-13, a Thursday |
| 2021-05-16 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-05-16, a Sunday; the 14th-15th are the weekend |
| 2021-07-18 | closed | Eid Al Adha: `Trading will discontinue at the end of trading day 15/7/2021. Trading will resume after the holiday on 25/7/2021.` (the entry prints its range as `22/07/2021 - 15/07/2021`) | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Thursday 15/07 discontinue; the 16th-17th are the weekend |
| 2021-07-19 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-07-19, a Monday |
| 2021-07-20 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-07-20, a Tuesday |
| 2021-07-21 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-07-21, a Wednesday |
| 2021-07-22 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-07-22, a Thursday |
| 2021-09-23 | closed | `National Day of Saudi Arabia is on 23/9/2021.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2021-09-23, a Thursday |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-02-22 | closed | `Founding Day of Saudi Arabia is on 22/02/2022.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-02-22, a Tuesday |
| 2022-04-28 | closed | `First day of Eid Al Fiter is 28/4/2022. Last day of Eid Al Fiter is 8/5/2022.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-04-28, a Thursday, the range's first leg |
| 2022-05-01 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-05-01, a Sunday; the 29th-30th April are the weekend |
| 2022-05-02 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-05-02, a Monday |
| 2022-05-03 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-05-03, a Tuesday |
| 2022-05-04 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-05-04, a Wednesday |
| 2022-05-05 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-05-05, a Thursday; the 6th-7th May are the weekend |
| 2022-07-07 | closed | Eid Al Adha, range `06/07/2022 - 13/07/2022`: `Trading will discontinue at the end of trading day 6/7/2022. Trading will resume after the holiday on 13/7/2022.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Wednesday 06/07 discontinue |
| 2022-07-10 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-07-10, a Sunday; the 8th-9th July are the weekend |
| 2022-07-11 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-07-11, a Monday |
| 2022-07-12 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-07-12, a Tuesday; trading resumes Wednesday 13/07 |
| 2022-09-22 | closed | `National Day of Saudi Arabia is on 22/9/2022.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2022-09-22, a Thursday |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-02-22 | closed | `Founding Day of Saudi Arabia is on 22/02/2023.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-02-22, a Wednesday |
| 2023-04-18 | closed | Eid Al Fiter, range `17/04/2023 - 25/04/2023`: `Trading will discontinue at the end of trading day 17-04-2023. Trading will resume after the holiday on 25-04-2023.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Monday 17/04 discontinue; the 21st-22nd April are the weekend |
| 2023-04-19 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-04-19, a Wednesday |
| 2023-04-20 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-04-20, a Thursday |
| 2023-04-23 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-04-23, a Sunday |
| 2023-04-24 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-04-24, a Monday; trading resumes Tuesday 25/04 |
| 2023-06-25 | closed | Eid Al Adha, range `22/06/2023 - 02/07/2023`: `Trading will discontinue at the end of trading day Thursday 22-06-2023. Trading will resume after the holiday on Sunday 02-07-2023. ** According to the UMM AL-QURA calendar` | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Thursday 22/06 discontinue; the 23rd-24th June are the weekend |
| 2023-06-26 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-06-26, a Monday |
| 2023-06-27 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-06-27, a Tuesday |
| 2023-06-28 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-06-28, a Wednesday |
| 2023-06-29 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2023-06-29, a Thursday; trading resumes Sunday 02/07 |
| 2023-09-24 | closed | National Day: `Trading will discontinue at the end of trading day 21-09-2023. Trading will resume after the holiday on 25-09-2023.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | The removed Sunday 24/09 between the printed Thursday 21/09 discontinue and the Monday 25/09 resume; the 22nd-23rd September are the weekend |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-02-22 | closed | `Founding Day of Saudi Arabia is on 22/02/2024.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-02-22, a Thursday |
| 2024-04-07 | closed | Eid Al Fiter, range `04/04/2024 - 14/04/2024`: `Trading will discontinue at the end of trading day 04/04/2024. Trading will resume after the holiday on 14/04/2024.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Thursday 04/04 discontinue; the 12th-13th April are the weekend |
| 2024-04-08 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-04-08, a Monday |
| 2024-04-09 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-04-09, a Tuesday |
| 2024-04-10 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-04-10, a Wednesday |
| 2024-04-11 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-04-11, a Thursday; trading resumes Sunday 14/04 |
| 2024-06-16 | closed | Eid Al Adha, range `13/06/2024 - 23/06/2024`: `Trading will discontinue at the end of trading day 13/06/2024. Trading will resume after the holiday on 23/06/2024.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Thursday 13/06 discontinue; the 14th-15th June are the weekend |
| 2024-06-17 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-06-17, a Monday |
| 2024-06-18 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-06-18, a Tuesday |
| 2024-06-19 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-06-19, a Wednesday |
| 2024-06-20 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-06-20, a Thursday; the 21st-22nd June are the weekend and trading resumes Sunday 23/06 |
| 2024-09-23 | closed | `National Day of Saudi Arabia is on 23/09/2024.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2024-09-23, a Monday |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-02-23 | closed | `Founding Day of Saudi Arabia is on 23/02/2025.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-02-23, a Sunday: the operator's own observance statement keys the row to the printed day (the 22nd was a Saturday) |
| 2025-03-30 | closed | Eid Al Fiter, range `27/03/2025 - 02/04/2025`: `Trading will discontinue at the end of trading day 27/03/2025. Trading will resume after the holiday on 03/04/2025. * According to the UMM AL-QURA calendar` | `TADAWUL-HOLCAL-2026-09-28` | T1 | First removed Sunday-Thursday trade date after the printed Thursday 27/03 discontinue |
| 2025-03-31 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-03-31, a Monday |
| 2025-04-01 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-04-01, a Tuesday |
| 2025-04-02 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-04-02, a Wednesday; trading resumes Thursday 03/04 |
| 2025-06-05 | closed | Eid Al Adha, range `05/06/2025 - 10/06/2025`: `Trading will discontinue at the end of trading day 04/06/2025. Trading will resume after the holiday on 11/06/2025. * According to the UMM AL-QURA calendar` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-06-05, a Thursday, the range's first leg |
| 2025-06-08 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-06-08, a Sunday; the printed 06-07 June legs are a weekend |
| 2025-06-09 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-06-09, a Monday |
| 2025-06-10 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-06-10, a Tuesday; trading resumes Wednesday 11/06 |
| 2025-09-23 | closed | `National Day of Saudi Arabia is on 23/09/2025.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2025-09-23, a Tuesday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-02-22 | closed | `Founding Day of Saudi Arabia is on 22/02/2026.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-02-22, a Sunday |
| 2026-03-17 | closed | Eid Al Fiter, range `17/03/2026 - 23/03/2026`: `Trading will discontinue at the end of trading day 16/03/2026.Trading will resume after the holiday on 24/03/2026.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-03-17, a Tuesday, the range's first leg |
| 2026-03-18 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-03-18, a Wednesday |
| 2026-03-19 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-03-19, a Thursday |
| 2026-03-22 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-03-22, a Sunday; the printed 20-21 March legs are a weekend |
| 2026-03-23 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-03-23, a Monday; trading resumes Tuesday 24/03 |
| 2026-05-24 | closed | Eid Al Adha, range `24/05/2026 - 28/05/2026`: `Trading will discontinue at the end of trading day 21/05/2026. Trading will resume after the holiday on 31/05/2026. * According to the UMM AL-QURA calendar` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-05-24, a Sunday, the range's first leg |
| 2026-05-25 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-05-25, a Monday |
| 2026-05-26 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-05-26, a Tuesday |
| 2026-05-27 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-05-27, a Wednesday |
| 2026-05-28 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-05-28, a Thursday; trading resumes Sunday 31/05 |
| 2026-09-23 | closed | `National Day of Saudi Arabia is on 23/09/2026.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2026-09-23, a Wednesday |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-02-22 | closed | `Founding Day of Saudi Arabia is on 22/02/2027.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-02-22, a Monday |
| 2027-03-07 | closed | Eid Al Fiter, range `07/03/2027 - 11/03/2027`: `Trading will discontinue at the end of trading day 04/03/2027. Trading will resume after the holiday on 14/03/2027. * According to the UMM AL-QURA calendar` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-03-07, a Sunday, the range's first leg |
| 2027-03-08 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-03-08, a Monday |
| 2027-03-09 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-03-09, a Tuesday |
| 2027-03-10 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-03-10, a Wednesday |
| 2027-03-11 | closed | same Eid Al Fiter entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-03-11, a Thursday; trading resumes Sunday 14/03 |
| 2027-05-16 | closed | Eid Al Adha, range `16/05/2027 - 20/05/2027`: `Trading will discontinue at the end of trading day 13/05/2027. Trading will resume after the holiday on 23/05/2027. * According to the UMM AL-QURA calendar` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-05-16, a Sunday, the range's first leg |
| 2027-05-17 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-05-17, a Monday |
| 2027-05-18 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-05-18, a Tuesday |
| 2027-05-19 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-05-19, a Wednesday |
| 2027-05-20 | closed | same Eid Al Adha entry | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-05-20, a Thursday; trading resumes Sunday 23/05 |
| 2027-09-23 | closed | `National Day of Saudi Arabia is on 23/09/2027.` | `TADAWUL-HOLCAL-2026-09-28` | T1 | Operator event date 2027-09-23, a Thursday |

**Gaps, inside the window:** none. The operator's page published 2021 through 2027 at the 2026-09-28 retrieval, every Sunday-Thursday trade date its entries remove ships a row, and every other date inside the window is audited normal. The 2028-2029 entries the page also prints (Eid ranges only, whose legs the crate has not worked up) lie outside the window and claim nothing.

**2013-06-29..2020-12-31 is an unaudited span.** The page's oldest complete entry is the 2020 Eid Al Fiter (`Trading will discontinue at the end of trading day 21/5/2020. Trading will resume after the holiday on 31/5/2020.`), but the same page states no Eid Al Adha 2020 or anything earlier — its entry list truncates there — so a 2020 window would claim Eid Al Adha dates it cannot audit. Bounded search 2026-09-29 (UTC): the Internet Archive's CDX over `saudiexchange.sa` holds no capture of the holiday-calendar page before 2023-01-29, and the pre-2023 operator portal (`tadawul.com.sa`) exposes only opaque IBM-portal URLs with no surviving named holiday page. **Closing condition:** an operator artifact (or a Wayback capture of one) printing the 2014-2020 holiday arrangements — e.g. a contemporaneous `Market Holidays` page capture or a yearly circular — at which point the window extends back toward the 2013-06-29 normal-week horizon.

**Interpretive steps.** Each Eid row is keyed to the printed range's Sunday-Thursday dates, bounded by the entry's own trading-discontinue and trading-resume days: a printed leg on a Friday or Saturday (e.g. 28-29 March 2025, 20-21 March 2026, 5-7 June 2025, 13-14 and 21-22 May 2027) is not a Main Market trade date and ships no row. The 2021 and 2022 Eid Al Fiter entries state no discontinue or resume day — they print the holiday's first and last day — so their rows key to the Sunday-Thursday dates inside those printed bounds and the days outside them stand or fall with the normal week. The UMM AL-QURA annotation is the operator's own caveat on its printed dates; the crate records the printed day and does not compute the Islamic calendar. The 2025 Founding Day row keys to Sunday 23 February because that is the day the operator's entry prints, not the anniversary's civil 22 February. This reading is fenced per date in `tests/global_equities/holidays.rs`.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `TADAWUL-HOLCAL-2026-09-28` | 2021-01-01 .. 2027-12-31 | <https://www.saudiexchange.sa/wps/portal/saudiexchange/about-saudi-exchange/exchange-media-centre/saudi-exchange-holiday-calendar?locale=en> | retrieved 2026-09-28 01:20 UTC | T1 | `6980261a27759b1fd80b79281d1f08aa30d19aa0e1b08d721020910c268343ff` |
| `TADAWUL-HOLCAL-2026-09-29` | 2021-01-01 .. 2027-12-31 | <https://www.saudiexchange.sa/wps/portal/saudiexchange/about-saudi-exchange/exchange-media-centre/saudi-exchange-holiday-calendar?locale=en> | retrieved 2026-09-29 22:22 UTC | T1 | `74fecb031d7503a51a4c81197d61928c753a2ed6afe4039d9f63e4297d73cad7` |
| `TADAWUL-TT-2010-01-12` | 2010-01-12 .. 2013-06-28 (the Arabic Trading Times page; Normal-week rows) | <https://web.archive.org/web/20100112091155id_/http://www.tadawul.com.sa/static/pages/ar/TradingTimes/tradingtimes.html> | Wayback `id_` replay of capture `20100112091155`, retrieved 2026-09-30 04:35 UTC | T1 | `8d5a8681b99bad99873fec880bb97e4d70528d6c1bb9b7d756d638fcf4b44262` |
| `TADAWUL-TT-2011-04-29` | 2010-01-12 .. 2013-06-28 (the English Trading Times page; Normal-week corroboration) | <https://web.archive.org/web/20110429234359id_/http://www.tadawul.com.sa/static/pages/en/TradingTimes/tradingtimes.html> | Wayback `id_` replay of capture `20110429234359`, retrieved 2026-09-30 04:35 UTC | T1 | `d4bf474d42d80e8b2de33be783e834f9357cef7550162bdabcb9fb2ba0c578de` |

The artifact was saved under `holidays/raw/equities/tadawul/2025-2027/` in the research store, whose `INDEX.md` repeats the URL and digest; the 2026-09-29 re-check artifact lives under `holidays/raw/equities/tadawul/forward-2027/` with its own `INDEX.md`.

## Gaps and residual risks

- **2013-06-29..2020-12-31 holiday span.** Recorded in the Holidays section above: the operator's own page states only the 2020 Eid Al Fiter for that era, no contemporaneous holiday page survives in the Internet Archive, and the dates refuse. Closing condition: an operator artifact printing the 2014-2020 holiday arrangements.
- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the 2013-06-29 and 2016-04-03 rows are dated by T3 artifacts.** Both revision rows carry `T3` on their own lines and rest on Saudi Press Agency releases (SPA news 7e453de27d and SPA news 1484000). Under LAW-PRIMARY-SOURCES a dated change needs an unconditional day stated by the operator, and T3 may date a change **only** when it mirrors an operator document verbatim; nothing in the record shows either release reproduces a Saudi Exchange or CMA document verbatim, so as recorded these two rows are not admissible and the bullet above understates that as a tier note rather than a defect. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; both rows are served exactly as before. Closing condition: retrieve the underlying Tadawul or CMA announcement behind each date, or establish that the SPA text is a verbatim reprint of it — either promotes both rows to T1. If neither holds, the rows must be withdrawn and the two grids served as an undated intersection instead. Served since this activation (monthly LAW-WATCH cadence); tracked under #116.
- **executable, pre-2016 opening-auction window.** The pre-2016 grids carry no pre-opening phase. Today's trading-cycle table documents the 09:30–10:00 opening auction, but no dated primary source states the opening-auction order window for the 11:00-open eras; a 10:00–11:00 window would be an inference from the later auction's shape, so under LAW-PRIMARY-SOURCES it is omitted and reads closed. The 2016–2018 era likewise carries no order-entry schedule: the 09:30 queue is evidenced only by the current trading-cycle page, which states nothing about that era. Closing condition: a dated Saudi Exchange or CMA artifact stating the queue for its era. Neither old grid had any close-side phase, so their extended slices are empty too.
- **Tier of the 2013 and 2016 rows.** Both rest on Saudi Press Agency releases. SPA is the Kingdom's state news agency and carries official announcements, but nothing in the record establishes that either release reproduces an exchange or CMA document verbatim, so they are recorded here at T3 rather than T1. Under LAW-PRIMARY-SOURCES a dated change needs an unconditional day stated by the operator, and T3 may date a change only when it mirrors an operator document verbatim. Closing condition: retrieve the underlying Tadawul or CMA announcement for each date, or confirm that the SPA text is a verbatim reprint of it. If neither holds, both rows are T4-keyed and must be rebuilt or withdrawn.
- **Horizon sourced from 2010-01-12.** The pre-2013 baseline — Saturday–Wednesday, 11:00–15:30 — is the operator's own Trading Times page (Arabic, capture 2010-01-12; English restatement, capture 2011-04-29), bracketed at the far end by the operator's own 2013-06-29 week change (see the Normal week section). The carried region below the earliest capture runs 2010-01-01..2010-01-11. The SPA publications-date closing condition is discharged for the baseline: the grid itself no longer rests on the SPA citation, though the 2013-06-29 and 2016-04-03 revision rows keep the T3 disclosure recorded above.
- **Interpretive step, order-entry classification.** The current 09:30–10:00 window is `order_entry`: it collects, amends and cancels opening-auction orders without any of them matching, and the first print is the 10:00 uncross. The closing auction and the trade-at-last tail both print — trade at last executes at the closing auction price — so both stay `extended`.
- **Source set has no monitoring feed.** `MIDEAST-TADAWUL` records that no consolidated schedule-notice feed is indexed; review means reopening the trading cycle and times page, the `Saudi Exchange Holiday Calendar` page, and the individual reports and notices above.
- **Service tier.** The consumer serves this venue live (it is one of the market-clock overview's venues), so the identity is **served** and the holiday-bearing calendar is reviewed monthly per LAW-WATCH. The holiday rows above are T1 throughout; the 2013-06-29 and 2016-04-03 normal-week rows below the current grid keep the T3 disclosure recorded above.
