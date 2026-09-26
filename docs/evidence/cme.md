<!-- SPDX-License-Identifier: MIT-0 -->

# `cme` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for the scoped CME/CBOT equity-index family. Current RTH/ETH and Pre-Open queues are primary-supported; matching and the exact 2010, 2012, 2015, and 2021 revisions are dated, but the Sunday queue's move from 16:15 to 16:00 lacks a source-stated day: the 2026-08-31 review narrowed it to 2012-05-28..2012-06-07 and found no operator notice in either CME channel across that window. Dated profiles now serve the sourced intersection — Sunday 16:15–17:00, carried from the January-2010 floor because the queue only ever widened — so only the disputed 16:00–16:15 quarter-hour waits on the undated move. Full-size `SP`, NKD, BTIC, and TACO products are excluded.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2012-11-18 — T1 — CME Globex notice 20121022 — new daily trading-hour schedule; post-halt slice becomes 15:30–16:15 CT including Fridays.
- 2015-09-20 — T1 — CME Globex notice 20150817 — CME Equity and CBOT Equity closes move 15 minutes earlier to 16:00 CT.
- 2021-06-27 — T1 — CME Globex notice 20210621 — the 15:15–15:30 CT halt is removed, producing the continuous 17:00–16:00 CT ETH envelope.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Evidence documents

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
| `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065430id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-christmas-holiday-schedule.xls> | archive capture 2022-07-04T06:54:30Z | T1 | `2dd1d531514989845dcb6ce6d767db5dd3f956ac6777ab1cfe36d6843f3762e7` |
| `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-good-friday-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `a82936ab14d1b1f7041583123289c4de401c66ace4eea7e90fa9f60c1a3f3b7e` |
| `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065450id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-independence-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:50Z | T1 | `1ea0459d8aa0fd7ec5147614855f6d0d43607efefdc3aa9c6b1e18ddbfd54fde` |
| `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220620200210id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-juneteenth-holiday-schedule.xls> | archive capture 2022-06-20T20:02:10Z | T1 | `bc9f2caf26a73a13029f177fcc6468bdc8662899ff935f44329ee1a95c8b667b` |
| `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065441id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-labor-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:41Z | T1 | `28d533f25d1af74af043e411635f1933fd4ff6359c80c049f87f019d2f1d57d0` |
| `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065438id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-memorial-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:38Z | T1 | `0d1b1f89a315cae22a5857a7027a514e3086c1cccfee42fddddff3caba0c2230` |
| `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220117212230id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-mlk-day-holiday-schedule.xls> | archive capture 2022-01-17T21:22:30Z | T1 | `896944fa701062e2e1ee305f8adb696ba8ab03655a96874c7885f2328177626e` |
| `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704073810id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-presidents-day-holiday-schedule.xls> | archive capture 2022-07-04T07:38:10Z | T1 | `07932975d04ccabad0fb53f33f946f1a6fd8a696bafdb21f95f030ede9a84516` |
| `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20221122060801id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-thanksgiving-holiday-schedule.xls> | archive capture 2022-11-22T06:08:01Z | T1 | `64341a65350de982a6a05760504132e173611151ae3164c6f9437509a406edcb` |
| `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2023-new-years-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `eefafd1066f406edbe6167ddf8ad13c0697b124a337c0893ce5a5da200c783d3` |
| `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260719095248id_/https://www.cmegroup.com/trading-hours/files/christmas-day-2023.pdf> | archive capture 2026-07-19T09:52:48Z | T1 | `edcde0fcf61d3414cee2a332453db861e0e2d7edf81d89a5f379c44b2e72d5cf` |
| `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230613185949id_/https://www.cmegroup.com/trading-hours/files/juneteenth-2023.pdf> | archive capture 2023-06-13T18:59:49Z | T1 | `831f7f63e197ce58436780830aa515cb08dcf92bdf82a540265c2d21a4bffa43` |
| `labor-day-2023.pdf @2023-08-02T19:24:46Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230802192446id_/https://www.cmegroup.com/trading-hours/files/labor-day-2023.pdf> | archive capture 2023-08-02T19:24:46Z | T1 | `39a4c075437fdc7134166328eca730e3cabf3bea00ab369466dff63c99f8e948` |
| `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230420224018id_/https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf> | archive capture 2023-04-20T22:40:18Z | T1 | `7657bc8089ca669cfd244c2e3e697b47a7e650f5d957efe5b32da001622acb35` |
| `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20231203205929id_/https://www.cmegroup.com/trading-hours/files/thanksgiving-day-2023.pdf> | archive capture 2023-12-03T20:59:29Z | T1 | `99e187b3f3899e1062d662e148d5978e0cd075b961e6fc79550d4393812307e8` |
| `CME-SVC-2023-01-15` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-01-15&toEventDate=2023-01-17&isProtected&_t=1720455278636> | archive capture 2024-07-08T16:14:38Z | T2 | `507fd196a7654ddd916218b2aa24a146eeaca4e1f7cb7daded4e7a9c689cda82` |
| `CME-SVC-2023-02-19` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-02-19&toEventDate=2023-02-21&isProtected&_t=1720455278640> | archive capture 2024-07-08T16:14:38Z | T2 | `d063238a83e8cb84d4484a86b26cb1d976f1ccfdb91972eecc08dcc0cec49412` |
| `CME-SVC-2023-04-06` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-04-06&toEventDate=2023-04-08&isProtected&_t=1720455278642> | archive capture 2024-07-08T16:14:38Z | T2 | `0543d5f6d2efd4aa6f4132ce9f5425b9ba9f64de08eccd677b78d6b43c6e9d4f` |
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
| `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230627125057id_/https://www.cmegroup.com/trading-hours/files/4th-of-july-2023.pdf> | archive capture 2023-06-27T12:50:57Z | T1 | `ccbc1f1fc39ba2219fd748372faa985798fee7eaffabf69f08956f5465a769e3` |
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-31&toEventDate=2025-01-02> | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-01-19&toEventDate=2025-01-21> | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-02-16&toEventDate=2025-02-18> | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-04-17&toEventDate=2025-04-19> | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-05-25&toEventDate=2025-05-27> | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-06-18&toEventDate=2025-06-20> | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-07-03&toEventDate=2025-07-05> | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-08-31&toEventDate=2025-09-02> | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | <https://web.archive.org/web/20260129012309id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-28> | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
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
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-22&toEventDate=2026-12-24> | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
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
Tier: **T1** for 2010-2012, 2013-2015, 2016-2018, 2019-2021 and the 2022-2024 rows
taken from the operator's own published holiday schedules; **T2** for the three 2023
markers and fourteen 2024 dates this table takes from its trading-hours service, and
for 2025-2027. Inside a window a date with no row is audited normal; outside every
window this table has no answer at all.

