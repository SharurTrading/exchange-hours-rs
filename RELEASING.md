<!-- SPDX-License-Identifier: MIT-0 -->

# Releasing exchange-hours

Releases are cut from a pull request and published manually. The crate has no
credentialed release workflow; CI remains read-only.

## Release pull request

1. Start from an up-to-date `main` and create `release/X.Y.Z`.
2. Set the package version in `Cargo.toml` and `Cargo.lock`.
3. Move the accumulated `[Unreleased]` entries to `## [X.Y.Z] - YYYY-MM-DD`,
   restore an empty `[Unreleased]` heading, and update comparison links.
4. Update the README installation, migration, coverage, and assurance text.
   Do not advance the schedule-review cutoff unless every non-synthetic
   `Exchange` row was reviewed through the new date.
5. Review and commit the intended release changes. Require a clean working tree
   before package/publish validation so the inspected archive exactly matches a
   PR commit.
6. Run the full gates from `AGENTS.md`, then verify the publish archive:

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

7. Inspect `cargo package --list --locked` for missing evidence/docs or
   accidental repository-only files.
8. For changes to the calendar query engine, run the informational performance
   baseline. This is not a correctness gate, but a release review should call
   out a material regression rather than hiding it:

   ```bash
   cargo bench --bench calendar_queries
   ```

9. Open the release pull request. Do not tag or publish from the branch. Any
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
