<!-- SPDX-License-Identifier: MIT-0 -->

# `comex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for the named COMEX metals/NYMEX energy family. Current matching and Pre-Open queues are primary-supported and the January-2010/2015 matching revisions are exact, but the Sunday queue's 16:15→16:00 onset day is unavailable: the 2026-08-31 review narrowed it to 2012-05-28..2012-06-07 with both CME notice channels silent across that window; dated profiles now serve the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, leaving only the 16:00–16:15 quarter-hour undated. TAS/TAM/BTIC, options, and other clocks are excluded.

## Revision rows

- 2015-09-20 — T1 — CME Globex notice 20150907 — every COMEX and NYMEX close moves to 16:00 CT for trade date Monday 2015-09-21.
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
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | <https://web.archive.org/web/20111124185246id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf> | archive capture 2011-11-24T18:52:46Z | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | <https://web.archive.org/web/20120125020548id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf> | archive capture 2012-01-25T02:05:48Z | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | <https://web.archive.org/web/20120125025430id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf> | archive capture 2012-01-25T02:54:30Z | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | <https://web.archive.org/web/20120505161526id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-martin-luther-king.pdf> | archive capture 2012-05-05T16:15:26Z | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | <https://web.archive.org/web/20120505161539id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-presidents-day.pdf> | archive capture 2012-05-05T16:15:39Z | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
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
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | <https://web.archive.org/web/20260129012309id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-28> | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
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
markers and eleven 2024 dates this table takes from its trading-hours service, and
for 2025-2027. Inside a window a date with no row is audited normal; outside every
window this table has no answer at all.

**Six audited eras, with no gap between them.** The table declares 6 coverage windows: `2010-01-01..2012-12-31`, `2013-01-01..2015-12-31`, `2016-01-01..2018-12-31`, `2019-01-01..2021-12-31`, `2022-01-01..2024-12-31`, `2025-01-01..2027-12-31`.
The rows of every era are the D17 intersection of the families routed here: an era's rows are the family tables' own where a venue routes one family, and the joint statement where it routes several.
Every interval from 2010-01-01 is declared, so the whole span has an answer; the eras before 2010 are out of scope below the crate's January-2010 floor.

