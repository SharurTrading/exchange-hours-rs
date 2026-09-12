<!-- SPDX-License-Identifier: MIT-0 -->

# `jse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`jse.rs`](../../src/calendar/schedules/equities/africa_middle_east/jse.rs)
- **Source sets:** [`AFRICA-JSE`](../schedules/sources.md#africa-jse)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Current v4.09 and the 2026-08-17 release were checked; main/liquid ZA01 only, with product-specific ZA03 excluded.

## Revision rows

- 2012-07-02 — T1 — JSE notice 20120525-049C — the opening auction moves from 08:35 to 08:30.
- 2013-11-11 — T1 — JSE notice 2013_158B — the Closing Price Cross session is introduced at 17:05–17:10.
- 2016-09-26 — T1 — JSE notice 461A — the EOD/GDX auction is added, extending the tail to 17:15.
- 2020-08-24 — T1 — JSE Service Hotline 18520 — the EOD auction is disabled at the trading and information system upgrade cutover, leaving the Closing Price Cross alone.
- 2021-02-01 — T1 — JSE Service Hotline 28220 — the Closing Price Cross session is extended to start at 17:01.
- 2021-02-15 — T1 — JSE Service Hotline 03721 — the Closing Price Cross session start moves to 17:02, the current grid.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://clientportal.jse.co.za/technical-library/trading-and-market-data-documentation> — JSE trading and market-data technical library, the stable source-set entry point.
- <https://clientportal.jse.co.za/Content/JSE%20Contract%20Specification%20Items/Volume%2000E%20-%20Trading%20and%20Information%20Overview%20for%20Equity%20Market%20v4.09.pdf> — JSE Volume 00E, Trading and Information Overview for the Equity Market, v4.09: current ZA01 — opening auction 08:30–09:00, continuous 09:00–16:50, closing auction 16:50–17:00, publication gap, Closing Price Cross 17:02–17:10.
- <https://clientportal.jse.co.za/Content/JSE%20Contract%20Specification%20Items/JSE%20Trading%20Session%20Times%20May%202026.xls> — JSE trading session times workbook, May 2026. Volume 00E v4.09 and this workbook were published with Release 7.8 on 2026-08-17; the release changed no ZA01 session boundary.
- <https://clientportal.jse.co.za/Content/JSENoticesandCircularsItems/20120525-049C.pdf> — JSE notice 20120525-049C, the 2012-07-02 opening-auction change.
- <https://clientportal.jse.co.za/Content/JSENoticesandCircularsItems/2013_158B.pdf> — JSE notice 2013_158B, the 2013-11-11 Closing Price Cross introduction.
- <https://clientportal.jse.co.za/Content/JSENoticesandCircularsItems/461A.pdf> — JSE notice 461A, the 2016-09-26 EOD auction.
- <https://clientportal.jse.co.za/Content/JSEHotlinesItems/JSE%20Service%20Hotline%2018520%20EDM%2C%20EQM%20and%20FXM%20-%20JSE%20Trading%20and%20Information%20System%20Upgrade%20-%20Final%20Go%20Live%20Cutover.pdf> — JSE Service Hotline 18520, the 2020-08-24 cutover that disabled the EOD auction.
- <https://clientportal.jse.co.za/Content/JSEHotlinesItems/JSE%20Service%20Hotline%2028220%20EQM%20-%20JSE%20Closing%20Price%20Cross%20%28CPX%29%20Session%20Extension.pdf> — JSE Service Hotline 28220, the 2021-02-01 extension.
- <https://clientportal.jse.co.za/Content/JSEHotlinesItems/JSE%20Service%20Hotline%2003721%20EQM%20-%20JSE%20Closing%20Price%20Cross%20%28CPX%29%20Session%20Extension.pdf> — JSE Service Hotline 03721, the 2021-02-15 extension.
- <https://www.jse.co.za/media/document/market-regulation/equities-directives> — JSE Equities Directives.
- <https://www.jse.co.za/regulation/market-regulation> — JSE Market Regulation hub.
- <https://clientportal.jse.co.za/communication/jse-service-hotlines> — JSE service hotlines, the monitoring entry point.

## Gaps and residual risks

- **Scope.** The profile covers the main and liquid ZA01 equity segment only. ZA03's midday auction is product-specific and excluded, and the EOD/GDX auction remains disabled.
- **Horizon.** The 08:35 opening-auction baseline below 2012-07-02 is attested only by the state that notice 20120525-049C replaces; no separate artifact for the pre-2012 grid is indexed. The horizon is recorded as that notice's own date, 2012-05-25, read from its document identifier, and everything below it is carried. Closing condition: a JSE notice, directive or Volume 00E edition dated at or before the January-2010 floor that prints the 08:35 grid. If the notice turns out not to restate the prior grid, the horizon must move up to 2012-07-02 and the interval below it recorded as unmodelled.
- **Interpretive step, empty order-entry slice.** No ZA01 phase is order-entry-only. The opening and closing auction call sessions each uncross into a printed auction trade, and the Closing Price Cross and EOD tails are crossing sessions that print at the closing price, so `order_entry` stays empty on every profile.
- **Source access.** The JSE technical library and notice archive are served from `clientportal.jse.co.za`. Where a document is reachable only behind the portal's authentication it is admissible at T2 as the operator's own channel, and the retrieved artifact must be saved in the research store with its retrieval date (LAW-PUBLIC-SOURCES). The direct document URLs above resolved without authentication at the last review.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