**Six audited eras, with no gap between them.** The table declares 6 coverage windows: `2010-01-01..2012-12-31`, `2013-01-01..2015-12-31`, `2016-01-01..2018-12-31`, `2019-01-01..2021-12-31`, `2022-01-01..2024-12-31`, `2025-01-01..2027-12-31`.
The rows of every era are the D17 intersection of the families routed here: an era's rows are the family tables' own where a venue routes one family, and the joint statement where it routes several.
Every interval from 2010-01-01 is declared, so the whole span has an answer; the eras before 2010 are out of scope below the crate's January-2010 floor.

### 2010

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `CME Globex is closed` | `2010-new-years.pdf @2010-02-15T05:16:52Z` | T1 | CME prints `Jan 1` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-01-15 | unsourced | - | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | the routed families disagree on this date |
| 2010-01-18 | unsourced | - | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | the routed families disagree on this date |
| 2010-02-12 | unsourced | - | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | the routed families disagree on this date |
| 2010-02-15 | unsourced | - | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | the routed families disagree on this date |
| 2010-04-02 | unsourced | - | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | the routed families disagree on this date |
| 2010-05-28 | unsourced | - | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | the routed families disagree on this date |
| 2010-05-31 | unsourced | - | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | the routed families disagree on this date |
| 2010-07-02 | unsourced | - | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | the routed families disagree on this date |
| 2010-07-05 | unsourced | - | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | the routed families disagree on this date |
| 2010-09-03 | unsourced | - | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | the routed families disagree on this date |
| 2010-09-06 | unsourced | - | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | the routed families disagree on this date |
| 2010-10-08 | unsourced | - | `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | T1 | the routed families disagree on this date |
| 2010-11-25 | unsourced | - | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | the routed families disagree on this date |
| 2010-11-26 | unsourced | - | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | the routed families disagree on this date |
| 2010-12-24 | closed | `CME Globex is closed` | `2010-christmas.pdf @2010-12-14T06:12:38Z` | T1 | CME prints `Dec 24` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-12-31 | unsourced | - | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | the routed families disagree on this date |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-14 | unsourced | - | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | the routed families disagree on this date |
| 2011-01-17 | unsourced | - | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | the routed families disagree on this date |
| 2011-02-18 | unsourced | - | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | the routed families disagree on this date |
| 2011-02-21 | unsourced | - | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | the routed families disagree on this date |
| 2011-04-21 | unsourced | - | `2011-good-friday.pdf @2011-10-28T02:37:07Z` | T1 | the routed families disagree on this date |
| 2011-04-22 | closed | `CME Globex is closed` | `2011-good-friday.pdf @2011-10-28T02:37:07Z` | T1 | CME prints `Apr 22` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-05-27 | unsourced | - | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | the routed families disagree on this date |
| 2011-05-30 | unsourced | - | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | the routed families disagree on this date |
| 2011-07-01 | unsourced | - | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | the routed families disagree on this date |
| 2011-07-04 | unsourced | - | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | the routed families disagree on this date |
| 2011-09-02 | unsourced | - | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | the routed families disagree on this date |
| 2011-09-05 | unsourced | - | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | the routed families disagree on this date |
| 2011-10-07 | unsourced | - | `2011-columbus-day.pdf @2011-11-01T14:39:16Z` | T1 | the routed families disagree on this date |
| 2011-11-24 | unsourced | - | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | the routed families disagree on this date |
| 2011-11-25 | unsourced | - | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | the routed families disagree on this date |
| 2011-12-26 | closed | `CME Globex is closed` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | CME prints `Dec 26` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-12-27 | unsourced | - | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | the routed families disagree on this date |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `CME Globex is closed` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | CME prints `Jan 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-01-03 | unsourced | - | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | the routed families disagree on this date |
| 2012-01-13 | unsourced | - | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | the routed families disagree on this date |
| 2012-01-16 | unsourced | - | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | the routed families disagree on this date |
| 2012-02-17 | unsourced | - | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | the routed families disagree on this date |
| 2012-02-20 | unsourced | - | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | the routed families disagree on this date |
| 2012-04-05 | unsourced | - | `2012-good-friday.pdf @2012-04-17T00:42:47Z` | T1 | the routed families disagree on this date |
| 2012-04-06 | unsourced | - | `2012-good-friday.pdf @2012-04-17T00:42:47Z` | T1 | the routed families disagree on this date |
| 2012-05-25 | unsourced | - | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | the routed families disagree on this date |
| 2012-05-28 | unsourced | - | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | the routed families disagree on this date |
| 2012-07-03 | unsourced | - | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | the routed families disagree on this date |
| 2012-07-04 | unsourced | - | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | the routed families disagree on this date |
| 2012-07-05 | unsourced | - | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | the routed families disagree on this date |
| 2012-08-31 | unsourced | - | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | the routed families disagree on this date |
| 2012-09-03 | unsourced | - | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | the routed families disagree on this date |
| 2012-10-05 | unsourced | - | `2012-columbus-day.pdf @2012-09-15T00:15:14Z` | T1 | the routed families disagree on this date |
| 2012-11-22 | unsourced | - | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | the routed families disagree on this date |
| 2012-11-23 | unsourced | - | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | the routed families disagree on this date |
| 2012-12-24 | unsourced | - | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | the routed families disagree on this date |
| 2012-12-25 | closed | `CME Globex is closed` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | CME prints `Dec 25` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-12-26 | unsourced | - | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | the routed families disagree on this date |

### 2016

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `every routed family states a closure` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | the intersection of the families routed to this venue |
| 2016-01-18 | unsourced | `—` | `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-02-15 | unsourced | `—` | `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-03-25 | closed | `every routed family states a closure` | `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-05-30 | unsourced | `—` | `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-07-04 | unsourced | `—` | `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-09-05 | unsourced | `—` | `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-11-24 | unsourced | `—` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-11-25 | unsourced | `—` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-12-23 | unsourced | `—` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-12-26 | closed | `every routed family states a closure` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |

