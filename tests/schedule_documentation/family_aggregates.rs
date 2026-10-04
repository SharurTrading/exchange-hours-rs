// SPDX-License-Identifier: MIT-0

//! Fence for the globex family evidence files' aggregate prose (issue #232).
//!
//! Each of the four CME family files — `globex_equity_index`, `globex_fx`,
//! `globex_grains`, `globex_interest_rates` — restates whole-table and per-era
//! aggregates in running prose: the twice-stated "The table as a whole carries
//! N rows over M windows" sentence with its per-era kind share, the
//! `## Holidays` wave paragraph's 2022-2024 row/tier split, `globex_grains`'s
//! 2025-2027 `**Rows:**` block, and the `### Documents` intro that states how
//! many artifacts its table resolves. Every one of those numbers drifted once
//! (issue #232: four files carried a stale whole-table total the 2010-2012
//! backfill had outrun; the #240 review then found the equity and fx
//! Documents intros stating a count their own tables never listed), and no
//! `schedule_documentation` fence spanned the sentences, so the drift class
//! was catchable only by hand.
//!
//! The other globex family whose file states these aggregates,
//! `globex_nikkei_225_dollar`, writes them in sentence shapes of its own
//! (issue #279): the whole-table sentence appears once, in the 2011-2015
//! section the 2010 floor-era share rides, with its tier and `Unsourced`
//! claims inline, the 2019-2021 share keeps the four families' shape, and the
//! 2022-2024 section states its share as a spelled `Unsourced` count broken
//! down by year.
//! `the_nikkei_aggregate_sentences_derive_from_the_shipped_tables` below
//! derives and pins them the same way.
//!
//! This is the #227/#230 pattern applied to the family files: the shipped
//! tables are re-derived through the **public** identity-backed surface — the
//! same walk `coverage_inventory.rs` derives its cells from — and the prose is
//! pinned by fragments built from the derivation, so a row that lands, a kind
//! that changes or a table row that moves fails the sentence until it is
//! restated. The sentence shapes live in the fragments themselves: a reworded
//! or renumbered sentence fails its `contains` with the derived text in the
//! message, which is the fence telling the editor exactly what the tables
//! derive.

use chrono::{Datelike as _, NaiveDate};
use exchange_hours::{
    EvidenceTier, ExchangeCalendar, HolidayKind, MarketHoursKey, calendar_for_market_hours_key,
};

use super::number_words;

/// The four CME families whose evidence files state whole-table aggregates.
///
/// `globex_nikkei_225_dollar` states the same kind of aggregates in its own
/// sentence shapes and is fenced by the nikkei test below; a further globex
/// family whose file grows either shape fails whichever fence it outgrows
/// until it is named deliberately.
const FAMILIES: [&str; 4] = [
    "globex_equity_index",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
];

/// One family's evidence file, as the compiler ships it.
fn evidence_file(family: &str) -> &'static str {
    match family {
        "globex_equity_index" => include_str!("../../docs/evidence/globex_equity_index.md"),
        "globex_fx" => include_str!("../../docs/evidence/globex_fx.md"),
        "globex_grains" => include_str!("../../docs/evidence/globex_grains.md"),
        "globex_nikkei_225_dollar" => {
            include_str!("../../docs/evidence/globex_nikkei_225_dollar.md")
        }
        _ => include_str!("../../docs/evidence/globex_interest_rates.md"),
    }
}

/// The identity-backed calendar whose shipped tables the prose must restate.
fn calendar_for(family: &str) -> ExchangeCalendar {
    let key = family
        .parse::<MarketHoursKey>()
        .expect("a family fixture names a known MarketHoursKey");
    calendar_for_market_hours_key(key)
}

/// Whitespace-normalizes hard-wrapped prose so a claim can straddle line
/// breaks, exactly as the sibling fences read the inventory and README prose.
fn flowed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// One audited window's row census, derived by walking the shipped table.
#[derive(Default)]
struct WindowCounts {
    /// Every row the window carries, whatever its kind.
    rows: usize,
    /// Rows whose kind is `Closed`.
    closed: usize,
    /// Rows whose kind is `EarlyClose`.
    early: usize,
    /// Rows whose kind is `LateOpen`.
    late: usize,
    /// Rows whose kind is `LateOpenAndEarlyClose`.
    late_and_early: usize,
    /// Rows whose kind is `ReplacementBlocks`.
    replacement: usize,
    /// Rows whose kind is `Unsourced`.
    unsourced: usize,
    /// Rows whose tier is T1.
    t1: usize,
    /// Rows whose tier is T2.
    t2: usize,
}

