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
no dated, day-level change to record, and no unconditional changeover day is asserted. Historical completion
remains tracked in #332 under the dated current OANDA scope exception.

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

## Holidays

**Coverage:** 2025-11-17..2026-10-30 (inclusive trade dates, T1 Forex category audit).

The archived UK hours page (`OANDA-UK-HOURS-2025-NOV-WAYBACK`) places the
ordinary FX row and `TRY Pairs` within the same `Forex CFDs` section. The
current UK hours page confirms that grouping. The UK holiday page
(`OANDA-UK-HOLIDAYS`) scopes the following restrictions to Forex as a
category. Intersecting those restrictions with the TRY grid yields two
closures.

On both preceding eves the 16:59 New York close is 22:59 CET, after
the 17:00 CET close. The other holiday sections in the audited range
specify changes to other books. The October 30 publication boundary and
earlier carried dates remain explicit coverage limits tracked in #332.

### 2025

| Trade date | Kind | Time as printed | Document | Tier | Effect on the TRY grid |
|---|---|---|---|---|---|
| 2025-12-25 | closed | `Forex: open at 17:05` | `OANDA-UK-HOLIDAYS` | T1 | The 17:05 New York reopen is 23:05 CET, after the daytime session ends. |

### 2026

| Trade date | Kind | Time as printed | Document | Tier | Effect on the TRY grid |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Forex: open at 17:05` | `OANDA-UK-HOLIDAYS` | T1 | The 17:05 New York reopen is 23:05 CET, after the daytime session ends. |

## Dated selectors

The 2025-07-08 replay prints the ordinary FX week under `All FX products`
without a TRY-specific row. The 2025-11-15 replay first witnesses the TRY
carve-out at 02:32:04 UTC on a Saturday. December 2025, February and April
2026 replays corroborate the restricted grid. The declaration starts
on Monday, 2025-11-17, the first opening day after the archived witness.
The UK effective day between the July and November observations remains
unresolved. The module carries only the one witnessed grid.

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page for the UK entity. The November 2025 through April 2026
  replays and the current UK page print the same TRY row.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, whose Forex category supplies the bounded
  2025-11-17..2026-10-30 audit above.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `OANDA-UK-HOURS-2025-JUL-WAYBACK` | 2025-07-08 observation | <https://web.archive.org/web/20250708202954id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2025-07-08; retrieved 2026-10-13 UTC | T1 | `12ace0b5ae89d132e82242e6fe697f5bce745e5699906028fcf67a0819a432c6` |
| `OANDA-UK-HOURS-2025-NOV-WAYBACK` | 2025-11-15 observation | <https://web.archive.org/web/20251115023204id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2025-11-15; retrieved 2026-10-13 UTC | T1 | `d5e3a4a9ef4309e2365866135957593eab1e007b86a9e2e8ca052aa65723d1b7` |
| `OANDA-UK-HOURS-2025-DEC-WAYBACK` | 2025-12-13 observation | <https://web.archive.org/web/20251213234655id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2025-12-13; retrieved 2026-10-13 UTC | T1 | `ccc90896b61f46552fb3f93e90e8f5021b28d441d2efc215ff2244f08112475a` |
| `OANDA-UK-HOURS-2026-FEB-WAYBACK` | 2026-02-09 observation | <https://web.archive.org/web/20260209190304id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2026-02-09; retrieved 2026-10-13 UTC | T1 | `4166cda4a13fa6acd7c3040ee51f027b39dfc7f50da1bb565d922ebf7b4b8d5b` |
| `OANDA-UK-HOURS-2026-APR-WAYBACK` | 2026-04-11 observation | <https://web.archive.org/web/20260411231933id_/https://www.oanda.com/uk-en/trading/hours-of-operation/> | capture 2026-04-11; retrieved 2026-10-13 UTC | T1 | `27bf3ed23f951e0db8fd1defe8781923f508166e518232bf28a107965a26c002` |
| `OANDA-UK-HOLIDAYS` | 2025-01-01 .. 2026-10-30 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours> | captured 2026-10-10 22:44 UTC | T1 | `c7137ce3da4cc19d0431955feedc10fb6b9731214ad3cfc7966160e72f8559da` |
| `OANDA-UK-HOURS-PDF` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | rendered 2026-10-10 (maintainer-saved page-to-PDF) | T1 | `d8a853cd621313feec96eb1ee0f45b81b7674dea2e0e6cb50b175e78347153d7` |
| `OANDA-UK-HOURS-HTML` | current | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | captured 2026-10-10 22:36 | T1 | `60f6143dd5d678b1a34a9937aa938d2884380a33bf3542a4c72f8f03fa3607b7` |
| `OANDA-UK-HOURS-CURRENT-2026-10-12` | current observation 2026-10-12; no cutover asserted | <https://www.oanda.com/uk-en/trading/hours-of-operation/> | retrieved 2026-10-12 00:39 UTC | T1 | `1c986aee21fdd7b76d04cabc2a8846fe00e08bb28257a27cfada828291c7e063` |
| `OANDA-UK-HOLIDAYS-CURRENT-2026-10-12` | published changes through 2026-10-30; current observation 2026-10-12 | <https://www.oanda.com/uk-en/trading/holiday-trading-hours/> | retrieved 2026-10-12 00:39 UTC | T1 | `ce7cd4c3d4db0e32ec629715ee321bdd4d48455845894bee1eb8df768d066eff` |
| `OANDA-JAPAN-TRY-2025` | Japan only; notice 2025-08-04, change 2025-08-11 | <https://www.oanda.jp/info/1305> | retrieved 2026-10-11 UTC | T1 | `b595964642087861a81d3a6aa3c648f9b89536719f36f849dbfab4b107251a6f` |
| `OANDA-TMS-TRY-2025-JUNE` | TMS only; valid from 2025-06-15 | <https://www.oanda.com/eu-en/sites/default/files/document_files/sif-tms-connect-pro-eng-15.06.2025.pdf> | retrieved 2026-10-11 UTC | T1 | `6dc257347389ca311f7313f3ff660e93d8b7b0fd9ed1f587a434e22ef1330e80` |
| `OANDA-TMS-TRY-2025-OCT` | TMS only; valid from 2025-10-20 | <https://www.oanda.com/eu-en/sites/default/files/document_files/sif-tms-connect-eng-20.10.2025.pdf> | retrieved 2026-10-11 UTC | T1 | `b8c02e7040cf1b690d22ff05cba971831dfe247c68aed33f05d3b0b19753a0ab` |

The UK hours replays are in
`normal-weeks/oanda-withheld-holidays-2026-10-13/hours-retries/` in the
research store. The UK holiday page and two October 12 captures are
filed as listed in [`oanda_fx.md`](oanda_fx.md).

## Gaps and residual risks

- The date-aware surface refuses the carried span before 2025-11-17.
  Historical completion in #332 still requires UK evidence dating the
  switch and establishing the earlier UK TRY sessions and holidays.
- The TRY carve-out exists because of the pair set's restricted schedule;
  if OANDA widens or narrows the TRY pair list the grid may change, and the
  page is the watch entry point.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).

## Regional applicability audit — 2026-10-11 UTC

The initial pass over 19 UK replays found no TRY-specific row.
The later retrieval of the November 2025 UK replay supplies an earlier
witness. Regional artifacts establish the following changes in their
own scopes:

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

The UK effective day requires an OANDA Europe Limited artifact. Its
execution policy says broker hours may differ from underlying exchanges.
See [the FX audit](oanda_fx.md#historical-normal-week-audit--2026-10-11-utc).
An inherited calendar selected by product timezone is a disclosed caller
policy. The native TRY schedule uses the UK artifacts listed above.

Audit artifact paths in the research store:
`normal-weeks/oanda-history-audit-2026-10-11/` with filenames
`oanda-japan-try-change-2025-08-04.html`, `tms-try-2025-june.pdf`,
and `tms-try-2025-october.pdf`.
