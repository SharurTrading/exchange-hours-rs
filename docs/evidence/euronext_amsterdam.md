<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_amsterdam` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs)
- **Source sets:** [`EU-EURONEXT`](../schedules/sources.md#eu-euronext), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Core shares at the published nominal exchange boundaries: legacy 07:15 pre-open, 09:00–17:30 continuous trading, 17:30–17:35 closing auction, and Trading-at-Last through 17:40; the 2023-03-20 move to a 07:30 pre-open is date-aware. Per-security 0–30-second auction uncross timing is outside scope and does not change venue availability.

## Revision rows

- 2023-03-20 — T1 — Euronext Go-Live Weekend Guidelines — legacy-market pre-opening moves from 07:15 to 07:30 CET.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.euronext.com/sites/default/files/european_cash_markets_trading_hours_for_24th_and_31st_december_2010.pdf> — Euronext, "European cash markets trading hours for 24th and 31st December 2010": the operator's 2010 special-day appendix, showing the legacy 07:15 CET pre-opening and the principal-share opening at 09:00.
- <https://connect.euronext.com/nl/listview/notice-download?attachmentId=201416&id=581906&type=PDF> — Euronext 2014 normal-hours trading appendix, repeating the same legacy grid.
- <https://live.euronext.com/en/listview/notice-download?id=598779&type=PDF&attachmentId=218289> — Euronext notice PAR_20150924_07448_EUR, 2015 cash-market auction-randomization notice: zero-to-30-second randomized uncrosses for Belgian, Dutch, French and Portuguese trading groups.
- <https://live.euronext.com/en/listview/notice-download?id=598933&type=PDF&attachmentId=218443> — companion Euronext cash-market notice for the same randomization change.
- <https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf> — Euronext Go-Live Weekend Guidelines: the phase-one timetable makes the legacy pre-opening change effective 2023-03-20 and separately gives 2023-03-27 for the Italian migration.
- <https://connect.euronext.com/sites/default/files/it-documentation/Guide%20to%20Trading%20System%20-%20Borsa%20Italiana%20Migration%20to%20Optiq%20-%20Functional%20Changes%20v.2.0.pdf> — Euronext guide to the trading system, Borsa Italiana migration to Optiq, functional changes v2.0.
- <https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx> — current appendix to Euronext Instructions 4-01/4-03 Trading Manuals: pre-opening 07:30 CET, nominal continuous trading 09:00–17:30, closing auction to 17:35, Trading-at-Last through 17:40 for principal shares, with randomized zero-to-30-second uncross timing per security.
- <https://www.euronext.com/en/trading/trading-hours-holidays> — Euronext trading hours and holidays, the source set's current entry point.
- <https://www.euronext.com/en/regulation/euronext-regulated-markets> — Euronext regulated-market manual hub, the monitoring entry point.
- <https://www.euronext.com/en/products-services/cash-market-notices> — Euronext cash-market notices, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the principal continuous-trading share segment, not every Euronext Amsterdam instrument or segment.
- **Interpretive step, order-entry classification.** The operator's trading appendix describes the pre-opening as a Call phase — the French and Dutch columns render it "phase d'accumulation" / "accumulatiefase" — and its liquidity-provider clause speaks of "the order-accumulation periods preceding pre-scheduled or other Uncrossings during a Trading Day". The first uncrossing of the day is the 09:00 opening uncrossing, so no central-order-book trade can match before continuous trading starts; the pre-opening window is therefore `order_entry` and the closing uncrossing and Trading-at-Last stay `extended`.
- **Interpretive step, randomized uncrosses.** The 2015 notice's instrument-level 0–30-second micro-events do not define one exchange-wide transition instant, so this exchange-level profile retains the published nominal boundaries. The notice's defective effective year is deliberately not used as a cutover.
- **Horizon carried below the first dated artifact.** The earliest artifact cited for the legacy grid is the operator's special-day appendix for 24 and 31 December 2010, so the ledger horizon is 2010-12-24, its own first attested day, and the January-2010 to December-2010 interval is carried rather than sourced. The source set's "January-2010-or-launch" status is not an artifact dated inside that interval and does not source it. Closing condition: a Euronext trading appendix or notice dated in or before January 2010 that prints the legacy 07:15/09:00/17:30/17:40 grid would move the horizon down to the January-2010 floor.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.

> Shared module. [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs) also carries
> [`euronext_paris`](euronext_paris.md), [`euronext_brussels`](euronext_brussels.md), [`euronext_lisbon`](euronext_lisbon.md), [`euronext_milan`](euronext_milan.md).
> The anchor identity for this module is [`euronext_paris`](euronext_paris.md), which receives the
> module narrative when LAW-EVIDENCE-FILES moves it; the module is not yet migrated.

## Holidays

**Coverage:** 2010-01-01..2026-12-31 (inclusive trade dates in
`Europe/Amsterdam`; tier T1 throughout). The table owns 108 rows: 83 closures,
23 early closes and two `Unsourced` dates. No 2027 calendar is published.

The pre-2022 operator calendars explicitly cover the European cash markets,
including Amsterdam. From 2022 the operator publishes separate market
columns: the Amsterdam column is read on its own. Every Amsterdam/Paris
cell agrees in the captured 2022-2026 calendars, including the ordinary
December substitute Fridays in 2022 and 2023; Dublin alone has half days on
those substitutes. Scope follows the named market, rather than its offset.

For 2010, notice `PAR_20101126_06372_EUR` (`EURONEXT-EOY-2010`),
header `All locations` / `All markets`, prints the Dutch regulated-market
J0 AEX and J1 equity rows at 09:00 opening, continuous trading to 13:55,
14:00 closing auction and TAL 14:00-14:05. The 2011 press release states
cash-market eves close at 17:35; the 2012 notice states 14:00. The 2013
Info-Flash states 14:00 while the later page restates 14:05; the sourced
intersection stays 14:00, with that five-minute residual disclosed.
Calendar prose from 2014-2021 states all instruments close by 14:05, except
2016/2017 substitute Fridays explicitly trading at usual hours.

The 2024 and 2025 appendices print their own Amsterdam J0/AEX row:
`09:00 Random - 13:55`, `14:00 Random`, and
`14:00 - 14:05 Continuous TAL Random`. Its TAL end is 14:05, so both
years' December eves close there. The 2026 calendar announces half days
on 2026-12-24 and 2026-12-31 but supplies no end-of-year hours; these dates
remain `Unsourced` until that appendix is published. The named calendar
rows are unconditional dates, so the unknown hours are explicit gaps
inside the audited window rather than a shortened publication horizon.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Friday 1 January 2010 (New Year's Day)` | ``EURONEXT-PR-2010`` | T1 | the operator's own printed date on the European cash markets' closure list; trade date is the same civil day |
| 2010-04-02 | closed | `Friday 2 April 2010 (Good Friday)` | ``EURONEXT-PR-2010`` | T1 | the operator's own printed date |
| 2010-04-05 | closed | `Monday 5 April 2010 (Easter Monday)` | ``EURONEXT-PR-2010`` | T1 | the operator's own printed date |
| 2010-12-24 | early close | `On Friday 24 December 2010 ... cash markets will close at 1.00 pm GMT (2.00 pm CET)`; the appendix grid prints Trading `09:00 - 13:55`, CA `14:00`, TAL `14:00 - 14:05` for every Amsterdam equity group | ``EURONEXT-EOY-2010`` | T1 | the printed event date plus the appendix's own Dutch Amsterdam schedule; the envelope close is the printed TAL end, 14:05 CET — see the interpretive step |
| 2010-12-31 | early close | the same appendix grid for `Friday 31 Decembre 2010` | ``EURONEXT-EOY-2010`` | T1 | same reading as 2010-12-24 |

