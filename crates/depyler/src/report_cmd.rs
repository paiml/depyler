//! Corpus Analysis Report Command
//!
//! Implements `depyler report` - deterministic scientific corpus analysis
//! following Toyota Way methodology (Jidoka, Genchi Genbutsu, Kaizen, 5S).
//!
//! # Usage
//! ```bash
//! depyler report                          # Default corpus (reprorusted-python-cli)
//! depyler report -c /path/to/corpus       # Custom corpus
//! depyler report -f markdown -o ./reports # Markdown output
//! ```

use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

/// Report command arguments
pub struct ReportArgs {
    pub corpus: Option<PathBuf>,
    pub format: String,
    pub output: Option<PathBuf>,
    pub skip_clean: bool,
    pub target_rate: f64,
}

/// Compilation result for a single file
#[derive(Debug)]
struct CompileResult {
    name: String,
    success: bool,
    error_code: Option<String>,
    error_message: Option<String>,
}

/// Error taxonomy entry
#[derive(Debug, Default)]
struct ErrorTaxonomy {
    count: usize,
    samples: Vec<String>,
}

/// Handle the report command
pub fn handle_report_command(args: ReportArgs) -> Result<()> {
    let corpus_path = args.corpus.unwrap_or_else(default_corpus_path);

    println!(
        "{} Depyler Corpus Analysis Report",
        "=".repeat(50).blue().bold()
    );
    println!("{} {}", "Corpus:".cyan(), corpus_path.display());
    println!("{} {:.0}%", "Target:".cyan(), args.target_rate * 100.0);
    println!();

    // Phase 1: Clean artifacts (5S)
    if !args.skip_clean {
        phase_clean(&corpus_path)?;
    }

    // Phase 2: Transpile + Compile Python to Rust (DEPYLER-0723)
    // This is now the main phase - compile command does transpile + build in one step
    let results = phase_transpile(&corpus_path)?;

    // Phase 4: Analyze errors
    let (pass, fail, taxonomy) = analyze_results(&results);
    let total = pass + fail;
    let rate = if total > 0 {
        (pass as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // Phase 5: Generate report
    println!();
    println!(
        "{} Report Generation",
        "=".repeat(50).blue().bold()
    );

    match args.format.as_str() {
        "json" => print_json_report(total, pass, fail, rate, &taxonomy, args.target_rate)?,
        "markdown" => {
            print_markdown_report(total, pass, fail, rate, &taxonomy, args.target_rate, &args.output)?
        }
        _ => print_terminal_report(total, pass, fail, rate, &taxonomy, args.target_rate),
    }

    // Return success/failure based on target rate
    if rate >= args.target_rate * 100.0 {
        println!(
            "\n{} Target rate achieved! ({:.1}% >= {:.1}%)",
            "SUCCESS".green().bold(),
            rate,
            args.target_rate * 100.0
        );
        Ok(())
    } else {
        println!(
            "\n{} Target rate NOT achieved ({:.1}% < {:.1}%)",
            "WARNING".yellow().bold(),
            rate,
            args.target_rate * 100.0
        );
        Ok(()) // Don't fail - just warn
    }
}

/// Find default corpus path
fn default_corpus_path() -> PathBuf {
    let candidates = [
        PathBuf::from("/home/noah/src/reprorusted-python-cli/examples"),
        PathBuf::from("../reprorusted-python-cli/examples"),
        PathBuf::from("./examples"),
    ];

    for path in &candidates {
        if path.exists() {
            return path.clone();
        }
    }

    PathBuf::from(".")
}

/// Phase 1: Clean artifacts (5S methodology)
fn phase_clean(corpus_path: &PathBuf) -> Result<()> {
    println!("{} Phase 1: Artifact Cleaning (5S)", "->".cyan());

    let mut rs_removed = 0;
    let mut cargo_removed = 0;
    let mut target_removed = 0;

    // Find and remove .rs files (transpiled output)
    for entry in WalkDir::new(corpus_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "rs") {
            if std::fs::remove_file(path).is_ok() {
                rs_removed += 1;
            }
        } else if path.file_name().is_some_and(|n| n == "Cargo.toml") {
            // Don't remove Cargo.toml in the root
            if path.parent() != Some(corpus_path.as_path()) {
                if std::fs::remove_file(path).is_ok() {
                    cargo_removed += 1;
                }
            }
        } else if path.file_name().is_some_and(|n| n == "Cargo.lock") {
            if std::fs::remove_file(path).is_ok() {
                // Count with Cargo.toml
            }
        } else if path.is_dir() && path.file_name().is_some_and(|n| n == "target") {
            if std::fs::remove_dir_all(path).is_ok() {
                target_removed += 1;
            }
        }
    }

    println!(
        "   {} Cleaned: {} .rs, {} Cargo.toml, {} target/",
        "✓".green(),
        rs_removed,
        cargo_removed,
        target_removed
    );

    Ok(())
}

/// Phase 2: Transpile and compile Python files to Rust (DEPYLER-0723)
/// Returns compile results with error details for taxonomy
fn phase_transpile(corpus_path: &PathBuf) -> Result<Vec<CompileResult>> {
    println!("{} Phase 2: Transpilation + Compilation", "->".cyan());

    // Find all Python files (excluding test files and __pycache__)
    let py_files: Vec<PathBuf> = WalkDir::new(corpus_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.extension().is_some_and(|ext| ext == "py")
                && !path.to_string_lossy().contains("__pycache__")
                && !path.file_name().is_some_and(|n| n.to_string_lossy().starts_with("test_"))
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if py_files.is_empty() {
        println!("   {} No Python files found in corpus", "!".yellow());
        return Ok(vec![]);
    }

    let pb = ProgressBar::new(py_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut results = Vec::new();

    // Get the path to the current depyler binary
    let depyler_bin = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("depyler"));

    for py_file in &py_files {
        let name = py_file.file_stem().unwrap_or_default().to_string_lossy().to_string();
        pb.set_message(name.clone());

        // Run depyler compile on the Python file (transpile + build)
        let output = Command::new(&depyler_bin)
            .arg("compile")
            .arg(py_file)
            .output();

        let result = match output {
            Ok(out) if out.status.success() => CompileResult {
                name,
                success: true,
                error_code: None,
                error_message: None,
            },
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let (code, msg) = extract_error(&stderr);
                CompileResult {
                    name,
                    success: false,
                    error_code: Some(code),
                    error_message: Some(msg),
                }
            }
            Err(e) => CompileResult {
                name,
                success: false,
                error_code: Some("EXEC".to_string()),
                error_message: Some(e.to_string()),
            },
        };

        results.push(result);
        pb.inc(1);
    }

    pb.finish_and_clear();

    let pass = results.iter().filter(|r| r.success).count();
    let fail = results.len() - pass;
    println!(
        "   {} Processed: {} pass, {} fail",
        "✓".green(),
        pass,
        fail
    );

    Ok(results)
}

/// Extract error code and message from cargo output
fn extract_error(stderr: &str) -> (String, String) {
    // Strip ANSI escape codes for cleaner output
    let strip_ansi = |s: &str| -> String {
        let mut result = String::new();
        let mut in_escape = false;
        for c in s.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape {
                if c == 'm' {
                    in_escape = false;
                }
            } else {
                result.push(c);
            }
        }
        result
    };

    // Look for error[EXXXX] pattern
    for line in stderr.lines() {
        if let Some(start) = line.find("error[E") {
            if let Some(end) = line[start..].find(']') {
                let code = &line[start + 6..start + end];
                let msg = strip_ansi(line[start + end + 1..].trim());
                return (code.to_string(), msg);
            }
        }
    }

    // Fallback to first error line
    for line in stderr.lines() {
        if line.starts_with("error") {
            return ("UNKNOWN".to_string(), strip_ansi(line));
        }
    }

    ("UNKNOWN".to_string(), "Unknown error".to_string())
}

