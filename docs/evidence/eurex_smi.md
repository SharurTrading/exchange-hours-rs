<!-- SPDX-License-Identifier: MIT-0 -->

# `eurex_smi` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`eurex_smi.rs`](../../src/calendar/schedules/futures/international/eurex_smi.rs)
- **Source sets:** [`EU-EUREX`](../schedules/sources.md#eu-eurex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-13 UTC)

Eurex SMI® index futures (`FSMI`) on the Zurich clock: continuous trading 07:50–17:27 CET from the January-2010 floor until the operator's Contract Specifications amendment of 2010-07-05 extended it to 07:50–22:00, with the Pre-Trading and Post-Trading order-entry phases beside it. The Trading Calendar editions 2011–2026 reprint the extended row every year.

## Revision rows

- 2010-07-05 — T1 — Eurex CS amendment 2010-07-05 — FSMI's Continuous Trading extends from 07:50–17:27 to 07:50–22:00 CET and its Post-Trading Full-Period from 17:27–20:30 to 22:00–22:30 CET; Pre-Trading 07:30–07:50 is untouched, and the off-book OTC Block Trading column moves 08:00–19:00 → 08:00–22:00 (not modelled — a wholesale facility, the same scope the fixed-income family draws over TES).

The change is a **dated cutover, not an undated changeover.** The 2010 edition prints the product's row `SMI® Futures 5 (FSMI) 07:50 – 17:27` and every edition from 2011 prints `07:50 – 22:00`, so the changeover day is not stated by the editions themselves; the operator's archived Contract Specifications amendment `cs_history_05072010` — the Annex C "Trading Hours Futures Contracts" amendment page the research store's `normal-weeks/wave-c1/INDEX.md` names "Extension of trading time" — carries the `Index Futures Contracts` table with the FSMI row struck from `07:30-07:50 | 07:50-17:27 | 17:27-20:30 | 08:00-19:00` to `07:30-07:50 | 07:50-22:00 | 22:00-22:30 | 08:00-22:00` (`All times in CET`), the strikethrough-and-replacement rendering interleaving as `07:50-17:2722:00` in plain text extraction. Its effective day is stated three times over in the artifact and its channel: the redlined header date (struck `April`, replaced `July 05, 2010` — the overprinted glyphs extract as `April July 205, 2010`), the PDF document title `historyeurex14e_10-07-05.doc`, and the archive's own `cs_history_05072010` as-of naming — the same doctrine the fixed-income family records for its 2019-02-25 amendment. The day is unconditional at T1 in session language, so 2010-07-05 keys the row and no residual remains. The `eurex` venue row's floor-baseline citations were cross-checked for exactly this: its archived-specification corpus is where this amendment is banked (`normal-weeks/wave-c1/eurex/exttime-2010.pdf`, whose INDEX entry notes it "carries index-futures rows only (MSCI Russia, OMXH25, SLI, SMI)").

## Holidays

**Coverage:** 2010-01-01..2026-12-31 (inclusive trade dates in `Europe/Zurich`; tier T1 throughout).

The table is the **all-derivatives rows plus the Swiss-scope rows**. The editions' `in all derivatives` lists close every product and key their rows exactly as they key the sibling `eurex` tables. The recurring Swiss clause — `Eurex is closed for trading and clearing (exercise and settlement) in Swiss fixed income as well as equity and equity index derivatives; no cash payment in CHF: …` — names Swiss equity index derivatives, which FSMI is, so each date its list prints keys a row here. The clause's `no cash payment in CHF` tail is a clearing-cash annotation; the 2015 edition's separate `No cash payment in CHF, GBP and KRW: 25 May` line is the same annotation on that year's all-derivatives Whit Monday closure (the trading-only closure `Eurex is closed for trading in all derivatives: 25 May, …` keys the row) and keys nothing of its own.

The German-scope notes — `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: …` — never name this family, so the five German-only rows the sibling `TABLE` carries (2014-10-03, 2016-10-03, 2017-10-03, 2017-10-31 and 2018-10-03) do **not** appear here and FSMI keeps trading through them; the three Whit Mondays the German note also closes (2016-05-16, 2017-06-05 and 2018-05-21) are Swiss clause dates as well, so they close here on the Swiss line regardless. The family's own test fences the split. The 2019–2021 editions' parenthetical (`trading in German equity index futures takes place!`) and the 2025–2026 `tba` lines verified to no closures (2026-10-05 UTC, #157) leave nothing further to encode; see [eurex](eurex.md) for the verification chain, which this family inherits.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EUREX-CS-2009-10-26` | the FSMI row in force at the January-2010 floor | <https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf> | retrieved 2026-09-30T04:33Z | T1 | `fea3a3c859112b482380a5e1ee017108978dbc9a3fadc7e953082225fc19d6bb` |
| `EUREX-CS-2010-07-05` | 2010-07-05 (the extension this amendment dates) | <https://www.eurex.com/resource/blob/330302/b900fed6f0a95921bd1b24e768b14c76/data/cs_history_05072010_en.pdf.pdf> | retrieved 2026-09-30T04:42Z | T1 | `21941e2d858e452ca415c2b2d98c35f8f15f19667543180ddade8c0a1a8ca845` |
| `EUREX-TH-2026-10-11` | no rows keyed (the current trading-hours page, corroboration for the carried order-entry windows) | <https://www.eurex.com/ex-en/trade/trading-hours> | retrieved 2026-10-11 | T1 | `83313d90dcfc4116fa0923f6bb7904703e9a722e4f1cc90af33c2de0ded4dc6e` |
| `EUREX-CAL-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100216003515id_/http://www.eurexchange.com/download/trading/tradingcalendar_2010_en.pdf> | Wayback `id_` replay of capture `20100216003515`, retrieved 2026-09-29 UTC | T1 | `9caaee91446b115fc1b069fec44653290cf1e57cefc26f86859fe5bee0e98f59` |
| `EUREX-CAL-2011` | 2011-01-01 .. 2011-12-31 | <https://web.archive.org/web/20110114201916id_/http://www.eurexchange.com/download/trading/tradingcalendar_2011_en.pdf> | Wayback `id_` replay of capture `20110114201916`, retrieved 2026-09-29 UTC | T1 | `329fe76877537c873b433ab0b7fe2093b58fd72d0162444479d53a90ef704b2c` |
| `EUREX-CAL-2012` | 2012-01-01 .. 2012-12-31 | <https://www.eurex.com/resource/blob/284994/e988f1cef4437150c0a74a5a8f285d64/data/tradingcalendar_2012_en.pdf> | retrieved 2026-09-29 UTC | T1 | `3045def26f7af8ab5228b5714fb3d75043d14287c2ea8e476c43f3b0ac8d5739` |
| `EUREX-CAL-2013` | 2013-01-01 .. 2013-12-31 | <https://www.eurex.com/resource/blob/244836/3ad19654e330995e7d07e4da214ccecd/data/tradingcalendar_2013_en.pdf> | retrieved 2026-09-29 UTC | T1 | `c22be70750ba8bac19e45e62bf638deb0c0efd0321a193b4bc633c3a6f21fd40` |
| `EUREX-CAL-2014` | 2014-01-01 .. 2014-12-31 | <https://www.eurex.com/resource/blob/249120/6fadad3fe12a3cc3fa392442532d4414/data/tradingcalendar_2014_en.pdf> | retrieved 2026-09-29 UTC | T1 | `684fe66cb662a827ef9f6b44a41d566ec122b98fab5781b81af0ec28e22a1805` |
| `EUREX-CAL-2015` | 2015-01-01 .. 2015-12-31 | <https://www.eurex.com/resource/blob/243714/8c6cea18bbe820e719a578e3bd94976c/data/tradingcalendar_2015_en.pdf> | retrieved 2026-09-29 UTC | T1 | `45b0dc151bca7bbe80f7f659b725afc3016ec8201f43a66880ba7b8782f61ef7` |
| `EUREX-CAL-2016` | 2016-01-01 .. 2016-12-31 | <https://www.eurex.com/resource/blob/1260/f42d8f907b73ce30a66148cc287f2fe8/data/tradingcalendar_2016_en.pdf> | retrieved 2026-09-29 UTC | T1 | `3ef1bb9885fe0d94f903269ff75dda52731752647ad93772e65e80aeaa4a32f7` |
| `EUREX-CAL-2017` | 2017-01-01 .. 2017-12-31 | <https://www.eurex.com/resource/blob/241960/7dc968da8ed8d80fa004bd695ac9b648/data/tradingcalendar_2017_en.pdf> | retrieved 2026-09-29 UTC | T1 | `86ce6888b3d98ad82da84eb25a36bc9e5d928f1eb0ee4fdc194c77e38cdfe2de` |
| `EUREX-CAL-2018` | 2018-01-01 .. 2018-12-31 | <https://www.eurex.com/resource/blob/2806/32828d43772ac5ee3dd3d77a79743b47/data/tradingcalendar_2018_en.pdf> | retrieved 2026-09-29 UTC | T1 | `e90fb8d6fbe90945678b8646f7f22ba30f6b2e7c3b42f2d6876b254888e8fa81` |
| `EUREX-CAL-2019` | 2019-01-01 .. 2019-12-31 | <https://www.eurex.com/resource/blob/1396378/2915487ad1e2f7d56508e1bd2f056995/data/tradingcalendar_2019_en.pdf> | retrieved 2026-09-29 UTC | T1 | `f0a745897a1f3153a61a0ae8d3eda5245e79ef1376e8abcbb629fa81910504c9` |
| `EUREX-CAL-2020` | 2020-01-01 .. 2020-12-31 | <https://www.eurex.com/resource/blob/1690338/91db2a4864f0634f3ff1695ff0dddde5/data/tradingcalendar_2020_en.pdf> | retrieved 2026-09-29 UTC | T1 | `5359f2632aa397a686615d2308fe145977ea42799c0819988d1c83ee8ca929be` |
| `EUREX-CAL-2021` | 2021-01-01 .. 2021-12-31 | <https://www.eurex.com/resource/blob/2348622/f4f9a1b370f2e74462532319f2b15ddb/data/tradingcalendar_2021_en.pdf> | retrieved 2026-09-29 UTC | T1 | `18a537935fbbf705a89485b428d525f3931467056d028a68a3d081e061b07cfd` |
| `EUREX-CAL-2022` | 2022-01-01 .. 2022-12-31 | <https://www.eurex.com/resource/blob/2886140/a13cf78f8e10f826df3bf79278dd518a/data/tradingcalendar_2022_en.pdf> | retrieved 2026-09-29 UTC | T1 | `395ab4adacb3a6596ba087f389365432cff1d63a7aadba5186a659aaf7f3b77f` |
| `EUREX-CAL-2023` | 2023-01-01 .. 2023-12-31 | <https://www.eurex.com/resource/blob/3378814/910cf372738890f691bc1bfbccfd3aef/data/tradingcalendar_2023_en.pdf> | retrieved 2026-09-29 UTC | T1 | `c1d936b4201a69d8dce24e12d97ad2d73f17f04f9e9f075511e1a74f8002892f` |
| `EUREX-CAL-2024` | 2024-01-01 .. 2024-12-31 | <https://www.eurex.com/resource/blob/3816864/92a1dacec4476fdef2819750d932968a/data/tradingcalendar_2024_en.pdf> | retrieved 2026-09-29 UTC | T1 | `9028606ab0b01bcf3d8dbae28ecd9b76984769f14eb2d6d0a6c136e2c227b7e2` |
| `EUREX-HOLREG-2025` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250913041407id_/https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> | archive capture 2025-09-13T04:14:07Z (retrieved 2026-09-26T07:15:50Z) | T1 | `878e46c01a166f107607c19da8842e270f97915cfc1675b8928d2167ca804ea8` |
| `EUREX-TC-2025` | 2025-01-01 .. 2025-12-31 | <https://www.eurex.com/resource/blob/4242284/1c8da6dc2702d508ee2a4740654ea77d/data/tradingcalendar_2025_en.pdf> | live retrieval 2026-09-26T07:14:29Z | T1 | `d51053a39e786022db3fa2f12e00d9646145db10aa308ea1bd4446ed0cd16614` |
| `EUREX-HOLREG-2026` | 2026-01-01 .. 2026-12-31 | <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> | live retrieval 2026-09-12T04:19Z | T1 | `7b28acd2d2fb126c01461ef5a4ae11fe93e8b6f318001c6304bbce0fc78f3821` |
| `EUREX-TC-2026` | 2026-01-01 .. 2026-12-31 | <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> | live retrieval 2026-09-12T04:18Z | T1 | `b0796b42819b38c0757d727d9b789360ba84cd0d45cea215544f86342158ac65` |

The Trading Calendar editions and Holiday regulations rows are copied byte for byte from [eurex](eurex.md) § Documents; the store holds their bytes under `holidays/raw/eurex-2010-2024/` (with `INDEX.md` and `sha256s.txt`), `holidays/raw/eurex-2025-2027/` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/`, and the two CS amendments plus the trading-hours page under `normal-weeks/wave-c1/eurex/` and `normal-weeks/eurex-smi-2026-10-11/`.

The Swiss clause sits inside the editions' `Overview of holidays by countries` panel, whose calendar grid interleaves the prose in text extraction; the clause's wording is quoted as the panel prints it (`Eurex is closed for trading and clearing (exercise and settlement) in … Swiss fixed income as well as equity and equity index derivatives; no cash payment in CHF: <dates>`), and each year's date list below was re-extracted from the edition's bytes on 2026-10-13 UTC. The Holiday-regulations rows quote the page's own day-by-day sentences, which are byte-contiguous.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2010-04-02 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-04-02 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2010-04-05 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-04-05 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2010-05-13 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-05-13 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2010-05-24 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-05-24 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2010-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2010-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2010` | T1 | Eurex event date 2010-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5`; `Eurex is closed for trading in all derivatives: December 24, December 31`. The Swiss clause: `… in Swiss fixed income as well as equity and equity index derivatives; no cash payment in CHF: May 13, May 24`.

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2011` | T1 | Eurex event date 2011-04-22 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2011-04-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2011` | T1 | Eurex event date 2011-04-25 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2011-06-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2011` | T1 | Eurex event date 2011-06-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2011-06-13 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2011` | T1 | Eurex event date 2011-06-13 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2011-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2011` | T1 | Eurex event date 2011-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2011-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2011` | T1 | Eurex event date 2011-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: April 22, April 25, December 26`. The Swiss clause: `… Swiss fixed income as well as equity and equity index derivatives; no cash payment in CHF: June 2, June 13, August 1`.

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-04-06 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-04-06 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-04-09 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-04-09 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-05-17 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-05-17 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-05-28 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-05-28 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2012-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: April 6, April 9, May 1, December 25, December 26`; `… trading in all derivatives: December 24, December 31`. The Swiss clause: `… Swiss … ; no cash payment in CHF: January 2, May 17, May 28, August 1`.

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-03-29 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-03-29 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-04-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-04-01 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-05-09 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-05-09 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-05-20 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-05-20 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2013-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26`; `… trading in all derivatives: December 24, December 31`. The Swiss clause: `… Swiss … ; no cash payment in CHF: January 2, May 9, May 20, August 1`.

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-04-18 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-04-18 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-04-21 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-04-21 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-05-29 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-05-29 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-06-09 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-06-09 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2014-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December`; `… trading in all derivatives: 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 29 May, 9 June, 1 August`.

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-04-03 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-04-03 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-04-06 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-04-06 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-05-14 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-14 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-05-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-25 (Whit Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2015-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December`; `… trading in all derivatives: 25 May, 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 14 May` — the edition's separate `No cash payment in CHF, GBP and KRW: 25 May` line is a clearing-cash annotation on the all-derivatives Whit Monday closure, not a scope date.

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2016-03-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-03-25 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2016-03-28 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-03-28 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2016-05-05 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-05-05 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2016-05-16 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-05-16 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2016-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2016-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2016` | T1 | Eurex event date 2016-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 25 March, 28 March, 26 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 5 May, 16 May, 1 August`.

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-04-14 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-04-14 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-04-17 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-04-17 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-05-25 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-05-25 (Whit Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-06-05 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-06-05 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2017-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2017` | T1 | Eurex event date 2017-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 25 May, 5 June, 1 August`.

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-03-30 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-03-30 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-04-02 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-04-02 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-05-10 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-05-10 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-05-21 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-05-21 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2018-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December`; `… trading in all derivatives: 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 10 May, 21 May, 1 August`.

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-04-19 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-04-19 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-04-22 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-04-22 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-05-30 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-05-30 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-06-10 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-06-10 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2019-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December`; `… trading in all derivatives: 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 30 May, 10 June, 1 August`.

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-04-10 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-04-10 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-04-13 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-04-13 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-05-21 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-05-21 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-06-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-06-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2020-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December`; `… trading in all derivatives: 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 21 May, 1 June` — no 1 August (a Saturday).

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2021-04-02 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-04-02 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2021-04-05 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-04-05 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2021-05-13 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-05-13 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2021-05-24 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-05-24 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2021-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2021-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2021` | T1 | Eurex event date 2021-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 2 April, 5 April`; `… trading in all derivatives: 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 13 May, 24 May` — no 1 August (a Sunday).

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2022` | T1 | Eurex event date 2022-04-15 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2022-04-18 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2022` | T1 | Eurex event date 2022-04-18 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2022-05-26 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2022` | T1 | Eurex event date 2022-05-26 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2022-06-06 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2022` | T1 | Eurex event date 2022-06-06 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2022-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2022` | T1 | Eurex event date 2022-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2022-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2022` | T1 | Eurex event date 2022-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 15 April, 18 April, 26 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 26 May, 6 June, 1 August`.

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-04-07 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-04-07 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-04-10 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-04-10 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-05-18 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-05-18 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-05-29 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-05-29 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2023-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2023` | T1 | Eurex event date 2023-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 18 May, 29 May, 1 August`.

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-03-29 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-03-29 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-04-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-04-01 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-05-01 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-05-09 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-05-09 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-05-20 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-05-20 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-12-24 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-12-25 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-12-26 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2024-12-31 | closed | the year preamble's all-derivatives list | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: `… in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December`; `… trading in all derivatives: 24 December, 31 December`. The Swiss clause: `… Swiss … ; no cash payment in CHF: 2 January, 9 May, 20 May, 1 August`.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-04-18 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-04-18 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-04-21 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-04-21 (Easter Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-05-01 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-05-29 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-05-29 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-06-09 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-06-09 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-08-01 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-08-01 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-12-24 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-12-25 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-12-26 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-26 (Boxing Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2025-12-31 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: the Holiday regulations § 2025 day-by-day all-derivatives rows, corroborated by the Trading Calendar 2025 PDF. The Swiss clause: `Eurex is closed for trading and clearing (exercise and settlement) in Swiss fixed income as well as equity and equity index derivatives; no cash payment in CHF.` — the § 2025 rows of 02 January, 29 May, 09 June and 01 August.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-01-01 (New Year's Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-01-02 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-01-02 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-04-03 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-04-03 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-04-06 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-04-06 (Good Friday); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-05-01 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-05-01 (Labour Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-05-14 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-05-14 (the printed entry); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-05-25 | closed | the year preamble's Swiss clause date list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-05-25 (Whit Monday); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-12-24 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-24 (Christmas Eve); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-12-25 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-25 (Christmas Day); one Zurich civil day per trade date, so the conversion is the identity |
| 2026-12-31 | closed | the year preamble's all-derivatives list | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-31 (New Year's Eve); one Zurich civil day per trade date, so the conversion is the identity |

The edition's all-derivatives list: the Holiday regulations § 2026 day-by-day all-derivatives rows, corroborated by the Trading Calendar 2026 PDF. The Swiss clause: the same § 2026 rows of 02 January, 14 May and 25 May.

## Sources

- The operator's annual Trading Calendar editions 2010–2024 (`EUREX-CAL-*`) and Holiday regulations page for 2025–2026 (`EUREX-HOLREG-*`), shared with the sibling Eurex identities; the monitoring entry points are the [trading calendar hub](https://www.eurex.com/ex-en/trade/trading-calendar) and the [Holiday regulations page](https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations) — T1.
- The archived Contract Specifications amendments `cs_history_26102009` and `cs_history_05072010` (`EUREX-CS-2009-10-26`, `EUREX-CS-2010-07-05`), reached through the [Contract Specifications index](https://www.eurex.com/ex-en/rules-regs/eurex-rules-regulations/03.-Contract-Specifications-4347288) — T1.
- The [trading-hours page](https://www.eurex.com/ex-en/trade/trading-hours) (`EUREX-TH-2026-10-11`) — T1, the normal-week monitoring entry point; its body states the venue-wide regular day and points per-product schedules at the contract specifications.

## Gaps and residual risks

- **The current order-entry windows are keyed by the 2010 amendment and carried.** Pre-Trading 07:30–07:50 and Post-Trading 22:00–22:30 CET are stated by `EUREX-CS-2010-07-05` and untouched by every later banked artifact; no current Annex C page for FSMI is banked in the research store, so the family's ledger basis is **Partial / order-entry**: the regular phase is re-witnessed yearly (the editions' `07:50 – 22:00` row through 2026), while the two order-entry windows rest on the last operator statement in hand. Named closer: a capture of the current Contract Specifications Annex C (the trading-hours page links it), which would re-key or correct the two windows; the banked page's own clearing note — "the clearing hours of all products which are continuously traded until 22:00 end at 22:30" (`EUREX-TH-2026-10-11`) — corroborates the 22:00–22:30 shape without stating a phase.
- **Seasonality is a reading, recorded here.** The editions print the FSMI row `07:50 – 22:00` with no seasonal footnote (the DAX rows carry the `02:10–22:00 CEST` footnote for their UTC-pinned Asian-hours opens), and the CS amendments print `All times in CET` over a table whose values the operator reprints unchanged across editions, so the endpoints are read as local-stable Zurich wall-clock times and one timeline serves the whole history. A per-product Annex C capture stating a CEST variant would move the grid to a two-timeline selector as a normal schedule fix.
- **The Last Trading Day column is expiry, not session.** The editions' `09:00` for FSMI is a per-contract last-trading-day trading-until instant (LAW-SESSION-NOT-EXPIRY); nothing of it is encoded.
- **Holiday horizon.** The audited window stops at 2026-12-31 because the operator publishes 2027–2036 only "on a preliminary and indicative basis … subject to change"; the sibling [eurex](eurex.md) row carries the same stop. Closing condition: the operator's unconditional 2027 calendar; the row is re-checked monthly per LAW-WATCH beside the sibling identities' review.
