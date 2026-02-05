//! DEPYLER-SCORE-001: 100-Point Single-Shot Compile Score CLI
//!
//! Compute comprehensive transpilation quality score across 5 categories.

use anyhow::Result;
use depyler_core::scoring::{
    BreakdownInput, CompilationError, CorpusScoreReport, OutputFormat, ScoreCalculator,
    ScoringConfig, ScoringMode, SingleShotResult,
};
use std::path::PathBuf;

/// Arguments for the score command
pub struct ScoreArgs {
    pub input_dir: PathBuf,
    pub mode: String,
    pub min_score: u8,
    pub format: String,
    pub output: Option<PathBuf>,
    pub semantic: bool,
    pub oracle_feedback: bool,
}

/// Parse scoring mode from string
fn parse_mode(mode: &str) -> ScoringMode {
    match mode.to_lowercase().as_str() {
        "quick" => ScoringMode::Quick,
        "full" => ScoringMode::Full,
        _ => ScoringMode::Fast,
    }
}

/// Parse output format from string
fn parse_format(format: &str) -> OutputFormat {
    match format.to_lowercase().as_str() {
        "json" => OutputFormat::Json,
        "markdown" | "md" => OutputFormat::Markdown,
        _ => OutputFormat::Human,
    }
}

/// Score a single Python file
fn score_file(
    path: &PathBuf,
    mode: ScoringMode,
    calculator: &ScoreCalculator,
    _semantic: bool,
) -> Result<SingleShotResult> {
    use depyler_core::DepylerPipeline;
    use std::process::Command;

    let pipeline = DepylerPipeline::new();
    let python_source = std::fs::read_to_string(path)?;

    // Phase A1: Parse
    let parse_ok = pipeline.transpile(&python_source).is_ok();

    // Get Rust code if parse succeeded
    let rust_code = if parse_ok {
        pipeline.transpile(&python_source).ok()
    } else {
        None
    };

    // Phase A2 & A3: Type check and build (if we have Rust code)
    let (type_check_ok, build_ok, errors) = if let Some(ref code) = rust_code {
        // Write to temp Cargo project
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        std::fs::create_dir_all(&src_dir)?;
        let rs_path = src_dir.join("lib.rs");
        std::fs::write(&rs_path, code)?;

        // Generate Cargo.toml with dependencies from transpilation context
        let cargo_toml = generate_cargo_toml_for_code(code);
        std::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

        // Run cargo check for type check (resolves dependencies)
        let type_output = Command::new("cargo")
            .args(["check", "--lib", "--message-format=short"])
            .current_dir(temp_dir.path())
            .output()?;

        let type_check_ok = type_output.status.success();

        // Parse errors
        let error_output = String::from_utf8_lossy(&type_output.stderr);
        let errors = parse_cargo_errors(&error_output);

        // Run cargo build for full compilation (only in Fast/Full mode)
        let build_ok = if matches!(mode, ScoringMode::Quick) {
            type_check_ok
        } else {
            let build_output = Command::new("cargo")
                .args(["build", "--lib"])
                .current_dir(temp_dir.path())
                .output()?;
            build_output.status.success()
        };

        (type_check_ok, build_ok, errors)
    } else {
        (false, false, vec![])
    };

    // Calculate breakdown
    let breakdown = calculator.breakdown_from_errors(&BreakdownInput {
        parse_ok,
        type_check_ok,
        build_ok,
        errors: &errors,
        doctest_pass: false,          // would need to run doctests
        unit_test_pass: false,        // would need to run tests
        property_test_pass: false,    // would need to run property tests
        clippy_clean: false,          // would need to run clippy
        tdg_grade_b_or_better: false, // would need PMAT
        complexity_ok: true,          // assume OK for now
        trace_match: false,           // requires Renacer
        output_equiv: false,          // requires running both
    });

    let score = calculator.calculate(&breakdown, mode);

    Ok(SingleShotResult {
        file_path: path.clone(),
        score,
        category_breakdown: breakdown,
        error_details: errors,
        transpiler_decisions: vec![],
    })
}

