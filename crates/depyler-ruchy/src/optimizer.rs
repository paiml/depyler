//! Optimization passes for Ruchy code generation

use crate::ast::{BinaryOp, Literal, Param, PipelineStage, RuchyExpr};
use depyler_core::hir::HirModule;
use std::collections::{HashMap, HashSet};

/// Ruchy code optimizer
pub struct RuchyOptimizer {
    /// Optimization level (0-3)
    level: u8,
    
    /// Enable pipeline fusion
    enable_pipeline_fusion: bool,
    
    /// Enable dead code elimination
    enable_dce: bool,
    
    /// Enable common subexpression elimination
    enable_cse: bool,
    
    /// Enable function inlining
    enable_inlining: bool,
}

impl RuchyOptimizer {
    /// Creates a new optimizer with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 2,
            enable_pipeline_fusion: true,
            enable_dce: true,
            enable_cse: true,
            enable_inlining: true,
        }
    }
    
    /// Creates optimizer with custom configuration
    #[must_use]
    pub fn with_config(config: &crate::RuchyConfig) -> Self {
        Self {
            level: config.optimization_level,
            enable_pipeline_fusion: config.optimization_level >= 1,
            enable_dce: config.optimization_level >= 1,
            enable_cse: config.optimization_level >= 2,
            enable_inlining: config.optimization_level >= 2,
        }
    }
    
    /// Optimize a Ruchy AST
    pub fn optimize(&self, expr: RuchyExpr) -> anyhow::Result<RuchyExpr> {
        let mut optimized = expr;
        
        if self.level == 0 {
            return Ok(optimized);
        }
        
        // Apply optimization passes in order
        if self.enable_pipeline_fusion {
            optimized = self.fuse_pipelines(optimized);
        }
        
        if self.enable_cse {
            optimized = self.eliminate_common_subexpressions(optimized);
        }
        
        if self.enable_inlining {
            optimized = self.inline_simple_functions(optimized);
        }
        
        if self.enable_dce {
            optimized = self.eliminate_dead_code(optimized);
        }
        
        // Constant folding is always enabled at level 1+
        optimized = self.fold_constants(optimized);
        
        Ok(optimized)
    }
    
    /// Optimize HIR before transpilation
    pub fn optimize_hir(&self, hir: HirModule) -> HirModule {
        // HIR-level optimizations
        // For now, return as-is
        hir
    }
    
    /// Fuse adjacent pipeline operations
    fn fuse_pipelines(&self, expr: RuchyExpr) -> RuchyExpr {
        self.transform_expr(expr, &|e| match e {
            RuchyExpr::Pipeline { expr, stages } => {
                let fused_stages = self.fuse_pipeline_stages(stages);
                RuchyExpr::Pipeline { expr, stages: fused_stages }
            }
            _ => e,
        })
    }
    
    /// Fuse compatible pipeline stages
    fn fuse_pipeline_stages(&self, stages: Vec<PipelineStage>) -> Vec<PipelineStage> {
        let mut fused = Vec::new();
        let mut i = 0;
        
        while i < stages.len() {
            if i + 1 < stages.len() {
                // Try to fuse current and next stage
                if let Some(merged) = self.try_fuse_stages(&stages[i], &stages[i + 1]) {
                    fused.push(merged);
                    i += 2;
                    continue;
                }
            }
            
            fused.push(stages[i].clone());
            i += 1;
        }
        
        fused
    }
    
    /// Try to fuse two pipeline stages
    fn try_fuse_stages(&self, first: &PipelineStage, second: &PipelineStage) -> Option<PipelineStage> {
        match (first, second) {
            // Fuse consecutive maps: map(f) |> map(g) => map(|x| g(f(x)))
            (PipelineStage::Map(f), PipelineStage::Map(g)) => {
                Some(PipelineStage::Map(Box::new(self.compose_functions(f, g))))
            }
            
            // Fuse consecutive filters: filter(p) |> filter(q) => filter(|x| p(x) && q(x))
            (PipelineStage::Filter(p), PipelineStage::Filter(q)) => {
                Some(PipelineStage::Filter(Box::new(self.and_predicates(p, q))))
            }
            
            // Fuse filter then map into flat_map with conditional
            (PipelineStage::Filter(p), PipelineStage::Map(f)) => {
                Some(PipelineStage::FlatMap(Box::new(self.filter_map_fusion(p, f))))
            }
            
            _ => None,
        }
    }
    
    /// Compose two functions
    fn compose_functions(&self, f: &RuchyExpr, g: &RuchyExpr) -> RuchyExpr {
        // Create: |x| g(f(x))
        let x_param = Param {
            name: "x".to_string(),
            typ: None,
            default: None,
        };
        
        let f_call = match f {
            RuchyExpr::Lambda { params, body } if params.len() == 1 => {
                // Substitute x for the parameter in f's body
                self.substitute_var(&params[0].name, &RuchyExpr::Identifier("x".to_string()), body)
            }
            _ => RuchyExpr::Call {
                func: Box::new(f.clone()),
                args: vec![RuchyExpr::Identifier("x".to_string())],
            },
        };
        
        let g_call = match g {
            RuchyExpr::Lambda { params, body } if params.len() == 1 => {
                // Substitute f(x) for the parameter in g's body
                self.substitute_var(&params[0].name, &f_call, body)
            }
            _ => RuchyExpr::Call {
                func: Box::new(g.clone()),
                args: vec![f_call],
            },
        };
        
        RuchyExpr::Lambda {
            params: vec![x_param],
            body: Box::new(g_call),
        }
    }
    
    /// Combine two predicates with AND
    fn and_predicates(&self, p: &RuchyExpr, q: &RuchyExpr) -> RuchyExpr {
        let x_param = Param {
            name: "x".to_string(),
            typ: None,
            default: None,
        };
        
        let p_call = self.apply_lambda_or_call(p, "x");
        let q_call = self.apply_lambda_or_call(q, "x");
        
        RuchyExpr::Lambda {
            params: vec![x_param],
            body: Box::new(RuchyExpr::Binary {
                left: Box::new(p_call),
                op: BinaryOp::And,
                right: Box::new(q_call),
            }),
        }
    }
    
    /// Fuse filter and map into flat_map
    fn filter_map_fusion(&self, predicate: &RuchyExpr, mapper: &RuchyExpr) -> RuchyExpr {
        let x_param = Param {
            name: "x".to_string(),
            typ: None,
            default: None,
        };
        
        let pred_call = self.apply_lambda_or_call(predicate, "x");
        let map_call = self.apply_lambda_or_call(mapper, "x");
        
        RuchyExpr::Lambda {
            params: vec![x_param],
            body: Box::new(RuchyExpr::If {
                condition: Box::new(pred_call),
                then_branch: Box::new(RuchyExpr::List(vec![map_call])),
                else_branch: Some(Box::new(RuchyExpr::List(vec![]))),
            }),
        }
    }
    
    /// Apply lambda or create function call
    fn apply_lambda_or_call(&self, func: &RuchyExpr, arg_name: &str) -> RuchyExpr {
        match func {
            RuchyExpr::Lambda { params, body } if params.len() == 1 => {
                self.substitute_var(&params[0].name, &RuchyExpr::Identifier(arg_name.to_string()), body)
            }
            _ => RuchyExpr::Call {
                func: Box::new(func.clone()),
                args: vec![RuchyExpr::Identifier(arg_name.to_string())],
            },
        }
    }
    
    /// Substitute a variable in an expression
    #[allow(clippy::only_used_in_recursion)]
    fn substitute_var(&self, var: &str, replacement: &RuchyExpr, expr: &RuchyExpr) -> RuchyExpr {
        match expr {
            RuchyExpr::Identifier(name) if name == var => replacement.clone(),
            
            RuchyExpr::Binary { left, op, right } => RuchyExpr::Binary {
                left: Box::new(self.substitute_var(var, replacement, left)),
                op: *op,
                right: Box::new(self.substitute_var(var, replacement, right)),
            },
            
            RuchyExpr::Call { func, args } => RuchyExpr::Call {
                func: Box::new(self.substitute_var(var, replacement, func)),
                args: args.iter()
                    .map(|arg| self.substitute_var(var, replacement, arg))
                    .collect(),
            },
            
            _ => expr.clone(),
        }
    }
    
    /// Eliminate common subexpressions
    fn eliminate_common_subexpressions(&self, expr: RuchyExpr) -> RuchyExpr {
        let mut cse_map = HashMap::new();
        let mut next_temp_id = 0;
        
        self.cse_transform(expr, &mut cse_map, &mut next_temp_id)
    }
    
    /// CSE transformation helper
    fn cse_transform(
        &self,
        expr: RuchyExpr,
        cse_map: &mut HashMap<String, String>,
        next_id: &mut usize,
    ) -> RuchyExpr {
        // Hash the expression
        let expr_hash = self.hash_expr(&expr);
        
        // Check if we've seen this expression before
        if let Some(temp_var) = cse_map.get(&expr_hash) {
            return RuchyExpr::Identifier(temp_var.clone());
        }
        
        // Process the expression
        match expr {
            RuchyExpr::Binary { left, op, right } if self.is_pure_expr(&left) && self.is_pure_expr(&right) => {
                let left_transformed = self.cse_transform(*left, cse_map, next_id);
                let right_transformed = self.cse_transform(*right, cse_map, next_id);
                
                let new_expr = RuchyExpr::Binary {
                    left: Box::new(left_transformed),
                    op,
                    right: Box::new(right_transformed),
                };
                
                // If this is a complex expression, create a temporary
                if self.is_complex_expr(&new_expr) {
                    let temp_var = format!("_cse_{}", next_id);
                    *next_id += 1;
                    cse_map.insert(self.hash_expr(&new_expr), temp_var.clone());
                    RuchyExpr::Identifier(temp_var)
                } else {
                    new_expr
                }
            }
            
            RuchyExpr::Block(exprs) => {
                let transformed = exprs.into_iter()
                    .map(|e| self.cse_transform(e, cse_map, next_id))
                    .collect();
                RuchyExpr::Block(transformed)
            }
            
            _ => expr,
        }
    }
    
    /// Hash an expression for CSE
    fn hash_expr(&self, expr: &RuchyExpr) -> String {
        // Simple string representation for hashing
        format!("{:?}", expr)
    }
    
    /// Check if expression is pure (no side effects)
    #[allow(clippy::only_used_in_recursion)]
    fn is_pure_expr(&self, expr: &RuchyExpr) -> bool {
        match expr {
            RuchyExpr::Literal(_) | RuchyExpr::Identifier(_) => true,
            RuchyExpr::Binary { left, right, .. } => {
                self.is_pure_expr(left) && self.is_pure_expr(right)
            }
            RuchyExpr::Unary { operand, .. } => self.is_pure_expr(operand),
            _ => false,
        }
    }
    
    /// Check if expression is complex enough to CSE
    fn is_complex_expr(&self, expr: &RuchyExpr) -> bool {
        matches!(expr, RuchyExpr::Binary { .. } | RuchyExpr::Call { .. })
    }
    
    /// Inline simple functions
    fn inline_simple_functions(&self, expr: RuchyExpr) -> RuchyExpr {
        let mut inline_map = HashMap::new();
        
        // First pass: collect inline candidates
        self.collect_inline_candidates(&expr, &mut inline_map);
        
        // Second pass: perform inlining
        self.perform_inlining(expr, &inline_map)
    }
    
    /// Collect functions that can be inlined
    fn collect_inline_candidates(&self, expr: &RuchyExpr, inline_map: &mut HashMap<String, RuchyExpr>) {
        match expr {
            RuchyExpr::Function { name, params, body, .. } => {
                if self.should_inline(params, body) {
                    inline_map.insert(name.clone(), (**body).clone());
                }
            }
            
            RuchyExpr::Block(exprs) => {
                for e in exprs {
                    self.collect_inline_candidates(e, inline_map);
                }
            }
            
            _ => {}
        }
    }
    
    /// Check if function should be inlined
    fn should_inline(&self, params: &[Param], body: &RuchyExpr) -> bool {
        // Inline if:
        // - No parameters or single parameter
        // - Body is simple (not recursive, small size)
        params.len() <= 1 && self.expr_size(body) < 10
    }
    
    /// Calculate expression size for inlining heuristic
    #[allow(clippy::only_used_in_recursion)]
    fn expr_size(&self, expr: &RuchyExpr) -> usize {
        match expr {
            RuchyExpr::Literal(_) | RuchyExpr::Identifier(_) => 1,
            RuchyExpr::Binary { left, right, .. } => {
                1 + self.expr_size(left) + self.expr_size(right)
            }
            RuchyExpr::Call { args, .. } => 2 + args.iter().map(|a| self.expr_size(a)).sum::<usize>(),
            RuchyExpr::Block(exprs) => exprs.iter().map(|e| self.expr_size(e)).sum(),
            _ => 5, // Default weight for complex expressions
        }
    }
    
    /// Perform function inlining
    fn perform_inlining(&self, expr: RuchyExpr, inline_map: &HashMap<String, RuchyExpr>) -> RuchyExpr {
        self.transform_expr(expr, &|e| match e {
            RuchyExpr::Call { func, args } => {
                if let RuchyExpr::Identifier(name) = func.as_ref() {
                    if let Some(body) = inline_map.get(name) {
                        // Inline the function body
                        return body.clone();
                    }
                }
                RuchyExpr::Call { func, args }
            }
            _ => e,
        })
    }
    
    /// Eliminate dead code
    fn eliminate_dead_code(&self, expr: RuchyExpr) -> RuchyExpr {
        let mut used_vars = HashSet::new();
        
        // First pass: collect used variables
        self.collect_used_vars(&expr, &mut used_vars);
        
        // Second pass: remove unused definitions
        self.remove_unused_defs(expr, &used_vars)
    }
    
    /// Collect used variables
    #[allow(clippy::only_used_in_recursion)]
    fn collect_used_vars(&self, expr: &RuchyExpr, used: &mut HashSet<String>) {
        match expr {
            RuchyExpr::Identifier(name) => {
                used.insert(name.clone());
            }
            
            RuchyExpr::Binary { left, right, .. } => {
                self.collect_used_vars(left, used);
                self.collect_used_vars(right, used);
            }
            
            RuchyExpr::Call { func, args } => {
                self.collect_used_vars(func, used);
                for arg in args {
                    self.collect_used_vars(arg, used);
                }
            }
            
            RuchyExpr::Block(exprs) => {
                for e in exprs {
                    self.collect_used_vars(e, used);
                }
            }
            
            _ => {}
        }
    }
    
    /// Remove unused definitions
    fn remove_unused_defs(&self, expr: RuchyExpr, used_vars: &HashSet<String>) -> RuchyExpr {
        match expr {
            RuchyExpr::Let { name, value, body, is_mutable } => {
                if used_vars.contains(&name) {
                    RuchyExpr::Let {
                        name,
                        value,
                        body: Box::new(self.remove_unused_defs(*body, used_vars)),
                        is_mutable,
                    }
                } else {
                    // Skip unused let binding
                    self.remove_unused_defs(*body, used_vars)
                }
            }
            
            RuchyExpr::Block(exprs) => {
                let filtered: Vec<_> = exprs.into_iter()
                    .filter_map(|e| {
                        let transformed = self.remove_unused_defs(e, used_vars);
                        // Remove empty blocks
                        if matches!(transformed, RuchyExpr::Block(ref v) if v.is_empty()) {
                            None
                        } else {
                            Some(transformed)
                        }
                    })
                    .collect();
                    
                RuchyExpr::Block(filtered)
            }
            
            _ => expr,
        }
    }
    
    /// Fold constant expressions
    fn fold_constants(&self, expr: RuchyExpr) -> RuchyExpr {
        self.transform_expr(expr, &|e| match e {
            RuchyExpr::Binary { left, op, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (RuchyExpr::Literal(l1), RuchyExpr::Literal(l2)) => {
                        if let Some(result) = self.fold_binary_op(l1, op, l2) {
                            return RuchyExpr::Literal(result);
                        }
                    }
                    _ => {}
                }
                RuchyExpr::Binary { left, op, right }
            }
            
            RuchyExpr::Unary { op, operand } => {
                if let RuchyExpr::Literal(lit) = operand.as_ref() {
                    if let Some(result) = self.fold_unary_op(op, lit) {
                        return RuchyExpr::Literal(result);
                    }
                }
                RuchyExpr::Unary { op, operand }
            }
            
            _ => e,
        })
    }
    
    /// Fold binary operation on literals
    fn fold_binary_op(&self, left: &Literal, op: BinaryOp, right: &Literal) -> Option<Literal> {
        match (left, right) {
            (Literal::Integer(a), Literal::Integer(b)) => {
                Some(match op {
                    BinaryOp::Add => Literal::Integer(a + b),
                    BinaryOp::Subtract => Literal::Integer(a - b),
                    BinaryOp::Multiply => Literal::Integer(a * b),
                    BinaryOp::Divide if *b != 0 => Literal::Integer(a / b),
                    BinaryOp::Modulo if *b != 0 => Literal::Integer(a % b),
                    BinaryOp::Equal => Literal::Bool(a == b),
                    BinaryOp::NotEqual => Literal::Bool(a != b),
                    BinaryOp::Less => Literal::Bool(a < b),
                    BinaryOp::LessEqual => Literal::Bool(a <= b),
                    BinaryOp::Greater => Literal::Bool(a > b),
                    BinaryOp::GreaterEqual => Literal::Bool(a >= b),
                    _ => return None,
                })
            }
            
            (Literal::Float(a), Literal::Float(b)) => {
                Some(match op {
                    BinaryOp::Add => Literal::Float(a + b),
                    BinaryOp::Subtract => Literal::Float(a - b),
                    BinaryOp::Multiply => Literal::Float(a * b),
                    BinaryOp::Divide if *b != 0.0 => Literal::Float(a / b),
                    _ => return None,
                })
            }
            
            (Literal::Bool(a), Literal::Bool(b)) => {
                Some(match op {
                    BinaryOp::And => Literal::Bool(*a && *b),
                    BinaryOp::Or => Literal::Bool(*a || *b),
                    BinaryOp::Equal => Literal::Bool(a == b),
                    BinaryOp::NotEqual => Literal::Bool(a != b),
                    _ => return None,
                })
            }
            
            _ => None,
        }
    }
    
    /// Fold unary operation on literal
    fn fold_unary_op(&self, op: crate::ast::UnaryOp, lit: &Literal) -> Option<Literal> {
        match (op, lit) {
            (crate::ast::UnaryOp::Negate, Literal::Integer(n)) => Some(Literal::Integer(-n)),
            (crate::ast::UnaryOp::Negate, Literal::Float(f)) => Some(Literal::Float(-f)),
            (crate::ast::UnaryOp::Not, Literal::Bool(b)) => Some(Literal::Bool(!b)),
            _ => None,
        }
    }
    
    /// Transform expression with a function
    fn transform_expr<F>(&self, expr: RuchyExpr, transform_fn: &F) -> RuchyExpr
    where
        F: Fn(RuchyExpr) -> RuchyExpr,
    {
        let transformed = match expr {
            RuchyExpr::Binary { left, op, right } => RuchyExpr::Binary {
                left: Box::new(self.transform_expr(*left, transform_fn)),
                op,
                right: Box::new(self.transform_expr(*right, transform_fn)),
            },
            
            RuchyExpr::Unary { op, operand } => RuchyExpr::Unary {
                op,
                operand: Box::new(self.transform_expr(*operand, transform_fn)),
            },
            
            RuchyExpr::Call { func, args } => RuchyExpr::Call {
                func: Box::new(self.transform_expr(*func, transform_fn)),
                args: args.into_iter()
                    .map(|arg| self.transform_expr(arg, transform_fn))
                    .collect(),
            },
            
            RuchyExpr::Block(exprs) => RuchyExpr::Block(
                exprs.into_iter()
                    .map(|e| self.transform_expr(e, transform_fn))
                    .collect(),
            ),
            
            RuchyExpr::If { condition, then_branch, else_branch } => RuchyExpr::If {
                condition: Box::new(self.transform_expr(*condition, transform_fn)),
                then_branch: Box::new(self.transform_expr(*then_branch, transform_fn)),
                else_branch: else_branch.map(|e| Box::new(self.transform_expr(*e, transform_fn))),
            },
            
            RuchyExpr::Pipeline { expr, stages } => RuchyExpr::Pipeline {
                expr: Box::new(self.transform_expr(*expr, transform_fn)),
                stages, // Stages are already optimized in fuse_pipelines
            },
            
            _ => expr,
        };
        
        transform_fn(transformed)
    }
}