### 2010

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `CME Globex is closed` | `2010-new-years.pdf @2010-02-15T05:16:52Z` | T1 | CME prints `Jan 1` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-01-15 | early close | `15:15 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 15` above it, and the year comes from the document's own identity |
| 2010-01-18 | early close | `12:15 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jan 18` above it, and the year comes from the document's own identity |
| 2010-02-12 | early close | `15:15 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Feb 12` above it, and the year comes from the document's own identity |
| 2010-02-15 | early close | `12:15 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Feb 15` above it, and the year comes from the document's own identity |
| 2010-04-02 | closed | `CME Globex is closed` | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | CME prints `Apr 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-05-28 | early close | `15:15 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `15:15 CT` is the date's own final close; CME prints `May 28` above it, and the year comes from the document's own identity |
| 2010-05-31 | early close | `12:15 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `12:15 CT` is the date's own final close; CME prints `May 31` above it, and the year comes from the document's own identity |
| 2010-07-02 | early close | `15:15 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jul 2` above it, and the year comes from the document's own identity |
| 2010-07-05 | early close | `12:15 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jul 5` above it, and the year comes from the document's own identity |
| 2010-09-03 | early close | `15:15 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Sep 3` above it, and the year comes from the document's own identity |
| 2010-09-06 | early close | `12:15 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Sep 6` above it, and the year comes from the document's own identity |
| 2010-10-08 | early close | `15:15 CT` | `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Oct 8` above it, and the year comes from the document's own identity |
| 2010-11-25 | early close | `12:15 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2010-11-26 | early close | `12:45 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Nov 26` above it, and the year comes from the document's own identity |
| 2010-12-24 | closed | `CME Globex is closed` | `2010-christmas.pdf @2010-12-14T06:12:38Z` | T1 | CME prints `Dec 24` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-12-31 | early close | `15:15 CT` | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Dec 31` above it, and the year comes from the document's own identity |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-14 | early close | `15:15 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 14` above it, and the year comes from the document's own identity |
| 2011-01-17 | early close | `12:15 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jan 17` above it, and the year comes from the document's own identity |
| 2011-02-21 | early close | `12:15 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Feb 21` above it, and the year comes from the document's own identity |
| 2011-04-22 | closed | `CME Globex is closed` | `2011-good-friday.pdf @2011-10-28T02:37:07Z` | T1 | CME prints `Apr 22` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-05-30 | early close | `12:15 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `12:15 CT` is the date's own final close; CME prints `May 30` above it, and the year comes from the document's own identity |
| 2011-07-04 | early close | `12:15 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jul 4` above it, and the year comes from the document's own identity |
| 2011-09-05 | early close | `12:15 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Sep 5` above it, and the year comes from the document's own identity |
| 2011-11-24 | early close | `12:15 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 24` above it, and the year comes from the document's own identity |
| 2011-11-25 | early close | `12:45 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2011-12-26 | closed | `CME Globex is closed` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | CME prints `Dec 26` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `CME Globex is closed` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | CME prints `Jan 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-01-16 | early close | `12:15 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jan 16` above it, and the year comes from the document's own identity |
| 2012-02-20 | early close | `12:15 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Feb 20` above it, and the year comes from the document's own identity |
| 2012-04-06 | closed | `CME Globex is closed` | `2012-good-friday.pdf @2012-04-17T00:42:47Z` | T1 | CME prints `Apr 6` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-05-28 | early close | `12:15 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `12:15 CT` is the date's own final close; CME prints `May 28` above it, and the year comes from the document's own identity |
| 2012-07-04 | early close | `12:15 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jul 4` above it, and the year comes from the document's own identity |
| 2012-09-03 | early close | `12:15 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Sep 3` above it, and the year comes from the document's own identity |
| 2012-11-22 | early close | `12:15 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 22` above it, and the year comes from the document's own identity |
| 2012-11-23 | early close | `12:45 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Nov 23` above it, and the year comes from the document's own identity |
| 2012-12-24 | early close | `12:45 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Dec 24` above it, and the year comes from the document's own identity |
| 2012-12-25 | closed | `CME Globex is closed` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | CME prints `Dec 25` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |

### 2016

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `every routed family states a closure` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | the intersection of the families routed to this venue |
| 2016-01-18 | early close | `12:00 CT` | `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-02-15 | early close | `12:00 CT` | `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-03-25 | closed | `every routed family states a closure` | `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-05-30 | early close | `12:00 CT` | `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-07-04 | early close | `12:00 CT` | `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-09-05 | early close | `12:00 CT` | `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-11-24 | early close | `12:00 CT` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-11-25 | early close | `12:45 CT` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2016-12-26 | closed | `every routed family states a closure` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |

### 2017

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `every routed family states a closure` | `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2017-01-16 | early close | `12:00 CT` | `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2017-02-20 | early close | `12:00 CT` | `2017-presidents-day-holiday-schedule.xls @2017-06-28` | T1 | the intersection of the families routed to this venue |
| 2017-04-14 | closed | `every routed family states a closure` | `2017-good-friday-holiday-schedule.xls @2017-05-05` | T1 | the intersection of the families routed to this venue |
| 2017-05-29 | early close | `12:00 CT` | `2017-memorial-day-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-07-04 | early close | `12:00 CT` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-09-04 | early close | `12:00 CT` | `2017-labor-day-holiday-schedule.xls @2017-10-25` | T1 | the intersection of the families routed to this venue |
| 2017-11-23 | early close | `12:00 CT` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |
| 2017-11-24 | early close | `12:45 CT` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |
| 2017-12-25 | closed | `every routed family states a closure` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | the intersection of the families routed to this venue |

### 2018

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `every routed family states a closure` | `2018-new-years-holiday-schedule.xls @2018-01-06` | T1 | the intersection of the families routed to this venue |
| 2018-01-15 | early close | `12:00 CT` | `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | T1 | the intersection of the families routed to this venue |
| 2018-02-19 | early close | `12:00 CT` | `2018-presidents-day-holiday-schedule.xls @2018-05-08` | T1 | the intersection of the families routed to this venue |
| 2018-03-30 | closed | `every routed family states a closure` | `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-05-28 | early close | `12:00 CT` | `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-07-04 | early close | `12:00 CT` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-09-03 | early close | `12:00 CT` | `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-11-22 | early close | `12:00 CT` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-11-23 | early close | `12:45 CT` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-12-24 | early close | `12:45 CT` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |
| 2018-12-25 | closed | `every routed family states a closure` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the intersection of the families routed to this venue |


