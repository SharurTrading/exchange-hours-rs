<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_fx` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`fx.rs`](../../src/calendar/schedules/futures/us/fx.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Standard-grid CME FX futures only; eFix, BTIC, TAS, options, and separately specified products are excluded. Matching and the 2010 weekday-queue revision are exact; the current Sunday Pre-Open is sourced but its 16:15→16:00 cutover day is unavailable — bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld — so the dated selector omits that phase.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

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
| `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | `2010-martin-luther-king.pdf` | <https://web.archive.org/web/20100331064226id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-martin-luther-king.pdf> | archive capture 2010-03-31T06:42:26Z | T1 | `11af96423f788565a434209ebb94a82f69dd2c129621927cd898c2ff6c0978d3` |
| `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | `2010-presidents-day.pdf` | <https://web.archive.org/web/20100215064641id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-presidents-day.pdf> | archive capture 2010-02-15T06:46:41Z | T1 | `ba379a7fa57efef43820583ada0002ea6cd8ccf0caf1b650d1cb6e8561f84253` |
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | <https://web.archive.org/web/20100601111916id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-good-friday.pdf> | archive capture 2010-06-01T11:19:16Z | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
| `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | `2010-memorial-day.pdf` | <https://web.archive.org/web/20100601094225id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-memorial-day.pdf> | archive capture 2010-06-01T09:42:25Z | T1 | `46a2f00f0f23c82189d86953092ee1171e0ec1c460891bda3ef0578518d12859` |
| `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | `2010-4th-of-july.pdf` | <https://web.archive.org/web/20100602005637id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-4th-of-july.pdf> | archive capture 2010-06-02T00:56:37Z | T1 | `c6a0f8c0b079b85e4500d30d942247bc9aa14d6c70fe3ed73d6d77b21b65ee2c` |
| `2010-labor-day.pdf @2010-06-02T00:56:41Z` | `2010-labor-day.pdf` | <https://web.archive.org/web/20100602005641id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-labor-day.pdf> | archive capture 2010-06-02T00:56:41Z | T1 | `2aecfe737c9613f82b975a01812607e96c9284638e8bd9b9bd403edfae510620` |
| `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | `2010-columbus-day.pdf` | <https://web.archive.org/web/20100821133122id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-columbus-day.pdf> | archive capture 2010-08-21T13:31:22Z | T1 | `e8fbf61d914ebd3cd74de611a7b2d9a4d13a67e1233045ccc2fcbb57556be3f9` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | <https://web.archive.org/web/20101122094012id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-thanksgiving.pdf> | archive capture 2010-11-22T09:40:12Z | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2010-christmas.pdf @2010-12-14T06:12:38Z` | `2010-christmas.pdf` | <https://web.archive.org/web/20101214061238id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-christmas.pdf> | archive capture 2010-12-14T06:12:38Z | T1 | `d4adb060f6eb592fb24e3a272db57b3d8c9d69f3f370682ecd8d57e4169c37be` |
| `2011-new-years.pdf @2011-11-01T14:39:45Z` | `2011-new-years.pdf` | <https://web.archive.org/web/20111101143945id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-new-years.pdf> | archive capture 2011-11-01T14:39:45Z | T1 | `42c289804cd3fa0830556ecb7fcc31493c452ba9e7325ebe7d7ce29613e41476` |
| `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | `2011-martin-luther-king.pdf` | <https://web.archive.org/web/20111028023429id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-martin-luther-king.pdf> | archive capture 2011-10-28T02:34:29Z | T1 | `2e389e2688d6760705220a11657329e76a9eb3a88d78664b4775eb7481be7b17` |
| `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | `2011-presidents-day.pdf` | <https://web.archive.org/web/20111028023516id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-presidents-day.pdf> | archive capture 2011-10-28T02:35:16Z | T1 | `0342359e135acada5cfaa1b74f806477f759924a0b40625e035242c7e81321d4` |
| `2011-good-friday.pdf @2011-10-28T02:37:07Z` | `2011-good-friday.pdf` | <https://web.archive.org/web/20111028023707id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-good-friday.pdf> | archive capture 2011-10-28T02:37:07Z | T1 | `6cf10359bb438eb49287dcef7c1e75a484df4d6e3b538fa9ee59dc3832210bda` |
| `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | `2011-memorial-day.pdf` | <https://web.archive.org/web/20130930105652id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-memorial-day.pdf> | archive capture 2013-09-30T10:56:52Z | T1 | `5482f7bf47e0ee61448cf5f60fd4a5373cc39cb0e46220150c1f6a2ab2d6caec` |
| `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | `2011-4th-of-july.pdf` | <https://web.archive.org/web/20111101144054id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-4th-of-july.pdf> | archive capture 2011-11-01T14:40:54Z | T1 | `4c3bfbbe927ed799006edce76b5f263b6299bab29be7ff1eb06326e184a9b443` |
| `2011-labor-day.pdf @2011-11-01T14:43:45Z` | `2011-labor-day.pdf` | <https://web.archive.org/web/20111101144345id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-labor-day.pdf> | archive capture 2011-11-01T14:43:45Z | T1 | `03f38fea761a6da7633c3e636a40de61431c866a87f2229f270685bf9bb4470b` |
| `2011-columbus-day.pdf @2011-11-01T14:39:16Z` | `2011-columbus-day.pdf` | <https://web.archive.org/web/20111101143916id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-columbus-day.pdf> | archive capture 2011-11-01T14:39:16Z | T1 | `cfbf082c5931fbb753b980f117746fdd3a3f2b678a3a08ec282aa1a38f3f2a1f` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | <https://web.archive.org/web/20111124185246id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf> | archive capture 2011-11-24T18:52:46Z | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | <https://web.archive.org/web/20120125020548id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf> | archive capture 2012-01-25T02:05:48Z | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | <https://web.archive.org/web/20120125025430id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf> | archive capture 2012-01-25T02:54:30Z | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | <https://web.archive.org/web/20120505161526id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-martin-luther-king.pdf> | archive capture 2012-05-05T16:15:26Z | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | <https://web.archive.org/web/20120505161539id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-presidents-day.pdf> | archive capture 2012-05-05T16:15:39Z | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
| `2012-good-friday.pdf @2012-04-17T00:42:47Z` | `2012-good-friday.pdf` | <https://web.archive.org/web/20120417004247id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-good-friday.pdf> | archive capture 2012-04-17T00:42:47Z | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | <https://web.archive.org/web/20120915003714id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf> | archive capture 2012-09-15T00:37:14Z | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | <https://web.archive.org/web/20120915003923id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf> | archive capture 2012-09-15T00:39:23Z | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | <https://web.archive.org/web/20120915003437id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf> | archive capture 2012-09-15T00:34:37Z | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-columbus-day.pdf @2012-09-15T00:15:14Z` | `2012-columbus-day.pdf` | <https://web.archive.org/web/20120915001514id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-columbus-day.pdf> | archive capture 2012-09-15T00:15:14Z | T1 | `aae7ddc8789c31fda8fbe6ccf09ffc5719cbfdf518750c2aa236e7519b11babf` |
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
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-04-17&toEventDate=2025-04-19> | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-07-03&toEventDate=2025-07-05> | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | <https://web.archive.org/web/20260129012309id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-28> | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | <https://web.archive.org/web/20260129012159id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-24&toEventDate=2025-12-26> | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-31&toEventDate=2026-01-02> | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | <https://web.archive.org/web/20260619114118id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-04-01&toEventDate=2026-04-03> | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | <https://web.archive.org/web/20260619113404id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-18&toEventDate=2026-06-20> | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | <https://web.archive.org/web/20260619114108id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-03&toEventDate=2026-07-05> | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-11-25&toEventDate=2026-11-27> | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-22&toEventDate=2026-12-24> | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-24&toEventDate=2026-12-26> | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-31&toEventDate=2027-01-02> | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-03-25&toEventDate=2027-03-27> | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-17&toEventDate=2027-06-19> | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-11-24&toEventDate=2027-11-26> | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-12-22&toEventDate=2027-12-25> | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |

