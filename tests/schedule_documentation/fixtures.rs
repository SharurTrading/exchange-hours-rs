// SPDX-License-Identifier: MIT-0

//! The ledger, README and guide texts every documentation fence reads, and
//! the row-parsing helpers shared by the fence suites.

use chrono::NaiveDate;
use exchange_hours::{
    Exchange, MarketHoursKey, calendar_for_exchange, calendar_for_market_hours_key,
};
use std::path::Path;

pub(super) const README: &str = include_str!("../../README.md");
pub(super) const VERIFICATION: &str = include_str!("../../docs/schedules/verification.md");
pub(super) const SOURCES: &str = include_str!("../../docs/schedules/sources.md");
pub(super) const UPDATING: &str = include_str!("../../docs/schedules/updating.md");
pub(super) const AUDIT: &str = include_str!("../../docs/schedules/audit-2026-08-22.md");
pub(super) const DATE_EXCEPTIONS: &str = include_str!("../../docs/schedules/date-exceptions.md");
pub(super) const UNSUPPORTED_FAMILIES: &str =
    include_str!("../../docs/schedules/unsupported-families.md");
pub(super) const DATABENTO_VENUES: &str = include_str!("../../docs/schedules/databento-venues.md");

pub(super) const EXPECTED_MARKET_HOURS_KEY_NAMES: [&str; 52] = [
    "globex_equity_index",
    "globex_energy",
    "globex_silver_100oz",
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
    "eurex_smi",
    "euronext_index_futures",
    "sgx_equity_index_japan",
    "sgx_equity_index_china",
    "sgx_equity_index_singapore",
    "sgx_equity_index_taiwan",
    "sgx_equity_index_ntr_usd",
    "globex_rough_rice",
    "globex_weather",
    "globex_spot_quoted",
    "globex_event_contracts",
    "globex_event_contracts_btc",
    "globex_gold_tas",
    "globex_silver_tas",
    "globex_copper_tas",
    "globex_platinum_tas",
    "globex_palladium_tas",
    "oanda_fx",
    "oanda_try_fx",
    "oanda_metals",
    "oanda_us_index_cfd",
    "oanda_us_bond_cfd",
    "oanda_us_commodity_cfd",
    "oanda_brent",
    "oanda_china50_cfd",
    "oanda_singapore30_cfd",
    "asx_spi_200",
    "bme_ibex",
    "hkex_index_futures",
    "ice_europe_gilts",
    "sgx",
    "always_open",
];

/// The closed basis vocabulary (LAW-EVIDENCE-FILES). `Secondary`, `Pragmatic`
/// and `Known issue` were retired on 2026-09-12 (UTC); a row that would need
/// one of them is a defect to fix, not a label to restore.
const VALID_BASES: [&str; 4] = [
    "Primary",
    "Partial / executable",
    "Partial / order-entry",
    "Synthetic",
];

const VALID_EVIDENCE_TIERS: [&str; 4] = ["T1", "T2", "T3", "T4"];

const VALID_SERVICE_TIERS: [&str; 2] = ["served", "dormant"];

const VALID_CADENCES: [&str; 3] = ["monthly", "quarterly", "on demand"];

/// Cells in one ledger row: identity, owner, source sets, basis, evidence
/// tier, service, horizon, holiday coverage, reviewed-on, cadence, basis note,
/// evidence link.
pub(super) const LEDGER_CELLS: usize = 12;

/// LAW-EVIDENCE-FILES caps the basis note at three sentences; everything else
/// belongs in the row's evidence file.
const MAX_BASIS_SENTENCES: usize = 3;

/// Abbreviations whose period is not a sentence terminator. Without these the
/// count trips on ICE's own product names ("ICE Futures U.S. Sugar No. 11").
const SENTENCE_ABBREVIATIONS: [&str; 7] = ["U.S.", "No.", "Nos.", "St.", "Inc.", "Ltd.", "Co."];

/// Evidence files live beside the ledger, one per row (LAW-EVIDENCE-FILES).
pub(super) const EVIDENCE_DIR: &str = "docs/evidence";

/// Today's UTC calendar date, for the "no record is future-dated" bound.
///
/// LAW-UTC-DATES says a recorded date later than the current UTC date is
/// future-dated and wrong, so the bound needs the real date. LAW-DETERMINISM
/// binds *library* code, and `chrono` is built without its `clock` feature so
/// `Utc::now` does not exist to call; tests are integration tests and may read
/// a clock, so this reads the Unix timestamp and converts it with
/// `from_timestamp`, which the feature gate does not remove.
pub(super) fn today_utc() -> NaiveDate {
    #[expect(
        clippy::disallowed_methods,
        reason = "LAW-DETERMINISM binds library code; this fence is a test and must \
                  compare the ledger's own recorded dates against the real UTC date"
    )]
    let since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the system clock must be at or after the Unix epoch");
    let seconds =
        i64::try_from(since_epoch.as_secs()).expect("the Unix timestamp must fit in an i64");
    chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, 0)
        .expect("the Unix timestamp must name a valid UTC instant")
        .date_naive()
}

