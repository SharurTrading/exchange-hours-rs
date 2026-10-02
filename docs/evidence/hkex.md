<!-- SPDX-License-Identifier: MIT-0 -->

# `hkex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`hkex.rs`](../../src/calendar/schedules/equities/apac/hkex.rs)
- **Source sets:** [`APAC-HKEX`](../schedules/sources.md#apac-hkex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Securities venue union: the executable Extended Morning Session is Regular under crate semantics and bridges the former lunch gap. The 2011 open change and 2016-07-25 CAS tail are observable cutovers; 2012 only rearranged internal phases.

## Revision rows

- 2011-03-07 — T1 — HKEX news release 110303news — Phase One: morning session 09:30–12:00, Extended Morning Session 12:00–13:30, afternoon session 13:30–16:00, with the Pre-opening Session moved to 09:00–09:30.
- 2016-07-25 — T1 — HKEX market communication 160725news — the Closing Auction Session adds the 16:00–16:10 tail for its first eligible securities.

## Normal week

**The pre-2011 grid is the operator's own Trading Hours page, and the page's
own lifetime spans the whole carried region.** The old site's
`eng/market/sec_tradinfo/tradcal/tradcal_1.htm` page — footer
`Updated: 23/03/2009` — states: "Trading is conducted on Monday to Friday
(excluding public holidays) at the following times: Auction Session
Pre-opening Session 9:30 a.m. to 10:00 a.m.; Continuous Trading Session Morning
Session 10:00 a.m. to 12:30 p.m.; Extended Morning Session 12:30 p.m. to 2:30
p.m.; Afternoon Session 2:30 p.m. to 4:00 p.m." That is exactly the baseline
profile the module encodes: `regular` 10:00-16:00 (the three back-to-back
sessions are one continuous envelope — the Extended Morning Session bridges the
former lunch), and the whole 09:30-10:00 Pre-opening Session in `extended`, with
no order-entry split because the page names no period boundaries inside it. The
grid is attested at capture `20100524085427` and unchanged at
`20101219233021` (the last pre-change observation; the page's other pre-change
captures, `20100628233229`, `20100818073934` and `20110119131654`, carry the
same session-table text), and the first post-change capture (`20110309232522`)
prints the Phase One grid — the page updated at the operator's own dated
changeover, which the 2011-03-07 revision row already keys. The ledger horizon
is therefore the January-2010 floor: the carried region is empty.

**Residual.** The 2010-01-01..2010-05-23 span rests on the page's own footer
date (the operator's statement that this content was last updated 2009-03-23)
plus the byte-identity of every observed capture through 2011-01-19, not on a
capture inside that span; the Wayback holds none. A capture of the page dated
inside January-May 2010 would tighten the record without moving the horizon.

**What the page does not state.** The pre-2011 POS's internal period boundaries
(order input vs matching) are stated by no operator text of that era, so the
whole 09:30-10:00 window stays `extended` — the module's existing disclosure,
unchanged by the sourcing move.

## Holidays

**Coverage:** 2010-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates in `Asia/Hong_Kong`; tier T1 throughout).

The rows key on the operator's own holiday schedule, retrieved per era. For 2010-2017 the operator published one bilingual `Trading Calendar` PDF per year under `eng/market/sec_tradinfo/tradcal/` (2016 and 2017 add a Mainland-holidays column for Stock Connect; only the `Hong Kong's Public Holidays` column is read); each PDF's holiday list plus its footer sentence `Markets are closed on Saturdays, Sundays and Public Holidays` states the year's closures, and its footer names the year's half-day trading days. For 2018-2024 the `Trading Calendar and Holiday Schedule` page's `Holiday Schedule` table names every day the markets are closed (`Holiday (no trading)`) and its `Notes` block names every shortened eve (`no afternoon and after-hours trading session`); each year's rows cite the edition that prints the year. For 2025-2027 three editions were retrieved: the 2025-10-07 capture (`HKEX-TC-2025`, page footer "Updated 21 Aug 2025") governs 2025; the live edition (`HKEX-TC-2026-2027`, "Updated 31 Jul 2026") governs 2026 and 2027; the 2026-02-13 capture (`HKEX-TC-2026`, "Updated 16 Jan 2026") corroborates 2026 unchanged between editions.

The half-day instant is era-bound, and every instant is the operator's own statement. For 2017-2024 it is the securities-market half-day schedule (`HKEX-HOURS-SEC`): on the eves of Christmas, New Year and Lunar New Year there is no Extended Morning Session and no Afternoon Session, and the Closing Auction Session runs `12:00 noon to a random closing between 12:08 p.m. and 12:10 p.m.`, so the row states the latest scheduled close edge 12:10 exactly as an ordinary day states 16:10; that arrangement has been in force since the 2016-07-25 CAS launch. For 2010-2011 the operator's own trading-news page (`HKEX-TN-2010`, updated 24/12/2010) states the era's half-day shape directly: 2010-12-24 was `a half trading day from 9:30am to 12:30pm` with `no afternoon trading session`, so those rows close at 12:30; the same session grid (Morning Session 10:00-12:30) was in force through 4 March 2011 per the `Current` column of the Phase One news release (`HKEX-NEWS-PHASE1`). For 2012-2015 the operator's own Trading Hours page (`HKEX-TH-PHASE2`) states the era's half-day shape in session language — `There is no Extended Morning Session and Afternoon Session on the eves of Christmas, New Year and Lunar New Year` — and the same page prints the Phase-Two grid whose Morning Session runs `9:30 a.m. to 12:00 noon`, so each calendar-named half day closes at the end of that session, 12:00 noon; the page is byte-identical at every Wayback capture from 2012-12-13 through 2016-01-20 (sha256 `31ef5478…`), brackets every one of the ten eves, and its Phase-1 edition of 2011-07-21 already carries the same eve sentence, so the arrangement spans the whole Phase-Two era (2012-03-05 Phase Two through the 2016-07-25 CAS launch, both dated operator changes the ledger already keys).

Severe-weather arrangements (typhoon signals, black-rain warnings) are conditional and key no row (LAW-NO-FABRICATED-DATES) — the many typhoon and black-rain halts of 2010-2024 are halts, not closures. The `#` footnotes on `25/5/2026` and `25/12/2025` suspend only the after-hours sessions of derivatives Holiday Trading Products (MSCI) and name no securities session; from 2022 the page's `Holiday Trading Exchange Contracts` column states a derivatives arrangement only, and the securities market's own statement that trading is "Monday to Friday (excluding public holidays)" makes each printed holiday a full closure of the venue envelope. Holidays that fall on a weekend are subsumed by the same Saturday/Sunday statement and key no weekday row: 2010-02-13, 2010-04-03, 2010-05-01, 2010-10-16, 2010-12-25, 2011-01-01, 2011-02-05, 2011-04-23, 2011-10-01, 2012-04-07, 2012-04-28, 2012-06-23, 2013-03-30, 2014-02-01, 2014-04-05, 2014-04-19, 2015-02-21, 2015-04-04, 2015-06-20, 2015-12-26, 2016-03-26, 2016-05-14, 2016-10-01, 2017-01-28, 2017-04-15, 2017-07-01, 2017-10-28 are those cases, as is every weekend date each edition's own table marks.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `1/1/2010` — The first day of January — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-02-15 | closed | `15/2/2010` — The second day of the Lunar New Year — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-02-15; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-02-16 | closed | `16/2/2010` — The third day of the Lunar New Year — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-02-16; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-04-02 | closed | `2/4/2010` — Good Friday — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-04-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-04-05 | closed | `5/4/2010` — Easter Monday — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-04-06 | closed | `6/4/2010` — The day following Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-04-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-05-21 | closed | `21/5/2010` — The Buddha's Birthday — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-05-21; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-06-16 | closed | `16/6/2010` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-06-16; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-07-01 | closed | `1/7/2010` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-09-23 | closed | `23/9/2010` — The day following Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-09-23; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-10-01 | closed | `1/10/2010` — National Day — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-12-24 | early close | `24/12/2010` — Eve of Christmas Day — named `a half-day trading day`; the era half day ran `9:30am to 12:30pm` with `no afternoon trading session` (`HKEX-TN-2010`) | `HKEX-TC-2010` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TN-2010`'s `9:30am to 12:30pm` with `no afternoon trading session`, so the close is 12:30 |
| 2010-12-27 | closed | `27/12/2010` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2010` | T1 | HKEX event date 2010-12-27; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2010-12-31 | early close | `31/12/2010` — Eve of New Year — named `a half-day trading day`; the era half day ran `9:30am to 12:30pm` with `no afternoon trading session` (`HKEX-TN-2010`) | `HKEX-TC-2010` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TN-2010`'s `9:30am to 12:30pm` with `no afternoon trading session`, so the close is 12:30 |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-02-02 | early close | `2/2/2011` — Eve of Lunar New Year — named `a half-day trading day`; the era half day ran `9:30am to 12:30pm` with `no afternoon trading session` (`HKEX-TN-2010`) | `HKEX-TC-2011` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TN-2010`'s `9:30am to 12:30pm` with `no afternoon trading session`, so the close is 12:30 |
| 2011-02-03 | closed | `3/2/2011` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-02-03; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-02-04 | closed | `4/2/2011` — The second day of the Lunar New Year — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-02-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-04-05 | closed | `5/4/2011` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-04-22 | closed | `22/4/2011` — Good Friday — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-04-22; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-04-25 | closed | `25/4/2011` — Easter Monday — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-04-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-05-02 | closed | `2/5/2011` — The day following Labour Day — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-05-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-05-10 | closed | `10/5/2011` — The Buddha's Birthday — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-05-10; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-06-06 | closed | `6/6/2011` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-06-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-07-01 | closed | `1/7/2011` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-09-13 | closed | `13/9/2011` — The day following Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-09-13; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-10-05 | closed | `5/10/2011` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-10-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-12-26 | closed | `26/12/2011` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2011-12-27 | closed | `27/12/2011` — The second weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2011` | T1 | HKEX event date 2011-12-27; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `2/1/2012` — The day following the first day of January — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-01-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-01-23 | closed | `23/1/2012` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-01-23; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-01-24 | closed | `24/1/2012` — The second day of the Lunar New Year — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-01-24; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-01-25 | closed | `25/1/2012` — The third day of the Lunar New Year — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-01-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-04-04 | closed | `4/4/2012` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-04-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-04-06 | closed | `6/4/2012` — Good Friday — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-04-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-04-09 | closed | `9/4/2012` — Easter Monday — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-04-09; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-05-01 | closed | `1/5/2012` — Labour Day — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-07-02 | closed | `2/7/2012` — The day following Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-07-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-10-01 | closed | `1/10/2012` — The day following Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-10-02 | closed | `2/10/2012` — The day following National Day — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-10-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-10-23 | closed | `23/10/2012` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-10-23; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-12-24 | early close | `24/12/2012` — Christmas Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2012` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |
| 2012-12-25 | closed | `25/12/2012` — Christmas Day — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-12-26 | closed | `26/12/2012` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2012` | T1 | HKEX event date 2012-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2012-12-31 | early close | `31/12/2012` — New Year's Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2012` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `1/1/2013` — The first day of January — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-02-11 | closed | `11/2/2013` — The second day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-02-11; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-02-12 | closed | `12/2/2013` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-02-12; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-02-13 | closed | `13/2/2013` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-02-13; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-03-29 | closed | `29/3/2013` — Good Friday — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-03-29; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-04-01 | closed | `1/4/2013` — Easter Monday — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-04-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-04-04 | closed | `4/4/2013` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-04-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-05-01 | closed | `1/5/2013` — Labour Day — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-05-17 | closed | `17/5/2013` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-05-17; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-06-12 | closed | `12/6/2013` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-06-12; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-07-01 | closed | `1/7/2013` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-09-20 | closed | `20/9/2013` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-09-20; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-10-01 | closed | `1/10/2013` — National Day — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-10-14 | closed | `14/10/2013` — The day following Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-10-14; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-12-24 | early close | `24/12/2013` — Christmas Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2013` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |
| 2013-12-25 | closed | `25/12/2013` — Christmas Day — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-12-26 | closed | `26/12/2013` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2013` | T1 | HKEX event date 2013-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2013-12-31 | early close | `31/12/2013` — New Year's Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2013` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `1/1/2014` — The first day of January — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-01-30 | early close | `30/1/2014` — Eve of Lunar New Year — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2014` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |
| 2014-01-31 | closed | `31/1/2014` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-01-31; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-02-03 | closed | `3/2/2014` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-02-03; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-04-18 | closed | `18/4/2014` — Good Friday — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-04-18; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-04-21 | closed | `21/4/2014` — Easter Monday — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-04-21; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-05-01 | closed | `1/5/2014` — Labour Day — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-05-06 | closed | `6/5/2014` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-05-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-06-02 | closed | `2/6/2014` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-06-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-07-01 | closed | `1/7/2014` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-09-09 | closed | `9/9/2014` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-09-09; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-10-01 | closed | `1/10/2014` — National Day — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-10-02 | closed | `2/10/2014` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-10-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-12-24 | early close | `24/12/2014` — Christmas Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2014` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |
| 2014-12-25 | closed | `25/12/2014` — Christmas Day — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-12-26 | closed | `26/12/2014` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2014` | T1 | HKEX event date 2014-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2014-12-31 | early close | `31/12/2014` — New Year's Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2014` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `1/1/2015` — The first day of January — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-02-18 | early close | `18/2/2015` — Eve of Lunar New Year — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2015` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |
| 2015-02-19 | closed | `19/2/2015` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-02-19; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-02-20 | closed | `20/2/2015` — The second day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-02-20; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-04-03 | closed | `3/4/2015` — Good Friday — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-04-03; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-04-06 | closed | `6/4/2015` — The day following Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-04-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-04-07 | closed | `7/4/2015` — The day following Easter Monday — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-04-07; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-05-01 | closed | `1/5/2015` — Labour Day — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-05-25 | closed | `25/5/2015` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-05-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-07-01 | closed | `1/7/2015` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-09-28 | closed | `28/9/2015` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-09-28; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-10-01 | closed | `1/10/2015` — National Day — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-10-21 | closed | `21/10/2015` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-10-21; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-12-24 | early close | `24/12/2015` — Christmas Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2015` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |
| 2015-12-25 | closed | `25/12/2015` — Christmas Day — the calendar's holiday list | `HKEX-TC-2015` | T1 | HKEX event date 2015-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2015-12-31 | early close | `31/12/2015` — New Year's Eve — named `a half-day trading day`; the eve deletes the Extended Morning and Afternoon Sessions, so the close is the Morning Session edge `12:00 noon` (`HKEX-TH-PHASE2`) | `HKEX-TC-2015` | T1 | the calendar names the eve a half-day trading day; the era's own half-day shape is `HKEX-TH-PHASE2`'s session-language deletion of the Extended Morning and Afternoon Sessions on the eves, so the close is the printed Morning Session end 12:00 noon |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `1/1/2016` — The first day of January — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-02-08 | closed | `8/2/2016` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-02-08; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-02-09 | closed | `9/2/2016` — The second day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-02-09; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-02-10 | closed | `10/2/2016` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-02-10; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-03-25 | closed | `25/3/2016` — Good Friday — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-03-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-03-28 | closed | `28/3/2016` — Easter Monday — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-03-28; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-04-04 | closed | `4/4/2016` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-04-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-05-02 | closed | `2/5/2016` — The day following the Labour Day — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-05-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-06-09 | closed | `9/6/2016` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-06-09; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-07-01 | closed | `1/7/2016` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-09-16 | closed | `16/9/2016` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-09-16; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-10-10 | closed | `10/10/2016` — The day following the Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-10-10; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-12-26 | closed | `26/12/2016` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2016-12-27 | closed | `27/12/2016` — The second weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2016` | T1 | HKEX event date 2016-12-27; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `2/1/2017` — The day following the first day of January — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-01-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-01-27 | early close | `27/1/2017` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-2017` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2017-01-30 | closed | `30/1/2017` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-01-30; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-01-31 | closed | `31/1/2017` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-01-31; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-04-04 | closed | `4/4/2017` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-04-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-04-14 | closed | `14/4/2017` — Good Friday — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-04-14; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-04-17 | closed | `17/4/2017` — Easter Monday — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-04-17; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-05-01 | closed | `1/5/2017` — Labour Day — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-05-03 | closed | `3/5/2017` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-05-03; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-05-30 | closed | `30/5/2017` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-05-30; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-10-02 | closed | `2/10/2017` — The day following National Day — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-10-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-10-05 | closed | `5/10/2017` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-10-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-12-25 | closed | `25/12/2017` — Christmas Day — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2017-12-26 | closed | `26/12/2017` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-2017` | T1 | HKEX event date 2017-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `1/1/2018` — The first day of January — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-02-15 | early close | `15/2/2018` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2018` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2018-02-16 | closed | `16/2/2018` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-02-16; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-02-19 | closed | `19/2/2018` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-02-19; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-03-30 | closed | `30/3/2018` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-03-30; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-04-02 | closed | `2/4/2018` — Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-04-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-04-05 | closed | `5/4/2018` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-05-01 | closed | `1/5/2018` — Labour day — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-05-22 | closed | `22/5/2018` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-05-22; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-06-18 | closed | `18/6/2018` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-06-18; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-07-02 | closed | `2/7/2018` — The day following Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-07-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-09-25 | closed | `25/9/2018` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-09-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-10-01 | closed | `1/10/2018` — National Day — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-10-17 | closed | `17/10/2018` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-10-17; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-12-24 | early close | `24/12/2018` — Eve of Christmas Day — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2018` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2018-12-25 | closed | `25/12/2018` — Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-12-26 | closed | `26/12/2018` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2018` | T1 | HKEX event date 2018-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2018-12-31 | early close | `31/12/2018` — Eve of New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2018` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `1/1/2019` — The first day of January — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-02-04 | early close | `4/2/2019` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2019` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2019-02-05 | closed | `5/2/2019` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-02-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-02-06 | closed | `6/2/2019` — The second day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-02-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-02-07 | closed | `7/2/2019` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-02-07; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-04-05 | closed | `5/4/2019` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-04-19 | closed | `19/4/2019` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-04-19; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-04-22 | closed | `22/4/2019` — Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-04-22; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-05-01 | closed | `1/5/2019` — Labour day — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-05-13 | closed | `13/5/2019` — The day following the Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-05-13; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-06-07 | closed | `7/6/2019` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-06-07; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-07-01 | closed | `1/7/2019` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-10-01 | closed | `1/10/2019` — National Day — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-10-07 | closed | `7/10/2019` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-10-07; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-12-24 | early close | `24/12/2019` — Eve of Christmas Day — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2019` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2019-12-25 | closed | `25/12/2019` — Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-12-26 | closed | `26/12/2019` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2019` | T1 | HKEX event date 2019-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2019-12-31 | early close | `31/12/2019` — Eve of New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2019` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `1/1/2020` — The first day of January — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-01-24 | early close | `24/1/2020` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2020` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2020-01-27 | closed | `27/1/2020` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-01-27; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-01-28 | closed | `28/1/2020` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-01-28; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-04-10 | closed | `10/4/2020` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-04-10; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-04-13 | closed | `13/4/2020` — Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-04-13; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-04-30 | closed | `30/4/2020` — Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-04-30; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-05-01 | closed | `1/5/2020` — Labour Day — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-06-25 | closed | `25/6/2020` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-06-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-07-01 | closed | `1/7/2020` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-10-01 | closed | `1/10/2020` — National Day — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-10-02 | closed | `2/10/2020` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-10-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-10-26 | closed | `26/10/2020` — The day following Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-10-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-12-24 | early close | `24/12/2020` — Eve of Christmas Day — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2020` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2020-12-25 | closed | `25/12/2020` — Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2020` | T1 | HKEX event date 2020-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2020-12-31 | early close | `31/12/2020` — Eve of New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2020` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `1/1/2021` — The first day of January — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-02-11 | early close | `11/2/2021` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2021` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2021-02-12 | closed | `12/2/2021` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-02-12; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-02-15 | closed | `15/2/2021` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-02-15; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-04-02 | closed | `2/4/2021` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-04-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-04-05 | closed | `5/4/2021` — The day following Ching Ming Festival — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-04-06 | closed | `6/4/2021` — The day following Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-04-06; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-05-19 | closed | `19/5/2021` — Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-05-19; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-06-14 | closed | `14/6/2021` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-06-14; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-07-01 | closed | `1/7/2021` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-09-22 | closed | `22/9/2021` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-09-22; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-10-01 | closed | `1/10/2021` — National Day — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-10-14 | closed | `14/10/2021` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-10-14; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-12-24 | early close | `24/12/2021` — Eve of Christmas Day — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2021` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2021-12-27 | closed | `27/12/2021` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2021` | T1 | HKEX event date 2021-12-27; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2021-12-31 | early close | `31/12/2021` — Eve of New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2021` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-31 | early close | `31/1/2022` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2022` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2022-02-01 | closed | `1/2/2022` — Lunar New Year's Day — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-02-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-02-02 | closed | `2/2/2022` — The Second day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-02-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-02-03 | closed | `3/2/2022` — The Third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-02-03; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-04-05 | closed | `5/4/2022` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-04-15 | closed | `15/4/2022` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-04-15; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-04-18 | closed | `18/4/2022` — Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-04-18; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-05-02 | closed | `2/5/2022` — The day following Labour Day — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-05-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-05-09 | closed | `9/5/2022` — The day following the Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-05-09; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-06-03 | closed | `3/6/2022` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-06-03; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-07-01 | closed | `1/7/2022` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-09-12 | closed | `12/9/2022` — The second day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-09-12; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-10-04 | closed | `4/10/2022` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-10-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-12-26 | closed | `26/12/2022` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2022-12-27 | closed | `27/12/2022` — The second weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2022` | T1 | HKEX event date 2022-12-27; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `2/1/2023` — The day following the first day of January — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-01-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-01-23 | closed | `23/1/2023` — The second day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-01-23; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-01-24 | closed | `24/1/2023` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-01-24; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-01-25 | closed | `25/1/2023` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-01-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-04-05 | closed | `5/4/2023` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-04-05; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-04-07 | closed | `7/4/2023` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-04-07; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-04-10 | closed | `10/4/2023` — Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-04-10; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-05-01 | closed | `1/5/2023` — Labour Day — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-05-26 | closed | `26/5/2023` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-05-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-06-22 | closed | `22/6/2023` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-06-22; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-10-02 | closed | `2/10/2023` — The day following of National Day — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-10-02; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-10-23 | closed | `23/10/2023` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-10-23; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-12-25 | closed | `25/12/2023` — Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2023-12-26 | closed | `26/12/2023` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2023` | T1 | HKEX event date 2023-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `1/1/2024` — The first day of January — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-01-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-02-09 | early close | `9/2/2024` — Eve of Lunar New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2024` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2024-02-12 | closed | `12/2/2024` — The third day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-02-12; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-02-13 | closed | `13/2/2024` — The fourth day of Lunar New Year — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-02-13; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-03-29 | closed | `29/3/2024` — Good Friday — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-03-29; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-04-01 | closed | `1/4/2024` — Easter Monday — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-04-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-04-04 | closed | `4/4/2024` — Ching Ming Festival — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-04-04; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-05-01 | closed | `1/5/2024` — Labour Day — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-05-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-05-15 | closed | `15/5/2024` — The Birthday of the Buddha — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-05-15; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-06-10 | closed | `10/6/2024` — Tuen Ng Festival — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-06-10; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-07-01 | closed | `1/7/2024` — Hong Kong Special Administrative Region Establishment Day — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-07-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-09-18 | closed | `18/9/2024` — The day following the Chinese Mid-Autumn Festival — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-09-18; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-10-01 | closed | `1/10/2024` — National Day — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-10-01; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-10-11 | closed | `11/10/2024` — Chung Yeung Festival — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-10-11; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-12-24 | early close | `24/12/2024` — Eve of Christmas Day — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2024` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |
| 2024-12-25 | closed | `25/12/2024` — Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-12-25; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-12-26 | closed | `26/12/2024` — The first weekday after Christmas Day — the calendar's holiday list | `HKEX-TC-PAGE-2024` | T1 | HKEX event date 2024-12-26; the calendar's "Markets are closed on Saturdays, Sundays and Public Holidays" deletes the day |
| 2024-12-31 | early close | `31/12/2024` — Eve of New Year — "no afternoon and after-hours trading session" | `HKEX-TC-PAGE-2024` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is ``HKEX-HOURS-SEC``'s half-day CAS edge, the arrangement in force since the 2016-07-25 CAS launch |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `1/1/2025 (Wednesday) — The first day of January — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-01; the schedule deletes the day |
| 2025-01-28 | early close | `28 January 2025 (Tuesday) – Eve of Lunar New Year` — "no afternoon and after-hours trading session"; half-day CAS `12:00 noon to a random closing between 12:08 p.m. and 12:10 p.m.` | `HKEX-TC-2025` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2025-01-29 | closed | `29/1/2025 (Wednesday) — Lunar New Year's Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-29 |
| 2025-01-30 | closed | `30/1/2025 (Thursday) — The second day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-30 |
| 2025-01-31 | closed | `31/1/2025 (Friday) — The third day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-31 |
| 2025-04-04 | closed | `4/4/2025 (Friday) — Ching Ming Festival — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-04-04 |
| 2025-04-18 | closed | `18/4/2025 (Friday) — Good Friday — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-04-18 |
| 2025-04-21 | closed | `21/4/2025 (Monday) — Easter Monday — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-04-21 |
| 2025-05-01 | closed | `1/5/2025 (Thursday) — Labour Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-05-01 |
| 2025-05-05 | closed | `5/5/2025 (Monday) — The Birthday of the Buddha — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-05-05 |
| 2025-07-01 | closed | `1/7/2025 (Tuesday) — Hong Kong Special Administrative Region Establishment Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-07-01 |
| 2025-10-01 | closed | `1/10/2025 (Wednesday) — National Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-10-01 |
| 2025-10-07 | closed | `7/10/2025 (Tuesday) — The day following the Chinese Mid-Autumn Festival — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-10-07 |
| 2025-10-29 | closed | `29/10/2025 (Wednesday) — Chung Yeung Festival — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-10-29 |
| 2025-12-24 | early close | `24 December 2025 (Wednesday) – Eve of Christmas Day` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2025` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2025-12-25 | closed | `25/12/2025 (Thursday) — Christmas Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-12-25 |
| 2025-12-26 | closed | `26/12/2025 (Friday) — The first weekday after Christmas Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-12-26 |
| 2025-12-31 | early close | `31 December 2025 (Wednesday) – Eve of New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2025` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `1/1/2026 (Thursday) — The first day of January — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-01-01 |
| 2026-02-16 | early close | `16 February 2026 (Monday) – Eve of Lunar New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2026-02-17 | closed | `17/2/2026 (Tuesday) — Lunar New Year's Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-02-17 |
| 2026-02-18 | closed | `18/2/2026 (Wednesday) — The second day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-02-18 |
| 2026-02-19 | closed | `19/2/2026 (Thursday) — The third day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-02-19 |
| 2026-04-03 | closed | `3/4/2026 (Friday) — Good Friday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-04-03 |
| 2026-04-06 | closed | `6/4/2026 (Monday) — The day following Ching Ming Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-04-06 |
| 2026-04-07 | closed | `7/4/2026 (Tuesday) — The day following Easter Monday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-04-07 |
| 2026-05-01 | closed | `1/5/2026 (Friday) — Labour Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-05-01 |
| 2026-05-25 | closed | `25/5/2026 (Monday) — The day following the Birthday of the Buddha — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-05-25; the `#` footnote suspends only derivatives MSCI after-hours trading |
| 2026-06-19 | closed | `19/6/2026 (Friday) — Tuen Ng Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-06-19 |
| 2026-07-01 | closed | `1/7/2026 (Wednesday) — Hong Kong Special Administrative Region Establishment Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-07-01 |
| 2026-10-01 | closed | `1/10/2026 (Thursday) — National Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-10-01 |
| 2026-10-19 | closed | `19/10/2026 (Monday) — The day following Chung Yeung Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-10-19 |
| 2026-12-24 | early close | `24 December 2026 (Thursday) – Eve of Christmas Day` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2026-12-25 | closed | `25/12/2026 (Friday) — Christmas Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-12-25 |
| 2026-12-31 | early close | `31 December 2026 (Thursday) – Eve of New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `1/1/2027 (Friday) — The first day of January — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-01-01 |
| 2027-02-05 | early close | `05 February 2027 (Friday) – Eve of Lunar New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2027-02-08 | closed | `8/2/2027 (Monday) — The third day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-02-08; Lunar New Year's Day 2027-02-06 falls on Saturday and names no weekday closure |
| 2027-02-09 | closed | `9/2/2027 (Tuesday) — The fourth day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-02-09 |
| 2027-03-26 | closed | `26/3/2027 (Friday) — Good Friday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-03-26 |
| 2027-03-29 | closed | `29/3/2027 (Monday) — Easter Monday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-03-29 |
| 2027-04-05 | closed | `5/4/2027 (Monday) — Ching Ming Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-04-05 |
| 2027-05-13 | closed | `13/5/2027 (Thursday) — The Birthday of the Buddha — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-05-13 |
| 2027-06-09 | closed | `9/6/2027 (Wednesday) — Tuen Ng Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-06-09 |
| 2027-07-01 | closed | `1/7/2027 (Thursday) — Hong Kong Special Administrative Region Establishment Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-07-01 |
| 2027-09-16 | closed | `16/9/2027 (Thursday) — The day following the Chinese Mid-Autumn Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-09-16 |
| 2027-10-01 | closed | `1/10/2027 (Friday) — National Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-10-01 |
| 2027-10-08 | closed | `8/10/2027 (Friday) — Chung Yeung Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-10-08 |
| 2027-12-24 | early close | `24 December 2027 (Friday) – Eve of Christmas Day` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2027-12-27 | closed | `27/12/2027 (Monday) — The first weekday after Christmas Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-12-27; Christmas Day 2027-12-25 falls on Saturday and names no weekday closure of its own |
| 2027-12-31 | early close | `31 December 2027 (Friday) – Eve of New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |

**Gaps.** None in 2010-2024 or 2025-2027. The window's former ten `Unsourced` dates — 2012-12-24, 2012-12-31, 2013-12-24, 2013-12-31, 2014-01-30, 2014-12-24, 2014-12-31, 2015-02-18, 2015-12-24 and 2015-12-31 — closed on 2026-09-30 UTC: the operator's own Trading Hours page (`HKEX-TH-PHASE2`, found by a Wayback sweep of the `eng/market/sec_tradinfo/tradcal/` page family once the archive service returned) states the era's half-day arrangement in session language and prints the Phase-Two grid, so each calendar-named eve closes at the Morning Session edge 12:00 noon (see Holidays above). The window is complete: every closure and eve the operator's own schedules print ships a row, and every other weekday in the window is audited normal. In 2025-2027 the operator's three editions print no 2026-12-28 holiday, so no row ships there and the crate answers it as an ordinary trading day (see the recorded check below).

**Recorded check, 2026-12-28.** The operator's 2026 table ends at `25/12/2026 (Friday) Christmas Day` in all three editions retrieved (2025-10-07, 2026-02-13 and live 2026-07-31), so no row ships for Monday 2026-12-28 and the crate answers it as an ordinary trading day. The operator's printing holds: Hong Kong's general holidays for 2026 name "the first weekday after Christmas Day" as **Saturday 26 December** (2026-12-28 is not a general holiday), so no conflict exists — the exchange schedule and the general-holiday list agree. A check against gov.hk's 2026 list was performed at review (2026-09-28 UTC); its bytes are the public gov.hk page. **Closing condition:** none — this is a verification record, not a gap; an HKEX table edition that adds a 2026-12-28 row becomes a schedule fix. Re-checked monthly per LAW-WATCH.

**Forward horizon.** The window ends at 2027-12-31 because that is the operator's own publication horizon: the live edition (`HKEX-TC-2026-2027`, footer "Updated 31 Jul 2026") prints 2026 and 2027 complete and names nothing later. Re-checked 2026-09-29 UTC with a fresh live read (saved under `holidays/raw/equities/hkex/forward-2027/` in the research store, sha256 `5a04e139…`): the footer still reads "Updated 31 Jul 2026", the page's 2027 rows are unchanged, and the page names 2028 zero times, so no 2028 edition exists yet and nothing past 2027-12-31 is claimed. **Closing condition:** the operator's 2028 trading-calendar edition, at which point the window extends. Re-checked monthly per LAW-WATCH.

### Documents

The 2025-2027 artifacts were retrieved on 2026-09-28 UTC and saved under `holidays/raw/equities/hkex/2025-2027/` in the research store; the 2010-2024 artifacts below were retrieved on 2026-09-29 UTC and saved under `holidays/raw/equities/hkex/2010-2024/`; the 2026-09-29 forward-horizon re-check artifact lives under `holidays/raw/equities/hkex/forward-2027/`; the 2026-09-30 Phase-Two half-day captures live under `holidays/raw/equities/hkex/2012-2015-halfday/`. Each directory's `INDEX.md` carries the same digests.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `HKEX-TC-2025` | 2025-01-01 .. 2026-12-31 | <https://web.archive.org/web/20251007115724id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20251007115724`, page footer "Updated 21 Aug 2025") | Wayback `id_` replay of capture `20251007115724`, retrieved 2026-09-28 UTC | T1 | `9914f794f9df82e35b83d0d4b2abd120e5cb524655e9ff16857b6319828a2390` |
| `HKEX-TC-2026` | 2026-01-01 .. 2027-12-31 | <https://web.archive.org/web/20260213065549id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20260213065549`, page footer "Updated 16 Jan 2026") | Wayback `id_` replay of capture `20260213065549`, retrieved 2026-09-28 UTC | T1 | `a88c8d316588eab88f1597dbb018e3d6cbc0e5b0e471edd1036936b3dd2b0b68` |
| `HKEX-TC-2026-2027` | 2025-01-01 .. 2027-12-31 | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (page footer "Updated 31 Jul 2026") | retrieved 2026-09-28 01:28 UTC | T1 | `c460176b89fa6bbcfc77393e0013cc27bf03de9cb660367a1e2990b657d87f47` |
| `HKEX-HOURS-SEC` | 2025-01-01 .. 2027-12-31 | <https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en> (page footer "Updated 16 Sep 2017") | retrieved 2026-09-28 01:28 UTC | T1 | `9faebf6ae87c7c21f83fc906a0709416f213ccf08147931cff6a03693d4b3e6c` |
| `HKEX-TC-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100331081510id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/documents/2010cal.pdf> (capture `20100331081510`) | Wayback `id_` replay of capture `20100331081510`, retrieved 2026-09-29 02:44:32 UTC | T1 | `84cb12d8c3343f5070fa53297a4ee10773993d6e06fd9e09b09b67f977a2f005` |
| `HKEX-TC-2011` | 2011-01-01 .. 2011-12-31 | <https://web.archive.org/web/20101202113436id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/documents/2011cal.pdf> (capture `20101202113436`) | Wayback `id_` replay of capture `20101202113436`, retrieved 2026-09-29 02:44:34 UTC | T1 | `fec9dc8da4926f0fd52e3c4fdee85e78da74619176c3ccf3e01ed7e6989ccf5c` |
| `HKEX-TC-2012` | 2012-01-01 .. 2012-12-31 | <https://web.archive.org/web/20111111202741id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2012cal.pdf> (capture `20111111202741`) | Wayback `id_` replay of capture `20111111202741`, retrieved 2026-09-29 02:44:35 UTC | T1 | `ebd2e24c8d3eca71f7648fb2f8bd838500db777a6ce2a28c52008a3aec85bdef` |
| `HKEX-TC-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20121118194251id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2013cal.pdf> (capture `20121118194251`) | Wayback `id_` replay of capture `20121118194251`, retrieved 2026-09-29 02:44:36 UTC | T1 | `c451bbf254a0cf51c5b6a252591003305b7129e9212b1e4a3d1eec8c4b5dda40` |
| `HKEX-TC-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20140124041841id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2014cal.pdf> (capture `20140124041841`) | Wayback `id_` replay of capture `20140124041841`, retrieved 2026-09-29 02:44:39 UTC | T1 | `9339d812849a5fe3392f6beb4fc33a577c268d57cfef37a74934e2c9beaf2929` |
| `HKEX-TC-2015` | 2015-01-01 .. 2015-12-31 | <https://web.archive.org/web/20141222040839id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2015cal.pdf> (capture `20141222040839`) | Wayback `id_` replay of capture `20141222040839`, retrieved 2026-09-29 02:44:41 UTC | T1 | `98a08cc02838c74531787ee563dd841e7c6ea01f0dc7b1b5c797c7103efe2887` |
| `HKEX-TC-2016` | 2016-01-01 .. 2016-12-31 | <https://web.archive.org/web/20160122222039id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2016cal.pdf> (capture `20160122222039`) | Wayback `id_` replay of capture `20160122222039`, retrieved 2026-09-29 02:44:44 UTC | T1 | `53bc04b1464f780a435ff3a60c35de88f4f247ab78cbfe98a6e35a94903fad23` |
| `HKEX-TC-2017` | 2017-01-01 .. 2017-12-31 | <https://web.archive.org/web/20161213062357id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2017calendar.pdf> (capture `20161213062357`) | Wayback `id_` replay of capture `20161213062357`, retrieved 2026-09-29 02:44:45 UTC | T1 | `5b2f374af743d39940a454e81a40d4641918859fe8a4b482a9b7e6bde8154110` |
| `HKEX-TC-PAGE-2018` | 2017-01-01 .. 2018-12-31 | <https://web.archive.org/web/20180121190642id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20180121190642`, page footer "Updated 12 Dec 2017") | Wayback `id_` replay of capture `20180121190642`, retrieved 2026-09-29 UTC | T1 | `a392ca52d8c8ccae55b52a0b0e4c887d59e1c3db8b1737d68a1035adea2ea01e` |
| `HKEX-TC-PAGE-2019` | 2019-01-01 .. 2020-12-31 | <https://web.archive.org/web/20190921174735id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20190921174735`, page footer "Updated 11 Jun 2019") | Wayback `id_` replay of capture `20190921174735`, retrieved 2026-09-29 UTC | T1 | `6c7c2403f059218d458fa554413ed18701da09d84677d604ab3e633e522ecfdf` |
| `HKEX-TC-PAGE-2020` | 2020-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210123020124id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20210123020124`, page footer "Updated 15 Jan 2021") | Wayback `id_` replay of capture `20210123020124`, retrieved 2026-09-29 UTC | T1 | `08a3609cab6cea4ce63d70b3a8ffb808933d3b327094ca90527161fa8cbf98ab` |
| `HKEX-TC-PAGE-2021` | 2021-01-01 .. 2022-12-31 | <https://web.archive.org/web/20211020103529id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20211020103529`, page footer "Updated 12 Oct 2021") | Wayback `id_` replay of capture `20211020103529`, retrieved 2026-09-29 UTC | T1 | `91d130eb5f3aa44a4501d002b4af436137e38ffabc2db4574801fc55af5e595b` |
| `HKEX-TC-PAGE-2022` | 2022-01-01 .. 2023-12-31 | <https://web.archive.org/web/20221209132640id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20221209132640`, page footer "Updated 25 Nov 2022") | Wayback `id_` replay of capture `20221209132640`, retrieved 2026-09-29 UTC | T1 | `7573121518edad691a624665d108b84858db0c5b75852ab25ae493b5f25c9cac` |
| `HKEX-TC-PAGE-2023` | 2023-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230921225643id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20230921225643`, page footer "Updated 27 Jun 2023") | Wayback `id_` replay of capture `20230921225643`, retrieved 2026-09-29 UTC | T1 | `386445a8a2f2dd6ab24d863bfa82ca83762cc2978fc714ba2bac8d8acfb95ccd` |
| `HKEX-TC-PAGE-2024` | 2024-01-01 .. 2025-12-31 | <https://web.archive.org/web/20240615174828id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20240615174828`, page footer "Updated 24 May 2024") | Wayback `id_` replay of capture `20240615174828`, retrieved 2026-09-29 UTC | T1 | `02a21b406ffad3c77fa4485392417acca030f49d941561fdd5bb9ec847092169` |
| `HKEX-TN-2010` | 2010-01-01 .. 2011-03-04 | <https://web.archive.org/web/20101226014449id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradnews/prvtrad_day/ehalf1.htm> (capture `20101226014449`, page updated 24/12/2010) | Wayback `id_` replay of capture `20101226014449`, retrieved 2026-09-29 03:02 UTC | T1 | `a6eb8d9e0f59b4088d68d267708ee1fb8c469b02feba2e596c1a45f066b5b500` |
| `HKEX-TH-2010-05-24` | 2010-01-01 .. 2011-03-06 (the pre-change Trading Hours page; Normal-week rows) | <https://web.archive.org/web/20100524085427id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/tradcal_1.htm> | Wayback `id_` replay of capture `20100524085427`, retrieved 2026-09-30 04:12 UTC | T1 | `5b5c556465d3c9b8db07a074709db03964a0190478fbc5c5b544a82ff153edf5` |
| `HKEX-TH-2010-12-19` | 2010-01-01 .. 2011-03-06 (the last pre-change page state; Normal-week corroboration) | <https://web.archive.org/web/20101219233021id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/tradcal_1.htm> | Wayback `id_` replay of capture `20101219233021`, retrieved 2026-09-30 04:12 UTC | T1 | `14772b65160765b7aaaceaf0fd006daacf205de3090446a3f06f3d5cbf3f6239` |
| `HKEX-TH-2011-03-09` | no rows keyed (the first post-Phase-One page state; Normal-week bracket) | <https://web.archive.org/web/20110309232522id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/tradcal_1.htm> | Wayback `id_` replay of capture `20110309232522`, retrieved 2026-09-30 04:14 UTC | T1 | `a281b6810551fdaf5000a53e562a2d69fc0ec636997e7c9e434cfbe48e3cdd4e` |
| `HKEX-NEWS-PHASE1` | 2010-01-01 .. 2012-03-04 | <https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en> (HKEX news release of 2011-03-03, with the session table for the pre-change, Phase 1 and Phase 2 grids) | retrieved 2026-09-29 03:02:28 UTC | T1 | `7544ebf7c125f82c4e714f1d94a0c408991615214037ce622ea7a1f325dc5141` |
| `HKEX-TH-PHASE2` | 2012-03-05 .. 2016-07-24 (the Phase-Two Trading Hours page; the ten 2012-2015 half-day instants) | <https://web.archive.org/web/20121213083938id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/tradcal_1.htm> — the same bytes at captures `20130218094843`, `20131204062020`, `20140204043814`, `20141130110157`, `20150201145531`, `20151102025014` and `20160120022906`, all bracketing the ten eves; the Phase-1 edition (`HKEX-TH-2011-03-09`'s page at capture `20110721125749`) already carries the same eve sentence | Wayback `id_` replay of capture `20121213083938`, retrieved 2026-09-30 UTC; the bytes are identical at the bracket captures `20130218094843`, `20131204062020`, `20140204043814`, `20141130110157`, `20150201145531`, `20151102025014` and `20160120022906`, all listed with digests in the store's `2012-2015-halfday/INDEX.md` (the sha256 below is their shared digest) | T1 | `31ef5478c08af90b26f8571d772867458e95fcff66b8438632d75512c326bf26` |

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en> — HKEX securities-market trading hours: POS 09:00–09:30, continuous trading 09:30–16:00, then CAS with a randomized 16:08–16:10 close.
- <https://www.hkex.com.hk/Global/Exchange/FAQ/Securities-Market/Trading/Pre_opening-Session?sc_lang=en> — HKEX Pre-opening Session FAQ. Orders "will be accumulated and updated but no matching will occur" during the order input and pre-order matching periods, so 09:00–09:20 is order entry.
- <https://www.hkex.com.hk/-/media/HKEX-Market/Services/Rules-and-Forms-and-Fees/Rules/SEHK/Securities/Rule-Update_Rules-of-the-Exchange/05-11-SEHK-StampDuty-TradingHour_e.pdf> — SEHK rule update. Rule 501G divides the 09:00–09:30 POS into four named periods: order input 09:00–09:15, pre-order matching (renamed no-cancellation in 2020) 09:15–09:20, order matching from 09:20, then a blocking period to 09:30.
- <https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en> — HKEX news release of 2011-03-03, announcing Phase One effective 2011-03-07 and stating the pre-change 10:00 morning open.
- <https://www.hkex.com.hk/News/Regulatory-Announcements/2012/120301news?sc_lang=en> — HKEX announcement of the 2012-03-05 Phase Two, which moved the internal Extended Morning/afternoon handoff to 13:00 without changing the envelope.
- <https://www.hkex.com.hk/News/Market-Communications/2016/160725news?sc_lang=en> — HKEX market communication, the 2016-07-25 CAS launch.
- <http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/tradcal_1.htm> (archived) — the old-site securities Trading Hours page: the pre-2011 grid (see Normal week) and, from Phase Two to the CAS launch, the eve sentence `There is no Extended Morning Session and Afternoon Session on the eves of Christmas, New Year and Lunar New Year` beside the Phase-Two session table (Morning Session 9:30 a.m. to 12:00 noon; Extended Morning 12:00 noon to 1:00 p.m.; Afternoon 1:00 p.m. to 4:00 p.m.), so each calendar-named half day ends at 12:00 noon. Wayback captures 2012-12-13 through 2016-01-20 are byte-identical (`HKEX-TH-PHASE2`); retrieved 2026-09-30 UTC.