## Holidays

**Coverage:** 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).
This wave is 14 rows over 2022-01-01..2024-12-31, **7 at T1** and **7 at T2**. The T1 rows are read from the 2022 per-asset-class workbooks, the 2023 one-pagers, and the two 2024 entries CME's own cmegroup.com worksheets `new-years-day-2024.pdf` and `christmas-day-2023.pdf` serve; the T2 rows are the operator's own `trading-hours-by-product` responses, which carry the rest of 2024 and the three 2023 holiday dates its one-pagers do not cover. 3 of the 14 are `unsourced`.
Tier: **T1** for 2010-2012 and 2016-2018, from the operator's own published
holiday schedules; **T2** for 2025-2027, from its trading-hours service. Inside a
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
| 2010-01-15 | early close | `15:15 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 15` above it, and the year comes from the document's own identity |
| 2010-01-18 | early close | `12:00 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jan 18` above it, and the year comes from the document's own identity |
| 2010-02-12 | early close | `15:15 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Feb 12` above it, and the year comes from the document's own identity |
| 2010-02-15 | early close | `12:00 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Feb 15` above it, and the year comes from the document's own identity |
| 2010-04-02 | early close | `10:15 CT` | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | `10:15 CT` is the date's own final close; CME prints `Apr 2` above it, and the year comes from the document's own identity |
| 2010-05-28 | early close | `15:15 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `15:15 CT` is the date's own final close; CME prints `May 28` above it, and the year comes from the document's own identity |
| 2010-05-31 | early close | `12:00 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `12:00 CT` is the date's own final close; CME prints `May 31` above it, and the year comes from the document's own identity |
| 2010-07-02 | early close | `15:15 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jul 2` above it, and the year comes from the document's own identity |
| 2010-07-05 | early close | `12:00 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jul 5` above it, and the year comes from the document's own identity |
| 2010-09-03 | early close | `15:15 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Sep 3` above it, and the year comes from the document's own identity |
| 2010-09-06 | early close | `12:00 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Sep 6` above it, and the year comes from the document's own identity |
| 2010-10-08 | early close | `15:15 CT` | `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Oct 8` above it, and the year comes from the document's own identity |
| 2010-11-25 | early close | `12:00 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2010-11-26 | early close | `12:15 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 26` above it, and the year comes from the document's own identity |
| 2010-12-24 | closed | `CME Globex is closed` | `2010-christmas.pdf @2010-12-14T06:12:38Z` | T1 | CME prints `Dec 24` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-12-31 | early close | `12:15 CT` | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Dec 31` above it, and the year comes from the document's own identity |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-14 | early close | `15:15 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 14` above it, and the year comes from the document's own identity |
| 2011-01-17 | early close | `12:00 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jan 17` above it, and the year comes from the document's own identity |
| 2011-02-18 | early close | `15:15 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Feb 18` above it, and the year comes from the document's own identity |
| 2011-02-21 | early close | `12:00 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Feb 21` above it, and the year comes from the document's own identity |
| 2011-04-22 | closed | `CME Globex is closed` | `2011-good-friday.pdf @2011-10-28T02:37:07Z` | T1 | CME prints `Apr 22` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-05-27 | early close | `15:15 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `15:15 CT` is the date's own final close; CME prints `May 27` above it, and the year comes from the document's own identity |
| 2011-05-30 | early close | `12:00 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `12:00 CT` is the date's own final close; CME prints `May 30` above it, and the year comes from the document's own identity |
| 2011-07-01 | early close | `15:15 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jul 1` above it, and the year comes from the document's own identity |
| 2011-07-04 | early close | `12:00 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jul 4` above it, and the year comes from the document's own identity |
| 2011-09-02 | early close | `15:15 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Sep 2` above it, and the year comes from the document's own identity |
| 2011-09-05 | early close | `12:00 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Sep 5` above it, and the year comes from the document's own identity |
| 2011-10-07 | early close | `15:15 CT` | `2011-columbus-day.pdf @2011-11-01T14:39:16Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Oct 7` above it, and the year comes from the document's own identity |
| 2011-11-24 | early close | `12:00 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Nov 24` above it, and the year comes from the document's own identity |
| 2011-11-25 | early close | `12:15 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2011-12-26 | closed | `CME Globex is closed` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | CME prints `Dec 26` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-12-27 | late open | `16:00 CT` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | `16:00 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `CME Globex is closed` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | CME prints `Jan 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-01-03 | late open | `16:00 CT` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | `16:00 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-01-13 | early close | `15:15 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 13` above it, and the year comes from the document's own identity |
| 2012-01-16 | early close | `12:00 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jan 16` above it, and the year comes from the document's own identity |
| 2012-02-17 | early close | `15:15 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Feb 17` above it, and the year comes from the document's own identity |
| 2012-02-20 | early close | `12:00 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Feb 20` above it, and the year comes from the document's own identity |
| 2012-04-06 | early close | `10:15 CT` | `2012-good-friday.pdf @2012-04-17T00:42:47Z` | T1 | `10:15 CT` is the date's own final close; CME prints `Apr 6` above it, and the year comes from the document's own identity |
| 2012-05-25 | early close | `15:15 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `15:15 CT` is the date's own final close; CME prints `May 25` above it, and the year comes from the document's own identity |
| 2012-05-28 | early close | `12:00 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `12:00 CT` is the date's own final close; CME prints `May 28` above it, and the year comes from the document's own identity |
| 2012-07-04 | early close | `12:00 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Jul 4` above it, and the year comes from the document's own identity |
| 2012-08-31 | early close | `15:15 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Aug 31` above it, and the year comes from the document's own identity |
| 2012-09-03 | early close | `12:00 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Sep 3` above it, and the year comes from the document's own identity |
| 2012-10-05 | early close | `15:15 CT` | `2012-columbus-day.pdf @2012-09-15T00:15:14Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Oct 5` above it, and the year comes from the document's own identity |
| 2012-11-22 | early close | `12:00 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:00 CT` is the date's own final close; CME prints `Nov 22` above it, and the year comes from the document's own identity |
| 2012-11-23 | early close | `12:15 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 23` above it, and the year comes from the document's own identity |
| 2012-12-24 | early close | `12:15 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Dec 24` above it, and the year comes from the document's own identity |
| 2012-12-25 | closed | `CME Globex is closed` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | CME prints `Dec 25` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-12-26 | late open | `16:00 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `16:00 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |


### 2013

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | — | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | CME prints no session running through this date |
| 2013-01-02 | late open | — | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run |
| 2013-01-18 | early close | — | `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2013-01-21 | early close | — | `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2013-02-15 | early close | — | `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2013-02-18 | early close | — | `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2013-03-29 | closed | — | `2013-good-friday.pdf @2013-06-23T19:59:25Z` | T1 | CME prints no session running through this date |
| 2013-05-24 | early close | — | `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2013-05-27 | early close | — | `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2013-07-04 | early close | `1215 CT` / `1700 CT` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2013-08-30 | early close | — | `2013-labor-day.pdf @2013-09-02T17:08:41Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2013-09-02 | early close | — | `2013-labor-day.pdf @2013-09-02T17:08:41Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2013-11-28 | early close | — | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2013-11-29 | early close | — | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT |
| 2013-12-24 | early close | `1215 CT` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT |
| 2013-12-25 | closed | `1215 CT` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | CME prints no session running through this date |
| 2013-12-26 | late open | `1215 CT` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run |

### 2014

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | — | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | CME prints no session running through this date |
| 2014-01-02 | late open | — | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run |
| 2014-01-17 | early close | — | `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2014-01-20 | early close | — | `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2014-02-14 | early close | — | `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2014-02-17 | early close | — | `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2014-04-18 | closed | — | `2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z` | T1 | CME prints no session running through this date |
| 2014-05-23 | early close | — | `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2014-05-26 | early close | — | `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2014-07-04 | early close | `1215 CT / 1315 ET / 1715 UTC` / `1700 CT / 1800 ET / 2200 UTC` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2014-08-29 | early close | — | `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2014-09-01 | early close | — | `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2014-11-27 | early close | — | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2014-11-28 | early close | — | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT |
| 2014-12-24 | early close | `1215 CT / 1315 ET / 1815 UTC` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT |
| 2014-12-25 | closed | `1215 CT / 1315 ET / 1815 UTC` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | CME prints no session running through this date |

### 2015

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | — | `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | T1 | CME prints no session running through this date |
| 2015-01-16 | early close | — | `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2015-01-19 | early close | — | `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2015-02-13 | early close | — | `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2015-02-16 | early close | — | `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2015-04-03 | early close | — | `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | T1 | the printed final close 10:15 CT is earlier than the family's ordinary 16:00 CT |
| 2015-05-22 | early close | — | `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | T1 | the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT |
| 2015-05-25 | early close | — | `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2015-07-03 | early close | — | `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2015-09-07 | early close | — | `2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2015-11-26 | early close | — | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT |
| 2015-11-27 | early close | — | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT |
| 2015-12-24 | early close | `1215 CT / 1315 ET / 1815 UTC` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT |
| 2015-12-25 | closed | `1215 CT / 1315 ET / 1815 UTC` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | CME prints no session running through this date |

