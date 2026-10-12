<!-- SPDX-License-Identifier: MIT-0 -->

# `bme` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bme.rs`](../../src/calendar/schedules/equities/europe/bme.rs)
- **Source sets:** [`EU-BME`](../schedules/sources.md#eu-bme), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

BME principal-share January-2010 phases use the sourced conservative 30-second auction edge; the 2023-12-04 Trading-At-Last launch is date-aware.

## Revision rows

- 2023-12-04 — T1 — BME Operating Instruction 47/2023 — ten-minute Trading-at-Last phase added for general trading, extending the executable envelope to 17:45.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.bolsasymercados.es/en/bme-exchange/trading-hours.html> — BME trading hours: opening auction 08:30–09:00, continuous trading 09:00–17:30, closing auction 17:30–17:35, Trading-at-Last to 17:45.
- <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2001/c20011uk.pdf> — Sociedad de Bolsas Circular 1/2001: the same opening, continuous and closing-auction grid, established before the January-2010 history floor.
- <https://www.bolsasymercados.es/es/sala-de-comunicacion/noticias/2023/las-subastas-en-la-bolsa-parte-2.html> — BME, "Las subastas en la Bolsa": SIBE auctions end in a random period of at most 30 seconds.
- <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2023/circular-1-23-english.pdf> — Sociedad de Bolsas Circular 1/2023, introducing the ten-minute Trading-at-Last phase for general trading.
- <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2023/oi-47-2023-application-of-tal-phase-for-fixing-instruments.pdf> — BME Operating Instruction 47/2023: records the day-level 2023-12-04 entry into force and expressly excludes only the separate Fixing system.
- <https://www.bolsasymercados.es/bme-exchange/en/Regulation/Regulation-Explorer> — BME Regulation Explorer, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the general trading segment for principal shares. The separate Fixing system, which Operating Instruction 47/2023 excludes from the Trading-at-Last phase, is not modelled.
- **Interpretive step, auction calls and uncrosses (corrected 2026-10-11 UTC).** Circular 1/2001 sections 3.2.1.1 and 3.2.1.4 expressly state that orders can be entered, modified and cancelled but no trades can be made during the 08:30–09:00 opening call and 17:30–17:35 closing call. Those calls are `order_entry`; they accept orders and `is_open` is false. The operator's randomized uncross can print during 09:00–09:00:30 and 17:35–17:35:30, so those conservative venue-level windows are `extended`. Regular trading starts at 09:00:30 and ends at 17:30; Circular 1/2023 section 3.2.1.5 states TAL lasts ten minutes until 17:45 after the closing auction (eight minutes for securities with an extended auction). The nominal 17:35–17:45 TAL and randomized uncross therefore share one executable extended union; no artificial 17:35:30 TAL start is asserted.
- **Interpretive step, randomized auction edge.** The deterministic profile uses the latest possible opening edge, 09:00:30, so it never reports continuous trading while the opening auction can still run; the closing auction likewise runs through 17:35:30.
- **Served identity.** Reviewed monthly after current-scope activation on
  2026-10-12 UTC; historical gaps remain tracked in #332 under the dated
  current OANDA scope exception in AGENTS.md.

## Module narrative (moved from src/calendar/schedules/equities/europe/bme.rs on 2026-10-02 UTC)

https://www.bolsasymercados.es/en/bme-exchange/trading-hours.html
Sociedad de Bolsas Circular 1/2001 establishes the same opening, continuous,
and closing-auction grid before the Jan-2010 history floor.
https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2001/c20011uk.pdf
BME also states that SIBE auctions end in a random period of at most 30
seconds. The deterministic profile uses the latest possible opening edge so
it never reports continuous trading while the opening auction can still run.
https://www.bolsasymercados.es/es/sala-de-comunicacion/noticias/2023/las-subastas-en-la-bolsa-parte-2.html

The former module narrative's empty-order-entry interpretation was corrected on
2026-10-11 UTC from the explicit no-trades wording of Circular 1/2001. Auction
calls are required order-entry phases; only their randomized uncross windows
and Trading-at-Last are executable extended phases.

## Holidays — reviewed 2026-10-11 UTC

**Coverage:** 2010-01-01..2026-12-31 (BME principal-share general trading).

The named BME principal-share scope now owns a T1 holiday table audited from
2010-01-01 through 2026-12-31. The operator's Sociedad de Bolsas operating
instructions publish each annual SIBE cash calendar independently; this is not
holiday inheritance from MEFF or another exchange. There are 106 rows: 86 full
closures, 18 complete replacement sessions and two `Unsourced` 2026 year-end
sessions. Dates without rows are audited normal within this window. The runtime
identity is served after current-scope activation on 2026-10-12 UTC;
historical gaps remain #332 under the dated AGENTS.md exception.

### Scope and half-day interpretation

The annual instructions circumscribe their dates to the Sistema de Interconexión
Bursátil. Their shorthand 14:00 half-day close is refined by their own later,
date-specific timetables. In the `ACCIONES (EQ)` column of instructions 116/2012,
116/2013, 142/2014, 162/2015, 80/2018, 84/2019 and 63/2020, the general continuous
session ends at 13:55 into a closing auction, whose nominal end is 14:00; they
also print the security-dependent auction extension to 14:02. The replacement
keeps the normal profile's published auction convention: 08:30–09:00 opening
call as order entry, 09:00–09:00:30 uncross as extended, 09:00:30–13:55
regular trading, 13:55–14:00 closing call as order entry, and 14:00–14:00:30
uncross as extended. Circular
1/2001 section 3.2.1.4 states the random period of at most 30 seconds after the
nominal closing instant, and conditions the two-minute extension on special
cases in section 6.2.4.4. Those per-security conditional outcomes do not change
this identity's nominal phase convention, just as they do not extend its normal
17:35:30 final close. These boundaries are the scoped conservative auction
envelope, not a guarantee of an individual security's actual uncross.

The 2014 detailed instruction 142/2014 cross-refers to 116/2013, and the
2015 detailed instruction 162/2015 cross-refers to 142/2014. Those referenced
numbers are the prior detailed timetables, rather than the corresponding annual
calendar numbers 117/2013 and 147/2014. The instructions themselves explicitly
name their own session years and 24/31 December, and the operator index titles
name the same years; the cross-reference errors do not change the stated scope
or dates. No runtime boundary is inferred from those erroneous references.

Instructions 47/2024 and 64/2025 explicitly print `CONTINUO: Fin fase TAL` at
14:10. Those dates have the same 13:55 end of continuous trading but retain the
14:00 uncross and Trading-at-Last executable union until 14:10. The later detailed
timetables govern over the annual summaries' 14:00 shorthand. Replacement
blocks retain the real regular/extended handoff; a scalar early close would
wrongly label the half-day closing auction as regular trading.

The `FIXING`, ETF and warrant columns concern other segments and are excluded.
`ESPECIALES` and `FUERA DEL HORARIO DE MERCADO` are negotiated and communicated
transactions outside general trading, rather than automated matching phases.
Circular 1/2001 section 10.1 describes opposite-side orders or trades without
public communication, outside established hours, subject to price/volume
conditions; its section 10.2 distinguishes operations requiring authorization.
They do not extend the general principal-share matching scope to 15:00.

### Publication horizon and residual gaps

Instruction 65/2025 unconditionally names 2026-12-24 and 2026-12-31 as half
sessions but says their detailed hours will be published in a later instruction.
The operator's public Regulation Renta Variable index, retrieved 2026-10-11 UTC,
contains no later 2026 timetable or 2027 annual cash calendar. The two known
2026 eves remain `Unsourced` inside the window; an annual shorthand or the
current general calendar page cannot establish the missing phase details.
Closing condition: the operator's detailed 2026 `ACCIONES (EQ)` timetable.
The absence of a 2027 publication ends the audited window at 2026-12-31; no
2027 date is asserted normal. The unresolved detailed timetable remains
disclosed in #332; these future
eves stay typed refusals under LAW-HOLIDAY-SCOPE.

### Per-date derivation

Every date is the operator's own local civil session date in Europe/Madrid;
there is no overnight reassignment. Annual editions list the full weekday
closure set, without invented national-holiday substitutes. In particular,
2010 and 2021 Christmas/New-Year eves are full closures, 2011/2016/2017/2022/2023
publish no substitute half days, and 2025-12-26 is expressly closed.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | −   Viernes 1 de Enero | `BME-CAL-2010` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2010-04-02 | closed | −   Viernes 2 de Abril | `BME-CAL-2010` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2010-04-05 | closed | −   Lunes 5 de Abril | `BME-CAL-2010` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2010-12-24 | closed | −   Viernes 24 de Diciembre | `BME-CAL-2010` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2010-12-31 | closed | −   Viernes 31 de Diciembre | `BME-CAL-2010` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | − Viernes 22 de Abril | `BME-CAL-2011` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2011-04-25 | closed | − Lunes 25 de Abril | `BME-CAL-2011` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2011-12-26 | closed | − Lunes 26 de Diciembre | `BME-CAL-2011` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-06 | closed | − Viernes, 6 de abril | `BME-CAL-2012` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2012-04-09 | closed | − Lunes, 9 de abril | `BME-CAL-2012` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2012-05-01 | closed | − Martes, 1 de mayo | `BME-CAL-2012` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2012-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2012` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2012-12-25 | closed | − Martes, 25 de diciembre | `BME-CAL-2012` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2012-12-26 | closed | − Miércoles, 26 de diciembre | `BME-CAL-2012` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2012-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2012` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | - Martes, 1 de enero | `BME-CAL-2013` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2013-03-29 | closed | - Viernes, 29 de marzo | `BME-CAL-2013` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2013-04-01 | closed | - Lunes, 1 de abril | `BME-CAL-2013` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2013-05-01 | closed | - Miércoles, 1 de mayo | `BME-CAL-2013` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2013-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2013` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2013-12-25 | closed | - Miércoles, 25 de diciembre | `BME-CAL-2013` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2013-12-26 | closed | - Jueves, 26 de diciembre | `BME-CAL-2013` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2013-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2013` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | - Miércoles, 1 de enero | `BME-CAL-2014` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2014-04-18 | closed | - Viernes, 18 de abril | `BME-CAL-2014` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2014-04-21 | closed | - Lunes, 21 de abril | `BME-CAL-2014` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2014-05-01 | closed | - Jueves, 1 de mayo | `BME-CAL-2014` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2014-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2014` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2014-12-25 | closed | - Jueves, 25 de diciembre | `BME-CAL-2014` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2014-12-26 | closed | - Viernes, 26 de diciembre | `BME-CAL-2014` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2014-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2014` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | -   Jueves, 1 de enero | `BME-CAL-2015` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2015-04-03 | closed | -   Viernes, 3 de abril | `BME-CAL-2015` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2015-04-06 | closed | -   Lunes, 6 de abril | `BME-CAL-2015` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2015-05-01 | closed | -   Viernes, 1 de mayo | `BME-CAL-2015` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2015-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2015` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2015-12-25 | closed | -   Viernes, 25 de diciembre | `BME-CAL-2015` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2015-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2015` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | -   Viernes, 1 de enero | `BME-CAL-2016` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2016-03-25 | closed | -   Viernes, 25 de marzo | `BME-CAL-2016` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2016-03-28 | closed | -   Lunes, 28 de marzo | `BME-CAL-2016` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2016-12-26 | closed | -   Lunes, 26 de diciembre | `BME-CAL-2016` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | -   Viernes, 14 de abril | `BME-CAL-2017` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2017-04-17 | closed | -   Lunes, 17 de abril | `BME-CAL-2017` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2017-05-01 | closed | -   Lunes, 1 de mayo | `BME-CAL-2017` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2017-12-25 | closed | -   Lunes, 25 de diciembre | `BME-CAL-2017` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2017-12-26 | closed | -   Martes, 26 de diciembre | `BME-CAL-2017` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | -   Lunes 1 de enero | `BME-CAL-2018` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2018-03-30 | closed | -   Viernes 30 de marzo | `BME-CAL-2018` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2018-04-02 | closed | -   Lunes 2 de abril | `BME-CAL-2018` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2018-05-01 | closed | -   Martes 1 de mayo | `BME-CAL-2018` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2018-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2018` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2018-12-25 | closed | -   Martes 25 de diciembre | `BME-CAL-2018` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2018-12-26 | closed | -   Miércoles 26 de diciembre | `BME-CAL-2018` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2018-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2018` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | -   Martes 1 de enero | `BME-CAL-2019` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2019-04-19 | closed | -   Viernes 19 de abril | `BME-CAL-2019` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2019-04-22 | closed | -   Lunes 22 de abril | `BME-CAL-2019` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2019-05-01 | closed | -   Miércoles 1 de mayo | `BME-CAL-2019` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2019-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2019` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2019-12-25 | closed | -   Miércoles 25 de diciembre | `BME-CAL-2019` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2019-12-26 | closed | -   Jueves 26 de diciembre | `BME-CAL-2019` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2019-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2019` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | -   Miércoles 1 de enero | `BME-CAL-2020` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2020-04-10 | closed | -   Viernes 10 de abril | `BME-CAL-2020` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2020-04-13 | closed | -   Lunes 13 de abril | `BME-CAL-2020` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2020-05-01 | closed | -   Viernes 1 de mayo | `BME-CAL-2020` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2020-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2020` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2020-12-25 | closed | -   Viernes 25 de diciembre | `BME-CAL-2020` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2020-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55` | `BME-EVES-2020` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | -   Viernes 1 de enero | `BME-CAL-2021` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2021-04-02 | closed | -   Viernes 2 de abril | `BME-CAL-2021` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2021-04-05 | closed | -   Lunes 5 de abril | `BME-CAL-2021` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2021-12-24 | closed | -   Viernes 24 de diciembre | `BME-CAL-2021` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2021-12-31 | closed | -   Viernes 31 de diciembre | `BME-CAL-2021` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | -   Viernes 15 de abril | `BME-CAL-2022` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2022-04-18 | closed | -   Lunes 18 de abril | `BME-CAL-2022` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2022-12-26 | closed | -   Lunes 26 de diciembre | `BME-CAL-2022` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | -    Viernes 7 de abril | `BME-CAL-2023` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2023-04-10 | closed | -    Lunes 10 de abril | `BME-CAL-2023` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2023-05-01 | closed | -    Lunes 1 de mayo | `BME-CAL-2023` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2023-12-25 | closed | -    Lunes 25 de diciembre | `BME-CAL-2023` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2023-12-26 | closed | -    Martes 26 de diciembre | `BME-CAL-2023` | T1 | Operator's printed SIBE non-working date; same local civil trade date |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | − Lunes 1 de enero | `BME-CAL-2024` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2024-03-29 | closed | − Viernes 29 de marzo | `BME-CAL-2024` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2024-04-01 | closed | − Lunes 1 de abril | `BME-CAL-2024` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2024-05-01 | closed | − Miércoles 1 de mayo | `BME-CAL-2024` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2024-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55`; TAL ends `14:10` | `BME-EVES-2024` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2024-12-25 | closed | − Miércoles 25 de diciembre | `BME-CAL-2024` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2024-12-26 | closed | − Jueves 26 de diciembre | `BME-CAL-2024` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2024-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55`; TAL ends `14:10` | `BME-EVES-2024` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | − Miércoles 1 de enero | `BME-CAL-2025` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2025-04-18 | closed | − Viernes 18 de abril | `BME-CAL-2025` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2025-04-21 | closed | − Lunes 21 de abril | `BME-CAL-2025` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2025-05-01 | closed | − Jueves 1 de mayo | `BME-CAL-2025` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2025-12-24 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55`; TAL ends `14:10` | `BME-EVES-2025` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |
| 2025-12-25 | closed | − Jueves 25 de diciembre | `BME-CAL-2025` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2025-12-26 | closed | − Viernes 26 de diciembre | `BME-CAL-2025` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2025-12-31 | replacement blocks | ACCIONES (EQ): opening auction starts `8:30`; general continuous trading ends `13:55`; TAL ends `14:10` | `BME-EVES-2025` | T1 | Same civil event date; detailed principal-share timetable with the normal profile's conservative thirty-second auction convention; negotiated outside-hours trades and conditional security-specific extensions excluded — see scope above |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | − Jueves, 1 de enero | `BME-CAL-2026` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2026-04-03 | closed | − Viernes, 3 de abril | `BME-CAL-2026` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2026-04-06 | closed | − Lunes, 6 de abril | `BME-CAL-2026` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2026-05-01 | closed | − Viernes, 1 de mayo | `BME-CAL-2026` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2026-12-24 | unsourced | Announced half session; detailed hours to be published | `BME-CAL-2026` | T1 | Unconditional named half session, detailed principal-share hours unpublished; same local civil date |
| 2026-12-25 | closed | − Viernes, 25 de diciembre | `BME-CAL-2026` | T1 | Operator's printed SIBE non-working date; same local civil trade date |
| 2026-12-31 | unsourced | Announced half session; detailed hours to be published | `BME-CAL-2026` | T1 | Unconditional named half session, detailed principal-share hours unpublished; same local civil date |

### Documents

The operator page embeds `data-six-component-path` for its public
`assetstaxonomyfilter.results.json` endpoint. Its saved JSON names each public
PDF, its issuer Sociedad de Bolsas, title and publication metadata, including
all seventeen annual calendars and the nine detailed half-day instructions.
The operator index is discovery and provenance; the saved PDFs themselves
supply every session statement. Public bytes are saved under
`holidays/raw/bme-cash-2010-2026/` in the research store. `retrieval.json` records
each exact URL, retrieval timestamp and digest; text extractions are only aids.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `BME-RULES-2001` | baseline auction and scope | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2001/c20011uk.pdf> | 2026-10-11 23:05:49 UTC | T1 | `7cdd41f352fa41b144d3e2da497a898fe0e27d572605e81a90a80ac1b973c37c` |
| `BME-CAL-2010` | 2010-01-01..2010-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2009/2010.pdf> | 2026-10-11 23:08:17 UTC | T1 | `3f7bce5ec07594f0ba5f18aa4261c3ff4fa07d8fe6ada036274c495d509068ed` |
| `BME-CAL-2011` | 2011-01-01..2011-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2010/2011.pdf> | 2026-10-11 23:08:17 UTC | T1 | `d0d2483628265bdd1cc170ddf287a95ef5c40730884b156977e229ed8314dddd` |
| `BME-CAL-2012` | 2012-01-01..2012-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2011/2012.pdf> | 2026-10-11 23:08:16 UTC | T1 | `7009fca9417b1b84eeeeda5cf2004f2b2f3d8800e6988d0ddc506a81bcbbaba3` |
| `BME-EVES-2012` | 2012-12-24 and 2012-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2012/116-2012.pdf> | 2026-10-11 23:08:16 UTC | T1 | `c2759c5f50b05e10701377e9082e76428648063190405eeec0e32b511f660cfb` |
| `BME-CAL-2013` | 2013-01-01..2013-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2012/120-12.pdf> | 2026-10-11 23:08:16 UTC | T1 | `fc627d07d7d474de28ebe0890f7e9d71694908ddf8d4908fa2b805506f982a88` |
| `BME-EVES-2013` | 2013-12-24 and 2013-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2013/2013-116-io-24-y-31-diciembre-2013.pdf> | 2026-10-11 23:08:15 UTC | T1 | `10993dd82661080da9a99c31fb75bc8bcb0dafcc096752cc9355535b70a814b5` |
| `BME-CAL-2014` | 2014-01-01..2014-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2013/2013-117-io-calendario-de-sesiones-2014.pdf> | 2026-10-11 23:08:16 UTC | T1 | `e4e83f5a093d9bbbf484764fdd12d84e21370a7e17f67a097e76e581405f7323` |
| `BME-EVES-2014` | 2014-12-24 and 2014-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2014/142-14.pdf> | 2026-10-11 23:08:15 UTC | T1 | `bc7ea433b3ab6a82c5aece04b8e9b37568f95d7817dc56d24e93114a8af2d1db` |
| `BME-CAL-2015` | 2015-01-01..2015-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2014/2014-147-io-calendario-de-sesiones-2015.pdf> | 2026-10-11 23:08:14 UTC | T1 | `96265827d7c059ab9557606d287967529a06452cb36c8dd568632705a4df877c` |
| `BME-EVES-2015` | 2015-12-24 and 2015-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2015/162-15.pdf> | 2026-10-11 23:08:14 UTC | T1 | `3575b22a36eff9edfb9d6cf2996714f4f99086b418de1dcaff099fd475372234` |
| `BME-CAL-2016` | 2016-01-01..2016-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2015/166-2015.pdf> | 2026-10-11 23:08:13 UTC | T1 | `1f2a6a7470c2762ae064b0f4781ab6628c950a9bb743bde1b593c06c38631092` |
| `BME-CAL-2017` | 2017-01-01..2017-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2016/139-16.pdf> | 2026-10-11 23:08:13 UTC | T1 | `11a7af127038bf91dd0bad349776c16371e8c4b8be1b09b28aadd6db1dd25130` |
| `BME-CAL-2018` | 2018-01-01..2018-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2017/io-84-2017.pdf> | 2026-10-11 23:08:13 UTC | T1 | `0c4a2e2f2255b059a8b502cc2a97d830a05158239b52d3a71e18d16c2abeb012` |
| `BME-EVES-2018` | 2018-12-24 and 2018-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2018/2018-80-io-24-y-31-diciembre-2018.pdf> | 2026-10-11 23:08:12 UTC | T1 | `328feee50b4a73aae363cc7cb964aba3e9c5447856d80af849e17077bc5bc182` |
| `BME-CAL-2019` | 2019-01-01..2019-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2018/82-18.pdf> | 2026-10-11 23:08:10 UTC | T1 | `2a59544e3e71dc6bbad05acb635a2963c21c3006130c0d52b354f26a24d4947d` |
| `BME-EVES-2019` | 2019-12-24 and 2019-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2019/84-19.pdf> | 2026-10-11 23:08:11 UTC | T1 | `3b37a8fab55db1149fb97a811586e13c2d41b874138fcd16d94126222970eec1` |
| `BME-CAL-2020` | 2020-01-01..2020-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2019/83-19.pdf> | 2026-10-11 23:08:11 UTC | T1 | `e6f13aea57e8aabc7daa85994bd92cc58c18653716b1d09fa6eaa5dc7e5ddbac` |
| `BME-EVES-2020` | 2020-12-24 and 2020-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2020/2020-63-io-24-y-31-diciembre-2020.pdf> | 2026-10-11 23:08:09 UTC | T1 | `f028a74f2ed3a1c5fc70a1e734ac8d3bd35573d8b40965bf68fa60f46ee3844c` |
| `BME-CAL-2021` | 2021-01-01..2021-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2020/61-20.pdf> | 2026-10-11 23:08:11 UTC | T1 | `72389ef38a8b81bcdb91749b935016b0a36a549ebe0bf5ba4709ad7d30632faa` |
| `BME-CAL-2022` | 2022-01-01..2022-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2021/2021-53-calendario-2022.pdf> | 2026-10-11 23:08:09 UTC | T1 | `a28d75272c05715edec7c71b078e7c1961bf495bc473c0c94d418a86ddba1dde` |
| `BME-CAL-2023` | 2023-01-01..2023-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2022/2022-44.pdf> | 2026-10-11 23:08:09 UTC | T1 | `daaa485e6ff4dc5df8dd0417267b24880b90f5a7800c0c25fba7c3957e4fb713` |
| `BME-CAL-2024` | 2024-01-01..2024-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2023/2023-45-calendario-de-sesiones-2024.pdf> | 2026-10-11 23:08:09 UTC | T1 | `2edc108e44e4ecb459a3d79c7b09e12419d69d3473019d8eeb19228026e1ad08` |
| `BME-EVES-2024` | 2024-12-24 and 2024-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2024/2024-47-io-horarios-24-y-31.pdf> | 2026-10-11 23:08:07 UTC | T1 | `6bbab4bf646528996eb5c2f5cadefc32936a6e02caf7a16f4eeb0e8e7f68deb1` |
| `BME-CAL-2025` | 2025-01-01..2025-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2024/2024-52-io-calendario-sesiones-2025-es.pdf> | 2026-10-11 23:08:07 UTC | T1 | `c06054989be4ec343f525b13d0ba85ccf2ee0a624a46b874e784005d33312196` |
| `BME-EVES-2025` | 2025-12-24 and 2025-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2025/2025-64-io-horarios-24-y-31.pdf> | 2026-10-11 23:08:07 UTC | T1 | `f1f9f5e22c1aaed4da98a27e299cd062cce2af24aeba72bdc2df5569086ff609` |
| `BME-CAL-2026` | 2026-01-01..2026-12-31 | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2025/2025-65-io-calendario-de-sesiones-2026.pdf> | 2026-10-11 23:08:07 UTC | T1 | `9067c20484938b1c5c3be7a29c7e72f065eec97ddc2bbba22619c492ba91a401` |
| `BME-REGULATION-INDEX-2026-10-11` | public discovery index | <https://www.bolsasymercados.es/es/bme-exchange/regulacion/renta-variable/_jcr_content/root/containers/container/assetstaxonomyfilter.results.json> | 2026-10-11 UTC | T1 | `cee89745ab76da14b5507f1b9b0f2246d5e766f27a4ac4ff71d63b278ec2c925` |
| `BME-TAL-RULES-2023` | nominal post-auction TAL phase | <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2023/circular-1-23-english.pdf> | 2026-10-11 23:22:29 UTC | T1 | `bb91f0bf4edfdb67e3a39c2e940f183d3db89363b44fcff6c4923a1d23511fc9` |
