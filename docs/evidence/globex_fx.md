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
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-good-friday.pdf, capture 2010-06-01T11:19:16Z, 44125 bytes | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
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
| `2011-columbus-day.pdf @2011-11-01T14:39:16Z` | `2011-columbus-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-columbus-day.pdf, capture 2011-11-01T14:39:16Z, 83897 bytes | T1 | `cfbf082c5931fbb753b980f117746fdd3a3f2b678a3a08ec282aa1a38f3f2a1f` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf, capture 2011-11-24T18:52:46Z, 131748 bytes | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf, capture 2012-01-25T02:05:48Z, 108183 bytes | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf, capture 2012-01-25T02:54:30Z, 130304 bytes | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-martin-luther-king.pdf, capture 2012-05-05T16:15:26Z, 153369 bytes | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-presidents-day.pdf, capture 2012-05-05T16:15:39Z, 208816 bytes | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
| `2012-good-friday.pdf @2012-05-05T16:16:49Z` | `2012-good-friday.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-good-friday.pdf, capture 2012-05-05T16:16:49Z of the canonical URL (the research store's INDEX.md still records the byte-identical query-string-variant capture 2012-04-17T00:42:47Z), 60600 bytes | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf, capture 2012-09-15T00:37:14Z, 70132 bytes | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf, capture 2012-09-15T00:39:23Z, 68805 bytes | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf, capture 2012-09-15T00:34:37Z, 71440 bytes | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-columbus-day.pdf @2012-09-15T00:15:14Z` | `2012-columbus-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-columbus-day.pdf, capture 2012-09-15T00:15:14Z, 64847 bytes | T1 | `aae7ddc8789c31fda8fbe6ccf09ffc5719cbfdf518750c2aa236e7519b11babf` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-thanksgiving.pdf, capture 2013-01-27T22:39:01Z, 73205 bytes | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-christmas.pdf, capture 2013-04-14T19:40:27Z, 125821 bytes | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |

## Holidays

**Coverage:** 2010-01-01..2012-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates in
`America/Chicago`), audited per era: **T1** for 2010-2012 in the per-year sections
below and **T2** for 2025-2027. Inside the window a date with no row is audited
normal; outside it this table has no answer at all.

**Two audited eras, and a gap between them.** The table declares two coverage
windows: `2010-01-01..2012-12-31`, from this era's documents, and
`2025-01-01..2027-12-31`, from the trading-hours service. The 2013-2024 interval
is audited by neither — those years are the remaining stage-2.2 waves — so it
lies outside every declared window and `holiday_on` has **no answer** there
rather than reporting an unaudited date as normal. `HolidayCoverage::windows()`
lists the two, and `contains` answers per date.

### 2010

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2010-01-15 | early close | `1515 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2010-01-15 beside it |
| 2010-01-18 | early close | `1200 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2010-01-18 beside it |
| 2010-02-12 | early close | `1515 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2010-02-12 beside it |
| 2010-02-15 | early close | `1200 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2010-02-15 beside it |
| 2010-04-02 | early close | `1015 CT` | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | `1015 CT` is the date's own final close; CME prints trade date 2010-04-02 beside it |
| 2010-05-28 | early close | `1515 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2010-05-28 beside it |
| 2010-05-31 | early close | `1200 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2010-05-31 beside it |
| 2010-07-02 | early close | `1515 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2010-07-02 beside it |
| 2010-07-05 | early close | `1200 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2010-07-05 beside it |
| 2010-09-03 | early close | `1515 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2010-09-03 beside it |
| 2010-09-06 | early close | `1200 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2010-09-06 beside it |
| 2010-10-08 | early close | `1515 CT` | `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2010-10-08 beside it |
| 2010-11-25 | early close | `1200 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2010-11-25 beside it |
| 2010-11-26 | early close | `1215 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2010-11-26 beside it |
| 2010-12-31 | early close | `1215 CT` | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2010-12-31 beside it |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-14 | early close | `1515 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2011-01-14 beside it |
| 2011-01-17 | early close | `1200 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2011-01-17 beside it |
| 2011-02-18 | early close | `1515 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2011-02-18 beside it |
| 2011-02-21 | early close | `1200 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2011-02-21 beside it |
| 2011-05-27 | early close | `1515 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2011-05-27 beside it |
| 2011-05-30 | early close | `1200 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2011-05-30 beside it |
| 2011-07-01 | early close | `1515 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2011-07-01 beside it |
| 2011-07-04 | early close | `1200 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2011-07-04 beside it |
| 2011-09-02 | early close | `1515 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2011-09-02 beside it |
| 2011-09-05 | early close | `1200 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2011-09-05 beside it |
| 2011-10-07 | early close | `1515 CT` | `2011-columbus-day.pdf @2011-11-01T14:39:16Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2011-10-07 beside it |
| 2011-11-24 | early close | `1200 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2011-11-24 beside it |
| 2011-11-25 | early close | `1215 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2011-11-25 beside it |
| 2011-12-27 | late open | `0500 CT` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | `0500 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-03 | late open | `0500 CT` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | `0500 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-01-13 | early close | `1515 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2012-01-13 beside it |
| 2012-01-16 | early close | `1200 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2012-01-16 beside it |
| 2012-02-17 | early close | `1515 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2012-02-17 beside it |
| 2012-02-20 | early close | `1200 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2012-02-20 beside it |
| 2012-04-06 | early close | `1015 CT` | `2012-good-friday.pdf @2012-05-05T16:16:49Z` | T1 | `1015 CT` is the date's own final close; CME prints trade date 2012-04-06 beside it |
| 2012-05-25 | early close | `1515 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2012-05-25 beside it |
| 2012-05-28 | early close | `1200 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2012-05-28 beside it |
| 2012-07-04 | early close | `1200 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2012-07-04 beside it |
| 2012-08-31 | early close | `1515 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2012-08-31 beside it |
| 2012-09-03 | early close | `1200 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2012-09-03 beside it |
| 2012-10-05 | early close | `1515 CT` | `2012-columbus-day.pdf @2012-09-15T00:15:14Z` | T1 | `1515 CT` is the date's own final close; CME prints trade date 2012-10-05 beside it |
| 2012-11-22 | early close | `1200 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `1200 CT` is the date's own final close; CME prints trade date 2012-11-22 beside it |
| 2012-11-23 | early close | `1215 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2012-11-23 beside it |
| 2012-12-24 | early close | `1215 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2012-12-24 beside it |
| 2012-12-26 | late open | `0500 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `0500 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |

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
