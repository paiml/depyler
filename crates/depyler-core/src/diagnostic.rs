//! World-class diagnostic formatting for transpilation errors
//!
//! Produces rich, bashrs-style error output with source snippets,
//! categorized notes, and actionable help messages.

use colored::Colorize;
use std::fmt;

/// Error category for diagnostic classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Syntax,
    UnsupportedFeature,
    TypeInference,
    CodeGeneration,
    Io,
    Internal,
}

impl ErrorCategory {
    /// Short tag used in the error header, e.g. `error[syntax]:`
    pub fn tag(self) -> &'static str {
        match self {
            Self::Syntax => "syntax",
            Self::UnsupportedFeature => "unsupported",
            Self::TypeInference => "type",
            Self::CodeGeneration => "codegen",
            Self::Io => "io",
            Self::Internal => "internal",
        }
    }
}

/// A rich diagnostic with source context, modeled after bashrs/rustc output.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub category: ErrorCategory,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub snippet: Option<Snippet>,
    pub note: Option<String>,
    pub help: Option<String>,
}

/// A 3-line source context window with gutter line numbers.
#[derive(Debug, Clone)]
pub struct Snippet {
    /// Line before the error (if available)
    pub before: Option<(usize, String)>,
    /// The error line
    pub error_line: (usize, String),
    /// Line after the error (if available)
    pub after: Option<(usize, String)>,
    /// Column to place the caret (1-indexed)
    pub caret_col: Option<usize>,
    /// Width of the underline (defaults to 1)
    pub caret_width: usize,
}

impl Diagnostic {
    /// Construct a diagnostic from an anyhow error, an optional file path, and optional source.
    pub fn from_anyhow(
        err: &anyhow::Error,
        file: Option<String>,
        source: Option<&str>,
    ) -> Self {
        let msg = err.to_string();
        let (category, note, help) = categorize_error(&msg);
        let (mut line, mut column) = extract_location(&msg);

        // Fall back to byte offset extraction if no line/column found
        if line.is_none() {
            if let Some(src) = source {
                if let Some((l, c)) = extract_location_from_byte_offset(&msg, src) {
                    line = Some(l);
                    column = Some(c);
                }
            }
        }

        let snippet = match (source, line) {
            (Some(src), Some(ln)) => Some(extract_snippet(src, ln, column)),
            _ => None,
        };

        Self {
            category,
            message: clean_message(&msg),
            file,
            line,
            column,
            snippet,
            note,
            help,
        }
    }

    /// Quality score for diagnostic completeness (0.0 - 1.0).
    ///
    /// A score >= 0.7 means the diagnostic is actionable.
    pub fn quality_score(&self) -> f64 {
        let mut score = 0.0;

        // Base: we always have a message and category
        score += 0.3;

        // File location
        if self.file.is_some() {
            score += 0.1;
        }

        // Line information
        if self.line.is_some() {
            score += 0.1;
        }

        // Source snippet
        if self.snippet.is_some() {
            score += 0.2;
        }

        // Note
        if self.note.is_some() {
            score += 0.15;
        }

        // Help
        if self.help.is_some() {
            score += 0.15;
        }

        score
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header: error[tag]: message
        write!(
            f,
            "{}[{}]: {}",
            "error".red().bold(),
            self.category.tag().red(),
            self.message.bold()
        )?;
        writeln!(f)?;

        // Location: --> file:line:column
        match (&self.file, self.line, self.column) {
            (Some(file), Some(line), Some(col)) => {
                writeln!(f, " {} {}:{}:{}", "-->".blue().bold(), file, line, col)?;
            }
            (Some(file), Some(line), None) => {
                writeln!(f, " {} {}:{}", "-->".blue().bold(), file, line)?;
            }
            (Some(file), None, None) => {
                writeln!(f, " {} {}", "-->".blue().bold(), file)?;
            }
            _ => {}
        }

        // Snippet
        if let Some(ref snippet) = self.snippet {
            format_snippet(f, snippet)?;
        }

        // Note
        if let Some(ref note) = self.note {
            writeln!(f, "  {}: {}", "note".yellow().bold(), note)?;
        }

        // Help
        if let Some(ref help) = self.help {
            writeln!(f, "  {}: {}", "help".green().bold(), help)?;
        }

        Ok(())
    }
}

