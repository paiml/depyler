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
//! depyler report --filter "argparse"      # Filter by pattern (DEPYLER-BISECT-001)
//! depyler report --tag Dict --limit 50    # Filter by semantic tag
//! depyler report --bisect                 # Bisect to find minimal failure
//! ```

pub mod filter;

use anyhow::{Context, Result};
use colored::Colorize;
use depyler_corpus::{ClusterAnalyzer, GraphAnalyzer, PythonDomain, SemanticClassifier};
use filter::{filter_files, BisectionState, FilterConfig};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

/// Report command arguments (DEPYLER-BISECT-001: Extended with filtering/bisection)
pub struct ReportArgs {
    pub corpus: Option<PathBuf>,
    pub format: String,
    pub output: Option<PathBuf>,
    pub skip_clean: bool,
    pub target_rate: f64,
    /// Filter by regex/glob pattern (DEPYLER-BISECT-001)
    pub filter: Option<String>,
    /// Filter by semantic tag (Dict, List, argparse, etc.)
    pub tag: Option<String>,
    /// Limit number of files to process
    pub limit: Option<usize>,
    /// Random sample size
    pub sample: Option<usize>,
    /// Enable bisection mode to find minimal failing set
    pub bisect: bool,
    /// Stop on first failure
    pub fail_fast: bool,
}

/// Compilation result for a single file
#[derive(Debug)]
struct CompileResult {
    name: String,
    success: bool,
    error_code: Option<String>,
    error_message: Option<String>,
    python_source: Option<String>,
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

    // Build filter config from args (DEPYLER-BISECT-001)
    let filter_config = FilterConfig {
        pattern: args.filter.clone(),
        tags: args.tag.iter().cloned().collect(),
        limit: args.limit,
        sample: args.sample,
        fail_fast: args.fail_fast,
    };

    let has_filters = filter_config.pattern.is_some()
        || !filter_config.tags.is_empty()
        || filter_config.limit.is_some()
        || filter_config.sample.is_some();

    println!(
        "{} Depyler Corpus Analysis Report",
        "=".repeat(50).blue().bold()
    );
    println!("{} {}", "Corpus:".cyan(), corpus_path.display());
    println!("{} {:.0}%", "Target:".cyan(), args.target_rate * 100.0);

    // Display active filters (DEPYLER-BISECT-001)
    if has_filters {
        println!("{}", "Active Filters:".cyan());
        if let Some(ref pattern) = filter_config.pattern {
            println!("  {} {}", "Pattern:".dimmed(), pattern);
        }
        if !filter_config.tags.is_empty() {
            println!("  {} {:?}", "Tags:".dimmed(), filter_config.tags);
        }
        if let Some(limit) = filter_config.limit {
            println!("  {} {}", "Limit:".dimmed(), limit);
        }
        if let Some(sample) = filter_config.sample {
            println!("  {} {}", "Sample:".dimmed(), sample);
        }
    }
    if args.bisect {
        println!("{} {}", "Mode:".cyan(), "Bisection (Delta Debugging)".yellow());
    }
    println!();

    // Phase 1: Clean artifacts (5S)
    if !args.skip_clean {
        phase_clean(&corpus_path)?;
    }

    // Handle bisection mode (DEPYLER-BISECT-001)
    if args.bisect {
        return handle_bisection(&corpus_path, &filter_config, args.target_rate);
    }

    // Phase 2: Transpile + Compile Python to Rust (DEPYLER-0723)
    // This is now the main phase - compile command does transpile + build in one step
    let results = phase_transpile(&corpus_path, &filter_config, args.fail_fast)?;

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
        "json" => print_json_report(total, pass, fail, rate, &taxonomy, args.target_rate, &results)?,
        "markdown" => {
            print_markdown_report(total, pass, fail, rate, &taxonomy, args.target_rate, &args.output)?
        }
        "rich" => print_rich_report(&results, &taxonomy, args.target_rate)?,
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

/// Handle bisection mode (DEPYLER-BISECT-001)
/// Uses Delta Debugging algorithm to isolate minimal failing set.
/// Reference: Zeller & Hildebrandt (2002) IEEE TSE DOI:10.1109/32.988498
fn handle_bisection(
    corpus_path: &PathBuf,
    filter_config: &FilterConfig,
    _target_rate: f64,
) -> Result<()> {
    println!("{} Bisection Mode: Delta Debugging", "->".cyan());
    println!("   Finding minimal failing set using binary search...");
    println!();

    // Find all Python files
    let all_py_files: Vec<PathBuf> = WalkDir::new(corpus_path)
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

    // Apply initial filters
    let py_files = filter_files(&all_py_files, filter_config);

    if py_files.is_empty() {
        println!("   {} No files to bisect", "!".yellow());
        return Ok(());
    }

    println!("   {} Starting with {} files", "◈".cyan(), py_files.len());

    let mut state = BisectionState::new(py_files);
    let depyler_bin = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("depyler"));

