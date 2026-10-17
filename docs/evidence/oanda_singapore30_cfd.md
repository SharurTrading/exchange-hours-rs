<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_singapore30_cfd` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda_asia.rs`](../../src/calendar/schedules/futures/international/oanda_asia.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-12 UTC)

The broker current day/night grid and bounded holiday audit are documented below.

## Revision rows

None. No `revisions!` block exists and no changeover day is asserted: the
module's era bounds are the archived page-print observations listed under
`## Dated selectors`, and the machine-channel witnessing (recorded in
[`oanda_china50_cfd.md`](oanda_china50_cfd.md)) dates no current-grid day.

## The normal week

OANDA Europe Limited's UK hours page lists `Singapore 30`, timezone `Singapore`,
with `Mon - Fri 08:31 - 17:19; 17:51 - 05:14`. The archived 2024-02-21 UK hours page already prints this exact grid. The
2026-10-10 captured HTML and maintainer-rendered PDF, and fresh 2026-10-12
HTML, corroborate it. Singapore is UTC+08:00 with no seasonal offset.
The first block is daytime and the second opens on each Monday–Friday and
wraps into the next civil day. Friday night therefore ends on Saturday.
The containing block's closing civil day supplies its trade date; Monday night and Tuesday daytime both carry Tuesday's
closing civil trade date, while session bounds preserve the separate blocks. This is the
crate's closing-day convention, not SGX's futures trade-date assignment.

