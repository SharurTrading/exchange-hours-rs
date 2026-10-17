<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_metals` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's spot-metals book, the XAU and XAG instrument families and their
crosses: Sunday 18:05 through Friday 16:59 New York time, with the printed
16:59-18:05 daily gap and no other maintenance. The printed pair implies the
daily gap exactly as the FX row's footnote states its own six-minute one.
Holiday boundaries below are the operator's literal dated statements,
including 2025 closes at 13:00 and 14:30; no uniform futures inset is assumed.

## Revision rows

- 2024-02-11 — T2 — `MARKET-DATA` — first current New York opening day,
encoded by the shared FX/metals selector constant rather than a `revisions!`
block. Historical completion remains tracked in #332 under the dated
current OANDA scope exception.

## The normal week

The hours-of-operation page prints every metals row identically:

> `XAU/USD | USD | New York | 100 | Sun-Fri: 18:05 - 16:59`

with the note:

> `**These instruments also have a break and are closed from 06:59 to 07:05 Auckland time`

The row prints no `***` daylight-saving marker, so the endpoints track New
York local time and `America/New_York` carries the transitions. The NZD-pair
Auckland break is stated per instrument and stays caller catalog data. The
rows cover XAU and XAG crosses against ten quote currencies; all share one
grid.

## Holidays

**Coverage:** 2024-02-21..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The page's `US energies and metals` group covers this book as well as the
commodity-metals book. On 2026-05-25 the operator explicitly distinguishes
the gold/silver reopen at 18:05 from the group's 18:01. That exception applies
where it is stated; other dated group reopens remain the literal printed
18:00 or 18:01, and standalone `Metals` reopens include this book. A holiday
statement overrides the normal week: the 2025-01-01 18:04 reopen
and the Holy Thursday 17:00 close are encoded numerically, even though the
page calls them "late" and "early" against today's normal 18:05/16:59 grid.
No rounding or retrospective extension of the 2026-05-25 parenthetical is
used. Evening reopens belong to the following trade date.

### 2024

`OANDA-UK-CARTER-2025-WAYBACK` gives these literal Eastern-time
arrangements. Evening reopens attach to the following trade date; normal
closes remain 16:59 except Thanksgiving Friday's explicit 14:45.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-03-29 | closed | — | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Good Friday: US energies and metals closed |
| 2024-05-27 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Memorial Day group close |
| 2024-05-28 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Memorial Day group reopen |
| 2024-06-19 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Juneteenth group close |
| 2024-06-20 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Juneteenth group reopen |
| 2024-07-04 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Independence Day group close |
| 2024-07-05 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Independence Day group reopen |
| 2024-09-02 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Labor Day group close |
| 2024-09-03 | replacement blocks | prior day 18:00..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Labor Day group reopen |
| 2024-11-28 | early close | 14:30 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Thanksgiving Thursday group close |
| 2024-11-29 | replacement blocks | prior day 18:00..14:45 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Thanksgiving Thursday reopen and Friday close |
| 2024-12-24 | early close | 13:44 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas Eve: Metals close early 13:44 |
| 2024-12-25 | closed | — | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas Day until 18:04 evening reopen |
| 2024-12-26 | replacement blocks | prior day 18:04..16:59 | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Christmas Day: Metals open late 18:04 |

These fourteen T1 rows extended the holiday audit to 2024-02-21 and
increased this table from 32 to 46 rows. The archive also prints earlier
January and February 2024 arrangements. Dating the normal-week changeover
does not work up those holiday arrangements: the existing holiday window
remains unchanged, with earlier holiday completion tracked in #332. Within
that window the operator's changes-only page supplies audited normal days
where it states no change to this metals book.
Artifact: `normal-weeks/oanda-history-audit-2026-10-11/oanda-uk-holidays-20250326184653.html`.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | — | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day section: the US holiday groups reopen at the evening open, so the trade date's session does not trade |
| 2025-01-02 | replacement blocks | `18:04; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-01-20 | early close | `14:30` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day group early close, previously omitted. |
| 2025-01-21 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-02-17 | early close | `close early at 2.30 pm and reopen at 6 pm` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day, the group's instant (ET) |
| 2025-02-18 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-04-17 | replacement blocks | `18:05; 17:00` | `OANDA-UK-HOLIDAYS` | T1 | Holy Thursday explicitly closes 17:00 ET, one minute later than the normal close; clipping cannot express this. |
| 2025-04-18 | closed | `are closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday, `US energies and metals are closed` |
| 2025-05-26 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day |
| 2025-05-27 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-06-19 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2025-06-20 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-07-04 | early close | `close early at 1 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2025-09-01 | early close | `close early at 2:30 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |
| 2025-09-02 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-11-27 | early close | `Close early at 2:30 PM, reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Thanksgiving Day |
| 2025-11-28 | replacement blocks | `18:00; 14:45` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2025-12-24 | early close | `Metals: close at 13:44` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Eve, the page's own split row |
| 2025-12-25 | closed | `Metals: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day: the stated metals evening reopen is keyed to the following trade date, so this trade date's session does not trade |

| 2025-12-26 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Metals: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: same derivation as Christmas Day |
| 2026-01-02 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2026-01-19 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day, the group's instant |
| 2026-01-20 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2026-02-16 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day |
| 2026-02-17 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |
| 2026-04-03 | closed | `US Energies and metals: Closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2026-05-25 | early close | `Close at 14:29; reopen at 18:01 (Gold, Silver and their crosses reopen at 18:05)` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day: the page's own gold/silver callout |
| 2026-06-19 | early close | `Close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2026-07-03 | early close | `Close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2026-09-07 | early close | `Early close at 14:29 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |

| 2026-09-08 | replacement blocks | `18:01; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Previous holiday's stated group reopen; this trade date's printed or normal close retained. |