    while !state.is_complete() {
        let test_set = state.current_test_set();
        println!(
            "   {} Iteration {}: Testing {} files (range {}-{})",
            "▶".blue(),
            state.iteration + 1,
            test_set.len(),
            state.low,
            state.high
        );

        // Run compilation on test set
        let mut has_failure = false;
        for py_file in &test_set {
            let output = Command::new(&depyler_bin)
                .arg("compile")
                .arg(py_file)
                .output();

            if let Ok(out) = output {
                if !out.status.success() {
                    has_failure = true;
                    break; // Found failure in this half
                }
            } else {
                has_failure = true;
                break;
            }
        }

        state.advance(has_failure);
    }

    // Report result
    println!();
    if let Some(failing_files) = state.get_result() {
        println!(
            "{} Bisection Complete",
            "=".repeat(50).green().bold()
        );
        println!(
            "   Isolated {} failing file(s) in {} iterations:",
            failing_files.len(),
            state.iteration
        );
        for file in failing_files {
            println!("   {} {}", "→".red(), file.display());

            // Run compile to get error details
            let output = Command::new(&depyler_bin)
                .arg("compile")
                .arg(file)
                .output();

            if let Ok(out) = output {
                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let (code, msg) = extract_error(&stderr);
                    println!("     {} {} - {}", "Error:".red(), code, msg);
                }
            }
        }

