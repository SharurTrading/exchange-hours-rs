<!-- SPDX-License-Identifier: MIT-0 -->

# `lse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`lse.rs`](../../src/calendar/schedules/equities/europe/lse.rs)
- **Source sets:** [`EU-LSE`](../schedules/sources.md#eu-lse), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SETS January-2010 baseline, the 2012-04-30 CPX launch, and the 2016-03-21 randomized midday auction are primary-sourced with conservative latest uncross edges.

## Revision rows

- 2012-04-30 — T1 — LSE MIT201 document history — Closing Price Crossing session added, closing envelope extended to 16:40.
- 2016-03-21 — T1 — LSE notice N01/16 — SETS intraday auction at 12:00 with its two-minute run and 30-second random end.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://docs.londonstockexchange.com/sites/default/files/documents/compliance_update_mar_09.pdf> — LSE compliance update, March 2009: the 2009 compliance parameters behind the January-2010 SETS baseline.
- <https://docs.londonstockexchange.com/sites/default/files/documents/live-001-300910-appendix-a.pdf> — LSE Millennium Exchange rehearsal timetable, September 2010, appendix A: pre-trading 07:00, opening call 07:50, randomized 08:00 uncross, closing call 16:30 to its latest 16:35:30 edge.
- <https://docs.londonstockexchange.com/sites/default/files/documents/mit201-guide-to-the-trading-system-15-6-20240429.pdf> — LSE MIT201, Guide to the Trading System: section 4.4 lists pre-trading as a scheduled session preceding the opening auction call; section 4.5 calls CPX "a short, modified regular trading session"; its operator-maintained document history records the 2012-04-30 production functional release.
- <https://docs.londonstockexchange.com/sites/default/files/documents/n1512_attach1.pdf> — LSE notice N15/12, official attachment: the closing-auction uncross starts at 16:35 and CPX is the up-to-five-minute executable session immediately following it.
- <https://docs.londonstockexchange.com/sites/default/files/documents/mit501.pdf> — LSE MIT501: CPX introduced in April 2012 with a default five-minute duration.
- <https://docs.londonstockexchange.com/sites/default/files/documents/servicetechnicaldescriptionintroductionofnewtradingcurrencies.pdf> — LSE service technical description: CPX scheduled grid 16:35:01–16:40:00.
- <https://docs.londonstockexchange.com/sites/default/files/documents/n0116.pdf> — LSE notice N01/16: SETS intraday auction effective 2016-03-21, starting at 12:00, running two minutes, with a random end of up to 30 seconds.
- <https://www.londonstockexchange.com/resources/equities-trading-resources?tab=technical-library> — LSE equities technical library: current technical parameters preserving the intraday-auction grid and CPX to 16:40.
- <https://www.londonstockexchange.com/equities-trading/asset-classes/shares-trading/sets> — LSE SETS product page, the source set's current entry point.
- <https://docs.londonstockexchange.com/sites/default/files/documents/international-order-book-introduction-sheet.pdf> — LSE SETS-aligned trading-day timetable.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only, never an effective date.

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive trade dates in `Europe/London`; tier T1 throughout).

LSE's holiday statement is the operator's own "Bank holidays and their impact on our trading
services" table on its `Business days` page, delivered by the content API behind
`londonstockexchange.com` (`api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days`).
The table is **rolling**: it lists only business days from the present forward to the next New
Year, so the audited years are pinned by three states of the one table — the 2024-02-07 capture
(which reaches 2025-01-01), the 2025-12-18 capture (2025-12-24 through 2027-12-31) and the live
table retrieved 2026-09-28 (2026-08-31 onward). The 2026-06-17 capture corroborates 2026-05-04
and 2026-05-25, and the live table corroborates the 2025-12-18 capture cell for cell on every
row they share. LSE runs no overnight session, so an event date and its trade date are one
civil day and the conversion is the identity.

