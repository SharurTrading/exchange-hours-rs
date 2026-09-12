// SPDX-License-Identifier: MIT-0

//! Fences on `docs/evidence/`, the narrative store LAW-EVIDENCE-FILES requires.
//!
//! The law splits one artifact into three: rule data and one citation line per
//! revision row in the module, a fixed-shape row in the ledger, and the
//! quotations, URLs, retrieval dates, conflicts and residual risks in
//! `docs/evidence/<owner>.md`. Nothing tied those three together, so a moved
//! narrative could silently lose a row, a row could point at a file that does
//! not exist, and a file could outlive the row it was written for.
//!
//! Attribution runs through the module's own declaration: immediately above
//! every `revisions!` block, one `// Evidence: docs/evidence/<file>.md` line
//! (comma-separated when one timeline serves several rows, continued on the
//! next comment line when it wraps). Mapping modules to identities any other
//! way fails on the 23 identities whose timeline lives in a `history.rs`
//! sibling and on the ten modules whose single timeline serves several rows.

use super::{EVIDENCE_DIR, evidence_target, exchange_rows, market_hours_key_rows, wire_name};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The schedule tree LAW-EVIDENCE-FILES governs: the ledger's owner modules,
/// their timeline siblings, and the synthetic profiles beside them.
const SCHEDULE_ROOTS: [&str; 3] = [
    "src/calendar/schedules",
    "src/calendar/futures_profile",
    "src/calendar/futures_profile.rs",
];

/// Longest run of plain `//` lines a migrated schedule module may carry.
///
/// Doc comments are exempt: `missing_docs` requires them and they document the
/// item, not the evidence. A longer plain-comment run is narrative, and
/// narrative belongs in the evidence file.
const MAX_COMMENT_RUN: usize = 6;

/// Schedule modules whose narrative has not moved yet.
///
/// The reshape drained the served families and most of the dormant ones; these
/// modules still carry prose. A listed module is skipped by
/// `modules_carry_no_narrative`, an unlisted one is asserted, and the list only
/// ever shrinks — that is what makes "a new module never carries one" a fence
/// rather than a hope. Tracked as issue #85.
const NARRATIVE_DEBT: [&str; 41] = [
    "src/calendar/schedules/equities/africa_middle_east/jse.rs",
    "src/calendar/schedules/equities/africa_middle_east/tadawul.rs",
    "src/calendar/schedules/equities/americas/b3.rs",
    "src/calendar/schedules/equities/americas/bmv.rs",
    "src/calendar/schedules/equities/americas/tsx.rs",
    "src/calendar/schedules/equities/apac/asx.rs",
    "src/calendar/schedules/equities/apac/bse.rs",
    "src/calendar/schedules/equities/apac/bursa.rs",
    "src/calendar/schedules/equities/apac/hkex.rs",
    "src/calendar/schedules/equities/apac/hose.rs",
    "src/calendar/schedules/equities/apac/idx.rs",
    "src/calendar/schedules/equities/apac/krx.rs",
    "src/calendar/schedules/equities/apac/nse.rs",
    "src/calendar/schedules/equities/apac/nzx.rs",
    "src/calendar/schedules/equities/apac/set.rs",
    "src/calendar/schedules/equities/apac/sgx.rs",
    "src/calendar/schedules/equities/apac/sse.rs",
    "src/calendar/schedules/equities/apac/szse.rs",
    "src/calendar/schedules/equities/apac/tmx_australia.rs",
    "src/calendar/schedules/equities/apac/tse.rs",
    "src/calendar/schedules/equities/apac/twse.rs",
    "src/calendar/schedules/equities/europe/bist.rs",
    "src/calendar/schedules/equities/europe/bme.rs",
    "src/calendar/schedules/equities/europe/euronext.rs",
    "src/calendar/schedules/equities/europe/euronext/dublin.rs",
    "src/calendar/schedules/equities/europe/lse.rs",
    "src/calendar/schedules/equities/europe/nasdaq_nordics.rs",
    "src/calendar/schedules/equities/europe/six.rs",
    "src/calendar/schedules/equities/europe/vienna.rs",
    "src/calendar/schedules/equities/europe/xetra.rs",
    "src/calendar/schedules/equities/us/cboe.rs",
    "src/calendar/schedules/equities/us/nyse.rs",
    "src/calendar/schedules/futures/international/binance.rs",
    "src/calendar/schedules/futures/international/ice_abu_dhabi.rs",
    "src/calendar/schedules/futures/international/ice_canada.rs",
    "src/calendar/schedules/futures/international/ice_endex.rs",
    "src/calendar/schedules/futures/international/ice_europe.rs",
    "src/calendar/schedules/futures/international/sgx.rs",
    "src/calendar/schedules/futures/international/sgx_equity_index/eras.rs",
    "src/calendar/schedules/futures/international/sgx_equity_index/history.rs",
    "src/calendar/schedules/futures/us/small_exchange.rs",
];

