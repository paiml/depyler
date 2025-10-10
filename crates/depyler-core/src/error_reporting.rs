//! Enhanced error reporting with source location tracking and helpful suggestions

use crate::error::ErrorKind;
use anyhow::Result;
use colored::Colorize;
use rustpython_ast::Ranged;
use std::fmt;

/// Enhanced error with source location and context
#[derive(Debug)]
pub struct EnhancedError {
    /// The base error
    pub error: ErrorKind,
    /// Source file path
    pub file_path: Option<String>,
    /// Line number (1-indexed)
    pub line: Option<usize>,
    /// Column number (1-indexed)
    pub column: Option<usize>,
    /// The source line containing the error
    pub source_line: Option<String>,
    /// Helpful suggestion for fixing the error
    pub suggestion: Option<String>,
    /// Related information
    pub notes: Vec<String>,
}

impl EnhancedError {
    pub fn new(error: ErrorKind) -> Self {
        Self {
            error,
            file_path: None,
            line: None,
            column: None,
            source_line: None,
            suggestion: None,
            notes: Vec::new(),
        }
    }

    pub fn with_location(mut self, file: &str, line: usize, column: usize) -> Self {
        self.file_path = Some(file.to_string());
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_source_line(mut self, line: &str) -> Self {
        self.source_line = Some(line.to_string());
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    pub fn add_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }

    /// Extract location from AST node
    pub fn from_ast_node<T: Ranged>(error: ErrorKind, node: &T, source: &str) -> Self {
        let range = node.range();
        let (line, column) = get_line_column(source, range.start().into());

        let mut enhanced = Self::new(error).with_location("<input>", line, column);

        // Extract the source line
        if let Some(line_text) = get_source_line(source, line) {
            enhanced = enhanced.with_source_line(&line_text);
        }

        // Add automatic suggestions based on error type
        enhanced = add_automatic_suggestions(enhanced);

        enhanced
    }

    /// Format location information (file:line:column)
    #[inline]
    fn format_location_info(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let (Some(file), Some(line), Some(column)) = (&self.file_path, self.line, self.column) {
            writeln!(f, "  {} {}:{}:{}", "-->".blue().bold(), file, line, column)?;
        }
        Ok(())
    }

    /// Format source context with line number and pointer
    #[inline]
    fn format_source_context(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let (Some(line_text), Some(line_num), Some(col)) =
            (&self.source_line, self.line, self.column)
        {
            writeln!(f, "   {} |", format!("{:4}", " ").dimmed())?;
            writeln!(
                f,
                "   {} | {}",
                format!("{:4}", line_num).blue().bold(),
                line_text
            )?;
            writeln!(
                f,
                "   {} | {}{}",
                format!("{:4}", " ").dimmed(),
                " ".repeat(col.saturating_sub(1)),
                "^".red().bold()
            )?;
        }
        Ok(())
    }

    /// Format suggestion if present
    #[inline]
    fn format_suggestion(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(suggestion) = &self.suggestion {
            writeln!(f, "\n{}: {}", "suggestion".green().bold(), suggestion)?;
        }
        Ok(())
    }

    /// Format all notes
    #[inline]
    fn format_notes(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for note in &self.notes {
            writeln!(f, "  {} {}", "note:".yellow(), note)?;
        }
        Ok(())
    }
}

impl fmt::Display for EnhancedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Error header
        writeln!(f, "{}: {}", "error".red().bold(), self.error)?;

        // Location info
        self.format_location_info(f)?;

        // Source context
        self.format_source_context(f)?;

        // Suggestion
        self.format_suggestion(f)?;

        // Notes
        self.format_notes(f)?;

        Ok(())
    }
}

/// Get line and column from byte offset
fn get_line_column(source: &str, offset: u32) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    let offset = offset as usize;

    for (i, ch) in source.chars().enumerate() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

/// Extract a specific line from source
fn get_source_line(source: &str, line_num: usize) -> Option<String> {
    source
        .lines()
        .nth(line_num.saturating_sub(1))
        .map(String::from)
}

