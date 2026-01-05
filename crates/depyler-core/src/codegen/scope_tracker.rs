//! Scope tracking for variable declarations in code generation
//!
//! This module provides utilities to track variable declarations across
//! nested scopes, ensuring correct `let` vs reassignment generation in Rust.

use std::collections::HashSet;

/// Tracks variable declarations across nested scopes
///
/// Used during code generation to determine whether a variable assignment
/// should generate `let mut x = ...` (new declaration) or `x = ...` (reassignment).
#[derive(Debug, Clone)]
pub struct ScopeTracker {
    declared_vars: Vec<HashSet<String>>,
}

impl Default for ScopeTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeTracker {
    /// Create a new scope tracker with a single empty scope
    pub fn new() -> Self {
        Self {
            declared_vars: vec![HashSet::new()],
        }
    }

    /// Enter a new nested scope
    ///
    /// Variables declared in this scope will be removed when `exit_scope` is called.
    pub fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    /// Exit the current scope
    ///
    /// Returns `true` if a scope was exited, `false` if already at the outermost scope.
    pub fn exit_scope(&mut self) -> bool {
        if self.declared_vars.len() > 1 {
            self.declared_vars.pop();
            true
        } else {
            false
        }
    }

    /// Check if a variable is declared in any enclosing scope
    pub fn is_declared(&self, var_name: &str) -> bool {
        self.declared_vars
            .iter()
            .any(|scope| scope.contains(var_name))
    }

    /// Check if a variable is declared in the current (innermost) scope only
    pub fn is_declared_in_current_scope(&self, var_name: &str) -> bool {
        self.declared_vars
            .last()
            .is_some_and(|scope| scope.contains(var_name))
    }

    /// Declare a variable in the current scope
    ///
    /// Returns `true` if the variable was newly declared, `false` if it was already declared.
    pub fn declare_var(&mut self, var_name: &str) -> bool {
        if let Some(current_scope) = self.declared_vars.last_mut() {
            current_scope.insert(var_name.to_string())
        } else {
            false
        }
    }

    /// Get the number of nested scopes (1 = outermost only)
    pub fn scope_depth(&self) -> usize {
        self.declared_vars.len()
    }

    /// Get all declared variables across all scopes
    pub fn all_declared_vars(&self) -> HashSet<String> {
        self.declared_vars
            .iter()
            .flat_map(|scope| scope.iter().cloned())
            .collect()
    }

    /// Get variables declared in the current scope only
    pub fn current_scope_vars(&self) -> Option<&HashSet<String>> {
        self.declared_vars.last()
    }

    /// Declare multiple variables at once in the current scope
    ///
    /// Useful for tuple unpacking: `let (a, b, c) = ...`
    pub fn declare_vars<I>(&mut self, var_names: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for var in var_names {
            self.declare_var(var.as_ref());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ new / Default tests ============

    #[test]
    fn test_new_creates_single_scope() {
        let tracker = ScopeTracker::new();
        assert_eq!(tracker.scope_depth(), 1);
    }

    #[test]
    fn test_default_creates_single_scope() {
        let tracker = ScopeTracker::default();
        assert_eq!(tracker.scope_depth(), 1);
    }

    #[test]
    fn test_new_scope_is_empty() {
        let tracker = ScopeTracker::new();
        assert!(!tracker.is_declared("x"));
    }

    // ============ declare_var tests ============

    #[test]
    fn test_declare_var_returns_true_for_new() {
        let mut tracker = ScopeTracker::new();
        assert!(tracker.declare_var("x"));
    }

    #[test]
    fn test_declare_var_returns_false_for_existing() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("x");
        assert!(!tracker.declare_var("x"));
    }

    #[test]
    fn test_declared_var_is_found() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("x");
        assert!(tracker.is_declared("x"));
    }

    #[test]
    fn test_undeclared_var_is_not_found() {
        let tracker = ScopeTracker::new();
        assert!(!tracker.is_declared("y"));
    }

    // ============ enter_scope / exit_scope tests ============