/// Sections every evidence file carries, in order.
const REQUIRED_SECTIONS: [&str; 4] = [
    "## Ledger basis (moved from docs/schedules/verification.md on ",
    "## Revision rows",
    "## Sources",
    "## Gaps and residual risks",
];

/// One `revisions!` block, with the evidence files its module declares for it.
struct RevisionBlock {
    module: String,
    files: Vec<String>,
    rows: Vec<RevisionRow>,
}

/// One `revisions!` tuple: its effective day and its citation literal.
struct RevisionRow {
    day: String,
    citation: String,
}

/// Whether a file name is a Markdown file.
fn is_markdown(name: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Every `.rs` file under the schedule tree, sorted.
fn schedule_sources() -> Vec<PathBuf> {
    let root = repository_root();
    let mut sources = Vec::new();
    for entry in SCHEDULE_ROOTS {
        collect_rust_files(&root.join(entry), &mut sources);
    }
    sources.sort();
    sources
}

fn collect_rust_files(path: &Path, into: &mut Vec<PathBuf>) {
    if path.is_file() {
        if path.extension().is_some_and(|extension| extension == "rs") {
            into.push(path.to_path_buf());
        }
        return;
    }
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("schedule tree must be readable");
        collect_rust_files(&entry.path(), into);
    }
}

/// Every `.rs` file in the crate, sorted: `revisions!` blocks are fenced
/// wherever they live, not only inside the schedule tree.
fn crate_sources() -> Vec<PathBuf> {
    let mut sources = Vec::new();
    collect_rust_files(&repository_root().join("src"), &mut sources);
    sources.sort();
    sources
}

fn relative(path: &Path) -> String {
    path.strip_prefix(repository_root())
        .expect("source file must live in the repository")
        .to_string_lossy()
        .replace('\\', "/")
}

/// Returns the contiguous `//` comment run immediately above a declaration.
///
/// `prefix` ends at the `revisions![` token, so the walk steps over the binding
/// line the macro sits on and any attribute stacked on it — nothing else — then
/// collects the comment run and stops at the first line that is neither.
fn comment_run(prefix: &str) -> Vec<String> {
    let mut lines: Vec<&str> = prefix.lines().collect();
    if lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    let mut run = Vec::new();
    let mut in_comments = false;
    for (position, line) in lines.iter().rev().enumerate() {
        let trimmed = line.trim();
        if !in_comments {
            if trimmed.starts_with("//") {
                in_comments = true;
            } else if position == 0 || trimmed.starts_with('#') {
                // Only the `static … =` line the macro is bound to, and any
                // attribute stacked on it, may be stepped over. Anything else
                // ends the run, so a block with no declaration of its own
                // cannot inherit the preceding block's.
                continue;
            } else {
                break;
            }
        }
        if trimmed.starts_with("//") {
            run.push(trimmed.to_owned());
        } else {
            break;
        }
    }
    run.reverse();
    run
}

