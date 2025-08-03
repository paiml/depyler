//! Optimization passes for generated Rust code

use crate::hir::{
    AssignTarget, BinOp, HirExpr, HirFunction, HirProgram, HirStmt, Literal, UnaryOp,
};
use std::collections::HashMap;

/// Main optimizer that runs various optimization passes
pub struct Optimizer {
    /// Configuration for optimization passes
    config: OptimizerConfig,
}

#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Enable inlining of small functions
    pub inline_functions: bool,
    /// Enable dead code elimination
    pub eliminate_dead_code: bool,
    /// Enable constant propagation
    pub propagate_constants: bool,
    /// Enable common subexpression elimination
    pub eliminate_common_subexpressions: bool,
    /// Maximum function size for inlining (in HIR nodes)
    pub inline_threshold: usize,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            inline_functions: true,
            eliminate_dead_code: true,
            propagate_constants: true,
            eliminate_common_subexpressions: true,
            inline_threshold: 20,
        }
    }
}

impl Optimizer {
    pub fn new(config: OptimizerConfig) -> Self {
        Self { config }
    }

    /// Run all optimization passes on a HIR program
    pub fn optimize_program(&mut self, mut program: HirProgram) -> HirProgram {
        // Pass 1: Constant propagation
        if self.config.propagate_constants {
            program = self.propagate_constants_program(program);
        }

        // Pass 2: Dead code elimination
        if self.config.eliminate_dead_code {
            program = self.eliminate_dead_code_program(program);
        }

        // Pass 3: Function inlining
        if self.config.inline_functions {
            program = self.inline_functions_program(program);
        }

        // Pass 4: Common subexpression elimination
        if self.config.eliminate_common_subexpressions {
            program = self.eliminate_common_subexpressions_program(program);
        }

        program
    }

    /// Propagate constant values through the program
    fn propagate_constants_program(&self, mut program: HirProgram) -> HirProgram {
        let mut constants = HashMap::new();

        // First pass: collect constants
        for func in &program.functions {
            self.collect_constants_function(func, &mut constants);
        }

        // Second pass: propagate constants
        for func in &mut program.functions {
            self.propagate_constants_function(func, &constants);
        }

        program
    }

    fn collect_constants_function(
        &self,
        func: &HirFunction,
        constants: &mut HashMap<String, HirExpr>,
    ) {
        for stmt in &func.body {
            self.collect_constants_stmt(stmt, constants);
        }
    }

