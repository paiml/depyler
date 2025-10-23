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

    fn analyze_assign(
        &mut self,
        target: &crate::hir::AssignTarget,
        value: &HirExpr,
        type_annotation: &Option<Type>,
    ) {
        if let crate::hir::AssignTarget::Symbol(name) = target {
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
    use crate::hir::{FunctionProperties, HirParam};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

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
    fn test_DEPYLER_0258_type_inference_from_literal_values() {
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
                    type_annotation: None,  // ← No annotation, must infer!
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
                            type_annotation: None,  // ← Reassignment, type already known
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