/// Walks one audited window and classifies every row it carries.
///
/// The window bounds are inclusive venue-local trade dates, the same unit the
/// coverage windows and the prose both state. A kind or tier the census does
/// not classify fails here rather than passing one fence later as a number
/// that silently dropped out of every total.
fn window_counts(calendar: ExchangeCalendar, first: NaiveDate, last: NaiveDate) -> WindowCounts {
    let mut counts = WindowCounts::default();
    let mut date = first;
    while date <= last {
        if let Some(holiday) = calendar.holiday_on(date) {
            counts.rows += 1;
            match holiday.kind() {
                HolidayKind::Closed => counts.closed += 1,
                HolidayKind::EarlyClose { .. } => counts.early += 1,
                HolidayKind::LateOpen { .. } => counts.late += 1,
                HolidayKind::LateOpenAndEarlyClose { .. } => counts.late_and_early += 1,
                HolidayKind::ReplacementBlocks(_) => counts.replacement += 1,
                HolidayKind::Unsourced => counts.unsourced += 1,
                other => panic!(
                    "{first}..{last}: the shipped table carries a kind this census does not \
                     classify: {other:?}"
                ),
            }
            match holiday.tier() {
                EvidenceTier::T1 => counts.t1 += 1,
                EvidenceTier::T2 => counts.t2 += 1,
                other => panic!(
                    "{first}..{last}: the shipped table carries a tier this census does not \
                     classify: {other:?}"
                ),
            }
        }
        date = date.succ_opt().expect("the window walk stays bounded");
    }
    counts
}

/// A `### ` subsection of an evidence file: its heading text and body.
struct Section {
    /// The heading with its `### ` marker stripped.
    heading: String,
    /// Everything from the heading line to the next `### ` or `## ` heading.
    body: String,
}

/// Every `### ` subsection of a file, in document order.
fn sections(file: &str) -> Vec<Section> {
    let mut out = Vec::new();
    let mut current: Option<Section> = None;
    for line in file.lines() {
        if let Some(heading) = line.strip_prefix("### ") {
            if let Some(section) = current.take() {
                out.push(section);
            }
            current = Some(Section {
                heading: heading.to_owned(),
                body: String::new(),
            });
            continue;
        }
        // A `## ` heading closes the subsection without opening one this walk
        // descends into.
        if line.starts_with("## ")
            && let Some(section) = current.take()
        {
            out.push(section);
        }
        if let Some(section) = current.as_mut() {
            section.body.push_str(line);
            section.body.push('\n');
        }
    }
    if let Some(section) = current.take() {
        out.push(section);
    }
    out
}

/// The `(first, last)` years a heading names, as `2013-2015` or
/// `2025-2027 (T2)` do: the first whitespace-delimited token of the shape
/// `YYYY-YYYY`. Headings without one name no audited era.
fn era_years(heading: &str) -> Option<(i32, i32)> {
    let token = heading.split_whitespace().find(|token| {
        let bytes = token.as_bytes();
        bytes.len() == 9
            && bytes[4] == b'-'
            && token[..4].parse::<i32>().is_ok()
            && token[5..].parse::<i32>().is_ok()
    })?;
    let (first, last) = token.split_once('-').expect("the token carries a dash");
    Some((
        first.parse().expect("the first year was just validated"),
        last.parse().expect("the last year was just validated"),
    ))
}

/// The audited window a heading's era names, looked up in the shipped
/// coverage. An era whose window the table does not declare fails here.
fn era_window(
    family: &str,
    coverage: &exchange_hours::HolidayCoverage,
    years: (i32, i32),
) -> (NaiveDate, NaiveDate) {
    let (first_year, last_year) = years;
    let first = NaiveDate::from_ymd_opt(first_year, 1, 1).expect("January 1st is a valid date");
    let last = NaiveDate::from_ymd_opt(last_year, 12, 31).expect("December 31st is a valid date");
    coverage
        .windows()
        .iter()
        .copied()
        .find(|(start, end)| *start == first && *end == last)
        .unwrap_or_else(|| {
            panic!(
                "{family}: the era {first_year}-{last_year} names no audited window the shipped \
                 table declares"
            )
        })
}

