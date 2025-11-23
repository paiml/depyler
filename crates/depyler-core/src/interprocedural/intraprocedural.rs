//! Intraprocedural analysis for per-function mutation and borrowing patterns
//!
//! This module analyzes individual functions in isolation to determine:
//! - Direct mutations to parameters
//! - Read-only accesses to parameters
//! - Call sites where parameters are passed to other functions
//! - Field-level access patterns
//! - Aliasing relationships

use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type as PythonType};
use std::collections::HashMap;

/// Location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn unknown() -> Self {
        Self { line: 0, column: 0 }
    }
}

/// Kind of mutation performed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationKind {
    /// Direct field write: param.field = value
    DirectFieldWrite,
    /// Method call that mutates: param.append(x)
    MethodCall,
    /// Index assignment: param[idx] = value
    IndexAssignment,
    /// Attribute assignment: setattr(param, "x", value)
    AttributeAssignment,
}

/// A site where a parameter is mutated
#[derive(Debug, Clone)]
pub struct MutationSite {
    pub location: Location,
    pub kind: MutationKind,
    pub field_path: Vec<String>,
}

/// Context in which a read occurs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadContext {
    /// Used in an expression
    InExpression,
    /// Returned from function
    InReturn,
    /// Used in a condition
    InCondition,
    /// Passed as argument
    InCall,
}

/// A site where a parameter is read
#[derive(Debug, Clone)]
pub struct ReadSite {
    pub location: Location,
    pub field_path: Vec<String>,
    pub context: ReadContext,
}

/// How a parameter is passed to another function
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassKind {
    /// Entire parameter passed: helper(state)
    Whole,
    /// Field of parameter passed: helper(state.items)
    Field(Vec<String>),
    /// Expression involving parameter: helper(state.x + 1)
    Expression,
}

/// Information about a call site where a parameter is used
#[derive(Debug, Clone)]
pub struct CallSiteUsage {
    pub location: Location,
    pub callee: String,
    pub arg_position: usize,
    pub callee_param_position: usize,
    pub pass_kind: PassKind,
}

/// Tracks field-level access patterns
#[derive(Debug, Clone, Default)]
pub struct FieldAccessPattern {
    /// Fields that are read
    pub read_fields: HashMap<String, Vec<Location>>,
    /// Fields that are written
    pub written_fields: HashMap<String, Vec<Location>>,
    /// Nested field accesses (e.g., state.items.values)
    pub nested_accesses: Vec<Vec<String>>,
}

/// Information about an alias to a parameter
#[derive(Debug, Clone)]
pub struct AliasInfo {
    pub alias_name: String,
    pub location: Location,
    pub is_mutated: bool,
}

/// Comprehensive usage analysis for a single parameter
#[derive(Debug, Clone)]
pub struct ParameterUsageAnalysis {
    /// Parameter name
    pub name: String,
    /// Direct mutations (e.g., param.field = value)
    pub direct_mutations: Vec<MutationSite>,
    /// Read-only accesses (e.g., x = param.field)
    pub read_sites: Vec<ReadSite>,
    /// Calls where this parameter is passed
    pub call_sites: Vec<CallSiteUsage>,
    /// Field-level access patterns
    pub field_access: FieldAccessPattern,
    /// Aliasing information
    pub aliases: Vec<AliasInfo>,
}

impl ParameterUsageAnalysis {
    pub fn new(name: String) -> Self {
        Self {
            name,
            direct_mutations: Vec::new(),
            read_sites: Vec::new(),
            call_sites: Vec::new(),
            field_access: FieldAccessPattern::default(),
            aliases: Vec::new(),
        }
    }

    /// Check if this parameter has any direct mutations
    pub fn has_direct_mutations(&self) -> bool {
        !self.direct_mutations.is_empty()
    }

    /// Check if this parameter has any reads
    pub fn has_reads(&self) -> bool {
        !self.read_sites.is_empty()
    }

    /// Check if this parameter is passed to other functions
    pub fn has_call_sites(&self) -> bool {
        !self.call_sites.is_empty()
    }

    /// Compute minimal mutability based on local (intraprocedural) evidence only
    pub fn minimal_mutability(&self) -> LocalMutability {
        if self.has_direct_mutations() {
            LocalMutability::NeedsMut
        } else if self.has_reads() {
            LocalMutability::CanBeShared
        } else if self.has_call_sites() {
            LocalMutability::PassedToCallees
        } else {
            LocalMutability::Unused
        }
    }
}