### 2017

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `every routed family states a closure` | `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2017-01-16 | unsourced | `—` | `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2017-02-20 | unsourced | `—` | `2017-presidents-day-holiday-schedule.xls @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2017-04-14 | closed | `every routed family states a closure` | `2017-good-friday-holiday-schedule.xls @2017-05-05` | T1 | the intersection of the families routed to this venue |
| 2017-05-29 | unsourced | `—` | `2017-memorial-day-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-07-03 | unsourced | `—` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-07-04 | unsourced | `—` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-09-04 | unsourced | `—` | `2017-labor-day-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-11-23 | unsourced | `—` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |
| 2017-11-24 | unsourced | `—` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |
| 2017-12-22 | unsourced | `—` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |
| 2017-12-25 | closed | `every routed family states a closure` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |

### 2018

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `every routed family states a closure` | `2018-new-years-holiday-schedule.xls @2018-01-06` | T1 | the intersection of the families routed to this venue |
| 2018-01-15 | unsourced | `—` | `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | T1 | the intersection of the families routed to this venue |
| 2018-02-19 | unsourced | `—` | `2018-presidents-day-holiday-schedule.xls @2018-05-08` | T1 | the intersection of the families routed to this venue |
| 2018-03-30 | closed | `every routed family states a closure` | `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-05-28 | unsourced | `—` | `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-07-03 | unsourced | `—` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-07-04 | unsourced | `—` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-09-03 | unsourced | `—` | `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-11-22 | unsourced | `—` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-11-23 | unsourced | `—` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-12-24 | unsourced | `—` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-12-25 | closed | `every routed family states a closure` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-12-26 | unsourced | `—` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |


### 2013-2015 (T1)
**This era brings the venue to six audited windows.** The table as a whole carries 276 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **58 rows**: 8 stated rows and 50 `Unsourced` rows. Every row is the intersection of the families routed here — `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` — by the D17 rule `venues.rs` states: a row ships only where every routed family states the same one, and a date on which they differ, or on which one states a row while another has audited the date normal, ships `Unsourced`; the three year tables below carry every date's row, its instant as printed and the id it is derived from.

**Columbus Day 2013 and Veterans Day 2014 and 2015 carry no row.** CME's own sheets for those three dates state in session language that Globex ran a normal schedule — `Products listed on Globex are unaffected and will run on a normal schedule` for 2013-10-14 (`2013-columbus-day.pdf @2012-11-19T00:15:54Z`) and `Regular CME Globex trading hours will be in effect` for 2014-11-11 and 2015-11-11 (`2014-veterans-day-holiday-schedule.pdf @2014-11-13T19:34:50Z` and `2015-veterans-day-schedule.pdf @2015-11-22T23:09:20Z`) — so the block records each as `normal` and this table ships nothing: inside a declared window silence is the positive claim that the date was audited normal, and these dates are audited rather than skipped. The three sheets have no row of their own to cite, so they are listed in this era's `### Documents` table without being any row's document.

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

### 2013

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `every routed family states a closure` | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | the intersection of the families routed to this venue |
| 2013-01-02 | unsourced | `—` | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | the intersection of the families routed to this venue |
| 2013-01-18 | unsourced | `—` | `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | T1 | the intersection of the families routed to this venue |
| 2013-01-21 | unsourced | `—` | `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | T1 | the intersection of the families routed to this venue |
| 2013-02-15 | unsourced | `—` | `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | T1 | the intersection of the families routed to this venue |
| 2013-02-18 | unsourced | `—` | `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | T1 | the intersection of the families routed to this venue |
| 2013-03-28 | unsourced | `—` | `2013-good-friday.pdf @2013-06-23T19:59:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-03-29 | closed | `every routed family states a closure` | `2013-good-friday.pdf @2013-06-23T19:59:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-05-24 | unsourced | `—` | `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | T1 | the intersection of the families routed to this venue |
| 2013-05-27 | unsourced | `—` | `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | T1 | the intersection of the families routed to this venue |
| 2013-07-03 | unsourced | `—` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-07-04 | unsourced | `—` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-07-05 | unsourced | `—` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-08-30 | unsourced | `—` | `2013-labor-day.pdf @2013-09-02T17:08:41Z` | T1 | the intersection of the families routed to this venue |
| 2013-09-02 | unsourced | `—` | `2013-labor-day.pdf @2013-09-02T17:08:41Z` | T1 | the intersection of the families routed to this venue |
| 2013-11-28 | unsourced | `—` | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the intersection of the families routed to this venue |
| 2013-11-29 | unsourced | `—` | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the intersection of the families routed to this venue |
| 2013-12-24 | unsourced | `—` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the intersection of the families routed to this venue |
| 2013-12-25 | closed | `every routed family states a closure` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the intersection of the families routed to this venue |
| 2013-12-26 | unsourced | `—` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the intersection of the families routed to this venue |

### 2014

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `every routed family states a closure` | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | the intersection of the families routed to this venue |
| 2014-01-02 | unsourced | `—` | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | the intersection of the families routed to this venue |
| 2014-01-17 | unsourced | `—` | `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | T1 | the intersection of the families routed to this venue |
| 2014-01-20 | unsourced | `—` | `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | T1 | the intersection of the families routed to this venue |
| 2014-02-14 | unsourced | `—` | `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | T1 | the intersection of the families routed to this venue |
| 2014-02-17 | unsourced | `—` | `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | T1 | the intersection of the families routed to this venue |
| 2014-04-17 | unsourced | `—` | `2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z` | T1 | the intersection of the families routed to this venue |
| 2014-04-18 | closed | `every routed family states a closure` | `2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z` | T1 | the intersection of the families routed to this venue |
| 2014-05-23 | unsourced | `—` | `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | T1 | the intersection of the families routed to this venue |
| 2014-05-26 | unsourced | `—` | `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | T1 | the intersection of the families routed to this venue |
| 2014-07-03 | unsourced | `—` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the intersection of the families routed to this venue |
| 2014-07-04 | unsourced | `—` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the intersection of the families routed to this venue |
| 2014-07-07 | unsourced | `—` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the intersection of the families routed to this venue |
| 2014-08-29 | unsourced | `—` | `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | T1 | the intersection of the families routed to this venue |
| 2014-09-01 | unsourced | `—` | `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | T1 | the intersection of the families routed to this venue |
| 2014-11-27 | unsourced | `—` | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the intersection of the families routed to this venue |
| 2014-11-28 | unsourced | `—` | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the intersection of the families routed to this venue |
| 2014-12-24 | unsourced | `—` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the intersection of the families routed to this venue |
| 2014-12-25 | closed | `every routed family states a closure` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the intersection of the families routed to this venue |
| 2014-12-26 | unsourced | `—` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the intersection of the families routed to this venue |

