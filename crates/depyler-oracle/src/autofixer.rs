//! Automatic code transformation based on oracle classifications.
//!
//! Uses the 97% accurate oracle to classify errors and apply fixes.

use crate::{ErrorCategory, Oracle, OracleError};
use regex::Regex;
use std::collections::HashMap;

/// Result of an automatic fix attempt.
#[derive(Debug, Clone)]
pub struct FixResult {
    /// Whether a fix was applied
    pub fixed: bool,
    /// The modified source code (if fixed)
    pub source: String,
    /// Description of what was fixed
    pub description: String,
    /// Confidence of the fix
    pub confidence: f32,
}

/// Automatic code fixer using oracle classifications.
pub struct AutoFixer {
    oracle: Oracle,
    /// Transformation rules per category
    rules: HashMap<ErrorCategory, Vec<TransformRule>>,
}

/// A transformation rule for fixing code.
#[derive(Clone)]
pub struct TransformRule {
    /// Name of the rule
    pub name: String,
    /// Pattern to match in error message
    pub error_pattern: Regex,
    /// Function to apply the fix
    pub transform: fn(&mut FixContext) -> bool,
}

/// Context for applying a fix.
pub struct FixContext<'a> {
    /// The source code
    pub source: &'a mut String,
    /// The error message
    pub error_msg: &'a str,
    /// Line number of the error
    pub line: usize,
    /// Variable name involved (if any)
    pub var_name: Option<String>,
    /// Type information (if any)
    pub type_info: Option<String>,
}

impl AutoFixer {
    /// Create a new AutoFixer with trained oracle.
    pub fn new() -> Result<Self, OracleError> {
        let oracle = Oracle::load_or_train()?;
        let rules = Self::default_rules();
        Ok(Self { oracle, rules })
    }

    /// Create with existing oracle.
    pub fn with_oracle(oracle: Oracle) -> Self {
        let rules = Self::default_rules();
        Self { oracle, rules }
    }

    /// Default transformation rules for each category.
    fn default_rules() -> HashMap<ErrorCategory, Vec<TransformRule>> {
        let mut rules = HashMap::new();

        // BorrowChecker rules
        rules.insert(
            ErrorCategory::BorrowChecker,
            vec![
                TransformRule {
                    name: "pre_compute_is_some".to_string(),
                    error_pattern: Regex::new(r"borrow of moved value.*\.is_some\(\)").expect("static regex"),
                    transform: fix_pre_compute_is_some,
                },
                TransformRule {
                    name: "pre_compute_is_none".to_string(),
                    error_pattern: Regex::new(r"borrow of moved value.*\.is_none\(\)").expect("static regex"),
                    transform: fix_pre_compute_is_none,
                },
                TransformRule {
                    name: "clone_before_move".to_string(),
                    error_pattern: Regex::new(r"use of moved value").expect("static regex"),
                    transform: fix_clone_before_move,
                },
            ],
        );

        // TypeMismatch rules
        rules.insert(
            ErrorCategory::TypeMismatch,
            vec![
                TransformRule {
                    name: "regex_new_str".to_string(),
                    error_pattern: Regex::new(r"Regex::new.*expected `&str`, found `String`")
                        .expect("static regex"),
                    transform: fix_regex_new_str,
                },
                TransformRule {
                    name: "string_to_str".to_string(),
                    error_pattern: Regex::new(r"expected `&str`, found `String`").expect("static regex"),
                    transform: fix_string_to_str,
                },
            ],
        );

        // MissingImport rules
        rules.insert(
            ErrorCategory::MissingImport,
            vec![TransformRule {
                name: "add_command_factory".to_string(),
                error_pattern: Regex::new(r"cannot find value `parser`").expect("static regex"),
                transform: fix_add_command_factory,
            }],
        );

        rules
    }