/// Parses the one `// Evidence:` declaration out of a comment run.
fn declared_evidence_files(run: &[String], module: &str) -> Vec<String> {
    const MARKER: &str = "// Evidence:";
    let declarations = run
        .iter()
        .enumerate()
        .filter(|(_, line)| line.starts_with(MARKER))
        .collect::<Vec<_>>();
    assert_eq!(
        declarations.len(),
        1,
        "every revisions! block needs exactly one `// Evidence:` line above it: {module}"
    );
    let (mut index, first) = declarations[0];
    let mut declaration = first[MARKER.len()..].trim().to_owned();
    // A long declaration wraps onto the next comment line; the break is only
    // legal after a comma, so the continuation is unambiguous.
    while declaration.ends_with(',') && index + 1 < run.len() {
        index += 1;
        declaration.push(' ');
        declaration.push_str(run[index].trim_start_matches('/').trim());
    }
    declaration
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let name = entry
                .strip_prefix("docs/evidence/")
                .expect("declared evidence path must live under docs/evidence/");
            assert!(
                is_markdown(name) && !name.contains('/'),
                "declared evidence path must name one Markdown file: {entry} ({module})"
            );
            name.to_owned()
        })
        .collect()
}

/// Returns the body of the `revisions![ … ]` invocation starting at `text`.
fn macro_body(text: &str) -> &str {
    let open = text.find('[').expect("revisions! must open its bracket");
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut close = None;
    for (index, character) in text.char_indices().skip(open) {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '[' => depth = depth.saturating_add(1),
            ']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    close = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close.expect("revisions! block must close its bracket");
    &text[open.saturating_add(1)..close]
}

/// Splits a tuple body on the commas that separate its fields.
fn tuple_fields(tuple: &str) -> Vec<&str> {
    let mut fields = Vec::new();
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut start = 0_usize;
    for (index, character) in tuple.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '(' | '[' | '{' => depth = depth.saturating_add(1),
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                fields.push(tuple[start..index].trim());
                start = index.saturating_add(1);
            }
            _ => {}
        }
    }
    fields.push(tuple[start..].trim());
    fields.retain(|field| !field.is_empty());
    fields
}

/// Parses every tuple of one `revisions!` body.
///
/// Reads the tuples rather than the source lines because rustfmt wraps most of
/// them across five lines; brace matching survives that, and survives a
/// citation literal that itself contains a comma or a bracket.
fn revision_rows(body: &str) -> Vec<RevisionRow> {
    let mut rows = Vec::new();
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut start = 0_usize;
    for (index, character) in body.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '(' => {
                depth = depth.saturating_add(1);
                if depth == 1 {
                    start = index.saturating_add(1);
                }
            }
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    rows.push(parse_revision_tuple(&body[start..index]));
                }
            }
            _ => {}
        }
    }
    rows
}

fn parse_revision_tuple(tuple: &str) -> RevisionRow {
    let fields = tuple_fields(tuple);
    assert!(
        fields.len() >= 5,
        "a revisions! tuple is (year, month, day, profile, citation): {tuple}"
    );
    let year: i32 = fields[0].parse().expect("revision year must be an integer");
    let month: u32 = fields[1]
        .parse()
        .expect("revision month must be an integer");
    let day: u32 = fields[2].parse().expect("revision day must be an integer");
    let citation = fields
        .last()
        .expect("a tuple has fields")
        .strip_prefix('"')
        .and_then(|literal| literal.strip_suffix('"'))
        .expect("a revisions! tuple ends with its citation literal");
    RevisionRow {
        day: format!("{year:04}-{month:02}-{day:02}"),
        citation: citation.to_owned(),
    }
}

fn revision_blocks() -> Vec<RevisionBlock> {
    let mut blocks = Vec::new();
    for path in crate_sources() {
        let text = fs::read_to_string(&path).expect("source file must be readable");
        let module = relative(&path);
        let mut searched = 0_usize;
        while let Some(offset) = text[searched..].find("revisions![") {
            let start = searched.saturating_add(offset);
            let run = comment_run(&text[..start]);
            blocks.push(RevisionBlock {
                module: module.clone(),
                files: declared_evidence_files(&run, &module),
                rows: revision_rows(macro_body(&text[start..])),
            });
            searched = start.saturating_add("revisions![".len());
        }
    }
    blocks
}

