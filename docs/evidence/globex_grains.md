<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_grains` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`grains.rs`](../../src/calendar/schedules/futures/us/grains.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Standard-size CBOT grain/oilseed futures only; mini grains are excluded. Current matching, morning Pre-Open, and PCP phases are exact. The 2010/2011 phase changes and 2012/2013/2015 matching revisions are dated; the 2013-03-22 operator notice dates the Sunday 16:00-19:00 and Monday-Thursday 16:45-19:00 queues and the 14:30-16:00 PCP to 2013-04-07, leaving only the 21-hour 2012-05-20..2013-04-06 regime's queue states omitted. The 2026-08-31 review sourced those states from CME's own trading-hours pages — Sunday Pre-Open 16:00, weekday "14:30-16:00, 16:45-17:00", ETH 17:00-14:00 on the 2012-05-28 and 2012-06-07 captures, against Sunday 16:15 and the 18:00-07:15/09:30-13:15 grid on 2012-05-11 — so the switch is bracketed to 2012-05-11..2012-05-28 around the sourced 2012-05-20 expansion. Advisory #20120518 states only the new matching hours, never the queue times, so no queue revision is keyed to that day and the states stay omitted.

## Revision rows

- 2010-04-19 — T1 — CME Globex notice 20100405 — the afternoon PCP expands to 13:15:30–16:00 CT.
- 2011-12-27 — T1 — CFTC filing rul120711cbot001 — the weekday morning queue moves to 08:00 CT.
- 2012-05-20 — T1 — CME market-data advisory 20120518 — matching expands to 17:00–14:00 CT.
- 2013-04-07 — T1 — CME SER-6617 and GCC notice 2013-03-22 — 19:00–07:45 CT electronic session around an 08:30–13:15 CT day session, with the full queue set.
- 2013-08-18 — T1 — CME market-data advisory 20130812 — the morning Pre-Open widens from 08:15 to 08:00 CT.
- 2015-07-05 — T1 — CME SER-7395R — the day-session close moves to 13:20 CT.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html> — CME press release of 2009-06-05, the pre-floor grain electronic-hours expansion.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html> — CME Globex notice 20100315, the March-2010 market-state table that supplies the then-live audit-floor queue and PCP phases.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2010-62.html> — CME market-data advisory Q2010-62.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html> — CME Globex notice 20100405, the 2010-04-19 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the generic afternoon-queue notice that does not enumerate this family.
- <https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf> — CBOT rule filing `rul120711cbot001` as published by the CFTC, the 2011-12-27 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html> — CME market-data advisory 20120518, the 2012-05-20 matching expansion.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html> — CME market-data advisory 20120904, the mini-grain divergence.
- <https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf> — CME SER-6617, the 2013-04-07 revision's source.
- <https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf> — CME Global Command Center client notice of 2013-03-22, which states every current queue's onset — capture 2013-04-23.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html> — CME market-data advisory 20130812, the 2013-08-18 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html> — CME SER-7395R, the 2015-07-05 revision's source.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the pre-expansion grain grid with a 16:15 Sunday Pre-Open. **Read at the 2026-08-31 targeted review of the 21-hour regime's queue states**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, the expanded 17:00–14:00 grid with a 16:00 Sunday Pre-Open. **Read at the 2026-08-31 targeted review of the 21-hour regime's queue states**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07. **Read at the 2026-08-31 targeted review of the 21-hour regime's queue states**, which is later than this row's review date and governs for this source.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the queue and PCP states of the 21-hour 2012-05-20..2013-04-06 regime have no operator-stated onset day. The 2026-08-31 review sourced the states themselves from CME's own trading-hours captures of 2012-05-28 and 2012-06-07 against the pre-expansion 2012-05-11 capture, which brackets the switch to 2012-05-11..2012-05-28; CME market-data advisory 20120518 states only the new matching hours and never the queue times, so no queue revision is keyed to 2012-05-20. Closing condition: a CME document that states those queue times in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the baseline queue and PCP phases rest on the operator's March-2010 market-state table, which states them as then-live rather than dating them, so they are carried back from 2010-03-15 to the January-2010 floor.
- **residual risk** — a later generic CME Globex notice broadly names CBOT in an afternoon queue change but does not enumerate this family and conflicts with the complete family-specific state table, so no separate evening queue is inferred from it.
- **scope** — standard-size CBOT grain and oilseed futures only; mini grains are excluded.

> Shared module. The narrative for
> [`grains.rs`](../../src/calendar/schedules/futures/us/grains.rs) lives in
> [`cbot`](cbot.md#module-narrative-moved-from-srccalendarschedulesfuturesusgrainsrs-on-2026-09-12-utc).
> Sibling identities: [`cbot`](cbot.md).

## Evidence documents
The 2019-2021 and 2022-2024 eras carry their own `### Documents` table in the section
for their years, and this table carries every other id.

Every id below resolves to one saved artifact behind this file's holiday rows.
The 2010-2012 ids are CME Group's own holiday-calendar PDFs at tier T1, retrieved
through the Internet Archive and saved; the 2025-2027 ids are responses of CME's
own `trading-hours-by-product` service at tier T2. Each id's row resolves it to the URL it was read at — an Internet Archive
raw replay for a saved capture, the operator's own endpoint for a live retrieval — and to
the capture time in UTC, the tier and the sha256, as the design memo's section 3.2
requires. The byte counts and, for a bundle member, the artifact's path inside the bundle
are in the research store's `holidays/raw/` indexes.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2010-new-years.pdf @2010-02-15T05:16:52Z` | `2010-new-years.pdf` | <https://web.archive.org/web/20100215051652id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-new-years.pdf> | archive capture 2010-02-15T05:16:52Z | T1 | `c30a6cef73fca23c54b25907f307ad52a2922d1e4b76c0a12de126dc6fc31a6d` |
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | <https://web.archive.org/web/20100601111916id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-good-friday.pdf> | archive capture 2010-06-01T11:19:16Z | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | <https://web.archive.org/web/20101122094012id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-thanksgiving.pdf> | archive capture 2010-11-22T09:40:12Z | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2010-christmas.pdf @2010-12-14T06:12:38Z` | `2010-christmas.pdf` | <https://web.archive.org/web/20101214061238id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-christmas.pdf> | archive capture 2010-12-14T06:12:38Z | T1 | `d4adb060f6eb592fb24e3a272db57b3d8c9d69f3f370682ecd8d57e4169c37be` |
| `2011-new-years.pdf @2011-11-01T14:39:45Z` | `2011-new-years.pdf` | <https://web.archive.org/web/20111101143945id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-new-years.pdf> | archive capture 2011-11-01T14:39:45Z | T1 | `42c289804cd3fa0830556ecb7fcc31493c452ba9e7325ebe7d7ce29613e41476` |
| `2011-good-friday.pdf @2011-10-28T02:37:07Z` | `2011-good-friday.pdf` | <https://web.archive.org/web/20111028023707id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-good-friday.pdf> | archive capture 2011-10-28T02:37:07Z | T1 | `6cf10359bb438eb49287dcef7c1e75a484df4d6e3b538fa9ee59dc3832210bda` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | <https://web.archive.org/web/20111124185246id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf> | archive capture 2011-11-24T18:52:46Z | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | <https://web.archive.org/web/20120125020548id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf> | archive capture 2012-01-25T02:05:48Z | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | <https://web.archive.org/web/20120125025430id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf> | archive capture 2012-01-25T02:54:30Z | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-good-friday.pdf @2012-04-17T00:42:47Z` | `2012-good-friday.pdf` | <https://web.archive.org/web/20120417004247id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-good-friday.pdf> | archive capture 2012-04-17T00:42:47Z | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | <https://web.archive.org/web/20120915003714id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf> | archive capture 2012-09-15T00:37:14Z | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | <https://web.archive.org/web/20120915003923id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf> | archive capture 2012-09-15T00:39:23Z | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | <https://web.archive.org/web/20120915003437id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf> | archive capture 2012-09-15T00:34:37Z | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | <https://web.archive.org/web/20130127223901id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-thanksgiving.pdf> | archive capture 2013-01-27T22:39:01Z | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | <https://web.archive.org/web/20130414194027id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-christmas.pdf> | archive capture 2013-04-14T19:40:27Z | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
| `2016-new-years-holiday-schedule.pdf @2016-01-08` | `2016-new-years-holiday-schedule.pdf` | <https://web.archive.org/web/20160108203007id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-new-years-holiday-schedule.pdf> | archive capture 2016-01-08T20:30:07Z | T1 | `118196a469dd40ad6a40594da273f726f6cb3e503f9cc1b8fd2057cf2fa32c61` |
| `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | `2016-martin-luther-king-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-martin-luther-king-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `0fac2d08a84f8e9a637573438adfab33ba91c69e76b6dce01f0ca4cbcde8aff1` |
| `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | `2016-presidents-day-holiday-schedule.pdf` | <https://web.archive.org/web/20151203081738id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-presidents-day-holiday-schedule.pdf> | archive capture 2015-12-03T08:17:38Z | T1 | `b5264dd479efd69df495de170cdd58326f64e9e31a2ed7cca6d7d96cc424c9c2` |
| `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | `2016-good-friday-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-good-friday-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `632c4e3e451fd972e4cccb743f7dfa7a41896044cafc0e82953e0c6fb6d0bc42` |
| `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | `2016-memorial-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-memorial-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `03fa4af1988ae7a54c6e5426b83a76e54825ac4398f7a2be6e77be82fe8ab9f9` |
| `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | `2016-4th-of-july-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-4th-of-july-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `533a49be68619dbd003b3e749485be46051018d3613d44a3043bf8a087fec9ad` |
| `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | `2016-labor-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-labor-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `a4c0676ad117a63894b84637e8e5a7734ed17d7e2d1ee09d5a96370c7fd53b4a` |
| `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | `2016-thanksgiving-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-thanksgiving-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `b3c4dbe2d60bf5530316653d381ba28f1eb4ed6ec55844dca976b1852941c2b1` |
| `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | `2016-christmas-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-christmas-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `42efeb845763d3d8a288dd9f9349bfcaf88badebf49ee1c6bc645a72ec01d252` |
| `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | `2017-new-years-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-new-years-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `493e9cd3fb8ae7ce8b059f30516dedd83631a101e63ecd4c9201fa24c8a2aae8` |
| `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | `2017-martin-luther-king-holiday-schedule.xls` | <https://web.archive.org/web/20170628172556id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-martin-luther-king-holiday-schedule.xls> | archive capture 2017-06-28T17:25:56Z | T1 | `c11937ee9995dd712f23631dd235571d87d2c387bc21e074cc87905b68980d58` |
| `2017-presidents-day-holiday-schedule.xls @2017-06-28` | `2017-presidents-day-holiday-schedule.xls` | <https://web.archive.org/web/20170628174148id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-presidents-day-holiday-schedule.xls> | archive capture 2017-06-28T17:41:48Z | T1 | `2ec7b623faa8f17ab5946091492be0dacbd29e6446ab3789e0ba4fa5ccd58b54` |
| `2017-good-friday-holiday-schedule.xls @2017-05-05` | `2017-good-friday-holiday-schedule.xls` | <https://web.archive.org/web/20170505072315id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-good-friday-holiday-schedule.xls> | archive capture 2017-05-05T07:23:15Z | T1 | `bdc8775f7061ae1e377421fe7a951446c0066ba11d1dbfc7bebba9e7c9415ccb` |
| `2017-memorial-day-holiday-schedule.xls @2017-10-25` | `2017-memorial-day-holiday-schedule.xls` | <https://web.archive.org/web/20170505072406id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-memorial-day-holiday-schedule.xls> | archive capture 2017-05-05T07:24:06Z | T1 | `2d5a64e7ba05c1889993e91cde352312a8fb5901c5f6a4d53439002ab067542a` |
| `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | `2017-4th-of-july-holiday-schedule.xls` | <https://web.archive.org/web/20170505072205id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-4th-of-july-holiday-schedule.xls> | archive capture 2017-05-05T07:22:05Z | T1 | `d8d668183f686591879e8ee0d2db13c650c09d5a3c13e2378be068c784679453` |
| `2017-labor-day-holiday-schedule.xls @2017-10-25` | `2017-labor-day-holiday-schedule.xls` | <https://web.archive.org/web/20170505072346id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-labor-day-holiday-schedule.xls> | archive capture 2017-05-05T07:23:46Z | T1 | `660473ca09893868560b64e544c31c984a715595b1311f136af389951ef56f63` |
| `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | `2017-thanksgiving-holiday-schedule.xls` | <https://web.archive.org/web/20210126id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-thanksgiving-holiday-schedule.xls> | 2021-01-26T09:48:35Z | T1 | `b6bc3dba9f1e0c5a86d543e389d15cea6006f456e6dc2cfd7813eae31db2f869` |
| `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | `2017-christmas-holiday-schedule.xls` | <https://web.archive.org/web/20210126id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-christmas-holiday-schedule.xls> | 2021-01-26T09:48:35Z | T1 | `bfb291c9c08764ee2bcf0327690b98c58cbb4cf493723c7b1464eccb0d062d30` |
| `2018-new-years-holiday-schedule.xls @2018-01-06` | `2018-new-years-holiday-schedule.xls` | <https://web.archive.org/web/20170505072653id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-new-years-holiday-schedule.xls> | archive capture 2017-05-05T07:26:53Z | T1 | `73332758cbf95363a04cf422a672157c1b1a5bb34d4c9bd496bd7b7491a4d8cd` |
| `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | `2018-martin-luther-king-holiday-schedule.xls` | <https://web.archive.org/web/20180106225650id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-martin-luther-king-holiday-schedule.xls> | archive capture 2018-01-06T22:56:50Z | T1 | `81f30b0be680898b82175b19ee6a2fccc85b7e340fac2418b0ad00973c4a93a8` |
| `2018-presidents-day-holiday-schedule.xls @2018-05-08` | `2018-presidents-day-holiday-schedule.xls` | <https://web.archive.org/web/20180106225732id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-presidents-day-holiday-schedule.xls> | archive capture 2018-01-06T22:57:32Z | T1 | `fddc713823c58cfdc4de2486ea3b2953ecda24c01579da64eb3933d8a7efc4b8` |
| `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | `2018-good-friday-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-good-friday-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `ec3ffb0a85d061adbb53678353428eaeade9817963d6b5069db228844d9fbedf` |
| `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | `2018-memorial-day-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-memorial-day-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `b3524e36abf39b038137145620cfe1a0d46022ce732ae623d6b71af8a4b8750b` |
| `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | `2018-4th-of-july-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-4th-of-july-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `8ed9e0f74c2bcdfdda8fa80c915e6bba5ed3c5ca7ec89476f11db03b256784ae` |
| `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | `2018-labor-day-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-labor-day-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `9b8c892dd3faba6900aae949f7f0c63b525191a76dccc938aa9859920b71b40d` |
| `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | `2018-thanksgiving-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-thanksgiving-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `31fc95f5b27f8689477b4abb557f14b2cf9ac481262926e30b78adf1455adff1` |
| `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | `2018-christmas-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-christmas-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `b97ee5f47d55c4383c7f1fa7d554eba3c7f0e8ded18343aadd06d7908d9e7a91` |
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-31&toEventDate=2025-01-02> | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-01-19&toEventDate=2025-01-21> | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-02-16&toEventDate=2025-02-18> | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-04-17&toEventDate=2025-04-19> | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-05-25&toEventDate=2025-05-27> | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-06-18&toEventDate=2025-06-20> | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-07-03&toEventDate=2025-07-05> | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-08-31&toEventDate=2025-09-02> | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | <https://web.archive.org/web/20260129012159id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-24&toEventDate=2025-12-26> | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-31&toEventDate=2026-01-02> | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-01-18&toEventDate=2026-01-20> | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-02-15&toEventDate=2026-02-17> | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | <https://web.archive.org/web/20260619114118id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-04-01&toEventDate=2026-04-03> | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-05-24&toEventDate=2026-05-26> | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | <https://web.archive.org/web/20260619113404id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-18&toEventDate=2026-06-20> | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-02` | 2026-07-02 .. 2026-07-04 | <https://web.archive.org/web/20260129012216id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-02&toEventDate=2026-07-04> | archive capture 2026-01-29T01:22:16Z | T2 | `d97339fb9b8be15b7e1be70293a5b40afc38e8b09f136158f63bbccd75ea93d7` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | <https://web.archive.org/web/20260619114108id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-03&toEventDate=2026-07-05> | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-09-06&toEventDate=2026-09-08> | live retrieval 2026-09-12T04:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-11-25&toEventDate=2026-11-27> | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-24&toEventDate=2026-12-26> | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-31&toEventDate=2027-01-02> | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-01-17&toEventDate=2027-01-19> | live retrieval 2026-09-12T04:30Z | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-02-14&toEventDate=2027-02-16> | live retrieval 2026-09-12T04:30Z | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-03-25&toEventDate=2027-03-27> | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-05-30&toEventDate=2027-06-01> | live retrieval 2026-09-12T04:30Z | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-17&toEventDate=2027-06-19> | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-07-04&toEventDate=2027-07-06> | live retrieval 2026-09-12T04:30Z | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-09-05&toEventDate=2027-09-07> | live retrieval 2026-09-12T04:30Z | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-11-24&toEventDate=2027-11-26> | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-12-22&toEventDate=2027-12-25> | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |

## Holidays

**Coverage:** 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).
This wave is 39 rows over 2022-01-01..2024-12-31, **23 at T1** and **16 at T2**. The T1 rows are read from the 2022 per-asset-class workbooks, the 2023 one-pagers, and the two 2024 entries CME's own cmegroup.com worksheets `new-years-day-2024.pdf` and `christmas-day-2023.pdf` serve; the T2 rows are the operator's own `trading-hours-by-product` responses, which carry the rest of 2024 and the three 2023 holiday dates its one-pagers do not cover. 3 of the 39 are `unsourced`.
Tier: **T1** for 2010-2012, 2013-2015, 2016-2018, 2019-2021 and the 2022-2024
rows read from the operator's own published holiday schedules; **T2** for
2025-2027 and the 2022-2024 rows the operator's trading-hours service
answers, since those dates have no published schedule this crate could read. Inside a
window a date with no row is audited normal; outside every window this table has no
answer at all.

**Six audited eras, with no gap between them.** The table declares 6 coverage windows: `2010-01-01..2012-12-31`, `2013-01-01..2015-12-31`, `2016-01-01..2018-12-31`, `2019-01-01..2021-12-31`, `2022-01-01..2024-12-31`, `2025-01-01..2027-12-31`.
The eras through 2023 are the operator's own published holiday schedules at **T1**, from CME's per-holiday PDFs and .xls workbooks; 2024 shares T1 and the operator's `trading-hours-by-product` responses at **T2**; and 2025-2027 is that service alone, at T2.
Every interval from 2010-01-01 is declared, so the whole span has an answer; the eras before 2010 are out of scope below the crate's January-2010 floor.
`HolidayCoverage::windows()` lists the 6, and `contains` answers per date.

### 2010

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `CME Globex is closed` | `2010-new-years.pdf @2010-02-15T05:16:52Z` | T1 | CME prints `Jan 1` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-04-02 | closed | `CME Globex is closed` | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | CME prints `Apr 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-11-26 | early close | `12:00 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Nov 26` above it, and the year comes from the document's own identity |
| 2010-12-24 | closed | `CME Globex is closed` | `2010-christmas.pdf @2010-12-14T06:12:38Z` | T1 | CME prints `Dec 24` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-12-31 | early close | `12:00 CT` | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Dec 31` above it, and the year comes from the document's own identity |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | `CME Globex is closed` | `2011-good-friday.pdf @2011-10-28T02:37:07Z` | T1 | CME prints `Apr 22` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-11-25 | early close | `12:00 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2011-12-26 | closed | `CME Globex is closed` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | CME prints `Dec 26` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-12-27 | late open | `09:30 CT` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | `09:30 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `CME Globex is closed` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | CME prints `Jan 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-01-03 | late open | `09:30 CT` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | `09:30 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-04-06 | closed | `CME Globex is closed` | `2012-good-friday.pdf @2012-04-17T00:42:47Z` | T1 | CME prints `Apr 6` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-05-28 | late open | `19:00 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `19:00 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-07-03 | early close | `12:00 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jul 3` above it, and the year comes from the document's own identity |
| 2012-07-04 | closed | `CME Globex is closed` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | CME prints `Jul 4` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-07-05 | late open | `09:30 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `09:30 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-09-03 | late open | `19:00 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `19:00 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-11-22 | closed | `CME Globex is closed` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | CME prints `Nov 22` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-11-23 | late open and early close | `12:00 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:00 CT` closes this trade date and its own first open is stated on it |
| 2012-12-24 | early close | `12:00 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Dec 24` above it, and the year comes from the document's own identity |
| 2012-12-25 | closed | `CME Globex is closed` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | CME prints `Dec 25` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-12-26 | late open | `09:30 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `09:30 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |


### 2013

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `New Years Observed - Globex closed` | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | CME prints no session running through this date |
| 2013-01-02 | late open | `0930 CT` | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | the trade date's first open is 9:30 CT: the evening leg that would have opened earlier did not run |
| 2013-01-21 | closed | `closed` | `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | T1 | CME prints no session running through this date |
| 2013-02-18 | closed | `closed` | `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | T1 | CME prints no session running through this date |
| 2013-03-29 | closed | `CME Globex is closed` | `2013-good-friday.pdf @2013-06-23T19:59:25Z` | T1 | CME prints no session running through this date |
| 2013-05-27 | closed | `closed` | `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | T1 | CME prints no session running through this date |
| 2013-07-03 | early close | `1200 CT (CBOT & KCBT)` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT; (1230 CT for CBOT Mini-Sized grain, which this key does not model) |
| 2013-07-04 | closed | `closed` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | CME prints no session running through this date |
| 2013-07-05 | late open | `0830 CT` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run |
| 2013-09-02 | closed | `closed` | `2013-labor-day.pdf @2013-09-02T17:08:41Z` | T1 | CME prints no session running through this date |
| 2013-11-28 | closed | `closed` | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | CME prints no session running through this date |
| 2013-11-29 | late open and early close | `1200 CT (CBOT & KCBT)` / `0830 CT` | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run; the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT; (1230 CT for CBOT Mini-Sized grain, which this key does not model) |
| 2013-12-24 | early close | `1200 CT (CBOT & KCBT)` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT; (1230 CT for CBOT Mini-Sized grain, which this key does not model) |
| 2013-12-25 | closed | `Christmas Day Observed - Globex closed` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | CME prints no session running through this date |
| 2013-12-26 | late open | `0830 CT` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run |

### 2014

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `New Year's Observed - Globex closed` | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | CME prints no session running through this date |
| 2014-01-02 | late open | `0830 CT` | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run |
| 2014-01-20 | closed | `closed` | `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | T1 | CME prints no session running through this date |
| 2014-02-17 | closed | `closed` | `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | T1 | CME prints no session running through this date |
| 2014-04-18 | closed | `CME Globex is closed` | `2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z` | T1 | CME prints no session running through this date |
| 2014-05-26 | closed | `closed` | `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | T1 | CME prints no session running through this date |
| 2014-07-03 | early close | `1200 CT / 1300 ET / 1700 UTC (1230 CT for Mini-Sized grain)` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT |
| 2014-07-04 | closed | `All products closed` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | CME prints no session running through this date |
| 2014-07-07 | late open | `0830 CT / 0930 ET / 1330 UTC` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run |
| 2014-09-01 | closed | `closed` | `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | T1 | CME prints no session running through this date |
| 2014-11-27 | closed | `closed` | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | CME prints no session running through this date |
| 2014-11-28 | late open and early close | `1200 CT / 1300 ET / 1800 UTC (1230 CT for Mini-Sized grain)` / `0830 CT / 0930 ET / 1430 UTC` | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run; the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT |
| 2014-12-24 | early close | `1200 CT / 1300 ET / 1800 UTC (1230 CT for Mini-Sized grain)` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT |
| 2014-12-25 | closed | `Christmas Day Observed - Globex closed` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | CME prints no session running through this date |
| 2014-12-26 | late open | `0830 CT / 0930 ET / 1430 UTC` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run |