## Dated selectors

The module serves the witnessed normal-week eras; `profile_at` keys each
opening day to its era. `sourcing.rs` declares the normal week witnessed from
**2021-12-08**, the operator's archived page print that first lists the exact
`XAU/USD` and `XAG/USD` pair rows.

The spans, and what each rests on:

- **Opening days 2021-12-08 through 2024-02-08 — the earlier USD-pair grid
  `18:03 - 16:58`.** The 2021-12-08 capture prints
  `Gold (XAU/USD) ... Sun-Fri: 18:03 - 16:58*` and the matching silver row
  (`OANDA-UK-HOURS-2021-DEC-WAYBACK`); the 2023-06-05 capture still prints
  `XAU/USD ... 18:03 - 16:58` (`OANDA-UK-HOURS-2023-JUN-WAYBACK`), and the
  audit's intervening replays print the identical pair grid. The era's cross
  rows print an `18:10` open (XAU/NZD with its Auckland break): those
  per-instrument minutes stay caller instrument data, disclosed here as the
  era's residual.
  The page prints witness the grid through 2023-06-05, and the operator's own
  machine channel witnesses it positively per day to the 2024-02-08
  opening day: every classified opening day carries an 18:03 New York
  minute tick, which the current grid's 16:59–18:05 break forbids (the
  section below). The span serves the earlier USD-pair state, open
  `18:03`, close `16:58` (one thin Sunday, 2023-11-12, is bracketed by
  witnessed neighbours).
- **Opening days from 2024-02-11 — the current grid `18:05 - 16:59`.** The
  changeover is positively dated: the session opened Sunday 2024-02-11
  closes on Monday at `16:59:00`, and its close minute contains S5
  buckets inside the earlier grid's break (the section below). The intervening
  Friday and Saturday open no session, so no unwitnessed opening day
  remains between the eras. The 2024-02-21 capture prints the grid
  (`OANDA-UK-HOURS-2024-WAYBACK`), and every later replay and the
  2026-10-10 capture corroborate; all rows share one grid again.
