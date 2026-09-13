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
use chrono::NaiveDate;
use exchange_hours::Exchange;
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
const NARRATIVE_DEBT: [&str; 40] = [
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

/// Splits a macro body into its top-level `( … )` tuples.
///
/// Reads the tuples rather than the source lines because rustfmt wraps most of
/// them across five lines; paren matching survives that, survives a literal
/// that itself contains a comma or a bracket, and survives a nested call in a
/// field — a holiday row's `early_close(12 * 3_600)` kind.
fn macro_tuples(body: &str) -> Vec<&str> {
    let mut tuples = Vec::new();
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
                    tuples.push(&body[start..index]);
                }
            }
            _ => {}
        }
    }
    tuples
}

/// Parses every tuple of one `revisions!` body.
fn revision_rows(body: &str) -> Vec<RevisionRow> {
    macro_tuples(body)
        .into_iter()
        .map(parse_revision_tuple)
        .collect()
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

/// One `holidays!` block, with the evidence files its module declares for it.
struct HolidayBlock {
    module: String,
    files: Vec<String>,
    coverage: (String, String),
    rows: Vec<HolidayRow>,
}

/// One `holidays!` tuple, reduced to what the evidence file has to record.
struct HolidayRow {
    day: String,
    document: String,
}

/// Returns whether the byte at `offset` sits on a comment line.
///
/// `holidays/mod.rs` prints a worked invocation inside the macro's own doc
/// comment, and the macro's arms name its own tokens; neither is a table.
fn on_comment_line(text: &str, offset: usize) -> bool {
    let line_start = text[..offset].rfind('\n').map_or(0, |index| index + 1);
    text[line_start..offset].trim_start().starts_with("//")
}

/// Parses the `coverage: (y, m, d) ..= (y, m, d)` clause of one block.
fn holiday_coverage(body: &str, module: &str) -> (String, String) {
    let opened = body.split_once("coverage:");
    assert!(
        opened.is_some(),
        "{module}: a holidays! block must declare its coverage window"
    );
    let clause = opened
        .expect("the coverage clause was just asserted present")
        .1;
    let closed = clause.split_once("rows:");
    assert!(
        closed.is_some(),
        "{module}: a holidays! block must declare its rows"
    );
    let bounds = macro_tuples(closed.expect("the rows list was just asserted present").0);
    assert_eq!(
        bounds.len(),
        2,
        "{module}: a coverage window reads `(year, month, day) ..= (year, month, day)`"
    );
    let day = |tuple: &str| {
        let fields = tuple_fields(tuple);
        assert_eq!(
            fields.len(),
            3,
            "{module}: a coverage bound reads `(year, month, day)`: {tuple}"
        );
        let year: i32 = fields[0].parse().expect("coverage year must be an integer");
        let month: u32 = fields[1]
            .parse()
            .expect("coverage month must be an integer");
        let date: u32 = fields[2].parse().expect("coverage day must be an integer");
        format!("{year:04}-{month:02}-{date:02}")
    };
    (day(bounds[0]), day(bounds[1]))
}

/// Parses the `rows: [ … ]` list of one block.
fn holiday_rows(body: &str) -> Vec<HolidayRow> {
    let list = body
        .split_once("rows:")
        .expect("a holidays! block must declare its rows")
        .1;
    macro_tuples(macro_body(list))
        .into_iter()
        .map(|tuple| {
            let fields = tuple_fields(tuple);
            assert_eq!(
                fields.len(),
                6,
                "a holidays! tuple is (year, month, day, kind, tier, document): {tuple}"
            );
            let year: i32 = fields[0].parse().expect("holiday year must be an integer");
            let month: u32 = fields[1].parse().expect("holiday month must be an integer");
            let day: u32 = fields[2].parse().expect("holiday day must be an integer");
            let document = fields[5]
                .strip_prefix('"')
                .and_then(|literal| literal.strip_suffix('"'))
                .expect("a holidays! tuple ends with its document-id literal");
            HolidayRow {
                day: format!("{year:04}-{month:02}-{day:02}"),
                document: document.to_owned(),
            }
        })
        .collect()
}

/// Every `holidays!` block in the crate, with its declared evidence files.
///
/// Empty while no family table ships, which is what makes the two fences below
/// pass trivially in the wave that introduces them.
fn holiday_blocks() -> Vec<HolidayBlock> {
    const MARKER: &str = "holidays! {";
    let mut blocks = Vec::new();
    for path in crate_sources() {
        let text = fs::read_to_string(&path).expect("source file must be readable");
        let module = relative(&path);
        let mut searched = 0_usize;
        while let Some(offset) = text[searched..].find(MARKER) {
            let start = searched.saturating_add(offset);
            searched = start.saturating_add(MARKER.len());
            if on_comment_line(&text, start) {
                continue;
            }
            let body = &text[start..];
            blocks.push(HolidayBlock {
                module: module.clone(),
                files: declared_evidence_files(&comment_run(&text[..start]), &module),
                coverage: holiday_coverage(body, &module),
                rows: holiday_rows(body),
            });
        }
    }
    blocks
}

/// Returns a `## Holidays` subsection's body, from `### <year>` to the next
/// `### ` heading.
fn holiday_year<'a>(holidays: &'a str, year: &str) -> Option<&'a str> {
    let (_, rest) = holidays.split_once(&format!("\n### {year}\n"))?;
    Some(rest.split("\n### ").next().unwrap_or(rest))
}

