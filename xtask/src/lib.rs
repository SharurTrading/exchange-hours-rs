// SPDX-License-Identifier: MIT-0

//! The evidence-chain verifier behind `cargo xtask verify-evidence`.
//!
//! The crate's only product is the claim that its values are sourced, and a
//! claim is only as good as the bytes behind it. Every session time, dated
//! change and holiday the crate encodes cites a document id; the id's row in
//! the evidence file's `### Documents` table carries the replay URL, the
//! capture instant, the tier and the artifact's sha256; the artifact itself
//! lives in the evidence store — the public mirror at
//! [`SharurTrading/exchange-hours-evidence`](https://github.com/SharurTrading/exchange-hours-evidence)
//! or the maintainer's working store beside the repository.
//!
//! This crate turns that chain into one mechanical check: resolve the id
//! across every `### Documents` table in `docs/evidence/`, recompute the
//! sha256 from the store's bytes, print the citation line and — best effort —
//! the quoted context in the artifact. [`run`] is the subcommand entry point;
//! `verify-evidence --all` walks every row of every evidence file, which is
//! what the `evidence-audit` CI workflow calls alongside the digest fence.
//!
//! The row grammar this parser reads is fixed by the crate's own fence
//! (`every_documents_digest_is_the_research_store_bytes` in
//! `tests/schedule_documentation/`), which enforces the strict shape on every
//! change. The verifier stays permissive about what it reads and strict about
//! what it asserts: a row whose digest is no store file's bytes fails,
//! whatever prose surrounds it.

use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The one shape a `### Documents` table may take — the same fixed header the
/// crate's documentation fences enforce, so a resolution is machine-readable.
const DOCUMENT_TABLE_HEADER: &str =
    "| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |";

/// The sibling directories tried, in order, when `$EXCHANGE_HOURS_RESEARCH`
/// is unset: the public evidence repo's local clone first, then the
/// maintainer's working store — both the convention beside the repository.
const STORE_SIBLINGS: [&str; 2] = ["exchange-hours-evidence", "exchange-hours-research"];

/// A verifier failure: the message a caller prints to stderr before exiting
/// non-zero. Every failure is a returned value — the tool never panics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XTaskError {
    /// The human-readable failure message.
    pub message: String,
}

impl fmt::Display for XTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for XTaskError {}

impl XTaskError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    fn from_io(what: &str, error: &io::Error) -> Self {
        Self::new(format!("{what}: {error}"))
    }
}

/// What a subcommand decided: report text for stdout with a zero exit, or a
/// failure for stderr with a non-zero one.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The run verified; print on stdout, exit 0.
    Success(String),
    /// The run failed — a mismatch, an unresolved id, a missing store, a bad
    /// invocation; print on stderr, exit non-zero.
    Failure(String),
}

/// One `### Documents` row, with the file and line that carry it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoundRow {
    /// The evidence file's name, e.g. `cme.md`.
    pub file: String,
    /// The row's 1-based line number in that file.
    pub line: usize,
    /// The raw table line — the citation line, printed verbatim.
    pub raw: String,
    /// The document id: `<artifact> @<capture label>`.
    pub id: String,
    /// The audited window the row's window cell states.
    pub window: String,
    /// The replay or service URL.
    pub url: String,
    /// The capture-or-retrieval cell, verbatim.
    pub capture: String,
    /// The tier cell: T1 through T4 (LAW-PRIMARY-SOURCES).
    pub tier: String,
    /// The sha256 digest cell, lowercase hex.
    pub sha256: String,
}