/// Format a source snippet with gutter line numbers and caret.
fn format_snippet(f: &mut fmt::Formatter<'_>, snippet: &Snippet) -> fmt::Result {
    let gutter_width = 4;

    // Before line
    if let Some((num, ref text)) = snippet.before {
        writeln!(
            f,
            " {:>gutter_width$} {} {}",
            num.to_string().blue().bold(),
            "|".blue().bold(),
            text,
            gutter_width = gutter_width
        )?;
    }

    // Error line
    let (num, ref text) = snippet.error_line;
    writeln!(
        f,
        " {:>gutter_width$} {} {}",
        num.to_string().blue().bold(),
        "|".blue().bold(),
        text,
        gutter_width = gutter_width
    )?;

    // Caret line
    if let Some(col) = snippet.caret_col {
        let padding = col.saturating_sub(1);
        let carets = "^".repeat(snippet.caret_width.max(1));
        writeln!(
            f,
            " {:>gutter_width$} {} {}{}",
            "",
            "|".blue().bold(),
            " ".repeat(padding),
            carets.red().bold(),
            gutter_width = gutter_width
        )?;
    }

    // After line
    if let Some((num, ref text)) = snippet.after {
        writeln!(
            f,
            " {:>gutter_width$} {} {}",
            num.to_string().blue().bold(),
            "|".blue().bold(),
            text,
            gutter_width = gutter_width
        )?;
    }

    Ok(())
}

/// Extract a 3-line snippet from source around the given line number.
pub fn extract_snippet(source: &str, line: usize, column: Option<usize>) -> Snippet {
    let lines: Vec<&str> = source.lines().collect();
    let idx = line.saturating_sub(1); // 0-indexed

    let before = if idx > 0 {
        lines.get(idx - 1).map(|l| (line - 1, l.to_string()))
    } else {
        None
    };

    let error_line = lines
        .get(idx)
        .map(|l| (line, l.to_string()))
        .unwrap_or((line, String::new()));

    let after = lines.get(idx + 1).map(|l| (line + 1, l.to_string()));

    // Try to estimate caret width from the error token
    let caret_width = estimate_caret_width(&error_line.1, column);

    Snippet {
        before,
        error_line,
        after,
        caret_col: column,
        caret_width,
    }
}

/// Estimate width of the token at the caret position.
fn estimate_caret_width(line: &str, column: Option<usize>) -> usize {
    let col = match column {
        Some(c) if c > 0 => c - 1, // 0-indexed
        _ => return 1,
    };

    let chars: Vec<char> = line.chars().collect();
    if col >= chars.len() {
        return 1;
    }

    // Walk forward while we're in the same "word" (alphanumeric/underscore)
    let start_char = chars[col];
    if start_char.is_alphanumeric() || start_char == '_' {
        let mut end = col;
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        (end - col).max(1)
    } else {
        1
    }
}

/// Extract line and column from an error message string.
///
/// Looks for patterns like:
/// - "at line N, column M" (rustpython style)
/// - "line N" (generic)
/// - ":N:M:" (file:line:col)
fn extract_location(msg: &str) -> (Option<usize>, Option<usize>) {
    // Pattern: "at row N, column M" (rustpython)
    if let Some(rest) = msg
        .find("at row ")
        .map(|i| &msg[i + "at row ".len()..])
    {
        if let Some((line_str, after)) = rest.split_once(',') {
            let line = line_str.trim().parse::<usize>().ok();
            let col = after
                .trim()
                .strip_prefix("column ")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse::<usize>().ok());
            if line.is_some() {
                return (line, col);
            }
        }
    }

    // Pattern: "at line N" or "line N"
    for prefix in &["at line ", "line "] {
        if let Some(rest) = msg.find(prefix).map(|i| &msg[i + prefix.len()..]) {
            if let Some(line) = rest
                .split(|c: char| !c.is_ascii_digit())
                .next()
                .and_then(|s| s.parse::<usize>().ok())
            {
                return (Some(line), None);
            }
        }
    }

    (None, None)
}

