<!-- SPDX-License-Identifier: MIT-0 -->

# `lse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`lse.rs`](../../src/calendar/schedules/equities/europe/lse.rs)
- **Source sets:** [`EU-LSE`](../schedules/sources.md#eu-lse), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SETS January-2010 baseline, the 2012-04-30 CPX launch, and the 2016-03-21 randomized midday auction are primary-sourced with conservative latest uncross edges.

## Revision rows

- 2012-04-30 — T1 — LSE MIT201 document history — Closing Price Crossing session added, closing envelope extended to 16:40.
- 2016-03-21 — T1 — LSE notice N01/16 — SETS intraday auction at 12:00 with its two-minute run and 30-second random end.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://docs.londonstockexchange.com/sites/default/files/documents/compliance_update_mar_09.pdf> — LSE compliance update, March 2009: the 2009 compliance parameters behind the January-2010 SETS baseline.
- <https://docs.londonstockexchange.com/sites/default/files/documents/live-001-300910-appendix-a.pdf> — LSE Millennium Exchange rehearsal timetable, September 2010, appendix A: pre-trading 07:00, opening call 07:50, randomized 08:00 uncross, closing call 16:30 to its latest 16:35:30 edge.
- <https://docs.londonstockexchange.com/sites/default/files/documents/mit201-guide-to-the-trading-system-15-6-20240429.pdf> — LSE MIT201, Guide to the Trading System: section 4.4 lists pre-trading as a scheduled session preceding the opening auction call; section 4.5 calls CPX "a short, modified regular trading session"; its operator-maintained document history records the 2012-04-30 production functional release.
- <https://docs.londonstockexchange.com/sites/default/files/documents/n1512_attach1.pdf> — LSE notice N15/12, official attachment: the closing-auction uncross starts at 16:35 and CPX is the up-to-five-minute executable session immediately following it.
- <https://docs.londonstockexchange.com/sites/default/files/documents/mit501.pdf> — LSE MIT501: CPX introduced in April 2012 with a default five-minute duration.
- <https://docs.londonstockexchange.com/sites/default/files/documents/servicetechnicaldescriptionintroductionofnewtradingcurrencies.pdf> — LSE service technical description: CPX scheduled grid 16:35:01–16:40:00.
- <https://docs.londonstockexchange.com/sites/default/files/documents/n0116.pdf> — LSE notice N01/16: SETS intraday auction effective 2016-03-21, starting at 12:00, running two minutes, with a random end of up to 30 seconds.
- <https://www.londonstockexchange.com/resources/equities-trading-resources?tab=technical-library> — LSE equities technical library: current technical parameters preserving the intraday-auction grid and CPX to 16:40.
- <https://www.londonstockexchange.com/equities-trading/asset-classes/shares-trading/sets> — LSE SETS product page, the source set's current entry point.
- <https://docs.londonstockexchange.com/sites/default/files/documents/international-order-book-introduction-sheet.pdf> — LSE SETS-aligned trading-day timetable.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only, never an effective date.

## Holidays

**Coverage:** 2010-01-01..2027-12-31 (inclusive trade dates in `Europe/London`; tier T1
throughout).

LSE's holiday statement is the operator's own holiday table, which has shipped under four
site generations: "The Public and Bank Holidays in England & Wales" on the `.htm` Business
days page (`about-the-exchange/company-overview/business-days/business-days.htm`, 2010-2014),
the same page on the operator group's own site
(`lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days`,
captured continuously 2013-10..2020-07 — `lseg.com/businessdays` 301s there, and the
operator's own December 2015 service announcement `live001-03122015` cites
`www.lseg.com/businessdays` as where "Full details of London Stock Exchange trading and EUI
settlement days can be found"), the server-rendered "Bank holidays and their impact on our
trading services" table of the 2020 SPA page (`trade/trading-access/business-days`,
2020-2022), and the same table delivered by the content API behind `londonstockexchange.com`
(`api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days`, 2023-2026).
The table is **rolling** at every generation — it lists business days from the present forward
to the next New Year — so each audited year is pinned by captures taken inside it or just
before it; the archived states 2010-2026 are the rows of the `### Documents`
table, and each year section below names its capture and its corroboration. The 2010-2012
generation states the December half days in page-level words — `all Exchange markets will
close from 12:30 London time onwards` — naming the dates in the sentence; the 2013 onward
generations carry the same statement per row. LSE runs no overnight session, so an event date
and its trade date are one civil day and the conversion is the identity.

**Five 2025 dates are not audited.** The rolling table had already moved past 2025-04-18,
2025-04-21, 2025-05-05, 2025-05-26 and 2025-08-25 when the first 2025-era capture was taken
(the 2025-12-18 table starts at 2025-12-24), the Wayback CDX index holds exactly four captures
of the API URL (2024-02-07, 2024-03-08, 2025-12-18, 2026-06-17 — queried 2026-09-28 ~01:25 UTC),
and archive.today holds no snapshot, so no surviving T1 or T2 artifact states those five dates.
T3 restatements exist on the open web (mrtopstep.com, ebc.com, apricotcapital.am) and cannot
key rows (LAW-PRIMARY-SOURCES). The five dates ship as `Unsourced` rather than as silence,
because a date inside the window without a row would claim "audited normal", which nothing
here can support. **Closing condition:** a capture of the business-days API dated inside the
2025-01-02..2025-12-17 gap, or the operator's re-publication of a 2025 table.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `New Year` — `Friday 1 January` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own printed date; the `*` marks OTC-reporting availability, not trading |
| 2010-04-02 | closed | `Easter` — `Friday 2 April` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own printed date |
| 2010-04-05 | closed | `Easter` — `Monday 5 April` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own printed date |
| 2010-05-03 | closed | `May` — `Monday 3 May*` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own printed date |
| 2010-05-31 | closed | `Spring` — `Monday 31 May*` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own printed date |
| 2010-08-30 | closed | `Summer` — `Monday 30 August*` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own printed date |
| 2010-12-24 | early close | `On 24 December and 31 December 2010 all Exchange markets will close from 12:30 London time onwards.` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the printed event date; the 12:30 closing-process start is the day's final close |
| 2010-12-27 | closed | `Christmas` — `Monday 27 December*` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own substitute-day print (25/26 December fell at the weekend) |
| 2010-12-28 | closed | `Christmas` — `Tuesday 28 December*` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the operator's own substitute-day print |
| 2010-12-31 | early close | `On 24 December and 31 December 2010 all Exchange markets will close from 12:30 London time onwards.` | ``LSE-BUSDAYS-2010-01-30`` | T1 | the printed event date; the 12:30 closing-process start is the day's final close |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | `New Year` — `Monday 3 January*` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own substitute-day print |
| 2011-04-22 | closed | `Easter` — `Friday 22 April` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own printed date |
| 2011-04-25 | closed | `Easter` — `Monday 25 April` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own printed date |
| 2011-04-29 | closed | `Royal Wedding` — `Friday 29 April*` | ``LSE-BUSDAYS-2010-12-03`` | T1 | first stated by the 2010-12-03 capture (the wedding was announced 2010-11-16); corroborated by the 2011-02-01 capture |
| 2011-05-02 | closed | `May` — `Monday 2 May*` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own printed date |
| 2011-05-30 | closed | `Spring` — `Monday 30 May*` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own printed date |
| 2011-08-29 | closed | `Summer` — `Monday 29 August*` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own printed date |
| 2011-12-23 | early close | `On Friday 23 December and Friday 30 December 2011 all Exchange markets will close from 12:30 London time onwards.` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the printed event date; 24/31 December fell at the weekend so the half days are the printed Fridays |
| 2011-12-26 | closed | `Christmas` — `Monday 26 December` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own printed date |
| 2011-12-27 | closed | `Christmas` — `Tuesday 27 December*` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the operator's own substitute-day print |
| 2011-12-30 | early close | `On Friday 23 December and Friday 30 December 2011 all Exchange markets will close from 12:30 London time onwards.` | ``LSE-BUSDAYS-2010-10-07`` | T1 | the printed event date |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `New Year` — `Monday 2 January*` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own substitute-day print; corroborated by the 2012-01-17 capture |
| 2012-04-06 | closed | `Good Friday` — `Friday 6 April` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date |
| 2012-04-09 | closed | `Easter Monday` — `Monday 9 April` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date |
| 2012-05-07 | closed | `Early May` — `Monday 7 May*` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date |
| 2012-06-04 | closed | `Spring` — `Monday 4 June*` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date (moved for the Diamond Jubilee) |
| 2012-06-05 | closed | `Queen's Diamond Jubilee` — `Tuesday 5 June*` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date |
| 2012-08-27 | closed | `Summer` — `Monday 27 August*` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date |
| 2012-12-24 | early close | `On Monday 24 December and Monday 31 December 2012 all Exchange markets will close from 12.30 London time onwards.` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the printed event date; restated by the 2013-01-01 capture's own per-row wording |
| 2012-12-25 | closed | `Christmas Day` — `Tuesday 25 December` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date; corroborated by the 2013-01-01 capture |
| 2012-12-26 | closed | `Boxing Day` — `Wednesday 26 December*` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the operator's own printed date; corroborated by the 2013-01-01 capture |
| 2012-12-31 | early close | `On Monday 24 December and Monday 31 December 2012 all Exchange markets will close from 12.30 London time onwards.` | ``LSE-BUSDAYS-2011-12-27`` | T1 | the printed event date; corroborated by the 2013-01-01 capture |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `New Year's Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date |
| 2013-03-29 | closed | `Good Friday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date |
| 2013-04-01 | closed | `Easter Monday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date |
| 2013-05-06 | closed | `Early May Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date |
| 2013-05-27 | closed | `Spring Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date |
| 2013-08-26 | closed | `Summer Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date |
| 2013-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time.` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the printed event date; the 12:30 closing-process start is the day's final close |
| 2013-12-25 | closed | `Christmas Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date; corroborated by the 2014-01-15 capture |
| 2013-12-26 | closed | `Boxing Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the operator's own printed date; corroborated by the 2014-01-15 capture |
| 2013-12-31 | early close | `New Years Eve` — `Markets closing process commences from 12:30 London time.` | ``LSE-BUSDAYS-2013-01-01`` | T1 | the printed event date; corroborated by the 2014-01-15 capture |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `New Year's Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-04-18 | closed | `Good Friday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-04-21 | closed | `Easter Monday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-05-05 | closed | `Early May Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-05-26 | closed | `Spring Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-08-25 | closed | `Summer Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the printed event date |
| 2014-12-25 | closed | `Christmas Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-12-26 | closed | `Boxing Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date |
| 2014-12-31 | early close | `New Year's` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the printed event date |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `New Year's Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-BUSDAYS-2014-01-15`` | T1 | the operator's own printed date; corroborated by the 2015-01-03 capture of the lseg.com channel |
| 2015-04-03 | closed | `Good Friday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own printed date; corroborated by the 2015-04-05 capture |
| 2015-04-06 | closed | `Easter Monday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own printed date; corroborated by the 2015-04-05 capture |
| 2015-05-04 | closed | `Early May Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own printed date; corroborated by the 2015-04-05 capture |
| 2015-05-25 | closed | `Spring Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own printed date; corroborated by the 2015-04-05 capture |
| 2015-08-31 | closed | `Summer Bank holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own printed date; corroborated by the 2015-04-05 capture |
| 2015-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the printed event date; corroborated by the 2015-04-05 and 2015-12-21 captures and by the operator's own 2015-12-03 announcement (`live001-03122015`: half trading day on 24 and 31 December 2015) |
| 2015-12-25 | closed | `Christmas Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own printed date; corroborated by the 2015-04-05 and 2015-12-21 captures |
| 2015-12-28 | closed | `Boxing Day (holiday)` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the operator's own substitute-day print (25 December fell on Friday); corroborated by the 2015-04-05 and 2015-12-21 captures |
| 2015-12-31 | early close | `New Year's Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2015-01-03`` | T1 | the printed event date; corroborated by the 2015-04-05 and 2015-12-21 captures and by the operator's own 2015-12-03 announcement |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `New Year's Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2015-01-03 capture |
| 2016-03-25 | closed | `Good Friday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2016-01-18 and 2016-04-17 captures |
| 2016-03-28 | closed | `Easter Monday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2016-01-18 and 2016-04-17 captures |
| 2016-05-02 | closed | `Early May Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2016-01-18 and 2016-04-17 captures |
| 2016-05-30 | closed | `Spring Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2016-01-18 and 2016-04-17 captures |
| 2016-08-29 | closed | `Summer Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2016-01-18 and 2016-04-17 captures |
| 2016-12-23 | early close | `Christmas` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the printed event date (24/25 December fell at the weekend so the half day is the printed Friday); corroborated by the 2016-01-18, 2016-04-17 and 2016-11-22 captures |
| 2016-12-26 | closed | `Boxing Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own printed date; corroborated by the 2016-01-18, 2016-04-17 and 2016-11-22 captures |
| 2016-12-27 | closed | `Christmas Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the operator's own substitute-day print; corroborated by the 2016-01-18, 2016-04-17 and 2016-11-22 captures |
| 2016-12-30 | early close | `New Year` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2015-12-21`` | T1 | the printed event date; corroborated by the 2016-01-18, 2016-04-17 and 2016-11-22 captures |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `New Year Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own substitute-day print; corroborated by the 2017-02-04 and 2017-05-05 captures |
| 2017-04-14 | closed | `Good Friday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04 and 2017-05-05 captures |
| 2017-04-17 | closed | `Easter Monday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04 and 2017-05-05 captures |
| 2017-05-01 | closed | `Early May Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04 and 2017-05-05 captures |
| 2017-05-29 | closed | `Spring Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04 and 2017-05-05 captures |
| 2017-08-28 | closed | `Summer Bank Holiday` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04 and 2017-05-05 captures |
| 2017-12-22 | early close | `Christmas` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the printed event date; corroborated by the 2017-02-04, 2017-05-05 and 2017-12-02 captures |
| 2017-12-25 | closed | `Christmas Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04, 2017-05-05 and 2017-12-02 captures |
| 2017-12-26 | closed | `Boxing Day` — `NON-trading day & NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the operator's own printed date; corroborated by the 2017-02-04, 2017-05-05 and 2017-12-02 captures |
| 2017-12-29 | early close | `New Year` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2016-11-22`` | T1 | the printed event date; corroborated by the 2017-02-04, 2017-05-05 and 2017-12-02 captures |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `New Year's Day` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2017-02-04 and 2018-02-03 captures |
| 2018-03-30 | closed | `Good Friday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03 and 2018-05-03 captures |
| 2018-04-02 | closed | `Easter Monday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03 and 2018-05-03 captures |
| 2018-05-07 | closed | `Early May Bank Holiday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03 and 2018-05-03 captures |
| 2018-05-28 | closed | `Spring Bank Holiday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03 and 2018-05-03 captures |
| 2018-08-27 | closed | `Summer bank Holiday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03, 2018-05-03 and 2018-10-01 captures |
| 2018-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the printed event date; corroborated by the 2018-02-03, 2018-05-03 and 2018-10-01 captures |
| 2018-12-25 | closed | `Christmas Day` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03, 2018-05-03 and 2018-10-01 captures |
| 2018-12-26 | closed | `Boxing Day` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03, 2018-05-03 and 2018-10-01 captures |
| 2018-12-31 | early close | `New Year's Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the printed event date; corroborated by the 2018-02-03, 2018-05-03 and 2018-10-01 captures |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `New Year's Day` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2017-12-02`` | T1 | the operator's own printed date; corroborated by the 2018-02-03, 2018-05-03 and 2018-10-01 captures |
| 2019-04-19 | closed | `Good Friday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2018-10-01`` | T1 | the operator's own printed date; corroborated by the 2019-03-26 capture |
| 2019-04-22 | closed | `Easter Monday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2018-10-01`` | T1 | the operator's own printed date; corroborated by the 2019-03-26 capture |
| 2019-05-06 | closed | `Early May Bank Holiday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2018-10-01`` | T1 | the operator's own printed date; corroborated by the 2019-03-26 capture |
| 2019-05-27 | closed | `Spring Bank Holiday` — `NON-trading day. Also NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2018-10-01`` | T1 | the operator's own printed date; corroborated by the 2019-03-26 and 2019-06-14 captures |
| 2019-08-26 | closed | `Summer Bank Holiday` — `NON-trading day. Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-03-26`` | T1 | the operator's own printed date; corroborated by the 2019-06-14 capture |
| 2019-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2019-03-26`` | T1 | the printed event date; corroborated by the 2019-06-14, 2019-12-19 and 2020-03-17 captures |
| 2019-12-25 | closed | `Christmas Day` — `NON-trading day. Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-03-26`` | T1 | the operator's own printed date; corroborated by the 2019-06-14, 2019-12-19 and 2020-03-17 captures |
| 2019-12-26 | closed | `Boxing Day` — `NON-trading day, Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-03-26`` | T1 | the operator's own printed date; corroborated by the 2019-06-14, 2019-12-19 and 2020-03-17 captures |
| 2019-12-31 | early close | `New Year's Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day` | ``LSE-LSEGBUSDAYS-2019-03-26`` | T1 | the printed event date; corroborated by the 2019-06-14, 2019-12-19 and 2020-03-17 captures |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `New Year's Day` — `NON-trading day. Also GBX / GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-03-26`` | T1 | the operator's own printed date; corroborated by the 2019-06-14, 2019-12-19 and 2020-03-17 captures |
| 2020-04-10 | closed | `Good Friday` — `NON-trading day. Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-06-14`` | T1 | the operator's own printed date; corroborated by the 2019-12-19 and 2020-03-17 captures |
| 2020-04-13 | closed | `Easter Monday` — `NON-trading day. Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-06-14`` | T1 | the operator's own printed date; corroborated by the 2019-12-19 and 2020-03-17 captures |
| 2020-05-08 | closed | `Early May Bank Holiday (VE Day)` — `NON-trading day. Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-06-14`` | T1 | the operator's own printed date — the Early May holiday moved from Monday 4 May for the 75th anniversary; corroborated by the 2019-12-19 and 2020-03-17 captures |
| 2020-05-25 | closed | `Spring Bank Holiday` — `NON-trading day. Also GBX/ GBP NON-settlement day in EUI` | ``LSE-LSEGBUSDAYS-2019-06-14`` | T1 | the operator's own printed date; corroborated by the 2019-12-19, 2020-03-17 and 2020-05-11 captures |
| 2020-08-31 | closed | `Summer Bank Holiday` — `NON-trading day.` | ``LSE-LSEGBUSDAYS-2019-12-19`` | T1 | the operator's own printed date; corroborated by the 2020-03-17, 2020-05-11 and 2020-07-31 captures |
| 2020-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-LSEGBUSDAYS-2019-12-19`` | T1 | the printed event date; corroborated by the 2020-03-17, 2020-05-11, 2020-07-31 and 2020-12-21 captures |
| 2020-12-25 | closed | `Christmas Day` — `NON-trading day.` | ``LSE-LSEGBUSDAYS-2019-12-19`` | T1 | the operator's own printed date; corroborated by the 2020-03-17, 2020-05-11, 2020-07-31 and 2020-12-21 captures |
| 2020-12-28 | closed | `Boxing Day (substitute)` — `NON-trading day.` | ``LSE-LSEGBUSDAYS-2019-12-19`` | T1 | the operator's own substitute-day print; corroborated by the 2020-03-17, 2020-05-11, 2020-07-31 and 2020-12-21 captures |
| 2020-12-31 | early close | `New Year's Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-LSEGBUSDAYS-2019-12-19`` | T1 | the printed event date; corroborated by the 2020-03-17, 2020-05-11, 2020-07-31 and 2020-12-21 captures |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `New Year's Day` — `NON-trading day.` | ``LSE-LSEGBUSDAYS-2019-12-19`` | T1 | the operator's own printed date; corroborated by the 2020-07-31, 2020-12-21 and 2021-01-21 captures |
| 2021-04-02 | closed | `Good Friday` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own printed date; corroborated by the 2021-01-21 capture |
| 2021-04-05 | closed | `Easter Monday` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own printed date; corroborated by the 2021-01-21 capture |
| 2021-05-03 | closed | `Early May Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own printed date; corroborated by the 2021-01-21 capture |
| 2021-05-31 | closed | `Spring Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own printed date; corroborated by the 2021-01-21 capture |
| 2021-08-30 | closed | `Summer Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own printed date; corroborated by the 2021-01-21 capture |
| 2021-12-24 | early close | `Christmas Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the printed event date; corroborated by the 2021-01-21 capture |
| 2021-12-27 | closed | `Christmas Day (substitute)` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own substitute-day print; corroborated by the 2021-01-21 capture |
| 2021-12-28 | closed | `Boxing Day (Substitute)` — `NON-trading day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the operator's own substitute-day print; corroborated by the 2021-01-21 capture |
| 2021-12-31 | early close | `New Year's Eve` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-2020-12-21`` | T1 | the printed event date; corroborated by the 2021-01-21 capture |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-03 | closed | `New Year's Day (substitute)` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own substitute-day print |
| 2022-04-15 | closed | `Good Friday` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2022-04-18 | closed | `Easter Monday` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2022-05-02 | closed | `Early May Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date |
| 2022-06-02 | closed | `Spring Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date (moved for the Platinum Jubilee) |
| 2022-06-03 | closed | `Platinum Jubilee Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2022-08-29 | closed | `Summer Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2022-12-23 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the printed event date; corroborated by the 2022-07-07 capture |
| 2022-12-26 | closed | `Boxing Day` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2022-12-27 | closed | `Christmas Day (substitute day)` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own substitute-day print; corroborated by the 2022-07-07 capture |
| 2022-12-30 | early close | `New Year Holiday half day` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the printed event date; corroborated by the 2022-07-07 capture |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `New Year Day (substitute day)` — `NON-trading day.` | ``LSE-BUSDAYS-2022-01-24`` | T1 | the operator's own substitute-day print |
| 2023-04-07 | closed | `Good Friday` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2023-04-10 | closed | `Easter Monday` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2023-05-01 | closed | `Early May Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date |
| 2023-05-08 | closed | `Bank Holiday for the coronation of King Charles III` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date |
| 2023-05-29 | closed | `Spring Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date |
| 2023-08-28 | closed | `Summer Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date; corroborated by the 2022-07-07 capture |
| 2023-12-22 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the printed event date |
| 2023-12-25 | closed | `Christmas Day` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date |
| 2023-12-26 | closed | `Boxing Day` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date |
| 2023-12-29 | early close | `New Year Holiday half day` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the printed event date |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `New Year Day` — `NON-trading day.` | ``LSE-BUSDAYS-API-2023-04-10`` | T1 | the operator's own printed date |
| 2024-03-29 | closed | `Good Friday` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-04-01 | closed | `Easter Monday` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-05-06 | closed | `Early May Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-05-27 | closed | `Spring Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-08-26 | closed | `Summer Bank Holiday` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the printed event date; corroborated by the 2024-03-08 capture |
| 2024-12-25 | closed | `Christmas Day` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-12-26 | closed | `Boxing Day` — `NON-trading day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the operator's own printed date; corroborated by the 2024-03-08 capture |
| 2024-12-31 | early close | `New Year Holiday half day` — `Markets closing process commences from 12:30 London time. Standard settlement day.` | ``LSE-BUSDAYS-2024-02-07`` | T1 | the printed event date; corroborated by the 2024-03-08 capture |
### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year Day` — `Wednesday 1 January 2025` — `NON-trading day.` | `LSE-BUSDAYS-2024-02-07` | T1 | the operator's own event date; no overnight session, so the trade date is the same civil day |
| 2025-04-18 | unsourced | — (the rolling table had rolled past the date before any surviving capture) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-04-21 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-05-05 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-05-26 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-08-25 | unsourced | — (same rolling-table gap) | `LSE-BUSDAYS-2025-12-18` | T1 | no surviving artifact states the date; no status claimed |
| 2025-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 24 December 2025; the day's final close is the printed 12:30 London time |
| 2025-12-25 | closed | `Christmas Day` — `Thursday 25 December 2025` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2025-12-26 | closed | `Boxing Day` — `Friday 26 December 2025` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2025-12-31 | early close | `New Year's Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 31 December 2025; the day's final close is the printed 12:30 London time |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day` — `Thursday 1 January 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2026-04-03 | closed | `Good Friday` — `Friday 3 April 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2026-04-06 | closed | `Easter Monday` — `Monday 6 April 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; trade date is the same civil day |
| 2026-05-04 | closed | `Early May Bank Holiday` — `Monday 4 May 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the 2026-06-17 capture |
| 2026-05-25 | closed | `Spring Bank Holiday` — `Monday 25 May 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the 2026-06-17 capture |
| 2026-08-31 | closed | `Summer Bank Holiday` — `Monday 31 August 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2026-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 24 December 2026; the day's final close is the printed 12:30 London time |
| 2026-12-25 | closed | `Christmas Day` — `Friday 25 December 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2026-12-28 | closed | `Boxing Day (substitute)` — `Monday 28 December 2026` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own substitute-day print; trade date is the same civil day |
| 2026-12-31 | early close | `New Year's Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 31 December 2026; the day's final close is the printed 12:30 London time |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day` — `Friday 1 January 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-03-26 | closed | `Good Friday` — `Friday 26 March 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-03-29 | closed | `Easter Monday` — `Monday 29 March 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-05-03 | closed | `Early May Bank Holiday` — `Monday 3 May 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-05-31 | closed | `Spring Bank Holiday` — `Monday 31 May 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-08-30 | closed | `Summer Bank Holiday` — `Monday 30 August 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own event date; corroborated by the live table |
| 2027-12-24 | early close | `Christmas Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 24 December 2027; the day's final close is the printed 12:30 London time |
| 2027-12-27 | closed | `Christmas Day (substitute)` — `Monday 27 December 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own substitute-day print; corroborated by the live table |
| 2027-12-28 | closed | `Boxing Day (substitute)` — `Tuesday 28 December 2027` — `NON-trading day.` | `LSE-BUSDAYS-2025-12-18` | T1 | the operator's own substitute-day print; corroborated by the live table |
| 2027-12-31 | early close | `New Year's Holiday half day` — `Markets closing process commences from 12:30 London time.` | `LSE-BUSDAYS-2025-12-18` | T1 | the printed event date 31 December 2027; the day's final close is the printed 12:30 London time |

**Interpretive step — the half-day scalar.** The sheet states "Markets closing process
commences from 12:30 London time." for each half day. On a full day the same printed boundary
kind — the closing-process start — sits at 16:30, so the 12:30 statement is the day's final
close of the availability envelope and the closing-auction phase that would follow it is
removed rather than modelled; the row is an `EarlyClose` at 45 000 seconds. No late opens and
no other half days are printed in any audited year.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `LSE-BUSDAYS-2010-01-30` | 2010-01-01 .. 2011-01-03 | <https://web.archive.org/web/20100130065529id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20100130065529`, retrieved 2026-09-29 05:30 UTC | T1 | `f1d64609b54a52ed2496fd105819c055789bb583e7d5c7577f1351691ab2a25e` |
| `LSE-BUSDAYS-2010-10-07` | 2010-01-01 .. 2011-12-30 | <https://web.archive.org/web/20101007091122id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20101007091122`, retrieved 2026-09-29 05:30 UTC | T1 | `ac99b16b81d82eba2c0557e727077f071fa477140293fe24ff1a2e434b886974` |
| `LSE-BUSDAYS-2010-12-03` | 2010-12-27 .. 2011-12-30 | <https://web.archive.org/web/20101203104856id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20101203104856`, retrieved 2026-09-29 05:30 UTC | T1 | `4416ea2bc9604d2370b35c8f1ae83221391b8087e5eba97734c7ea9f0eaef53f` |
| `LSE-BUSDAYS-2011-02-01` | 2011-01-03 .. 2011-12-30 | <https://web.archive.org/web/20110201180705id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20110201180705`, retrieved 2026-09-29 05:30 UTC | T1 | `2eb6f1af8aafb912d2096264b91c3ff8e09ebd44df1e8baf01825cd5b92bafbb` |
| `LSE-BUSDAYS-2011-12-27` | 2011-12-26 .. 2012-12-31 | <https://web.archive.org/web/20111227075826id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20111227075826`, retrieved 2026-09-29 05:30 UTC | T1 | `2ca2bf1e08277cb0314229ea354f1532261e9dcba60eccb50c63c5632af36316` |
| `LSE-BUSDAYS-2012-01-17` | 2012-01-02 .. 2012-12-31 | <https://web.archive.org/web/20120117133012id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20120117133012`, retrieved 2026-09-29 05:30 UTC | T1 | `9884a611e2743bc80ac54f98cf893647740613dda6b040118a921b6c741233b7` |
| `LSE-BUSDAYS-2013-01-01` | 2012-12-24 .. 2013-12-31 | <https://web.archive.org/web/20130101122127id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20130101122127`, retrieved 2026-09-29 05:30 UTC | T1 | `9defbe1e5f0ac1a1e4d4c2aa36b1c24bfdbde0da21e75be6afbba6da505eb5ba` |
| `LSE-BUSDAYS-2014-01-15` | 2013-12-24 .. 2015-01-01 | <https://web.archive.org/web/20140115110512id_/http://www.londonstockexchange.com/about-the-exchange/company-overview/business-days/business-days.htm> | Wayback `id_` replay of capture `20140115110512`, retrieved 2026-09-29 05:30 UTC | T1 | `14cfffbe01cc2d618aa9c0df5b04dd8f0b9e4ac781944e3b3389bf443ec50931` |
| `LSE-LSEGBUSDAYS-2015-01-03` | 2014-12-24 .. 2016-01-01 | <https://web.archive.org/web/20150103045430id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20150103045430`, retrieved 2026-10-02 UTC | T1 | `ce4ae71a9f28650fd18be394c52a8aa8d79b052038aadeebf82c94a9cc42ec6d` |
| `LSE-LSEGBUSDAYS-2015-04-05` | 2014-12-24 .. 2016-01-01 | <https://web.archive.org/web/20150405132414id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20150405132414`, retrieved 2026-10-02 UTC | T1 | `f176db644e228dc3abda9b35ffffd9e0f2af3c9c74a6c9a8010ef80d7769f4ea` |
| `LSE-LSEGBUSDAYS-2015-12-21` | 2015-12-24 .. 2017-01-02 | <https://web.archive.org/web/20151221080205id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20151221080205`, retrieved 2026-10-02 UTC | T1 | `b9a2f41338f4218302c8eed80d830ab9b78ff3eb296bc088bdf7a40a4688cb68` |
| `LSE-LSEGBUSDAYS-2016-01-18` | 2016-03-25 .. 2017-01-02 | <https://web.archive.org/web/20160118223452id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20160118223452`, retrieved 2026-10-02 UTC | T1 | `cd6a561ecaaba90eeb6ed4d72037fa74cc9aa258ff5320aa138b56bf3f9495db` |
| `LSE-LSEGBUSDAYS-2016-04-17` | 2016-03-25 .. 2017-01-02 | <https://web.archive.org/web/20160417162806id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20160417162806`, retrieved 2026-10-02 UTC | T1 | `12742854a82949f8fc1d20828676fc4dbbd27bf1cf8d60da9ba31a8549039566` |
| `LSE-LSEGBUSDAYS-2016-11-22` | 2016-12-23 .. 2018-01-01 | <https://web.archive.org/web/20161122160354id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20161122160354`, retrieved 2026-10-02 UTC | T1 | `28d39168f2563b922e08b814ae3d22a7e999cdc29eba01278059561395c8ffff` |
| `LSE-LSEGBUSDAYS-2017-02-04` | 2016-12-23 .. 2018-01-01 | <https://web.archive.org/web/20170204222808id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20170204222808`, retrieved 2026-10-02 UTC | T1 | `0ddf56559725a9e92bd9dad1a7f226b76fa52c867af536ba045a099e83328560` |
| `LSE-LSEGBUSDAYS-2017-05-05` | 2016-12-23 .. 2018-01-01 | <https://web.archive.org/web/20170505203121id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20170505203121`, retrieved 2026-10-02 UTC | T1 | `aa2945fcc327ad31ab3fc4e498a11d485bb4fbaab0dd334b27f9d14e7fb3dd5a` |
| `LSE-LSEGBUSDAYS-2017-12-02` | 2017-12-22 .. 2019-01-01 | <https://web.archive.org/web/20171202110229id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20171202110229`, retrieved 2026-10-02 UTC | T1 | `bf71ef582ca25a3f8ff0e2930d863d5acb67317762795b707372bceb07c0cb0c` |
| `LSE-LSEGBUSDAYS-2018-02-03` | 2017-12-22 .. 2019-01-01 | <https://web.archive.org/web/20180203095647id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20180203095647`, retrieved 2026-10-02 UTC | T1 | `16413c9465bf8d3bd909b8b67c1b7b87cfdfa8ee5ed560e5145098bd6c077a35` |
| `LSE-LSEGBUSDAYS-2018-05-03` | 2018-03-30 .. 2019-01-01 | <https://web.archive.org/web/20180503121105id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20180503121105`, retrieved 2026-10-02 UTC | T1 | `12a39452aef92b53044e4bdce9fe186cbfdd23839137944ce91194abd87b1bdd` |
| `LSE-LSEGBUSDAYS-2018-10-01` | 2018-08-27 .. 2019-05-27 | <https://web.archive.org/web/20181001173859id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20181001173859`, retrieved 2026-10-02 UTC | T1 | `ec2fd1ace6e7747a965c74c7fe23b3b9f872303c51bec85cc04b63b586b20c08` |
| `LSE-LSEGBUSDAYS-2019-03-26` | 2019-04-19 .. 2020-01-01 | <https://web.archive.org/web/20190326054958id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20190326054958`, retrieved 2026-10-02 UTC | T1 | `e8ef57a5bee4b009da09ea3772e6b49be73198b52b9fdad4f263a0af5e8f8632` |
| `LSE-LSEGBUSDAYS-2019-06-14` | 2019-08-26 .. 2020-05-25 | <https://web.archive.org/web/20190614074153id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20190614074153`, retrieved 2026-10-02 UTC | T1 | `a7745f884c79ab3683a219645f306950a782fb07d8f26e4fe9bb102eb6d99a09` |
| `LSE-LSEGBUSDAYS-2019-12-19` | 2019-12-24 .. 2021-01-01 | <https://web.archive.org/web/20191219013737id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20191219013737`, retrieved 2026-10-02 UTC | T1 | `d86ab5d9ce39a8d552e9f2a7a70d93c159ebc8fd4871729294b0e5135bd9a0d4` |
| `LSE-LSEGBUSDAYS-2020-03-17` | 2020-04-10 .. 2020-12-31 | <https://web.archive.org/web/20200317072237id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20200317072237` (gzip response decoded), retrieved 2026-10-02 UTC | T1 | `5e8911451971b371d7d70dc556ca8887feba8dd8c15c5eb101b6ea2dde314ed1` |
| `LSE-LSEGBUSDAYS-2020-05-11` | 2020-04-10 .. 2020-12-31 | <https://web.archive.org/web/20200511062750id_/http://www.lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days> | Wayback `id_` replay of capture `20200511062750` (gzip response decoded), retrieved 2026-10-02 UTC | T1 | `2bab0d7d84bed7347e17bbd3e880045f09877f0a00c1f2a8f9127ab818307310` |
| `LSE-NOTICE-2015-12-03` | 2015-12-24 .. 2015-12-31 | <https://web.archive.org/web/20170916002832id_/http://www.londonstockexchange.com/products-and-services/technical-library/service-announcements/2015/live-001-03122015.doc> | Wayback `id_` replay of capture `20170916002832`, retrieved 2026-10-02 UTC | T1 | `52044cc2fac37dbc3de24dac2c1a863a59cd09df4dd9b37f1b4b99ca594323a8` |
| `LSE-BUSDAYS-2020-07-31` | 2020-08-31 .. 2021-05-31 | <https://web.archive.org/web/20200731082235id_/https://www.londonstockexchange.com/trade/trading-access/business-days> | Wayback `id_` replay of capture `20200731082235`, retrieved 2026-09-29 05:31 UTC | T1 | `005b59af3fc83bcbfac0d96bb1cefaf9036134feeaf5fa0c58201a17841f240e` |
| `LSE-BUSDAYS-2020-12-21` | 2020-12-24 .. 2022-01-03 | <https://web.archive.org/web/20201221145636id_/https://www.londonstockexchange.com/trade/trading-access/business-days> | Wayback `id_` replay of capture `20201221145636`, retrieved 2026-09-29 05:31 UTC | T1 | `d1bf0e7da56dbd525afa6d28f2ec2306e2864902aaa26bd820cf4b727a14ba19` |
| `LSE-BUSDAYS-2021-01-21` | 2021-04-02 .. 2022-01-03 | <https://web.archive.org/web/20210121011235id_/https://www.londonstockexchange.com/trade/trading-access/business-days> | Wayback `id_` replay of capture `20210121011235`, retrieved 2026-09-29 05:31 UTC | T1 | `1ea15180a8868d07efe5a4b9a44e69429dbd5d1031cd6cccafffd118fc3fc962` |
| `LSE-BUSDAYS-2022-01-24` | 2021-08-30 .. 2023-01-02 | <https://web.archive.org/web/20220124140748id_/https://www.londonstockexchange.com/trade/trading-access/business-days> | Wayback `id_` replay of capture `20220124140748`, retrieved 2026-09-29 05:31 UTC | T1 | `d50c450a10954da7f44639241f215d3bc1ff9dafcff970eb7eb24409795f83b2` |
| `LSE-BUSDAYS-2022-07-07` | 2022-06-02 .. 2023-08-28 | <https://web.archive.org/web/20220707035146id_/https://www.londonstockexchange.com/trade/trading-access/business-days> | Wayback `id_` replay of capture `20220707035146`, retrieved 2026-09-29 05:31 UTC | T1 | `1f0050b6d4bb11f1d173faeac44600df0ee178404a5b9dbfb3699d523049b620` |
| `LSE-BUSDAYS-API-2023-04-10` | 2023-04-07 .. 2024-01-01 | <https://web.archive.org/web/20230410163738id_/https://api.londonstockexchange.com/api/v1/pages?path=securities-trading%2Ftrading-access%2Fbusiness-days&parameters=> | Wayback `id_` replay of capture `20230410163738`, retrieved 2026-09-29 05:36 UTC | T1 | `e4db7c06f9263f60ededc8fea5fecd6b58269d38928a0567b5e6950292f856e5` |
| `LSE-BUSDAYS-2024-02-07` | 2024-03-29 .. 2025-01-01 | <https://web.archive.org/web/20240207115235id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20240207115235`, retrieved 2026-09-28 01:28 UTC | T1 | `4730c509be75be779d106ccad036eae4f515bc6524613951126b33491b40a90b` |
| `LSE-BUSDAYS-2024-03-08` | 2024-03-29 .. 2025-01-01 | <https://web.archive.org/web/20240308200156id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20240308200156`, retrieved 2026-09-28 01:36 UTC | T1 | `dbcdccd7ef5bae333856d86819ad9f1908df3b5bab4fa6354e891d3ce99300db` |
| `LSE-BUSDAYS-2025-12-18` | 2025-12-24 .. 2027-12-31 | <https://web.archive.org/web/20251218172240id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20251218172240`, retrieved 2026-09-28 01:30 UTC | T1 | `e7d81b189bd75137b2e51136b2d6979d780dd3d50b95c111ef5acbdd38d49a11` |
| `LSE-BUSDAYS-2026-06-17` | 2026-05-04 .. 2029-01-01 | <https://web.archive.org/web/20260617134453id_/https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days&parameters=> | Wayback `id_` replay of capture `20260617134453`, retrieved 2026-09-28 01:30 UTC | T1 | `1759a375014f079d7725ac0c08293073d9d27c8d9a28e1999d7f611d7145270a` |
| `LSE-BUSDAYS-LIVE-2026-09-28` | 2026-08-31 .. 2029-01-01 | <https://api.londonstockexchange.com/api/v1/pages?path=equities-trading/business-days> | retrieved 2026-09-28 01:16 | T1 | `4f9f6f3aef9ab02874112c05c0c07ac4529e6c38e7ad74d0aa41735853a87a7f` |

`LSE-BUSDAYS-2010-12-03` and `LSE-BUSDAYS-2011-02-01` corroborate the 2010-10-07 capture on
every row they share and add the one row it lacks (the Royal Wedding). `LSE-BUSDAYS-2010-01-30`,
`LSE-BUSDAYS-2012-01-17` and `LSE-BUSDAYS-2013-01-01` are likewise full-state corroborations
beside the earliest artifact that prints each of their years. In the lseg.com generation the
2015-04-05, 2015-12-21, 2016-04-17, 2017-05-05, 2018-05-03 and 2020-05-11 captures
corroborate the December captures that first print each year, and the 2020-03-17 capture is
dated inside the former 2020-01-01..2020-05-25 window (it prints 2020-04-10..2020-12-31).
`LSE-NOTICE-2015-12-03` keys no row: it is the operator's own service announcement restating
the 2015 half days and pointing at `www.lseg.com/businessdays`, the pointer that led to the
lseg.com channel. `LSE-BUSDAYS-2021-01-21` and `LSE-BUSDAYS-2022-07-07` corroborate the
December captures before them. `LSE-BUSDAYS-2024-03-08`
keys no row: it confirms the February capture's 2025-01-01 endpoint and that no further 2025
rows were listed at that date. `LSE-BUSDAYS-2026-06-17` and `LSE-BUSDAYS-LIVE-2026-09-28` key
no row either: they corroborate the 2025-12-18 capture's rows on every date they share, which
is what lets one document id carry 2026 and 2027. The store's `holidays/raw/equities/lse/
2025-2027/` holds the 2025-2027 artifacts with its own index,
`holidays/raw/equities/lse/2010-2024/` holds the fourteen 2010-2024 `.htm`/SPA/API artifacts
above, and `holidays/raw/equities/lse/evidence-thread/` holds the lseg.com captures and the
2015 announcement (sha256s in `SHA256SUMS-lseg-2026-10-02.txt`, the hunt record in that
directory's `INDEX.md`).

## Gaps and residual risks

- **Interpretive step, order-entry classification.** Pre-trading 07:00–07:50 is modelled `order_entry` on MIT201 section 4.4, which lists it as a scheduled trading session distinct from the executable phases of the order-book day. No on-book execution can occur before the opening auction uncrosses. Closing condition: none needed; a later MIT201 edition that reclassifies the phase would move it.
- **Interpretive step, randomized uncrosses.** The opening uncross, the intraday auction uncross and the closing uncross are each randomized. The deterministic profile holds the auction classification through the latest possible edge (08:00:30, 12:02:30, 16:35:30), so the calendar never reports continuous trading while an auction can still run. This is the crate's conservative-envelope convention, not an operator statement about any individual security.
- **Holiday coverage, five 2025 dates.** 2025-04-18, 2025-04-21, 2025-05-05, 2025-05-26 and 2025-08-25 are inside the audited window with `Unsourced` rows: the operator's rolling business-days table had moved past them before any surviving capture, and no archived or live artifact states them (checked 2026-09-28). The dates are Good Friday, Easter Monday and the three bank-holiday Mondays, so the crate claims nothing about them rather than an unaudited closure. Closing condition: a capture of the business-days API dated inside the 2025-01-02..2025-12-17 gap. Re-checked 2026-10-09 (UTC): the CDX query for the API URL across 2025-03..2025-12 returns exactly one capture, the 2025-12-18 one this file already reads, so no earlier 2025 capture exists to close the gap from the API side. The 2026-10-09 (UTC) Common Crawl sweep closed the second archive: the API has zero captures in the 2025 collections (store `holidays/raw/cc-sweeps-2026-10-09.txt`).
- **Holiday coverage gap 2015-01-02..2019-12-31 and 2020-01-01..2020-05-25 — CLOSED as data (2026-10-02 UTC; [#218](https://github.com/SharurTrading/exchange-hours-rs/issues/218)).** The gap existed because the `.htm` Business days page died in February 2014 and the 2020 SPA page's first Wayback capture is 2020-07-31. The 2026-09-29 search covered both `.htm` URL forms, the `/trade/` prefix, `docs.londonstockexchange.com`, the business-days API URL and a domain-wide `lseg.com` sweep (5 000 collapsed url keys — a CDX truncation, as it turned out: the sweep's cap hid the exchange's own page on the group site). The close came through the operator's own pointer: the December 2015 service announcement `live001-03122015` (capture `20170916002832` — a class the earlier holiday/business-day URL-name greps could not see) cites `www.lseg.com/businessdays` for "Full details of London Stock Exchange trading and EUI settlement days"; that URL 301s (capture `20160423171007`) to `lseg.com/areas-expertise/our-markets/london-stock-exchange/equities-markets/trading-services/business-days`, whose the URL's 155 CDX rows (2013-10-04..2020-07-23) comprise 89 HTTP-200 captures — running continuously from 2013-10-04 to 2020-06-14 — plus 37 redirects and 29 revisits. Seventeen `id_` replays (sha256s in the store's `evidence-thread/SHA256SUMS-lseg-2026-10-02.txt`) tile both former gaps: the 2015-01-03 capture prints all 2015 rows, the 2015-12-21 capture all 2016 rows, the 2016-11-22 capture all 2017 rows, the 2017-12-02 capture all 2018 rows and 2019-01-01, the 2018-10-01 and 2019-03-26 captures the rest of 2019, the 2019-06-14 capture the moved 2020-05-08 VE Day holiday, and the 2019-12-19 capture the whole 2020 year — while the 2020-03-17 capture is dated inside 2020-01-01..2020-05-25 and prints 2020-04-10..2020-12-31, meeting the issue's second closing condition by the letter. The rows and their citations are above; the coverage verdict over 2015-2020 is now audited normal/closed exactly as printed. The five 2025 rolling-table dates below remain the only unaudited dates in the window.
- **Holiday coverage gap, 2020-01-01..2020-08-30 (tracked as [#218](https://github.com/SharurTrading/exchange-hours-rs/issues/218)).** The first capture of the `/trade/trading-access/business-days` page is 2020-07-31 and its table starts at 2020-08-31, so New Year 2020, the Easter dates, the VE-Day-moved Early May holiday and Spring 2020 are stated by no surviving artifact (CDX checked 2026-09-29 UTC, including `?lang=` and `?mod=` variants). **Retried 2026-09-29 UTC with the CDX service back up:** the URL's full capture list (26 entries, 2020-07-31 through 2026-03-15) still opens at 2020-07-31, the `?lang=en` and `?mod=article_inline` variants are captured only from 2020-09-19 and 2020-12-21, and a Memento TimeTravel timemap query over both Business-days URLs returned no additional archive. Queries inside the span refuse. Closing condition: a capture dated 2020-01-01..2020-05-25.

## Module narrative (moved from src/calendar/schedules/equities/europe/lse.rs on 2026-10-02 UTC)

MIT501 confirms that CPX was introduced in April
2012 with a default five-minute duration; the operator's service description
gives its scheduled 16:35:01-16:40:00 grid. MIT201's operator-maintained
document history records the production functional release on 2012-04-30.
https://docs.londonstockexchange.com/sites/default/files/documents/n1512_attach1.pdf
https://docs.londonstockexchange.com/sites/default/files/documents/mit501.pdf
https://docs.londonstockexchange.com/sites/default/files/documents/servicetechnicaldescriptionintroductionofnewtradingcurrencies.pdf
https://docs.londonstockexchange.com/sites/default/files/documents/mit201-guide-to-the-trading-system-15-6-20240429.pdf
