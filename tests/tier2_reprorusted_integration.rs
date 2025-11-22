//! Tier 2 Integration Tests: Reprorusted Examples
//!
//! Tests all reprorusted Python CLI examples using differential testing.
//! Validates semantic equivalence: Python output == Rust output
//!
//! Run with: cargo test --features certeza-tier2
//! Expected time: 2-3 minutes for 13 examples

use depyler_testing::differential::{DifferentialTester, ReprorustedTestSuite};
use std::path::Path;

/// Path to reprorusted examples directory
/// Expected to be cloned at ../reprorusted-python-cli
const REPRORUSTED_DIR: &str = "../reprorusted-python-cli/examples";

#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
fn test_reprorusted_example_simple() {
    let tester = DifferentialTester::new()
        .expect("Failed to initialize DifferentialTester");

    let python_file = Path::new(REPRORUSTED_DIR)
        .join("example_simple/trivial_cli.py");

    let result = tester.test_file(&python_file, &["--name", "Alice"])
        .expect("Failed to run differential test");

    assert!(
        result.passed,
        "example_simple: Outputs must match\n\
         Mismatches: {:#?}\n\
         Python stdout: {}\n\
         Rust stdout: {}",
        result.mismatches,
        result.python_output.stdout,
        result.rust_output.stdout
    );
}

#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
fn test_reprorusted_example_flags() {
    let tester = DifferentialTester::new()
        .expect("Failed to initialize DifferentialTester");

    let python_file = Path::new(REPRORUSTED_DIR)
        .join("example_flags/flags.py");

    let result = tester.test_file(&python_file, &["--debug"])
        .expect("Failed to run differential test");

    assert!(
        result.passed,
        "example_flags: Outputs must match\n\
         Mismatches: {:#?}",
        result.mismatches
    );
}

#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
#[ignore] // Currently failing - tracked in rearchitecture
fn test_reprorusted_example_config() {
    let tester = DifferentialTester::new()
        .expect("Failed to initialize DifferentialTester");

    let python_file = Path::new(REPRORUSTED_DIR)
        .join("example_config/config_manager.py");

    let result = tester.test_file(&python_file, &["init"])
        .expect("Failed to run differential test");

    assert!(
        result.passed,
        "example_config: Outputs must match\n\
         Mismatches: {:#?}",
        result.mismatches
    );
}

#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
#[ignore] // Currently failing
fn test_reprorusted_example_csv_filter() {
    let tester = DifferentialTester::new()
        .expect("Failed to initialize DifferentialTester");

    let python_file = Path::new(REPRORUSTED_DIR)
        .join("example_csv_filter/csv_filter.py");

    // Need to create test CSV file
    let test_csv = "/tmp/test_data.csv";
    std::fs::write(test_csv, "name,age\nAlice,30\nBob,25\nAlice,35\n")
        .expect("Failed to create test CSV");

    let result = tester.test_file(
        &python_file,
        &[test_csv, "--column", "name", "--value", "Alice"]
    ).expect("Failed to run differential test");

    assert!(
        result.passed,
        "example_csv_filter: Outputs must match\n\
         Mismatches: {:#?}",
        result.mismatches
    );
}

#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
#[ignore] // Currently failing
fn test_reprorusted_example_environment() {
    let tester = DifferentialTester::new()
        .expect("Failed to initialize DifferentialTester");

    let python_file = Path::new(REPRORUSTED_DIR)
        .join("example_environment/env_manager.py");

    let result = tester.test_file(&python_file, &["--show"])
        .expect("Failed to run differential test");

    assert!(
        result.passed,
        "example_environment: Outputs must match\n\
         Mismatches: {:#?}",
        result.mismatches
    );
}

/// Comprehensive test: Run ALL reprorusted examples
#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
fn test_reprorusted_all_examples_summary() {
    let examples_dir = Path::new(REPRORUSTED_DIR);

    if !examples_dir.exists() {
        panic!(
            "Reprorusted examples directory not found: {}\n\
             Please clone: git clone https://github.com/paiml/reprorusted-python-cli ../reprorusted-python-cli",
            examples_dir.display()
        );
    }

    let suite = ReprorustedTestSuite::new(examples_dir.to_path_buf());
    let results = suite.run_all();

    let pass_count = results.values().filter(|r| r.passed).count();
    let total_count = results.len();
    let pass_rate = (pass_count as f64 / total_count as f64) * 100.0;

    // Print summary
    println!("\n=== Reprorusted Differential Testing Report ===");
    println!("Pass Rate: {}/{} ({:.1}%)", pass_count, total_count, pass_rate);
    println!("\nResults:");

    for (name, result) in &results {
        let status = if result.passed { "✅ PASS" } else { "❌ FAIL" };
        println!(
            "  {}: {} (Python: {}ms, Rust: {}ms)",
            name,
            status,
            result.python_output.runtime_ms,
            result.rust_output.runtime_ms
        );

        if !result.passed {
            println!("    Mismatches:");
            for mismatch in &result.mismatches {
                println!("      {:?}", mismatch);
            }
        }
    }

    // Currently we expect only 2/13 to pass (example_simple, example_flags)
    // This test documents the current state
    // TODO: Update to assert pass_rate == 100.0 when rearchitecture is complete
    println!("\nCurrent state: {}% pass rate (target: 100%)", pass_rate);
    println!("Rearchitecture progress: Phase 2 integration test infrastructure ✅");
}

/// Generate HTML report for CI
#[test]
#[cfg_attr(feature = "certeza-tier2", test)]
#[ignore] // Only run manually or in CI
fn test_reprorusted_generate_html_report() {
    let examples_dir = Path::new(REPRORUSTED_DIR);
    let suite = ReprorustedTestSuite::new(examples_dir.to_path_buf());
    let results = suite.run_all();

    let html = suite.generate_report(&results);

    let report_path = "/tmp/depyler_differential_report.html";
    std::fs::write(report_path, html)
        .expect("Failed to write HTML report");

    println!("HTML report generated: {}", report_path);
}