### 2013-2015 (T1)
**This era brings the venue to six audited windows.** The table as a whole carries 206 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **33 rows**: 33 stated rows and 0 `Unsourced` rows. Every row is the intersection of the families routed here — `globex_energy` — by the D17 rule `venues.rs` states: a row ships only where every routed family states the same one, and a date on which they differ, or on which one states a row while another has audited the date normal, ships `Unsourced`, with the disagreement named per date in the three year tables below.

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

### 2013

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `every routed family states a closure` | `2013-new-years.pdf @2013-04-14T19:41:46Z` | T1 | the intersection of the families routed to this venue |
| 2013-01-21 | early close | `12:15 CT` | `2013-martin-luther-king.pdf @2012-11-19T00:16:09Z` | T1 | the intersection of the families routed to this venue |
| 2013-02-18 | early close | `12:15 CT` | `2013-presidents-day.pdf @2013-03-09T11:53:37Z` | T1 | the intersection of the families routed to this venue |
| 2013-03-29 | closed | `every routed family states a closure` | `2013-good-friday.pdf @2013-06-23T19:59:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-05-27 | early close | `12:15 CT` | `2013-memorial-day.pdf @2013-06-23T20:36:04Z` | T1 | the intersection of the families routed to this venue |
| 2013-07-04 | early close | `12:15 CT` | `2013-4th-of-july.pdf @2013-06-23T20:58:25Z` | T1 | the intersection of the families routed to this venue |
| 2013-09-02 | early close | `12:15 CT` | `2013-labor-day.pdf @2013-09-02T17:08:41Z` | T1 | the intersection of the families routed to this venue |
| 2013-11-28 | early close | `12:15 CT` | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the intersection of the families routed to this venue |
| 2013-11-29 | early close | `12:45 CT` | `2013-thanksgiving.pdf @2014-02-14T06:28:36Z` | T1 | the intersection of the families routed to this venue |
| 2013-12-24 | early close | `12:45 CT` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the intersection of the families routed to this venue |
| 2013-12-25 | closed | `every routed family states a closure` | `2013-christmas.pdf @2014-04-12T06:24:28Z` | T1 | the intersection of the families routed to this venue |

### 2014

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `every routed family states a closure` | `2014-new-years.pdf @2013-10-07T20:58:00Z` | T1 | the intersection of the families routed to this venue |
| 2014-01-20 | early close | `12:15 CT` | `2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z` | T1 | the intersection of the families routed to this venue |
| 2014-02-17 | early close | `12:15 CT` | `2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z` | T1 | the intersection of the families routed to this venue |
| 2014-04-18 | closed | `every routed family states a closure` | `2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z` | T1 | the intersection of the families routed to this venue |
| 2014-05-26 | early close | `12:00 CT` | `2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z` | T1 | the intersection of the families routed to this venue |
| 2014-07-04 | early close | `12:00 CT` | `2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z` | T1 | the intersection of the families routed to this venue |
| 2014-09-01 | early close | `12:00 CT` | `2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z` | T1 | the intersection of the families routed to this venue |
| 2014-11-27 | early close | `12:00 CT` | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the intersection of the families routed to this venue |
| 2014-11-28 | early close | `12:45 CT` | `2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z` | T1 | the intersection of the families routed to this venue |
| 2014-12-24 | early close | `12:45 CT` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the intersection of the families routed to this venue |
| 2014-12-25 | closed | `every routed family states a closure` | `2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z` | T1 | the intersection of the families routed to this venue |

### 2015

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `every routed family states a closure` | `2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z` | T1 | the intersection of the families routed to this venue |
| 2015-01-19 | early close | `12:00 CT` | `2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z` | T1 | the intersection of the families routed to this venue |
| 2015-02-16 | early close | `12:00 CT` | `2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z` | T1 | the intersection of the families routed to this venue |
| 2015-04-03 | closed | `every routed family states a closure` | `2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z` | T1 | the intersection of the families routed to this venue |
| 2015-05-25 | early close | `12:00 CT` | `2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z` | T1 | the intersection of the families routed to this venue |
| 2015-07-03 | early close | `12:00 CT` | `2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z` | T1 | the intersection of the families routed to this venue |
| 2015-09-07 | early close | `12:00 CT` | `2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z` | T1 | the intersection of the families routed to this venue |
| 2015-11-26 | early close | `12:00 CT` | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the intersection of the families routed to this venue |
| 2015-11-27 | early close | `12:45 CT` | `2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z` | T1 | the intersection of the families routed to this venue |
| 2015-12-24 | early close | `12:45 CT` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | the intersection of the families routed to this venue |
| 2015-12-25 | closed | `every routed family states a closure` | `2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z` | T1 | the intersection of the families routed to this venue |

### 2019-2021 (T1)
**This era brought the venue to five audited windows; the 2013-2015 wave has since added a sixth.** The table as a whole carries 206 rows over 6 windows — 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **35 rows**: 9 stated closures and 3 `Unsourced` rows. The remaining 23 rows state the era's early closes. Every row is the intersection of the families routed here — `globex_energy` alone — by the D17 rule `venues.rs` states: a closure ships only where every routed family states the same one, and a date on which they differ, or on which one states a row while another has audited the date normal, ships `Unsourced`, with the disagreement named per date below. `comex` and `nymex` route `globex_energy` alone, so their rows are that family's own, unchanged. The three Juneteenth dates ship `Unsourced` because every routed family states the crate's not-worked-up marker there.

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
| 2019-01-21 | early close | `12:00 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:00 CT |
| 2019-02-18 | early close | `12:00 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:00 CT |
| 2019-04-19 | closed | `every routed family states a closure` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | T1 | the intersection of the families routed to this venue |
| 2019-05-27 | early close | `12:00 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:00 CT |
| 2019-06-19 | unsourced | `—` | `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | T1 | energy and metals states `Unsourced` |
| 2019-07-04 | early close | `12:00 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:00 CT |
| 2019-09-02 | early close | `12:00 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:00 CT |
| 2019-11-28 | early close | `12:00 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:00 CT |
| 2019-11-29 | early close | `12:45 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:45 CT |
| 2019-12-24 | early close | `12:45 CT` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | metals early close 12:45 CT |
| 2019-12-25 | closed | `every routed family states a closure` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | the intersection of the families routed to this venue |
### 2020
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `every routed family states a closure` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | the intersection of the families routed to this venue |
| 2020-01-20 | early close | `12:00 CT` | `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:00 CT |
| 2020-02-17 | early close | `12:00 CT` | `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:00 CT |
| 2020-04-10 | closed | `every routed family states a closure` | `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | T1 | the intersection of the families routed to this venue |
| 2020-05-25 | early close | `12:00 CT` | `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:00 CT |
| 2020-06-19 | unsourced | `—` | `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | T1 | energy and metals states `Unsourced` |
| 2020-07-03 | early close | `12:00 CT` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:00 CT |
| 2020-09-07 | early close | `12:00 CT` | `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:00 CT |
| 2020-11-26 | early close | `12:00 CT` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:00 CT |
| 2020-11-27 | early close | `12:45 CT` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:45 CT |
| 2020-12-24 | early close | `12:45 CT` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | metals early close 12:45 CT |
| 2020-12-25 | closed | `every routed family states a closure` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | the intersection of the families routed to this venue |
### 2021
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `every routed family states a closure` | `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | the intersection of the families routed to this venue |
| 2021-01-18 | early close | `12:00 CT` | `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:00 CT |
| 2021-02-15 | early close | `12:00 CT` | `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:00 CT |
| 2021-04-02 | closed | `every routed family states a closure` | `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | the intersection of the families routed to this venue |
| 2021-05-31 | early close | `12:00 CT` | `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:00 CT |
| 2021-06-19 | unsourced | `—` | `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | T1 | energy and metals states `Unsourced` |
| 2021-07-05 | early close | `12:00 CT` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:00 CT |
| 2021-09-06 | early close | `12:00 CT` | `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:00 CT |
| 2021-11-25 | early close | `12:00 CT` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:00 CT |
| 2021-11-26 | early close | `12:45 CT` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | metals early close 12:45 CT |
| 2021-12-24 | closed | `every routed family states a closure` | `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | the intersection of the families routed to this venue |
### 2022-2024 (T1/T2)

**Counts in this subsection are the 2022-2024 era's.** The table as a whole carries 206 rows over
its six audited windows: 38 in 2010-2012 (8 stated closures and 30 early closes), 33 in 2013-2015
(9 stated closures and 24 early closes), 31 in 2016-2018 (9 stated closures and 22 early closes),
35 in 2019-2021 (9 stated closures, 23 early closes and 3 `Unsourced`), 33 in this era (7 stated
closures, 23 early closes and 3 `Unsourced`) and 36 in 2025-2027 (10 stated closures and 26 early
closes). The derivation rule is the same in all six; only the documents differ.

**The venue drops no date.** Metals and energy are one key and the operator prints
them as one product row on every date this table audits, so its one routed family
decides each date and the intersection is total. Twenty-three dates are early
closes — nineteen at 13:30 CT (the Monday and Thursday holidays), 2022-11-25,
2023-11-24 and 2024-12-24 at 12:45 CT, and 2024-11-29 at 13:45 CT — and seven are
the full closures. Three are the family's own `Unsourced` markers, 2023-01-16,
2023-02-20 and 2023-04-07: the wave retrieved no document for those 2023 sheets,
so `globex_energy` states the crate's not-worked-up marker. The venue repeats the
family's answer rather than inventing a row or claiming the dates were audited
normal — the same treatment a disagreement gets, for the opposite reason: there is
no second family to disagree with.

### 2022
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-01-17 | early close | `13:30 CT` | `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | T1 | metals early close 13:30 CT |
| 2022-02-21 | early close | `13:30 CT` | `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | T1 | metals early close 13:30 CT |
| 2022-04-15 | closed | `every routed family states a closure` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | the intersection of the families routed to this venue |
| 2022-05-30 | early close | `13:30 CT` | `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | T1 | metals early close 13:30 CT |
| 2022-06-20 | early close | `13:30 CT` | `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | T1 | metals early close 13:30 CT |
| 2022-07-04 | early close | `13:30 CT` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | metals early close 13:30 CT |
| 2022-09-05 | early close | `13:30 CT` | `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | T1 | metals early close 13:30 CT |
| 2022-11-24 | early close | `13:30 CT` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | metals early close 13:30 CT |
| 2022-11-25 | early close | `12:45 CT` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | metals early close 12:45 CT |
| 2022-12-26 | closed | `every routed family states a closure` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | the intersection of the families routed to this venue |

