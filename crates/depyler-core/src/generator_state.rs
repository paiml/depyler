//! Generator State Analysis
//!
//! Analyzes generator functions to determine:
//! - Which local variables need to be preserved across yields
//! - Yield points and control flow
//! - State machine structure

use crate::hir::{HirExpr, HirFunction, HirStmt, Type};
use std::collections::HashSet;

/// Information about a generator's state requirements
#[derive(Debug, Clone)]
pub struct GeneratorStateInfo {
    /// Local variables that need to be in the state struct
    pub state_variables: Vec<StateVariable>,
    /// Parameters that are used after yield points
    pub captured_params: Vec<String>,
    /// Number of yield points in the function
    pub yield_count: usize,
    /// Whether the generator has loops
    pub has_loops: bool,
}

#[derive(Debug, Clone)]
pub struct StateVariable {
    pub name: String,
    pub ty: Type,
}

impl GeneratorStateInfo {
    /// Analyze a generator function to determine state requirements
    pub fn analyze(func: &HirFunction) -> Self {
        let mut analyzer = StateAnalyzer {
            state_variables: Vec::new(),
            captured_params: HashSet::new(),
            yield_count: 0,
            has_loops: false,
            declared_vars: HashSet::new(),
        };

        analyzer.analyze_statements(&func.body);

        // Convert param names to Vec
        let captured_params: Vec<String> = func
            .params
            .iter()
            .filter(|p| analyzer.captured_params.contains(&p.name))
            .map(|p| p.name.clone())
            .collect();

        GeneratorStateInfo {
            state_variables: analyzer.state_variables,
            captured_params,
            yield_count: analyzer.yield_count,
            has_loops: analyzer.has_loops,
        }
    }
}

struct StateAnalyzer {
    state_variables: Vec<StateVariable>,
    captured_params: HashSet<String>,
    yield_count: usize,
    has_loops: bool,
    declared_vars: HashSet<String>,
}

impl StateAnalyzer {
    fn analyze_statements(&mut self, stmts: &[HirStmt]) {
        for stmt in stmts {
            self.analyze_statement(stmt);
        }
    }

