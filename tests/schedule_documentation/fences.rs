// SPDX-License-Identifier: MIT-0

//! Fences over the verification ledger, the README and the schedule guides.

use super::{
    AUDIT, DATE_EXCEPTIONS, EXPECTED_MARKET_HOURS_KEY_NAMES, LEDGER_CELLS, README,
    UNSUPPORTED_FAMILIES, UPDATING, VERIFICATION, assert_row_shape, assert_source_links_resolve,
    basis_of, exchange_rows, is_partial, market_hours_key_rows, number_words, owner_targets,
    repository_cutoff, row_cells, today_utc, validated_source_link_count, wire_name,
};
use chrono::NaiveDate;
use exchange_hours::Exchange;
use std::path::Path;

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
    let today = today_utc();

    for row in exchange_rows() {
        assert_row_shape(row, cutoff, today, "unknown");
    }
}

#[test]
fn market_hours_key_rows_have_complete_review_metadata() {
    let cutoff = NaiveDate::parse_from_str(repository_cutoff(), "%Y-%m-%d")
        .expect("repository cutoff must be an ISO calendar date");
    let today = today_utc();

    for row in market_hours_key_rows() {
        assert_row_shape(row, cutoff, today, "always_open");
    }
}

#[test]
fn every_market_hours_key_owner_and_source_link_resolves() {
    let docs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/schedules");

    for row in market_hours_key_rows() {
        let cells = row_cells(row);
        assert_eq!(
            cells.len(),
            LEDGER_CELLS,
            "unexpected verification row shape: {row}"
        );

        for owner in owner_targets(cells[1]) {
            assert!(
                docs_dir.join(owner).is_file(),
                "MarketHoursKey owner link does not resolve: {owner}"
            );
        }
        assert_source_links_resolve(cells[2], row);
    }
}

