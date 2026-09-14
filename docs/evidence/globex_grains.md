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

Every id below resolves to one saved artifact behind this file's holiday rows.
The 2010-2012 ids are CME Group's own holiday-calendar PDFs at tier T1, retrieved
through the Internet Archive and saved; the 2025-2027 ids are responses of CME's
own `trading-hours-by-product` service at tier T2. Each id's row resolves it to the replay URL, the capture time in UTC,
the tier and the sha256, as the design memo's section 3.2 requires; the byte counts and
the operator's original URLs behind those replays are in the research store's
`holidays/raw/` indexes.

| Document | Window | Internet Archive raw replay | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2010-new-years.pdf @2010-02-15T05:16:52Z` | `2010-new-years.pdf` | <https://web.archive.org/web/20100215051652id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-new-years.pdf> | 2010-02-15T05:16:52Z | T1 | `c30a6cef73fca23c54b25907f307ad52a2922d1e4b76c0a12de126dc6fc31a6d` |
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | <https://web.archive.org/web/20100601111916id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-good-friday.pdf> | 2010-06-01T11:19:16Z | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | <https://web.archive.org/web/20101122094012id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-thanksgiving.pdf> | 2010-11-22T09:40:12Z | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2010-christmas.pdf @2010-12-14T06:12:38Z` | `2010-christmas.pdf` | <https://web.archive.org/web/20101214061238id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-christmas.pdf> | 2010-12-14T06:12:38Z | T1 | `d4adb060f6eb592fb24e3a272db57b3d8c9d69f3f370682ecd8d57e4169c37be` |
| `2011-new-years.pdf @2011-11-01T14:39:45Z` | `2011-new-years.pdf` | <https://web.archive.org/web/20111101143945id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-new-years.pdf> | 2011-11-01T14:39:45Z | T1 | `42c289804cd3fa0830556ecb7fcc31493c452ba9e7325ebe7d7ce29613e41476` |
| `2011-good-friday.pdf @2011-10-28T02:37:07Z` | `2011-good-friday.pdf` | <https://web.archive.org/web/20111028023707id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-good-friday.pdf> | 2011-10-28T02:37:07Z | T1 | `6cf10359bb438eb49287dcef7c1e75a484df4d6e3b538fa9ee59dc3832210bda` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | <https://web.archive.org/web/20111124185246id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf> | 2011-11-24T18:52:46Z | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | <https://web.archive.org/web/20120125020548id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf> | 2012-01-25T02:05:48Z | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | <https://web.archive.org/web/20120125025430id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf> | 2012-01-25T02:54:30Z | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-good-friday.pdf @2012-04-17T00:42:47Z` | `2012-good-friday.pdf` | <https://web.archive.org/web/20120417004247id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-good-friday.pdf> | 2012-04-17T00:42:47Z | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | <https://web.archive.org/web/20120915003714id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf> | 2012-09-15T00:37:14Z | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | <https://web.archive.org/web/20120915003923id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf> | 2012-09-15T00:39:23Z | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | <https://web.archive.org/web/20120915003437id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf> | 2012-09-15T00:34:37Z | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | <https://web.archive.org/web/20130127223901id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-thanksgiving.pdf> | 2013-01-27T22:39:01Z | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | <https://web.archive.org/web/20130414194027id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-christmas.pdf> | 2013-04-14T19:40:27Z | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
| `2016-new-years-holiday-schedule.pdf @2016-01-08` | `2016-new-years-holiday-schedule.pdf` | <https://web.archive.org/web/20160108id_/https://web.archive.org/web/20160108203007id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2016-new-years-holiday-schedule.pdf> | 2014-12-12T03:40:39Z | T1 | `7d032ee5457cc5c06acb512e54006fa0678761e12d2b60bf625d924741896755` |
| `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | `2016-martin-luther-king-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-martin-luther-king-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `0fac2d08a84f8e9a637573438adfab33ba91c69e76b6dce01f0ca4cbcde8aff1` |
| `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | `2016-presidents-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-presidents-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `b5264dd479efd69df495de170cdd58326f64e9e31a2ed7cca6d7d96cc424c9c2` |
| `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | `2016-good-friday-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-good-friday-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `632c4e3e451fd972e4cccb743f7dfa7a41896044cafc0e82953e0c6fb6d0bc42` |
| `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | `2016-memorial-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-memorial-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `03fa4af1988ae7a54c6e5426b83a76e54825ac4398f7a2be6e77be82fe8ab9f9` |
| `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | `2016-4th-of-july-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-4th-of-july-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `533a49be68619dbd003b3e749485be46051018d3613d44a3043bf8a087fec9ad` |
| `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | `2016-labor-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-labor-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `a4c0676ad117a63894b84637e8e5a7734ed17d7e2d1ee09d5a96370c7fd53b4a` |
| `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | `2016-thanksgiving-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-thanksgiving-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `b3c4dbe2d60bf5530316653d381ba28f1eb4ed6ec55844dca976b1852941c2b1` |
| `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | `2016-christmas-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-christmas-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `42efeb845763d3d8a288dd9f9349bfcaf88badebf49ee1c6bc645a72ec01d252` |
| `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | `2017-new-years-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-new-years-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `493e9cd3fb8ae7ce8b059f30516dedd83631a101e63ecd4c9201fa24c8a2aae8` |
| `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | `2017-martin-luther-king-holiday-schedule.xls` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-martin-luther-king-holiday-schedule.xls> | 2017-06-28T17:25:56Z | T1 | `c11937ee9995dd712f23631dd235571d87d2c387bc21e074cc87905b68980d58` |
| `2017-presidents-day-holiday-schedule.xls @2017-06-28` | `2017-presidents-day-holiday-schedule.xls` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-presidents-day-holiday-schedule.xls> | 2017-06-28T17:41:48Z | T1 | `2ec7b623faa8f17ab5946091492be0dacbd29e6446ab3789e0ba4fa5ccd58b54` |
| `2017-good-friday-holiday-schedule.xls @2017-05-05` | `2017-good-friday-holiday-schedule.xls` | <https://web.archive.org/web/20170505id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-good-friday-holiday-schedule.xls> | 2017-05-05T07:23:15Z | T1 | `bdc8775f7061ae1e377421fe7a951446c0066ba11d1dbfc7bebba9e7c9415ccb` |
| `2017-memorial-day-holiday-schedule.xls @2017-10-25` | `2017-memorial-day-holiday-schedule.xls` | <https://web.archive.org/web/20171025id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-memorial-day-holiday-schedule.xls> | 2017-05-05T07:24:06Z | T1 | `2d5a64e7ba05c1889993e91cde352312a8fb5901c5f6a4d53439002ab067542a` |
| `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | `2017-4th-of-july-holiday-schedule.xls` | <https://web.archive.org/web/20171025id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-4th-of-july-holiday-schedule.xls> | 2017-05-05T07:22:05Z | T1 | `d8d668183f686591879e8ee0d2db13c650c09d5a3c13e2378be068c784679453` |
| `2017-labor-day-holiday-schedule.xls @2017-10-25` | `2017-labor-day-holiday-schedule.xls` | <https://web.archive.org/web/20171025id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-labor-day-holiday-schedule.xls> | 2017-05-05T07:23:46Z | T1 | `660473ca09893868560b64e544c31c984a715595b1311f136af389951ef56f63` |
| `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | `2017-thanksgiving-holiday-schedule.xls` | <https://web.archive.org/web/20210126id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-thanksgiving-holiday-schedule.xls> | 2021-01-26T09:48:35Z | T1 | `b6bc3dba9f1e0c5a86d543e389d15cea6006f456e6dc2cfd7813eae31db2f869` |
| `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | `2017-christmas-holiday-schedule.xls` | <https://web.archive.org/web/20210126id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-christmas-holiday-schedule.xls> | 2021-01-26T09:48:35Z | T1 | `bfb291c9c08764ee2bcf0327690b98c58cbb4cf493723c7b1464eccb0d062d30` |
| `2018-new-years-holiday-schedule.xls @2018-01-06` | `2018-new-years-holiday-schedule.xls` | <https://web.archive.org/web/20180106id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-new-years-holiday-schedule.xls> | 2017-05-05T07:26:53Z | T1 | `73332758cbf95363a04cf422a672157c1b1a5bb34d4c9bd496bd7b7491a4d8cd` |
| `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | `2018-martin-luther-king-holiday-schedule.xls` | <https://web.archive.org/web/20180508id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-martin-luther-king-holiday-schedule.xls> | 2018-01-06T22:56:50Z | T1 | `81f30b0be680898b82175b19ee6a2fccc85b7e340fac2418b0ad00973c4a93a8` |
| `2018-presidents-day-holiday-schedule.xls @2018-05-08` | `2018-presidents-day-holiday-schedule.xls` | <https://web.archive.org/web/20180508id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-presidents-day-holiday-schedule.xls> | 2018-01-06T22:57:32Z | T1 | `fddc713823c58cfdc4de2486ea3b2953ecda24c01579da64eb3933d8a7efc4b8` |
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

**Coverage:** 2010-01-01..2012-12-31, 2016-01-01..2018-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).
Tier: **T1** for 2010-2012 and 2016-2018, from the operator's own published
holiday schedules; **T2** for 2025-2027, from its trading-hours service. Inside a
window a date with no row is audited normal; outside every window this table has no
answer at all.

**Three audited eras, and two gaps between them.** The table declares three
coverage windows: `2010-01-01..2012-12-31` and `2016-01-01..2018-12-31`, from the
operators' own published holiday schedules at **T1**, and
`2025-01-01..2027-12-31`, from the trading-hours service at **T2**. The
2013-2015 and 2019-2024 intervals are audited by neither — those years are the
remaining stage-2.2 waves — so they lie outside every declared window and
`holiday_on` has **no answer** there rather than reporting an unaudited date as
normal. `HolidayCoverage::windows()` lists the three, and `contains` answers per
date.

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

### 2025-2027 (T2)

**Rows:** 40 — 31 `closed`, 4 `late open`, 2 `early close`, 3 `late open and early close`, 0 `unsourced`.

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
   removes it. Fourteen such dates in this block ship **no row**: 2025-04-17, 2025-06-18,
   2025-07-03, 2025-11-26, 2025-12-31, 2026-04-02, 2026-06-18, 2026-07-02, 2026-11-25,
   2026-12-31, 2027-03-25, 2027-06-17, 2027-11-24 and 2027-12-23.
3. **A day after a closure.** CME prints a `06:00 preopen` in place of the usual
   `07:45 paused; 08:00 preopen` pair, so the prior-evening leg did not run and matching
   still begins 08:30 CT. That is a `late open` at 08:30 CT stated on the trade date. 08:30 is
   numerically earlier than the trading day's normal 19:00 CT first open, so the cutoff lands
   on the trade date itself and not on the preceding local date (memo D7).
4. **The day after Thanksgiving.** No prior-evening leg *and* a 12:05 CT final close, so both
   boundaries move: `late open and early close`.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `2025-01-01: no events published` | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01 -> TD none printed |
| 2025-01-02 | late open | `2025-01-02: 06:00 preopen; 08:30 open; 13:20 paused; 13:30 closed; 14:30 pcp; 16:00 closed; 16:45 preopen; 19:00 open` | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-02 -> TD 2025-01-02, 2025-01-03 |
| 2025-01-20 | closed | `2025-01-19: 16:00 preopen / 2025-01-20: 19:00 open` | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-19 -> TD 2025-01-21; eventDate 2025-01-20 -> TD 2025-01-21 |
| 2025-02-17 | closed | `2025-02-16: 16:00 preopen / 2025-02-17: 19:00 open` | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-16 -> TD 2025-02-18; eventDate 2025-02-17 -> TD 2025-02-18 |
| 2025-04-18 | closed | `2025-04-18: no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18 -> TD none printed |
| 2025-05-26 | closed | `2025-05-25: 16:00 preopen / 2025-05-26: 19:00 open` | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-25 -> TD 2025-05-27; eventDate 2025-05-26 -> TD 2025-05-27 |
| 2025-06-19 | closed | `2025-06-19: 19:00 open` | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19 -> TD 2025-06-20 |
| 2025-07-04 | closed | `2025-07-04: no events published` | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04 -> TD none printed |
| 2025-09-01 | closed | `2025-08-31: 16:00 preopen / 2025-09-01: 19:00 open` | `CME-SVC-2025-08-31` | T2 | eventDate 2025-08-31 -> TD 2025-09-02; eventDate 2025-09-01 -> TD 2025-09-02 |
| 2025-11-27 | closed | `2025-11-27: no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-27 -> TD none printed |
| 2025-11-28 | late open and early close | `2025-11-28: 07:00 preopen; 08:30 open; 12:05 closed` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-28 -> TD 2025-11-28 |
| 2025-11-29 | closed | `2025-11-29: no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29 -> TD none printed |
| 2025-12-24 | early close | `2025-12-24: 07:45 paused; 08:00 preopen; 08:30 open; 12:05 closed` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24 -> TD 2025-12-24 |
| 2025-12-25 | closed | `2025-12-25: no events published` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25 -> TD none printed |
| 2025-12-26 | late open | `2025-12-26: 06:00 preopen; 08:30 open; 13:20 paused; 13:30 closed; 14:30 pcp; 16:00 closed` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-26 -> TD 2025-12-26 |

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
| 2026-01-02 | late open | `2026-01-02: 06:00 preopen; 08:30 open; 13:20 paused; 13:30 closed; 14:30 pcp; 16:00 closed` | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-02 -> TD 2026-01-02 |
| 2026-01-19 | closed | `2026-01-18: 16:00 preopen / 2026-01-19: 19:00 open` | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-18 -> TD 2026-01-20; eventDate 2026-01-19 -> TD 2026-01-20 |
| 2026-02-16 | closed | `2026-02-15: 16:00 preopen / 2026-02-16: 19:00 open` | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-15 -> TD 2026-02-17; eventDate 2026-02-16 -> TD 2026-02-17 |
| 2026-04-03 | closed | `2026-04-03: no events published` | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03 -> TD none printed |
| 2026-05-25 | closed | `2026-05-24: 16:00 preopen / 2026-05-25: 19:00 open` | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-24 -> TD 2026-05-26; eventDate 2026-05-25 -> TD 2026-05-26 |
| 2026-06-19 | closed | `2026-06-19: no events published` | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19 -> TD none printed |
| 2026-07-03 | closed | `2026-07-03: no events published` | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03 -> TD none printed |
| 2026-09-07 | closed | `2026-09-06: 16:00 preopen / 2026-09-07: 19:00 open` | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-06 -> TD 2026-09-08; eventDate 2026-09-07 -> TD 2026-09-08 |
| 2026-11-26 | closed | `2026-11-26: no events published` | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26 -> TD none printed |
| 2026-11-27 | late open and early close | `2026-11-27: 08:30 open; 12:05 closed` | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27 -> TD 2026-11-27 |
| 2026-12-24 | early close | `2026-12-24: 07:45 paused; 08:00 preopen; 08:30 open; 12:05 closed` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-24 -> TD 2026-12-24 |
| 2026-12-25 | closed | `2026-12-25: no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25 -> TD none printed |

**Artifacts, 2026.** Files are relative to `exchange-hours-research/holidays/raw/`.

**Interpretive steps, 2026.**

- **Live retrieval and archived capture agree byte for byte.** The three live artifacts above
  hash identically to their archived counterparts — `arc/thbp_2026-09-06_2026-09-08_20260830142930.json`,
  `arc/thbp_2026-11-25_2026-11-27_20260830142904.json` and
  `arc/thbp_2026-12-24_2026-12-26_20260830142904.json`, all captured 2026-08-30 — so each row
  rests on two vintages of the same channel.
- **Two windows cover Juneteenth and Independence Day 2026; the later one governs.**
  `arc/thbp_2026-06-17_2026-06-19_20260129012310.json` and
  `arc/thbp_2026-07-02_2026-07-04_20260129012216.json` (both captured 2026-01-29) print the
  same `ZC` schedules as the cited 2026-06-19 captures. The later capture is cited; the
  earlier one corroborates and is recorded here rather than beside the row.
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
| 2027-03-26 | closed | `2027-03-26: no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26 -> TD none printed |
| 2027-05-31 | closed | `2027-05-30: 16:00 preopen / 2027-05-31: 19:00 open` | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-30 -> TD 2027-06-01; eventDate 2027-05-31 -> TD 2027-06-01 |
| 2027-06-18 | closed | `2027-06-18: no events published` | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18 -> TD none printed |
| 2027-07-05 | closed | `2027-07-04: no events published / 2027-07-05: no events published` | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-04 -> TD none printed; eventDate 2027-07-05 -> TD none printed |
| 2027-07-06 | late open | `2027-07-06: 06:00 preopen; 08:30 open; 13:20 paused; 13:30 closed; 14:30 pcp; 16:00 closed; 16:45 preopen; 19:00 open` | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-06 -> TD 2027-07-06, 2027-07-07 |
| 2027-09-06 | closed | `2027-09-05: 16:00 preopen / 2027-09-06: 19:00 open` | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-05 -> TD 2027-09-07; eventDate 2027-09-06 -> TD 2027-09-07 |
| 2027-11-25 | closed | `2027-11-25: no events published` | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25 -> TD none printed |
| 2027-11-26 | late open and early close | `2027-11-26: 08:30 open; 12:05 closed` | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26 -> TD 2027-11-26 |
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
  whole day session and no evening leg, so it is audited normal under conversion 2 above.
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