    #[test]
    fn test_enter_scope_increases_depth() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        assert_eq!(tracker.scope_depth(), 2);
    }

    #[test]
    fn test_exit_scope_decreases_depth() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        tracker.exit_scope();
        assert_eq!(tracker.scope_depth(), 1);
    }

    #[test]
    fn test_exit_scope_returns_true_when_nested() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        assert!(tracker.exit_scope());
    }

    #[test]
    fn test_exit_scope_returns_false_at_outermost() {
        let mut tracker = ScopeTracker::new();
        assert!(!tracker.exit_scope());
    }

    #[test]
    fn test_exit_scope_removes_vars() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        tracker.declare_var("inner");
        assert!(tracker.is_declared("inner"));
        tracker.exit_scope();
        assert!(!tracker.is_declared("inner"));
    }

    #[test]
    fn test_parent_scope_vars_visible_in_child() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("outer");
        tracker.enter_scope();
        assert!(tracker.is_declared("outer"));
    }

    #[test]
    fn test_parent_scope_vars_survive_exit() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("outer");
        tracker.enter_scope();
        tracker.declare_var("inner");
        tracker.exit_scope();
        assert!(tracker.is_declared("outer"));
    }

    // ============ is_declared_in_current_scope tests ============

    #[test]
    fn test_is_declared_in_current_scope_true() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("x");
        assert!(tracker.is_declared_in_current_scope("x"));
    }

    #[test]
    fn test_is_declared_in_current_scope_false_for_parent() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("outer");
        tracker.enter_scope();
        assert!(!tracker.is_declared_in_current_scope("outer"));
        assert!(tracker.is_declared("outer")); // but it's visible
    }

    #[test]
    fn test_is_declared_in_current_scope_false_for_undeclared() {
        let tracker = ScopeTracker::new();
        assert!(!tracker.is_declared_in_current_scope("x"));
    }

    // ============ all_declared_vars tests ============

    #[test]
    fn test_all_declared_vars_empty() {
        let tracker = ScopeTracker::new();
        assert!(tracker.all_declared_vars().is_empty());
    }

    #[test]
    fn test_all_declared_vars_single_scope() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("a");
        tracker.declare_var("b");
        let vars = tracker.all_declared_vars();
        assert!(vars.contains("a"));
        assert!(vars.contains("b"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_all_declared_vars_multiple_scopes() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("outer");
        tracker.enter_scope();
        tracker.declare_var("inner");
        let vars = tracker.all_declared_vars();
        assert!(vars.contains("outer"));
        assert!(vars.contains("inner"));
        assert_eq!(vars.len(), 2);
    }

    // ============ current_scope_vars tests ============

    #[test]
    fn test_current_scope_vars_returns_some() {
        let tracker = ScopeTracker::new();
        assert!(tracker.current_scope_vars().is_some());
    }

    #[test]
    fn test_current_scope_vars_contains_declared() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("x");
        let vars = tracker.current_scope_vars().unwrap();
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_current_scope_vars_excludes_parent() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("outer");
        tracker.enter_scope();
        tracker.declare_var("inner");
        let vars = tracker.current_scope_vars().unwrap();
        assert!(!vars.contains("outer"));
        assert!(vars.contains("inner"));
    }

    // ============ declare_vars tests ============

    #[test]
    fn test_declare_vars_multiple() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_vars(["a", "b", "c"]);
        assert!(tracker.is_declared("a"));
        assert!(tracker.is_declared("b"));
        assert!(tracker.is_declared("c"));
    }

    #[test]
    fn test_declare_vars_empty() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_vars(Vec::<&str>::new());
        assert!(tracker.all_declared_vars().is_empty());
    }

    #[test]
    fn test_declare_vars_with_strings() {
        let mut tracker = ScopeTracker::new();
        let vars = vec!["x".to_string(), "y".to_string()];
        tracker.declare_vars(vars);
        assert!(tracker.is_declared("x"));
        assert!(tracker.is_declared("y"));
    }

    // ============ scope_depth tests ============

    #[test]
    fn test_scope_depth_nested() {
        let mut tracker = ScopeTracker::new();
        assert_eq!(tracker.scope_depth(), 1);
        tracker.enter_scope();
        assert_eq!(tracker.scope_depth(), 2);
        tracker.enter_scope();
        assert_eq!(tracker.scope_depth(), 3);
        tracker.exit_scope();
        assert_eq!(tracker.scope_depth(), 2);
        tracker.exit_scope();
        assert_eq!(tracker.scope_depth(), 1);
    }

    // ============ shadowing tests ============

    #[test]
    fn test_redeclare_in_nested_scope() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("x");
        tracker.enter_scope();
        // Redeclaring in nested scope shadows
        assert!(tracker.declare_var("x"));
        assert!(tracker.is_declared_in_current_scope("x"));
    }

    #[test]
    fn test_shadow_persists_only_in_scope() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("x");
        tracker.enter_scope();
        tracker.declare_var("x"); // shadow
        tracker.exit_scope();
        // Original x still visible from outer scope
        assert!(tracker.is_declared("x"));
    }

    // ============ complex scenario tests ============

    #[test]
    fn test_deeply_nested_scopes() {
        let mut tracker = ScopeTracker::new();
        tracker.declare_var("a");
        tracker.enter_scope();
        tracker.declare_var("b");
        tracker.enter_scope();
        tracker.declare_var("c");
        tracker.enter_scope();
        tracker.declare_var("d");

        // All visible
        assert!(tracker.is_declared("a"));
        assert!(tracker.is_declared("b"));
        assert!(tracker.is_declared("c"));
        assert!(tracker.is_declared("d"));

        // Only d in current scope
        assert!(tracker.is_declared_in_current_scope("d"));
        assert!(!tracker.is_declared_in_current_scope("a"));

        // Exit scopes one by one
        tracker.exit_scope();
        assert!(!tracker.is_declared("d"));
        tracker.exit_scope();
        assert!(!tracker.is_declared("c"));
        tracker.exit_scope();
        assert!(!tracker.is_declared("b"));
        assert!(tracker.is_declared("a"));
    }

    #[test]
    fn test_function_like_usage() {
        // Simulate: fn foo(x, y) { let z = ...; if cond { let w = ...; } }
        let mut tracker = ScopeTracker::new();

        // Function parameters
        tracker.declare_vars(["x", "y"]);

        // Local variable
        tracker.declare_var("z");

        // If block
        tracker.enter_scope();
        tracker.declare_var("w");
        assert!(tracker.is_declared("w"));
        assert!(tracker.is_declared("x")); // params visible
        tracker.exit_scope();

        // w no longer visible
        assert!(!tracker.is_declared("w"));
        assert!(tracker.is_declared("x")); // params still visible
        assert!(tracker.is_declared("z")); // local still visible
    }
}