pub(super) fn repository_cutoff() -> &'static str {
    const PREFIX: &str = "**Repository source-review cutoff:** `";
    let line = VERIFICATION
        .lines()
        .find(|line| line.starts_with(PREFIX))
        .expect("verification ledger must declare its repository cutoff");
    line.strip_prefix(PREFIX)
        .and_then(|value| value.strip_suffix('`'))
        .expect("repository cutoff must be a single backtick-delimited ISO date")
}

pub(super) fn exchange_rows() -> Vec<&'static str> {
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

pub(super) fn market_hours_key_rows() -> Vec<&'static str> {
    let (_, profiles) = VERIFICATION
        .split_once("## `MarketHoursKey` profiles")
        .expect("verification ledger must have a MarketHoursKey profiles section");

    profiles
        .lines()
        .filter(|line| line.starts_with("| `"))
        .collect()
}

pub(super) fn row_cells(row: &str) -> Vec<&str> {
    row.trim_matches('|').split('|').map(str::trim).collect()
}

pub(super) fn wire_name(row: &str) -> &str {
    row_cells(row)[0]
        .strip_prefix('`')
        .and_then(|name| name.strip_suffix('`'))
        .expect("wire names must be backtick-delimited")
}

/// Returns every module link destination in an owner cell.
///
/// The cell is `<br>`-separated because 23 identities keep their `revisions!`
/// block in a sibling of the module that holds their profiles; both files are
/// named, timeline module last, so a reader lands on the timeline the row's
/// history actually comes from.
pub(super) fn owner_targets(owner: &str) -> Vec<&str> {
    let targets = owner
        .split("<br>")
        .map(|link| {
            link.split_once("](")
                .and_then(|(_, target)| target.strip_suffix(')'))
                .expect("owner cell must contain Markdown file links")
        })
        .collect::<Vec<_>>();
    assert!(
        !targets.is_empty(),
        "owner cell must name at least one module: {owner}"
    );
    targets
}

/// The row's Basis cell: one of `VALID_BASES`.
pub(super) fn basis_of(row: &str) -> &str {
    row_cells(row)[3]
}

/// Whether the row names a gap, in either window kind.
pub(super) fn is_partial(row: &str) -> bool {
    basis_of(row).starts_with("Partial")
}

/// Returns the evidence file a row links, asserting the cell's exact form.
pub(super) fn evidence_target(row: &str) -> &str {
    let cell = row_cells(row)[LEDGER_CELLS - 1];
    let name = cell
        .strip_prefix("[evidence](../evidence/")
        .and_then(|rest| rest.strip_suffix(')'))
        .expect("evidence cell must be exactly [evidence](../evidence/<name>.md)");
    assert!(
        Path::new(name)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
            && !name.contains('/'),
        "evidence link must name one file in docs/evidence: {cell}"
    );
    name
}

/// Returns the ledger's Holidays cell for `wire_name`, derived from the table
/// that identity actually ships.
///
/// This is the fence that makes the column a record rather than a claim: a
/// family whose table lands, moves its coverage window, or is withdrawn fails
/// the ledger until the cell is corrected, and no cell can name a window the
/// crate does not answer over.
fn shipped_holiday_window(wire_name: &str) -> String {
    let coverage = wire_name
        .parse::<Exchange>()
        .ok()
        .and_then(|exchange| calendar_for_exchange(exchange).holiday_coverage())
        .or_else(|| {
            wire_name
                .parse::<MarketHoursKey>()
                .ok()
                .and_then(|key| calendar_for_market_hours_key(key).holiday_coverage())
        });
    coverage.map_or_else(
        || "\u{2014}".to_owned(),
        |coverage| {
            coverage
                .windows()
                .iter()
                .map(|(first, last)| format!("{first}..{last}"))
                .collect::<Vec<_>>()
                .join(", ")
        },
    )
}

/// Counts sentences in a basis note, ignoring ellipses and the abbreviations
/// in `SENTENCE_ABBREVIATIONS`.
fn basis_note_sentences(note: &str) -> usize {
    let mut text = note.replace("...", " ").replace('\u{2026}', " ");
    for abbreviation in SENTENCE_ABBREVIATIONS {
        text = text.replace(abbreviation, abbreviation.trim_end_matches('.'));
    }
    let mut chars = text.chars().peekable();
    let mut sentences = 0_usize;
    while let Some(character) = chars.next() {
        if matches!(character, '.' | '!' | '?')
            && chars.peek().is_none_or(|next| next.is_whitespace())
        {
            sentences = sentences.saturating_add(1);
        }
    }
    sentences
}