- **Order-entry-only deviations are not representable.** The table's vocabulary is the scalar
  vocabulary of `DayPolicy`, which has no order-entry boundary (design memo D3, §1.4). Five
  classes of deviation in this block are therefore recorded and not modelled. None of them
  moves an `is_open`, `session_bounds`, `trade_date` or `candle_end` answer; all of them can
  move `is_accepting_orders` or `is_order_entry_only`.
  1. On 2025-01-02, 2025-12-26, 2026-01-02 and 2027-07-06 CME prints a `06:00 preopen` where
     the normal week has an `08:00 preopen`; the crate keeps the normal 08:00-08:30 CT queue.
  2. On 2025-11-28 CME prints a `07:00 preopen`, and on 2026-11-27 and 2027-11-26 it prints
     no pre-open at all; the crate keeps the normal 08:00-08:30 CT queue.
  3. On the twelve Sundays before a Monday holiday CME prints a `16:00 preopen` carrying the
     *Tuesday* trade date and no `19:00 open`; the crate keeps its normal Sunday 16:00-19:00 CT
     queue.
  4. On 2025-06-18, 2025-11-26, 2026-11-25 and 2027-11-24 CME prints a `16:45 preopen`
     carrying the post-holiday trade date and no `19:00 open`; the crate keeps its normal
     Monday-Thursday 16:45-19:00 CT queue.
  5. On the eves whose `closed` row deletes the following trade date, the crate's normal
     14:30-16:00 CT post-close queue is deleted with it, because the crate assigns that
     occurrence to the *next* trade date while CME's service labels it with the eve's own.
     CME does print `14:30 pcp; 16:00 closed` on those eves. This is the one deviation in
     which the built-in row removes an order-entry window the operator publishes, so it is
     named here rather than merely noted.

  Closing condition for all five: an order-entry boundary in the holiday vocabulary, which
  design memo §7 defers to the block-row v2 the scalar table cannot express (#93). Served identity,
  so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **No intraday-topology gap.** All eighteen `modified` grain rows in the block triage to
  design memo §1.6 categories 3, 4 and 5 — fourteen to "normal, the neighbouring closure
  carries it" and four to a late open. None is a category-6 topology change, so this family
  contributes no unrepresentable special day in this window.
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