/// The lowercase-hex sha256 of `bytes` — the exact encoding the digest cells
/// and the crate's fence use (sha2 0.11's output type does not implement
/// `LowerHex`, so the hex is written byte by byte).
#[must_use]
pub fn digest_hex(bytes: &[u8]) -> String {
    let mut digest = String::with_capacity(64);
    for &byte in &Sha256::digest(bytes) {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}

/// Parses every `### Documents` row of one evidence file's text.
///
/// A file may carry several such tables — a family can keep one per era — so
/// every table is read, in file order. A left cell may name several ids
/// separated by `, ` when one artifact serves more than one. Tables end at
/// the first line that is not a row, exactly as the crate's fence reads them;
/// the strict grammar (real URLs, real capture stamps, 64-hex digests) is the
/// documentation fences' enforcement, so this parser reports what shipped
/// rather than rejecting it.
#[must_use]
pub fn parse_document_rows(file: &str, text: &str) -> Vec<FoundRow> {
    let mut rows = Vec::new();
    let mut lines = text.lines().enumerate();
    while let Some((_, line)) = lines.next() {
        if line.trim() != DOCUMENT_TABLE_HEADER {
            continue;
        }
        // The separator (`|---|…`) and stray blank lines sit between the
        // header and the first row.
        let mut next = lines.next();
        while let Some((_, candidate)) = next {
            if candidate.is_empty() || candidate.starts_with("|---") {
                next = lines.next();
            } else {
                break;
            }
        }
        // Rows, until the first line that is not one.
        while let Some((at, line)) = next {
            if !line.starts_with("| `") {
                break;
            }
            rows.extend(parse_row(file, at + 1, line));
            next = lines.next();
        }
    }
    rows
}

/// Parses one table line into one row per id its left cell names; an empty
/// result when the line does not carry the six cells the table's shape fixes.
fn parse_row(file: &str, at: usize, line: &str) -> Vec<FoundRow> {
    let cells: Vec<&str> = line
        .trim_start_matches('|')
        .split('|')
        .map(str::trim)
        .collect();
    if cells.len() < 6 {
        return Vec::new();
    }
    let window = cells[1].to_owned();
    let url = cells[2]
        .trim_start_matches('<')
        .trim_end_matches('>')
        .to_owned();
    let capture = cells[3].to_owned();
    let tier = cells[4].to_owned();
    let sha256 = cells[5].trim_matches('`').to_owned();
    cells[0]
        .split(", ")
        .map(|id| FoundRow {
            id: id.trim().trim_matches('`').to_owned(),
            file: file.to_owned(),
            line: at,
            raw: line.to_owned(),
            window: window.clone(),
            url: url.clone(),
            capture: capture.clone(),
            tier: tier.clone(),
            sha256: sha256.clone(),
        })
        .collect()
}

/// The evidence directory of the workspace root `root`.
#[must_use]
pub fn evidence_dir(root: &Path) -> PathBuf {
    root.join("docs").join("evidence")
}

/// Reads every evidence file under `evidence_dir` and parses its Documents
/// rows. Files are visited in sorted name order, so a run's output is
/// deterministic; only `*.md` files are read.
///
/// # Errors
/// When the directory, a directory entry or a Markdown file cannot be read.
pub fn collect_rows(evidence_dir: &Path) -> Result<Vec<FoundRow>, XTaskError> {
    let mut names = Vec::new();
    let entries = fs::read_dir(evidence_dir).map_err(|error| {
        XTaskError::from_io(
            &format!(
                "the evidence directory must be readable at {}",
                evidence_dir.display()
            ),
            &error,
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            XTaskError::from_io("an evidence directory entry must be readable", &error)
        })?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".md") {
            names.push(name);
        }
    }
    names.sort();
    let mut rows = Vec::new();
    for name in names {
        let path = evidence_dir.join(&name);
        let text = fs::read_to_string(&path)
            .map_err(|error| XTaskError::from_io(&format!("{name} must be readable"), &error))?;
        rows.extend(parse_document_rows(&name, &text));
    }
    Ok(rows)
}