### Documents
| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2013-4th-of-july-done.pdf @2013-07-17T05:03:33Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130717050333id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-4th-of-july-done.pdf> | archive capture 2013-07-17T05:03:33Z | T1 | `768c7813542459dbc5443e79e514bb12fd511d93284beeb880af167f6049e0f0` |
| `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20130623205825id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-4th-of-july.pdf> | archive capture 2013-06-23T20:58:25Z | T1 | `52c62e72329866f12726363d762335c8dadfdd3407c3643f38d7ed74608e8eb7` |
| `2013-christmas.pdf @2014-04-12T06:24:28Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20140412062428id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-christmas.pdf> | archive capture 2014-04-12T06:24:28Z | T1 | `1389ade6b120383d05e8d6e8ed9c38f1e2394dd250c86b19606548618285e848` |
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
| `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150905222733id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-4th-of-july-holiday-schedule.pdf> | archive capture 2015-09-05T22:27:33Z | T1 | `1013e6ebea1829591946c9aa513ec6e3f99c64d99be14814610e601bb5dcc0fb` |
| `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20151123061520id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-christmas-holiday-schedule.pdf> | archive capture 2015-11-23T06:15:20Z | T1 | `7fde46210798f2cb4299903400b41ef6e2cb9aa287f8d3d360ba59d835b482ec` |
| `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150905223230id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-good-friday-holiday-schedule.pdf> | archive capture 2015-09-05T22:32:30Z | T1 | `67873caa987c9eee8de38d02a5c18d8f7d46fd82a486bab34821f6e6f6363e0b` |
| `2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150824023039id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-labor-day-holiday-schedule.pdf> | archive capture 2015-08-24T02:30:39Z | T1 | `4aab7fdb5e57420a243bbc26be6ecfa00a6317937afe3b772dfcdd8db1dd50fd` |
| `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121141012id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-martin-luther-king-holiday-schedule.pdf> | archive capture 2015-01-21T14:10:12Z | T1 | `67395dcb63d86b6e1f573a5aa37269a685dfe1b17d23edd7d95449899bce090a` |
| `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150326113938id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-memorial-day-holiday-schedule.pdf> | archive capture 2015-03-26T11:39:38Z | T1 | `4ad4398fa092393fa77cd8ed9c58ecc0f1669a41fd38a9b4cab5b05790d2dfec` |
| `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121141043id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-new-years-holiday-schedule.pdf> | archive capture 2015-01-21T14:10:43Z | T1 | `196b4ec04bcfce2cfd9783262023afe9da73bcdef1b983c8c4b464b2e115bb8a` |
| `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150121192401id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-presidents-day-holiday-schedule.pdf> | archive capture 2015-01-21T19:24:01Z | T1 | `c81b04bcf43dbfd2b7c2f7aba559992cde8bb8eee59e7b310b643b636d290b94` |
| `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | 2013-01-01 .. 2015-12-31 | <https://web.archive.org/web/20160205162519id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf> | archive capture 2016-02-05T16:25:19Z | T1 | `ef96d05289667c635f435884a0a2407bcbec63149ff96e802b719100c85c887f` |

### Gaps and residual risks, 2013-2015

**This era declares the family's sixth audited window.** The table as a whole carries 197 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **47 rows**: 8 stated closures, 36 early closes and 3 late opens. Every row is at T1.

**The Friday eves of the Monday holidays close at 15:15 CT.** Every 2013-2015 holiday schedule prints `1515 CT - Early close` for the Interest Rate and FX lines on the Friday before a Monday holiday, an hour and a quarter earlier than the family's ordinary 16:00 CT close, while the Equity line on the same sheet prints its ordinary 16:15 CT. The rows are that printed early close. From 2014 the same line is `1515 CT / 1615 ET / 2115 UTC`; the 2014 Martin Luther King sheet's UTC column reads `2215 UTC` where 1515 CT is 2115 UTC, and the block records CME's printed value with the arithmetic flagged rather than correcting it, so the crate row is the CT value both revisions agree on.

### 2016

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Globex closed` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-01-18 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-02-15 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-03-25 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-05-30 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-07-04 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-09-05 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-11-24 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-11-25 | early close | `1215 CT / 1315 ET / 1815 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-12-26 | closed | `Globex closed` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2017

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `Globex closed` | `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-01-16 | early close | `12:00 CT / 13:00 ET` | `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-02-20 | early close | `12:00 CT / 13:00 ET` | `2017-presidents-day-holiday-schedule.xls @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-04-14 | closed | `no session printed` | `2017-good-friday-holiday-schedule.xls @2017-05-05` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-05-29 | early close | `12:00 CT / 13:00 ET` | `2017-memorial-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-07-04 | early close | `12:00 CT / 13:00 ET` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-09-04 | early close | `12:00 CT / 13:00 ET` | `2017-labor-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-11-23 | early close | `12:00 CT / 13:00 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-11-24 | early close | `12:15 CT / 13:15 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-12-25 | closed | `no session printed` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2018

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `no session printed` | `2018-new-years-holiday-schedule.xls @2018-01-06` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-01-15 | early close | `12:00 CT / 13:00 ET` | `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-02-19 | early close | `12:00 CT / 13:00 ET` | `2018-presidents-day-holiday-schedule.xls @2018-05-08` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-03-30 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-05-28 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-07-04 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-09-03 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-11-22 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-11-23 | early close | `12:15 CT / 13:15 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-24 | early close | `12:15 CT / 13:15 ET` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-25 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |


### 2019

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Closed for New Year's` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-01-21 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-02-18 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-04-19 | closed | `Closed for Good Friday` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-05-27 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-06-19 | unsourced | no CME document covers this date (see `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`) | `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2019-07-04 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-09-02 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-11-28 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-11-29 | early close | `1215 CT / 1815 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-24 | early close | `1215 CT / 1815 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `FX` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-25 | closed | `Closed for Christmas` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2020

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Closed for New Year's` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-01-20 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-02-17 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-04-10 | closed | `Closed for Good Friday` | `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-05-25 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-06-19 | unsourced | no CME document covers this date (see `2020-holiday-calendars.zip @2026-07-30T11:18:34Z`) | `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2020-07-03 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-09-07 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-11-26 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-11-27 | early close | `1215 CT / 1815 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-24 | early close | `1215 CT / 1815 UTC` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `FX` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-25 | closed | `Closed for Christmas` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2021

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Closed for New Year's` | `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-01-18 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-02-15 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-04-02 | early close | `1015 CT / 1515 UTC` | `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `10:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-05-31 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-06-19 | unsourced | no CME document covers this date (see `2021-holiday-calendars.zip @2026-08-30T10:03:27Z`) | `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2021-07-05 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-09-06 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-11-25 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-11-26 | early close | `1215 CT / 1815 UTC` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `FX` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-12-24 | closed | `Closed for Christmas` | `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

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
**This era declares the family's fifth audited window.** The table as a whole carries 144 rows over 5 windows — 2010-01-01..2012-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **35 rows**: 8 full closures, 24 early closes and 3 `Unsourced` rows. Every row is at T1. The interval 2013-01-01 .. 2015-12-31 remains audited by no wave, so `holiday_coverage` reports it as outside every window rather than as audited normal.
**Juneteenth 2019, 2020 and 2021 — three `Unsourced` rows.** CME published no Juneteenth schedule in any of the three years. Each row cites that year's own consolidated bundle — `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`, `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` and `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — whose member lists are CME's own account of every Globex holiday schedule it published that year and which carry no Juneteenth sheet; the four archived `holiday-calendar.html` index pages name none either, and a fresh 2018-2027 prefix CDX enumeration (`raw/cme-2019-2021-fix/cdx/cdx-files-2018-2027.json`, 369 rows, 340 distinct filenames) finds no `juneteenth` filename before 2022. Inside a contiguous window silence is the positive claim that a date was audited normal, which is false for a date the operator later marks as a holiday, so all three ship `Unsourced`, which clips nothing. 2021-06-19 is a **Saturday**: no family has a trade date there and the row changes no answer, and the row is keyed to the operator's own calendar date for the holiday rather than to an observed date CME never states. Closing condition: a CME holiday schedule naming Juneteenth in one of these three years.
**Columbus Day and Veterans Day — six dates with no row.** 2019-10-14, 2019-11-11, 2020-10-12, 2020-11-11, 2021-10-11 and 2021-11-11 lie inside this window and carry no row, so the family's ordinary week stands there. CME published settlement-time and OTC-clearing advisories for these dates — the 2019 ZIP's `settlement-notices/*-settlement-times.pdf` members and, for example, `2021-veterans-day-advisory.pdf` — but never a Globex trading schedule for them. A settlement notice is not session language (LAW-SESSION-NOT-EXPIRY), so no row is keyed to one and the block's `missing` register records the dates as gaps rather than as sourced normality. Closing condition: a CME Globex holiday schedule naming one of these dates.

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `"Globex Closed"` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-11-25 | early close | `12:15` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `12:15` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-12-26 | closed | `"Globex Closed"` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `"Globex Closed"` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-01-16 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-01-15` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-02-20 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-02-19` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-04-07 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-04-06` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-11-24 | early close | `12:15` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `12:15` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-12-25 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-03-29 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-03-28` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-11-29 | early close | `12:15` | `CME-SVC-2024-11-27` | T2 | CME prints `12:15` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-24 | early close | `12:45` | `CME-SVC-2024-12-24` | T2 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-25 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `CME-SVC-2024-12-24` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
### Documents

