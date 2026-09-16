// SPDX-License-Identifier: MIT-0

//! CME, CBOT, COMEX and NYMEX venue holiday rows, 2010-2027.
//!
//! One module per venue: `cme.rs`, `cbot.rs`, `comex.rs`, `nymex.rs`.
//!
//! `Exchange::Cme`, `Cbot`, `Comex` and `Nymex` each route several product
//! families onto one schedule, and a holiday row is a statement about **one**
//! clock. A venue row may therefore be stated only where the families that
//! route to the venue say the same thing: this module is the **intersection**
//! of those families' tables (design memo D17), not a union and not a
//! compromise between them.
//!
//! Every row here is **derived, not retrieved**. The operator statements behind
//! them are the documents the family tables already cite: CME Group's own
//! holiday-calendar PDFs, at tier **T1** for the 2010-2012 rows; CME's own
//! published Globex holiday schedules, at tier **T1** for the 2016-2018,
//! 2019-2021 and 2022-2024 rows, which are the design memo's D17 intersection
//! of the routed families' T1 rows rather than a retrieval of their own, except
//! for the 2022-2024 dates the trading-hours service answers (the three 2023
//! `Unsourced` markers every venue table carries, and the 2024 service dates,
//! whose count differs by venue); and that service, at tier **T2** for the
//! 2025-2027 ones. Nothing in this module rests on an artifact a family module
//! does not already carry, and each venue declares the five eras as five
//! coverage windows — the 2013-2015 interval between them is audited by no wave
//! and ships no row. There is accordingly no venue evidence of its own to add:
//! the four venue evidence files record the derivation, the routing, the
//! intersection rule and every date the intersection drops.
//!
//! On the 2016-2018 era's **thirty-six dates** the CME and CBOT tables state
//! nine `Closed` rows — the dates every routed family shut — and withhold the
//! other twenty-seven as [`HolidayKind::Unsourced`], while `comex` and `nymex`
//! carry `globex_energy`'s thirty-one rows unchanged. A date is disputed
//! wherever the routed families do not state the same row, and the two tables
//! dispute different dates, because they route different families.
//!
//! **CME's twenty-seven** are three shapes: eighteen dates on which the four
//! financial families halt at 12:00 CT while `globex_grains` is shut outright;
//! four on which `globex_grains` closes at 12:05 CT or reopens at 08:30 CT
//! while `globex_equity_index` closes at 12:15 CT or reopens at 15:30 CT and
//! `globex_energy` closes at 12:45 CT — the three Thanksgiving Fridays and
//! 2018-12-24; and five on which `globex_grains` alone states a row and
//! `globex_equity_index` states a *different* one, a 12:15 CT close on
//! 2017-07-03 and 2018-07-03 or a 15:30 CT open on 2018-12-26, with the other
//! four families audited normal (2016-12-23 and 2017-12-22 are grains' alone,
//! every financial family having audited them normal).
//!
//! **CBOT's twenty-seven** are the two families' own disagreements:
//! `globex_grains` closes at 12:05 CT where `globex_interest_rates` closes at
//! 12:15 CT — the three Thanksgiving Fridays and 2018-12-24 — and states a row
//! on 2016-12-23, 2017-07-03, 2017-12-22, 2018-07-03 and 2018-12-26, dates the
//! rate leg audited normal. A routed family **with no table for an era
//! abstains** rather than disputing: `globex_livestock` covers 2010-2012 and
//! 2025-2027 and not 2016-2018, so it neither supplies nor withholds a venue
//! row there, and the families that do cover the era decide each date.
//! Metals and energy are one key and CME prints them as one product row, so the
//! `globex_energy` rows carry through untouched.
//!
//! On the 2019-2021 era's **forty-two dates** the CME and CBOT tables state
//! eight `Closed` rows — the same eight on both, the dates every routed family
//! shut — and withhold the other thirty-four as [`HolidayKind::Unsourced`],
//! while `comex` and `nymex` carry `globex_energy`'s thirty-five rows
//! unchanged: nine closures, twenty-three early closes and the family's own
//! three `Unsourced` markers. Every row in the era is **T1**, from CME's own
//! published Globex holiday schedules — the compact sheets inside the 2019,
//! 2020 and 2021 annual bundles, plus the December-2018 supplement that
//! carries 1-2 January 2019 — so unlike 2022-2024 no date here rests on the
//! trading-hours service and the era holds no T2 row. All six CME families and
//! both CBOT families cover the era, so no routed family abstains.
//!
//! **CME's thirty-four** are four shapes: eighteen Monday and Thursday
//! holidays, on which the four financial families halt at 12:00 CT while
//! `globex_grains` and `globex_livestock` are shut outright; five on which
//! `globex_grains` alone states a day-after-closure `late_open(08:30)` and the
//! other five families audited the date normal (2019-01-02, 2019-07-05,
//! 2019-12-26, 2020-01-02 and 2021-07-06); eight dates whose stated instants
//! all differ — the three Thanksgiving Fridays, the two Christmas Eves, the two
//! July days 2019-07-03 and 2020-07-02, and 2021-04-02, on which equity closes
//! at 08:15 CT, FX and rates at 10:15 CT, and energy, grains and livestock are
//! shut all day; and the three Juneteenth dates, 2019-06-19, 2020-06-19 and
//! 2021-06-19, on which every routed family states the same `Unsourced` marker,
//! so the venue repeats their agreement rather than treating it as a dispute.
//!
//! **CBOT's thirty-four** are the same eighteen Monday and Thursday holidays,
//! the same five grains-only late opens, the same three Juneteenth agreements,
//! and eight dates on which its two families state different rows: the three
//! Thanksgiving Fridays, where the grain day session reopens at 08:30 CT and
//! closes at 12:05 CT while the rate leg halts at 12:15 CT; the two Christmas
//! Eves, where grains closes at 12:05 CT and the rate leg at 12:15 CT;
//! 2019-07-03 and 2020-07-02, where grains closes at 12:05 CT and the rate leg
//! audited the date normal; and 2021-04-02, where grains is shut and the rate
//! leg closes at 10:15 CT.
//!
//! On the 2022-2024 era's **forty-one dates** the CME table states seven
//! `Closed` rows — the same shape as 2016-2018 — and withholds the other
//! thirty-four as [`HolidayKind::Unsourced`]; `comex` and `nymex` carry
//! `globex_energy`'s thirty-three rows unchanged, three of which are that
//! family's own `Unsourced` markers. A withheld date is not automatically a
//! dispute: on three of them — 2023-01-16, 2023-02-20 and 2023-04-07 — every
//! routed family states the same `Unsourced` marker, so the venue repeats their
//! agreement. That leaves **CME thirty-one disputes**: nineteen Monday and
//! Thursday holidays, four half-days (2022-11-25, 2023-11-24, 2024-11-29 and
//! 2024-12-24), and eight dates on which one family states a row the others
//! audited normal. **CBOT's twenty-nine** are the same nineteen Monday and
//! Thursday holidays, the same four half-days, and the same six
//! `globex_grains` half-days the rate leg audited normal; it does not see the
//! two equity-only dates, because it routes `globex_interest_rates` rather than
//! `globex_equity_index`. Each table withholds its own dispute count plus the
//! same three agreed markers.
//!
//! # The routing
//!
//! The routing is a **decision recorded here, not a fact the crate can derive**.
//! `hours_for_exchange(Exchange::Cme, _)` resolves to a schedule, not to a
//! membership list, and by the consumer contract the map from product families
//! to venues belongs to `SharurPlatform`. The list below is the plan's Wave 7
//! decision (`docs/plans/2026-09-12-path-to-release.md` §3 2.1); each venue's
//! evidence file states what the choice costs and what would change it.
//!
//! | Venue | Families | Why these |
//! |---|---|---|
//! | `cme` | `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` | the plan's six. The two CME families left out are `globex_nikkei_225_dollar` and `globex_cryptocurrency`; what each would cost is stated below |
//! | `cbot` | `globex_grains`, `globex_interest_rates` | the two families whose sessions the venue profile is built from |
//! | `comex` | `globex_energy`, metals half | the venue's documented scope |
//! | `nymex` | `globex_energy`, energy half | the venue's documented scope |
//!
//! COMEX and NYMEX share one family key, because CME prints the two halves as
//! one product row on every date either table audits: no date is dropped for a
//! disagreement between `GC` and `CL`. Their two tables hold the same rows —
//! they are separate tables rather than one alias, because a venue's table is a
//! decision about that venue — and
//! `the_energy_venues_carry_the_family_table_unchanged` holds each against the
//! family's own answers.
//!
//! **What the two excluded families would cost is known, and the 2022-2024 era
//! changed the answer.** `globex_nikkei_225_dollar` agrees with
//! `globex_equity_index` row for row in 2016-2018 (the same thirty-four rows)
//! and in 2025-2027 (the same thirty-seven), so routing it would have moved no
//! answer in either era. In 2022-2024 it is **not** answer-neutral: its thirteen
//! 2024 `Unsourced` rows — the wave did not retrieve the 2024 Nikkei channel —
//! would turn the era's two service-answered closures, 2024-03-29 and
//! 2024-12-25, into `Unsourced`, and would add a third on 2024-12-31, a date
//! the six-family intersection audits normal. Those three dates therefore rest
//! on the routing decision rather than on the intersection alone, and a
//! re-routing that admitted Nikkei would have to state them differently.
//! `globex_cryptocurrency` moves no answer in any era: it shares all nine
//! 2025-2027 closures and its fourteen 2022-2024 rows change nothing, while it
//! states nothing on seventeen of the other 2025-2027 dates and would join
//! seventeen more disagreements without moving an answer.
//!
//! # Two kinds of row
//!
//! **Where every routed family states the same row**, the venue ships that row.
//! In 2025-2027 that is the nine Globex full closures; in 2010-2012 it is the
//! six that CME published for those years; in 2016-2018 and 2022-2024 it is
//! nine and seven dates. On each, all six CME families, both CBOT families, or
//! the single energy family behind COMEX/NYMEX agree to the status, and the
//! venue is closed. A routed
//! family whose table does not cover an era **abstains** there: it has no
//! answer, so it cannot make the families that do cover the era dispute one,
//! and it neither supplies nor withholds a venue row. A family whose table does
//! cover a date and holds no row has audited it normal, which is an answer and
//! does dispute a row.
//!
//! **Where they disagree**, the venue ships [`HolidayKind::Unsourced`]. That is
//! neither silence nor a compromise instant. An audited window is contiguous,
//! so a date carrying no row inside one is the positive claim that it was
//! audited normal, which on these dates is false; and any single instant would
//! be wrong for someone, because on 2026-12-24 the families' sourced closes are 12:05, 12:15
//! and 12:45 CT at once, so a row at either end stops someone early or runs
//! someone past their own close. `Unsourced`
//! clips nothing, answers nothing, and tells the caller what the crate knows:
//! the date is special and the venue has no one answer for it. ICE Futures
//! U.S.'s venue table, which shipped first, is the precedent, and the
//! disagreement behind each row is named in the venue's evidence file. The same
//! marker ships where every routed family states `Unsourced` itself — the three
//! 2023 dates of the 2022-2024 era — because that is agreement too: the
//! families say the same thing, and the venue repeats their answer rather than
//! inventing a row or claiming the date was audited normal.
//!
//! Evidence: [`docs/evidence/cme.md`](../../../../../docs/evidence/cme.md),
//! [`cbot.md`](../../../../../docs/evidence/cbot.md),
//! [`comex.md`](../../../../../docs/evidence/comex.md),
//! [`nymex.md`](../../../../../docs/evidence/nymex.md).

mod cbot;
mod cme;
mod comex;
mod nymex;

pub(crate) use cbot::CBOT;
pub(crate) use cme::CME;
pub(crate) use comex::COMEX;
pub(crate) use nymex::NYMEX;
