<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_interest_rates` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`interest_rates.rs`](../../src/calendar/schedules/futures/us/interest_rates.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CBOT Treasury/micro-Treasury, 30-Day Fed Funds, and CME SOFR standard grid. The January-2010 queues, 2010 weekday-queue revision, and 2011 matching-open revision are exact; the current Sunday queue is sourced but its 16:15→16:00 cutover day is unavailable — bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld — and is omitted after 2011 in dated routing.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2011-10-02 — T1 — CME Globex notice 20110926 — every legacy CBOT interest-rate open moves to 17:00 CT for trade date Monday 2011-10-03.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Holidays

**Coverage:** 2025-01-01 .. 2027-12-31 (inclusive venue-local trade dates,
`America/Chicago`). Tier: T2 throughout — CME Group's own trading-hours
service, the endpoint the `trading-hours.html` page calls to render its
per-asset-class Holiday Hours table, read as bytes and saved. Wave 1 of the
built-in holiday tables; no CME Globex holiday hours below 2025 are encoded
yet, so a date outside the window has no answer rather than a normal one.

**Key.** Every row is keyed by the crate's venue-local trade date, never by
CME's event date (design memo D1). CME publishes a Monday holiday's noon halt
on the Monday; the trading day it shortens opened at 17:00 CT the previous
evening and closes on the Monday, so the crate's trade date is the Monday and
the clip is stated there. An eve record is evidence for the holiday's row, not
a row of its own, unless the eve carries an early close of its **own** trade
date — as Christmas Eve does.

**Family line.** The retrieval's vocabulary maps to this key as
`Interest Rates (ZN)` — CME's own representative product for the group, the
10-Year T-Note. Rows whose published event list is shared with other groups are
printed by CME under a joined label
(`Equity Index (ES); Interest Rates (ZN)`, and the wider all-groups labels on
full closures); each is read here as the Interest Rates line of that label.
CME's T1 rendering of Thanksgiving 2026 prints a row labelled `Interest Rates`
whose events equal the service's ZN events for that window, which is what
validates the mapping.

**Zone.** The service prints no zone per row. The operator page states
`Trading hours are subject to change and are in U.S. Central Time unless
otherwise stated.`, so the `CT` token in every instant cell below is this
file's editorial expansion of that sentence, never a token CME printed beside
the instant. CME publishes no ET column in this channel and no ET value is
asserted here.

**Event vocabulary**, verbatim from the same page: `PREOPEN` — "Order Entry,
modification, and cancel are allowed. No order matching."; `OPEN` — "Start of
continuous trading phase. Order matching begins."; `closed` — "Final Close of
the date." On the Monday and Thursday holidays CME publishes a **pre-open**, not
a `closed`, at the noon halt: matching stops but the date's own final close is
never published and the span carries the following business day's trade date.
The crate records those dates as early closes at the stated instant, because
matching — what `is_open` answers — stops there.

**Documents.** All ids expand to CME Group's trading-hours service,
`https://www.cmegroup.com/services/trading-hours-by-product`, queried with the
ten-product id set `316,133,425,300,58,437,22,8478,5201,10191` (`THBP-A`; ZN is
id 316. There is one id per shipped trade date — `CME-SVC-<trade date>` — so a
row's citation and this table line up one to one even where two rows are read
from a single service response, as 2025-12-24 and 2025-12-25 are. The
six-product second set (`THBP-B`, `168,167,320,323,19,27`) keys no row of this
family. Bytes, sha256 and the per-document retrieval or capture time are in the
research store under `holidays/raw/cme-2025-2027*/INDEX.md`; archive captures
are Wayback `id_` replays of the same endpoint. Retrieval of the retrieved
corpus and of the repair round: 2026-09-12 (UTC). The research-store code for
each document is in that `INDEX.md`.

### Documents

| Document | Window | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|
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

One further document is cited in the notes below and keys no row: `CME-TH-PAGE`,
the operator's own `trading-hours.html` page, archive capture
`2025-08-30T02:14:20Z` and live read 2026-09-12, research codes `D50` and
`D51`, which is T1 but states no instants.

Research codes are the local ids of
`exchange-hours-research/holidays/cme-2025-2027.json`, whose `coverage` legend
expands each one to its full URL and whose `raw/cme-2025-2027*/INDEX.md` carries
the sha256 of the retrieved bytes. They are file-local and collide across
research blocks, which is why the crate re-keys every citation to the
`CME-SVC-<trade date>` form above.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — no final close of this date is published | `CME-SVC-2024-12-31` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-01-01, CME trade date 2025-01-02; the eve eventDate 2024-12-31 prints `16:00 closed` for CME trade date 2024-12-31 and no evening re-open |
| 2025-01-20 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2025-01-19` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-01-20, CME trade date 2025-01-21; the same date's `17:00 open` belongs to that next trade date |
| 2025-02-17 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2025-02-16` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-02-17, CME trade date 2025-02-18 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-04-18; the eve eventDate 2025-04-17 prints `16:00 closed` for CME trade date 2025-04-17 and no evening re-open |
| 2025-05-26 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2025-05-25` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-05-26, CME trade date 2025-05-27 |
| 2025-06-19 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2025-06-18` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-06-19, CME trade date 2025-06-20 |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2025-08-31` — capture `2024-12-20T15:53:40Z` | T2 | eventDate 2025-09-01, CME trade date 2025-09-02 |
| 2025-11-27 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2025-11-26` — capture `2026-01-29T01:23:09Z` | T2 | eventDate 2025-11-27, CME trade date 2025-11-28 |
| 2025-11-28 | early close | `07:00 preopen; 07:30 open; 12:15 closed` — 12:15 CT | `CME-SVC-2025-11-26` — capture `2026-01-29T01:23:09Z` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28; the morning pair is the gap recorded below |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` — live `2026-09-12T08:55:12Z` | T2 | eventDate 2025-11-29; CME's own 2025 Globex table states the Thanksgiving period as `27 - 29 November 2025` |
| 2025-12-24 | early close | `12:15 closed` — 12:15 CT | `CME-SVC-2025-12-24` — capture `2026-01-29T01:21:59Z` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24; no evening re-open is published |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — no final close of this date is published | `CME-SVC-2025-12-24` — capture `2026-01-29T01:21:59Z` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 |

**Interpretive steps, 2025.**

- **The noon halts are early closes, not closures.** On 2025-01-20, 2025-02-17,
  2025-05-26, 2025-06-19, 2025-09-01 and 2025-11-27 CME prints a pre-open at
  the halt and gives the whole span the next business day's trade date. Matching
  stops at the printed instant, so the crate clips its own trade date there and
  keeps the trading that happened. Making those dates `Closed` would delete a
  Sunday-or-weekday evening leg that really traded.
- **The `16:00 closed` eves are normal, and carry no row.** 2024-12-31,
  2025-04-17 and the Thursday before each full closure print the family's
  ordinary weekday final close. What differs is the missing evening leg, and the
  neighbouring `closed` row already deletes it, because that leg's trade date is
  the closed date.
- **Saturday 2025-11-29 ships a row that changes no answer.** CME's own 2025
  Globex table states the Thanksgiving period as `27 - 29 November 2025`, and the
  service, queried live for 2025-11-26 .. 2025-11-29, publishes a 2025-11-29
  schedule for all ten products with no events. The family's normal week has no
  Saturday session, so the row deletes nothing at runtime; it ships because the
  venue tables of design memo D17 are the date-by-date intersection of the
  families that route to a venue, and a family that silently omitted an audited
  closure would drop it from every venue that includes this one.
- **Columbus Day and Veterans Day are not CME Globex holidays.** They appear
  nowhere in CME's own Globex holiday list and Globex trades a normal session on
  both. Coverage is contiguous, so they read as audited normal, which is what
  this note makes explicit rather than implicit.

**Gaps, 2025.**

- **Intraday topology, 2025-11-28 — unrepresentable, sourced.** The finalised
  publication additionally prints `07:00 preopen; 07:30 open` on a trading day
  that opened at 17:00 CT on 2025-11-27, i.e. a pause and re-open inside a
  running session. The crate's scalar vocabulary states a first open and a final
  close only, so the early close ships and the morning pair does not. No
  `is_open` answer before 07:00 CT is asserted as a halt. LAW-HOLIDAY-SCOPE
  records this as a gap; design memo §1.3 and §5.4 item 7 name it.
- **Order-entry deviations — unrepresentable, sourced, no `is_open`
  consequence.** On 2025-01-01 and 2025-12-25 the pre-open feeding the next
  trade date starts at 16:00 CT instead of the family's normal 16:45 CT. The
  built-in table copies `DayPolicy`'s vocabulary, which has no order-entry
  boundary, so the deviation is recorded and not modelled. It changes
  `is_accepting_orders` for 45 minutes and `is_open` never.
- **Residual risk — pre-finalisation publication.** The eight windows from New
  Year 2025 through Labor Day 2025 rest on the single archive capture
  `2024-12-20T15:53:40Z`, and CME states on the same page that trading hours are
  usually finalised about two weeks before the holiday. The service's retention
  edge falls between Labor Day 2025 and Thanksgiving 2025 — probed live on
  2026-09-12, those eight windows return the products with empty schedules — so
  the channel cannot restate them. Their instants are internally consistent with
  the 2026 and 2027 rows for the same holidays, which are sourced from
  post-holiday captures and from live reads. Closing condition: any later CME
  artifact covering those windows.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — no final close of this date is published | `CME-SVC-2025-12-31` — capture `2026-06-19T11:41:05Z` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02; the eve eventDate 2025-12-31 prints `16:00 closed` for CME trade date 2025-12-31 |
| 2026-01-19 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2026-01-18` — capture `2026-06-19T11:41:05Z` | T2 | eventDate 2026-01-19, CME trade date 2026-01-20 |
| 2026-02-16 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2026-02-15` — capture `2026-06-19T11:41:05Z` | T2 | eventDate 2026-02-16, CME trade date 2026-02-17 |
| 2026-04-03 | early close | `10:15 closed` — 10:15 CT | `CME-SVC-2026-04-01` — capture `2026-06-19T11:41:18Z` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03 |
| 2026-05-25 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2026-05-24` — capture `2026-06-19T11:41:05Z` | T2 | eventDate 2026-05-25, CME trade date 2026-05-26 |
| 2026-06-19 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-06-18` — capture `2026-06-19T11:34:04Z` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22 — see the trade-date note below |
| 2026-07-03 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-07-03` — capture `2026-06-19T11:41:08Z` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 — see the trade-date note below |
| 2026-09-07 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2026-09-06` — live `2026-09-12T04:30Z` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08 |
| 2026-11-26 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2026-11-25` — live `2026-09-12T04:30Z` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27 |
| 2026-11-27 | early close | `12:15 closed` — 12:15 CT | `CME-SVC-2026-11-25` — live `2026-09-12T04:30Z` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:15 closed` — 12:15 CT | `CME-SVC-2026-12-22` — live `2026-09-12T04:30Z` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24; no evening re-open is published |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` — live `2026-09-12T04:30Z` | T2 | eventDate 2026-12-25 |

**Interpretive steps, 2026.**

- **Good Friday 2026 is the year rates keep trading.** CME's own page states
  `Due to the US Employment Situation Release on April 3, 2026, CME Group
  Equities, FX, Cryptocurrency and Interest Rate products will have unique
  Closes and Settlements for trade date April 3rd.` That statement
  (`CME-TH-PAGE`, T1) corroborates that this family traded on a day every other
  CME family was shut, but it states no instant. The 10:15 CT close comes only
  from the T2 service window `CME-SVC-2026-04-01`, which is what the row cites.
  The equity-index line closes 08:15 CT the same day, so the two families do not
  share this row.
- **Saturday 2026-04-04 carries no session** for this family; the service was
  read for the 2026-04-02 .. 2026-04-04 window and publishes no Saturday events.
- **The trade-date divergence on 2026-06-19 and 2026-07-03.** CME prints a
  `12:00 closed` event on the holiday Friday but gives it the following Monday's
  trade date, so by CME's reckoning the Friday has no trade date of its own. The
  crate assigns a session the venue-local date of its final close, and this
  family has no following-business-day roll — only the cryptocurrency and
  event-contract identities do — so the shortened session that opened Thursday
  at 17:00 CT keeps trade date Friday here. Trading is modelled exactly as
  published; only the label differs. Recorded as a gap below.

**Gaps, 2026.**

- **The Saturday sessions of 2026-06-20 and 2026-07-04 — unrepresentable,
  sourced, executable hours.** Both print `05:00 open; 17:00 closed` carrying
  the following Monday's trade date, on a grid whose normal week has no Saturday
  session at all. A late open can only push an existing occurrence later; it
  cannot create one, so these sessions are not encoded and the crate reports the
  Saturday closed. Design memo §1.5 and §5.4 item 8; the natural first customer
  for the memo's v2 block rows (#93).
- **The rolled trade date on 2026-06-19 and 2026-07-03 — unrepresentable,
  sourced, labelling only.** As above: the crate's trade date for the shortened
  Friday is the Friday, CME's is the following Monday. `is_open`,
  `session_bounds` and the daily candle are unaffected; `trade_date` and
  anything keyed on it differ from CME's own numbering for that one session.
- **No T1 per-asset-class rendering.** From the 2025 calendar year CME publishes
  its holiday hours only as the interactive table on `trading-hours.html`, whose
  holiday selector could not be driven from a URL; the per-holiday PDFs that
  exist carry settlement and clearing information and state no trading hours. So
  2025-2027 is a T2 block by construction. The one T1 rendering that was
  captured — Thanksgiving 2026 — matches the service group for group, which is
  the validation this tier rests on.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-01-01; the eve eventDate 2026-12-31 prints `16:00 closed` for CME trade date 2026-12-31 and no evening re-open |
| 2027-01-18 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2027-01-17` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19 |
| 2027-02-15 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2027-02-14` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-03-26; the eve eventDate 2027-03-25 prints `16:00 closed` for CME trade date 2027-03-25 and no evening re-open |
| 2027-05-31 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2027-05-30` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01 |
| 2027-06-18 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2027-06-17` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21 — the same trade-date divergence as 2026-06-19 |
| 2027-07-05 | early close | `13:30 preopen` — 13:30 CT | `CME-SVC-2027-07-04` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06 |
| 2027-09-06 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2027-09-05` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07 |
| 2027-11-25 | early close | `12:00 preopen` — 12:00 CT | `CME-SVC-2027-11-24` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26 |
| 2027-11-26 | early close | `12:15 closed` — 12:15 CT | `CME-SVC-2027-11-24` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` — live `2026-09-12T04:30Z` | T2 | eventDate 2027-12-24; the CME holiday date is Thursday 2027-12-23, whose eventDate prints `16:00 closed` for CME trade date 2027-12-23 and no evening re-open |

**Interpretive steps, 2027.**

- **Independence Day observed, 2027-07-05, halts at 13:30 CT, not noon.** CME
  prints `13:30 preopen` for this family alongside Energy and Metals, and no
  equity-index line shares the row. The noon default of the other Monday
  holidays is not assumed anywhere.
- **Christmas 2027 shuts the Friday, not the Thursday.** CME's holiday date is
  Thursday 2027-12-23, which trades to its ordinary 16:00 CT close; the Globex
  closure is Friday 2027-12-24, which is the row. 2027-12-25 is a Saturday.
- **2027-12-31 is the last audited trade date** and is audited normal: CME
  publishes a schedule for the 2027-12-30 .. 2028-01-02 window in which this
  family's Friday is its ordinary one, and the 2028-01-01 Saturday carries no
  session. Nothing above 2027-12-31 is encoded, so a later known holiday — MLK
  2028, for instance — is deliberately **not** applied.

**Gaps, 2027.**

- **The Saturday session of 2027-06-19 — unrepresentable, sourced, executable
  hours.** `05:00 open; 17:00 closed` carrying trade date 2027-06-21, on a grid
  whose normal week has no Saturday session. Same class as 2026-06-20 and
  2026-07-04.
- **The rolled trade date on 2027-06-18 — unrepresentable, sourced, labelling
  only.** Same class as 2026-06-19 and 2026-07-03.
- **Eurex-style preliminary calendars do not apply here**, but the general rule
  does: CME's 2027 rows are the operator's published future, stated
  unconditionally, which LAW-NO-FABRICATED-DATES permits encoding ahead of the
  effective day. They record the schedule as published when retrieved on
  2026-09-12 (UTC) and are not a guarantee that it holds: each must be
  revalidated against the operator before its effective day under LAW-WATCH,
  and a revised or withdrawn date is corrected as a schedule fix.


## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20080121.html> — CME Globex notice 20080121, the January-2008 CBOT migration notice establishing the 17:30–16:00 CT schedule.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090326.html> — CME Globex notice 20090326, the 2009 table that pins the audit-floor queues at Sunday 16:15 and weekdays 16:50.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20110926.html> — CME Globex notice 20110926, the 2011-10-02 revision's source.
- <https://www.cmegroup.com/notices/electronic-trading/2018/04/20180409.html> — CME Globex notice 20180409, the SOFR launch.
- <https://www.cmegroup.com/trading/interest-rates/files/us-treasury-futures-delivery-process.pdf> — CME U.S. Treasury futures delivery-process guide.
- <https://www.cmegroup.com/markets/interest-rates/stirs/30-day-federal-fund.contractSpecs.html> — CME 30-Day Federal Funds contract specification, current grid.
- <https://www.cmegroup.com/education/articles-and-reports/understanding-sofr-futures> — CME SOFR futures explainer.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120505161543/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.xls> — CME Memorial Day 2012 holiday workbook — capture 2012-05-05, every complex's Sunday pre-opening still at 16:15.
- <https://web.archive.org/web/20120616195651/http://www.cmegroup.com:80/trading_hours/interest-rates-hours.html> — CME interest-rate trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11.
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
- **scope** — individual contract launch dates (SOFR joined the already-live family in May 2018) are catalog facts, not revisions of this product-neutral family clock.

## Module narrative (moved from src/calendar/schedules/futures/us/interest_rates.rs on 2026-09-12 UTC)

The January-2008 CBOT migration notice establishes the 17:30-16:00 CT
schedule inherited by the January-2010 audit-floor Treasury and 30-Day Fed
Funds family. CME moved every legacy CBOT interest-rate open to 17:00 CT
effective Sunday 2011-10-02 (trade date Monday 2011-10-03), aligning the
family with the current 17:00-16:00 grid. The 2009 table also pins the
audit-floor queues at Sunday 16:15 and weekdays 16:50; CME moved the weekday
queue to 16:45 on 2010-11-15. Current material publishes a Sunday 16:00
queue, but no primary source states the exact day on which 16:15 moved to
16:00: the holiday workbook updated 2012-05-03 still schedules every
complex's Sunday pre-opening at 16:15, the interest-rate hours page crawled
2012-06-16 already shows 16:00, and no notice in between states the day.
The fixed-current profile includes the exact current queue. The dated
selector retains the sourced audit-floor queue, then omits only that Sunday
phase after the exact 2011 matching-open revision rather than inventing a
queue cutover.

SOFR joined this already-live
family in May 2018; individual contract launch dates remain catalog facts,
not separate revisions of this product-neutral family clock.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20080121.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090326.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20110926.html
https://www.cmegroup.com/notices/electronic-trading/2018/04/20180409.html
https://www.cmegroup.com/trading/interest-rates/files/us-treasury-futures-delivery-process.pdf
https://www.cmegroup.com/markets/interest-rates/stirs/30-day-federal-fund.contractSpecs.html
https://www.cmegroup.com/education/articles-and-reports/understanding-sofr-futures
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
https://web.archive.org/web/20120505161543/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.xls
https://web.archive.org/web/20120616195651/http://www.cmegroup.com:80/trading_hours/interest-rates-hours.html

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
The two profiles above already carry Sunday 16:15; this one dropped it purely
because the 16:15->16:00 day is undated. It now keeps the same intersection.
