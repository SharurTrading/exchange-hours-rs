// SPDX-License-Identifier: MIT-0

//! Integration tests over the verifier's public surface: the Documents-row
//! parser, the id resolution, the store resolution order, the digest check
//! and the `run` outcomes — against fixtures the tests build under the
//! system temp directory, never against a real store.

use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use xtask::{
    FoundRow, IdMatch, Outcome, collect_rows, digest_hex, match_id, parse_document_rows,
    quote_contexts, resolve_store, run, store_digest_map, verify_rows,
};

/// The canonical sha256 of the empty string — pins the hex encoding the
/// digest cells carry.
const EMPTY_SHA: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/// The fixture artifact's bytes; the quoted excerpt the evidence text carries
/// appears in them verbatim.
const ARTIFACT: &[u8] = b"prose the market is closed on Jan 1, the market is closed\n";

/// One Documents table with two rows (the second naming two ids), and a
/// holiday-table line beneath it citing the first id with a quoted excerpt.
fn fixture(sha_a: &str, sha_b: &str) -> String {
    format!(
        "## An owner\n\n### Documents\n\n\
         | Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |\n\
         |---|---|---|---|---|---|\n\
         | `example.pdf @2020-01-02T03:04:05Z` | 2010-01-01 .. 2024-12-31 | \
         <https://example.com/example.pdf> | archive capture 2020-01-02T03:04:05Z | T1 | `{sha_a}` |\n\
         | `paired-a.pdf @2020-01-02T03:04:05Z`, `paired-b.pdf @2020-01-02T03:04:05Z` | \
         2010-01-01 .. 2024-12-31 | <https://example.com/paired.pdf> | \
         archive capture 2020-01-02T03:04:05Z | T1 | `{sha_b}` |\n\n\
         ### 2020\n\n\
         | trade date | kind | instant as printed | document | tier | derived from |\n\
         |---|---|---|---|---|---|\n\
         | 2020-01-01 | closed | `the market is closed` | `example.pdf @2020-01-02T03:04:05Z` | \
         T1 | the sheet prints Jan 1 |\n\n\
         ## Next section\n"
    )
}

fn row(file: &str, line: usize, id: &str, sha: &str) -> FoundRow {
    FoundRow {
        file: file.to_owned(),
        line,
        raw: format!("| `{id}` | window | <https://example.com> | capture | T1 | `{sha}` |"),
        id: id.to_owned(),
        window: "window".to_owned(),
        url: "https://example.com".to_owned(),
        capture: "capture".to_owned(),
        tier: "T1".to_owned(),
        sha256: sha.to_owned(),
    }
}

/// A unique temp directory per call, removed and recreated.
fn temp_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let ordinal = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "xtask-verify-evidence-{}-{tag}-{ordinal}",
        process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Lays out a workspace (`base/repo`) beside an evidence store
/// (`base/exchange-hours-evidence`), writes the evidence file and the
/// artifact, and returns the repo root.
fn fixture_workspace(tag: &str, sha_a: &str, sha_b: &str) -> PathBuf {
    let base = temp_dir(tag);
    let repo = base.join("repo");
    fs::create_dir_all(repo.join("docs/evidence")).unwrap();
    let evidence_repo = base.join("exchange-hours-evidence");
    fs::create_dir_all(evidence_repo.join("task")).unwrap();
    fs::write(evidence_repo.join("task/example.pdf"), ARTIFACT).unwrap();
    fs::write(repo.join("docs/evidence/owner.md"), fixture(sha_a, sha_b)).unwrap();
    repo
}