## Gaps and residual risks

- **horizon sourced from the floor** — the pre-2011 profile (10:00 open, POS 09:30–10:00) is the operator's own Trading Hours page (`eng/market/sec_tradinfo/tradcal/tradcal_1.htm`, footer `Updated: 23/03/2009`), attested at captures 2010-05-24 through 2011-01-19 and superseded at the dated 2011-03-07 change (see the Normal week section); the carried region below the floor is empty. The residual — no capture inside 2010-01-01..2010-05-23 — is recorded there.
- No primary SEHK text for the pre-2011-03-07 POS period boundaries was located, so the whole 09:30–10:00 window is left `extended` rather than guessing where its matching period began.
- The 2012-03-05 Phase Two is deliberately not a revision row: it rearranged internal phases without changing the venue-level open or close (LAW-HOLIDAY-SCOPE's companion rule on topology, and the ledger's own statement that it is not an observable envelope cutover).
- Later CAS eligibility expansions do not create new exchange-level open/close cutovers; the static profile uses the maximum scheduled CAS edge and not every security is eligible for every phase.
- The CAS 16:00–16:10 window ends in a randomised uncrossing that prints the closing trades, so the whole auction stays `extended`.

## Module narrative (moved from src/calendar/schedules/equities/apac/hkex.rs on 2026-10-02 UTC)