This era's rows cite the ids below: CME Group's own published holiday schedules at **T1** (2022 per-asset-class workbooks, 2023 one-pagers, and the 2024 Good Friday and year-end service windows as archived), and responses of CME's own `trading-hours-by-product` service at **T2**. Each id resolves to the URL it was read at — an Internet Archive raw replay for a saved capture, the operator's own endpoint for a live retrieval — with the capture or retrieval time in UTC, the tier and the sha256. All thirty artifacts resolve in the research store's `holidays/raw/cme-2022-2024/` and `holidays/raw/cme-2022-2024-fix/`, whose `INDEX.md` files carry the byte counts.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-good-friday-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `a82936ab14d1b1f7041583123289c4de401c66ace4eea7e90fa9f60c1a3f3b7e` |
| `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20221122060801id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-thanksgiving-holiday-schedule.xls> | archive capture 2022-11-22T06:08:01Z | T1 | `64341a65350de982a6a05760504132e173611151ae3164c6f9437509a406edcb` |
| `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065430id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-christmas-holiday-schedule.xls> | archive capture 2022-07-04T06:54:30Z | T1 | `2dd1d531514989845dcb6ce6d767db5dd3f956ac6777ab1cfe36d6843f3762e7` |
| `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2023-new-years-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `eefafd1066f406edbe6167ddf8ad13c0697b124a337c0893ce5a5da200c783d3` |
| `CME-SVC-2023-01-15` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-01-15&toEventDate=2023-01-17&isProtected&_t=1720455278636> | archive capture 2024-07-08T16:14:38Z | T2 | `507fd196a7654ddd916218b2aa24a146eeaca4e1f7cb7daded4e7a9c689cda82` |
| `CME-SVC-2023-02-19` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-02-19&toEventDate=2023-02-21&isProtected&_t=1720455278640> | archive capture 2024-07-08T16:14:38Z | T2 | `d063238a83e8cb84d4484a86b26cb1d976f1ccfdb91972eecc08dcc0cec49412` |
| `CME-SVC-2023-04-06` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-04-06&toEventDate=2023-04-08&isProtected&_t=1720455278642> | archive capture 2024-07-08T16:14:38Z | T2 | `0543d5f6d2efd4aa6f4132ce9f5425b9ba9f64de08eccd677b78d6b43c6e9d4f` |
| `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20231203205929id_/https://www.cmegroup.com/trading-hours/files/thanksgiving-day-2023.pdf> | archive capture 2023-12-03T20:59:29Z | T1 | `99e187b3f3899e1062d662e148d5978e0cd075b961e6fc79550d4393812307e8` |
| `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260719095248id_/https://www.cmegroup.com/trading-hours/files/christmas-day-2023.pdf> | archive capture 2026-07-19T09:52:48Z | T1 | `edcde0fcf61d3414cee2a332453db861e0e2d7edf81d89a5f379c44b2e72d5cf` |
| `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260811165716id_/https://www.cmegroup.com/trading-hours/files/new-years-day-2024.pdf> | archive capture 2026-08-11T16:57:16Z | T1 | `34e60f8c97623df30e00f0ad8e4eeda20b99001b6c35d64f735828fecec5b9b8` |
| `CME-SVC-2024-03-28` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-03-28&toEventDate=2024-03-30&isProtected&_t=1720455278672> | archive capture 2024-07-08T16:14:39Z | T2 | `9b41709219e36f56296843fe589e362a7513e9a98132a682a1b956f2e40c593b` |
| `CME-SVC-2024-11-27` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-11-27&toEventDate=2024-11-29&isProtected&_t=1720455278685> | archive capture 2024-07-08T16:14:39Z | T2 | `f6a15f26991d25f8c6821fa6c967d0e76b2ad3f775bed62a2a001ec2fd7e389a` |
| `CME-SVC-2024-12-24` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-24&toEventDate=2024-12-26&isProtected&_t=1734710019537> | archive capture 2024-12-20T15:53:40Z | T2 | `183c85160e31d53fde30c422048b8e778937d8866f3ea1c45e2dd570ea9db1c8` |
### Gaps and residual risks, 2022-2024

