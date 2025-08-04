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
}

impl fmt::Display for EnhancedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Error header
        writeln!(f, "{}: {}", "error".red().bold(), self.error)?;

        // Location info
        if let (Some(file), Some(line), Some(column)) = (&self.file_path, self.line, self.column) {
            writeln!(f, "  {} {}:{}:{}", "-->".blue().bold(), file, line, column)?;
        }

        // Source context
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

        // Suggestion
        if let Some(suggestion) = &self.suggestion {
            writeln!(f, "\n{}: {}", "suggestion".green().bold(), suggestion)?;
        }

        // Notes
        for note in &self.notes {
            writeln!(f, "  {} {}", "note:".yellow(), note)?;
        }

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
fn add_automatic_suggestions(mut error: EnhancedError) -> EnhancedError {
    let suggestion = match &error.error {
        ErrorKind::UnsupportedFeature(feature) => {
            let base_suggestion = format!(
                "This Python feature '{}' is not yet supported. Consider using a simpler construct.",
                feature
            );

            // Add specific suggestions for common features
            match feature.as_str() {
                "yield" => Some((
                    "Generator functions are not supported. Consider returning a list instead."
                        .to_string(),
                    vec![
                        "Example: Instead of 'yield x', collect values and 'return values'"
                            .to_string(),
                    ],
                )),
                "async for" => Some((
                    "Async iteration is not supported. Use regular iteration with async/await."
                        .to_string(),
                    vec![],
                )),
                "match" => Some((
                    "Pattern matching is not supported. Use if/elif/else chains instead."
                        .to_string(),
                    vec![],
                )),
                _ => Some((base_suggestion, vec![])),
            }
        }
        ErrorKind::TypeInferenceError(msg) => {
            if msg.contains("incompatible types") {
                Some((
                    "Check that all type annotations are correct and consistent.".to_string(),
                    vec!["Rust has stricter type checking than Python".to_string()],
                ))
            } else {
                None
            }
        }
        ErrorKind::InvalidTypeAnnotation(msg) => {
            if msg.contains("borrow") {
                Some((
                    "Consider using .clone() or restructuring to avoid multiple borrows."
                        .to_string(),
                    vec!["Rust's borrow checker ensures memory safety".to_string()],
                ))
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some((suggestion_text, notes)) = suggestion {
        error = error.with_suggestion(&suggestion_text);
        for note in notes {
            error = error.add_note(&note);
        }
    }

    error
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
}
