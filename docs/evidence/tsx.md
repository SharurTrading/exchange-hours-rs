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

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.tsx.com/en/trading/calendars-and-trading-hours/trading-hours> — TSX trading hours: orders accepted from 07:00, continuous trading 09:30–16:00, the conditional Market-on-Close Price Movement Extension through 16:10, and Extended Trading at the last sale price 16:15–17:00. The venue's session table describes Pre-Open as a phase in which orders may be entered but will not be executed.
- <https://www.osc.ca/sites/default/files/pdfs/bulletins/oscb_20050114_2802.pdf> — Ontario Securities Commission Bulletin of 2005-01-14, volume 28 issue 2: the regulator record establishing that both the Price Movement Extension and the last-sale session existed before the January-2010 history floor.

## Holidays

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates in `America/Toronto`; tier T1 throughout).

The operator's holiday statement is TMX Group's own "Calendar" page,
`tsx.com/en/trading/calendars-and-trading-hours/calendar` (found through the tsx.com sitemap),
one server-rendered page carrying the "2025 Stock Market Holidays - Stock Markets Closed" and
"2026 Stock Market Holidays - Stock Markets Closed" lists with a "Canadian Holidays" heading
each. TSX runs no overnight session, so an event date and its trade date are one civil day and
the conversion is the identity. The page was retrieved live on 2026-09-28 and both lists are
read from that one artifact.

**2027 is not published.** The page's newest section is 2026; TMX historically adds the next
year's calendar in Q4. Verified 2026-09-28: no 2027 list exists on the page or behind its
"Settlement Schedule" links. Nothing past 2026-12-31 is claimed; **closing condition:** the
operator's 2027 calendar section.

**The one early close is the operator's own footnote.** Each Christmas Eve entry carries `*`,
footnoted `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)`. This identity's
scope is the Toronto Stock Exchange cash-equity market, whose close is the printed 1:00 PM
TSX/TSXV instant; the 1:30 half belongs to the ALPHA/ALPHA X/DRK book systems, which are
separate trading venues outside this row. No other date in either list carries an early close,
late open or half-day marking.

**The U.S. holidays are settlement data, not closures.** The page's "U.S. Holidays" block
(MLK Day, Memorial Day, Juneteenth, Independence Day or its in-lieu, U.S. Thanksgiving) is
footnoted `** U.S. Holidays with Special Settlement for Issues trading in USD` — a settlement
arrangement for USD issues, not a TSX trading closure, so none of those dates is encoded
(LAW-SESSION-NOT-EXPIRY).

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day - Wednesday, January 1, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-02-17 | closed | `Family Day - Monday, February 17, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-04-18 | closed | `Good Friday - Friday, April 18, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-05-19 | closed | `Victoria Day - Monday, May 19, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-07-01 | closed | `Canada Day - Tuesday, July 1, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-08-04 | closed | `Civic Holiday - Monday, August 4, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-09-01 | closed | `Labour Day - Monday, September 1, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-10-13 | closed | `Thanksgiving Day - Monday, October 13, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-12-24 | early close | `Christmas Eve - Wednesday, December 24, 2025*` — `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)` | `TSX-CAL-2026-09-28` | T1 | the printed event date; the TSX/TSXV half of the footnote is the day's final close, 13:00 Toronto time |
| 2025-12-25 | closed | `Christmas Day - Thursday, December 25, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-12-26 | closed | `Boxing Day - Friday, December 26, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day - Thursday, January 1, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-02-16 | closed | `Family Day - Monday, February 16, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-04-03 | closed | `Good Friday - Friday, April 3, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-05-18 | closed | `Victoria Day - Monday, May 18, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-07-01 | closed | `Canada Day - Wednesday, July 1, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-08-03 | closed | `Civic Holiday - Monday, August 3, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-09-07 | closed | `Labour Day - Monday, September 7, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-10-12 | closed | `Thanksgiving Day - Monday, October 12, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-12-24 | early close | `Christmas Eve - Thursday, December 24, 2026*` — `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)` | `TSX-CAL-2026-09-28` | T1 | the printed event date; the TSX/TSXV half of the footnote is the day's final close, 13:00 Toronto time |
| 2026-12-25 | closed | `Christmas Day - Friday, December 25, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-12-28 | closed | `In Lieu of Boxing Day - Monday, December 28, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own in-lieu print (Boxing Day falls on the Saturday); trade date is the same civil day |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `TSX-CAL-2026-09-28` | 2025-01-01 .. 2026-12-31 | <https://www.tsx.com/en/trading/calendars-and-trading-hours/calendar> | retrieved 2026-09-28 01:06 | T1 | `983d3108683e59f3b6045ffd862e699113316a14960f54880ef33a627718e7d2` |

The one artifact keys every row: the page is the operator's own calendar and both year lists
were live on it at retrieval. The store's `holidays/raw/equities/tsx/2025-2027/` holds the
artifact with this index.

## Gaps and residual risks

- **Interpretive step, order-entry classification.** The 07:00–09:30 Pre-Open is `order_entry`: no trade can match inside it, and the first print of the day is the 09:30 Market-on-Open cross that starts continuous trading.
- **Interpretive step, conditional extension.** The Price Movement Extension rule is modelled as the venue's maximum envelope. On ordinary days and symbols the 16:00–16:10 interval is cancel-only, but when the extension fires it is the delayed Market-on-Close cross for that symbol and it prints, so the window is not order-entry-only. The separate 16:10–16:15 Post Market Cancel Session is not modelled at all.
- **Interpretive step, the Christmas Eve half.** The footnote states two closes — `1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)`. The row encodes the TSX/TSXV 13:00 close because this identity is the Toronto Stock Exchange cash-equity venue; the ALPHA/ALPHA X/DRK book systems are separate order books, not part of this row's scope, and no TSX-listed session is clipped by their later close.
- **No dated revision.** The reviewed grid holds for the whole audit window, so every instant resolves to the one profile. A sourced revision later replaces this with a real timeline row and needs no routing change. Closing condition for a future change: a TMX notice stating an unconditional day-level effective date.
- **Holiday horizon.** The audited holiday window stops at 2026-12-31 because the operator has published nothing past it (verified 2026-09-28). Closing condition: TMX's 2027 calendar section; the row is re-checked monthly per LAW-WATCH.