fn evidence_dir() -> PathBuf {
    repository_root().join(EVIDENCE_DIR)
}

/// Every evidence file, by file name.
fn evidence_files() -> BTreeMap<String, String> {
    fs::read_dir(evidence_dir())
        .expect("docs/evidence must exist")
        .map(|entry| {
            let path = entry.expect("evidence directory must be readable").path();
            let name = path
                .file_name()
                .expect("evidence file must have a name")
                .to_string_lossy()
                .into_owned();
            assert!(is_markdown(&name), "docs/evidence holds Markdown only");
            let text = fs::read_to_string(&path).expect("evidence file must be readable");
            (name, text)
        })
        .collect()
}

/// Returns a section's body, from its heading to the next `## ` heading.
fn section<'a>(text: &'a str, heading: &str) -> Option<&'a str> {
    let (_, rest) = text.split_once(&format!("\n{heading}"))?;
    Some(rest.split("\n## ").next().unwrap_or(rest))
}

/// The evidence file each ledger row links, by file name.
fn linked_evidence_files() -> Vec<(String, &'static str, &'static str)> {
    let exchange_names = exchange_rows()
        .into_iter()
        .map(wire_name)
        .collect::<BTreeSet<_>>();
    let mut linked = Vec::new();
    for row in exchange_rows() {
        linked.push((format!("{}.md", wire_name(row)), "`Exchange`", row));
    }
    for row in market_hours_key_rows() {
        let wire = wire_name(row);
        // A key that shares its wire name with an exchange takes the `_key`
        // suffix; no exchange wire name ends in `_key`, so it cannot collide.
        let suffix = if exchange_names.contains(wire) {
            "_key"
        } else {
            ""
        };
        linked.push((format!("{wire}{suffix}.md"), "`MarketHoursKey`", row));
    }
    linked
}

#[test]
fn every_revisions_block_declares_its_evidence_files() {
    let linked = linked_evidence_files()
        .into_iter()
        .map(|(name, _, _)| name)
        .collect::<BTreeSet<_>>();
    let blocks = revision_blocks();
    assert!(!blocks.is_empty(), "the crate must hold revision timelines");

    for block in blocks {
        assert!(
            !block.files.is_empty(),
            "revisions! block in {} declares no evidence file",
            block.module
        );
        for file in block.files {
            assert!(
                evidence_dir().join(&file).is_file(),
                "{} declares a missing evidence file: {file}",
                block.module
            );
            assert!(
                linked.contains(&file),
                "{} declares an evidence file no ledger row links: {file}",
                block.module
            );
        }
    }
}

#[test]
fn every_revision_row_day_appears_in_its_evidence_file() {
    let files = evidence_files();

    for block in revision_blocks() {
        for name in &block.files {
            let text = files.get(name).unwrap_or_else(|| {
                panic!("{} declares a missing evidence file: {name}", block.module)
            });
            let rows = section(text, "## Revision rows")
                .unwrap_or_else(|| panic!("{name} must carry a `## Revision rows` section"));
            for row in &block.rows {
                assert!(
                    rows.contains(&format!("- {} \u{2014}", row.day)),
                    "{name} does not record the {} revision row of {}",
                    row.day,
                    block.module
                );
            }
        }
    }
}

