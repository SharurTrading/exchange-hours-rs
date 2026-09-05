// SPDX-License-Identifier: MIT-0

//! Documentation fences for the schedule-review ledger.

mod databento;
mod source_registry;

use chrono::NaiveDate;
use exchange_hours::Exchange;
use std::path::Path;

const README: &str = include_str!("../../README.md");
const VERIFICATION: &str = include_str!("../../docs/schedules/verification.md");
const SOURCES: &str = include_str!("../../docs/schedules/sources.md");
const UPDATING: &str = include_str!("../../docs/schedules/updating.md");
const AUDIT: &str = include_str!("../../docs/schedules/audit-2026-08-22.md");
const DATE_EXCEPTIONS: &str = include_str!("../../docs/schedules/date-exceptions.md");
const UNSUPPORTED_FAMILIES: &str = include_str!("../../docs/schedules/unsupported-families.md");
const DATABENTO_VENUES: &str = include_str!("../../docs/schedules/databento-venues.md");

const EXPECTED_MARKET_HOURS_KEY_NAMES: [&str; 27] = [
    "globex_equity_index",
    "globex_energy",
    "globex_grains",
    "globex_mini_grains",
    "globex_fx",
    "globex_interest_rates",
    "globex_livestock",
    "globex_cryptocurrency",
    "cfe_vix",
    "eurex",
    "ice_us",
    "ice_us_sugar",
    "ice_us_coffee",
    "ice_us_cocoa",
    "ice_us_cotton",
    "ice_us_orange_juice",
    "ice_us_dollar_index",
    "globex_nikkei_225_dollar",
    "eurex_fixed_income",
    "sgx_equity_index_japan",
    "sgx_equity_index_china",
    "sgx_equity_index_singapore",
    "sgx_equity_index_taiwan",
    "sgx_equity_index_ntr_usd",
    "globex_rough_rice",
    "sgx",
    "always_open",
];

const VALID_BASES: [&str; 6] = [
    "Primary",
    "Partial",
    "Secondary",
    "Pragmatic",
    "Known issue",
    "Synthetic",
];

fn repository_cutoff() -> &'static str {
    const PREFIX: &str = "**Repository source-review cutoff:** `";
    let line = VERIFICATION
        .lines()
        .find(|line| line.starts_with(PREFIX))
        .expect("verification ledger must declare its repository cutoff");
    line.strip_prefix(PREFIX)
        .and_then(|value| value.strip_suffix('`'))
        .expect("repository cutoff must be a single backtick-delimited ISO date")
}

fn exchange_rows() -> Vec<&'static str> {
    let (_, exchanges) = VERIFICATION
        .split_once("## Exchanges")
        .expect("verification ledger must have an Exchanges section");
    let (exchanges, _) = exchanges
        .split_once("## `MarketHoursKey` profiles")
        .expect("exchange table must end before MarketHoursKey profiles");

    exchanges
        .lines()
        .filter(|line| line.starts_with("| `"))
        .collect()
}

fn market_hours_key_rows() -> Vec<&'static str> {
    let (_, profiles) = VERIFICATION
        .split_once("## `MarketHoursKey` profiles")
        .expect("verification ledger must have a MarketHoursKey profiles section");

    profiles
        .lines()
        .filter(|line| line.starts_with("| `"))
        .collect()
}

fn row_cells(row: &str) -> Vec<&str> {
    row.trim_matches('|').split('|').map(str::trim).collect()
}

fn wire_name(row: &str) -> &str {
    row_cells(row)[0]
        .strip_prefix('`')
        .and_then(|name| name.strip_suffix('`'))
        .expect("wire names must be backtick-delimited")
}

fn owner_target(owner: &str) -> &str {
    owner
        .split_once("](")
        .and_then(|(_, target)| target.strip_suffix(')'))
        .expect("owner cell must contain one Markdown file link")
}

fn validated_source_link_count(text: &str) -> u16 {
    let mut remainder = text;
    let mut links = 0_u16;

    while let Some((_, after_prefix)) = remainder.split_once("(sources.md#") {
        let (anchor, after_link) = after_prefix
            .split_once(')')
            .expect("source-set link must close its destination");
        let declaration = format!("<a id=\"{anchor}\"></a>");
        assert_eq!(
            SOURCES.matches(&declaration).count(),
            1,
            "source-set link must have exactly one registry anchor: {anchor}"
        );
        links = links.saturating_add(1);
        remainder = after_link;
    }

    links
}

fn assert_source_links_resolve(source_cell: &str, row: &str) {
    assert!(
        validated_source_link_count(source_cell) > 0,
        "ledger row must reference a source set: {row}"
    );
}

