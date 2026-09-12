<!-- SPDX-License-Identifier: MIT-0 -->

# `krx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`krx.rs`](../../src/calendar/schedules/equities/apac/krx.rs)
- **Source sets:** [`APAC-KRX`](../schedules/sources.md#apac-krx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

KOSPI/KOSDAQ exchange default; executable pre-market block/basket trading begins 08:00 as of the 2026-08-22 review and began 07:30 before the sourced 2019-04-29 reduction. The 2016 regular-session extension is also date-aware and sourced.

## Revision rows

- 2016-08-01 — T1 — FSC notice 73613 — the regular close extends 14:50 → 15:20.
- 2019-04-29 — T1 — KRX rulebook law 000111 — pre-market block/basket trading moves 07:30 → 08:00, prior-close trading moves 07:30–08:30 → 08:30–08:40, and opening-call order reception moves 08:00 → 08:30.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://global.krx.co.kr/contents/GLB/06/0602/0602020204/GLB0602020204T1.jsp> — KRX cash-equity trading hours: continuous 09:00–15:20, closing call, and order/trading phases through 18:00.
- <https://global.krx.co.kr/contents/GLB/01/0107/0107010000/20170630_eng_brochure.pdf> — KRX English brochure dated 2017-06-30, the earliest dated artifact in the set.
- <https://www.fsc.go.kr/po010106/73613> — Financial Services Commission notice 73613.
- <https://law.krx.co.kr/las/LawBon.jsp?lawid=000111> — KRX rulebook, law 000111.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2016-08-01 baseline (regular close 14:50, pre-market from 07:30) is not attested by any artifact dated at or near the January-2010 floor; the earliest dated document in the set is the 2017-06-30 brochure, which describes the post-2016 state. The ledger horizon is therefore 2016-08-01, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a dated pre-2016 KRX rulebook edition or trading-hours page, which would move the horizon earlier.
- **citation discrepancy, resolved 2026-09-12 — `sources.md` was the wrong record and was corrected.** The `2016-08-01` revision row's citation literal is `"FSC notice 73613"` and the `2019-04-29` row's is `"KRX rulebook law 000111"`; those literals are the crate's own timeline data in `krx.rs`, they are what the `schedule_documentation` revision-row fences compare against, and the pre-reshape module comment grouped all three URLs above `KRX_PROFILE_POST_2016_08_01` with the same assignment. `sources.md` contradicted that on two counts — it called the FSC notice "the Financial Services Commission's 2019 pre-market reduction" and it called the 2017-06-30 brochure "the exact 2016 brochure". The artifact assignment that stands is the module's: the regulator's notice dates the 2016-08-01 regular-close extension, the KRX rulebook dates the 2019-04-29 pre-market reduction, and the brochure is dated 2017-06-30 and describes the post-2016 state, so it corroborates rather than dates either row. `sources.md` was fixed rather than this file or the timeline, because correcting the row would have been a schedule change and this PR makes none.
- KRX profiles carry no `order_entry` window: the pre-market block/basket window is executable, so the earliest edge used is 08:00 rather than the 08:30 opening-call order reception.
