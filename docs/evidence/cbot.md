<!-- SPDX-License-Identifier: MIT-0 -->

# `cbot` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`grains.rs`](../../src/calendar/schedules/futures/us/grains.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for standard-size CBOT grain/oilseed futures. Current matching, morning Pre-Open, and PCP envelopes are primary-supported, and every exact matching/RTH revision is dated. The 2013-03-22 operator notice dates the Sunday 16:00-19:00 and Monday-Thursday 16:45-19:00 queues and the 14:30-16:00 PCP to 2013-04-07; only the 21-hour 2012-05-20..2013-04-06 regime's queue states remain undocumented. Mini grains are excluded.

## Revision rows

- 2010-04-19 — T1 — CME Globex notice 20100405 — the afternoon PCP expands to 13:15:30–16:00 CT.
- 2011-12-27 — T1 — CFTC filing rul120711cbot001 — the weekday morning queue moves to 08:00 CT.
- 2012-05-20 — T1 — CME market-data advisory 20120518 — matching expands to 17:00–14:00 CT.
- 2013-04-07 — T1 — CME SER-6617 and GCC notice 2013-03-22 — 19:00–07:45 CT electronic session around an 08:30–13:15 CT day session, with the full queue set.
- 2013-08-18 — T1 — CME market-data advisory 20130812 — the morning Pre-Open widens from 08:15 to 08:00 CT.
- 2015-07-05 — T1 — CME SER-7395R — the day-session close moves to 13:20 CT.

## Holidays

**Coverage:** 2025-01-01 .. 2027-12-31 (inclusive venue-local trade dates in
`America/Chicago`). Tier: **T2** throughout. Inside the window a date with no row is
audited and normal; outside it this table has no answer at all.

**This table is derived, not retrieved.** It is the **intersection** of the two
families that route to `Exchange::Cbot` — `globex_grains` and
`globex_interest_rates` — and every id it cites resolves to the same CME Group
`trading-hours-by-product` artifact those families' evidence files already record.
There is no CBOT evidence of its own to add, and the per-family rows with their
event-date-to-trade-date conversions are in those two files.

**The intersection rule (design memo D17).** The two families trade the same
building around different sessions, and the day session is where they touch. On a
full Globex closure both are closed and the venue states it: **nine dates** in this
window qualify, and they are the only rows here with a sourced **closure** status.
The other **thirty-one** dates carry `unsourced`, in two shapes:

- **Twenty-seven are close disagreements.** On every CBOT holiday early close the two
  families move by different amounts, and by a different pair on different dates: the
  grain and oilseed day session ends at 12:05 CT on 2025-11-28, 2025-12-24,
  2026-11-27, 2026-12-24 and 2027-11-26, while the interest-rate overnight leg halts
  at 12:15 CT on those five dates and at 12:00 CT, 10:15 CT or 13:30 CT on the others.
  Five minutes apart or three hours apart, a difference is a difference, so the venue
  states no instant.
- **Four are late-open-only.** 2025-01-02, 2025-12-26, 2026-01-02 and 2027-07-06 open
  the grain day session at 08:30 CT with no prior-evening leg, while
  `globex_interest_rates` states nothing at all: its overnight leg runs straight
  through. One family states a row and the other has audited the date normal, which is
  a different answer rather than a missing one, so the venue cannot state the late open
  as its own either.

**`unsourced` is neither silence nor a compromise.** The coverage window is
contiguous, so a date carrying no row is the positive claim that it was audited
normal, which is false on every one of these dates. Nor is a compromise available: on
2025-11-28 a row at 12:05 CT would leave the rates family trading ten minutes after
the venue said it stopped, and a row at 12:15 CT would cut the grain day session
short at its own sourced close. `unsourced` clips nothing and tells a caller
what the crate knows. `iceus`, whose venue table shipped first, is the precedent.