#[test]
fn every_evidence_revision_line_exists_in_source() {
    let mut by_file: BTreeMap<String, Vec<RevisionRow>> = BTreeMap::new();
    for block in revision_blocks() {
        for name in block.files {
            by_file.entry(name).or_default().extend(
                block
                    .rows
                    .iter()
                    .map(|row| RevisionRow {
                        day: row.day.clone(),
                        citation: row.citation.clone(),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }

    for (name, text) in evidence_files() {
        let rows = section(&text, "## Revision rows")
            .unwrap_or_else(|| panic!("{name} must carry a `## Revision rows` section"));
        let bullets = rows
            .lines()
            .filter(|line| line.starts_with("- "))
            .collect::<Vec<_>>();
        if bullets.is_empty() {
            assert!(
                rows.trim_start().starts_with("None."),
                "{name} lists no revision row and does not say why"
            );
            assert!(
                !by_file.contains_key(&name),
                "{name} says it has no revision row, but a revisions! block declares it"
            );
            continue;
        }
        for bullet in bullets {
            assert_revision_bullet(&name, bullet, by_file.get(&name));
        }
    }
}

/// The two directional fences above test membership only, so counts can drift:
/// a file may repeat one evidence bullet, or carry one bullet for two timeline
/// rows that share a day, and both directions still pass. Compare multisets of
/// `(day, citation)` so the counts have to match as well.
///
/// Scope matches the reverse fence: a file no `revisions!` block declares has
/// no source multiset to compare against and is checked for grammar only
/// (issue #86).
///
/// Multiplicities add **within** one timeline and merge by maximum **across**
/// timelines that declare the same file. A seasonal twin — `eurex_fixed_income`
/// carries a CEST timeline and a CET one holding the identical two days — is
/// two encodings of one dated change, and LAW-EVIDENCE-FILES asks for one
/// bullet per change, not one per encoding.
#[test]
fn revision_rows_and_evidence_bullets_match_as_multisets() {
    let mut declared: BTreeMap<String, BTreeMap<(String, String), usize>> = BTreeMap::new();
    for block in revision_blocks() {
        let mut per_block: BTreeMap<(String, String), usize> = BTreeMap::new();
        for row in &block.rows {
            let slot = per_block
                .entry((row.day.clone(), row.citation.clone()))
                .or_default();
            *slot = slot.saturating_add(1);
        }
        for name in block.files {
            let counts = declared.entry(name).or_default();
            for (key, count) in &per_block {
                let slot = counts.entry(key.clone()).or_default();
                *slot = (*slot).max(*count);
            }
        }
    }

    let files = evidence_files();
    for (name, expected) in &declared {
        let text = files
            .get(name)
            .unwrap_or_else(|| panic!("missing evidence file: {name}"));
        let rows = section(text, "## Revision rows")
            .unwrap_or_else(|| panic!("{name} must carry a `## Revision rows` section"));

        let mut recorded: BTreeMap<(String, String), usize> = BTreeMap::new();
        for bullet in rows.lines().filter(|line| line.starts_with("- ")) {
            let fields = bullet
                .trim_start_matches("- ")
                .splitn(4, " \u{2014} ")
                .collect::<Vec<_>>();
            assert_eq!(
                fields.len(),
                4,
                "{name} revision line must read \
                 `- YYYY-MM-DD \u{2014} T<n> \u{2014} <id> \u{2014} <label>`: {bullet}"
            );
            let slot = recorded
                .entry((fields[0].to_owned(), fields[2].to_owned()))
                .or_default();
            *slot = slot.saturating_add(1);
        }

        assert_eq!(
            &recorded, expected,
            "{name}'s `## Revision rows` bullets must match its declaring timelines as a \
             multiset, one bullet per row: left is the evidence file, right is the source"
        );
    }
}

/// Asserts one `## Revision rows` bullet parses and matches its source tuple.
///
/// A module that encodes its dated cutovers as `NaiveDate` constants rather
/// than `revisions!` tuples (`vienna.rs`, `europe.rs`, `binance.rs` and the
/// ICE siblings) declares no block, so its bullets are checked for grammar
/// only; extending the day fence to those boundaries is issue #86.
fn assert_revision_bullet(name: &str, bullet: &str, source: Option<&Vec<RevisionRow>>) {
    let fields = bullet
        .trim_start_matches("- ")
        .splitn(4, " \u{2014} ")
        .collect::<Vec<_>>();
    assert_eq!(
        fields.len(),
        4,
        "{name} revision line must read `- YYYY-MM-DD \u{2014} T<n> \u{2014} <id> \u{2014} <label>`: {bullet}"
    );
    let (day, tier, id) = (fields[0], fields[1], fields[2]);
    assert!(
        day.len() == 10 && day.split('-').count() == 3,
        "{name} revision line must open with an ISO day: {bullet}"
    );
    assert!(
        matches!(tier, "T1" | "T2" | "T3" | "T4"),
        "{name} revision line must carry that row's own evidence tier: {bullet}"
    );
    assert!(
        !fields[3].trim().is_empty(),
        "{name} revision line must carry a label: {bullet}"
    );
    let Some(rows) = source else {
        return;
    };
    let matching = rows.iter().filter(|row| row.day == day).collect::<Vec<_>>();
    assert!(
        !matching.is_empty(),
        "{name} records a revision row no declaring timeline holds: {bullet}"
    );
    assert!(
        matching.iter().any(|row| row.citation == id),
        "{name}'s citation for {day} is not the timeline's literal: {bullet}"
    );
}

#[test]
fn every_ledger_row_links_an_existing_evidence_file() {
    for (expected, _, row) in linked_evidence_files() {
        assert_eq!(
            evidence_target(row),
            expected,
            "ledger row must link its own evidence file: {row}"
        );
        assert!(
            evidence_dir().join(&expected).is_file(),
            "ledger row links a missing evidence file: {expected}"
        );
    }
}

#[test]
fn every_evidence_file_is_linked_by_exactly_one_row() {
    let mut linked: BTreeMap<String, usize> = BTreeMap::new();
    for (name, _, _) in linked_evidence_files() {
        *linked.entry(name).or_default() += 1;
    }
    for (name, count) in &linked {
        assert_eq!(*count, 1, "{name} is linked by {count} ledger rows");
    }

    let present = evidence_files().into_keys().collect::<BTreeSet<_>>();
    let expected = linked.into_keys().collect::<BTreeSet<_>>();
    assert_eq!(
        present, expected,
        "docs/evidence and the ledger's evidence links must be a bijection"
    );
}

#[test]
fn every_evidence_file_has_the_required_sections() {
    for (name, text) in evidence_files() {
        let mut lines = text.lines();
        assert_eq!(
            lines.next(),
            Some("<!-- SPDX-License-Identifier: MIT-0 -->"),
            "{name} must open with the SPDX header"
        );
        assert!(
            text.contains("\n# `"),
            "{name} must title itself with its identity"
        );
        let mut position = 0_usize;
        for heading in REQUIRED_SECTIONS {
            let found = text
                .find(&format!("\n{heading}"))
                .unwrap_or_else(|| panic!("{name} is missing the section `{heading}`"));
            assert!(
                found > position,
                "{name} carries its sections out of order at `{heading}`"
            );
            position = found;
        }
    }
}

#[test]
fn every_evidence_file_names_its_ledger_row() {
    let files = evidence_files();
    for (name, kind, _) in linked_evidence_files() {
        let text = files
            .get(&name)
            .unwrap_or_else(|| panic!("missing evidence file: {name}"));
        assert!(
            text.contains(&format!("- **Kind:** {kind}")),
            "{name} must declare the surface its ledger row sits on: {kind}"
        );
    }
}

#[test]
fn modules_carry_no_narrative() {
    let debt = NARRATIVE_DEBT.iter().copied().collect::<BTreeSet<_>>();
    let mut migrated = 0_usize;

    for path in schedule_sources() {
        let module = relative(&path);
        let text = fs::read_to_string(&path).expect("schedule module must be readable");
        let longest = longest_comment_run(&text);
        if debt.contains(module.as_str()) {
            assert!(
                longest > MAX_COMMENT_RUN,
                "{module} carries no narrative any more; remove it from NARRATIVE_DEBT"
            );
            continue;
        }
        migrated = migrated.saturating_add(1);
        assert!(
            longest <= MAX_COMMENT_RUN,
            "LAW-EVIDENCE-FILES: {module} carries a {longest}-line comment run; \
             narrative belongs in its evidence file"
        );
    }

    assert!(migrated > 0, "the schedule tree must hold migrated modules");
}

/// Returns the longest run of plain `//` lines in a module.
fn longest_comment_run(text: &str) -> usize {
    let mut longest = 0_usize;
    let mut run = 0_usize;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            run = 0;
        } else if trimmed.starts_with("//") {
            run = run.saturating_add(1);
            longest = longest.max(run);
        } else {
            run = 0;
        }
    }
    longest
}
