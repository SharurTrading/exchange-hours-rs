<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_nikkei_225_dollar` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cme_nikkei.rs`](../../src/calendar/schedules/futures/us/cme_nikkei.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Nikkei 225 Dollar (`NKD`) only. Current 17:00→16:00 CT grid sourced from the contract specification, with a dated timeline from 2012-11-18: SER-6465 (session-opening day 2012-11-18) extended the close to 16:15 CT with a 15:15–15:30 CT halt, SER-6554R (2013-03-03) removed that halt for International Equity Index futures naming NKD explicitly, and CME Globex Notice #20150817 (2015-09-20, trade date 2015-09-21) moved the CME Equity close to 16:00 CT. Partial because the pre-2011 interval is omitted rather than modelled. The 2026-09-01 review found why it must be: CME's own trading-hours pages captured 2010-03-10 and 2010-04-07 show a **materially different** NKD grid — CDT 03:00–15:15 reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15 with **no Sunday hours** — so the 17:00–15:15 continuous grid cannot be carried across 2010 without reporting the contract open all night when it was closed. An earlier revision of this branch did exactly that; it is corrected. The changeover is undated (2010-04-07 still shows the old grid, 2011-01-12 already shows the new one, with no capture or located notice between), so dates before the first sourced appearance of the served grid are sessionless and the 2010 grid is left sourced-but-unmodelled — encoding it would need seasonal CDT/CST rules and a boundary that is still undated. CME's trading-hours pages captured 2011-01-12 onward state the served grid — Sunday Pre-Open 16:15, ETH (Sunday) 17:00-15:15, weekday Pre-Open "15:25, 16:45", ETH (Weekday) "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row on the same page — so the pre-2012 evening open is 17:00 CT and is primary-sourced. What stays undated is when that grid began, since the 2010 change is attested only by a third-party aggregator; keying a revision to a capture date would fabricate a cutover. [Sentence removed at the 2026-09-12 migration: it claimed the pre-2012 grid was carried to the January-2010 floor, which contradicts `nkd_profile_at` (NKD_CLOSED before 2011-01-12).]

## Revision rows

- 2011-01-12 — T1 — first sourced CME trading-hours capture of this grid — knowledge boundary: 17:00–15:15 CT with the 15:30–16:30 CT post-halt segment.
- 2012-11-18 — T1 — CME SER-6465 — the close is extended to 16:15 CT with a 15:15–15:30 CT electronic halt.
- 2013-03-03 — T1 — CME SER-6554R — the 15:15–15:30 CT halt is removed for International Equity Index futures, naming NKD explicitly.
- 2015-09-20 — T1 — CME Globex notice 20150817 — the CME Equity close moves to 16:00 CT for trade date Monday 2015-09-21.

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive trade dates). Tier: T2 throughout —
CME's own trading-hours service, read as bytes and saved. No T1 per-asset-class rendering
exists for these years; `cme-2025-2027.json` `missing[3]` records that the operator's
trading-hours page renders only the next upcoming holiday and its holiday selector cannot
be driven from the URL.

**Channel.** Every row below comes from
`https://www.cmegroup.com/services/trading-hours-by-product?id=<set>&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=<from>&toEventDate=<to>`
— the endpoint `cmegroup.com/trading-hours.html` itself calls to render its per-asset-class
Holiday Hours table. That is the operator's own machine channel, so **T2** under
LAW-PRIMARY-SOURCES, not T1. Two product-id sets are in play. `THBP-A` is
`id=316,133,425,300,58,437,22,8478,5201,10191` — the ten headline products CME's own page
queries: `ES`, `ZN`, `6E`, `CL`, `GC`, `ZC`, `LE`, `CSC`, `BTC`, `LBR`. `THBP-B` is
`id=168,167,320,323,19,27` — `NKD`, `NIY`, `ZS`, `ZW`, `HE`, `DC`, and it is the only set
that carries the Nikkei line. Document ids taken from `THBP-B` carry a `-B-` infix;
unsuffixed `CME-SVC-` ids are `THBP-A` captures.

**Zone.** Quoted verbatim from the operator's page: "Trading hours are subject to change
and are in U.S. Central Time unless otherwise stated." CME prints no ET column in this
channel, so no ET value is asserted here and none is bracketed in.

