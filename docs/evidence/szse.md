<!-- SPDX-License-Identifier: MIT-0 -->

# `szse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`szse.rs`](../../src/calendar/schedules/equities/apac/szse.rs)
- **Source sets:** [`APAC-CHINA-CASH`](../schedules/sources.md#apac-china-cash)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue union includes the 15:00–15:30 block-order/trading phase already operative at the January-2010 floor. Later ChiNext eligibility does not create a new outer-envelope cutover.

## Revision rows

- 2016-05-09 — T1 — SZSE notice t20160930_518722 — the trading host stops accepting declarations in the 09:25–09:30 blocking interval; the session intervals are otherwise unchanged.

## Sources

- <https://www.szse.cn/lawrules/rule/trade/current/t20260424_620190.html> — current SZSE trading rule, including the 15:00–15:30 block and fixed-price phase.
- <https://www.szse.cn/disclosure/notice/general/t20060515_499577.html> — SZSE block-trading rule effective 2006-07-01, establishing block declarations through 15:30 before the January-2010 audit floor.
- <https://docs.static.szse.cn/www/disclosure/notice/W020180328432928783546.pdf> — SZSE Trading Rules (2013 revision). Art. 2.4.2 sets the opening call auction at 09:15–09:25 and continuous auction from 09:30; Art. 3.3.1 adds "每个交易日 9:25 至 9:30，交易主机只接受申报，但不对买卖申报或撤销申报作处理" — from 09:25 to 09:30 the trading host only accepts declarations and processes neither orders nor cancellations.
- <https://www.szse.cn/aboutus/trends/news/t20160930_518722.html> — SZSE notice dating the end of order acceptance in the blocking interval to 2016-05-09.

## Gaps and residual risks

- The pre-2016 09:25–09:30 `order_entry` window rests on the 2013 revision of the SZSE Trading Rules, an artifact issued after the January-2010 floor. The state is carried back to the floor and the residual risk is recorded here (AGENTS.md, *Carry the earliest sourced state back to the floor*).
- Block and fixed-price phases are `extended` by convention; not every security is eligible for them.
- The 2026-07-06 generic fixed-price expansion is deliberately not a revision row: it changed eligibility inside the existing venue envelope, not the exchange-level close.