/// Whether the audited windows run contiguously from the 2010 floor, which is
/// the claim the 2019-2021 era paragraphs make in prose.
fn contiguous_from_floor(coverage: &exchange_hours::HolidayCoverage) -> bool {
    let windows = coverage.windows();
    let Some(first) = windows.first() else {
        return false;
    };
    if first.0 != NaiveDate::from_ymd_opt(2010, 1, 1).expect("the floor is a valid date") {
        return false;
    }
    windows
        .windows(2)
        .all(|pair| pair[0].1.succ_opt().expect("a window end has a successor") == pair[1].0)
}

/// The kind breakdown one era-share sentence states, spelled from the shipped
/// tables' derivation.
///
/// Each wording is the sentence's own; a family's share that changes shape —
/// a kind landing in an era whose sentence does not state it — fails the
/// fence with the derived text in the message, and the template is extended
/// to the sentence the editor restates. A new era section fails the
/// `carries no derived breakdown shape` panic until its sentence's shape is
/// named here.
fn era_breakdown(family: &str, era_first_year: i32, counts: &WindowCounts) -> String {
    match (family, era_first_year) {
        ("globex_equity_index" | "globex_fx" | "globex_interest_rates", 2013) => {
            format!(
                "{} stated closures, {} early closes and {} late opens",
                counts.closed, counts.early, counts.late
            )
        }
        ("globex_equity_index" | "globex_fx" | "globex_interest_rates", 2019) => {
            format!(
                "{} full closures, {} early closes and {} `Unsourced` rows",
                counts.closed, counts.early, counts.unsourced
            )
        }
        ("globex_grains", 2013) => format!(
            "{} stated closures, {} early closes, {} late opens and {} combined \
             late-open-and-early-close rows",
            counts.closed, counts.early, counts.late, counts.late_and_early
        ),
        ("globex_grains", 2019) => format!(
            "{} full closures, {} early closes, {} late opens, {} late opens with early closes \
             and {} `Unsourced` rows",
            counts.closed, counts.early, counts.late, counts.late_and_early, counts.unsourced
        ),
        _ => panic!(
            "{family}: the {era_first_year} era's whole-table sentence carries no derived \
             breakdown shape; name it in era_breakdown deliberately"
        ),
    }
}

/// The whole-table sentences are prose is data (review step 5): each era-gap
/// section restates the table's whole row total, its window count and list,
/// and its own era's kind share, and every one of those numbers rotted once
/// already (issue #232). The derivation walks the shipped tables through the
/// public surface, so a row that lands anywhere in the table fails both of a
/// file's sentences until they are restated, and the sentence count is pinned
/// so a third copy cannot appear unverified.
///
/// The same sentences claim "Every row is at T1" — derived here from the rows'
/// tiers — and the 2019-2021 paragraphs claim the audited windows run
/// contiguously from the floor so `holiday_coverage` answers for the whole
/// span, which is derived from the windows themselves.
#[test]
fn the_whole_table_sentences_derive_from_the_shipped_tables() {
    for family in FAMILIES {
        let calendar = calendar_for(family);
        let coverage = calendar
            .holiday_coverage()
            .expect("a served family ships a holiday table");
        let window_list = coverage
            .windows()
            .iter()
            .map(|(first, last)| format!("{first}..{last}"))
            .collect::<Vec<_>>()
            .join(", ");
        let total: usize = coverage
            .windows()
            .iter()
            .map(|(start, end)| window_counts(calendar, *start, *end).rows)
            .sum();
        let file = flowed(evidence_file(family));
        let sentence = format!(
            "The table as a whole carries {total} rows over {} windows \u{2014} {window_list} \
             \u{2014} and this era's share is **",
            coverage.windows().len()
        );
        assert_eq!(
            file.matches(&sentence).count(),
            2,
            "{family}: the whole-table sentence must state the shipped table's derivation \
             exactly twice"
        );

        let file_sections = sections(evidence_file(family));
        let gap_sections: Vec<&Section> = file_sections
            .iter()
            .filter(|section| {
                section.heading.starts_with("Gaps and residual risks")
                    && section.body.contains("The table as a whole carries")
            })
            .collect();
        assert_eq!(
            gap_sections.len(),
            2,
            "{family}: two era sections carry the whole-table sentence"
        );
        for section in &gap_sections {
            let years = era_years(&section.heading)
                .unwrap_or_else(|| panic!("{family}: the gap section heading names no era"));
            let (first, last) = era_window(family, &coverage, years);
            let counts = window_counts(calendar, first, last);
            let breakdown = era_breakdown(family, years.0, &counts);
            let expected = format!(
                "and this era's share is **{} rows**: {breakdown}. Every row is at T1.",
                counts.rows
            );
            assert!(
                flowed(&section.body).contains(&expected),
                "{family}: the {}-{} era share must restate the shipped table's derivation \
                 ({first}..{last}): {expected:?}",
                years.0,
                years.1
            );
            assert_eq!(
                counts.t1, counts.rows,
                "{family}: the {}-{} sentence claims every row is at T1",
                years.0, years.1
            );
            if years.0 == 2019 {
                assert!(
                    contiguous_from_floor(&coverage),
                    "{family}: the 2019-2021 paragraph claims every interval from 2010-01-01 is \
                     inside a declared window"
                );
                assert!(
                    flowed(&section.body).contains(
                        "Every interval from 2010-01-01 is inside a declared window, so \
                         `holiday_coverage` answers for the whole span rather than reporting an \
                         unaudited gap.",
                    ),
                    "{family}: the 2019-2021 paragraph must state the contiguity claim its \
                     windows derive"
                );
            }
        }
    }
}