### 2015

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `New Year's Observed - Globex closed` | `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | T1 | CME prints no session running through this date |
| 2015-01-02 | late open | `0830 CT / 0930 ET / 1430 UTC` | `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run |
| 2015-01-19 | closed | `closed` | `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | T1 | CME prints no session running through this date |
| 2015-02-16 | closed | `closed` | `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | T1 | CME prints no session running through this date |
| 2015-04-03 | closed | `CME Globex is closed` | `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | T1 | CME prints no session running through this date |
| 2015-05-25 | closed | `closed` | `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | T1 | CME prints no session running through this date |
| 2015-07-02 | early close | `1200 CT / 1300 ET / 1700 UTC (1230 CT for Mini-Sized grain)` | `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT |
| 2015-07-03 | closed | `All products closed` | `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | T1 | CME prints no session running through this date |
| 2015-09-07 | closed | `closed` | `2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z` | T1 | CME prints no session running through this date |
| 2015-11-26 | closed | `closed` | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | CME prints no session running through this date |
| 2015-11-27 | late open and early close | `1205 CT / 1305 ET / 1805 UTC (1230 CT for Mini-Sized grain)` / `0830 CT / 0930 ET / 1430 UTC` | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run; the printed final close 12:05 CT is earlier than the family's ordinary 13:20 CT |
| 2015-12-24 | early close | `1205 CT / 1305 ET / 1805 UTC (1230 CT for Mini-Sized grain)` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | the printed final close 12:05 CT is earlier than the family's ordinary 13:20 CT |
| 2015-12-25 | closed | `Christmas Day Observed - Globex closed` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | CME prints no session running through this date |

### Documents
| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2013-4th-of-july-done.pdf @2013-07-17T05:03:33Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130717050333id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-4th-of-july-done.pdf> | archive capture 2013-07-17T05:03:33Z | T1 | `768c7813542459dbc5443e79e514bb12fd511d93284beeb880af167f6049e0f0` |
| `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130623205825id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-4th-of-july.pdf> | archive capture 2013-06-23T20:58:25Z | T1 | `52c62e72329866f12726363d762335c8dadfdd3407c3643f38d7ed74608e8eb7` |
| `2013-christmas.pdf @2014-04-12T06:24:28Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140412062428id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-christmas.pdf> | archive capture 2014-04-12T06:24:28Z | T1 | `1389ade6b120383d05e8d6e8ed9c38f1e2394dd250c86b19606548618285e848` |
| `2013-columbus-day.pdf @2012-11-19T00:15:54Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20121119001554id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-columbus-day.pdf> | archive capture 2012-11-19T00:15:54Z | T1 | `9679853fc45be642403cf58cac17852bd267f228706a696128223f7093005d2d` |
| `2013-good-friday.pdf @2013-06-23T19:59:25Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130623195925id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-good-friday.pdf> | archive capture 2013-06-23T19:59:25Z | T1 | `c05e590c950581e19ac2e8ee5327ebcfb4fc111b231b506abefae44534ee9b68` |
| `2013-labor-day.pdf @2013-09-02T17:08:41Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130902170841id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-labor-day.pdf> | archive capture 2013-09-02T17:08:41Z | T1 | `8f22671fa723d33fe36d58bb29e80ebd15370d3bf9000a9b45d56a0cab0608ef` |
| `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20121119001609id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-martin-luther-king.pdf> | archive capture 2012-11-19T00:16:09Z | T1 | `43a646480cc7ffca137890901c0fc716ed04403e0dabc1ef07749d2b21b70c4c` |
| `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130623203604id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-memorial-day.pdf> | archive capture 2013-06-23T20:36:04Z | T1 | `7cc47e352ff4874614fdb583cecd41b3ac3fcf95be6199ea5d489b8b34d2afe7` |
| `2013-new-years.pdf @2013-04-14T19:41:46Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130414194146id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-new-years.pdf> | archive capture 2013-04-14T19:41:46Z | T1 | `e29c2c968cdf20883d55acd13bab50e9df02c547826bca9d2867f5bf9b2df3f7` |
| `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130309115337id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-presidents-day.pdf> | archive capture 2013-03-09T11:53:37Z | T1 | `e1242116eec5b7f3c5748e61adbf5bf7a809f48739ad456391f1d8b4bafceff6` |
| `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140214062836id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-thanksgiving.pdf> | archive capture 2014-02-14T06:28:36Z | T1 | `1f7f6428990a5bd01065f31c4388bf5aeeb12d9219e5af6d16b1cec86bbc158b` |
| `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140708015736id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-4th-of-july-holiday-schedule.pdf> | archive capture 2014-07-08T01:57:36Z | T1 | `34faa82435c6d8bb1088593c8945b6a5faaaa67b36c6a6422a24e4a25572ba77` |
| `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121141000id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-christmas-holiday-schedule.pdf> | archive capture 2015-01-21T14:10:00Z | T1 | `8125a18c9cad9b770be39b89c3193ee419240d409e39aa510d6eb3e8eba94e25` |
| `2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140326152735id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-good-friday-holiday-schedule.pdf> | archive capture 2014-03-26T15:27:35Z | T1 | `4e8593e96eb42af2cde99f6ed906d4fe4a2ea8f976fc9f8d8f561f3049cc7f0e` |
| `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140912071608id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-labor-day-holiday-schedule.pdf> | archive capture 2014-09-12T07:16:08Z | T1 | `dfd92a530d114a1f1c68dd52be4315fd307d8f8cfb344d71de78334b9c73e4d6` |
| `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140326160215id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-martin-luther-king-holiday-schedule.pdf> | archive capture 2014-03-26T16:02:15Z | T1 | `5069a46a6acfb69bf05261abc23097fd675d159cf55e45361fb7e1e41dc0dcd4` |
| `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140708020155id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-memorial-day-holiday-schedule.pdf> | archive capture 2014-07-08T02:01:55Z | T1 | `a8e020149657b4e4730c2a357015d096a9fd47e2054ca59a85d6e6548ccb6023` |
| `2014-new-years.pdf @2013-10-07T20:58:00Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20131007205800id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-new-years.pdf> | archive capture 2013-10-07T20:58:00Z | T1 | `ca6da5edc26537d341b5148781c43c8ef00e33e832c8e602b97c57d6508b8f28` |
| `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140214192332id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-presidents-day-holiday-schedule.pdf> | archive capture 2014-02-14T19:23:32Z | T1 | `b689b4f8e9f62ba6f8c32bba46d35923a4017d1edbc83cc9ffcc2a85baded4fb` |
| `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121145456id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-thanksgiving-holiday-schedule.pdf> | archive capture 2015-01-21T14:54:56Z | T1 | `a90ae1f39414337f1a8600f55bf3a0a587214d987074885a47cfea307bc8054a` |
| `2014-veterans-day-holiday-schedule.pdf @2014-11-13T19:34:50Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20141113193450id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2014-veterans-day-holiday-schedule.pdf> | archive capture 2014-11-13T19:34:50Z | T1 | `53ea2a8b03d8b71640a2a054eb1d4cc5dd220d4ebc0fd21f7f2890e21a55c2b6` |
| `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150905222733id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-4th-of-july-holiday-schedule.pdf> | archive capture 2015-09-05T22:27:33Z | T1 | `1013e6ebea1829591946c9aa513ec6e3f99c64d99be14814610e601bb5dcc0fb` |
| `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20151123061520id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-christmas-holiday-schedule.pdf> | archive capture 2015-11-23T06:15:20Z | T1 | `7fde46210798f2cb4299903400b41ef6e2cb9aa287f8d3d360ba59d835b482ec` |
| `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150905223230id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-good-friday-holiday-schedule.pdf> | archive capture 2015-09-05T22:32:30Z | T1 | `67873caa987c9eee8de38d02a5c18d8f7d46fd82a486bab34821f6e6f6363e0b` |
| `2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150824023039id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-labor-day-holiday-schedule.pdf> | archive capture 2015-08-24T02:30:39Z | T1 | `4aab7fdb5e57420a243bbc26be6ecfa00a6317937afe3b772dfcdd8db1dd50fd` |
| `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121141012id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-martin-luther-king-holiday-schedule.pdf> | archive capture 2015-01-21T14:10:12Z | T1 | `67395dcb63d86b6e1f573a5aa37269a685dfe1b17d23edd7d95449899bce090a` |
| `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150326113938id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-memorial-day-holiday-schedule.pdf> | archive capture 2015-03-26T11:39:38Z | T1 | `4ad4398fa092393fa77cd8ed9c58ecc0f1669a41fd38a9b4cab5b05790d2dfec` |
| `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121141043id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-new-years-holiday-schedule.pdf> | archive capture 2015-01-21T14:10:43Z | T1 | `196b4ec04bcfce2cfd9783262023afe9da73bcdef1b983c8c4b464b2e115bb8a` |
| `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121192401id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-presidents-day-holiday-schedule.pdf> | archive capture 2015-01-21T19:24:01Z | T1 | `c81b04bcf43dbfd2b7c2f7aba559992cde8bb8eee59e7b310b643b636d290b94` |
| `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20160205162519id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf> | archive capture 2016-02-05T16:25:19Z | T1 | `ef96d05289667c635f435884a0a2407bcbec63149ff96e802b719100c85c887f` |
| `2015-veterans-day-schedule.pdf @2015-11-22T23:09:20Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20151122230920id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-veterans-day-schedule.pdf> | archive capture 2015-11-22T23:09:20Z | T1 | `50300dd36faff3b7d52128ea48a04e4c27012a78985b6c390ea9883e1696f98e` |

### Gaps and residual risks, 2013-2015

**This era brings the family to six audited windows.** The table as a whole carries 222 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **43 rows**: 27 stated closures, 6 early closes, 7 late opens and 3 combined late-open-and-early-close rows. Every row is at T1.

**Columbus Day 2013 and Veterans Day 2014 and 2015 carry no row.** CME's own sheets for those three dates state in session language that Globex ran a normal schedule — `Products listed on Globex are unaffected and will run on a normal schedule` for 2013-10-14 (`2013-columbus-day.pdf @2012-11-19T00:15:54Z`) and `Regular CME Globex trading hours will be in effect` for 2014-11-11 and 2015-11-11 (`2014-veterans-day-holiday-schedule.pdf @2014-11-13T19:34:50Z` and `2015-veterans-day-schedule.pdf @2015-11-22T23:09:20Z`) — so the block records each as `normal` and this table ships nothing: inside a declared window silence is the positive claim that the date was audited normal, and these dates are audited rather than skipped. The three sheets have no row of their own to cite, so they are listed in this era's `### Documents` table without being any row's document.

**The grains day session does not run on the Monday and Thursday holidays, so those dates are `Closed`.** CME halts the `Grain, Oilseed & MGEX Products` line at 12:00 CT on the holiday itself and reopens it at 19:00 CT the same evening for the next trade date: the holiday's own trade date has no final close of its own, and the closure removes it with the evening leg that would have opened it. The three `late_open_and_early_close` rows are the Thanksgiving Fridays, where the 08:30 CT reopen is the printed first open and the printed final close is 12:00 CT in 2013 and 2014 and 12:05 CT in 2015. The grains close is 13:15 CT through 2015-07-04 and 13:20 CT from 2015-07-05 (CME SER-7395R), which is why both 2015-11-27 and 2015-12-24 read 12:05 CT where the two earlier years read 12:00 CT.

### 2016

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Globex closed` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-01-18 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-02-15 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-03-25 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-05-30 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-07-04 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-09-05 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-11-24 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-11-25 | late open and early close | `0830 CT / 0930 ET / 1430 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | the operator prints this date's own first open after the holiday; the row is keyed to the trade date that open belongs to |
| 2016-12-23 | early close | `1205 CT / 1305 ET / 1805 UTC` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-12-26 | closed | `Globex closed` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2017

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `Globex closed` | `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-01-16 | closed | `no session printed` | `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-02-20 | closed | `no session printed` | `2017-presidents-day-holiday-schedule.xls @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-04-14 | closed | `no session printed` | `2017-good-friday-holiday-schedule.xls @2017-05-05` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-05-29 | closed | `no session printed` | `2017-memorial-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-07-03 | early close | `12:05 CT / 13:05 ET` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-07-04 | closed | `no session printed` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-09-04 | closed | `no session printed` | `2017-labor-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-11-23 | closed | `no session printed` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-11-24 | late open and early close | `08:30 CT / 09:30 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | the operator prints this date's own first open after the holiday; the row is keyed to the trade date that open belongs to |
| 2017-12-22 | early close | `12:05 CT / 13:05 ET` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-12-25 | closed | `no session printed` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2018

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `no session printed` | `2018-new-years-holiday-schedule.xls @2018-01-06` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-01-15 | closed | `no session printed` | `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-02-19 | closed | `no session printed` | `2018-presidents-day-holiday-schedule.xls @2018-05-08` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-03-30 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-05-28 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-07-03 | early close | `12:05 CT / 13:05 ET` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-07-04 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-09-03 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-11-22 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-11-23 | late open and early close | `08:30 CT / 09:30 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | the operator prints this date's own first open after the holiday; the row is keyed to the trade date that open belongs to |
| 2018-12-24 | early close | `12:05 CT / 13:05 ET` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-25 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-12-26 | late open | `08:30 CT / 09:30 ET (Wednesday, December 26)` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the operator prints this date's own first open after the holiday; the row is keyed to the trade date that open belongs to |

