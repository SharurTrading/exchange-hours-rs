<!-- SPDX-License-Identifier: MIT-0 -->

# `borsa_istanbul` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bist.rs`](../../src/calendar/schedules/equities/europe/bist.rs)
- **Source sets:** [`EU-BIST`](../schedules/sources.md#eu-bist)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Equity Market; all modeled post-2010 changes have dated official evidence.

## Revision rows

- 2012-03-02 — T1 — Borsa Istanbul closing_session — closing auction added: the afternoon continuous session ends at 17:17 and a 17:17–17:30 closing envelope follows.
- 2012-07-16 — T1 — Borsa Istanbul Genelge gn2012394 — afternoon session extended: continuous to 17:30 with a 17:30–17:40 closing envelope.
- 2013-04-05 — T1 — Borsa Istanbul Genelge gn2013421 — morning opening session moves to 09:15–09:45.
- 2013-06-10 — T1 — Borsa Istanbul Genelge gn2013430 — morning opening session shortens to 09:15–09:35.
- 2015-11-30 — T1 — Borsa Istanbul announcement 13472 — midday single-price call introduced at 12:30–13:30 and the afternoon continuous session starts at 13:30.
- 2016-03-28 — T1 — Borsa Istanbul announcement 13446 — midday call moves to 13:00–14:00, morning continuous trading runs to 13:00 and the afternoon session to 17:30.
- 2016-11-14 — T1 — Borsa Istanbul announcement 13376 — extended day: order collection 09:40–09:55, opening print to 10:00, continuous 10:00–13:00 and 14:00–18:00, closing envelope 18:00–18:10.
- 2019-10-04 — T1 — Borsa Istanbul duyuru 2019/56 — the midday single-price section is removed, leaving one continuous 10:00–18:00 session.

## Holidays

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves the `borsa_istanbul` venue: Borsa İstanbul publishes one holiday arrangement per market, and the Equity Market's own `Tatil Tablosu` annex states it. Two artifacts key each year: the corporate `Resmi Tatil Günleri` page, which prints the year's holidays with the half-days marked `Yarım Gün Tatil / Saat 13:00'e kadar` (half-day holiday until 13:00), and the per-market annex `EK-3 — Borsa İstanbul A.Ş. Pay Piyasası <year> Yılı Tatil Tablosu`, which states per holiday whether a session takes place (`seans yapılmayacaktır`) or a half-day session is held (`yarım gün seans yapılacaktır`). The annex keys the closure rows; the corporate page keys the early-close rows, because the 13:00 instant those rows state is printed only there. The document ids record which: `BIST-PP-TATIL-<year>` for the annex, `BIST-RESMI-TATIL-GUNLERI` for the page.

Every listed holiday that falls on a Saturday or Sunday — Zafer Bayramı 2025-08-30 and 2026-08-30, Ramazan Bayramı Arefesi 2025-03-29 — changes no Monday-Friday trade date and ships no row; the annex prints those legs as `seans yapılmayacaktır` beside the holiday that does remove a trade date.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `1 Ocak 2025 Çarşamba seans yapılmayacaktır` (Yılbaşı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-01-01, a Wednesday; the annex prints no session for it |
| 2025-03-31 | closed | `31 Mart 2025 Pazartesi ve 1 Nisan 2025 Salı seans yapılmayacaktır` (Ramazan Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-03-31, a Monday; the annex also prints 29-30 March, a Saturday-Sunday weekend, as no-session legs |
| 2025-04-01 | closed | `31 Mart 2025 Pazartesi ve 1 Nisan 2025 Salı seans yapılmayacaktır` (Ramazan Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-04-01, a Tuesday |
| 2025-04-23 | closed | `23 Nisan 2025 Çarşamba seans yapılmayacaktır` (Ulusal Egemenlik ve Çocuk Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-04-23, a Wednesday |
| 2025-05-01 | closed | `1 Mayıs 2025 Perşembe seans yapılmayacaktır` (Emek ve Dayanışma Günü) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-05-01, a Thursday |
| 2025-05-19 | closed | `19 Mayıs 2025 Pazartesi seans yapılmayacaktır` (Atatürk'ü Anma, Gençlik ve Spor Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-05-19, a Monday |
| 2025-06-05 | early close | `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2025-06-05, a Thursday (Kurban Bayramı Arefesi); the corporate page states the half-day holiday until 13:00 and the annex states `yarım gün seans yapılacaktır`, so the final close moves to 13:00 |
| 2025-06-06 | closed | `6 Haziran 2025 Cuma … 9 Haziran 2025 Pazartesi seans yapılmayacaktır` (Kurban Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-06-06, a Friday; the printed 7-8 June legs are a weekend |
| 2025-06-09 | closed | `6 Haziran 2025 Cuma … 9 Haziran 2025 Pazartesi seans yapılmayacaktır` (Kurban Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-06-09, a Monday |
| 2025-07-15 | closed | `15 Temmuz 2025 Salı seans yapılmayacaktır` (Demokrasi ve Milli Birlik Günü) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-07-15, a Tuesday |
| 2025-10-28 | early close | `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2025-10-28, a Tuesday (Cumhuriyet Bayramı Arefesi); same two-artifact reading as 2025-06-05 |
| 2025-10-29 | closed | `29 Ekim 2025 Çarşamba seans yapılmayacaktır` (Cumhuriyet Bayramı) | `BIST-PP-TATIL-2025` | T1 | Borsa İstanbul event date 2025-10-29, a Wednesday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `1 Ocak 2026 Perşembe seans yapılmayacaktır` (Yılbaşı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-01-01, a Thursday |
| 2026-03-19 | early close | `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2026-03-19, a Thursday (Ramazan Bayramı Arefesi); the corporate page states the half-day holiday until 13:00 and the annex states `19 Mart 2026 Perşembe yarım gün seans yapılacaktır` |
| 2026-03-20 | closed | `20 Mart 2026 Cuma … seans yapılmayacaktır` (Ramazan Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-03-20, a Friday; the printed 21-22 March legs are a weekend |
| 2026-04-23 | closed | `23 Nisan 2026 Perşembe seans yapılmayacaktır` (Ulusal Egemenlik ve Çocuk Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-04-23, a Thursday |
| 2026-05-01 | closed | `1 Mayıs 2026 Cuma seans yapılmayacaktır` (Emek ve Dayanışma Günü) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-05-01, a Friday |
| 2026-05-19 | closed | `19 Mayıs 2026 Salı seans yapılmayacaktır` (Atatürk'ü Anma, Gençlik ve Spor Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-05-19, a Tuesday |
| 2026-05-26 | early close | `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2026-05-26, a Tuesday (Kurban Bayramı Arefesi); same two-artifact reading as 2026-03-19 |
| 2026-05-27 | closed | `27 Mayıs 2026 Çarşamba … seans yapılmayacaktır` (Kurban Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-05-27, a Wednesday |
| 2026-05-28 | closed | `28 Mayıs 2026 Perşembe … seans yapılmayacaktır` (Kurban Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-05-28, a Thursday |
| 2026-05-29 | closed | `29 Mayıs 2026 Cuma ve 30 Mayıs 2026 Cumartesi seans yapılmayacaktır` (Kurban Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-05-29, a Friday; the printed 30 May leg is a Saturday |
| 2026-07-15 | closed | `15 Temmuz 2026 Çarşamba seans yapılmayacaktır` (Demokrasi ve Milli Birlik Günü) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-07-15, a Wednesday |
| 2026-10-28 | early close | `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2026-10-28, a Wednesday (Cumhuriyet Bayramı Arefesi); same two-artifact reading as 2026-03-19 |
| 2026-10-29 | closed | `29 Ekim 2026 Perşembe seans yapılmayacaktır` (Cumhuriyet Bayramı) | `BIST-PP-TATIL-2026` | T1 | Borsa İstanbul event date 2026-10-29, a Thursday |

**Gaps, 2025-2026:** none inside the window. Every Equity Market trade date the operator's two 2025-2026 artifacts modify ships a row, and every other date inside the window is audited normal.

**Gaps, 2027:** the operator has published no 2027 arrangement. Verified 2026-09-28 UTC: the `Resmi Tatil Günleri` page's year list runs 2012-2026 and names no 2027 table, and no 2027 `Pay Piyasası Tatil Tablosu` annex is linked from it. Nothing is being withheld by this crate — there is no 2027 table yet. **Closing condition:** the corporate 2027 holiday page or the 2027 equity-market annex, at which point the window extends to 2027-12-31. Re-checked monthly per LAW-WATCH: the identity is holiday-bearing, so its cadence is monthly.

**Interpretive steps.** On the five half-day dates the corporate page prints the holiday as `Yarım Gün Tatil / Saat 13:00'e kadar` — a half-day holiday *until 13:00* — and the market annex states for the same day that a half-day session takes place (`yarım gün seans yapılacaktır`). The crate reads the pair as one statement: the trade date keeps its session with the final close moved to the printed 13:00, which is what `EarlyClose { 13:00 }` states. The half days' own settlement column (`takas yapılmayacaktır`) is settlement data, not a session boundary (LAW-SESSION-NOT-EXPIRY), and nothing is modelled for it. This reading is fenced per date in `tests/global_equities/holidays.rs`.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `BIST-PP-TATIL-2025` | 2025-01-01 .. 2026-12-31 | <https://www.borsaistanbul.com/files/pay-piyasasi-2025-yili-tatil-tablosu.pdf> | retrieved 2026-09-28 01:02 UTC | T1 | `55d639083395fb1374718582d5cf439a70855ad49fe321379b2c53991eeaf182` |
| `BIST-PP-TATIL-2026` | 2025-01-01 .. 2026-12-31 | <https://www.borsaistanbul.com/files/pay-piyasasi-2026-yili-tatil-tablosu.pdf> | retrieved 2026-09-28 01:02 UTC | T1 | `e45fa97e2f67d85b3571ce9e6c4758292a57379c2216fc88183f32504584e680` |
| `BIST-RESMI-TATIL-GUNLERI` | 2025-01-01 .. 2026-12-31 | <https://www.borsaistanbul.com/resmi-tatil-gunleri> | retrieved 2026-09-28 01:02 UTC | T1 | `a598c8021f1bfa4887612816a9845232eac5f8afdcbce8be278f359edd885ed2` |

All three artifacts were saved under `holidays/raw/equities/borsa_istanbul/2025-2027/` in the research store, whose `INDEX.md` repeats the URLs and digests.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.borsaistanbul.com/files/equity-market-procedure.pdf> — Borsa İstanbul Equity Market Procedure, the current rulebook. Its call-auction rules state that "[n]o transactions are executed during the order collection period", and its session table splits the 09:40–10:00 opening auction into an Order Collection Process (09:40–09:55) and Determination of Opening Price (09:55 onward).
- <https://www.borsaistanbul.com/datum/closing_session.pdf> — Borsa İstanbul closing-session document, the 2012-03-02 revision's source.
- <https://www.borsaistanbul.com/data/Genelge/gn2012394.pdf> — Borsa İstanbul Genelge gn2012394, the 2012-07-16 afternoon extension.
- <https://www.borsaistanbul.com/data/Genelge/gn2013421.pdf> — Borsa İstanbul Genelge gn2013421, the 2013-04-05 opening change.
- <https://www.borsaistanbul.com/data/Genelge/gn2013430.pdf> — Borsa İstanbul Genelge gn2013430, the 2013-06-10 opening change.
- <https://www.borsaistanbul.com/en/announcement/13472/single-session-era-borsa-istanbul> — Borsa İstanbul announcement 13472, the 2015-11-30 midday call.
- <https://www.borsaistanbul.com/en/announcement/13446/new-arrangement-borsa-istanbul-equity-market-midday-session> — Borsa İstanbul announcement 13446, the 2016-03-28 midday-session rearrangement.
- <https://www.borsaistanbul.com/en/announcement/13376/borsa-istanbul-trading-session-hours-change> — Borsa İstanbul announcement 13376, the 2016-11-14 extended day. It states that "the trading session shall start at 09:40 with order collection" and that "[f]ollowing the end of the order collection phase at 09:55, continuous auction shall start at 10:00".
- <https://www.borsaistanbul.com/duyuru/11640/pay-piyasasi-seansinda-gun-ortasi-tek-fiyat-bolumu-hk-201956-sayili-duyuru> — Borsa İstanbul duyuru 2019/56, removing the midday single-price section on 2019-10-04.
- <https://www.borsaistanbul.com/resmi-tatil-gunleri> — Borsa İstanbul `Resmi Tatil Günleri`, the corporate official-holiday page and the holiday watch entry point; its year tables 2012-2026 print the half-day instants.
- <https://www.borsaistanbul.com/files/pay-piyasasi-2025-yili-tatil-tablosu.pdf> — `EK-3`, Pay Piyasası 2025 Yılı Tatil Tablosu.
- <https://www.borsaistanbul.com/files/pay-piyasasi-2026-yili-tatil-tablosu.pdf> — `EK-3`, Pay Piyasası 2026 Yılı Tatil Tablosu.

## Gaps and residual risks

- **Horizon carried below the first dated row.** The baseline profile below 2012-03-02 — morning opening call 09:30–09:50 and continuous 09:50–12:30, afternoon call 14:00–14:20 and continuous 14:20–17:30 — cites no artifact of its own. Every source indexed here states a change and its replacement table; none is identified as the document that attests the pre-2012 grid, and no retrieval date is recorded for the closing-session document, so its own publication day cannot be read off the citation. The ledger horizon is therefore 2012-03-02, the first day at which this row's state is sourced, with everything below it carried. Closing condition: retrieve a Borsa İstanbul circular, procedure edition or announcement dated at or before the January-2010 floor that prints the pre-2012 grid, or record the closing-session document's publication date so the baseline can be carried from it; either would move the horizon earlier.
- **Interpretive step, order-entry classification.** From 2016-11-14 only the 09:40–09:55 Order Collection Process is `order_entry`; 09:55–10:00 carries the opening print and stays `extended`. The midday single-price call and the 18:00–18:10 closing envelope each bundle collection with a price-determination leg that prints, so both stay `extended` whole. Earlier eras carry no `order_entry` window because no source separates their collection legs.
- **Source set has no monitoring feed.** `EU-BIST` records that no stable consolidated announcements-feed URL is indexed; review means reopening the Equity Market Procedure, the individual circulars listed above, and the `Resmi Tatil Günleri` holiday page.
- **Service tier.** The consumer serves this venue live (it is one of the market-clock overview's venues), so the identity is **served** and the holiday-bearing calendar is reviewed monthly per LAW-WATCH; the 2027 closing condition above is the one tracked gap.