/// LAW-EVIDENCE-FILES for holiday rows: a row ships only with its quotation.
///
/// The module carries the trade date, the kind, the tier and the document id;
/// the evidence file carries the instant as the operator printed it, the
/// document's URL and capture, and the event-date-to-trade-date conversion
/// that produced the row. This fence ties the two together in the forward
/// direction, per family and per year — the unit a LAW-WATCH review works in;
/// `every_evidence_holiday_line_exists_in_its_module` closes the reverse
/// direction.
#[test]
fn every_holiday_row_appears_in_its_evidence_file() {
    let files = evidence_files();

    for block in holiday_blocks() {
        for name in &block.files {
            let text = files.get(name).unwrap_or_else(|| {
                panic!("{} declares a missing evidence file: {name}", block.module)
            });
            let holidays = section(text, "## Holidays").unwrap_or_else(|| {
                panic!(
                    "{name} must carry a `## Holidays` section: {} ships holiday rows",
                    block.module
                )
            });
            for row in &block.rows {
                let year = row
                    .day
                    .get(..4)
                    .expect("a formatted holiday day opens with its year");
                let body = holiday_year(holidays, year).unwrap_or_else(|| {
                    panic!("{name} must carry a `### {year}` holiday subsection")
                });
                let prefix = format!("| {} |", row.day);
                let recorded = body
                    .lines()
                    .find(|line| line.starts_with(&prefix))
                    .unwrap_or_else(|| {
                        panic!(
                            "{name} does not record the {} holiday row of {} under `### {year}`",
                            row.day, block.module
                        )
                    });
                assert!(
                    recorded.contains(&format!("`{}`", row.document)),
                    "{name}'s {} row does not cite the module's document id `{}`: {recorded}",
                    row.day,
                    row.document
                );
            }
        }
    }
}

/// One row of an evidence file's fixed-shape `### Documents` table.
struct DocumentRow {
    id: String,
    window: String,
    sha: String,
}

/// The `### Documents` table of one evidence file, one entry per resolved id.
///
/// The table's shape is fixed — `| Document | Window | Capture or retrieval,
/// UTC | Tier | sha256 |` — so a resolution is machine-readable and the three
/// fences below can be written at all. A left cell may name several ids
/// separated by `, ` when one artifact carries more than one.
fn document_rows(text: &str) -> Vec<DocumentRow> {
    let Some(body) = text.split_once("\n### Documents\n").map(|(_, rest)| rest) else {
        return Vec::new();
    };
    let Some(table) = body.split_once(DOCUMENT_TABLE_HEADER) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for line in table
        .1
        .lines()
        .skip_while(|line| line.is_empty() || line.starts_with("|---"))
    {
        if !line.starts_with("| `") {
            break;
        }
        let cells = line
            .trim_start_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        assert!(
            cells.len() >= 5,
            "a `### Documents` row reads {DOCUMENT_TABLE_HEADER}: {line}"
        );
        let window = cells[1].to_owned();
        let sha = cells[4].trim_matches('`').to_owned();
        for id in cells[0].split(", ") {
            rows.push(DocumentRow {
                id: id.trim().trim_matches('`').to_owned(),
                window: window.clone(),
                sha: sha.clone(),
            });
        }
    }
    rows
}

