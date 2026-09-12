<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_cryptocurrency` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cryptocurrency.rs`](../../src/calendar/schedules/futures/us/cryptocurrency.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CME non-spot-quoted cryptocurrency futures. The exact current 24/7 phases, 2026-05-29 transition, multi-day bounds, weekly close, and following-open-business-day convention are retained, as are the three one-day Saturday maintenance extensions CME's Globex notices state for this family's channels 326/327 — 2026-08-01 to 09:00 CT (notice 20260727), 2026-08-29 to 06:00 and 2026-09-19 to 08:00 CT (notice 20260824, restated by 20260831) — each without a replacement Pre-Open and each reverting to the 02:00–04:00 standard window; the September row is forward-dated on the operator's statement, and the two later rows were added on 2026-09-06 (#61). The 2017–2026 matching grid is exact, but primary evidence does not date the five-day era's Sunday/weekday Pre-Open onset, so dated history omits those queues. The 2026-08-31 review confirmed this at the source: the bitcoin contract specification captured 2017-12-14 — carrying the launch statement for trade date 2017-12-18 — and the 2017-12-22 and 2018-01-04 captures publish the Globex matching grid only and state no Pre-Open. Later member-product listings remain catalog data.

## Revision rows

- 2017-12-17 — T1 — CME SER-8051R — five-day launch grid, 17:00–16:00 CT.
- 2026-05-29 — T1 — CME filing 26-114 — one-day bridge into the 24/7 grid.
- 2026-05-30 — T1 — CME filing 26-114 — permanent 24/7 normal week.
- 2026-08-01 — T1 — CME Globex notice 20260727 — Saturday reopen 09:00 CT.
- 2026-08-02 — T1 — CME Globex notice 20260727 — revert to the standard window.
- 2026-08-29 — T1 — CME Globex notice 20260824 — Saturday reopen 06:00 CT.
- 2026-08-30 — T1 — CME Globex notice 20260824 — revert to the standard window.
- 2026-09-19 — T1 — CME Globex notice 20260824 — Saturday reopen 08:00 CT.
- 2026-09-20 — T1 — CME Globex notice 20260824 — revert to the standard window.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-09-06, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html> — CME SER-8051R, bitcoin futures launch, the 2017-12-17 revision's source.
- <https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf> — CME rule filing 17-417, the launch filing carrying the original 17:00–16:00 CT weekday grid.
- <https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html> — CME bitcoin contract specification — capture 2017-12-14, carrying the launch statement and publishing the matching grid only.
- <https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf> — CME clearing advisory Chadv21-028, the ETH launch.
- <https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html> — CME Globex notice 20210426, the MBT launch.
- <https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html> — CME Globex notice 20211129, the MET launch.
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf> — CME rule filing 26-114, 24/7 cryptocurrency trading, the 2026-05-29 and 2026-05-30 revisions' source.
- <https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html> — CME Globex notice 20260525, corroborating the 24/7 transition.
- <https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html> — CME cryptocurrency futures FAQ, current grid.
- <https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html> — CME Globex notice 20260727, the 2026-08-01 Saturday extension.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html> — CME Globex notice 20260824, the 2026-08-29 and 2026-09-19 Saturday extensions.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html> — CME Globex notice 20260831, restating notice 20260824.

Official origin of the specification capture: <http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html>.

## Gaps and residual risks

- **order-entry** — the five-day era's Sunday and weekday Pre-Open onset is undated; the 2017-12-14, 2017-12-22 and 2018-01-04 contract-specification captures publish the matching grid only. Closing condition: a CME artifact that states the Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **forward-dated row** — the 2026-09-19 Saturday extension is encoded ahead of its effective day on CME Globex notice 20260824, restated by notice 20260831. It must be confirmed against the operator before that day (LAW-WATCH).
- **scope** — ETH, MBT and MET joined this already-live family in 2021 and later member-product listings remain catalog data; their individual launch dates are not family-clock revisions.

## Module narrative (moved from src/calendar/schedules/futures/us/cryptocurrency.rs on 2026-09-12 UTC)

2026-08-31 five-day-era Pre-Open review — confirmed knowledge-bound. The CME
bitcoin contract specification captured 2017-12-14, which itself carries the
launch statement "Effective Sunday 17 December 2017 for trade date Monday 18
December 2017 ... CME will launch Bitcoin Futures", publishes only the
matching grid: "CME Globex: Sunday - Friday 6:00 p.m. - 5:00 p.m. (5:00 p.m.
- 4:00 p.m. CT) with a 60-minute break each day beginning at 5:00 p.m. (4:00
p.m. CT)". It states no Pre-Open, and neither do the 2017-12-22 or
2018-01-04 captures. The five-day era's Sunday/weekday Pre-Open onset is
therefore undated at the source, not merely unsearched. Official origin
http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html
delivered via
https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html

Bitcoin futures opened Sunday 2017-12-17 at 17:00 CT for trade date Monday
2017-12-18. The launch filing gives the original 17:00-16:00 weekday grid.
ETH, MBT, and MET joined this already-live family in 2021; their individual
launch dates are intentionally not family-clock revisions.

CME filing 26-114 changed all non-spot-quoted cryptocurrency futures to
24/7 Globex trading effective Friday 2026-05-29: matching maintenance is
16:00-16:02 CT Monday-Friday with Pre-Open from 16:01, and 02:00-04:00 CT
Saturday with Pre-Open from 03:45. Three one-day Globex notices then
temporarily extended the Saturday window for the 24/7 markets — 2026-08-01
through 09:00 CT (notice 20260727), 2026-08-29 through 06:00 CT and
2026-09-19 through 08:00 CT (notice 20260824, restated by 20260831) — each
without publishing a replacement Pre-Open, each followed by the standard
02:00-04:00 window. The notices' tables name this family's channels, "CME
Crypto Futures | 74 | 326" and "CME Crypto Options | 327", alongside the
event-contract channels. The September row is forward-dated on the
operator's statement.

`SessionRule` spans at most one local midnight, so the multi-day weekend
session is stored in adjacent pieces. The key-backed calendar joins those
storage-only pieces at query time, while retaining the 02:00-03:45 Saturday
closed break and 03:45-04:00 Pre-Open. Both weekend blocks carry the following
open business date: normally Monday, or Tuesday when a caller policy closes
Monday. The corresponding daily bar runs from Friday 16:01 Pre-Open through
that business date's 16:00 close.
https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html
https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf
https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf
https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html
https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf
https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html
https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html
https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html
https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html
https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html
