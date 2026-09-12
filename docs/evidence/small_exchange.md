<!-- SPDX-License-Identifier: MIT-0 -->

# `small_exchange` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`small_exchange.rs`](../../src/calendar/schedules/futures/us/small_exchange.rs)
- **Source sets:** [`US-SMALL-EXCHANGE`](../schedules/sources.md#us-small-exchange)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so the disputed hours are withheld. Closed before trade date 2020-05-18, when the three launch certifications listed SM75, SPRE and SFX and trades executed; the public opening on 2020-06-01 came after. 07:00–16:00 CT Monday–Friday with Pre-Open from 06:30 is stated from the launch-era market-info page through the info hub's 2024-11-03 capture. The S5C certification dated 2024-11-21 states the Exchange's hours as 08:30–15:00, and the 2025 info hub adds Pre-Open from 08:00; no source dates the move inside 2024-11-04..2024-11-21, so the new grid, which lies inside the old one, is served from 2024-11-04 and only 07:00–08:30 and 15:00–16:00 are withheld in that window. Closed from 2025-03-24: SMFE 2025-001 delisted every contract at the 2025-03-21 close, the CFTC records the last trade on 2025-01-10, and the venue, now Kraken Derivatives Exchange, is dormant under CFTC relief. Expiration-day 15:00 closes are date exceptions.

## Revision rows

- 2020-05-18 — T1 — SMFE 2020-003..005 launch certifications — launch grid 07:00–16:00 CT Monday–Friday with Pre-Open quoting from 06:30.
- 2024-11-04 — T1 — SMFE info hub, last 07:00-16:00 capture 2024-11-03, keyed to the Monday; SMFE 2024-010 — narrower 08:30–15:00 CT grid with Pre-Open from 08:00, served as the sourced intersection from the Monday after the old grid's last capture.
- 2025-03-24 — T1 — SMFE 2025-001 delisting — every contract delisted at the 2025-03-21 close, so the venue accepts no orders and the identity is closed.

## Sources

Row review: 2026-09-11 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cftc.gov/filings/ptc/ptc051120smfedcm006.pdf> — SMFE launch certification (SM75, SPRE, SFX) for trade date 2020-05-18.
- <https://public.data.smallexchange.com/ipf/20200518/products-2020-05-18.csv> — the operator's own instrument file for trade date 2020-05-18, whose `TRADING_HOURS` field reads `td=12345;...;0=p06300659n06590700r07001600`.
- <https://smallexchange-com.cdn.prismic.io/smallexchange-com/ebb1c3a8-4072-4f4e-88d5-122554f04687_MN-2020-106+Trade+Cancellation.pdf> — SMFE member notice MN-2020-106, trade cancellation on the launch trade date.
- <https://web.archive.org/web/20200612014112/https://smallexchange.com/page-data/market-info-page/page-data.json> — launch-era market-info page stating 07:00–16:00 CT with Pre-Open from 06:30.
- <https://web.archive.org/web/20241103095322/https://smallexchange.com/reference/info-hub/> — info hub, last capture of the 07:00–16:00 grid, 2024-11-03.
- <https://www.cftc.gov/filings/ptc/ptc1121249243.pdf> — SMFE S5C certification dated 2024-11-21, listing S5C for trade date 2024-11-25 and stating Exchange hours of 08:30–15:00 CT.
- <https://web.archive.org/web/20250119163716/https://smallexchange.com/reference/info-hub/> — 2025 info hub adding Pre-Open quoting from 08:00.
- <https://www.cftc.gov/sites/default/files/filings/orgrules/25/03/rules03212518130.pdf> — SMFE 2025-001, delisting every contract as of the close of business 2025-03-21.
- <https://www.cftc.gov/csl/26-21/download> — CFTC record showing the last trade on 2025-01-10 and the venue dormant under CFTC relief.

## Gaps and residual risks

- **executable** — no source dates the move off the 07:00–16:00 grid inside 2024-11-04..2024-11-21. The narrower S5C grid lies wholly inside the old one, so it is served from 2024-11-04 as the sourced intersection and only 07:00–08:30 and 15:00–16:00 are withheld in that window. Closing condition: an SMFE artifact that states the new hours on a day-level effective date. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- Expiration-day 15:00 closes are date exceptions, not schedule rows (LAW-HOLIDAY-SCOPE, LAW-SESSION-NOT-EXPIRY).
- A relisting would need its own dated source before the closed state is lifted.
- **No holiday table, and that is a service-tier fact rather than a gap (LAW-SERVICE-TIERS, LAW-HOLIDAY-SCOPE).** The Wave 8 venue retrieval (2026-09-12 UTC) established that Small Exchange publishes no 2026 or 2027 holiday calendar because the DCM is dormant. The venue, renamed Kraken Derivatives Exchange Inc., has listed nothing since SMFE 2025-001 delisted every contract at the 2025-03-21 close, so the crate's profile is `CLOSED` from 2025-03-24 and a holiday row could only shorten a day that already has no session. `smallexchange.com` still serves a home page (last published 2025-11-15, contact `dcm@kraken.com`) but every subpage tried — `/reference/info-hub`, `/documents`, `/reference/regulation`, `/futures-markets`, `/regulation-page` — returns 403 AccessDenied, and nothing on the site links a trading-hours or holiday page. The basis for the dormancy itself is **T3**, the regulator rather than the operator: CFTC press release 9272-26 of 2026-07-24 records that the Division of Market Oversight “issued a no-action letter to Kraken Derivatives Exchange Inc., formerly Small Exchange Inc., a designated contract market, which addresses certain procedures related to dormancy”, extending CFTC Letter No. 25-46 (<https://www.cftc.gov/PressRoom/PressReleases/9272-26>, retrieved 2026-09-12 04:53 UTC, sha256 `9706e3b2e457047956c2a7b74eba979400cb7803ccfb83fa78ac41a98909c028`). A T3 source never keys a row and none is keyed here: `table_for` answers `None` for `Exchange::Smfe`. Closing condition: an operator statement from Kraken Derivatives Exchange when it relists products, at which point the identity needs a normal-week review before a holiday table.