/// The one shape a `### Documents` table may take.
const DOCUMENT_TABLE_HEADER: &str =
    "| Document | Window | Capture or retrieval, UTC | Tier | sha256 |";

/// Design memo section 3.2: document ids are unique repository-wide. An id that
/// resolves to two artifacts stops keying the bytes its row rests on, which is
/// the supersession-audit failure the id scheme exists to prevent (section 8.3,
/// D15/D16).
///
/// The id names the artifact, so for a CME service window it is
/// `CME-SVC-<first eventDate>` — never the trade date of a row that reads it,
/// which two families can reach out of two different windows.
#[test]
fn every_document_id_resolves_to_one_artifact_repository_wide() {
    let mut seen: BTreeMap<String, (String, String)> = BTreeMap::new();
    let mut resolved = 0_usize;
    for (name, text) in evidence_files() {
        for row in document_rows(&text) {
            resolved = resolved.saturating_add(1);
            let artifact = format!("{} / {}", row.window, row.sha);
            match seen.get(&row.id) {
                None => {
                    seen.insert(row.id.clone(), (artifact, name.clone()));
                }
                Some((first, first_file)) => {
                    assert_eq!(
                        *first, artifact,
                        "`{}` resolves to two artifacts: {first} in {first_file}, \
                         {artifact} in {name}",
                        row.id
                    );
                }
            }
        }
    }
    assert!(
        resolved > 0,
        "docs/evidence must carry at least one `### Documents` table"
    );
}

/// The inverse: one artifact, one id. Two ids on one sha256 make a supersession
/// undiscoverable by id, which is the same audit failure read the other way.
#[test]
fn every_artifact_carries_one_document_id() {
    let mut by_sha: BTreeMap<String, (String, String)> = BTreeMap::new();
    for (name, text) in evidence_files() {
        for row in document_rows(&text) {
            match by_sha.get(&row.sha) {
                None => {
                    by_sha.insert(row.sha.clone(), (row.id.clone(), name.clone()));
                }
                Some((first, first_file)) => {
                    assert_eq!(
                        *first, row.id,
                        "sha256 `{}` carries two ids: `{first}` in {first_file}, `{}` in {name}",
                        row.sha, row.id
                    );
                }
            }
        }
    }
}

/// Every id a module cites is resolved exactly once in its evidence file's
/// `### Documents` table, so a citation always lands on bytes.
///
/// Scoped to the evidence files that carry such a table: the CME service
/// windows, whose date-shaped ids are the ones a rename can collide. Extending
/// the fixed shape to the remaining families is recorded in the research
/// store's `DECISIONS.md` as W1-ASM-7 and tracked as issue #98.
#[test]
fn every_cited_document_id_is_resolved_exactly_once() {
    let files = evidence_files();
    for block in holiday_blocks() {
        for name in &block.files {
            let text = files.get(name).unwrap_or_else(|| {
                panic!("{} declares a missing evidence file: {name}", block.module)
            });
            let rows = document_rows(text);
            if rows.is_empty() {
                continue;
            }
            for row in &block.rows {
                let hits = rows
                    .iter()
                    .filter(|resolved| resolved.id == row.document)
                    .count();
                assert_eq!(
                    hits, 1,
                    "{name} resolves the document `{}` that {} cites for {} {hits} times, \
                     not once",
                    row.document, block.module, row.day
                );
            }
        }
    }
}

