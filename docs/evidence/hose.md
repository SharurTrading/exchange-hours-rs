<!-- SPDX-License-Identifier: MIT-0 -->

# `hose` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`hose.rs`](../../src/calendar/schedules/equities/apac/hose.rs)
- **Source sets:** [`APAC-HOSE`](../schedules/sources.md#apac-hose)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The archived January-2010 grid and exact 2010/2012/2013 revisions are date-aware. Each era retains its sourced put-through tail after the main order book closes.

## Revision rows

- 2010-09-13 — T1 — HOSE news notice 48784 — session moves to 08:45–10:30 with the closing call to 10:45 and negotiated trading through the 11:00 market close.
- 2012-03-05 — T1 — HOSE 2012 annual report — pilot afternoon session: continuous II ends 13:45, the closing call ends 14:00, and put-through remains available through 14:15.
- 2013-07-22 — T1 — HOSE 2013 annual report — the afternoon session is extended 45 minutes to the current grid, with put-through through 15:00.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://staticfile.hsx.vn/Uploads/UploadDocuments/2372209/2.Trading%20hours.pdf> — HOSE's current trading-hours table, explicitly printing the 13:00–15:00 put-through window.
- <https://web.archive.org/web/20140501225025id_/http://www.hsx.vn:80/hsx_en/Modules/annual/annual_files/BCTN-ANNUAL%20REPORT%202013.pdf> — HOSE 2013 annual report, dating the 45-minute extension to 2013-07-22.
- <https://staticfile.hsx.vn/Uploads/Annual/6dfe6cf6-93b2-4871-966f-2bb9bb92c110/10dd075f-c751-46d2-b598-022850e517f6> — HOSE 2012 annual report, printing the complete pilot schedule and dating its start to 2012-03-05.
- <https://web.archive.org/web/20100830155813id_/http://www.hsx.vn/hsx/Modules/News/NewsDetail.aspx?id=48784> — HOSE archived operator notice 48784, making the new grid effective 2010-09-13 and printing negotiated trading through the 11:00 market close.
- <https://staticfile.hsx.vn/Uploads/Annual/20326c45-3ba9-4fe4-89c3-fe16f9777467/10dd075f-c751-46d2-b598-022850e517f6> — HOSE 2010 annual report, independently confirming negotiation throughout the extended session from that day.
- <https://web.archive.org/web/20100215053559id_/http://www.hsx.vn:80/hsx/Uploaded/quy_dinh_file/2.Thoi%20gian%20giao%20dich..pdf> — HOSE's own trading-hours PDF, archived 2010-02-15, supplying the exact January-2010 audit-floor grid including 10:30–11:00 put-through trading.

## Gaps and residual risks

- Put-through (negotiated) trading is classified `extended` by convention; not every security is eligible for every phase.
- HOSE profiles carry no `order_entry` window: the opening and closing calls print, and the put-through tail is tradeable, so nothing is order-entry-only in any era.
- The 2010-02-15 capture is the January-2010 evidence. Nothing dates a change inside January or February 2010, so the grid is sourced through the floor rather than carried.
