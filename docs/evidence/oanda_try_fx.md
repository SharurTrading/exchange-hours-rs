<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_try_fx` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's TRY-pair book, carved out of the FX row by name: a daily Monday–Friday
08:00–17:00 Central European session with an overnight gap and no stated
maintenance.


## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and a dormant identity owes its current
schedule rather than a worked-up history (LAW-BOUNDED-WORK).

## The normal week

The hours-of-operation page prints:

> `TRY Pairs | Central Europe | 100,000 | Mon-Fri: 08:00 - 17:00`

Interpretation, each step from the operator's own words:

- The row prints **no `***` daylight-saving marker**, so the endpoints track
  the printed zone: "Central Europe" names a CET/CEST offset, and the grid
  anchors to `Europe/Berlin` (any CET zone answers this grid identically).
- The `Mon-Fri: 08:00 - 17:00` row shape is the page's **daily-session**
  notation — the same shape the share-CFD rows use — not the `Sun-Fri`
  weekly span of the FX book, so the grid is five daily sessions with an
  overnight gap, `has_daily_close`/`has_weekend_close` both true.

## Holiday rows

None ship. The operator's UK holiday page prints no TRY-specific
arrangement on any dated section 2025-01-01..2026-10-30, and whether its
`Forex` rows (Christmas Day and New Year's Day only) reach the TRY book is
unworked up. Closing condition: an OANDA support answer naming the TRY
book's Christmas and New Year arrangement; until then the key carries no
holiday layer and the caller's `DayPolicy` is the only layer.

## Dated selectors

None. The module has one sourced grid; `sourcing.rs` declares every day
below the 2026-10-10 capture carried, because the living hours page is the
grid's only witness.

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page for the UK entity, captured 2026-10-10 22:36 UTC (HTML) and
  rendered by the maintainer to PDF the same UTC date. The TRY row is the
  grid's only statement.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, whose dated sections 2025-01-01..2026-10-30
  state no TRY arrangement; the negative that leaves this key without a
  holiday table.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOURS-HTML` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | captured 2026-10-10 22:36 | T1 | `60f6143dd5d678b1a34a9937aa938d2884380a33bf3542a4c72f8f03fa3607b7` |

Store paths: see [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- **Every day before the 2026-10-10 capture is carried** — the TRY grid's
  only witness is the living page. The date-aware surface refuses the whole
  carried span; activation requires a Wayback sweep or an operator statement
  dating the grid.
- The TRY carve-out exists because of the pair set's restricted schedule;
  if OANDA widens or narrows the TRY pair list the grid may change, and the
  page is the watch entry point.
