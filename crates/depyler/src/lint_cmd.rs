//! DEPYLER-99MODE: Lint command for Depyler Python compliance (Path A)
//!
//! This module implements `depyler lint --strict` which validates Python code
//! against the "Depyler Python" subset specification.
//!
//! ## Path A Specification
//!
//! Depyler Python enforces the following constraints:
//! - Type annotations REQUIRED on all functions
//! - `eval`, `exec` PROHIBITED (computationally irreducible)
//! - `getattr`, `setattr` PROHIBITED (dynamic dispatch)
//! - Metaclasses PROHIBITED (runtime type manipulation)
//! - Multiple inheritance PROHIBITED (no Rust equivalent)
//! - Monkey patching PROHIBITED (violates static analysis)

use anyhow::Result;
use colored::Colorize;
use depyler_core::DepylerPipeline;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Lint violation severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    /// Error - code will not compile
    Error,
    /// Warning - code may have issues
    Warning,
}

/// A single lint violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Error code (e.g., "DP001")
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Severity level
    pub severity: Severity,
    /// The source line causing the issue
    pub source_line: Option<String>,
}

/// Lint report for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReport {
    /// Path to the file
    pub path: PathBuf,
    /// List of violations found
    pub violations: Vec<Violation>,
    /// Whether the file is compliant
    pub compliant: bool,
}

/// Corpus-level lint report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusReport {
    /// Total files analyzed
    pub total_files: usize,
    /// Files that are compliant
    pub compliant_files: usize,
    /// Compliance percentage
    pub compliance_rate: f64,
    /// Per-file reports
    pub files: Vec<FileReport>,
    /// Violation counts by code
    pub violation_counts: std::collections::HashMap<String, usize>,
}

/// Depyler Python violation codes
pub mod codes {
    /// Missing return type annotation
    pub const DP001: &str = "DP001";
    /// Missing parameter type annotation
    pub const DP002: &str = "DP002";
    /// Prohibited: eval() call
    pub const DP003: &str = "DP003";
    /// Prohibited: exec() call
    pub const DP004: &str = "DP004";
    /// Prohibited: getattr() call
    pub const DP005: &str = "DP005";
    /// Prohibited: setattr() call
    pub const DP006: &str = "DP006";
    /// Prohibited: metaclass usage
    pub const DP007: &str = "DP007";
    /// Prohibited: multiple inheritance
    pub const DP008: &str = "DP008";
    /// Prohibited: __getattr__ definition
    pub const DP009: &str = "DP009";
    /// Prohibited: __setattr__ definition
    pub const DP010: &str = "DP010";
    /// Warning: untyped **kwargs
    pub const DP011: &str = "DP011";
    /// Warning: untyped *args
    pub const DP012: &str = "DP012";
    /// Prohibited: globals() call
    pub const DP013: &str = "DP013";
    /// Prohibited: locals() call
    pub const DP014: &str = "DP014";
    /// Warning: dynamic import
    pub const DP015: &str = "DP015";
}