### 2015

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `every routed family states a closure` | `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | T1 | the intersection of the families routed to this venue |
| 2015-01-02 | unsourced | `—` | `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | T1 | the intersection of the families routed to this venue |
| 2015-01-16 | unsourced | `—` | `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | T1 | the intersection of the families routed to this venue |
| 2015-01-19 | unsourced | `—` | `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | T1 | the intersection of the families routed to this venue |
| 2015-02-13 | unsourced | `—` | `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | T1 | the intersection of the families routed to this venue |
| 2015-02-16 | unsourced | `—` | `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | T1 | the intersection of the families routed to this venue |
| 2015-04-02 | unsourced | `—` | `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | T1 | the intersection of the families routed to this venue |
| 2015-04-03 | unsourced | `—` | `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | T1 | the intersection of the families routed to this venue |
| 2015-05-22 | unsourced | `—` | `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | T1 | the intersection of the families routed to this venue |
| 2015-05-25 | unsourced | `—` | `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | T1 | the intersection of the families routed to this venue |
| 2015-07-02 | unsourced | `—` | `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | T1 | the intersection of the families routed to this venue |
| 2015-07-03 | unsourced | `—` | `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | T1 | the intersection of the families routed to this venue |
| 2015-09-07 | unsourced | `—` | `2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z` | T1 | the intersection of the families routed to this venue |
| 2015-11-26 | unsourced | `—` | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the intersection of the families routed to this venue |
| 2015-11-27 | unsourced | `—` | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the intersection of the families routed to this venue |
| 2015-12-24 | unsourced | `—` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | the intersection of the families routed to this venue |
| 2015-12-25 | closed | `every routed family states a closure` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | the intersection of the families routed to this venue |
| 2015-12-31 | unsourced | `—` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | the intersection of the families routed to this venue |

### 2019-2021 (T1)
**This era brought the venue to five audited windows; the 2013-2015 wave has since added a sixth.** The table as a whole carries 276 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **42 rows**: 8 stated closures and 34 `Unsourced` rows. Every row is the intersection of the families routed here — `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates` and `globex_livestock` — by the D17 rule `venues.rs` states: a closure ships only where every routed family states the same one, and a date on which they differ, or on which one states a row while another has audited the date normal, ships `Unsourced`, with the disagreement named per date below. `comex` and `nymex` route `globex_energy` alone, so their rows are that family's own, unchanged. The three Juneteenth dates ship `Unsourced` because every routed family states the crate's not-worked-up marker there.

### Documents
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

### 2019
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `every routed family states a closure` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | the intersection of the families routed to this venue |
| 2019-01-02 | unsourced | `—` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2019-01-21 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2019-02-18 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2019-04-19 | closed | `every routed family states a closure` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | T1 | the intersection of the families routed to this venue |
| 2019-05-27 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2019-06-19 | unsourced | `—` | `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | T1 | equity index states `Unsourced` |
| 2019-07-03 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:15 CT; energy and metals no row; FX no row; grains early close 12:05 CT; interest rates no row; livestock early close 12:15 CT |
| 2019-07-04 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2019-07-05 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2019-09-02 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2019-11-28 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2019-11-29 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:15 CT |
| 2019-12-24 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:15 CT |
| 2019-12-25 | closed | `every routed family states a closure` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | the intersection of the families routed to this venue |
| 2019-12-26 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
### 2020
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `every routed family states a closure` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | the intersection of the families routed to this venue |
| 2020-01-02 | unsourced | `—` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2020-01-20 | unsourced | `—` | `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2020-02-17 | unsourced | `—` | `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2020-04-10 | closed | `every routed family states a closure` | `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | T1 | the intersection of the families routed to this venue |
| 2020-05-25 | unsourced | `—` | `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2020-06-19 | unsourced | `—` | `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | T1 | equity index states `Unsourced` |
| 2020-07-02 | unsourced | `—` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index no row; energy and metals no row; FX no row; grains early close 12:05 CT; interest rates no row; livestock early close 12:15 CT |
| 2020-07-03 | unsourced | `—` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2020-09-07 | unsourced | `—` | `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2020-11-26 | unsourced | `—` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2020-11-27 | unsourced | `—` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2020-12-24 | unsourced | `—` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2020-12-25 | closed | `every routed family states a closure` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | the intersection of the families routed to this venue |
### 2021
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `every routed family states a closure` | `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | the intersection of the families routed to this venue |
| 2021-01-18 | unsourced | `—` | `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2021-02-15 | unsourced | `—` | `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2021-04-02 | unsourced | `—` | `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 08:15 CT; energy and metals closed; FX early close 10:15 CT; grains closed; interest rates early close 10:15 CT; livestock closed |
| 2021-05-31 | unsourced | `—` | `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2021-06-19 | unsourced | `—` | `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | T1 | equity index states `Unsourced` |
| 2021-07-05 | unsourced | `—` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2021-07-06 | unsourced | `—` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2021-09-06 | unsourced | `—` | `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2021-11-25 | unsourced | `—` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:00 CT; energy and metals early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2021-11-26 | unsourced | `—` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2021-12-24 | closed | `every routed family states a closure` | `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | the intersection of the families routed to this venue |
### 2022-2024 (T1/T2)

**Counts in this subsection are the 2022-2024 era's.** The table as a whole carries 276 rows over
its six audited windows: 55 in 2010-2012 (6 stated closures and 49 `Unsourced`), 58 in 2013-2015
(8 stated closures and 50 `Unsourced`), 36 in 2016-2018 (9 stated closures and 27 `Unsourced`), 42
in 2019-2021 (8 stated closures and 34 `Unsourced`), 41 in this era (7 stated closures and 34
`Unsourced`) and 44 in 2025-2027 (9 stated closures and 35 `Unsourced`). The derivation rule is the
same in all six; only the documents differ.

**This era's rows are derived, not retrieved.** They are the **intersection** of
the same six CME product families, and every id they cite is one a routed family's
own row on that date cites. The per-family rows, with their
event-date-to-trade-date conversions, are in the six key evidence files; this
section records the derivation, every date the intersection drops, and why.

**Seven closures, thirty-four withheld dates.** Seven dates are shut in every
routed family and ship `closed`: 2022-04-15 and 2022-12-26, 2023-01-02 and
2023-12-25, and 2024-01-01, 2024-03-29 and 2024-12-25. The other thirty-four
dates carry `unsourced`, and the `Derived from` column above prints each routed
family's own answer on each of them. They fall into four shapes:

- **Nineteen Monday and Thursday holidays**: 2022-01-17, 2022-02-21, 2022-05-30,
  2022-06-20, 2022-07-04, 2022-09-05, 2022-11-24; 2023-05-29, 2023-06-19,
  2023-07-04, 2023-09-04, 2023-11-23; and 2024-01-15, 2024-02-19, 2024-05-27,
  2024-06-19, 2024-07-04, 2024-09-02, 2024-11-28. The equity-index and
  interest-rate families halt at 12:00 CT and energy and metals at 13:30 CT,
  `globex_grains` and `globex_livestock` are **closed outright**, and `globex_fx`
  carries no row: its halt falls at its ordinary 16:00 CT final close, which its own
  table does not state as a holiday row, so it has audited the date normal.
- **Four half-days**: 2022-11-25, 2023-11-24, 2024-11-29 and 2024-12-24. The grain
  and livestock day sessions end at 12:05 CT (grains after an 08:30 CT late open on
  the three Fridays) and the equity-index and interest-rate families halt at
  12:15 CT; energy and metals close at 12:45 CT, and at 13:45 CT on 2024-11-29. FX
  closes at 12:15 CT on the three Fridays and at 12:45 CT on 2024-12-24, when
  livestock ends at 12:15 CT rather than at 12:05 CT.
- **Eight one-family dates**: `globex_grains` states a late open at 08:30 CT on
  2022-07-05, 2023-07-05, 2023-12-26, 2024-01-02, 2024-07-05 and 2024-12-26, and
  `globex_equity_index` an early close at 12:15 CT on 2023-07-03 and 2024-07-03.
  Every other routed family audited those dates normal, which is an answer and
  disputes the row.
- **Three dates every routed family marks `Unsourced`**: 2023-01-16, 2023-02-20 and
  2023-04-07. The wave retrieved no document for those three 2023 sheets, so all six
  families state the crate's not-worked-up marker and the venue repeats the answer
  they agree on. It is neither a dispute nor silence.

**Why no instant stands for the disputed dates.** Each one is a date on which two
families state different boundaries, or one states a boundary while another has
**audited the date normal** — and the second shape blocks a venue row just as
firmly, because an audited normal is a different answer rather than a missing one.
`unsourced` clips nothing and changes no answer; it tells the caller the date is
special and that this venue deliberately does not claim to know what the building
did. A caller with a product routes the question through the family key.

### 2022
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-01-17 | unsourced | `—` | `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-02-21 | unsourced | `—` | `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-04-15 | closed | `every routed family states a closure` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | the intersection of the families routed to this venue |
| 2022-05-30 | unsourced | `—` | `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-06-20 | unsourced | `—` | `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-07-04 | unsourced | `—` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-07-05 | unsourced | `—` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2022-09-05 | unsourced | `—` | `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-11-24 | unsourced | `—` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2022-11-25 | unsourced | `—` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2022-12-26 | closed | `every routed family states a closure` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | the intersection of the families routed to this venue |