/// Resolves the evidence store's root.
///
/// Order: `$EXCHANGE_HOURS_RESEARCH` where the checkout sits elsewhere — it
/// must name a directory when set, because a wrong setting is an error, never
/// a silent fall-through — then `../exchange-hours-evidence` (the local clone
/// of the public evidence repo), then `../exchange-hours-research` (the
/// maintainer's working store), both relative to the workspace root `root`'s
/// parent.
///
/// # Errors
/// When the env value names a missing path, or no candidate resolves: the
/// message names every path tried and the clone that fixes it.
pub fn resolve_store(root: &Path, env_value: Option<OsString>) -> Result<PathBuf, XTaskError> {
    if let Some(value) = env_value {
        let path = PathBuf::from(value);
        if path.is_dir() {
            return Ok(path);
        }
        return Err(XTaskError::new(format!(
            "EXCHANGE_HOURS_RESEARCH is set to {}, which is not a directory; \
             set it to a local checkout of SharurTrading/exchange-hours-evidence \
             or unset it to use the sibling directories",
            path.display()
        )));
    }
    let parent = root.parent().unwrap_or(root);
    let mut tried = Vec::new();
    for name in STORE_SIBLINGS {
        let candidate = parent.join(name);
        if candidate.is_dir() {
            return Ok(candidate);
        }
        tried.push(candidate.display().to_string());
    }
    Err(XTaskError::new(format!(
        "the evidence store did not resolve; tried {}. Clone the public \
         evidence repo beside this checkout (`git clone \
         https://github.com/SharurTrading/exchange-hours-evidence`) or set \
         $EXCHANGE_HOURS_RESEARCH to a checkout of it",
        tried.join(", ")
    )))
}

/// Hashes every file under the store, keyed by sha256.
///
/// A digest's entry holds every path whose bytes produce it — one artifact is
/// often mirrored across task directories — so a row's digest resolves to the
/// paths a re-verification reads. A checkout's own `.git` directories are
/// skipped: git's object store is not evidence.
///
/// # Errors
/// When any directory or file under the store cannot be read.
pub fn store_digest_map(root: &Path) -> Result<BTreeMap<String, Vec<String>>, XTaskError> {
    fn walk(
        root: &Path,
        dir: &Path,
        map: &mut BTreeMap<String, Vec<String>>,
    ) -> Result<(), XTaskError> {
        let entries = fs::read_dir(dir).map_err(|error| {
            XTaskError::from_io(
                &format!("the evidence store must be readable at {}", dir.display()),
                &error,
            )
        })?;
        for entry in entries {
            let path = entry
                .map_err(|error| XTaskError::from_io("a store entry must be readable", &error))?
                .path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == ".git") {
                    continue;
                }
                walk(root, &path, map)?;
            } else {
                let bytes = fs::read(&path).map_err(|error| {
                    XTaskError::from_io(&format!("{} must be readable", path.display()), &error)
                })?;
                let digest = digest_hex(&bytes);
                let shown = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                map.entry(digest).or_default().push(shown);
            }
        }
        Ok(())
    }
    let mut map = BTreeMap::new();
    walk(root, root, &mut map)?;
    Ok(map)
}

/// The resolution of a document-id query against the parsed rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdMatch<'a> {
    /// Exactly one row resolves.
    Exact(&'a FoundRow),
    /// The query names an artifact several rows cite with distinct capture
    /// labels; the caller must pass the full id.
    Ambiguous(Vec<&'a FoundRow>),
    /// No row resolves.
    NotFound,
}

/// Resolves a document-id query: the full `<artifact> @<capture label>` id
/// matches exactly; a bare artifact name resolves when exactly one row cites
/// it.
#[must_use]
pub fn match_id<'a>(rows: &'a [FoundRow], query: &str) -> IdMatch<'a> {
    if let Some(row) = rows.iter().find(|row| row.id == query) {
        return IdMatch::Exact(row);
    }
    if query.contains(" @") {
        return IdMatch::NotFound;
    }
    let candidates: Vec<&FoundRow> = rows
        .iter()
        .filter(|row| row.id.split(" @").next() == Some(query))
        .collect();
    match candidates.as_slice() {
        [one] => IdMatch::Exact(one),
        [] => IdMatch::NotFound,
        many => IdMatch::Ambiguous(many.to_vec()),
    }
}

/// Verifies every row's digest against the store map; returns the rows whose
/// digest is no file's bytes in the store.
#[must_use]
pub fn verify_rows<'a>(
    rows: &'a [FoundRow],
    store: &BTreeMap<String, Vec<String>>,
) -> Vec<&'a FoundRow> {
    rows.iter()
        .filter(|row| !store.contains_key(&row.sha256))
        .collect()
}

