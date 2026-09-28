<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_securities` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`sgx.rs`](../../src/calendar/schedules/equities/apac/sgx.rs)
- **Source sets:** [`APAC-SGX-SECURITIES`](../schedules/sources.md#apac-sgx-securities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SGX-ST normal week; sourced 2011/2017/2019 phases.

## Revision rows

- 2011-08-01 — T1 — SGX-ST Rules 2011-08-01 — continuous all-day trading 09:00–17:00; the midday break is removed.
- 2017-11-13 — T1 — SGX announcement 2017-07-18 — the midday break returns: regular 09:00–12:00 and 13:00–17:00 with a 12:00–13:00 routine.
- 2019-06-03 — T1 — SGX announcement 2019-05-14 — Trade at Close extends the closing tail to 17:16.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles> — SGX-ST Regulatory Notice 8.2.1. Every routine has a Pre-Open/Pre-Close Phase that "allows order entry, order modification and withdrawal of orders but no matching of orders", and a Non-Cancel Phase in which "all existing orders that can be matched are matched at a single price".
- <https://rulebook.sgx.com/sites/default/files/net_file_store/SGX_ST_Rules_August_1_2011.pdf> — SGX-ST Rules as at 2011-08-01, introducing continuous all-day trading. Its Practice Note 8.2.1 carries the pre-2017 routine boundaries used by the two oldest profiles: Pre-Open 08:30–08:59 / Non-Cancel 08:59–09:00, lunch-break Adjust 12:30–13:59 with no matching and its 13:59–14:00 match, and Pre-Close 17:00–17:05 / Non-Cancel 17:05–17:06.
- <https://links.sgx.com/1.0.0/corporate-announcements/AYXNAX3DG8RCFZT7/20170718_SGX_to_adjust_equities_market_structure_after_supportive_feedback.pdf> — SGX announcement of 2017-07-18, restoring the midday break from 2017-11-13.
- <https://links.sgx.com/1.0.0/corporate-announcements/46OQY4VBYIHO4ARN/20190514_SGX_to_launch_securities_market_trade_at_close_session_on_3_June.pdf> — SGX announcement of 2019-05-14, launching Trade at Close on 2019-06-03.
- <https://www.sgx.com/stock-exchange/trading> — the operator's own securities-market trading page (this page's own holiday and half-day statements), read 2026-09-28 (UTC) as bytes from SGX's content API: `holidays/raw/equities/sgx_securities/2025-2027/api2_content-api_stock-exchange_trading.json` (research store), sha256 `45dbdc61…`. The live page is an SPA shell to non-JS clients (`live_sgx_trading_hours_holidays.html`, sha256 `d87ad3e0…`); the JSON above is the feed the page itself renders.
- <https://www.mom.gov.sg/employment-practices/public-holidays> — the Singapore holiday calendar the operator designates (page last updated 19 June 2026), retrieved 2026-09-28 (UTC) as `holidays/raw/equities/sgx_securities/2025-2027/live_mom_public_holidays.html`, sha256 `a4f175a7…`; it prints the gazetted 2025, 2026 and 2027 dates.

## Holidays

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates, the operator's published horizon). Tier: T2 throughout.

The live `www.sgx.com` holiday pages render no server-side content, so the
operator's statements were read as bytes from SGX's own content API
(`api2.sgx.com/content-api`), the CMS feed the page renders, for
`/stock-exchange/trading` — the securities-market page. That artifact carries
three load-bearing statements in the operator's own words:

1. **The closure calendar is designated.** `SGX follows the Singapore holiday
   calendar available on the Ministry of Manpower website.` — so a gazetted
   Singapore holiday falling on an SGX-ST trading day is an SGX-ST closure,
   and MOM's printed gazetted list (saved beside the operator artifact) is
   the calendar the operator names.
2. **The half-day phase grid.** On half days the operator prints Opening
   Routine as usual, `Trading Open: 09:00am - 12:00pm`, Closing Routine
   `Pre-Close: 12:00pm – 12:04pm/12:05pm*`, `Non-Cancel: 12:04pm/12:05pm* –
   12:06pm`, `Trade at Close: 12:06pm – 12:16pm`, `Close: 12:16pm` — the same
   phase structure as a full day compressed to a 12:16 close.
3. **The half-day dates.** `The Eve of Chinese New Year, Eve of Christmas and
   Eve of New Year in 2025 & 2026 fall on business days. As such, there will
   be half-day trading in 2025 & 2026.` with the six dates printed in the
   page's own table.

**Why the window ends at 2026-12-31.** The operator's own sheet is printed for
2025 & 2026 only. MOM has gazetted 2027, but the operator has not stated the
2027 treatment of the Chinese New Year, Christmas and New Year eves — whether
any of them is a half day is exactly the fact statement 3 exists to make — so
a 2027 date without a row could not be audited normal and the window cannot
honestly extend. **Closing condition:** SGX's next annual securities schedule
naming the 2027 half days, at which point the window extends. Re-checked
monthly per LAW-WATCH.

**Tier.** The rows key at T2 because the artifact behind every row is the
operator's own machine channel read as bytes (LAW-PRIMARY-SOURCES), which
carries both the designation sentence and the half-day schedule. The closure
*dates* are printed on the designated MOM page, the government's own gazetted
list; no member firm, vendor or press restatement touches any row.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | MOM: `1 January 2025 Wednesday New Year's Day`; SGX designates the MOM calendar | `SGX-ST-SCHED` | T2 | the operator's designation resolved against MOM's printed gazetted date |
| 2025-01-28 | replacement blocks | SGX: `28 Jan 2025 Tuesday Eve of Chinese New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim; the row restates the day's complete session structure as a replacement-block set keyed to the operator's printed half-day close |
| 2025-01-29 | closed | MOM: `29 January 2025 Wednesday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-01-30 | closed | MOM: `30 January 2025 Thursday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-03-31 | closed | MOM: `31 March 2025 Monday Hari Raya Puasa` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-04-18 | closed | MOM: `18 April 2025 Friday Good Friday` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-05-01 | closed | MOM: `1 May 2025 Thursday Labour Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-05-12 | closed | MOM: `12 May 2025 Monday Vesak Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-10-20 | closed | MOM: `20 October 2025 Monday Deepavali` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-12-24 | replacement blocks | SGX: `24 Dec 2025 Wednesday Eve of Christmas`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |
| 2025-12-25 | closed | MOM: `25 December 2025 Thursday Christmas Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-12-31 | replacement blocks | SGX: `31 Dec 2025 Wednesday Eve of New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |

The 2025 gazetted holidays that fall on a Saturday — Polling Day 3 May, Hari
Raya Haji 7 June, National Day 9 August — print no row: they are outside the
Mon-Fri trading week, and the operator's designation closes no weekday for
them. MOM's own Sunday-substitution sentences never arise in 2025.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | MOM: `1 January 2026 Thursday New Year's Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-02-16 | replacement blocks | SGX: `16 Feb 2026 Monday Eve of Chinese New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |
| 2026-02-17 | closed | MOM: `17 February 2026 Tuesday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-02-18 | closed | MOM: `18 February 2026 Wednesday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-04-03 | closed | MOM: `3 April 2026 Friday Good Friday` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-05-01 | closed | MOM: `1 May 2026 Friday Labour Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-05-27 | closed | MOM: `27 May 2026 Wednesday Hari Raya Haji` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-06-01 | closed | MOM: `31 May 2026 Sunday Vesak Day` — `Monday, 1 June 2026, will be a public holiday if your rest day falls on 31 May 2026.` | `SGX-ST-SCHED` | T2 | MOM's own Sunday-substitution sentence moves the holiday to the Monday the exchange does not trade |
| 2026-08-10 | closed | MOM: `9 August 2026 Sunday National Day` — `Monday, 10 August 2026, will be a public holiday …` | `SGX-ST-SCHED` | T2 | MOM's Sunday-substitution sentence |
| 2026-11-09 | closed | MOM: `8 November 2026 Sunday Deepavali` — `Monday, 9 November 2026, will be a public holiday …` | `SGX-ST-SCHED` | T2 | MOM's Sunday-substitution sentence |
| 2026-12-24 | replacement blocks | SGX: `24 Dec 2026 Thursday Eve of Christmas`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |
| 2026-12-25 | closed | MOM: `25 December 2026 Friday Christmas Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-12-31 | replacement blocks | SGX: `31 Dec 2026 Thursday Eve of New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |

`Hari Raya Puasa 2026` falls on Saturday 21 March (MOM prints it with no
substitution sentence), so it closes no weekday and ships no row.

**The Sunday substitutions are MOM's own sentences, not inference.** MOM
words each as conditional on the employee's rest day; the exchange reads a
market closure because the substituted day is itself a gazetted public
holiday under Singapore's Holidays Act arrangement the calendar publishes,
and the operator's designation adopts the calendar whole. Each substituted
Monday above quotes MOM's sentence in full beside it so a reader can check
the derivation from the saved bytes.

**The half-day rows state the sheet exactly.** Every matching phase the
operator prints on a half day — the morning session to 12:00, the closing
Non-Cancel to 12:06 and Trade at Close to 12:16 — ends at or before the
printed `Close: 12:16pm`. The rows ship as `ReplacementBlocks` restating the
complete half-day structure rather than a scalar early close, because a
scalar clip on the envelope's final close would drag the order-entry-only
Pre-Close into `is_open`: the restated midday order-entry slice
(12:00-12:04) only ever marks order acceptance where the sheet prints
Pre-Close order entry, and the closing routine runs to the printed 12:16.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `SGX-ST-SCHED` | 2025-01-01 .. 2026-12-31 | <https://api2.sgx.com/content-api?queryId=dd24dd8e5b3ef52e535a662e01b58d76471f335e%3Apage&variables=%7B%22path%22%3A%22%2Fstock-exchange%2Ftrading%22%2C%22lang%22%3A%22EN%22%7D> | retrieved 2026-09-28 01:59 UTC | T2 | `45dbdc61d808b4f72bb8bbddb198107f08759a20b85b288271d6d5a0326facd7` |
| `SGX-MOM-CAL-2025-2026` | 2025-01-01 .. 2026-12-31 | <https://www.mom.gov.sg/employment-practices/public-holidays> | retrieved 2026-09-28 01:48 UTC | T1 | `a4f175a7d33222b91f1c8c2f84e6d1e75f0c0b15e265addb478b73e3e65dcf3d` |

`SGX-ST-SCHED` is the document id every row cites. `SGX-MOM-CAL-2025-2026` is
the designated calendar the closure dates are read from; it is the
government's own gazetted list, recorded here so each closure date's bytes
resolve, and no row keys on it alone. The store's
`holidays/raw/equities/sgx_securities/2025-2027/` also holds the live SPA
shell and the SGX Group desk calendar PDF (derivatives day notes; context
only) — neither keys a row.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2011-08-01 session bounds (09:00–12:30 and 14:00–17:00) are not attested by any artifact named in the repository; the 2011-08-01 rulebook supplies only the routine phase boundaries carried by the two oldest profiles. The ledger horizon is therefore 2011-08-01, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a dated pre-2011 SGX-ST rulebook or practice-note edition stating the lunch-break session bounds, which would move the horizon earlier.
- Current routine ends are randomized: Pre-Open ends 08:58–08:59 and 12:58–12:59, Pre-Close ends 17:04–17:05. Each order-entry slice stops at the earliest possible end so no matching time is claimed as order entry.
- Trade at Close matches at the Equilibrium Price and is therefore tradeable throughout its window.
