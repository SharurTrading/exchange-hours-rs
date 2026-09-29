<!-- SPDX-License-Identifier: MIT-0 -->

# `hkex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`hkex.rs`](../../src/calendar/schedules/equities/apac/hkex.rs)
- **Source sets:** [`APAC-HKEX`](../schedules/sources.md#apac-hkex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Securities venue union: the executable Extended Morning Session is Regular under crate semantics and bridges the former lunch gap. The 2011 open change and 2016-07-25 CAS tail are observable cutovers; 2012 only rearranged internal phases.

## Revision rows

- 2011-03-07 — T1 — HKEX news release 110303news — Phase One: morning session 09:30–12:00, Extended Morning Session 12:00–13:30, afternoon session 13:30–16:00, with the Pre-opening Session moved to 09:00–09:30.
- 2016-07-25 — T1 — HKEX market communication 160725news — the Closing Auction Session adds the 16:00–16:10 tail for its first eligible securities.

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive venue-local trade dates in `Asia/Hong_Kong`; tier T1 throughout).

The rows key on HKEX's own `Trading Calendar and Holiday Schedule` page, whose `Holiday Schedule` table names every day the markets are closed (`Holiday (no trading)`) and whose `Notes` block names every shortened eve (`no afternoon and after-hours trading session`). Three editions were retrieved: the 2025-10-07 capture (`HKEX-TC-2025`, page footer "Updated 21 Aug 2025") governs 2025; the live edition (`HKEX-TC-2026-2027`, "Updated 31 Jul 2026") governs 2026 and 2027; the 2026-02-13 capture (`HKEX-TC-2026`, "Updated 16 Jan 2026") corroborates 2026 unchanged between editions. The half-day instant is the operator's own securities-market half-day schedule (`HKEX-HOURS-SEC`): on the eves of Christmas, New Year and Lunar New Year there is no Extended Morning Session and no Afternoon Session, and the Closing Auction Session runs `12:00 noon to a random closing between 12:08 p.m. and 12:10 p.m.`, so the row states the latest scheduled close edge 12:10 exactly as an ordinary day states 16:10.

