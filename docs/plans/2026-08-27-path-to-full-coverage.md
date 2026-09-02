<!-- SPDX-License-Identifier: MIT-0 -->

# The path to 100% exchange-level accuracy

Plan date: 2026-08-27 · Branch context: built on `feat/architecture-hardening`
(PR #22: single-path routing, knowledge-bound rows, no-split fence,
pre-coding law).

## What "100%" means here — and what it permanently does not

Measurably, the end state is:

1. **Every automated order-capable system** of every venue is in exactly one
   place — its venue envelope or its own identity — verified by audit.
2. **Every non-synthetic ledger row reaches its honest terminal state**:
   `Primary` with a complete dated amendment chain since January 2010, or
   `Partial` with a precisely named, permanent knowledge-bound gap. A
   knowledge-bound row *is* a history gap, so its ledger row stays
   `Partial` and never upgrades without dated primary evidence — the
   target is not "93 `Primary` rows".
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

Phases 1–2 and 3 are independent of Phases 4–6 in principle; the merge order at
the end makes the structural dependencies explicit (Phase 2 needs Phase 1's
findings; Phases 5–6 need Phase 4's engine).

**Sequencing decision, 2026-08-31: Phase 3 completes before Phase 4 starts.**
Every incomplete exchange row is closed to its terminal state — a sourced grid
carried to the January-2010 floor, or a precisely named permanent gap — before
any work begins on holidays and the exception layer. Normal-week accuracy back
to 2010 is the release-blocking goal; holiday topology is the next project, not
a parallel one.

---

## Phase 1 — System-coverage audit (foundational, one-time)

The rule already exists (AGENTS.md cash-equity envelope: availability union
of automated order-capable systems; separately modeled identities excluded).
Completeness is a verification question.

**Method established and first tranche done, 2026-09-02.** The nineteen US
cash-equity rows (`nasdaq` through `blue_ocean_ats`) are enumerated and carry a
`Systems in scope` clause; the repeatable eight-step method, the discrepancy
list, and both side-lists live in the **System-coverage audit** section of
`docs/schedules/verification.md`. Five systems landed in neither place — NYSE
Bonds, the NYSE and NYSE American Off-Hours Trading Facilities, IEX Options, and
MIAX Pearl Equities' 03:30 Live Order Window — and are routed, not absorbed;
every one is a window the crate omits rather than one it wrongly serves, so the
dated surface stays conservative. **Cancellation-only windows observed: none**,
which is the evidence Phase 2 was waiting for. The remaining 74 rows follow the
same method; no `Reviewed on` value was advanced, because an enumeration pass
opens a venue's system-inventory sources, not every source set a row review
needs.

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


### Resolved 2026-09-01: do NOT bound the timeline at the January-2010 floor

Raised in review on PR #25 against `cme_nikkei.rs` and answered there rather
than patched, because it is not specific to that venue. `select_revision`
returns a venue's baseline profile for **every** date before its first revision,
with no lower bound, so pre-2010 instants get the January-2010 grid rather than
an absence. Measured on that branch at 18:30 CT:

```
NKD        open in: [2005, 2008, 2009, 2010]
globex_eq  open in: [2005, 2008, 2009, 2010]
```

`globex_equity_index` behaves identically and predates the branch — this is the
crate's existing design for every venue whose baseline profile is non-empty, not
a regression. But LAW-NO-FABRICATED-DATES says amendment history is recorded
back to January 2010 and earlier changes are out of scope, and answering a 2005
query with a 2010 grid is arguably a claim outside that scope.

**Decision: leave it.** The profile returned is the earliest state the crate has
sourced, and the caller asked for a date this crate never undertook to model —
`AGENTS.md` already scopes amendment history to January 2010 and puts earlier
changes out of scope by design. Adding a lower bound would touch every venue with
a non-empty baseline and would trade one unreviewed answer for another, since
neither `Closed` nor the 2010 grid is sourced below the floor.

Recorded as a modelling convention in `AGENTS.md` so it is not re-argued. The
caller-facing consequence is stated in the README: a pre-2010 answer is the
oldest profile on record, not a reviewed one.

### Priority, set 2026-08-31: executable hours before queues

The hours where trades actually print matter more than the windows where orders
merely queue. An executable-only audit of all sixteen US futures keys (sampled
weeks in 2011/2015/2019/2023, `is_open` — regular plus extended, order-entry
excluded) puts the current state beyond doubt:

- **No US futures key withholds executable time that the current grid serves**,
  except where the venue genuinely traded fewer hours then (`cfe_vix` 2011,
  `globex_cryptocurrency` and `ice_us` before launch) or a sourced revision
  applies. `globex_fx` reads 0/0 in every sampled year.
- **All six `ice_us_*` keys withhold nothing executable in any year.**
- Every remaining `Partial` gap in US futures is therefore an
  **order-entry/queue** question — the CME Sunday 16:00–16:15 slice, the options
  queue onsets, the livestock and grains queue days. No trade can print in any
  of them.

Two places where executable time is *served but rests on carry-back* rather than
direct sourcing, and which are the real remaining executable-hours work:

1. `ice_us_*` — January 2010 to August 2011 is filled by carrying the August
   2011 master table back. **Resolved 2026-09-01 as terminal:** ICE Futures U.S.
   sets these hours administratively, not by rule (no product rulebook chapter
   carries an hours provision), so no filing can date them and the master hours
   table — earliest surviving edition August 2011 — is the only source.
2. `globex_nikkei_225_dollar` — **resolved 2026-09-01.** The 2010 change is real
   and primary-sourced: CME's trading-hours captures of 2010-03-10 and
   2010-04-07 show a different, DST-dependent grid. The served grid now starts at
   its first sourced appearance (2011-01-12) and 2010 is sessionless rather than
   filled with a grid that was wrong for it.

**Order of work when this resumes:** (1) executable-hours gaps above, (2) the
rest of US futures, (3) other futures (Eurex, ICE Europe/Endex, SGX SORA,
Binance), (4) everything else. Queue-onset dating is last: it is the largest
remaining pile by row count and the smallest by trading impact.


46 ledger rows currently carry `Partial` (27 venue rows plus the key
families). Each closes one of two ways: a sourced dated chain, or a
documented permanent knowledge-bound row. Batches are ordered by
evidence-leverage — several rows likely share one industry timeline, so one
notice may close many.

- [x] **Batch A — US options queue onsets** (17 `Partial` `options.rs` rows;
      MEMX Options and the count of 20 were wrong — `options.rs` owns 18
      venues, of which 17 are `Partial`). **Closed 2026-08-31 as
      knowledge-bound, not as a search that ran out of time.** The
      coordinated-industry-event hypothesis was wrong: there is no dated
      artifact to find, because on every venue the generic order-acceptance
      start is an operator *system setting* on a mutable hours/system-settings
      page rather than a rulebook boundary with a filed operative date. The
      two filings that codified the Cboe queuing periods state it outright
      (SR-C2-2019-009, SR-CboeBZX-2020-012: 07:30 was already "the same
      time at which the System begins accepting orders and quotes today"; Cboe
      Options Rule 6.2(a) bounds the pre-opening period rather than fixing
      it). Nasdaq publishes each start in a per-venue System Settings
      document, NYSE on its hours page, MIAX on its trade-hours calendar —
      none with a dated change notice. Outcome: all 17 rows keep `Partial`
      with a named permanent gap; three gained sourced lower bounds (C2
      2019-05-10, BZX Options 2020-02-04, ISE 2019-10-17); MIAX Options gained
      a true bracket (2012-12-09..2013-05-07) plus positive sourcing for its
      queue-free launch row. No dates invented, no profile changed. Review
      dates were **not** advanced: `nasdaqtrader.com` served a bot-protection
      interstitial and BOX notice bodies were not machine-readable, so those
      source sets were not fully opened.
- [x] **Batch B — CME family onsets** (equity-index, grains, energy/metals,
      FX, interest rates, livestock, cryptocurrency, NKD): the Sunday
      16:00 CT pre-open queue and PCP onsets. **Closed 2026-08-31.** The
      **Sunday-queue question is closed
      2026-08-31 as knowledge-bound.** The CME advisory archives were the right
      place to look and the answer is negative: the platform-wide 16:15→16:00
      Sunday Pre-Open bracket narrows to 2012-05-28..2012-06-07 on CME's own
      archived trading-hours pages, and both dated notice channels covering
      that window (Globex Notices 2012-05-21/05-28/06-04; Market Data Notices
      2012-05-28) contain no pre-open or trading-hours item at all — CME made
      the change without announcing it. CBOT grains were separated out: they
      moved at the already-dated 2012-05-20 expansion, not with the platform.
      No cutover encoded; the only Sunday in the bracket (2012-06-03) is an
      inference, not a stated day. Note for future passes: CME serves an
      explicit anti-scraping block to automated clients, so cmegroup.com must
      be read by hand or through the public archive.
      The other four sub-gaps were then worked to their terminal state.
      `globex_grains`: the 21-hour regime's queue and PCP states are now
      sourced from CME's own hours pages and the switch bracketed to
      2012-05-11..2012-05-28 around the sourced expansion, but Advisory
      #20120518 states matching hours only, so nothing is keyed to it.
      `globex_nikkei_225_dollar`: CME's hours pages DO state the pre-2012 grid
      (17:00-15:15 with the 15:30-16:30 slice), superseding the earlier record
      that no primary source states the pre-2012 evening open; only the onset
      stays undated. `globex_cryptocurrency`: the launch-era bitcoin
      specification publishes matching hours and no Pre-Open — undated at the
      source. `globex_livestock`: the specification channel is silent across
      2016-11..2020-03 too, corroborating the trading-hours omission. All four
      keep `Partial` with named, evidence-backed gaps; no dates invented.
- [x] **Batch C — ICE US softs and USDX** (sugar, coffee, cocoa, cotton,
      FCOJ, dollar index). **Closed 2026-08-31.** The gaps were baseline gaps
      rather than queue-onset gaps. Two dated editions of ICE's *Regular
      Trading Hours* master table (AUGUST 2011, JANUARY 2, 2013) print the
      pre-2014 grids outright, superseding the record that coffee's and
      cocoa's were only corroborated; the same two editions show FCOJ-A
      unchanged, and repeat the `*` / `**` / `***` footnote contrast that
      excludes Cotton from the Sunday-evening clause — turning that omission
      from silence into positive evidence. Sugar's January-2010 close and
      USDX's pre-2011 grid were re-worked and confirmed negative: August 2011
      is the earliest surviving edition of the table, and ICE's 2007
      currencies release never prints a USDX grid. All six keep `Partial`
      with one shared residual gap, January 2010 to August 2011.
- [x] **Batch D — SGX equity-index keys** (five rows). **Closed 2026-08-31 —
      and it was the most serious defect in the crate, not a documentation
      gap.** These five rows carried today's grid to the January-2010 floor
      across transitions the module already recorded as real, making them the
      only rows that could report a market *open* on hours that were not in
      force. SGX's own Derivatives Trading Calendar (2020, 2021, 2025, 2026
      editions, all retrievable as static PDFs from api2.sgx.com) proves the
      movement and supplies the dated grids. Six editions (2020, 2021-07, 2024, 2025-01, 2025-11, 2026-01) show two
      undated transitions; the dated surface serves their intersection as one
      window rather than keying revisions to an edition's year; dates before the 2020 edition are sessionless. An
      intersection taken from only the 2021 and 2026 editions — briefly
      shipped — reported Japan open between 15:10 and 15:25 through 2025;
      reading the intervening editions was what caught it.
      **Updated 2026-09-02:** three further editions (2021-01, 2022-06 and
      2025-07 — the last byte-identical to the 2025-11 file) tightened the
      brackets, and the *later* transition is now dated to **2025-04-07** by
      SGX-DT Circular DT/AM 15 of 2025 (24 Feb 2025), read from a trading
      member's verbatim public mirror because SGX hosts no publicly reachable
      copy. Still Partial: the earlier Japan T-session extension has no public
      exchange document (a member notice attests Monday 4 Nov 2024, which cannot
      date a row) and the pre-2020 era is unsourced; closing either needs a
      member to open the password-locked Titan DTDC newsletters ("SR12.5, Japan
      Derivatives Trading Hours Extension and I2 Timing Change" 28 Aug 2024;
      "Change of Trading Hours" 21 Sep 2017; "Extension of T+1 Trading Hours"
      15 Jul 2019; "Ext of T+1 Trading Hours Go Live Schedule" 8 Oct 2019 —
      release dates from SGX's own public Titan DT/DC portal index). Note for
      future passes: the SGX circular archive is a JavaScript app whose
      /circulars route now returns `null`, but the calendar PDFs under
      api2.sgx.com/sites/default/files/, the Titan portal's document index, and
      member mirrors of DT/AM circulars are all productive channels.

- [ ] **Batch E — US equity early sessions** (Cboe ×2, NYSE ×4): the 07:00
      early session and 06:30/02:30 queue onsets. **Started 2026-08-31; the key
      structural fact is established and the search is expected to succeed.**
      Unlike Batch A's options queues, these are *rulebook* provisions:
      SR-EDGX-2015-03 (80 FR 2163) and SR-EDGA-2015-03 (80 FR 2125) quote Rule
      11.1(a)(1) as already providing order entry "from 6:00 a.m. until 8:00
      p.m. Eastern Time", which puts the start no later than 2015-01-08 and
      locates it in a rule an SEC filing must have established on a stated day.
      The 2010 registration order (75 FR 13151) does not carry the hours, so the
      filing that first set Rule 11.1(a)(1) to 06:00 is the remaining target.
      Do NOT close these rows with the permanent-gap language Batch A uses —
      they are an unfinished search, and the Federal Register full-text API is
      the tool that will finish it. The four NYSE rows have not been started.
- [ ] Per row: follow `docs/schedules/updating.md` (open the monitoring
      sources, review everything since the row's `Reviewed on` date); add
      dated rows + `HISTORICAL_CUTOVERS`/instant entries + both-sides
      tests, or leave the knowledge-bound row with the ledger note naming
      the gap as permanent — the row keeps its `Partial` basis label
      either way until dated evidence upgrades it to `Primary`.
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

## Phase 5 — Built-in exception data: pilot (public-source-gated)

- [ ] Pick 1–2 pilot venues whose holiday calendars are primary and publicly
      available without authentication (NYSE and CME Group are the natural
      candidates).
- [ ] Source check per venue before any data drives runtime answers: the
      calendar must be primary and publicly available without
      authentication, and the check outcome is recorded beside the data.
      Per the sourcing policy in `date-exceptions.md`, the crate encodes the
      facts a public schedule states rather than reproducing the document, so
      this is a scope check, not a rights negotiation. A venue whose calendar
      exists only behind a member portal stays `OutOfCoverage`.
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
      evidence per `updating.md`, public-source check, tables, ledger row,
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
5. Phase 5 — after Phase 4; the public-source check is a hard gate before data
   ships.
6. Phase 6 — after Phase 5, batch by batch.

## Plan lifecycle

Working document: check boxes off as batches land, and delete this file once
Phase 6's final batch merges — the ledger, AGENTS.md, and date-exceptions.md
are the permanent record. If the effort stalls, the unchecked boxes are the
honest statement of remaining scope.
