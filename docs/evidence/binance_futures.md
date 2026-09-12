<!-- SPDX-License-Identifier: MIT-0 -->

# `binance_futures` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`binance.rs`](../../src/calendar/schedules/futures/international/binance.rs)
- **Source sets:** [`CRYPTO-BINANCE`](../schedules/sources.md#crypto-binance)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Binance USDⓈ-M perpetuals' normal 24×7 availability: closed before the archived official 2019-09-13 04:00 UTC platform launch, then continuously open. Ad-hoc maintenance, contract launch/delist events, and incidents are excluded.

## Revision rows

None. binance.rs encodes its dated cutovers as constants rather than revisions! tuples, so it has no dated revision row.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://arquivo.pt/noFrame/replay/20200608065459id_/https://www.binance.com/en/support/articles/360033314152> — Binance's archived official launch article, stating that Binance Futures went live at 2019-09-13 04:00 UTC.
- <https://t.me/binance_announcements/799> — Binance announcement channel post for the launch.
- <https://www.binance.com/en-TR/support/announcement/detail/2bfb6f8dccf447ada57165b7e6a4cf1b> — current USDⓈ-M perpetual launch specifications, publishing 24/7 trading.

## Gaps and residual risks

- The module encodes the launch as the constants `LAUNCH_UNIX_SECONDS` (2019-09-13 04:00:00 UTC) and `LAUNCH_DAY_END_UNIX_SECONDS` rather than as `revisions!` tuples, so this file records no revision-row bullets even though the identity has a dated launch. An exact instant is required here: the launch is not a venue-local-midnight revision (LAW-NO-FABRICATED-DATES).
- This is a normal-availability profile. Ad-hoc maintenance windows, per-contract listing and delisting windows, and operational incidents are excluded and are not modelled anywhere in the crate.
- The profile has no daily close and no weekend close, so date-aware `trade_date` is always `None` for it.