**Interpretive step, 2018-12-26:** the block records this date's status as `normal`, and the crate ships a row for it. The Christmas sheet prints no Tuesday-evening grain leg — `Tuesday, December 25` carries `Globex Closed` — and prints the Wednesday session as `Pre-opening` 06:00, `Open` 08:30, `Close` 13:20, so the trade date's first open is 08:30 CT and the row is a late open. **Interpretive step, the three days after Thanksgiving:** their kind is `late open and early close`, not a plain early close: CME withdrew the prior-evening leg and printed the day session's own 08:30 CT open beside the 12:05 CT close.


### 2019

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Closed for New Year's` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-01-02 | late open | `"Pre-open 6:00CT /12:00 UTC Open 8:30 CT /14:30 UTC" (cell as printed, Wednesday Jan 2 Open column)` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | CME's next open for trade date 2019-01-02 is the day session's own `08:30 CT` on that date, later than the family's ordinary `19:00 CT` first open, which falls on the preceding local date: the leg that would have carried this trade date did not run. `08:30` is earlier than that ordinary open, so the crate's cutoff lands on the trade date itself. |
| 2019-01-21 | closed | `closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-02-18 | closed | `closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-04-19 | closed | `Closed for Good Friday` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-05-27 | closed | `closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-06-19 | unsourced | no CME document covers this date (see `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`) | `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2019-07-03 | early close | `1205 CT / 1705 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Grain & Oilseed` line prints `12:05 CT` as this date's own final close, so the leg that opened the previous evening at 19:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-07-04 | closed | `Markets Closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-07-05 | late open | `"Friday @ 0830 / 1330 UTC" (cell as printed; CME prints no CT token in this cell — the column is Central Time, as the same column's other cells show, e.g. "Regular @ 1700 CT / 2200 UTC")` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME's next open for trade date 2019-07-05 is the day session's own `08:30 CT` on that date, later than the family's ordinary `19:00 CT` first open, which falls on the preceding local date: the leg that would have carried this trade date did not run. `08:30` is earlier than that ordinary open, so the crate's cutoff lands on the trade date itself. |
| 2019-09-02 | closed | `closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-11-28 | closed | `closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-11-29 | late open and early close | `Friday 29 Nov, Regular per Product` / `1205 CT / 1805 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME withdrew the prior-evening leg and printed the day session's own `08:30 CT` open beside the `12:05 CT` final close on trade date 2019-11-29, so both boundaries move and the crate keys one row carrying both. |
| 2019-12-24 | early close | `1205 CT / 1805 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Grain & Oilseed` line prints `12:05 CT` as this date's own final close, so the leg that opened the previous evening at 19:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-25 | closed | `Closed for Christmas` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-12-26 | late open | `Thu 26 Dec, Open 8:30 CT /14:30 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME's next open for trade date 2019-12-26 is the day session's own `08:30 CT` on that date, later than the family's ordinary `19:00 CT` first open, which falls on the preceding local date: the leg that would have carried this trade date did not run. `08:30` is earlier than that ordinary open, so the crate's cutoff lands on the trade date itself. |

