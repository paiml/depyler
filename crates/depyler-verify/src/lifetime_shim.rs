//! Lifetime Analysis Shim - pure logic separated from I/O
//!
//! Extracts testable logic from lifetime_analysis.rs

use std::collections::{HashMap, HashSet};

/// Borrow state tracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowState {
    Unborrowed,
    SharedBorrow(usize),
    MutableBorrow,
}

impl BorrowState {
    pub fn can_borrow_shared(&self) -> bool {
        matches!(self, Self::Unborrowed | Self::SharedBorrow(_))
    }

    pub fn can_borrow_mutable(&self) -> bool {
        matches!(self, Self::Unborrowed)
    }

    pub fn add_shared_borrow(&mut self) {
        match self {
            Self::Unborrowed => *self = Self::SharedBorrow(1),
            Self::SharedBorrow(count) => *count += 1,
            Self::MutableBorrow => {} // Error case
        }
    }

    pub fn release_shared_borrow(&mut self) {
        if let Self::SharedBorrow(count) = self {
            *count = count.saturating_sub(1);
            if *count == 0 {
                *self = Self::Unborrowed;
            }
        }
    }
}

/// Lifetime scope tracking
#[derive(Debug, Clone, Default)]
pub struct ScopeTracker {
    current_depth: usize,
    variable_scopes: HashMap<String, usize>,
}

impl ScopeTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter_scope(&mut self) {
        self.current_depth += 1;
    }

    pub fn exit_scope(&mut self) -> Vec<String> {
        let depth = self.current_depth;
        let expired: Vec<String> = self
            .variable_scopes
            .iter()
            .filter(|(_, &scope)| scope == depth)
            .map(|(name, _)| name.clone())
            .collect();

        self.variable_scopes.retain(|_, scope| *scope < depth);
        self.current_depth = self.current_depth.saturating_sub(1);
        expired
    }

    pub fn register_variable(&mut self, name: String) {
        self.variable_scopes.insert(name, self.current_depth);
    }

    pub fn is_in_scope(&self, name: &str) -> bool {
        self.variable_scopes.contains_key(name)
    }

    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    pub fn get_variable_depth(&self, name: &str) -> Option<usize> {
        self.variable_scopes.get(name).copied()
    }
}

/// Violation type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    UseAfterMove,
    DanglingReference,
    ConflictingBorrows,
    LifetimeTooShort,
    EscapingReference,
}

impl ViolationType {
    pub fn severity(&self) -> u8 {
        match self {
            Self::UseAfterMove => 5,
            Self::DanglingReference => 5,
            Self::ConflictingBorrows => 4,
            Self::LifetimeTooShort => 3,
            Self::EscapingReference => 4,
        }
    }

    pub fn is_critical(&self) -> bool {
        self.severity() >= 5
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::UseAfterMove => "Variable used after being moved",
            Self::DanglingReference => "Reference to freed memory",
            Self::ConflictingBorrows => "Multiple incompatible borrows",
            Self::LifetimeTooShort => "Reference outlives its data",
            Self::EscapingReference => "Reference escapes its scope",
        }
    }

    pub fn suggestion(&self) -> &'static str {
        match self {
            Self::UseAfterMove => "Consider cloning or borrowing instead of moving",
            Self::DanglingReference => "Ensure the data lives long enough",
            Self::ConflictingBorrows => "Use separate scopes or Rc/RefCell",
            Self::LifetimeTooShort => "Extend the lifetime or copy the data",
            Self::EscapingReference => "Return owned data instead of a reference",
        }
    }
}

/// Move tracking for ownership analysis
#[derive(Debug, Clone, Default)]
pub struct MoveTracker {
    moved_vars: HashSet<String>,
    copy_types: HashSet<String>,
}