/// Analyze compilation results
fn analyze_results(results: &[CompileResult]) -> (usize, usize, HashMap<String, ErrorTaxonomy>) {
    let mut pass = 0;
    let mut fail = 0;
    let mut taxonomy: HashMap<String, ErrorTaxonomy> = HashMap::new();

    for result in results {
        if result.success {
            pass += 1;
        } else {
            fail += 1;

            if let Some(code) = &result.error_code {
                let entry = taxonomy.entry(code.clone()).or_default();
                entry.count += 1;
                if entry.samples.len() < 3 {
                    let sample = format!(
                        "{}: {}",
                        result.name,
                        result.error_message.as_deref().unwrap_or("?")
                    );
                    entry.samples.push(sample);
                }
            }
        }
    }

    (pass, fail, taxonomy)
}

/// Print terminal report
fn print_terminal_report(
    total: usize,
    pass: usize,
    fail: usize,
    rate: f64,
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    _target: f64,
) {
    println!();
    println!("{}", "Executive Summary".bold().underline());
    println!("  Total Projects:     {}", total);
    println!("  Compiles (PASS):    {}", pass.to_string().green());
    println!("  Fails:              {}", fail.to_string().red());
    println!(
        "  Single-Shot Rate:   {}",
        format!("{:.1}%", rate).cyan().bold()
    );

    // Andon status
    println!();
    let andon = if rate >= 80.0 {
        "GREEN".green().bold()
    } else if rate >= 50.0 {
        "YELLOW".yellow().bold()
    } else {
        "RED".red().bold()
    };
    println!("{} Andon Status: {}", "⚑".cyan(), andon);

    // Error taxonomy
    if !taxonomy.is_empty() {
        println!();
        println!("{}", "Error Taxonomy (Prioritized)".bold().underline());

        let mut sorted: Vec<_> = taxonomy.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        for (code, entry) in sorted.iter().take(10) {
            let impact = (entry.count as f64 / fail as f64) * 100.0;
            let desc = error_description(code);

            let priority = if entry.count >= 20 {
                "P0-CRITICAL".red().bold()
            } else if entry.count >= 10 {
                "P1-HIGH".yellow().bold()
            } else if entry.count >= 5 {
                "P2-MEDIUM".cyan()
            } else {
                "P3-LOW".white()
            };

            println!(
                "  {} {} ({}) - {:.0}% - {}",
                priority,
                code.yellow(),
                entry.count,
                impact,
                desc
            );
        }
    }

    // Actionable recommendations
    println!();
    println!("{}", "Actionable Fix Items".bold().underline());

    let mut sorted: Vec<_> = taxonomy.iter().collect();
    sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    for (i, (code, entry)) in sorted.iter().take(3).enumerate() {
        println!(
            "  {}. Fix {} ({} occurrences)",
            i + 1,
            code.yellow().bold(),
            entry.count
        );
        if let Some(sample) = entry.samples.first() {
            println!("     Sample: {}", sample.dimmed());
        }
        println!("     Action: {}", fix_recommendation(code));
        println!();
    }
}

