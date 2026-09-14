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
own `trading-hours-by-product` service at tier T2. Byte counts, capture times and
sha256 are in each id's row, so a row can be re-verified from this file together
with the research store's `holidays/raw/` indexes.

| Document | Window | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|
| `2010-new-years.pdf @2010-02-15T05:16:52Z` | `2010-new-years.pdf` | 2010-02-15T05:16:52Z | T1 | `c30a6cef73fca23c54b25907f307ad52a2922d1e4b76c0a12de126dc6fc31a6d` |
| `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | `2010-martin-luther-king.pdf` | 2010-03-31T06:42:26Z | T1 | `11af96423f788565a434209ebb94a82f69dd2c129621927cd898c2ff6c0978d3` |
| `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | `2010-presidents-day.pdf` | 2010-02-15T06:46:41Z | T1 | `ba379a7fa57efef43820583ada0002ea6cd8ccf0caf1b650d1cb6e8561f84253` |
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | 2010-06-01T11:19:16Z | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
| `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | `2010-memorial-day.pdf` | 2010-06-01T09:42:25Z | T1 | `46a2f00f0f23c82189d86953092ee1171e0ec1c460891bda3ef0578518d12859` |
| `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | `2010-4th-of-july.pdf` | 2010-06-02T00:56:37Z | T1 | `c6a0f8c0b079b85e4500d30d942247bc9aa14d6c70fe3ed73d6d77b21b65ee2c` |
| `2010-labor-day.pdf @2010-06-02T00:56:41Z` | `2010-labor-day.pdf` | 2010-06-02T00:56:41Z | T1 | `2aecfe737c9613f82b975a01812607e96c9284638e8bd9b9bd403edfae510620` |
| `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | `2010-columbus-day.pdf` | 2010-08-21T13:31:22Z | T1 | `e8fbf61d914ebd3cd74de611a7b2d9a4d13a67e1233045ccc2fcbb57556be3f9` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | 2010-11-22T09:40:12Z | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2010-christmas.pdf @2010-12-14T06:12:38Z` | `2010-christmas.pdf` | 2010-12-14T06:12:38Z | T1 | `d4adb060f6eb592fb24e3a272db57b3d8c9d69f3f370682ecd8d57e4169c37be` |
| `2011-new-years.pdf @2011-11-01T14:39:45Z` | `2011-new-years.pdf` | 2011-11-01T14:39:45Z | T1 | `42c289804cd3fa0830556ecb7fcc31493c452ba9e7325ebe7d7ce29613e41476` |
| `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | `2011-martin-luther-king.pdf` | 2011-10-28T02:34:29Z | T1 | `2e389e2688d6760705220a11657329e76a9eb3a88d78664b4775eb7481be7b17` |
| `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | `2011-presidents-day.pdf` | 2011-10-28T02:35:16Z | T1 | `0342359e135acada5cfaa1b74f806477f759924a0b40625e035242c7e81321d4` |
| `2011-good-friday.pdf @2011-10-28T02:37:07Z` | `2011-good-friday.pdf` | 2011-10-28T02:37:07Z | T1 | `6cf10359bb438eb49287dcef7c1e75a484df4d6e3b538fa9ee59dc3832210bda` |
| `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | `2011-memorial-day.pdf` | 2013-09-30T10:56:52Z | T1 | `5482f7bf47e0ee61448cf5f60fd4a5373cc39cb0e46220150c1f6a2ab2d6caec` |
| `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | `2011-4th-of-july.pdf` | 2011-11-01T14:40:54Z | T1 | `4c3bfbbe927ed799006edce76b5f263b6299bab29be7ff1eb06326e184a9b443` |
| `2011-labor-day.pdf @2011-11-01T14:43:45Z` | `2011-labor-day.pdf` | 2011-11-01T14:43:45Z | T1 | `03f38fea761a6da7633c3e636a40de61431c866a87f2229f270685bf9bb4470b` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | 2011-11-24T18:52:46Z | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | 2012-01-25T02:05:48Z | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | 2012-01-25T02:54:30Z | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | 2012-05-05T16:15:26Z | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | 2012-05-05T16:15:39Z | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
| `2012-good-friday.pdf @2012-04-17T00:42:47Z` | `2012-good-friday.pdf` | 2012-04-17T00:42:47Z | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | 2012-09-15T00:37:14Z | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | 2012-09-15T00:39:23Z | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | 2012-09-15T00:34:37Z | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | 2013-01-27T22:39:01Z | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | 2013-04-14T19:40:27Z | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
| `2016-new-years-holiday-schedule.pdf @2016-01-08` | `2016-new-years-holiday-schedule.pdf` | 2014-12-12T03:40:39Z | T1 | `7d032ee5457cc5c06acb512e54006fa0678761e12d2b60bf625d924741896755` |
| `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | `2016-martin-luther-king-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `0fac2d08a84f8e9a637573438adfab33ba91c69e76b6dce01f0ca4cbcde8aff1` |
| `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | `2016-presidents-day-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `b5264dd479efd69df495de170cdd58326f64e9e31a2ed7cca6d7d96cc424c9c2` |
| `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | `2016-good-friday-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `632c4e3e451fd972e4cccb743f7dfa7a41896044cafc0e82953e0c6fb6d0bc42` |
| `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | `2016-memorial-day-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `03fa4af1988ae7a54c6e5426b83a76e54825ac4398f7a2be6e77be82fe8ab9f9` |
| `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | `2016-4th-of-july-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `533a49be68619dbd003b3e749485be46051018d3613d44a3043bf8a087fec9ad` |
| `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | `2016-labor-day-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `a4c0676ad117a63894b84637e8e5a7734ed17d7e2d1ee09d5a96370c7fd53b4a` |
| `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | `2016-thanksgiving-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `b3c4dbe2d60bf5530316653d381ba28f1eb4ed6ec55844dca976b1852941c2b1` |
| `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | `2016-christmas-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `42efeb845763d3d8a288dd9f9349bfcaf88badebf49ee1c6bc645a72ec01d252` |
| `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | `2017-new-years-holiday-schedule.pdf` | 2017-06-28T11:58:19Z | T1 | `493e9cd3fb8ae7ce8b059f30516dedd83631a101e63ecd4c9201fa24c8a2aae8` |
| `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | `2017-martin-luther-king-holiday-schedule.xls` | 2017-06-28T17:25:56Z | T1 | `c11937ee9995dd712f23631dd235571d87d2c387bc21e074cc87905b68980d58` |
| `2017-presidents-day-holiday-schedule.xls @2017-06-28` | `2017-presidents-day-holiday-schedule.xls` | 2017-06-28T17:41:48Z | T1 | `2ec7b623faa8f17ab5946091492be0dacbd29e6446ab3789e0ba4fa5ccd58b54` |
| `2017-good-friday-holiday-schedule.xls @2017-05-05` | `2017-good-friday-holiday-schedule.xls` | 2017-05-05T07:23:15Z | T1 | `bdc8775f7061ae1e377421fe7a951446c0066ba11d1dbfc7bebba9e7c9415ccb` |
| `2017-memorial-day-holiday-schedule.xls @2017-10-25` | `2017-memorial-day-holiday-schedule.xls` | 2017-05-05T07:24:06Z | T1 | `2d5a64e7ba05c1889993e91cde352312a8fb5901c5f6a4d53439002ab067542a` |
| `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | `2017-4th-of-july-holiday-schedule.xls` | 2017-05-05T07:22:05Z | T1 | `d8d668183f686591879e8ee0d2db13c650c09d5a3c13e2378be068c784679453` |
| `2017-labor-day-holiday-schedule.xls @2017-10-25` | `2017-labor-day-holiday-schedule.xls` | 2017-05-05T07:23:46Z | T1 | `660473ca09893868560b64e544c31c984a715595b1311f136af389951ef56f63` |
| `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | `2017-thanksgiving-holiday-schedule.xls` | 2021-01-26T09:48:35Z | T1 | `b6bc3dba9f1e0c5a86d543e389d15cea6006f456e6dc2cfd7813eae31db2f869` |
| `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | `2017-christmas-holiday-schedule.xls` | 2021-01-26T09:48:35Z | T1 | `bfb291c9c08764ee2bcf0327690b98c58cbb4cf493723c7b1464eccb0d062d30` |
| `2018-new-years-holiday-schedule.xls @2018-01-06` | `2018-new-years-holiday-schedule.xls` | 2017-05-05T07:26:53Z | T1 | `73332758cbf95363a04cf422a672157c1b1a5bb34d4c9bd496bd7b7491a4d8cd` |
| `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | `2018-martin-luther-king-holiday-schedule.xls` | 2018-01-06T22:56:50Z | T1 | `81f30b0be680898b82175b19ee6a2fccc85b7e340fac2418b0ad00973c4a93a8` |
| `2018-presidents-day-holiday-schedule.xls @2018-05-08` | `2018-presidents-day-holiday-schedule.xls` | 2018-01-06T22:57:32Z | T1 | `fddc713823c58cfdc4de2486ea3b2953ecda24c01579da64eb3933d8a7efc4b8` |
| `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | `2018-good-friday-holiday-schedule.xls` | 2026-08-30T10:02:25Z | T1 | `ec3ffb0a85d061adbb53678353428eaeade9817963d6b5069db228844d9fbedf` |
| `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | `2018-memorial-day-holiday-schedule.xls` | 2026-08-30T10:02:25Z | T1 | `b3524e36abf39b038137145620cfe1a0d46022ce732ae623d6b71af8a4b8750b` |
| `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | `2018-4th-of-july-holiday-schedule.xls` | 2026-08-30T10:02:25Z | T1 | `8ed9e0f74c2bcdfdda8fa80c915e6bba5ed3c5ca7ec89476f11db03b256784ae` |
| `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | `2018-labor-day-holiday-schedule.xls` | 2026-08-30T10:02:25Z | T1 | `9b8c892dd3faba6900aae949f7f0c63b525191a76dccc938aa9859920b71b40d` |
| `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | `2018-thanksgiving-holiday-schedule.xls` | 2026-08-30T10:02:25Z | T1 | `31fc95f5b27f8689477b4abb557f14b2cf9ac481262926e30b78adf1455adff1` |
| `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | `2018-christmas-holiday-schedule.xls` | 2026-08-30T10:02:25Z | T1 | `b97ee5f47d55c4383c7f1fa7d554eba3c7f0e8ded18343aadd06d7908d9e7a91` |
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | live retrieval 2026-09-12T04:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | live retrieval 2026-09-12T04:30Z | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | live retrieval 2026-09-12T04:30Z | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | live retrieval 2026-09-12T04:30Z | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | live retrieval 2026-09-12T04:30Z | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | live retrieval 2026-09-12T04:30Z | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |

## Holidays

**Coverage:** 2010-01-01..2012-12-31, 2016-01-01..2018-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).
Tier: **T1** for 2010-2012 and 2016-2018, from the operator's own published
holiday schedules; **T2** for 2025-2027, from its trading-hours service. Inside a
window a date with no row is audited normal; outside every window this table has no
answer at all.
at all.

**Two audited eras, and a gap between them.** The table declares two coverage
windows: `2010-01-01..2012-12-31`, from the families' 2010-2012 documents, and
`2025-01-01..2027-12-31`, from the trading-hours service. The 2013-2024 interval
is audited by neither — those years are the remaining stage-2.2 waves — so it
lies outside every declared window and `holiday_on` has **no answer** there
rather than reporting an unaudited date as normal.

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

### 2025-2027 (T2)

**Counts in this subsection are the 2025-2027 era's.** The table as a whole
carries 74 rows over its two audited windows: 36 in this era (10 stated closures
and 26 early closes) and 38 in 2010-2012 (8 stated closures and 30 early
closes). The intersection is total in both eras, so nothing is withheld.

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
- **The window is the family's window, not the venue's horizon.** Coverage starts
  2025-01-01 because that is where the family's table starts. The 2010-2024 blocks land
  in stage 2.2's waves, each of which extends this table over its own years; the
  cross-wave agreement audit (#95) closes with the last of them, and it is what would
  catch a future divergence between the two halves.
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
  it; see the `## Holidays` section above. The intersection is total in this window, so nothing is dropped, and the cross-wave agreement audit that memo §7 follow-up 10 asks for (#95) is still open: it closes with the last stage-2.2 family wave, when the same assertion can be re-run over 2010-2027 rather than over this window alone.

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