/// Generate minimal Cargo.toml based on imports detected in code
fn generate_cargo_toml_for_code(code: &str) -> String {
    let mut deps = vec![];

    // Detect common dependencies from use statements
    if code.contains("once_cell::") {
        deps.push("once_cell = \"1.20\"");
    }
    if code.contains("use clap::") || code.contains("clap::Parser") {
        deps.push("clap = { version = \"4.5\", features = [\"derive\"] }");
    }
    if code.contains("use serde") || code.contains("serde::") {
        deps.push("serde = { version = \"1.0\", features = [\"derive\"] }");
    }
    if code.contains("serde_json::") {
        deps.push("serde_json = \"1.0\"");
    }
    if code.contains("use regex") || code.contains("regex::") {
        deps.push("regex = \"1.0\"");
    }
    if code.contains("use chrono") || code.contains("chrono::") {
        deps.push("chrono = \"0.4\"");
    }
    if code.contains("use itertools") || code.contains("itertools::") {
        deps.push("itertools = \"0.12\"");
    }
    if code.contains("use csv") || code.contains("csv::") {
        deps.push("csv = \"1.0\"");
    }
    if code.contains("use tempfile") || code.contains("tempfile::") {
        deps.push("tempfile = \"3.0\"");
    }
    if code.contains("use rand") || code.contains("rand::") {
        deps.push("rand = \"0.8\"");
    }

    let deps_section = if deps.is_empty() {
        String::new()
    } else {
        deps.join("\n")
    };

    format!(
        r#"[package]
name = "score_test"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
{deps_section}
"#
    )
}

/// Parse cargo error output into CompilationError structs
fn parse_cargo_errors(output: &str) -> Vec<CompilationError> {
    let mut errors = vec![];

    for line in output.lines() {
        // Look for error codes like "error[E0308]"
        if let Some(start) = line.find("error[E") {
            if let Some(end) = line[start..].find(']') {
                let code = &line[start + 6..start + end];
                errors.push(CompilationError {
                    code: code.to_string(),
                    message: line.to_string(),
                    location: None,
                    line: None,
                });
            }
        }
    }

    errors
}

/// Format score report for human output
fn format_human(report: &CorpusScoreReport) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "📊 Single-Shot Compile Score: {} ({:.1}/100)\n",
        report.grade.as_str(),
        report.aggregate_score
    ));
    output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

    // Category breakdown
    let ca = &report.category_averages;
    let a_total = ca.a1_parse + ca.a2_type_check + ca.a3_cargo_build;
    let b_total = ca.b1_no_e0308 + ca.b2_no_e0599 + ca.b3_no_e0425;
    let c_total = ca.c1_doctest + ca.c2_unit_test + ca.c3_property_test;
    let d_total = ca.d1_clippy + ca.d2_tdg + ca.d3_complexity;
    let e_total = ca.e1_trace_match + ca.e2_output_equiv;

    output.push_str("Category Breakdown:\n");
    output.push_str(&format!(
        "  A. Compilation:        {}/40 ({:.1}%)\n",
        a_total,
        (a_total as f32 / 40.0) * 100.0
    ));
    output.push_str(&format!(
        "  B. Type Inference:     {}/25 ({:.1}%)\n",
        b_total,
        (b_total as f32 / 25.0) * 100.0
    ));
    output.push_str(&format!(
        "  C. Test Coverage:      {}/15 ({:.1}%)\n",
        c_total,
        (c_total as f32 / 15.0) * 100.0
    ));
    output.push_str(&format!(
        "  D. Code Quality:       {}/10 ({:.1}%)\n",
        d_total,
        (d_total as f32 / 10.0) * 100.0
    ));
    output.push_str(&format!(
        "  E. Semantic Equiv:     {}/10 ({:.1}%)\n",
        e_total,
        (e_total as f32 / 10.0) * 100.0
    ));

    output.push('\n');

    // Top blockers
    if !report.top_blockers.is_empty() {
        output.push_str("Top Blockers (Pareto):\n");
        for (i, blocker) in report.top_blockers.iter().enumerate() {
            output.push_str(&format!(
                "  {}. {} ({} files, -{:.1} pts avg)\n",
                i + 1,
                blocker.pattern,
                blocker.affected_files,
                blocker.avg_points_lost
            ));
        }
        output.push('\n');
    }

    // Summary
    output.push_str(&format!("Files scored: {}\n", report.results.len()));

    let gateway_passed = report
        .results
        .iter()
        .filter(|r| r.score.gateway_passed)
        .count();
    output.push_str(&format!(
        "Gateway passed: {}/{} ({:.1}%)\n",
        gateway_passed,
        report.results.len(),
        (gateway_passed as f32 / report.results.len().max(1) as f32) * 100.0
    ));

    output
}

