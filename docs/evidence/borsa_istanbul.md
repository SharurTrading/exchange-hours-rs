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

## Normal week

**The pre-2012 grid is the operator's own İşlem Saatleri page.** IMKB's
`Markets/StockMarket/TransactionHours.aspx` page as served 2010-03-25
(`IMKB-TH-2010-03-25`) prints, for Hisse Senetleri Piyasası (Ulusal Pazar,
Kurumsal Ürünler Pazarı, İkinci Ulusal Pazar, Yeni Ekonomi Pazarı ve Fon
Pazarı): 1. Seans 09:30-12:30 — Açılış Seansı 09:30-09:50, (i) Emir Toplama
09:30-09:45, (ii) Açılış Fiyatının Belirlenmesi ve Açılış İşlemleri 09:45-09:50,
Sürekli Müzayede Seansı 09:50-12:30 — and 2. Seans 14:00-17:30 — Açılış Seansı
14:00-14:20, (i) Emir Toplama, (ii) Açılış Fiyatının Belirlenmesi ve Açılış
İşlemleri, Sürekli Müzayede Seansı 14:20-17:30. That is exactly the baseline
profile the module encodes: opening calls 09:30-09:50 and 14:00-14:20,
continuous 09:50-12:30 and 14:20-17:30. The 2011-07-27 capture
(`IMKB-TH-2011-07-27`) prints the same session bounds with the auction methods
spelled out (Piyasa Yapıcılı SM, Tek Fiyat Yöntemi), so the grid is attested at
both ends of the carried era, and the 2012-03-02 revision row keys the era's
one dated change (the closing auction). The ledger horizon is 2010-03-25, the
first capture day; below it, 2010-01-01..2010-03-24 is carried.

**What the page states and what the module leaves whole.** The page's own
sub-phases split each call: Emir Toplama (order collection) then Açılış
Fiyatının Belirlenmesi ve Açılış İşlemleri (price determination and opening
trades), so the call prints its auction at its end. The baseline profile keeps
each whole call 09:30-09:50 / 14:00-14:20 in `extended` — the pre-existing
classification that carries no order-entry window for the eras whose sources do
not separate the collection leg (the interpretive step below) — and the
sourcing move changes the grid's attestation, not its phase split.

## Holidays