**Event vocabulary**, verbatim from the same page: `preopen` — "Order Entry, modification,
and cancel are allowed. No order matching."; `open` — "Start of continuous trading phase.
Order matching begins."; `closed` — "Final Close of the date. Day and GTD (current trade
date) orders are eliminated." `/TD` below is CME's own `tradingDate` field on each event.

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
| `CME-SVC-B-2025-11-26` | 2025-11-26 .. 2025-11-29 | live retrieval 2026-09-12T08:54:32Z | T2 | `50da5636342b8352826debc016850594cc84bc2d50ac0f3121f1c79843435fa2` |
| `CME-SVC-B-2025-12-24` | 2025-12-24 .. 2025-12-26 | live retrieval 2026-09-12T08:54:56Z | T2 | `11dc4de5bf662e60d6bf71e37adb6247e217433991e1b29af0cbbf973c9288fd` |
| `CME-SVC-B-2025-12-31` | 2025-12-31 .. 2026-01-02 | live retrieval 2026-09-12T08:54:57Z | T2 | `c558c9f399eb1b83b55f6dd1c00dac81a8a55e8d25b5eaa9cecd1b6bb229a216` |
| `CME-SVC-B-2026-01-18` | 2026-01-18 .. 2026-01-20 | live retrieval 2026-09-12T08:54:58Z | T2 | `15f55c10115e7a37cf85f0583e577c2fb7421b08a545ee03f3c0261e46c5d09d` |
| `CME-SVC-B-2026-02-15` | 2026-02-15 .. 2026-02-17 | live retrieval 2026-09-12T08:54:58Z | T2 | `5bbecf07eecbdb2dbe5325f39f6b464bf817f535d38841fccdafef19103aaef0` |
| `CME-SVC-B-2026-04-01` | 2026-04-01 .. 2026-04-03 | live retrieval 2026-09-12T08:55:08Z | T2 | `b4569c685450baab17749fbed20c8c0e37c40910b77c411f72b6c596cd3c0567` |
| `CME-SVC-B-2026-05-24` | 2026-05-24 .. 2026-05-26 | live retrieval 2026-09-12T08:55:09Z | T2 | `3c7628b898d8069067836a36c44769f2f2b76a1dee5edbd48225f76b363881d0` |
| `CME-SVC-B-2026-06-18` | 2026-06-18 .. 2026-06-20 | live retrieval 2026-09-12T08:55:10Z | T2 | `41791460e029bb7db0d280374048c36f015594fb71e9f1aae86a6145429ce2e5` |
| `CME-SVC-B-2026-07-03` | 2026-07-03 .. 2026-07-05 | live retrieval 2026-09-12T08:55:11Z | T2 | `dfc4aff36f0e44fb8fdb78de59d13bad90707c0d108673094ad0a012cefad898` |
| `CME-SVC-B-2026-09-06` | 2026-09-06 .. 2026-09-08 | live retrieval 2026-09-12T04:30Z | T2 | `f7cc43f8d90b571b945901f826277ca43ec21c4438c36bbb26e231c859a83923` |
| `CME-SVC-B-2026-11-25` | 2026-11-25 .. 2026-11-27 | live retrieval 2026-09-12T04:30Z | T2 | `f6007a75d6009dada85fe6c57d660598f21ed8a8385364c454ee015565f94dd4` |
| `CME-SVC-B-2026-12-24` | 2026-12-24 .. 2026-12-26 | live retrieval 2026-09-12T04:30Z | T2 | `b622712c1c45c3efb90443172617636ba4c7d7dfb4c6851b70a22ee1c9fa978d` |
| `CME-SVC-B-2026-12-31` | 2026-12-31 .. 2027-01-02 | live retrieval 2026-09-12T04:30Z | T2 | `b21ac047d6d67cb9026940a37ad345c7b2ece40e73dbca7be69cf7171646613c` |
| `CME-SVC-B-2027-01-17` | 2027-01-17 .. 2027-01-19 | live retrieval 2026-09-12T04:30Z | T2 | `3fc8c80ea1222cd0bb3d46c8a7df904acd3859441ffdfd9de6145625177f9bfc` |
| `CME-SVC-B-2027-02-14` | 2027-02-14 .. 2027-02-16 | live retrieval 2026-09-12T04:30Z | T2 | `c8ca83f4594756ed338368a0035b790e7a27efe6872506985ca6f7d16bcc97eb` |
| `CME-SVC-B-2027-03-25` | 2027-03-25 .. 2027-03-27 | live retrieval 2026-09-12T04:30Z | T2 | `022dcd1f61e54cc316a621e01f519bbb723a446c000323dd9725d2d6434effe8` |
| `CME-SVC-B-2027-05-30` | 2027-05-30 .. 2027-06-01 | live retrieval 2026-09-12T04:30Z | T2 | `1c28c61151bd26e844c5b2ea6f046102b6da45a5f88566e91a421604a8c566e6` |
| `CME-SVC-B-2027-06-17` | 2027-06-17 .. 2027-06-19 | live retrieval 2026-09-12T04:30Z | T2 | `011d4f666198a4427faa01d7e91ebf2412f4614ae27188ccd35f84c726a5d05d` |
| `CME-SVC-B-2027-07-04` | 2027-07-04 .. 2027-07-06 | live retrieval 2026-09-12T04:30Z | T2 | `1a9550357fbf3c1615fcbeefebbc64e271a6dfe0ad0ffdff10c770a7e2d77aaf` |
| `CME-SVC-B-2027-09-05` | 2027-09-05 .. 2027-09-07 | live retrieval 2026-09-12T04:30Z | T2 | `9ceb6df48d2278a807fd2e3081828eb41dd207cc64c389078b3a529f762da928` |
| `CME-SVC-B-2027-11-24` | 2027-11-24 .. 2027-11-26 | live retrieval 2026-09-12T04:30Z | T2 | `90320ed581b1d09f6c1b85d98da3a7a097b09d5f4eaac4abbb3d9368ee1cfdfa` |
| `CME-SVC-B-2027-12-22` | 2027-12-22 .. 2027-12-25 | live retrieval 2026-09-12T04:30Z | T2 | `1ee3bd5fb9a99f765f96ac60377e8013cfca1c54ceba53beeb616d12862e2b12` |