/// Local mutability inference (without interprocedural context)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalMutability {
    /// Parameter has direct mutations
    NeedsMut,
    /// Parameter is only read
    CanBeShared,
    /// Parameter is passed to other functions (needs interprocedural analysis)
    PassedToCallees,
    /// Parameter is not used
    Unused,
}

/// Summary of intraprocedural analysis for a function
#[derive(Debug, Clone)]
pub struct IntraproceduralSummary {
    pub function_name: String,
    pub parameters: Vec<ParameterUsageAnalysis>,
    pub all_call_sites: Vec<CallSite>,
}

/// A call site in the function
#[derive(Debug, Clone)]
pub struct CallSite {
    pub location: Location,
    pub callee: String,
    pub arguments: Vec<ArgumentInfo>,
}

/// Information about an argument at a call site
#[derive(Debug, Clone)]
pub struct ArgumentInfo {
    pub position: usize,
    pub source: ArgumentSource,
}

/// Source of an argument
#[derive(Debug, Clone)]
pub enum ArgumentSource {
    /// Direct parameter: helper(state)
    Parameter(String),
    /// Field access: helper(state.field)
    ParameterField {
        param: String,
        field_path: Vec<String>,
    },
    /// Local variable
    Local(String),
    /// Literal or expression
    Expression,
}

/// Analyzer for a single function
pub struct IntraproceduralAnalyzer<'a> {
    function: &'a HirFunction,
    param_usage: HashMap<String, ParameterUsageAnalysis>,
    local_aliases: HashMap<String, AliasInfo>,
    call_sites: Vec<CallSite>,
}

impl<'a> IntraproceduralAnalyzer<'a> {
    pub fn new(function: &'a HirFunction) -> Self {
        let mut param_usage = HashMap::new();
        for param in &function.params {
            param_usage.insert(
                param.name.clone(),
                ParameterUsageAnalysis::new(param.name.clone()),
            );
        }

        Self {
            function,
            param_usage,
            local_aliases: HashMap::new(),
            call_sites: Vec::new(),
        }
    }

    /// Perform complete intraprocedural analysis
    pub fn analyze(mut self) -> IntraproceduralSummary {
        // Scan all statements in the function body
        for stmt in &self.function.body {
            self.analyze_statement(stmt);
        }

        // Build summary
        IntraproceduralSummary {
            function_name: self.function.name.clone(),
            parameters: self.param_usage.into_values().collect(),
            all_call_sites: self.call_sites,
        }
    }