/// Capitalizes a spelled count the way a paragraph's opening bold states it.
fn capitalized(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// The `Unsourced` trade dates one calendar year ships, walked from the
/// shipped table.
fn unsourced_dates(calendar: ExchangeCalendar, year: i32) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut date = NaiveDate::from_ymd_opt(year, 1, 1).expect("January 1st is a valid date");
    let last = NaiveDate::from_ymd_opt(year, 12, 31).expect("December 31st is a valid date");
    while date <= last {
        if calendar
            .holiday_on(date)
            .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
        {
            dates.push(date);
        }
        date = date.succ_opt().expect("the year walk stays bounded");
    }
    dates
}

/// The `globex_nikkei_225_dollar` aggregates are prose is data too (issue
/// #279): the file restates the shipped table's whole row total, window count
/// and list, and per-era kind shares, in sentence shapes of its own. The
/// whole-table sentence appears once, in the 2011-2015 section the 2010
/// floor-era share rides, and continues `of which the 2011-2015 share is`
/// with its tier and `Unsourced` claims inline where the four families write
/// `and this era's share is` and `Every row is at T1`; the 2019-2021 section's
/// share uses that four-family shape; the 2022-2024 section states its share
/// as a spelled `Unsourced` count broken down by year. Each sentence is pinned
/// by a fragment built from the derivation, so a row that lands anywhere in
/// the table fails the sentences it aggregates until they are restated, and a
/// new era section fails the panic below until its shape is named here. The
/// 2025-2027 era states no whole-table aggregate today; a section that grows
/// one fails the same way.
#[test]
fn the_nikkei_aggregate_sentences_derive_from_the_shipped_tables() {
    let family = "globex_nikkei_225_dollar";
    let calendar = calendar_for(family);
    let coverage = calendar
        .holiday_coverage()
        .expect("a served family ships a holiday table");
    let window_list = coverage
        .windows()
        .iter()
        .map(|(first, last)| format!("{first}..{last}"))
        .collect::<Vec<_>>()
        .join(", ");
    let total: usize = coverage
        .windows()
        .iter()
        .map(|(start, end)| window_counts(calendar, *start, *end).rows)
        .sum();

    // The 2011-2015 share spans the first two windows, so it is walked over
    // its years rather than looked up as one declared window; the 2010
    // floor-era share rides the same section, and the sentence decomposes it
    // into the 2010-01-01 venue-wide closure plus the post-changeover rows.
    let share_2011_2015 = window_counts(
        calendar,
        NaiveDate::from_ymd_opt(2011, 1, 1).expect("January 1st is a valid date"),
        NaiveDate::from_ymd_opt(2015, 12, 31).expect("December 31st is a valid date"),
    );
    assert_eq!(
        share_2011_2015.unsourced, 0,
        "{family}: the 2011-2015 sentence claims none `Unsourced`"
    );
    assert_eq!(
        share_2011_2015.t1, share_2011_2015.rows,
        "{family}: the 2011-2015 sentence claims every one at T1"
    );
    let floor_day = NaiveDate::from_ymd_opt(2010, 1, 1).expect("the floor is a valid date");
    let era_2010 = window_counts(
        calendar,
        floor_day,
        NaiveDate::from_ymd_opt(2010, 12, 31).expect("December 31st is a valid date"),
    );
    assert!(
        calendar
            .holiday_on(floor_day)
            .is_some_and(|holiday| holiday.kind() == HolidayKind::Closed),
        "{family}: the 2010 share's parenthetical names the 2010-01-01 venue-wide closure"
    );
    let post_changeover = era_2010.rows - 1;
    let whole_table_sentence = format!(
        "The table as a whole carries {total} rows over {} windows \u{2014} {window_list} \
         \u{2014} of which the 2011-2015 share is **{} rows**: {} closures, {} early closes \
         and {} late opens, every one at T1, none `Unsourced`; the 2010 share is the {} rows \
         of the era blocks above (the 2010-01-01 venue-wide closure on the modelled old \
         grid, plus the {} post-changeover rows).",
        coverage.windows().len(),
        share_2011_2015.rows,
        share_2011_2015.closed,
        share_2011_2015.early,
        share_2011_2015.late,
        number_words(era_2010.rows),
        number_words(post_changeover),
    );
    let file = flowed(evidence_file(family));
    assert_eq!(
        file.matches(&whole_table_sentence).count(),
        1,
        "{family}: the whole-table sentence must state the shipped table's derivation \
         exactly once: {whole_table_sentence:?}"
    );

    let file_sections = sections(evidence_file(family));
    let gap_sections: Vec<&Section> = file_sections
        .iter()
        .filter(|section| section.heading.starts_with("Gaps and residual risks"))
        .collect();
    assert_eq!(
        gap_sections.len(),
        3,
        "{family}: three era sections carry the aggregate sentence shapes"
    );
    for section in &gap_sections {
        let years = era_years(&section.heading)
            .unwrap_or_else(|| panic!("{family}: the gap section heading names no era"));
        let flowed_body = flowed(&section.body);
        match years {
            // The floor-era special: the 2010 share rides this section, whose
            // whole-table sentence the derivation above pins.
            (2011, 2015) => assert!(
                flowed_body.contains(&whole_table_sentence),
                "{family}: the 2011-2015 section must carry the whole-table sentence the \
                 shipped tables derive: {whole_table_sentence:?}"
            ),
            (2019, 2021) => {
                let (first, last) = era_window(family, &coverage, years);
                let counts = window_counts(calendar, first, last);
                assert_eq!(
                    counts.t1, counts.rows,
                    "{family}: the 2019-2021 sentence claims every row is at T1"
                );
                let expected = format!(
                    "and this era's share is **{} rows**: {} full closures, {} early closes \
                     and {} `Unsourced` rows. Every row is at T1.",
                    counts.rows, counts.closed, counts.early, counts.unsourced
                );
                assert!(
                    flowed_body.contains(&expected),
                    "{family}: the 2019-2021 share must restate the shipped table's \
                     derivation ({first}..{last}): {expected:?}"
                );
            }
            (2022, 2024) => {
                let (first, last) = era_window(family, &coverage, years);
                let counts = window_counts(calendar, first, last);
                let dates_2022 = unsourced_dates(calendar, 2022);
                let unsourced_2022 = match dates_2022.as_slice() {
                    [only] => only.to_string(),
                    _ => panic!(
                        "{family}: the 2022-2024 paragraph names one 2022 date and the \
                         shipped table ships {}",
                        dates_2022.len()
                    ),
                };
                let unsourced_2023 = unsourced_dates(calendar, 2023).len();
                let unsourced_2024 = unsourced_dates(calendar, 2024).len();
                assert_eq!(
                    counts.unsourced,
                    dates_2022.len() + unsourced_2023 + unsourced_2024,
                    "{family}: the 2022-2024 paragraph's year split must cover the window's \
                     `Unsourced` rows"
                );
                let expected = format!(
                    "**{} `Unsourced` rows: {unsourced_2022}, the {} 2023 dates and {} 2024 \
                     dates.**",
                    capitalized(&number_words(counts.unsourced)),
                    number_words(unsourced_2023),
                    number_words(unsourced_2024),
                );
                assert!(
                    flowed_body.contains(&expected),
                    "{family}: the 2022-2024 `Unsourced` share must restate the shipped \
                     table's derivation ({first}..{last}): {expected:?}"
                );
            }
            _ => panic!(
                "{family}: the {}-{} era section carries no derived share shape; name it in \
                 the_nikkei_aggregate_sentences_derive_from_the_shipped_tables deliberately",
                years.0, years.1
            ),
        }
    }
}

