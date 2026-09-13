<!-- SPDX-License-Identifier: MIT-0 -->

# `nymex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for the named NYMEX energy/PGM and COMEX metals family. Current matching and Pre-Open queues are primary-supported and the January-2010/2015 matching revisions are exact, but the Sunday queue's 16:15→16:00 onset day is unavailable: the 2026-08-31 review narrowed it to 2012-05-28..2012-06-07 with both CME notice channels silent across that window; dated profiles now serve the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, leaving only the 16:00–16:15 quarter-hour undated. TAS/TAM/BTIC, options, and other clocks are excluded.

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
| `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | `2010-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-martin-luther-king.pdf, capture 2010-03-31T06:42:26Z, 54521 bytes | T1 | `11af96423f788565a434209ebb94a82f69dd2c129621927cd898c2ff6c0978d3` |
| `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | `2010-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-presidents-day.pdf, capture 2010-02-15T06:46:41Z, 49335 bytes | T1 | `ba379a7fa57efef43820583ada0002ea6cd8ccf0caf1b650d1cb6e8561f84253` |
| `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | `2010-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-memorial-day.pdf, capture 2010-06-01T09:42:25Z, 90478 bytes | T1 | `46a2f00f0f23c82189d86953092ee1171e0ec1c460891bda3ef0578518d12859` |
| `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | `2010-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-4th-of-july.pdf, capture 2010-06-02T00:56:37Z, 49205 bytes | T1 | `c6a0f8c0b079b85e4500d30d942247bc9aa14d6c70fe3ed73d6d77b21b65ee2c` |
| `2010-labor-day.pdf @2010-06-02T00:56:41Z` | `2010-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-labor-day.pdf, capture 2010-06-02T00:56:41Z, 48708 bytes | T1 | `2aecfe737c9613f82b975a01812607e96c9284638e8bd9b9bd403edfae510620` |
| `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | `2010-columbus-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-columbus-day.pdf, capture 2010-08-21T13:31:22Z, 43838 bytes | T1 | `e8fbf61d914ebd3cd74de611a7b2d9a4d13a67e1233045ccc2fcbb57556be3f9` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-thanksgiving.pdf, capture 2010-11-22T09:40:12Z, 97946 bytes | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2011-new-years.pdf @2011-11-01T14:39:45Z` | `2011-new-years.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-new-years.pdf, capture 2011-11-01T14:39:45Z, 125412 bytes | T1 | `42c289804cd3fa0830556ecb7fcc31493c452ba9e7325ebe7d7ce29613e41476` |
| `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | `2011-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-martin-luther-king.pdf, capture 2011-10-28T02:34:29Z, 126115 bytes | T1 | `2e389e2688d6760705220a11657329e76a9eb3a88d78664b4775eb7481be7b17` |
| `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | `2011-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-presidents-day.pdf, capture 2011-10-28T02:35:16Z, 146317 bytes | T1 | `0342359e135acada5cfaa1b74f806477f759924a0b40625e035242c7e81321d4` |
| `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | `2011-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-memorial-day.pdf, capture 2013-09-30T10:56:52Z, 66172 bytes | T1 | `5482f7bf47e0ee61448cf5f60fd4a5373cc39cb0e46220150c1f6a2ab2d6caec` |
| `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | `2011-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-4th-of-july.pdf, capture 2011-11-01T14:40:54Z, 126279 bytes | T1 | `4c3bfbbe927ed799006edce76b5f263b6299bab29be7ff1eb06326e184a9b443` |
| `2011-labor-day.pdf @2011-11-01T14:43:45Z` | `2011-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-labor-day.pdf, capture 2011-11-01T14:43:45Z, 129524 bytes | T1 | `03f38fea761a6da7633c3e636a40de61431c866a87f2229f270685bf9bb4470b` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf, capture 2011-11-24T18:52:46Z, 131748 bytes | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-martin-luther-king.pdf, capture 2012-05-05T16:15:26Z, 153369 bytes | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-presidents-day.pdf, capture 2012-05-05T16:15:39Z, 208816 bytes | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf, capture 2012-09-15T00:37:14Z, 70132 bytes | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf, capture 2012-09-15T00:39:23Z, 68805 bytes | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf, capture 2012-09-15T00:34:37Z, 71440 bytes | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-thanksgiving.pdf, capture 2013-01-27T22:39:01Z, 73205 bytes | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-christmas.pdf, capture 2013-04-14T19:40:27Z, 125821 bytes | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
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