/// Add automatic suggestions based on error type
fn add_automatic_suggestions(error: EnhancedError) -> EnhancedError {
    let suggestion = match &error.error {
        ErrorKind::TypeMismatch {
            expected,
            found,
            context,
        } => generate_type_mismatch_suggestion(expected, found, context),
        ErrorKind::UnsupportedFeature(feature) => suggest_unsupported_feature(feature),
        ErrorKind::TypeInferenceError(msg) => suggest_type_inference_fix(msg),
        ErrorKind::InvalidTypeAnnotation(msg) => suggest_annotation_fix(msg),
        _ => None,
    };

    apply_suggestion_to_error(error, suggestion)
}

/// Apply a suggestion and notes to an error
#[inline]
fn apply_suggestion_to_error(
    mut error: EnhancedError,
    suggestion: Option<(String, Vec<String>)>,
) -> EnhancedError {
    if let Some((suggestion_text, notes)) = suggestion {
        error = error.with_suggestion(&suggestion_text);
        for note in notes {
            error = error.add_note(&note);
        }
    }
    error
}

/// Generate suggestion for unsupported Python features
#[inline]
fn suggest_unsupported_feature(feature: &str) -> Option<(String, Vec<String>)> {
    // Special case for yield (most common unsupported feature)
    if feature == "yield" {
        return Some((
            "Generator functions are not supported. Consider returning a list instead.".to_string(),
            vec!["Example: Instead of 'yield x', collect values and 'return values'".to_string()],
        ));
    }

    // Generic message for all other unsupported features
    Some((
        format!(
            "This Python feature '{feature}' is not yet supported. Consider using a simpler construct."
        ),
        vec![],
    ))
}

/// Generate suggestion for type inference errors
#[inline]
fn suggest_type_inference_fix(msg: &str) -> Option<(String, Vec<String>)> {
    if msg.contains("incompatible types") {
        Some((
            "Check that all type annotations are correct and consistent.".to_string(),
            vec!["Rust has stricter type checking than Python".to_string()],
        ))
    } else {
        None
    }
}

/// Generate suggestion for invalid type annotations
#[inline]
fn suggest_annotation_fix(msg: &str) -> Option<(String, Vec<String>)> {
    if msg.contains("borrow") {
        Some((
            "Consider using .clone() or restructuring to avoid multiple borrows.".to_string(),
            vec!["Rust's borrow checker ensures memory safety".to_string()],
        ))
    } else {
        None
    }
}

/// Generate helpful suggestions for type mismatches
fn generate_type_mismatch_suggestion(
    expected: &str,
    found: &str,
    context: &str,
) -> Option<(String, Vec<String>)> {
    // Try specific type mismatch patterns first
    if let Some(suggestion) = suggest_string_mismatch(expected, found) {
        return Some(suggestion);
    }
    if let Some(suggestion) = suggest_division_mismatch(expected, found) {
        return Some(suggestion);
    }
    if let Some(suggestion) = suggest_option_mismatch(expected, found) {
        return Some(suggestion);
    }
    if let Some(suggestion) = suggest_ownership_mismatch(expected, found) {
        return Some(suggestion);
    }
    if let Some(suggestion) = suggest_collection_mismatch(expected, found) {
        return Some(suggestion);
    }

    // Generic fallback for unmatched cases
    suggest_generic_mismatch(expected, found, context)
}

/// Suggest fix for string type mismatches (str vs String, &str)
#[inline]
fn suggest_string_mismatch(expected: &str, found: &str) -> Option<(String, Vec<String>)> {
    if (expected == "String" && found == "&str") || (expected == "str" && found == "String") {
        Some((
            "String type mismatch - Python 'str' maps to both Rust '&str' and 'String'".to_string(),
            vec![
                "In Rust:".to_string(),
                "  • '&str' is a borrowed string slice (cheap, read-only)".to_string(),
                "  • 'String' is an owned, heap-allocated string".to_string(),
                "Python string methods (.upper(), .lower(), .strip()) return owned String".to_string(),
                "Use '.to_string()' to convert &str → String, or '&s' to convert String → &str".to_string(),
            ],
        ))
    } else {
        None
    }
}