/// Extract line and column from a "byte offset N" pattern in the error message.
fn extract_location_from_byte_offset(msg: &str, source: &str) -> Option<(usize, usize)> {
    let lower = msg.to_lowercase();
    let prefix = "byte offset ";
    let idx = lower.find(prefix)?;
    let rest = &msg[idx + prefix.len()..];
    let offset_str = rest
        .split(|c: char| !c.is_ascii_digit())
        .next()?;
    let offset: usize = offset_str.parse().ok()?;

    // Convert byte offset to line/column
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    Some((line, col))
}

type Categorization = (ErrorCategory, Option<String>, Option<String>);

/// Categorize an error message into (category, note, help).
fn categorize_error(msg: &str) -> Categorization {
    let lower = msg.to_lowercase();

    // Try each matcher in priority order
    try_match_io(&lower)
        .or_else(|| try_match_type(&lower))
        .or_else(|| try_match_internal(&lower))
        .or_else(|| try_match_codegen(&lower))
        .or_else(|| try_match_syntax(&lower))
        .or_else(|| try_match_unsupported(&lower))
        .or_else(|| try_match_generator(&lower))
        .or_else(|| try_match_import(&lower))
        .unwrap_or_else(default_categorization)
}

fn try_match_io(lower: &str) -> Option<Categorization> {
    let is_io = lower.contains("no such file")
        || lower.contains("permission denied")
        || lower.contains("is a directory")
        || lower.contains("not found")
            && (lower.contains("file") || lower.contains("path") || lower.contains("directory"));
    is_io.then(|| {
        (
            ErrorCategory::Io,
            Some("The specified file could not be read".to_string()),
            Some("Check the file path and permissions".to_string()),
        )
    })
}

fn try_match_type(lower: &str) -> Option<Categorization> {
    let is_type =
        lower.contains("type") && (lower.contains("mismatch") || lower.contains("inference"));
    is_type.then(|| {
        (
            ErrorCategory::TypeInference,
            Some("The type checker could not reconcile the types".to_string()),
            Some("Add explicit type annotations to help the transpiler".to_string()),
        )
    })
}

fn try_match_internal(lower: &str) -> Option<Categorization> {
    let is_internal = lower.contains("internal error") || lower.contains("internal transpiler");
    is_internal.then(|| {
        (
            ErrorCategory::Internal,
            Some("An internal transpiler error occurred".to_string()),
            Some("Please report this bug with a minimal reproducer".to_string()),
        )
    })
}

fn try_match_codegen(lower: &str) -> Option<Categorization> {
    let is_codegen = lower.contains("code generation") || lower.contains("codegen");
    is_codegen.then(|| {
        (
            ErrorCategory::CodeGeneration,
            Some("The transpiler failed during Rust code generation".to_string()),
            Some("This may be a transpiler bug — please report it with a minimal reproducer".to_string()),
        )
    })
}

fn try_match_syntax(lower: &str) -> Option<Categorization> {
    let is_syntax = lower.contains("parse error")
        || lower.contains("syntax error")
        || lower.contains("unexpected token")
        || lower.contains("invalid syntax")
        || lower.contains("expected")
            && (lower.contains("found") || lower.contains("got") || lower.contains("token"));
    is_syntax.then(|| {
        (
            ErrorCategory::Syntax,
            Some("The Python parser could not understand this code".to_string()),
            extract_parse_help(lower),
        )
    })
}

fn try_match_unsupported(lower: &str) -> Option<Categorization> {
    let is_unsupported = lower.contains("unsupported")
        || lower.contains("not supported")
        || lower.contains("not yet supported")
        || lower.contains("not implemented");
    is_unsupported.then(|| {
        let (note, help) = categorize_unsupported(lower);
        (ErrorCategory::UnsupportedFeature, Some(note), Some(help))
    })
}

fn try_match_generator(lower: &str) -> Option<Categorization> {
    let is_gen = lower.contains("yield") && lower.contains("generator");
    is_gen.then(|| {
        (
            ErrorCategory::UnsupportedFeature,
            Some("Generator functions are not supported by the transpiler".to_string()),
            Some("Return a list or Vec instead of yielding values".to_string()),
        )
    })
}