Severe-weather arrangements (typhoon signals) are conditional and key no row (LAW-NO-FABRICATED-DATES). The `#` footnotes on `25/5/2026` and `25/12/2025` suspend only the after-hours sessions of derivatives Holiday Trading Products (MSCI) and name no securities session. The securities market's own statement that trading is "Monday to Friday (excluding public holidays)" makes each printed holiday a full closure of the venue envelope.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `1/1/2025 (Wednesday) — The first day of January — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-01; the schedule deletes the day |
| 2025-01-28 | early close | `28 January 2025 (Tuesday) – Eve of Lunar New Year` — "no afternoon and after-hours trading session"; half-day CAS `12:00 noon to a random closing between 12:08 p.m. and 12:10 p.m.` | `HKEX-TC-2025` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2025-01-29 | closed | `29/1/2025 (Wednesday) — Lunar New Year's Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-29 |
| 2025-01-30 | closed | `30/1/2025 (Thursday) — The second day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-30 |
| 2025-01-31 | closed | `31/1/2025 (Friday) — The third day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-01-31 |
| 2025-04-04 | closed | `4/4/2025 (Friday) — Ching Ming Festival — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-04-04 |
| 2025-04-18 | closed | `18/4/2025 (Friday) — Good Friday — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-04-18 |
| 2025-04-21 | closed | `21/4/2025 (Monday) — Easter Monday — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-04-21 |
| 2025-05-01 | closed | `1/5/2025 (Thursday) — Labour Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-05-01 |
| 2025-05-05 | closed | `5/5/2025 (Monday) — The Birthday of the Buddha — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-05-05 |
| 2025-07-01 | closed | `1/7/2025 (Tuesday) — Hong Kong Special Administrative Region Establishment Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-07-01 |
| 2025-10-01 | closed | `1/10/2025 (Wednesday) — National Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-10-01 |
| 2025-10-07 | closed | `7/10/2025 (Tuesday) — The day following the Chinese Mid-Autumn Festival — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-10-07 |
| 2025-10-29 | closed | `29/10/2025 (Wednesday) — Chung Yeung Festival — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-10-29 |
| 2025-12-24 | early close | `24 December 2025 (Wednesday) – Eve of Christmas Day` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2025` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2025-12-25 | closed | `25/12/2025 (Thursday) — Christmas Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-12-25 |
| 2025-12-26 | closed | `26/12/2025 (Friday) — The first weekday after Christmas Day — Holiday (no trading)` | `HKEX-TC-2025` | T1 | HKEX event date 2025-12-26 |
| 2025-12-31 | early close | `31 December 2025 (Wednesday) – Eve of New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2025` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `1/1/2026 (Thursday) — The first day of January — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-01-01 |
| 2026-02-16 | early close | `16 February 2026 (Monday) – Eve of Lunar New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2026-02-17 | closed | `17/2/2026 (Tuesday) — Lunar New Year's Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-02-17 |
| 2026-02-18 | closed | `18/2/2026 (Wednesday) — The second day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-02-18 |
| 2026-02-19 | closed | `19/2/2026 (Thursday) — The third day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-02-19 |
| 2026-04-03 | closed | `3/4/2026 (Friday) — Good Friday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-04-03 |
| 2026-04-06 | closed | `6/4/2026 (Monday) — The day following Ching Ming Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-04-06 |
| 2026-04-07 | closed | `7/4/2026 (Tuesday) — The day following Easter Monday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-04-07 |
| 2026-05-01 | closed | `1/5/2026 (Friday) — Labour Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-05-01 |
| 2026-05-25 | closed | `25/5/2026 (Monday) — The day following the Birthday of the Buddha — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-05-25; the `#` footnote suspends only derivatives MSCI after-hours trading |
| 2026-06-19 | closed | `19/6/2026 (Friday) — Tuen Ng Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-06-19 |
| 2026-07-01 | closed | `1/7/2026 (Wednesday) — Hong Kong Special Administrative Region Establishment Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-07-01 |
| 2026-10-01 | closed | `1/10/2026 (Thursday) — National Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-10-01 |
| 2026-10-19 | closed | `19/10/2026 (Monday) — The day following Chung Yeung Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-10-19 |
| 2026-12-24 | early close | `24 December 2026 (Thursday) – Eve of Christmas Day` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2026-12-25 | closed | `25/12/2026 (Friday) — Christmas Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2026-12-25 |
| 2026-12-31 | early close | `31 December 2026 (Thursday) – Eve of New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `1/1/2027 (Friday) — The first day of January — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-01-01 |
| 2027-02-05 | early close | `05 February 2027 (Friday) – Eve of Lunar New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2027-02-08 | closed | `8/2/2027 (Monday) — The third day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-02-08; Lunar New Year's Day 2027-02-06 falls on Saturday and names no weekday closure |
| 2027-02-09 | closed | `9/2/2027 (Tuesday) — The fourth day of Lunar New Year — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-02-09 |
| 2027-03-26 | closed | `26/3/2027 (Friday) — Good Friday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-03-26 |
| 2027-03-29 | closed | `29/3/2027 (Monday) — Easter Monday — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-03-29 |
| 2027-04-05 | closed | `5/4/2027 (Monday) — Ching Ming Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-04-05 |
| 2027-05-13 | closed | `13/5/2027 (Thursday) — The Birthday of the Buddha — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-05-13 |
| 2027-06-09 | closed | `9/6/2027 (Wednesday) — Tuen Ng Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-06-09 |
| 2027-07-01 | closed | `1/7/2027 (Thursday) — Hong Kong Special Administrative Region Establishment Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-07-01 |
| 2027-09-16 | closed | `16/9/2027 (Thursday) — The day following the Chinese Mid-Autumn Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-09-16 |
| 2027-10-01 | closed | `1/10/2027 (Friday) — National Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-10-01 |
| 2027-10-08 | closed | `8/10/2027 (Friday) — Chung Yeung Festival — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-10-08 |
| 2027-12-24 | early close | `24 December 2027 (Friday) – Eve of Christmas Day` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |
| 2027-12-27 | closed | `27/12/2027 (Monday) — The first weekday after Christmas Day — Holiday (no trading)` | `HKEX-TC-2026-2027` | T1 | HKEX event date 2027-12-27; Christmas Day 2027-12-25 falls on Saturday and names no weekday closure of its own |
| 2027-12-31 | early close | `31 December 2027 (Friday) – Eve of New Year` — "no afternoon and after-hours trading session"; half-day CAS to between 12:08 p.m. and 12:10 p.m. | `HKEX-TC-2026-2027` | T1 | the eve names a trading day whose afternoon is deleted; the 12:10 close is `HKEX-HOURS-SEC`'s half-day CAS edge |

**Gaps: none inside the window.** Every date the operator's three editions print ships a row, and every other trade date in 2025-2027 is audited normal: the operator's publication pattern is one complete schedule per year pair, so a weekday with no row is an ordinary trading day rather than unworked material.

**Recorded check, 2026-12-28.** The operator's 2026 table ends at `25/12/2026 (Friday) Christmas Day` in all three editions retrieved (2025-10-07, 2026-02-13 and live 2026-07-31), so no row ships for Monday 2026-12-28 and the crate answers it as an ordinary trading day. The operator's printing holds: Hong Kong's general holidays for 2026 name "the first weekday after Christmas Day" as **Saturday 26 December** (2026-12-28 is not a general holiday), so no conflict exists — the exchange schedule and the general-holiday list agree. A check against gov.hk's 2026 list was performed at review (2026-09-28 UTC); its bytes are the public gov.hk page. **Closing condition:** none — this is a verification record, not a gap; an HKEX table edition that adds a 2026-12-28 row becomes a schedule fix. Re-checked monthly per LAW-WATCH.

### Documents

Every artifact was retrieved on 2026-09-28 UTC and saved under `holidays/raw/equities/hkex/2025-2027/` in the research store, whose `INDEX.md` carries the same digests.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `HKEX-TC-2025` | 2025-01-01 .. 2026-12-31 | <https://web.archive.org/web/20251007115724id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20251007115724`, page footer "Updated 21 Aug 2025") | Wayback `id_` replay of capture `20251007115724`, retrieved 2026-09-28 UTC | T1 | `9914f794f9df82e35b83d0d4b2abd120e5cb524655e9ff16857b6319828a2390` |
| `HKEX-TC-2026` | 2026-01-01 .. 2027-12-31 | <https://web.archive.org/web/20260213065549id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20260213065549`, page footer "Updated 16 Jan 2026") | Wayback `id_` replay of capture `20260213065549`, retrieved 2026-09-28 UTC | T1 | `a88c8d316588eab88f1597dbb018e3d6cbc0e5b0e471edd1036936b3dd2b0b68` |
| `HKEX-TC-2026-2027` | 2025-01-01 .. 2027-12-31 | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (page footer "Updated 31 Jul 2026") | retrieved 2026-09-28 01:28 UTC | T1 | `c460176b89fa6bbcfc77393e0013cc27bf03de9cb660367a1e2990b657d87f47` |
| `HKEX-HOURS-SEC` | 2025-01-01 .. 2027-12-31 | <https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en> (page footer "Updated 16 Sep 2017") | retrieved 2026-09-28 01:28 UTC | T1 | `9faebf6ae87c7c21f83fc906a0709416f213ccf08147931cff6a03693d4b3e6c` |

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en> — HKEX securities-market trading hours: POS 09:00–09:30, continuous trading 09:30–16:00, then CAS with a randomized 16:08–16:10 close.
- <https://www.hkex.com.hk/Global/Exchange/FAQ/Securities-Market/Trading/Pre_opening-Session?sc_lang=en> — HKEX Pre-opening Session FAQ. Orders "will be accumulated and updated but no matching will occur" during the order input and pre-order matching periods, so 09:00–09:20 is order entry.
- <https://www.hkex.com.hk/-/media/HKEX-Market/Services/Rules-and-Forms-and-Fees/Rules/SEHK/Securities/Rule-Update_Rules-of-the-Exchange/05-11-SEHK-StampDuty-TradingHour_e.pdf> — SEHK rule update. Rule 501G divides the 09:00–09:30 POS into four named periods: order input 09:00–09:15, pre-order matching (renamed no-cancellation in 2020) 09:15–09:20, order matching from 09:20, then a blocking period to 09:30.
- <https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en> — HKEX news release of 2011-03-03, announcing Phase One effective 2011-03-07 and stating the pre-change 10:00 morning open.
- <https://www.hkex.com.hk/News/Regulatory-Announcements/2012/120301news?sc_lang=en> — HKEX announcement of the 2012-03-05 Phase Two, which moved the internal Extended Morning/afternoon handoff to 13:00 without changing the envelope.
- <https://www.hkex.com.hk/News/Market-Communications/2016/160725news?sc_lang=en> — HKEX market communication, the 2016-07-25 CAS launch.

## Gaps and residual risks

- **horizon 2011-03-03** — the pre-2011 profile (10:00 open, POS 09:30–10:00) is attested only by the 2011-03-03 news release and the SEHK rule update that accompanied Phase One, so below 2011-03-03 the grid is carried back to the January-2010 floor rather than independently sourced (AGENTS.md, *Carry the earliest sourced state back to the floor*). Closing condition: a dated pre-2011 SEHK rulebook edition or trading-hours page.
- No primary SEHK text for the pre-2011-03-07 POS period boundaries was located, so the whole 09:30–10:00 window is left `extended` rather than guessing where its matching period began.
- The 2012-03-05 Phase Two is deliberately not a revision row: it rearranged internal phases without changing the venue-level open or close (LAW-HOLIDAY-SCOPE's companion rule on topology, and the ledger's own statement that it is not an observable envelope cutover).
- Later CAS eligibility expansions do not create new exchange-level open/close cutovers; the static profile uses the maximum scheduled CAS edge and not every security is eligible for every phase.
- The CAS 16:00–16:10 window ends in a randomised uncrossing that prints the closing trades, so the whole auction stays `extended`.
