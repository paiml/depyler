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
        doctest_pass: false,       // would need to run doctests
        unit_test_pass: false,     // would need to run tests
        property_test_pass: false, // would need to run property tests
        clippy_clean: false,       // would need to run clippy
        tdg_grade_b_or_better: false, // would need PMAT
        complexity_ok: true,       // assume OK for now
        trace_match: false,        // requires Renacer
        output_equiv: false,       // requires running both
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

    let gateway_passed = report.results.iter().filter(|r| r.score.gateway_passed).count();
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

    let gateway_passed = report.results.iter().filter(|r| r.score.gateway_passed).count();
    output.push_str(&format!(
        "- **Gateway passed**: {}/{} ({:.1}%)\n",
        gateway_passed,
        report.results.len(),
        (gateway_passed as f32 / report.results.len().max(1) as f32) * 100.0
    ));

    output
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
    eprintln!("\r[{}/{}] Scoring... 100%", python_files.len(), python_files.len());
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
