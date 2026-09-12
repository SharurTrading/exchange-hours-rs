<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_milan` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs), [`milan.rs`](../../src/calendar/schedules/equities/europe/euronext/milan.rs)
- **Source sets:** [`EU-EURONEXT`](../schedules/sources.md#eu-euronext), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Borsa Italiana principal-share history includes its January-2010 baseline, sourced 2013 and 2015 phase revisions, and the 2023-03-27 Optiq migration. Randomized uncrosses use the documented conservative latest-edge envelope.

## Revision rows

- 2013-09-30 — T1 — Borsa Italiana CPX launch notice — executable Closing Price Cross phase launched on Monday 2013-09-30, extending the closing envelope through 17:40.
- 2015-11-23 — T1 — Borsa Italiana notice 17016 — continuous close moves to 17:30, closing-auction latest uncross to 17:35:59, Closing Price Cross end to 17:42.
- 2023-03-27 — T1 — Euronext Go-Live Weekend Guidelines — Milan equities migrate to Euronext Optiq phase one: 07:30 pre-opening, open by the latest 09:00:30 uncross, continuous to 17:30, executable through 17:40.

## Sources

- <https://www.borsaitaliana.it/borsaitaliana/ufficio-stampa/comunicati-stampa/2009/090908nuoviorari.htm> — Borsa Italiana press release, new hours effective 2009-09-29: the resulting January-2010 baseline starts pre-opening at 08:00, uncrosses randomly from 09:00–09:01, trades continuously through 17:25, and closes its randomized auction by 17:31.
- <https://www.borsaitaliana.it/azioni/notiziedettaglio/cpx.en.htm> — Borsa Italiana Closing Price Cross launch notice, Monday 2013-09-30.
- <https://www.borsaitaliana.it/borsaitaliana/regolamenti/avvisi/17016orarineg.pdf> — Borsa Italiana notice 17016, effective 2015-11-23.
- <https://connect.euronext.com/sites/default/files/it-documentation/Guide%20to%20Trading%20System%20-%20Borsa%20Italiana%20Migration%20to%20Optiq%20-%20Functional%20Changes%20v.2.0.pdf> — Euronext guide to the trading system, Borsa Italiana migration to Optiq, functional changes v2.0.
- <https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf> — Euronext Go-Live Weekend Guidelines, giving 2023-03-27 for the Italian migration and 2023-03-20 for the legacy markets.
- <https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx> — current appendix to Euronext Instructions 4-01/4-03 Trading Manuals: the opening uncrossing "will randomly occur between CET 09:00:00 and 09:00:30".
- <https://www.euronext.com/en/trading/trading-hours-holidays> — Euronext trading hours and holidays, the source set's current entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the principal-share segment of the former Borsa Italiana MTA book, now Euronext Milan, and not every listed segment.
- **Interpretive step, order-entry classification.** In every era the pre-auction leg is order accumulation and only the uncross prints. Before Optiq the 08:00–09:00 leg is `order_entry` and the 09:00–09:01 randomized uncross stays `extended`; after Optiq the 07:30–09:00 pre-opening is `order_entry` because nothing matches on the central order book before 09:00.
- **Interpretive step, randomized uncrosses.** Opening and closing uncrosses are modelled at their conservative latest edge in each era: 09:01 and 17:31 at the baseline, 17:36 and 17:42 after notice 17016, and 09:00:30 and 17:35:30 after Optiq.
- **Owner cell.** The rule data and timeline for this identity live in `milan.rs`, a private child of `euronext.rs`; both modules are named in the Owner cell, timeline module last, per the reshape spec's Owner grammar.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.

> Shared module. [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs) also carries
> [`euronext_paris`](euronext_paris.md), [`euronext_amsterdam`](euronext_amsterdam.md), [`euronext_brussels`](euronext_brussels.md), [`euronext_lisbon`](euronext_lisbon.md).
> The anchor identity for that module is [`euronext_paris`](euronext_paris.md), which receives its narrative
> when LAW-EVIDENCE-FILES moves it. `milan.rs` is carried by this row alone, so its narrative belongs here;
> neither module is yet migrated.