impl MoveTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_copy_types(types: &[&str]) -> Self {
        Self {
            moved_vars: HashSet::new(),
            copy_types: types.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn mark_moved(&mut self, var: &str) {
        if !self.is_copy_type(var) {
            self.moved_vars.insert(var.to_string());
        }
    }

    pub fn is_moved(&self, var: &str) -> bool {
        self.moved_vars.contains(var)
    }

    pub fn reassign(&mut self, var: &str) {
        self.moved_vars.remove(var);
    }

    pub fn is_copy_type(&self, var: &str) -> bool {
        self.copy_types.contains(var)
    }

    pub fn register_copy_type(&mut self, ty: &str) {
        self.copy_types.insert(ty.to_string());
    }
}

/// Loop borrow analysis
#[derive(Debug, Clone)]
pub struct LoopBorrowAnalysis {
    pub borrows_in_loop: Vec<String>,
    pub mutations_in_loop: Vec<String>,
    pub potential_invalidations: Vec<(String, String)>,
}

impl LoopBorrowAnalysis {
    pub fn new() -> Self {
        Self {
            borrows_in_loop: Vec::new(),
            mutations_in_loop: Vec::new(),
            potential_invalidations: Vec::new(),
        }
    }

    pub fn add_borrow(&mut self, var: &str) {
        if !self.borrows_in_loop.contains(&var.to_string()) {
            self.borrows_in_loop.push(var.to_string());
        }
    }

    pub fn add_mutation(&mut self, var: &str) {
        if !self.mutations_in_loop.contains(&var.to_string()) {
            self.mutations_in_loop.push(var.to_string());
        }
    }

    pub fn check_invalidations(&mut self) {
        for borrow in &self.borrows_in_loop {
            for mutation in &self.mutations_in_loop {
                if borrow == mutation {
                    self.potential_invalidations
                        .push((borrow.clone(), mutation.clone()));
                }
            }
        }
    }

    pub fn has_invalidations(&self) -> bool {
        !self.potential_invalidations.is_empty()
    }
}

impl Default for LoopBorrowAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

/// Reference escape checker
pub fn check_reference_escape(return_depth: usize, var_depth: usize) -> bool {
    var_depth > return_depth
}

/// Check if a method is mutating
pub fn is_mutating_method(method: &str) -> bool {
    matches!(
        method,
        "push"
            | "pop"
            | "insert"
            | "remove"
            | "clear"
            | "append"
            | "extend"
            | "push_str"
            | "truncate"
            | "drain"
            | "retain"
            | "sort"
            | "sort_by"
            | "reverse"
    )
}

/// Check if two borrow kinds conflict
pub fn borrows_conflict(kind1: &BorrowState, kind2: &BorrowState) -> bool {
    match (kind1, kind2) {
        (BorrowState::MutableBorrow, _) => true,
        (_, BorrowState::MutableBorrow) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BorrowState tests
    #[test]
    fn test_borrow_state_can_borrow_shared() {
        assert!(BorrowState::Unborrowed.can_borrow_shared());
        assert!(BorrowState::SharedBorrow(1).can_borrow_shared());
        assert!(!BorrowState::MutableBorrow.can_borrow_shared());
    }

    #[test]
    fn test_borrow_state_can_borrow_mutable() {
        assert!(BorrowState::Unborrowed.can_borrow_mutable());
        assert!(!BorrowState::SharedBorrow(1).can_borrow_mutable());
        assert!(!BorrowState::MutableBorrow.can_borrow_mutable());
    }

    #[test]
    fn test_borrow_state_add_shared() {
        let mut state = BorrowState::Unborrowed;
        state.add_shared_borrow();
        assert_eq!(state, BorrowState::SharedBorrow(1));

        state.add_shared_borrow();
        assert_eq!(state, BorrowState::SharedBorrow(2));
    }

    #[test]
    fn test_borrow_state_release_shared() {
        let mut state = BorrowState::SharedBorrow(2);
        state.release_shared_borrow();
        assert_eq!(state, BorrowState::SharedBorrow(1));

        state.release_shared_borrow();
        assert_eq!(state, BorrowState::Unborrowed);
    }

    #[test]
    fn test_borrow_state_release_from_unborrowed() {
        let mut state = BorrowState::Unborrowed;
        state.release_shared_borrow(); // Should not panic
        assert_eq!(state, BorrowState::Unborrowed);
    }

    // ScopeTracker tests
    #[test]
    fn test_scope_tracker_new() {
        let tracker = ScopeTracker::new();
        assert_eq!(tracker.current_depth(), 0);
    }

    #[test]
    fn test_scope_tracker_enter_exit() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        assert_eq!(tracker.current_depth(), 1);

        tracker.enter_scope();
        assert_eq!(tracker.current_depth(), 2);

        tracker.exit_scope();
        assert_eq!(tracker.current_depth(), 1);
    }

    #[test]
    fn test_scope_tracker_register_variable() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        tracker.register_variable("x".to_string());

        assert!(tracker.is_in_scope("x"));
        assert!(!tracker.is_in_scope("y"));
    }