/// Suggest fix for division type mismatches (int vs float)
#[inline]
fn suggest_division_mismatch(expected: &str, found: &str) -> Option<(String, Vec<String>)> {
    if expected.contains("f64") && found.contains("i") {
        Some((
            "Division result type mismatch - Python '/' always returns float".to_string(),
            vec![
                "In Python: 10 / 3 = 3.333... (always float)".to_string(),
                "In Rust: 10 / 3 = 3 (integer), 10.0 / 3.0 = 3.333... (float)".to_string(),
                "Use '.as_f64()' or ensure operands are floats for division".to_string(),
            ],
        ))
    } else {
        None
    }
}

/// Suggest fix for Option/None type mismatches
#[inline]
fn suggest_option_mismatch(expected: &str, found: &str) -> Option<(String, Vec<String>)> {
    if expected.contains("Option") && (found == "None" || found == "()") {
        Some((
            "None type mismatch - Python None maps to Rust Option<T>".to_string(),
            vec![
                "In Rust, optional values use Option<T>:".to_string(),
                "  • Some(value) for present values".to_string(),
                "  • None for absent values".to_string(),
                "Return type must be Option<T> if function can return None".to_string(),
            ],
        ))
    } else {
        None
    }
}

/// Suggest fix for ownership mismatches (borrowed vs owned)
#[inline]
fn suggest_ownership_mismatch(expected: &str, found: &str) -> Option<(String, Vec<String>)> {
    if expected.starts_with('&') && !found.starts_with('&') {
        Some((
            format!("Ownership mismatch - expected borrowed reference '{expected}', found owned value '{found}'"),
            vec![
                "Rust distinguishes between owned values and borrowed references".to_string(),
                format!("Add '&' to borrow the value: '&{found}'"),
                "Or use .as_ref() to get a reference without moving the value".to_string(),
            ],
        ))
    } else {
        None
    }
}

/// Suggest fix for collection type mismatches (list vs Vec)
#[inline]
fn suggest_collection_mismatch(expected: &str, found: &str) -> Option<(String, Vec<String>)> {
    if (expected.contains("Vec") && found.contains("list"))
        || (expected.contains("list") && found.contains("Vec"))
    {
        Some((
            "Collection type mismatch - Python list maps to Rust Vec<T>".to_string(),
            vec![
                "Python lists are dynamic arrays, similar to Rust Vec<T>".to_string(),
                "Ensure element types match: Python list[int] → Rust Vec<i32>".to_string(),
            ],
        ))
    } else {
        None
    }
}

/// Generic fallback suggestion for unmatched type mismatches
#[inline]
fn suggest_generic_mismatch(
    expected: &str,
    found: &str,
    context: &str,
) -> Option<(String, Vec<String>)> {
    if context.contains("return") {
        Some((
            format!(
                "Return type mismatch in {context}: expected '{expected}', found '{found}'"
            ),
            vec![
                "Check that your function's return type annotation matches what you're actually returning".to_string(),
                "Python and Rust may have different type representations".to_string(),
            ],
        ))
    } else {
        Some((
            format!("Type mismatch in {context}: expected '{expected}', found '{found}'"),
            vec![
                "Rust's type system is stricter than Python's".to_string(),
                "Ensure all type annotations are explicit and consistent".to_string(),
            ],
        ))
    }
}

/// Error reporter that collects and displays enhanced errors
pub struct ErrorReporter {
    errors: Vec<EnhancedError>,
    source: String,
    file_path: String,
}

impl ErrorReporter {
    pub fn new(source: String, file_path: String) -> Self {
        Self {
            errors: Vec::new(),
            source,
            file_path,
        }
    }

    pub fn report_error(&mut self, error: ErrorKind) {
        let enhanced = EnhancedError::new(error);
        self.errors.push(enhanced);
    }