/// `cargo xtask verify-evidence --all`: verify every Documents row of every
/// evidence file against the store.
fn verify_all(rows: &[FoundRow], root: &Path, env_store: Option<OsString>) -> Outcome {
    if rows.is_empty() {
        return Outcome::Failure(format!(
            "no Documents rows were found under {}; the audit would pass \
             vacuously, which it must not",
            evidence_dir(root).display()
        ));
    }
    let store = match resolve_store(root, env_store) {
        Ok(store) => store,
        Err(error) => return Outcome::Failure(error.message),
    };
    let map = match store_digest_map(&store) {
        Ok(map) => map,
        Err(error) => return Outcome::Failure(error.message),
    };
    let failures = verify_rows(rows, &map);
    let files = rows
        .iter()
        .map(|row| row.file.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let distinct = rows
        .iter()
        .map(|row| row.sha256.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    if failures.is_empty() {
        return Outcome::Success(format!(
            "verified {} Documents rows across {files} evidence files \
             ({distinct} distinct digests) against the evidence store at {}: \
             every digest is bytes the store holds",
            rows.len(),
            store.display()
        ));
    }
    let mut report = format!(
        "{} of {} Documents rows' digests are no file in the evidence store \
         at {}:\n",
        failures.len(),
        rows.len(),
        store.display()
    );
    for row in &failures {
        report.push_str(&format!(
            "  {}:{}: `{}` digest `{}`\n",
            row.file, row.line, row.id, row.sha256
        ));
    }
    Outcome::Failure(report)
}

/// `cargo xtask verify-evidence <id>`: resolve one id, print its citation
/// line, check the digest, print the quote context.
fn verify_one(rows: &[FoundRow], root: &Path, env_store: Option<OsString>, query: &str) -> Outcome {
    let row = match match_id(rows, query) {
        IdMatch::Exact(row) => row,
        IdMatch::NotFound => {
            return Outcome::Failure(format!(
                "no Documents row resolves '{query}' across the evidence \
                 files; pass the full '<artifact> @<capture label>' id or the \
                 artifact's file name"
            ));
        }
        IdMatch::Ambiguous(candidates) => {
            let listed = candidates
                .iter()
                .map(|row| row.id.as_str())
                .collect::<Vec<_>>()
                .join("\n  ");
            return Outcome::Failure(format!(
                "'{query}' names an artifact {} Documents rows cite; pass the \
                 full id including the capture label:\n  {listed}",
                candidates.len()
            ));
        }
    };
    let store = match resolve_store(root, env_store) {
        Ok(store) => store,
        Err(error) => return Outcome::Failure(error.message),
    };
    let map = match store_digest_map(&store) {
        Ok(map) => map,
        Err(error) => return Outcome::Failure(error.message),
    };
    let Some(paths) = map.get(&row.sha256) else {
        return Outcome::Failure(format!(
            "{}:{}: `{}`'s digest `{}` is no file in the evidence store at {}",
            row.file,
            row.line,
            row.id,
            row.sha256,
            store.display()
        ));
    };
    let store_root = store.display();
    let mut report = format!("{}:{}: {}\n", row.file, row.line, row.raw);
    report.push_str(&format!(
        "  document: {}\n  window:   {}\n  url:      {}\n  capture:  {}\n  tier:     {}\n  sha256:   {}\n",
        row.id, row.window, row.url, row.capture, row.tier, row.sha256
    ));
    for path in paths {
        report.push_str(&format!("  bytes:    {store_root}/{path}\n"));
    }
    report.push_str(&quote_report(root, &store, row, paths));
    Outcome::Success(report)
}

/// The best-effort quote section of a single verification: the evidence
/// file's quoted excerpts for the row, searched in the artifact's bytes.
/// Reading failures are swallowed — the digest check above is the
/// verification; this is the reader's aid.
fn quote_report(root: &Path, store: &Path, row: &FoundRow, paths: &[String]) -> String {
    let Some(path) = paths.first() else {
        return String::new();
    };
    let Ok(text) = fs::read_to_string(evidence_dir(root).join(&row.file)) else {
        return String::new();
    };
    let Ok(artifact) = fs::read(store.join(path)) else {
        return String::new();
    };
    let contexts = quote_contexts(&text, row, &artifact);
    if contexts.is_empty() {
        "  quote:    none of the evidence text's quoted excerpts matched the \
         artifact's bytes (best effort)\n"
            .to_owned()
    } else {
        contexts
            .iter()
            .map(|context| format!("  quote:    {context}\n"))
            .collect()
    }
}

/// Finds the evidence file's quoted excerpts for `row` inside the artifact's
/// bytes, best effort.
///
/// The excerpts are the backticked spans on the lines that cite the row's id
/// (or its artifact name), excluding the id and the digest themselves; each
/// is searched in the artifact's bytes, and the first hits (up to three) are
/// returned with some surrounding context. Binary artifacts and prose that
/// quotes nothing simply produce an empty result.
#[must_use]
pub fn quote_contexts(evidence_text: &str, row: &FoundRow, artifact: &[u8]) -> Vec<String> {
    const MAX_HITS: usize = 3;
    const CONTEXT: usize = 64;
    const MIN_EXCERPT: usize = 8;
    let artifact_text = String::from_utf8_lossy(artifact);
    let mut hits: Vec<String> = Vec::new();
    let mut quoted: BTreeSet<&str> = BTreeSet::new();
    let artifact_name = row.id.split(" @").next().unwrap_or(&row.id);
    for line in evidence_text.lines() {
        if !line.contains(&row.id) && !line.contains(artifact_name) {
            continue;
        }
        for excerpt in backticked_spans(line) {
            if excerpt.len() < MIN_EXCERPT
                || excerpt == row.id
                || excerpt == row.sha256
                || !quoted.insert(excerpt)
            {
                continue;
            }
            if let Some(at) = find_subslice(artifact, excerpt.as_bytes()) {
                let start = at.saturating_sub(CONTEXT);
                let end = (at + excerpt.len() + CONTEXT).min(artifact.len());
                let context =
                    String::from_utf8_lossy(&artifact[start..end]).replace(['\n', '\r', '\t'], " ");
                hits.push(format!("`{excerpt}` — found in the bytes: …{context}…"));
                if hits.len() >= MAX_HITS {
                    return hits;
                }
            }
        }
    }
    drop(artifact_text);
    hits
}

/// The backticked spans of one line — the text between each pair of backticks.
fn backticked_spans(line: &str) -> impl Iterator<Item = &str> {
    line.split('`')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, span)| span)
}