    fn collect_constants_stmt(&self, stmt: &HirStmt, constants: &mut HashMap<String, HirExpr>) {
        match stmt {
            HirStmt::Assign { target, value } => {
                if let AssignTarget::Symbol(name) = target {
                    if self.is_constant_expr(value) {
                        constants.insert(name.clone(), value.clone());
                    }
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    self.collect_constants_stmt(s, constants);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_constants_stmt(s, constants);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for s in body {
                    self.collect_constants_stmt(s, constants);
                }
            }
            _ => {}
        }
    }

    fn is_constant_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(_) => true,
            HirExpr::Unary { operand, .. } => self.is_constant_expr(operand),
            HirExpr::Binary { left, right, .. } => {
                self.is_constant_expr(left) && self.is_constant_expr(right)
            }
            _ => false,
        }
    }

    fn propagate_constants_function(
        &self,
        func: &mut HirFunction,
        constants: &HashMap<String, HirExpr>,
    ) {
        for stmt in &mut func.body {
            self.propagate_constants_stmt(stmt, constants);
        }
    }

    fn propagate_constants_stmt(&self, stmt: &mut HirStmt, constants: &HashMap<String, HirExpr>) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.propagate_constants_expr(value, constants);
            }
            HirStmt::Return(Some(expr)) => {
                self.propagate_constants_expr(expr, constants);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.propagate_constants_expr(condition, constants);
                for s in then_body {
                    self.propagate_constants_stmt(s, constants);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.propagate_constants_stmt(s, constants);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.propagate_constants_expr(condition, constants);
                for s in body {
                    self.propagate_constants_stmt(s, constants);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.propagate_constants_expr(iter, constants);
                for s in body {
                    self.propagate_constants_stmt(s, constants);
                }
            }
            HirStmt::Expr(expr) => {
                self.propagate_constants_expr(expr, constants);
            }
            _ => {}
        }
    }

    fn propagate_constants_expr(&self, expr: &mut HirExpr, constants: &HashMap<String, HirExpr>) {
        match expr {
            HirExpr::Var(name) => {
                if let Some(const_expr) = constants.get(name) {
                    *expr = const_expr.clone();
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.propagate_constants_expr(left, constants);
                self.propagate_constants_expr(right, constants);

                // Try to evaluate constant expressions
                if let Some(result) = self.evaluate_constant_binop(expr) {
                    *expr = result;
                }
            }
            HirExpr::Unary { operand, .. } => {
                self.propagate_constants_expr(operand, constants);

                // Try to evaluate constant expressions
                if let Some(result) = self.evaluate_constant_unaryop(expr) {
                    *expr = result;
                }
            }
            HirExpr::List(items) => {
                for item in items {
                    self.propagate_constants_expr(item, constants);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.propagate_constants_expr(k, constants);
                    self.propagate_constants_expr(v, constants);
                }
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.propagate_constants_expr(arg, constants);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.propagate_constants_expr(object, constants);
                for arg in args {
                    self.propagate_constants_expr(arg, constants);
                }
            }
            HirExpr::Lambda { body, .. } => {
                self.propagate_constants_expr(body, constants);
            }
            _ => {}
        }
    }

    fn evaluate_constant_binop(&self, expr: &HirExpr) -> Option<HirExpr> {
        if let HirExpr::Binary { left, right, op } = expr {
            match (left.as_ref(), right.as_ref(), op) {
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Add,
                ) => Some(HirExpr::Literal(Literal::Int(a + b))),
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Sub,
                ) => Some(HirExpr::Literal(Literal::Int(a - b))),
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Mul,
                ) => Some(HirExpr::Literal(Literal::Int(a * b))),
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Div,
                ) if *b != 0 => Some(HirExpr::Literal(Literal::Int(a / b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Add,
                ) => Some(HirExpr::Literal(Literal::Float(a + b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Sub,
                ) => Some(HirExpr::Literal(Literal::Float(a - b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Mul,
                ) => Some(HirExpr::Literal(Literal::Float(a * b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Div,
                ) if *b != 0.0 => Some(HirExpr::Literal(Literal::Float(a / b))),
                _ => None,
            }
        } else {
            None
        }
    }

    fn evaluate_constant_unaryop(&self, expr: &HirExpr) -> Option<HirExpr> {
        if let HirExpr::Unary { op, operand } = expr {
            match (operand.as_ref(), op) {
                (HirExpr::Literal(Literal::Int(n)), UnaryOp::Neg) => {
                    Some(HirExpr::Literal(Literal::Int(-n)))
                }
                (HirExpr::Literal(Literal::Float(f)), UnaryOp::Neg) => {
                    Some(HirExpr::Literal(Literal::Float(-f)))
                }
                (HirExpr::Literal(Literal::Bool(b)), UnaryOp::Not) => {
                    Some(HirExpr::Literal(Literal::Bool(!b)))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Eliminate dead code from the program
    fn eliminate_dead_code_program(&self, mut program: HirProgram) -> HirProgram {
        for func in &mut program.functions {
            self.eliminate_dead_code_function(func);
        }
        program
    }

    fn eliminate_dead_code_function(&self, func: &mut HirFunction) {
        // Collect used variables
        let mut used_vars = HashMap::new();
        for stmt in &func.body {
            self.collect_used_vars_stmt(stmt, &mut used_vars);
        }

        // Remove assignments to unused variables
        func.body.retain(|stmt| {
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } = stmt
            {
                used_vars.contains_key(name)
            } else {
                true
            }
        });
    }

    fn collect_used_vars_stmt(&self, stmt: &HirStmt, used: &mut HashMap<String, bool>) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.collect_used_vars_expr(value, used);
            }
            HirStmt::Return(Some(expr)) => {
                self.collect_used_vars_expr(expr, used);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_used_vars_expr(condition, used);
                for s in then_body {
                    self.collect_used_vars_stmt(s, used);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_used_vars_stmt(s, used);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.collect_used_vars_expr(condition, used);
                for s in body {
                    self.collect_used_vars_stmt(s, used);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.collect_used_vars_expr(iter, used);
                for s in body {
                    self.collect_used_vars_stmt(s, used);
                }
            }
            HirStmt::Expr(expr) => {
                self.collect_used_vars_expr(expr, used);
            }
            _ => {}
        }
    }

    fn collect_used_vars_expr(&self, expr: &HirExpr, used: &mut HashMap<String, bool>) {
        match expr {
            HirExpr::Var(name) => {
                used.insert(name.clone(), true);
            }
            HirExpr::Binary { left, right, .. } => {
                self.collect_used_vars_expr(left, used);
                self.collect_used_vars_expr(right, used);
            }
            HirExpr::Unary { operand, .. } => {
                self.collect_used_vars_expr(operand, used);
            }
            HirExpr::List(items) => {
                for item in items {
                    self.collect_used_vars_expr(item, used);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.collect_used_vars_expr(k, used);
                    self.collect_used_vars_expr(v, used);
                }
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.collect_used_vars_expr(arg, used);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.collect_used_vars_expr(object, used);
                for arg in args {
                    self.collect_used_vars_expr(arg, used);
                }
            }
            HirExpr::Lambda { body, .. } => {
                self.collect_used_vars_expr(body, used);
            }
            _ => {}
        }
    }

    /// Inline small functions
    fn inline_functions_program(&self, program: HirProgram) -> HirProgram {
        // TODO: Implement function inlining
        // This is complex and requires careful handling of:
        // - Recursive functions
        // - Functions with side effects
        // - Functions with multiple return points
        program
    }

    /// Eliminate common subexpressions
    fn eliminate_common_subexpressions_program(&self, program: HirProgram) -> HirProgram {
        // TODO: Implement CSE
        // This requires:
        // - Expression hashing/comparison
        // - Temporary variable generation
        // - Side effect analysis
        program
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{
        AssignTarget, BinOp, FunctionProperties, HirExpr, HirFunction, HirProgram, HirStmt, Literal, Type,
    };
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_constant_propagation() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("x".to_string()),
                        value: HirExpr::Literal(Literal::Int(5)),
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("y".to_string()),
                        value: HirExpr::Binary {
                            left: Box::new(HirExpr::Var("x".to_string())),
                            op: BinOp::Add,
                            right: Box::new(HirExpr::Literal(Literal::Int(3))),
                        },
                    },
                    HirStmt::Return(Some(HirExpr::Var("y".to_string()))),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // Check that constant propagation occurred
        let func = &optimized.functions[0];
        if let HirStmt::Assign { value, .. } = &func.body[1] {
            // The expression x + 3 should be optimized to 8
            assert!(matches!(value, HirExpr::Literal(Literal::Int(8))));
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("unused".to_string()),
                        value: HirExpr::Literal(Literal::Int(42)),
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("used".to_string()),
                        value: HirExpr::Literal(Literal::Int(10)),
                    },
                    HirStmt::Return(Some(HirExpr::Var("used".to_string()))),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // Check optimization results
        let func = &optimized.functions[0];
        // After constant propagation and dead code elimination, we should only have the return statement
        // Both assignments should be eliminated since the value 10 is propagated directly to the return
        assert_eq!(func.body.len(), 1, "Expected 1 statement after optimization, got: {:?}", func.body);
        
        // The return should directly contain the literal value
        assert!(matches!(&func.body[0], HirStmt::Return(Some(HirExpr::Literal(Literal::Int(10))))));
    }
}
