// SPDX-License-Identifier: MIT-0

//! Contract for the checked Databento venue crosswalk.

use super::{DATABENTO_VENUES, README, exchange_rows, row_cells, wire_name};
use exchange_hours::Exchange;

const EXPECTED_MAPPINGS: [(&str, &str, Exchange); 50] = [
    ("Equities", "Blue Ocean ATS", Exchange::BlueOceanAts),
    ("Equities", "Cboe BYX", Exchange::CboeByx),
    ("Equities", "Cboe BZX", Exchange::CboeBzx),
    ("Equities", "Cboe EDGA", Exchange::CboeEdga),
    ("Equities", "Cboe EDGX", Exchange::CboeEdgx),
    (
        "Equities",
        "FINRA/Nasdaq TRF Carteret",
        Exchange::FinraTrfCarteret,
    ),
    (
        "Equities",
        "FINRA/Nasdaq TRF Chicago",
        Exchange::FinraTrfChicago,
    ),
    ("Equities", "FINRA/NYSE TRF", Exchange::FinraTrfNyse),
    ("Equities", "IEX", Exchange::Iex),
    ("Equities", "MEMX", Exchange::MemxEq),
    ("Equities", "MIAX Pearl Equities", Exchange::MiaxPearlEq),
    ("Equities", "Nasdaq", Exchange::Nasdaq),
    ("Equities", "Nasdaq PSX", Exchange::NasdaqPsx),
    ("Equities", "Nasdaq Texas", Exchange::NasdaqBx),
    ("Equities", "NYSE", Exchange::Nyse),
    ("Equities", "NYSE American", Exchange::NyseAmerican),
    ("Equities", "NYSE Arca", Exchange::NyseArca),
    ("Equities", "NYSE National", Exchange::NyseNational),
    ("Equities", "NYSE Texas", Exchange::NyseTexas),
    ("Equity options", "BOX Options", Exchange::BoxOptions),
    (
        "Equity options",
        "Cboe BZX Options",
        Exchange::CboeBzxOptions,
    ),
    ("Equity options", "Cboe C2 Options", Exchange::CboeC2Options),
    (
        "Equity options",
        "Cboe EDGX Options",
        Exchange::CboeEdgxOptions,
    ),
    ("Equity options", "Cboe Options", Exchange::CboeOptionsC1),
    ("Equity options", "MEMX Options", Exchange::MemxOptions),
    (
        "Equity options",
        "MIAX Emerald",
        Exchange::MiaxEmeraldOptions,
    ),
    ("Equity options", "MIAX Options", Exchange::MiaxOptions),
    ("Equity options", "MIAX Pearl", Exchange::MiaxPearlOptions),
    (
        "Equity options",
        "MIAX Sapphire",
        Exchange::MiaxSapphireOptions,
    ),
    ("Equity options", "Nasdaq GEMX", Exchange::NasdaqGemx),
    ("Equity options", "Nasdaq ISE", Exchange::NasdaqIse),
    ("Equity options", "Nasdaq MRX", Exchange::NasdaqMrx),
    ("Equity options", "Nasdaq Options", Exchange::NasdaqNom),
    ("Equity options", "Nasdaq PHLX", Exchange::NasdaqPhlx),
    (
        "Equity options",
        "Nasdaq Texas Options",
        Exchange::NasdaqBxOptions,
    ),
    (
        "Equity options",
        "NYSE American Options",
        Exchange::NyseAmericanOptions,
    ),
    (
        "Equity options",
        "NYSE Arca Options",
        Exchange::NyseArcaOptions,
    ),
    ("Futures and options on futures", "CBOT", Exchange::Cbot),
    ("Futures and options on futures", "CFE", Exchange::Cfe),
    ("Futures and options on futures", "CME", Exchange::Cme),
    ("Futures and options on futures", "COMEX", Exchange::Comex),
    ("Futures and options on futures", "EEX", Exchange::Eex),
    ("Futures and options on futures", "Eurex", Exchange::Eurex),
    (
        "Futures and options on futures",
        "ICE Abu Dhabi",
        Exchange::IceAbuDhabi,
    ),
    (
        "Futures and options on futures",
        "ICE Canada",
        Exchange::IceCanada,
    ),
    (
        "Futures and options on futures",
        "ICE Endex",
        Exchange::IceEndex,
    ),
    (
        "Futures and options on futures",
        "ICE Europe Commodities",
        Exchange::IceEuropeCommodities,
    ),
    (
        "Futures and options on futures",
        "ICE Europe Financials",
        Exchange::IceEuropeFinancials,
    ),
    (
        "Futures and options on futures",
        "ICE Futures US",
        Exchange::Iceus,
    ),
    ("Futures and options on futures", "NYMEX", Exchange::Nymex),
];

fn mapping_rows() -> Vec<Vec<&'static str>> {
    DATABENTO_VENUES
        .lines()
        .filter_map(|line| {
            let cells = line
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            (cells.len() == 5
                && matches!(
                    cells[0],
                    "Equities" | "Equity options" | "Futures and options on futures"
                ))
            .then_some(cells)
        })
        .collect()
}

#[test]
fn supplied_venue_inventory_maps_every_distinct_label_to_a_real_exchange() {
    let rows = mapping_rows();
    assert_eq!(rows.len(), EXPECTED_MAPPINGS.len());

    for (row, (section, vendor_label, exchange)) in rows.iter().zip(EXPECTED_MAPPINGS) {
        assert_eq!(row[0], section);
        assert_eq!(row[1], vendor_label);
        assert_eq!(row[2], format!("`Exchange::{exchange:?}`"));
        assert_eq!(row[3], format!("`{}`", exchange.as_str()));
        assert_ne!(exchange, Exchange::Unknown);

        let ledger_row = exchange_rows()
            .into_iter()
            .find(|ledger_row| wire_name(ledger_row) == exchange.as_str())
            .expect("mapped exchange must have a verification-ledger row");
        assert_eq!(row[4], row_cells(ledger_row)[3]);
        assert!(matches!(row[4], "Primary" | "Partial"));

        let parsed = exchange
            .as_str()
            .parse::<Exchange>()
            .expect("documented canonical exchange name must parse");
        assert_eq!(parsed, exchange);
    }

    assert!(README.contains("docs/schedules/databento-venues.md"));
}

#[test]
fn supplied_venue_inventory_has_expected_family_counts_and_unique_identities() {
    let equities = EXPECTED_MAPPINGS
        .iter()
        .filter(|(section, _, _)| *section == "Equities")
        .count();
    let equity_options = EXPECTED_MAPPINGS
        .iter()
        .filter(|(section, _, _)| *section == "Equity options")
        .count();
    let futures = EXPECTED_MAPPINGS
        .iter()
        .filter(|(section, _, _)| *section == "Futures and options on futures")
        .count();

    assert_eq!((equities, equity_options, futures), (19, 18, 13));

    for (index, (_, label, exchange)) in EXPECTED_MAPPINGS.iter().enumerate() {
        assert!(
            !EXPECTED_MAPPINGS[..index]
                .iter()
                .any(|(_, earlier_label, earlier_exchange)| {
                    earlier_label == label || earlier_exchange == exchange
                }),
            "duplicate vendor label or exchange mapping: {label} / {exchange:?}"
        );
    }
}