### 2023
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `every routed family states a closure` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | the intersection of the families routed to this venue |
| 2023-01-16 | unsourced | `—` | `CME-SVC-2023-01-15` | T2 | energy and metals states `Unsourced`: the wave retrieved no document for this date |
| 2023-02-20 | unsourced | `—` | `CME-SVC-2023-02-19` | T2 | energy and metals states `Unsourced`: the wave retrieved no document for this date |
| 2023-04-07 | unsourced | `—` | `CME-SVC-2023-04-06` | T2 | energy and metals states `Unsourced`: the wave retrieved no document for this date |
| 2023-05-29 | early close | `13:30 CT` | `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | T1 | metals early close 13:30 CT |
| 2023-06-19 | early close | `13:30 CT` | `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | T1 | metals early close 13:30 CT |
| 2023-07-04 | early close | `13:30 CT` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | metals early close 13:30 CT |
| 2023-09-04 | early close | `13:30 CT` | `labor-day-2023.pdf @2023-08-02T19:24:46Z` | T1 | metals early close 13:30 CT |
| 2023-11-23 | early close | `13:30 CT` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | metals early close 13:30 CT |
| 2023-11-24 | early close | `12:45 CT` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | metals early close 12:45 CT |
| 2023-12-25 | closed | `every routed family states a closure` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | the intersection of the families routed to this venue |

