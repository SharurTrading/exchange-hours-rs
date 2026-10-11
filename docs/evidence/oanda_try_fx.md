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
`Forex` rows (the three reopen statements on New Year's Day 2025, Christmas
Day and New Year's Day 2026, plus the two close-at-16:59 no-reopen
statements on Christmas Eve and New Year's Eve 2025) reach the TRY book is
unworked up. Closing condition: an admissible UK-scoped operator artifact naming the TRY
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
| `OANDA-JAPAN-TRY-2025` | Japan only; notice 2025-08-04, change 2025-08-11 | <https://www.oanda.jp/info/1305> | retrieved 2026-10-11 UTC | T1 | `b595964642087861a81d3a6aa3c648f9b89536719f36f849dbfab4b107251a6f` |
| `OANDA-TMS-TRY-2025-JUNE` | TMS only; valid from 2025-06-15 | <https://www.oanda.com/eu-en/sites/default/files/document_files/sif-tms-connect-pro-eng-15.06.2025.pdf> | retrieved 2026-10-11 UTC | T1 | `6dc257347389ca311f7313f3ff660e93d8b7b0fd9ed1f587a434e22ef1330e80` |
| `OANDA-TMS-TRY-2025-OCT` | TMS only; valid from 2025-10-20 | <https://www.oanda.com/eu-en/sites/default/files/document_files/sif-tms-connect-eng-20.10.2025.pdf> | retrieved 2026-10-11 UTC | T1 | `b8c02e7040cf1b690d22ff05cba971831dfe247c68aed33f05d3b0b19753a0ab` |

Store paths: see [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- **Every day before the 2026-10-10 capture is carried** — the TRY grid's
  only witness is the living page. The date-aware surface refuses the whole
  carried span; activation requires a Wayback sweep or an operator statement
  dating the grid.
- The TRY carve-out exists because of the pair set's restricted schedule;
  if OANDA widens or narrows the TRY pair list the grid may change, and the
  page is the watch entry point.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).

## Regional applicability audit — 2026-10-11 UTC

None of the 19 usable UK hours replays in the existing archive capture set
prints a TRY-specific row. The current UK row remains this identity's only
normal-week witness. Distinct operator evidence establishes why a regional
announcement cannot silently provide the missing UK day:

- `OANDA-JAPAN-TRY-2025`, issued 2025-08-04, names TRY/JPY, EUR/TRY and
  USD/TRY and announces changed trading hours from 2025-08-11. It describes
  replacing the full FX week with daily European-session trading. Its scope
  is OANDA Japan; it does not name OANDA Europe Limited.
- `OANDA-TMS-TRY-2025-JUNE`, generated 2025-06-12 and valid from
  2025-06-15, prints the full FX week for EURTRY.pro and USDTRY.pro, marked
  close only. `OANDA-TMS-TRY-2025-OCT`, generated 2025-09-18 and valid from
  2025-10-20, instead prints `monday–friday 08:00–17:00` for those two
  instruments, also close only. These dated specifications belong to
  OANDA TMS Brokers S.A., not OANDA Europe Limited; the later specification
  does not prove the first effective day of a UK change.

The UK historical-day refusal remains honest. The Japan and TMS effective
days must not key a UK revision, and a common current session does not prove
common holiday arrangements. A UK-scoped dated change artifact would close
the normal-week history gap; a UK-scoped holiday statement would close exact
broker holiday applicability (#332). The operator's dated OEL execution
policy also says broker hours may differ from underlying exchanges; see
[the FX audit](oanda_fx.md#historical-normal-week-audit--2026-10-11-utc).
An inherited calendar selected by product timezone is an explicit caller
policy, with its source and limitations disclosed, rather than T1 proof of
this book's holiday arrangements.

Audit artifact paths in the research store:
`normal-weeks/oanda-history-audit-2026-10-11/` with filenames
`oanda-japan-try-change-2025-08-04.html`, `tms-try-2025-june.pdf`,
and `tms-try-2025-october.pdf`.
