//! The `expectations.json` baseline, and the comparison of a run against it.

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::exports::wasi_gfx::webgpu_cts::cts_tests::CaseResultStatus;
use crate::{SpecResult, SpecResultStatus};

/// Baseline expectations
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub(crate) struct Expectations {
    /// How each spec's cases came out. Specs with no cases at all are omitted.
    #[serde(default)]
    case_counts: BTreeMap<String, CaseCounts>,
    /// Specs that trap the host runtime before reporting any case.
    #[serde(default)]
    runtime_trapped_specs: BTreeSet<String>,
}

/// How one spec's cases came out.
#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
struct CaseCounts {
    passed: usize,
    warned: usize,
    failed: usize,
    skipped: usize,
}

/// The case counts and runtime-trapped specs in a set of records.
fn extract_expectations<'a>(records: impl IntoIterator<Item = &'a SpecResult>) -> Expectations {
    let mut case_counts = BTreeMap::new();
    let mut runtime_trapped_specs = BTreeSet::new();
    for spec in records {
        match &spec.status {
            SpecResultStatus::Ran(run) => {
                let mut counts = CaseCounts::default();
                for case in run.tests.iter().flat_map(|test| &test.cases) {
                    match case.status {
                        CaseResultStatus::Passed => counts.passed += 1,
                        CaseResultStatus::Warned => counts.warned += 1,
                        CaseResultStatus::Failed => counts.failed += 1,
                        CaseResultStatus::Skipped => counts.skipped += 1,
                    }
                }
                if counts != CaseCounts::default() {
                    case_counts.insert(spec.name.clone(), counts);
                }
            }
            SpecResultStatus::RuntimeTrapped(_) => {
                runtime_trapped_specs.insert(spec.name.clone());
            }
            SpecResultStatus::GuestTrapped(_)
            | SpecResultStatus::Skipping(_)
            | SpecResultStatus::TimedOut => {}
        }
    }
    Expectations {
        case_counts,
        runtime_trapped_specs,
    }
}

/// The committed baseline, with the specs that just ran updated to match.
pub(crate) fn bless_expectations(
    mut blessed: Expectations,
    records: &[SpecResult],
) -> Expectations {
    let observed = extract_expectations(records);

    for spec in records.iter().map(|record| record.name.as_str()) {
        match observed.case_counts.get(spec) {
            Some(&counts) => blessed.case_counts.insert(spec.to_owned(), counts),
            None => blessed.case_counts.remove(spec),
        };
        if observed.runtime_trapped_specs.contains(spec) {
            blessed.runtime_trapped_specs.insert(spec.to_owned());
        } else {
            blessed.runtime_trapped_specs.remove(spec);
        }
    }

    blessed
}

