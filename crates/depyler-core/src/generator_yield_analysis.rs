//! Generator Yield Point Analysis
//!
//! DEPYLER-0262: This module analyzes generator functions to identify yield points
//! and plan the state machine transformation needed for proper coroutine execution.
//!
//! ## Problem
//! Current implementation executes the entire function body in state 0 and returns
//! at the first yield, never resuming. This causes:
//! - Unreachable code after yields
//! - Only one value yielded per generator
//! - Loops with yields don't iterate
//!
//! ## Solution
//! Transform the function into a resumable state machine where each yield point
//! becomes a state transition. Similar to async/await lowering.

use crate::hir::{HirExpr, HirFunction, HirStmt};
use std::collections::HashMap;

/// Represents a single yield point in the generator
#[derive(Debug, Clone, PartialEq)]
pub struct YieldPoint {
    /// State number for this yield point
    pub state_id: usize,
    /// The expression being yielded
    pub yield_expr: HirExpr,
    /// Variables that must be preserved across this yield
    pub live_vars: Vec<String>,
    /// Nesting depth (for loop handling)
    pub depth: usize,
}

/// Analysis result containing all yield points and transformation metadata
#[derive(Debug, Clone)]
pub struct YieldAnalysis {
    /// All yield points found in the function, in execution order
    pub yield_points: Vec<YieldPoint>,
    /// Variables that need to be in state struct (modified between yields)
    pub state_variables: Vec<String>,
    /// Map from state_id to the next statement after the yield
    pub resume_points: HashMap<usize, usize>,
}

impl YieldAnalysis {
    /// Create a new empty analysis
    pub fn new() -> Self {
        Self {
            yield_points: Vec::new(),
            state_variables: Vec::new(),
            resume_points: HashMap::new(),
        }
    }

    /// Analyze a generator function to find all yield points
    ///
    /// This is the entry point for yield point analysis. It walks the function
    /// body and identifies every yield statement, assigning state numbers.
    ///
    /// # Complexity: 3 (create + analyze + finalize)
    pub fn analyze(func: &HirFunction) -> Self {
        let mut analysis = Self::new();
        let mut state_counter = 1; // State 0 is initialization

        // Walk function body and find all yields
        for (idx, stmt) in func.body.iter().enumerate() {
            Self::analyze_stmt(stmt, &mut analysis, &mut state_counter, 0, idx);
        }

        // Finalize analysis (compute live variables, etc.)
        analysis.finalize();
        analysis
    }

    /// Recursively analyze a statement for yield points
    ///
    /// # Complexity: 6 (delegated to helpers)
    fn analyze_stmt(
        stmt: &HirStmt,
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        depth: usize,
        stmt_idx: usize,
    ) {
        match stmt {
            HirStmt::Expr(expr) => {
                Self::analyze_expr_stmt(expr, analysis, state_counter, depth, stmt_idx);
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                Self::analyze_if_stmt(
                    then_body,
                    else_body,
                    analysis,
                    state_counter,
                    depth,
                    stmt_idx,
                );
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                Self::analyze_loop_stmt(body, analysis, state_counter, depth, stmt_idx);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                Self::analyze_try_stmt(
                    body,
                    handlers,
                    orelse,
                    finalbody,
                    analysis,
                    state_counter,
                    (depth, stmt_idx),
                );
            }
            HirStmt::With { body, .. } => {
                Self::analyze_with_stmt(body, analysis, state_counter, depth, stmt_idx);
            }
            _ => {
                // Other statements don't affect control flow for yields
            }
        }
    }

    /// Analyze expression statement for yield
    ///
    /// # Complexity: 2
    #[inline]
    fn analyze_expr_stmt(
        expr: &HirExpr,
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        depth: usize,
        stmt_idx: usize,
    ) {
        if let Some(yield_expr) = Self::extract_yield_expr(expr) {
            let yield_point = YieldPoint {
                state_id: *state_counter,
                yield_expr,
                live_vars: Vec::new(),
                depth,
            };
            analysis.yield_points.push(yield_point);
            analysis.resume_points.insert(*state_counter, stmt_idx + 1);
            *state_counter += 1;
        }
    }