### 2023
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `every routed family states a closure` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | the intersection of the families routed to this venue |
| 2023-01-16 | unsourced | `—` | `CME-SVC-2023-01-15` | T2 | equity index states `Unsourced`: the wave retrieved no document for this date |
| 2023-02-20 | unsourced | `—` | `CME-SVC-2023-02-19` | T2 | equity index states `Unsourced`: the wave retrieved no document for this date |
| 2023-04-07 | unsourced | `—` | `CME-SVC-2023-04-06` | T2 | equity index states `Unsourced`: the wave retrieved no document for this date |
| 2023-05-29 | unsourced | `—` | `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2023-06-19 | unsourced | `—` | `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2023-07-03 | unsourced | `—` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | equity index early close 12:15 CT; energy and metals no row; FX no row; grains no row; interest rates no row; livestock no row |
| 2023-07-04 | unsourced | `—` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2023-07-05 | unsourced | `—` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2023-09-04 | unsourced | `—` | `labor-day-2023.pdf @2023-08-02T19:24:46Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2023-11-23 | unsourced | `—` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2023-11-24 | unsourced | `—` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:15 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2023-12-25 | closed | `every routed family states a closure` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | the intersection of the families routed to this venue |
| 2023-12-26 | unsourced | `—` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |

### 2024
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `every routed family states a closure` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | the intersection of the families routed to this venue |
| 2024-01-02 | unsourced | `—` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2024-01-15 | unsourced | `—` | `CME-SVC-2024-01-14` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-02-19 | unsourced | `—` | `CME-SVC-2024-02-18` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-03-29 | closed | `every routed family states a closure` | `CME-SVC-2024-03-28` | T2 | the intersection of the families routed to this venue |
| 2024-05-27 | unsourced | `—` | `CME-SVC-2024-05-26` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-06-19 | unsourced | `—` | `CME-SVC-2024-06-18` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-07-03 | unsourced | `—` | `CME-SVC-2024-07-03` | T2 | equity index early close 12:15 CT; energy and metals no row; FX no row; grains no row; interest rates no row; livestock no row |
| 2024-07-04 | unsourced | `—` | `CME-SVC-2024-07-03` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-07-05 | unsourced | `—` | `CME-SVC-2024-07-03` | T2 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |
| 2024-09-02 | unsourced | `—` | `CME-SVC-2024-09-01` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-11-28 | unsourced | `—` | `CME-SVC-2024-11-27` | T2 | equity index early close 12:00 CT; energy and metals early close 13:30 CT; FX no row; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2024-11-29 | unsourced | `—` | `CME-SVC-2024-11-27` | T2 | equity index early close 12:15 CT; energy and metals early close 13:45 CT; FX early close 12:15 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2024-12-24 | unsourced | `—` | `CME-SVC-2024-12-24` | T2 | equity index early close 12:15 CT; energy and metals early close 12:45 CT; FX early close 12:45 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:15 CT |
| 2024-12-25 | closed | `every routed family states a closure` | `CME-SVC-2024-12-24` | T2 | the intersection of the families routed to this venue |
| 2024-12-26 | unsourced | `—` | `CME-SVC-2024-12-24` | T2 | equity index no row; energy and metals no row; FX no row; grains late open 08:30 CT; interest rates no row; livestock no row |

### 2025-2027 (T2)

**Counts in this subsection are the 2025-2027 era's.** The table as a whole carries 276 rows over
its six audited windows: 55 in 2010-2012 (6 stated closures and 49 `Unsourced`), 58 in 2013-2015
(8 stated closures and 50 `Unsourced`), 36 in 2016-2018 (9 stated closures and 27 `Unsourced`), 42
in 2019-2021 (8 stated closures and 34 `Unsourced`), 41 in 2022-2024 (7 stated closures and 34
`Unsourced`) and 44 in this era (9 stated closures and 35 `Unsourced`). The derivation rule is the
same in all six; only the documents differ.

**This table is derived, not retrieved.** It is the **intersection** of the holiday
tables the crate ships for the six CME product families that route to `Exchange::Cme`
— `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`,
`globex_interest_rates` and `globex_livestock` — and every id it cites resolves to the
same CME Group `trading-hours-by-product` artifact those families' evidence files
already record. There is no venue evidence of its own to add here, and nothing in the
venue module rests on a document a family module does not carry. The per-family rows,
with their event-date-to-trade-date conversions, are in the six key evidence files;
this section records the derivation, the routing, every date the intersection drops,
and the interpretive steps that produced both.

**The intersection rule (design memo D17).** A `MarketHoursKey` holiday row is a
statement about one family's clock, and the six families do not share one. On the
Monday holidays the equity-index and interest-rate families halt matching at 12:00 CT
while the grain and livestock families are **closed outright**; on the Friday holidays
the equity-index and interest-rate closes move to 12:15 CT, energy and FX to 13:45 CT,
grains to 12:05 CT and livestock to 12:05 CT; on Christmas Eve 2026 the same date
carries 12:05, 12:15 and 12:45 CT closes at once. A venue row may therefore be stated
only where every routed family states the same row. **Nine dates** qualify in this
window — the Globex full closures — and the other **thirty-five** dates carry
`unsourced`.

**`unsourced` is neither silence nor a compromise.** The coverage window is
contiguous **within each audited window**, so a date carrying no row there is the
positive claim that it was audited normal, which is false on every one of these
dates. Nor can the venue state an
instant. A venue row would have to pick one, and on these dates there is no single
one to pick, in either of two shapes: two families state **different boundaries**, or
one family states a boundary while another has **audited the date normal** — and the
second shape blocks a venue row just as firmly, because an audited normal is a
different answer rather than a missing one.

The shapes are counted from the six tables rather than assumed. **Ten** of the
thirty-five dates have all six families stating a boundary that they do not agree on.
**Twenty-five** have at least one family silent, and eight of those are the sharpest
case, where exactly **one** family states anything at all: 2025-01-02, 2025-12-26,
2026-01-02 and 2027-07-06, where only `globex_grains` states a late open,
2025-07-03, where only `globex_equity_index` states an early close, and 2026-06-22,
2026-07-06 and 2027-06-21, where only `globex_energy` states the Saturday-session
block. The other seventeen are a mixture in between. Here is what the boundary
disagreements look like across the three shapes of holiday, which is why no single
instant can stand for one.

**The Christmas Eves are the widest** (a ten-date shape): on 2026-12-24 the grain and
livestock day sessions end at 12:05 CT, the equity-index and interest-rate families at
12:15 CT, and energy and FX at 12:45 CT — a forty-minute spread. **The Friday holidays
are nearly as wide** (again all six, except that grains and livestock carry a late
open as well): on the three days after Thanksgiving the grain and livestock sessions
end at 12:05 CT, the equity-index and interest-rate families at 12:15 CT, and energy
and FX at 13:45 CT. **The Monday holidays disagree in kind rather than by minutes**
(these are the five-silent shape — `globex_fx` states nothing): the equity-index and
interest-rate families halt at 12:00 CT, energy at 13:30 CT, and grains and livestock
are closed outright. A venue row at the shallowest close would stop trading the
families that run later; one at the deepest would run the families that stop earlier
past their own sourced close.

`unsourced` clips nothing, changes no answer, and tells
a caller what the crate knows: the date is special and the venue has no single answer
for it. `iceus`, whose venue table shipped first, is the precedent.

**Cite the family, not the venue, for holiday behaviour.** These thirty-five rows are
the reason the consumer contract tells a caller to route holiday questions through
the product-family key: `globex_equity_index` knows what CME equity index does on
2026-12-24 and this venue deliberately does not claim to.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `no events published` | `CME-SVC-2024-12-31` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-01-02 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2024-12-31` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |
| 2025-01-20 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-01-19` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-02-17 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-02-16` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-05-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-05-25` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-06-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-06-18` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-07-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-07-03` | T2 | equity index early close 12:15 CT; no row in energy, FX, grains, interest rates, livestock |
| 2025-07-04 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-07-03` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2025-09-01 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-08-31` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-11-27 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-11-26-SAT` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-11-28 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-11-26` | T2 | equity index early close 12:15 CT; energy early close 13:45 CT; FX early close 13:45 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-12-24 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-24` | T2 | equity index early close 12:15 CT; energy early close 12:45 CT; FX early close 12:45 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:15 CT |
| 2025-12-25 | closed | `no events published` | `CME-SVC-2025-12-24` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-12-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-24` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-2025-12-31` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2026-01-02 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-31` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |
| 2026-01-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-01-18` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-02-16 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-02-15` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-04-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-04-01` | T2 | equity index early close 08:15 CT; energy closed; FX early close 10:15 CT; grains closed; interest rates early close 10:15 CT; livestock closed |
| 2026-05-25 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-05-24` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-06-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-06-18` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2026-06-22 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-06-18` | T2 | energy replacement blocks: Saturday 05:00-17:00 CT, Sunday Pre-Open 16:00-17:00 CT and the Sunday-17:00-to-Monday-16:00 session; no row in the five financial families |
| 2026-07-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-07-03` | T2 | energy replacement blocks: Saturday 05:00-17:00 CT, Sunday Pre-Open 16:00-17:00 CT and the Sunday-17:00-to-Monday-16:00 session; no row in the five financial families |
| 2026-07-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-07-03` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2026-09-07 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-09-06` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-11-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-11-25` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-11-27 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-11-25` | T2 | equity index early close 12:15 CT; energy early close 13:45 CT; FX early close 13:45 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2026-12-24 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-12-22` | T2 | equity index early close 12:15 CT; energy early close 12:45 CT; FX early close 12:45 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2027-01-18 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-01-17` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-02-15 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-02-14` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2027-05-31 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-05-30` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-06-18 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-06-17` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2027-06-21 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-06-17` | T2 | energy replacement blocks: Saturday 05:00-17:00 CT, Sunday Pre-Open 16:00-17:00 CT and the Sunday-17:00-to-Monday-16:00 session; no row in the five financial families |
| 2027-07-05 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-07-04` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 13:30 CT; livestock closed; no row in FX |
| 2027-07-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-07-04` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |
| 2027-09-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-09-05` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-11-25 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-11-24` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-11-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-11-24` | T2 | equity index early close 12:15 CT; energy early close 13:45 CT; FX early close 13:45 CT; grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |

**Gaps, 2025-2027.**

- **The thirty-five `unsourced` dates are the intersection's residue, not a research
  gap.** Each one is a date on which the six families disagree — by stating different
  rows, or by one of them stating a row while another states none. A family with no
  row has **audited the date normal**, which is a different answer rather than a
  missing one. The disagreement is printed per date in the `Derived from` column
  above. Every underlying row is sourced; what is missing is a single venue-wide
  answer, which no operator document states and which the crate will not invent. Closing
  condition: an operator statement of venue-wide holiday hours — CME's holiday-hours
  table on `cmegroup.com/trading-hours.html` is per asset class, not per venue — or a
  `DayPolicy`-shaped boundary that can express a per-family answer inside one venue
  calendar. Neither exists today.
- **`no row in FX` on twenty-two of the older thirty-two disputed dates, plus the
  three 2026-2027 Saturday-session dates it now states.** `globex_fx`
  carries no row on 2025-01-02, 2025-01-20, 2025-02-17, 2025-05-26, 2025-06-19,
  2025-07-03, 2025-09-01, 2025-11-27, 2025-12-26, 2026-01-02, 2026-01-19, 2026-02-16,
  2026-05-25, 2026-09-07, 2026-11-26, 2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05,
  2027-07-06, 2027-09-06 and 2027-11-25, while at least one of the other five families
  states a row on each of them. FX's own evidence file audits those dates normal: CME
  prints `16:00 closed` for `6E` as the family's **ordinary** final close, not a holiday
  halt. That is a row the FX family deliberately does not carry, so the venue sees a
  disagreement where the families may in fact agree about trading. It is recorded here
  rather than resolved because resolving it would mean the venue second-guessing a
  family's own audited normal, which is exactly what D17 forbids. Closing condition: an
  operator statement that FX's 16:00 CT close on those dates is a holiday halt rather
  than the ordinary grid, which would let `globex_fx` carry a row.
- **The four single-family `late open` dates.** 2025-01-02, 2025-12-26, 2026-01-02 and
  2027-07-06 carry a `globex_grains` late open at 08:30 CT and nothing else. The other
  five families run a continuous overnight leg through those dates, so for them no
  boundary moves and no row exists. The venue therefore cannot state the grain late
  open as its own. It is also inert for this venue calendar, whose profile already opens
  its day session at 08:30 CT; it is recorded because a venue row must be true of every
  routed family, not only of the one that states it.
- **The window is the families' window, not the venue's horizon.** This subsection
  covers 2025-2027 because that is the era the trading-hours service reaches, not
  because this venue has no history: the earlier eras are above, and the 2013-2015
  and 2019-2021 blocks are the remaining stage-2.2 waves. Each wave extends this
  table over its own years by the same derivation, and the cross-wave agreement audit
  (#95) closes with the last of them. The venue's *schedule* rows still reach the
  January-2010 floor; the two are different claims, and outside the holiday window the
  crate answers the normal week correctly and has no holiday answer at all.
- **2028-01-01 ships no row.** CME's service publishes a 2027-12-30 .. 2028-01-02
  window and every family's table ends at 2027-12-31, so the venue's window ends
  there too. Extending it is stage 2.4's refresh, not a gap in this change.

**Interpretive steps, 2025-2027.**

- **The family list is a decision this crate cannot derive, and it is recorded as
  such.** `hours_for_exchange`'s `Exchange::Cme` arm resolves to `cme_profile_at`,
  which is a schedule and not a membership list: the crate holds no map from product
  families to venues, because by the consumer contract that map belongs to
  SharurPlatform. The six families above are therefore the plan's Wave 7 decision
  ("`cme` = the intersection of the six families that route to it"), and the file at
  `docs/plans/2026-09-12-path-to-release.md` §3 2.1 is where it is written down.
  Eight CME families ship tables; the two left out are `globex_cryptocurrency` and
  `globex_nikkei_225_dollar`.
- **What adding each of the two would cost is known, and they differ.** Nikkei agrees
  with `globex_equity_index` on all thirty-seven dates in this window, so adding it
  would change nothing at all. Cryptocurrency shares the same **nine** closures — its
  `Closed` rows cover every one of them — so it would not touch the nine stated rows.
  What it would change is the residue: it ships twenty-four rows against this list's
  forty-four dates, because it trades 24/7 and states no closure on the other
  twenty, so on each of those twenty it has **audited the date normal** while
  another routed family states a closure or an early close. Those twenty dates
  carry `unsourced` either way, so the venue's answers would be unchanged and only the
  evidence would record one more family as party to each disagreement. The choice is
  therefore not load-bearing for any answer in this window, which is exactly why it
  has to be written down rather than inferred.
  Closing condition: the consumer's own root map, disclosed and reviewed against this
  list — the audit `#95` carries into the stage-2.2 waves. The crate cannot settle it,
  and this file states both readings so a reviewer can.