**Cite the family, not the venue, for holiday behaviour.** A caller that needs
CBOT holiday hours should read `globex_grains` for the day session and
`globex_interest_rates` for the overnight leg; this table states only what both agree
on.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `no events published` | `CME-SVC-2024-12-31` | T2 | grains closed; interest rates closed |
| 2025-01-02 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2024-12-31` | T2 | grains late open 08:30 CT; no row in interest rates |
| 2025-01-20 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-01-19` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-02-17 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-02-16` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | grains closed; interest rates closed |
| 2025-05-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-05-25` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-06-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-06-18` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-07-04 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-07-03` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-09-01 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-08-31` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-11-27 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-11-26-SAT` | T2 | grains closed; interest rates early close 12:00 CT |
| 2025-11-28 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-11-26` | T2 | grains late open 00:10 CT and early close 12:05 CT; interest rates early close 12:15 CT |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | grains closed; interest rates closed |
| 2025-12-24 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-24` | T2 | grains early close 12:05 CT; interest rates early close 12:15 CT |
| 2025-12-25 | closed | `no events published` | `CME-SVC-2025-12-24` | T2 | grains closed; interest rates closed |
| 2025-12-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-24` | T2 | grains late open 08:30 CT; no row in interest rates |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-2025-12-31` | T2 | grains closed; interest rates closed |
| 2026-01-02 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-31` | T2 | grains late open 08:30 CT; no row in interest rates |
| 2026-01-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-01-18` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-02-16 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-02-15` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-04-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-04-01` | T2 | grains closed; interest rates early close 10:15 CT |
| 2026-05-25 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-05-24` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-06-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-06-18` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-07-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-07-03` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-09-07 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-09-06` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-11-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-11-25` | T2 | grains closed; interest rates early close 12:00 CT |
| 2026-11-27 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-11-25` | T2 | grains late open 00:10 CT and early close 12:05 CT; interest rates early close 12:15 CT |
| 2026-12-24 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-12-22` | T2 | grains early close 12:05 CT; interest rates early close 12:15 CT |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | grains closed; interest rates closed |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | grains closed; interest rates closed |
| 2027-01-18 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-01-17` | T2 | grains closed; interest rates early close 12:00 CT |
| 2027-02-15 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-02-14` | T2 | grains closed; interest rates early close 12:00 CT |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | grains closed; interest rates closed |
| 2027-05-31 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-05-30` | T2 | grains closed; interest rates early close 12:00 CT |
| 2027-06-18 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-06-17` | T2 | grains closed; interest rates early close 12:00 CT |
| 2027-07-05 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-07-04` | T2 | grains closed; interest rates early close 13:30 CT |
| 2027-07-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-07-04` | T2 | grains late open 08:30 CT; no row in interest rates |
| 2027-09-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-09-05` | T2 | grains closed; interest rates early close 12:00 CT |
| 2027-11-25 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-11-24` | T2 | grains closed; interest rates early close 12:00 CT |
| 2027-11-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-11-24` | T2 | grains late open 00:10 CT and early close 12:05 CT; interest rates early close 12:15 CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | grains closed; interest rates closed |

**Gaps, 2025-2027.**

- **The thirty-one `unsourced` dates are the intersection's residue, not a research
  gap.** Each is a date on which the two families disagree — by stating different
  rows, or by one of them stating a row while the other states none, which is the
  `late open` shape of 2025-01-02, 2025-12-26, 2026-01-02 and 2027-07-06. A family
  with no row has **audited the date normal**, which is a different answer rather
  than a missing one. The disagreement is printed per date in the `Derived from`
  column above. Every
  underlying row is sourced; what is missing is a single venue-wide answer, which no
  operator document states and which the crate will not invent. Closing condition: an
  operator statement of CBOT-wide holiday hours — CME's holiday-hours table on
  `cmegroup.com/trading-hours.html` is per asset class — or a `DayPolicy`-shaped
  boundary that can express a per-family answer inside one venue calendar.
- **The four single-family `late open` dates.** 2025-01-02, 2025-12-26, 2026-01-02 and
  2027-07-06 carry a `globex_grains` late open at 08:30 CT and nothing on the rates
  side. The interest-rate family runs a continuous overnight leg through those dates,
  so for it no boundary moves. The venue therefore cannot state the grain late open as
  its own. It is also inert for the `cbot` venue calendar, whose grains-backed profile
  already opens its day session at 08:30 CT; it is recorded because a venue row must be
  true of every routed family.