SEHK Rule 501G divides the 09:00–09:30 POS into
four named periods: order input 09:00–09:15, pre-order matching (renamed
no-cancellation in 2020) 09:15–09:20, order matching from 09:20, then a
blocking period to 09:30. HKEX states that orders "will be accumulated and
updated but no matching will occur" during the first two periods, so
09:00–09:20 is order entry. Matching begins at 09:20 and prints the opening
trades at the final IEP, so 09:20–09:30 stays extended (the blocking tail is
kept with the match because the 2020 enhancement randomised the match end).
https://www.hkex.com.hk/Global/Exchange/FAQ/Securities-Market/Trading/Pre_opening-Session?sc_lang=en
https://www.hkex.com.hk/-/media/HKEX-Market/Services/Rules-and-Forms-and-Fees/Rules/SEHK/Securities/Rule-Update_Rules-of-the-Exchange/05-11-SEHK-StampDuty-TradingHour_e.pdf

---

The Extended Morning Session keeps eligible securities continuously tradable
through the ordinary-board lunch, so the venue-level regular envelope has no
midday gap. The static profile uses the maximum scheduled CAS edge; not
every security is eligible for every phase.
https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en

---

Phase Two moved the internal Extended
Morning/afternoon handoff to 13:00 on 2012-03-05 without changing that
envelope. CAS first changed the venue envelope for a subset of securities on
2016-07-25; later eligibility expansions do not create new exchange-level
open/close cutovers.
https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en
https://www.hkex.com.hk/News/Regulatory-Announcements/2012/120301news?sc_lang=en
https://www.hkex.com.hk/News/Market-Communications/2016/160725news?sc_lang=en