    /// Attempt to fix compilation errors in source code.
    ///
    /// Takes rustc error output and the source file, returns fixed source.
    pub fn fix(&self, source: &str, errors: &str) -> FixResult {
        let mut fixed_source = source.to_string();
        let mut fixes_applied = Vec::new();
        let mut total_confidence = 0.0;
        let mut fix_count = 0;

        // Parse each error from rustc output
        for error_block in Self::parse_errors(errors) {
            // Classify with oracle using full feature extraction
            if let Ok(classification) = self.oracle.classify_message(&error_block.message) {
                // Try to apply fixes for this category
                if let Some(rules) = self.rules.get(&classification.category) {
                    for rule in rules {
                        if rule.error_pattern.is_match(&error_block.message) {
                            let mut ctx = FixContext {
                                source: &mut fixed_source,
                                error_msg: &error_block.message,
                                line: error_block.line,
                                var_name: Self::extract_var_name(&error_block.message),
                                type_info: Self::extract_type_info(&error_block.message),
                            };

                            if (rule.transform)(&mut ctx) {
                                fixes_applied.push(format!(
                                    "Applied '{}' at line {} ({:?}, {:.0}% confidence)",
                                    rule.name,
                                    error_block.line,
                                    classification.category,
                                    classification.confidence * 100.0
                                ));
                                total_confidence += classification.confidence;
                                fix_count += 1;
                                break; // One fix per error
                            }
                        }
                    }
                }
            }
        }

        let avg_confidence = if fix_count > 0 {
            total_confidence / fix_count as f32
        } else {
            0.0
        };

        FixResult {
            fixed: !fixes_applied.is_empty(),
            source: fixed_source,
            description: fixes_applied.join("\n"),
            confidence: avg_confidence,
        }
    }

    /// Parse rustc error output into structured errors.
    fn parse_errors(errors: &str) -> Vec<ParsedError> {
        let mut parsed = Vec::new();
        let error_re = Regex::new(r"error\[E\d+\]:[^\n]+").expect("static regex");
        let line_re = Regex::new(r"--> [^:]+:(\d+):\d+").expect("static regex");

        let mut current_error = String::new();
        let mut current_line = 0;

        for line in errors.lines() {
            if error_re.is_match(line) {
                if !current_error.is_empty() {
                    parsed.push(ParsedError {
                        message: current_error.clone(),
                        line: current_line,
                    });
                }
                current_error = line.to_string();
                current_line = 0;
            } else if let Some(caps) = line_re.captures(line) {
                current_line = caps[1].parse().unwrap_or(0);
                current_error.push('\n');
                current_error.push_str(line);
            } else if !current_error.is_empty() {
                current_error.push('\n');
                current_error.push_str(line);
            }
        }

        if !current_error.is_empty() {
            parsed.push(ParsedError {
                message: current_error,
                line: current_line,
            });
        }

        parsed
    }

    /// Extract variable name from error message.
    fn extract_var_name(msg: &str) -> Option<String> {
        let re = Regex::new(r"`([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)?)`").expect("static regex");
        re.captures(msg).map(|c| c[1].to_string())
    }

    /// Extract type information from error message.
    fn extract_type_info(msg: &str) -> Option<String> {
        let re = Regex::new(r"type `([^`]+)`").expect("static regex");
        re.captures(msg).map(|c| c[1].to_string())
    }
}

impl Default for AutoFixer {
    fn default() -> Self {
        Self::new().expect("Failed to create AutoFixer")
    }
}

/// Parsed error from rustc output.
struct ParsedError {
    message: String,
    line: usize,
}

// ============================================================================
// Transformation Functions
// ============================================================================

/// Fix: Pre-compute .is_some() before value is moved.
///
/// Pattern: `let x = foo(args.hash); ... args.hash.is_some()`
/// Fix: `let has_hash = args.hash.is_some(); let x = foo(args.hash); ... has_hash`
fn fix_pre_compute_is_some(ctx: &mut FixContext) -> bool {
    let var = match &ctx.var_name {
        Some(v) => v.clone(),
        None => return false,
    };

    // Find the pattern: var.is_some() after var was moved
    let is_some_pattern = format!("{}.is_some()", var);

    if !ctx.source.contains(&is_some_pattern) {
        return false;
    }

    // Generate the fix variable name
    let fix_var = format!("has_{}", var.split('.').next_back().unwrap_or(&var));

    // First, replace all is_some() usages with the fix variable (except in the declaration we'll add)
    let result = ctx.source.replace(&is_some_pattern, &fix_var);

    // Now find where to insert the pre-computation
    let lines: Vec<&str> = result.lines().collect();
    let mut new_lines = Vec::new();
    let mut inserted = false;

    for line in lines.iter() {
        // Look for a line that uses the var (this is where it gets moved)
        // Insert the pre-computation BEFORE this line
        if !inserted && line.contains(&var) {
            // Insert pre-computation before this line
            let indent = line.len() - line.trim_start().len();
            let indent_str: String = " ".repeat(indent);
            new_lines.push(format!(
                "{}let {} = {};",
                indent_str, fix_var, is_some_pattern
            ));
            inserted = true;
        }
        new_lines.push(line.to_string());
    }

    if !inserted {
        return false;
    }

    *ctx.source = new_lines.join("\n");
    true
}