/// Lint a single Python file for Depyler Python compliance
pub fn lint_file(path: &Path, strict: bool) -> Result<FileReport> {
    let source = fs::read_to_string(path)?;
    let lines: Vec<&str> = source.lines().collect();
    let mut violations = Vec::new();

    // Check for prohibited patterns
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_number = line_num + 1;

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        // DP003: eval() prohibited
        if contains_function_call(trimmed, "eval") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "eval"),
                code: codes::DP003.to_string(),
                message: "'eval' is prohibited in Depyler Python (computationally irreducible)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP004: exec() prohibited
        if contains_function_call(trimmed, "exec") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "exec"),
                code: codes::DP004.to_string(),
                message: "'exec' is prohibited in Depyler Python (computationally irreducible)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP005: getattr() prohibited
        if contains_function_call(trimmed, "getattr") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "getattr"),
                code: codes::DP005.to_string(),
                message: "'getattr' is prohibited in Depyler Python (dynamic dispatch)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP006: setattr() prohibited
        if contains_function_call(trimmed, "setattr") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "setattr"),
                code: codes::DP006.to_string(),
                message: "'setattr' is prohibited in Depyler Python (dynamic dispatch)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP013: globals() prohibited
        if contains_function_call(trimmed, "globals") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "globals"),
                code: codes::DP013.to_string(),
                message: "'globals' is prohibited in Depyler Python (dynamic scope)".to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP014: locals() prohibited
        if contains_function_call(trimmed, "locals") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "locals"),
                code: codes::DP014.to_string(),
                message: "'locals' is prohibited in Depyler Python (dynamic scope)".to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP009/DP010: __getattr__/__setattr__ definitions
        if trimmed.starts_with("def __getattr__") {
            violations.push(Violation {
                line: line_number,
                column: 1,
                code: codes::DP009.to_string(),
                message: "'__getattr__' is prohibited in Depyler Python (dynamic dispatch)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        if trimmed.starts_with("def __setattr__") {
            violations.push(Violation {
                line: line_number,
                column: 1,
                code: codes::DP010.to_string(),
                message: "'__setattr__' is prohibited in Depyler Python (dynamic dispatch)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP007: metaclass usage
        if trimmed.contains("metaclass=") || trimmed.contains("__metaclass__") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "metaclass"),
                code: codes::DP007.to_string(),
                message: "Metaclasses are prohibited in Depyler Python (runtime type manipulation)"
                    .to_string(),
                severity: Severity::Error,
                source_line: Some(line.to_string()),
            });
        }

        // DP015: __import__ dynamic import
        if contains_function_call(trimmed, "__import__") {
            violations.push(Violation {
                line: line_number,
                column: find_pattern_column(line, "__import__"),
                code: codes::DP015.to_string(),
                message: "Dynamic import is discouraged in Depyler Python".to_string(),
                severity: Severity::Warning,
                source_line: Some(line.to_string()),
            });
        }
    }

    // Check for type annotations using AST-level analysis
    if strict {
        check_type_annotations(&source, &mut violations)?;
        check_multiple_inheritance(&source, &mut violations)?;
    }

    let compliant = !violations.iter().any(|v| v.severity == Severity::Error);

    Ok(FileReport {
        path: path.to_path_buf(),
        violations,
        compliant,
    })
}

/// Check if a line contains a function call (not just the word)
fn contains_function_call(line: &str, func: &str) -> bool {
    let pattern = format!("{}(", func);
    // Avoid false positives like "evaluate" for "eval"
    if let Some(pos) = line.find(&pattern) {
        // Check it's not part of a larger identifier
        if pos > 0 {
            let prev_char = line.chars().nth(pos - 1).unwrap_or(' ');
            if prev_char.is_alphanumeric() || prev_char == '_' {
                return false;
            }
        }
        true
    } else {
        false
    }
}

/// Find column position of a pattern in a line
fn find_pattern_column(line: &str, pattern: &str) -> usize {
    line.find(pattern).map(|p| p + 1).unwrap_or(1)
}

/// Check type annotations using AST
fn check_type_annotations(source: &str, violations: &mut Vec<Violation>) -> Result<()> {
    let pipeline = DepylerPipeline::new();

    // Try to parse and check for annotation issues
    if let Ok(hir) = pipeline.parse_to_hir(source) {
        for func in &hir.functions {
            // Check return type - Unknown or Custom("Any") indicates missing annotation
            if is_untyped(&func.ret_type) {
                // Find the line number by searching for the function definition
                let line_num = find_function_line(source, &func.name);
                violations.push(Violation {
                    line: line_num,
                    column: 1,
                    code: codes::DP001.to_string(),
                    message: format!(
                        "Function '{}' missing return type annotation",
                        func.name
                    ),
                    severity: Severity::Error,
                    source_line: get_source_line(source, line_num),
                });
            }

            // Check parameter types
            for param in &func.params {
                if is_untyped(&param.ty) {
                    let line_num = find_function_line(source, &func.name);
                    violations.push(Violation {
                        line: line_num,
                        column: 1,
                        code: codes::DP002.to_string(),
                        message: format!(
                            "Parameter '{}' in function '{}' missing type annotation",
                            param.name, func.name
                        ),
                        severity: Severity::Error,
                        source_line: get_source_line(source, line_num),
                    });
                }
            }
        }
    }

    Ok(())
}