/// The reverse direction of the holiday evidence fence: an evidence file's
/// holiday table is a claim about the rows the crate ships, so a row that
/// leaves a module may not keep its quotation.
///
/// The forward fence only proves that every shipped row has evidence. It is
/// blind to a row dropped while a long table is reordered: the module stops
/// answering for that trade date, the evidence file still records it, and
/// nothing fails. Design memo section 3.3 asks for both directions, so require
/// every `| <trade date> |` line under a `## Holidays` heading to name a row of
/// a module that declares the file. A date the crate deliberately does not
/// carry is a declared gap, which is prose beside the year's table and never a
/// line in it.
#[test]
fn every_evidence_holiday_line_exists_in_its_module() {
    let mut by_file: BTreeMap<String, BTreeMap<String, BTreeSet<String>>> = BTreeMap::new();
    for block in holiday_blocks() {
        for name in &block.files {
            let shipped = by_file.entry(name.clone()).or_default();
            for row in &block.rows {
                shipped
                    .entry(row.day.clone())
                    .or_default()
                    .insert(row.document.clone());
            }
        }
    }

    for (name, text) in evidence_files() {
        let Some(holidays) = section(&text, "## Holidays") else {
            continue;
        };
        let shipped = by_file.get(&name);
        for line in holidays.lines().filter(|line| line.starts_with("| 2")) {
            let day = line
                .trim_start_matches('|')
                .split('|')
                .next()
                .map(str::trim)
                .expect("a holiday line carries at least one cell");
            let documents = shipped.and_then(|rows| rows.get(day)).unwrap_or_else(|| {
                panic!(
                    "{name} records a {day} holiday row that no holidays! block declaring \
                     the file ships: restore the row, or move the date to the year's gaps"
                )
            });
            assert!(
                documents
                    .iter()
                    .any(|document| line.contains(&format!("`{document}`"))),
                "{name}'s {day} row cites no document id the module ships for that \
                 trade date: {line}"
            );
        }
    }
}

/// A table's coverage window is a claim, so it is recorded where a reader can
/// check it.
///
/// Inside the window a date with no row means "audited and normal". That is a
/// statement about every date in the range, not only about the rows, so the
/// range itself has to be stated in the evidence file rather than inferred
/// from the module.
#[test]
fn every_holiday_table_states_its_coverage_window() {
    let files = evidence_files();

    for block in holiday_blocks() {
        let (first, last) = &block.coverage;
        for name in &block.files {
            let text = files.get(name).unwrap_or_else(|| {
                panic!("{} declares a missing evidence file: {name}", block.module)
            });
            let holidays = section(text, "## Holidays").unwrap_or_else(|| {
                panic!(
                    "{name} must carry a `## Holidays` section: {} ships holiday rows",
                    block.module
                )
            });
            assert!(
                holidays.contains(&format!("**Coverage:** {first} .. {last}")),
                "{name} must state {}'s coverage window as \
                 `**Coverage:** {first} .. {last}`",
                block.module
            );
        }
    }
}

