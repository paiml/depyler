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
    /// # Complexity: 9 (match with 8 recursive arms)
    fn analyze_stmt(
        stmt: &HirStmt,
        analysis: &mut YieldAnalysis,
        state_counter: &mut usize,
        depth: usize,
        stmt_idx: usize,
    ) {
        match stmt {
            HirStmt::Expr(expr) => {
                // Check if this expression contains a yield
                if let Some(yield_expr) = Self::extract_yield_expr(expr) {
                    let yield_point = YieldPoint {
                        state_id: *state_counter,
                        yield_expr,
                        live_vars: Vec::new(), // Will be computed in finalize()
                        depth,
                    };
                    analysis.yield_points.push(yield_point);
                    analysis.resume_points.insert(*state_counter, stmt_idx + 1);
                    *state_counter += 1;
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                // Loops with yields need special handling - increased depth
                for s in body {
                    Self::analyze_stmt(s, analysis, state_counter, depth + 1, stmt_idx);
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
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
            HirStmt::With { body, .. } => {
                for s in body {
                    Self::analyze_stmt(s, analysis, state_counter, depth, stmt_idx);
                }
            }
            _ => {
                // Other statements don't affect control flow for yields
            }
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
        // TODO: Implement liveness analysis to determine which variables
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

    #[test]
    #[allow(non_snake_case)]
    fn test_DEPYLER_0262_simple_yield_detection() {
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
    fn test_DEPYLER_0262_loop_with_yield() {
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
    fn test_DEPYLER_0262_multiple_yields() {
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
}
