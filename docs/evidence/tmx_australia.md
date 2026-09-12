<!-- SPDX-License-Identifier: MIT-0 -->

# `tmx_australia` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tmx_australia.rs`](../../src/calendar/schedules/equities/apac/tmx_australia.rs)
- **Source sets:** [`APAC-TMX-AUSTRALIA`](../schedules/sources.md#apac-tmx-australia)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue envelope across auction-eligible and @Last/MOC products.

## Revision rows

- 2011-10-31 — T3 — ASIC media release 12-295MR — Chi-X Australia launch; continuous trading 10:00–16:13 with no close-side tail yet.
- 2013-12-09 — T1 — Chi-X Australia Compliance Notice 0009-13 — MOC launches after the 2013-11-25 release was rolled back; @Last/MOC trading runs 16:13–16:20.
- 2015-08-31 — T1 — Chi-X Australia Compliance Notice 0006-15 — @Last moves the close-side open to 16:12.
- 2025-03-17 — T1 — Cboe Australia Technical Notice 0003-25 — auctions launch; Pre-Open from 07:00, randomized 09:59:45–10:00 Opening Auction, and a 16:00–16:20 close-side envelope.

## Sources

- <https://www.tmxaustralia.com/about/hours> — TMX Australia hours page. It states that during the pre-market period "trade reports may be lodged in accordance with the Cboe Operating Rules and the Market Integrity Rules", which is why Pre-Open is tradeable `extended` rather than order-entry-only.
- <https://cdn.cboe.com/resources/au/tmx/participant_resources/Operating_Rules_Procedures_Clean.pdf> — Operating Rules and Procedures.
- <https://www.asic.gov.au/about-asic/news-centre/find-a-media-release/2012-releases/12-295mr-asic-releases-first-chi-x-assessment-report/> — ASIC media release 12-295MR, dating Chi-X Australia's launch to 2011-10-31.
- <https://cdn.cboe.com/resources/compliance_notice/Compliance-Notice-0008-13.pdf> — Compliance Notice 0008-13, the 2013-11-25 MOC release that was rolled back.
- <https://cdn.cboe.com/resources/compliance_notice/Compliance-Notice-0009-13.pdf> — Compliance Notice 0009-13, the actual 2013-12-09 MOC launch.
- <https://cdn.cboe.com/resources/compliance_notice/Compliance-Notice-0006-15.pdf> — Compliance Notice 0006-15, the 2015-08-31 @Last change.
- <https://cdn.cboe.com/resources/technical_notice/Technical-Notice-0003-25.pdf> — Technical Notice 0003-25, the 2025-03-17 auction launch.

## Gaps and residual risks

- The 2011-10-31 launch day rests on an ASIC assessment report, a regulator restating the venue's launch rather than a venue document, so that revision row is recorded at T3. The current schedule is T1 from the venue's own hours page and Operating Rules, which is what the ledger's Evidence tier records (LAW-PRIMARY-SOURCES). Closing condition: a dated Chi-X Australia notice stating the first trading day.
- Pre-auction eras carry no order-entry-only window: the only non-regular phases are @Last and MOC, both of which execute at the closing price.
- The close-side 16:00–16:20 window stays tradeable throughout, because non-auction-eligible products keep trading continuously to 16:13, MOC matches at the ASX closing auction price around 16:10, and the Closing Auction uncrosses 16:12:45–16:13.