/// Fix: Pre-compute .is_none() before value is moved.
fn fix_pre_compute_is_none(ctx: &mut FixContext) -> bool {
    let var = match &ctx.var_name {
        Some(v) => v.clone(),
        None => return false,
    };

    let is_none_pattern = format!("{}.is_none()", var);

    if !ctx.source.contains(&is_none_pattern) {
        return false;
    }

    let fix_var = format!("is_{}_none", var.split('.').next_back().unwrap_or(&var));
    let lines: Vec<&str> = ctx.source.lines().collect();
    let mut new_lines = Vec::new();
    let mut inserted = false;

    for line in lines.iter() {
        if !inserted
            && line.contains(&var)
            && !line.contains(".is_none()")
            && (line.contains(&format!("({}", var)) || line.contains(&format!(", {}", var)))
        {
            let indent = line.len() - line.trim_start().len();
            let indent_str: String = " ".repeat(indent);
            new_lines.push(format!(
                "{}let {} = {}.is_none();",
                indent_str, fix_var, var
            ));
            inserted = true;
        }
        new_lines.push(line.to_string());
    }

    if !inserted {
        return false;
    }

    let result = new_lines.join("\n").replace(&is_none_pattern, &fix_var);
    *ctx.source = result;
    true
}

/// Fix: Clone value before it's moved.
fn fix_clone_before_move(ctx: &mut FixContext) -> bool {
    let var = match &ctx.var_name {
        Some(v) => v.clone(),
        None => return false,
    };

    // Simple approach: add .clone() to the moved value
    // This is a conservative fix that always works for Clone types
    let pattern = format!("({})", var);
    let replacement = format!("({}.clone())", var);

    if ctx.source.contains(&pattern) {
        *ctx.source = ctx.source.replace(&pattern, &replacement);
        return true;
    }

    false
}