### 2024
| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `every routed family states a closure` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | the intersection of the families routed to this venue |
| 2024-01-15 | early close | `13:30 CT` | `CME-SVC-2024-01-14` | T2 | metals early close 13:30 CT |
| 2024-02-19 | early close | `13:30 CT` | `CME-SVC-2024-02-18` | T2 | metals early close 13:30 CT |
| 2024-03-29 | closed | `every routed family states a closure` | `CME-SVC-2024-03-28` | T2 | the intersection of the families routed to this venue |
| 2024-05-27 | early close | `13:30 CT` | `CME-SVC-2024-05-26` | T2 | metals early close 13:30 CT |
| 2024-06-19 | early close | `13:30 CT` | `CME-SVC-2024-06-18` | T2 | metals early close 13:30 CT |
| 2024-07-04 | early close | `13:30 CT` | `CME-SVC-2024-07-03` | T2 | metals early close 13:30 CT |
| 2024-09-02 | early close | `13:30 CT` | `CME-SVC-2024-09-01` | T2 | metals early close 13:30 CT |
| 2024-11-28 | early close | `13:30 CT` | `CME-SVC-2024-11-27` | T2 | metals early close 13:30 CT |
| 2024-11-29 | early close | `13:45 CT` | `CME-SVC-2024-11-27` | T2 | metals early close 13:45 CT |
| 2024-12-24 | early close | `12:45 CT` | `CME-SVC-2024-12-24` | T2 | metals early close 12:45 CT |
| 2024-12-25 | closed | `every routed family states a closure` | `CME-SVC-2024-12-24` | T2 | the intersection of the families routed to this venue |

