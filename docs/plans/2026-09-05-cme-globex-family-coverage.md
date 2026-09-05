<!-- SPDX-License-Identifier: MIT-0 -->

# CME/Globex product-family coverage — handoff plan

Plan date: 2026-09-05 · Revised 2026-09-05 after the first two deliveries.

Delivered so far: **Rough Rice** (PR #47) and **mini grains** (PR #51, work item
1 below). Both are on `main`; the research that produced them is unchanged and
still applies to items 2-6.

Driver: SharurPlatform's Databento GLBX.MDP3 catalog audit found `ZR` on the
wrong key and 1,342 roots with no authored product-family selector. This plan
sequences the exchange-hours side of that.

## Where the research already lives — READ THIS FIRST

The research store is **deliberately not in this repository**, and it is not a
committed artifact. Two reasons, in order of importance:

1. **It holds retrieved primary documents** — exchange PDFs, archived operator
   pages, a member-mirrored circular. This repository's posture is to *cite*
   primary sources and quote what they state, never to redistribute them.
   Committing the store would republish third-party material.
2. Sessions and their `/tmp` scratchpads are disposable, and agent runs have
   died mid-flight on usage limits, so the notes need somewhere durable that is
   also not version-controlled noise.

**Convention:** the store lives at `$EXCHANGE_HOURS_RESEARCH`, defaulting to
`../exchange-hours-research` beside this checkout. Set the variable if you keep
it elsewhere. If it is absent you have not lost anything reproducible — every
conclusion in it is reconstructible from the citations recorded in the source
comments, the ledger, and `docs/schedules/sources.md`; the store is a cache of
that retrieval, not a second source of truth.

```
$EXCHANGE_HOURS_RESEARCH/          # default: ../exchange-hours-research
├── STATUS.md                      start here — threads, state, next steps
├── cme-globex/
│   ├── rough-rice.md / .json      DELIVERED as PR #47
│   ├── rough-rice-VERIFIED.md     the two verifications done by hand
│   ├── cbot-18-001.pdf            the primary filing behind PR #47
│   ├── mini-grains-*.md / .json   8 dated revisions, ready to implement
│   ├── cme-weather-*.md / .json   1 dated revision (SER-9519)
│   ├── cme-cbot-spot-quoted-*.md  5 dated revisions
│   ├── event-contracts.md / .json 4 dated revisions
│   ├── workflow-result.json       full raw return
│   └── journal-live.jsonl         per-agent raw returns
└── sgx-pre2020/
    ├── FINDINGS.md                see "SGX" below — affects merged PR #44
    ├── sweep-result.json          raw, 5 archive channels
    └── artifacts/                 20 retrieved documents
```

Each `cme-globex/<family>.md` carries: products, timezone, current grid with
verbatim quotes, dated revisions each with a working citation, an explicit
**unsourced** list, and a reuse verdict. The unsourced lists are as important as
the findings — do not silently close a gap another agent recorded as open.

The audit inputs come from the SharurPlatform repository, at
`catalog-artifacts/glbx-mdp3/` within that checkout —
`market-hours-profile-inventory.md` is the work list, with
`market-hours-electronic-envelope-candidates.tsv` and
`market-hours-root-classification.tsv` beside it. They are generated artifacts
owned by that repository and are not vendored here; regenerate them there rather
than copying them in.

## The constraint that governs every item

CME's weekly `TradingSessionList.dat` publishes the **electronic envelope**
(Pre-Open, Ready, Halt, Close) and **not** the Regular-versus-Extended
classification. The inventory says so itself. Repo law requires that
classification plus a dated history, so **the schedule file seeds the queue and
never closes a profile.** Every family needs CME product specs plus dated
notices.

`cmegroup.com` returns 403 to curl and other non-browser clients. Working routes:
web-archive replay (`https://web.archive.org/web/<TIMESTAMP>id_/<URL>`), which
works for CME PDFs; and a real browser for live spec pages. CME's CFTC
rule-filing indexes proved the decisive route for dated changes, because
archived spec pages from 2015 onward render their table from a client-side API
the archive never captured.

## Work items, in order

Each is one PR. Land them separately: one PR carrying five families with full
histories is unreviewable and stalls on whichever family sources worst.

### 1. Mini grains — `XC`, `XK`, `XW`, `MKC` — **DELIVERED, PR #51**
Landed with seven dated revision rows. The eighth item in the research, `MKC`'s
2014-03-23 listing, became catalog data rather than a revision row, per the
crate's member-listing convention. Kept below because the reasoning generalises.

Research was complete, 8 dated revisions, high confidence. Verdict: **one family**,
not four, and not foldable into `globex_grains`. The discriminator is sharp —
on captures taken the same day in 2015, KC HRW Wheat (`KE`) reads
`8:30 a.m. – 1:20 p.m. CT` while Mini-Sized KC HRW Wheat (`MKC`) reads
`8:30 a.m. – 1:45 p.m. CT`, so `MKC` follows the **mini** grid despite its KC
lineage. Note `SER-9049` (2022-09-01) documents an `08:00–08:30 CT` morning
Pre-Open for the four mini contracts.

### 2. CME Weather
Research complete, one dated revision (**2025-04-14**, `SER-9519`). Verdict: one
schedule family, no split by contract type. ~179 roots ride on it.

### 3. Spot-quoted — `QSPX QNDX QDOW QRTY QBTC QETH QSOL QXRP`
Research complete, 5 dated revisions. Verdict: **its own family** — CME/CBOT
Rulebook Chapter 24. Explicitly **not** the equity-index key and **not** the
cryptocurrency key: the crypto family went 24/7 on 2026-05-29 and spot-quoted
deliberately did not.