**The three 2023 dates the operator published nothing for — 2023-01-16, 2023-02-20 and 2023-04-07.** They ship `Unsourced`, cited to the T2 captures `CME-SVC-2023-01-15`, `CME-SVC-2023-02-19` and `CME-SVC-2023-04-06`, each of which is the operator's own machine channel read as bytes and returns an empty event list for its window: 2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023), 2023 Presidents Day (Monday 20 February 2023) and 2023 Good Friday (Friday 7 April 2023). The block's `missing` register records the channels searched — `holiday-calendar/files/*.xls`, of which only a compact MGEX/DME workbook exists for MLK, and `trading-hours/files/<holiday>-2023.pdf`, which 404s — so the gap is "not worked up", not "no source exists". `Unsourced` clips nothing, so each of these dates still resolves to the family's ordinary week. Closing condition: a CME holiday schedule for 2023 covering this date at T1, or a T2 window that carries its events. The day after each of the three is a second, unmodelled gap: 2023-01-17 and 2023-02-21 (and 2023-04-10 for `globex_grains`) may have lost their prior-evening leg the way 2024-01-02 and 2023-12-26 did, and no artifact this crate read states whether they did, so they ship no row and the family's ordinary week stands there.

**The 2024-04-01 re-open.** The Good Friday window `CME-SVC-2024-03-28` ends at 2024-03-30 and prints empty event lists for both 2024-03-29 and 2024-03-30, so CME published nothing this crate read for the Sunday 2024-03-31 evening leg that would carry trade date 2024-04-01. Inside a contiguous window silence would read as audited normal, so 2024-04-01 ships **no row** and is a declared gap rather than an audited one. Closing condition: a CME service response covering 2024-03-31, or a T1 statement of that Sunday's re-open. For `globex_grains` the same silence is what withholds a possible late open on 2024-04-01.

**Normal-week notes that ship no row.** Where a printed token falls outside the family's ordinary week but moves no boundary a scalar holiday row can state, the date ships nothing and the token is recorded here: the 2022 New Year's workbooks print the `Nikkei/TOPIX BTIC` `Close 00:00` on 2022-01-01, a BTIC close for the next trade date; the 2023 and 2024 Independence Day and New Year grain entries print the next trade date's `06:00 (PREOPEN)`, which is the no-evening-leg marker the six `globex_grains` late opens are read from — those rows are in that family's own table and evidence file and this family has none; and the 2024-12-31 grain entry points at `2025-01-02 06:00 preopen`, outside this window, so no row ships and the 2025-2027 table must state that trade date.

### 2025-2027 (T2)

**Zone.** Verbatim from the operator's page: "Trading hours are subject to
change and are in U.S. Central Time unless otherwise stated." CME prints no ET
column in this channel, so every instant below is quoted in CT exactly as
printed and **no ET value is asserted**.

**The conversion (design memo D1).** This family's grid is one wrapping
Sunday-to-Thursday 17:00→16:00 CT matching block, so a trading day always opens
on the previous local evening. CME keys its records by *event date* and prints
its own trade date beside each event; the crate keys by *trade date*. An eve
record is therefore evidence for the holiday's row, not a row of its own —
except where the eve carries an early close of its own trade date, which is
what Christmas Eve does. The **Derived from** column records the operator event
dates and the operator's own printed trade date for every row, so the
conversion can be re-checked.

**Grid this was compared against.** CME's own reference week 2026-10-18 ..
2026-10-24, pulled from the same service: Sun `16:00 preopen`, `17:00 open`;
Mon–Thu `16:00 closed`, `16:45 preopen`, `17:00 open`; Fri `16:00 closed`;
Sat none.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — both events carry CME trade date 2025-01-02, so no session belongs to trade date 2025-01-01 | `CME-SVC-2024-12-31` | T2 | eventDate 2024-12-31 (`16:00 closed`, CME trade date 2024-12-31, no evening re-open) and eventDate 2025-01-01, CME trade date 2025-01-02 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-17 (`16:00 closed`, CME trade date 2025-04-17, no evening re-open) and eventDate 2025-04-18 |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-11-28 | early close | `07:00 preopen; 07:30 open; 13:45 closed` — 13:45 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28 |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29 |
| 2025-12-24 | early close | `12:45 closed` — 12:45 CT | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — both events carry CME trade date 2025-12-26 | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 |

**Interpretive steps, 2025.**

- **2025-01-01 and 2025-12-25 are closures, not late opens.** CME publishes a
  16:00 CT pre-open and a 17:00 CT open on each of those dates, but both events
  already carry the *following* trade date. The crate's own Tuesday-evening
  (resp. Wednesday-evening) leg is the one whose trade date is the holiday, and
  `Closed` removes exactly that leg; the holiday-evening leg, whose trade date
  is the next day, is untouched and opens at its normal 17:00 CT.
- **2025-04-18 needs no eve row.** `16:00 closed` on Thursday 2025-04-17 is the
  family's ordinary final close for its own trade date. What is missing that
  evening is the 17:00 CT leg, and `Closed(2025-04-18)` already removes it —
  the eve record is evidence for the Good Friday row, not a row of its own.
- **2025-12-24 keeps its own trade date.** CME prints `12:45 closed (trade date
  2025-12-24)`, so this is an early final close of the Christmas-Eve trading
  day, which opened Tuesday 2025-12-23 at 17:00 CT. The absent evening re-open
  is carried by `Closed(2025-12-25)`.
- **2025-11-29 (Saturday) is a sourced closure, not an unchecked date.** The
  live service publishes a 2025-11-29 schedule for all ten headline products
  with no events. The family's normal week has no Saturday session, so the row
  changes no answer; it ships so that the venue-level intersection tables of a
  later wave see the same audited dates in every family.

**Gaps, 2025.**