The research store holds the bytes: the `D01`–`D08` captures under
`holidays/raw/cme-2025-2027/arc/`, `D21`–`D47` under
`holidays/raw/cme-2025-2027/live/extra/`, and `D54`–`D65` under
`holidays/raw/cme-2025-2027-repair/live/`, each with its own `INDEX.md` row.
`cme-2025-2027.verify.json` (round 2, 2026-09-12) is the verdict that governs; it re-parsed
all 313 family rows from the cited bytes with zero mismatches, and its four material
findings — the nine `THBP-B` windows the retrieval had wrongly declared missing among them —
are the reason the Nikkei line exists at all from Thanksgiving 2025 onward.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen /TD 2025-01-02; 17:00 open /TD 2025-01-02` — no close event on 2025-01-01 | `CME-SVC-2024-12-31` | T2 | eventDates 2024-12-31 and 2025-01-01; CME prints `16:00 closed /TD 2024-12-31` with no evening re-open, and assigns no trade date 2025-01-01 |
| 2025-01-20 | early close | `12:00 preopen /TD 2025-01-21; 17:00 open /TD 2025-01-21` — 12:00 CT | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME trade date 2025-01-21; the Sunday-evening leg opened 2025-01-19 17:00 CT |
| 2025-02-17 | early close | `12:00 preopen /TD 2025-02-18; 17:00 open /TD 2025-02-18` — 12:00 CT | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME trade date 2025-02-18 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDates 2025-04-17 and 2025-04-18; CME prints `16:00 closed /TD 2025-04-17` with no evening re-open |
| 2025-05-26 | early close | `12:00 preopen /TD 2025-05-27; 17:00 open /TD 2025-05-27` — 12:00 CT | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME trade date 2025-05-27 |
| 2025-06-19 | early close | `12:00 preopen /TD 2025-06-20; 17:00 open /TD 2025-06-20` — 12:00 CT | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME trade date 2025-06-20 |
| 2025-07-03 | early close | `12:15 closed /TD 2025-07-03; 16:45 preopen /TD 2025-07-04; 17:00 open /TD 2025-07-04` — 12:15 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-03, CME trade date 2025-07-03; the evening leg runs normally |
| 2025-07-04 | early close | `12:00 closed /TD 2025-07-04` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `12:00 preopen /TD 2025-09-02; 17:00 open /TD 2025-09-02` — 12:00 CT | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME trade date 2025-09-02 |
| 2025-11-27 | early close | `12:00 preopen /TD 2025-11-28; 17:00 open /TD 2025-11-28` — 12:00 CT | `CME-SVC-B-2025-11-26` | T2 | eventDate 2025-11-27, CME trade date 2025-11-28; `NKD` and `NIY` print this line themselves |
| 2025-11-28 | early close | `07:00 preopen /TD 2025-11-28; 07:30 open /TD 2025-11-28; 12:15 closed /TD 2025-11-28` — 12:15 CT | `CME-SVC-B-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28; the morning pre-open pair is the gap recorded below |
| 2025-11-29 | closed | `no events published` | `CME-SVC-B-2025-11-26` | T2 | eventDate 2025-11-29; `NKD` and `NIY` publish no events, and CME's 2025 Globex table states the period as "27 - 29 November 2025" |
| 2025-12-24 | early close | `12:15 closed /TD 2025-12-24` — 12:15 CT, no evening re-open | `CME-SVC-B-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen /TD 2025-12-26; 17:00 open /TD 2025-12-26` — no close event on 2025-12-25 | `CME-SVC-B-2025-12-24` | T2 | eventDates 2025-12-24 and 2025-12-25; the 2025-12-24 record's missing evening re-open is what this row removes |