/// The `## Holidays` wave paragraph is prose is data too: it states the
/// 2022-2024 wave's row count, its T1/T2 split, the 2023 dates the operator's
/// one-pagers do not cover, and the wave's `Unsourced` share. The tier split
/// rotted in two files at once before #240 (grains 23/16, rates 19/14), so
/// the derivation walks the wave window's rows and pins every number the
/// paragraph states, with the count words spelled and inflected the way the
/// sentences spell and inflect them.
#[test]
fn the_wave_paragraphs_derive_from_the_shipped_tables() {
    for family in FAMILIES {
        let calendar = calendar_for(family);
        let coverage = calendar
            .holiday_coverage()
            .expect("a served family ships a holiday table");
        let file = flowed(evidence_file(family));
        assert_eq!(
            file.matches("This wave is ").count(),
            1,
            "{family}: one wave paragraph states the 2022-2024 wave"
        );
        let (_, rest) = file
            .split_once("This wave is ")
            .expect("the wave paragraph was just asserted present");
        let (_, after_count) = rest
            .split_once(" rows over ")
            .expect("the wave paragraph states its row count and window");
        let window = after_count
            .split(',')
            .next()
            .expect("a window string follows");
        let (first, last) = coverage
            .windows()
            .iter()
            .copied()
            .find(|(start, end)| format!("{start}..{end}") == window)
            .unwrap_or_else(|| {
                panic!(
                    "{family}: the wave paragraph names {window}, which the shipped table does \
                     not declare"
                )
            });
        let counts = window_counts(calendar, first, last);
        assert_eq!(
            counts.t1 + counts.t2,
            counts.rows,
            "{family}: the wave window {first}..{last} carries a row at neither T1 nor T2"
        );
        assert!(
            file.contains(&format!(
                "This wave is {} rows over {window}, **{} at T1** and **{} at T2**.",
                counts.rows, counts.t1, counts.t2
            )),
            "{family}: the wave paragraph must restate the shipped table's derivation \
             ({first}..{last}: {} rows, {} at T1, {} at T2)",
            counts.rows,
            counts.t1,
            counts.t2
        );

        // The 2023 dates the one-pagers do not cover are the wave window's
        // `Unsourced` rows dated in 2023, and the share sentence states the
        // window's whole `Unsourced` count against its whole row count.
        let mut unsourced_2023 = 0_usize;
        let mut date = first;
        while date <= last {
            if date.year() == 2023
                && calendar
                    .holiday_on(date)
                    .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
            {
                unsourced_2023 += 1;
            }
            date = date.succ_opt().expect("the 2023 walk stays bounded");
        }
        let (verb, noun) = if counts.unsourced == 1 {
            ("is", "date")
        } else {
            ("are", "dates")
        };
        assert!(
            file.contains(&format!(
                "the {} 2023 holiday {noun} its one-pagers do not cover",
                number_words(unsourced_2023),
            )),
            "{family}: the wave paragraph must restate the shipped table's 2023 `Unsourced` \
             count ({unsourced_2023})"
        );
        assert!(
            file.contains(&format!(
                "{} of the {} {} `unsourced`",
                counts.unsourced, counts.rows, verb
            )),
            "{family}: the wave paragraph must restate the shipped table's `Unsourced` share \
             ({} of the {} rows)",
            counts.unsourced,
            counts.rows
        );
    }
}

