<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_us_index_cfd` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's US-anchored index book: US SPX 500, US Nas 100, US Russ
2000 and US Wall St 30 on one Sunday-to-Friday clock, Chicago
17:01-15:59 — the same
instants as New York 18:01-16:59 — with the printed 16:59-18:01 daily gap
and no other maintenance.

## Revision rows

None. No `revisions!` block exists and no unconditional changeover day is
asserted. The dated selectors below preserve the earlier grid, the sourced
intersection and the current page-print observation bound. Historical
completion remains tracked in #332 under the dated current OANDA scope exception.

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
the hedge market's own day. Japan 225 (USD and JPY) shares this normal clock in the current hours table,
but that equality does not establish its inclusion in every generic
US-index holiday statement. The 2022 capture prints an earlier Japan 225
maintenance structure; its changeover is undated. These observations are
context, not exact holiday routing permission (#332).

## Holidays

**Coverage:** 2024-01-01..2024-12-31, 2025-01-01..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern). Tier: T1 throughout.

The key's exact scope is the four named US indices, not Treasury CFDs or
Japan 225. The holiday page's `US indices and bonds`, split `US Indices` /
`US Bonds`, and archived `US index futures` / `US bonds` statements apply
through those separate index and bond keys. Good Friday 2026 closes indices
at 09:14 ET and bonds at 11:14 ET. Carter's Day 2025-01-09 closes the named
US indices at 09:30 ET, derived from the dated section's 24-hour context and
ordered 18:00 reopening; the bond close is explicit 13:15 ET.

Standalone `JP225:` rows match the US-index arrangements on the specific
Christmas/New Year dates where they print. That does not prove inclusion in
other generic US-index rows, including Carter's Day. Japan 225's exact holiday
scope requires an operator definition or its own sourced calendar before
consumer routing can claim it is native/exact (#332).

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
| 2024-07-03 | early close | `13:15` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-07-04 | replacement blocks | `18:00; 13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Previous dated UK holiday section supplies the literal 18:00 ET reopen; this trade date supplies its printed holiday close. Exact book scope retained. |
| 2024-07-05 | replacement blocks | `18:00; 16:59` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Literal 18:00 ET holiday reopen from the previous dated UK section, combined with the sourced normal-week 16:59 ET close from OANDA-UK-HOURS-2024-WAYBACK; exact book scope retained. |
| 2024-09-02 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-09-03 | replacement blocks | `18:00; 16:59` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Literal 18:00 ET holiday reopen from the previous dated UK section, combined with the sourced normal-week 16:59 ET close from OANDA-UK-HOURS-2024-WAYBACK; exact book scope retained. |
| 2024-11-28 | early close | `13:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-11-29 | replacement blocks | `18:00; 13:15` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Previous dated UK holiday section supplies the literal 18:00 ET reopen; this trade date supplies its printed holiday close. Exact book scope retained. |
| 2024-12-24 | early close | `13:14` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-12-25 | closed | `closed` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |
| 2024-12-26 | late open | `18:04` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Archived UK dated section; exact book scope retained. |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `US indices open late 18:04`, `US bonds open late 18:04` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day: the holiday trade date is closed; the following trade date opens at the explicitly stated 18:04 |
| 2025-01-02 | late open | `18:04` | `OANDA-UK-HOLIDAYS` | T1 | 2025-01-01 evening reopen is keyed to 2025-01-02, whose opening day is the holiday. |
| 2025-01-09 | early close | `US index close early at 9:30 and reopen at 18:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The section's 24-hour context and close-before-reopen sequence select 09:30 ET; see the contextual derivation below. |
| 2025-01-10 | replacement blocks | `reopen at 18:00` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The January 9 evening reopen belongs to the following trade date; its ordinary final close is 16:59 ET. |
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

The module serves the witnessed normal-week eras; `profile_at` keys each
opening day to its era. `sourcing.rs` declares the normal week witnessed from
**2021-08-04**, the operator's archived page print that first lists the four
US index rows on the continuous whole-hour clock.

The spans, and what each rests on:

- **Opening days 2021-08-04 through 2023-06-04 — the whole-hour clock
  `18:00 - 17:00` New York** (the 2022-era prints label the rows New York
  `Sun - Fri 18:00 - 17:00`; the 2024 page relabels the same clock Chicago).
  The 2021-08-04 capture prints it (`OANDA-UK-HOURS-2021-AUG-WAYBACK`); the
  2023-06-05 capture still prints it (`OANDA-UK-HOURS-2023-JUN-WAYBACK`),
  and the audit's intervening replays print the identical grid.