/// Fix: Regex::new expects &str, not String - remove .to_string()
fn fix_regex_new_str(ctx: &mut FixContext) -> bool {
    // Pattern: Regex::new("...".to_string())
    // Fix: Regex::new("...")
    let re = Regex::new(r#"Regex::new\(\s*"([^"]+)"\.to_string\(\)\s*\)"#).expect("static regex");

    if re.is_match(ctx.source) {
        *ctx.source = re
            .replace_all(ctx.source, r#"Regex::new("$1")"#)
            .to_string();
        return true;
    }

    false
}

/// Fix: Expected &str, found String - add &
fn fix_string_to_str(_ctx: &mut FixContext) -> bool {
    // This is context-dependent, so we use a conservative approach
    // Look for function calls where String is passed but &str expected
    false // Conservative: don't auto-fix without more context
}

/// Fix: parser.print_help() -> Args::command().print_help()
fn fix_add_command_factory(ctx: &mut FixContext) -> bool {
    if ctx.source.contains("parser.print_help()") {
        // Need to add CommandFactory import
        if !ctx.source.contains("CommandFactory") {
            // Update the use statement
            *ctx.source = ctx
                .source
                .replace("use clap::Parser;", "use clap::{CommandFactory, Parser};");
        }
        // Replace parser.print_help() with Args::command().print_help()
        *ctx.source = ctx
            .source
            .replace("parser.print_help()", "Args::command().print_help()?");
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_errors() {
        let errors = r#"error[E0382]: borrow of moved value: `args.hash`
   --> src/main.rs:10:5
    |
5   |     let x = foo(args.hash);
    |                 --------- value moved here
...
10  |     args.hash.is_some()
    |     ^^^^^^^^^ value borrowed here after move"#;

        let parsed = AutoFixer::parse_errors(errors);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].line, 10);
        assert!(parsed[0].message.contains("E0382"));
    }

    #[test]
    fn test_extract_var_name() {
        let msg = "borrow of moved value: `args.hash`";
        let var = AutoFixer::extract_var_name(msg);
        assert_eq!(var, Some("args.hash".to_string()));
    }

    #[test]
    fn test_fix_pre_compute_is_some() {
        let source = r#"
fn main() {
    let info = get_file_info(args.file, args.hash, args.time_format)?;
    let output = format_output(&info, args.hash.is_some());
}
"#;
        let mut fixed = source.to_string();
        let mut ctx = FixContext {
            source: &mut fixed,
            error_msg: "borrow of moved value: `args.hash`",
            line: 4,
            var_name: Some("args.hash".to_string()),
            type_info: None,
        };

        let result = fix_pre_compute_is_some(&mut ctx);
        assert!(result);
        assert!(fixed.contains("let has_hash = args.hash.is_some();"));
        assert!(fixed.contains("format_output(&info, has_hash)"));
    }

    #[test]
    fn test_fix_regex_new_str() {
        let source = r#"let re = Regex::new("\\d+".to_string()).unwrap();"#;
        let mut fixed = source.to_string();
        let mut ctx = FixContext {
            source: &mut fixed,
            error_msg: "expected `&str`, found `String`",
            line: 1,
            var_name: None,
            type_info: None,
        };

        let result = fix_regex_new_str(&mut ctx);
        assert!(result);
        assert_eq!(fixed, r#"let re = Regex::new("\\d+").unwrap();"#);
    }

    #[test]
    fn test_fix_command_factory() {
        let source = r#"use clap::Parser;

fn main() {
    parser.print_help();
}
"#;
        let mut fixed = source.to_string();
        let mut ctx = FixContext {
            source: &mut fixed,
            error_msg: "cannot find value `parser`",
            line: 4,
            var_name: None,
            type_info: None,
        };

        let result = fix_add_command_factory(&mut ctx);
        assert!(result);
        assert!(fixed.contains("use clap::{CommandFactory, Parser};"));
        assert!(fixed.contains("Args::command().print_help()?"));
    }

    // ============================================
    // EXTREME TDD: Additional tests
    // ============================================

    #[test]
    fn test_default_rules_has_all_categories() {
        let rules = AutoFixer::default_rules();
        // Should have rules for BorrowChecker, TypeMismatch, MissingImport
        assert!(rules.contains_key(&ErrorCategory::BorrowChecker));
        assert!(rules.contains_key(&ErrorCategory::TypeMismatch));
        assert!(rules.contains_key(&ErrorCategory::MissingImport));
    }

    #[test]
    fn test_parse_errors_multiple() {
        let errors = r#"error[E0382]: borrow of moved value
   --> src/main.rs:10:5
error[E0308]: mismatched types
   --> src/main.rs:20:10"#;

        let parsed = AutoFixer::parse_errors(errors);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].line, 10);
        assert_eq!(parsed[1].line, 20);
    }

    #[test]
    fn test_parse_errors_empty() {
        let errors = "warning: unused variable";
        let parsed = AutoFixer::parse_errors(errors);
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_extract_var_name_none() {
        let msg = "no var name here";
        let var = AutoFixer::extract_var_name(msg);
        assert!(var.is_none());
    }

    #[test]
    fn test_extract_var_name_complex() {
        // Regex only matches one-level deep: var.field
        let msg = "value borrowed here: `self.data`";
        let var = AutoFixer::extract_var_name(msg);
        assert_eq!(var, Some("self.data".to_string()));
    }

    #[test]
    fn test_fix_result_default() {
        let result = FixResult {
            fixed: false,
            source: String::new(),
            description: "No fix".to_string(),
            confidence: 0.0,
        };
        assert!(!result.fixed);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_transform_rule_pattern_matches() {
        let rule = TransformRule {
            name: "test_rule".to_string(),
            error_pattern: Regex::new(r"borrow.*moved").unwrap(),
            transform: |_| true,
        };
        assert!(rule.error_pattern.is_match("borrow of moved value"));
        assert!(!rule.error_pattern.is_match("type mismatch"));
    }

    #[test]
    fn test_fix_context_modification() {
        let mut source = "let x = 1;".to_string();
        let ctx = FixContext {
            source: &mut source,
            error_msg: "test",
            line: 1,
            var_name: Some("x".to_string()),
            type_info: Some("i32".to_string()),
        };

        // Modify source through context
        *ctx.source = "let x: i32 = 1;".to_string();
        assert_eq!(source, "let x: i32 = 1;");
    }
}