### 2025-2027 (T2)

**Counts in this subsection are the 2025-2027 era's.** The table as a whole carries 206 rows over
its six audited windows: 38 in 2010-2012 (8 stated closures and 30 early closes), 33 in 2013-2015
(9 stated closures and 24 early closes), 31 in 2016-2018 (9 stated closures and 22 early closes),
35 in 2019-2021 (9 stated closures, 23 early closes and 3 `Unsourced`), 33 in 2022-2024 (7 stated
closures, 23 early closes and 3 `Unsourced`) and 36 in this era (10 stated closures and 26 early
closes). The intersection is total in all six eras, so nothing is withheld for a disagreement.

**This table is derived, not retrieved, and the intersection is total.** It is the
intersection of the families that route to `Exchange::Comex` — which is one family,
`globex_energy`, whose metals half is this venue's documented scope — so there is no
disagreement to withhold: the venue carries that family's table unchanged, all
thirty-six rows, and every id resolves to the same CME Group
`trading-hours-by-product` artifact `globex_energy`'s evidence file already records.
The per-family rows, with their event-date-to-trade-date conversions and the
`13:30 preopen` reading that turns CME's pre-open-only records into early closes,
are there.

**Why one family rather than two.** CME prints the metals and NYMEX energy products as one
product-group row on every date in this window — `CL` and `GC` carry identical event
lists — so `globex_energy` is one key and one table, and design memo D17's
intersection rule is never reached. `docs/evidence/globex_energy.md` records the same
fact from the retrieval side. A consumer that maps a COMEX product to this calendar
is therefore using a clock the crate claims for both halves at once; if the operator
ever publishes them apart, the two halves become two keys and this table splits with
them.