### 2020

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Closed for New Year's` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-01-02 | late open | `Thu 2 Jan, Open 8:30 CT /14:30 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME's next open for trade date 2020-01-02 is the day session's own `08:30 CT` on that date, later than the family's ordinary `19:00 CT` first open, which falls on the preceding local date: the leg that would have carried this trade date did not run. `08:30` is earlier than that ordinary open, so the crate's cutoff lands on the trade date itself. |
| 2020-01-20 | closed | `closed` | `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-02-17 | closed | `closed` | `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-04-10 | closed | `Closed for Good Friday` | `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-05-25 | closed | `closed` | `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-06-19 | unsourced | no CME document covers this date (see `2020-holiday-calendars.zip @2026-07-30T11:18:34Z`) | `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2020-07-02 | early close | `1205 CT / 1705 UTC` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Grain & Oilseed` line prints `12:05 CT` as this date's own final close, so the leg that opened the previous evening at 19:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-07-03 | closed | `Markets Closed` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-09-07 | closed | `closed` | `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-11-26 | closed | `closed` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-11-27 | late open and early close | `Friday 27 Nov, Regular per Product` / `1205 CT / 1805 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME withdrew the prior-evening leg and printed the day session's own `08:30 CT` open beside the `12:05 CT` final close on trade date 2020-11-27, so both boundaries move and the crate keys one row carrying both. |
| 2020-12-24 | early close | `1205 CT / 1805 UTC` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Grain & Oilseed` line prints `12:05 CT` as this date's own final close, so the leg that opened the previous evening at 19:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-25 | closed | `Closed for Christmas` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2021

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Closed for New Year's` | `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-01-18 | closed | `closed` | `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-02-15 | closed | `closed` | `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-04-02 | closed | `Closed for Good Friday` | `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-05-31 | closed | `closed` | `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-06-19 | unsourced | no CME document covers this date (see `2021-holiday-calendars.zip @2026-08-30T10:03:27Z`) | `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2021-07-05 | closed | `Markets Closed` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-07-06 | late open | `"Tuesday July 6 Regular @ 0830 CT / 1330 UTC" (cell as printed)` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME's next open for trade date 2021-07-06 is the day session's own `08:30 CT` on that date, later than the family's ordinary `19:00 CT` first open, which falls on the preceding local date: the leg that would have carried this trade date did not run. `08:30` is earlier than that ordinary open, so the crate's cutoff lands on the trade date itself. |
| 2021-09-06 | closed | `closed` | `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-11-25 | closed | `closed` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-11-26 | late open and early close | `Friday 26 Nov, Regular @ 8:30 CT / 1430 UTC` / `1205 CT / 1805 UTC` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME withdrew the prior-evening leg and printed the day session's own `08:30 CT` open beside the `12:05 CT` final close on trade date 2021-11-26, so both boundaries move and the crate keys one row carrying both. |
| 2021-12-24 | closed | `Closed for Christmas` | `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### Documents
This era's rows cite the ids below — CME Group's own Globex holiday schedules, all at **T1**. The 2020 and 2021 sheets and the three `Unsourced` Juneteenth rows are members of their year's consolidated annual bundle; the 2019 sheets likewise, and 1-2 January 2019 come from the December-2018 supplement CME published before the bundle. Each id resolves to the URL the bytes were read at — an Internet Archive raw replay of CME's own file — with the capture time in UTC, the tier and the sha256. All of them resolve in the research store's `holidays/raw/cme-2019-2021/`, whose `INDEX.md` carries the byte counts and whose `shasum.txt` hashes every workbook. The next `### Documents` table in this file is the 2022-2024 era's.
| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `1e861e355238903b013c1288f4eb9e8026e6ddd5fc1001dfd2acdbfcc1832e05` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `8546b2a9e42c92a4bcf2d2e8209906f482467aeb549ed2a02eb95690954ddd16` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `8ccc652c9e420f20000110bf9e6522ce5ab3a4ac1acd6cca465320da42409dfc` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `cfaa08b30242010a65af781d3e485f72642a9ed55ecfe8fbad1aa9b3b20b7ad9` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `68b3ac93f8b9077bb82b561d8043c0661475de44f5e5b869b900faa33cadf758` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `d439d73aa100a9e960a121cac158d2813dceb570f7af80f9cf163e17abedc42a` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `4d55425087a171b10b37a634d9d95cba0ae863e4e2cba282599ef1016b404fbe` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `6625c29097309fb5c209459438b0e868d88623a350324db9404430cf3f13d7fe` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `cd6c895435ef2e7a14408ee74ab4fba140d462abefe00d2ee0333577253049c3` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `99ea9b63ef7d756116312e0061275b1ee7cd50ac547a55f09098a4e58973cb31` |
| `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20180107041343id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2019-new-years-holiday-schedule-compact.xls> | archive capture 2018-01-07T04:13:43Z | T1 | `2684a5f1b3a9f65802f6911ca6089e2cb68c3cdf2520dfaf3cdcf3105328c188` |
| `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `5263a4a5e9076bc0e69c7cd0e3fd9f82dc4d80b66f1b56f14070f08c1d762b59` |
| `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `05fee0fd5ec2e9d403ac32ad103612569c8f269299b4bfef931c50c311b8740a` |
| `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `d342b6b0a075591255b45b0dc58dfede5bf5cc28077aa8b362591b82224c24ae` |
| `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `aa0936e9278904f40bc23abba57c2c379207af49d0cb6971f07289010fe1745e` |
| `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `85acc34db0b86dfe09824c3cfe2f4c7fb21f02ce8ad5593587afe7affa84221f` |
| `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `bacf30ad37e6843f7162a6457e17dec7e0024474e9b8e0d35f9ef12fc9671d25` |
| `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `f3052963d22aca969eefb81ff8996662fc1b0a17f0bfdf53e8df37b0af5ddc5b` |
| `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `a42697d72f03fdcc9229a17f99320418fd61a52ceb7bb82636c01cb8d818606d` |
| `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `bcf6b5e616b91663abe532953d47dd85af9fb05e26cc4d9fe72329414ef598b4` |
| `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `f7b782e1effb08ff84b75daf4a7d59e1cd7d64645c8504cf10d54af9d2199928` |
| `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `0ee0860a3a0e035eb9d079419aca3cafcc4256d296fa916647987c396dda8c59` |
| `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `e80ee0b766f582091d04f04e1a79595c988848dec269e37d0045d7dc33a68235` |
| `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `2d54d45609bc774e61d65d92d9a3165372950999400dd5b102b7a9319c560356` |
| `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `6444dd5b3513a79c4bce6b9db0746c969e432a3c2cdb5bb918c80ad44bf8fe9b` |
| `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `613623b6d3bb4ac76120d284e198799e4eedf6870119449df762a613ab53888e` |
| `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `75b06b120bd92f77aa85f8acaca2ec661e06dba98bb64f3863a8d24987475968` |
| `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `83724afe5e7f073bf58d3fd0118269c92d7d1c0c8c18ff716fbebe5e51b7c64c` |
| `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `7a8d5ce35c639998abf64723de2cc2d649dc015b1aa6cfc3bc326ab5cdfd0c4f` |
| `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `df6cdbc4c996109fe5a5829b5e8aaf622c20c6f16f1464dcdd9d5250c4163a47` |
### Gaps and residual risks, 2019-2021
**This era brought the family to five audited windows; the 2013-2015 wave has since added a sixth.** The table as a whole carries 222 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **42 rows**: 27 full closures, 4 early closes, 5 late opens, 3 late opens with early closes and 3 `Unsourced` rows. Every row is at T1. Every interval from 2010-01-01 is inside a declared window, so `holiday_coverage` answers for the whole span rather than reporting an unaudited gap.
**Juneteenth 2019, 2020 and 2021 — three `Unsourced` rows.** CME published no Juneteenth schedule in any of the three years. Each row cites that year's own consolidated bundle — `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`, `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` and `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — whose member lists are CME's own account of every Globex holiday schedule it published that year and which carry no Juneteenth sheet; the four archived `holiday-calendar.html` index pages name none either, and a fresh 2018-2027 prefix CDX enumeration (`raw/cme-2019-2021-fix/cdx/cdx-files-2018-2027.json`, 369 rows, 340 distinct filenames) finds no `juneteenth` filename before 2022. Inside a contiguous window silence is the positive claim that a date was audited normal, which is false for a date the operator later marks as a holiday, so all three ship `Unsourced`, which clips nothing. 2021-06-19 is a **Saturday**: no family has a trade date there and the row changes no answer, and the row is keyed to the operator's own calendar date for the holiday rather than to an observed date CME never states. Closing condition: a CME holiday schedule naming Juneteenth in one of these three years.
**Columbus Day and Veterans Day — six dates with no row.** 2019-10-14, 2019-11-11, 2020-10-12, 2020-11-11, 2021-10-11 and 2021-11-11 lie inside this window and carry no row, so the family's ordinary week stands there. CME published settlement-time and OTC-clearing advisories for these dates — the 2019 ZIP's `settlement-notices/*-settlement-times.pdf` members and, for example, `2021-veterans-day-advisory.pdf` — but never a Globex trading schedule for them. A settlement notice is not session language (LAW-SESSION-NOT-EXPIRY), so no row is keyed to one and the block's `missing` register records the dates as gaps rather than as sourced normality. Closing condition: a CME Globex holiday schedule naming one of these dates.
**The day-after-closure rows (8).** CME withdraws the prior-evening leg on the eve of a closure and prints the next trade date's own day session instead, so the first trade of that date is the ordinary 08:30 CT open, later than the 19:00 CT first open that would otherwise carry it. `late_open(08:30)` ships on `2019-01-02`, `2019-07-05`, `2019-12-26`, `2020-01-02`, `2021-07-06`, and `late_open_and_early_close(08:30, 12:05)` on `2019-11-29`, `2020-11-27`, `2021-11-26`, where the same day also prints the family's half-day close. The counter-examples ship **no** row because the operator's sheet prints the ordinary 19:00 CT evening leg that carries them: 2019-04-22 (after Good Friday), 2020-07-06 (after the observed 2020-07-03), 2020-12-28 and 2021-12-27 (after the two Christmas closures) and 2021-01-04 (after New Year's Day 2021). `globex_grains` is the only family whose era needs the shape: every other wrapped family's sheets print an ordinary evening open on the day after a closure.
**The `dairy` fold (reporting only).** CME prints a `Dairy` line on the same sheets and the crate has no key for it, so this wave folds it into this family for reporting only: the folded group's own instants are compared against this family's and never key a row, because the two run on different clocks (`dairy` closes at its own hour, where this family's ordinary close is 13:20 CT). The two lines differ on the 17 dates below; the closing condition is a consumer that maps the group (LAW-SERVICE-TIERS).

- `2019-01-02` — this family late open 08:30 CT; the `dairy` line ships no row.
- `2019-04-18` — this family ships no row; the `dairy` line early close 13:55 CT.
- `2019-07-03` — this family early close 12:05 CT; the `dairy` line early close 12:00 CT.
- `2019-07-05` — this family late open 08:30 CT; the `dairy` line ships no row.
- `2019-11-29` — this family late open 08:30 CT and early close 12:05 CT; the `dairy` line closed: no trade date.
- `2019-12-24` — this family early close 12:05 CT; the `dairy` line early close 12:00 CT.
- `2019-12-26` — this family late open 08:30 CT; the `dairy` line ships no row.
- `2019-12-31` — this family ships no row; the `dairy` line early close 13:55 CT.
- `2020-01-02` — this family late open 08:30 CT; the `dairy` line ships no row.
- `2020-04-09` — this family ships no row; the `dairy` line early close 13:55 CT.
- `2020-07-02` — this family early close 12:05 CT; the `dairy` line early close 12:00 CT.
- `2020-11-27` — this family late open 08:30 CT and early close 12:05 CT; the `dairy` line closed: no trade date.
- `2020-12-24` — this family early close 12:05 CT; the `dairy` line early close 12:00 CT.
- `2020-12-31` — this family ships no row; the `dairy` line early close 13:55 CT.
- `2021-04-01` — this family ships no row; the `dairy` line early close 13:55 CT.
- `2021-07-06` — this family late open 08:30 CT; the `dairy` line ships no row.
- `2021-11-26` — this family late open 08:30 CT and early close 12:05 CT; the `dairy` line closed: no trade date.

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-01-17 | closed | `no session printed` — the operator prints no session for this date | `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-02-21 | closed | `no session printed` — the operator prints no session for this date | `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-04-15 | closed | `"Globex Closed"` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-05-30 | closed | `no session printed` — the operator prints no session for this date | `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-06-20 | closed | `no session printed` — the operator prints no session for this date | `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-07-04 | closed | `no session printed` — the operator prints no session for this date | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-07-05 | late open | `08:30` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | the closure date printed no evening leg, so this trade date's first open is the day session's own `08:30`, earlier than the family's ordinary 19:00 CT first open; the row is therefore keyed to the trade date itself rather than the preceding local date |
| 2022-09-05 | closed | `no session printed` — the operator prints no session for this date | `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-11-24 | closed | `no session printed` — the operator prints no session for this date | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-11-25 | late open and early close | `08:30` / `12:05` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | the closure date printed no evening leg and this trade date ends early, so both boundaries move: the day session opens `08:30` and its own final close is `12:05`, both on the trade date the block prints |
| 2022-12-26 | closed | `"Globex Closed"` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `"Globex Closed"` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-01-16 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-01-15` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-02-20 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-02-19` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-04-07 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-04-06` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-05-29 | closed | `no session printed` — the operator prints no session for this date | `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-06-19 | closed | `19:00 (OPEN)` — the only clock in this date's own entry is the evening re-open that carries the next trade date | `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-07-04 | closed | `no session printed` — the operator prints no session for this date | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-07-05 | late open | `08:30` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | the closure date printed no evening leg, so this trade date's first open is the day session's own `08:30`, earlier than the family's ordinary 19:00 CT first open; the row is therefore keyed to the trade date itself rather than the preceding local date |
| 2023-09-04 | closed | `19:00 (OPEN)` — the only clock in this date's own entry is the evening re-open that carries the next trade date | `labor-day-2023.pdf @2023-08-02T19:24:46Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-11-23 | closed | `no session printed` — the operator prints no session for this date | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-11-24 | late open and early close | `08:30` / `12:05` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | the closure date printed no evening leg and this trade date ends early, so both boundaries move: the day session opens `08:30` and its own final close is `12:05`, both on the trade date the block prints |
| 2023-12-25 | closed | `no session printed` — the operator prints no session for this date | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-12-26 | late open | `08:30` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | the closure date printed no evening leg, so this trade date's first open is the day session's own `08:30`, earlier than the family's ordinary 19:00 CT first open; the row is therefore keyed to the trade date itself rather than the preceding local date |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `no session printed` — the operator prints no session for this date | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-01-02 | late open | `08:30` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | the closure date printed no evening leg, so this trade date's first open is the day session's own `08:30`, earlier than the family's ordinary 19:00 CT first open; the row is therefore keyed to the trade date itself rather than the preceding local date |
| 2024-01-15 | closed | `19:00 (OPEN)` — the only clock in this date's own entry is the evening re-open that carries the next trade date | `CME-SVC-2024-01-14` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-02-19 | closed | `19:00 (OPEN)` — the only clock in this date's own entry is the evening re-open that carries the next trade date | `CME-SVC-2024-02-18` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-03-29 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-03-28` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-05-27 | closed | `19:00 (OPEN)` — the only clock in this date's own entry is the evening re-open that carries the next trade date | `CME-SVC-2024-05-26` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-06-19 | closed | `19:00 (OPEN)` — the only clock in this date's own entry is the evening re-open that carries the next trade date | `CME-SVC-2024-06-18` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-07-04 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-07-03` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-07-05 | late open | `08:30` | `CME-SVC-2024-07-03` | T2 | the closure date printed no evening leg, so this trade date's first open is the day session's own `08:30`, earlier than the family's ordinary 19:00 CT first open; the row is therefore keyed to the trade date itself rather than the preceding local date |
| 2024-09-02 | closed | `no session printed` — the operator prints no session for this date | `CME-SVC-2024-09-01` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-11-28 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-11-27` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-11-29 | late open and early close | `08:30` / `12:05` | `CME-SVC-2024-11-27` | T2 | the closure date printed no evening leg and this trade date ends early, so both boundaries move: the day session opens `08:30` and its own final close is `12:05`, both on the trade date the block prints |
| 2024-12-24 | early close | `12:05` | `CME-SVC-2024-12-24` | T2 | CME prints `12:05` as the final close of the session that opened the previous evening at 19:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-25 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-12-24` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 19:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-12-26 | late open | `08:30` | `CME-SVC-2024-12-24` | T2 | the closure date printed no evening leg, so this trade date's first open is the day session's own `08:30`, earlier than the family's ordinary 19:00 CT first open; the row is therefore keyed to the trade date itself rather than the preceding local date |
### Documents

This era's rows cite the ids below: CME Group's own published holiday schedules at **T1** (2022 per-asset-class workbooks, 2023 one-pagers, and the 2024 Good Friday and year-end service windows as archived), and responses of CME's own `trading-hours-by-product` service at **T2**. Each id resolves to the URL it was read at — an Internet Archive raw replay for a saved capture, the operator's own endpoint for a live retrieval — with the capture or retrieval time in UTC, the tier and the sha256. All thirty artifacts resolve in the research store's `holidays/raw/cme-2022-2024/` and `holidays/raw/cme-2022-2024-fix/`, whose `INDEX.md` files carry the byte counts.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220117212230id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-mlk-day-holiday-schedule.xls> | archive capture 2022-01-17T21:22:30Z | T1 | `896944fa701062e2e1ee305f8adb696ba8ab03655a96874c7885f2328177626e` |
| `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704073810id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-presidents-day-holiday-schedule.xls> | archive capture 2022-07-04T07:38:10Z | T1 | `07932975d04ccabad0fb53f33f946f1a6fd8a696bafdb21f95f030ede9a84516` |
| `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-good-friday-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `a82936ab14d1b1f7041583123289c4de401c66ace4eea7e90fa9f60c1a3f3b7e` |
| `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065438id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-memorial-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:38Z | T1 | `0d1b1f89a315cae22a5857a7027a514e3086c1cccfee42fddddff3caba0c2230` |
| `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220620200210id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-juneteenth-holiday-schedule.xls> | archive capture 2022-06-20T20:02:10Z | T1 | `bc9f2caf26a73a13029f177fcc6468bdc8662899ff935f44329ee1a95c8b667b` |
| `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065450id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-independence-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:50Z | T1 | `1ea0459d8aa0fd7ec5147614855f6d0d43607efefdc3aa9c6b1e18ddbfd54fde` |
| `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065441id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-labor-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:41Z | T1 | `28d533f25d1af74af043e411635f1933fd4ff6359c80c049f87f019d2f1d57d0` |
| `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20221122060801id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-thanksgiving-holiday-schedule.xls> | archive capture 2022-11-22T06:08:01Z | T1 | `64341a65350de982a6a05760504132e173611151ae3164c6f9437509a406edcb` |
| `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065430id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-christmas-holiday-schedule.xls> | archive capture 2022-07-04T06:54:30Z | T1 | `2dd1d531514989845dcb6ce6d767db5dd3f956ac6777ab1cfe36d6843f3762e7` |
| `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2023-new-years-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `eefafd1066f406edbe6167ddf8ad13c0697b124a337c0893ce5a5da200c783d3` |
| `CME-SVC-2023-01-15` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-01-15&toEventDate=2023-01-17&isProtected&_t=1720455278636> | archive capture 2024-07-08T16:14:38Z | T2 | `507fd196a7654ddd916218b2aa24a146eeaca4e1f7cb7daded4e7a9c689cda82` |
| `CME-SVC-2023-02-19` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-02-19&toEventDate=2023-02-21&isProtected&_t=1720455278640> | archive capture 2024-07-08T16:14:38Z | T2 | `d063238a83e8cb84d4484a86b26cb1d976f1ccfdb91972eecc08dcc0cec49412` |
| `CME-SVC-2023-04-06` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-04-06&toEventDate=2023-04-08&isProtected&_t=1720455278642> | archive capture 2024-07-08T16:14:38Z | T2 | `0543d5f6d2efd4aa6f4132ce9f5425b9ba9f64de08eccd677b78d6b43c6e9d4f` |
| `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230420224018id_/https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf> | archive capture 2023-04-20T22:40:18Z | T1 | `7657bc8089ca669cfd244c2e3e697b47a7e650f5d957efe5b32da001622acb35` |
| `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230613185949id_/https://www.cmegroup.com/trading-hours/files/juneteenth-2023.pdf> | archive capture 2023-06-13T18:59:49Z | T1 | `831f7f63e197ce58436780830aa515cb08dcf92bdf82a540265c2d21a4bffa43` |
| `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230627125057id_/https://www.cmegroup.com/trading-hours/files/4th-of-july-2023.pdf> | archive capture 2023-06-27T12:50:57Z | T1 | `ccbc1f1fc39ba2219fd748372faa985798fee7eaffabf69f08956f5465a769e3` |
| `labor-day-2023.pdf @2023-08-02T19:24:46Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230802192446id_/https://www.cmegroup.com/trading-hours/files/labor-day-2023.pdf> | archive capture 2023-08-02T19:24:46Z | T1 | `39a4c075437fdc7134166328eca730e3cabf3bea00ab369466dff63c99f8e948` |
| `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20231203205929id_/https://www.cmegroup.com/trading-hours/files/thanksgiving-day-2023.pdf> | archive capture 2023-12-03T20:59:29Z | T1 | `99e187b3f3899e1062d662e148d5978e0cd075b961e6fc79550d4393812307e8` |
| `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260719095248id_/https://www.cmegroup.com/trading-hours/files/christmas-day-2023.pdf> | archive capture 2026-07-19T09:52:48Z | T1 | `edcde0fcf61d3414cee2a332453db861e0e2d7edf81d89a5f379c44b2e72d5cf` |
| `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260811165716id_/https://www.cmegroup.com/trading-hours/files/new-years-day-2024.pdf> | archive capture 2026-08-11T16:57:16Z | T1 | `34e60f8c97623df30e00f0ad8e4eeda20b99001b6c35d64f735828fecec5b9b8` |
| `CME-SVC-2024-01-14` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-01-14&toEventDate=2024-01-16&isProtected&_t=1720455278663> | archive capture 2024-07-08T16:14:39Z | T2 | `a8fe0f3eed4c67939d3add5fa656173bc4119c3765174b084c0622a1934e0d22` |
| `CME-SVC-2024-02-18` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-02-18&toEventDate=2024-02-20&isProtected&_t=1720455278669> | archive capture 2024-07-08T16:14:39Z | T2 | `af5ddb57cfa8bd6d377a0784fcdd33589e2055ede0a09b7fe4ae3e41d0d20f9a` |
| `CME-SVC-2024-03-28` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-03-28&toEventDate=2024-03-30&isProtected&_t=1720455278672> | archive capture 2024-07-08T16:14:39Z | T2 | `9b41709219e36f56296843fe589e362a7513e9a98132a682a1b956f2e40c593b` |
| `CME-SVC-2024-05-26` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-05-26&toEventDate=2024-05-28&isProtected&_t=1720455278675> | archive capture 2024-07-08T16:14:39Z | T2 | `ccff9685aac5670c5eb4a7e2b86cf1d78a5c2c08325cd03673ab8c1e6f560138` |
| `CME-SVC-2024-06-18` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-06-18&toEventDate=2024-06-20&isProtected&_t=1720455278677> | archive capture 2024-07-08T16:14:39Z | T2 | `57acecac3e1ad1a50dda8e3bcd6ca926b4d4c6c2ebc9d8324eb73332ed6a20ec` |
| `CME-SVC-2024-07-03` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-07-03&toEventDate=2024-07-05&isProtected&_t=1720455278680> | archive capture 2024-07-08T16:14:39Z | T2 | `6ef0e2b055c349249f5cc7ea7c98132a31111a01ce7af028e13584c0f82d4d38` |
| `CME-SVC-2024-09-01` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-09-01&toEventDate=2024-09-03&isProtected&_t=1720455278683> | archive capture 2024-07-08T16:14:39Z | T2 | `bbe6c78555cba0437fe4b9c6eac0063983ecae49056877db618ed2b29c109546` |
| `CME-SVC-2024-11-27` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-11-27&toEventDate=2024-11-29&isProtected&_t=1720455278685> | archive capture 2024-07-08T16:14:39Z | T2 | `f6a15f26991d25f8c6821fa6c967d0e76b2ad3f775bed62a2a001ec2fd7e389a` |
| `CME-SVC-2024-12-24` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-24&toEventDate=2024-12-26&isProtected&_t=1734710019537> | archive capture 2024-12-20T15:53:40Z | T2 | `183c85160e31d53fde30c422048b8e778937d8866f3ea1c45e2dd570ea9db1c8` |
### Gaps and residual risks, 2022-2024

**The three 2023 dates the operator published nothing for — 2023-01-16, 2023-02-20 and 2023-04-07.** They ship `Unsourced`, cited to the T2 captures `CME-SVC-2023-01-15`, `CME-SVC-2023-02-19` and `CME-SVC-2023-04-06`, each of which is the operator's own machine channel read as bytes and returns an empty event list for its window: 2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023), 2023 Presidents Day (Monday 20 February 2023) and 2023 Good Friday (Friday 7 April 2023). The block's `missing` register records the channels searched — `holiday-calendar/files/*.xls`, of which only a compact MGEX/DME workbook exists for MLK, and `trading-hours/files/<holiday>-2023.pdf`, which 404s — so the gap is "not worked up", not "no source exists". `Unsourced` clips nothing, so each of these dates still resolves to the family's ordinary week. Closing condition: a CME holiday schedule for 2023 covering this date at T1, or a T2 window that carries its events. The day after each of the three is a second, unmodelled gap: 2023-01-17 and 2023-02-21 (and 2023-04-10 for `globex_grains`) may have lost their prior-evening leg the way 2024-01-02 and 2023-12-26 did, and no artifact this crate read states whether they did, so they ship no row and the family's ordinary week stands there.

**The 2024-04-01 re-open.** The Good Friday window `CME-SVC-2024-03-28` ends at 2024-03-30 and prints empty event lists for both 2024-03-29 and 2024-03-30, so CME published nothing this crate read for the Sunday 2024-03-31 evening leg that would carry trade date 2024-04-01. Inside a contiguous window silence would read as audited normal, so 2024-04-01 ships **no row** and is a declared gap rather than an audited one. Closing condition: a CME service response covering 2024-03-31, or a T1 statement of that Sunday's re-open. For `globex_grains` the same silence is what withholds a possible late open on 2024-04-01.

**The `dairy` product group has no crate key and is folded into `globex_grains`.** CME publishes it under the joined label "Grains & Oilseeds include Dairy", but dairy's own clock is the wrapped 17:00-16:00 CT leg (16:00 CT, 13:55 CT on Fridays) while this family's is 19:00 CT into a 07:45 CT pause, the 08:30-13:20 CT day session and the 14:30-16:00 CT post-close. A family table holds one scalar boundary per trade date, so the folded group cannot be represented by this family's row and the crate ships the modelled family's own row. On four dates the two lines state different instants, and both are recorded here: 2022-11-25, 2023-11-24 and 2024-11-29 — dairy states a closure of its own ("Dairy products will remain closed until their regularly scheduled open on Sunday Nov 27th @ 17:00" in the 2022 workbook; no Thursday or Friday dairy entry at all on the 2023 one-pager; `events []` for both 2024-11-28 and 2024-11-29 in `CME-SVC-2024-11-27`) where this family ships a late open at `08:30` and an early close at `12:05` — and 2024-12-24, where dairy states `12:00 closed` in `CME-SVC-2024-12-24` and this family ships `12:05`. The crate ships the `globex_grains` row on all four.

**Six late opens are keyed to trade dates the block carries no entry for.** 2022-07-05, 2023-07-05, 2023-12-26, 2024-01-02, 2024-07-05 and 2024-12-26 each ship a `late open` at `08:30`, read from the closure date's own printed re-open rather than from an entry of their own: `2022-independence-day-holiday-schedule.xls` prints "Tuesday, July 5 Pre-opening** 06:00, Open 08:30" with the note that the grain market did not re-open between Friday's close and Tuesday morning; `4th-of-july-2023.pdf` prints "TRADE DATE: WED 5 JULY / 06:00 (PREOPEN) / 08:30 (OPEN)"; `christmas-day-2023.pdf` prints the same pair for Tuesday 26 December; `new-years-day-2024.pdf` for Tuesday 2 January; `CME-SVC-2024-07-03` prints `2024-07-05: 06:00 preopen, 08:30 open`; and `CME-SVC-2024-12-24` prints `2024-12-26: 06:00 preopen, 08:30 open`. The counter-example that keeps the rule honest is 2022-12-27, which ships no row: the 2022 Christmas entry prints "Globex Closed, Pre-opening** 16:00, Open 19:00", i.e. the holiday itself carries the ordinary evening leg for that trade date. Closing condition for all six: a CME document stating the trade date's own first open in session language.

**Normal-week notes that ship no row.** Where a printed token falls outside the family's ordinary week but moves no boundary a scalar holiday row can state, the date ships nothing and the token is recorded here: the 2022 New Year's workbooks print the `Nikkei/TOPIX BTIC` `Close 00:00` on 2022-01-01, a BTIC close for the next trade date; the 2023 and 2024 Independence Day and New Year grain entries print the next trade date's `06:00 (PREOPEN)`, which is the no-evening-leg marker the six `globex_grains` late opens are read from — those rows are in that family's own table and evidence file and this family has none; and the 2024-12-31 grain entry points at `2025-01-02 06:00 preopen`, outside this window, so no row ships and the 2025-2027 table must state that trade date.

### 2025-2027 (T2)

**Rows:** 54 — 31 `closed`, 2 `early close`, 3 `late open and early close`, 18
`replacement blocks`, 0 `unsourced`.

**Channel.** Every row comes from CME Group's own trading-hours service,
`https://www.cmegroup.com/services/trading-hours-by-product`, the endpoint
`cmegroup.com/trading-hours.html` itself calls to render its per-asset-class Holiday Hours
table. It is the operator's own machine channel, read as bytes and saved, so the block is T2
under LAW-PRIMARY-SOURCES. Below, `[THBP-A]` abbreviates the ten-product query
`?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`
(the set CME's own page queries, `ZC` among them) and `[THBP-B]` the five-product query
`?id=168,167,320,323,19,27` (`ZS` and `ZW` among them); a window is appended as
`&fromEventDate=<first>&toEventDate=<last>`. `WA:<ts>id_/` is the
`https://web.archive.org/web/<ts>id_/` one-step replay prefix. Bytes, per-file sha256 and
the verbatim re-fetch URLs are in the research store under
`holidays/raw/cme-2025-2027/INDEX.md`, `holidays/raw/cme-2025-2027-fix/INDEX.md` and
`holidays/raw/cme-2025-2027-repair/INDEX.md`.

**Zone.** The service prints a bare wall clock with no zone token. The zone comes from
CME's own sentence on the page that calls it, quoted verbatim: "Trading hours are subject to
change and are in U.S. Central Time unless otherwise stated." Every `Instant as printed`
cell below is the service's own text; a `[CT]` reading is this crate's editorial addition and
is never inserted into a quotation.

**What the audit is.** CME's Globex holiday tables for 2025, 2026 and 2027 on
`cmegroup.com/trading-hours.html` (T1; `holidays/raw/cme-2025-2027/html/trading-hours-20250830.html`,
sha256 `62fc524c79e0bd1269ad89b5fe1ae9b617c1b082243d4f5386c5d3fdf6ed5e58`, captured
2025-08-30T02:14:20Z, and `live/trading-hours-live.md`, sha256
`ac85d05d1fcf2c6fc5afaad7bef5efa7bed407d53df7bdfc9bc724ae18c3449f`, retrieved 2026-09-12)
enumerate the holidays for each year; the service supplies each one's instants. A trade date
inside the coverage window that appears on neither list is audited normal and ships no row.

**How an event-date record becomes a trade-date row (design memo D1, D5-D8).** The sourced
grid wraps: an evening leg opens 19:00 CT and runs to 07:45 CT, then an 08:30-13:20 CT day
session closes the trading day, so trade date `D` opens on `D - 1`. Four conversions follow,
and each row's `Derived from` cell records which one produced it.

1. **A full closure.** CME prints either nothing for the date, or only a `19:00 open` (or a
   `16:00 preopen`) whose own printed trade date is the *next* business day. Either way the
   date has no final close and therefore no trade date of its own, so the row is `closed`.
   One row then deletes the prior evening's leg as well, because that leg's derived trade
   date is the closed one (memo D6) — which is exactly what CME's withheld
   `16:45 preopen`/`19:00 open` pair on the eve is describing.
2. **An eve with the whole day session and no evening leg.** `07:45 paused; 08:00 preopen;
   08:30 open; 13:20 paused; 13:30 closed; 14:30 pcp; 16:00 closed` is the ordinary CBOT
   grain day. What is missing is the evening leg, and the neighbouring `closed` row already
   removes it. Fourteen such dates in this block ship a `replacement blocks` row keyed to
   the eve's own trade date, because the `14:30 pcp`/`16:00 closed` pair the operator prints
   there carries **that trade date**: 2025-04-17, 2025-06-18, 2025-07-03, 2025-11-26,
   2025-12-31, 2026-04-02, 2026-06-18, 2026-07-02, 2026-11-25, 2026-12-31, 2027-03-25,
   2027-06-17, 2027-11-24 and 2027-12-23. The row states the ordinary prior-evening queue
   and leg as well, so the eve's complete day is restated rather than a fragment of it; the
   `Closed` row on the following date still removes that date's own prior-evening leg.
3. **A day after a closure.** CME prints a `06:00 preopen` in place of the usual
   `07:45 paused; 08:00 preopen` pair, so the prior-evening leg did not run. `08:30` is
   numerically earlier than the trading day's normal 19:00 CT first open, so the cutoff lands
   on the trade date itself and not on the preceding local date (memo D7). Four such dates —
   2025-01-02, 2025-12-26, 2026-01-02 and 2027-07-06 — also ship a `replacement blocks` row,
   because a scalar `late open` cannot state the `06:00-08:30` order-entry window the
   operator publishes beside the open.
4. **The day after Thanksgiving.** No prior-evening leg *and* a 12:05 CT final close, so both
   boundaries move: `late open and early close`.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `2025-01-01: no events published` | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01 -> TD none printed |
| 2025-01-02 | replacement blocks | `06:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-01-02, all carrying trade date 2025-01-02 | `CME-SVC-2024-12-31` | T2 | no prior-evening leg (eventDate 2025-01-01 is empty), so the day is the operator's `06:00` queue, the ordinary `08:30-13:20` CT day session and the ordinary post-close queue; the operator also prints `13:30 closed` beside the `13:20 paused`, its probe artefact |
| 2025-01-20 | closed | `2025-01-19: 16:00 preopen / 2025-01-20: 19:00 open` | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-19 -> TD 2025-01-21; eventDate 2025-01-20 -> TD 2025-01-21 |
| 2025-02-17 | closed | `2025-02-16: 16:00 preopen / 2025-02-17: 19:00 open` | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-16 -> TD 2025-02-18; eventDate 2025-02-17 -> TD 2025-02-18 |
| 2025-04-17 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-04-17, all carrying trade date 2025-04-17 | `CME-SVC-2025-04-17` | T2 | Good Friday eve: the `14:30 pcp`/`16:00 closed` pair carries this trade date, so the whole day ships and the queue survives the next date's closure; the prior-evening queue and leg are the family's ordinary week; the operator also prints `13:30 closed` beside the `13:20 paused`, its probe artefact |
| 2025-04-18 | closed | `2025-04-18: no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18 -> TD none printed |
| 2025-05-26 | closed | `2025-05-25: 16:00 preopen / 2025-05-26: 19:00 open` | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-25 -> TD 2025-05-27; eventDate 2025-05-26 -> TD 2025-05-27 |
| 2025-06-18 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-06-18, all carrying trade date 2025-06-18 | `CME-SVC-2025-06-18` | T2 | Juneteenth eve, as 2025-04-17; the operator additionally prints a `16:45 preopen` carrying trade date 2025-06-20 on this eventDate, which this row does not state (the deviation recorded under Gaps) |
| 2025-06-19 | closed | `2025-06-19: 19:00 open` | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19 -> TD 2025-06-20 |
| 2025-07-03 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-07-03, all carrying trade date 2025-07-03 | `CME-SVC-2025-07-03` | T2 | Independence Day eve, as 2025-04-17 |
| 2025-07-04 | closed | `2025-07-04: no events published` | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04 -> TD none printed |
| 2025-09-01 | closed | `2025-08-31: 16:00 preopen / 2025-09-01: 19:00 open` | `CME-SVC-2025-08-31` | T2 | eventDate 2025-08-31 -> TD 2025-09-02; eventDate 2025-09-01 -> TD 2025-09-02 |
| 2025-11-26 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-11-26, all carrying trade date 2025-11-26 | `CME-SVC-2025-11-26-SAT` | T2 | Thanksgiving eve, as 2025-04-17; the operator additionally prints a `16:45 preopen` carrying trade date 2025-11-28 on this eventDate, which this row does not state (the deviation recorded under Gaps) |
| 2025-11-27 | closed | `2025-11-27: no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-27 -> TD none printed |
| 2025-11-28 | late open and early close | `2025-11-28: 07:00 preopen; 08:30 open; 12:05 closed` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-28 -> TD 2025-11-28 |
| 2025-11-29 | closed | `2025-11-29: no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29 -> TD none printed |
| 2025-12-24 | early close | `2025-12-24: 07:45 paused; 08:00 preopen; 08:30 open; 12:05 closed` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24 -> TD 2025-12-24 |
| 2025-12-25 | closed | `2025-12-25: no events published` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25 -> TD none printed |
| 2025-12-26 | replacement blocks | `06:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-12-26, all carrying trade date 2025-12-26 | `CME-SVC-2025-12-24` | T2 | no prior-evening leg (eventDate 2025-12-25 is empty), as 2025-01-02 |
| 2025-12-31 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2025-12-31, all carrying trade date 2025-12-31 | `CME-SVC-2025-12-31` | T2 | New Year's Day eve, as 2025-04-17 |

**Interpretive steps, 2025.**

- **Thanksgiving 2025 is sourced from a post-holiday publication.** `CME-SVC-2025-11-26-SAT`
  is the repair round's live retrieval, which covers the Saturday the archived windows stop
  short of. Two archived captures corroborate its `ZC` rows instant for instant:
  `cme-2025-2027-fix/arc/thbp_2025-11-26_2025-11-28_20260129012309.json`
  (sha256 `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1`, captured
  2026-01-29T01:23:09Z, two months after the holiday) and the superseded pre-holiday
  `cme-2025-2027/arc/thbp_2025-11-26_2025-11-28_20241220155340.json`
  (sha256 `34f38de416a997f7a1a095ce33261327c0b604abecdbe221a35305005ffa2764`). The finalised
  publications add a `07:00 preopen` on 2025-11-28 that the pre-holiday one does not print;
  the `08:30 open` and `12:05 closed` are identical in all three.
- **Christmas 2025** is additionally corroborated by
  `cme-2025-2027-fix/arc/thbp_2025-12-24_2025-12-26_20260129012309.json`, which is
  byte-identical to the cited capture (same sha256).
- **Saturday 2025-11-29 ships a row although it changes no answer.** CME's own 2025 Globex
  table states the Thanksgiving period as "27 - 29 November 2025", and the round-2 verdict's
  material finding 3 required the Saturday be answered rather than left a gap. The repair
  round answered it: the service returns all ten products with empty schedules. The normal
  week has no Saturday grain session, so the row records a sourced closure and moves nothing.
- **`ZS` and `ZW` corroboration.** The family covers standard-size corn, soybeans and wheat.
  The `[THBP-B]` windows in `cme-2025-2027-repair/INDEX.md` (`D54`, `D55`, `D56`) carry `ZS`
  and `ZW` for Thanksgiving 2025, Christmas 2025 and New Year 2026 and agree with the `ZC`
  rows date for date and instant for instant.

**Gaps, 2025.**

- **No T1 rendering of the instants.** CME publishes no per-asset-class T1 rendering of these
  holiday hours; the T1 Globex table names the holidays and the periods but states no
  per-family instants. The rows are therefore T2 throughout. Closing condition: a CME notice,
  circular or PDF that prints the grain complex's holiday instants for these dates.
- **2025-01-02 00:00-05:59:59 CT still refuses below the floor.** `session_state` and
  `is_maintenance` walk back to the session that opened on 2024-12-31 for the gap that runs into
  this trade date, so those instants answer `Err(BeforeSupportFloor { date: 2024-12-31 })` while
  `trade_date` answers `Ok(None)` and `is_open` `Ok(false)`. That is the support-floor boundary
  (#115), not this row: before it the row states nothing and the refusal is unchanged. The row
  **does** change the printed window's own instants — 06:00:00 through 08:29:59 CT answered
  `Err(BeforeSupportFloor { date: 2024-12-31 })` before it and answer `Ok(OrderEntry)` with
  `trade_date = 2025-01-02` after it, which is the repair. Closing condition for the remainder: the
  boundary contract #115 lands.
- **Eight 2025 windows survive only in a pre-holiday publication.** New Year 2025 through
  Labor Day 2025 are sourced from the single Wayback capture 2024-12-20T15:53:40Z, and CME
  states on the same page that "Trading hours are usually finalized approximately two weeks
  prior to the holiday". No later capture of those windows exists in the archive and the
  service's retention edge falls after Labor Day 2025, so they are CME's published future as
  of 2024-12-20 rather than a post-finalisation statement. Residual risk, recorded; the
  Thanksgiving 2025 comparison above is the only case in this block where a finalised
  publication could be compared against a pre-holiday one, and it moved no instant.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `2026-01-01: no events published` | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01 -> TD none printed |
| 2026-01-02 | replacement blocks | `06:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2026-01-02, all carrying trade date 2026-01-02 | `CME-SVC-2025-12-31` | T2 | no prior-evening leg (eventDate 2026-01-01 is empty), as 2025-01-02 |
| 2026-01-19 | closed | `2026-01-18: 16:00 preopen / 2026-01-19: 19:00 open` | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-18 -> TD 2026-01-20; eventDate 2026-01-19 -> TD 2026-01-20 |
| 2026-02-16 | closed | `2026-02-15: 16:00 preopen / 2026-02-16: 19:00 open` | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-15 -> TD 2026-02-17; eventDate 2026-02-16 -> TD 2026-02-17 |
| 2026-04-02 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2026-04-02, all carrying trade date 2026-04-02 | `CME-SVC-2026-04-01` | T2 | Good Friday eve, as 2025-04-17 |
| 2026-04-03 | closed | `2026-04-03: no events published` | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03 -> TD none printed |
| 2026-05-25 | closed | `2026-05-24: 16:00 preopen / 2026-05-25: 19:00 open` | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-24 -> TD 2026-05-26; eventDate 2026-05-25 -> TD 2026-05-26 |
| 2026-06-18 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2026-06-18, all carrying trade date 2026-06-18 | `CME-SVC-2026-06-18` | T2 | Juneteenth eve, as 2025-04-17 |
| 2026-06-19 | closed | `2026-06-19: no events published` | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19 -> TD none printed |
| 2026-07-02 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2026-07-02, all carrying trade date 2026-07-02 | `CME-SVC-2026-07-02` | T2 | Independence Day eve, as 2025-04-17; the id is minted for the 2026-01-29 capture, whose window starts on this eventDate |
| 2026-07-03 | closed | `2026-07-03: no events published` | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03 -> TD none printed |
| 2026-09-07 | closed | `2026-09-06: 16:00 preopen / 2026-09-07: 19:00 open` | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-06 -> TD 2026-09-08; eventDate 2026-09-07 -> TD 2026-09-08 |
| 2026-11-25 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2026-11-25, all carrying trade date 2026-11-25 | `CME-SVC-2026-11-25` | T2 | Thanksgiving eve, as 2025-04-17; the operator additionally prints a `16:45 preopen` carrying trade date 2026-11-27 on this eventDate, which this row does not state (the deviation recorded under Gaps) |
| 2026-11-26 | closed | `2026-11-26: no events published` | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26 -> TD none printed |
| 2026-11-27 | late open and early close | `2026-11-27: 08:30 open; 12:05 closed` | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27 -> TD 2026-11-27 |
| 2026-12-24 | early close | `2026-12-24: 07:45 paused; 08:00 preopen; 08:30 open; 12:05 closed` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-24 -> TD 2026-12-24 |
| 2026-12-25 | closed | `2026-12-25: no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25 -> TD none printed |
| 2026-12-31 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2026-12-31, all carrying trade date 2026-12-31 | `CME-SVC-2026-12-31` | T2 | New Year's Day eve, as 2025-04-17 |

**Artifacts, 2026.** Files are relative to `exchange-hours-research/holidays/raw/`.

**Interpretive steps, 2026.**

- **Live retrieval and archived capture agree byte for byte.** The three live artifacts above
  hash identically to their archived counterparts — `arc/thbp_2026-09-06_2026-09-08_20260830142930.json`,
  `arc/thbp_2026-11-25_2026-11-27_20260830142904.json` and
  `arc/thbp_2026-12-24_2026-12-26_20260830142904.json`, all captured 2026-08-30 — so each row
  rests on two vintages of the same channel.
- **Two windows cover Juneteenth and Independence Day 2026.** `arc/thbp_2026-06-17_2026-06-19_20260129012310.json`
  and `arc/thbp_2026-07-02_2026-07-04_20260129012216.json` (both captured 2026-01-29) print the
  same `ZC` schedules as the cited 2026-06-19 captures. For the two **eve** trade dates the
  later capture is at the row: 2026-06-18 cites `CME-SVC-2026-06-18` and 2026-07-02 cites
  `CME-SVC-2026-07-02`, the 2026-01-29 window whose own first `eventDate` is that date and
  which no earlier id covered. For the closure dates the later capture governs and the earlier
  one corroborates, so `CME-SVC-2026-07-03` stays the id beside 2026-07-03.
- **Christmas Eve 2026 has a third corroborating window.**
  `arc/thbp_2026-12-23_2026-12-25_20260310063244.json` (2026-03-10T06:32:44Z, sha256
  `0cc5fa9d78ba2d6d1246faab67acfc1388b52f16a7f2c32b2dddec07238e87c2`) and
  `live/thbp/thbp_2026-12-22_2026-12-24.json` (sha256
  `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab`) both print
  `07:45 paused; 08:00 preopen; 08:30 open; 12:05 closed` for 2026-12-24.
- **`ZS` and `ZW` corroboration.** The `[THBP-B]` windows `D57`-`D64` in
  `cme-2025-2027-repair/INDEX.md` and the `live/extra/extra_2026-*.json` files in
  `cme-2025-2027/INDEX.md` carry soybeans and wheat for every 2026 holiday window and agree
  with the `ZC` rows date for date and instant for instant.

**Gaps, 2026.**

- **No T1 rendering of the instants**, as for 2025. One T1 operator page corroborates the
  Good Friday 2026 closure but states no instants.
- **Saturdays inside a published window ship no row.** 2026-06-20, 2026-07-04 and 2026-12-26
  are returned with empty schedules. The normal week has no Saturday grain session, so unlike
  2025-11-29 — which CME names as part of its Thanksgiving period — these carry no operator
  statement that a period extends over them and record nothing.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `2027-01-01: no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01 -> TD none printed |
| 2027-01-18 | closed | `2027-01-17: 16:00 preopen / 2027-01-18: 19:00 open` | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-17 -> TD 2027-01-19; eventDate 2027-01-18 -> TD 2027-01-19 |
| 2027-02-15 | closed | `2027-02-14: 16:00 preopen / 2027-02-15: 19:00 open` | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-14 -> TD 2027-02-16; eventDate 2027-02-15 -> TD 2027-02-16 |
| 2027-03-25 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2027-03-25, all carrying trade date 2027-03-25 | `CME-SVC-2027-03-25` | T2 | Good Friday eve, as 2025-04-17 |
| 2027-03-26 | closed | `2027-03-26: no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26 -> TD none printed |
| 2027-05-31 | closed | `2027-05-30: 16:00 preopen / 2027-05-31: 19:00 open` | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-30 -> TD 2027-06-01; eventDate 2027-05-31 -> TD 2027-06-01 |
| 2027-06-17 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2027-06-17, all carrying trade date 2027-06-17 | `CME-SVC-2027-06-17` | T2 | Juneteenth eve, as 2025-04-17 |
| 2027-06-18 | closed | `2027-06-18: no events published` | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18 -> TD none printed |
| 2027-07-05 | closed | `2027-07-04: no events published / 2027-07-05: no events published` | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-04 -> TD none printed; eventDate 2027-07-05 -> TD none printed |
| 2027-07-06 | replacement blocks | `06:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2027-07-06, all carrying trade date 2027-07-06 | `CME-SVC-2027-07-04` | T2 | no prior-evening leg (eventDate 2027-07-05 is empty), as 2025-01-02 |
| 2027-09-06 | closed | `2027-09-05: 16:00 preopen / 2027-09-06: 19:00 open` | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-05 -> TD 2027-09-07; eventDate 2027-09-06 -> TD 2027-09-07 |
| 2027-11-24 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2027-11-24, all carrying trade date 2027-11-24 | `CME-SVC-2027-11-24` | T2 | Thanksgiving eve, as 2025-04-17; the operator additionally prints a `16:45 preopen` carrying trade date 2027-11-26 on this eventDate, which this row does not state (the deviation recorded under Gaps) |
| 2027-11-25 | closed | `2027-11-25: no events published` | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25 -> TD none printed |
| 2027-11-26 | late open and early close | `2027-11-26: 08:30 open; 12:05 closed` | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26 -> TD 2027-11-26 |
| 2027-12-23 | replacement blocks | `07:45 paused; 08:00 preopen; 08:30 open; 13:20 paused; 14:30 pcp; 16:00 closed` on eventDate 2027-12-23, all carrying trade date 2027-12-23 | `CME-SVC-2027-12-22` | T2 | Christmas Friday eve, as 2025-04-17 |
| 2027-12-24 | closed | `2027-12-24: no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-24 -> TD none printed |

**Artifacts, 2027.** Files are relative to `exchange-hours-research/holidays/raw/`, all in
`cme-2025-2027/live/thbp/` and all retrieved live on 2026-09-12 (UTC).

**Interpretive steps, 2027.**

- **This is the operator's published future.** LAW-NO-FABRICATED-DATES permits encoding an
  unconditional, fully sourced future date ahead of its effective day, and these are CME's own
  unconditional publications rather than a preliminary or conditional calendar. The review
  date is the UTC date the evidence was checked, 2026-09-12, and LAW-WATCH's monthly cadence
  applies because CME finalises holiday hours roughly two weeks before each holiday.
- **Independence Day 2027 falls on a Sunday and is observed on Monday 2027-07-05.** CME prints
  no events for either 2027-07-04 or 2027-07-05. The Sunday needs no row: no occurrence is
  ever assigned to a Sunday trade date on this grid, and the Sunday-evening leg that would
  have opened trade date Monday is deleted by the Monday's own `closed` row.
- **Christmas 2027 falls on a Saturday.** CME's holiday date is Thursday 2027-12-23, but the
  Globex closure is Friday 2027-12-24, which is where the row sits. 2027-12-23 prints the
  whole day session and no evening leg, so it is the eve conversion 2 above states and ships
  the family's full `replacement blocks` day; its `14:30 pcp` carries trade date 2027-12-23.
- **`ZS` and `ZW` corroboration.** The `live/extra/extra_2027-*.json` files in
  `cme-2025-2027/INDEX.md` carry soybeans and wheat for all ten 2027 windows and agree with
  the `ZC` rows date for date and instant for instant.

**Gaps, 2027.**

- **No T1 rendering of the instants**, as for 2025 and 2026.
- **2027-12-26 .. 2027-12-29 are not covered by a service window.** No retrieved window spans
  them. They are audited normal on the T1 holiday list alone — CME's 2027 Globex table names
  no holiday between 2027-12-24 and 2028-01-01 — and no instant for them is asserted from the
  service. Closing condition: a service window covering 2027-12-26 .. 2027-12-29.

### Gaps across the block

- **The post-close queue's trade-date label is the crate's convention, not the operator's
  printing (#152).** CME's own service prints a trade date on every `14:30 pcp` event, and on
  an ordinary date that is the date the queue is printed on. The crate dates an order-entry
  occurrence by the **session it feeds**, so the same queue reads with the next trade date
  instead: `D + 1` on a Monday to Thursday and `D + 3` over a weekend. Measured through
  `calendar_for_market_hours_key(MarketHoursKey::GlobexGrains)` over
  2025-01-01..2027-12-31, 15:00 CT is inside the queue on **746** trade dates, and **all 746**
  answer `session_state = OrderEntry`, `is_accepting_orders = true` and a `trade_date` other
  than the date printed on. Two instants show it on a date no row covers: **2025-06-10** (a
  Tuesday) answers `trade_date = 2025-06-11`, and **2025-06-13** (a Friday) answers
  `trade_date = 2025-06-16`, where `CME-SVC-2024-12-31` prints `2025-01-02 14:30 pcp` and
  `2026-07-02 14:30 pcp` carrying their own dates. This is the one answer in this block that a
  correction to `trade_date` moves, and the earlier blanket claim that no order-entry deviation
  moves one was false for it. The divergence is **deliberate**: CME's own T1 description of the
  Post-Close — "GTC and GTD orders may be entered, modified and cancelled 1:45.30 - 4:00 p.m.
  CT / The markets will become unavailable at 4:00 p.m. CT" — describes orders that persist
  into the next session, which is the session the crate assigns them to. It is **not fixable by
  any data row**: three candidate row shapes were measured and all were rejected — one leaves
  the label unchanged, and the only shape that yields the operator's own label is `tradeable`,
  which would assert matching in a window the operator marks `pcp` (LAW-SESSION-NOT-EXPIRY).
  The scope therefore declares the gap (`CoverageGapReason::PostCloseQueueTradeDateLabel`,
  whole-domain, closing condition #152 in `schedules/sourcing.rs`) and neither
  `globex_grains` nor `globex_livestock`, which carries the same queue, claims a complete
  calendar. The declaration withholds no answer and refuses no query: the window and both of
  its verdicts are served. Served identity, so tracked as issue #152
  (LAW-FOLLOW-UPS-ARE-ISSUES).
- **Order-entry-only deviations the replacement rows do not state.** Five classes of deviation
  in this block are recorded and not modelled as rows; all of them move `is_accepting_orders`
  or `is_order_entry_only` alone, and none moves `is_open`, `session_bounds`, `candle_end`, or
  — apart from the label deviation above — `trade_date`.
  1. On 2025-11-28 CME prints a `07:00 preopen`, and on 2026-11-27 and 2027-11-26 it prints
     no pre-open at all; the crate keeps the normal 08:00-08:30 CT queue.
  2. On the twelve Sundays before a Monday holiday CME prints a `16:00 preopen` carrying the
     *Tuesday* trade date and no `19:00 open`; the crate keeps its normal Sunday 16:00-19:00 CT
     queue.
  3. On 2025-06-18, 2025-11-26, 2026-11-25 and 2027-11-24 — four of the fourteen eves the new
     replacement rows state — CME prints a `16:45 preopen` carrying the **post-holiday** trade
     date on the eve's eventDate and no `19:00 open`. The row keyed to the eve's own trade date
     cannot state it: the occurrence it names belongs to the post-holiday trade date, and
     stating it on the eve would assign the eve's date to an event the operator dates two days
     later. Stating it on the post-holiday date would need that date's own replacement row,
     which is a separate arrangement. Measured through the built-in calendar, the crate answers
     `Closed` at 16:45 and 17:30 CT on those four eves — the neighbouring `closed` row deletes
     the queue with the trade date it feeds — so the window the operator publishes is absent
     rather than merely mislabelled.
  4. The `14:30 pcp` window on the four `replacement blocks` late-open dates is stated by the
     row, but its trade-date **label** is not fixed by it: the row's blocks carry the trade
     date, and `trade_date` at 15:00 CT still resolves through the session the queue feeds, as
     on every other date.

  Closing condition for the first two: an order-entry boundary the scalar vocabulary cannot
  state, which the replacement-block rows now supply where a whole day is published. Closing
  condition for the third and fourth: #152, as for the label deviation above.
- **No intraday-topology gap.** All eighteen `modified` grain rows in the block triage to
  design memo §1.6 categories 3, 4 and 5 — fourteen to conversion 2 and four to conversion 3 —
  and each of the eighteen now ships a replacement-block row stating its complete day. None is
  a category-6 topology change, so this family contributes no unrepresentable special day in
  this window.
- **No `unsourced` row.** Every trade date this table names has an operator artifact behind it
  at T2, and every other date in the coverage window is audited normal on CME's own T1 holiday
  list.
- **Verdict reconciliation.** The governing verdict for this block is
  `holidays/cme-2025-2027.verify.json` (round 2, `matches: false`, four material findings).
  Only one of the four touches this family — finding 3, the unchecked Saturday 2025-11-29 —
  and the repair round closed it from the operator's own channel; the row above is the result.
  Findings 1, 2 and 4 concern Nikkei, Cryptocurrency and the `[THBP-B]` product set and move
  no grain row. No disagreement between the repaired result and the verdict remains for
  `globex_grains`.
