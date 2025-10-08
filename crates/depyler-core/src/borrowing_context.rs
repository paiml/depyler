//! Enhanced borrowing context for proper ownership pattern inference
//!
//! This module provides comprehensive analysis of parameter usage patterns
//! to determine optimal borrowing strategies for function parameters.

use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type as PythonType};
use crate::type_mapper::{RustType, TypeMapper};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

/// Comprehensive borrowing context for analyzing parameter usage
#[derive(Debug)]
pub struct BorrowingContext {
    /// Usage patterns for each parameter
    param_usage: HashMap<String, ParameterUsagePattern>,
    /// Variables that are moved or consumed
    moved_vars: HashSet<String>,
    /// Variables that are borrowed mutably
    mut_borrowed_vars: HashSet<String>,
    /// Variables that are borrowed immutably
    immut_borrowed_vars: HashSet<String>,
    /// Control flow context stack
    context_stack: Vec<AnalysisContext>,
    /// Function return type for escape analysis
    return_type: Option<PythonType>,
}

/// Detailed parameter usage pattern
#[derive(Debug, Clone, Default)]
pub struct ParameterUsagePattern {
    /// Parameter is read without modification
    pub is_read: bool,
    /// Parameter is modified (assigned to)
    pub is_mutated: bool,
    /// Parameter is moved/consumed (passed to function that takes ownership)
    pub is_moved: bool,
    /// Parameter escapes through return
    pub escapes_through_return: bool,
    /// Parameter is stored in a struct/container
    pub is_stored: bool,
    /// Parameter is used in a closure
    pub used_in_closure: bool,
    /// Parameter is used in loops (affects borrowing strategy)
    pub used_in_loop: bool,
    /// Nested field access patterns
    pub field_accesses: HashSet<String>,
    /// Method calls on the parameter
    pub method_calls: HashSet<String>,
    /// Specific expressions where parameter is used
    pub usage_sites: Vec<UsageSite>,
}

/// Site where a parameter is used
#[derive(Debug, Clone)]
pub struct UsageSite {
    /// Type of usage
    pub usage_type: UsageType,
    /// Whether in a loop context
    pub in_loop: bool,
    /// Whether in a conditional context
    pub in_conditional: bool,
    /// Depth of borrowing (for nested borrows)
    pub borrow_depth: usize,
}

/// Type of parameter usage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsageType {
    /// Simple read access
    Read,
    /// Mutable access
    Write,
    /// Method call that might mutate
    MethodCall(String),
    /// Passed to another function
    FunctionArg { takes_ownership: bool },
    /// Returned from function
    Return,
    /// Stored in a data structure
    Store,
    /// Used in a closure
    Closure { captures_by_value: bool },
    /// Field access
    FieldAccess(String),
    /// Index access
    IndexAccess,
}

/// Analysis context for control flow
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum AnalysisContext {
    Loop,
    Conditional,
    Closure { captures: HashSet<String> },
    Function,
}

/// Result of borrowing analysis
#[derive(Debug, Clone)]
pub struct BorrowingAnalysisResult {
    /// Recommended borrowing strategy for each parameter
    pub param_strategies: IndexMap<String, BorrowingStrategy>,
    /// Additional insights
    pub insights: Vec<BorrowingInsight>,
}

/// Recommended borrowing strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowingStrategy {
    /// Take ownership (move)
    TakeOwnership,
    /// Borrow immutably
    BorrowImmutable { lifetime: Option<String> },
    /// Borrow mutably
    BorrowMutable { lifetime: Option<String> },
    /// Use Cow for flexibility
    UseCow { lifetime: String },
    /// Use Arc/Rc for shared ownership
    UseSharedOwnership { is_thread_safe: bool },
}

/// Insights from borrowing analysis
#[derive(Debug, Clone)]
pub enum BorrowingInsight {
    /// Parameter could be borrowed but is currently moved
    UnnecessaryMove(String),
    /// Parameter could use a more specific lifetime
    LifetimeOptimization { param: String, suggestion: String },
    /// Parameter access pattern suggests Copy trait
    SuggestCopyDerive(String),
    /// Multiple mutable borrows detected
    PotentialBorrowConflict {
        param: String,
        locations: Vec<String>,
    },
}

