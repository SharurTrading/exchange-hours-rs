<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_rough_rice` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`rough_rice.rs`](../../src/calendar/schedules/futures/us/rough_rice.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — every window in which a trade can print is dated and sourced; what is not dated is a queue phase. CBOT Rough Rice futures (`ZR`) and options (`OZR`), Rulebook chapters 17 and 17A. Current grid exact from CME's Rough Rice contract specification read 2026-09-05: RTH Monday–Friday 08:30–13:20 CT, extended Sunday–Thursday 19:00–21:00 CT with no midnight wrap, and Pre-Opens Sunday 16:00–19:00 and Monday–Thursday 16:45–19:00 CT. The key-backed calendar carries the operator's trade-date assignment from that same filing: the non-wrapping evening leg belongs to the following local date, so the 21:00–08:30 CT break is a halt inside one trade date and the daily bar ends at 13:20 CT; a detached fixed snapshot has no identity and falls back to the close-date default. The divergence is dated by CBOT Submission 18-001 to Sunday 2018-01-21 for trade date Monday 2018-01-22, which states both the outgoing Sunday–Friday 19:00–07:45 CT leg and the incoming Sunday–Thursday 19:00–21:00 CT one. Before that day Rough Rice ran on the standard CBOT grain and oilseed grid, so the six 2010–2015 revisions and their sources are inherited wholesale from the `globex_grains` chain rather than independently sourced for `ZR`; that inheritance also carries the grains row's undated 2012-05-20..2013-04-06 queue gap. **Partial for two reasons, both order-entry.** First, that inherited 21-hour-regime queue gap. Second, 18-001 is silent on the queues: CME's Rough Rice specification publishes no morning Pre-Open and no post-close Pre-Open, unlike standard grains, so this profile withdraws the inherited 08:00–08:30 and 14:30–16:00 CT windows at the 2018-01-21 boundary — the only sourced day in the interval — without a source stating that day for the queue change. Withdrawing them under-reports queueing, which is the safe direction and leaves every matching window untouched, but the withdrawal day is a modelling choice and is recorded here as such.

## Revision rows

- 2010-04-19 — T1 — CME Globex notice 20100405 — post-close Pre-Open expands to 13:15:30-16:00 CT; inherited from the CBOT grain and oilseed chain.
- 2011-12-27 — T1 — CFTC filing rul120711cbot001 — weekday morning queue moves to 08:00 CT; inherited.
- 2012-05-20 — T1 — CME market-data advisory 20120518 — 21-hour continuous session; inherited, and the era serves no queues.
- 2013-04-07 — T1 — CME SER-6617 and GCC notice 2013-03-22 — reduction to 19:00-07:45 and 08:30-13:15 CT with the standard grains' queue set; inherited.
- 2013-08-18 — T1 — CME market-data advisory 20130812 — morning Pre-Open widened to 08:00-08:30 CT; inherited.
- 2015-07-05 — T1 — CME SER-7395R — day close moves to 13:20 CT; inherited.
- 2018-01-21 — T1 — CBOT Submission 18-001 — the divergence: the evening leg stops wrapping and becomes Sunday-Thursday 19:00-21:00 CT, and the inherited morning and post-close queues are withdrawn.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-09-05, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf> — CBOT Submission 18-001 — "effective on Sunday, January 21, 2018 for trade date Monday, January 22, 2018", naming "Rough Rice Futures ZR 17" and "Rough Rice Options OZR 17A".
- <https://web.archive.org/web/20240314032026id_/https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf> — CBOT Submission 18-001, archived capture — CME serves an anti-scraping block to automated clients.
- <https://www.cmegroup.com/markets/agriculture/grains/rough-rice/specs> — CME Rough Rice contract specification, read 2026-09-05 — the current grid and both evening Pre-Opens.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html> — CME Globex notice 20100315 — the operator's March-2010 market-state table behind the inherited floor grid.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html> — CME Globex notice 20100405 — the 2010-04-19 post-close Pre-Open expansion.
- <https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf> — CFTC filing rul120711cbot001 — the 2011-12-27 morning queue move.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html> — CME market-data advisory 20120518 — the 2012-05-20 expansion.
- <https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf> — CME SER-6617 — the 2013-04-07 reduction.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html> — CME market-data advisory 20130812 — the 2013-08-18 morning Pre-Open widening.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html> — CME SER-7395R — the 2015-07-05 move to a 13:20 CT close.

## Gaps and residual risks

- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the queue withdrawal is routed at a day no source states for it.** The bullet above records that CBOT Submission 18-001 is silent on the queues; the reviewer's point is the routing consequence. `REVISIONS` in `src/calendar/schedules/futures/us/rough_rice.rs` withdraws the inherited 08:00–08:30 and 14:30–16:00 CT Pre-Opens **at** the 2018-01-21 executable-leg row, which means the crate asserts a day-level queue cutover on a day 18-001 states only for the evening leg. That is a dating claim the evidence does not carry, even though the direction of the error is safe. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; the withdrawal day is served exactly as before. Closing condition: either a CME artifact dating the `ZR` queue change — which makes the routing sourced — or a routing that keeps the queue change undated, for instance withholding both windows across the whole undated interval under the widen-only rule so no day-level queue cutover is asserted at all. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **order-entry** — the inherited 21-hour regime of 2012-05-20..2013-04-06 carries the `globex_grains` row's undated queue gap: no source states the queue state for that era, so none is served.
- **order-entry** — CBOT Submission 18-001 is silent on the queues. CME's Rough Rice specification publishes no morning Pre-Open and no post-close Pre-Open, so the profile withdraws the inherited 08:00-08:30 and 14:30-16:00 CT windows at the 2018-01-21 boundary — the only sourced day in the interval — without a source stating that day for the queue change. Withdrawing them under-reports queueing, which is the safe direction and leaves every matching window untouched, but the withdrawal day is a modelling choice and is recorded as such. Closing condition: a CME artifact dating the queue change for `ZR`. Dormant identity, so recorded here rather than opened as an issue.
- **inherited, not independently sourced** — Rough Rice's own queue and RTH/ETH split were not sourced separately for the 2010, 2012, 2013 and 2015 eras; those are inherited from the reviewed grains encoding. The CBOT grain and oilseed reorganisations name Rough Rice in their product lists, and 18-001 itself confirms the outgoing state by quoting `ZR`'s then-current Sunday-Friday 19:00-07:45 CT extended leg.
- **horizon** — the inherited floor grid rests on the operator's March-2010 market-state table (CME Globex notice 20100315), so the baseline is sourced from 2010-03-15 and carried below that to the January-2010 floor.
- **trade-date assignment** — the key-backed calendar carries the operator's trade-date assignment from 18-001: the non-wrapping evening leg belongs to the following local date, so the 21:00-08:30 CT break is a halt inside one trade date and the daily bar ends at 13:20 CT. A detached fixed snapshot has no identity and falls back to the close-date default.

## Module narrative (moved from src/calendar/schedules/futures/us/rough_rice.rs on 2026-09-12 UTC)

Rough Rice futures (CME Globex `ZR`) and Rough Rice options (`OZR`), CBOT
Rulebook chapters 17 and 17A, in America/Chicago.

INHERITED PRE-2018 ERAS — NOT INDEPENDENTLY SOURCED FOR ROUGH RICE.
Rough Rice ran on the standard CBOT grain and oilseed grid from the
January-2010 audit floor until it diverged permanently on 2018-01-21, so the
six eras before that day reuse the `CBOT_*` rule tables in `grains.rs` and
the primary sources quoted there (2010-04-19, 2011-12-27, 2012-05-20,
2013-04-07, 2013-08-18, 2015-07-05). The CBOT grain and oilseed
reorganisations name Rough Rice in their product lists, and 18-001 itself
confirms the outgoing state by quoting ZR's then-current Sunday-Friday
19:00-07:45 CT extended leg — the grid `grains.rs` already serves for that
day. What this crate has *not* done is source Rough Rice's own queue and
RTH/ETH split separately for the 2010, 2012, 2013 and 2015 eras: those are
inherited from the reviewed grains encoding, including its own undated
2012-05-20..2013-04-06 queue gap and its 08:00-08:30 CT morning Pre-Open.
A later Rough Rice-specific finding repoints one era below; it must not
edit the grains tables.

2018-01-21 — THE DIVERGENCE. CBOT Submission 18-001 certifies "the reduction
of the extended trading hours on the CME Globex electronic trading platform
for the Rough Rice Futures and Rough Rice Options contracts (the
\"Contracts\") effective on Sunday, January 21, 2018 for trade date Monday,
January 22, 2018", naming "Rough Rice Futures ZR 17" and "Rough Rice Options
OZR 17A". Its table, headed "Current Extended Trading Hours" against
"Extended Trading Hours Effective on Trade Date January 22, 2018", moves the
leg from "Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT" to "Sunday - Thursday,
7:00 p.m. - 9:00 p.m. CT". This is an unconditional day-level effective date
keyed to the venue-local opening Sunday, and the operator's own column
heading classifies the evening leg as extended. Regular trading hours are
untouched, so the 08:30-13:20 CT session carries through from SER-7395R.
From this row Rough Rice has no midnight-wrapping session at all.
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf
Archived, CME serves an anti-scraping block to automated clients:
https://web.archive.org/web/20240314032026id_/https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf

CURRENT GRID. CME's Rough Rice contract specification, read 2026-09-05,
states under TRADING HOURS: "CME Globex: Sunday - Thursday, 7:00 p.m. - 9:00
p.m. CT / Pre-Open Sunday: 4:00 p.m. - 7:00 p.m. CT // Monday - Friday: 8:30
a.m. - 1:20 p.m. CT / Pre-Open Monday - Thursday: 4:45 p.m. - 7:00 p.m. CT",
with PRODUCT CODE "CME Globex: ZR" and EXCHANGE RULEBOOK "CBOT 17". It
therefore sources every session time this profile serves at the 2026-09-05 review, and confirms
18-001's grid is still the live one.
https://www.cmegroup.com/markets/agriculture/grains/rough-rice/specs

NO MORNING PRE-OPEN AND NO POST-CLOSE PRE-OPEN ARE MODELLED FROM 2018-01-21.
Standard grains carry an 08:00-08:30 CT morning Pre-Open and a 14:30-16:00 CT
post-close Pre-Open; CME's Rough Rice specification publishes neither, and no
other primary source retrieved states either one for `ZR`. This profile
therefore claims only the two evening queues the specification does publish,
and drops the inherited grains morning Pre-Open and PCP at the divergence.
Two things about that are worth stating plainly rather than hiding behind the
basis label. First, omitting an order-entry window under-reports queueing,
which is the safe direction: no window in which a trade can print is
affected, so under the executable-windows-first rule this is the cheap error
to make. Second, 18-001 is silent on the queues, so the day on which the
morning Pre-Open and PCP stopped applying to Rough Rice is *not* sourced;
attaching their withdrawal to the one sourced day in the interval is a
modelling choice, not evidence, and it is why the ledger row is `Partial`
rather than `Primary`.

---

The divergence. 18-001 dates ONE thing: the executable evening leg, which
stops wrapping and becomes 19:00-21:00 CT. It is silent on the queues, so
nothing here claims the queues changed on 2018-01-21 and no queue cutover is
asserted anywhere in this timeline.

What the queue set records instead is the sourced intersection over the
interval that begins here. Two queue states are sourced for this contract:
the standard grain set this profile inherits for the earlier eras, and the
two evening pre-opens CME's Rough Rice specification published at the 2026-09-05 review. No
document dates a move between them, so the interval serves the set that
holds under both -- the narrower one -- exactly as the crate's
sourced-intersection convention requires.

The alternative, carrying the inherited 08:00-08:30 and 14:30-16:00 CT
windows forward, would make the dated surface report order acceptance at the 2026-09-05 review
that CME's current specification does not publish: an over-report of
queueing, and in the present tense. Serving the intersection under-reports
queueing for whatever part of the interval still had the wider set. That is
the safe direction and it touches no window in which a trade can print.

---

Revision evidence — each row's day-level effective date and the primary
source that states it. The first six are the CBOT grain and oilseed chain
Rough Rice shared; their full quotations sit in `grains.rs`.
  2010-04-19 "CME Globex notice 20100405"
    https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
  2011-12-27 "CFTC filing rul120711cbot001"
    https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
  2012-05-20 "CME market-data advisory 20120518"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
  2013-04-07 "CME SER-6617 and GCC notice 2013-03-22"
    https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
  2013-08-18 "CME market-data advisory 20130812"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
  2015-07-05 "CME SER-7395R"
    https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
  2018-01-21 "CBOT Submission 18-001"
    https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2018/01/18-001.pdf
Evidence: docs/evidence/globex_rough_rice.md
