//! Control Statement Code Generation Helpers
//!
//! This module contains pure helper functions for generating Rust code
//! for control flow statements (pass, break, continue).
//! Extracted from stmt_gen.rs for better testability.
//!
//! DEPYLER-0140: Statement code generation helpers

use anyhow::Result;
use quote::quote;

/// Generate code for Pass statement (no-op)
///
/// Python's `pass` statement is a no-op that produces no Rust code.
#[inline]
pub fn codegen_pass_stmt() -> Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}

/// Generate code for Break statement with optional label
///
/// Converts Python `break` to Rust `break` or `break 'label`.
/// Labels are used for breaking out of nested loops.
#[inline]
pub fn codegen_break_stmt(label: &Option<String>) -> Result<proc_macro2::TokenStream> {
    if let Some(label_name) = label {
        let label_ident =
            syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
        Ok(quote! { break #label_ident; })
    } else {
        Ok(quote! { break; })
    }
}

/// Generate code for Continue statement with optional label
///
/// Converts Python `continue` to Rust `continue` or `continue 'label`.
/// Labels are used for continuing in nested loops.
#[inline]
pub fn codegen_continue_stmt(label: &Option<String>) -> Result<proc_macro2::TokenStream> {
    if let Some(label_name) = label {
        let label_ident =
            syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
        Ok(quote! { continue #label_ident; })
    } else {
        Ok(quote! { continue; })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to convert TokenStream to normalized string for comparison
    fn tokens_to_string(tokens: proc_macro2::TokenStream) -> String {
        tokens.to_string()
    }

    // ============ codegen_pass_stmt tests ============

    #[test]
    fn test_pass_stmt_returns_ok() {
        let result = codegen_pass_stmt();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pass_stmt_is_empty() {
        let result = codegen_pass_stmt().unwrap();
        assert!(tokens_to_string(result).is_empty());
    }

    #[test]
    fn test_pass_stmt_multiple_calls_consistent() {
        let result1 = codegen_pass_stmt().unwrap();
        let result2 = codegen_pass_stmt().unwrap();
        assert_eq!(tokens_to_string(result1), tokens_to_string(result2));
    }

    // ============ codegen_break_stmt tests - no label ============

    #[test]
    fn test_break_stmt_no_label_returns_ok() {
        let result = codegen_break_stmt(&None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_break_stmt_no_label_generates_break() {
        let result = codegen_break_stmt(&None).unwrap();
        assert_eq!(tokens_to_string(result), "break ;");
    }

    #[test]
    fn test_break_stmt_no_label_multiple_calls_consistent() {
        let result1 = codegen_break_stmt(&None).unwrap();
        let result2 = codegen_break_stmt(&None).unwrap();
        assert_eq!(tokens_to_string(result1), tokens_to_string(result2));
    }

    // ============ codegen_break_stmt tests - with label ============

    #[test]
    fn test_break_stmt_with_label_returns_ok() {
        let result = codegen_break_stmt(&Some("outer".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_break_stmt_with_label_outer() {
        let result = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "break 'outer ;");
    }

    #[test]
    fn test_break_stmt_with_label_inner() {
        let result = codegen_break_stmt(&Some("inner".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "break 'inner ;");
    }

    #[test]
    fn test_break_stmt_with_label_loop() {
        let result = codegen_break_stmt(&Some("loop".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "break 'loop ;");
    }

    #[test]
    fn test_break_stmt_with_numeric_label() {
        let result = codegen_break_stmt(&Some("loop1".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "break 'loop1 ;");
    }

    #[test]
    fn test_break_stmt_with_underscore_label() {
        let result = codegen_break_stmt(&Some("outer_loop".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "break 'outer_loop ;");
    }

    #[test]
    fn test_break_stmt_with_single_char_label() {
        let result = codegen_break_stmt(&Some("a".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "break 'a ;");
    }

    #[test]
    fn test_break_stmt_with_long_label() {
        let result =
            codegen_break_stmt(&Some("very_long_label_name_for_loop".to_string())).unwrap();
        assert_eq!(
            tokens_to_string(result),
            "break 'very_long_label_name_for_loop ;"
        );
    }

    // ============ codegen_continue_stmt tests - no label ============

    #[test]
    fn test_continue_stmt_no_label_returns_ok() {
        let result = codegen_continue_stmt(&None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_continue_stmt_no_label_generates_continue() {
        let result = codegen_continue_stmt(&None).unwrap();
        assert_eq!(tokens_to_string(result), "continue ;");
    }

    #[test]
    fn test_continue_stmt_no_label_multiple_calls_consistent() {
        let result1 = codegen_continue_stmt(&None).unwrap();
        let result2 = codegen_continue_stmt(&None).unwrap();
        assert_eq!(tokens_to_string(result1), tokens_to_string(result2));
    }

    // ============ codegen_continue_stmt tests - with label ============

    #[test]
    fn test_continue_stmt_with_label_returns_ok() {
        let result = codegen_continue_stmt(&Some("outer".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_continue_stmt_with_label_outer() {
        let result = codegen_continue_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "continue 'outer ;");
    }

    #[test]
    fn test_continue_stmt_with_label_inner() {
        let result = codegen_continue_stmt(&Some("inner".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "continue 'inner ;");
    }

    #[test]
    fn test_continue_stmt_with_label_loop() {
        let result = codegen_continue_stmt(&Some("loop".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "continue 'loop ;");
    }

    #[test]
    fn test_continue_stmt_with_numeric_label() {
        let result = codegen_continue_stmt(&Some("loop2".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "continue 'loop2 ;");
    }

    #[test]
    fn test_continue_stmt_with_underscore_label() {
        let result = codegen_continue_stmt(&Some("inner_loop".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "continue 'inner_loop ;");
    }

    #[test]
    fn test_continue_stmt_with_single_char_label() {
        let result = codegen_continue_stmt(&Some("x".to_string())).unwrap();
        assert_eq!(tokens_to_string(result), "continue 'x ;");
    }

    #[test]
    fn test_continue_stmt_with_long_label() {
        let result =
            codegen_continue_stmt(&Some("very_long_label_name_for_loop".to_string())).unwrap();
        assert_eq!(
            tokens_to_string(result),
            "continue 'very_long_label_name_for_loop ;"
        );
    }

    // ============ Comparison tests ============

    #[test]
    fn test_break_and_continue_no_label_differ() {
        let break_result = codegen_break_stmt(&None).unwrap();
        let continue_result = codegen_continue_stmt(&None).unwrap();
        assert_ne!(
            tokens_to_string(break_result),
            tokens_to_string(continue_result)
        );
    }

    #[test]
    fn test_break_and_continue_with_same_label_differ() {
        let label = Some("outer".to_string());
        let break_result = codegen_break_stmt(&label).unwrap();
        let continue_result = codegen_continue_stmt(&label).unwrap();
        assert_ne!(
            tokens_to_string(break_result),
            tokens_to_string(continue_result)
        );
    }

    #[test]
    fn test_break_with_different_labels_differ() {
        let result1 = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        let result2 = codegen_break_stmt(&Some("inner".to_string())).unwrap();
        assert_ne!(tokens_to_string(result1), tokens_to_string(result2));
    }

    #[test]
    fn test_continue_with_different_labels_differ() {
        let result1 = codegen_continue_stmt(&Some("outer".to_string())).unwrap();
        let result2 = codegen_continue_stmt(&Some("inner".to_string())).unwrap();
        assert_ne!(tokens_to_string(result1), tokens_to_string(result2));
    }

    #[test]
    fn test_break_with_and_without_label_differ() {
        let with_label = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        let without_label = codegen_break_stmt(&None).unwrap();
        assert_ne!(
            tokens_to_string(with_label),
            tokens_to_string(without_label)
        );
    }

    #[test]
    fn test_continue_with_and_without_label_differ() {
        let with_label = codegen_continue_stmt(&Some("outer".to_string())).unwrap();
        let without_label = codegen_continue_stmt(&None).unwrap();
        assert_ne!(
            tokens_to_string(with_label),
            tokens_to_string(without_label)
        );
    }

    // ============ Token structure tests ============

    #[test]
    fn test_break_no_label_token_count() {
        let result = codegen_break_stmt(&None).unwrap();
        // "break ;" = 2 tokens (break, ;)
        let token_count = result.into_iter().count();
        assert_eq!(token_count, 2);
    }

    #[test]
    fn test_break_with_label_token_count() {
        let result = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        // "break 'outer ;" = 4 tokens (break, ', outer, ;)
        // Note: quote! tokenizes lifetime as multiple tokens
        let token_count = result.into_iter().count();
        assert_eq!(token_count, 4);
    }

    #[test]
    fn test_continue_no_label_token_count() {
        let result = codegen_continue_stmt(&None).unwrap();
        // "continue ;" = 2 tokens (continue, ;)
        let token_count = result.into_iter().count();
        assert_eq!(token_count, 2);
    }

    #[test]
    fn test_continue_with_label_token_count() {
        let result = codegen_continue_stmt(&Some("outer".to_string())).unwrap();
        // "continue 'outer ;" = 4 tokens (continue, ', outer, ;)
        let token_count = result.into_iter().count();
        assert_eq!(token_count, 4);
    }

    #[test]
    fn test_pass_stmt_token_count() {
        let result = codegen_pass_stmt().unwrap();
        // Empty = 0 tokens
        let token_count = result.into_iter().count();
        assert_eq!(token_count, 0);
    }
}

// ============ Additional coverage tests ============

#[cfg(test)]
mod coverage_tests {
    use super::*;

    fn tokens_to_string(tokens: proc_macro2::TokenStream) -> String {
        tokens.to_string()
    }

    // ============ Pass stmt idempotency and invariants ============

    #[test]
    fn test_pass_stmt_is_deterministic() {
        let results: Vec<String> = (0..10)
            .map(|_| tokens_to_string(codegen_pass_stmt().unwrap()))
            .collect();
        assert!(results.iter().all(|r| r == &results[0]));
    }

    #[test]
    fn test_pass_stmt_produces_no_semicolons() {
        let result = tokens_to_string(codegen_pass_stmt().unwrap());
        assert!(!result.contains(';'));
    }

    #[test]
    fn test_pass_stmt_produces_no_keywords() {
        let result = tokens_to_string(codegen_pass_stmt().unwrap());
        assert!(!result.contains("pass"));
        assert!(!result.contains("break"));
        assert!(!result.contains("continue"));
    }

    // ============ Break stmt content checks ============

    #[test]
    fn test_break_no_label_contains_break_keyword() {
        let result = tokens_to_string(codegen_break_stmt(&None).unwrap());
        assert!(result.contains("break"));
    }

    #[test]
    fn test_break_no_label_does_not_contain_continue() {
        let result = tokens_to_string(codegen_break_stmt(&None).unwrap());
        assert!(!result.contains("continue"));
    }

    #[test]
    fn test_break_with_label_contains_lifetime_tick() {
        let result = tokens_to_string(codegen_break_stmt(&Some("my_loop".to_string())).unwrap());
        assert!(result.contains("'my_loop"));
    }

    #[test]
    fn test_break_with_label_contains_semicolon() {
        let result = tokens_to_string(codegen_break_stmt(&Some("x".to_string())).unwrap());
        assert!(result.contains(';'));
    }

    // ============ Continue stmt content checks ============

    #[test]
    fn test_continue_no_label_contains_continue_keyword() {
        let result = tokens_to_string(codegen_continue_stmt(&None).unwrap());
        assert!(result.contains("continue"));
    }

    #[test]
    fn test_continue_no_label_does_not_contain_break() {
        let result = tokens_to_string(codegen_continue_stmt(&None).unwrap());
        assert!(!result.contains("break"));
    }

    #[test]
    fn test_continue_with_label_contains_lifetime_tick() {
        let result =
            tokens_to_string(codegen_continue_stmt(&Some("my_loop".to_string())).unwrap());
        assert!(result.contains("'my_loop"));
    }

    #[test]
    fn test_continue_with_label_contains_semicolon() {
        let result = tokens_to_string(codegen_continue_stmt(&Some("x".to_string())).unwrap());
        assert!(result.contains(';'));
    }

    // ============ Determinism across many labels ============

    #[test]
    fn test_break_deterministic_with_same_label() {
        let label = Some("outer".to_string());
        let results: Vec<String> = (0..5)
            .map(|_| tokens_to_string(codegen_break_stmt(&label).unwrap()))
            .collect();
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }

    #[test]
    fn test_continue_deterministic_with_same_label() {
        let label = Some("inner".to_string());
        let results: Vec<String> = (0..5)
            .map(|_| tokens_to_string(codegen_continue_stmt(&label).unwrap()))
            .collect();
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }

    // ============ Various label names ============

    #[test]
    fn test_break_with_label_starting_with_underscore() {
        let result =
            tokens_to_string(codegen_break_stmt(&Some("_private".to_string())).unwrap());
        assert_eq!(result, "break '_private ;");
    }

    #[test]
    fn test_continue_with_label_starting_with_underscore() {
        let result =
            tokens_to_string(codegen_continue_stmt(&Some("_private".to_string())).unwrap());
        assert_eq!(result, "continue '_private ;");
    }

    #[test]
    fn test_break_with_label_all_underscores() {
        let result = tokens_to_string(codegen_break_stmt(&Some("___".to_string())).unwrap());
        assert_eq!(result, "break '___ ;");
    }

    #[test]
    fn test_continue_with_label_all_underscores() {
        let result =
            tokens_to_string(codegen_continue_stmt(&Some("___".to_string())).unwrap());
        assert_eq!(result, "continue '___ ;");
    }

    // ============ String length of generated tokens ============

    #[test]
    fn test_pass_stmt_string_length_zero() {
        let result = tokens_to_string(codegen_pass_stmt().unwrap());
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_break_no_label_string_length() {
        let result = tokens_to_string(codegen_break_stmt(&None).unwrap());
        assert_eq!(result, "break ;");
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_continue_no_label_string_length() {
        let result = tokens_to_string(codegen_continue_stmt(&None).unwrap());
        assert_eq!(result, "continue ;");
        assert_eq!(result.len(), 10);
    }

    // ============ Combined statement generation ============

    #[test]
    fn test_all_three_stmts_produce_valid_results() {
        let pass = codegen_pass_stmt();
        let brk = codegen_break_stmt(&None);
        let cont = codegen_continue_stmt(&None);

        assert!(pass.is_ok());
        assert!(brk.is_ok());
        assert!(cont.is_ok());
    }

    #[test]
    fn test_all_three_stmts_different_output() {
        let pass = tokens_to_string(codegen_pass_stmt().unwrap());
        let brk = tokens_to_string(codegen_break_stmt(&None).unwrap());
        let cont = tokens_to_string(codegen_continue_stmt(&None).unwrap());

        assert_ne!(pass, brk);
        assert_ne!(pass, cont);
        assert_ne!(brk, cont);
    }

    #[test]
    fn test_labeled_break_and_continue_same_label_differ() {
        let label = Some("same_label".to_string());
        let brk = tokens_to_string(codegen_break_stmt(&label).unwrap());
        let cont = tokens_to_string(codegen_continue_stmt(&label).unwrap());
        assert_ne!(brk, cont);
        assert!(brk.starts_with("break"));
        assert!(cont.starts_with("continue"));
    }
}