- **Trade-date merge — 2025-01-20, 2025-02-17, 2025-05-26, 2025-06-19,
  2025-09-01, 2025-11-27 (no row).** On these Monday and Thursday holidays CME
  publishes `16:00 preopen; 17:00 open` for this family instead of `16:00
  closed; 16:45 preopen; 17:00 open`, with both events carrying the next
  business day's trade date. Matching still stops at 16:00 CT and still resumes
  at 17:00 CT, so **no executable phase moves and no `is_open` answer changes**;
  what changes is that the holiday has no final close of its own and the whole
  Sunday-evening-through-Tuesday-16:00 CT (resp.
  Wednesday-evening-through-Friday-16:00 CT) span carries one trade date. The
  scalar vocabulary cannot merge two trade dates, and a `Closed` row would
  instead delete a full evening and day of trading CME in fact ran, so no row
  ships and the crate keeps its normal-week trade-date assignment for the span.
  Closing condition: a block-row vocabulary (#93) that can state a trading day's own
  trade date (design memo §7). Served identity, so tracked as an issue
  (LAW-FOLLOW-UPS-ARE-ISSUES).
- **Order-entry window — the same six dates, plus 2025-01-01 and 2025-12-25.**
  The Globex pre-open opens at 16:00 CT instead of the normal 16:45 CT. The
  table shares `DayPolicy`'s vocabulary, which has no order-entry boundary, so
  this is not representable. It changes no `is_open` answer, only
  `is_accepting_orders` and `is_order_entry_only`, for 45 minutes.
- **Intraday topology — 2025-11-28.** CME's finalised publication additionally
  prints `07:00 preopen; 07:30 open` on the morning of the early close. This
  family has no separate regular open — the leg has been continuously open
  since Thursday 17:00 CT — so the pair is neither a late open nor a second
  session the scalar vocabulary can state. The early-close row is unaffected;
  the superseded 2024-12-20 publication does not print the pair and agrees on
  the 13:45 CT instant.
- **Residual risk — 2025-01-01, 2025-04-18 and 2025-07-04.** These three rows
  rest on the single Wayback capture 2024-12-20T15:53:40Z of the service, i.e.
  CME's published future as of that date rather than a post-holiday statement;
  CME's own page says "This schedule is subject to change. Trading hours are
  usually finalized approximately two weeks prior to the holiday." A fresh CDX
  enumeration of the endpoint (688 rows, 2026-09-12) confirms no later capture
  of those windows exists, and the live service's retention edge falls between
  Labor Day 2025 and Thanksgiving 2025, so the channel cannot restate them. The
  rows are internally consistent with the 2026 and 2027 rows for the same
  holidays, which are sourced from later captures and from live retrieval.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — both events carry CME trade date 2026-01-02 | `CME-SVC-2025-12-31` | T2 | eventDate 2025-12-31 (`16:00 closed`, CME trade date 2025-12-31, no evening re-open) and eventDate 2026-01-01, CME trade date 2026-01-02 |
| 2026-04-03 | early close | `10:15 closed` — 10:15 CT | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03 |
| 2026-06-19 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22 |
| 2026-07-03 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 |
| 2026-11-27 | early close | `13:45 closed` — 13:45 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:45 closed` — 12:45 CT | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-24 (early close above, no evening re-open) and eventDate 2026-12-25 |

**Interpretive steps, 2026.**

- **Good Friday 2026 is CME's own flagged exception.** Verbatim from the T1
  operator page D50: "Due to the US Employment Situation Release on April 3,
  2026, CME Group Equities, FX, Cryptocurrency and Interest Rate products will
  have unique Closes and Settlements for trade date April 3rd." D50 is T1
  corroboration that this family traded that morning and states no instants;
  the 10:15 CT close comes only from the T2 service document, which is what the
  row cites.
- **2026-06-19 and 2026-07-03 keep the crate's own trade date.** CME prints
  `12:00 closed` on each of those Fridays but attaches its own trade date
  2026-06-22 / 2026-07-06 to the event. Under design memo D1 the row is keyed
  by the crate's venue-local trade date, which is the Friday itself; the early
  close therefore lands on the correct civil instant and clips the leg that
  opened Thursday at 17:00 CT. The operator's differing trade-date label is a
  declared gap below, not a reason to withhold the instant.
- **2026-12-24 keeps its own trade date**, exactly as 2025-12-24 does, and its
  missing evening re-open is carried by `Closed(2026-12-25)`.

**Gaps, 2026.**

- **Trade-date merge — 2026-01-19, 2026-02-16, 2026-05-25, 2026-09-07,
  2026-11-26 (no row).** Same shape and same reasoning as the 2025 entry.
- **Trade-date attribution — 2026-06-19 and 2026-07-03 (row ships).** The row's
  instant is the operator's; the operator assigns the shortened day to trade
  date 2026-06-22 / 2026-07-06 and the crate assigns it to the Friday. Nothing
  the scalar vocabulary can state moves a trading day's trade date forward for
  a family with no business-date roll, so the label differs. Same closing
  condition as the trade-date merge.
- **Saturday sessions — 2026-06-20 and 2026-07-04 (no row).** CME publishes
  `05:00 open; 17:00 closed`, both carrying the following Monday's trade date,
  on a grid whose normal week has no Saturday session at all. `late_open_ssm`
  can only push an existing occurrence later; it cannot create one (design memo
  §1.5 / D7). Sourced, unrepresentable. Saturday 2026-04-04 after Good Friday
  2026 carries no such session, checked against the same service.
- **Order-entry window — 2026-01-19, 2026-02-16, 2026-05-25, 2026-09-07,
  2026-11-26, 2026-01-01.** Pre-open at 16:00 CT rather than 16:45 CT, as in
  2025.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2026-12-31 (`16:00 closed`, CME trade date 2026-12-31, no evening re-open) and eventDate 2027-01-01 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-25 (`16:00 closed`, CME trade date 2027-03-25, no evening re-open) and eventDate 2027-03-26 |
| 2027-06-18 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21 |
| 2027-11-26 | early close | `13:45 closed` — 13:45 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-23 (`16:00 closed`, CME trade date 2027-12-23, no evening re-open) and eventDate 2027-12-24 |

**Interpretive steps, 2027.**

- **Christmas 2027 has no early close for this family.** CME's holiday date is
  Thursday 2027-12-23 and Globex is closed Friday 2027-12-24, with 25 December
  falling on a Saturday. On 2027-12-23 the family publishes the ordinary
  `16:00 closed` for its own trade date and no evening re-open, so that date is
  normal and the missing leg is carried by `Closed(2027-12-24)`.
- **Independence Day 2027 has no row.** The observed holiday is Monday
  2027-07-05, and CME publishes the Monday-holiday shape for this family
  (`16:00 preopen; 17:00 open`, trade date 2027-07-06), which moves no
  executable phase. See the trade-date-merge gap below.
- **2027-06-18 keeps the crate's own trade date**, exactly as 2026-06-19 does.
- **2028-01-01 is outside coverage.** CME's record for it (`no events
  published`, a Saturday) was read with the 2027 year-end window and is the
  ordinary Saturday answer for this family; it ships no row and the coverage
  window stops at 2027-12-31.

**Gaps, 2027.**

- **Trade-date merge — 2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05,
  2027-09-06, 2027-11-25 (no row).** Same shape and same reasoning as the 2025
  entry.
- **Trade-date attribution — 2027-06-18 (row ships).** As 2026-06-19.
- **Saturday session — 2027-06-19 (no row).** `05:00 open; 17:00 closed`,
  carrying trade date 2027-06-21, on a week with no Saturday session. Sourced,
  unrepresentable.
- **Order-entry window — 2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05,
  2027-09-06, 2027-11-25.** Pre-open at 16:00 CT rather than 16:45 CT.

### All years

**Gaps, all years.**

- **No T1 rendering, 2025-2027.** Every row is T2. CME's trading-hours page
  renders only the next upcoming holiday by default and its holiday selector
  could not be driven from the URL, so no T1 per-asset-class rendering could be
  captured for these years; one T1 capture (Thanksgiving 2026) was used to
  validate that the service rows group exactly as the page prints them, and
  D50 corroborates Good Friday 2026 without stating instants. Closing
  condition: a captured T1 rendering of the per-asset-class holiday table.
- **Columbus Day and Veterans Day are not CME Globex holidays.** They appear
  nowhere in CME's own Globex holiday list; CME publishes settlement and
  clearing advisories for them and Globex trades a normal session. Coverage
  here is contiguous, so those dates read as audited normal, which is stated
  rather than left implicit.
- **Scope.** These rows are the standard-grid CME FX futures holiday calendar
  only. eFix, BTIC, TAS, options and separately specified products are excluded
  from this key and from these rows; CME prints TAS/TAM holiday instants in the
  same page's free-text notes and none of them is a family session.
- **Late opens.** This family ships none in the coverage window: CME never
  reopens standard-grid FX later than its normal 17:00 CT. Both branches of the
  late-open disambiguation are therefore untested *by this family's rows*; they
  are exercised by the engine's own suite.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html> — CME Globex notice 20081229.
- <https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf> — CME `FX248` 2010 FX product guide and calendar, which publishes the 17:00–16:00 CT matching grid at the audit floor.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 revision's source.
- <https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf> — CME FX products report, Q1 2018.
- <https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf> — CME emerging-market FX brochure, Q3 2020.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html> — CME FX calendar-spread FAQ.
- <https://web.archive.org/web/20120503103452/http://www.cmegroup.com/trading_hours/fx-hours.html> — CME FX trading-hours page — capture 2012-05-03, Sunday Pre-Open still 16:15. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120616190153/http://www.cmegroup.com/trading_hours/fx-hours.html> — CME FX trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — standard-grid CME FX futures only; eFix, BTIC, TAS, options and separately specified products are excluded.

## Module narrative (moved from src/calendar/schedules/futures/us/fx.rs on 2026-09-12 UTC)

CME's 2010 guide publishes the 17:00-16:00 matching grid for its standard FX
futures. This family is not a promise for eFix, BTIC, TAS, options, or any
product whose own specification publishes a different grid. The exact
Monday-Thursday Pre-Open changed from 16:50 to 16:45 on 2010-11-15. Current
primary material publishes Sunday 16:00-17:00, but calls it a long-term
practice without stating the day on which the earlier queue moved; primary
documents updated 2012-05-03 still publish Sunday 16:15 while pages crawled
2012-06-15/16 already publish 16:00, and no notice in between states the
day. The fixed-current profile includes that exact current phase; dated
profiles carry the sourced Sunday 16:15–17:00 intersection from the
January-2010 floor and withhold only the disputed 16:00–16:15 quarter-hour.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html
https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf
https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html
https://web.archive.org/web/20120503103452/http://www.cmegroup.com/trading_hours/fx-hours.html
https://web.archive.org/web/20120616190153/http://www.cmegroup.com/trading_hours/fx-hours.html

2026-08-31 Sunday-queue review — bracket narrowed, notice channels negative.
Three archived captures of CME's own trading-hours pages, unused by the
earlier review, move the bracket from 2012-05-03..2012-06-15 down to
2012-05-28..2012-06-07. The move was platform-wide and simultaneous: on the
2012-05-28 capture the Sunday Pre-Open is 16:15 for E-mini S&P 500,
Eurodollar, 30-Year Interest Rate Swap, Euroyen TIBOR and (as "17:15 ET
(16:15 CT)") Gold, Silver, Light Sweet Crude and Henry Hub; on the
2012-06-07 capture every one of them reads 16:00. Weekday Pre-Opens are
unchanged across both captures, so this is a Sunday-only change.
CBOT grains are NOT part of it: the 2012-05-11 capture still shows the
pre-expansion 18:00-07:15/09:30-13:15 grain grid with a 16:15 Sunday
Pre-Open, and the 2012-05-28 capture shows the expanded 17:00-14:00 grid
with 16:00 — so grains moved at the separately sourced 2012-05-20
expansion (CME Globex Advisory #20120518), which the grains module already
dates.
Both of CME's dated notice channels were then read in full across the
narrowed window and none announces the change: CME Globex Notices of
2012-05-21, 2012-05-28 and 2012-06-04, and Market Data Notices of
2012-05-28, contain no occurrence of "Pre-Open", "trading hours", "16:00"
or "16:15". The change was therefore made without a dated operator notice,
which is why no cutover is encoded. (The only Sunday inside the narrowed
bracket is 2012-06-03; that is an observation about the bracket, not a
source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the
tables.) Official origin http://www.cmegroup.com/trading_hours/ delivered
via:
https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html
https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html
https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html
https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html

SUNDAY QUEUE, CARRIED BACK AS THE SOURCED INTERSECTION. CME's Sunday Pre-Open
only ever widened inside the modelled window: the audit-floor material pins it
at 16:15 and the verified-current value is 16:00, with the undated 2012 move
(bracketed 2012-05-28..2012-06-07) the only change between them. The
16:15-17:00 window is therefore order-entry under *every* sourced state, so
carrying it from the January-2010 floor asserts no cutover at all - it is the
intersection of the two regimes, not a guess at either. The undated change
adds only the 16:00-16:15 quarter-hour, which the knowledge-bound row supplies
from the repository review date onward. Previously these dated profiles
omitted the Sunday queue entirely, which under-reported order acceptance for
the whole 16:00-17:00 hour rather than only the disputed quarter-hour.
