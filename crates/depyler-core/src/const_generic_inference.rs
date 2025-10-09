use crate::hir::{
    AssignTarget, ConstGeneric, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type,
};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Analyzes Python code to detect fixed-size array patterns and infer const generics
pub struct ConstGenericInferencer {
    /// Maps variable names to their inferred const values
    const_values: HashMap<String, usize>,
    /// Set of const generic parameters needed for functions
    const_params: HashSet<String>,
}

impl ConstGenericInferencer {
    pub fn new() -> Self {
        Self {
            const_values: HashMap::new(),
            const_params: HashSet::new(),
        }
    }

    /// Analyze a module and infer const generic requirements
    pub fn analyze_module(&mut self, module: &mut HirModule) -> Result<()> {
        for function in &mut module.functions {
            self.analyze_function(function)?;
        }
        Ok(())
    }

    /// Analyze a function and convert fixed-size lists to arrays
    pub fn analyze_function(&mut self, function: &mut HirFunction) -> Result<()> {
        // First pass: detect const values from literals and parameters
        self.collect_const_values(function)?;

        // Second pass: transform types and expressions
        self.transform_function_types(function)?;

        // Third pass: transform function body
        for stmt in &mut function.body {
            self.transform_statement(stmt)?;
        }

        Ok(())
    }

    /// Collect const values from function parameters and literals
    fn collect_const_values(&mut self, function: &HirFunction) -> Result<()> {
        // Look for patterns like: def process_array(arr: List[int], size: int = 10)
        for param in &function.params {
            if let Type::Int = param.ty {
                // If this parameter has a literal default, it might be a const
                // For now, we'll detect const usage in the function body
            }
        }

        // Scan function body for const patterns
        for stmt in &function.body {
            self.scan_statement_for_consts(stmt)?;
        }

        Ok(())
    }