- **A disagreement ships `unsourced` rather than nothing.** This is the one place the
  module departs from D17's literal "ships no venue row", and it follows `iceus`,
  which shipped first and states the reason: with a contiguous coverage window,
  silence is a claim. The row is cited to a document the crate holds, because the
  `holidays!` fence requires a non-empty id and because the id is what lets a reader
  reach the bytes behind the disagreement.
- **The venue's normal week is unchanged.** Every row here is a clip on a schedule
  `hours_for_exchange(Exchange::Cme, _)` already serves; no row creates a session the
  profile does not have, and `unsourced` rows do not touch the schedule at all. The
  golden normal-week grids are therefore untouched by this change.
- **Saturday 2025-11-29 is the only weekend closure in the intersection.** It is a
  full closure in all six families and ships as `closed`. The `cme` profile has no
  Saturday session to remove, so the row changes no answer for this venue; it ships
  because the operator answered, exactly as the family table records.

### The cross-wave D17 audit (#95)

**The intersection is re-derived over every audited era, not only this wave's.** `tools/check_wave5.py` recomputes the D17 rule over 2010-01-01..2027-12-31 — every date on which at least one routed family declares a window — and compares the result with the tables above date by date: **276 rows over the six windows**, 47 the routed families state `Closed` and 229 withheld as `Unsourced`. This is the agreement audit memo §7 follow-up 10 asks for (#95).