impl Default for RuchyOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_folding() {
        let optimizer = RuchyOptimizer::new();
        
        let expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
            op: BinaryOp::Add,
            right: Box::new(RuchyExpr::Literal(Literal::Integer(3))),
        };
        
        let result = optimizer.fold_constants(expr);
        
        assert_eq!(result, RuchyExpr::Literal(Literal::Integer(5)));
    }
    
    #[test]
    fn test_pipeline_fusion() {
        let optimizer = RuchyOptimizer::new();
        
        // Create a pipeline with two consecutive maps
        let pipeline = RuchyExpr::Pipeline {
            expr: Box::new(RuchyExpr::List(vec![])),
            stages: vec![
                PipelineStage::Map(Box::new(RuchyExpr::Lambda {
                    params: vec![Param { name: "x".to_string(), typ: None, default: None }],
                    body: Box::new(RuchyExpr::Binary {
                        left: Box::new(RuchyExpr::Identifier("x".to_string())),
                        op: BinaryOp::Add,
                        right: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
                    }),
                })),
                PipelineStage::Map(Box::new(RuchyExpr::Lambda {
                    params: vec![Param { name: "y".to_string(), typ: None, default: None }],
                    body: Box::new(RuchyExpr::Binary {
                        left: Box::new(RuchyExpr::Identifier("y".to_string())),
                        op: BinaryOp::Multiply,
                        right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
                    }),
                })),
            ],
        };
        
        let result = optimizer.fuse_pipelines(pipeline);
        
        // Should have fused the two maps into one
        if let RuchyExpr::Pipeline { stages, .. } = result {
            assert_eq!(stages.len(), 1);
        } else {
            panic!("Expected pipeline");
        }
    }
}