**Interpretive steps, 2025.**

- **The Nikkei line is absent through Labor Day 2025, and those nine rows are taken from
  the Equity Index line of the same capture.** CME's service still answers for past windows
  back to Thanksgiving 2025 but no further, so `NKD`/`NIY` return empty schedules for the
  eight windows New Year 2025 through Labor Day 2025 (`raw/cme-2025-2027-repair/live/edgeB_*.md`,
  probed 2026-09-12). The rows for 2025-01-01, 2025-01-20, 2025-02-17, 2025-04-18,
  2025-05-26, 2025-06-19, 2025-07-03, 2025-07-04 and 2025-09-01 are therefore the `ES` line
  of the ten-product capture, and they ship with the `THBP-A` document id. The corroboration
  is direct rather than assumed: on every one of the 36 product-dates from Thanksgiving 2025
  to 2028-01-01 where CME publishes both lines, `NKD` and `NIY` match `ES` event for event
  and trade date for trade date, including the single date on which the Equity Index line
  diverges from its neighbours — Good Friday 2026, where `NKD`/`NIY` close 08:15 CT with `ES`
  and not 10:15 CT with `ZN`, `6E` and `BTC`. The date this inference carries most weight on
  is 2025-07-03, where `ES` closes 12:15 CT while `ZN`, `6E`, `CL`, `GC` and `BTC` all close
  16:00 CT; it is listed as a residual risk below.
- **Trade-date key, and where it parts company with CME's printed trade date.** The table is
  keyed by the crate's own venue-local trade date — the Chicago date of the containing
  session's final close (design memo D1). On the seven dates whose noon event CME publishes
  as a `preopen` rather than a `closed`, CME assigns the whole span the *following* business
  day's trade date, because no settlement occurs on the holiday. The crate keeps the holiday
  as the trade date, because a final close does occur there at 12:00 CT. `is_open` agrees
  with CME event for event on every one of those dates; only the label differs. Recorded as
  a residual risk below.
- **Eve records are evidence, not rows.** `16:00 closed /TD <eve>` with no `16:45 preopen`
  and no `17:00 open` — CME's `[N6]` shape, printed on 2024-12-31, 2025-04-17 and 2025-12-31 —
  is the family's ordinary daytime close plus a missing evening leg. The neighbouring
  `Closed` row already deletes that leg, so the eve carries no row of its own.

**Gaps, 2025.**