**Coverage:** 2012-03-02..2026-12-31 (inclusive trade dates; the window opens at the operator's own first sourced normal-week day). Tier: T1 throughout.

One table serves the `borsa_istanbul` venue: Borsa İstanbul publishes one holiday arrangement per market, and the Equity Market's own `Tatil Tablosu` annex states it. Two artifacts key each year: the corporate `Resmi Tatil Günleri` page, which prints the year's holidays with the half-days marked `Yarım Gün Tatil / Saat 13:00'e kadar` (half-day holiday until 13:00), and the per-market annex `EK-3 — Borsa İstanbul A.Ş. Pay Piyasası <year> Yılı Tatil Tablosu`, which states per holiday whether a session takes place (`seans yapılmayacaktır`) or a half-day session is held (`yarım gün seans yapılacaktır`). The annex keys the closure rows; the corporate page keys the early-close rows, because the 13:00 instant those rows state is printed only there. The document ids record which: `BIST-PP-TATIL-<year>` for the annex, `BIST-RESMI-TATIL-GUNLERI` for the page.

The 2012-2024 rows were backfilled on 2026-09-29 (UTC) from the same corporate page the early-close rows already keyed: its server-rendered year tabs print every year 2012-2026 in one artifact, so no new retrieval was needed. In the backfilled years the corporate page alone states the arrangement — `Kapalı` for a closure, `Yarım Gün Tatil / Saat 13:00'e kadar` for a half day — so every 2012-2024 row keys to `BIST-RESMI-TATIL-GUNLERI`; the 2025-2026 closure rows keep their annex citations.

Every listed holiday that falls on a Saturday or Sunday — Zafer Bayramı 2025-08-30 and 2026-08-30, Ramazan Bayramı Arefesi 2025-03-29, and the weekend legs of the backfilled years (for example 1 Ocak 2016, 23 Nisan 2016, 1 Mayıs 2016 and 2017's 1 Ocak, 23 Nisan, 15 Temmuz, 28 Ekim and 29 Ekim) — changes no Monday-Friday trade date and ships no row; the page prints those legs as `Kapalı` beside the holiday that does remove a trade date, and the annex prints them as `seans yapılmayacaktır`.


### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-23 | closed | `23 Nisan 2012, Pazartesi` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-04-23, a Monday |
| 2012-05-01 | closed | `1 Mayıs 2012, Salı` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-05-01, a Tuesday |
| 2012-08-20 | closed | `20 Ağustos 2012, Pazartesi` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-08-20, a Monday |
| 2012-08-21 | closed | `21 Ağustos 2012, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-08-21, a Tuesday |
| 2012-08-30 | closed | `30 Ağustos 2012, Perşembe` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-08-30, a Thursday |
| 2012-10-24 | early close | `24 Ekim 2012, Çarşamba` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-10-24, a Wednesday |
| 2012-10-25 | closed | `25 Ekim 2012, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-10-25, a Thursday |
| 2012-10-26 | closed | `26 Ekim 2012, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-10-26, a Friday |
| 2012-10-29 | closed | `29 Ekim 2012, Pazartesi` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2012-10-29, a Monday |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `1 Ocak 2013, Salı` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-01-01, a Tuesday |
| 2013-04-23 | closed | `23 Nisan 2013, Salı` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-04-23, a Tuesday |
| 2013-05-01 | closed | `1 Mayıs 2013, Çarşamba` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-05-01, a Wednesday |
| 2013-08-07 | early close | `7 Ağustos 2013, Çarşamba` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-08-07, a Wednesday |
| 2013-08-08 | closed | `8 Ağustos 2013, Perşembe` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-08-08, a Thursday |
| 2013-08-09 | closed | `9 Ağustos 2013, Cuma` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-08-09, a Friday |
| 2013-08-30 | closed | `30 Ağustos 2013, Cuma` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-08-30, a Friday |
| 2013-10-14 | early close | `14 Ekim 2013, Pazartesi` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-14, a Monday |
| 2013-10-15 | closed | `15 Ekim 2013, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-15, a Tuesday |
| 2013-10-16 | closed | `16 Ekim 2013, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-16, a Wednesday |
| 2013-10-17 | closed | `17 Ekim 2013, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-17, a Thursday |
| 2013-10-18 | closed | `18 Ekim 2013, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-18, a Friday |
| 2013-10-28 | early close | `28 Ekim 2013, Pazartesi` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-28, a Monday |
| 2013-10-29 | closed | `29 Ekim 2013, Salı` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2013-10-29, a Tuesday |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `1 Ocak 2014, Çarşamba` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-01-01, a Wednesday |
| 2014-04-23 | closed | `23 Nisan 2014, Çarşamba` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-04-23, a Wednesday |
| 2014-05-01 | closed | `1 Mayıs 2014, Perşembe` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-05-01, a Thursday |
| 2014-05-19 | closed | `19 Mayıs 2014, Pazartesi` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-05-19, a Monday |
| 2014-07-28 | closed | `28 Temmuz 2014, Pazartesi` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-07-28, a Monday |
| 2014-07-29 | closed | `29 Temmuz 2014, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-07-29, a Tuesday |
| 2014-07-30 | closed | `30 Temmuz 2014, Çarşamba` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-07-30, a Wednesday |
| 2014-10-03 | early close | `3 Ekim 2014, Cuma` — Kurban Bayramı Arifesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-10-03, a Friday |
| 2014-10-06 | closed | `6 Ekim 2014, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-10-06, a Monday |
| 2014-10-07 | closed | `7 Ekim 2014, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-10-07, a Tuesday |
| 2014-10-28 | early close | `28 Ekim 2014, Salı` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-10-28, a Tuesday |
| 2014-10-29 | closed | `29 Ekim 2014, Çarşamba` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2014-10-29, a Wednesday |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `1 Ocak 2015, Perşembe` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-01-01, a Thursday |
| 2015-04-23 | closed | `23 Nisan 2015, Perşembe` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-04-23, a Thursday |
| 2015-05-01 | closed | `1 Mayıs 2015, Cuma` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-05-01, a Friday |
| 2015-05-19 | closed | `19 Mayıs 2015, Salı` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-05-19, a Tuesday |
| 2015-07-16 | early close | `16 Temmuz 2015, Perşembe` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-07-16, a Thursday |
| 2015-07-17 | closed | `17 Temmuz 2015, Cuma` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-07-17, a Friday |
| 2015-09-23 | early close | `23 Eylül 2015, Çarşamba` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-09-23, a Wednesday |
| 2015-09-24 | closed | `24 Eylül 2015, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-09-24, a Thursday |
| 2015-09-25 | closed | `25 Eylül 2015, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-09-25, a Friday |
| 2015-10-28 | early close | `28 Ekim 2015, Çarşamba` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-10-28, a Wednesday |
| 2015-10-29 | closed | `29 Ekim 2015, Perşembe` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2015-10-29, a Thursday |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `1 Ocak 2016, Cuma` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-01-01, a Friday |
| 2016-05-19 | closed | `19 Mayıs 2016, Perşembe` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-05-19, a Thursday |
| 2016-07-04 | early close | `4 Temmuz 2016, Pazartesi` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-07-04, a Monday |
| 2016-07-05 | closed | `5 Temmuz 2016, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-07-05, a Tuesday |
| 2016-07-06 | closed | `6 Temmuz 2016, Çarşamba` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-07-06, a Wednesday |
| 2016-07-07 | closed | `7 Temmuz 2016, Perşembe` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-07-07, a Thursday |
| 2016-08-30 | closed | `30 Ağustos 2016, Salı` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-08-30, a Tuesday |
| 2016-09-12 | closed | `12 Eylül 2016, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-09-12, a Monday |
| 2016-09-13 | closed | `13 Eylül 2016, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-09-13, a Tuesday |
| 2016-09-14 | closed | `14 Eylül 2016, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-09-14, a Wednesday |
| 2016-09-15 | closed | `15 Eylül 2016, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-09-15, a Thursday |
| 2016-10-28 | early close | `28 Ekim 2016, Cuma` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2016-10-28, a Friday |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-05-01 | closed | `1 Mayıs 2017, Pazartesi` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-05-01, a Monday |
| 2017-05-19 | closed | `19 Mayıs 2017, Cuma` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-05-19, a Friday |
| 2017-06-26 | closed | `26 Haziran 2017, Pazartesi` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-06-26, a Monday |
| 2017-06-27 | closed | `27 Haziran 2017, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-06-27, a Tuesday |
| 2017-08-30 | closed | `30 Ağustos 2017, Çarşamba` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-08-30, a Wednesday |
| 2017-08-31 | early close | `31 Ağustos 2017, Perşembe` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-08-31, a Thursday |
| 2017-09-01 | closed | `1 Eylül 2017, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-09-01, a Friday |
| 2017-09-04 | closed | `4 Eylül 2017, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2017-09-04, a Monday |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `1 Ocak 2018, Pazartesi` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-01-01, a Monday |
| 2018-04-23 | closed | `23 Nisan 2018, Pazartesi` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-04-23, a Monday |
| 2018-05-01 | closed | `1 Mayıs 2018, Salı` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-05-01, a Tuesday |
| 2018-06-14 | early close | `14 Haziran 2018, Perşembe` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-06-14, a Thursday |
| 2018-06-15 | closed | `15 Haziran 2018, Cuma` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-06-15, a Friday |
| 2018-08-20 | early close | `20 Ağustos 2018, Pazartesi` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-08-20, a Monday |
| 2018-08-21 | closed | `21 Ağustos 2018, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-08-21, a Tuesday |
| 2018-08-22 | closed | `22 Ağustos 2018, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-08-22, a Wednesday |
| 2018-08-23 | closed | `23 Ağustos 2018, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-08-23, a Thursday |
| 2018-08-24 | closed | `24 Ağustos 2018, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-08-24, a Friday |
| 2018-08-30 | closed | `30 Ağustos 2018, Perşembe` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-08-30, a Thursday |
| 2018-10-29 | closed | `29 Ekim 2018, Pazartesi` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2018-10-29, a Monday |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `1 Ocak 2019, Salı` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-01-01, a Tuesday |
| 2019-04-23 | closed | `23 Nisan 2019, Salı` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-04-23, a Tuesday |
| 2019-05-01 | closed | `1 Mayıs 2019, Çarşamba` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-05-01, a Wednesday |
| 2019-06-03 | early close | `3 Haziran 2019, Pazartesi` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-06-03, a Monday |
| 2019-06-04 | closed | `4 Haziran 2019, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-06-04, a Tuesday |
| 2019-06-05 | closed | `5 Haziran 2019, Çarşamba` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-06-05, a Wednesday |
| 2019-06-06 | closed | `6 Haziran 2019, Perşembe` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-06-06, a Thursday |
| 2019-07-15 | closed | `15 Temmuz 2019, Pazartesi` — Demokrasi ve Milli Birlik Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-07-15, a Monday |
| 2019-08-12 | closed | `12 Ağustos 2019, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-08-12, a Monday |
| 2019-08-13 | closed | `13 Ağustos 2019, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-08-13, a Tuesday |
| 2019-08-14 | closed | `14 Ağustos 2019, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-08-14, a Wednesday |
| 2019-08-30 | closed | `30 Ağustos 2019, Cuma` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-08-30, a Friday |
| 2019-10-28 | early close | `28 Ekim 2019, Pazartesi` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-10-28, a Monday |
| 2019-10-29 | closed | `29 Ekim 2019, Salı` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2019-10-29, a Tuesday |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `1 Ocak 2020, Çarşamba` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-01-01, a Wednesday |
| 2020-04-23 | closed | `23 Nisan 2020, Perşembe` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-04-23, a Thursday |
| 2020-05-01 | closed | `1 Mayıs 2020, Cuma` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-05-01, a Friday |
| 2020-05-19 | closed | `19 Mayıs 2020, Salı` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-05-19, a Tuesday |
| 2020-05-25 | closed | `25 Mayıs 2020, Pazartesi` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-05-25, a Monday |
| 2020-05-26 | closed | `26 Mayıs 2020, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-05-26, a Tuesday |
| 2020-07-15 | closed | `15 Temmuz 2020, Çarşamba` — Demokrasi ve Milli Birlik Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-07-15, a Wednesday |
| 2020-07-30 | early close | `30 Temmuz 2020, Perşembe` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-07-30, a Thursday |
| 2020-07-31 | closed | `31 Temmuz 2020, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-07-31, a Friday |
| 2020-08-03 | closed | `3 Ağustos 2020, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-08-03, a Monday |
| 2020-10-28 | early close | `28 Ekim 2020, Çarşamba` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-10-28, a Wednesday |
| 2020-10-29 | closed | `29 Ekim 2020, Perşembe` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2020-10-29, a Thursday |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `1 Ocak 2021, Cuma` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-01-01, a Friday |
| 2021-04-23 | closed | `23 Nisan 2021, Cuma` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-04-23, a Friday |
| 2021-05-12 | early close | `12 Mayıs 2021, Çarşamba` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-05-12, a Wednesday |
| 2021-05-13 | closed | `13 Mayıs 2021, Perşembe` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-05-13, a Thursday |
| 2021-05-14 | closed | `14 Mayıs 2021, Cuma` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-05-14, a Friday |
| 2021-05-19 | closed | `19 Mayıs 2021, Çarşamba` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-05-19, a Wednesday |
| 2021-07-15 | closed | `15 Temmuz 2021, Perşembe` — Demokrasi ve Milli Birlik Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-07-15, a Thursday |
| 2021-07-19 | early close | `19 Temmuz 2021, Pazartesi` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-07-19, a Monday |
| 2021-07-20 | closed | `20 Temmuz 2021, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-07-20, a Tuesday |
| 2021-07-21 | closed | `21 Temmuz 2021, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-07-21, a Wednesday |
| 2021-07-22 | closed | `22 Temmuz 2021, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-07-22, a Thursday |
| 2021-07-23 | closed | `23 Temmuz 2021, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-07-23, a Friday |
| 2021-08-30 | closed | `30 Ağustos 2021, Pazartesi` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-08-30, a Monday |
| 2021-10-28 | early close | `28 Ekim 2021, Perşembe` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-10-28, a Thursday |
| 2021-10-29 | closed | `29 Ekim 2021, Cuma` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2021-10-29, a Friday |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-05-02 | closed | `2 Mayıs 2022, Pazartesi` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-05-02, a Monday |
| 2022-05-03 | closed | `3 Mayıs 2022, Salı` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-05-03, a Tuesday |
| 2022-05-04 | closed | `4 Mayıs 2022, Çarşamba` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-05-04, a Wednesday |
| 2022-05-19 | closed | `19 Mayıs 2022, Perşembe` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-05-19, a Thursday |
| 2022-07-08 | early close | `8 Temmuz 2022, Cuma` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-07-08, a Friday |
| 2022-07-11 | closed | `11 Temmuz 2022, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-07-11, a Monday |
| 2022-07-12 | closed | `12 Temmuz 2022, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-07-12, a Tuesday |
| 2022-07-15 | closed | `15 Temmuz 2022, Cuma` — Demokrasi ve Milli Birlik Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-07-15, a Friday |
| 2022-08-30 | closed | `30 Ağustos 2022, Salı` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-08-30, a Tuesday |
| 2022-10-28 | early close | `28 Ekim 2022, Cuma` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2022-10-28, a Friday |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-20 | early close | `20 Nisan 2023, Perşembe` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-04-20, a Thursday |
| 2023-04-21 | closed | `21 Nisan 2023, Cuma` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-04-21, a Friday |
| 2023-05-01 | closed | `1 Mayıs 2023, Pazartesi` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-05-01, a Monday |
| 2023-05-19 | closed | `19 Mayıs 2023, Cuma` — Atatürk'ü Anma, Gençlik ve Spor Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-05-19, a Friday |
| 2023-06-27 | early close | `27 Haziran 2023, Salı` — Kurban Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-06-27, a Tuesday |
| 2023-06-28 | closed | `28 Haziran 2023, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-06-28, a Wednesday |
| 2023-06-29 | closed | `29 Haziran 2023, Perşembe` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-06-29, a Thursday |
| 2023-06-30 | closed | `30 Haziran 2023, Cuma` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-06-30, a Friday |
| 2023-08-30 | closed | `30 Ağustos 2023, Çarşamba` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2023-08-30, a Wednesday |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `1 Ocak 2024, Pazartesi` — Yeni Yıl Tatili — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-01-01, a Monday |
| 2024-04-09 | early close | `9 Nisan 2024, Salı` — Ramazan Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-04-09, a Tuesday |
| 2024-04-10 | closed | `10 Nisan 2024, Çarşamba` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-04-10, a Wednesday |
| 2024-04-11 | closed | `11 Nisan 2024, Perşembe` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-04-11, a Thursday |
| 2024-04-12 | closed | `12 Nisan 2024, Cuma` — Ramazan Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-04-12, a Friday |
| 2024-04-23 | closed | `23 Nisan 2024, Salı` — Ulusal Egemenlik ve Çocuk Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-04-23, a Tuesday |
| 2024-05-01 | closed | `1 Mayıs 2024, Çarşamba` — Emek ve Dayanışma Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-05-01, a Wednesday |
| 2024-06-17 | closed | `17 Haziran 2024, Pazartesi` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-06-17, a Monday |
| 2024-06-18 | closed | `18 Haziran 2024, Salı` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-06-18, a Tuesday |
| 2024-06-19 | closed | `19 Haziran 2024, Çarşamba` — Kurban Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-06-19, a Wednesday |
| 2024-07-15 | closed | `15 Temmuz 2024, Pazartesi` — Demokrasi ve Milli Birlik Günü — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-07-15, a Monday |
| 2024-08-30 | closed | `30 Ağustos 2024, Cuma` — Zafer Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-08-30, a Friday |
| 2024-10-28 | early close | `28 Ekim 2024, Pazartesi` — Cumhuriyet Bayramı Arefesi (Yarım Gün Tatil) — `Yarım Gün Tatil / Saat 13:00'e kadar` — 13:00 | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-10-28, a Monday |
| 2024-10-29 | closed | `29 Ekim 2024, Salı` — Cumhuriyet Bayramı — `Kapalı` | `BIST-RESMI-TATIL-GUNLERI` | T1 | Borsa İstanbul event date 2024-10-29, a Tuesday |

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

**Gaps, 2012-2024:** none inside the window. The corporate page's year tabs print each year's complete table, every Monday-Friday date its rows modify ships a row, and every other date inside the window is audited normal.

**Gaps, 2027:** the operator has published no 2027 arrangement. Verified 2026-09-28 UTC: the `Resmi Tatil Günleri` page's year list runs 2012-2026 and names no 2027 table, and no 2027 `Pay Piyasası Tatil Tablosu` annex is linked from it. Re-checked 2026-09-29 UTC with a fresh live read of the same page (artifact under `holidays/raw/equities/borsa_istanbul/forward-2027/`): its bytes carry zero `2027` mentions. Nothing is being withheld by this crate — there is no 2027 table yet. **Closing condition:** the corporate 2027 holiday page or the 2027 equity-market annex, at which point the window extends to 2027-12-31. Re-checked monthly per LAW-WATCH: the identity is holiday-bearing, so its cadence is monthly.

**Interpretive steps.** On the half-day dates the corporate page prints the holiday as `Yarım Gün Tatil / Saat 13:00'e kadar` — a half-day holiday *until 13:00* — and the 2025-2026 market annex states for the same day that a half-day session takes place (`yarım gün seans yapılacaktır`). The crate reads the pair as one statement: the trade date keeps its session with the final close moved to the printed 13:00, which is what `EarlyClose { 13:00 }` states. Two eras sit under that scalar. From the 2015-11-30 midday call onward the 13:00 bound falls inside a live session, so the day trades to one second before 13:00. On the 2012-2015 half days the 13:00 bound sits inside the era's lunch gap: the executable morning ends at its own 12:30 and the clip deletes the 14:00 afternoon, so the day's last trade is 12:29:59.999... and the candle ends at 12:30. The half days' own settlement column (`takas yapılmayacaktır`) is settlement data, not a session boundary (LAW-SESSION-NOT-EXPIRY), and nothing is modelled for it. This reading is fenced per date in `tests/global_equities/holidays.rs`.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `BIST-PP-TATIL-2025` | 2025-01-01 .. 2026-12-31 | <https://www.borsaistanbul.com/files/pay-piyasasi-2025-yili-tatil-tablosu.pdf> | retrieved 2026-09-28 01:02 UTC | T1 | `55d639083395fb1374718582d5cf439a70855ad49fe321379b2c53991eeaf182` |
| `BIST-PP-TATIL-2026` | 2025-01-01 .. 2026-12-31 | <https://www.borsaistanbul.com/files/pay-piyasasi-2026-yili-tatil-tablosu.pdf> | retrieved 2026-09-28 01:02 UTC | T1 | `e45fa97e2f67d85b3571ce9e6c4758292a57379c2216fc88183f32504584e680` |
| `BIST-RESMI-TATIL-GUNLERI` | 2012-03-02 .. 2026-12-31 | <https://www.borsaistanbul.com/resmi-tatil-gunleri> | retrieved 2026-09-28 01:02 UTC | T1 | `a598c8021f1bfa4887612816a9845232eac5f8afdcbce8be278f359edd885ed2` |
| `IMKB-TH-2010-03-25` | 2010-03-25 .. 2012-03-01 (the pre-2012 İşlem Saatleri page; Normal-week rows) | <https://web.archive.org/web/20100325232952id_/http://www.imkb.gov.tr/Markets/StockMarket/TransactionHours.aspx> | Wayback `id_` replay of capture `20100325232952`, retrieved 2026-09-30 04:30 UTC | T1 | `f09a1962671b53fde20f02edb86bee6de72d22f98ca719c94bb02d05a33277a2` |
| `IMKB-TH-2011-07-27` | 2010-03-25 .. 2012-03-01 (the same page with auction methods detailed; Normal-week corroboration) | <https://web.archive.org/web/20110727072510id_/http://www.imkb.gov.tr/Markets/StockMarket/TransactionHours.aspx> | Wayback `id_` replay of capture `20110727072510`, retrieved 2026-09-30 04:32 UTC | T1 | `c0815e3e31fecbb9bbf6b4c88514e7b6de82688dd7e2a03fd9b3af26bbbb0e23` |

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

- **Horizon sourced from 2010-03-25.** The baseline profile below 2012-03-02 — morning opening call 09:30–09:50 and continuous 09:50–12:30, afternoon call 14:00–14:20 and continuous 14:20–17:30 — is the operator's own İşlem Saatleri page print of 2010-03-25, corroborated by the 2011-07-27 capture and ended by the operator's dated 2012-03-02 closing-auction change (see the Normal week section). The carried region below it runs 2010-01-01..2010-03-24.
- **Interpretive step, order-entry classification.** From 2016-11-14 only the 09:40–09:55 Order Collection Process is `order_entry`; 09:55–10:00 carries the opening print and stays `extended`. The midday single-price call and the 18:00–18:10 closing envelope each bundle collection with a price-determination leg that prints, so both stay `extended` whole. Earlier eras carry no `order_entry` window because no source separates their collection legs.
- **Source set has no monitoring feed.** `EU-BIST` records that no stable consolidated announcements-feed URL is indexed; review means reopening the Equity Market Procedure, the individual circulars listed above, and the `Resmi Tatil Günleri` holiday page.
- **Service tier.** The consumer serves this venue live (it is one of the market-clock overview's venues), so the identity is **served** and the holiday-bearing calendar is reviewed monthly per LAW-WATCH; the 2027 closing condition above is the one tracked gap.
