<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_europe_gilts` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_europe_gilts.rs`](../../src/calendar/schedules/futures/international/ice_europe_gilts.rs)
- **Source sets:** [`ICE-EU-FIXED-INCOME`](../schedules/sources.md#ice-eu-fixed-income)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-14 UTC)

ICE Futures Europe gilt futures share one London clock — Monday–Friday
08:00–18:00 with a 06:03–08:00 pre-open — until circular 26/063 extends Long
and Medium Gilt alone to a 00:45 pre-open and 01:00–21:00 trading on
2026-07-06; the holiday layer is audited over four operator-era windows with
the 2015–2020 editions and the 2010 full-year notice recorded as gaps.

## Revision rows

- 2026-07-06 — T1 — ICE Futures Europe circular 26/063 — the circular of 23 April 2026 states "Effective from Monday 06 July 2026 the revised daily trading hours for the Contracts will be as follows" and prints Long Gilt Futures and Medium Gilt Futures at Pre Open 00:45, Open 01:00, Close 21:00 UK time; Short and Ultra Long Gilt are not among "the Contracts".

## The normal week

The shared gilt grid through 2026-07-05 is the operator's own Long Gilt
product page. The Wayback capture of 2026-03-11 (`ICE-GILT-PAGE-20260311`)
and the capture of 2026-05-09 (`ICE-GILT-PAGE-20260509`) both print
"LONDON | 8:00 AM - 6:00 PM | 08:00 - 18:00" with Pre-Open 06:03, and the
live page (`ICE-GILT-PAGE-LIVE`, retrieved 2026-10-11 UTC) prints the same
pre-change grid for the Short, Medium and Ultra Long Gilt products that
circular 26/063 left unchanged. The page names all four gilt futures as this
contract's related products, so the family trades one clock in this era. The
operator's temporary-trading-times document for 25–30 October 2026
(`ICE-EU-DST-2026`) reprints the split in its own table: "Medium and Long
Gilt Futures … 01:00-21:00 GMT" against "Short, Ultra Long Gilt Futures and
Long Gilt Options … 08:00-18:00 GMT".

The pre-open phases are `order_entry`: the page's Pre-Open column and the
circular's "Pre Open (UK Time)" column both precede the printed Open, and
nothing matches before it. `regular` carries the trading window; `extended`
is empty; the sessions are same-day and never wrap.

**The 2026-07-06 split and the disputed pre-open quarter-hour.** Circular
26/063 covers exactly four contract families — Three Month SONIA Indexed
Futures, MPC Dated SONIA Futures, Long Gilt Futures and Medium Gilt Futures —
and states "there will be no change to the current settlement windows for any
of the Contracts; and the time at which trading ceases on the Last Trading Day
remains unchanged". This key therefore narrows to the Long and Medium scope on
the cutover day; Short and Ultra Long Gilt stay on 08:00–18:00 and are a
different clock from that day, outside this key. On the pre-open the
circular's own table prints 00:45 while the operator's live product page
prints Pre-Open 00:35; the timeline holds the window both state — 00:45–01:00 —
and withholds the disputed 00:35–00:45 quarter-hour as a residual. Closing
condition: a desk answer or the operator's next spec-page revision stating one
instant.

**LAW-SESSION-NOT-EXPIRY.** The circular's unchanged "settlement windows" and
the Last-Trading-Day cessation are fixing and expiry instants, never session
boundaries; the Christmas documents' "designated settlement periods" columns
(12:13–12:15 GMT and siblings) are likewise not encoded — only the "Trading
ceases" column states a session instant.

**Carried normal week.** The full 08:00–18:00 grid is witnessed at the two
2026 captures; the NYSE Liffe notices of 2010–2013 open the gilt rows at
08:00 on their half days (corroborating the open era) but state no full
session grid, and no IFEU schedule edition survives for 2015–2020. Everything
below 2026-03-11 is therefore the carried baseline declared in
`sourcing.rs`, with the interim states unproven rather than asserted.

## Holidays

**Coverage:** 2010-12-24..2014-12-31, 2021-01-01..2021-12-31, 2022-12-23..2023-01-02, 2024-01-01..2026-12-31 (gilt trade dates, Europe/London; sterling-based-product closures under NYSE Liffe and the IFEU Fixed Income schedule editions thereafter).

The pattern is stable across every audited era: the exchange closes 1
January, Good Friday and Christmas Day outright; the gilt row additionally
closes on Easter Monday and the UK bank holidays (early May, spring bank
holiday, late August, and the in-lieu Boxing Day / New Year holidays), and
stays **open on 1 May when it falls in a weekday-only closure year** (the
2024, 2025 and 2026 editions print "Open" there). Christmas-eve and
New-Year-eve half days close at the instant the era's document prints — 12:15
GMT in the LIFFE attachments (2010–2013), 13:00 GMT on 2022-12-23, and 12:15
GMT on 2022-12-30 and both 2025 eves. Rows are keyed to the London civil
session date; these sessions are same-day, so no wrap reassignment arises.

**Announced eves with unpublished instants ship `Unsourced`**: LON3770 says
the 2014 eves close "at various times commencing at midday" — no exact
instant — and the 2021, 2024 and 2026 schedule editions defer their partial
trading days to a circular the captured set does not hold. Those dates are
inside a window and expressly **not** audited normal.

2015–2020 sit inside no window: no IFEU Fixed Income schedule edition for
those years survives in the archive (the Migrated-Liffe captures jump from
2021-10-22 back to nothing, and the ICE-era circular archive was not
enumerated). 2010 before 24 December is likewise outside the windows: the
full-year 2010 trading-days notice would be in the 2009 zip, which the hunt
did not pull. Both are the charter's bridged residuals — session questions
answer from the sourced normal week, the holiday classification refuses
typed. Closing conditions: an ICE circular-archive sweep for 2015–2020 and
the `nyse_liffe/historical_notices/2009.zip` for the 2010 notice.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-12-24 | early close | 12:15 GMT — "Long Gilt F … Open 08:00 … Close 12:15" (attachment) | `LIFFE-ZIP-2010` | T1 | LON3351's summary half day, precise instant from its Attachment 1 gilt row; same London civil trade date |
| 2010-12-27 | closed | − "Sterling-based products … will not be made available for trading" | `LIFFE-ZIP-2010` | T1 | LON3351, holiday in lieu of Christmas; same local civil trade date |
| 2010-12-28 | closed | − "Sterling-based products … will not be made available for trading" | `LIFFE-ZIP-2010` | T1 | LON3351, holiday in lieu of Boxing Day; same local civil trade date |
| 2010-12-31 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2010` | T1 | LON3351's summary half day with the attachment's precise instant; same London civil trade date |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-04-22 | closed | − "Friday 22 April 2011 (Good Friday)" | `LIFFE-ZIP-2010` | T1 | LON3352 §1 market-closed list; same local civil trade date |
| 2011-04-25 | closed | − "Monday 25 April 2011 (Easter Monday)" | `LIFFE-ZIP-2010` | T1 | LON3352 §1 market-closed list; same local civil trade date |
| 2011-05-02 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-05-30 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-08-29 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-12-23 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2011` | T1 | LON3526's sterling half day with the attachment's precise instant; same London civil trade date |
| 2011-12-26 | closed | − "Market Closed (UK Public Holiday)" | `LIFFE-ZIP-2011` | T1 | LON3526 and LON3352 §1; same local civil trade date |
| 2011-12-27 | closed | − "Sterling-based products … will not be made available" | `LIFFE-ZIP-2011` | T1 | LON3526 in-lieu closure, restated by LON3352 §2; same local civil trade date |
| 2011-12-30 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2011` | T1 | LON3526's sterling half day with the attachment's precise instant; same London civil trade date |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | − sterling-based products not available | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-04-06 | closed | − "Friday 6 April 2012 (Good Friday)" | `LIFFE-ZIP-2011` | T1 | LON3530 §1 market-closed list; same local civil trade date |
| 2012-04-09 | closed | − "Monday 9 April 2012 (Easter Monday)" | `LIFFE-ZIP-2011` | T1 | LON3530 §1 market-closed list; same local civil trade date |
| 2012-05-07 | closed | − sterling-based products not available | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-06-04 | closed | − sterling-based products not available (spring bank holiday) | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-06-05 | closed | − sterling-based products not available (Jubilee holiday) | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-08-27 | closed | − sterling-based products not available | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-12-24 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2012` | T1 | LON3655's summary half day with the attachment's precise instant; same London civil trade date |
| 2012-12-25 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2012` | T1 | LON3655 and LON3530 §1; same local civil trade date |
| 2012-12-26 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2012` | T1 | LON3655 and LON3530 §1; same local civil trade date |
| 2012-12-31 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2012` | T1 | LON3655's summary half day with the attachment's precise instant; LON3664 amended only Euribor, not gilts |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | − "Tuesday 1 January 2013 (New Year's Day)" | `LIFFE-ZIP-2012` | T1 | LON3657 §1 market-closed list; same local civil trade date |
| 2013-03-29 | closed | − "Friday 29 March 2013 (Good Friday)" | `LIFFE-ZIP-2012` | T1 | LON3657 §1 market-closed list; same local civil trade date |
| 2013-04-01 | closed | − "Monday 1 April 2013 (Easter Monday)" | `LIFFE-ZIP-2012` | T1 | LON3657 §1 market-closed list; same local civil trade date |
| 2013-05-06 | closed | − sterling-based products not available | `LIFFE-ZIP-2012` | T1 | LON3657 §2 UK Public Holiday list; same local civil trade date |
| 2013-05-27 | closed | − sterling-based products not available | `LIFFE-ZIP-2012` | T1 | LON3657 §2 UK Public Holiday list; same local civil trade date |
| 2013-08-26 | closed | − sterling-based products not available | `LIFFE-ZIP-2012` | T1 | LON3657 §2 UK Public Holiday list; same local civil trade date |
| 2013-12-24 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2013` | T1 | LON3762's summary half day with the attachment's precise instant; same London civil trade date |
| 2013-12-25 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2013` | T1 | LON3762 and LON3657 §1; same local civil trade date |
| 2013-12-26 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2013` | T1 | LON3762 and LON3657 §1; same local civil trade date |
| 2013-12-31 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2013` | T1 | LON3762's summary half day with the attachment's precise instant; same London civil trade date |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | − "Wednesday 1 January 2014 (New Year's Day)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-04-18 | closed | − "Friday 18 April 2014 (Good Friday)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-04-21 | closed | − "Monday 21 April 2014 (Easter Monday)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-05-05 | closed | − sterling-based products not available | `LIFFE-ZIP-2013` | T1 | LON3770 §2 UK Public Holiday list; same local civil trade date |
| 2014-05-26 | closed | − sterling-based products not available | `LIFFE-ZIP-2013` | T1 | LON3770 §2 UK Public Holiday list; same local civil trade date |
| 2014-08-25 | closed | − sterling-based products not available | `LIFFE-ZIP-2013` | T1 | LON3770 §2 UK Public Holiday list; same local civil trade date |
| 2014-12-24 | unsourced | announced half day; "contracts will close at various times commencing at midday" | `LIFFE-ZIP-2013` | T1 | LON3770 §3 states the arrangement without an exact instant; the date is expressly not audited normal |
| 2014-12-25 | closed | − "Thursday 25 December 2014 (Christmas Day)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-12-26 | closed | − "Friday 26 December 2014 (Boxing Day)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-12-31 | unsourced | announced half day; "contracts will close at various times commencing at midday" | `LIFFE-ZIP-2013` | T1 | LON3770 §3 states the arrangement without an exact instant; the date is expressly not audited normal |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | − "Friday 1 January 2021 (New Year's Day)" | `IFEU-FI-2021` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2021-04-02 | closed | − "Friday 2 April 2021 (Good Friday)" | `IFEU-FI-2021` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2021-04-05 | closed | − gilt row "Closed", 5 April column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-05-03 | closed | − gilt row "Closed", 3 May column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-05-31 | closed | − gilt row "Closed", 31 May column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-08-30 | closed | − gilt row "Closed", 30 August column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-12-24 | unsourced | announced partial trading day; "will be confirmed via an Exchange Circular" | `IFEU-FI-2021` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |
| 2021-12-27 | closed | − gilt row "Closed", 27 December column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table, Christmas in lieu; same local civil trade date |
| 2021-12-28 | closed | − gilt row "Closed", 28 December column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table, Boxing Day in lieu; same local civil trade date |
| 2021-12-31 | unsourced | announced partial trading day; "will be confirmed via an Exchange Circular" | `IFEU-FI-2021` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-12-23 | early close | 13:00 GMT — gilt rows "Trading ceases 13:00 (GMT)" | `IFEU-XMAS-2022` | T1 | The Christmas document's Friday 23 December page; same London civil trade date |
| 2022-12-26 | closed | − "The Exchange will be closed" | `IFEU-XMAS-2022` | T1 | The Christmas document's Monday 26 December page; same local civil trade date |
| 2022-12-27 | closed | − gilts listed among the contracts excepted from trading | `IFEU-XMAS-2022` | T1 | The Christmas document's Tuesday 27 December page; in-lieu closure, same local civil trade date |
| 2022-12-30 | early close | 12:15 GMT — gilt rows "Trading ceases 12:15 (GMT)" | `IFEU-XMAS-2022` | T1 | The Christmas document's Friday 30 December page; same London civil trade date |

### 2023

| 2023-01-02 | closed | − exchange "CLOSED for trading EXCEPT for the contracts listed below"; gilts not listed | `IFEU-XMAS-2022` | T1 | The Christmas document's Monday 02 January 2023 page; in-lieu closure, same local civil trade date |
| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|

The rest of 2023 sits inside no window: the 2023 Fixed Income edition is not
in the captured set (the 2023-09-27 capture is the Oil/Utilities/Agricultural
edition), so its Good Friday, Easter Monday, May and Coronation and August
bank-holiday treatments are unaudited. Closing condition: the IFEU 2023 Fixed
Income edition.

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | − "Monday 1 January 2024 (New Year's Day)" | `IFEU-FI-2024` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2024-03-29 | closed | − "Friday 29 March 2024 (Good Friday)" | `IFEU-FI-2024` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2024-04-01 | closed | − gilt row "Closed", 1 Apr column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-05-06 | closed | − gilt row "Closed", 6 May column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-05-27 | closed | − gilt row "Closed", 27 May column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-08-26 | closed | − gilt row "Closed", 26 Aug column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-12-24 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2024` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |
| 2024-12-25 | closed | − "Wednesday 25 December 2024 (Christmas Day)" | `IFEU-FI-2024` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2024-12-26 | closed | − gilt row "Closed", 26 Dec column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table, Boxing Day in lieu; same local civil trade date |
| 2024-12-31 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2024` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |

The January and November 2024 editions print identical closure sets and gilt
rows; the November edition (`IFEU-FI-2024`) is the cited artifact and the
January edition is corroboration in the store.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | − "Wednesday 1 January 2025 (New Year's Day)" | `IFEU-FI-2025` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2025-04-18 | closed | − "Friday 18 April 2025 (Good Friday)" | `IFEU-FI-2025` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2025-04-21 | closed | − gilt row "Closed", 21 April column | `IFEU-FI-2025` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2025-05-26 | closed | − gilt row "Closed", 26 May column | `IFEU-FI-2025` | T1 | IFEU fixed-income availability table; 5 May prints "Open" and is audited normal |
| 2025-08-25 | closed | − gilt row "Closed", 25 August column | `IFEU-FI-2025` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2025-12-24 | early close | 12:15 GMT — gilt rows "Trading ceases 12:15 (GMT)" | `IFEU-XMAS-2025` | T1 | The Christmas document's Wednesday 24 December page; same London civil trade date |
| 2025-12-25 | closed | − "The Exchange will be closed" | `IFEU-FI-2025` | T1 | The edition's Christmas Day closure list, restated by the Christmas document's Thursday 25 December page; same local civil trade date |
| 2025-12-26 | closed | − exchange "OPEN … for the following contracts" only; gilts not listed | `IFEU-XMAS-2025` | T1 | The Christmas document's Friday 26 December page; gilt closure by omission from the open list, same local civil trade date |
| 2025-12-31 | early close | 12:15 GMT — gilt rows "Trading ceases 12:15 (GMT)" | `IFEU-XMAS-2025` | T1 | The Christmas document's Wednesday 31 December page; same London civil trade date |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | − "Thursday 1 January 2026 (New Year's Day)" | `IFEU-FI-2026` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2026-04-03 | closed | − "Friday 03 April 2026 (Good Friday)" | `IFEU-FI-2026` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2026-04-06 | closed | − gilt row "Closed", 6 Apr column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2026-05-04 | closed | − gilt row "Closed", 4 May column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; 1 May prints "Open" and is audited normal |
| 2026-05-25 | closed | − gilt row "Closed", 25 May column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2026-08-31 | closed | − gilt row "Closed", 31 Aug column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2026-12-24 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2026` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |
| 2026-12-25 | closed | − "Friday 25 December 2026 (Christmas Day)" | `IFEU-FI-2026` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2026-12-28 | closed | − gilt row "Closed", 28 Dec column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table, Boxing Day in lieu; same local civil trade date |
| 2026-12-31 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2026` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |

### Documents

Artifacts live in the research store under
`normal-weeks/ice-gilts-2026-10-11/` (product pages, the circular, the live
August 2026 schedule and Christmas PDFs, the T2 workbook, the notice zips)
and `holidays/raw/ice-europe-gilts-2010-2027/` (the archived schedule
editions and a second copy of the zips). Digests below re-verified against
the store bytes on 2026-10-14 UTC. The LIFFE ids resolve to the operator's
annual notice-archive zips; each zip's per-notice citations (LON3351 and
siblings) are named in the year tables above and the quoted rows were read
from the notice PDFs inside the named zip.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `LIFFE-ZIP-2010` | 2010-12-24..2011-12-30 (LON3351, LON3352) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2010.zip> | retrieved 2026-10-11 UTC | T1 | `31a87b7d07e69a40fbf89e7f13ca730286c48c2d349ec04e20a56a27d953271b` |
| `LIFFE-ZIP-2011` | 2011-12-23..2012-12-31 (LON3526, LON3530) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2011.zip> | retrieved 2026-10-11 UTC | T1 | `d1025dd65a551be278cd7d0bfb888d7b35340125624b8eeb93b960333dad71c9` |
| `LIFFE-ZIP-2012` | 2012-12-24..2013-08-26 (LON3655, LON3657) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2012.zip> | retrieved 2026-10-11 UTC | T1 | `da9498b4cdbbab44dc299f449edbd82a8d2344b45077b08c7e4bbe296b183084` |
| `LIFFE-ZIP-2013` | 2013-12-24..2014-12-31 (LON3762, LON3770) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2013.zip> | retrieved 2026-10-11 UTC | T1 | `905cfaaea0817a316af55024d45da84b0537ef5b613480297938f2b13adefb23` |
| `IFEU-FI-2021` | 2021-01-01..2021-12-31 | <https://web.archive.org/web/20211022182425id_/https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20211022182425`, retrieved 2026-10-11 UTC | T1 | `7e7b897601ed455406d6c61e6a20074e8af907915cf9550f6c29303b79fdefb8` |
| `IFEU-XMAS-2022` | 2022-12-23..2023-01-02 | <https://www.ice.com/publicdocs/futures/IFEU_Christmas_and_New_Year_Trading_Hours_2022.pdf> | retrieved 2026-10-11 UTC | T1 | `1dea6fe3b03798f3117919d707eeaca1753fd50fbf7b173626c2542105c0b950` |
| `IFEU-FI-2024` | 2024-01-01..2024-12-31 | <https://web.archive.org/web/20241203121313id_/https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20241203121313`, retrieved 2026-10-11 UTC | T1 | `9fc431d63fed0b3d4b037b411b6ce5c1f26d616942f2b33de650c25860408894` |
| `IFEU-FI-2025` | 2025-01-01..2025-12-31 | <https://web.archive.org/web/20260728113518id_/https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20260728113518`, retrieved 2026-10-11 UTC | T1 | `45d51a2c389a5d634f2c9404b930573e84b1c5c601eeaf98c7444f28d040377e` |
| `IFEU-XMAS-2025` | 2025-12-24..2025-12-31 | <https://www.ice.com/publicdocs/futures/IFEu_Christmas_New_Year_Trading_Hours_2025.pdf> | retrieved 2026-10-11 UTC | T1 | `ec7a173ea733a17c4e3fd12006bd0ea703bfce95b3f5c7e6cd8192f6a85c6cd0` |
| `IFEU-FI-2026` | 2026-01-01..2026-12-31 | <https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | retrieved 2026-10-11 UTC | T1 | `8947f6eb6659cbe3ecb8ed909562a1013c6ca788e9167c74811d4d52f5865ff6` |
| `ICE-CIRC-26-063` | the 2026-07-06 cutover | <https://www.ice.com/publicdocs/circulars/26063.pdf> | retrieved 2026-10-11 UTC | T1 | `385479b77d7c78467006909dafb69d8691edd507e94eb1c305c5b0f909d1c6f7` |
| `ICE-EU-DST-2026` | the 2026 split restated | <https://www.ice.com/publicdocs/futures/Trading_Schedule_Temporary_Trading_Hours_for_DST.pdf> | retrieved 2026-10-11 UTC | T1 | `c9ffb649c802465a7f5ceedef403140c36a3575966cd619e3d4a1526fe6d9579` |
| `ICE-GILT-PAGE-LIVE` | current Long Gilt grid | <https://www.ice.com/products/37650336/Long-Gilt-Future> | retrieved 2026-10-11 UTC | T1 | `01c6371a9e3211dd4ecb43b6d5d100250a99a9823600d5a2f247657213de481a` |
| `ICE-GILT-PAGE-20260311` | the pre-change grid | <https://web.archive.org/web/20260311095734id_/https://www.ice.com/products/37650336/Long-Gilt-Future> | Wayback `id_` replay of capture `20260311095734`, retrieved 2026-10-11 UTC | T1 | `9795400c3e148d7fea202e7ceb70c781d41eaf6df2a9c18b963b6e98c5c26759` |
| `ICE-GILT-PAGE-20260509` | the pre-change grid | <https://web.archive.org/web/20260509id_/https://www.ice.com/products/37650336> | Wayback replay of capture 2026-05-09, retrieved 2026-10-11 UTC | T1 | `59f523fff54e830a2473a8a0616dedada156849207def21b73cbe15e3969dad9` |
| `ICE-EU-XLSX-2026` | the T2 change tracker | <https://www.ice.com/publicdocs/futures/Excel_Trading_Schedule.xlsx> | retrieved 2026-10-11 UTC | T2 | `33afaf80680739bb20686af842e62f21f9af2e394abd0c49ee293a804e844d1b` |

## Sources

- <https://www.ice.com/products/37650336/Long-Gilt-Future> — the Long Gilt product page; the monitoring entry point for the family grid and its Pre-Open column.
- <https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> — the IFEU Trading Schedule: Fixed Income, Equity and Indices, republished as editions; the holiday-table monitoring entry point.
- <https://www.ice.com/publicdocs/futures/IFEu_Christmas_New_Year_Trading_Hours_2025.pdf> — the December Christmas/New Year trading-hours document series.
- <https://www.ice.com/publicdocs/circulars/26063.pdf> — ICE Futures Europe circular 26/063; the circulars index is <https://www.ice.com/futures-europe/circulars>.
- <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/> — the NYSE Liffe historical-notice zip archive (2005–2014 offered).
- <https://www.ice.com/publicdocs/futures/Excel_Trading_Schedule.xlsx> — the operator's T2 Excel trading-schedule workbook, whose Change Tracker cites the governing circular per change.
- <https://www.ice.com/holiday-hours> — the ICE holiday-hours hub that links the schedule editions above (captured byte-identical 2026-10-10 and 2026-10-11 UTC).

## Gaps and residual risks

- **2015–2020 holiday editions.** Inside no audited window. No IFEU Fixed
  Income schedule edition for those years survives in the archive, and the
  ICE-era circular archive was not enumerated. Closing condition: a sweep of
  `theice.com/publicdocs/circulars/` notice numbers or the
  `ice.com/futures-europe/circulars` service for the era's gilt holiday
  circulars. The span answers session questions as the bridged residual.
- **2010 before 24 December.** The full-year 2010 trading-days notice would
  sit in the 2009 zip, which the hunt did not pull (the 2010 zip begins at
  LON3239). Closing condition: `nyse_liffe/historical_notices/2009.zip`.
- **2022 January–November and 2023 February–December.** Inside no window: the
  2022 and 2023 Fixed Income editions are not in the captured set, so only
  the Christmas-document dates audited. Closing condition: those editions.
- **The 00:35 vs 00:45 pre-open.** The live product page prints Pre-Open
  00:35; circular 26/063 prints 00:45. The timeline serves 00:45–01:00 — the
  window both state — and the disputed 00:35–00:45 quarter-hour is withheld.
  Closing condition: a desk answer or the operator's next spec-page revision.
- **Announced eves with unpublished instants** (2014-12-24, 2014-12-31,
  2021-12-24, 2021-12-31, 2024-12-24, 2024-12-31, 2026-12-24, 2026-12-31) are
  `Unsourced` rows, never audited normal. Closing condition per year: the
  operator's Christmas circular.
- **2027** is unpublished (the 2027 edition and the Christmas-2026 circular
  both 404 live, checked 2026-10-11 UTC). The audited window ends at
  2026-12-31; no 2027 date is asserted normal.
- The live product page names Medium, Short and Ultra Long Gilt as related
  products of the Long Gilt page and the shared-grid claim rests on that page
  family plus the DST document's split table; the Short/Ultra product pages
  were not separately captured. A separate capture would strengthen the
  shared-era claim but the October 2026 temporary-trading-times document
  states the split directly.