**Coverage:** 2010-01-01..2012-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates in
`America/Chicago`). Tier: **T1** for 2010-2012, from CME's own holiday-calendar
PDFs, and **T2** for 2025-2027, from the operator's trading-hours service. Inside
the window a date with no row is audited normal; outside it this table has no answer
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
| 2010-01-15 | early close | `1515 CT / 1615 ET` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-01-18 | early close | `1215 CT / 1315 ET` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-02-12 | early close | `1515 CT / 1615 ET` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-02-15 | early close | `1215 CT / 1315 ET` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-05-28 | early close | `1515 CT / 1615 ET` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-05-31 | early close | `1215 CT / 1315 ET` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-07-02 | early close | `1515 CT / 1615 ET` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-07-05 | early close | `1215 CT / 1315 ET` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-09-03 | early close | `1515 CT / 1615 ET` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-09-06 | early close | `1215 CT / 1315 ET` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-10-08 | early close | `1515 CT / 1615 ET` | `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-11-25 | early close | `1215 CT / 1315 ET` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-11-26 | early close | `1245 CT / 1345 ET` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | every routed family states this row, so the intersection is total |
| 2010-12-31 | early close | `1515 CT / 1615 ET` | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | every routed family states this row, so the intersection is total |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-14 | early close | `1515 CT / 1615 ET` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-01-17 | early close | `1215 CT / 1315 ET` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-02-21 | early close | `1215 CT / 1315 ET` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-05-30 | early close | `1215 CT / 1315 ET` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-07-04 | early close | `1215 CT / 1315 ET` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-09-05 | early close | `1215 CT / 1315 ET` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-11-24 | early close | `1215 CT / 1315 ET` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | every routed family states this row, so the intersection is total |
| 2011-11-25 | early close | `1245 CT / 1345 ET` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | every routed family states this row, so the intersection is total |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-16 | early close | `1215 CT / 1315 ET` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-02-20 | early close | `1215 CT / 1315 ET` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-05-28 | early close | `1215 CT / 1315 ET` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-07-04 | early close | `1215 CT / 1315 ET` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-09-03 | early close | `1215 CT / 1315 ET` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-11-22 | early close | `1215 CT / 1315 ET` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-11-23 | early close | `1245 CT / 1345 ET` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | every routed family states this row, so the intersection is total |
| 2012-12-24 | early close | `1245 CT / 1345 ET` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | every routed family states this row, so the intersection is total |

### 2025-2027 (T2)

**This table is derived, not retrieved, and the intersection is total.** It is the
intersection of the families that route to `Exchange::Nymex` — which is one family,
`globex_energy`, whose energy half is this venue's documented scope — so there is no
disagreement to withhold: the venue carries that family's table unchanged, all
thirty-six rows, and every id resolves to the same CME Group
`trading-hours-by-product` artifact `globex_energy`'s evidence file already records.
The per-family rows, with their event-date-to-trade-date conversions and the
`13:30 preopen` reading that turns CME's pre-open-only records into early closes,
are there.

**Why one family rather than two.** CME prints the energy and COMEX metals products as one
product-group row on every date in this window — `CL` and `GC` carry identical event
lists — so `globex_energy` is one key and one table, and design memo D17's
intersection rule is never reached. `docs/evidence/globex_energy.md` records the same
fact from the retrieval side. A consumer that maps a NYMEX product to this calendar
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
  states a row ships that row here, because the energy half and the COMEX metals half agree
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
  `Exchange::Nymex` arm resolves to
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
- <https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-05-01, "17:15 ET (16:15 CT)".
- <https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-06-16, "17:00 ET (16:00 CT)".
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the earliest artifact in the Sunday-queue intersection.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — TAS/TAM/BTIC, options and other product clocks are excluded; they take their own keys when a consumer maps one.
- **holidays** — this venue ships the intersection of the families that route to
  it; see the `## Holidays` section above. The intersection is total in this window, so nothing is dropped, and the cross-wave agreement audit that memo §7 follow-up 10 asks for (#95) is still open: it closes with the last stage-2.2 family wave, when the same assertion can be re-run over 2010-2027 rather than over this window alone.

> Shared module. The narrative for
> [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs) lives in
> [`comex`](comex.md#module-narrative-moved-from-srccalendarschedulesfuturesusenergy_metalsrs-on-2026-09-12-utc).
> Sibling identities: [`comex`](comex.md), [`globex_energy`](globex_energy.md).