    /// Analyze a single statement
    fn analyze_statement(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                self.analyze_assignment(target, value);
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr(expr);
            }
            HirStmt::Return(value) => {
                if let Some(expr) = value {
                    self.analyze_expr_with_context(expr, ReadContext::InReturn);
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.analyze_expr_with_context(condition, ReadContext::InCondition);
                for s in then_body {
                    self.analyze_statement(s);
                }
                if let Some(else_body) = else_body {
                    for s in else_body {
                        self.analyze_statement(s);
                    }
                }
            }
            HirStmt::While {
                condition, body, ..
            } => {
                self.analyze_expr_with_context(condition, ReadContext::InCondition);
                for s in body {
                    self.analyze_statement(s);
                }
            }
            HirStmt::For {
                target, iter, body, ..
            } => {
                self.analyze_expr(iter);
                for s in body {
                    self.analyze_statement(s);
                }
            }
            HirStmt::FunctionDef { .. } => {
                // Nested function - skip for now
            }
            _ => {
                // Other statement types - no parameter usage
            }
        }
    }

    /// Analyze an assignment
    fn analyze_assignment(&mut self, target: &AssignTarget, value: &HirExpr) {
        match target {
            AssignTarget::Symbol(name) => {
                // Check if this is an alias to a parameter
                if let HirExpr::Var(param_name) = value {
                    if self.param_usage.contains_key(param_name) {
                        self.local_aliases.insert(
                            name.clone(),
                            AliasInfo {
                                alias_name: name.clone(),
                                location: Location::unknown(),
                                is_mutated: false,
                            },
                        );
                    }
                }
                self.analyze_expr(value);
            }
            AssignTarget::Attribute {
                value: obj, attr, ..
            } => {
                // Assignment to attribute: obj.attr = value
                if let HirExpr::Var(param_name) = &**obj {
                    if self.param_usage.contains_key(param_name) {
                        self.record_mutation(
                            param_name,
                            vec![attr.clone()],
                            Location::unknown(),
                            MutationKind::DirectFieldWrite,
                        );
                    }
                }
                self.analyze_expr(value);
            }
            AssignTarget::Index { base: obj, .. } => {
                // Assignment to subscript: obj[idx] = value
                if let HirExpr::Var(param_name) = &**obj {
                    if self.param_usage.contains_key(param_name) {
                        self.record_mutation(
                            param_name,
                            Vec::new(),
                            Location::unknown(),
                            MutationKind::IndexAssignment,
                        );
                    }
                }
                self.analyze_expr(value);
            }
            _ => {
                self.analyze_expr(value);
            }
        }
    }

    /// Analyze an expression
    fn analyze_expr(&mut self, expr: &HirExpr) {
        self.analyze_expr_with_context(expr, ReadContext::InExpression);
    }

    /// Analyze an expression with context
    fn analyze_expr_with_context(&mut self, expr: &HirExpr, context: ReadContext) {
        match expr {
            HirExpr::Var(name) => {
                if self.param_usage.contains_key(name) {
                    self.record_read(name, Vec::new(), Location::unknown(), context);
                }
            }
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(param_name) = &**value {
                    if self.param_usage.contains_key(param_name) {
                        self.record_read(
                            param_name,
                            vec![attr.clone()],
                            Location::unknown(),
                            context,
                        );
                    }
                } else {
                    self.analyze_expr_with_context(value, context.clone());
                }
            }
            HirExpr::Call { func, args, .. } => {
                self.analyze_call_with_func_name(func, args);
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expr_with_context(left, context.clone());
                self.analyze_expr_with_context(right, context);
            }
            HirExpr::Unary { operand, .. } => {
                self.analyze_expr_with_context(operand, context);
            }
            HirExpr::IfExpr { test, body, orelse } => {
                self.analyze_expr_with_context(test, ReadContext::InCondition);
                self.analyze_expr_with_context(body, context.clone());
                self.analyze_expr_with_context(orelse, context);
            }
            HirExpr::List(elts) | HirExpr::Tuple(elts) | HirExpr::Set(elts) => {
                for elt in elts {
                    self.analyze_expr_with_context(elt, context.clone());
                }
            }
            HirExpr::Dict(pairs) => {
                for (key, val) in pairs {
                    self.analyze_expr_with_context(key, context.clone());
                    self.analyze_expr_with_context(val, context.clone());
                }
            }
            HirExpr::Index { base, index } => {
                self.analyze_expr_with_context(base, context.clone());
                self.analyze_expr_with_context(index, context);
            }
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                // Check if method is mutating
                if let HirExpr::Var(param_name) = &**object {
                    if self.param_usage.contains_key(param_name) {
                        if is_mutating_method(method) {
                            self.record_mutation(
                                param_name,
                                Vec::new(),
                                Location::unknown(),
                                MutationKind::MethodCall,
                            );
                        } else {
                            self.record_read(param_name, Vec::new(), Location::unknown(), context);
                        }
                    }
                } else {
                    self.analyze_expr_with_context(object, context.clone());
                }
                for arg in args {
                    self.analyze_expr(arg);
                }
            }
            _ => {
                // Other expression types - no parameter usage
            }
        }
    }

    /// Analyze a function call
    fn analyze_call_with_func_name(&mut self, func_name: &str, args: &[HirExpr]) {
        // Build call site
        let mut arguments = Vec::new();

        // Collect call site usages first
        let mut call_site_usages: Vec<(String, CallSiteUsage)> = Vec::new();

        for (pos, arg) in args.iter().enumerate() {
            let source = self.classify_argument(arg);
            arguments.push(ArgumentInfo {
                position: pos,
                source,
            });

            // Also record parameter usage at call sites
            if let Some(param_name) = self.extract_param_from_arg(arg) {
                if self.param_usage.contains_key(&param_name) {
                    let pass_kind = self.classify_pass_kind(arg, &param_name);
                    call_site_usages.push((
                        param_name,
                        CallSiteUsage {
                            location: Location::unknown(),
                            callee: func_name.to_string(),
                            arg_position: pos,
                            callee_param_position: pos, // Assume positional for now
                            pass_kind,
                        },
                    ));
                }
            }
        }

        // Now update param_usage with the collected usages
        for (param_name, usage) in call_site_usages {
            if let Some(param) = self.param_usage.get_mut(&param_name) {
                param.call_sites.push(usage);
            }
        }

        self.call_sites.push(CallSite {
            location: Location::unknown(),
            callee: func_name.to_string(),
            arguments,
        });
    }

    /// Classify the source of an argument
    fn classify_argument(&self, arg: &HirExpr) -> ArgumentSource {
        match arg {
            HirExpr::Var(name) => {
                if self.param_usage.contains_key(name) {
                    ArgumentSource::Parameter(name.clone())
                } else {
                    ArgumentSource::Local(name.clone())
                }
            }
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(param_name) = &**value {
                    if self.param_usage.contains_key(param_name) {
                        return ArgumentSource::ParameterField {
                            param: param_name.clone(),
                            field_path: vec![attr.clone()],
                        };
                    }
                }
                ArgumentSource::Expression
            }
            _ => ArgumentSource::Expression,
        }
    }

    /// Extract parameter name from argument if it's a parameter
    fn extract_param_from_arg(&self, arg: &HirExpr) -> Option<String> {
        match arg {
            HirExpr::Var(name) if self.param_usage.contains_key(name) => Some(name.clone()),
            HirExpr::Attribute { value, .. } => {
                if let HirExpr::Var(param_name) = &**value {
                    if self.param_usage.contains_key(param_name) {
                        return Some(param_name.clone());
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Classify how a parameter is passed
    fn classify_pass_kind(&self, arg: &HirExpr, param_name: &str) -> PassKind {
        match arg {
            HirExpr::Var(name) if name == param_name => PassKind::Whole,
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(name) = &**value {
                    if name == param_name {
                        return PassKind::Field(vec![attr.clone()]);
                    }
                }
                PassKind::Expression
            }
            _ => PassKind::Expression,
        }
    }

    /// Record a mutation to a parameter
    fn record_mutation(
        &mut self,
        param: &str,
        field_path: Vec<String>,
        location: Location,
        kind: MutationKind,
    ) {
        if let Some(usage) = self.param_usage.get_mut(param) {
            usage.direct_mutations.push(MutationSite {
                location,
                kind,
                field_path: field_path.clone(),
            });

            // Also update field access pattern
            if !field_path.is_empty() {
                usage
                    .field_access
                    .written_fields
                    .entry(field_path[0].clone())
                    .or_insert_with(Vec::new)
                    .push(location.clone());
            }
        }
    }

    /// Record a read of a parameter
    fn record_read(
        &mut self,
        param: &str,
        field_path: Vec<String>,
        location: Location,
        context: ReadContext,
    ) {
        if let Some(usage) = self.param_usage.get_mut(param) {
            usage.read_sites.push(ReadSite {
                location,
                field_path: field_path.clone(),
                context,
            });

            // Also update field access pattern
            if !field_path.is_empty() {
                usage
                    .field_access
                    .read_fields
                    .entry(field_path[0].clone())
                    .or_insert_with(Vec::new)
                    .push(location.clone());
            }
        }
    }
}

/// Check if a method name represents a mutating operation
fn is_mutating_method(method: &str) -> bool {
    matches!(
        method,
        // List methods
        "append" | "extend" | "insert" | "remove" | "pop" | "clear" | "reverse" | "sort" |
        // Dict methods
        "update" | "setdefault" | "popitem" |
        // Set methods
        "add" | "discard" | "difference_update" | "intersection_update" | "symmetric_difference_update" |
        "union_update" |
        // Other mutating methods
        "push" | "pop_front" | "push_front" | "pop_back" | "push_back"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_usage_new() {
        let usage = ParameterUsageAnalysis::new("state".to_string());
        assert_eq!(usage.name, "state");
        assert!(!usage.has_direct_mutations());
        assert!(!usage.has_reads());
        assert!(!usage.has_call_sites());
    }

    #[test]
    fn test_minimal_mutability_direct_mutation() {
        let mut usage = ParameterUsageAnalysis::new("state".to_string());
        usage.direct_mutations.push(MutationSite {
            location: Location::unknown(),
            kind: MutationKind::DirectFieldWrite,
            field_path: vec!["value".to_string()],
        });
        assert_eq!(usage.minimal_mutability(), LocalMutability::NeedsMut);
    }

    #[test]
    fn test_minimal_mutability_read_only() {
        let mut usage = ParameterUsageAnalysis::new("state".to_string());
        usage.read_sites.push(ReadSite {
            location: Location::unknown(),
            field_path: vec!["value".to_string()],
            context: ReadContext::InExpression,
        });
        assert_eq!(usage.minimal_mutability(), LocalMutability::CanBeShared);
    }
}