#[test]
fn readme_and_review_dates_match_the_repository_cutoff() {
    let cutoff = repository_cutoff();
    let cutoff_date = NaiveDate::parse_from_str(cutoff, "%Y-%m-%d")
        .expect("repository cutoff must be an ISO calendar date");
    let today = today_utc();
    assert!(
        cutoff_date <= today,
        "LAW-UTC-DATES: the repository source-review cutoff {cutoff_date} is later than \
         today's UTC date {today}, so it is future-dated"
    );
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
        assert_eq!(
            cells.len(),
            LEDGER_CELLS,
            "unexpected verification row shape: {row}"
        );
        let reviewed = NaiveDate::parse_from_str(cells[8], "%Y-%m-%d")
            .expect("every non-synthetic Exchange identity must have an ISO review date");
        assert!(
            reviewed >= cutoff_date,
            "exchange review date predates repository cutoff: {row}"
        );
        assert!(
            reviewed <= today,
            "LAW-UTC-DATES: reviewed-on {reviewed} is later than today's UTC date \
             {today}, so the row is future-dated: {row}"
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

/// The README's test-inventory sentence states how many `Exchange` and
/// `MarketHoursKey` rows the documentation harness keeps in canonical order.
/// It was hand-written, so it lagged the public surface by two key additions
/// before anyone noticed. Derive it from the ledger instead, flowing the
/// README's hard-wrapped prose so the claim can straddle line breaks.
#[test]
fn readme_test_inventory_counts_match_the_ledger() {
    let exchange_rows = exchange_rows();
    let real_exchange_rows = exchange_rows
        .iter()
        .filter(|row| wire_name(row) != "unknown")
        .count();
    let key_rows = market_hours_key_rows();
    let real_key_rows = key_rows
        .iter()
        .filter(|row| wire_name(row) != "always_open")
        .count();

    let claim = format!(
        "keep all {} `Exchange` rows ({real_exchange_rows} non-synthetic plus `Unknown`) and {} `MarketHoursKey` rows ({real_key_rows} operator-derived plus `AlwaysOpen`) in canonical order",
        exchange_rows.len(),
        key_rows.len()
    );
    let flowed = README.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flowed.contains(&claim),
        "README test-inventory counts drifted from the ledger: expected {claim:?}"
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
        |rows: &[&str], basis: &str| rows.iter().filter(|row| basis_of(row) == basis).count();
    let partial_count = |rows: &[&str]| rows.iter().filter(|row| is_partial(row)).count();

    let primary = basis_count(&real_exchange_rows, "Primary");
    let partial = partial_count(&real_exchange_rows);
    let verified = primary + partial;
    let executable = basis_count(&exchange_rows, "Partial / executable");
    let order_entry = basis_count(&exchange_rows, "Partial / order-entry");
    let history_gap_rows = partial;
    let synthetic = basis_count(&exchange_rows, "Synthetic");
    let verified_keys = basis_count(&real_key_rows, "Primary") + partial_count(&real_key_rows);

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
        "**Date-aware queries answer or refuse explicitly — never a silent guess.**".to_string(),
        format!(
            "`{history_gap_rows} of {}` exchange rows still name a Basis gap in the ledger, each with its disclosed residual or external closer",
            real_exchange_rows.len()
        ),
        format!(
            "**Hours verified at the review date for each product family:** `{verified_keys} of {}`",
            real_key_rows.len()
        ),
    ];

    assert_key_basis_prose_matches_the_ledger(&real_key_rows);

    // The first three claims are the current README assurance block (the
    // 2026-10-05 sourced-or-verified-carried rework); they are checked against
    // the README only. The rest are also carried by the dated 2026-08-22 audit
    // and are asserted in both.
    let readme_flowed = README.split_whitespace().collect::<Vec<_>>().join(" ");
    let readme_block = &claims[..3];
    for claim in readme_block {
        assert!(
            readme_flowed.contains(claim),
            "README assurance count drifted: {claim}"
        );
    }
    let audit_claims = &claims[3..];
    for claim in audit_claims {
        assert!(
            README.contains(claim),
            "README assurance count drifted: {claim}"
        );
        assert!(AUDIT.contains(claim), "dated audit count drifted: {claim}");
    }

    let exchange_distribution = format!(
        "| {} `Exchange` identifiers | {primary} | {executable} | {order_entry} | {synthetic} |",
        exchange_rows.len()
    );
    let key_distribution = format!(
        "| {} `MarketHoursKey` values | {} | {} | {} | {} |",
        key_rows.len(),
        basis_count(&key_rows, "Primary"),
        basis_count(&key_rows, "Partial / executable"),
        basis_count(&key_rows, "Partial / order-entry"),
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

/// Asserts the README's holiday-coverage count derives from the ledger.
///
/// LAW-HOLIDAY-SCOPE makes a served identity owe a table, so the count moves
/// every time a family's table lands. The README states it in prose twice, and
/// the ledger's own Holidays column is already fenced against each identity's
/// `holiday_coverage()`, so deriving the prose from that column chains the
/// README to the shipped tables with nothing hand-maintained in between.
#[test]
fn readme_states_the_holiday_coverage_count_from_the_ledger() {
    let rows = exchange_rows()
        .into_iter()
        .chain(market_hours_key_rows())
        .collect::<Vec<_>>();
    let with_a_table = rows
        .iter()
        .filter(|row| row_cells(row)[7] != "\u{2014}")
        .count();
    assert!(
        with_a_table > 0,
        "at least one identity must ship a holiday table"
    );

    // README prose is hard-wrapped, so the claim straddles line breaks.
    let readme = README.split_whitespace().collect::<Vec<_>>().join(" ");
    let claim = format!("{with_a_table} of the {} ledger rows", rows.len());
    assert_eq!(
        readme.matches(&claim).count(),
        2,
        "README holiday-coverage count drifted from the ledger: expected {claim:?} twice"
    );
}

/// Asserts the README's service-tier split derives from the ledger.
///
/// LAW-SERVICE-TIERS decides what a row owes, and the README states the split
/// in prose. Nothing derived it, so the first identity a consumer reaches — or
/// stops reaching — would leave the sentence stale while the tests stayed
/// green, exactly as the basis and gap-kind counts did before their fences.
#[test]
fn readme_states_the_service_tier_split_from_the_ledger() {
    let rows = exchange_rows()
        .into_iter()
        .chain(market_hours_key_rows())
        .collect::<Vec<_>>();
    let served = rows
        .iter()
        .filter(|row| row_cells(row)[5] == "served")
        .count();
    let dormant = rows.len().saturating_sub(served);

    // README prose is hard-wrapped, so these claims straddle line breaks.
    let readme = README.split_whitespace().collect::<Vec<_>>().join(" ");
    for claim in [
        format!("{served} of the {} rows are `served`", rows.len()),
        format!("and {dormant} are `dormant`"),
    ] {
        assert!(
            readme.contains(&claim),
            "README service-tier split drifted from the ledger: expected {claim:?}"
        );
    }
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
        for owner in owner_targets(cells[1]) {
            assert!(
                docs_dir.join(owner).is_file(),
                "exchange owner link does not resolve: {owner}"
            );
        }
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
        |rows: &[&str], basis: &str| rows.iter().filter(|row| basis_of(row) == basis).count();
    // The prose Primary/Partial split for keys is written in words, and drifted
    // silently twice: README.md once said "Four key rows are Primary" while the
    // ledger held five, and the headline bullet said "11 operator-derived" long
    // after the count reached 24. Derive both from the ledger instead.
    // Both counts are spelled out through `number_words`, which is valid below
    // 100 and therefore covers the whole CME trade-type sequence. The local
    // word list this replaced stopped at twenty and silently fell back to
    // digits, which is how the README came to read "Six key rows are
    // **Primary** and 24 are **Partial**" in one sentence.
    let key_primary = basis_count(real_key_rows, "Primary");
    let key_partial = real_key_rows.iter().filter(|row| is_partial(row)).count();
    let capitalized = |word: String| -> String {
        let mut chars = word.chars();
        chars.next().map_or_else(String::new, |first| {
            first.to_uppercase().collect::<String>() + chars.as_str()
        })
    };
    let key_split = format!(
        "{} key rows are **Primary** and {} are **Partial**",
        capitalized(number_words(key_primary)),
        number_words(key_partial)
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

/// Asserts the gap-kind split is quoted consistently wherever it appears.
///
/// Three narrative claims restate the `Gap: order-entry` / `Gap: executable`
/// tally in prose: the README's split sentence, the ledger's own summary, and
/// the audit's spelled-out Partial-key count. None of them was derived from the
/// ledger, so every new key silently staled all three while the tests stayed
/// green -- it happened on two consecutive product-family additions before this
/// fence existed. Derive them here instead, along with the executable tally the
/// README and the ledger spell out beside the rows they name.
#[test]
fn the_gap_kind_split_is_quoted_consistently_everywhere() {
    let rows = exchange_rows()
        .into_iter()
        .chain(market_hours_key_rows())
        .collect::<Vec<_>>();
    let count = |basis: &str| rows.iter().filter(|row| basis_of(row) == basis).count();
    let order_entry = count("Partial / order-entry");
    let executable = count("Partial / executable");
    let partial_rows = order_entry + executable;

    let flowed = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let readme = flowed(README);
    let ledger = flowed(VERIFICATION);
    let audit = flowed(AUDIT);

    let readme_claim = format!(
        "the split is **{order_entry} order-entry to {executable} executable** across the {partial_rows} rows in the ledger."
    );
    assert!(
        readme.contains(&readme_claim),
        "README gap-kind split drifted from the ledger: expected {readme_claim:?}"
    );

    let ledger_claim = format!("Of the {partial_rows}, **{order_entry} are order-entry**");
    assert!(
        ledger.contains(&ledger_claim),
        "the ledger's own summary drifted from its rows: expected {ledger_claim:?}"
    );

    let executable_claim = format!("**{executable} are executable**");
    assert!(
        ledger.contains(&executable_claim),
        "the ledger's executable count drifted from its rows: expected {executable_claim:?}"
    );

    // The same tally is spelled out beside the rows it names, where the digit
    // claims above cannot see it.
    let executable_words = number_words(executable);
    for (text, claim) in [
        (&readme, format!("The executable {executable_words} —")),
        (&ledger, format!("The executable {executable_words} are")),
        (&ledger, format!("None of the {executable_words} serves")),
    ] {
        assert!(
            text.contains(&claim),
            "a spelled-out executable count drifted from the ledger: expected {claim:?}"
        );
    }

    // The audit states its Partial product-family key count in words beside a
    // table that states it in digits; they drifted apart once already.
    let key_rows = market_hours_key_rows();
    let real_keys: Vec<&str> = key_rows
        .iter()
        .copied()
        .filter(|row| basis_of(row) != "Synthetic")
        .collect();
    let partial_keys = real_keys.iter().filter(|row| is_partial(row)).count();
    // This count used to be spelled from a local list that stopped at
    // twenty-five and panicked past it. The ledger holds twenty-four Partial
    // keys today and the CME trade-type sequence adds thirty-two rows, so the
    // list would have panicked on the first of them, with a message that named
    // the fix but not the cause. `number_words` is valid below 100.
    let spelled = number_words(partial_keys);
    let audit_claim = format!("{spelled} product-family keys are **Partial**");
    assert!(
        audit.to_lowercase().contains(&audit_claim.to_lowercase()),
        "the audit's spelled-out Partial key count drifted from the ledger: expected {audit_claim:?}"
    );
}

/// Asserts every restatement of the `Exchange` basis counts in running prose.
///
/// `readme_and_audit_quantify_assurance_from_the_ledger` pins the headline
/// form of each count, but the README restates both totals in running prose
/// and the audit spells the basis counts out. Adding two venues on 2026-09-09
/// updated every headline yet left four restatements at 26, one at 93, and the
/// audit's "Twenty-six" behind while the tests stayed green. Derive them here
/// instead.
#[test]
fn assurance_prose_restates_exchange_counts_from_the_ledger() {
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
        |rows: &[&str], basis: &str| rows.iter().filter(|row| basis_of(row) == basis).count();
    let partial_count = |rows: &[&str]| rows.iter().filter(|row| is_partial(row)).count();
    let real = real_exchange_rows.len();
    let primary = basis_count(&real_exchange_rows, "Primary");
    let partial = partial_count(&real_exchange_rows);

    // README prose is hard-wrapped, so these claims straddle line breaks.
    let flowed = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let readme = flowed(README);
    for claim in [
        "**Date-aware queries answer or refuse explicitly — never a silent guess.**"
        .to_string(),
        "**sourced** (dated primary sources run the whole way back to January 2010)".to_string(),
        "**verified carried**".to_string(),
        "the crate serves the window that holds under every sourced state".to_string(),
        "the crate refuses with an explicit typed error rather than claiming hours it cannot support".to_string(),
        "Where a residual uncertainty survives, it is **instant-level and disclosed**".to_string(),
        format!(
            "Per-family holiday and early-close tables ship for {} of the {} ledger rows",
            exchange_rows.iter().chain(&key_rows).filter(|row| row_cells(row)[7] != "—").count(),
            exchange_rows.len() + key_rows.len()
        ),
        format!("All {real} current profiles are primary-supported"),
        format!("The {primary} **Primary** rows have no known modeled-history gap"),
        format!("{partial} **Partial** rows name"),
        format!("is not one of the {real} source-backed identities"),
    ] {
        assert!(
            readme.contains(&claim),
            "README assurance prose drifted from the ledger: expected {claim:?}"
        );
    }

    // The audit spells both counts out, capitalized where a sentence opens.
    let audit = flowed(AUDIT).to_lowercase();
    for (exchanges, keys, basis) in [
        (primary, basis_count(&real_key_rows, "Primary"), "primary"),
        (partial, partial_count(&real_key_rows), "partial"),
    ] {
        let claim = format!(
            "{} exchange rows and {} product-family keys are **{basis}**",
            number_words(exchanges),
            number_words(keys)
        );
        assert!(
            audit.contains(&claim),
            "the audit's spelled-out exchange count drifted from the ledger: expected {claim:?}"
        );
    }
}