/// Check if a type is untyped (Unknown or Any)
fn is_untyped(ty: &depyler_core::hir::Type) -> bool {
    use depyler_core::hir::Type;
    match ty {
        Type::Unknown => true,
        Type::Custom(s) if s == "Any" => true,
        _ => false,
    }
}

/// Check for multiple inheritance
fn check_multiple_inheritance(source: &str, violations: &mut Vec<Violation>) -> Result<()> {
    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("class ") && trimmed.contains('(') {
            // Extract the inheritance list
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.find(')') {
                    let bases = &trimmed[start + 1..end];
                    // Count base classes (split by comma, excluding type params)
                    let base_count = bases
                        .split(',')
                        .filter(|b| !b.trim().is_empty())
                        .filter(|b| !b.contains('[')) // Skip Generic[T] etc.
                        .count();

                    if base_count > 1 {
                        violations.push(Violation {
                            line: line_num + 1,
                            column: 1,
                            code: codes::DP008.to_string(),
                            message: "Multiple inheritance is prohibited in Depyler Python"
                                .to_string(),
                            severity: Severity::Error,
                            source_line: Some(line.to_string()),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}

/// Find the line number of a function definition
fn find_function_line(source: &str, func_name: &str) -> usize {
    let pattern = format!("def {}(", func_name);
    for (i, line) in source.lines().enumerate() {
        if line.contains(&pattern) {
            return i + 1;
        }
    }
    1
}

/// Get a specific line from source
fn get_source_line(source: &str, line_num: usize) -> Option<String> {
    source.lines().nth(line_num.saturating_sub(1)).map(String::from)
}

/// Lint a directory of Python files
pub fn lint_corpus(path: &Path, strict: bool) -> Result<CorpusReport> {
    let mut files = Vec::new();
    let mut violation_counts = std::collections::HashMap::new();

    // Collect Python files
    let py_files: Vec<PathBuf> = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "py").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    let total_files = py_files.len();

    for py_file in py_files {
        if let Ok(report) = lint_file(&py_file, strict) {
            for v in &report.violations {
                *violation_counts.entry(v.code.clone()).or_insert(0) += 1;
            }
            files.push(report);
        }
    }

    let compliant_files = files.iter().filter(|f| f.compliant).count();
    let compliance_rate = if total_files > 0 {
        (compliant_files as f64 / total_files as f64) * 100.0
    } else {
        100.0
    };

    Ok(CorpusReport {
        total_files,
        compliant_files,
        compliance_rate,
        files,
        violation_counts,
    })
}

/// Run the lint command
pub fn lint_command(
    input: PathBuf,
    strict: bool,
    format: String,
    fail_fast: bool,
    corpus: bool,
) -> Result<()> {
    if corpus || input.is_dir() {
        let report = lint_corpus(&input, strict)?;

        match format.as_str() {
            "json" => println!("{}", serde_json::to_string_pretty(&report)?),
            _ => print_corpus_report(&report, fail_fast),
        }

        if report.compliance_rate < 100.0 {
            std::process::exit(1);
        }
    } else {
        let report = lint_file(&input, strict)?;

        match format.as_str() {
            "json" => println!("{}", serde_json::to_string_pretty(&report)?),
            _ => print_file_report(&report, fail_fast),
        }

        if !report.compliant {
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Print file report in text format
fn print_file_report(report: &FileReport, fail_fast: bool) {
    if report.violations.is_empty() {
        println!("{} {} is Depyler Python compliant", "✓".green(), report.path.display());
        return;
    }

    println!("{}", format!("{}:", report.path.display()).bold());

    for v in &report.violations {
        let severity_str = match v.severity {
            Severity::Error => "error".red(),
            Severity::Warning => "warning".yellow(),
        };

        println!(
            "  {}:{}: {} [{}] {}",
            v.line,
            v.column,
            severity_str,
            v.code.cyan(),
            v.message
        );

        if let Some(ref source) = v.source_line {
            println!("    {}", source.dimmed());
        }

        if fail_fast && v.severity == Severity::Error {
            std::process::exit(1);
        }
    }

    let error_count = report.violations.iter().filter(|v| v.severity == Severity::Error).count();
    let warning_count = report.violations.iter().filter(|v| v.severity == Severity::Warning).count();

    println!(
        "\nFound {} error(s), {} warning(s)",
        error_count.to_string().red(),
        warning_count.to_string().yellow()
    );
}

/// Print corpus report in text format
fn print_corpus_report(report: &CorpusReport, fail_fast: bool) {
    println!("{}", "Depyler Python Compliance Report".bold());
    println!("================================\n");

    println!(
        "Files analyzed: {}\nCompliant files: {}\nCompliance rate: {:.1}%\n",
        report.total_files,
        report.compliant_files,
        report.compliance_rate
    );

    if !report.violation_counts.is_empty() {
        println!("{}", "Violation Summary:".bold());
        let mut counts: Vec<_> = report.violation_counts.iter().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        for (code, count) in counts {
            println!("  {}: {} occurrences", code.cyan(), count);
        }
        println!();
    }

    // Show non-compliant files
    let non_compliant: Vec<_> = report.files.iter().filter(|f| !f.compliant).collect();
    if !non_compliant.is_empty() {
        println!("{}", "Non-compliant files:".bold());
        for file in non_compliant.iter().take(10) {
            print_file_report(file, fail_fast);
            println!();
        }

        if non_compliant.len() > 10 {
            println!("... and {} more", non_compliant.len() - 10);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_lint_clean_file() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("clean.py");
        fs::write(
            &py_file,
            "def add(a: int, b: int) -> int:\n    return a + b\n",
        )
        .unwrap();

        let report = lint_file(&py_file, true).unwrap();
        assert!(report.compliant);
        assert!(report.violations.is_empty());
    }

    #[test]
    fn test_lint_eval_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "x = eval('1 + 1')\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP003));
    }

    #[test]
    fn test_lint_exec_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "exec('print(1)')\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP004));
    }

    #[test]
    fn test_lint_getattr_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "val = getattr(obj, 'name')\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP005));
    }

    #[test]
    fn test_lint_setattr_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "setattr(obj, 'name', value)\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP006));
    }

    #[test]
    fn test_lint_metaclass_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "class Foo(metaclass=Meta):\n    pass\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP007));
    }

    #[test]
    fn test_lint_multiple_inheritance() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "class Foo(Bar, Baz):\n    pass\n").unwrap();

        let report = lint_file(&py_file, true).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP008));
    }

    #[test]
    fn test_lint_globals_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "g = globals()\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP013));
    }

    #[test]
    fn test_lint_locals_prohibited() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("bad.py");
        fs::write(&py_file, "l = locals()\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        assert!(!report.compliant);
        assert!(report.violations.iter().any(|v| v.code == codes::DP014));
    }

    #[test]
    fn test_lint_false_positive_evaluate() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("ok.py");
        // "evaluate" should not trigger eval check
        fs::write(&py_file, "def evaluate(x: int) -> int:\n    return x\n").unwrap();

        let report = lint_file(&py_file, false).unwrap();
        // Should not have DP003 violation
        assert!(!report.violations.iter().any(|v| v.code == codes::DP003));
    }

    #[test]
    fn test_contains_function_call() {
        assert!(contains_function_call("eval('x')", "eval"));
        assert!(contains_function_call("x = eval('x')", "eval"));
        assert!(!contains_function_call("evaluate('x')", "eval"));
        assert!(!contains_function_call("my_eval('x')", "eval"));
    }

    #[test]
    fn test_corpus_lint() {
        let temp = TempDir::new().unwrap();

        // Create compliant file
        let good = temp.path().join("good.py");
        fs::write(&good, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

        // Create non-compliant file
        let bad = temp.path().join("bad.py");
        fs::write(&bad, "x = eval('1')\n").unwrap();

        let report = lint_corpus(temp.path(), false).unwrap();
        assert_eq!(report.total_files, 2);
        assert_eq!(report.compliant_files, 1);
        assert!((report.compliance_rate - 50.0).abs() < 0.1);
    }
}