The press release lists no other weekday closure: 1 May, 25 and 26 December 2010 fell at
the weekend, and the UK bank holidays it names (3 May, 31 May, 30 August, 27-28 December)
are expressly `NYSE Liffe's sterling-based products and UK-based commodity products`, not
the cash markets — no Amsterdam row for them.

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | `Friday 22 April 2011 (Good Friday)` | ``EURONEXT-PR-2011`` | T1 | the operator's own printed date on the European Cash markets' closure list |
| 2011-04-25 | closed | `Monday 25 April 2011 (Easter Monday)` | ``EURONEXT-PR-2011`` | T1 | the operator's own printed date |
| 2011-12-23 | early close | `On Friday 23 December 2011 ... trading on the Cash markets will close at 5.35 pm CET` | ``EURONEXT-PR-2011`` | T1 | the printed event date and the operator's own close instant; the envelope close is the printed 17:35 CET (the closing-auction end) — see the interpretive step |
| 2011-12-26 | closed | `Monday 26 December 2011` — the list's unnamed weekday closure | ``EURONEXT-PR-2011`` | T1 | the operator's own printed date |
| 2011-12-30 | early close | `On Friday 30 December 2011 trading on the Cash markets will close at 5.35 pm CET` | ``EURONEXT-PR-2011`` | T1 | the printed event date; same reading as 2011-12-23 |