### 4. Event contracts — `ECES ECNQ ECRTY ECYM ECBTC EC6E ECCL ECNG ECGC ECSI ECHG`
Research complete, 4 dated revisions. Open design question to settle from the
notices before coding: pre-2026 closes were **underlying-specific**, so decide
whether that era needs several profiles collapsing to one 24/7 profile at
2026-05-29. `SER-8968` (original grids) and the 2026-03-30 notice (24/7
transition) are the anchors.

### 5. The 37-shape research queue
Rows 2, 3, 6–34 and 36–41 of the inventory: BTIC, TAS, TAM, TACO, TMAC, housing,
dairy, lumber, commodity indexes. **Deliverable is a handoff table**
(`root → new/existing MarketHoursKey → citation`) plus an explicit unresolved
list with reasons — *not* 37 new keys. Most will resolve to "reuse an existing
key" or "cannot source, leave unmapped". Combine shapes under one key only where
product-family semantics and history prove it, never because envelopes match.

### 6. Review the 30 roots already mapped
Update key documentation where needed; change no schedule unless evidence
requires it.

## Adding a MarketHoursKey — the registration surface

Tests catch omissions, but do these deliberately:

- enum variant + canonical string via the macro in `src/calendar/futures_profile.rs`
- selector match arm; fixed profile in `futures_profile/profiles.rs`
- `docs/schedules/verification.md` ledger row **with a Basis**. Two different
  families of narrative counts restate that ledger, and until 2026-09-05 only one
  of them was fenced — which read as "the tests derive every count" and let two
  consecutive additions ship stale prose. Both are fenced now:
  `assert_key_basis_prose_matches_the_ledger` covers the Primary/Partial key
  split and headline key count, and
  `the_gap_kind_split_is_quoted_consistently_everywhere` covers the
  `Gap: order-entry` / `Gap: executable` tally as restated in the README, in the
  ledger's own summary, and in the audit's spelled-out Partial-key sentence.
  A green suite now really does mean the counts reconcile — but read the failure
  message rather than guessing, because it prints the exact expected string
- `EXPECTED_MARKET_HOURS_KEY_NAMES`, `named_profiles.rs`,
  `unsupported_market_hours_keys.rs`, `golden_grids.rs` header count
- regenerate `tests/golden/normal_week_grids.txt`; confirm **no other key's rows move**
- `docs/schedules/sources.md` (check whether `US-CME-GROUP` already covers it)
- `CHANGELOG.md` under `[Unreleased]`
- tests in a submodule of `tests/futures_family_boundaries/`, never fattening the root

Per `AGENTS.md` item 6: the published open and the instant before it,
regular/extended classification, every gap, the end-exclusive close, weekend
behaviour, serde, and **both sides of every cutover at venue-local midnight**.
**Mutation-check each cutover fence** — move the revision a day, confirm failure,
restore, and say you did it.

## Traps already paid for

- **Envelope match is not family identity.** Two products can share an
  electronic envelope and have different RTH classification and history.
- **Check the contract set, not just the grid.** A source only sources a family
  if that family appears in it. SGX FTSE Taiwan was served from a 2020 edition
  that contains only the MSCI predecessors — identical hours made the error
  invisible in the times. Same shape as `MKC` vs `KE` above, inverted.
- **A two-endpoint bracket assumes nothing happened between.** That assumption
  was wrong three times this cycle (SGX twice, CME Nikkei once).
- **"Unsourced" means "not worked up", never "no source exists."** Check retired
  operator sites in the archive before writing that nothing survives.
- **Paper trails are not always clean.** The 2012-05-20 CBOT grain cutover was
  certified by Submission 12-144 for a June date that was superseded; the May
  go-live rests on a Globex notice plus a retrospective filing that agree.
- **Every one of these PRs touches the same ledger, so each merge breaks the
  next one's mergeability.** That is expected, not a mistake: land one, rebase
  the rest. Sequence deliberately — put a PR that adds a fence *before* the PRs
  it protects.
- **Two PRs can move the counts along different axes, so neither branch's
  figures survive the merge.** #51 added a key row while #49 changed exchange
  rows and reclassified two into executable; both were internally right and both
  were wrong once merged. Re-derive every tally from the merged ledger rather
  than resolving the conflict by picking a side.
- **`docs/schedules/databento-venues.md` snapshots each venue's ledger Basis**
  and a test asserts the two agree. Changing a row's Basis means changing that
  file too.
- **A non-wrapping evening leg needs a trade-date exception.** The generic
  convention names a trade date by the local date of its final close, which puts
  a 19:00-21:00 session on the *opening* day. Rough Rice needed a sourced
  exception in `src/calendar/query/identity.rs` for exactly this; a wrapping leg
  does not, because the default already gives the right answer. Check which
  shape a new family has before assuming the default works.

## SGX — unrelated to CME, but queued and affecting merged work

`sgx-pre2020/FINDINGS.md`. The **SGX Derivatives Product Catalogue `read_me`
change log** (SGX-hosted, public, inside the products workbook on the Titan
DT/DC portal) states effective days and dates: **2019-11-11** (T+1 close
04:45→05:15, closes issue #45), **2024-11-04** (the Japan T-session extension
that merged PR #44 leaves undated and serves via a conservative intersection),
and corroborates **2025-04-07** from an SGX-hosted document where #44 could only
cite a member mirror.

Before encoding: those cells write "eff 4 Nov" / "Effective 11 Nov" **with no
year**; the year is inferred from the surrounding block's date cell. Re-verify
directly under LAW-NO-FABRICATED-DATES. Also unresolved: an intermediate
02:00 → 04:45 T+1 step between 2018-07-11 and 2019-05-31 that nothing retrieved
dates, so the pre-2020 history has at least three steps, not one.
