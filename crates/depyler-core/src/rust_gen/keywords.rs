//! DEPYLER-0023: Rust keyword escaping utilities
//!
//! Centralized module for handling Rust keywords when generating identifiers.
//! Ensures Python variable names that are Rust keywords get properly escaped.

use proc_macro2::Span;
use syn::Ident;

/// Check if a name is a Rust keyword that needs escaping
pub fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}

/// Sanitize a name to be a valid Rust identifier
///
/// Handles:
/// - Rust keywords (via raw identifier r#)
/// - Names starting with digits (prefix with underscore)
/// - Invalid characters (replace with underscore)
/// - Empty names (return "__empty")
fn sanitize_name(name: &str) -> String {
    if name.is_empty() {
        return "__empty".to_string();
    }

    let mut result = String::with_capacity(name.len());
    let mut chars = name.chars().peekable();

    // First character must be letter or underscore
    if let Some(&first) = chars.peek() {
        if first.is_ascii_digit() {
            result.push('_');
        }
    }

    for c in chars {
        if c.is_ascii_alphanumeric() || c == '_' {
            result.push(c);
        } else {
            result.push('_');
        }
    }

    // Handle empty result after sanitization
    if result.is_empty() || result.chars().all(|c| c == '_') {
        return "__sanitized".to_string();
    }

    result
}

/// Create a safe Rust identifier, using raw identifier syntax (r#) if needed
///
/// # Examples
/// ```ignore
/// use depyler_core::rust_gen::keywords::safe_ident;
///
/// let ident = safe_ident("match");  // Creates r#match
/// let ident = safe_ident("value");  // Creates value
/// let ident = safe_ident("2d");     // Creates _2d
/// ```
pub fn safe_ident(name: &str) -> Ident {
    let sanitized = sanitize_name(name);
    if is_rust_keyword(&sanitized) {
        // Use raw identifier syntax: r#match
        Ident::new_raw(&sanitized, Span::call_site())
    } else {
        Ident::new(&sanitized, Span::call_site())
    }
}

/// Try to create a safe Rust identifier, returning None if impossible
///
/// DEPYLER-0588: This is used for fallback code paths where we can't
/// guarantee the name is valid even after sanitization.
pub fn try_safe_ident(name: &str) -> Option<Ident> {
    let sanitized = sanitize_name(name);
    // Extra validation: ensure it's a valid identifier
    if sanitized.is_empty() {
        return None;
    }
    // Attempt to create the ident (should be safe after sanitization)
    std::panic::catch_unwind(|| {
        if is_rust_keyword(&sanitized) {
            Ident::new_raw(&sanitized, Span::call_site())
        } else {
            Ident::new(&sanitized, Span::call_site())
        }
    })
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rust_keyword() {
        assert!(is_rust_keyword("match"));
        assert!(is_rust_keyword("type"));
        assert!(is_rust_keyword("impl"));
        assert!(is_rust_keyword("async"));
        assert!(!is_rust_keyword("value"));
        assert!(!is_rust_keyword("result"));
    }

    #[test]
    fn test_safe_ident_keyword() {
        let ident = safe_ident("match");
        assert_eq!(ident.to_string(), "r#match");
    }

    #[test]
    fn test_safe_ident_non_keyword() {
        let ident = safe_ident("value");
        assert_eq!(ident.to_string(), "value");
    }

    #[test]
    fn test_sanitize_name_empty() {
        assert_eq!(sanitize_name(""), "__empty");
    }

    #[test]
    fn test_sanitize_name_starts_with_digit() {
        assert_eq!(sanitize_name("2d"), "_2d");
        assert_eq!(sanitize_name("123abc"), "_123abc");
    }

    #[test]
    fn test_sanitize_name_invalid_chars() {
        assert_eq!(sanitize_name("foo-bar"), "foo_bar");
        assert_eq!(sanitize_name("foo.bar"), "foo_bar");
        assert_eq!(sanitize_name("foo::bar"), "foo__bar");
    }

    #[test]
    fn test_sanitize_name_all_underscores() {
        assert_eq!(sanitize_name("___"), "__sanitized");
    }

    #[test]
    fn test_safe_ident_digit_prefix() {
        let ident = safe_ident("2d");
        assert_eq!(ident.to_string(), "_2d");
    }

    #[test]
    fn test_try_safe_ident() {
        assert!(try_safe_ident("value").is_some());
        assert!(try_safe_ident("match").is_some());
        assert!(try_safe_ident("2d").is_some());
    }
}