- **The window is the families' window, not the venue's horizon.** Coverage starts
  2025-01-01 because that is where both families' tables start. The 2010-2024 blocks
  land in stage 2.2's waves, each of which extends this table over its own years by
  the same derivation; the cross-wave agreement audit (#95) closes with the last of
  them.
- **2028-01-01 ships no row.** CME's service publishes a 2027-12-30 .. 2028-01-02
  window and both families' tables end at 2027-12-31, so the venue's window ends there
  too. Extending it is stage 2.4's refresh, not a gap in this change.

**Interpretive steps, 2025-2027.**

- **The routing is read from production, not assumed.** `hours_for_exchange`'s
  `Exchange::Cbot` arm resolves to `cbot_profile_at`, and `globex_grains` and
  `globex_interest_rates` are the two `MarketHoursKey` values that resolve to the same
  schedules the venue profile is built from. The test
  `the_venue_table_is_the_intersection_of_its_families` recomputes the whole table
  from the two families' public `holiday_on` answers on every run, so the routing list
  and the table cannot drift apart silently.
- **A disagreement ships `unsourced` rather than nothing.** This follows `iceus`,
  which shipped first and states the reason: with a contiguous coverage window,
  silence is a claim. The row is cited to a document the crate holds, because the
  `holidays!` fence requires a non-empty id and because the id is what lets a reader
  reach the bytes behind the disagreement.
- **`cbot`'s ledger cadence moves to `monthly` in this change.** LAW-WATCH makes a
  served identity that ships a holiday table a monthly review, and the operator
  republishes its calendar yearly and issues errata.

### Documents

Every id below resolves to one response of CME Group's own
`trading-hours-by-product` service — the endpoint `cmegroup.com/trading-hours.html`
itself calls — for the `THBP-A` product set
`id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`.
Archived responses were replayed through `https://web.archive.org/web/<timestamp>id_/`;
live responses were read through the public reader `https://r.jina.ai/`, because
cmegroup.com returns HTTP 403 to the retrieving machine. Paths are relative to
`exchange-hours-research/holidays/`. The table is the same one the routed families'
evidence files carry — this venue rests on no artifact they do not — and several ids
resolve to one artifact, because one service window answers two or three trade dates.
The research-store id of each artifact is in the `Derived from` column of the year
tables in the family files.

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
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the pre-expansion grain grid with a 16:15 Sunday Pre-Open.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, the expanded 17:00–14:00 grid with a 16:00 Sunday Pre-Open.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the queue and PCP states of the 21-hour 2012-05-20..2013-04-06 regime have no operator-stated onset day. The 2026-08-31 review sourced the states themselves from CME's own trading-hours captures of 2012-05-28 and 2012-06-07 against the pre-expansion 2012-05-11 capture, which brackets the switch to 2012-05-11..2012-05-28; CME market-data advisory 20120518 states only the new matching hours and never the queue times, so no queue revision is keyed to 2012-05-20. Closing condition: a CME document that states those queue times in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the baseline queue and PCP phases rest on the operator's March-2010 market-state table, which states them as then-live rather than dating them, so they are carried back from 2010-03-15 to the January-2010 floor.
- **scope** — mini-sized Corn, Soybean, Wheat and KC HRW Wheat diverged on 2012-09-16 and are owned by `mini_grains.rs`; Rough Rice borrows the pre-2018 eras but owns its own timeline in `rough_rice.rs`.
- **holidays** — this venue ships the intersection of the families that route to it; see `## Holidays` above. Every date the intersection drops is named there, and the cross-wave agreement audit that memo §7 follow-up 10 asks for (#95) is still open: it closes with the last stage-2.2 family wave, when the same assertion can be re-run over 2010-2027 rather than over this window alone.

## Module narrative (moved from src/calendar/schedules/futures/us/grains.rs on 2026-09-12 UTC)

This profile is deliberately limited to standard-size CBOT grain and oilseed
futures. Mini-sized Corn, Soybean, Wheat, and KC HRW Wheat diverged on
2012-09-16 and are not represented by this key; `mini_grains.rs` owns their
sourced timeline.

The `CBOT_*` rule tables below are `pub(crate)` because Rough Rice
(`rough_rice.rs`) ran on this same grid until its 2018-01-21 divergence and
borrows the pre-divergence eras rather than copying them. That key owns its
own `StaticHoursProfile` values and its own timeline, so a future Rough
Rice-specific finding repoints one of its eras instead of editing anything
here. Nothing in this file may be changed on Rough Rice evidence.

At the January-2010 audit floor, matching ran 18:00-07:15 around the
09:30-13:15 RTH. The operator's March-2010 market-state table supplies the
then-live 16:15-18:00 Sunday, 07:15-09:30 weekday, and 14:30-16:00 PCP
phases. On 2010-04-19 PCP expanded to 13:15:30-16:00. The CFTC filing makes
the weekday morning queue's move to 08:00 effective Tuesday 2011-12-27.
A later generic Globex notice broadly names CBOT in an afternoon queue
change, but it does not enumerate this family and conflicts with the complete
family-specific state table. No separate evening queue is inferred from it.
https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2010-62.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf

Matching expanded to 17:00-14:00 on 2012-05-20. SER-6617 then established
19:00-07:45 and 08:30-13:15 effective Sunday 2013-04-07. CME expanded the
exact morning Pre-Open to 08:00-08:30 on 2013-08-18, and SER-7395R moved the
RTH close to 13:20 on 2015-07-05.
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html

The 22 March 2013 Global Command Center client notice that carried the
SER-6617 change also states every current queue's unconditional onset:
"Effective Sunday, April 7, 2013 (trade date Monday, April 8) ... Pre-Opens
(including MGEX): Sunday night: 16:00-19:00 CT / Monday-Thursday night:
16:45-19:00 CT / Monday-Friday morning: 08:15-08:30 CT. Post Close Pre-Open:
Monday-Friday: 14:30-16:00 CT", with a 07:45-08:15 cancellation-only slice
inside the break that no order-entry rule models. Only the 21-hour
2012-05-20..2013-04-06 regime's queue and PCP states remain undocumented, so
2026-08-31 grains-regime review — the states are now sourced, only their
switch-on day is not. CME's own trading-hours pages inside the 21-hour
regime publish, for Corn/Wheat/Soybean/Soybean Oil futures and options,
Sunday Pre-Open 16:00, weekday Pre-Open "14:30-16:00, 16:45-17:00" (the PCP
plus the evening queue) and ETH 17:00-14:00. The pre-expansion capture of
2012-05-11 shows the other side: Sunday Pre-Open 16:15, weekday
"14:30-16:00 16:45 08:00" and ETH 18:00-07:15, 09:30-13:15. The switch is
therefore bracketed to 2012-05-11..2012-05-28, which contains the sourced
2012-05-20 expansion — but Advisory #20120518 states only the new matching
hours, never the queue times, so no queue revision is keyed to that day.
Official origin http://www.cmegroup.com/trading_hours/ delivered via:
https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
that dated profile conservatively omits them instead of inventing onsets.
https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf

ORDER-ENTRY CLASSIFICATION. The comment above distinguishes the matching
windows from the market-state phases the operator's tables publish around
them. Only the matching windows can print a trade, so the Sunday evening
queue, the weekday morning queue (07:15, later 08:00, briefly 08:15 from
2013-04-07, back to 08:00 from 2013-08-18, up to the day-session open), and
the afternoon PCP are `order_entry`; the electronic session and the
post-2012 afternoon matching slice stay `extended`.

Revision evidence — each row's day-level effective date and the primary
source that states it (full quotations sit in the blocks above):
  2010-04-19 "CME Globex notice 20100405"
    https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
  2011-12-27 "CFTC filing rul120711cbot001"
    https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
  2012-05-20 "CME market-data advisory 20120518"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
  2013-04-07 "CME SER-6617 and GCC notice 2013-03-22"
    https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
    https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
  2013-08-18 "CME market-data advisory 20130812"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
  2015-07-05 "CME SER-7395R"
    https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