- **Opening days 2023-06-05 through 2024-02-19 — the sourced intersection.**
  The changeover is undated (last earlier-grid print 2023-06-05, first
  current-grid print 2024-02-21, no Wayback capture between). The current
  grid nests inside the legacy print, so the intersection is the current
  grid's own shape — the span serves `18:01 - 16:59` per the charter's
  2026-10-04 rule, under-reporting the legacy state's extra boundary
  minutes; the residual is disclosed below. The machine-channel witnessing
  of 2026-10-16 does not move these bounds: the nested grids admit no
  positive candle witness for the current state (the section below).
- **Opening days from 2024-02-20 — the current grid `17:01 - 15:59`
  Chicago** (the 2024-02-21 capture, `OANDA-UK-HOURS-2024-WAYBACK`; later
  replays and the 2026-10-10 capture corroborate).
- **Below the 2021-08-04 witness**: carried. The 2021-03-03 and 2021-05-08
  prints list the same instruments at the futures-exact `18:00 - 16:15;
  16:30 - 17:00` pair — a real witnessed arrangement, but one that would
  model the CME maintenance halt inside a broker book, which the shipped
  book shape does not do; it stays a recorded #332 residual with its own
  build as the named closer.

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page, captured 2026-10-10 22:36 UTC and rendered by the maintainer to
  PDF the same UTC date.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

## Machine-channel witnessing — 2026-10-16 UTC, corrected 2026-10-17 in review

On 2026-10-16 (UTC), under the maintainer's direction, the changeover was
witnessed from the operator's own machine channel — the
practice-environment candle service (`api-fxpractice.oanda.com`, the
`/v3/accounts/<id>/instruments/SPX500_USD/candles` endpoint, read as bytes
through the `oanda-rs` client and paced by its own admission control), the
same T2 class the MLK-2023 closure used under the `MARKET-DATA`
convention. The confirming raw responses are retained privately, so the id
deliberately does not resolve in the research store.

The census and what it can and cannot state:

- The per-day M1 sweep (2023-06-06..2024-02-19, one request per day) found
  an 18:00-minute bucket on 183 opening days and none on 76 quiet days.
  The first pass labelled those 183 days legacy and the span's tail
  current; that was retracted in review — an 18:00-minute M1 bucket is
  ambiguous between a sharp legacy open and the observed five-second
  current-open lead (18:00:55), which lands in the same minute. The 76 quiet days are
  Fridays, Saturdays, and the 2023-12-24/2023-12-31 holiday Sundays.
- The retained February 8 S5 close-side sample reaches `16:59:40` New
  York, inside the earlier whole-hour candidate and after the current
  candidate's `16:59` close. The same day's first reopen start is
  `23:00:00Z` (`18:00` New York). These positive observations support
  the earlier grid in the two-candidate comparison. No universal
  five-second lead/linger bound is inferred; the shared timestamp and
  quoting limitations are recorded in [`oanda_fx.md`](oanda_fx.md).

- **No observation positively proves the current grid on any day.** The
  current `18:01 - 16:59` window is a strict subset of the legacy
  `18:00 - 17:00` window: every tick a current-era day can print also fits
  the legacy grid, so the changeover cannot be dated from candles for this
  book. The bond and commodity candidates have the same nesting
  relationship; their own observations are recorded in their evidence files. The archived page-print bounds
  — the 2023-06-05 last legacy print and the 2024-02-21 first current
  print — therefore stand, and the span keeps the sourced intersection.
- The retained inputs do not supply sub-minute evidence for every day
  of the span; unretained mid-span S5 summaries do not substantiate a
  per-day or exact-cutover claim.