- **Below the 2021-12-08 witness**: carried. The 2021-03-03..2021-08-18 page
  generation prints coarser `All gold instruments (XAU)` /
  `All silver instruments (XAG)` rows at `18:00 - 17:00` — a different table
  scope, not the book's exact pair rows — so the book's first complete
  witness stays 2021-12-08 (#332).

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page, captured 2026-10-10 22:36 UTC and rendered by the maintainer to
  PDF the same UTC date.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOLIDAYS` | 2025-01-01 .. 2026-10-30 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours> | captured 2026-10-10 22:44 | T1 | `c7137ce3da4cc19d0431955feedc10fb6b9731214ad3cfc7966160e72f8559da` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |
| `OANDA-UK-CARTER-2025-WAYBACK` | 2025-01-09 | <https://web.archive.org/web/20250326184653id_/https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | capture 2025-03-26; retrieved 2026-10-11 UTC | T1 | `467313685d4d64a4b95e5112a58d0fa04583f0abe988b1478d41fddf8b04cf1c` |
| `OANDA-UK-HOURS-2021-DEC-WAYBACK` | 2021-12-08 observation; not a cutover | <https://web.archive.org/web/20211208123011id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2021-12-08; retrieved 2026-10-11 UTC | T1 | `63e6d54029b72e0ef1ec32f92b8c3412bd24edf05df9fda9fa2b4dce98cb78fb` |
| `OANDA-UK-HOURS-2023-JUN-WAYBACK` | 2023-06-05 observation; not a cutover | <https://web.archive.org/web/20230605093523id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2023-06-05; retrieved 2026-10-11 UTC | T1 | `98ddc07cf38727b2a861fd1f48c503b1b597054cbec9d1dde4b3dde1801ffb56` |

Store paths: see [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- **RESOLVED 2026-10-17 (UTC): the changeover is positively dated to the
  2024-02-11 opening day.** The disputed 2023-06-05..2024-02-20 span
  served as the sourced intersection; the operator's own machine channel
  witnesses the earlier pair grid positively on classified opening days through
  2024-02-08, and the Sunday 2024-02-11-opened session's Monday close
  minute quotes inside the earlier grid's break, dating the current era
  from 2024-02-11 (section below, `MARKET-DATA`). The
  2021-03-03..2021-08-18 coarse gold/silver rows and the era's cross opens
  stay #332 residuals.
- Dates where the page prints no US-group row at all are audited normal for
  this book: the page's US vocabulary is rows-only, so its silence affirms no
  change.

## Machine-channel witnessing and the dated boundary — 2026-10-16 UTC, corrected 2026-10-17

The operator Practice candle service was read through `oanda-rs` for
`XAU_USD`. The timestamp convention, the limits of midpoint observations
and the positive-presence method are documented in
[`oanda_fx.md`](oanda_fx.md#machine-channel-witnessing-and-the-dated-boundary--2026-10-16-utc-corrected-2026-10-17-in-review).
An observed five-second lead is not a universal upper bound for this or
another book. No silent or lead-only bucket dates a changeover.

The candidates are earlier `18:03 - 16:58` and current `18:05 - 16:59`.
The per-day M1 transcript has 175 positive `18:03`-minute observations
through opening day 2024-02-08. Sunday 2023-11-12 is thin and bracketed by
witnessed neighbours; the holiday Sundays 2023-12-24 and 2023-12-31 are
also quiet. None supplies a positive observation. The independent February
8 S5 read includes the `18:03:00` reopen, and the February 9 closing leg's
final bucket starts at `16:58:00`.

The Monday February 12 closing leg contains buckets well inside the earlier
candidate's break. The retained close-minute starts are:

| Closing date, New York | S5 starts at 16:58:ss | 16:59:00 bucket |
|---|---|---|
| 2024-02-12 | :00, :05, :10, :15, :20, :25, :30, :35, :45, :50, :55 | present |
| 2024-02-13 | :10, :15, :20, :30, :40 | present |
| 2024-02-14 | :25, :50, :55 | present |
| 2024-02-15 | :15, :50, :55 | absent |
| 2024-02-19 | none | absent |

Absence in the last two rows does not establish a different close. The
February 12 positive observations support the current candidate for the
session opened **Sunday 2024-02-11**. Friday and Saturday open no session
between the last earlier Thursday opening and that Sunday, so the
intersection hosts no opening days and is retired. The first pass's
lead-only current classification and the later February 12 opening-day key
were corrected in review.

`MARKET-DATA` — T2, initial reads 2026-10-16 UTC, independently retrieved
and corrected 2026-10-17 UTC. Retention root:
`~/Developer/oanda-feed-captures-2026-10-16/`, `MANIFEST.md` and
`SHA256SUMS.txt`. The per-day input is `witness.jsonl`; original S5 inputs
are `raw/XAU_USD_S5_*.json`. Full independent responses are
`raw/validation-2026-10-17/era-XAU_USD-2024-02-{08,09,12}.json`.
These observations date the USD-pair candidate grid; they do not remove
the historical cross-pair residual or establish live-host equivalence.
Those checks remain in #332.

## Publication audit — 2026-10-11 UTC

The source's minute values are literal. The 2025 18:00 ET US reopens and
20:00 ET Brent reopens are not rounded to 18:01 or 20:01. Evening opens
attach to the following trade date. Replacement blocks retain the entire
trading day, including any next-day early close and Brent's late segment.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).