The 2011 list names no New Year substitute (1 January fell on Saturday and `3 January` is
expressly a UK-only NYSE Liffe holiday), no Labour Day (1 May, a Sunday) and no Christmas
Day weekday (25 December, a Sunday), so the cash markets traded those days and no row
exists.

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-06 | closed | `Friday 6 April 2012 ... Good Friday` | ``EURONEXT-TC-2012`` | T1 | the operator's own printed date on the European cash-market notice's closure list |
| 2012-04-09 | closed | `Monday 9 April 2012 ... Easter Monday` | ``EURONEXT-TC-2012`` | T1 | the operator's own printed date |
| 2012-05-01 | closed | `Tuesday 1 May 2012 ... Labour Day` | ``EURONEXT-TC-2012`` | T1 | the operator's own printed date |
| 2012-12-24 | early close | `On Christmas Eve, Monday 24 December 2012 ... the markets will close at 14:00 CET` | ``EURONEXT-TC-2012`` | T1 | the printed event date; the envelope close is the printed 14:00 CET |
| 2012-12-25 | closed | `Tuesday 25 December 2012 ... Christmas Day` | ``EURONEXT-TC-2012`` | T1 | the operator's own printed date |
| 2012-12-26 | closed | `Wednesday 26 December 2012 ... Boxing Day` | ``EURONEXT-TC-2012`` | T1 | the operator's own printed date |
| 2012-12-31 | early close | `... New Year's Eve, Monday 31 December 2012, the markets will close at 14:00 CET` | ``EURONEXT-TC-2012`` | T1 | the printed event date; same reading as 2012-12-24 |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `Tuesday 1 January 2013 (New Year's Day)` | ``EURONEXT-IF-2013`` | T1 | the operator's own printed date on the cash-markets closure list; corroborated by the 2014-01-12 Trading Calendar page's 2013 section |
| 2013-03-29 | closed | `Friday 29 March 2013 (Good Friday)` | ``EURONEXT-IF-2013`` | T1 | the operator's own printed date; corroborated likewise |
| 2013-04-01 | closed | `Monday 1 April 2013 (Easter Monday)` | ``EURONEXT-IF-2013`` | T1 | the operator's own printed date; corroborated likewise |
| 2013-05-01 | closed | `Wednesday 1 May 2013 (Labour Day)` | ``EURONEXT-IF-2013`` | T1 | the operator's own printed date; corroborated likewise |
| 2013-12-24 | early close | `On Tuesday 24 December 2013 ... trading on the Cash Markets will close at 14:00 CET` (Info-Flash); the 2014-01-12 page's 2013 section prints `close at 14:05 CET` | ``EURONEXT-IF-2013`` | T1 | the printed event date; the two statements conflict and lineage cannot order them, so the eve holds the narrowest sourced value, 14:00 CET — see the conflict note |
| 2013-12-25 | closed | `Wednesday 25 December 2013 (Christmas Day)` | ``EURONEXT-IF-2013`` | T1 | the operator's own printed date; corroborated likewise |
| 2013-12-26 | closed | `Thursday 26 December 2013 (Boxing Day)` | ``EURONEXT-IF-2013`` | T1 | the operator's own printed date; corroborated likewise |
| 2013-12-31 | early close | the same 14:00-vs-14:05 statements for `Tuesday 31 December 2013` | ``EURONEXT-IF-2013`` | T1 | same reading as 2013-12-24 |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `Wednesday 1 January 2014` — `New Year's Day` | ``EURONEXT-HH-2014-01-12`` | T1 | the operator's own printed date on the cash-markets calendar; trade date is the same civil day |
| 2014-04-18 | closed | `Friday 18 April 2014` — `Good Friday` | ``EURONEXT-HH-2014-01-12`` | T1 | the operator's own printed date; corroborated by the 2014-03-31, 06-26 and 07-02 captures |
| 2014-04-21 | closed | `Monday 21 April 2014` — `Easter Monday` | ``EURONEXT-HH-2014-01-12`` | T1 | the operator's own printed date; corroborated by the later 2014 captures |
| 2014-05-01 | closed | `Thursday 1 May 2014` — `Labour Day` | ``EURONEXT-HH-2014-01-12`` | T1 | the operator's own printed date; corroborated by the later 2014 captures |
| 2014-12-24 | early close | `Wednesday 24 December 2014` — `all instruments closing by 14:05 CET` | ``EURONEXT-HH-2014-01-12`` | T1 | the printed event date; the envelope close is the printed 14:05 CET |
| 2014-12-25 | closed | `Thursday 25 December 2014` — `Christmas Day` | ``EURONEXT-HH-2014-01-12`` | T1 | the operator's own printed date |
| 2014-12-26 | closed | `Friday 26 December 2014` — `Boxing Day` | ``EURONEXT-HH-2014-01-12`` | T1 | the operator's own printed date |
| 2014-12-31 | early close | `Wednesday 31 December 2014` — `all instruments closing by 14:05 CET` | ``EURONEXT-HH-2014-01-12`` | T1 | the printed event date; the envelope close is the printed 14:05 CET |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `Thursday 1 January 2015` — `New Year's Day` | ``EURONEXT-HH-2015-03-22`` | T1 | the operator's own printed date |
| 2015-04-03 | closed | `Friday 3 April 2015` — `Good Friday` | ``EURONEXT-HH-2015-03-22`` | T1 | the operator's own printed date |
| 2015-04-06 | closed | `Monday 6 April 2015` — `Easter Monday` | ``EURONEXT-HH-2015-03-22`` | T1 | the operator's own printed date |
| 2015-05-01 | closed | `Friday 1 May 2015` — `Labour Day` | ``EURONEXT-HH-2015-03-22`` | T1 | the operator's own printed date |
| 2015-12-24 | early close | `Thursday 24 December 2015` — `all instruments closing by 14:05 CET` | ``EURONEXT-HH-2015-03-22`` | T1 | the printed event date; the envelope close is the printed 14:05 CET |
| 2015-12-25 | closed | `Friday 25 December 2015` — `Christmas Day` | ``EURONEXT-HH-2015-03-22`` | T1 | the operator's own printed date |
| 2015-12-31 | closed | `Thursday 31 December 2015` — `New Year's Eve`, in the calendar's own closed-day list | ``EURONEXT-HH-2015-03-22`` | T1 | the operator's own printed date; 2015 is the one year NYE is a full closure |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Friday 1 January 2016` — `New Year's Day` | ``EURONEXT-HH-2016-03-04`` | T1 | the operator's own printed date |
| 2016-03-25 | closed | `Friday 25 March 2016` — `Good Friday` | ``EURONEXT-HH-2016-03-04`` | T1 | the operator's own printed date |
| 2016-03-28 | closed | `Monday 28 March 2016` — `Easter Monday` | ``EURONEXT-HH-2016-03-04`` | T1 | the operator's own printed date |
| 2016-12-26 | closed | `Monday 26 December 2016` — `Boxing Day`; 23 and 30 December `will close at the usual times` | ``EURONEXT-HH-2016-03-04`` | T1 | the operator's own printed date; no 2016 half day exists |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | `Friday 14 April 2017 (Good Friday)` | ``EURONEXT-HH-2017-08-05`` | T1 | the operator's own printed date |
| 2017-04-17 | closed | `Monday 17 April 2017 (Easter Monday)` | ``EURONEXT-HH-2017-08-05`` | T1 | the operator's own printed date |
| 2017-05-01 | closed | `Monday 1 May 2017 (Labour Day)` | ``EURONEXT-HH-2017-08-05`` | T1 | the operator's own printed date; 1 January fell on a Sunday and the calendar prints no substitute |
| 2017-12-25 | closed | `Monday 25 December 2017 (Christmas Day)` | ``EURONEXT-HH-2017-08-05`` | T1 | the operator's own printed date |
| 2017-12-26 | closed | `Tuesday 26 December 2017 (Boxing Day)`; 22 and 29 December `will close at the usual times` | ``EURONEXT-HH-2017-08-05`` | T1 | the operator's own printed date; no 2017 half day exists |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `Monday 1 January 2018 (New Year's Day)` | ``EURONEXT-HH-2018-08-26`` | T1 | the operator's own printed date; restated identically by the 2019-03-25 capture's archive section |
| 2018-03-30 | closed | `Friday 30 March 2018 (Good Friday)` | ``EURONEXT-HH-2018-08-26`` | T1 | the operator's own printed date |
| 2018-04-02 | closed | `Monday 2 April 2018 (Easter Monday)` | ``EURONEXT-HH-2018-08-26`` | T1 | the operator's own printed date |
| 2018-05-01 | closed | `Tuesday 1 May 2018 (Labour Day)` | ``EURONEXT-HH-2018-08-26`` | T1 | the operator's own printed date |
| 2018-12-24 | early close | `Monday 24 December 2018 (Christmas Eve)` — `all instruments closing by 14:05 CET` | ``EURONEXT-HH-2018-08-26`` | T1 | the printed event date; the envelope close is the printed 14:05 CET |
| 2018-12-25 | closed | `Tuesday 25 December 2018 (Christmas Day)` | ``EURONEXT-HH-2018-08-26`` | T1 | the operator's own printed date |
| 2018-12-26 | closed | `Wednesday 26 December 2018 (Boxing Day)` | ``EURONEXT-HH-2018-08-26`` | T1 | the operator's own printed date |
| 2018-12-31 | early close | `Monday 31 December 2018 (New Year's Eve)` — `all instruments closing by 14:05 CET` | ``EURONEXT-HH-2018-08-26`` | T1 | the printed event date |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Tuesday 1 January 2019 (New Year's Day)` | ``EURONEXT-HH-2019-03-25`` | T1 | the operator's own printed date; corroborated by the 2019-07-17 capture |
| 2019-04-19 | closed | `Friday 19 April 2019 (Good Friday)` | ``EURONEXT-HH-2019-03-25`` | T1 | the operator's own printed date |
| 2019-04-22 | closed | `Monday 22 April 2019 (Easter Monday)` | ``EURONEXT-HH-2019-03-25`` | T1 | the operator's own printed date |
| 2019-05-01 | closed | `Wednesday 1 May 2019 (Labour Day)` | ``EURONEXT-HH-2019-03-25`` | T1 | the operator's own printed date |
| 2019-12-24 | early close | `Tuesday 24 December 2019 (Christmas Eve)` — trading on `the Cash Markets, including Euronext Dublin`, all instruments `closing by 14:05 CET` | ``EURONEXT-HH-2019-03-25`` | T1 | the printed event date; Amsterdam is inside the sentence's cash-market scope and the 14:05 CET close is the envelope close |
| 2019-12-25 | closed | `Wednesday 25 December 2019 (Christmas Day)` | ``EURONEXT-HH-2019-03-25`` | T1 | the operator's own printed date |
| 2019-12-26 | closed | `Thursday 26 December 2019 (Boxing Day)` | ``EURONEXT-HH-2019-03-25`` | T1 | the operator's own printed date |
| 2019-12-31 | early close | `Tuesday 31 December 2019 (New Year's Eve)` — the same 14:05 CET sentence | ``EURONEXT-HH-2019-03-25`` | T1 | the printed event date |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Wednesday 1 January 2020 (New Year's Day)` | ``EURONEXT-HH-2019-12-10`` | T1 | the operator's own printed date; corroborated by the 2020-01-09 and 2020-12-05 captures |
| 2020-04-10 | closed | `Friday 10 April 2020 (Good Friday)` | ``EURONEXT-HH-2019-12-10`` | T1 | the operator's own printed date |
| 2020-04-13 | closed | `Monday 13 April 2020 (Easter Monday)` | ``EURONEXT-HH-2019-12-10`` | T1 | the operator's own printed date |
| 2020-05-01 | closed | `Friday 1 May 2020 (Labour Day)` | ``EURONEXT-HH-2019-12-10`` | T1 | the operator's own printed date |
| 2020-12-24 | early close | `Thursday 24 December 2020` — `On the Euronext Amsterdam, Brussels, Lisbon and Paris Cash Markets, all instruments will close by 14:05 CET` | ``EURONEXT-HH-2019-12-10`` | T1 | the printed event date; the Amsterdam part of the sentence is the envelope close; restated by the 2020-12-05 capture |
| 2020-12-25 | closed | `Friday 25 December 2020 (Christmas Day)` | ``EURONEXT-HH-2019-12-10`` | T1 | the operator's own printed date |
| 2020-12-31 | early close | `Thursday 31 December 2020` — the same 14:05 CET sentence | ``EURONEXT-HH-2019-12-10`` | T1 | the printed event date; 2020-12-28 is the Dublin substitute only and prints no Amsterdam row |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Friday 1 January 2021 (New Year's Day)` | ``EURONEXT-HH-2020-12-05`` | T1 | the operator's own Amsterdam cell; Dublin- and Oslo-only holidays elsewhere in the table print `Full Day Trading` in the Amsterdam column |
| 2021-04-02 | closed | `Friday 2 April 2021 (Good Friday)` | ``EURONEXT-HH-2020-12-05`` | T1 | the operator's own Amsterdam cell; corroborated by the 2021-01-16 capture |
| 2021-04-05 | closed | `Monday 5 April 2021 (Easter Monday)` | ``EURONEXT-HH-2020-12-05`` | T1 | the operator's own Amsterdam cell |
| 2021-12-24 | early close | `Friday 24 December 2021 (Christmas Eve)` — the Amsterdam, Brussels, Lisbon and Paris cash markets `will close by 14:05 CET` | ``EURONEXT-HH-2020-12-05`` | T1 | the printed event date; restated by the 2021-01-16 and 2022-05-23 captures |
| 2021-12-31 | early close | `Friday 31 December 2021 (New Year's Eve)` — the same 14:05 CET sentence | ``EURONEXT-HH-2020-12-05`` | T1 | the printed event date |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `Friday 15 April 2022 (Good Friday)` — Amsterdam `Closed` | ``EURONEXT-HH-2022-05-23`` | T1 | the operator's own Amsterdam cell; corroborated by the 2023-01-27 capture |
| 2022-04-18 | closed | `Monday 18 April 2022 (Easter Monday)` — Amsterdam `Closed` | ``EURONEXT-HH-2022-05-23`` | T1 | the operator's own Amsterdam cell |
| 2022-12-26 | closed | `Monday 26 December 2022 (Stephen's Day/Boxing Day)` — Amsterdam `Closed` | ``EURONEXT-HH-2022-05-23`` | T1 | the operator's own Amsterdam cell; 01-03, 12-23, 12-27 and 12-30 print `Full Day Trading` for Amsterdam, the substitutes being Dublin's |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | `Friday 7 April 2023 (Good Friday)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell; corroborated by the 2024-02-09 capture |
| 2023-04-10 | closed | `Monday 10 April 2023 (Easter Monday)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2023-05-01 | closed | `Monday 1 May 2023 (May Day)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2023-12-25 | closed | `Monday 25 December 2023 (Christmas)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2023-12-26 | closed | `Tuesday 26 December 2023 (St Stephens Day / Boxing Day)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell; 01-02, 12-22 and 12-29 print `Full Day Trading` for Amsterdam (Dublin's substitutes) |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Monday 1 January (New Year's Day)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell; corroborated by the 2024-02-09 capture |
| 2024-03-29 | closed | `Friday 29 March 2024 (Good Friday)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell; the 27/28 March half days are Oslo's and print `Full Day Trading` for Amsterdam |
| 2024-04-01 | closed | `Monday 1 April 2024 (Easter Monday)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2024-05-01 | closed | `Wednesday 1 May 2024 (Labour Day)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2024-12-24 | early close | `Tuesday 24 December 2024` — Amsterdam `**Half Trading Day`; the operator's 2024 end-of-year appendix prints the Amsterdam equity segments' TAL `14:00 - 14:05` | ``EURONEXT-EOY-2024`` | T1 | the printed event date plus the appendix's own Dutch Amsterdam schedule; the envelope close is the printed TAL end, 14:05 CET. The appendix is a live retrieval (2026-09-29) whose identity as the 2024 edition is witnessed by the 2025-01-02 page capture that links it by name — see the Documents note |
| 2024-12-25 | closed | `Wednesday 25 December 2024 (Christmas)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2024-12-26 | closed | `Thursday 26 December 2024 (St Stephens Day / Boxing Day)` — Amsterdam `Closed` | ``EURONEXT-HH-2023-11-27`` | T1 | the operator's own Amsterdam cell |
| 2024-12-31 | early close | `Tuesday 31 December 2024` — Amsterdam `**Half Trading Day`; the same appendix grid, TAL `14:00 - 14:05` | ``EURONEXT-EOY-2024`` | T1 | same reading as 2024-12-24 |
### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Wednesday 1 January (New Year's Day)` — Amsterdam `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Amsterdam cell; trade date is the same civil day |
| 2025-04-18 | closed | `Friday 18 April 2025 (Good Friday)` — Amsterdam `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Amsterdam cell; trade date is the same civil day |
| 2025-04-21 | closed | `Monday 21 April 2025 (Easter Monday)` — Amsterdam `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Amsterdam cell; trade date is the same civil day |
| 2025-05-01 | closed | `Thursday 1 May 2025 (Labour Day)` — Amsterdam `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Amsterdam cell; trade date is the same civil day |
| 2025-12-24 | early close | `Wednesday 24 December 2025` — Amsterdam `**Half Trading Day`; the appendix prints the closing uncross `14:00 Random` and TAL `14:00 - 14:05` for the Amsterdam equity segments | `EURONEXT-EOY-2025` | T1 | the printed event date plus the appendix's own Dutch Amsterdam schedule; the envelope close is the printed TAL end, 14:05 CET |
| 2025-12-25 | closed | `Thursday 25 December 2025 (Christmas)` — Amsterdam `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Amsterdam cell; trade date is the same civil day |
| 2025-12-26 | closed | `Friday 26 December 2025 (St Stephens Day / Boxing Day)` — Amsterdam `Closed` | `EURONEXT-HH-2025-12-06` | T1 | the operator's own Amsterdam cell; trade date is the same civil day |
| 2025-12-31 | early close | `Wednesday 31 December 2025` — Amsterdam `**Half Trading Day`; the appendix prints the same `14:00 Random` uncross and `14:00 - 14:05` TAL | `EURONEXT-EOY-2025` | T1 | the printed event date plus the appendix's own Dutch Amsterdam schedule; the envelope close is the printed TAL end, 14:05 CET |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Thursday 1 January 2026 (New Year's Day)` — Amsterdam `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Amsterdam cell, printed identically on the live page; trade date is the same civil day |
| 2026-04-03 | closed | `Friday 3 April 2026 (Good Friday)` — Amsterdam `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Amsterdam cell, printed identically on the live page; trade date is the same civil day |
| 2026-04-06 | closed | `Monday 6 April 2026 (Easter Monday)` — Amsterdam `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Amsterdam cell, printed identically on the live page; trade date is the same civil day |
| 2026-05-01 | closed | `Friday 1 May 2026 (Labour Day)` — Amsterdam `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Amsterdam cell, printed identically on the live page; trade date is the same civil day |
| 2026-12-24 | unsourced | `Thursday 24 December 2026` — Amsterdam `**Half Trading Day`, hours "To be announced" | `EURONEXT-IF-2026` | T1 | the half day is announced by the operator but its instants are unpublished; no status claimed for the hours |
| 2026-12-25 | closed | `Friday 25 December 2026 (Christmas)` — Amsterdam `Closed` | `EURONEXT-IF-2026` | T1 | the INFO-FLASH Amsterdam cell, printed identically on the live page; trade date is the same civil day |
| 2026-12-31 | unsourced | `Thursday 31 December 2026` — Amsterdam `**Half Trading Day`, hours "To be announced" | `EURONEXT-IF-2026` | T1 | the half day is announced by the operator but its instants are unpublished; no status claimed for the hours |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EURONEXT-PR-2010` | 2010-01-01 .. 2010-12-31 (the 2010 holiday calendar and early closing dates) | <https://web.archive.org/web/20181111124554id_/https://www.euronext.com/sites/www.euronext.com/files/press_release_14_oct_2009.pdf> | Wayback `id_` replay of capture `20181111124554`, retrieved 2026-09-29 UTC | T1 | `736892c0c3712b1f0e530dbcae188464f26d24647a306d96c09cf01fb8ef4547` |
| `EURONEXT-EOY-2010` | 2010-12-24 and 2010-12-31 | <https://web.archive.org/web/20181111124543id_/https://www.euronext.com/sites/www.euronext.com/files/european_cash_markets_trading_hours_for_24th_and_31st_december_2010.pdf> | Wayback `id_` replay of capture `20181111124543`, retrieved 2026-09-29 UTC | T1 | `e34ca550ca0db30fedae627928e7974a634c57c04e53cff5d74b2c74a1fb61bc` |
| `EURONEXT-CCBD-2010` | no rows keyed (the 2010 Calendar of Cash Business Days, notice PAR_20091015_05039_EUR; the Normal-week bounded-search record) | <https://web.archive.org/web/20181111124550id_/https://www.euronext.com/sites/www.euronext.com/files/calendar_of_cash_business_days_15_oct_2009.pdf> | Wayback `id_` replay of capture `20181111124550`, retrieved 2026-09-30 03:35 UTC | T1 | `0cc923a604d6e263348d77b88953dce05cd885f91bbd692dbf7933b9a83229cb` |
| `EURONEXT-PR-2011` | 2011-01-01 .. 2011-12-31 (the 2011 holiday calendar and early closing dates) | <https://web.archive.org/web/20181111124531id_/https://www.euronext.com/sites/www.euronext.com/files/holiday_calendar_and_early_closing_dates_for_european_markets_press_release.pdf> | Wayback `id_` replay of capture `20181111124531`, retrieved 2026-09-29 UTC | T1 | `5358973c8ce8d0197a917ac2e9aa7debc8341bc16b964b065f434a3bdd28f830` |
| `EURONEXT-TC-2012` | 2012-01-01 .. 2012-12-31 (notice PAR_20111114_07899_EUR, the 2012 Calendar of Cash Business Days) | <https://web.archive.org/web/20181111124523id_/https://www.euronext.com/sites/www.euronext.com/files/calendar_2012.pdf> | Wayback `id_` replay of capture `20181111124523`, retrieved 2026-09-29 UTC | T1 | `4da0f4141496d2d39a52379157ef1d1f98c5920a5f5eb353f7d2cd19952de035` |
| `EURONEXT-IF-2013` | 2013-01-01 .. 2013-12-31 (Info-Flash IFCA121112, the 2013 holiday calendar) | <https://web.archive.org/web/20181111124519id_/https://www.euronext.com/sites/www.euronext.com/files/ifca121112.pdf> | Wayback `id_` replay of capture `20181111124519`, retrieved 2026-09-29 UTC | T1 | `d2f2c26bf770378802c2ce738ee4c93fe04b5c458d1b7c828bdd2fc37fb9a27f` |
| `EURONEXT-TC-2014-01-12` | 2013-01-01 .. 2014-12-31 (the 2014 calendar and the 2013 restatement) | <https://web.archive.org/web/20140112174408id_/https://www.euronext.com/trading/nyse-euronext-trading-calendar> | Wayback `id_` replay of capture `20140112174408`, retrieved 2026-09-29 UTC | T1 | `196262708d0a8f31d7d6eb425d0d0d002196e6a27502e7c4edb7b87762c643a9` |
| `EURONEXT-TC-ARCHIVES-2015-05-01` | no rows keyed (the operator's per-year Trading Calendar Archives index, 1999-2014) | <https://web.archive.org/web/20150501182220id_/https://www.euronext.com/trading/nyse-euronext-trading-calendar/archives> | Wayback `id_` replay of capture `20150501182220`, retrieved 2026-09-29 UTC | T1 | `442dc57086edd9c5f57bc11c346dc6934d602a9459e2d5eb30f04429b84472e3` |
| `EURONEXT-ARCHIVES-CASH-2015-10-08` | no rows keyed (the same index on the next generation's path) | <https://web.archive.org/web/20151008051552id_/https://www.euronext.com/en/trading-calendars-hours/archives/cash> | Wayback `id_` replay of capture `20151008051552`, retrieved 2026-09-29 UTC | T1 | `160e25f9128cc6fa9c122bd893257ea5e8699a8ca03f1159da8a4fe5817a498b` |
| `EURONEXT-HH-2014-01-12` | 2014-01-01 .. 2014-12-31 (the 2014 cash-markets calendar) | <https://web.archive.org/web/20140112152726id_/https://www.euronext.com/trading/trading-hours-and-holidays> | Wayback `id_` replay of capture `20140112152726`, retrieved 2026-09-29 06:17 UTC | T1 | `fd8329e9dda392f69d21ef350f3083e15d4cd79ef58e68e8c467bc0347239bb0` |
| `EURONEXT-HH-2015-03-22` | 2015-01-01 .. 2015-12-31 (the 2015 cash-markets calendar) | <https://web.archive.org/web/20150322042835id_/https://www.euronext.com/trading/trading-hours-and-holidays> | Wayback `id_` replay of capture `20150322042835`, retrieved 2026-09-29 06:17 UTC | T1 | `5794e756a358eb5c3402f1ef93feb661679d572cd790cfb1914a202777fece47` |
| `EURONEXT-HH-2016-03-04` | 2016-01-01 .. 2016-12-31 (the 2016 calendar) | <https://web.archive.org/web/20160304192247id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20160304192247`, retrieved 2026-09-29 06:22 UTC | T1 | `15aadf7933c975c82ef903e70f9d8366fe8787fe99ad1bb28dd3a0416b2595da` |
| `EURONEXT-HH-2017-08-05` | 2017-01-01 .. 2017-12-31 (the 2017 calendar) | <https://web.archive.org/web/20170805202027id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20170805202027`, retrieved 2026-09-29 06:23 UTC | T1 | `5267845e3bd2fe339dce61d878724cb846f1e3aaed8ec1514317600e76a7f9bc` |
| `EURONEXT-HH-2018-08-26` | 2018-01-01 .. 2018-12-31 (the 2018 calendar, plus the 2017 restatement) | <https://web.archive.org/web/20180826042950id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20180826042950`, retrieved 2026-09-29 06:23 UTC | T1 | `d213f40a8a53ebc2f0a24f27f65d7f17cc2f6e4961e2407b986be92248b93bca` |
| `EURONEXT-HH-2019-03-25` | 2019-01-01 .. 2019-12-31 (the 2019 calendar, plus the 2018 restatement) | <https://web.archive.org/web/20190325100632id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20190325100632`, retrieved 2026-09-29 06:23 UTC | T1 | `1af181cc6794aee7048c4c7ee8f82ea133fa35d2673f81eaef7f8918815f8457` |
| `EURONEXT-HH-2019-12-10` | 2019-01-01 .. 2020-12-31 (the 2019 restatement and the 2020 calendar) | <https://web.archive.org/web/20191210215605id_/http://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20191210215605`, retrieved 2026-09-29 06:23 UTC | T1 | `35fbf47ffe78b754d080b27001f721eb4cee28dcfaacdc42123d614780f5a702` |
| `EURONEXT-HH-2020-12-05` | 2020-01-01 .. 2021-12-31 (the 2020 restatement and the 2021 per-market table) | <https://web.archive.org/web/20201205195829id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20201205195829`, retrieved 2026-09-29 06:18 UTC | T1 | `1299b5f1048e91c927e1589eb0c36ea88dcfd6215f36b153fd35476bb3e64d9d` |
| `EURONEXT-HH-2022-05-23` | 2021-01-01 .. 2022-12-31 (the 2021 restatement and the 2022 per-market table) | <https://web.archive.org/web/20220523023131id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20220523023131`, retrieved 2026-09-29 06:18 UTC | T1 | `c6dfa1fa31ae0a3ea7bcd09ab3707ffb50cc95ebc481334f01d0acd696b3348b` |
| `EURONEXT-HH-2023-11-27` | 2022-01-01 .. 2024-12-31 (the 2022 restatement, the 2023 and 2024 per-market tables) | <https://web.archive.org/web/20231127145642id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20231127145642`, retrieved 2026-09-29 06:18 UTC | T1 | `550939767ce4869dc486c45e24f1b11c0276b6f53a8b4a5fa5f99e60c088a23c` |
| `EURONEXT-HH-2025-01-02` | no rows keyed (the page state that links the 2024 end-of-year appendix by name) | <https://web.archive.org/web/20250102162019id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20250102162019`, retrieved 2026-09-29 UTC | T1 | `25f93b5ab4573d5dc03f8a90e3e39df62a940273aa5c77eb4b04281380647800` |
| `EURONEXT-HH-2025-12-06` | 2025-01-01 .. 2025-12-31 (the page's 2025 table) | <https://web.archive.org/web/20251206154319id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20251206154319`, retrieved 2026-09-28 01:50 UTC | T1 | `dc96c4f7f1e6a51cd2e6743385faa5cbc4156623ec45499c89e2ec3871398a18` |
| `EURONEXT-EOY-2024` | 2024-12-24 and 2024-12-31 (the 2024 end-of-year appendix to the Euronext Instructions 4-01/4-03) | <https://www.euronext.com/media/12642/download> | retrieved 2026-09-29 (live; identity witnessed by `EURONEXT-HH-2025-01-02`, which links this URL as the `2024 end of year - appendix`) | T1 | `ce588e41552777b77b550758616ae524d4dc96ae935d56bbe28cb1520a49021f` |
| `EURONEXT-EOY-2025` | 2025-12-24 and 2025-12-31 | <https://www.euronext.com/media/14656/download> | retrieved 2026-09-28 01:50 | T1 | `5850b4b4f5a031e637a0c44a0c7c1388ddc638c652133e66636c674af19bb6b9` |
| `EURONEXT-IF-2026` | 2026-01-01 .. 2026-12-31 | <https://connect2.euronext.com/sites/default/files/2025-11/IF251107CADE%202026%20Holiday%20Calendar%20for%20Euronexts%20Cash%20and%20Derivatives%20markets_1.pdf?VersionId=3ehe2.c9cMxv1K36.i6o4tz.5CAJmuw2> | retrieved 2026-09-28 01:09 | T1 | `617bc559510ef3a3d86d4a8b804422d0ba4c4dae43782a0a728d57a23fc4b1c5` |
| `EURONEXT-HH-LIVE-2026-09-28` | 2025-01-01 .. 2026-12-31 (the page's 2026 table; the 2025 table corroborated) | <https://live.euronext.com/en/resources/trading-hours-holidays> | retrieved 2026-09-28 01:07 | T1 | `5a1165a52361a81350fa69401910c76f046f28e16f757dadb6d33d68ccb6e5e3` |

`EURONEXT-HH-LIVE-2026-09-28` keys no 2026 row: the live page's 2026 table is cell for cell
the INFO-FLASH's Amsterdam column, and the INFO-FLASH keys the rows. The INFO-FLASH's printed
header date reads `07 November 2026` while its publishing path and file name say 2025-11-07;
both are recorded here as printed and neither is used to date any row — the document is cited
only for its table cells. The store's `holidays/raw/equities/euronext_paris/2025-2027/`
directory holds those four artifacts with its index, and
`holidays/raw/equities/euronext_paris/2010-2024/` holds the twenty 2014-2025 artifacts plus
the nine 2026-09-29 CDX-retrieval additions above — the four per-year operator documents and
the 2010 appendix that key the 2010-2013 and 2024 rows, the Trading Calendar page and
archives captures, and the 2024 end-of-year appendix XLSX (their sha256s appended to that
directory's `SHA256SUMS.txt`, their retrieval stamps and provenance in its `INDEX.md`; the
previous save's sums named one superseded filename and three stale digests,
and every on-disk byte was re-verified against its Wayback replay on 2026-09-29). The
2014-03-31/06-26/07-02, 2019-07-17, 2020-01-09, 2021-01-16, 2023-01-27 and 2024-02-09
captures are corroborations that key no row of their own; the 2015-09-20 replay renders as a
script shell in the current index and keys no row. `EURONEXT-TC-2014-01-12` corroborates the
2014 rows of `EURONEXT-HH-2014-01-12` cell for cell and keys the 2013 rows beside the
Info-Flash (its 14:05 eve restatement is the conflict recorded above);
`EURONEXT-TC-ARCHIVES-2015-05-01` and `EURONEXT-ARCHIVES-CASH-2015-10-08` key no row — they
are the operator's own per-year document index whose file URLs led to the 2010-2013
documents; `EURONEXT-HH-2025-01-02` keys no row — it witnesses the identity of
`EURONEXT-EOY-2024`.
