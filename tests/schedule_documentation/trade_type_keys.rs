// SPDX-License-Identifier: MIT-0

//! Fences for the CME trade-type key sequence (issue #58).
//!
//! Five artifacts that the last two key additions restated by hand, and that
//! the thirty-two additions planned in
//! `docs/plans/2026-09-12-cme-trade-type-keys.md` each restate again. None of
//! them was derived from the ledger before this module existed.

use super::{
    README, SOURCES, UNSUPPORTED_FAMILIES, VERIFICATION, market_hours_key_rows, number_words,
    row_cells, wire_name,
};
use exchange_hours::MarketHoursKey;

const HANDOFF: &str = include_str!("../../docs/plans/2026-09-05-cme-trade-type-handoff.md");
const TRADE_TYPE_PLAN: &str = include_str!("../../docs/plans/2026-09-12-cme-trade-type-keys.md");
const GOLDEN_GRIDS: &str = include_str!("../golden_grids.rs");

/// The three handoff rows answered with a rejection on evidence, anchored on
/// the first root of each.
///
/// Handwritten, because deciding that a row is a rejection rather than an open
/// research item is a judgement and a fourth one must be made deliberately. The
/// roots themselves are read out of the handoff, so a root added to one of
/// these rows goes red until `unsupported-families.md` names it.
const REJECTED_HANDOFF_ROWS: [&str; 3] = ["TNT", "TAS", "AWT"];

/// Collective phrases the executable-gap enumerations use in place of naming
/// every member key.
///
/// `(wire-name prefix, phrase)`. Handwritten so that folding a family into a
/// collective phrase is a deliberate edit;
/// `every_executable_gap_row_is_named_in_the_prose` also fails if a prefix
/// stops matching any executable row, or if a written-out count beside a
/// phrase disagrees with the ledger.
const EXECUTABLE_COLLECTIVE_NAMES: [(&str, &str); 3] = [
    ("ice_us_", "ICE Futures U.S. keys"),
    ("sgx_equity_index_", "SGX equity-index keys"),
    ("globex_nikkei_225_dollar", "CME Nikkei 225 Dollar"),
];

/// Returns every single-line span between a pair of backticks.
fn backtick_spans(text: &str) -> Vec<&str> {
    let mut spans = Vec::new();
    let mut remainder = text;

    while let Some((_, after_open)) = remainder.split_once('`') {
        let Some((span, after_close)) = after_open.split_once('`') else {
            break;
        };
        if !span.contains('\n') {
            spans.push(span);
        }
        remainder = after_close;
    }

    spans
}