**Abstention and audited-normal silence are both states the audit reads.** A family with no window on a date abstains and the families that cover it decide: `globex_livestock` has no 2016-2018 table, so the other five decide that era's 36 dates — 9 closures and 27 withheld. Inside a window a family that states no row has audited the date normal, which is not the same state as silence: a date one family states and another audited normal is a disagreement and ships `Unsourced`. Of the 229 rows withheld, **223 are disagreements of that kind and 6 are dates every routed family itself marks not worked up** — 2019-06-19, 2020-06-19, 2021-06-19, 2023-01-16, 2023-02-20 and 2023-04-07 — which are the families' own marker rather than a dispute. **The reading this audit uses for an `Unsourced` family never decides a row.** A routed family's `Unsourced` marker appears only where every routed family states one — the six agreed markers above — so the venue ships `Unsourced` under either reading, and there is **no date in any of the four venue tables on which a routed family marks a date while another routed family states a row** (recomputed: 0). The shapes that look like counterexamples are not: 2024-03-29 and 2024-12-25 have all six routed families stating `Closed` (the `Unsourced` marker printed beside them belongs to `globex_nikkei_225_dollar`, which `venues.rs` excludes from this venue's routing), 2025-01-02 is the ordinary disagreement — `globex_grains` states a late open and the other five are silent — and on 2024-12-31 all six are silent, so the table ships nothing. `Exchange::Cme` applies one reading, not two.