    pub fn report_error_at<T: Ranged>(&mut self, error: ErrorKind, node: &T) {
        let enhanced = EnhancedError::from_ast_node(error, node, &self.source).with_location(
            &self.file_path,
            0,
            0,
        ); // Will be overridden by from_ast_node
        self.errors.push(enhanced);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn display_errors(&self) {
        for (i, error) in self.errors.iter().enumerate() {
            if i > 0 {
                println!();
            }
            println!("{}", error);
        }

        if self.errors.len() > 1 {
            println!(
                "\n{}: Found {} errors",
                "summary".red().bold(),
                self.errors.len()
            );
        }
    }

    pub fn into_result<T>(self, value: T) -> Result<T> {
        if self.has_errors() {
            self.display_errors();
            anyhow::bail!("Transpilation failed with {} errors", self.errors.len())
        } else {
            Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_error_display() {
        let error = EnhancedError::new(ErrorKind::UnsupportedFeature("yield".to_string()))
            .with_location("test.py", 5, 10)
            .with_source_line("    yield x")
            .with_suggestion("Use return with a list instead")
            .add_note("Generators are not supported");

        let display = format!("{}", error);
        assert!(display.contains("yield"));
        assert!(display.contains("test.py:5:10"));
        assert!(display.contains("yield x"));
        assert!(display.contains("suggestion"));
    }

    #[test]
    fn test_line_column_calculation() {
        let source = "line1\nline2\nline3";
        assert_eq!(get_line_column(source, 0), (1, 1));
        assert_eq!(get_line_column(source, 6), (2, 1));
        assert_eq!(get_line_column(source, 12), (3, 1));
    }

    #[test]
    fn test_automatic_suggestions() {
        let error = EnhancedError::new(ErrorKind::UnsupportedFeature("yield".to_string()));
        let enhanced = add_automatic_suggestions(error);

        assert!(enhanced.suggestion.is_some());
        assert!(enhanced.suggestion.unwrap().contains("Generator"));
    }

    #[test]
    fn test_string_type_mismatch_suggestion() {
        let error = EnhancedError::new(ErrorKind::TypeMismatch {
            expected: "String".to_string(),
            found: "&str".to_string(),
            context: "return type".to_string(),
        });
        let enhanced = add_automatic_suggestions(error);

        assert!(enhanced.suggestion.is_some());
        let suggestion = enhanced.suggestion.unwrap();
        assert!(suggestion.contains("String type mismatch"));
        assert!(!enhanced.notes.is_empty());
        assert!(enhanced.notes.iter().any(|n| n.contains("&str")));
    }

    #[test]
    fn test_division_type_mismatch_suggestion() {
        let error = EnhancedError::new(ErrorKind::TypeMismatch {
            expected: "f64".to_string(),
            found: "i32".to_string(),
            context: "division result".to_string(),
        });
        let enhanced = add_automatic_suggestions(error);

        assert!(enhanced.suggestion.is_some());
        let suggestion = enhanced.suggestion.unwrap();
        assert!(suggestion.contains("Division result type mismatch"));
        assert!(enhanced.notes.iter().any(|n| n.contains("always float")));
    }

    #[test]
    fn test_option_type_mismatch_suggestion() {
        let error = EnhancedError::new(ErrorKind::TypeMismatch {
            expected: "Option<i32>".to_string(),
            found: "None".to_string(),
            context: "return value".to_string(),
        });
        let enhanced = add_automatic_suggestions(error);

        assert!(enhanced.suggestion.is_some());
        let suggestion = enhanced.suggestion.unwrap();
        assert!(suggestion.contains("None type mismatch"));
        assert!(enhanced.notes.iter().any(|n| n.contains("Option<T>")));
    }

    #[test]
    fn test_ownership_mismatch_suggestion() {
        let error = EnhancedError::new(ErrorKind::TypeMismatch {
            expected: "&String".to_string(),
            found: "String".to_string(),
            context: "parameter".to_string(),
        });
        let enhanced = add_automatic_suggestions(error);

        assert!(enhanced.suggestion.is_some());
        let suggestion = enhanced.suggestion.unwrap();
        assert!(suggestion.contains("Ownership mismatch"));
        assert!(enhanced.notes.iter().any(|n| n.contains("borrowed reference")));
    }
}