/// Print JSON report
fn print_json_report(
    total: usize,
    pass: usize,
    fail: usize,
    rate: f64,
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    target: f64,
) -> Result<()> {
    let andon = if rate >= 80.0 {
        "GREEN"
    } else if rate >= 50.0 {
        "YELLOW"
    } else {
        "RED"
    };

    let errors: Vec<_> = taxonomy
        .iter()
        .map(|(code, entry)| {
            serde_json::json!({
                "code": code,
                "count": entry.count,
                "impact_pct": (entry.count as f64 / fail.max(1) as f64) * 100.0,
                "description": error_description(code),
                "samples": entry.samples,
            })
        })
        .collect();

    let report = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total": total,
            "pass": pass,
            "fail": fail,
            "rate_pct": rate,
            "target_pct": target * 100.0,
            "andon": andon,
        },
        "errors": errors,
    });

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

/// Print markdown report
fn print_markdown_report(
    total: usize,
    pass: usize,
    fail: usize,
    rate: f64,
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    target: f64,
    output: &Option<PathBuf>,
) -> Result<()> {
    let timestamp = chrono::Utc::now().to_rfc3339();

    let andon = if rate >= 80.0 {
        "**GREEN** - Target met (>= 80%)"
    } else if rate >= 50.0 {
        "**YELLOW** - Below target (50-80%)"
    } else {
        "**RED** - Critical (< 50%)"
    };

    let mut report = String::new();
    report.push_str("# Depyler Corpus Analysis Report\n\n");
    report.push_str(&format!("**Generated**: {}\n\n", timestamp));

    report.push_str("## Executive Summary\n\n");
    report.push_str("| Metric | Value |\n");
    report.push_str("|--------|-------|\n");
    report.push_str(&format!("| Total Projects | {} |\n", total));
    report.push_str(&format!("| Compiles (PASS) | {} |\n", pass));
    report.push_str(&format!("| Fails | {} |\n", fail));
    report.push_str(&format!("| **Single-Shot Rate** | **{:.1}%** |\n", rate));
    report.push_str(&format!("| Target Rate | {:.1}% |\n\n", target * 100.0));

    report.push_str("## Andon Status\n\n");
    report.push_str(&format!("{}\n\n", andon));

    if !taxonomy.is_empty() {
        report.push_str("## Error Taxonomy (Prioritized)\n\n");
        report.push_str("| Priority | Error | Count | Impact | Description |\n");
        report.push_str("|----------|-------|-------|--------|-------------|\n");

        let mut sorted: Vec<_> = taxonomy.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        for (code, entry) in sorted.iter().take(15) {
            let impact = (entry.count as f64 / fail.max(1) as f64) * 100.0;
            let priority = if entry.count >= 20 {
                "P0-CRITICAL"
            } else if entry.count >= 10 {
                "P1-HIGH"
            } else if entry.count >= 5 {
                "P2-MEDIUM"
            } else {
                "P3-LOW"
            };

            report.push_str(&format!(
                "| {} | {} | {} | {:.0}% | {} |\n",
                priority,
                code,
                entry.count,
                impact,
                error_description(code)
            ));
        }

        report.push_str("\n## Actionable Fix Items\n\n");

        for (i, (code, entry)) in sorted.iter().take(3).enumerate() {
            report.push_str(&format!(
                "### {}. Fix {} ({} occurrences)\n\n",
                i + 1,
                code,
                entry.count
            ));
            if let Some(sample) = entry.samples.first() {
                report.push_str(&format!("**Sample**: `{}`\n\n", sample));
            }
            report.push_str(&format!("**Root Cause**: {}\n\n", error_description(code)));
            report.push_str(&format!("**Action**: {}\n\n", fix_recommendation(code)));
        }
    }

    report.push_str("\n---\n*Generated by depyler report*\n");

    // Output
    if let Some(out_dir) = output {
        std::fs::create_dir_all(out_dir)?;
        let file_path = out_dir.join(format!(
            "corpus-report-{}.md",
            chrono::Utc::now().format("%Y%m%d-%H%M%S")
        ));
        std::fs::write(&file_path, &report)
            .context(format!("Failed to write report to {}", file_path.display()))?;
        println!("Report saved to: {}", file_path.display());
    } else {
        println!("{}", report);
    }

    Ok(())
}

