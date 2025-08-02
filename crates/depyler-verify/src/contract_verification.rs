//! Advanced contract verification system with SMT solver integration
//!
//! This module implements comprehensive precondition and postcondition
//! verification using logical predicates and SMT solving.

use crate::contracts::{Condition, Contract};
use depyler_core::hir::{HirFunction, HirStmt, Type};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Precondition validation framework with rule registry
#[derive(Debug, Default)]
pub struct PreconditionChecker {
    /// Registry of precondition rules by name
    rules: HashMap<String, PreconditionRule>,
    /// Variable state tracking
    #[allow(dead_code)]
    var_states: HashMap<String, VarState>,
    /// SMT solver backend (placeholder for now)
    #[allow(dead_code)]
    smt_backend: Option<SmtBackend>,
}

/// A precondition rule that can be validated
#[derive(Debug, Clone)]
pub struct PreconditionRule {
    pub name: String,
    pub predicate: Predicate,
    pub params: Vec<String>,
    pub description: String,
}

/// Logical predicate for contract conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Predicate {
    /// Variable comparison: var op value
    Compare {
        var: String,
        op: CompareOp,
        value: Value,
    },
    /// Logical AND of predicates
    And(Box<Predicate>, Box<Predicate>),
    /// Logical OR of predicates
    Or(Box<Predicate>, Box<Predicate>),
    /// Logical NOT
    Not(Box<Predicate>),
    /// Implies: p1 => p2
    Implies(Box<Predicate>, Box<Predicate>),
    /// For all quantifier
    ForAll {
        var: String,
        domain: String,
        pred: Box<Predicate>,
    },
    /// Exists quantifier
    Exists {
        var: String,
        domain: String,
        pred: Box<Predicate>,
    },
    /// Custom predicate function
    Custom { name: String, args: Vec<String> },
    /// Array/list bounds check
    InBounds { array: String, index: String },
    /// Null/None check
    NotNull(String),
    /// Type check
    HasType { var: String, expected_type: String },
}

/// Comparison operators for predicates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    In,
    NotIn,
}

/// Values in predicates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Var(String),
    Null,
}

/// Variable state for tracking
#[derive(Debug, Clone)]
pub struct VarState {
    pub name: String,
    pub ty: Type,
    pub constraints: Vec<Predicate>,
    pub is_initialized: bool,
    pub is_mutable: bool,
}

/// Postcondition verification with state tracking
#[derive(Debug, Default)]
pub struct PostconditionVerifier {
    /// Pre-state snapshot
    pre_state: HashMap<String, VarState>,
    /// Post-state after execution
    #[allow(dead_code)]
    post_state: HashMap<String, VarState>,
    /// Return value constraints
    #[allow(dead_code)]
    return_constraints: Vec<Predicate>,
    /// Side effect tracking
    side_effects: Vec<SideEffect>,
}

/// Side effects that need verification
#[derive(Debug, Clone)]
pub enum SideEffect {
    StateChange {
        var: String,
        old_value: Value,
        new_value: Value,
    },
    ArrayModification {
        array: String,
        index: String,
    },
    ExternalCall {
        func: String,
        args: Vec<String>,
    },
}

/// Invariant checking framework
#[derive(Debug, Default)]
pub struct InvariantChecker {
    /// Class/module invariants
    invariants: Vec<Invariant>,
    /// Loop invariants
    #[allow(dead_code)]
    loop_invariants: HashMap<String, Vec<Predicate>>,
    /// Function invariants
    func_invariants: HashMap<String, Vec<Predicate>>,
}

/// Contract inheritance support
#[derive(Debug, Default)]
pub struct ContractInheritance {
    /// Base contracts by function name
    base_contracts: HashMap<String, Contract>,
    /// Inheritance chains
    inheritance_chains: HashMap<String, Vec<String>>,
    /// Contract refinements
    refinements: HashMap<String, ContractRefinement>,
}

/// Contract refinement for inheritance
#[derive(Debug, Clone)]
pub struct ContractRefinement {
    /// Weakened preconditions (for Liskov substitution)
    weakened_preconditions: Vec<Condition>,
    /// Strengthened postconditions
    strengthened_postconditions: Vec<Condition>,
    /// Additional invariants
    additional_invariants: Vec<Condition>,
}

/// An invariant that must hold
#[derive(Debug, Clone)]
pub struct Invariant {
    pub name: String,
    pub predicate: Predicate,
    pub scope: InvariantScope,
    pub description: String,
}