/// Returns every `globex_*` product-family name the handoff proposes.
fn handoff_proposed_key_names() -> Vec<&'static str> {
    let mut names: Vec<&str> = backtick_spans(HANDOFF)
        .into_iter()
        .filter(|span| {
            span.starts_with("globex_")
                && span
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
        .collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// Flows hard-wrapped prose onto one line so a claim can straddle line breaks.
fn flowed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Capitalizes the first letter of a spelled-out count.
fn capitalized(word: &str) -> String {
    let mut chars = word.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

/// Asserts the handwritten ledger key list is the `MarketHoursKey` enum.
///
/// Guards `EXPECTED_MARKET_HOURS_KEY_NAMES` against the public enum.
/// `verification_ledger_has_every_market_hours_key_once_and_in_order` compares
/// the ledger to that array, and nothing compared the array to the enum: this
/// module did not so much as import `MarketHoursKey`. A key added to the enum,
/// to `named_profiles.rs`, to `unsupported_market_hours_keys.rs` and to the
/// golden file but forgotten in the array *and* in the ledger shipped green —
/// with no Basis, no `Reviewed on`, no owner link and no source set, which is
/// the one thing LAW-PRIMARY-SOURCES exists to prevent.
///
/// A red here means the enum and the ledger disagree: add the missing ledger
/// row and array entry, or remove the stale one. This *compares* two
/// independently maintained artifacts rather than generating either from the
/// other, exactly as `named_profiles.rs` already does with
/// `assert_eq!(MarketHoursKey::ALL, expected_keys)`, so it stays inside
/// `AGENTS.md`'s "never generate those independent fences from `ALL`".
#[test]
fn ledger_covers_every_market_hours_key_variant() {
    let from_enum = MarketHoursKey::ALL
        .iter()
        .map(|key| key.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        super::EXPECTED_MARKET_HOURS_KEY_NAMES.as_slice(),
        from_enum.as_slice(),
        "the handwritten ledger key list and MarketHoursKey::ALL disagree"
    );
}

/// Returns whether `quoted` is a name the unsupported-family register
/// *refuses*, as opposed to merely mentioning.
///
/// The register's own preamble says it "has three parts": the ambiguous name,
/// the rejections on evidence, and the blocked keys. Only a first-cell table
/// entry (the ambiguous and blocked tables) or a rejection-section heading
/// puts a name in one of those three. A bare `contains` over the whole file
/// also matches prose that says the opposite — the commodity-index BTIC
/// section's "They must **not** ride `globex_bloomberg_commodity_index`" would
/// answer for that name and let it be dropped from the plan unnoticed.
fn names_in_unsupported_register(quoted: &str) -> bool {
    UNSUPPORTED_FAMILIES.lines().any(|line| {
        if line.starts_with("###") {
            return line.contains(quoted);
        }
        line.starts_with("| ") && row_cells(line).first() == Some(&quoted)
    })
}

/// Asserts every key name the trade-type handoff proposes is accounted for.
///
/// Guards `docs/plans/2026-09-05-cme-trade-type-handoff.md` against the three
/// registers a proposed name may legitimately live in: the shipped
/// `MarketHoursKey` enum, the rejected/blocked register
/// `docs/schedules/unsupported-families.md`, or the live work list
/// `docs/plans/2026-09-12-cme-trade-type-keys.md`. The handoff proposes
/// thirty-eight names the crate does not yet ship; thirty-one of them are
/// answered by the work list alone. Presence there is textual and permanent —
/// the plan is a dated record and no later PR in the sequence prunes it — so
/// this fence cannot tell a key its PR shipped from one its PR silently
/// skipped. What it does guarantee is that no surveyed name becomes
/// unanswerable: removing a name from the work list or the register without
/// authoring or rejecting the key goes red, which is how a deliberate drop
/// (PR 12, gated on issue #72) is caught.
///
/// A red here means a `globex_*` name appears in the handoff and nowhere else.
/// **The correct response is to author or reject the key, never to edit the
/// handoff.** The handoff is a historical record that happens to be this work's
/// list; deleting a name from it to get green would destroy the only statement
/// that the family was ever surveyed.
#[test]
fn handoff_keys_are_registered_or_rejected() {
    let proposed = handoff_proposed_key_names();
    assert!(
        proposed.len() >= 40,
        "the handoff's proposed-key extraction collapsed: found only {}",
        proposed.len()
    );

    for name in proposed {
        let registered = name.parse::<MarketHoursKey>().is_ok();
        let quoted = format!("`{name}`");
        let rejected = names_in_unsupported_register(&quoted);
        let scheduled = TRADE_TYPE_PLAN.contains(&quoted);

        assert!(
            registered || rejected || scheduled,
            "the handoff proposes `{name}` and no register answers it: author the key, \
             reject it in unsupported-families.md, or schedule it in the trade-type plan \
             — never edit the handoff"
        );
    }
}

/// Asserts the register names every root of every rejected handoff row.
///
/// Guards the rejection sections of `docs/schedules/unsupported-families.md`
/// against the handoff rows they answer. A rejection that names five of six
/// roots reads as a decision and is a gap: the sixth root resolves nowhere and
/// nothing says why.
///
/// A red here means a root was added to one of the three rejected handoff rows
/// and the register did not follow, or a rejected row now names a key. Name the
/// root in the register, or move the row out of the rejection set deliberately.
#[test]
fn rejected_handoff_roots_are_named_in_the_register() {
    for anchor in REJECTED_HANDOFF_ROWS {
        let matching = HANDOFF
            .lines()
            .filter(|line| line.starts_with("| "))
            .filter(|line| {
                let cells = row_cells(line);
                cells.len() >= 2
                    && cells[0]
                        .split(',')
                        .next()
                        .is_some_and(|root| root == anchor)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            matching.len(),
            1,
            "expected exactly one handoff row anchored on {anchor}"
        );

        let cells = row_cells(matching[0]);
        assert!(
            cells[1].contains("UNMAPPED"),
            "{anchor}'s handoff row no longer reads UNMAPPED: {}",
            cells[1]
        );

        for root in cells[0].split(',').map(str::trim) {
            assert!(
                UNSUPPORTED_FAMILIES.contains(&format!("`{root}`")),
                "rejected root {root} is not named in the unsupported-family register"
            );
        }
    }
}

/// Asserts the `US-CME-GROUP` source-set prose counts derive from the ledger.
///
/// Guards the **Status** paragraph of `docs/schedules/sources.md`, which hand
/// writes how many fixed-current CME-family profiles there are and how many of
/// them are `Partial`. The only fences on that file were anchor resolution and
/// ledger reference; nothing tied either number to anything, and all thirteen
/// remaining PRs of the trade-type sequence change both.
///
/// A red here means a CME-family key row was added, removed, or reclassified
/// and the source registry's prose did not follow. Update the paragraph; the
/// failure message prints the string the ledger implies.
#[test]
fn cme_source_set_prose_counts_match_the_ledger() {
    let rows = market_hours_key_rows();
    let cme_rows = rows
        .iter()
        .copied()
        .filter(|row| row_cells(row)[2].contains("sources.md#us-cme-group"))
        .collect::<Vec<_>>();
    assert!(
        !cme_rows.is_empty(),
        "no ledger key row references the US-CME-GROUP source set"
    );
    let partial = cme_rows
        .iter()
        .filter(|row| row_cells(row)[3] == "Partial")
        .count();

    let total_words = number_words(cme_rows.len());
    let sources = flowed(SOURCES);
    for claim in [
        format!("All {total_words} fixed-current CME-family profiles"),
        format!(
            "{} of the {total_words} are Partial",
            capitalized(&number_words(partial))
        ),
    ] {
        assert!(
            sources.contains(&claim),
            "the US-CME-GROUP status prose drifted from the ledger: expected {claim:?}"
        );
    }
}

/// Asserts every executable-gap row is named where the executable count is.
///
/// Guards the two hand-written enumerations that follow the executable tally —
/// the README's "The executable N — …" sentence and the same list in
/// `docs/schedules/verification.md`. The tally itself is fenced; the
/// enumerations were not, so a PR could bump the count word and leave its own
/// key out of the sentence that count refers to. That is the defect class the
/// two previous key PRs fixed, one level down.
///
/// A red here means a row carries `Gap: executable` and is missing from one of
/// the two sentences, or a collective phrase's written-out count disagrees with
/// the ledger. Name the row, or fold it into a declared collective phrase in
/// `EXECUTABLE_COLLECTIVE_NAMES` and correct that phrase's count.
#[test]
fn every_executable_gap_row_is_named_in_the_prose() {
    let executable = VERIFICATION
        .lines()
        .filter(|line| line.starts_with("| `") && line.contains("Gap: executable"))
        .map(wire_name)
        .collect::<Vec<_>>();
    let words = number_words(executable.len());

    let readme = flowed(README);
    let ledger = flowed(VERIFICATION);
    let spans = [
        (
            "README",
            span_between(
                &readme,
                &format!("The executable {words} —"),
                "— are each served",
            ),
        ),
        (
            "verification.md",
            span_between(
                &ledger,
                &format!("The executable {words} are"),
                &format!("None of the {words} serves"),
            ),
        ),
    ];

    for (label, span) in spans {
        let Some(span) = span else {
            panic!("{label} lost the sentence that opens its executable enumeration");
        };
        assert_collective_counts(label, &span, &executable);
        for name in &executable {
            if span.contains(&format!("`{name}`")) {
                continue;
            }
            let collective = EXECUTABLE_COLLECTIVE_NAMES
                .iter()
                .find(|(prefix, _)| name.starts_with(prefix));
            let Some((_, phrase)) = collective else {
                panic!("{label} does not name the executable row `{name}`");
            };
            assert!(
                span.contains(phrase),
                "{label} names neither `{name}` nor its collective phrase {phrase:?}"
            );
        }
    }
}

/// Returns the text between two markers, or `None` when either is missing.
fn span_between(text: &str, start: &str, end: &str) -> Option<String> {
    let (_, rest) = text.split_once(start)?;
    let (span, _) = rest.split_once(end)?;
    Some(span.to_owned())
}

/// Asserts each declared collective phrase is live and correctly counted.
fn assert_collective_counts(label: &str, span: &str, executable: &[&str]) {
    for (prefix, phrase) in EXECUTABLE_COLLECTIVE_NAMES {
        let covered = executable
            .iter()
            .filter(|name| name.starts_with(prefix))
            .count();
        assert!(
            covered > 0,
            "collective phrase {phrase:?} covers no executable ledger row; remove it"
        );
        // A one-row entry is an alias for a single key's display name, never a
        // collective, so no count is written beside it.
        if covered == 1 || !span.contains(phrase) {
            continue;
        }
        let counted = format!("the {} {phrase}", number_words(covered));
        let uncounted = format!("the {phrase}");
        assert!(
            span.contains(&counted) || span.contains(&uncounted),
            "{label}'s count beside {phrase:?} drifted from the ledger: expected {counted:?} \
             or the uncounted form"
        );
    }
}

/// Asserts the golden-grid header's identity counts derive from the ledger.
///
/// Guards the module documentation of `tests/golden_grids.rs`, which states
/// how many exchanges and keys the golden file covers. The golden file is the
/// crate's main defence against a silent table edit, and the sentence that
/// tells a reader how much it covers was hand-written and unfenced.
///
/// A red here means an identity was added or removed and the golden header was
/// not updated. Fix the header, then regenerate the golden file and confirm no
/// other identity's rows moved.
#[test]
fn golden_header_identity_counts_match_the_ledger() {
    let claim = format!(
        "it covers all {} exchanges and all {} keys at once",
        super::exchange_rows().len(),
        market_hours_key_rows().len()
    );
    // The header is a hard-wrapped `//!` doc comment, so the claim straddles
    // both a line break and the comment marker that follows it.
    let header = GOLDEN_GRIDS
        .lines()
        .map(|line| line.trim_start().trim_start_matches("//!"))
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        flowed(&header).contains(&claim),
        "the golden-grid header's identity counts drifted from the ledger: expected {claim:?}"
    );
}