- **executable** — 2025-11-28 additionally prints `07:00 preopen /TD 2025-11-28; 07:30 open
  /TD 2025-11-28` before the 12:15 CT close, with no `closed` or `paused` event between the
  2025-11-27 17:00 CT open and that 07:00 pre-open. Read literally, matching stopped at some
  unstated instant that morning and resumed at 07:30 CT. That is intraday topology, not a
  scalar boundary, so LAW-HOLIDAY-SCOPE records it as a gap rather than approximating it;
  the early-close row is unaffected. It is the only date in the whole 2025-2027 window where
  this shape appears for this family — Thanksgiving 2026 and 2027 print the 12:15 CT close
  alone. Closing condition: an operator statement naming the morning halt's start, or the
  block rows of design memo §7 follow-up 8 (#93).
- **residual risk** — the nine 2025 rows through Labor Day 2025 rest on the Equity Index
  line rather than on a published Nikkei line, because CME's channel no longer answers for
  those windows and no archived capture of the `THBP-B` id set exists for them. Closing
  condition: an archived `THBP-B` capture of any of the eight windows, or a `THBP-A` capture
  that carries `NKD`.
- **residual risk** — the crate's trade date differs from CME's printed trade date on every
  `preopen`-at-noon holiday. No query the crate answers is wrong; a consumer comparing the
  crate's `trade_date` against a CME settlement file will see the holiday where CME shows the
  next business day.
- **not representable** — order-entry deviations. On 2025-01-01 and 2025-12-25 the pre-open
  starts at 16:00 CT rather than the normal 16:45 CT. The table's vocabulary is `DayPolicy`'s
  and has no order-entry boundary; the family models no order-entry phase at all, so this
  changes nothing the crate reports.
- **Saturday 2025-11-29 ships a row.** `CME-SVC-B-2025-11-26` publishes no events for it,
  the ten-product capture agrees, and CME's 2025 Globex table states the period as
  "27 - 29 November 2025", so it is a sourced closure. The family's grid has no
  Friday-evening open and therefore no Saturday trade date, so the row changes no answer
  here; it ships because one audited operator closure ships in every family that routes to
  the venue, so the D17 venue intersection is computed from one uniform input.
- **audited, no row** — Columbus Day and Veterans Day. CME publishes settlement and clearing
  advisories but no Globex trading schedule for either, and the service returns the normal
  grid. Coverage is contiguous, so those dates read as audited normal; that is stated here
  rather than left implicit.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-B-2025-12-31` | T2 | eventDates 2025-12-31 and 2026-01-01; CME prints `16:00 closed /TD 2025-12-31` with no evening re-open |
| 2026-01-19 | early close | `12:00 preopen /TD 2026-01-20; 17:00 open /TD 2026-01-20` — 12:00 CT | `CME-SVC-B-2026-01-18` | T2 | eventDate 2026-01-19, CME trade date 2026-01-20; the Sunday-evening leg opened 2026-01-18 17:00 CT carrying the same CME trade date |
| 2026-02-16 | early close | `12:00 preopen /TD 2026-02-17; 17:00 open /TD 2026-02-17` — 12:00 CT | `CME-SVC-B-2026-02-15` | T2 | eventDate 2026-02-16, CME trade date 2026-02-17 |
| 2026-04-03 | early close | `08:15 closed /TD 2026-04-03` — 08:15 CT | `CME-SVC-B-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03; `NKD` and `NIY` track the Equity Index instant here, not the 10:15 CT of `ZN`, `6E` and `BTC` |
| 2026-05-25 | early close | `12:00 preopen /TD 2026-05-26; 17:00 open /TD 2026-05-26` — 12:00 CT | `CME-SVC-B-2026-05-24` | T2 | eventDate 2026-05-25, CME trade date 2026-05-26 |
| 2026-06-19 | early close | `12:00 closed /TD 2026-06-22` — 12:00 CT | `CME-SVC-B-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22; the Thursday-evening leg opened 2026-06-18 17:00 CT |
| 2026-07-03 | early close | `12:00 closed /TD 2026-07-06` — 12:00 CT | `CME-SVC-B-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 |
| 2026-09-07 | early close | `12:00 preopen /TD 2026-09-08; 17:00 open /TD 2026-09-08` — 12:00 CT | `CME-SVC-B-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08 |
| 2026-11-26 | early close | `12:00 preopen /TD 2026-11-27; 17:00 open /TD 2026-11-27` — 12:00 CT | `CME-SVC-B-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27 |
| 2026-11-27 | early close | `12:15 closed /TD 2026-11-27` — 12:15 CT | `CME-SVC-B-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:15 closed /TD 2026-12-24` — 12:15 CT, no evening re-open | `CME-SVC-B-2026-12-24` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-B-2026-12-24` | T2 | eventDates 2026-12-24 and 2026-12-25; the 2026-12-24 record's missing evening re-open is what this row removes |

**Interpretive steps, 2026.**

- Every row is the family's own published line: `NKD` and `NIY` appear in the `THBP-B`
  captures for all twelve dates, so no inference from the Equity Index line is made anywhere
  in 2026.
- Good Friday 2026 is the date that proves the 2025 inference rather than resting on it.
  `NKD` and `NIY` print `08:15 closed /TD 2026-04-03`; `ZN`, `6E` and `BTC` print 10:15 CT on
  the same date. The Nikkei follows the Equity Index instant, which is what the 2025 rows
  assume.
- On 2026-06-19 and 2026-07-03 CME prints a real `closed` event at 12:00 CT and still labels
  it with the following Monday's trade date. The crate keys the row to the Friday, for the
  same reason as the 2025 `preopen` dates.

**Gaps, 2026.**

- **executable, not representable** — the Saturday sessions of 2026-06-20 and 2026-07-04.
  CME publishes `05:00 open /TD 2026-06-22; 17:00 closed /TD 2026-06-22` and
  `05:00 open /TD 2026-07-06; 17:00 closed /TD 2026-07-06`. The family's normal week has no
  Saturday session at all, and a late open can only push an existing occurrence later, never
  create one (design memo D7), so neither is a row. The crate reports these Saturdays closed.
  Closing condition: the block rows of design memo §7 follow-up 8 (#93).
- **residual risk** — the trade-date divergence described under 2025 applies to 2026-01-19,
  2026-02-16, 2026-05-25, 2026-06-19, 2026-07-03, 2026-09-07 and 2026-11-26.
- **audited, no row** — Saturday 2026-04-04, the day after Good Friday. CME's service
  publishes no events for it; the grid has no Saturday trade date.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-B-2026-12-31` | T2 | eventDates 2026-12-31 and 2027-01-01; CME prints `16:00 closed /TD 2026-12-31` with no evening re-open |
| 2027-01-18 | early close | `12:00 preopen /TD 2027-01-19; 17:00 open /TD 2027-01-19` — 12:00 CT | `CME-SVC-B-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19 |
| 2027-02-15 | early close | `12:00 preopen /TD 2027-02-16; 17:00 open /TD 2027-02-16` — 12:00 CT | `CME-SVC-B-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-B-2027-03-25` | T2 | eventDates 2027-03-25 and 2027-03-26; CME prints `16:00 closed /TD 2027-03-25` with no evening re-open |
| 2027-05-31 | early close | `12:00 preopen /TD 2027-06-01; 17:00 open /TD 2027-06-01` — 12:00 CT | `CME-SVC-B-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01 |
| 2027-06-18 | early close | `12:00 closed /TD 2027-06-21` — 12:00 CT | `CME-SVC-B-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21; the Thursday-evening leg opened 2027-06-17 17:00 CT |
| 2027-07-05 | early close | `12:00 preopen /TD 2027-07-06; 17:00 open /TD 2027-07-06` — 12:00 CT | `CME-SVC-B-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06 |
| 2027-09-06 | early close | `12:00 preopen /TD 2027-09-07; 17:00 open /TD 2027-09-07` — 12:00 CT | `CME-SVC-B-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07 |
| 2027-11-25 | early close | `12:00 preopen /TD 2027-11-26; 17:00 open /TD 2027-11-26` — 12:00 CT | `CME-SVC-B-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26 |
| 2027-11-26 | early close | `12:15 closed /TD 2027-11-26` — 12:15 CT | `CME-SVC-B-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-B-2027-12-22` | T2 | eventDates 2027-12-23 and 2027-12-24; CME prints `16:00 closed /TD 2027-12-23` with no evening re-open |

**Interpretive steps, 2027.**

- All eleven rows are the family's own published line, from the `THBP-B` live retrievals of
  2026-09-12.
- 2027-12-23 is CME's own holiday date for Christmas 2027 but is **not** a crate row. Its
  daytime close is the ordinary 16:00 CT; what is missing is the Thursday-evening leg, and
  `Closed(2027-12-24)` already removes it. 2027-12-31 is likewise normal: `16:00 closed
  /TD 2027-12-31` is the family's own Friday close, and the grid has no Friday-evening open
  to lose.

**Gaps, 2027.**

- **executable, not representable** — the Saturday session of 2027-06-19,
  `05:00 open /TD 2027-06-21; 17:00 closed /TD 2027-06-21`. Same shape and same closing
  condition as the two 2026 Saturdays.
- **residual risk** — the trade-date divergence described under 2025 applies to 2027-01-18,
  2027-02-15, 2027-05-31, 2027-06-18, 2027-07-05, 2027-09-06 and 2027-11-25.
- **coverage limit** — CME's published future for this family ends with the 2027-12-30 ..
  2028-01-02 window, whose only non-normal record is Saturday 2028-01-01 and which CME itself
  labels normal for this grid. Coverage therefore stops at 2027-12-31 and 2028-01-01 ships no
  row; a query for it gets the normal week and `holiday_on` answers `None` because the date
  is outside the window, which `holiday_coverage` is what distinguishes.

## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html> — CME Nikkei 225 Dollar contract specification, the current grid.
- <https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168> — CME `ContractSpecs` service for product id 168, corroborating the current grid.
- <https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf> — CME Japanese equity index futures fact card.
- <https://www.cmegroup.com/trading-hours.html> — CME trading-hours page, which states that hours are U.S. Central unless otherwise noted.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf> — CME SER-6465, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf> — CME SER-6554R, the 2013-03-03 revision's source, naming Nikkei 225 Dollar Futures explicitly.
- <https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html> — NKD contract specification — capture 2015-09-05, "5:00 p.m. previous day - 4:15 p.m.".
- <https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html> — NKD contract specification — capture 2015-11-27, "5:00 p.m. - 4:00 p.m. Chicago Time/CT".
- <https://web.archive.org/web/20100310022002id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2010-03-10, the materially different 2010 NKD grid.
- <https://web.archive.org/web/20100407094843id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2010-04-07, still the 2010 grid.
- <https://web.archive.org/web/20110112032949id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2011-01-12, the first sourced appearance of the served grid.
- <https://web.archive.org/web/20110811113223id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2011-08-11, the served grid restated.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **executable** — the 2010 grid is sourced but structurally different and its changeover day is undated, so dates before 2011-01-12 resolve to a sessionless profile rather than carrying either grid. CME's 2010-03-10 and 2010-04-07 captures read a daytime-anchored, DST-dependent grid — CDT 03:00–15:15 reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15 with no Sunday hours — and the 2011-01-12 capture already reads the served grid, with no capture and no located CME notice in between. Serving the continuous grid across 2010 would report the contract open all night when it was closed, which an earlier revision of the module did and which is corrected. Closing condition: a CME document that dates the changeover, or a capture inside the 2010-04-07..2011-01-12 window. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — encoding the 2010 grid itself would need seasonal CDT/CST rules and a boundary that is still undated, so it is left sourced-but-unmodelled.
- **corrected at the migration** — the ledger note carried into this file ended with a sentence, written for an earlier revision of the module, saying the pre-2012 grid "is now extended to the January-2010 floor". The module does not do that and must not: `nkd_profile_at` returns `NKD_CLOSED` below 2011-01-12. That sentence was removed on 2026-09-12 and an editorial marker left in its place; the surviving correction earlier in the same note is the authoritative statement.
- **scope** — Nikkei 225 Dollar outrights only. BTIC (`NKT`) is separately scheduled on its own CME-published hours and takes its own key if a consumer maps one; the 16:00–17:00 CT daily break is a maintenance period, not an order-entry phase.

## Module narrative (moved from src/calendar/schedules/futures/us/cme_nikkei.rs on 2026-09-12 UTC)

NKD outrights run one continuous Globex envelope per trade date: the session
opens 17:00 CT on the previous calendar evening and closes 16:00 CT on the
trade date, with the 60-minute 16:00-17:00 CT break separating consecutive
trade dates. CME's own wording: "Sunday - Friday 6:00 p.m. - 5:00 p.m. ET
(5:00 p.m. - 4:00 p.m. CT) with a 60-minute break each day beginning at
5:00 p.m. ET (4:00 p.m. CT)". There is no intraday halt as of the 2026-08-24 review. Friday is
absent from the opening-day mask because a Friday-evening open would belong
to a Saturday trade date, which does not exist; that omission is what
produces the Friday 16:00 CT weekly wrap.

HOW THE NKD GRID DIFFERS FROM THE STANDARD CME EQUITY-INDEX GRID (the reason
`MarketHoursKey::GlobexEquityIndex` explicitly excludes NKD): the U.S.-grid
contracts carry a pit-anchored 08:30-15:15 CT regular session with the
electronic envelope modelled around it as extended hours, and they carried a
15:15-15:30 CT halt until 2021-06-27. NKD is a pure-Globex international
equity-index contract: it has no pit/RTH split, so its entire envelope is the
regular session, and its 15:15-15:30 CT halt was removed eight years earlier,
on 2013-03-04, by a notice scoped to International Equity Index futures only.
The two grids differed from the (undatable, see below) 2010 change through
2012-11-18, and again through the 2013-03-04 halt removal. Today the envelopes
coincide, but the regular/extended split does not, so the key stays separate.

https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html
https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168
https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf
https://www.cmegroup.com/trading-hours.html

CME publishes no normal-week pre-open or order-entry start time for NKD on the
contract specs page, the ContractSpecs service, or the Japanese equity index
fact card, so no extended phase is asserted. The 16:00-17:00 CT daily break is
a maintenance/closed period, not an order-entry phase, and BTIC ("Sunday -
Friday 6:00 p.m. ET - 3:30 p.m. Tokyo time ... and Monday - Friday Noon to
5:00 p.m. ET") is separately scheduled, on its own published hours, so it is
not a phase of this outright order book. Both are deliberately omitted rather
than modelled as extended sessions.

That is a statement about scope, not about tradability: the Nikkei BTIC
instruments are their own order book with their own CME-published hours, and
the quoted sentence above is itself the primary source for their second daily
window. Should they be authored, they take their own key rather than becoming
a phase here — see the trade-type handoff in `docs/plans/`, whose survey found
exactly one trade-type root out of roughly 180 that genuinely rides its
underlying's clock.

NKD now carries a dated timeline from 2012-11-18. The 16:15 -> 16:00 CT
close, which was previously undatable and forced this family to be modelled
current-only, is dated by CME Globex Notice #20150817 of 17 August 2015:

  "Effective Monday, September 21, the daily CME Globex maintenance period
   will begin 15 minutes earlier Monday through Thursday from 16:00 until
   16:45 Central Time (CT). ... With this change, the closing times for the
   following markets will now occur 15 minutes earlier Monday through Friday
   at 16:00 CT. CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME
   Globex markets trading hours remain unchanged."

https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html

NKD sits in the named "CME Equity" Globex product group, and CME's own NKD
contract-specification captures bracket the change directly: 2015-09-05 reads
"MON - FRI: 5:00 p.m. previous day - 4:15 p.m.", and 2015-11-27 reads
"5:00 p.m. - 4:00 p.m. Chicago Time/CT".
https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html
https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html

Revisions are keyed by the local session-opening day, matching `cme_group`:
the first close at 16:00 CT is trade date Monday 2015-09-21, whose session
opened Sunday 2015-09-20.

THE 2011 GRID, AND WHY IT IS NOT CARRIED TO THE FLOOR. CME's trading-hours
pages publish this grid — Electronic Trading (Sunday) "17:00-15:15" and
(Weekday) "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row
beside it — from the 2011-01-12 capture onward. SER-6465 corroborates the
outgoing 15:15 CT close by describing its own change as an extension of it.

It is NOT carried back to the January-2010 floor, because the 2010 grid was
materially different and is sourced. The 2010-03-10 and 2010-04-07 captures of
the same page read, for "Nikkei 225 (Dollar) Futures":

  Electronic (weekday)  CDT: 03:00-15:15 reopens 15:30-16:30; closes
                        16:30-17:00; reopens 17:00-18:00
                        CST: 02:00-15:15; reopens 15:30-16:30; closes 16:30
  Sunday                CDT: Opens 17:00-18:00    CST: No Sunday Hours

That is a daytime-anchored, DST-dependent grid whose evening segment ran only
17:00-18:00 and which had no Sunday session at all in CST. Serving the
17:00-15:15 continuous grid across it would report the contract open through
the whole overnight window when it was closed — a false open, in executable
hours. An earlier revision of this module did exactly that, on the reasoning
that no source named a cutover; a source does, and the carry-back convention
requires that none exists.

The transition is undated: 2010-04-07 still shows the old grid and 2011-01-12
already shows the new one, with no capture and no located CME notice in
between. Dates before the first sourced appearance of the served grid are
therefore modelled sessionless, the same knowledge boundary a launch day
provides. Encoding the 2010 grid itself would need seasonal CDT/CST rules and
a boundary that is still undated, so it is left as sourced-but-unmodelled and
recorded here.
Official origin http://www.cmegroup.com/trading_hours/ delivered via:
https://web.archive.org/web/20100310022002id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20100407094843id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20110112032949id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20110811113223id_/http://www.cmegroup.com/trading_hours/

2012-11-18: "CME Group announces that the new daily trading hour schedule for
  CBOT and CME Equity Index futures and Options on Equity Index futures will
  begin on Sunday, November 18, 2012 for trade date Monday, November 19, 2012."
  https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf
2013-03-03: "The modified Globex trading hours will be effective Monday,
  March 4, 2013. The 15 minute trading halt between 3:15 p.m. and 3:30 p.m.,
  Central Time, Monday through Friday, will be eliminated for CME
  International Equity [Index futures] ..." - NKD is named explicitly as
  "Nikkei 225 Dollar Futures". Keyed to the Sunday session-opening day.
  https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf
2015-09-20: CME Globex Notice #20150817, quoted above; trade date Monday
  2015-09-21, session-opening day Sunday 2015-09-20.