/// Format score report as JSON
fn format_json(report: &CorpusScoreReport) -> Result<String> {
    let json = serde_json::json!({
        "aggregate_score": report.aggregate_score,
        "grade": report.grade.as_str(),
        "files_scored": report.results.len(),
        "gateway_passed": report.results.iter().filter(|r| r.score.gateway_passed).count(),
        "category_averages": {
            "compilation": report.category_averages.a1_parse + report.category_averages.a2_type_check + report.category_averages.a3_cargo_build,
            "type_inference": report.category_averages.b1_no_e0308 + report.category_averages.b2_no_e0599 + report.category_averages.b3_no_e0425,
            "test_coverage": report.category_averages.c1_doctest + report.category_averages.c2_unit_test + report.category_averages.c3_property_test,
            "code_quality": report.category_averages.d1_clippy + report.category_averages.d2_tdg + report.category_averages.d3_complexity,
            "semantic_equivalence": report.category_averages.e1_trace_match + report.category_averages.e2_output_equiv,
        },
        "top_blockers": report.top_blockers.iter().map(|b| {
            serde_json::json!({
                "pattern": b.pattern,
                "affected_files": b.affected_files,
                "avg_points_lost": b.avg_points_lost,
            })
        }).collect::<Vec<_>>(),
    });

    Ok(serde_json::to_string_pretty(&json)?)
}