        println!();
        println!(
            "   {} O(log n) complexity: {} iterations for {} files",
            "✓".green(),
            state.iteration,
            state.files.len()
        );
    } else {
        println!(
            "{} Bisection did not converge",
            "WARNING".yellow().bold()
        );
        println!(
            "   Max iterations ({}) reached without isolating failure",
            state.max_iterations
        );
    }

    Ok(())
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
            if path.parent() != Some(corpus_path.as_path())
                && std::fs::remove_file(path).is_ok()
            {
                cargo_removed += 1;
            }
        } else if path.file_name().is_some_and(|n| n == "Cargo.lock") {
            if std::fs::remove_file(path).is_ok() {
                // Count with Cargo.toml
            }
        } else if path.is_dir()
            && path.file_name().is_some_and(|n| n == "target")
            && std::fs::remove_dir_all(path).is_ok()
        {
            target_removed += 1;
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
/// DEPYLER-BISECT-001: Added filter_config and fail_fast parameters
fn phase_transpile(
    corpus_path: &PathBuf,
    filter_config: &FilterConfig,
    fail_fast: bool,
) -> Result<Vec<CompileResult>> {
    println!("{} Phase 2: Transpilation + Compilation", "->".cyan());

    // Find all Python files (excluding test files and __pycache__)
    let all_py_files: Vec<PathBuf> = WalkDir::new(corpus_path)
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

    // Apply filters (DEPYLER-BISECT-001)
    let py_files = filter_files(&all_py_files, filter_config);

    let filtered_count = all_py_files.len() - py_files.len();
    if filtered_count > 0 {
        println!(
            "   {} Filtered: {} files → {} files ({} excluded)",
            "◈".cyan(),
            all_py_files.len(),
            py_files.len(),
            filtered_count
        );
    }

    if py_files.is_empty() {
        println!("   {} No Python files found in corpus (after filtering)", "!".yellow());
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

        // Read Python source for semantic classification
        let python_source = std::fs::read_to_string(py_file).ok();

        let result = match output {
            Ok(out) if out.status.success() => CompileResult {
                name,
                success: true,
                error_code: None,
                error_message: None,
                python_source,
            },
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let (code, msg) = extract_error(&stderr);
                CompileResult {
                    name,
                    success: false,
                    error_code: Some(code),
                    error_message: Some(msg),
                    python_source,
                }
            }
            Err(e) => CompileResult {
                name,
                success: false,
                error_code: Some("EXEC".to_string()),
                error_message: Some(e.to_string()),
                python_source,
            },
        };

        let is_failure = !result.success;
        results.push(result);
        pb.inc(1);

        // DEPYLER-BISECT-001: Support fail-fast mode
        if fail_fast && is_failure {
            pb.finish_with_message("Stopped (fail-fast)");
            println!(
                "   {} Fail-fast triggered after {} files",
                "⚠".yellow(),
                results.len()
            );
            break;
        }
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

    // Check for transpiler errors (anyhow style: "Error: ...")
    // DEPYLER-0764: Handle capital "Error:" from depyler transpiler failures
    for line in stderr.lines() {
        if line.starts_with("Error: Failed to transpile") {
            // Extract the root cause from "Caused by:" if available
            for cause_line in stderr.lines() {
                if cause_line.trim().starts_with("Expression type not yet supported") {
                    return ("TRANSPILE".to_string(), strip_ansi(cause_line.trim()));
                }
                if cause_line.trim().starts_with("Unsupported") {
                    return ("TRANSPILE".to_string(), strip_ansi(cause_line.trim()));
                }
            }
            return ("TRANSPILE".to_string(), strip_ansi(line));
        }
        if line.starts_with("Error:") {
            return ("DEPYLER".to_string(), strip_ansi(line));
        }
    }

    // Fallback to first error line (case-insensitive)
    for line in stderr.lines() {
        let lower = line.to_lowercase();
        if lower.starts_with("error") {
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

/// Print JSON report with semantic classification
fn print_json_report(
    total: usize,
    pass: usize,
    fail: usize,
    rate: f64,
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    target: f64,
    results: &[CompileResult],
) -> Result<()> {
    let andon = if rate >= 80.0 {
        "GREEN"
    } else if rate >= 50.0 {
        "YELLOW"
    } else {
        "RED"
    };

    // Semantic Classification
    let semantic_classifier = SemanticClassifier::new();
    let files_for_classification: Vec<(String, String, bool)> = results
        .iter()
        .filter_map(|r| {
            r.python_source.as_ref().map(|src| {
                (r.name.clone(), src.clone(), r.success)
            })
        })
        .collect();
    let semantic = semantic_classifier.classify_corpus(&files_for_classification);

    // Build semantic classification JSON
    let semantic_json: serde_json::Value = {
        let by_domain: Vec<_> = semantic
            .by_domain
            .iter()
            .map(|(domain, stats)| {
                let domain_name = match domain {
                    PythonDomain::Core => "core",
                    PythonDomain::Stdlib => "stdlib",
                    PythonDomain::External => "external",
                };
                serde_json::json!({
                    "domain": domain_name,
                    "total": stats.total,
                    "passed": stats.passed,
                    "pass_rate": stats.pass_rate,
                })
            })
            .collect();
        serde_json::json!({
            "domains": by_domain,
            "confidence": semantic.confidence,
        })
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
        "semantic_classification": semantic_json,
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
        // DEPYLER-0764: New transpiler-specific error codes
        "TRANSPILE" => "Unsupported Python expression/statement (transpiler limitation)",
        "DEPYLER" => "General transpiler error (input/output issue)",
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
        // DEPYLER-0764: New transpiler-specific error codes
        "TRANSPILE" => "Add support for unsupported expression type in rust_gen/expr_gen.rs",
        "DEPYLER" => "Fix general transpiler error (check error message for details)",
        _ => "Investigate error pattern and update transpiler",
    }
}

/// Print rich text report with semantic classification, clustering, and graph analysis.
/// Direct implementation without HtmlReportGenerator for simpler integration (DEPYLER-REPORT-V2).
fn print_rich_report(
    results: &[CompileResult],
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    target: f64,
) -> Result<()> {
    let total = results.len();
    let pass = results.iter().filter(|r| r.success).count();
    let fail = total - pass;
    let rate = if total > 0 {
        (pass as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // 1. Semantic Classification
    let semantic_classifier = SemanticClassifier::new();
    let files_for_classification: Vec<(String, String, bool)> = results
        .iter()
        .filter_map(|r| {
            r.python_source.as_ref().map(|src| {
                (r.name.clone(), src.clone(), r.success)
            })
        })
        .collect();
    let semantic = semantic_classifier.classify_corpus(&files_for_classification);

    // 2. Error Clustering (only on failed files)
    let cluster_analyzer = ClusterAnalyzer::new();
    let error_counts: HashMap<String, usize> = taxonomy
        .iter()
        .map(|(code, entry)| (code.clone(), entry.count))
        .collect();
    let clusters = cluster_analyzer.analyze(&error_counts);

    // 3. Graph Analysis
    let graph_analyzer = GraphAnalyzer::new();
    let co_occurrences = build_co_occurrence_map(results);
    let graph = graph_analyzer.analyze(&error_counts, &co_occurrences);

    // 4. Generate Rich Text Report
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    DEPYLER RICH CORPUS ANALYSIS REPORT                       ║".cyan().bold());
    println!("{}", "╚══════════════════════════════════════════════════════════════════════════════╝".cyan());
    println!();

    // Executive Summary
    println!("{}", "┌─────────────────────────────────────────┐".blue());
    println!("{}", "│         EXECUTIVE SUMMARY               │".blue().bold());
    println!("{}", "└─────────────────────────────────────────┘".blue());
    println!("  Total Files:        {}", total);
    println!("  Compiled (PASS):    {}", pass.to_string().green());
    println!("  Failed:             {}", fail.to_string().red());
    println!("  Single-Shot Rate:   {}", format!("{:.1}%", rate).cyan().bold());
    println!("  Target Rate:        {:.1}%", target * 100.0);
    println!();

    // Andon Status
    let andon = if rate >= 80.0 {
        "GREEN ✓".green().bold()
    } else if rate >= 50.0 {
        "YELLOW ⚠".yellow().bold()
    } else {
        "RED ✗".red().bold()
    };
    println!("  Andon Status:       {}", andon);
    println!();

    // Semantic Classification
    println!("{}", "┌─────────────────────────────────────────┐".blue());
    println!("{}", "│     SEMANTIC CLASSIFICATION (Domain)    │".blue().bold());
    println!("{}", "└─────────────────────────────────────────┘".blue());
    for (domain, stats) in &semantic.by_domain {
        let domain_name = match domain {
            PythonDomain::Core => "Core    ",
            PythonDomain::Stdlib => "Stdlib  ",
            PythonDomain::External => "External",
        };
        let bar = ascii_bar(stats.pass_rate / 100.0, 20);
        println!(
            "  {} │{} {:.1}% ({}/{} passed)",
            domain_name.cyan(),
            bar,
            stats.pass_rate,
            stats.passed,
            stats.total
        );
    }
    println!();

    // Error Clustering
    println!("{}", "┌─────────────────────────────────────────┐".blue());
    println!("{}", "│         ERROR CLUSTERS (K-Means)        │".blue().bold());
    println!("{}", "└─────────────────────────────────────────┘".blue());
    println!("  Optimal K:          {}", clusters.k);
    println!("  Silhouette Score:   {:.3}", clusters.silhouette_score);
    for cluster in &clusters.clusters {
        println!("  Cluster {}: {} errors - {}", cluster.id, cluster.members.len(), cluster.label.cyan());
        for member in cluster.members.iter().take(3) {
            println!("    • {}", member);
        }
        if cluster.members.len() > 3 {
            println!("    ... and {} more", cluster.members.len() - 3);
        }
    }
    println!();

    // Graph Analysis
    println!("{}", "┌─────────────────────────────────────────┐".blue());
    println!("{}", "│        GRAPH ANALYSIS (PageRank)        │".blue().bold());
    println!("{}", "└─────────────────────────────────────────┘".blue());
    println!("  Total Nodes:        {}", graph.nodes.len());
    println!("  Communities:        {}", graph.communities.len());

    // Top errors by PageRank
    let mut sorted_nodes: Vec<_> = graph.nodes.iter().collect();
    sorted_nodes.sort_by(|a, b| b.pagerank.partial_cmp(&a.pagerank).unwrap_or(std::cmp::Ordering::Equal));
    println!("  Top Errors (PageRank):");
    for node in sorted_nodes.iter().take(5) {
        let bar = ascii_bar(node.pagerank, 15);
        println!(
            "    {} │{} PR={:.3}",
            node.code.yellow(),
            bar,
            node.pagerank
        );
    }
    println!();

    // Error Taxonomy (from existing data)
    println!("{}", "┌─────────────────────────────────────────┐".blue());
    println!("{}", "│         ERROR TAXONOMY (Sorted)         │".blue().bold());
    println!("{}", "└─────────────────────────────────────────┘".blue());
    let mut sorted: Vec<_> = taxonomy.iter().collect();
    sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));
    for (code, entry) in sorted.iter().take(10) {
        let impact = (entry.count as f64 / fail.max(1) as f64) * 100.0;
        let bar = ascii_bar(impact / 100.0, 20);
        println!(
            "  {} │{} {:>3} ({:.0}%)",
            code.yellow(),
            bar,
            entry.count,
            impact
        );
    }
    println!();

    println!("{}", "════════════════════════════════════════════════════════════════════════════════".cyan());
    println!("{}", "Report generated by depyler report --format rich".dimmed());

    Ok(())
}

/// Generate ASCII bar chart.
fn ascii_bar(ratio: f64, width: usize) -> String {
    let filled = (ratio.clamp(0.0, 1.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled).green(), "░".repeat(empty).dimmed())
}

/// Build co-occurrence map for graph analysis.
/// Maps (error_code_1, error_code_2) -> count of files with both errors.
fn build_co_occurrence_map(results: &[CompileResult]) -> HashMap<(String, String), usize> {
    use depyler_corpus::graph::extract_co_occurrences;

    // Group errors by file
    let mut file_errors: Vec<(String, Vec<String>)> = Vec::new();
    for r in results {
        if !r.success {
            if let Some(code) = &r.error_code {
                // Find existing entry or create new
                if let Some((_, errors)) = file_errors.iter_mut().find(|(name, _)| name == &r.name) {
                    errors.push(code.clone());
                } else {
                    file_errors.push((r.name.clone(), vec![code.clone()]));
                }
            }
        }
    }

    // Build co-occurrence from file errors
    extract_co_occurrences(&file_errors)
}