impl BorrowingContext {
    pub fn new(return_type: Option<PythonType>) -> Self {
        Self {
            param_usage: HashMap::new(),
            moved_vars: HashSet::new(),
            mut_borrowed_vars: HashSet::new(),
            immut_borrowed_vars: HashSet::new(),
            context_stack: vec![AnalysisContext::Function],
            return_type,
        }
    }

    /// Analyze a function to determine optimal borrowing strategies
    pub fn analyze_function(
        &mut self,
        func: &HirFunction,
        type_mapper: &TypeMapper,
    ) -> BorrowingAnalysisResult {
        // Initialize parameter tracking
        for param in &func.params {
            self.param_usage
                .insert(param.name.clone(), ParameterUsagePattern::default());
        }

        // Analyze function body
        for stmt in &func.body {
            self.analyze_statement(stmt);
        }

        // Determine borrowing strategies based on usage patterns
        self.determine_strategies(func, type_mapper)
    }

    /// Analyze a statement for parameter usage
    fn analyze_statement(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // Check if assigning to a parameter (mutation)
                let in_loop = self.is_in_loop();
                let in_conditional = self.is_in_conditional();
                if let AssignTarget::Symbol(symbol) = target {
                    if let Some(usage) = self.param_usage.get_mut(symbol) {
                        usage.is_mutated = true;
                        usage.usage_sites.push(UsageSite {
                            usage_type: UsageType::Write,
                            in_loop,
                            in_conditional,
                            borrow_depth: 0,
                        });
                    }
                }
                self.analyze_expression(value, 0);
            }
            HirStmt::Return(expr) => {
                if let Some(e) = expr {
                    self.analyze_expression_for_return(e);
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expression(condition, 0);
                self.context_stack.push(AnalysisContext::Conditional);
                for stmt in then_body {
                    self.analyze_statement(stmt);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.analyze_statement(stmt);
                    }
                }
                self.context_stack.pop();
            }
            HirStmt::While { condition, body } => {
                self.context_stack.push(AnalysisContext::Loop);
                self.analyze_expression(condition, 0);
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.context_stack.pop();
            }
            HirStmt::For {
                target: _,
                iter,
                body,
            } => {
                self.context_stack.push(AnalysisContext::Loop);
                self.analyze_expression(iter, 0);
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.context_stack.pop();
            }
            HirStmt::Expr(expr) => {
                self.analyze_expression(expr, 0);
            }
            HirStmt::Raise { exception, cause } => {
                if let Some(exc) = exception {
                    self.analyze_expression(exc, 0);
                }
                if let Some(c) = cause {
                    self.analyze_expression(c, 0);
                }
            }
            HirStmt::Break { .. } | HirStmt::Continue { .. } | HirStmt::Pass => {
                // Break, continue, and pass don't analyze any expressions
            }
            HirStmt::With {
                context,
                target: _,
                body,
            } => {
                // Analyze context expression
                self.analyze_expression(context, 0);

                // Track the target variable if present
                // Note: We don't track local variables here, only parameters

                // Analyze body statements
                for stmt in body {
                    self.analyze_statement(stmt);
                }
            }
        }
    }

    /// Analyze an expression for parameter usage
    fn analyze_expression(&mut self, expr: &HirExpr, borrow_depth: usize) {
        match expr {
            HirExpr::Var(name) => {
                let in_loop = self.is_in_loop();
                let in_conditional = self.is_in_conditional();
                if let Some(usage) = self.param_usage.get_mut(name) {
                    usage.is_read = true;
                    usage.usage_sites.push(UsageSite {
                        usage_type: UsageType::Read,
                        in_loop,
                        in_conditional,
                        borrow_depth,
                    });
                    if in_loop {
                        usage.used_in_loop = true;
                    }
                }
            }
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(name) = &**value {
                    let in_loop = self.is_in_loop();
                    let in_conditional = self.is_in_conditional();
                    if let Some(usage) = self.param_usage.get_mut(name) {
                        usage.field_accesses.insert(attr.clone());
                        usage.usage_sites.push(UsageSite {
                            usage_type: UsageType::FieldAccess(attr.clone()),
                            in_loop,
                            in_conditional,
                            borrow_depth: borrow_depth + 1,
                        });
                    }
                }
                self.analyze_expression(value, borrow_depth + 1);
            }
            HirExpr::Call { func, args } => {
                // Analyze function calls to determine if parameters are moved
                let in_loop = self.is_in_loop();
                let in_conditional = self.is_in_conditional();
                for (i, arg) in args.iter().enumerate() {
                    if let HirExpr::Var(name) = arg {
                        let takes_ownership = self.function_takes_ownership(func, i);
                        if let Some(usage) = self.param_usage.get_mut(name) {
                            // Conservative: assume ownership transfer unless we know better
                            if takes_ownership {
                                usage.is_moved = true;
                                self.moved_vars.insert(name.clone());
                            }
                            usage.usage_sites.push(UsageSite {
                                usage_type: UsageType::FunctionArg { takes_ownership },
                                in_loop,
                                in_conditional,
                                borrow_depth,
                            });
                        }
                    }
                    self.analyze_expression(arg, borrow_depth);
                }
            }
            HirExpr::Index { base, index } => {
                if let HirExpr::Var(name) = &**base {
                    let in_loop = self.is_in_loop();
                    let in_conditional = self.is_in_conditional();
                    if let Some(usage) = self.param_usage.get_mut(name) {
                        usage.usage_sites.push(UsageSite {
                            usage_type: UsageType::IndexAccess,
                            in_loop,
                            in_conditional,
                            borrow_depth: borrow_depth + 1,
                        });
                    }
                }
                self.analyze_expression(base, borrow_depth + 1);
                self.analyze_expression(index, borrow_depth);
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expression(left, borrow_depth);
                self.analyze_expression(right, borrow_depth);
            }
            HirExpr::Unary { operand, .. } => {
                self.analyze_expression(operand, borrow_depth);
            }
            HirExpr::List(elements) | HirExpr::Tuple(elements) => {
                for elem in elements {
                    self.analyze_expression(elem, borrow_depth);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.analyze_expression(k, borrow_depth);
                    self.analyze_expression(v, borrow_depth);
                }
            }
            HirExpr::Borrow { expr, mutable } => {
                if let HirExpr::Var(name) = &**expr {
                    if *mutable {
                        self.mut_borrowed_vars.insert(name.clone());
                    } else {
                        self.immut_borrowed_vars.insert(name.clone());
                    }
                }
                self.analyze_expression(expr, borrow_depth + 1);
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.analyze_expression(object, borrow_depth);
                for arg in args {
                    self.analyze_expression(arg, borrow_depth);
                }
            }
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                self.analyze_expression(base, borrow_depth);
                if let Some(s) = start {
                    self.analyze_expression(s, borrow_depth);
                }
                if let Some(s) = stop {
                    self.analyze_expression(s, borrow_depth);
                }
                if let Some(s) = step {
                    self.analyze_expression(s, borrow_depth);
                }
            }
            HirExpr::Literal(_) => {}
            HirExpr::ListComp {
                element,
                target: _,
                iter,
                condition,
            } => {
                // List comprehensions create a new scope
                self.context_stack.push(AnalysisContext::Loop);

                // Analyze the iterator
                self.analyze_expression(iter, borrow_depth);

                // The target variable is local to the comprehension
                // We don't track it as a parameter usage

                // Analyze the element expression
                self.analyze_expression(element, borrow_depth);

                // Analyze the condition if present
                if let Some(cond) = condition {
                    self.analyze_expression(cond, borrow_depth);
                }

                self.context_stack.pop();
            }
            HirExpr::Lambda { params: _, body } => {
                // Lambda functions capture variables by reference
                // For now, treat lambda bodies like any other expression
                self.analyze_expression(body, borrow_depth);
            }
            HirExpr::Set(elements) | HirExpr::FrozenSet(elements) => {
                for elem in elements {
                    self.analyze_expression(elem, borrow_depth);
                }
            }
            HirExpr::SetComp {
                element,
                target: _,
                iter,
                condition,
            } => {
                // Set comprehensions create a new scope
                self.context_stack.push(AnalysisContext::Loop);

                // Analyze the iterator
                self.analyze_expression(iter, borrow_depth);

                // The target variable is local to the comprehension
                // We don't track it as a parameter usage

                // Analyze the element expression
                self.analyze_expression(element, borrow_depth);

                // Analyze the condition if present
                if let Some(cond) = condition {
                    self.analyze_expression(cond, borrow_depth);
                }

                self.context_stack.pop();
            }
            HirExpr::Await { value } => {
                // Await expressions don't change parameter usage patterns
                self.analyze_expression(value, borrow_depth);
            }
        }
    }

    /// Analyze expression in return context
    fn analyze_expression_for_return(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Var(name) => {
                if let Some(usage) = self.param_usage.get_mut(name) {
                    usage.escapes_through_return = true;
                    usage.usage_sites.push(UsageSite {
                        usage_type: UsageType::Return,
                        in_loop: false,
                        in_conditional: false,
                        borrow_depth: 0,
                    });
                }
            }
            HirExpr::Binary { left, right, .. } => {
                // For binary operations in return context, parameters are used but not directly returned
                self.analyze_expression(left, 0);
                self.analyze_expression(right, 0);
                // Mark any parameter used here as escaping since it contributes to the return value
                if let HirExpr::Var(name) = &**left {
                    if let Some(usage) = self.param_usage.get_mut(name) {
                        usage.escapes_through_return = true;
                    }
                }
            }
            _ => self.analyze_expression(expr, 0),
        }
    }

    /// Determine if a function takes ownership of its argument
    fn function_takes_ownership(&self, func_name: &str, _arg_index: usize) -> bool {
        // Known functions that borrow
        let borrowing_functions = [
            "len",
            "str",
            "repr",
            "format",
            "print",
            "isinstance",
            "hasattr",
            "getattr",
            "contains",
            "startswith",
            "endswith",
            "find",
            "index",
            "count",
        ];

        // Known functions that take ownership
        let ownership_functions = ["append", "extend", "insert", "remove", "pop", "sort"];

        if borrowing_functions.contains(&func_name) {
            false
        } else if ownership_functions.contains(&func_name) {
            true
        } else {
            // Conservative default: assume ownership transfer
            true
        }
    }

    /// Check if currently in a loop context
    fn is_in_loop(&self) -> bool {
        self.context_stack
            .iter()
            .any(|ctx| matches!(ctx, AnalysisContext::Loop))
    }

    /// Check if currently in a conditional context
    fn is_in_conditional(&self) -> bool {
        self.context_stack
            .iter()
            .any(|ctx| matches!(ctx, AnalysisContext::Conditional))
    }

    /// Determine optimal borrowing strategies based on usage patterns
    fn determine_strategies(
        &self,
        func: &HirFunction,
        type_mapper: &TypeMapper,
    ) -> BorrowingAnalysisResult {
        let mut strategies = IndexMap::new();
        let mut insights = Vec::new();

        for param in &func.params {
            let usage = self
                .param_usage
                .get(&param.name)
                .cloned()
                .unwrap_or_default();
            let rust_type = type_mapper.map_type(&param.ty);

            let strategy = self.determine_parameter_strategy(
                &param.name,
                &usage,
                &rust_type,
                &param.ty,
                &mut insights,
            );

            strategies.insert(param.name.clone(), strategy);
        }

        BorrowingAnalysisResult {
            param_strategies: strategies,
            insights,
        }
    }

    /// Determine strategy for a single parameter
    fn determine_parameter_strategy(
        &self,
        param_name: &str,
        usage: &ParameterUsagePattern,
        rust_type: &RustType,
        python_type: &PythonType,
        insights: &mut Vec<BorrowingInsight>,
    ) -> BorrowingStrategy {
        // Always check if type is Copy for insights
        if self.is_copy_type(rust_type) {
            insights.push(BorrowingInsight::SuggestCopyDerive(param_name.to_string()));
        }

        // If parameter is moved, we must take ownership
        if usage.is_moved {
            // Check if move is necessary
            if !usage.escapes_through_return && !usage.is_stored {
                insights.push(BorrowingInsight::UnnecessaryMove(param_name.to_string()));
            }
            return BorrowingStrategy::TakeOwnership;
        }

        // If parameter escapes through return and matches return type, take ownership
        // (except for strings which have special handling)
        if usage.escapes_through_return && !matches!(python_type, PythonType::String) {
            if let Some(ref ret_type) = self.return_type {
                if python_type == ret_type {
                    return BorrowingStrategy::TakeOwnership;
                }
            }
        }

        // If parameter is stored in a structure, consider shared ownership
        if usage.is_stored {
            return BorrowingStrategy::UseSharedOwnership {
                is_thread_safe: false,
            };
        }

        // If used in closure, determine capture strategy
        if usage.used_in_closure {
            // Complex analysis needed - for now, be conservative
            return BorrowingStrategy::TakeOwnership;
        }

        // Check if type is Copy - take ownership (cheap)
        if self.is_copy_type(rust_type) {
            return BorrowingStrategy::TakeOwnership; // Cheap to copy
        }

        // String-specific optimizations
        if matches!(python_type, PythonType::String) {
            return self.determine_string_strategy(param_name, usage);
        }

        // Determine mutability needs
        if usage.is_mutated {
            BorrowingStrategy::BorrowMutable { lifetime: None }
        } else if usage.is_read {
            BorrowingStrategy::BorrowImmutable { lifetime: None }
        } else {
            // Parameter unused - take ownership (simplest)
            BorrowingStrategy::TakeOwnership
        }
    }

    /// Determine optimal string handling strategy
    fn determine_string_strategy(
        &self,
        _param_name: &str,
        usage: &ParameterUsagePattern,
    ) -> BorrowingStrategy {
        // For strings that are reassigned (not string mutation itself),
        // we can actually take ownership since we're replacing the entire string
        // This is a Python-specific pattern where `s = s + "!"` creates a new string

        // If string is moved to another function, we need ownership
        if usage.is_moved {
            return BorrowingStrategy::TakeOwnership;
        }

        // If string is reassigned (Python pattern), take ownership
        // This must come before escape check to handle mutation correctly
        if usage.is_mutated {
            return BorrowingStrategy::TakeOwnership;
        }

        // If string escapes, we need to check lifetime requirements
        if usage.escapes_through_return {
            // Use Cow for maximum flexibility
            return BorrowingStrategy::UseCow {
                lifetime: "'static".to_string(),
            };
        }

        // For read-only strings, prefer borrowing
        if usage.is_read && !usage.is_moved && !usage.is_mutated {
            return BorrowingStrategy::BorrowImmutable { lifetime: None };
        }

        // Default to ownership for simplicity
        BorrowingStrategy::TakeOwnership
    }

    /// Check if a type implements Copy
    #[allow(clippy::only_used_in_recursion)]
    fn is_copy_type(&self, rust_type: &RustType) -> bool {
        match rust_type {
            RustType::Primitive(_) => true,
            RustType::Unit => true,
            RustType::Tuple(types) => types.iter().all(|t| self.is_copy_type(t)),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{FunctionProperties, HirParam, Literal};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_basic_borrowing_analysis() {
        let mut ctx = BorrowingContext::new(Some(PythonType::Int));
        let type_mapper = TypeMapper::new();

        let func = HirFunction {
            name: "add_one".to_string(),
            params: smallvec![HirParam::new("x".to_string(), PythonType::Int)],
            ret_type: PythonType::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: crate::hir::BinOp::Add,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = ctx.analyze_function(&func, &type_mapper);

        // Integer parameter should be taken by value (Copy type)
        let x_strategy = result.param_strategies.get("x").unwrap();
        assert_eq!(*x_strategy, BorrowingStrategy::TakeOwnership);
    }

    #[test]
    fn test_string_borrowing() {
        let mut ctx = BorrowingContext::new(Some(PythonType::Int));
        let type_mapper = TypeMapper::new();

        let func = HirFunction {
            name: "string_len".to_string(),
            params: smallvec![HirParam::new("s".to_string(), PythonType::String)],
            ret_type: PythonType::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("s".to_string())],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = ctx.analyze_function(&func, &type_mapper);

        // String parameter should be borrowed (not moved)
        let s_strategy = result.param_strategies.get("s").unwrap();
        assert!(matches!(
            s_strategy,
            BorrowingStrategy::BorrowImmutable { .. }
        ));
    }

    #[test]
    fn test_mutation_detection() {
        let mut ctx = BorrowingContext::new(None);
        let type_mapper = TypeMapper::new();

        let func = HirFunction {
            name: "mutate_list".to_string(),
            params: smallvec![HirParam::new(
                "lst".to_string(),
                PythonType::List(Box::new(PythonType::Int))
            )],
            ret_type: PythonType::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "append".to_string(),
                args: vec![
                    HirExpr::Var("lst".to_string()),
                    HirExpr::Literal(Literal::Int(42)),
                ],
            })],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = ctx.analyze_function(&func, &type_mapper);

        // List parameter should be moved (append takes ownership in our analysis)
        let lst_strategy = result.param_strategies.get("lst").unwrap();
        assert_eq!(*lst_strategy, BorrowingStrategy::TakeOwnership);
    }
}