/// Format score report as Markdown
fn format_markdown(report: &CorpusScoreReport) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "# Single-Shot Compile Score: {} ({:.1}/100)\n\n",
        report.grade.as_str(),
        report.aggregate_score
    ));

    // Category breakdown table
    let ca = &report.category_averages;
    output.push_str("## Category Breakdown\n\n");
    output.push_str("| Category | Score | Max | Percentage |\n");
    output.push_str("|----------|-------|-----|------------|\n");

    let a_total = ca.a1_parse + ca.a2_type_check + ca.a3_cargo_build;
    let b_total = ca.b1_no_e0308 + ca.b2_no_e0599 + ca.b3_no_e0425;
    let c_total = ca.c1_doctest + ca.c2_unit_test + ca.c3_property_test;
    let d_total = ca.d1_clippy + ca.d2_tdg + ca.d3_complexity;
    let e_total = ca.e1_trace_match + ca.e2_output_equiv;

    output.push_str(&format!(
        "| A. Compilation | {} | 40 | {:.1}% |\n",
        a_total,
        (a_total as f32 / 40.0) * 100.0
    ));
    output.push_str(&format!(
        "| B. Type Inference | {} | 25 | {:.1}% |\n",
        b_total,
        (b_total as f32 / 25.0) * 100.0
    ));
    output.push_str(&format!(
        "| C. Test Coverage | {} | 15 | {:.1}% |\n",
        c_total,
        (c_total as f32 / 15.0) * 100.0
    ));
    output.push_str(&format!(
        "| D. Code Quality | {} | 10 | {:.1}% |\n",
        d_total,
        (d_total as f32 / 10.0) * 100.0
    ));
    output.push_str(&format!(
        "| E. Semantic Equiv | {} | 10 | {:.1}% |\n",
        e_total,
        (e_total as f32 / 10.0) * 100.0
    ));

    output.push('\n');

    // Top blockers
    if !report.top_blockers.is_empty() {
        output.push_str("## Top Blockers (Pareto)\n\n");
        output.push_str("| Error | Files | Avg Points Lost |\n");
        output.push_str("|-------|-------|----------------|\n");
        for blocker in &report.top_blockers {
            output.push_str(&format!(
                "| {} | {} | {:.1} |\n",
                blocker.pattern, blocker.affected_files, blocker.avg_points_lost
            ));
        }
        output.push('\n');
    }

    // Summary
    output.push_str("## Summary\n\n");
    output.push_str(&format!("- **Files scored**: {}\n", report.results.len()));

    let gateway_passed = report
        .results
        .iter()
        .filter(|r| r.score.gateway_passed)
        .count();
    output.push_str(&format!(
        "- **Gateway passed**: {}/{} ({:.1}%)\n",
        gateway_passed,
        report.results.len(),
        (gateway_passed as f32 / report.results.len().max(1) as f32) * 100.0
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::scoring::{
        Blocker, CategoryBreakdown, CorpusScoreReport, Grade, SingleShotResult, SingleShotScore,
    };

    #[test]
    fn test_parse_mode_fast() {
        assert!(matches!(parse_mode("fast"), ScoringMode::Fast));
    }

    #[test]
    fn test_parse_mode_quick() {
        assert!(matches!(parse_mode("quick"), ScoringMode::Quick));
    }

    #[test]
    fn test_parse_mode_full() {
        assert!(matches!(parse_mode("full"), ScoringMode::Full));
    }

    #[test]
    fn test_parse_mode_default_is_fast() {
        assert!(matches!(parse_mode("unknown"), ScoringMode::Fast));
        assert!(matches!(parse_mode(""), ScoringMode::Fast));
    }

    #[test]
    fn test_parse_mode_case_insensitive() {
        assert!(matches!(parse_mode("QUICK"), ScoringMode::Quick));
        assert!(matches!(parse_mode("Full"), ScoringMode::Full));
        assert!(matches!(parse_mode("FAST"), ScoringMode::Fast));
    }

    #[test]
    fn test_parse_format_json() {
        assert!(matches!(parse_format("json"), OutputFormat::Json));
    }

    #[test]
    fn test_parse_format_markdown() {
        assert!(matches!(parse_format("markdown"), OutputFormat::Markdown));
    }

    #[test]
    fn test_parse_format_md() {
        assert!(matches!(parse_format("md"), OutputFormat::Markdown));
    }

    #[test]
    fn test_parse_format_human_default() {
        assert!(matches!(parse_format("human"), OutputFormat::Human));
        assert!(matches!(parse_format(""), OutputFormat::Human));
        assert!(matches!(parse_format("text"), OutputFormat::Human));
    }

    #[test]
    fn test_parse_format_case_insensitive() {
        assert!(matches!(parse_format("JSON"), OutputFormat::Json));
        assert!(matches!(parse_format("Markdown"), OutputFormat::Markdown));
    }

    #[test]
    fn test_generate_cargo_toml_empty_code() {
        let toml = generate_cargo_toml_for_code("fn main() {}");
        assert!(toml.contains("[package]"));
        assert!(toml.contains("score_test"));
        assert!(toml.contains("[dependencies]"));
    }

    #[test]
    fn test_generate_cargo_toml_with_serde_json() {
        let code = "use serde_json::Value;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("serde_json"));
    }

    #[test]
    fn test_generate_cargo_toml_with_once_cell() {
        let code = "use once_cell::sync::Lazy;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("once_cell"));
    }

    #[test]
    fn test_generate_cargo_toml_with_clap() {
        let code = "use clap::Parser;\n#[derive(clap::Parser)]";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("clap"));
        assert!(toml.contains("derive"));
    }

    #[test]
    fn test_generate_cargo_toml_with_regex() {
        let code = "use regex::Regex;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("regex"));
    }

    #[test]
    fn test_generate_cargo_toml_with_chrono() {
        let code = "use chrono::NaiveDate;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("chrono"));
    }

    #[test]
    fn test_generate_cargo_toml_with_multiple_deps() {
        let code = "use serde::Serialize;\nuse regex::Regex;\nuse rand::Rng;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("serde"));
        assert!(toml.contains("regex"));
        assert!(toml.contains("rand"));
    }

    #[test]
    fn test_generate_cargo_toml_with_itertools() {
        let code = "use itertools::Itertools;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("itertools"));
    }

    #[test]
    fn test_generate_cargo_toml_with_csv() {
        let code = "use csv::Reader;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("csv"));
    }

    #[test]
    fn test_generate_cargo_toml_with_tempfile() {
        let code = "use tempfile::TempDir;";
        let toml = generate_cargo_toml_for_code(code);
        assert!(toml.contains("tempfile"));
    }

    #[test]
    fn test_parse_cargo_errors_empty() {
        let errors = parse_cargo_errors("");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_cargo_errors_e0308() {
        let output = "error[E0308]: mismatched types";
        let errors = parse_cargo_errors(output);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E0308");
    }

    #[test]
    fn test_parse_cargo_errors_multiple() {
        let output = "error[E0308]: mismatched types\nerror[E0425]: cannot find value";
        let errors = parse_cargo_errors(output);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].code, "E0308");
        assert_eq!(errors[1].code, "E0425");
    }

    #[test]
    fn test_parse_cargo_errors_no_error_lines() {
        let output = "warning: unused variable\nnote: some hint";
        let errors = parse_cargo_errors(output);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_cargo_errors_preserves_message() {
        let output = "error[E0599]: no method named `foo` found for struct `Bar`";
        let errors = parse_cargo_errors(output);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("no method named"));
    }

    fn make_test_breakdown() -> CategoryBreakdown {
        CategoryBreakdown {
            a1_parse: 10,
            a2_type_check: 10,
            a3_cargo_build: 10,
            b1_no_e0308: 8,
            b2_no_e0599: 8,
            b3_no_e0425: 5,
            c1_doctest: 0,
            c2_unit_test: 0,
            c3_property_test: 0,
            d1_clippy: 5,
            d2_tdg: 3,
            d3_complexity: 2,
            e1_trace_match: 5,
            e2_output_equiv: 5,
        }
    }

    fn make_test_report() -> CorpusScoreReport {
        let breakdown = make_test_breakdown();
        CorpusScoreReport {
            aggregate_score: 75.5,
            grade: Grade::B,
            results: vec![SingleShotResult {
                file_path: PathBuf::from("test.py"),
                score: SingleShotScore {
                    total: 75,
                    compilation: 30,
                    type_inference: 21,
                    test_coverage: 0,
                    code_quality: 10,
                    semantic_equivalence: 10,
                    gateway_passed: true,
                    mode: ScoringMode::Fast,
                },
                category_breakdown: breakdown.clone(),
                error_details: vec![],
                transpiler_decisions: vec![],
            }],
            category_averages: breakdown,
            top_blockers: vec![Blocker {
                pattern: "E0308".to_string(),
                affected_files: 5,
                avg_points_lost: 3.2,
            }],
        }
    }

    #[test]
    fn test_format_human_contains_score() {
        let report = make_test_report();
        let output = format_human(&report);
        assert!(output.contains("75.5"));
        assert!(output.contains("Compilation"));
        assert!(output.contains("Type Inference"));
        assert!(output.contains("Test Coverage"));
        assert!(output.contains("Code Quality"));
        assert!(output.contains("Semantic Equiv"));
    }

    #[test]
    fn test_format_human_contains_blockers() {
        let report = make_test_report();
        let output = format_human(&report);
        assert!(output.contains("E0308"));
        assert!(output.contains("5 files"));
    }

    #[test]
    fn test_format_human_contains_summary() {
        let report = make_test_report();
        let output = format_human(&report);
        assert!(output.contains("Files scored: 1"));
        assert!(output.contains("Gateway passed: 1/1"));
    }

    #[test]
    fn test_format_json_valid() {
        let report = make_test_report();
        let json_str = format_json(&report).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["aggregate_score"], 75.5);
        assert_eq!(parsed["grade"], "B");
        assert_eq!(parsed["files_scored"], 1);
    }

    #[test]
    fn test_format_json_contains_categories() {
        let report = make_test_report();
        let json_str = format_json(&report).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed["category_averages"]["compilation"].as_u64().unwrap() > 0);
        assert!(
            parsed["category_averages"]["type_inference"]
                .as_u64()
                .unwrap()
                > 0
        );
    }

    #[test]
    fn test_format_json_contains_blockers() {
        let report = make_test_report();
        let json_str = format_json(&report).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let blockers = parsed["top_blockers"].as_array().unwrap();
        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0]["pattern"], "E0308");
    }

    #[test]
    fn test_format_markdown_contains_table() {
        let report = make_test_report();
        let output = format_markdown(&report);
        assert!(output.contains("| Category |"));
        assert!(output.contains("| A. Compilation |"));
        assert!(output.contains("| B. Type Inference |"));
        assert!(output.contains("| C. Test Coverage |"));
        assert!(output.contains("| D. Code Quality |"));
        assert!(output.contains("| E. Semantic Equiv |"));
    }

    #[test]
    fn test_format_markdown_contains_header() {
        let report = make_test_report();
        let output = format_markdown(&report);
        assert!(output.contains("# Single-Shot Compile Score:"));
        assert!(output.contains("75.5"));
    }

    #[test]
    fn test_format_markdown_contains_blockers_section() {
        let report = make_test_report();
        let output = format_markdown(&report);
        assert!(output.contains("## Top Blockers"));
        assert!(output.contains("E0308"));
    }

    #[test]
    fn test_format_markdown_contains_summary() {
        let report = make_test_report();
        let output = format_markdown(&report);
        assert!(output.contains("## Summary"));
        assert!(output.contains("**Files scored**: 1"));
    }

    #[test]
    fn test_format_human_no_blockers() {
        let mut report = make_test_report();
        report.top_blockers.clear();
        let output = format_human(&report);
        assert!(!output.contains("Top Blockers"));
    }

    #[test]
    fn test_format_markdown_no_blockers() {
        let mut report = make_test_report();
        report.top_blockers.clear();
        let output = format_markdown(&report);
        assert!(!output.contains("## Top Blockers"));
    }
}