    #[test]
    fn test_scope_tracker_variable_depth() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        tracker.register_variable("x".to_string());

        assert_eq!(tracker.get_variable_depth("x"), Some(1));
        assert_eq!(tracker.get_variable_depth("y"), None);
    }

    #[test]
    fn test_scope_tracker_exit_removes_vars() {
        let mut tracker = ScopeTracker::new();
        tracker.enter_scope();
        tracker.register_variable("x".to_string());
        tracker.enter_scope();
        tracker.register_variable("y".to_string());

        let expired = tracker.exit_scope();
        assert_eq!(expired, vec!["y"]);
        assert!(tracker.is_in_scope("x"));
        assert!(!tracker.is_in_scope("y"));
    }

    // ViolationType tests
    #[test]
    fn test_violation_type_severity() {
        assert_eq!(ViolationType::UseAfterMove.severity(), 5);
        assert_eq!(ViolationType::DanglingReference.severity(), 5);
        assert_eq!(ViolationType::ConflictingBorrows.severity(), 4);
        assert_eq!(ViolationType::LifetimeTooShort.severity(), 3);
        assert_eq!(ViolationType::EscapingReference.severity(), 4);
    }

    #[test]
    fn test_violation_type_is_critical() {
        assert!(ViolationType::UseAfterMove.is_critical());
        assert!(ViolationType::DanglingReference.is_critical());
        assert!(!ViolationType::ConflictingBorrows.is_critical());
        assert!(!ViolationType::LifetimeTooShort.is_critical());
    }

    #[test]
    fn test_violation_type_description() {
        assert!(!ViolationType::UseAfterMove.description().is_empty());
        assert!(!ViolationType::DanglingReference.description().is_empty());
        assert!(!ViolationType::ConflictingBorrows.description().is_empty());
    }

    #[test]
    fn test_violation_type_suggestion() {
        assert!(!ViolationType::UseAfterMove.suggestion().is_empty());
        assert!(!ViolationType::EscapingReference.suggestion().is_empty());
    }

    // MoveTracker tests
    #[test]
    fn test_move_tracker_new() {
        let tracker = MoveTracker::new();
        assert!(!tracker.is_moved("x"));
    }

    #[test]
    fn test_move_tracker_mark_moved() {
        let mut tracker = MoveTracker::new();
        tracker.mark_moved("x");
        assert!(tracker.is_moved("x"));
        assert!(!tracker.is_moved("y"));
    }

    #[test]
    fn test_move_tracker_reassign() {
        let mut tracker = MoveTracker::new();
        tracker.mark_moved("x");
        tracker.reassign("x");
        assert!(!tracker.is_moved("x"));
    }

    #[test]
    fn test_move_tracker_copy_types() {
        let tracker = MoveTracker::with_copy_types(&["i32", "bool"]);
        assert!(tracker.is_copy_type("i32"));
        assert!(tracker.is_copy_type("bool"));
        assert!(!tracker.is_copy_type("String"));
    }

    #[test]
    fn test_move_tracker_copy_types_no_move() {
        let mut tracker = MoveTracker::with_copy_types(&["x"]);
        tracker.mark_moved("x");
        assert!(!tracker.is_moved("x")); // Copy types aren't moved
    }

    #[test]
    fn test_move_tracker_register_copy_type() {
        let mut tracker = MoveTracker::new();
        tracker.register_copy_type("MyType");
        assert!(tracker.is_copy_type("MyType"));
    }

    // LoopBorrowAnalysis tests
    #[test]
    fn test_loop_borrow_analysis_new() {
        let analysis = LoopBorrowAnalysis::new();
        assert!(analysis.borrows_in_loop.is_empty());
        assert!(analysis.mutations_in_loop.is_empty());
        assert!(!analysis.has_invalidations());
    }

    #[test]
    fn test_loop_borrow_analysis_add_borrow() {
        let mut analysis = LoopBorrowAnalysis::new();
        analysis.add_borrow("x");
        analysis.add_borrow("x"); // Duplicate
        assert_eq!(analysis.borrows_in_loop.len(), 1);
    }

    #[test]
    fn test_loop_borrow_analysis_add_mutation() {
        let mut analysis = LoopBorrowAnalysis::new();
        analysis.add_mutation("x");
        analysis.add_mutation("x"); // Duplicate
        assert_eq!(analysis.mutations_in_loop.len(), 1);
    }

    #[test]
    fn test_loop_borrow_analysis_check_invalidations() {
        let mut analysis = LoopBorrowAnalysis::new();
        analysis.add_borrow("x");
        analysis.add_mutation("x");
        analysis.check_invalidations();

        assert!(analysis.has_invalidations());
        assert_eq!(analysis.potential_invalidations.len(), 1);
    }

    #[test]
    fn test_loop_borrow_analysis_no_invalidations() {
        let mut analysis = LoopBorrowAnalysis::new();
        analysis.add_borrow("x");
        analysis.add_mutation("y");
        analysis.check_invalidations();

        assert!(!analysis.has_invalidations());
    }

    #[test]
    fn test_loop_borrow_analysis_default() {
        let analysis = LoopBorrowAnalysis::default();
        assert!(analysis.borrows_in_loop.is_empty());
    }

    // Helper function tests
    #[test]
    fn test_check_reference_escape() {
        assert!(!check_reference_escape(0, 0)); // Same depth
        assert!(check_reference_escape(0, 1)); // Deeper variable
        assert!(!check_reference_escape(2, 1)); // Shallower variable
    }

    #[test]
    fn test_is_mutating_method() {
        assert!(is_mutating_method("push"));
        assert!(is_mutating_method("pop"));
        assert!(is_mutating_method("insert"));
        assert!(is_mutating_method("remove"));
        assert!(is_mutating_method("clear"));
        assert!(is_mutating_method("append"));
        assert!(is_mutating_method("extend"));
        assert!(is_mutating_method("sort"));
        assert!(is_mutating_method("reverse"));
        assert!(!is_mutating_method("get"));
        assert!(!is_mutating_method("len"));
        assert!(!is_mutating_method("iter"));
    }

    #[test]
    fn test_borrows_conflict() {
        assert!(borrows_conflict(&BorrowState::MutableBorrow, &BorrowState::Unborrowed));
        assert!(borrows_conflict(&BorrowState::Unborrowed, &BorrowState::MutableBorrow));
        assert!(borrows_conflict(
            &BorrowState::MutableBorrow,
            &BorrowState::SharedBorrow(1)
        ));
        assert!(!borrows_conflict(&BorrowState::Unborrowed, &BorrowState::Unborrowed));
        assert!(!borrows_conflict(
            &BorrowState::SharedBorrow(1),
            &BorrowState::SharedBorrow(1)
        ));
    }
}
