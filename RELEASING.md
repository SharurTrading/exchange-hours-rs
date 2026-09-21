<!-- SPDX-License-Identifier: MIT-0 -->

# Releasing exchange-hours

Releases are cut from a pull request and published manually. The crate has no
credentialed release workflow; CI remains read-only.

## Coverage gate for the planned 2025-onward release

The 2026-09-21 UTC [plan amendment](docs/plans/2026-09-12-path-to-release.md)
is documentation only; its new contract is not implemented yet. Before the
release tracked by #119, require reviewed Stages 1–5 and candidate consumer
validation from Stage 6 (#118). Every served instrument must have an exact
family or documented venue scope with complete schedules, required phases and
all holiday arrangements from 2025 (or later launch) through its sufficiently
specified, unconditional publication horizon, including the inspection present.
An unresolved in-window gap blocks this gate. Dormant completeness and broad
venue intersections do not block it; their limitations remain explicit.

Check both repository tags and crates.io before choosing the version. The
planned identity-query `Result` migration (#115) is breaking: an unpublished
pending 1.0.0 may absorb it, but a published 1.x contract requires a major
version. Preserve detached snapshot compatibility and identity wire names.
Run consumer tests against the candidate before publication; pin and verify
the published version afterwards so the release and consumer gates do not
wait on each other's nonexistent tag. Source refresh and actual coverage
claims belong to the implementation/release changes, not the plan PR.

## Release pull request

1. Start from an up-to-date `main` and create `release/X.Y.Z`.
2. Set the package version in `Cargo.toml` and `Cargo.lock`.
3. Move the accumulated `[Unreleased]` entries to `## [X.Y.Z] - YYYY-MM-DD`,
   restore an empty `[Unreleased]` heading, and update comparison links.
4. Update the README installation, migration, coverage, and assurance text.
   Do not advance the schedule-review cutoff unless every non-synthetic
   `Exchange` row was reviewed through the new date.
5. For every ledger row still carrying the **Scheduled** marker whose
   effective date has passed, confirm against the operator's published
   schedule that the change took effect, then clear the marker in this release;
   if it slipped or was cancelled, remove the revision row and record the
   correction under **Fixed** in `CHANGELOG.md` instead of shipping it.
6. Review and commit the intended release changes. Require a clean working tree
   before package/publish validation so the inspected archive exactly matches a
   PR commit.
7. Run the full gates from `AGENTS.md`, then verify the publish archive:

   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets -- -D warnings
   cargo nextest run --all-targets
   cargo test --doc
   RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
   cargo deny check
   cargo +1.95 check --all-targets
   cargo publish --dry-run --locked
   ```

8. Inspect `cargo package --list --locked` for missing evidence/docs or
   accidental repository-only files.
9. For changes to the calendar query engine, run the informational performance
   baseline. This is not a correctness gate, but a release review should call
   out a material regression rather than hiding it:

   ```bash
   cargo bench --bench calendar_queries
   ```

10. Open the release pull request. Do not tag or publish from the branch. Any
    later commit requires the gates and archive checks to be rerun.

## Publish after merge

1. Confirm the release pull request's exact merge commit passed the required
   `quality` and `msrv` checks.
2. Use a clean checkout of that merge commit and rerun the full gates plus
   `cargo publish --dry-run --locked`.
3. Before creating a public tag, verify crates.io owner authentication with
   `cargo owner --list exchange-hours` and GitHub authentication with
   `gh auth status`.
4. Create and push an annotated tag on that exact commit:

   ```bash
   git tag -a vX.Y.Z -m "exchange-hours X.Y.Z"
   git push origin vX.Y.Z
   ```

5. Publish the same clean tree with the crates.io owner credential:

   ```bash
   cargo publish --locked
   ```

6. Create the GitHub release from the verified tag and use the matching
   changelog section as its notes.
7. Verify the new version on crates.io, wait for docs.rs to build successfully,
   and confirm the README's registry-derived version/MSRV badges.

Never move or reuse a published tag. If the crate is wrong after publication,
release a correction; yank only when the published version is unsafe to select.