    /// Analyze if statement branches
    ///
    /// # Complexity: 3
    #[inline]
    fn analyze_if_stmt(
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        depth: usize,
        stmt_idx: usize,
    ) {
        for s in then_body {
            Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
        }
        if let Some(else_stmts) = else_body {
            for s in else_stmts {
                Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
            }
        }
    }

    /// Analyze loop body with increased depth
    ///
    /// # Complexity: 2
    #[inline]
    fn analyze_loop_stmt(
        body: &[HirStmt],
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        depth: usize,
        stmt_idx: usize,
    ) {
        for s in body {
            Self::analyze_stmt(s, analysis, state_counter, depth + 1, stmt_idx);
        }
    }

    /// Analyze try/except/else/finally blocks
    ///
    /// # Complexity: 5
    #[inline]
    fn analyze_try_stmt(
        body: &[HirStmt],
        handlers: &[crate::hir::ExceptHandler],
        orelse: &Option<Vec<HirStmt>>,
        finalbody: &Option<Vec<HirStmt>>,
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        context: (usize, usize), // (depth, stmt_idx)
    ) {
        let (depth, stmt_idx) = context;
        for s in body {
            Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
        }
        for handler in handlers {
            for s in &handler.body {
                Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
            }
        }
        if let Some(else_stmts) = orelse {
            for s in else_stmts {
                Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
            }
        }
        if let Some(finally_stmts) = finalbody {
            for s in finally_stmts {
                Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
            }
        }
    }

    /// Analyze with statement body
    ///
    /// # Complexity: 2
    #[inline]
    fn analyze_with_stmt(
        body: &[HirStmt],
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        depth: usize,
        stmt_idx: usize,
    ) {
        for s in body {
            Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
        }
    }

    /// Extract yield expression if present
    ///
    /// # Complexity: 2 (match + clone)
    fn extract_yield_expr(expr: &HirExpr) -> Option<HirExpr> {
        match expr {
            HirExpr::Yield { value } => value.as_ref().map(|v| *v.clone()),
            _ => None,
        }
    }

    /// Finalize analysis by computing live variables and state variables
    ///
    /// # Complexity: 2 (placeholder for future liveness analysis)
    fn finalize(&mut self) {
        // NOTE: Implement liveness analysis to determine which variables need capturing (tracked in DEPYLER-0424)
        // need to be preserved in the state struct.
        // For now, we'll rely on the existing GeneratorStateInfo analysis.
    }

    /// Check if this function contains any yields
    #[inline]
    pub fn has_yields(&self) -> bool {
        !self.yield_points.is_empty()
    }

    /// Get the number of states needed (including state 0 for initialization)
    #[inline]
    pub fn num_states(&self) -> usize {
        self.yield_points.len() + 1
    }
}