    fn analyze_statement(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign {
                target,
                value,
                type_annotation,
            } => {
                self.analyze_assign(target, value, type_annotation);
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_for_loop(iter, body);
            }
            HirStmt::While { condition, body } => {
                self.analyze_while_loop(condition, body);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_if_stmt(condition, then_body, else_body);
            }
            HirStmt::Expr(expr) | HirStmt::Return(Some(expr)) => {
                self.analyze_expression(expr);
            }
            // DEPYLER-0561: Analyze with statements for generator state capture
            HirStmt::With { context, body, .. } => {
                self.analyze_expression(context);
                self.analyze_statements(body);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                self.analyze_statements(body);
                for handler in handlers {
                    self.analyze_statements(&handler.body);
                }
                if let Some(else_stmts) = orelse {
                    self.analyze_statements(else_stmts);
                }
                if let Some(final_stmts) = finalbody {
                    self.analyze_statements(final_stmts);
                }
            }
            _ => {}
        }
    }

    /// DEPYLER-0258: Infer type from value expression when no annotation provided
    /// Complexity: 8 (within ≤10 target)
    fn infer_type_from_expression(expr: &HirExpr) -> Type {
        match expr {
            HirExpr::Literal(lit) => match lit {
                crate::hir::Literal::Int(_) => Type::Int,
                crate::hir::Literal::Float(_) => Type::Float,
                crate::hir::Literal::String(_) => Type::String,
                crate::hir::Literal::Bytes(_) => Type::Custom("bytes".to_string()),
                crate::hir::Literal::Bool(_) => Type::Bool,
                crate::hir::Literal::None => Type::None,
            },
            HirExpr::List(items) => {
                // Infer element type from first item
                let elem_type = items
                    .first()
                    .map(Self::infer_type_from_expression)
                    .unwrap_or(Type::Unknown);
                Type::List(Box::new(elem_type))
            }
            HirExpr::Dict(_) => Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
            HirExpr::Set(_) => Type::Set(Box::new(Type::Unknown)),
            // For complex expressions, default to Unknown
            _ => Type::Unknown,
        }
    }

    /// DEPYLER-0494: Analyze assignment target (handles both Symbol and Tuple)
    /// Complexity: 7 (within ≤10 target)
    fn analyze_assign(
        &mut self,
        target: &crate::hir::AssignTarget,
        value: &HirExpr,
        type_annotation: &Option<Type>,
    ) {
        match target {
            crate::hir::AssignTarget::Symbol(name) => {
                let name_str = name.as_str();
                if !self.declared_vars.contains(name_str) {
                    self.declared_vars.insert(name_str.to_string());
                    // DEPYLER-0258 FIX: Infer type from value expression if no annotation
                    let ty = type_annotation
                        .clone()
                        .unwrap_or_else(|| Self::infer_type_from_expression(value));
                    self.state_variables.push(StateVariable {
                        name: name_str.to_string(),
                        ty,
                    });
                }
            }
            // DEPYLER-0494 FIX: Handle tuple unpacking (a, b) = (0, 1)
            crate::hir::AssignTarget::Tuple(targets) => {
                // Infer element types from tuple value if possible
                let element_types = if let HirExpr::Tuple(values) = value {
                    // Parallel tuple: (a, b) = (val1, val2)
                    values
                        .iter()
                        .map(Self::infer_type_from_expression)
                        .collect::<Vec<_>>()
                } else {
                    // Can't infer from value, use Unknown
                    vec![]
                };

                for (idx, target_elem) in targets.iter().enumerate() {
                    if let crate::hir::AssignTarget::Symbol(name) = target_elem {
                        let name_str = name.as_str();
                        if !self.declared_vars.contains(name_str) {
                            self.declared_vars.insert(name_str.to_string());
                            // Try to get type from parallel value or use Unknown
                            let ty = element_types.get(idx).cloned().unwrap_or(Type::Unknown);
                            self.state_variables.push(StateVariable {
                                name: name_str.to_string(),
                                ty,
                            });
                        }
                    }
                }
            }
            _ => {} // Index and Attribute assignments don't declare new variables
        }
        self.analyze_expression(value);
    }

    fn analyze_for_loop(&mut self, iter: &HirExpr, body: &[HirStmt]) {
        self.has_loops = true;
        self.analyze_expression(iter);
        self.analyze_statements(body);
    }

    fn analyze_while_loop(&mut self, condition: &HirExpr, body: &[HirStmt]) {
        self.has_loops = true;
        self.analyze_expression(condition);
        self.analyze_statements(body);
    }

    fn analyze_if_stmt(
        &mut self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) {
        self.analyze_expression(condition);
        self.analyze_statements(then_body);
        if let Some(else_stmts) = else_body {
            self.analyze_statements(else_stmts);
        }
    }

    fn analyze_expression(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Yield { value } => self.analyze_yield(value),
            HirExpr::Var(name) => self.analyze_variable(name),
            HirExpr::Binary { left, right, .. } => self.analyze_binary(left, right),
            HirExpr::Unary { operand, .. } => self.analyze_expression(operand),
            HirExpr::Call { args, .. } | HirExpr::List(args) | HirExpr::Tuple(args) => {
                self.analyze_expressions(args);
            }
            HirExpr::Index { base, index } => self.analyze_binary(base, index),
            HirExpr::MethodCall { object, args, .. } => {
                self.analyze_expression(object);
                self.analyze_expressions(args);
            }
            _ => {}
        }
    }

    fn analyze_yield(&mut self, value: &Option<Box<HirExpr>>) {
        self.yield_count += 1;
        if let Some(v) = value {
            self.analyze_expression(v);
        }
    }

    fn analyze_variable(&mut self, name: &str) {
        let name_str = name;
        if !self.declared_vars.contains(name_str) {
            self.captured_params.insert(name_str.to_string());
        }
    }

    fn analyze_binary(&mut self, left: &HirExpr, right: &HirExpr) {
        self.analyze_expression(left);
        self.analyze_expression(right);
    }

    fn analyze_expressions(&mut self, exprs: &[HirExpr]) {
        for expr in exprs {
            self.analyze_expression(expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, ExceptHandler, FunctionProperties, HirParam, Literal, UnaryOp};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    // ============================================
    // Helper function for creating test functions
    // ============================================

    fn make_func(name: &str, params: Vec<HirParam>, body: Vec<HirStmt>) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params.into(),
            ret_type: Type::Int,
            body,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    fn make_param(name: &str, ty: Type) -> HirParam {
        HirParam::new(name.to_string(), ty)
    }

    fn make_assign(name: &str, value: HirExpr, ty: Option<Type>) -> HirStmt {
        HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol(name.to_string()),
            value,
            type_annotation: ty,
        }
    }

    fn make_yield(value: Option<HirExpr>) -> HirExpr {
        HirExpr::Yield {
            value: value.map(Box::new),
        }
    }

    // ============================================
    // GeneratorStateInfo struct tests
    // ============================================

    #[test]
    fn test_generator_state_info_clone() {
        let info = GeneratorStateInfo {
            state_variables: vec![StateVariable {
                name: "x".to_string(),
                ty: Type::Int,
            }],
            captured_params: vec!["n".to_string()],
            yield_count: 2,
            has_loops: true,
        };
        let cloned = info.clone();
        assert_eq!(cloned.state_variables.len(), 1);
        assert_eq!(cloned.yield_count, 2);
        assert!(cloned.has_loops);
    }

    #[test]
    fn test_generator_state_info_debug() {
        let info = GeneratorStateInfo {
            state_variables: vec![],
            captured_params: vec![],
            yield_count: 0,
            has_loops: false,
        };
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("GeneratorStateInfo"));
    }

    // ============================================
    // StateVariable struct tests
    // ============================================

    #[test]
    fn test_state_variable_clone() {
        let var = StateVariable {
            name: "counter".to_string(),
            ty: Type::Float,
        };
        let cloned = var.clone();
        assert_eq!(cloned.name, "counter");
        assert!(matches!(cloned.ty, Type::Float));
    }

    #[test]
    fn test_state_variable_debug() {
        let var = StateVariable {
            name: "x".to_string(),
            ty: Type::Bool,
        };
        let debug_str = format!("{:?}", var);
        assert!(debug_str.contains("StateVariable"));
        assert!(debug_str.contains("x"));
    }

    // ============================================
    // Empty and minimal function tests
    // ============================================

    #[test]
    fn test_analyze_empty_function() {
        let func = make_func("empty", vec![], vec![]);
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.state_variables.len(), 0);
        assert_eq!(info.captured_params.len(), 0);
        assert_eq!(info.yield_count, 0);
        assert!(!info.has_loops);
    }

    #[test]
    fn test_analyze_function_with_only_yield() {
        let func = make_func(
            "simple_yield",
            vec![],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                Literal::Int(42),
            ))))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 1);
        assert!(!info.has_loops);
    }

    #[test]
    fn test_analyze_multiple_yields() {
        let func = make_func(
            "multi_yield",
            vec![],
            vec![
                HirStmt::Expr(make_yield(Some(HirExpr::Literal(Literal::Int(1))))),
                HirStmt::Expr(make_yield(Some(HirExpr::Literal(Literal::Int(2))))),
                HirStmt::Expr(make_yield(Some(HirExpr::Literal(Literal::Int(3))))),
            ],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 3);
    }

    #[test]
    fn test_analyze_yield_none() {
        let func = make_func("yield_none", vec![], vec![HirStmt::Expr(make_yield(None))]);
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 1);
    }

    // ============================================
    // Type inference tests
    // ============================================

    #[test]
    fn test_infer_type_int_literal() {
        let func = make_func(
            "int_infer",
            vec![],
            vec![make_assign(
                "x",
                HirExpr::Literal(Literal::Int(42)),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.state_variables.len(), 1);
        assert_eq!(info.state_variables[0].name, "x");
        assert!(matches!(info.state_variables[0].ty, Type::Int));
    }

    #[test]
    fn test_infer_type_float_literal() {
        let func = make_func(
            "float_infer",
            vec![],
            vec![make_assign(
                "y",
                HirExpr::Literal(Literal::Float(3.14)),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::Float));
    }

    #[test]
    fn test_infer_type_string_literal() {
        let func = make_func(
            "string_infer",
            vec![],
            vec![make_assign(
                "s",
                HirExpr::Literal(Literal::String("hello".to_string())),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::String));
    }

    #[test]
    fn test_infer_type_bool_literal() {
        let func = make_func(
            "bool_infer",
            vec![],
            vec![make_assign(
                "b",
                HirExpr::Literal(Literal::Bool(true)),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::Bool));
    }

    #[test]
    fn test_infer_type_none_literal() {
        let func = make_func(
            "none_infer",
            vec![],
            vec![make_assign("n", HirExpr::Literal(Literal::None), None)],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::None));
    }

    #[test]
    fn test_infer_type_bytes_literal() {
        let func = make_func(
            "bytes_infer",
            vec![],
            vec![make_assign(
                "data",
                HirExpr::Literal(Literal::Bytes(vec![1, 2, 3])),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::Custom(ref s) if s == "bytes"));
    }

    #[test]
    fn test_infer_type_list_with_elements() {
        let func = make_func(
            "list_infer",
            vec![],
            vec![make_assign(
                "items",
                HirExpr::List(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Literal(Literal::Int(2)),
                ]),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        if let Type::List(inner) = &info.state_variables[0].ty {
            assert!(matches!(**inner, Type::Int));
        } else {
            panic!("Expected List type");
        }
    }

    #[test]
    fn test_infer_type_empty_list() {
        let func = make_func(
            "empty_list",
            vec![],
            vec![make_assign("items", HirExpr::List(vec![]), None)],
        );
        let info = GeneratorStateInfo::analyze(&func);

        if let Type::List(inner) = &info.state_variables[0].ty {
            assert!(matches!(**inner, Type::Unknown));
        } else {
            panic!("Expected List type");
        }
    }

    #[test]
    fn test_infer_type_dict() {
        let func = make_func(
            "dict_infer",
            vec![],
            vec![make_assign("d", HirExpr::Dict(vec![]), None)],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::Dict(_, _)));
    }

    #[test]
    fn test_infer_type_set() {
        let func = make_func(
            "set_infer",
            vec![],
            vec![make_assign(
                "s",
                HirExpr::Set(vec![HirExpr::Literal(Literal::Int(1))]),
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(matches!(info.state_variables[0].ty, Type::Set(_)));
    }

    #[test]
    fn test_infer_type_complex_expression() {
        let func = make_func(
            "complex",
            vec![],
            vec![make_assign(
                "z",
                HirExpr::Binary {
                    left: Box::new(HirExpr::Var("a".to_string())),
                    op: BinOp::Add,
                    right: Box::new(HirExpr::Var("b".to_string())),
                },
                None,
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        // Complex expressions default to Unknown
        assert!(matches!(info.state_variables[0].ty, Type::Unknown));
    }

    #[test]
    fn test_explicit_type_annotation_overrides_inference() {
        let func = make_func(
            "explicit",
            vec![],
            vec![make_assign(
                "x",
                HirExpr::Literal(Literal::Int(42)),
                Some(Type::Float), // Explicit annotation
            )],
        );
        let info = GeneratorStateInfo::analyze(&func);

        // Explicit annotation takes precedence
        assert!(matches!(info.state_variables[0].ty, Type::Float));
    }

    // ============================================
    // Loop detection tests
    // ============================================

    #[test]
    fn test_for_loop_detection() {
        let func = make_func(
            "for_gen",
            vec![],
            vec![HirStmt::For {
                target: crate::hir::AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::List(vec![]),
                body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Var(
                    "i".to_string(),
                ))))],
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
        assert_eq!(info.yield_count, 1);
    }

    #[test]
    fn test_while_loop_detection() {
        let func = make_func(
            "while_gen",
            vec![],
            vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Expr(make_yield(None))],
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
    }

    #[test]
    fn test_nested_loops() {
        let func = make_func(
            "nested",
            vec![],
            vec![HirStmt::For {
                target: crate::hir::AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::List(vec![]),
                body: vec![HirStmt::For {
                    target: crate::hir::AssignTarget::Symbol("j".to_string()),
                    iter: HirExpr::List(vec![]),
                    body: vec![HirStmt::Expr(make_yield(None))],
                }],
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
    }

    // ============================================
    // Parameter capture tests
    // ============================================

    #[test]
    fn test_capture_parameter_used_in_expression() {
        let func = make_func(
            "param_capture",
            vec![make_param("n", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Var(
                "n".to_string(),
            ))))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"n".to_string()));
    }

    #[test]
    fn test_unused_parameter_not_captured() {
        let func = make_func(
            "unused_param",
            vec![make_param("unused", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                Literal::Int(42),
            ))))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.is_empty());
    }

    #[test]
    fn test_multiple_params_partial_capture() {
        let func = make_func(
            "partial",
            vec![
                make_param("a", Type::Int),
                make_param("b", Type::Int),
                make_param("c", Type::Int),
            ],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Binary {
                left: Box::new(HirExpr::Var("a".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("c".to_string())),
            })))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"a".to_string()));
        assert!(info.captured_params.contains(&"c".to_string()));
        assert!(!info.captured_params.contains(&"b".to_string()));
    }

    // ============================================
    // State variable tracking tests
    // ============================================

    #[test]
    fn test_variable_declared_before_use_not_param() {
        let func = make_func(
            "declared",
            vec![],
            vec![
                make_assign("x", HirExpr::Literal(Literal::Int(0)), Some(Type::Int)),
                HirStmt::Expr(make_yield(Some(HirExpr::Var("x".to_string())))),
            ],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.is_empty());
        assert_eq!(info.state_variables.len(), 1);
        assert_eq!(info.state_variables[0].name, "x");
    }

    #[test]
    fn test_variable_reassignment_not_duplicated() {
        let func = make_func(
            "reassign",
            vec![],
            vec![
                make_assign("x", HirExpr::Literal(Literal::Int(0)), Some(Type::Int)),
                make_assign("x", HirExpr::Literal(Literal::Int(1)), None),
                make_assign("x", HirExpr::Literal(Literal::Int(2)), None),
            ],
        );
        let info = GeneratorStateInfo::analyze(&func);

        // Only one state variable entry despite multiple assignments
        assert_eq!(info.state_variables.len(), 1);
    }

    // ============================================
    // Tuple unpacking tests (DEPYLER-0494)
    // ============================================

    #[test]
    fn test_tuple_unpacking_simple() {
        let func = make_func(
            "tuple_unpack",
            vec![],
            vec![HirStmt::Assign {
                target: crate::hir::AssignTarget::Tuple(vec![
                    crate::hir::AssignTarget::Symbol("a".to_string()),
                    crate::hir::AssignTarget::Symbol("b".to_string()),
                ]),
                value: HirExpr::Tuple(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Literal(Literal::Int(2)),
                ]),
                type_annotation: None,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.state_variables.len(), 2);
        let names: Vec<&str> = info.state_variables.iter().map(|v| v.name.as_str()).collect();
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
    }

    #[test]
    fn test_tuple_unpacking_type_inference() {
        let func = make_func(
            "tuple_types",
            vec![],
            vec![HirStmt::Assign {
                target: crate::hir::AssignTarget::Tuple(vec![
                    crate::hir::AssignTarget::Symbol("x".to_string()),
                    crate::hir::AssignTarget::Symbol("y".to_string()),
                ]),
                value: HirExpr::Tuple(vec![
                    HirExpr::Literal(Literal::Int(42)),
                    HirExpr::Literal(Literal::String("hello".to_string())),
                ]),
                type_annotation: None,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        let x_var = info.state_variables.iter().find(|v| v.name == "x").unwrap();
        let y_var = info.state_variables.iter().find(|v| v.name == "y").unwrap();
        assert!(matches!(x_var.ty, Type::Int));
        assert!(matches!(y_var.ty, Type::String));
    }

    #[test]
    fn test_tuple_unpacking_non_tuple_value() {
        let func = make_func(
            "non_tuple",
            vec![],
            vec![HirStmt::Assign {
                target: crate::hir::AssignTarget::Tuple(vec![
                    crate::hir::AssignTarget::Symbol("a".to_string()),
                    crate::hir::AssignTarget::Symbol("b".to_string()),
                ]),
                value: HirExpr::Var("some_tuple".to_string()), // Not a literal tuple
                type_annotation: None,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        // Can't infer types, should be Unknown
        for var in &info.state_variables {
            assert!(matches!(var.ty, Type::Unknown));
        }
    }

    // ============================================
    // If statement analysis tests
    // ============================================

    #[test]
    fn test_if_statement_then_only() {
        let func = make_func(
            "if_then",
            vec![make_param("cond", Type::Bool)],
            vec![HirStmt::If {
                condition: HirExpr::Var("cond".to_string()),
                then_body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                    Literal::Int(1),
                ))))],
                else_body: None,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 1);
        assert!(info.captured_params.contains(&"cond".to_string()));
    }

    #[test]
    fn test_if_statement_with_else() {
        let func = make_func(
            "if_else",
            vec![],
            vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                    Literal::Int(1),
                ))))],
                else_body: Some(vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                    Literal::Int(2),
                ))))]),
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 2);
    }

    // ============================================
    // Try/Except statement tests
    // ============================================

    #[test]
    fn test_try_except_body() {
        let func = make_func(
            "try_except",
            vec![],
            vec![HirStmt::Try {
                body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                    Literal::Int(1),
                ))))],
                handlers: vec![ExceptHandler {
                    exception_type: None,
                    name: None,
                    body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                        Literal::Int(2),
                    ))))],
                }],
                orelse: None,
                finalbody: None,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 2);
    }

    #[test]
    fn test_try_with_finally() {
        let func = make_func(
            "try_finally",
            vec![],
            vec![HirStmt::Try {
                body: vec![HirStmt::Expr(make_yield(None))],
                handlers: vec![],
                orelse: None,
                finalbody: Some(vec![
                    make_assign("cleanup", HirExpr::Literal(Literal::Bool(true)), None),
                ]),
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 1);
        assert!(info.state_variables.iter().any(|v| v.name == "cleanup"));
    }

    #[test]
    fn test_try_with_orelse() {
        let func = make_func(
            "try_else",
            vec![],
            vec![HirStmt::Try {
                body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                    Literal::Int(1),
                ))))],
                handlers: vec![],
                orelse: Some(vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
                    Literal::Int(2),
                ))))]),
                finalbody: None,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 2);
    }

    // ============================================
    // With statement tests (DEPYLER-0561)
    // ============================================

    #[test]
    fn test_with_statement() {
        let func = make_func(
            "with_gen",
            vec![make_param("ctx", Type::Custom("ContextManager".to_string()))],
            vec![HirStmt::With {
                context: HirExpr::Var("ctx".to_string()),
                target: Some("f".to_string()),
                body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Var(
                    "f".to_string(),
                ))))],
                is_async: false,
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.yield_count, 1);
        // 'ctx' is a param used in the context expression, so it's captured
        assert!(info.captured_params.contains(&"ctx".to_string()));
    }

    // ============================================
    // Expression analysis tests
    // ============================================

    #[test]
    fn test_binary_expression_captures_variables() {
        let func = make_func(
            "binary",
            vec![make_param("a", Type::Int), make_param("b", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Binary {
                left: Box::new(HirExpr::Var("a".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("b".to_string())),
            })))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"a".to_string()));
        assert!(info.captured_params.contains(&"b".to_string()));
    }

    #[test]
    fn test_unary_expression() {
        let func = make_func(
            "unary",
            vec![make_param("x", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(HirExpr::Var("x".to_string())),
            })))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"x".to_string()));
    }

    #[test]
    fn test_call_expression() {
        let func = make_func(
            "call",
            vec![make_param("arg", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Call {
                func: "some_func".to_string(),
                args: vec![HirExpr::Var("arg".to_string())],
                kwargs: vec![],
            })))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"arg".to_string()));
    }

    #[test]
    fn test_method_call_expression() {
        let func = make_func(
            "method_call",
            vec![make_param("obj", Type::Custom("MyClass".to_string()))],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("obj".to_string())),
                method: "method".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(42))],
                kwargs: vec![],
            })))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"obj".to_string()));
    }

    #[test]
    fn test_index_expression() {
        let func = make_func(
            "index",
            vec![make_param("arr", Type::List(Box::new(Type::Int)))],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            })))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"arr".to_string()));
    }

    #[test]
    fn test_list_expression_analyzes_elements() {
        let func = make_func(
            "list_expr",
            vec![make_param("x", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::List(vec![
                HirExpr::Var("x".to_string()),
                HirExpr::Literal(Literal::Int(42)),
            ]))))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"x".to_string()));
    }

    #[test]
    fn test_tuple_expression_analyzes_elements() {
        let func = make_func(
            "tuple_expr",
            vec![make_param("a", Type::Int), make_param("b", Type::Int)],
            vec![HirStmt::Expr(make_yield(Some(HirExpr::Tuple(vec![
                HirExpr::Var("a".to_string()),
                HirExpr::Var("b".to_string()),
            ]))))],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"a".to_string()));
        assert!(info.captured_params.contains(&"b".to_string()));
    }

    // ============================================
    // Return statement tests
    // ============================================

    #[test]
    fn test_return_with_expression() {
        let func = make_func(
            "return_expr",
            vec![make_param("x", Type::Int)],
            vec![
                HirStmt::Expr(make_yield(Some(HirExpr::Var("x".to_string())))),
                HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
            ],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.captured_params.contains(&"x".to_string()));
    }

    #[test]
    fn test_return_none() {
        let func = make_func("return_none", vec![], vec![HirStmt::Return(None)]);
        let info = GeneratorStateInfo::analyze(&func);

        // Return(None) should not crash
        assert_eq!(info.yield_count, 0);
    }

    // ============================================
    // Other statement types tests
    // ============================================

    #[test]
    fn test_pass_statement() {
        let func = make_func("pass_func", vec![], vec![HirStmt::Pass]);
        let info = GeneratorStateInfo::analyze(&func);

        assert_eq!(info.state_variables.len(), 0);
        assert_eq!(info.yield_count, 0);
    }

    #[test]
    fn test_break_statement() {
        let func = make_func(
            "break_func",
            vec![],
            vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Break { label: None }],
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
    }

    #[test]
    fn test_continue_statement() {
        let func = make_func(
            "continue_func",
            vec![],
            vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Continue { label: None }],
            }],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
    }

    // ============================================
    // Complex scenario tests
    // ============================================

    #[test]
    fn test_fibonacci_generator() {
        // def fib(n): a, b = 0, 1; while a < n: yield a; a, b = b, a + b
        let func = make_func(
            "fib",
            vec![make_param("n", Type::Int)],
            vec![
                HirStmt::Assign {
                    target: crate::hir::AssignTarget::Tuple(vec![
                        crate::hir::AssignTarget::Symbol("a".to_string()),
                        crate::hir::AssignTarget::Symbol("b".to_string()),
                    ]),
                    value: HirExpr::Tuple(vec![
                        HirExpr::Literal(Literal::Int(0)),
                        HirExpr::Literal(Literal::Int(1)),
                    ]),
                    type_annotation: None,
                },
                HirStmt::While {
                    condition: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("a".to_string())),
                        op: BinOp::Lt,
                        right: Box::new(HirExpr::Var("n".to_string())),
                    },
                    body: vec![
                        HirStmt::Expr(make_yield(Some(HirExpr::Var("a".to_string())))),
                        HirStmt::Assign {
                            target: crate::hir::AssignTarget::Tuple(vec![
                                crate::hir::AssignTarget::Symbol("a".to_string()),
                                crate::hir::AssignTarget::Symbol("b".to_string()),
                            ]),
                            value: HirExpr::Tuple(vec![
                                HirExpr::Var("b".to_string()),
                                HirExpr::Binary {
                                    left: Box::new(HirExpr::Var("a".to_string())),
                                    op: BinOp::Add,
                                    right: Box::new(HirExpr::Var("b".to_string())),
                                },
                            ]),
                            type_annotation: None,
                        },
                    ],
                },
            ],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
        assert_eq!(info.yield_count, 1);
        assert!(info.captured_params.contains(&"n".to_string()));
        assert_eq!(info.state_variables.len(), 2); // a and b
    }

    #[test]
    fn test_range_like_generator() {
        // def range_gen(start, stop, step): i = start; while i < stop: yield i; i += step
        let func = make_func(
            "range_gen",
            vec![
                make_param("start", Type::Int),
                make_param("stop", Type::Int),
                make_param("step", Type::Int),
            ],
            vec![
                make_assign("i", HirExpr::Var("start".to_string()), Some(Type::Int)),
                HirStmt::While {
                    condition: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("i".to_string())),
                        op: BinOp::Lt,
                        right: Box::new(HirExpr::Var("stop".to_string())),
                    },
                    body: vec![
                        HirStmt::Expr(make_yield(Some(HirExpr::Var("i".to_string())))),
                        make_assign(
                            "i",
                            HirExpr::Binary {
                                left: Box::new(HirExpr::Var("i".to_string())),
                                op: BinOp::Add,
                                right: Box::new(HirExpr::Var("step".to_string())),
                            },
                            None,
                        ),
                    ],
                },
            ],
        );
        let info = GeneratorStateInfo::analyze(&func);

        assert!(info.has_loops);
        assert_eq!(info.yield_count, 1);
        // start is used to initialize i, which is a local var
        // stop and step are used after yield points
        assert!(info.captured_params.contains(&"start".to_string()));
        assert!(info.captured_params.contains(&"stop".to_string()));
        assert!(info.captured_params.contains(&"step".to_string()));
        assert_eq!(info.state_variables.len(), 1); // i
    }

    #[test]
    fn test_simple_counter_analysis() {
        // def counter(n): current = 0; while current < n: yield current; current += 1
        let func = HirFunction {
            name: "counter".to_string(),
            params: smallvec![HirParam::new("n".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Assign {
                    target: crate::hir::AssignTarget::Symbol("current".to_string()),
                    value: HirExpr::Literal(crate::hir::Literal::Int(0)),
                    type_annotation: Some(Type::Int),
                },
                HirStmt::While {
                    condition: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("current".to_string())),
                        op: crate::hir::BinOp::Lt,
                        right: Box::new(HirExpr::Var("n".to_string())),
                    },
                    body: vec![
                        HirStmt::Expr(HirExpr::Yield {
                            value: Some(Box::new(HirExpr::Var("current".to_string()))),
                        }),
                        HirStmt::Assign {
                            target: crate::hir::AssignTarget::Symbol("current".to_string()),
                            value: HirExpr::Binary {
                                left: Box::new(HirExpr::Var("current".to_string())),
                                op: crate::hir::BinOp::Add,
                                right: Box::new(HirExpr::Literal(crate::hir::Literal::Int(1))),
                            },
                            type_annotation: Some(Type::Int),
                        },
                    ],
                },
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let state_info = GeneratorStateInfo::analyze(&func);

        assert_eq!(state_info.yield_count, 1, "Should find 1 yield");
        assert!(state_info.has_loops, "Should detect loop");
        assert_eq!(
            state_info.state_variables.len(),
            1,
            "Should find 'current' variable"
        );
        assert_eq!(state_info.state_variables[0].name, "current");
        assert!(
            state_info.captured_params.contains(&"n".to_string()),
            "Should capture parameter 'n'"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0258_type_inference_from_literal_values() {
        // BUG #1: DynamicType inference should infer from value expressions
        // Current: i = 0 (no type annotation) → Type::Unknown
        // Expected: i = 0 (no type annotation) → Type::Int

        let func = HirFunction {
            name: "count_up".to_string(),
            params: smallvec![HirParam::new("n".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![
                // Assignment WITHOUT type annotation - should infer from literal
                HirStmt::Assign {
                    target: crate::hir::AssignTarget::Symbol("i".to_string()),
                    value: HirExpr::Literal(crate::hir::Literal::Int(0)),
                    type_annotation: None, // ← No annotation, must infer!
                },
                HirStmt::While {
                    condition: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("i".to_string())),
                        op: crate::hir::BinOp::Lt,
                        right: Box::new(HirExpr::Var("n".to_string())),
                    },
                    body: vec![
                        HirStmt::Expr(HirExpr::Yield {
                            value: Some(Box::new(HirExpr::Var("i".to_string()))),
                        }),
                        HirStmt::Assign {
                            target: crate::hir::AssignTarget::Symbol("i".to_string()),
                            value: HirExpr::Binary {
                                left: Box::new(HirExpr::Var("i".to_string())),
                                op: crate::hir::BinOp::Add,
                                right: Box::new(HirExpr::Literal(crate::hir::Literal::Int(1))),
                            },
                            type_annotation: None, // ← Reassignment, type already known
                        },
                    ],
                },
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let state_info = GeneratorStateInfo::analyze(&func);

        // Assert: Should find state variable 'i'
        assert_eq!(
            state_info.state_variables.len(),
            1,
            "Should find 'i' variable"
        );
        assert_eq!(state_info.state_variables[0].name, "i");

        // Assert: Type should be inferred as Int from literal value
        // This WILL FAIL (RED phase) because current code uses Type::Unknown
        assert_eq!(
            state_info.state_variables[0].ty,
            Type::Int,
            "DEPYLER-0258: Should infer Type::Int from literal value, not Type::Unknown"
        );
    }
}
