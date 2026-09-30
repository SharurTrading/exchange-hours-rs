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
//! compromise between them — with one exception the 2026-09-30 profile-clock
//! decision added (AGENTS.md, LAW-HOLIDAY-SCOPE, issue #153): each venue
//! speaks for its **profile clock** — `cbot`'s is the grain grid, `cme`'s the
//! equity-index grid, `comex`'s and `nymex`'s the energy grid — and a family
//! whose clock the venue does not serve cannot withhold a date the clock
//! family audits normal. The venue answers every date its own clock answers.
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
//! does not already carry, and each venue declares the six eras as six
//! coverage windows — every interval from 2010-01-01 to 2027-12-31 is inside
//! one of them. There is accordingly no venue evidence of its own to add:
//! the four venue evidence files record the derivation, the routing, the
//! intersection rule and every date the intersection drops.
//!
//! On the 2016-2018 era's **thirty-six dates** the CME and CBOT tables state
//! nine `Closed` rows — the dates every routed family shut — and withhold the
//! other dates as [`HolidayKind::Unsourced`] (twenty-five for `cme`,
//! twenty-seven for `cbot`), while `comex` and `nymex` carry
//! `globex_energy`'s thirty-one rows unchanged. A date is disputed
//! wherever the routed families do not state the same row, and the two tables
//! dispute different dates, because they route different families.
//!
//! **CME's twenty-five** are three shapes: eighteen dates on which the four
//! financial families halt at 12:00 CT while `globex_grains` and
//! `globex_livestock` are shut outright; four on which grains closes at
//! 12:05 CT, equity, FX, rates and livestock at 12:15 CT and energy at 12:45 CT
//! — the three Thanksgiving Fridays and 2018-12-24; and three on which the
//! stated rows disagree — a 12:15 CT close from equity and livestock on
//! 2017-07-03 and 2018-07-03 and an equity 15:30 CT reopen with a grains 08:30
//! CT reopen on 2018-12-26, dates some of the other families audited normal.
//! (The two dates on which grains' 12:05 CT and livestock's 12:15 CT closes
//! were the only rows stated — 2016-12-23 and 2017-12-22 — were disputes under
//! the earlier rule; the profile-clock rule retires them, because the venue's
//! own clock audited both normal.)
//!
//! **CBOT's twenty-seven** are the two families' own disagreements:
//! `globex_grains` closes at 12:05 CT where `globex_interest_rates` closes at
//! 12:15 CT — the three Thanksgiving Fridays and 2018-12-24 — and states a row
//! on 2016-12-23, 2017-07-03, 2017-12-22, 2018-07-03 and 2018-12-26, dates the
//! rate leg audited normal. Metals and energy are one key and CME prints them
//! as one product row, so the `globex_energy` rows carry through untouched.
//!
//! On the 2019-2021 era's **forty-two dates** the CME and CBOT tables state
//! eight `Closed` rows — the same eight on both, the dates every routed family
//! shut — and withhold the other dates as [`HolidayKind::Unsourced`]
//! (twenty-eight for `cme`, thirty-four for `cbot`),
//! while `comex` and `nymex` carry `globex_energy`'s thirty-five rows
//! unchanged: nine closures, twenty-three early closes and the family's own
//! three `Unsourced` markers. Every row in the era is **T1**, from CME's own
//! published Globex holiday schedules — the compact sheets inside the 2019,
//! 2020 and 2021 annual bundles, plus the December-2018 supplement that
//! carries 1-2 January 2019 — so unlike 2022-2024 no date here rests on the
//! trading-hours service and the era holds no T2 row. All six CME families and
//! both CBOT families cover the era, so no routed family abstains.
//!
//! **CME's twenty-eight** are three shapes: eighteen Monday and Thursday
//! holidays, on which the four financial families halt at 12:00 CT while
//! `globex_grains` and `globex_livestock` are shut outright; seven dates whose
//! families state different rows — the three Thanksgiving Fridays, the two
//! Christmas Eves and 2019-07-03 on differing instants, and 2021-04-02, where
//! equity closes at 08:15 CT and FX and rates at 10:15 CT while energy, grains
//! and livestock are shut all day; and the three Juneteenth dates, 2019-06-19,
//! 2020-06-19 and 2021-06-19, on which every routed family states the same
//! `Unsourced` marker, so the venue repeats their agreement rather than
//! treating it as a dispute. (The five grains-only day-after-closure late
//! opens and 2020-07-02, where grains' and livestock's closes were the only
//! rows stated, were disputes under the earlier rule; the profile-clock rule
//! retires them, because the venue's own clock audited those dates normal.)
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
//! twenty-eight as [`HolidayKind::Unsourced`]; `comex` and `nymex` carry
//! `globex_energy`'s thirty-three rows unchanged, three of which are that
//! family's own `Unsourced` markers. A withheld date is not automatically a
//! dispute: on one of them — 2023-01-16 — every routed family states the same
//! `Unsourced` marker, so the venue repeats their agreement. On 2023-02-20 and
//! 2023-04-07 the operator's own unsuffixed holiday sheets state the two
//! families different session shapes, so the venue withholds the
//! disagreement. That leaves **CME twenty-seven disputes**: nineteen Monday
//! and Thursday holidays, four half-days (2022-11-25, 2023-11-24, 2024-11-29
//! and 2024-12-24), two dates on which the clock family itself states a row
//! the others audited normal (`globex_equity_index`'s 12:15 CT closes of
//! 2023-07-03 and 2024-07-03), and the two 2023 sheet-stated dates.
//! **CBOT's thirty-one** are the same nineteen Monday and Thursday holidays,
//! the same four half-days, the same six `globex_grains` half-days the rate
//! leg audited normal, and the same two 2023 dates; it does not see the two
//! equity-only dates, because it routes `globex_interest_rates` rather than
//! `globex_equity_index`. Each table withholds its own dispute count plus the
//! one agreed marker.
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
//! `globex_equity_index` row for row in 2016-2018 (the same thirty-four rows),
//! and its 2025-2027 table differs from that family's on exactly three dates:
//! the Saturday trade dates 2026-06-22, 2026-07-06 and 2027-06-21 that Stage 4
//! (#116) added as complete-day replacement rows. It ships **forty** dated rows
//! at 2025 and later, and on every other date the two both state, they state
//! the same row. Routing it would therefore have moved no answer in 2016-2018
//! and none on the dates the two state alike, and on the three Saturdays it can
//! add no shipped row either: the families this venue routes do not all state
//! Nikkei's row there, so the intersection can only report `Unsourced`. In
//! 2022-2024 it is **not** answer-neutral: its thirteen 2024 `Unsourced` rows
//! — the wave did not retrieve the 2024 Nikkei channel — would turn the era's
//! two service-answered closures, 2024-03-29 and 2024-12-25, into `Unsourced`,
//! and would add a third on 2024-12-31, a date the six-family intersection
//! audits normal. Those three dates therefore rest on the routing decision
//! rather than on the intersection alone, and a re-routing that admitted Nikkei
//! would have to state them differently.
//! `globex_cryptocurrency` moves no answer in any era: it shares all nine
//! 2025-2027 closures and its fourteen 2022-2024 rows change nothing, while it
//! states nothing on twenty of the other 2025-2027 dates and would join
//! twenty more disagreements without moving an answer.
//!
//! # Three kinds of date
//!
//! **Where every routed family states the same row**, the venue ships that row.
//! In 2025-2027 that is the nine Globex full closures; in 2010-2012 it is the
//! six that CME published for those years; in 2016-2018 and 2022-2024 it is
//! nine and seven dates. On each, all six CME families, both CBOT families, or
//! the single energy family behind COMEX/NYMEX agree to the status, and the
//! venue is closed. A routed
//! family whose table does not cover an era **abstains** there: it has no
//! answer, so it cannot make the families that do cover the era dispute one,
//! and it neither supplies nor withholds a venue row.
//!
//! **Where the profile clock audits a date normal**, the venue ships no row —
//! the date is audited normal for the venue — whatever a family whose clock
//! the venue does not serve states (the 2026-09-30 decision, issue #153; the
//! data move is #242). The venue's own schedule is the clock's grid, so the
//! clock's answer is the venue's; the dissenting arrangement stays in its own
//! family table, where the consumer's exact-family routing (#118) reads it.
//! This is the rule that retired the pre-2025 artefact rows: eighty-five in
//! `cme` and fifty-nine in `cbot` across every era.
//!
//! **Where the routed families disagree on a date the clock answers** — the
//! clock states a row that another non-abstaining routed family does not
//! match — the venue ships [`HolidayKind::Unsourced`]. That
//! is
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