This era's 50 withheld rows carry the intersection cell in the year tables below; the per-date disagreements are named in `venues/cme.rs`'s row comments and in the 2019-2021 and 2022-2024 tables.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf> — CME `EQ240` equity-index education module, the October-2009 product guide that supplies the complete audit-floor grid.
- <https://www.cmegroup.com/education/files/eq-trading-hours.pdf> — CME equity-index trading-hours sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html> — CME clearing advisory Chadv12-423, the 2012 trade-date boundary change.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html> — CME Globex notice 20121022, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html> — CME market-data advisory 20121015, corroborating the 2012 change.
- <https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf> — CME clearing advisory Chadv19-182.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 weekday Pre-Open move.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html> — CME Globex Notice #20150914, the "Effective this Monday" repeat of the same article.
- <https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html> — CME Globex notice 20210621, the 2021-06-27 halt removal.
- <https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf> — CME rule filing 21-244R, corroborating the halt removal.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-05-03, Sunday Pre-Open still 16:15.
- <https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open.

Official origin of the four trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — full-size `SP`, NKD, BTIC and TACO products are excluded from this compatibility default; NKD has its own key and module.
- **holidays** — this venue ships the intersection of the six CME families'
  holiday tables; see the `## Holidays` section above. Every date the
  intersection drops is named there, and the cross-wave agreement audit that
  memo section 7 follow-up 10 asks for (#95) is recorded in *The cross-wave D17
  audit (#95)* above, where the same assertion is recomputed over 2010-2027.

## Module narrative (moved from src/calendar/schedules/futures/us/cme_group.rs on 2026-09-12 UTC)

U.S.-grid CME and CBOT equity-index futures, including CBOT YM/MYM but not
CME Nikkei 225 Dollar (NKD), whose historical grid differs. CME's
October-2009 product guide supplies the complete grid at the audit floor:
Sunday 17:00–Monday 15:15, then Monday–Thursday 17:00–15:15 and
15:30–16:30, with 16:30–17:00 maintenance. The 2012 notice changed the
trade-date boundary and the post-halt slice to 15:30–16:15, including
Fridays, effective Sunday 2012-11-18. CME Globex then moved that close
15 minutes earlier to 16:00 CT effective Sunday 2015-09-20 for trade date
Monday 2015-09-21. CME then removed the 15:15-15:30 halt for the scoped
contracts effective Sunday 2021-06-27, producing the current continuous
17:00-16:00 ETH envelope around the unchanged 08:30-15:15 RTH.

The exact Monday-Thursday Pre-Open changed from 16:50 to 16:45 on
2010-11-15. Current primary material also establishes Sunday 16:00-17:00,
but calls it a long-term practice without giving the day when the earlier
16:15 start moved: primary documents updated 2012-05-03 still publish
Sunday 16:15, trading-hours pages crawled 2012-06-15/16 already publish
16:00, and no notice in between states the day. The fixed-current table
includes that sourced current queue. Dated profiles carry the sourced
Sunday 16:15–17:00 intersection from the January-2010 floor and withhold
only the disputed 16:00–16:15 quarter-hour rather than inventing its
cutover; their executable trading and weekday queues remain exact. Revisions are keyed by the local
session-opening day.
https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf
https://www.cmegroup.com/education/files/eq-trading-hours.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html
https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf
https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html

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
https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html
The 2015-09-20 revision's original announcement, CME Globex Notice #20150817
of 17 August 2015: "Effective Monday, September 21, the daily CME Globex
maintenance period will begin 15 minutes earlier Monday through Thursday from
16:00 until 16:45 Central Time (CT). ... the closing times for the following
markets will now occur 15 minutes earlier Monday through Friday at 16:00 CT.
CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME Globex markets
trading hours remain unchanged." The #20150914 repeat below carries the same
article with "Effective this Monday" wording.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html
https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf

ORDER-ENTRY CLASSIFICATION. The evening phases the citations above name as
the "Pre-Open" (Monday-Thursday 16:50, later 16:45, to the 17:00 Globex open)
and the Sunday 16:00-17:00 queue are Globex pre-open states: the book accepts,
amends, and cancels orders, but the matching engine is not running and no
trade can print until the 17:00 open. They are therefore `order_entry`, not
`extended`. Everything that remains in the extended slices below is a
matching phase: the post-halt afternoon slice and the 17:00 electronic
session both print trades.

Pre-Open queues. No trade can match in any of these windows.
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