/// The evidence file's own holiday tables must parse, whether or not a fence
/// above reached them.
///
/// The forward fences check the rows a module ships. This one checks the shape
/// of everything written under `## Holidays`, so a malformed row — a missing
/// document id, a tier LAW-PRIMARY-SOURCES does not admit for a holiday, a
/// date that is not an ISO day — fails even before the module that needs it
/// exists.
#[test]
fn every_evidence_holiday_line_is_well_formed() {
    for (name, text) in evidence_files() {
        let Some(holidays) = section(&text, "## Holidays") else {
            continue;
        };
        assert!(
            holidays.contains("**Coverage:** "),
            "{name}'s `## Holidays` section must open with its coverage window"
        );
        for line in holidays.lines().filter(|line| line.starts_with("| 2")) {
            let cells = line
                .trim_start_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            assert!(
                cells.len() >= 6,
                "{name}: a holiday line reads \
                 `| trade date | kind | instant as printed | document | tier | derived from |`: \
                 {line}"
            );
            let day = cells[0];
            assert!(
                day.len() == 10 && NaiveDate::parse_from_str(day, "%Y-%m-%d").is_ok(),
                "{name}: a holiday line must open with a real ISO calendar date: {line}"
            );
            assert!(
                matches!(
                    cells[1],
                    "closed"
                        | "early close"
                        | "late open"
                        | "late open and early close"
                        | "unsourced"
                ),
                "{name}: a holiday line's kind must be one the crate can represent: {line}"
            );
            assert!(
                cells[3].starts_with('`') && cells[3].ends_with('`') && cells[3].len() > 2,
                "{name}: a holiday line must name its document id in backticks: {line}"
            );
            assert!(
                matches!(cells[4], "T1" | "T2"),
                "{name}: a holiday line must carry tier T1 or T2; \
                 LAW-PRIMARY-SOURCES admits nothing lower for a holiday row: {line}"
            );
            assert!(
                !cells[5].is_empty(),
                "{name}: a holiday line must record the operator event dates it was \
                 derived from: {line}"
            );
        }
    }
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

/// Day-level boundaries an identity's `profile_at` selects on directly — the
/// `NaiveDate` and Unix-second constants in `eurex_profile_at`, `eex_profile_at`,
/// `b3.rs`, `bmv.rs`, `ice_endex.rs`, `ice_abu_dhabi.rs`, `binance.rs`,
/// `ice_canada.rs` and `coinbase_derivatives.rs` — never reach a `revisions!`
/// block, so every fence above is blind to them: a selector could move a date
/// with no evidence file recording it.
///
/// The handwritten cutover lists in
/// `tests/contract/session_invariants/historical_expectations.rs` do see them,
/// because they are compared against observable behaviour rather than generated
/// from the timelines. Require every date they record for an `Exchange` to
/// appear in that identity's evidence file, in its `## Revision rows` section
/// or in a `## Dated selectors` section beside it.
///
/// `HISTORICAL_INSTANT_CUTOVERS` records a UTC instant, and an evening boundary
/// belongs to the previous venue-local day (ICE Canada's 18:30 CT pre-open is
/// `2011-03-01 00:30 UTC`), so either day satisfies the fence for those rows.
///
/// Scope: both lists are keyed by `Exchange`, so a `MarketHoursKey` whose file
/// records the same shared selector — `eurex_key` against `eurex_profile_at`'s
/// 2018-12-10 boundary — is not reached here. Its `## Dated selectors` line is
/// kept for the reader; extending the cutover lists to keys is issue #86.
#[test]
fn every_handwritten_cutover_date_appears_in_its_evidence_file() {
    const DAY_LIST: &str = "HISTORICAL_CUTOVERS:";
    const INSTANT_LIST: &str = "HISTORICAL_INSTANT_CUTOVERS:";

    let path =
        repository_root().join("tests/contract/session_invariants/historical_expectations.rs");
    let source = fs::read_to_string(&path).expect("cutover expectations must be readable");
    let files = evidence_files();

    let wire_names = Exchange::ALL
        .iter()
        .map(|exchange| (format!("{exchange:?}"), exchange.as_str()))
        .collect::<BTreeMap<_, _>>();

    let mut checked = 0_usize;
    for (list, allow_previous_day) in [(DAY_LIST, false), (INSTANT_LIST, true)] {
        for (variant, day) in cutover_entries(&source, list) {
            let wire = wire_names
                .get(&variant)
                .unwrap_or_else(|| panic!("{list} names an unknown Exchange variant: {variant}"));
            let name = format!("{wire}.md");
            let text = files.get(&name).unwrap_or_else(|| {
                panic!("{list} names an identity with no evidence file: {name}")
            });

            let mut keyed = BTreeSet::new();
            for heading in ["## Revision rows", "## Dated selectors"] {
                let body = section(text, heading).unwrap_or_default();
                for bullet in body.lines().filter(|line| line.starts_with("- ")) {
                    keyed.extend(dated_bullet_key(&name, heading, bullet));
                }
            }

            let previous = day
                .pred_opt()
                .expect("a cutover date must have a previous day")
                .format("%Y-%m-%d")
                .to_string();
            let day = day.format("%Y-%m-%d").to_string();

            assert!(
                keyed.contains(&day) || (allow_previous_day && keyed.contains(&previous)),
                "{name} keys no revision row or dated selector to the {day} cutover \
                 that {list} holds for {wire}; a date mentioned only in prose does not \
                 count — add a bullet under `## Revision rows` or `## Dated selectors` \
                 reading `- {day} \u{2014} T<n> \u{2014} <document id> \u{2014} <label>`"
            );
            checked = checked.saturating_add(1);
        }
    }

    assert!(
        checked > 100,
        "the cutover lists must parse; only {checked} entries were read"
    );
}

/// Returns the ISO day a dated bullet is keyed to, if it is one.
///
/// The grammar is the revision-row grammar `assert_revision_bullet` enforces:
/// `- YYYY-MM-DD — T<n> — <document id> — <label>`. A bullet that opens with an
/// ISO day must carry the rest of it, so a cutover cannot be satisfied by a
/// bullet that names a date without saying which document dates it; a bullet
/// that opens with anything else is ordinary prose and is skipped.
fn dated_bullet_key(name: &str, heading: &str, bullet: &str) -> Option<String> {
    let rest = bullet.trim_start_matches("- ");
    let day = rest.get(..10)?;
    let mut parts = day.split('-');
    let looks_dated = parts.clone().count() == 3
        && parts.all(|field| !field.is_empty() && field.chars().all(|c| c.is_ascii_digit()));
    if !looks_dated {
        return None;
    }
    assert!(
        NaiveDate::parse_from_str(day, "%Y-%m-%d").is_ok(),
        "{name}: a `{heading}` bullet opens with {day}, which is not a real calendar date: {bullet}"
    );

    let fields = rest.splitn(4, " \u{2014} ").collect::<Vec<_>>();
    assert_eq!(
        fields.len(),
        4,
        "{name}: a bullet under `{heading}` that opens with an ISO day must read \
         `- YYYY-MM-DD \u{2014} T<n> \u{2014} <document id> \u{2014} <label>`: {bullet}"
    );
    assert!(
        matches!(fields[1], "T1" | "T2" | "T3" | "T4"),
        "{name}: a dated bullet under `{heading}` must carry its own evidence tier: {bullet}"
    );
    assert!(
        !fields[2].trim().is_empty(),
        "{name}: a dated bullet under `{heading}` must name the document that dates it: {bullet}"
    );
    Some(day.to_owned())
}

/// Parses `(Exchange::Variant, (y, m, d), ..)` entries out of one cutover list.
fn cutover_entries(source: &str, list: &str) -> Vec<(String, NaiveDate)> {
    let start = source.find(list);
    assert!(start.is_some(), "cutover expectations must declare {list}");
    let start = start.expect("the list name was just asserted present");

    let opening = source[start..].find("= &[");
    assert!(opening.is_some(), "{list} must open a slice literal");
    let open = opening
        .expect("the slice literal was just asserted present")
        .saturating_add(start)
        .saturating_add(4);

    let mut depth = 1_usize;
    let mut end = open;
    for (offset, byte) in source[open..].bytes().enumerate() {
        match byte {
            b'[' => depth = depth.saturating_add(1),
            b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
        if depth == 0 {
            end = open.saturating_add(offset);
            break;
        }
    }

    let mut entries = Vec::new();
    let mut rest = &source[open..end];
    while let Some(offset) = rest.find("Exchange::") {
        let after = &rest[offset.saturating_add("Exchange::".len())..];
        let variant = after
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect::<String>();
        let Some(tuple_start) = after.find('(') else {
            break;
        };
        let Some(tuple_end) = after[tuple_start..].find(')') else {
            break;
        };
        let numbers = after[tuple_start.saturating_add(1)..tuple_start.saturating_add(tuple_end)]
            .split(',')
            .filter_map(|field| field.trim().parse::<i32>().ok())
            .collect::<Vec<_>>();
        assert!(
            numbers.len() >= 3,
            "{list} entry for {variant} must open with (year, month, day)"
        );
        let date = NaiveDate::from_ymd_opt(
            numbers[0],
            u32::try_from(numbers[1]).expect("month must be non-negative"),
            u32::try_from(numbers[2]).expect("day must be non-negative"),
        )
        .expect("every cutover entry must name a real calendar date");
        entries.push((variant, date));
        rest = &after[tuple_start.saturating_add(tuple_end)..];
    }
    entries
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