#[test]
fn digest_hex_pins_the_canonical_encoding() {
    assert_eq!(digest_hex(b""), EMPTY_SHA);
    assert_eq!(
        digest_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn the_parser_reads_rows_and_multi_id_cells() {
    let rows = parse_document_rows("owner.md", &fixture(&"a".repeat(64), &"b".repeat(64)));
    assert_eq!(rows.len(), 3, "one row per id, two from the paired cell");
    assert_eq!(rows[0].id, "example.pdf @2020-01-02T03:04:05Z");
    assert_eq!(rows[0].file, "owner.md");
    assert_eq!(rows[0].line, 7, "the first table row, 1-based");
    assert_eq!(rows[0].window, "2010-01-01 .. 2024-12-31");
    assert_eq!(rows[0].url, "https://example.com/example.pdf");
    assert_eq!(rows[0].capture, "archive capture 2020-01-02T03:04:05Z");
    assert_eq!(rows[0].tier, "T1");
    assert_eq!(rows[0].sha256, "a".repeat(64));
    assert_eq!(rows[1].id, "paired-a.pdf @2020-01-02T03:04:05Z");
    assert_eq!(rows[2].id, "paired-b.pdf @2020-01-02T03:04:05Z");
    assert_eq!(
        rows[1].sha256,
        "b".repeat(64),
        "the paired ids share a digest"
    );
    assert!(
        rows[0].raw.starts_with("| `example.pdf"),
        "the raw citation line is kept verbatim"
    );
}

#[test]
fn the_parser_skips_files_without_a_documents_table() {
    let text = "## An owner\n\n| trade date | document |\n|---|---|\n| 2020-01-01 | `x` |\n";
    assert!(parse_document_rows("owner.md", text).is_empty());
}

#[test]
fn id_resolution_prefers_the_full_id_and_reports_ambiguity() {
    let rows = vec![
        row(
            "owner.md",
            5,
            "example.pdf @2020-01-02T03:04:05Z",
            &"a".repeat(64),
        ),
        row(
            "owner.md",
            6,
            "example.pdf @2021-01-02T03:04:05Z",
            &"b".repeat(64),
        ),
    ];
    let exact = match_id(&rows, "example.pdf @2020-01-02T03:04:05Z");
    assert_eq!(exact, IdMatch::Exact(&rows[0]));
    let ambiguous = match_id(&rows, "example.pdf");
    assert_eq!(
        ambiguous,
        IdMatch::Ambiguous(vec![&rows[0], &rows[1]]),
        "a bare artifact name two rows cite is ambiguous"
    );
    assert_eq!(match_id(&rows, "missing.pdf"), IdMatch::NotFound);
    assert_eq!(
        match_id(&rows, "no-such-id @2020-01-02T03:04:05Z"),
        IdMatch::NotFound,
        "an id-shaped query never filename-matches"
    );
}

#[test]
fn the_store_resolves_env_first_then_the_evidence_repo_then_the_working_store() {
    let base = temp_dir("resolve");
    let root = base.join("repo");
    fs::create_dir_all(&root).unwrap();
    let evidence = base.join("exchange-hours-evidence");
    let working = base.join("exchange-hours-research");

    assert!(resolve_store(&root, None).is_err(), "nothing resolves yet");

    fs::create_dir_all(&working).unwrap();
    assert_eq!(
        resolve_store(&root, None).unwrap(),
        working,
        "the working store is the last candidate"
    );

    fs::create_dir_all(&evidence).unwrap();
    assert_eq!(
        resolve_store(&root, None).unwrap(),
        evidence,
        "the public evidence repo outranks the working store"
    );

    let explicit = base.join("elsewhere");
    fs::create_dir_all(&explicit).unwrap();
    assert_eq!(
        resolve_store(&root, Some(explicit.clone().into_os_string())).unwrap(),
        explicit,
        "an env value that names a directory wins"
    );

    let error = resolve_store(&root, Some(base.join("missing").into_os_string()))
        .expect_err("a wrong env setting is an error, never a fall-through");
    assert!(
        error.message.contains("not a directory"),
        "the error names the problem: {}",
        error.message
    );
}

#[test]
fn digest_verification_reports_only_unknown_digests() {
    let base = temp_dir("digests");
    let artifact = b"the bytes".to_vec();
    fs::create_dir_all(base.join("task")).unwrap();
    fs::write(base.join("task/example.pdf"), &artifact).unwrap();
    let map = store_digest_map(&base).unwrap();
    assert_eq!(
        map.get(&digest_hex(&artifact)).unwrap(),
        &vec!["task/example.pdf".to_owned()],
        "a digest resolves to the store-relative path"
    );
    let rows = vec![
        row(
            "owner.md",
            5,
            "known.pdf @2020-01-02T03:04:05Z",
            &digest_hex(&artifact),
        ),
        row(
            "owner.md",
            6,
            "unknown.pdf @2020-01-02T03:04:05Z",
            &"0".repeat(64),
        ),
    ];
    let failures = verify_rows(&rows, &map);
    assert_eq!(failures, vec![&rows[1]]);
    // A checkout's own .git is not evidence and never enters the map.
    fs::create_dir_all(base.join(".git/objects")).unwrap();
    fs::write(base.join(".git/objects/pack.pack"), b"pack").unwrap();
    let without_git = store_digest_map(&base).unwrap();
    assert_eq!(without_git.len(), map.len(), ".git is skipped");
}

#[test]
fn run_all_and_run_one_verify_the_fixture_workspace() {
    let sha = &digest_hex(ARTIFACT);
    let root = fixture_workspace("run", sha, sha);

    let all = run(["verify-evidence", "--all"].map(Into::into), &root, None);
    let Outcome::Success(report) = &all else {
        panic!("--all must verify: {all:?}");
    };
    assert!(
        report.contains("verified 3 Documents rows across 1 evidence files"),
        "--all counts every id: {report}"
    );
    assert!(
        report.contains("exchange-hours-evidence"),
        "the report names the store: {report}"
    );

    let one = run(
        ["verify-evidence", "example.pdf @2020-01-02T03:04:05Z"].map(Into::into),
        &root,
        None,
    );
    let Outcome::Success(report) = &one else {
        panic!("the full id must resolve: {one:?}");
    };
    assert!(
        report.contains("owner.md:7"),
        "the citation line names file and line: {report}"
    );
    assert!(
        report.contains("task/example.pdf"),
        "the report names the artifact's path: {report}"
    );
    assert!(
        report.contains("`the market is closed` — found in the bytes"),
        "the quote context comes from the artifact: {report}"
    );

    let bare = run(
        ["verify-evidence", "example.pdf"].map(Into::into),
        &root,
        None,
    );
    let Outcome::Success(report) = &bare else {
        panic!("a bare artifact name one row cites resolves: {bare:?}");
    };
    assert!(report.contains("sha256"), "{report}");
}

#[test]
fn a_digest_no_store_file_produces_fails_the_single_verification() {
    let sha = &digest_hex(ARTIFACT);
    let root = fixture_workspace("mismatch", sha, &"0".repeat(64));
    let outcome = run(
        ["verify-evidence", "paired-a.pdf @2020-01-02T03:04:05Z"].map(Into::into),
        &root,
        None,
    );
    let Outcome::Failure(report) = &outcome else {
        panic!("the zero filler is no store file: {outcome:?}");
    };
    assert!(
        report.contains("is no file in the evidence store"),
        "{report}"
    );
    assert!(
        report.contains("owner.md:8"),
        "the failing row is named: {report}"
    );
}

#[test]
fn run_reports_missing_ids_missing_stores_and_usage() {
    let sha = &digest_hex(ARTIFACT);
    let root = fixture_workspace("run-errors", sha, sha);

    let missing = run(
        ["verify-evidence", "absent.pdf"].map(Into::into),
        &root,
        None,
    );
    assert!(
        matches!(missing, Outcome::Failure(ref report) if report.contains("no Documents row resolves")),
        "{missing:?}"
    );

    // An evidence directory with rows but no store anywhere: the store
    // resolution fails and names what was tried.
    let base = temp_dir("storeless");
    let storeless = base.join("repo");
    fs::create_dir_all(storeless.join("docs/evidence")).unwrap();
    fs::write(storeless.join("docs/evidence/owner.md"), fixture(sha, sha)).unwrap();
    let no_store = run(
        ["verify-evidence", "--all"].map(Into::into),
        &storeless,
        None,
    );
    assert!(
        matches!(no_store, Outcome::Failure(ref report) if report.contains("did not resolve")),
        "{no_store:?}"
    );

    for bad in [
        vec![],
        vec!["frobnicate"],
        vec!["verify-evidence"],
        vec!["verify-evidence", "--all", "extra"],
        vec!["verify-evidence", "--flag"],
    ] {
        let outcome = run(bad.iter().map(Into::into), &root, None);
        assert!(
            matches!(outcome, Outcome::Failure(ref report) if report.contains("usage:")),
            "{bad:?} must print usage: {outcome:?}"
        );
    }
    let help = run(["--help"].map(Into::into), &root, None);
    assert!(matches!(help, Outcome::Success(ref report) if report.contains("usage:")));
}

#[test]
fn an_empty_evidence_directory_fails_instead_of_passing_vacuously() {
    let base = temp_dir("vacuous");
    let root = base.join("repo");
    fs::create_dir_all(root.join("docs/evidence")).unwrap();
    fs::create_dir_all(base.join("exchange-hours-evidence")).unwrap();
    let outcome = run(["verify-evidence", "--all"].map(Into::into), &root, None);
    assert!(
        matches!(outcome, Outcome::Failure(ref report) if report.contains("vacuously")),
        "an audit over zero rows must fail loudly: {outcome:?}"
    );
}

#[test]
fn collect_rows_walks_the_directory_in_name_order() {
    let base = temp_dir("collect");
    let dir = base.join("docs/evidence");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("b.md"), fixture(&"a".repeat(64), &"b".repeat(64))).unwrap();
    fs::write(dir.join("a.md"), fixture(&"c".repeat(64), &"d".repeat(64))).unwrap();
    fs::write(dir.join("notes.txt"), "not evidence markdown").unwrap();
    let rows = collect_rows(&dir).unwrap();
    assert_eq!(
        rows.len(),
        6,
        "three ids per markdown file, none from the txt"
    );
    assert_eq!(rows[0].file, "a.md", "sorted name order");
    assert_eq!(rows[3].file, "b.md");
}

#[test]
fn quote_contexts_skip_the_id_and_digest_and_cap_the_hits() {
    let row = row(
        "owner.md",
        5,
        "example.pdf @2020-01-02T03:04:05Z",
        &"a".repeat(64),
    );
    let text = "| trade date | kind | instant | document | tier | derived from |\n\
                |---|---|---|---|---|---|\n\
                | 2020-01-01 | closed | `open quote one` | `example.pdf @2020-01-02T03:04:05Z` | T1 | x |\n\
                | 2020-01-02 | closed | `open quote two` | `example.pdf @2020-01-02T03:04:05Z` | T1 | x |\n\
                | 2020-01-03 | closed | `open quote three` | `example.pdf @2020-01-02T03:04:05Z` | T1 | x |\n\
                | 2020-01-04 | closed | `open quote four` | `example.pdf @2020-01-02T03:04:05Z` | T1 | x |\n";
    let artifact =
        b"prose open quote one, open quote two, open quote three, open quote four".to_vec();
    let hits = quote_contexts(&text, &row, &artifact);
    assert_eq!(hits.len(), 3, "at most three quote hits");
    assert!(hits[0].contains("open quote one"), "{hits:?}");
    assert!(
        !hits.iter().any(|hit| hit.contains(&row.sha256)),
        "the digest cell is never a quote"
    );
    assert!(quote_contexts(&text, &row, b"unrelated bytes").is_empty());
    assert!(quote_contexts("no quotes cite the row", &row, &artifact).is_empty());
}