`MARKET-DATA` — T2, confirming artifacts retained privately (with sha256
manifest), retrieval 2026-10-16 UTC, corrected 2026-10-17 UTC.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOLIDAYS` | 2025-01-01 .. 2026-10-30 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours> | captured 2026-10-10 22:44 | T1 | `c7137ce3da4cc19d0431955feedc10fb6b9731214ad3cfc7966160e72f8559da` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |
| `OANDA-UK-CARTER-2025-WAYBACK` | 2025-01-09 | <https://web.archive.org/web/20250326184653id_/https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | Wayback `id_` replay of capture `20250326184653`, retrieved 2026-10-11 UTC | T1 | `467313685d4d64a4b95e5112a58d0fa04583f0abe988b1478d41fddf8b04cf1c` |
| `OANDA-UK-HOURS-2021-AUG-WAYBACK` | 2021-08-04 observation; not a cutover | <https://web.archive.org/web/20210804085600id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2021-08-04; retrieved 2026-10-12 UTC | T1 | `a4962cdf91f4acd0c2149a2bb1a6d6bfdc81bcfd0523eb3fdb72dcee381d8256` |
| `OANDA-UK-HOURS-2023-JUN-WAYBACK` | 2023-06-05 observation; not a cutover | <https://web.archive.org/web/20230605093523id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2023-06-05; retrieved 2026-10-11 UTC | T1 | `98ddc07cf38727b2a861fd1f48c503b1b597054cbec9d1dde4b3dde1801ffb56` |
| `OANDA-UK-CARTER-2026-WAYBACK` | 2025-01-09 | <https://web.archive.org/web/20260217213704id_/https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | Wayback `id_` replay of capture `20260217213704`, retrieved 2026-10-11 UTC | T1 | `33988f4e6cd9603250923da1660ce71dc3ee1e1a66610b7734b1ad0cbfbaa0c9` |

Store paths: see [`oanda_fx.md`](oanda_fx.md). The additional Carter replays are
`normal-weeks/oanda-uk-wayback-audit-2026-10-11/holiday-captures/20250326184653.html`
and `20260217213704.html` in that directory.

## Gaps and residual risks

- **The changeover stays undated after the machine-channel witnessing
  (2026-10-16, reviewed 2026-10-17).** An earlier version of this file
  claimed the boundary at 2024-02-11 from S5 lead buckets; retracted — the
  lead buckets fit both grids (a 18:00:55 tick is 55 seconds into a legacy
  session exactly as it is a 5-second lead of the current one), and the
  current grid nests inside the whole-hour legacy grid, so no candle
  observation can positively prove the current state on any day. The
  witnessing that stands: the sharp-open and deep-close positives on
  2024-02-08 (open `23:00:00Z` exact, close-minute ticks to `21:59:40Z`)
  witness the legacy candidate on that date; the other days' M1 buckets are
  ambiguous between the states. The machine-channel section above holds
  the corrected census; the named closer stays a dated operator artifact or desk answer.
- Bonds route to `oanda_us_bond_cfd`; no intersection is used as their exact calendar.
- **Japan 225 holiday scope is unresolved.** Its two listings share the normal
  Chicago grid, but the source separately names JP225 on only some dates.
  Closing condition: an operator definition proving the generic US-index
  holiday scope includes both listings, or a separate exact sourced calendar.
  The consumer must gate that scope until then (#332).
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

## Carter context derivation — 2026-10-11 UTC

The March 2025 and February 2026 UK holiday replays both carry the
Eastern-Time header and the dated 2025-01-09 Carter section. The exact
index statement is `US index close early at 9:30 and reopen at 18:00`.
Adjacent bond and commodity statements use `13:15` closes and `18:00`/`20:00`
reopens. This local 24-hour context and the stated close-then-reopen sequence
select 09:30 ET: 21:30 would fall after that dated 18:00 reopening. The
ordinary 16:59 close corroborates this reading, but the word `early` alone
is not the derivation. The operator prints `9:30`, not `09:30` or `AM`;
09:30 is the contextual interpretation recorded here. The following trade
date retains the literal 18:00 opening through a complete replacement block.
The earlier meridiem withholding was overly conservative; Memorial's
missing-minute withholding and the historical/scope gaps in #332 remain.

## Archived 2024 holiday window — 2026-10-11 UTC

The UK operator page captured on 2025-03-26 retains its dated 2024 holiday
changes from New Year's Day through New Year's Eve. Its page-wide
Eastern-Time instruction governs every printed instant. The audited window
is 2024-01-01..2024-12-31; December coverage rests on its expressly dated
2024 Christmas and New Year sections. The January 2, January 16 and
February 20 following-holiday final closes remain explicitly `Unsourced`: their exact normal-week close is not dated
by the page-print bounds. Earlier holiday history remains historical
completion work tracked in #332.

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

The Documents table above resolves the byte-identical archive for this
2024 work-up as well.

## Incomplete 2026 reopen recheck — 2026-10-11 UTC

A fresh UK-page retrieval at 22:54 UTC still prints the literal incomplete
Memorial Day reopen `18:0`; both the rendered HTML and embedded structured
text agree. Canada's corresponding page prints 18:01, but its different
legal-entity scope does not establish the UK minute. This recheck changes
no runtime row and keeps the 2026-05-26 refusal tracked in #332.