The operator publishes no separate regular-hours designation or timed
order-entry phase for this CFD. `regular` is empty; both published executable
blocks are `extended`, and `order_entry` is empty. Pending-order storage is a
broker facility rather than a sourced timed phase (#333).
No SGX expiry, last-trading-day, first-settlement-day, auction or futures
maintenance time is copied into the broker profile.

## Holidays

**Coverage:** 2026-10-10..2026-10-30 (inclusive trade dates).

Reviewed 2026-10-12 UTC, with **zero exception rows**. The operator's UK holiday changes list names CN50 and SG30
when it changes these scopes (for example New Year's Day 2026), so these
names are audited directly rather than inferred from a generic SGX calendar.
Within the current window the only listed future changes are CHINAH/HK33 on
2026-10-19 and Swedish share CFDs on 2026-10-30; neither names this book.
The witnessed ordinary grid therefore supplies the audited current sessions.
No blank year-end publication is treated as normal beyond October 30.

## Dated selectors

The module serves the witnessed normal-week eras; `profile_at` keys each
Singapore opening day to its era. `sourcing.rs` declares the normal week
witnessed from **2021-03-03**, the operator's archived page print that first
lists the Singapore 30 row.

The spans, and what each rests on (the module's era-bound constants are
keyed observations, not effective days — LAW-NO-FABRICATED-DATES):

- 2023-06-06 — T1 — the 2023-06-05 05:35 UTC page print (`OANDA-UK-HOURS-2023-JUN-WAYBACK`) is the last earlier-grid observation, taken 13:35 Singapore inside that day's day session; the era-bound constant keys the following opening day.
- 2024-02-21 — T1 — the 2024-02-21 00:55 UTC page print (`OANDA-UK-HOURS-2024-WAYBACK`) is the first current-grid observation, taken 08:55 Singapore inside that day's day session; the era-bound constant keys this opening day.
- **Opening days 2021-03-03 through 2023-06-05 — the earlier grid
  `Mon - Fri 08:30 - 17:20; 17:50 - 04:45`.** The 2021-03-03 capture prints
  it (`OANDA-UK-HOURS-2021-WAYBACK`); the 2023-06-05 capture — taken 13:35
  Singapore, inside that day's day session — still prints it
  (`OANDA-UK-HOURS-2023-JUN-WAYBACK`), and every intervening replay prints
  the identical grid.
- **Opening days 2023-06-06 through 2024-02-20 — the sourced intersection.**
  The changeover is undated (last earlier-grid print 2023-06-05, first
  current-grid print 2024-02-21 08:55 Singapore; no Wayback capture sits
  between). The span serves the window both witnessed states hold — day
  `08:31 - 17:19`, night `17:51 - 04:45`, the night close held at the
  earlier 04:45 rather than adopting the current 05:14 — per the charter's
  2026-10-04 rule; the residual is disclosed below. The machine-channel
  witnessing of 2026-10-16 witnesses the legacy grid positively through
  2024-02-09 but cannot date the current state (the note in
  [`oanda_china50_cfd.md`](oanda_china50_cfd.md) applies to this book's
  shared engine).
- **Opening days from 2024-02-12 — the current grid
  `08:31 - 17:19; 17:51 - 05:14`** (the 2024-02-21 capture,
  `OANDA-UK-HOURS-2024-WAYBACK`; the 2026-10-10 capture corroborates).
- **Holiday dates below the current audited window remain unclassified**;
  the coverage bridge answers their session questions from the served week
  while the classification refuses typed. Historical normal-week proof below
  the 2021-03-03 witness and holiday history remain disclosed in #332;
  closing evidence is a UK-scoped broker artifact establishing the missing
  earlier sessions.
The last audited Friday night opens on 2026-10-30 and closes on the local
Saturday 2026-10-31. On the audited Friday, `is_open` answers and `session_bounds` returns the
complete running block through its Saturday end-exclusive close. Instant
queries at and after Singapore midnight on the unaudited Saturday refuse,
even while that returned Friday block is still running.
`trade_date` refuses its October 31 closing-date classification because that
date is beyond the holiday audit. The night close and later instant queries likewise refuse rather than
certify the unaudited Saturday or next opening;
a monthly candle begun in October also refuses when it needs October 31.
A sufficiently specified UK holiday publication must extend the audited
window before those classifications or later sessions can answer. This scope
is served and reviewed monthly after current-scope activation on 2026-10-12 UTC
under the dated AGENTS.md exception; historical gaps remain #332.

## Sources

The operator UK normal-hours and holiday pages are the monitoring entry points.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-2021-WAYBACK` | 2021-03-03 | <https://web.archive.org/web/20210303210448id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20210303210448`, retrieved 2026-10-10 UTC; prints no Forex table | T1 | `ec5987239f2c79dd29fcfc40abe5a0d58c6e4ed446fa7dea653f1791b186cbed` |
| `OANDA-UK-HOURS-2022-WAYBACK` | 2022-05-18 | <https://web.archive.org/web/20220518092953id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20220518092953`, retrieved 2026-10-10 UTC | T1 | `8dd71db7accc2a40faeb531f0c5c29db0eb289c9ef45a69bc14b75148257cbea` |
| `OANDA-UK-HOURS-2023-JUN-WAYBACK` | 2023-06-05 observation; not a cutover | <https://web.archive.org/web/20230605093523id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2023-06-05; retrieved 2026-10-11 UTC | T1 | `98ddc07cf38727b2a861fd1f48c503b1b597054cbec9d1dde4b3dde1801ffb56` |
| `OANDA-UK-HOURS-2024-WAYBACK` | 2024-02-21 | <https://web.archive.org/web/20240221005503id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | Wayback `id_` replay of capture `20240221005503`, retrieved 2026-10-10 UTC | T1 | `aab9b3032c1d53e6155009fce6a5156983c2d21a2f0bd768f1860f465b8b6a63` |
| `OANDA-UK-HOURS-HTML` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | captured 2026-10-10 22:36 | T1 | `60f6143dd5d678b1a34a9937aa938d2884380a33bf3542a4c72f8f03fa3607b7` |
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOURS-CURRENT-2026-10-12` | current observation 2026-10-12; no cutover asserted | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | retrieved 2026-10-12 00:39 UTC | T1 | `1c986aee21fdd7b76d04cabc2a8846fe00e08bb28257a27cfada828291c7e063` |
| `OANDA-UK-HOLIDAYS-CURRENT-2026-10-12` | published changes through 2026-10-30; current observation 2026-10-12 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | retrieved 2026-10-12 00:39 UTC | T1 | `ce7cd4c3d4db0e32ec629715ee321bdd4d48455845894bee1eb8df768d066eff` |

Store paths: `normal-weeks/oanda-uk-hours-2026-10-10/` for the original HTML/PDF and
`oanda-uk-hours-of-operation@2024-02-21-wayback.html` historical corroboration;
`normal-weeks/oanda-current-source-audit-2026-10-12/uk-hours.html` and
`uk-holidays.html` for the fresh corroboration and current holiday audit.

## Gaps and residual risks

- **The changeover stays undated after the machine-channel witnessing
  (2026-10-16, reviewed 2026-10-17).** The first pass claimed a 2024-02-12
  boundary; retracted — the day session nests (current `08:31 - 17:19`
  inside legacy `08:30 - 17:20`), so its ticks cannot prove the current
  state. This book is **not** nested on the night leg (legacy night close
  `04:45`, current `05:14` — the current night session runs later), so a
  quote well inside 04:45–05:14 Singapore could positively distinguish the
  night state; the probe of those mornings is recorded below and dates no
  changeover. What stands: deep day-close-minute ticks to 17:19:45
  Singapore witness the legacy day grid positively on 2024-02-06..09; the
  2024-02-12/13 day close minutes are wholly silent and witness nothing
  either way. The archived page-print bounds stand; the span keeps the
  sourced intersection (night close 04:45 through it, 05:14 from the
  2024-02-21 page-print bound). The unaudited holiday dates inside the
  witnessed span keep their typed refusals; they are the remaining #332
  residual here.

## Night-leg machine-channel record — 2026-10-17 UTC

The night legs are offset between the page states (legacy close `04:45`,
current close `05:14` Singapore), so the night close minutes were probed
at S5 across the changeover window (mornings 2024-02-06..09 and
2024-02-12..16, 19, 20; window `04:44:00–05:15:00` Singapore, read through
the `oanda-rs` client as bytes, retained privately with the sha256
manifest). Result: continuous quoting through `[04:45:15, 05:14:00]`
Singapore on **every sampled morning on both flanks** — 02-06 through
02-09 (legacy by the day-session witnesses) and 02-13 through 02-16 and
02-20 (current by the day-session witnesses) — each morning ending at the
`05:14:00` close bucket (the 02-12 and 02-19 mornings are wholly silent).
Two consequences, both recorded rather than encoded:

- The night leg cannot date the day-session changeover: its close minute
  shows the same shape on both flanks.
- The night close is a standing T1/T2 conflict in the legacy era: the
  2023-06-05 page prints `04:45` while the operator's own machine channel
  quotes the leg past `04:45:15` continuously on every sampled
  legacy-side morning. The sourced intersection withholds
  `[04:45, 05:14]` accordingly (the charter's conflict rule: serve the
  window both page states state; record the dispute). Closing condition:
  a legacy-era operator artifact stating the night close, or a desk
  answer.

Pre-witness history and the forward publication horizon remain explicit in #332.