fn try_match_import(lower: &str) -> Option<Categorization> {
    let is_import = lower.contains("import") || lower.contains("module") && lower.contains("not");
    is_import.then(|| {
        (
            ErrorCategory::UnsupportedFeature,
            Some("Module imports are not available in transpiled code".to_string()),
            Some("Use the stdlib mapping or inline the functionality".to_string()),
        )
    })
}

fn default_categorization() -> Categorization {
    (
        ErrorCategory::Syntax,
        Some("Transpilation failed".to_string()),
        Some("Check the Python source for syntax or feature issues".to_string()),
    )
}

/// Generate help text for parse errors.
fn extract_parse_help(lower: &str) -> Option<String> {
    if lower.contains("indent") {
        Some("Check indentation — Python requires consistent whitespace".to_string())
    } else if lower.contains("colon") || lower.contains("':'") {
        Some("A colon ':' is expected after def, class, if, for, while, etc.".to_string())
    } else if lower.contains("parenthes") || lower.contains("bracket") {
        Some("Check for mismatched parentheses, brackets, or braces".to_string())
    } else {
        Some("Check the Python syntax at the indicated line".to_string())
    }
}

/// Categorize unsupported feature errors into (note, help).
fn categorize_unsupported(lower: &str) -> (String, String) {
    if lower.contains("yield") || lower.contains("generator") {
        (
            "Generator functions are not supported by the transpiler".to_string(),
            "Return a list or Vec instead of yielding values".to_string(),
        )
    } else if lower.contains("async") || lower.contains("await") {
        (
            "Async/await is not supported by the transpiler".to_string(),
            "Use synchronous code or restructure with threads".to_string(),
        )
    } else if lower.contains("decorator") {
        (
            "Decorators are not supported by the transpiler".to_string(),
            "Apply the decorator logic manually in the function body".to_string(),
        )
    } else if lower.contains("metaclass") || lower.contains("__metaclass__") {
        (
            "Metaclasses are not supported by the transpiler".to_string(),
            "Use composition or trait-based patterns instead".to_string(),
        )
    } else {
        (
            "This Python feature is not yet supported by the transpiler".to_string(),
            "Consider using a simpler construct that the transpiler can handle".to_string(),
        )
    }
}

