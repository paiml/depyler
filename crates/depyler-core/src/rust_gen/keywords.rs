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

/// Check if a keyword cannot be used as a raw identifier
///
/// These special keywords (self, Self, super, crate) cannot use r# syntax.
/// They are reserved in Rust and must be avoided as variable names.
///
/// # Examples
/// ```ignore
/// use depyler_core::rust_gen::keywords::is_non_raw_keyword;
///
/// assert!(is_non_raw_keyword("self"));   // Cannot use r#self
/// assert!(is_non_raw_keyword("Self"));   // Cannot use r#Self
/// assert!(is_non_raw_keyword("super"));  // Cannot use r#super
/// assert!(is_non_raw_keyword("crate"));  // Cannot use r#crate
/// assert!(!is_non_raw_keyword("match")); // Can use r#match
/// ```
#[inline]
pub fn is_non_raw_keyword(name: &str) -> bool {
    matches!(name, "self" | "Self" | "super" | "crate")
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

    // ============ is_rust_keyword - strict keywords ============

    #[test]
    fn test_is_rust_keyword_control_flow() {
        assert!(is_rust_keyword("if"));
        assert!(is_rust_keyword("else"));
        assert!(is_rust_keyword("for"));
        assert!(is_rust_keyword("while"));
        assert!(is_rust_keyword("loop"));
        assert!(is_rust_keyword("break"));
        assert!(is_rust_keyword("continue"));
        assert!(is_rust_keyword("return"));
    }

    #[test]
    fn test_is_rust_keyword_declarations() {
        assert!(is_rust_keyword("fn"));
        assert!(is_rust_keyword("let"));
        assert!(is_rust_keyword("const"));
        assert!(is_rust_keyword("static"));
        assert!(is_rust_keyword("struct"));
        assert!(is_rust_keyword("enum"));
        assert!(is_rust_keyword("trait"));
        assert!(is_rust_keyword("impl"));
        assert!(is_rust_keyword("type"));
        assert!(is_rust_keyword("mod"));
        assert!(is_rust_keyword("use"));
    }

    #[test]
    fn test_is_rust_keyword_modifiers() {
        assert!(is_rust_keyword("pub"));
        assert!(is_rust_keyword("mut"));
        assert!(is_rust_keyword("ref"));
        assert!(is_rust_keyword("move"));
        assert!(is_rust_keyword("unsafe"));
        assert!(is_rust_keyword("extern"));
        assert!(is_rust_keyword("dyn"));
    }

    #[test]
    fn test_is_rust_keyword_special() {
        assert!(is_rust_keyword("self"));
        assert!(is_rust_keyword("Self"));
        assert!(is_rust_keyword("super"));
        assert!(is_rust_keyword("crate"));
        assert!(is_rust_keyword("as"));
        assert!(is_rust_keyword("in"));
        assert!(is_rust_keyword("where"));
    }

    #[test]
    fn test_is_rust_keyword_async() {
        assert!(is_rust_keyword("async"));
        assert!(is_rust_keyword("await"));
    }

    #[test]
    fn test_is_rust_keyword_literals() {
        assert!(is_rust_keyword("true"));
        assert!(is_rust_keyword("false"));
    }

    #[test]
    fn test_is_rust_keyword_reserved_future() {
        // Reserved for future use
        assert!(is_rust_keyword("abstract"));
        assert!(is_rust_keyword("become"));
        assert!(is_rust_keyword("box"));
        assert!(is_rust_keyword("do"));
        assert!(is_rust_keyword("final"));
        assert!(is_rust_keyword("macro"));
        assert!(is_rust_keyword("override"));
        assert!(is_rust_keyword("priv"));
        assert!(is_rust_keyword("typeof"));
        assert!(is_rust_keyword("unsized"));
        assert!(is_rust_keyword("virtual"));
        assert!(is_rust_keyword("yield"));
        assert!(is_rust_keyword("try"));
    }

    #[test]
    fn test_is_rust_keyword_non_keywords() {
        // Common identifiers that are NOT keywords
        assert!(!is_rust_keyword("value"));
        assert!(!is_rust_keyword("result"));
        assert!(!is_rust_keyword("data"));
        assert!(!is_rust_keyword("name"));
        assert!(!is_rust_keyword("count"));
        assert!(!is_rust_keyword("index"));
        assert!(!is_rust_keyword("item"));
        assert!(!is_rust_keyword("key"));
        assert!(!is_rust_keyword("x"));
        assert!(!is_rust_keyword("y"));
        assert!(!is_rust_keyword("i"));
        assert!(!is_rust_keyword("n"));
    }

    #[test]
    fn test_is_rust_keyword_similar_but_not_keywords() {
        // Similar to keywords but not actual keywords
        assert!(!is_rust_keyword("If"));      // Capitalized
        assert!(!is_rust_keyword("IF"));      // All caps
        assert!(!is_rust_keyword("lets"));    // Similar to let
        assert!(!is_rust_keyword("fns"));     // Similar to fn
        assert!(!is_rust_keyword("matches")); // Similar to match
        assert!(!is_rust_keyword("types"));   // Similar to type
        assert!(!is_rust_keyword("impls"));   // Similar to impl
        assert!(!is_rust_keyword("loops"));   // Similar to loop
        assert!(!is_rust_keyword("_"));       // Underscore
        assert!(!is_rust_keyword("__"));      // Double underscore
    }

    #[test]
    fn test_is_rust_keyword_empty_string() {
        assert!(!is_rust_keyword(""));
    }

    #[test]
    fn test_is_rust_keyword_whitespace() {
        assert!(!is_rust_keyword(" "));
        assert!(!is_rust_keyword("  "));
        assert!(!is_rust_keyword(" fn"));  // Leading space
        assert!(!is_rust_keyword("fn "));  // Trailing space
        assert!(!is_rust_keyword(" fn ")); // Both
    }

    // ============ is_non_raw_keyword tests ============

    #[test]
    fn test_is_non_raw_keyword_self_lowercase() {
        assert!(is_non_raw_keyword("self"));
    }

    #[test]
    fn test_is_non_raw_keyword_self_titlecase() {
        assert!(is_non_raw_keyword("Self"));
    }

    #[test]
    fn test_is_non_raw_keyword_super() {
        assert!(is_non_raw_keyword("super"));
    }

    #[test]
    fn test_is_non_raw_keyword_crate() {
        assert!(is_non_raw_keyword("crate"));
    }

    #[test]
    fn test_is_non_raw_keyword_all_four() {
        // All four non-raw keywords
        let non_raw = ["self", "Self", "super", "crate"];
        for kw in non_raw {
            assert!(
                is_non_raw_keyword(kw),
                "{} should be a non-raw keyword",
                kw
            );
        }
    }

    #[test]
    fn test_is_non_raw_keyword_regular_keywords_are_not_non_raw() {
        // Regular keywords CAN use raw identifier syntax
        assert!(!is_non_raw_keyword("match"));
        assert!(!is_non_raw_keyword("type"));
        assert!(!is_non_raw_keyword("impl"));
        assert!(!is_non_raw_keyword("fn"));
        assert!(!is_non_raw_keyword("let"));
        assert!(!is_non_raw_keyword("if"));
        assert!(!is_non_raw_keyword("else"));
        assert!(!is_non_raw_keyword("for"));
        assert!(!is_non_raw_keyword("while"));
        assert!(!is_non_raw_keyword("loop"));
        assert!(!is_non_raw_keyword("break"));
        assert!(!is_non_raw_keyword("continue"));
        assert!(!is_non_raw_keyword("return"));
        assert!(!is_non_raw_keyword("async"));
        assert!(!is_non_raw_keyword("await"));
        assert!(!is_non_raw_keyword("try"));
    }

    #[test]
    fn test_is_non_raw_keyword_non_keywords() {
        // Non-keywords are definitely not non-raw keywords
        assert!(!is_non_raw_keyword("value"));
        assert!(!is_non_raw_keyword("result"));
        assert!(!is_non_raw_keyword("data"));
        assert!(!is_non_raw_keyword("name"));
        assert!(!is_non_raw_keyword("x"));
    }

    #[test]
    fn test_is_non_raw_keyword_similar_strings() {
        // Similar but not exactly the non-raw keywords
        assert!(!is_non_raw_keyword("SELF"));     // All caps
        assert!(!is_non_raw_keyword("selfs"));    // Extra s
        assert!(!is_non_raw_keyword("_self"));    // Leading underscore
        assert!(!is_non_raw_keyword("self_"));    // Trailing underscore
        assert!(!is_non_raw_keyword("SUPER"));    // All caps
        assert!(!is_non_raw_keyword("Super"));    // Title case
        assert!(!is_non_raw_keyword("supers"));   // Extra s
        assert!(!is_non_raw_keyword("CRATE"));    // All caps
        assert!(!is_non_raw_keyword("Crate"));    // Title case
        assert!(!is_non_raw_keyword("crates"));   // Extra s
    }

    #[test]
    fn test_is_non_raw_keyword_empty_string() {
        assert!(!is_non_raw_keyword(""));
    }

    #[test]
    fn test_is_non_raw_keyword_whitespace() {
        assert!(!is_non_raw_keyword(" "));
        assert!(!is_non_raw_keyword(" self"));
        assert!(!is_non_raw_keyword("self "));
    }

    #[test]
    fn test_non_raw_keywords_are_also_keywords() {
        // All non-raw keywords should also be regular keywords
        let non_raw = ["self", "Self", "super", "crate"];
        for kw in non_raw {
            assert!(
                is_rust_keyword(kw),
                "{} should be both a keyword and a non-raw keyword",
                kw
            );
            assert!(
                is_non_raw_keyword(kw),
                "{} should be both a keyword and a non-raw keyword",
                kw
            );
        }
    }

    #[test]
    fn test_non_raw_keywords_subset_of_keywords() {
        // Every non-raw keyword must be a keyword (inverse not true)
        let test_strings = [
            "self", "Self", "super", "crate", "match", "type", "fn", "let", "value", "",
        ];
        for s in test_strings {
            if is_non_raw_keyword(s) {
                assert!(
                    is_rust_keyword(s),
                    "non-raw keyword {} must also be a rust keyword",
                    s
                );
            }
        }
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
