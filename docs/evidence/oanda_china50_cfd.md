<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_china50_cfd` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda_asia.rs`](../../src/calendar/schedules/futures/international/oanda_asia.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-12 UTC)

The broker current day/night grid and bounded holiday audit are documented below.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; the witness is not a dated change.

## The normal week

OANDA Europe Limited's UK hours page lists `China A50`, timezone `Singapore`,
with `Mon - Fri 09:01 - 16:29; 17:01 - 04:44`. The archived 2024-02-21 UK hours page already prints this exact grid. The
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

There is no dated revision: 2024-02-21 dates the earliest cited witness of
this current grid, not a broker cutover. Earlier normal weeks remain carried.
The older UK captures print different grids but do not state an unconditional
changeover day. Holiday dates below the current audited window remain
unclassified; the coverage bridge does not certify them as ordinary trading.
Historical normal-week and holiday proof remains disclosed in #332; closing
evidence is a UK-scoped broker artifact establishing the missing earlier sessions.
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

Pre-witness history and the forward publication horizon remain explicit in #332.
