<!-- SPDX-License-Identifier: MIT-0 -->

# `tsx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tsx.rs`](../../src/calendar/schedules/equities/americas/tsx.rs)
- **Source sets:** [`AMER-TSX`](../schedules/sources.md#amer-tsx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Cash equities; conditional MOC extension represented by its maximum envelope.

## Revision rows

None. `tsx.rs` holds a single static profile with no dated revision row.

## Sources

- <https://www.tsx.com/en/trading/calendars-and-trading-hours/trading-hours> — TSX trading hours: orders accepted from 07:00, continuous trading 09:30–16:00, the conditional Market-on-Close Price Movement Extension through 16:10, and Extended Trading at the last sale price 16:15–17:00. The venue's session table describes Pre-Open as a phase in which orders may be entered but will not be executed.
- <https://www.osc.ca/sites/default/files/pdfs/bulletins/oscb_20050114_2802.pdf> — Ontario Securities Commission Bulletin of 2005-01-14, volume 28 issue 2: the regulator record establishing that both the Price Movement Extension and the last-sale session existed before the January-2010 history floor.

## Gaps and residual risks

- **Interpretive step, order-entry classification.** The 07:00–09:30 Pre-Open is `order_entry`: no trade can match inside it, and the first print of the day is the 09:30 Market-on-Open cross that starts continuous trading.
- **Interpretive step, conditional extension.** The Price Movement Extension rule is modelled as the venue's maximum envelope. On ordinary days and symbols the 16:00–16:10 interval is cancel-only, but when the extension fires it is the delayed Market-on-Close cross for that symbol and it prints, so the window is not order-entry-only. The separate 16:10–16:15 Post Market Cancel Session is not modelled at all.
- **No dated revision.** The reviewed grid holds for the whole audit window, so every instant resolves to the one profile. A sourced revision later replaces this with a real timeline row and needs no routing change. Closing condition for a future change: a TMX notice stating an unconditional day-level effective date.
- **Dormant identity.** No consumer adapter admits a TSX namespace, so this row is reviewed on demand and its follow-ups are recorded here rather than as issues.
