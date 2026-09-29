<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_paris` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs)
- **Source sets:** [`EU-EURONEXT`](../schedules/sources.md#eu-euronext), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Core shares at the published nominal exchange boundaries: legacy 07:15 pre-open, 09:00–17:30 continuous trading, 17:30–17:35 closing auction, and Trading-at-Last through 17:40; the 2023-03-20 move to a 07:30 pre-open is date-aware. Per-security 0–30-second auction uncross timing is outside scope and does not change venue availability.

## Revision rows

- 2023-03-20 — T1 — Euronext Go-Live Weekend Guidelines — legacy-market pre-opening moves from 07:15 to 07:30 CET.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.euronext.com/sites/default/files/european_cash_markets_trading_hours_for_24th_and_31st_december_2010.pdf> — Euronext, "European cash markets trading hours for 24th and 31st December 2010": the operator's 2010 special-day appendix, showing the legacy 07:15 CET pre-opening and the principal-share opening at 09:00.
- <https://connect.euronext.com/nl/listview/notice-download?attachmentId=201416&id=581906&type=PDF> — Euronext 2014 normal-hours trading appendix, repeating the same legacy grid.
- <https://live.euronext.com/en/listview/notice-download?id=598779&type=PDF&attachmentId=218289> — Euronext notice PAR_20150924_07448_EUR, 2015 cash-market auction-randomization notice: zero-to-30-second randomized uncrosses for Belgian, Dutch, French and Portuguese trading groups.
- <https://live.euronext.com/en/listview/notice-download?id=598933&type=PDF&attachmentId=218443> — companion Euronext cash-market notice for the same randomization change.
- <https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf> — Euronext Go-Live Weekend Guidelines: the phase-one timetable makes the legacy pre-opening change effective 2023-03-20 and separately gives 2023-03-27 for the Italian migration.
- <https://connect.euronext.com/sites/default/files/it-documentation/Guide%20to%20Trading%20System%20-%20Borsa%20Italiana%20Migration%20to%20Optiq%20-%20Functional%20Changes%20v.2.0.pdf> — Euronext guide to the trading system, Borsa Italiana migration to Optiq, functional changes v2.0.
- <https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx> — current appendix to Euronext Instructions 4-01/4-03 Trading Manuals: pre-opening 07:30 CET, nominal continuous trading 09:00–17:30, closing auction to 17:35, Trading-at-Last through 17:40 for principal shares, with randomized zero-to-30-second uncross timing per security.
- <https://www.euronext.com/en/trading/trading-hours-holidays> — Euronext trading hours and holidays, the source set's current entry point.
- <https://www.euronext.com/en/regulation/euronext-regulated-markets> — Euronext regulated-market manual hub, the monitoring entry point.
- <https://www.euronext.com/en/products-services/cash-market-notices> — Euronext cash-market notices, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Holidays

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates in `Europe/Paris`; tier T1 throughout).

Euronext publishes one holiday calendar covering its cash markets with a per-market column —
Paris is the last column — on its own `Trading hours & Holidays` page
(`live.euronext.com/en/resources/trading-hours-holidays`), and a per-year INFO-FLASH PDF
beside it. The page keeps only the latest two years, so the 2025 column is read from the
page's archived 2025-12-06 state and the 2026 column from the live page, corroborated by the
INFO-FLASH "2026 Holiday Calendar for Euronext's Cash and Derivatives markets". Euronext Paris
runs no overnight session, so an event date and its trade date are one civil day and the
conversion is the identity.

**December half days are stated only by the end-of-year appendix.** Both year tables print
`**Half Trading Day` for Paris on 24 and 31 December, and their footnote defers the hours to
"an end of year appendix to the Euronext Instructions 4-01 4-03 Trading Manuals". For 2025
that appendix exists — the operator's own XLSX ("Appendix of Trading Manual 4-01"), header
`24th and 31st of December 2025`, scope `Amsterdam, Brussels, Dublin, Lisbon, Milan, Oslo &
Paris` — and its Paris equity segments (SHARES - IPO - CONTINUOUS, Equities SRD, Foreign
Equities SRD) print OU `09:00 Random`, continuous trading `09:00 Random - 13:55`, closing
uncross `14:00 Random` and TAL `14:00 - 14:05 Continuous TAL Random`. The day's availability
envelope therefore ends at CET 14:05:00 and each 2025 eve ships as an `EarlyClose` at 50 700
seconds. For 2026 the appendix is announced but unpublished — the live page states "2026 end
of year Trading hours: To be announced" — so 2026-12-24 and 2026-12-31 ship as `Unsourced`
with the closing condition "the 2026 end-of-year appendix to the Euronext Instructions
4-01/4-03". Nothing is inferred from the 2025 appendix; the operator's own precedent is not
evidence of 2026.

**2027 is not published.** The live page's newest calendar is the 2026 one and no 2027 edition
exists on live.euronext.com (sitemap grep over `holiday|calendar|trading-hours`, queried
2026-09-28T01:03Z). Nothing past 2026-12-31 is claimed. **Closing condition:** the operator's
2027 calendar.

**Rows of other markets never touch Paris.** The "Half trading day, Wednesday before Easter"
row is Oslo-only (Paris prints `Full Trading Day` on 2025-04-16 and 2026-04-01); the Irish May
Bank Holiday is Dublin-only; Ascension Day and Whit Monday close Oslo only; Ferragosto closes
Milan only; and the 2026-01-02 and 2026-12-28 substitutes close Dublin only. None of these
ships a Paris row.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Wednesday 1 January (New Year's Day)` — Paris `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Paris cell; trade date is the same civil day |
| 2025-04-18 | closed | `Friday 18 April 2025 (Good Friday)` — Paris `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Paris cell; trade date is the same civil day |
| 2025-04-21 | closed | `Monday 21 April 2025 (Easter Monday)` — Paris `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Paris cell; trade date is the same civil day |
| 2025-05-01 | closed | `Thursday 1 May 2025 (Labour Day)` — Paris `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Paris cell; trade date is the same civil day |
| 2025-12-24 | early close | `Wednesday 24 December 2025` — Paris `**Half Trading Day`; the appendix prints the closing uncross `14:00 Random` and TAL `14:00 - 14:05` for the Paris equity segments | `EURONEXT-EOY-2025` | T1 | the printed event date plus the appendix's own Paris schedule; the envelope close is the printed TAL end, 14:05 CET |
| 2025-12-25 | closed | `Thursday 25 December 2025 (Christmas)` — Paris `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Paris cell; trade date is the same civil day |
| 2025-12-26 | closed | `Friday 26 December 2025 (St Stephens Day / Boxing Day)` — Paris `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Paris cell; trade date is the same civil day |
| 2025-12-31 | early close | `Wednesday 31 December 2025` — Paris `**Half Trading Day`; the appendix prints the same `14:00 Random` uncross and `14:00 - 14:05` TAL | `EURONEXT-EOY-2025` | T1 | the printed event date plus the appendix's own Paris schedule; the envelope close is the printed TAL end, 14:05 CET |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Thursday 1 January 2026 (New Year's Day)` — Paris `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Paris cell, printed identically on the live page; trade date is the same civil day |
| 2026-04-03 | closed | `Friday 3 April 2026 (Good Friday)` — Paris `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Paris cell, printed identically on the live page; trade date is the same civil day |
| 2026-04-06 | closed | `Monday 6 April 2026 (Easter Monday)` — Paris `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Paris cell, printed identically on the live page; trade date is the same civil day |
| 2026-05-01 | closed | `Friday 1 May 2026 (Labour Day)` — Paris `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Paris cell, printed identically on the live page; trade date is the same civil day |
| 2026-12-24 | unsourced | `Thursday 24 December 2026` — Paris `**Half Trading Day`, hours "To be announced" | `EURONEXT-IF-2026` | T1 | the half day is announced by the operator but its instants are unpublished; no status claimed for the hours |
| 2026-12-25 | closed | `Friday 25 December 2026 (Christmas)` — Paris `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Paris cell, printed identically on the live page; trade date is the same civil day |
| 2026-12-31 | unsourced | `Thursday 31 December 2026` — Paris `**Half Trading Day`, hours "To be announced" | `EURONEXT-IF-2026` | T1 | the half day is announced by the operator but its instants are unpublished; no status claimed for the hours |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EURONEXT-HH-2025-12-06` | 2025-01-01 .. 2025-12-31 (the page's 2025 table) | <https://web.archive.org/web/20251206154319id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20251206154319`, retrieved 2026-09-28 01:50 UTC | T1 | `dc96c4f7f1e6a51cd2e6743385faa5cbc4156623ec45499c89e2ec3871398a18` |
| `EURONEXT-EOY-2025` | 2025-12-24 and 2025-12-31 | <https://www.euronext.com/media/14656/download> | retrieved 2026-09-28 01:50 | T1 | `5850b4b4f5a031e637a0c44a0c7c1388ddc638c652133e66636c674af19bb6b9` |
| `EURONEXT-IF-2026` | 2026-01-01 .. 2026-12-31 | <https://connect2.euronext.com/sites/default/files/2025-11/IF251107CADE%202026%20Holiday%20Calendar%20for%20Euronexts%20Cash%20and%20Derivatives%20markets_1.pdf?VersionId=3ehe2.c9cMxv1K36.i6o4tz.5CAJmuw2> | retrieved 2026-09-28 01:09 | T1 | `617bc559510ef3a3d86d4a8b804422d0ba4c4dae43782a0a728d57a23fc4b1c5` |
| `EURONEXT-HH-LIVE-2026-09-28` | 2025-01-01 .. 2026-12-31 (the page's 2026 table; the 2025 table corroborated) | <https://live.euronext.com/en/resources/trading-hours-holidays> | retrieved 2026-09-28 01:07 | T1 | `5a1165a52361a81350fa69401910c76f046f28e16f757dadb6d33d68ccb6e5e3` |

`EURONEXT-HH-LIVE-2026-09-28` keys no 2026 row: the live page's 2026 table is cell for cell
the INFO-FLASH's Paris column, and the INFO-FLASH keys the rows. The INFO-FLASH's printed
header date reads `07 November 2026` while its publishing path and file name say 2025-11-07;
both are recorded here as printed and neither is used to date any row — the document is cited
only for its table cells. The store's `holidays/raw/equities/euronext_paris/2025-2027/`
directory holds all four artifacts with this index.

## Gaps and residual risks

- **Scope.** The profile represents the principal continuous-trading share segment, not every Euronext Paris instrument or segment.
- **Interpretive step, order-entry classification.** The operator's trading appendix describes the pre-opening as a Call phase — the French and Dutch columns render it "phase d'accumulation" / "accumulatiefase" — and its liquidity-provider clause speaks of "the order-accumulation periods preceding pre-scheduled or other Uncrossings during a Trading Day". The first uncrossing of the day is the 09:00 opening uncrossing, so no central-order-book trade can match before continuous trading starts; the pre-opening window is therefore `order_entry` and the closing uncrossing and Trading-at-Last stay `extended`.
- **Interpretive step, randomized uncrosses.** The 2015 notice's instrument-level 0–30-second micro-events do not define one exchange-wide transition instant, so this exchange-level profile retains the published nominal boundaries. The notice's defective effective year is deliberately not used as a cutover.
- **Interpretive step, the 2025 half-day envelope.** The appendix prints a phase schedule, not a single close instant: continuous trading to 13:55, a randomized 14:00 uncross and TAL to 14:05. The row encodes the envelope's final close, 14:05 CET, because the availability envelope is what the crate's answers are made of; the 14:00–14:05 TAL window stays inside the session and the randomization follows the operator's own zero-to-30-second convention already carried by the normal-week profile.
- **Follow-up, the other Euronext cash markets (dormant identities).** `euronext_amsterdam`, `euronext_brussels`, `euronext_lisbon` and `euronext_milan` route no consumer instrument and ship no holiday tables; the same retrieved calendars carry their columns. Per LAW-FOLLOW-UPS-ARE-ISSUES for dormant identities this is recorded here with its closing condition: each dormant identity's table can be keyed from the already-retrieved artifacts (and per-market end-of-year appendix sheets) when a consumer reaches it or the maintainer names the market.
- **Horizon carried below the first dated artifact.** The earliest artifact cited for the legacy grid is the operator's special-day appendix for 24 and 31 December 2010, so the ledger horizon is 2010-12-24, its own first attested day, and the January-2010 to December-2010 interval is carried rather than sourced. The source set's "January-2010-or-launch" status is not an artifact dated inside that interval and does not source it. Closing condition: a Euronext trading appendix or notice dated in or before January 2010 that prints the legacy 07:15/09:00/17:30/17:40 grid would move the horizon down to the January-2010 floor.
- **Holiday horizon.** The audited holiday window stops at 2026-12-31 because the operator has published nothing for 2027 (verified 2026-09-28). Closing condition: Euronext's 2027 holiday calendar; the row is re-checked monthly per LAW-WATCH.

> Shared module. [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs) also carries
> [`euronext_amsterdam`](euronext_amsterdam.md), [`euronext_brussels`](euronext_brussels.md),
> [`euronext_lisbon`](euronext_lisbon.md) and [`euronext_milan`](euronext_milan.md).
> `euronext_paris` is the anchor identity, so the module's narrative belongs in this file
> when LAW-EVIDENCE-FILES moves it; the module is not yet migrated.
