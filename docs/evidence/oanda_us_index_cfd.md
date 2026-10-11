<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_us_index_cfd` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's US-anchored index book: US SPX 500, US Nas 100, US Russ
2000, US Wall St 30, Japan 225 (USD and JPY) on one Sunday-to-Friday clock, Chicago 17:01-15:59 — the same
instants as New York 18:01-16:59 — with the printed 16:59-18:01 daily gap
and no other maintenance.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and a dormant identity owes its current
schedule rather than a worked-up history (LAW-BOUNDED-WORK).

## The normal week

The page prints the identical pair across the group:

> `US SPX 500 | USD | Chicago | 1 | Sun - Fri 17:01 - 15:59`
> `US 10Y T-Note | USD | Chicago | 1 | Sun - Fri 17:01 - 15:59`

> `Japan 225 | USD | Chicago | 1 | Sun - Fri 17:01 - 15:59`

Converted to New York, `17:01 - 15:59` Chicago is `18:01 - 16:59` — the same
instants the New-York-quoted energy rows print — so the whole US-anchored
family sits on one absolute clock and the pair implies the daily 16:59-18:01
gap. The instants sit one minute inside the futures grid's 17:00-16:00 CT,
which is the hedge-desk relationship: the book opens and closes just inside
the hedge market's own day. Japan 225 rides this book rather than a CME
Nikkei mapping because its holiday rows are the US indices' own (below), and
the 2022 capture shows it printed the CME maintenance structure then — a
different, earlier regime the horizon excludes.

## Holidays

**Coverage:** 2025-01-01..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The group is the page's `US indices and bonds`, including its split `US
Indices` / `US Bonds` rows and the `US index futures` / `US bonds` wording of
the 2025 archive. The standalone `JP225:` rows state the US indices' own
arrangement on every date they print (Christmas Eve `close at 13:14`,
Christmas Day `open at 18:01`, New Year's Eve `close at 16:59`), so the
family ships together. Indices and bonds now have separate keys (2026-10-11 UTC). Good Friday
2026 closes indices at 09:14 ET and bonds at 11:14 ET. Carter's Day
2025-01-09 withholds only the index key; the bond close is explicit 13:15 ET.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `US indices open late 18:04`, `US bonds open late 18:04` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: the holiday trade date is closed; the following trade date opens at the explicitly stated 18:04 |
| 2025-01-02 | late open | `18:04` | `OANDA-UK-HOLIDAYS` | T1 | 2025-01-01 evening reopen is keyed to 2025-01-02, whose opening day is the holiday. |
| 2025-01-09 | unsourced | `close early at 9:30` (no meridiem) | `OANDA-UK-HOLIDAYS` | T1 | Carter's Day of Mourning: the instant is ambiguous (`9:30` AM or PM unmatched by any other row; bonds print 13:15); withheld until a desk answer or a clearer capture |
| 2025-01-20 | early close | `13:00` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day group early close, previously omitted. |
| 2025-01-21 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-02-17 | early close | `close early at 1 pm and reopen at 6 pm` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day |
| 2025-02-18 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-04-17 | replacement blocks | `18:01; 17:00` | `OANDA-UK-HOLIDAYS` | T1 | Holy Thursday explicitly closes 17:00 ET, one minute later than the normal close; clipping cannot express this. |
| 2025-04-18 | closed | `are closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday: `US index futures are closed`, `US bonds are closed` |
| 2025-05-26 | early close | `close early at 1:00 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day |
| 2025-05-27 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-06-19 | early close | `close early at 1:00 PM and reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2025-06-20 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-07-03 | early close | `close early at 1:15 pm and reopen at 6 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day eve |
| 2025-07-04 | replacement blocks | `18:00; 13:00` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-09-01 | early close | `Close early at 1:00 PM, reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |
| 2025-09-02 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-11-27 | early close | `Close early at 1:00 PM, reopen at 6:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Thanksgiving Day |
| 2025-11-28 | replacement blocks | `18:00; 13:15` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-12-24 | early close | `JP225: close at 13:14`, `US indices: close at 13:14`, `US bonds: close at 13:14` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Eve |
| 2025-12-25 | closed | `JP225: open at 18:01`, `US indices: open at 18:01`, `US bonds: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day: the reopens are the book's own evening open |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `JP225: open at 18:01`, `US indices: open at 18:01`, `US bonds: open at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: same derivation as Christmas Day |
| 2026-01-19 | early close | `Early close at 12:59 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | MLK Day |
| 2026-02-16 | early close | `Early close at 12:59 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Presidents Day |
| 2026-04-03 | early close | `US Indices: Early close at 9:14`, `US Bonds: Early close at 11:14` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday: indices close at 09:14; bonds use their separate key at 11:14 |
| 2026-05-25 | early close | `Early close at 12:59; reopen at 18:0` (incomplete source time; the following trade date is withheld) | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day |
| 2026-05-26 | unsourced | `reopen at 18:0` | `OANDA-UK-HOLIDAYS` | T1 | The previous evening's missing minute digit does not establish exact session bounds. The following trade date refuses until the UK operator clarifies it (#332). |
| 2026-06-19 | early close | `Early close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2026-07-03 | early close | `Early close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2026-09-07 | early close | `Early close at 12:59 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |

## Dated selectors

None. One sourced grid; `sourcing.rs` declares the normal week witnessed from
**2024-02-21**, the operator's archived page print of the current `17:01 -
15:59` grid. The 2022-05-18 capture prints the futures-exact `17:00 - 16:00`
pair (no one-minute inset), so the changeover is undated and the span below
the 2024 witness is carried, not served.

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

Store paths: see [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- **The pre-2024-02-21 normal week is carried, not sourced.** The 2022
  capture prints the futures-exact `17:00 - 16:00` grid and Japan 225 on the
  CME maintenance structure (`18:00 - 16:15; 16:30 - 17:00` New York); the
  changeovers are undated and the span refuses on the date-aware surface.
  Closing condition: dated operator artifacts, or hunts closed negative
  (then the 2026-10-04 intersection rule applies).
- The 2025-01-09 withholding closes with an OANDA desk answer or a clearer
  capture (AM/PM ambiguity is a page-edit question).
- Bonds route to `oanda_us_bond_cfd`; no intersection is used as their exact calendar.
- US Share CFD and ETF arrangements (`Closed` on US holidays) are
  caller-side listings of equities and stay out of this book.

## Publication audit — 2026-10-11 UTC

The source's minute values are literal. The 2025 18:00 ET US reopens and
20:00 ET Brent reopens are not rounded to 18:01 or 20:01. Evening opens
attach to the following trade date. Replacement blocks retain the entire
trading day, including any next-day early close and Brent's late segment.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).

The 2026-05-25 section's incomplete `18:0` reopen does not justify an
18:01 opening or any exact intersection boundary. The 2026-05-25 close
remains sourced at 12:59; 2026-05-26 is `Unsourced` because its prior-evening
opening is unknown. Other OANDA legal entities' 18:01 statements may
corroborate a possible correction but do not establish this UK scope (#332).