/// Get error description
fn error_description(code: &str) -> &'static str {
    match code {
        "E0425" => "Cannot find value in scope (undefined variable/function)",
        "E0412" => "Cannot find type in scope (missing generic/type)",
        "E0308" => "Mismatched types (type inference failure)",
        "E0277" => "Trait not implemented (missing impl)",
        "E0432" => "Unresolved import (missing crate/module)",
        "E0599" => "Method not found (wrong type/missing impl)",
        "E0433" => "Failed to resolve (unresolved module path)",
        "E0423" => "Expected value, found type (usage error)",
        "E0369" => "Binary operation not supported between types",
        "E0255" => "Name already defined in this scope",
        "E0618" => "Expected function, found different type",
        "E0609" => "No field on type (struct field access)",
        "E0601" => "main function not found",
        "E0573" => "Expected type, found something else",
        "E0666" => "Ambiguous use of lifetime parameter",
        _ => "See `rustc --explain` for details",
    }
}

/// Get fix recommendation
fn fix_recommendation(code: &str) -> &'static str {
    match code {
        "E0425" => "Update codegen.rs to properly declare variables before use",
        "E0412" => "Add generic parameter detection in type_inference.rs",
        "E0308" => "Standardize numeric types in rust_type_mapper.rs",
        "E0277" => "Add missing trait implementations or bounds",
        "E0432" => "Fix import resolution in module_mapper.rs",
        "E0599" => "Check method resolution and trait bounds",
        "E0433" => "Update module path resolution",
        "E0423" => "Fix value/type confusion in codegen",
        "E0369" => "Add operator overloading or type coercion",
        _ => "Investigate error pattern and update transpiler",
    }
}