/// Reads the count a Documents intro states as a number word, the inverse of
/// the `number_words` the sibling fences spell with. Counts past ninety-nine
/// fail here and ask for the parser to be extended rather than passing
/// silently.
fn words_to_number(word: &str) -> usize {
    const UNITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const TEENS: [&str; 10] = [
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
    const TENS: [&str; 8] = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let part = |word: &str| -> usize {
        if let Some(at) = UNITS.iter().position(|unit| *unit == word) {
            return at + 1;
        }
        if let Some(at) = TEENS.iter().position(|teen| *teen == word) {
            return 10 + at;
        }
        if let Some(at) = TENS.iter().position(|ten| *ten == word) {
            return 10 * (at + 2);
        }
        panic!("a Documents intro states the count word {word:?}, which this fence cannot read")
    };
    let number = match word.split_once('-') {
        Some((tens, units)) => {
            let tens_value = part(tens);
            assert!(
                tens_value % 10 == 0 && tens_value >= 20,
                "a Documents intro states the count word {word:?}, which this fence cannot read"
            );
            tens_value + part(units)
        }
        None => part(word),
    };
    assert!(
        (1..100).contains(&number),
        "the Documents-intro fence reads one through ninety-nine; {number} needs it extended"
    );
    number
}

/// The Documents-section intro counts are prose is data: the intro above each
/// counted table states how many artifacts the table resolves — "All
/// forty-one artifacts resolve in the research store's …" — and the equity
/// and fx intros were born already not matching their own tables (29 and 13
/// rows under a "thirty" the #240 review could not re-derive under any
/// reading). The derivation counts the table rows the intro introduces and
/// pins the sentence with the count spelled the way intros spell it, so a
/// table row that lands or is withdrawn fails the intro until it is
/// restated.
#[test]
fn the_documents_intro_counts_derive_from_the_documents_tables() {
    for family in FAMILIES {
        let file_sections = sections(evidence_file(family));
        let counted: Vec<&str> = file_sections
            .iter()
            .filter(|section| section.heading == "Documents")
            .map(|section| section.body.as_str())
            .filter(|body| body.contains("artifacts resolve in the research store's"))
            .collect();
        assert_eq!(
            counted.len(),
            1,
            "{family}: exactly one Documents section states an artifact count"
        );
        let flowed_body = flowed(counted[0]);
        let (_, rest) = flowed_body
            .split_once("All ")
            .expect("the intro states its count after `All `");
        let (word, _) = rest
            .split_once(" artifacts resolve in the research store's")
            .unwrap_or_else(|| {
                panic!(
                    "{family}: the intro states `All <count> artifacts resolve in the research \
                     store's`"
                )
            });
        let rows = counted[0]
            .lines()
            .filter(|line| line.starts_with("| `"))
            .count();
        assert_eq!(
            words_to_number(word),
            rows,
            "{family}: the Documents intro states {word:?} artifacts, but its own table lists \
             {rows} rows"
        );
    }
}

/// The `globex_grains` 2025-2027 block's `**Rows:**` line is the era's whole
/// aggregate in one place — total, closures, early closes, combined rows,
/// replacement blocks and `Unsourced` — and it was corrected from a stale 54
/// in #240 with nothing to keep it true. The derivation walks the 2025-2027
/// window and pins the line flowed, so a row that lands there fails it.
///
/// The other three families' files state no such block today; one that grows
/// one fails the assertion below until it is fenced deliberately.
#[test]
fn the_2025_2027_rows_block_derives_from_the_shipped_tables() {
    for family in FAMILIES {
        let file_sections = sections(evidence_file(family));
        let blocks: Vec<&Section> = file_sections
            .iter()
            .filter(|section| section.body.contains("**Rows:**"))
            .collect();
        if family != "globex_grains" {
            assert!(
                blocks.is_empty(),
                "{family}: a `**Rows:**` block appeared outside globex_grains; fence it in this \
                 test deliberately"
            );
            continue;
        }
        assert_eq!(
            blocks.len(),
            1,
            "{family}: exactly one section states a `**Rows:**` aggregate"
        );
        let section = blocks[0];
        let years = era_years(&section.heading)
            .unwrap_or_else(|| panic!("{family}: the Rows block's heading names no audited era"));
        let calendar = calendar_for(family);
        let coverage = calendar
            .holiday_coverage()
            .expect("a served family ships a holiday table");
        let (first, last) = era_window(family, &coverage, years);
        let counts = window_counts(calendar, first, last);
        let expected = format!(
            "**Rows:** {} \u{2014} {} `closed`, {} `early close`, {} `late open and early \
             close`, {} `replacement blocks`, {} `unsourced`.",
            counts.rows,
            counts.closed,
            counts.early,
            counts.late_and_early,
            counts.replacement,
            counts.unsourced
        );
        assert!(
            flowed(&section.body).contains(&expected),
            "{family}: the {}-{} Rows block must restate the shipped table's derivation \
             ({first}..{last}): {expected:?}",
            years.0,
            years.1
        );
    }
}