/// The byte offset of the first `needle` in `haystack`, if any.
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// The subcommand entry point: `verify-evidence --all`, `verify-evidence
/// <document-id>`, or a usage message for anything else.
///
/// `root` is the workspace root (the exchange-hours repository checkout);
/// `env_store` is the raw `$EXCHANGE_HOURS_RESEARCH` value when set.
pub fn run<I>(args: I, root: &Path, env_store: Option<OsString>) -> Outcome
where
    I: IntoIterator<Item = OsString>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["verify-evidence", "--all"] => match collect_rows(&evidence_dir(root)) {
            Ok(rows) => verify_all(&rows, root, env_store),
            Err(error) => Outcome::Failure(error.message),
        },
        ["verify-evidence", query] if !query.starts_with('-') => {
            match collect_rows(&evidence_dir(root)) {
                Ok(rows) => verify_one(&rows, root, env_store, query),
                Err(error) => Outcome::Failure(error.message),
            }
        }
        ["--help"] | ["-h"] | ["help"] => Outcome::Success(usage()),
        _ => Outcome::Failure(usage()),
    }
}

/// The usage message.
fn usage() -> String {
    "usage: cargo xtask verify-evidence '<document-id>'
       cargo xtask verify-evidence --all

The id is the `<artifact> @<capture label>` string a Documents row cites;
a bare artifact name resolves when exactly one row cites it. The store
resolves from $EXCHANGE_HOURS_RESEARCH, then ../exchange-hours-evidence,
then ../exchange-hours-research."
        .to_owned()
}