impl Default for YieldAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    // === YieldPoint tests ===

    #[test]
    fn test_yield_point_new() {
        let yp = YieldPoint {
            state_id: 5,
            yield_expr: HirExpr::Literal(Literal::Int(42)),
            live_vars: vec!["x".to_string(), "y".to_string()],
            depth: 2,
        };
        assert_eq!(yp.state_id, 5);
        assert_eq!(yp.live_vars.len(), 2);
        assert_eq!(yp.depth, 2);
    }

    #[test]
    fn test_yield_point_clone() {
        let yp = YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Var("x".to_string()),
            live_vars: vec!["x".to_string()],
            depth: 0,
        };
        let cloned = yp.clone();
        assert_eq!(cloned.state_id, yp.state_id);
        assert_eq!(cloned.depth, yp.depth);
    }

    #[test]
    fn test_yield_point_eq() {
        let yp1 = YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        };
        let yp2 = YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        };
        assert_eq!(yp1, yp2);
    }

    #[test]
    fn test_yield_point_debug() {
        let yp = YieldPoint {
            state_id: 3,
            yield_expr: HirExpr::Literal(Literal::Int(99)),
            live_vars: vec!["z".to_string()],
            depth: 1,
        };
        let debug = format!("{:?}", yp);
        assert!(debug.contains("state_id"));
        assert!(debug.contains("3"));
    }

    // === YieldAnalysis tests ===

    #[test]
    fn test_yield_analysis_new() {
        let analysis = YieldAnalysis::new();
        assert!(analysis.yield_points.is_empty());
        assert!(analysis.state_variables.is_empty());
        assert!(analysis.resume_points.is_empty());
    }

    #[test]
    fn test_yield_analysis_default() {
        let analysis = YieldAnalysis::default();
        assert!(analysis.yield_points.is_empty());
    }

    #[test]
    fn test_yield_analysis_has_yields_empty() {
        let analysis = YieldAnalysis::new();
        assert!(!analysis.has_yields());
    }

    #[test]
    fn test_yield_analysis_has_yields_true() {
        let mut analysis = YieldAnalysis::new();
        analysis.yield_points.push(YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        });
        assert!(analysis.has_yields());
    }

    #[test]
    fn test_yield_analysis_num_states_empty() {
        let analysis = YieldAnalysis::new();
        assert_eq!(analysis.num_states(), 1); // Just state 0
    }

    #[test]
    fn test_yield_analysis_num_states_with_yields() {
        let mut analysis = YieldAnalysis::new();
        for i in 1..=5 {
            analysis.yield_points.push(YieldPoint {
                state_id: i,
                yield_expr: HirExpr::Literal(Literal::Int(i as i64)),
                live_vars: vec![],
                depth: 0,
            });
        }
        assert_eq!(analysis.num_states(), 6); // State 0 + 5 yields
    }

    #[test]
    fn test_yield_analysis_clone() {
        let mut analysis = YieldAnalysis::new();
        analysis.yield_points.push(YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec!["a".to_string()],
            depth: 0,
        });
        analysis.state_variables.push("x".to_string());
        analysis.resume_points.insert(1, 2);

        let cloned = analysis.clone();
        assert_eq!(cloned.yield_points.len(), 1);
        assert_eq!(cloned.state_variables.len(), 1);
        assert_eq!(cloned.resume_points.len(), 1);
    }

    // === analyze function tests ===

    #[test]
    fn test_analyze_empty_function() {
        let func = HirFunction {
            name: "empty".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert!(!analysis.has_yields());
        assert_eq!(analysis.num_states(), 1);
    }

    #[test]
    fn test_analyze_non_generator_function() {
        let func = HirFunction {
            name: "regular".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42)))),
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert!(!analysis.has_yields());
    }

    #[test]
    fn test_analyze_yield_in_if_then() {
        let func = HirFunction {
            name: "if_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("cond".to_string()),
                then_body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                })],
                else_body: None,
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_analyze_yield_in_if_else() {
        let func = HirFunction {
            name: "if_else_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("cond".to_string()),
                then_body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                })],
                else_body: Some(vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
                })]),
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 2);
    }

    #[test]
    fn test_analyze_yield_in_for_loop() {
        let func = HirFunction {
            name: "for_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("range".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Var("i".to_string()))),
                })],
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
        assert_eq!(analysis.yield_points[0].depth, 1); // Inside loop = depth 1
    }

    #[test]
    fn test_analyze_yield_in_nested_loops() {
        let func = HirFunction {
            name: "nested_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::For {
                    target: AssignTarget::Symbol("i".to_string()),
                    iter: HirExpr::Var("items".to_string()),
                    body: vec![HirStmt::Expr(HirExpr::Yield {
                        value: Some(Box::new(HirExpr::Var("i".to_string()))),
                    })],
                }],
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
        assert_eq!(analysis.yield_points[0].depth, 2); // Nested = depth 2
    }

    #[test]
    fn test_analyze_yield_in_try_body() {
        let func = HirFunction {
            name: "try_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Try {
                body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                })],
                handlers: vec![],
                orelse: None,
                finalbody: None,
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_analyze_yield_in_except_handler() {
        let func = HirFunction {
            name: "except_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Try {
                body: vec![],
                handlers: vec![ExceptHandler {
                    exception_type: Some("Exception".to_string()),
                    name: Some("e".to_string()),
                    body: vec![HirStmt::Expr(HirExpr::Yield {
                        value: Some(Box::new(HirExpr::Literal(Literal::Int(99)))),
                    })],
                }],
                orelse: None,
                finalbody: None,
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_analyze_yield_in_finally() {
        let func = HirFunction {
            name: "finally_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Try {
                body: vec![],
                handlers: vec![],
                orelse: None,
                finalbody: Some(vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(0)))),
                })]),
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_analyze_yield_in_with() {
        let func = HirFunction {
            name: "with_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::With {
                context: HirExpr::Var("ctx".to_string()),
                target: Some("f".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Var("f".to_string()))),
                })],
                is_async: false,
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_resume_points_tracking() {
        let func = HirFunction {
            name: "resume".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                }),
                HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
                }),
            ],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.resume_points.len(), 2);
        // Resume after first yield (index 0) should be statement index 1
        assert_eq!(analysis.resume_points.get(&1), Some(&1));
        // Resume after second yield (index 1) should be statement index 2
        assert_eq!(analysis.resume_points.get(&2), Some(&2));
    }

    #[test]
    fn test_yield_none_value() {
        let func = HirFunction {
            name: "yield_none".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Yield { value: None })],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        // yield without value is not extracted (returns None from extract_yield_expr)
        // because extract_yield_expr returns value.as_ref().map(...)
        assert_eq!(analysis.yield_points.len(), 0);
    }

    // === Original tests ===

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0262_simple_yield_detection() {
        // Test: Single yield in function body
        let func = HirFunction {
            name: "simple".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Expr(HirExpr::Yield {
                value: Some(Box::new(HirExpr::Literal(Literal::Int(42)))),
            })],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);

        assert_eq!(analysis.yield_points.len(), 1, "Should find 1 yield");
        assert_eq!(analysis.yield_points[0].state_id, 1);
        assert_eq!(analysis.num_states(), 2); // State 0 + 1 yield
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0262_loop_with_yield() {
        // Test: Yield inside a while loop (the main bug scenario)
        let func = HirFunction {
            name: "counter".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Var("i".to_string()))),
                })],
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);

        assert_eq!(analysis.yield_points.len(), 1);
        assert_eq!(analysis.yield_points[0].depth, 1, "Yield is at depth 1");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0262_multiple_yields() {
        // Test: Multiple sequential yields
        let func = HirFunction {
            name: "multi".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                }),
                HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
                }),
                HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(3)))),
                }),
            ],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);

        assert_eq!(analysis.yield_points.len(), 3);
        assert_eq!(analysis.yield_points[0].state_id, 1);
        assert_eq!(analysis.yield_points[1].state_id, 2);
        assert_eq!(analysis.yield_points[2].state_id, 3);
        assert_eq!(analysis.num_states(), 4); // State 0 + 3 yields
    }

    // === Additional edge case tests ===

    #[test]
    fn test_extract_yield_expr_with_value() {
        let expr = HirExpr::Yield {
            value: Some(Box::new(HirExpr::Literal(Literal::Int(42)))),
        };
        let result = YieldAnalysis::extract_yield_expr(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Int(n))) = result {
            assert_eq!(n, 42);
        } else {
            panic!("Expected Int literal");
        }
    }

    #[test]
    fn test_extract_yield_expr_without_value() {
        let expr = HirExpr::Yield { value: None };
        let result = YieldAnalysis::extract_yield_expr(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_yield_expr_not_yield() {
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = YieldAnalysis::extract_yield_expr(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_analyze_yield_in_try_else() {
        let func = HirFunction {
            name: "try_else_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Try {
                body: vec![],
                handlers: vec![],
                orelse: Some(vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(77)))),
                })]),
                finalbody: None,
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_analyze_all_try_blocks_with_yields() {
        let func = HirFunction {
            name: "try_all_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Try {
                body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                })],
                handlers: vec![ExceptHandler {
                    exception_type: Some("Exception".to_string()),
                    name: None,
                    body: vec![HirStmt::Expr(HirExpr::Yield {
                        value: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
                    })],
                }],
                orelse: Some(vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(3)))),
                })]),
                finalbody: Some(vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(4)))),
                })]),
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 4);
    }

    #[test]
    fn test_yield_analysis_debug() {
        let analysis = YieldAnalysis::new();
        let debug = format!("{:?}", analysis);
        assert!(debug.contains("yield_points"));
        assert!(debug.contains("state_variables"));
        assert!(debug.contains("resume_points"));
    }

    #[test]
    fn test_yield_point_different_state_ids() {
        let yp1 = YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        };
        let yp2 = YieldPoint {
            state_id: 2,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        };
        assert_ne!(yp1, yp2);
    }

    #[test]
    fn test_yield_point_different_depths() {
        let yp1 = YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        };
        let yp2 = YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 1,
        };
        assert_ne!(yp1, yp2);
    }

    #[test]
    fn test_analyze_while_loop() {
        let func = HirFunction {
            name: "while_yield".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::While {
                condition: HirExpr::Var("running".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Yield {
                    value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                })],
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
        assert_eq!(analysis.yield_points[0].depth, 1);
    }

    #[test]
    fn test_analyze_other_statements() {
        // Test that non-yield statements don't affect analysis
        let func = HirFunction {
            name: "mixed".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(1)),
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert!(!analysis.has_yields());
    }

    #[test]
    fn test_finalize_method() {
        let mut analysis = YieldAnalysis::new();
        analysis.yield_points.push(YieldPoint {
            state_id: 1,
            yield_expr: HirExpr::Literal(Literal::Int(1)),
            live_vars: vec![],
            depth: 0,
        });
        analysis.finalize();
        // Currently finalize is a placeholder, just ensure it doesn't crash
        assert_eq!(analysis.yield_points.len(), 1);
    }

    #[test]
    fn test_multiple_handlers_with_yields() {
        let func = HirFunction {
            name: "multi_handler".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Try {
                body: vec![],
                handlers: vec![
                    ExceptHandler {
                        exception_type: Some("ValueError".to_string()),
                        name: None,
                        body: vec![HirStmt::Expr(HirExpr::Yield {
                            value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                        })],
                    },
                    ExceptHandler {
                        exception_type: Some("TypeError".to_string()),
                        name: None,
                        body: vec![HirStmt::Expr(HirExpr::Yield {
                            value: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
                        })],
                    },
                ],
                orelse: None,
                finalbody: None,
            }],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 2);
    }

    #[test]
    fn test_yield_expr_variable() {
        let func = HirFunction {
            name: "yield_var".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Expr(HirExpr::Yield {
                value: Some(Box::new(HirExpr::Var("result".to_string()))),
            })],
            properties: FunctionProperties {
                is_generator: true,
                ..Default::default()
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let analysis = YieldAnalysis::analyze(&func);
        assert_eq!(analysis.yield_points.len(), 1);
        if let HirExpr::Var(name) = &analysis.yield_points[0].yield_expr {
            assert_eq!(name, "result");
        } else {
            panic!("Expected Var expression");
        }
    }
}
