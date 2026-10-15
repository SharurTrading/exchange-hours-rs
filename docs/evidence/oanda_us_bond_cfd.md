<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_us_bond_cfd` — evidence

Established 2026-10-11 UTC. This key covers only the four US Treasury CFD listings;
the four US indices use `oanda_us_index_cfd`. Japan 225's exact holiday scope
remains unresolved (#332). A shared normal-week clock does not merge distinct
or unproved holiday arrangements.

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-11 UTC)

OANDA's four US Treasury CFD listings on one Sunday-to-Friday clock, Chicago 17:01-15:59 — the same
instants as New York 18:01-16:59 — with the printed 16:59-18:01 daily gap
and no other maintenance.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and no unconditional changeover day is asserted. Historical completion
remains tracked in #332 under the dated current OANDA scope exception.

## The normal week

The page prints the identical pair across the group:

> `US SPX 500 | USD | Chicago | 1 | Sun - Fri 17:01 - 15:59`
> `US 10Y T-Note | USD | Chicago | 1 | Sun - Fri 17:01 - 15:59`
>
> `Japan 225 | USD | Chicago | 1 | Sun - Fri 17:01 - 15:59`

Converted to New York, `17:01 - 15:59` Chicago is `18:01 - 16:59` — the same
instants the New-York-quoted energy rows print — so the whole US-anchored
family sits on one absolute clock and the pair implies the daily 16:59-18:01
gap. The instants sit one minute inside the futures grid's 17:00-16:00 CT,
which is the hedge-desk relationship: the book opens and closes just inside
the hedge market's own day. The shared clock establishes the Treasury
listings' normal week; it does not establish their holiday agreement with
indices or Japan 225.

## Holidays

**Coverage:** 2024-01-01..2024-12-31, 2025-01-01..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The group is the page's `US indices and bonds`, including its split `US
Indices` / `US Bonds` rows and the `US index futures` / `US bonds` wording of
the 2025 archive. Explicit bond rows govern this key; shared rows apply
where the operator groups indices and bonds. The keys were separated on
2026-10-11 UTC. Good Friday
2026 closes indices at 09:14 ET and bonds at 11:14 ET. Carter's Day
2025-01-09 closes indices at 09:30 ET from the UK archive's dated 24-hour
context (`OANDA-UK-CARTER-2025-WAYBACK`), while bonds explicitly close at
13:15 ET.

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `closed` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-01-02 | unsourced | `18:00` ET reopen known; final close unstated | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The previous dated UK holiday section states the reopen. The normal-week final close for this pre-2024-02-21 trade date is unresolved; no complete replacement is claimed (#332). |
| 2024-01-15 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-01-16 | unsourced | `18:00` ET reopen known; final close unstated | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The previous dated UK holiday section states the reopen. The normal-week final close for this pre-2024-02-21 trade date is unresolved; no complete replacement is claimed (#332). |
| 2024-02-19 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-02-20 | unsourced | `18:00` ET reopen known; final close unstated | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The previous dated UK holiday section states the reopen. The normal-week final close for this pre-2024-02-21 trade date is unresolved; no complete replacement is claimed (#332). |
| 2024-03-29 | closed | `closed` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-05-27 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-05-28 | replacement blocks | `18:00; 16:59` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Literal 18:00 ET holiday reopen from the previous dated UK section, combined with the sourced normal-week 16:59 ET close from OANDA-UK-HOURS-2024-WAYBACK; exact book scope retained. |
| 2024-06-19 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-06-20 | replacement blocks | `18:00; 16:59` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Literal 18:00 ET holiday reopen from the previous dated UK section, combined with the sourced normal-week 16:59 ET close from OANDA-UK-HOURS-2024-WAYBACK; exact book scope retained. |
| 2024-07-04 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-07-05 | replacement blocks | `18:00; 16:59` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Literal 18:00 ET holiday reopen from the previous dated UK section, combined with the sourced normal-week 16:59 ET close from OANDA-UK-HOURS-2024-WAYBACK; exact book scope retained. |
| 2024-09-02 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-09-03 | replacement blocks | `18:00; 16:59` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Literal 18:00 ET holiday reopen from the previous dated UK section, combined with the sourced normal-week 16:59 ET close from OANDA-UK-HOURS-2024-WAYBACK; exact book scope retained. |
| 2024-11-28 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-11-29 | replacement blocks | `18:00; 13:15` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Previous dated UK holiday section supplies the literal 18:00 ET reopen; this trade date supplies its printed holiday close. Exact book scope retained. |
| 2024-12-24 | unsourced | `—` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-12-25 | closed | `closed` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-12-26 | late open | `18:04` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `US indices open late 18:04`, `US bonds open late 18:04` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: the holiday trade date is closed; the following trade date opens at the explicitly stated 18:04 |
| 2025-01-02 | late open | `18:04` | `OANDA-UK-HOLIDAYS` | T1 | 2025-01-01 evening reopen is keyed to 2025-01-02, whose opening day is the holiday. |
| 2025-01-09 | early close | `US bonds close early at 13:15` | `OANDA-UK-HOLIDAYS` | T1 | Carter's Day of Mourning: the bonds' close is stated unambiguously at 13:15 ET |
| 2025-01-10 | replacement blocks | `18:00; 16:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
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
| 2025-07-04 | early close | `US bonds close early at 1 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day: the bond close is explicit; the previous day names only indices, so bonds retain their normal 18:01 opening. |
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
| 2026-04-03 | early close | `US Indices: Early close at 9:14`, `US Bonds: Early close at 11:14` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday: this bond key closes at 11:14 ET |
| 2026-05-25 | early close | `Early close at 12:59; reopen at 18:0` (incomplete source time; the following trade date is withheld) | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day |
| 2026-05-26 | unsourced | `reopen at 18:0` | `OANDA-UK-HOLIDAYS` | T1 | The previous evening's missing minute digit does not establish exact session bounds. The following trade date refuses until the UK operator clarifies it (#332). |
| 2026-06-19 | early close | `Early close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth |
| 2026-07-03 | early close | `Early close at 12:59` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day |
| 2026-09-07 | early close | `Early close at 12:59 and reopening at 18:01` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day |