/// Compares this run against the committed baseline.
pub(crate) fn check_against_expectations(expected: &Expectations, records: &[SpecResult]) -> bool {
    let actual = extract_expectations(records);

    let ran: HashSet<&str> = records.iter().map(|r| r.name.as_str()).collect();
    let counts_in =
        |e: &Expectations, spec: &str| e.case_counts.get(spec).copied().unwrap_or_default();

    let mut regressions = Vec::new();
    for (spec, &before) in &expected.case_counts {
        let after = counts_in(&actual, spec);
        if ran.contains(spec.as_str()) && after.passed < before.passed {
            regressions.push((spec.as_str(), before, after));
        }
    }
    let mut new_passes = Vec::new();
    for (spec, &after) in &actual.case_counts {
        let before = counts_in(expected, spec);
        if after.passed > before.passed {
            new_passes.push((spec.as_str(), before, after));
        }
    }

    let expected_trapped: HashSet<&str> = expected
        .runtime_trapped_specs
        .iter()
        .map(String::as_str)
        .collect();
    let new_traps: Vec<&str> = actual
        .runtime_trapped_specs
        .iter()
        .map(String::as_str)
        .filter(|spec| !expected_trapped.contains(spec))
        .collect();

    for (spec, before, after) in &new_passes {
        println!(
            "🎉 {} {spec}  {}",
            "[NEW PASSES]".green().bold(),
            describe_change(before, after),
        );
    }
    for (spec, before, after) in &regressions {
        println!(
            "{} {spec}  {}",
            "[REGRESSION]".red().bold(),
            describe_change(before, after),
        );
    }
    for spec in &new_traps {
        println!(
            "🚨 {} {spec}",
            "[NEW RUNTIME TRAP]".red().bold().on_yellow()
        );
    }

    let gained: usize = new_passes.iter().map(|(_, b, a)| a.passed - b.passed).sum();
    let lost: usize = regressions
        .iter()
        .map(|(_, b, a)| b.passed - a.passed)
        .sum();

    println!();
    println!("{}", "Against committed expectations:".bold());
    println!(
        "  {} specs with new passes: {} (+{gained} cases)",
        "🎉".green(),
        new_passes.len(),
    );
    println!(
        "  {} specs regressed:       {} (-{lost} cases)",
        "✗".red(),
        regressions.len(),
    );
    println!("  {} new runtime traps:    {}", "🚨".red(), new_traps.len());

    // In GitHub Actions, also show the comparison on the run's Summary page.
    if let Ok(path) = std::env::var("GITHUB_STEP_SUMMARY") {
        let summary = step_summary(&new_passes, &regressions, &new_traps, gained, lost);
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .and_then(|mut file| std::io::Write::write_all(&mut file, summary.as_bytes()))
            .unwrap();
    }

    !regressions.is_empty() || !new_traps.is_empty()
}

/// The comparison against the baseline as Markdown, for the GitHub Actions job summary.
fn step_summary(
    new_passes: &[(&str, CaseCounts, CaseCounts)],
    regressions: &[(&str, CaseCounts, CaseCounts)],
    new_traps: &[&str],
    gained: usize,
    lost: usize,
) -> String {
    let mut md = String::from("## WebGPU CTS against committed expectations\n\n");
    md += "| | Specs | Cases |\n|---|---|---|\n";
    md += &format!("| 🎉 New passes | {} | +{gained} |\n", new_passes.len());
    md += &format!("| ✗ Regressions | {} | -{lost} |\n", regressions.len());
    md += &format!("| 🚨 New runtime traps | {} | |\n\n", new_traps.len());

    let list = |title: &str, lines: Vec<String>, open: bool| {
        if lines.is_empty() {
            return String::new();
        }
        let open = if open { " open" } else { "" };
        let items: String = lines.iter().map(|line| format!("- {line}\n")).collect();
        format!("<details{open}><summary>{title}</summary>\n\n{items}\n</details>\n\n")
    };
    let changes = |specs: &[(&str, CaseCounts, CaseCounts)]| {
        specs
            .iter()
            .map(|(spec, before, after)| format!("`{spec}`: {}", describe_change(before, after)))
            .collect()
    };
    md += &list("Regressions", changes(regressions), true);
    md += &list(
        "New runtime traps",
        new_traps.iter().map(|spec| format!("`{spec}`")).collect(),
        true,
    );
    md += &list("New passes", changes(new_passes), false);
    md
}

/// `18 → 23 passed`, with the warned, failed, skipped, counts appended when they moved.
fn describe_change(before: &CaseCounts, after: &CaseCounts) -> String {
    let mut context = Vec::new();
    if before.warned != after.warned {
        context.push(format!("warned {} → {}", before.warned, after.warned));
    }
    if before.failed != after.failed {
        context.push(format!("failed {} → {}", before.failed, after.failed));
    }
    if before.skipped != after.skipped {
        context.push(format!("skipped {} → {}", before.skipped, after.skipped));
    }

    let mut change = format!("{} → {} passed", before.passed, after.passed);
    if !context.is_empty() {
        change += &format!("  ({})", context.join(", "));
    }
    change
}