/// Strip common prefixes from error messages to get a cleaner display.
fn clean_message(msg: &str) -> String {
    let msg = msg.trim();

    // Strip "Python parse error: " prefix
    for prefix in &[
        "Python parse error: ",
        "Failed to parse: ",
        "Transpilation error: ",
    ] {
        if let Some(rest) = msg.strip_prefix(prefix) {
            return rest.to_string();
        }
    }

    msg.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- ErrorCategory ----

    #[test]
    fn test_error_category_tags() {
        assert_eq!(ErrorCategory::Syntax.tag(), "syntax");
        assert_eq!(ErrorCategory::UnsupportedFeature.tag(), "unsupported");
        assert_eq!(ErrorCategory::TypeInference.tag(), "type");
        assert_eq!(ErrorCategory::CodeGeneration.tag(), "codegen");
        assert_eq!(ErrorCategory::Io.tag(), "io");
        assert_eq!(ErrorCategory::Internal.tag(), "internal");
    }

    // ---- Snippet extraction ----

    #[test]
    fn test_extract_snippet_middle_of_file() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let snippet = extract_snippet(source, 3, Some(2));

        assert_eq!(snippet.before, Some((2, "line2".to_string())));
        assert_eq!(snippet.error_line, (3, "line3".to_string()));
        assert_eq!(snippet.after, Some((4, "line4".to_string())));
        assert_eq!(snippet.caret_col, Some(2));
    }

    #[test]
    fn test_extract_snippet_first_line() {
        let source = "first\nsecond\nthird";
        let snippet = extract_snippet(source, 1, Some(1));

        assert_eq!(snippet.before, None);
        assert_eq!(snippet.error_line, (1, "first".to_string()));
        assert_eq!(snippet.after, Some((2, "second".to_string())));
    }

    #[test]
    fn test_extract_snippet_last_line() {
        let source = "first\nsecond\nthird";
        let snippet = extract_snippet(source, 3, Some(1));

        assert_eq!(snippet.before, Some((2, "second".to_string())));
        assert_eq!(snippet.error_line, (3, "third".to_string()));
        assert_eq!(snippet.after, None);
    }

    #[test]
    fn test_extract_snippet_single_line() {
        let source = "only line";
        let snippet = extract_snippet(source, 1, Some(5));

        assert_eq!(snippet.before, None);
        assert_eq!(snippet.error_line, (1, "only line".to_string()));
        assert_eq!(snippet.after, None);
    }

    #[test]
    fn test_extract_snippet_no_column() {
        let source = "line1\nline2\nline3";
        let snippet = extract_snippet(source, 2, None);

        assert_eq!(snippet.caret_col, None);
        assert_eq!(snippet.error_line, (2, "line2".to_string()));
    }

    // ---- Location extraction ----

    #[test]
    fn test_extract_location_rustpython_style() {
        let msg = "invalid syntax at row 5, column 12";
        let (line, col) = extract_location(msg);
        assert_eq!(line, Some(5));
        assert_eq!(col, Some(12));
    }

    #[test]
    fn test_extract_location_at_line() {
        let msg = "error at line 42";
        let (line, col) = extract_location(msg);
        assert_eq!(line, Some(42));
        assert_eq!(col, None);
    }

    #[test]
    fn test_extract_location_no_location() {
        let msg = "something went wrong";
        let (line, col) = extract_location(msg);
        assert_eq!(line, None);
        assert_eq!(col, None);
    }

    // ---- Categorization ----

    #[test]
    fn test_categorize_parse_error() {
        let (cat, note, help) = categorize_error("Python parse error: unexpected token");
        assert_eq!(cat, ErrorCategory::Syntax);
        assert!(note.is_some());
        assert!(help.is_some());
    }

    #[test]
    fn test_categorize_unsupported_feature() {
        let (cat, note, help) = categorize_error("Unsupported Python feature: yield");
        assert_eq!(cat, ErrorCategory::UnsupportedFeature);
        assert!(note.unwrap().contains("not"));
        assert!(help.is_some());
    }

    #[test]
    fn test_categorize_type_mismatch() {
        let (cat, _, _) = categorize_error("type mismatch: expected i32, found String");
        assert_eq!(cat, ErrorCategory::TypeInference);
    }

    #[test]
    fn test_categorize_io_error() {
        let (cat, _, _) = categorize_error("No such file or directory");
        assert_eq!(cat, ErrorCategory::Io);
    }

    #[test]
    fn test_categorize_codegen_error() {
        let (cat, _, _) = categorize_error("Code generation failed for function foo");
        assert_eq!(cat, ErrorCategory::CodeGeneration);
    }

    #[test]
    fn test_categorize_internal_error() {
        let (cat, _, _) = categorize_error("Internal error: assertion failed");
        assert_eq!(cat, ErrorCategory::Internal);
    }

    // ---- Quality score ----

    #[test]
    fn test_quality_score_minimal() {
        let diag = Diagnostic {
            category: ErrorCategory::Syntax,
            message: "error".to_string(),
            file: None,
            line: None,
            column: None,
            snippet: None,
            note: None,
            help: None,
        };
        // Only message + category = 0.3
        assert!((diag.quality_score() - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_quality_score_full() {
        let diag = Diagnostic {
            category: ErrorCategory::Syntax,
            message: "error".to_string(),
            file: Some("test.py".to_string()),
            line: Some(1),
            column: Some(1),
            snippet: Some(Snippet {
                before: None,
                error_line: (1, "bad code".to_string()),
                after: None,
                caret_col: Some(1),
                caret_width: 1,
            }),
            note: Some("note".to_string()),
            help: Some("help".to_string()),
        };
        assert!(diag.quality_score() >= 0.99);
    }

    #[test]
    fn test_quality_score_with_file_and_note() {
        let diag = Diagnostic {
            category: ErrorCategory::Syntax,
            message: "error".to_string(),
            file: Some("test.py".to_string()),
            line: None,
            column: None,
            snippet: None,
            note: Some("note".to_string()),
            help: None,
        };
        // 0.3 + 0.1 (file) + 0.15 (note) = 0.55
        assert!((diag.quality_score() - 0.55).abs() < 0.01);
    }

    // ---- quality_score >= 0.7 for all categories when fully formed ----

    #[test]
    fn test_quality_score_ge_07_for_all_categories() {
        for category in [
            ErrorCategory::Syntax,
            ErrorCategory::UnsupportedFeature,
            ErrorCategory::TypeInference,
            ErrorCategory::CodeGeneration,
            ErrorCategory::Io,
            ErrorCategory::Internal,
        ] {
            let diag = Diagnostic {
                category,
                message: "test error".to_string(),
                file: Some("test.py".to_string()),
                line: Some(1),
                column: Some(1),
                snippet: Some(Snippet {
                    before: None,
                    error_line: (1, "code".to_string()),
                    after: None,
                    caret_col: Some(1),
                    caret_width: 1,
                }),
                note: Some("note".to_string()),
                help: Some("help".to_string()),
            };
            assert!(
                diag.quality_score() >= 0.7,
                "Quality score for {:?} was {} (expected >= 0.7)",
                category,
                diag.quality_score()
            );
        }
    }

    // ---- Display format ----

    #[test]
    fn test_display_format_matches_pattern() {
        let diag = Diagnostic {
            category: ErrorCategory::Syntax,
            message: "unexpected token 'echo'".to_string(),
            file: Some("/tmp/bad.py".to_string()),
            line: Some(3),
            column: Some(5),
            snippet: Some(Snippet {
                before: Some((2, "if x > 0:".to_string())),
                error_line: (3, "    echo missing".to_string()),
                after: Some((4, "    pass".to_string())),
                caret_col: Some(5),
                caret_width: 4,
            }),
            note: Some("The Python parser could not understand this code".to_string()),
            help: Some("Check the Python syntax at the indicated line".to_string()),
        };

        let output = format!("{}", diag);
        // Contains error header with tag
        assert!(output.contains("syntax"));
        assert!(output.contains("unexpected token"));
        // Contains file location
        assert!(output.contains("/tmp/bad.py:3:5"));
        // Contains snippet lines
        assert!(output.contains("echo missing"));
        assert!(output.contains("if x > 0:"));
        // Contains caret
        assert!(output.contains("^^^^"));
        // Contains note and help
        assert!(output.contains("note"));
        assert!(output.contains("help"));
    }

    #[test]
    fn test_display_no_file() {
        let diag = Diagnostic {
            category: ErrorCategory::Internal,
            message: "oops".to_string(),
            file: None,
            line: None,
            column: None,
            snippet: None,
            note: None,
            help: None,
        };
        let output = format!("{}", diag);
        assert!(output.contains("oops"));
        assert!(!output.contains("-->"));
    }

    #[test]
    fn test_display_file_only() {
        let diag = Diagnostic {
            category: ErrorCategory::Io,
            message: "not found".to_string(),
            file: Some("missing.py".to_string()),
            line: None,
            column: None,
            snippet: None,
            note: None,
            help: None,
        };
        let output = format!("{}", diag);
        assert!(output.contains("missing.py"));
        assert!(output.contains("-->"));
    }

    #[test]
    fn test_display_file_and_line_no_column() {
        let diag = Diagnostic {
            category: ErrorCategory::Syntax,
            message: "bad".to_string(),
            file: Some("test.py".to_string()),
            line: Some(10),
            column: None,
            snippet: None,
            note: None,
            help: None,
        };
        let output = format!("{}", diag);
        assert!(output.contains("test.py:10"));
    }

    // ---- from_anyhow integration ----

    #[test]
    fn test_from_anyhow_parse_error() {
        let err = anyhow::anyhow!("Python parse error: invalid syntax at row 3, column 5");
        let source = "line1\nline2\n    bad syntax\nline4";
        let diag = Diagnostic::from_anyhow(&err, Some("test.py".to_string()), Some(source));

        assert_eq!(diag.category, ErrorCategory::Syntax);
        assert_eq!(diag.line, Some(3));
        assert_eq!(diag.column, Some(5));
        assert!(diag.snippet.is_some());
        assert!(diag.note.is_some());
        assert!(diag.help.is_some());
        assert!(diag.quality_score() >= 0.7);
    }

    #[test]
    fn test_from_anyhow_io_error() {
        let err = anyhow::anyhow!("No such file or directory");
        let diag = Diagnostic::from_anyhow(&err, Some("missing.py".to_string()), None);

        assert_eq!(diag.category, ErrorCategory::Io);
        assert!(diag.snippet.is_none());
    }

    #[test]
    fn test_from_anyhow_unsupported() {
        let err = anyhow::anyhow!("Unsupported Python feature: async/await");
        let diag = Diagnostic::from_anyhow(&err, None, None);

        assert_eq!(diag.category, ErrorCategory::UnsupportedFeature);
    }

    // ---- clean_message ----

    #[test]
    fn test_clean_message_strips_prefix() {
        assert_eq!(
            clean_message("Python parse error: bad syntax"),
            "bad syntax"
        );
        assert_eq!(
            clean_message("Failed to parse: something"),
            "something"
        );
        assert_eq!(
            clean_message("no prefix here"),
            "no prefix here"
        );
    }

    // ---- estimate_caret_width ----

    #[test]
    fn test_estimate_caret_width_word() {
        assert_eq!(estimate_caret_width("    echo missing", Some(5)), 4); // "echo"
    }

    #[test]
    fn test_estimate_caret_width_symbol() {
        assert_eq!(estimate_caret_width("x + y", Some(3)), 1); // "+"
    }

    #[test]
    fn test_estimate_caret_width_no_column() {
        assert_eq!(estimate_caret_width("anything", None), 1);
    }

    #[test]
    fn test_estimate_caret_width_past_end() {
        assert_eq!(estimate_caret_width("short", Some(100)), 1);
    }

    // ---- categorize_unsupported ----

    #[test]
    fn test_categorize_unsupported_yield() {
        let (note, _help) = categorize_unsupported("yield expression not supported");
        assert!(note.contains("Generator"));
    }

    #[test]
    fn test_categorize_unsupported_async() {
        let (note, _help) = categorize_unsupported("async function not supported");
        assert!(note.contains("Async"));
    }

    #[test]
    fn test_categorize_unsupported_decorator() {
        let (note, _help) = categorize_unsupported("decorator not supported");
        assert!(note.contains("Decorator"));
    }

    #[test]
    fn test_categorize_unsupported_generic() {
        let (note, _help) = categorize_unsupported("walrus operator not supported");
        assert!(note.contains("not yet supported"));
    }

    // ---- parse help ----

    #[test]
    fn test_parse_help_indent() {
        let help = extract_parse_help("indentation error");
        assert!(help.unwrap().contains("indentation"));
    }

    #[test]
    fn test_parse_help_colon() {
        let help = extract_parse_help("expected ':'");
        assert!(help.unwrap().contains("colon"));
    }

    #[test]
    fn test_parse_help_paren() {
        let help = extract_parse_help("unmatched parenthesis");
        assert!(help.unwrap().contains("parentheses"));
    }

    #[test]
    fn test_parse_help_generic() {
        let help = extract_parse_help("something else");
        assert!(help.unwrap().contains("syntax"));
    }

    // ---- Byte offset extraction ----

    #[test]
    fn test_extract_location_from_byte_offset() {
        let source = "def foo(x):\n    return x\n\ndef bar(:\n    pass\n";
        let msg = "Got unexpected token ':' at byte offset 34";
        let result = extract_location_from_byte_offset(msg, source);
        assert!(result.is_some());
        let (line, col) = result.unwrap();
        assert_eq!(line, 4);
        assert_eq!(col, 9);
    }

    #[test]
    fn test_extract_location_from_byte_offset_no_match() {
        let result = extract_location_from_byte_offset("no offset here", "source");
        assert!(result.is_none());
    }

    #[test]
    fn test_from_anyhow_with_byte_offset() {
        let err = anyhow::anyhow!(
            "Python parse error: invalid syntax. Got unexpected token ':' at byte offset 34"
        );
        let source = "def foo(x):\n    return x\n\ndef bar(:\n    pass\n";
        let diag = Diagnostic::from_anyhow(&err, Some("test.py".to_string()), Some(source));

        assert_eq!(diag.category, ErrorCategory::Syntax);
        assert_eq!(diag.line, Some(4));
        assert_eq!(diag.column, Some(9));
        assert!(diag.quality_score() >= 0.7);
        assert!(diag.snippet.is_some());

        let snippet = diag.snippet.as_ref().unwrap();
        assert!(snippet.error_line.1.contains("def bar("));
    }
}