/// Handle the score command
pub fn handle_score_command(args: ScoreArgs) -> Result<()> {
    let mode = parse_mode(&args.mode);
    let format = parse_format(&args.format);

    let config = ScoringConfig {
        enable_semantic_check: args.semantic,
        oracle_feedback: args.oracle_feedback,
        ..Default::default()
    };

    let calculator = ScoreCalculator::with_config(config);

    // Find all Python files
    let python_files: Vec<PathBuf> = walkdir::WalkDir::new(&args.input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "py"))
        .filter(|e| !e.path().to_string_lossy().contains("__pycache__"))
        .map(|e| e.path().to_path_buf())
        .collect();

    if python_files.is_empty() {
        println!("⚠️  No Python files found in {}", args.input_dir.display());
        return Ok(());
    }

    println!(
        "🔍 Scoring {} Python files (mode: {:?})...\n",
        python_files.len(),
        mode
    );

    // Score each file
    let mut results = vec![];
    for (i, path) in python_files.iter().enumerate() {
        if i % 10 == 0 {
            eprint!(
                "\r[{}/{}] Scoring... {:.0}%",
                i,
                python_files.len(),
                (i as f32 / python_files.len() as f32) * 100.0
            );
        }

        match score_file(path, mode, &calculator, args.semantic) {
            Ok(result) => results.push(result),
            Err(e) => {
                eprintln!("\n⚠️  Error scoring {}: {}", path.display(), e);
            }
        }
    }
    eprintln!(
        "\r[{}/{}] Scoring... 100%",
        python_files.len(),
        python_files.len()
    );
    println!();

    // Aggregate results
    let report = calculator.aggregate(&results);

    // Format output
    let output_text = match format {
        OutputFormat::Json => format_json(&report)?,
        OutputFormat::Markdown => format_markdown(&report),
        _ => format_human(&report),
    };

    // Write output
    if let Some(output_path) = args.output {
        std::fs::write(&output_path, &output_text)?;
        println!("📝 Report written to {}", output_path.display());
    } else {
        println!("{}", output_text);
    }

    // Check threshold
    if report.aggregate_score < args.min_score as f32 {
        anyhow::bail!(
            "Score {:.1} below minimum threshold {}",
            report.aggregate_score,
            args.min_score
        );
    }

    Ok(())
}