    /// Scan statements to detect const usage patterns
    fn scan_statement_for_consts(&mut self, stmt: &HirStmt) -> Result<()> {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(symbol),
                value,
                ..
            } => self.scan_assign_for_const(symbol, value),
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => self.scan_if_branches(then_body, else_body),
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                self.scan_stmt_block(body)
            }
            _ => Ok(()),
        }
    }

    fn scan_assign_for_const(&mut self, symbol: &str, value: &HirExpr) -> Result<()> {
        if let Some(size) = self.detect_fixed_size_pattern(value) {
            self.const_values.insert(symbol.to_string(), size);
        }
        Ok(())
    }

    fn scan_if_branches(
        &mut self,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) -> Result<()> {
        self.scan_stmt_block(then_body)?;
        if let Some(else_stmts) = else_body {
            self.scan_stmt_block(else_stmts)?;
        }
        Ok(())
    }

    fn scan_stmt_block(&mut self, stmts: &[HirStmt]) -> Result<()> {
        for stmt in stmts {
            self.scan_statement_for_consts(stmt)?;
        }
        Ok(())
    }

    /// Detect patterns that indicate fixed-size arrays
    fn detect_fixed_size_pattern(&self, expr: &HirExpr) -> Option<usize> {
        match expr {
            HirExpr::Binary {
                op: crate::hir::BinOp::Mul,
                left,
                right,
            } => self.detect_multiply_pattern(left, right),
            HirExpr::List(elements) => self.detect_literal_list_size(elements),
            HirExpr::Call { func, args } => self.detect_array_func_call(func, args),
            _ => None,
        }
    }

    fn detect_multiply_pattern(&self, left: &HirExpr, right: &HirExpr) -> Option<usize> {
        self.check_list_times_int(left, right)
            .or_else(|| self.check_list_times_int(right, left))
    }

    fn check_list_times_int(&self, list_side: &HirExpr, int_side: &HirExpr) -> Option<usize> {
        if let (HirExpr::List(elements), HirExpr::Literal(Literal::Int(size))) =
            (list_side, int_side)
        {
            if elements.len() == 1 && *size > 0 {
                return Some(*size as usize);
            }
        }
        None
    }

    fn detect_literal_list_size(&self, elements: &[HirExpr]) -> Option<usize> {
        if !elements.is_empty() && elements.len() < 1000 {
            Some(elements.len())
        } else {
            None
        }
    }

    fn detect_array_func_call(&self, func: &str, args: &[HirExpr]) -> Option<usize> {
        match func {
            "zeros" | "ones" | "full" => {
                if let Some(HirExpr::Literal(Literal::Int(size))) = args.first() {
                    if *size > 0 && *size < 1000 {
                        return Some(*size as usize);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Transform function types to use const generics where appropriate
    fn transform_function_types(&mut self, function: &mut HirFunction) -> Result<()> {
        // First, collect information about parameter sizes
        let mut param_sizes = HashMap::new();
        for param in &function.params {
            if let Type::List(_) = param.ty {
                if let Some(size) = self.infer_const_size_for_param(&param.name, function) {
                    param_sizes.insert(param.name.clone(), size);
                }
            }
        }

        // Then transform parameter types
        for param in &mut function.params {
            if let Type::List(element_type) = &param.ty {
                if let Some(size) = param_sizes.get(&param.name) {
                    param.ty = Type::Array {
                        element_type: element_type.clone(),
                        size: ConstGeneric::Literal(*size),
                    };
                }
            }
        }

        // Transform return type
        if let Type::List(element_type) = &function.ret_type {
            if let Some(size) = self.infer_const_size_for_return(function) {
                function.ret_type = Type::Array {
                    element_type: element_type.clone(),
                    size: ConstGeneric::Literal(size),
                };
            }
        }

        Ok(())
    }

    /// Infer const size for a parameter based on usage
    fn infer_const_size_for_param(
        &self,
        param_name: &str,
        function: &HirFunction,
    ) -> Option<usize> {
        // Look for patterns like len(param) == constant
        // or indexing with known bounds
        for stmt in &function.body {
            if let Some(size) = self.find_const_usage_in_stmt(param_name, stmt) {
                return Some(size);
            }
        }
        None
    }

    /// Infer const size for return type based on return statements
    fn infer_const_size_for_return(&self, function: &HirFunction) -> Option<usize> {
        // First, collect variable assignments
        let mut var_sizes = HashMap::new();
        for stmt in &function.body {
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(symbol),
                value,
                ..
            } = stmt
            {
                if let Some(size) = self.detect_fixed_size_pattern(value) {
                    var_sizes.insert(symbol.clone(), size);
                }
            }
        }

        // Then check return statements
        for stmt in &function.body {
            if let HirStmt::Return(Some(expr)) = stmt {
                // Check if returning a literal pattern
                if let Some(size) = self.detect_fixed_size_pattern(expr) {
                    return Some(size);
                }
                // Check if returning a variable with known size
                if let HirExpr::Var(var_name) = expr {
                    if let Some(size) = var_sizes.get(var_name) {
                        return Some(*size);
                    }
                }
            }
        }
        None
    }

    /// Find const usage patterns in statements
    fn find_const_usage_in_stmt(&self, param_name: &str, stmt: &HirStmt) -> Option<usize> {
        match stmt {
            HirStmt::Assign { value, .. } => self.find_const_usage_in_expr(param_name, value),
            HirStmt::If {
                condition: _,
                then_body,
                else_body,
            } => {
                // Check condition for len(param) == N
                // Recursively check bodies
                for s in then_body {
                    if let Some(size) = self.find_const_usage_in_stmt(param_name, s) {
                        return Some(size);
                    }
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        if let Some(size) = self.find_const_usage_in_stmt(param_name, s) {
                            return Some(size);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Find const usage patterns in expressions
    fn find_const_usage_in_expr(&self, param_name: &str, expr: &HirExpr) -> Option<usize> {
        match expr {
            HirExpr::Binary {
                op: crate::hir::BinOp::Eq,
                left,
                right,
            } => self.find_len_equality_pattern(param_name, left, right),
            HirExpr::Index { base, index } => self.find_index_pattern(param_name, base, index),
            _ => None,
        }
    }

    fn find_len_equality_pattern(
        &self,
        param_name: &str,
        left: &HirExpr,
        right: &HirExpr,
    ) -> Option<usize> {
        self.check_len_eq_side(param_name, left, right)
            .or_else(|| self.check_len_eq_side(param_name, right, left))
    }

    fn check_len_eq_side(
        &self,
        param_name: &str,
        call_side: &HirExpr,
        size_side: &HirExpr,
    ) -> Option<usize> {
        if let (HirExpr::Call { func, args }, HirExpr::Literal(Literal::Int(size))) =
            (call_side, size_side)
        {
            if func == "len" && args.len() == 1 {
                if let HirExpr::Var(var_name) = &args[0] {
                    if var_name == param_name && *size > 0 {
                        return Some(*size as usize);
                    }
                }
            }
        }
        None
    }

    fn find_index_pattern(
        &self,
        param_name: &str,
        base: &HirExpr,
        index: &HirExpr,
    ) -> Option<usize> {
        if let HirExpr::Var(var_name) = base {
            if var_name == param_name {
                if let HirExpr::Literal(Literal::Int(idx)) = index {
                    if *idx >= 0 {
                        return Some((*idx + 1) as usize);
                    }
                }
            }
        }
        None
    }

    /// Transform statements to use array operations
    fn transform_statement(&mut self, stmt: &mut HirStmt) -> Result<()> {
        match stmt {
            HirStmt::Assign { value, .. } => self.transform_expression(value),
            HirStmt::Return(Some(expr)) => self.transform_expression(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.transform_if_stmt(condition, then_body, else_body),
            HirStmt::While { condition, body } => self.transform_while_stmt(condition, body),
            HirStmt::For { iter, body, .. } => self.transform_for_stmt(iter, body),
            _ => Ok(()),
        }
    }

    fn transform_if_stmt(
        &mut self,
        condition: &mut HirExpr,
        then_body: &mut [HirStmt],
        else_body: &mut Option<Vec<HirStmt>>,
    ) -> Result<()> {
        self.transform_expression(condition)?;
        self.transform_stmt_block(then_body)?;
        if let Some(else_stmts) = else_body {
            self.transform_stmt_block(else_stmts)?;
        }
        Ok(())
    }

    fn transform_while_stmt(&mut self, condition: &mut HirExpr, body: &mut [HirStmt]) -> Result<()> {
        self.transform_expression(condition)?;
        self.transform_stmt_block(body)
    }

    fn transform_for_stmt(&mut self, iter: &mut HirExpr, body: &mut [HirStmt]) -> Result<()> {
        self.transform_expression(iter)?;
        self.transform_stmt_block(body)
    }

    fn transform_stmt_block(&mut self, stmts: &mut [HirStmt]) -> Result<()> {
        for stmt in stmts {
            self.transform_statement(stmt)?;
        }
        Ok(())
    }

    /// Transform expressions to use array literals where appropriate
    #[allow(clippy::only_used_in_recursion)]
    fn transform_expression(&mut self, expr: &mut HirExpr) -> Result<()> {
        match expr {
            HirExpr::List(elements) => self.transform_list_expr(elements),
            HirExpr::Binary { left, right, .. } => self.transform_binary_expr(left, right),
            HirExpr::Unary { operand, .. } => self.transform_expression(operand),
            HirExpr::Call { args, .. } => self.transform_call_args(args),
            HirExpr::MethodCall { object, args, .. } => {
                self.transform_method_call(object, args)
            }
            HirExpr::Index { base, index } => self.transform_index_expr(base, index),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => self.transform_slice_expr(base, start, stop, step),
            HirExpr::Dict(pairs) => self.transform_dict_expr(pairs),
            HirExpr::Tuple(elements) => self.transform_tuple_expr(elements),
            HirExpr::Borrow { expr, .. } => self.transform_expression(expr),
            HirExpr::ListComp {
                element,
                iter,
                condition,
                ..
            } => self.transform_list_comp(element, iter, condition),
            _ => Ok(()),
        }
    }

    fn transform_list_expr(&mut self, elements: &mut [HirExpr]) -> Result<()> {
        for elem in elements {
            self.transform_expression(elem)?;
        }
        Ok(())
    }

    fn transform_binary_expr(&mut self, left: &mut HirExpr, right: &mut HirExpr) -> Result<()> {
        self.transform_expression(left)?;
        self.transform_expression(right)
    }

    fn transform_call_args(&mut self, args: &mut [HirExpr]) -> Result<()> {
        for arg in args {
            self.transform_expression(arg)?;
        }
        Ok(())
    }

    fn transform_method_call(
        &mut self,
        object: &mut HirExpr,
        args: &mut [HirExpr],
    ) -> Result<()> {
        self.transform_expression(object)?;
        self.transform_call_args(args)
    }

    fn transform_index_expr(&mut self, base: &mut HirExpr, index: &mut HirExpr) -> Result<()> {
        self.transform_expression(base)?;
        self.transform_expression(index)
    }

    fn transform_slice_expr(
        &mut self,
        base: &mut HirExpr,
        start: &mut Option<Box<HirExpr>>,
        stop: &mut Option<Box<HirExpr>>,
        step: &mut Option<Box<HirExpr>>,
    ) -> Result<()> {
        self.transform_expression(base)?;
        if let Some(start_expr) = start {
            self.transform_expression(start_expr)?;
        }
        if let Some(stop_expr) = stop {
            self.transform_expression(stop_expr)?;
        }
        if let Some(step_expr) = step {
            self.transform_expression(step_expr)?;
        }
        Ok(())
    }

    fn transform_dict_expr(&mut self, pairs: &mut [(HirExpr, HirExpr)]) -> Result<()> {
        for (k, v) in pairs {
            self.transform_expression(k)?;
            self.transform_expression(v)?;
        }
        Ok(())
    }

    fn transform_tuple_expr(&mut self, elements: &mut [HirExpr]) -> Result<()> {
        for elem in elements {
            self.transform_expression(elem)?;
        }
        Ok(())
    }

    fn transform_list_comp(
        &mut self,
        element: &mut HirExpr,
        iter: &mut HirExpr,
        condition: &mut Option<Box<HirExpr>>,
    ) -> Result<()> {
        self.transform_expression(element)?;
        self.transform_expression(iter)?;
        if let Some(cond) = condition {
            self.transform_expression(cond)?;
        }
        Ok(())
    }

    /// Get the set of const generic parameters needed for code generation
    pub fn get_const_params(&self) -> &HashSet<String> {
        &self.const_params
    }

    /// Check if a type should be converted to an array
    pub fn should_convert_to_array(&self, _list_type: &Type) -> Option<(Type, ConstGeneric)> {
        // This would be called during code generation to determine
        // if a List<T> should become [T; N]
        None // Implementation depends on usage analysis
    }
}

impl Default for ConstGenericInferencer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, FunctionProperties, HirExpr, HirFunction, HirParam, HirStmt};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_detect_fixed_size_list() {
        let inferencer = ConstGenericInferencer::new();

        // Test [1, 2, 3] pattern
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        assert_eq!(inferencer.detect_fixed_size_pattern(&expr), Some(3));
    }

    #[test]
    fn test_detect_multiply_pattern() {
        let inferencer = ConstGenericInferencer::new();

        // Test [0] * 5 pattern
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::List(vec![HirExpr::Literal(Literal::Int(0))])),
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        };

        assert_eq!(inferencer.detect_fixed_size_pattern(&expr), Some(5));
    }

    #[test]
    fn test_detect_zeros_call() {
        let inferencer = ConstGenericInferencer::new();

        // Test zeros(10) pattern
        let expr = HirExpr::Call {
            func: "zeros".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
        };

        assert_eq!(inferencer.detect_fixed_size_pattern(&expr), Some(10));
    }

    #[test]
    fn test_function_analysis() {
        let mut inferencer = ConstGenericInferencer::new();

        let mut function = HirFunction {
            name: "process_array".to_string(),
            params: smallvec![HirParam::new("arr".to_string(), Type::List(Box::new(Type::Int)))],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("result".to_string()),
                    value: HirExpr::List(vec![
                        HirExpr::Literal(Literal::Int(0)),
                        HirExpr::Literal(Literal::Int(1)),
                        HirExpr::Literal(Literal::Int(2)),
                    ]),
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        inferencer.analyze_function(&mut function).unwrap();

        // Should detect size 3 for the return type
        assert!(matches!(function.ret_type, Type::Array { .. }));
    }

    #[test]
    fn test_len_equality_detection() {
        let inferencer = ConstGenericInferencer::new();

        // Test len(arr) == 5
        let expr = HirExpr::Binary {
            op: BinOp::Eq,
            left: Box::new(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("arr".to_string())],
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        };

        assert_eq!(inferencer.find_const_usage_in_expr("arr", &expr), Some(5));
    }

    #[test]
    fn test_index_access_detection() {
        let inferencer = ConstGenericInferencer::new();

        // Test arr[4] (implies size >= 5)
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(4))),
        };

        assert_eq!(inferencer.find_const_usage_in_expr("arr", &expr), Some(5));
    }
}