/// Asserts one ledger row carries the fixed eleven-cell shape.
///
/// `synthetic_name` is the one identity in this table whose profile is library
/// policy rather than a venue schedule; every other row must be a real
/// identity reviewed at or after the repository cutoff and at or before
/// `today`, the current UTC calendar date (LAW-UTC-DATES).
pub(super) fn assert_row_shape(
    row: &str,
    cutoff: NaiveDate,
    today: NaiveDate,
    synthetic_name: &str,
) {
    let cells = row_cells(row);
    assert_eq!(
        cells.len(),
        LEDGER_CELLS,
        "unexpected verification row shape: {row}"
    );
    let (basis, tier, service) = (cells[3], cells[4], cells[5]);
    let (horizon, holidays) = (cells[6], cells[7]);
    let (reviewed, cadence, note) = (cells[8], cells[9], cells[10]);

    assert!(
        VALID_BASES.contains(&basis),
        "unrecognized verification basis: {row}"
    );
    assert!(
        VALID_SERVICE_TIERS.contains(&service),
        "service tier must be served or dormant: {row}"
    );
    assert!(
        VALID_CADENCES.contains(&cadence),
        "review cadence must be monthly, quarterly or on demand: {row}"
    );
    assert_eq!(
        service == "dormant",
        cadence == "on demand",
        "LAW-WATCH: a dormant identity is reviewed on demand and a served one is not: {row}"
    );
    assert!(
        horizon == "\u{2014}" || NaiveDate::parse_from_str(horizon, "%Y-%m-%d").is_ok(),
        "horizon must be an ISO date or an em dash: {row}"
    );
    assert!(
        service != "served" || holidays == "\u{2014}" || cadence == "monthly",
        "LAW-WATCH: a served identity that ships a holiday table is reviewed \
         monthly, because the operator republishes its calendar yearly and \
         issues errata: {row}"
    );
    assert_eq!(
        holidays,
        shipped_holiday_window(wire_name(row)),
        "LAW-HOLIDAY-SCOPE: the Holidays cell is the identity's own built-in \
         coverage window, not a claim written by hand: {row}"
    );
    assert!(
        !note.contains("<br>"),
        "basis note must be plain prose, not a line-broken cell: {row}"
    );
    let sentences = basis_note_sentences(note);
    assert!(
        (1..=MAX_BASIS_SENTENCES).contains(&sentences),
        "basis note must hold one to three sentences, found {sentences}: {row}"
    );
    assert!(
        !note.contains("Gap: executable") && !note.contains("Gap: order-entry"),
        "the gap kind lives in the Basis cell, never in the note: {row}"
    );
    evidence_target(row);

    if wire_name(row) == synthetic_name {
        assert_eq!(basis, "Synthetic", "{synthetic_name} must remain synthetic");
        for (label, value) in [
            ("evidence tier", tier),
            ("horizon", horizon),
            ("review date", reviewed),
        ] {
            assert_eq!(
                value, "\u{2014}",
                "a synthetic profile has no {label}: {row}"
            );
        }
        assert_eq!(service, "dormant", "a synthetic profile is never served");
        assert_eq!(cadence, "on demand", "a synthetic profile has no cadence");
    } else {
        assert_ne!(
            basis, "Synthetic",
            "non-synthetic identity cannot be synthetic: {row}"
        );
        assert!(
            VALID_EVIDENCE_TIERS.contains(&tier),
            "unrecognized evidence tier: {row}"
        );
        assert!(
            matches!(tier, "T1" | "T2"),
            "LAW-PRIMARY-SOURCES: a current schedule at T3 or T4 is a defect: {row}"
        );
        let reviewed = NaiveDate::parse_from_str(reviewed, "%Y-%m-%d")
            .expect("every non-synthetic identity must have an ISO review date");
        assert!(
            reviewed >= cutoff,
            "review date predates repository cutoff: {row}"
        );
        assert!(
            reviewed <= today,
            "LAW-UTC-DATES: reviewed-on {reviewed} is later than today's UTC date \
             {today}, so the row is future-dated: {row}"
        );
    }
}

pub(super) fn validated_source_link_count(text: &str) -> u16 {
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

pub(super) fn assert_source_links_resolve(source_cell: &str, row: &str) {
    assert!(
        validated_source_link_count(source_cell) > 0,
        "ledger row must reference a source set: {row}"
    );
}

/// Spells `n` the way the schedule prose writes a count: "six",
/// "twenty-eight", "sixty-seven".
pub(super) fn number_words(n: usize) -> String {
    const ONES: [&str; 20] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    assert!(n < 100, "extend the number words past {n}");
    match (n / 10, n % 10) {
        _ if n < 20 => ONES[n].to_owned(),
        (tens, 0) => TENS[tens].to_owned(),
        (tens, ones) => format!("{}-{}", TENS[tens], ONES[ones]),
    }
}
