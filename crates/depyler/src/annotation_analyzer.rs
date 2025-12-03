//! Annotation analysis logic for interactive transpilation.
//!
//! This module contains the pure analysis functions that can be tested
//! without requiring terminal I/O.

use depyler_core::hir::{BinOp, HirExpr, HirFunction, HirStmt, Literal, Type};

/// Analyzer for generating annotation suggestions based on code patterns.
#[derive(Debug, Default)]
pub struct AnnotationAnalyzer;

impl AnnotationAnalyzer {
    /// Create a new annotation analyzer.
    pub fn new() -> Self {
        Self
    }

    /// Check if statements contain any loops (for or while).
    pub fn has_loops(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { .. } | HirStmt::While { .. } => true,
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => self.has_loops(then_body) || else_body.as_ref().is_some_and(|e| self.has_loops(e)),
            _ => false,
        })
    }

    /// Check if statements contain nested loops.
    pub fn has_nested_loops(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => self.has_loops(body),
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.has_nested_loops(then_body)
                    || else_body.as_ref().is_some_and(|e| self.has_nested_loops(e))
            }
            _ => false,
        })
    }

    /// Check if statements contain a simple numeric loop (e.g., range iteration).
    pub fn has_simple_numeric_loop(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { iter, body, .. } => {
                // Check if iterating over range
                matches!(iter, HirExpr::Call { func, .. } if func == "range") &&
                // Check if body is simple (no nested loops)
                !self.has_loops(body)
            }
            _ => false,
        })
    }

    /// Check if a function has large collection parameters.
    pub fn has_large_collections(&self, func: &HirFunction) -> bool {
        func.params
            .iter()
            .any(|param| matches!(param.ty, Type::List(_) | Type::Dict(_, _)))
    }

    /// Check if a function modifies its collection arguments.
    pub fn is_collection_modified(&self, func: &HirFunction) -> bool {
        self.has_modification_patterns(&func.body)
    }

    /// Check if statements have collection modification patterns.
    pub fn has_modification_patterns(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::Expr(HirExpr::Call { func, .. }) => {
                matches!(
                    func.as_str(),
                    "append" | "extend" | "insert" | "remove" | "pop" | "clear"
                )
            }
            HirStmt::Expr(_) => false,
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.has_modification_patterns(then_body)
                    || else_body
                        .as_ref()
                        .is_some_and(|e| self.has_modification_patterns(e))
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                self.has_modification_patterns(body)
            }
            _ => false,
        })
    }

    /// Check if a function has frequent lookups in loops.
    pub fn has_frequent_lookups(&self, func: &HirFunction) -> bool {
        self.has_lookup_in_loop(&func.body)
    }

    /// Check if there are lookups inside loop bodies.
    pub fn has_lookup_in_loop(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => self.has_dict_access(body),
            _ => false,
        })
    }

    /// Check if a function has string parameters or returns string.
    pub fn has_string_operations(&self, func: &HirFunction) -> bool {
        func.params
            .iter()
            .any(|param| matches!(param.ty, Type::String))
            || matches!(func.ret_type, Type::String)
    }

    /// Check if statements contain string concatenation.
    pub fn has_string_concatenation(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::Assign { value, .. } => self.has_string_concat_expr(value),
            HirStmt::Return(Some(expr)) => self.has_string_concat_expr(expr),
            _ => false,
        })
    }

    /// Check if an expression is a string concatenation.
    pub fn has_string_concat_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Binary {
                op: BinOp::Add,
                left,
                right,
            } => {
                // Check if either operand is a string
                self.is_string_expr(left) || self.is_string_expr(right)
            }
            _ => false,
        }
    }

    /// Check if an expression is likely a string.
    pub fn is_string_expr(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Literal(Literal::String(_)) | HirExpr::Var(_))
    }

    /// Check if a function only reads strings without modification.
    pub fn only_reads_strings(&self, func: &HirFunction) -> bool {
        !self.has_string_concatenation(&func.body) && !self.has_modification_patterns(&func.body)
    }

    /// Check if a function has array/list index access.
    pub fn has_array_access(&self, func: &HirFunction) -> bool {
        self.has_index_access(&func.body)
    }

    /// Check if statements contain index access operations.
    pub fn has_index_access(&self, stmts: &[HirStmt]) -> bool {
        stmts.iter().any(|stmt| match stmt {
            HirStmt::Assign { value, .. } => self.has_index_expr(value),
            HirStmt::Return(Some(expr)) => self.has_index_expr(expr),
            HirStmt::Expr(expr) => self.has_index_expr(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.has_index_expr(condition)
                    || self.has_index_access(then_body)
                    || else_body.as_ref().is_some_and(|e| self.has_index_access(e))
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => self.has_index_access(body),
            _ => false,
        })
    }

    /// Check if an expression is an index expression.
    pub fn has_index_expr(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Index { .. })
    }

    /// Check if statements contain dictionary access (same as index).
    pub fn has_dict_access(&self, stmts: &[HirStmt]) -> bool {
        self.has_index_access(stmts)
    }

    /// Check if a function accesses shared state.
    pub fn has_shared_state(&self, _func: &HirFunction) -> bool {
        // Simplified check - would need more sophisticated analysis
        false
    }

    /// Calculate cyclomatic complexity of statements.
    pub fn calculate_complexity(&self, stmts: &[HirStmt]) -> u32 {
        stmts
            .iter()
            .map(|stmt| match stmt {
                HirStmt::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    1 + self.calculate_complexity(then_body)
                        + else_body
                            .as_ref()
                            .map_or(0, |e| self.calculate_complexity(e))
                }
                HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                    3 + self.calculate_complexity(body)
                }
                _ => 1,
            })
            .sum()
    }

    /// Find the line number of a function definition in source code.
    pub fn find_function_line(&self, source: &str, func_name: &str) -> usize {
        source
            .lines()
            .enumerate()
            .find(|(_, line)| line.contains(&format!("def {func_name}")))
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::hir::{AssignTarget, HirParam};

    fn make_param(name: &str, ty: Type) -> HirParam {
        HirParam {
            name: name.to_string(),
            ty,
            default: None,
            is_vararg: false,
        }
    }

    fn make_func(
        name: &str,
        params: Vec<HirParam>,
        ret_type: Type,
        body: Vec<HirStmt>,
    ) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params.into_iter().collect(),
            ret_type,
            body,
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    fn make_assign(target: &str, value: HirExpr) -> HirStmt {
        HirStmt::Assign {
            target: AssignTarget::Symbol(target.to_string()),
            value,
            type_annotation: None,
        }
    }

    // --- has_loops tests ---

    #[test]
    fn test_has_loops_empty_body() {
        let analyzer = AnnotationAnalyzer::new();
        assert!(!analyzer.has_loops(&[]));
    }

    #[test]
    fn test_has_loops_no_loops() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![
            make_assign("x", HirExpr::Literal(Literal::Int(1))),
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ];
        assert!(!analyzer.has_loops(&stmts));
    }

    #[test]
    fn test_has_loops_for_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Pass],
        }];
        assert!(analyzer.has_loops(&stmts));
    }

    #[test]
    fn test_has_loops_while_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break { label: None }],
        }];
        assert!(analyzer.has_loops(&stmts));
    }

    #[test]
    fn test_has_loops_in_then_branch() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(5))],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Pass],
            }],
            else_body: None,
        }];
        assert!(analyzer.has_loops(&stmts));
    }

    #[test]
    fn test_has_loops_in_else_branch() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(false)),
            then_body: vec![HirStmt::Pass],
            else_body: Some(vec![HirStmt::While {
                condition: HirExpr::Var("cond".to_string()),
                body: vec![HirStmt::Break { label: None }],
            }]),
        }];
        assert!(analyzer.has_loops(&stmts));
    }

    // --- has_nested_loops tests ---

    #[test]
    fn test_has_nested_loops_empty() {
        let analyzer = AnnotationAnalyzer::new();
        assert!(!analyzer.has_nested_loops(&[]));
    }

    #[test]
    fn test_has_nested_loops_single_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Pass],
        }];
        assert!(!analyzer.has_nested_loops(&stmts));
    }

    #[test]
    fn test_has_nested_loops_for_for() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("j".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(10))],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Pass],
            }],
        }];
        assert!(analyzer.has_nested_loops(&stmts));
    }

    #[test]
    fn test_has_nested_loops_while_for() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Var("cond".to_string()),
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("x".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Pass],
            }],
        }];
        assert!(analyzer.has_nested_loops(&stmts));
    }

    #[test]
    fn test_has_nested_loops_for_while() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::While {
                condition: HirExpr::Var("cond".to_string()),
                body: vec![HirStmt::Break { label: None }],
            }],
        }];
        assert!(analyzer.has_nested_loops(&stmts));
    }

    #[test]
    fn test_has_nested_loops_in_if_then() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("a".to_string()),
                body: vec![HirStmt::For {
                    target: AssignTarget::Symbol("j".to_string()),
                    iter: HirExpr::Var("b".to_string()),
                    body: vec![HirStmt::Pass],
                }],
            }],
            else_body: None,
        }];
        assert!(analyzer.has_nested_loops(&stmts));
    }

    // --- has_simple_numeric_loop tests ---

    #[test]
    fn test_has_simple_numeric_loop_range() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(100))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Pass],
        }];
        assert!(analyzer.has_simple_numeric_loop(&stmts));
    }

    #[test]
    fn test_has_simple_numeric_loop_not_range() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("item".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Pass],
        }];
        assert!(!analyzer.has_simple_numeric_loop(&stmts));
    }

    #[test]
    fn test_has_simple_numeric_loop_with_nested() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("j".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Pass],
            }],
        }];
        // Has nested loop, so not simple
        assert!(!analyzer.has_simple_numeric_loop(&stmts));
    }

    // --- has_large_collections tests ---

    #[test]
    fn test_has_large_collections_list_param() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "process",
            vec![make_param("items", Type::List(Box::new(Type::Int)))],
            Type::Int,
            vec![],
        );
        assert!(analyzer.has_large_collections(&func));
    }

    #[test]
    fn test_has_large_collections_dict_param() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "lookup",
            vec![make_param(
                "data",
                Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            )],
            Type::Int,
            vec![],
        );
        assert!(analyzer.has_large_collections(&func));
    }

    #[test]
    fn test_has_large_collections_no_collections() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "add",
            vec![make_param("a", Type::Int), make_param("b", Type::Int)],
            Type::Int,
            vec![],
        );
        assert!(!analyzer.has_large_collections(&func));
    }

    // --- has_modification_patterns tests ---

    #[test]
    fn test_has_modification_patterns_append() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_extend() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "extend".to_string(),
            args: vec![HirExpr::Var("other".to_string())],
            kwargs: vec![],
        })];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_insert() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "insert".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(0)),
                HirExpr::Literal(Literal::Int(1)),
            ],
            kwargs: vec![],
        })];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_remove() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "remove".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(5))],
            kwargs: vec![],
        })];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_pop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "pop".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_clear() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "clear".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_no_modification() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        })];
        assert!(!analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_in_if() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "append".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            })],
            else_body: None,
        }];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_modification_patterns_in_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "append".to_string(),
                args: vec![HirExpr::Var("i".to_string())],
                kwargs: vec![],
            })],
        }];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    // --- has_string_operations tests ---

    #[test]
    fn test_has_string_operations_string_param() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "greet",
            vec![make_param("name", Type::String)],
            Type::String,
            vec![],
        );
        assert!(analyzer.has_string_operations(&func));
    }

    #[test]
    fn test_has_string_operations_string_return() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func("get_name", vec![], Type::String, vec![]);
        assert!(analyzer.has_string_operations(&func));
    }

    #[test]
    fn test_has_string_operations_no_strings() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "add",
            vec![make_param("a", Type::Int), make_param("b", Type::Int)],
            Type::Int,
            vec![],
        );
        assert!(!analyzer.has_string_operations(&func));
    }

    // --- has_string_concatenation tests ---

    #[test]
    fn test_has_string_concatenation_in_assign() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![make_assign(
            "s",
            HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::String("hello ".to_string()))),
                right: Box::new(HirExpr::Var("name".to_string())),
            },
        )];
        assert!(analyzer.has_string_concatenation(&stmts));
    }

    #[test]
    fn test_has_string_concatenation_in_return() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Literal(Literal::String(" world".to_string()))),
        }))];
        assert!(analyzer.has_string_concatenation(&stmts));
    }

    #[test]
    fn test_has_string_concatenation_numeric_add() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![make_assign(
            "x",
            HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
        )];
        // This is numeric add, not string concat
        assert!(!analyzer.has_string_concatenation(&stmts));
    }

    // --- has_index_access tests ---

    #[test]
    fn test_has_index_access_in_assign() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![make_assign(
            "x",
            HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
        )];
        assert!(analyzer.has_index_access(&stmts));
    }

    #[test]
    fn test_has_index_access_in_return() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Return(Some(HirExpr::Index {
            base: Box::new(HirExpr::Var("data".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        }))];
        assert!(analyzer.has_index_access(&stmts));
    }

    #[test]
    fn test_has_index_access_in_expr() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Index {
            base: Box::new(HirExpr::Var("items".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(5))),
        })];
        assert!(analyzer.has_index_access(&stmts));
    }

    #[test]
    fn test_has_index_access_in_if_condition() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Index {
                base: Box::new(HirExpr::Var("flags".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Pass],
            else_body: None,
        }];
        assert!(analyzer.has_index_access(&stmts));
    }

    #[test]
    fn test_has_index_access_in_loop_body() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("indices".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Var("i".to_string())),
            })],
        }];
        assert!(analyzer.has_index_access(&stmts));
    }

    #[test]
    fn test_has_index_access_no_index() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![make_assign("x", HirExpr::Var("y".to_string()))];
        assert!(!analyzer.has_index_access(&stmts));
    }

    // --- calculate_complexity tests ---

    #[test]
    fn test_calculate_complexity_empty() {
        let analyzer = AnnotationAnalyzer::new();
        assert_eq!(analyzer.calculate_complexity(&[]), 0);
    }

    #[test]
    fn test_calculate_complexity_simple_statements() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![
            make_assign("x", HirExpr::Literal(Literal::Int(1))),
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ];
        assert_eq!(analyzer.calculate_complexity(&stmts), 2);
    }

    #[test]
    fn test_calculate_complexity_if_no_else() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Pass],
            else_body: None,
        }];
        // 1 for the if + 1 for the body
        assert_eq!(analyzer.calculate_complexity(&stmts), 2);
    }

    #[test]
    fn test_calculate_complexity_if_else() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Pass],
            else_body: Some(vec![HirStmt::Pass]),
        }];
        // 1 for the if + 1 for then + 1 for else
        assert_eq!(analyzer.calculate_complexity(&stmts), 3);
    }

    #[test]
    fn test_calculate_complexity_for_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Pass],
        }];
        // 3 for the for + 1 for the body
        assert_eq!(analyzer.calculate_complexity(&stmts), 4);
    }

    #[test]
    fn test_calculate_complexity_while_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break { label: None }],
        }];
        // 3 for the while + 1 for the body
        assert_eq!(analyzer.calculate_complexity(&stmts), 4);
    }

    #[test]
    fn test_calculate_complexity_nested_loops() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("a".to_string()),
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("j".to_string()),
                iter: HirExpr::Var("b".to_string()),
                body: vec![HirStmt::Pass],
            }],
        }];
        // Outer: 3 + inner (3 + 1) = 7
        assert_eq!(analyzer.calculate_complexity(&stmts), 7);
    }

    #[test]
    fn test_calculate_complexity_nested_if_in_loop() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::If {
                condition: HirExpr::Var("cond".to_string()),
                then_body: vec![HirStmt::Pass],
                else_body: Some(vec![HirStmt::Pass]),
            }],
        }];
        // 3 for loop + 1 for if + 1 for then + 1 for else = 6
        assert_eq!(analyzer.calculate_complexity(&stmts), 6);
    }

    // --- find_function_line tests ---

    #[test]
    fn test_find_function_line_first() {
        let analyzer = AnnotationAnalyzer::new();
        let source = "def foo():\n    pass";
        assert_eq!(analyzer.find_function_line(source, "foo"), 1);
    }

    #[test]
    fn test_find_function_line_middle() {
        let analyzer = AnnotationAnalyzer::new();
        let source = "# comment\n\ndef bar(x):\n    return x";
        assert_eq!(analyzer.find_function_line(source, "bar"), 3);
    }

    #[test]
    fn test_find_function_line_not_found() {
        let analyzer = AnnotationAnalyzer::new();
        let source = "def foo():\n    pass";
        assert_eq!(analyzer.find_function_line(source, "baz"), 0);
    }

    #[test]
    fn test_find_function_line_multiple_functions() {
        let analyzer = AnnotationAnalyzer::new();
        let source = "def first():\n    pass\n\ndef second():\n    pass\n\ndef third():\n    pass";
        assert_eq!(analyzer.find_function_line(source, "first"), 1);
        assert_eq!(analyzer.find_function_line(source, "second"), 4);
        assert_eq!(analyzer.find_function_line(source, "third"), 7);
    }

    // --- is_string_expr tests ---

    #[test]
    fn test_is_string_expr_literal() {
        let analyzer = AnnotationAnalyzer::new();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(analyzer.is_string_expr(&expr));
    }

    #[test]
    fn test_is_string_expr_var() {
        let analyzer = AnnotationAnalyzer::new();
        let expr = HirExpr::Var("name".to_string());
        // Variables might be strings
        assert!(analyzer.is_string_expr(&expr));
    }

    #[test]
    fn test_is_string_expr_int_literal() {
        let analyzer = AnnotationAnalyzer::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!analyzer.is_string_expr(&expr));
    }

    #[test]
    fn test_is_string_expr_call() {
        let analyzer = AnnotationAnalyzer::new();
        let expr = HirExpr::Call {
            func: "str".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        assert!(!analyzer.is_string_expr(&expr));
    }

    // --- has_index_expr tests ---

    #[test]
    fn test_has_index_expr_true() {
        let analyzer = AnnotationAnalyzer::new();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(analyzer.has_index_expr(&expr));
    }

    #[test]
    fn test_has_index_expr_false() {
        let analyzer = AnnotationAnalyzer::new();
        let expr = HirExpr::Var("x".to_string());
        assert!(!analyzer.has_index_expr(&expr));
    }

    // --- has_shared_state tests ---

    #[test]
    fn test_has_shared_state_always_false() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func("test", vec![], Type::None, vec![]);
        // Current implementation always returns false
        assert!(!analyzer.has_shared_state(&func));
    }

    // --- is_collection_modified tests ---

    #[test]
    fn test_is_collection_modified_true() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "modify",
            vec![make_param("items", Type::List(Box::new(Type::Int)))],
            Type::None,
            vec![HirStmt::Expr(HirExpr::Call {
                func: "append".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            })],
        );
        assert!(analyzer.is_collection_modified(&func));
    }

    #[test]
    fn test_is_collection_modified_false() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "read_only",
            vec![make_param("items", Type::List(Box::new(Type::Int)))],
            Type::Int,
            vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("items".to_string())],
                kwargs: vec![],
            }))],
        );
        assert!(!analyzer.is_collection_modified(&func));
    }

    // --- only_reads_strings tests ---

    #[test]
    fn test_only_reads_strings_true() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "read",
            vec![make_param("s", Type::String)],
            Type::Int,
            vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("s".to_string())],
                kwargs: vec![],
            }))],
        );
        assert!(analyzer.only_reads_strings(&func));
    }

    #[test]
    fn test_only_reads_strings_false_concat() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "concat",
            vec![make_param("a", Type::String), make_param("b", Type::String)],
            Type::String,
            vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
        );
        assert!(!analyzer.only_reads_strings(&func));
    }

    // --- has_array_access tests ---

    #[test]
    fn test_has_array_access_true() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "get_first",
            vec![make_param("arr", Type::List(Box::new(Type::Int)))],
            Type::Int,
            vec![HirStmt::Return(Some(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            }))],
        );
        assert!(analyzer.has_array_access(&func));
    }

    #[test]
    fn test_has_array_access_false() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "sum",
            vec![make_param("arr", Type::List(Box::new(Type::Int)))],
            Type::Int,
            vec![HirStmt::Return(Some(HirExpr::Call {
                func: "sum".to_string(),
                args: vec![HirExpr::Var("arr".to_string())],
                kwargs: vec![],
            }))],
        );
        assert!(!analyzer.has_array_access(&func));
    }

    // --- has_frequent_lookups tests ---

    #[test]
    fn test_has_frequent_lookups_true() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "search",
            vec![make_param(
                "d",
                Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            )],
            Type::Int,
            vec![HirStmt::For {
                target: AssignTarget::Symbol("key".to_string()),
                iter: HirExpr::Var("keys".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Index {
                    base: Box::new(HirExpr::Var("d".to_string())),
                    index: Box::new(HirExpr::Var("key".to_string())),
                })],
            }],
        );
        assert!(analyzer.has_frequent_lookups(&func));
    }

    #[test]
    fn test_has_frequent_lookups_false() {
        let analyzer = AnnotationAnalyzer::new();
        let func = make_func(
            "simple",
            vec![make_param(
                "d",
                Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            )],
            Type::Int,
            vec![HirStmt::Return(Some(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            }))],
        );
        // Single lookup, not in loop
        assert!(!analyzer.has_frequent_lookups(&func));
    }

    // --- Additional edge case tests ---

    #[test]
    fn test_default_trait() {
        let analyzer = AnnotationAnalyzer;
        assert!(!analyzer.has_loops(&[]));
    }

    #[test]
    fn test_has_dict_access_same_as_index() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Index {
            base: Box::new(HirExpr::Var("dict".to_string())),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        })];
        assert_eq!(
            analyzer.has_dict_access(&stmts),
            analyzer.has_index_access(&stmts)
        );
    }

    #[test]
    fn test_has_modification_patterns_in_else() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(false)),
            then_body: vec![HirStmt::Pass],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Call {
                func: "pop".to_string(),
                args: vec![],
                kwargs: vec![],
            })]),
        }];
        assert!(analyzer.has_modification_patterns(&stmts));
    }

    #[test]
    fn test_has_loops_nested_if() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::For {
                    target: AssignTarget::Symbol("i".to_string()),
                    iter: HirExpr::Var("x".to_string()),
                    body: vec![HirStmt::Pass],
                }],
                else_body: None,
            }],
            else_body: None,
        }];
        assert!(analyzer.has_loops(&stmts));
    }

    #[test]
    fn test_calculate_complexity_mixed() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![
            make_assign("x", HirExpr::Literal(Literal::Int(0))),
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![
                    HirStmt::If {
                        condition: HirExpr::Var("cond".to_string()),
                        then_body: vec![HirStmt::Continue { label: None }],
                        else_body: None,
                    },
                    make_assign("x", HirExpr::Var("i".to_string())),
                ],
            },
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ];
        // 1 (assign) + 3 (for) + 1 (if) + 1 (continue) + 1 (assign) + 1 (return) = 8
        assert_eq!(analyzer.calculate_complexity(&stmts), 8);
    }

    #[test]
    fn test_has_index_access_in_else_branch() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(false)),
            then_body: vec![HirStmt::Pass],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            })]),
        }];
        assert!(analyzer.has_index_access(&stmts));
    }

    #[test]
    fn test_has_index_access_in_while_body() {
        let analyzer = AnnotationAnalyzer::new();
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Var("cond".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            })],
        }];
        assert!(analyzer.has_index_access(&stmts));
    }
}