## Dated selectors

The module serves the witnessed normal-week eras; `profile_at` keys each
opening day to its era. `sourcing.rs` declares the normal week witnessed from
**2021-03-03**, the operator's archived page print that first lists the four
US Treasury rows.

The spans, and what each rests on:

- **Opening days 2021-03-03 through 2023-06-04 — the whole-hour clock
  `17:00 - 16:00` Chicago** (the same instants as New York `18:00 - 17:00`,
  the clock the book serves in New York terms). The 2021-03-03 capture
  prints it (`OANDA-UK-HOURS-2021-WAYBACK`); the 2023-06-05 capture still
  prints it (`OANDA-UK-HOURS-2023-JUN-WAYBACK`), and every intervening
  replay prints the identical grid — this book's legacy era has no interior
  change.
- **Opening days 2023-06-05 through 2024-02-19 — the sourced intersection.**
  The changeover to `17:01 - 15:59` is undated (last earlier-grid print
  2023-06-05, first current-grid print 2024-02-21, no Wayback capture
  between). The current grid nests inside the legacy print, so the span
  serves `18:01 - 16:59` New York per the charter's 2026-10-04 rule; the
  residual is disclosed below.
- **Opening days from 2024-02-20 — the current grid `17:01 - 15:59`
  Chicago** (the 2024-02-21 capture, `OANDA-UK-HOURS-2024-WAYBACK`; later
  replays corroborate).