**Five 2025 dates are not audited.** The rolling table had already moved past 2025-04-18,
2025-04-21, 2025-05-05, 2025-05-26 and 2025-08-25 when the first 2025-era capture was taken
(the 2025-12-18 table starts at 2025-12-24), the Wayback CDX index holds exactly four captures
of the API URL (2024-02-07, 2024-03-08, 2025-12-18, 2026-06-17 — queried 2026-09-28 ~01:25 UTC),
and archive.today holds no snapshot, so no surviving T1 or T2 artifact states those five dates.
T3 restatements exist on the open web (mrtopstep.com, ebc.com, apricotcapital.am) and cannot
key rows (LAW-PRIMARY-SOURCES). The five dates ship as `Unsourced` rather than as silence,
because a date inside the window without a row would claim "audited normal", which nothing
here can support. **Closing condition:** a capture of the business-days API dated inside the
2025-01-02..2025-12-17 gap, or the operator's re-publication of a 2025 table.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year Day` — `Wednesday 1 January 2025` — `NON-trading day.` | `LSE-BUSDAYS-2024-02-07` | T1 | the operator's own event date; no overnight session, so the trade date is the same civil day |
| 2025-04-18 | unsourced | — (the rolling table had rolled past the date before any surviving capture) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-04-21 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-05-05 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-05-26 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-08-25 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 24 December 2025; the day's final close is the printed 12:30 London time |
| 2025-12-25 | closed | `Christmas Day` — `Thursday 25 December 2025` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2025-12-26 | closed | `Boxing Day` — `Friday 26 December 2025` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2025-12-31 | early close | `New Year's Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 31 December 2025; the day's final close is the printed 12:30 London time |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day` — `Thursday 1 January 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2026-04-03 | closed | `Good Friday` — `Friday 3 April 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2026-04-06 | closed | `Easter Monday` — `Monday 6 April 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2026-05-04 | closed | `Early May Bank Holiday` — `Monday 4 May 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the 2026-06-17 capture |
| 2026-05-25 | closed | `Spring Bank Holiday` — `Monday 25 May 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the 2026-06-17 capture |
| 2026-08-31 | closed | `Summer Bank Holiday` — `Monday 31 August 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2026-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 24 December 2026; the day's final close is the printed 12:30 London time |
| 2026-12-25 | closed | `Christmas Day` — `Friday 25 December 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2026-12-28 | closed | `Boxing Day (substitute)` — `Monday 28 December 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own substitute-day print; trade date is the same civil day |
| 2026-12-31 | early close | `New Year's Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 31 December 2026; the day's final close is the printed 12:30 London time |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day` — `Friday 1 January 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-03-26 | closed | `Good Friday` — `Friday 26 March 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-03-29 | closed | `Easter Monday` — `Monday 29 March 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-05-03 | closed | `Early May Bank Holiday` — `Monday 3 May 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-05-31 | closed | `Spring Bank Holiday` — `Monday 31 May 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-08-30 | closed | `Summer Bank Holiday` — `Monday 30 August 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 24 December 2027; the day's final close is the printed 12:30 London time |
| 2027-12-27 | closed | `Christmas Day (substitute)` — `Monday 27 December 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own substitute-day print; corroborated by the live table |
| 2027-12-28 | closed | `Boxing Day (substitute)` — `Tuesday 28 December 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own substitute-day print; corroborated by the live table |
| 2027-12-31 | early close | `New Year's Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 31 December 2027; the day's final close is the printed 12:30 London time |

**Interpretive step — the half-day scalar.** The sheet states "Markets closing process
commences from 12:30 London time." for each half day. On a full day the same printed boundary
kind — the closing-process start — sits at 16:30, so the 12:30 statement is the day's final
close of the availability envelope and the closing-auction phase that would follow it is
removed rather than modelled; the row is an `EarlyClose` at 45 000 seconds. No late opens and
no other half days are printed in any audited year.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `LSE-BUSDAYS-2024-02-07` | 2024-03-29 .. 2025-01-01 | <https://web.archive.org/web/20240207115235id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20240207115235`, retrieved 2026-09-28 01:28 UTC | T1 | `4730c509be75be779d106ccad036eae4f515bc6524613951126b33491b40a90b` |
| `LSE-BUSDAYS-2024-03-08` | 2024-03-29 .. 2025-01-01 | <https://web.archive.org/web/20240308200156id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20240308200156`, retrieved 2026-09-28 01:36 UTC | T1 | `dbcdccd7ef5bae333856d86819ad9f1908df3b5bab4fa6354e891d3ce99300db` |
| `LSE-BUSDAYS-2025-12-18` | 2025-12-24 .. 2027-12-31 | <https://web.archive.org/web/20251218172240id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20251218172240`, retrieved 2026-09-28 01:30 UTC | T1 | `e7d81b189bd75137b2e51136b2d6979d780dd3d50b95c111ef5acbdd38d49a11` |
| `LSE-BUSDAYS-2026-06-17` | 2026-05-04 .. 2029-01-01 | <https://web.archive.org/web/20260617134453id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20260617134453`, retrieved 2026-09-28 01:30 UTC | T1 | `1759a375014f079d7725ac0c08293073d9d27c8d9a28e1999d7f611d7145270a` |
| `LSE-BUSDAYS-LIVE-2026-09-28` | 2026-08-31 .. 2029-01-01 | <https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days> | retrieved 2026-09-28 01:16 | T1 | `4f9f6f3aef9ab02874112c05c0c07ac4529e6c38e7ad74d0aa41735853a87a7f` |

`LSE-BUSDAYS-2024-03-08` keys no row: it confirms the February capture's 2025-01-01 endpoint
and that no further 2025 rows were listed at that date. `LSE-BUSDAYS-2026-06-17` and
`LSE-BUSDAYS-LIVE-2026-09-28` key no row either: they corroborate the 2025-12-18 capture's
rows on every date they share, which is what lets one document id carry 2026 and 2027. The
store's `holidays/raw/equities/lse/2025-2027/` holds all five artifacts with this index.

## Gaps and residual risks

- **Interpretive step, order-entry classification.** Pre-trading 07:00–07:50 is modelled `order_entry` on MIT201 section 4.4, which lists it as a scheduled trading session distinct from the executable phases of the order-book day. No on-book execution can occur before the opening auction uncrosses. Closing condition: none needed; a later MIT201 edition that reclassifies the phase would move it.
- **Interpretive step, randomized uncrosses.** The opening uncross, the intraday auction uncross and the closing uncross are each randomized. The deterministic profile holds the auction classification through the latest possible edge (08:00:30, 12:02:30, 16:35:30), so the calendar never reports continuous trading while an auction can still run. This is the crate's conservative-envelope convention, not an operator statement about any individual security.
- **Holiday coverage, five 2025 dates.** 2025-04-18, 2025-04-21, 2025-05-05, 2025-05-26 and 2025-08-25 are inside the audited window with `Unsourced` rows: the operator's rolling business-days table had moved past them before any surviving capture, and no archived or live artifact states them (checked 2026-09-28). The dates are Good Friday, Easter Monday and the three bank-holiday Mondays, so the crate claims nothing about them rather than an unaudited closure. Closing condition: a capture of the business-days API dated inside the 2025-01-02..2025-12-17 gap.
- **Holiday horizon below 2025-01-01.** The audited holiday window is 2025-01-01..2027-12-31; dates before 2025-01-01 ship no holiday rows and the table answers nothing there. The operator's own table is rolling and cannot be rolled back; each earlier year needs an archived capture of the same API. Closing condition: per-year captures retrieved and audited, one wave per year set.