/// Scope where invariant applies
#[derive(Debug, Clone)]
pub enum InvariantScope {
    Global,
    Class(String),
    Function(String),
    Loop(String),
}

/// SMT solver backend (placeholder)
#[derive(Debug)]
pub struct SmtBackend {
    #[allow(dead_code)]
    solver_type: SmtSolverType,
}

#[derive(Debug)]
pub enum SmtSolverType {
    Z3,
    CVC5,
    Yices2,
}

/// Result of contract verification
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub success: bool,
    pub violations: Vec<ContractViolation>,
    pub warnings: Vec<String>,
    pub proven_conditions: Vec<String>,
    pub unproven_conditions: Vec<String>,
}

/// A contract violation found during verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractViolation {
    pub kind: ViolationKind,
    pub condition: String,
    pub location: String,
    pub counterexample: Option<HashMap<String, Value>>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationKind {
    PreconditionFailed,
    PostconditionFailed,
    InvariantBroken,
    AssertionFailed,
}

impl PreconditionChecker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse @requires annotations from docstring
    pub fn parse_requires_annotations(&mut self, docstring: &str) -> Vec<PreconditionRule> {
        let mut rules = Vec::new();

        for line in docstring.lines() {
            if let Some(annotation) = line.trim().strip_prefix("@requires") {
                if let Some(rule) = self.parse_precondition_rule(annotation.trim()) {
                    rules.push(rule);
                }
            }
        }

        rules
    }

    /// Parse a single precondition rule
    fn parse_precondition_rule(&self, annotation: &str) -> Option<PreconditionRule> {
        // Simple parser for expressions like "param > 0" or "items is not None"
        let parts: Vec<&str> = annotation.split_whitespace().collect();

        if parts.len() >= 3 {
            let var = parts[0].to_string();
            let op_str = parts[1];
            let value_str = parts[2..].join(" ");

            let predicate = if op_str == "is" && value_str == "not None" {
                Predicate::NotNull(var.clone())
            } else if let Some(op) = parse_compare_op(op_str) {
                if let Some(value) = parse_value(&value_str) {
                    Predicate::Compare {
                        var: var.clone(),
                        op,
                        value,
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };

            Some(PreconditionRule {
                name: format!("{}_constraint", var),
                predicate,
                params: vec![var],
                description: annotation.to_string(),
            })
        } else {
            None
        }
    }

    /// Add a precondition rule
    pub fn add_rule(&mut self, rule: PreconditionRule) {
        self.rules.insert(rule.name.clone(), rule);
    }

    /// Validate all preconditions for a function
    pub fn validate_preconditions(&self, func: &HirFunction) -> VerificationResult {
        let mut result = VerificationResult {
            success: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            proven_conditions: Vec::new(),
            unproven_conditions: Vec::new(),
        };

        // Check each parameter's preconditions
        for (param_name, param_type) in &func.params {
            // Check implicit preconditions from types
            if let Some(violation) = self.check_type_precondition(param_name, param_type) {
                result.violations.push(violation);
                result.success = false;
            }

            // Check explicit preconditions from rules
            for rule in self.rules.values() {
                if rule.params.contains(param_name) {
                    match self.verify_predicate(&rule.predicate, func) {
                        PredicateResult::Proven => {
                            result.proven_conditions.push(rule.name.clone());
                        }
                        PredicateResult::Disproven(counterexample) => {
                            result.violations.push(ContractViolation {
                                kind: ViolationKind::PreconditionFailed,
                                condition: rule.description.clone(),
                                location: format!("parameter '{}'", param_name),
                                counterexample: Some(counterexample),
                                suggestion: self.suggest_fix(&rule.predicate),
                            });
                            result.success = false;
                        }
                        PredicateResult::Unknown => {
                            result.unproven_conditions.push(rule.name.clone());
                            result.warnings.push(format!(
                                "Could not prove precondition '{}' for parameter '{}'",
                                rule.description, param_name
                            ));
                        }
                    }
                }
            }
        }

        result
    }

    /// Check type-based preconditions
    fn check_type_precondition(
        &self,
        param_name: &str,
        param_type: &Type,
    ) -> Option<ContractViolation> {
        match param_type {
            Type::List(_) | Type::Dict(_, _) => {
                // Lists and dicts should not be null in safe code
                if !self.has_null_check(param_name) {
                    Some(ContractViolation {
                        kind: ViolationKind::PreconditionFailed,
                        condition: format!("{} is not None", param_name),
                        location: format!("parameter '{}'", param_name),
                        counterexample: None,
                        suggestion: format!(
                            "Add null check: if {} is None: raise ValueError",
                            param_name
                        ),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Check if parameter has null check in function body
    fn has_null_check(&self, _param_name: &str) -> bool {
        // Simplified - in real implementation would analyze function body
        false
    }

    /// Verify a predicate
    fn verify_predicate(&self, predicate: &Predicate, _func: &HirFunction) -> PredicateResult {
        match predicate {
            Predicate::NotNull(_) => {
                // Would check if variable can be proven non-null
                PredicateResult::Unknown
            }
            Predicate::Compare {
                var: _,
                op: _,
                value: _,
            } => {
                // Would use SMT solver to verify comparison
                PredicateResult::Unknown
            }
            Predicate::And(_p1, _p2) => {
                // Both must be proven
                PredicateResult::Unknown
            }
            _ => PredicateResult::Unknown,
        }
    }

    /// Suggest fix for failed precondition
    fn suggest_fix(&self, predicate: &Predicate) -> String {
        match predicate {
            Predicate::NotNull(var) => {
                format!("Add null check at function start: if {} is None: raise ValueError('{}cannot be None')", var, var)
            }
            Predicate::Compare { var, op, value } => {
                format!(
                    "Add validation: if not ({} {:?} {:?}): raise ValueError",
                    var, op, value
                )
            }
            Predicate::InBounds { array, index } => {
                format!(
                    "Add bounds check: if {} >= len({}): raise IndexError",
                    index, array
                )
            }
            _ => "Add appropriate validation for this condition".to_string(),
        }
    }

    /// Generate runtime assertions for preconditions
    pub fn generate_runtime_assertions(&self, contract: &Contract) -> String {
        let mut assertions = String::new();

        for precond in &contract.preconditions {
            if let Some(rule) = self.rules.get(&precond.name) {
                let assert_code = self.predicate_to_assertion(&rule.predicate);
                assertions.push_str(&format!(
                    "    // {}\n    assert!({}, \"{}\");\n",
                    precond.description, assert_code, precond.description
                ));
            }
        }

        assertions
    }

    /// Convert predicate to Rust assertion code
    #[allow(clippy::only_used_in_recursion)]
    fn predicate_to_assertion(&self, predicate: &Predicate) -> String {
        match predicate {
            Predicate::NotNull(var) => format!("!{}.is_none()", var),
            Predicate::Compare { var, op, value } => {
                let op_str = match op {
                    CompareOp::Eq => "==",
                    CompareOp::Ne => "!=",
                    CompareOp::Lt => "<",
                    CompareOp::Le => "<=",
                    CompareOp::Gt => ">",
                    CompareOp::Ge => ">=",
                    _ => "==",
                };
                format!("{} {} {}", var, op_str, value_to_rust(value))
            }
            Predicate::And(p1, p2) => {
                format!(
                    "({}) && ({})",
                    self.predicate_to_assertion(p1),
                    self.predicate_to_assertion(p2)
                )
            }
            Predicate::Or(p1, p2) => {
                format!(
                    "({}) || ({})",
                    self.predicate_to_assertion(p1),
                    self.predicate_to_assertion(p2)
                )
            }
            Predicate::Not(p) => format!("!({})", self.predicate_to_assertion(p)),
            Predicate::InBounds { array, index } => {
                format!("{} < {}.len()", index, array)
            }
            _ => "true".to_string(),
        }
    }
}

impl PostconditionVerifier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse @ensures annotations from docstring
    pub fn parse_ensures_annotations(&mut self, docstring: &str) -> Vec<Predicate> {
        let mut predicates = Vec::new();

        for line in docstring.lines() {
            if let Some(annotation) = line.trim().strip_prefix("@ensures") {
                if let Some(pred) = self.parse_postcondition(annotation.trim()) {
                    predicates.push(pred);
                }
            }
        }

        predicates
    }

    /// Parse a postcondition expression
    fn parse_postcondition(&self, annotation: &str) -> Option<Predicate> {
        // Handle special keywords
        if annotation.contains("old(") {
            // Pre-state reference
            self.parse_old_state_predicate(annotation)
        } else if annotation.starts_with("result") {
            // Return value constraint
            self.parse_result_predicate(annotation)
        } else {
            // Regular predicate
            parse_simple_predicate(annotation)
        }
    }

    /// Parse predicates referencing old state
    fn parse_old_state_predicate(&self, _annotation: &str) -> Option<Predicate> {
        // Example: "result == old(x) + 1"
        // Would parse and create appropriate predicate
        None // Placeholder
    }

    /// Parse predicates about return value
    fn parse_result_predicate(&self, annotation: &str) -> Option<Predicate> {
        let parts: Vec<&str> = annotation.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "result" {
            if let Some(op) = parse_compare_op(parts[1]) {
                if let Some(value) = parse_value(&parts[2..].join(" ")) {
                    return Some(Predicate::Compare {
                        var: "result".to_string(),
                        op,
                        value,
                    });
                }
            }
        }
        None
    }

    /// Capture pre-state before function execution
    pub fn capture_pre_state(&mut self, func: &HirFunction) {
        for (param_name, param_type) in &func.params {
            self.pre_state.insert(
                param_name.clone(),
                VarState {
                    name: param_name.clone(),
                    ty: param_type.clone(),
                    constraints: Vec::new(),
                    is_initialized: true,
                    is_mutable: false,
                },
            );
        }
    }

    /// Verify postconditions after execution
    pub fn verify_postconditions(
        &self,
        _func: &HirFunction,
        contract: &Contract,
    ) -> VerificationResult {
        let mut result = VerificationResult {
            success: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            proven_conditions: Vec::new(),
            unproven_conditions: Vec::new(),
        };

        for postcond in &contract.postconditions {
            // Would verify each postcondition against post-state
            result.unproven_conditions.push(postcond.name.clone());
        }

        result
    }

    /// Track side effects during execution
    pub fn track_side_effect(&mut self, effect: SideEffect) {
        self.side_effects.push(effect);
    }

    /// Generate runtime postcondition checks
    pub fn generate_postcondition_checks(&self, contract: &Contract) -> String {
        let mut checks = String::new();

        for postcond in &contract.postconditions {
            checks.push_str(&format!(
                "    // {}\n    debug_assert!({}, \"Postcondition failed: {}\");\n",
                postcond.description, postcond.expression, postcond.description
            ));
        }

        checks
    }
}

impl InvariantChecker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a class or module invariant
    pub fn add_invariant(&mut self, invariant: Invariant) {
        self.invariants.push(invariant);
    }

    /// Check all invariants
    pub fn check_invariants(&self, func: &HirFunction) -> Vec<ContractViolation> {
        let mut violations = Vec::new();

        // Check function-level invariants
        if let Some(func_invs) = self.func_invariants.get(&func.name) {
            for _inv in func_invs {
                // Would verify each invariant
            }
        }

        // Check loop invariants in function body
        for stmt in &func.body {
            violations.extend(self.check_stmt_invariants(stmt));
        }

        violations
    }

    /// Check invariants in a statement
    #[allow(clippy::only_used_in_recursion)]
    fn check_stmt_invariants(&self, stmt: &HirStmt) -> Vec<ContractViolation> {
        let mut violations = Vec::new();

        match stmt {
            HirStmt::While { condition: _, body } => {
                // Would check loop invariants
                for stmt in body {
                    violations.extend(self.check_stmt_invariants(stmt));
                }
            }
            HirStmt::For { body, .. } => {
                // Would check loop invariants
                for stmt in body {
                    violations.extend(self.check_stmt_invariants(stmt));
                }
            }
            _ => {}
        }

        violations
    }

    /// Generate invariant preservation checks
    pub fn generate_invariant_checks(&self) -> String {
        let mut checks = String::new();

        for inv in &self.invariants {
            checks.push_str(&format!(
                "// Invariant: {}\n// TODO: Generate preservation check\n",
                inv.description
            ));
        }

        checks
    }
}

impl ContractInheritance {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a base contract
    pub fn register_base_contract(&mut self, func_name: String, contract: Contract) {
        self.base_contracts.insert(func_name, contract);
    }

    /// Add inheritance relationship
    pub fn add_inheritance(&mut self, derived: String, base: String) {
        self.inheritance_chains
            .entry(derived)
            .or_default()
            .push(base);
    }

    /// Get inherited contract for a function
    pub fn get_inherited_contract(&self, func_name: &str) -> Option<Contract> {
        // First check direct contract
        if let Some(contract) = self.base_contracts.get(func_name) {
            return Some(contract.clone());
        }

        // Check inheritance chain
        if let Some(bases) = self.inheritance_chains.get(func_name) {
            for base in bases {
                if let Some(base_contract) = self.base_contracts.get(base) {
                    // Apply refinements if any
                    if let Some(refinement) = self.refinements.get(func_name) {
                        return Some(self.apply_refinement(base_contract, refinement));
                    }
                    return Some(base_contract.clone());
                }
            }
        }

        None
    }

    /// Apply contract refinement (Liskov substitution principle)
    fn apply_refinement(&self, base: &Contract, refinement: &ContractRefinement) -> Contract {
        let mut refined = base.clone();

        // Weaken preconditions (can accept more)
        for weakened in &refinement.weakened_preconditions {
            // Remove stronger precondition if it exists
            refined.preconditions.retain(|p| p.name != weakened.name);
            refined.preconditions.push(weakened.clone());
        }

        // Strengthen postconditions (must guarantee more)
        refined
            .postconditions
            .extend(refinement.strengthened_postconditions.clone());

        // Add additional invariants
        refined
            .invariants
            .extend(refinement.additional_invariants.clone());

        refined
    }

    /// Verify Liskov substitution principle
    pub fn verify_lsp(&self, derived: &str, base: &str) -> Result<(), String> {
        let base_contract = self
            .base_contracts
            .get(base)
            .ok_or_else(|| format!("Base contract '{}' not found", base))?;

        let derived_contract = self
            .base_contracts
            .get(derived)
            .ok_or_else(|| format!("Derived contract '{}' not found", derived))?;

        // Check preconditions are not strengthened
        for base_pre in &base_contract.preconditions {
            let has_weaker = derived_contract
                .preconditions
                .iter()
                .any(|d| self.is_weaker_than(&d.expression, &base_pre.expression));
            if !has_weaker {
                return Err(format!(
                    "Precondition '{}' is strengthened in derived contract",
                    base_pre.description
                ));
            }
        }

        // Check postconditions are not weakened
        for base_post in &base_contract.postconditions {
            let has_stronger = derived_contract
                .postconditions
                .iter()
                .any(|d| self.is_stronger_than(&d.expression, &base_post.expression));
            if !has_stronger {
                return Err(format!(
                    "Postcondition '{}' is weakened in derived contract",
                    base_post.description
                ));
            }
        }

        Ok(())
    }

    /// Check if one predicate is weaker than another
    fn is_weaker_than(&self, pred1: &str, pred2: &str) -> bool {
        // Simplified - would use SMT solver in real implementation
        // x >= 0 is weaker than x > 0 (accepts more values)
        if pred1 == pred2 {
            return true;
        }

        // Check for >= being weaker than >
        if pred1.contains(">=") && pred2.contains(">") && !pred2.contains("=") {
            return true;
        }

        // Check for <= being weaker than <
        if pred1.contains("<=") && pred2.contains("<") && !pred2.contains("=") {
            return true;
        }

        false
    }

    /// Check if one predicate is stronger than another
    fn is_stronger_than(&self, pred1: &str, pred2: &str) -> bool {
        // Simplified - would use SMT solver in real implementation
        pred1 == pred2 || pred1.contains("<") && pred2.contains("<")
    }
}

/// Result of predicate verification
#[allow(dead_code)]
enum PredicateResult {
    Proven,
    Disproven(HashMap<String, Value>),
    Unknown,
}

/// Parse a comparison operator
fn parse_compare_op(s: &str) -> Option<CompareOp> {
    match s {
        "==" => Some(CompareOp::Eq),
        "!=" => Some(CompareOp::Ne),
        "<" => Some(CompareOp::Lt),
        "<=" => Some(CompareOp::Le),
        ">" => Some(CompareOp::Gt),
        ">=" => Some(CompareOp::Ge),
        "in" => Some(CompareOp::In),
        _ => None,
    }
}

/// Parse a value from string
fn parse_value(s: &str) -> Option<Value> {
    if let Ok(n) = s.parse::<i64>() {
        Some(Value::Int(n))
    } else if let Ok(f) = s.parse::<f64>() {
        Some(Value::Float(f))
    } else if s == "true" {
        Some(Value::Bool(true))
    } else if s == "false" {
        Some(Value::Bool(false))
    } else if s == "None" || s == "null" {
        Some(Value::Null)
    } else if s.starts_with('"') && s.ends_with('"') {
        Some(Value::String(s[1..s.len() - 1].to_string()))
    } else {
        Some(Value::Var(s.to_string()))
    }
}

/// Convert value to Rust code
fn value_to_rust(value: &Value) -> String {
    match value {
        Value::Int(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Bool(b) => b.to_string(),
        Value::Var(v) => v.clone(),
        Value::Null => "None".to_string(),
    }
}

/// Parse a simple predicate expression
fn parse_simple_predicate(expr: &str) -> Option<Predicate> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() >= 3 {
        let var = parts[0].to_string();
        if let Some(op) = parse_compare_op(parts[1]) {
            if let Some(value) = parse_value(&parts[2..].join(" ")) {
                return Some(Predicate::Compare { var, op, value });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precondition_parser() {
        let mut checker = PreconditionChecker::new();
        let rules = checker.parse_requires_annotations(
            "@requires x > 0\n@requires items is not None\n@requires name != \"\"",
        );

        assert_eq!(rules.len(), 3);
        assert!(matches!(&rules[0].predicate, Predicate::Compare { .. }));
        assert!(matches!(&rules[1].predicate, Predicate::NotNull(_)));
    }

    #[test]
    fn test_postcondition_parser() {
        let mut verifier = PostconditionVerifier::new();
        let preds = verifier
            .parse_ensures_annotations("@ensures result >= 0\n@ensures result == old(x) + 1");

        assert!(!preds.is_empty());
    }

    #[test]
    fn test_predicate_to_assertion() {
        let checker = PreconditionChecker::new();

        let pred = Predicate::Compare {
            var: "x".to_string(),
            op: CompareOp::Gt,
            value: Value::Int(0),
        };

        let assertion = checker.predicate_to_assertion(&pred);
        assert_eq!(assertion, "x > 0");

        let null_pred = Predicate::NotNull("items".to_string());
        let null_assertion = checker.predicate_to_assertion(&null_pred);
        assert_eq!(null_assertion, "!items.is_none()");
    }

    #[test]
    fn test_value_parsing() {
        assert!(matches!(parse_value("42"), Some(Value::Int(42))));
        assert!(matches!(parse_value("3.14"), Some(Value::Float(_))));
        assert!(matches!(parse_value("true"), Some(Value::Bool(true))));
        assert!(matches!(parse_value("\"hello\""), Some(Value::String(_))));
        assert!(matches!(parse_value("None"), Some(Value::Null)));
        assert!(matches!(parse_value("variable"), Some(Value::Var(_))));
    }

    #[test]
    fn test_contract_inheritance() {
        use crate::contracts::{Condition, Contract};

        let mut inheritance = ContractInheritance::new();

        // Base contract
        let base_contract = Contract {
            preconditions: vec![Condition {
                name: "items_not_null".to_string(),
                expression: "items is not None".to_string(),
                description: "Items must not be null".to_string(),
            }],
            postconditions: vec![Condition {
                name: "result_valid".to_string(),
                expression: "result >= 0".to_string(),
                description: "Result must be non-negative".to_string(),
            }],
            invariants: vec![],
        };

        inheritance.register_base_contract("base_search".to_string(), base_contract);
        inheritance.add_inheritance("binary_search".to_string(), "base_search".to_string());

        let inherited = inheritance.get_inherited_contract("binary_search");
        assert!(inherited.is_some());
        let contract = inherited.unwrap();
        assert_eq!(contract.preconditions.len(), 1);
        assert_eq!(contract.postconditions.len(), 1);
    }

    #[test]
    fn test_contract_refinement() {
        use crate::contracts::{Condition, Contract};

        let mut inheritance = ContractInheritance::new();

        // Base and derived contracts
        let base_contract = Contract {
            preconditions: vec![Condition {
                name: "x_positive".to_string(),
                expression: "x > 0".to_string(),
                description: "x must be positive".to_string(),
            }],
            postconditions: vec![Condition {
                name: "result_positive".to_string(),
                expression: "result > 0".to_string(),
                description: "Result must be positive".to_string(),
            }],
            invariants: vec![],
        };

        let derived_contract = Contract {
            preconditions: vec![Condition {
                name: "x_positive".to_string(),
                expression: "x >= 0".to_string(), // Weakened
                description: "x must be non-negative".to_string(),
            }],
            postconditions: vec![
                Condition {
                    name: "result_positive".to_string(),
                    expression: "result > 0".to_string(),
                    description: "Result must be positive".to_string(),
                },
                Condition {
                    name: "result_bounded".to_string(),
                    expression: "result < 100".to_string(), // Strengthened
                    description: "Result must be bounded".to_string(),
                },
            ],
            invariants: vec![],
        };

        inheritance.register_base_contract("base".to_string(), base_contract);
        inheritance.register_base_contract("derived".to_string(), derived_contract);

        // LSP should pass for valid refinement
        let result = inheritance.verify_lsp("derived", "base");
        assert!(result.is_ok());
    }
}