- **Below the 2021-03-03 witness**: carried (#332).

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
| `OANDA-UK-HOURS-2021-WAYBACK` | 2021-03-03 | <https://web.archive.org/web/20210303210448id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20210303210448`, retrieved 2026-10-10 UTC; prints no Forex table | T1 | `ec5987239f2c79dd29fcfc40abe5a0d58c6e4ed446fa7dea653f1791b186cbed` |
| `OANDA-UK-HOURS-2023-JUN-WAYBACK` | 2023-06-05 observation; not a cutover | <https://web.archive.org/web/20230605093523id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2023-06-05; retrieved 2026-10-11 UTC | T1 | `98ddc07cf38727b2a861fd1f48c503b1b597054cbec9d1dde4b3dde1801ffb56` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |

Store paths: see [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- **The undated changeover's disputed remainder (2023-06-05..2024-02-19
  opening days) is a disclosed residual.** What is served: the sourced
  intersection, which here is the current `18:01 - 16:59` New York grid (it
  nests inside the whole-hour legacy print). The disputed remainder: the
  legacy state's extra minutes — open 18:00 and close 17:00 New York. Named
  closer: a dated OANDA UK operator artifact or desk answer; the sweep is
  recorded in [`oanda_fx.md`](oanda_fx.md).
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

The 2025-07-03 section names only US index futures. Its 13:15 close
and 18:00 reopen do not apply to this bond key. July 3 is audited normal
under the page's rows-only bond vocabulary; July 4 retains the stated
bond close at 13:00 and the normal prior-evening opening at 18:01.

## Archived 2024 holiday window — 2026-10-11 UTC

The UK operator page captured on 2025-03-26 retains its dated 2024 holiday
changes from New Year's Day through New Year's Eve. Its page-wide
Eastern-Time instruction governs every printed instant. The audited window
is 2024-01-01..2024-12-31; December coverage rests on its expressly dated
2024 Christmas and New Year sections. The existing normal-week coverage gate still applies
before its 2024-02-21 witness. Earlier holiday history remains historical completion work tracked in #332.

The archive states `US indices open late 6 pm` and `US bonds open late 6 pm`
for 2024-01-01. The holiday trade date is closed and the literal 18:00 ET
reopen belongs to 2024-01-02, not a rounded 18:01 opening. The final closes
of 2024-01-02, 2024-01-16 and 2024-02-20 are not stated in the holiday
sections; the 2024-02-21 normal-hours witness cannot source those earlier
closes. Each date therefore remains `Unsourced`, preserving its known
18:00 ET reopen in this evidence record. Closing condition: a UK-scoped
operator artifact stating the normal final close in force on those trade
dates (#332). The archive prints 13:00 ET
closes and 18:00 ET reopens for MLK Day, Presidents Day, Memorial Day,
Juneteenth, Independence Day and Labor Day; Good Friday is closed.

| Trade date | Arrangement | Following trade date | Arrangement |
|---|---|---|---|
| 2024-01-01 | Closed | 2024-01-02 | Unsourced: 18:00 ET reopen known; pre-witness final close unknown |
| 2024-01-15 | Close 13:00 ET | 2024-01-16 | Unsourced: 18:00 ET reopen known; pre-witness final close unknown |
| 2024-02-19 | Close 13:00 ET | 2024-02-20 | Unsourced: 18:00 ET reopen known; pre-witness final close unknown |
| 2024-03-29 | Closed | — | — |
| 2024-05-27 | Close 13:00 ET | 2024-05-28 | 18:00 ET previous evening to normal 16:59 ET close |
| 2024-06-19 | Close 13:00 ET | 2024-06-20 | 18:00 ET previous evening to normal 16:59 ET close |
| 2024-07-04 | Close 13:00 ET | 2024-07-05 | 18:00 ET previous evening to normal 16:59 ET close |
| 2024-09-02 | Close 13:00 ET | 2024-09-03 | 18:00 ET previous evening to normal 16:59 ET close |
| 2024-11-28 | Close 13:00 ET | 2024-11-29 | 18:00 ET previous evening to printed 13:15 ET close |

The Thanksgiving section's outer heading names Friday 29 November; its
inner headings separately state Thursday 28 November and Friday 29 November.
The Thursday reopen therefore governs Friday's complete replacement block.
The 2024-07-03 section names only US index futures: indices close at 13:15 ET
and reopen at 18:00 ET, while bonds retain the normal prior-evening 18:01 ET
opening for July 4. The July 4 index replacement preserves its 13:00 ET close.
These product distinctions are retained instead of treating the books as one
holiday family.

Christmas Eve 2024 prints indices closing at 13:14 ET, but names no US-bond
arrangement. The index row is exact; the bond row remains `Unsourced`, rather
than borrowing indices' close or certifying a normal bond session on this
holiday (#332). The 2024-12-25 section explicitly prints both books reopening
at 18:04 ET; each holiday trade date is closed and 2024-12-26 opens at 18:04
on the preceding evening. The 2024-12-31 close at 16:59 ET is already the
normal close and needs no exception row.

All new rows cite `OANDA-UK-CARTER-2025-WAYBACK`, T1. The existing document
id is retained because the archive is byte-identical to the Carter-day
witness; that artifact also records these 2024 sections. The archive also prints
other products' exceptions, which do not key these two US scopes.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-CARTER-2025-WAYBACK` | 2025-01-09 | <https://web.archive.org/web/20250326184653id_/https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | Wayback `id_` replay of capture `20250326184653`, retrieved 2026-10-11 UTC | T1 | `467313685d4d64a4b95e5112a58d0fa04583f0abe988b1478d41fddf8b04cf1c` |

## Incomplete 2026 reopen recheck — 2026-10-11 UTC

A fresh UK-page retrieval at 22:54 UTC still prints the literal incomplete
Memorial Day reopen `18:0`; both the rendered HTML and embedded structured
text agree. Canada's corresponding page prints 18:01, but its different
legal-entity scope does not establish the UK minute. This recheck changes
no runtime row and keeps the 2026-05-26 refusal tracked in #332.