**No date is dropped.** There is no `unsourced` row and no declared gap behind one in
this window. The table is the family's own evidence at one remove, and nothing here
rests on a document the family module does not carry.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `no events published` | `CME-SVC-2024-12-31` | T2 | energy closed |
| 2025-01-20 | early close | `early close 13:30 CT` | `CME-SVC-2025-01-19` | T2 | energy early close 13:30 CT |
| 2025-02-17 | early close | `early close 13:30 CT` | `CME-SVC-2025-02-16` | T2 | energy early close 13:30 CT |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | energy closed |
| 2025-05-26 | early close | `early close 13:30 CT` | `CME-SVC-2025-05-25` | T2 | energy early close 13:30 CT |
| 2025-06-19 | early close | `early close 13:30 CT` | `CME-SVC-2025-06-18` | T2 | energy early close 13:30 CT |
| 2025-07-04 | early close | `early close 12:00 CT` | `CME-SVC-2025-07-03` | T2 | energy early close 12:00 CT |
| 2025-09-01 | early close | `early close 13:30 CT` | `CME-SVC-2025-08-31` | T2 | energy early close 13:30 CT |
| 2025-11-27 | early close | `early close 13:30 CT` | `CME-SVC-2025-11-26` | T2 | energy early close 13:30 CT |
| 2025-11-28 | early close | `early close 13:45 CT` | `CME-SVC-2025-11-26` | T2 | energy early close 13:45 CT |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | energy closed |
| 2025-12-24 | early close | `early close 12:45 CT` | `CME-SVC-2025-12-24` | T2 | energy early close 12:45 CT |
| 2025-12-25 | closed | `no events published` | `CME-SVC-2025-12-24` | T2 | energy closed |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-2025-12-31` | T2 | energy closed |
| 2026-01-19 | early close | `early close 13:30 CT` | `CME-SVC-2026-01-18` | T2 | energy early close 13:30 CT |
| 2026-02-16 | early close | `early close 13:30 CT` | `CME-SVC-2026-02-15` | T2 | energy early close 13:30 CT |
| 2026-04-03 | closed | `no events published` | `CME-SVC-2026-04-01` | T2 | energy closed |
| 2026-05-25 | early close | `early close 13:30 CT` | `CME-SVC-2026-05-24` | T2 | energy early close 13:30 CT |
| 2026-06-19 | early close | `early close 12:00 CT` | `CME-SVC-2026-06-18` | T2 | energy early close 12:00 CT |
| 2026-07-03 | early close | `early close 12:00 CT` | `CME-SVC-2026-07-03` | T2 | energy early close 12:00 CT |
| 2026-09-07 | early close | `early close 13:30 CT` | `CME-SVC-2026-09-06` | T2 | energy early close 13:30 CT |
| 2026-11-26 | early close | `early close 13:30 CT` | `CME-SVC-2026-11-25` | T2 | energy early close 13:30 CT |
| 2026-11-27 | early close | `early close 13:45 CT` | `CME-SVC-2026-11-25` | T2 | energy early close 13:45 CT |
| 2026-12-24 | early close | `early close 12:45 CT` | `CME-SVC-2026-12-22` | T2 | energy early close 12:45 CT |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | energy closed |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | energy closed |
| 2027-01-18 | early close | `early close 13:30 CT` | `CME-SVC-2027-01-17` | T2 | energy early close 13:30 CT |
| 2027-02-15 | early close | `early close 13:30 CT` | `CME-SVC-2027-02-14` | T2 | energy early close 13:30 CT |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | energy closed |
| 2027-05-31 | early close | `early close 13:30 CT` | `CME-SVC-2027-05-30` | T2 | energy early close 13:30 CT |
| 2027-06-18 | early close | `early close 12:00 CT` | `CME-SVC-2027-06-17` | T2 | energy early close 12:00 CT |
| 2027-07-05 | early close | `early close 13:30 CT` | `CME-SVC-2027-07-04` | T2 | energy early close 13:30 CT |
| 2027-09-06 | early close | `early close 13:30 CT` | `CME-SVC-2027-09-05` | T2 | energy early close 13:30 CT |
| 2027-11-25 | early close | `early close 13:30 CT` | `CME-SVC-2027-11-24` | T2 | energy early close 13:30 CT |
| 2027-11-26 | early close | `early close 13:45 CT` | `CME-SVC-2027-11-24` | T2 | energy early close 13:45 CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | energy closed |

**Gaps, 2025-2027.**

- **No intersection gap exists in this window.** Every date on which `globex_energy`
  states a row ships that row here, because the metals half and the NYMEX energy half agree
  on all of them. This is a positive statement, checked per date by
  `the_venue_table_is_the_intersection_of_its_families`.
- **The operator's own gaps are inherited, not restated.** The unrepresentable
  order-entry deviation on 2025-11-28, the 2025 windows that survive only in a
  pre-holiday capture, and the Saturday sessions after the Friday holidays are gaps in
  `docs/evidence/globex_energy.md`; they are gaps here too, and that file is where
  their closing conditions live.
- **The window is the family's window, not the venue's horizon.** This subsection
  covers 2025-2027 because that is the era the trading-hours service reaches. The
  2013-2015 and 2019-2021 blocks are the remaining stage-2.2 waves, each of which
  extends this table over its own years; the cross-wave agreement audit (#95) closes
  with the last of them, and it is what would catch a future divergence between the
  two halves.
- **2028-01-01 ships no row.** CME's service publishes a 2027-12-30 .. 2028-01-02
  window and the family's table ends at 2027-12-31, so the venue's window ends there
  too. Extending it is stage 2.4's refresh, not a gap in this change.

**Interpretive steps, 2025-2027.**

- **The routing is read from production, not assumed.** `hours_for_exchange`'s
  `Exchange::Comex` arm resolves to
  `energy_metals_profile_at`, the same selector `globex_energy` resolves to; the test
  `the_venue_table_is_the_intersection_of_its_families` recomputes the table from that
  key's public `holiday_on` answers on every run.
- **The two venue tables hold the same rows, and each is fenced on its own.** Both
  derive from the one key, so a change to `globex_energy` reaches both or neither.
  They are separate tables rather than one shared binding, because a venue's table is
  a decision about that venue; `the_energy_venues_carry_the_family_table_unchanged`
  holds each against the family's own answers, so an edit to one that the family table
  does not justify fails the gate.
- **`comex` and `nymex` already carry a `monthly` ledger cadence**, so this change
  moves no cadence cell for them; it moves only their `Holidays` cell.

### The cross-wave D17 audit (#95)

**The intersection is re-derived over every audited era, not only this wave's.** `tools/check_wave5.py` recomputes the D17 rule over 2010-01-01..2027-12-31 — every date on which at least one routed family declares a window — and compares the result with the tables above date by date: **206 rows over the six windows**, 52 closures, 148 early closes and 6 withheld as `Unsourced` (the three 2019-2021 and three 2022-2024 dates `globex_energy` itself marks not worked up). This is the agreement audit memo §7 follow-up 10 asks for (#95).

**The venue routes one family, so the intersection is that family's table.** Metals and energy are one key and CME prints them as one product row on every date the table audits, so nothing abstains and nothing is withheld for a disagreement; the audit's job here is to confirm the venue ships `globex_energy`'s rows unchanged, which it does, date by date, over all 206.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html> — CME Globex notice 20090130, the pre-floor 17:00–16:15 CT energy and metals grid.
- <https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf> — CME `MT-027` gold futures cheat sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html> — CME SER-5391.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, which observes the weekday 16:45 queue already in effect without dating its onset.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html> — CME Globex notice 20150907, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html> — CME gold futures contract specification, current grid.
- <https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html> — CME light sweet crude contract specification, current grid.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-05-01, "17:15 ET (16:15 CT)". **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-06-16, "17:00 ET (16:00 CT)". **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the earliest artifact in the Sunday-queue intersection. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — TAS/TAM/BTIC, options and other product clocks are excluded; they take their own keys when a consumer maps one.
- **holidays** — this venue ships the intersection of the families that route to
  it; see the `## Holidays` section above. The intersection is total in this window, so nothing is dropped, and the cross-wave agreement audit that memo §7 follow-up 10 asks for (#95) is recorded in *The cross-wave D17 audit (#95)* above, where the same assertion is recomputed over 2010-2027.

