<!-- SPDX-License-Identifier: MIT-0 -->

# `krx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`krx.rs`](../../src/calendar/schedules/equities/apac/krx.rs)
- **Source sets:** [`APAC-KRX`](../schedules/sources.md#apac-krx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

KOSPI/KOSDAQ exchange default; executable pre-market block/basket trading begins 08:00 as of the 2026-08-22 review and began 07:30 before the sourced 2019-04-29 reduction. The 2016 regular-session extension is also date-aware and sourced.

## Revision rows

- 2016-08-01 — T4 — 2016-08-01 KRX regular-close extension, artifact unrecovered — the regular close extends 14:50 → 15:20. **The tier is `T4` because no artifact in this source set dates this day**; see the defect bullet below. A non-synthetic row at T4 is a defect, not a label (LAW-PRIMARY-SOURCES), and it is recorded here rather than hidden.
- 2019-04-29 — T1 — KRX rulebook law 000111 — pre-market block/basket trading moves 07:30 → 08:00, prior-close trading moves 07:30–08:30 → 08:30–08:40, and opening-call order reception moves 08:00 → 08:30.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://global.krx.co.kr/contents/GLB/06/0602/0602020204/GLB0602020204T1.jsp> — KRX cash-equity trading hours: continuous 09:00–15:20, closing call, and order/trading phases through 18:00.
- <https://global.krx.co.kr/contents/GLB/01/0107/0107010000/20170630_eng_brochure.pdf> — KRX English brochure dated 2017-06-30, the earliest dated artifact in the set.
- <https://www.fsc.go.kr/po010106/73613> — Financial Services Commission notice 73613, 「장개시전 시간외시장 및 시가단일가 시간 단축」, **dated 2019-04-03 and retrieved 2026-09-12 (UTC)**; that retrieval date is later than this row's review date and governs for this source. It states the 2019-04-29 effective day and both pre-market moves, and says nothing about the regular close — T1.
- <https://law.krx.co.kr/las/LawBon.jsp?lawid=000111> — KRX rulebook, law 000111.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2016-08-01 baseline (regular close 14:50, pre-market from 07:30) is not attested by any artifact dated at or near the January-2010 floor; the earliest dated document in the set is the 2017-06-30 brochure, which describes the post-2016 state. The ledger horizon is therefore 2016-08-01, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a dated pre-2016 KRX rulebook edition or trading-hours page, which would move the horizon earlier.
- **citation discrepancy, settled 2026-09-12 from the artifact — the *module* was the wrong record, and `sources.md` was right all along.** The notice at <https://www.fsc.go.kr/po010106/73613> was retrieved and read on 2026-09-12 (UTC). It is dated **2019-04-03**, is titled 「장개시전 시간외시장 및 시가단일가 시간 단축」 ("Shortening of the pre-opening off-hours market and the opening single-price auction"), and its body reads:
  「장개시전 시간외시장의 매매거래시간을 단축하는 거래소 업무규정이 ‘19.4.3일(수) 제6차 금융위원회 정례회의에서 의결」, 「금번 의결된 개정안은 ‘19.4.29일(월)부터 시행될 예정」, 「1) 장개시전 시간외 대량매매 (07:30∼09:00 → 08:00∼09:00)」 and 「2) 장개시전 시간외 종가매매 (07:30∼08:30 → 08:30∼08:40)」 — the amended Exchange business regulations were resolved at the FSC's 6th regular meeting on 2019-04-03 and take effect from Monday **2019-04-29**, shortening pre-market off-hours block trading from 07:30–09:00 to 08:00–09:00 and moving pre-market off-hours closing-price trading from 07:30–08:30 to 08:30–08:40.
  That is the `2019-04-29` row, to the minute. The notice says nothing about the regular close or 15:20. So `krx.rs`'s `2016-08-01` citation literal `"FSC notice 73613"` was the misattribution, `sources.md`'s original description was correct, and the "correction" made to `sources.md` earlier on 2026-09-12 was wrong and has been reverted. The 2016 row's citation label has been changed to name no document; the day, the profiles and the routing are untouched, because this PR makes no schedule change.
- **defect — the 2016-08-01 row's day is unsourced in this repository.** With FSC notice 73613 reassigned to 2019, nothing in the `APAC-KRX` source set dates the 14:50 → 15:20 regular-close extension: the KRX trading-hours page and the 2017-06-30 brochure both describe the post-2016 state without dating the change, and KRX rulebook law 000111 is cited for the 2019 row. Under LAW-PRIMARY-SOURCES a dated change needs an unconditional day at T1 or T2, so the row as it stands is a defect rather than a label, and the revision bullet carries `T4` to say so. The ledger row's `Evidence tier` cell stays `T1`, which is correct: that cell records the tier behind the row's *current* schedule, which rests on the KRX cash-equity trading-hours page. Closing condition, and it needs its own PR because it may change the schedule: recover the FSC resolution or the KRX business-regulation amendment that dates the extension — which would restore the row at T1 — or, failing that, withdraw the dated row and serve the pre- and post-2016 grids as a sourced intersection. The ledger's `Primary` Basis for this row should be re-examined in that PR.
- **the 2019-04-29 row is unaffected and is now doubly sourced.** Its citation literal stays `"KRX rulebook law 000111"`, and FSC notice 73613 corroborates the same day and both moves from the regulator's side.
- KRX profiles carry no `order_entry` window: the pre-market block/basket window is executable, so the earliest edge used is 08:00 rather than the 08:30 opening-call order reception.