#[test]
fn verification_ledger_has_every_exchange_once_and_in_order() {
    let rows = exchange_rows();
    let documented = rows.iter().map(|row| wire_name(row));
    let expected = Exchange::ALL.iter().map(|exchange| exchange.as_str());

    assert_eq!(documented.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
}

#[test]
fn verification_ledger_has_every_market_hours_key_once_and_in_order() {
    let documented = market_hours_key_rows()
        .into_iter()
        .map(wire_name)
        .collect::<Vec<_>>();

    assert_eq!(documented, EXPECTED_MARKET_HOURS_KEY_NAMES);
}

#[test]
fn market_hours_key_selection_contract_is_explicit() {
    for claim in [
        "stable persisted wire identity",
        "does **not** map symbols, roots, product codes, or MICs",
        "Those defaults are the wrong choice for any product outside the named family.",
        "the ambiguous name\n`sgx_equity_index` stays rejected",
    ] {
        assert!(
            README.contains(claim),
            "README lost a product-family selection contract: {claim}"
        );
    }

    assert!(
        README.contains("docs/schedules/unsupported-families.md"),
        "README must link the explicit unsupported-family register"
    );
    for sourced_name in [
        "sgx_equity_index_japan",
        "sgx_equity_index_china",
        "sgx_equity_index_singapore",
        "sgx_equity_index_taiwan",
        "sgx_equity_index_ntr_usd",
    ] {
        assert!(
            UNSUPPORTED_FAMILIES.contains(sourced_name),
            "register must name the specific grid that replaces the ambiguous key: \
             {sourced_name}"
        );
    }
    assert!(
        UNSUPPORTED_FAMILIES.contains("`sgx_equity_index` | SGX equity-index products do not"),
        "register must state why the ambiguous name is refused"
    );
}

#[test]
fn exchange_rows_have_complete_review_metadata() {
    let cutoff = NaiveDate::parse_from_str(repository_cutoff(), "%Y-%m-%d")
        .expect("repository cutoff must be an ISO calendar date");

    for row in exchange_rows() {
        let cells = row_cells(row);
        assert_eq!(cells.len(), 6, "unexpected verification row shape: {row}");

        let name = wire_name(row);
        let basis = cells[3];
        let reviewed = cells[4];
        assert!(
            VALID_BASES.contains(&basis),
            "unrecognized verification basis: {row}"
        );

        if name == "unknown" {
            assert_eq!(basis, "Synthetic", "unknown must remain synthetic");
            assert_eq!(reviewed, "—", "synthetic profiles have no review date");
        } else {
            assert_ne!(
                basis, "Synthetic",
                "non-synthetic Exchange identity cannot be synthetic: {row}"
            );
            let reviewed = NaiveDate::parse_from_str(reviewed, "%Y-%m-%d")
                .expect("every non-synthetic Exchange identity must have an ISO review date");
            assert!(
                reviewed >= cutoff,
                "exchange review date predates repository cutoff: {row}"
            );
        }
    }
}

#[test]
fn market_hours_key_rows_have_complete_review_metadata() {
    let cutoff = NaiveDate::parse_from_str(repository_cutoff(), "%Y-%m-%d")
        .expect("repository cutoff must be an ISO calendar date");

    for row in market_hours_key_rows() {
        let cells = row_cells(row);
        assert_eq!(cells.len(), 6, "unexpected verification row shape: {row}");

        let name = wire_name(row);
        let basis = cells[3];
        let reviewed = cells[4];
        assert!(
            VALID_BASES.contains(&basis),
            "unrecognized verification basis: {row}"
        );

        if name == "always_open" {
            assert_eq!(basis, "Synthetic", "always_open must remain synthetic");
            assert_eq!(reviewed, "—", "synthetic profiles have no review date");
        } else {
            assert_ne!(
                basis, "Synthetic",
                "real profile cannot be synthetic: {row}"
            );
            let reviewed = NaiveDate::parse_from_str(reviewed, "%Y-%m-%d")
                .expect("every real MarketHoursKey profile must have an ISO review date");
            assert!(
                reviewed >= cutoff,
                "MarketHoursKey review date predates repository cutoff: {row}"
            );
        }
    }
}

#[test]
fn every_market_hours_key_owner_and_source_link_resolves() {
    let docs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/schedules");

    for row in market_hours_key_rows() {
        let cells = row_cells(row);
        assert_eq!(cells.len(), 6, "unexpected verification row shape: {row}");

        let owner = owner_target(cells[1]);
        assert!(
            docs_dir.join(owner).is_file(),
            "MarketHoursKey owner link does not resolve: {owner}"
        );
        assert_source_links_resolve(cells[2], row);
    }
}

#[test]
fn readme_and_review_dates_match_the_repository_cutoff() {
    let cutoff = repository_cutoff();
    let cutoff_date = NaiveDate::parse_from_str(cutoff, "%Y-%m-%d")
        .expect("repository cutoff must be an ISO calendar date");
    let readme_claim = format!("**Repository-wide review completed:** `{cutoff}`");

    assert!(
        README.contains(&readme_claim),
        "README freshness claim must match the verification ledger"
    );

    let mut minimum_reviewed: Option<NaiveDate> = None;
    for row in exchange_rows()
        .into_iter()
        .filter(|row| wire_name(row) != "unknown")
    {
        let cells = row_cells(row);
        assert_eq!(cells.len(), 6, "unexpected verification row shape: {row}");
        let reviewed = NaiveDate::parse_from_str(cells[4], "%Y-%m-%d")
            .expect("every non-synthetic Exchange identity must have an ISO review date");
        assert!(
            reviewed >= cutoff_date,
            "exchange review date predates repository cutoff: {row}"
        );
        minimum_reviewed =
            Some(minimum_reviewed.map_or(reviewed, |earliest| earliest.min(reviewed)));
    }
    assert_eq!(
        minimum_reviewed,
        Some(cutoff_date),
        "repository cutoff must equal the oldest non-synthetic Exchange review date"
    );
}

#[test]
fn readme_and_audit_quantify_assurance_from_the_ledger() {
    let exchange_rows = exchange_rows();
    let real_exchange_rows = exchange_rows
        .iter()
        .copied()
        .filter(|row| wire_name(row) != "unknown")
        .collect::<Vec<_>>();
    let key_rows = market_hours_key_rows();
    let real_key_rows = key_rows
        .iter()
        .copied()
        .filter(|row| wire_name(row) != "always_open")
        .collect::<Vec<_>>();

    let basis_count =
        |rows: &[&str], basis: &str| rows.iter().filter(|row| row_cells(row)[3] == basis).count();

    let primary = basis_count(&real_exchange_rows, "Primary");
    let partial = basis_count(&real_exchange_rows, "Partial");
    let verified = primary + partial;
    let secondary = basis_count(&exchange_rows, "Secondary");
    let pragmatic = basis_count(&exchange_rows, "Pragmatic");
    let known_issues = basis_count(&real_exchange_rows, "Known issue");
    let history_gap_rows = partial
        + basis_count(&real_exchange_rows, "Secondary")
        + basis_count(&real_exchange_rows, "Pragmatic")
        + known_issues;
    let synthetic = basis_count(&exchange_rows, "Synthetic");
    let verified_keys =
        basis_count(&real_key_rows, "Primary") + basis_count(&real_key_rows, "Partial");

    let readme_identity_claims = [
        format!(
            "**{} source-backed market identities**",
            real_exchange_rows.len()
        ),
        format!("({} `Exchange` variants total)", exchange_rows.len()),
        format!(
            "{} variants—{} operator-derived product-family keys",
            key_rows.len(),
            real_key_rows.len()
        ),
    ];
    for claim in readme_identity_claims {
        assert!(
            README.contains(&claim),
            "README identity count drifted: {claim}"
        );
    }

    let claims = [
        format!(
            "**Hours verified against the exchange at the review date:** `{verified} of {}`",
            real_exchange_rows.len()
        ),
        format!(
            "**Full dated history back to January 2010:** `{primary} of {}`",
            real_exchange_rows.len()
        ),
        format!(
            "**History complete except for one named gap:** `{history_gap_rows} of {}`",
            real_exchange_rows.len()
        ),
        format!(
            "**Hours verified at the review date for each product family:** `{verified_keys} of {}`",
            real_key_rows.len()
        ),
    ];

    assert_key_basis_prose_matches_the_ledger(&real_key_rows);

    for claim in claims {
        assert!(
            README.contains(&claim),
            "README assurance count drifted: {claim}"
        );
        assert!(AUDIT.contains(&claim), "dated audit count drifted: {claim}");
    }

    let exchange_distribution = format!(
        "| {} `Exchange` identifiers | {primary} | {partial} | {secondary} | {pragmatic} | \
         {known_issues} | {synthetic} |",
        exchange_rows.len()
    );
    let key_distribution = format!(
        "| {} `MarketHoursKey` values | {} | {} | {} | {} | {} | {} |",
        key_rows.len(),
        basis_count(&key_rows, "Primary"),
        basis_count(&key_rows, "Partial"),
        basis_count(&key_rows, "Secondary"),
        basis_count(&key_rows, "Pragmatic"),
        basis_count(&key_rows, "Known issue"),
        basis_count(&key_rows, "Synthetic")
    );
    assert!(
        AUDIT.contains(&exchange_distribution),
        "dated audit exchange distribution drifted: {exchange_distribution}"
    );
    assert!(
        AUDIT.contains(&key_distribution),
        "dated audit key distribution drifted: {key_distribution}"
    );

    assert!(
        README.contains("docs/schedules/audit-2026-08-22.md"),
        "README must link the dated audit report"
    );
}

#[test]
fn every_ledger_source_link_has_a_registry_anchor() {
    assert!(
        validated_source_link_count(VERIFICATION) > 0,
        "verification ledger must reference source sets"
    );
}

#[test]
fn every_exchange_owner_link_resolves() {
    let docs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/schedules");

    for row in exchange_rows() {
        let cells = row_cells(row);
        let owner = owner_target(cells[1]);
        assert!(
            docs_dir.join(owner).is_file(),
            "exchange owner link does not resolve: {owner}"
        );
    }
}

#[test]
fn conditional_future_revisions_remain_unencoded_pending_confirmation() {
    assert!(
        UPDATING.contains("keep it\nout of runtime selectors"),
        "conditional-date policy must prohibit provisional runtime selectors"
    );
    let (_, pending) = UPDATING
        .split_once("### Pending effective-date confirmations")
        .expect("updating guide must retain a pending-confirmations section");
    let (pending, _) = pending
        .split_once("\n## ")
        .expect("pending confirmations must end before the next guide section");

    for revision in [
        "**Nasdaq — 2026-12-06:**",
        "**Cboe EDGX — 2026-12-06:**",
        "**FINRA TRFs — 2026-12-06:**",
        "**NYSE Arca — target 2026-12-06:**",
        "**MEMX — target 2026-12-06:**",
        "**24X overnight phase — no unconditional day:**",
        "**MX2 Options — target 2026-09-14:**",
        "**IEX Options — target 2026-10-02:**",
        "**Green Impact Exchange — no unconditional day:**",
        "**Nasdaq MRX Options 3C — awaiting operative alert:**",
    ] {
        assert!(
            pending.contains(revision),
            "conditional future revision is missing from the update checklist: {revision}"
        );
    }
}

#[test]
fn date_exception_contract_distinguishes_boundaries_coverage_and_finality() {
    for claim in [
        "`StaticDayPolicy` gives callers an allocation-free, validated table format",
        "`ReplaceSessions`",
        "`OutOfCoverage`",
        "announced/final distinction",
        "publicly available",
    ] {
        assert!(
            DATE_EXCEPTIONS.contains(claim),
            "date-exception contract lost a required distinction: {claim}"
        );
    }

    assert!(
        README.contains("docs/schedules/date-exceptions.md"),
        "README must link the date-exception contract"
    );
    assert!(
        UPDATING.contains("[date-exceptions.md](date-exceptions.md)"),
        "schedule-update guide must route special dates to the exception contract"
    );
}

/// Asserts the README's spelled-out Primary/Partial key split and headline
/// product-family count both derive from the ledger.
///
/// Split out of `readme_and_audit_quantify_assurance_from_the_ledger` to keep
/// that test inside the crate's 100-line function limit.
fn assert_key_basis_prose_matches_the_ledger(real_key_rows: &[&str]) {
    let basis_count =
        |rows: &[&str], basis: &str| rows.iter().filter(|row| row_cells(row)[3] == basis).count();
    // The prose Primary/Partial split for keys is written in words, and drifted
    // silently twice: README.md once said "Four key rows are Primary" while the
    // ledger held five, and the headline bullet said "11 operator-derived" long
    // after the count reached 24. Derive both from the ledger instead.
    let key_primary = basis_count(real_key_rows, "Primary");
    let key_partial = basis_count(real_key_rows, "Partial");
    let spelled = |n: usize| -> String {
        const WORDS: [&str; 21] = [
            "Zero",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
            "Twenty",
        ];
        WORDS
            .get(n)
            .map_or_else(|| n.to_string(), ToString::to_string)
    };
    let key_split = format!(
        "{} key rows are **Primary** and {} are **Partial**",
        spelled(key_primary),
        spelled(key_partial).to_lowercase()
    );
    // README prose is hard-wrapped, so these claims straddle line breaks.
    let flowed = README.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flowed.contains(&key_split),
        "README key basis split drifted from the ledger: expected {key_split:?}"
    );
    assert!(
        flowed.contains(&format!(
            "all {} operator-derived `MarketHoursKey`",
            real_key_rows.len()
        )),
        "README headline product-family count drifted from the ledger"
    );
}