## Module narrative (moved from src/calendar/schedules/futures/us/energy_metals.rs on 2026-09-12 UTC)

This family covers the shared grid used by NYMEX CL/MCL/QM, NG/MNG/QG,
HO/RB/BZ, and PL/PA plus COMEX GC/MGC, SI/SIL, and HG/MHG. Platinum and
palladium are NYMEX products, not COMEX products. At the January-2010 audit
floor these families opened 17:00 CT and closed 16:15 CT. CME's 2015 Globex
notice moved every COMEX and NYMEX close to 16:00 CT for Monday 2015-09-21
while leaving opens unchanged, so a separate metals clock would duplicate
both the current grid and the in-scope history. Current CME material also
publishes Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 Pre-Open. Its
correction calls both queues a long-term practice without giving their
unconditional onset days. A 2010 notice observes that the weekday queue was
already 16:45 but likewise supplies no onset, and the Sunday queue's
16:15→16:00 move is only bracketed: metals hours pages crawled 2012-05-01
still publish "17:15 ET (16:15 CT)" while the 2012-06-16 crawl shows
"17:00 ET (16:00 CT)", with no notice stating the day. The fixed-current
table includes both sourced current queues; dated profiles retain matching
only.
The revision is keyed to Sunday 2015-09-20, the local opening day of that
Monday trade-date session, so a wrapped rule gives Monday the sourced close.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html
https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html
https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html
https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html
https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html

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

DATED QUEUES, CARRIED BACK AS THE SOURCED INTERSECTION. These profiles
previously ran with no queue at all, which reported the venue as closed
through windows it was demonstrably accepting orders in. Both phases are now
carried from the January-2010 floor at their narrowest sourced values, so no
cutover is asserted:
  Sunday 16:15-17:00 - CME's trading-hours pages read "17:15 ET (16:15 CT)"
  for Light Sweet Crude, Henry Hub, Gold and Silver on the 2012-05-11 and
  2012-05-28 captures and "17:00 ET (16:00 CT)" on 2012-06-07. The queue only
  widened, so 16:15-17:00 holds under both regimes; the knowledge-bound row
  supplies the extra 16:00-16:15 quarter-hour.
  Monday-Thursday 16:45-17:00 - the 2010 notice cited above observes this
  queue was already in effect, and no primary source names an earlier value.
