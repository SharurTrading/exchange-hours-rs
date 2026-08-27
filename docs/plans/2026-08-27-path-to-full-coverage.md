<!-- SPDX-License-Identifier: MIT-0 -->

# The path to 100% exchange-level accuracy

Plan date: 2026-08-27 · Branch context: built on `feat/architecture-hardening`
(PR #22: single-path routing, knowledge-bound rows, no-split fence,
pre-coding law).

## What "100%" means here — and what it permanently does not

Measurably, the end state is:

1. **Every automated order-capable system** of every venue is in exactly one
   place — its venue envelope or its own identity — verified by audit.
2. **Every non-synthetic ledger row is `Primary`**: a complete dated
   amendment chain since January 2010, or a documented permanent
   knowledge-bound row the ledger names as the honest answer.
3. **Holiday and special-day topology** is servable through the exception
   layer: the engine ships, and built-in data exists for every venue that
   publishes a primary holiday calendar, with coverage and finality
   metadata.
4. **Cancellation-only windows are decided** — excluded by recorded
   evidence, or represented by their own additive surface.

Permanent non-goals, because pulling them in would *reduce* accuracy:
per-product grids under a venue identity (that is `MarketHoursKey`'s job),
per-security auction uncross seconds (microstructure), unplanned intraday
halts (not published schedules), history before January 2010, and
wall-clock/"now" semantics anywhere in the crate. Accuracy is also a
maintenance property: the review cutoff must keep advancing and `Scheduled`
markers must keep cycling — this plan builds toward a state, not a one-time
destination.

Phases 1–2 and 3 are independent of Phases 4–6; the merge order at the end
makes the only hard dependency explicit (Phase 2 needs Phase 1's findings;
Phases 5–6 need Phase 4's engine).

---

## Phase 1 — System-coverage audit (foundational, one-time)

The rule already exists (AGENTS.md cash-equity envelope: availability union
of automated order-capable systems; separately modeled identities excluded).
Completeness is a verification question.

- [ ] For each of the 93 non-synthetic rows, enumerate the venue's automated
      order-capable systems from its primary documentation (rulebook hours
      sections, system timetables, member notices) — the source sets in
      `docs/schedules/sources.md` are the entry points.
- [ ] For each system, verify it is in exactly one place: inside the row's
      documented envelope, or an identity the crate already models. Record
      the enumeration beside the row in `docs/schedules/verification.md`
      (a short "systems in scope" note per row).
- [ ] Produce the discrepancy list: systems in neither place. Each becomes
      either an envelope amendment (schedule fix path) or a new identity
      (full AGENTS.md venue-addition change set) — never a silent widening.
- [ ] While enumerating, capture two side-lists for Phase 2 and the
      exclusion record: any **cancellation-only windows** observed, and any
      **settlement/fixing windows** observed (the latter to confirm the
      exclusion is documented, not to model them).
- [ ] Update the ledger rows touched; changelog entries for any envelope
      corrections; full verification gates.

## Phase 2 — Cancellation-only decision (rides on Phase 1's findings)

- [ ] Assess the Phase 1 capture list against two gates: **material** (the
      window meaningfully affects a trading platform's order management)
      and **sourced** (a primary timetable states it).
- [ ] **Expected outcome — none pass both gates:** record the decision as a
      dated convention note in AGENTS.md (cash-equity envelope section) so
      the exclusion is evidence-based rather than assumed, and close this
      phase.
- [ ] **If a venue passes both gates:** design an additive surface — a
      fourth rule set (`cancel_only`) plus a `SessionState` variant, never a
      distortion of `OrderEntry` (whose doc promises entry/amend/cancel) or
      of `is_accepting_orders`. `SessionState` is `#[non_exhaustive]`, so
      this is a minor release with the full venue/test/docs change set.
      Decide `is_accepting_orders`'s answer for the window explicitly and
      fence it with tests.

## Phase 3 — The Partial-history backlog (independent; batched by family)

46 ledger rows currently carry `Partial` (27 venue rows plus the key
families). Each closes one of two ways: a sourced dated chain, or a
documented permanent knowledge-bound row. Batches are ordered by
evidence-leverage — several rows likely share one industry timeline, so one
notice may close many.

- [ ] **Batch A — US options queue onsets** (the 20 `options.rs` rows).
      Investigate the coordinated industry events first: the 2016
      early-matching move, any GTH-era queue changes, operator system
      settings archives. One dated artifact may close a dozen rows.
- [ ] **Batch B — CME family onsets** (equity-index, grains, energy/metals,
      FX, interest rates, livestock, cryptocurrency, NKD): the Sunday
      16:00 CT pre-open queue and PCP onsets. CME advisory archives are the
      likely single source.
- [ ] **Batch C — ICE US softs and USDX** (sugar, coffee, cocoa, cotton,
      FCOJ, dollar index): each family's own queue-onset gaps.
- [ ] **Batch D — SGX equity-index keys** (five rows).
- [ ] **Batch E — US equity early sessions** (Cboe ×2, NYSE ×4): the 07:00
      early session and 06:30/02:30 queue onsets.
- [ ] Per row: follow `docs/schedules/updating.md` (open the monitoring
      sources, review everything since the row's `Reviewed on` date); add
      dated rows + `HISTORICAL_CUTOVERS`/instant entries + both-sides
      tests, or leave the knowledge-bound row and upgrade the ledger note to
      name the gap as permanent.
- [ ] Advance the README assurance counts batch by batch (66 → … of 93);
      `Fixed` changelog entries per correction; full verification gates per
      merged batch.

## Phase 4 — Exception-session engine (caller-owned records, no evidence burden)

Mirror how `DayPolicy` shipped: engine and public surface first, zero
built-in data. This immediately lets SharurPlatform encode its own holiday
tables.

- [ ] Types per the `date-exceptions.md` contract: an exception record
      carrying `KnownNormal` / `Closed` / `ReplaceSessions` /
      `OutOfCoverage` for its dates, where `ReplaceSessions` is a complete
      ordered set of regular/extended blocks with explicit local-day
      offsets and a trade-date assignment — same SSM representation,
      end-exclusive closes, and DST bias as normal profiles.
- [ ] Caller-owned static table format (the `StaticDayPolicy` analogue)
      with validation: coverage bounds published, block sanity checked,
      closed/replaced mutual exclusivity enforced by construction.
- [ ] Engine integration: a replacement resolution path beside
      `resolve_rule_bounds`'s clamp path, so a replaced trade date serves
      its blocks instead of the profile rules — including wrapped-session
      and trade-date-assignment interactions, and the CME-crypto
      following-business-day convention.
- [ ] **Recorded decision — precedence:** the caller's `DayPolicy` overlays
      the exception layer exactly as it overlays normal weeks (boundary
      clamps on top of replacement blocks). Two replacement layers never
      compose; opt-in surfaces stay singular.
- [ ] Fence tests over the two documented impossibles made possible: CME's
      2015 Thanksgiving pause-and-reopen, and Nasdaq's 2011 regular-only
      early close with extended continuing.
- [ ] Public docs (README, date-exceptions.md updated from "path to" to
      "shipped for caller-owned data"); changelog **Added**; full gates.

## Phase 5 — Built-in exception data: pilot (licensing-gated)

- [ ] Pick 1–2 pilot venues whose holiday calendars are primary, public,
      and unencumbered (NYSE and CME Group are the natural candidates).
- [ ] Licensing check per source before any data drives runtime answers —
      public access is not permission to republish a licensed feed; the
      check outcome is recorded beside the data.
- [ ] Ship the pilot tables with full metadata: source, revision,
      review-date, finality, first/last covered trade date, scoped to one
      exact `CalendarSource`.
- [ ] AGENTS.md amendment with this phase: the modeling convention
      "Built-in profiles contain normal-week data, not holidays" gains its
      exception-layer clause — profiles stay normal-week; holiday truth
      lives in the opt-in, metadata-carrying exception layer. The venue
      change-set checklist gains its exception-data step.
- [ ] Ledger gains an exception-coverage basis per pilot row; changelog;
      full gates.

## Phase 6 — Exception-data rollout

- [ ] Region batches mirroring Phase 3's leverage ordering; per venue:
      evidence per `updating.md`, licensing check, tables, ledger row,
      changelog.
- [ ] Venues with no published primary calendar stay explicitly
      `OutOfCoverage` — the provider contract makes absence honest rather
      than silently normal.
- [ ] README assurance section gains exception-coverage counts alongside
      the history counts; full gates.

---

## Verification (every phase, in order)

```bash
cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check
cargo +1.95 check --all-targets
```

## Merge order and gates

1. Phase 1 — first; its findings feed Phase 2 and seed the exclusion record.
2. Phase 2 — after Phase 1; expected to close as a documentation decision.
3. Phase 3 — independent; batches merge in A→E order, interleaving freely
   with Phases 4–6.
4. Phase 4 — before any built-in exception data.
5. Phase 5 — after Phase 4; the licensing check is a hard gate before data
   ships.
6. Phase 6 — after Phase 5, batch by batch.

## Plan lifecycle

Working document: check boxes off as batches land, and delete this file once
Phase 6's final batch merges — the ledger, AGENTS.md, and date-exceptions.md
are the permanent record. If the effort stalls, the unchecked boxes are the
honest statement of remaining scope.
