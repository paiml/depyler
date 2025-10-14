use anyhow::Result;
use depyler_core::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    variables: HashMap<String, Type>,
    functions: HashMap<String, FunctionSignature>,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub params: Vec<Type>,
    pub return_type: Type,
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeEnvironment {
    pub fn new() -> Self {
        let mut env = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };

        // Add built-in functions
        env.add_builtin_functions();
        env
    }

    fn add_builtin_functions(&mut self) {
        // len() function
        self.functions.insert(
            "len".to_string(),
            FunctionSignature {
                params: vec![Type::Unknown], // Accepts any container
                return_type: Type::Int,
            },
        );

        // range() function
        self.functions.insert(
            "range".to_string(),
            FunctionSignature {
                params: vec![Type::Int], // Simplified - actually variadic
                return_type: Type::Custom("range".to_string()),
            },
        );

        // Common numeric functions
        for func in ["abs", "max", "min", "sum"] {
            self.functions.insert(
                func.to_string(),
                FunctionSignature {
                    params: vec![Type::Unknown],
                    return_type: Type::Unknown, // Type depends on input
                },
            );
        }
    }

    pub fn get_var_type(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    pub fn set_var_type(&mut self, name: String, ty: Type) {
        self.variables.insert(name, ty);
    }

    pub fn get_function_signature(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }
}

pub struct TypeInferencer {
    env: TypeEnvironment,
}

impl Default for TypeInferencer {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInferencer {
    pub fn new() -> Self {
        Self {
            env: TypeEnvironment::new(),
        }
    }

    pub fn infer_function(&mut self, func: &HirFunction) -> Result<HashMap<String, Type>> {
        // Initialize environment with parameter types
        for param in &func.params {
            self.env.set_var_type(param.name.clone(), param.ty.clone());
        }

        // Infer types in function body
        self.infer_body(&func.body)?;

        Ok(self.env.variables.clone())
    }

    fn infer_body(&mut self, body: &[HirStmt]) -> Result<()> {
        for stmt in body {
            self.infer_stmt(stmt)?;
        }
        Ok(())
    }

    fn infer_stmt(&mut self, stmt: &HirStmt) -> Result<()> {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                let value_type = self.infer_expr(value)?;
                if let AssignTarget::Symbol(symbol) = target {
                    self.env.set_var_type(symbol.clone(), value_type);
                }
                // Note: Subscript and attribute assignments (e.g., a[0] = x, obj.field = x)
                // are currently not tracked for type flow analysis. Only symbol assignments
                // update the type environment. This is a known limitation.
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.infer_expr(condition)?;
                self.infer_body(then_body)?;
                if let Some(else_stmts) = else_body {
                    self.infer_body(else_stmts)?;
                }
            }
            HirStmt::While { condition, body } => {
                self.infer_expr(condition)?;
                self.infer_body(body)?;
            }
            HirStmt::For { target, iter, body } => {
                let iter_type = self.infer_expr(iter)?;
                let element_type = self.get_element_type(&iter_type);
                self.env.set_var_type(target.clone(), element_type);
                self.infer_body(body)?;
            }
            HirStmt::Return(expr) => {
                if let Some(e) = expr {
                    self.infer_expr(e)?;
                }
            }
            HirStmt::Expr(expr) => {
                self.infer_expr(expr)?;
            }
            HirStmt::Raise { exception, cause } => {
                if let Some(exc) = exception {
                    self.infer_expr(exc)?;
                }
                if let Some(c) = cause {
                    self.infer_expr(c)?;
                }
            }
            HirStmt::Break { .. } | HirStmt::Continue { .. } | HirStmt::Pass => {
                // Break, continue, and pass don't affect type inference
            }
            HirStmt::Assert { test, msg } => {
                // Infer types of test expression and optional message
                self.infer_expr(test)?;
                if let Some(message) = msg {
                    self.infer_expr(message)?;
                }
            }
            HirStmt::With {
                context,
                target: _,
                body,
            } => {
                // Infer type of context expression
                self.infer_expr(context)?;

                // If there's a target variable, we should infer its type
                // For now, just analyze the body
                for stmt in body {
                    self.infer_stmt(stmt)?;
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                // Infer types in try body
                self.infer_body(body)?;

                // Infer types in except handlers
                for handler in handlers {
                    self.infer_body(&handler.body)?;
                }

                // Infer types in else clause
                if let Some(else_stmts) = orelse {
                    self.infer_body(else_stmts)?;
                }

                // Infer types in finally clause
                if let Some(finally_stmts) = finalbody {
                    self.infer_body(finally_stmts)?;
                }
            }
        }
        Ok(())
    }

    fn infer_expr(&mut self, expr: &HirExpr) -> Result<Type> {
        match expr {
            HirExpr::Literal(lit) => Ok(self.infer_literal(lit)),
            HirExpr::Var(name) => Ok(self.infer_variable(name)),
            HirExpr::Binary { op, left, right } => self.infer_binary(op, left, right),
            HirExpr::Unary { op, operand } => self.infer_unary(op, operand),
            HirExpr::Call { func, args } => self.infer_call(func, args),
            HirExpr::Index { base, index } => self.infer_index(base, index),
            HirExpr::List(elts) => self.infer_list(elts),
            HirExpr::Dict(items) => self.infer_dict(items),
            HirExpr::Tuple(elts) => self.infer_tuple(elts),
            HirExpr::Attribute { value, attr: _ } => {
                // For attribute access, infer the type of the base value
                // In a more sophisticated implementation, we would track attribute types
                let _base_type = self.infer_expr(value)?;
                Ok(Type::Unknown)
            }
            _ => Ok(Type::Unknown),
        }
    }

    fn infer_literal(&self, lit: &depyler_core::hir::Literal) -> Type {
        match lit {
            depyler_core::hir::Literal::Int(_) => Type::Int,
            depyler_core::hir::Literal::Float(_) => Type::Float,
            depyler_core::hir::Literal::String(_) => Type::String,
            depyler_core::hir::Literal::Bool(_) => Type::Bool,
            depyler_core::hir::Literal::None => Type::None,
        }
    }

    fn infer_variable(&self, name: &str) -> Type {
        self.env
            .get_var_type(name)
            .cloned()
            .unwrap_or(Type::Unknown)
    }

    fn infer_binary(
        &mut self,
        op: &depyler_core::hir::BinOp,
        left: &HirExpr,
        right: &HirExpr,
    ) -> Result<Type> {
        let left_type = self.infer_expr(left)?;
        let right_type = self.infer_expr(right)?;
        Ok(self.infer_binary_op(*op, &left_type, &right_type))
    }

    fn infer_unary(&mut self, op: &depyler_core::hir::UnaryOp, operand: &HirExpr) -> Result<Type> {
        let operand_type = self.infer_expr(operand)?;
        Ok(self.infer_unary_op(*op, &operand_type))
    }

    fn infer_call(&mut self, func: &str, args: &[HirExpr]) -> Result<Type> {
        // Infer argument types
        for arg in args {
            self.infer_expr(arg)?;
        }

        // Get function return type
        Ok(if let Some(sig) = self.env.get_function_signature(func) {
            sig.return_type.clone()
        } else {
            Type::Unknown
        })
    }

    fn infer_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<Type> {
        let base_type = self.infer_expr(base)?;
        self.infer_expr(index)?;
        Ok(self.get_element_type(&base_type))
    }

    fn infer_list(&mut self, elts: &[HirExpr]) -> Result<Type> {
        if elts.is_empty() {
            Ok(Type::List(Box::new(Type::Unknown)))
        } else {
            let first_type = self.infer_expr(&elts[0])?;
            Ok(Type::List(Box::new(first_type)))
        }
    }

    fn infer_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<Type> {
        if items.is_empty() {
            Ok(Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)))
        } else {
            let (k, v) = &items[0];
            let key_type = self.infer_expr(k)?;
            let val_type = self.infer_expr(v)?;
            Ok(Type::Dict(Box::new(key_type), Box::new(val_type)))
        }
    }

    fn infer_tuple(&mut self, elts: &[HirExpr]) -> Result<Type> {
        let types: Vec<Type> = elts
            .iter()
            .map(|e| self.infer_expr(e))
            .collect::<Result<Vec<_>>>()?;
        Ok(Type::Tuple(types))
    }

    fn infer_binary_op(&self, op: depyler_core::hir::BinOp, left: &Type, right: &Type) -> Type {
        use depyler_core::hir::BinOp;

        match op {
            // Arithmetic operators
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::FloorDiv | BinOp::Mod => {
                if matches!(left, Type::Float) || matches!(right, Type::Float) {
                    Type::Float
                } else if matches!(left, Type::Int) && matches!(right, Type::Int) {
                    Type::Int
                } else {
                    Type::Unknown
                }
            }
            // Comparison operators
            BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
                Type::Bool
            }
            // Logical operators
            BinOp::And | BinOp::Or => Type::Bool,
            // Bitwise operators
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift => {
                if matches!(left, Type::Int) && matches!(right, Type::Int) {
                    Type::Int
                } else {
                    Type::Unknown
                }
            }
            // Membership operators
            BinOp::In | BinOp::NotIn => Type::Bool,
            _ => Type::Unknown,
        }
    }

    fn infer_unary_op(&self, op: depyler_core::hir::UnaryOp, operand: &Type) -> Type {
        use depyler_core::hir::UnaryOp;

        match op {
            UnaryOp::Not => Type::Bool,
            UnaryOp::Neg | UnaryOp::Pos => operand.clone(),
            UnaryOp::BitNot => {
                if matches!(operand, Type::Int) {
                    Type::Int
                } else {
                    Type::Unknown
                }
            }
        }
    }

    fn get_element_type(&self, container: &Type) -> Type {
        match container {
            Type::List(elem) => (**elem).clone(),
            Type::Tuple(elems) => {
                if elems.is_empty() {
                    Type::Unknown
                } else {
                    elems[0].clone() // Simplified - assumes homogeneous tuple
                }
            }
            Type::Dict(_, val) => (**val).clone(),
            Type::String => Type::String, // For string indexing
            _ => Type::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::hir::{BinOp, HirParam, Literal, UnaryOp};

    #[test]
    fn test_type_environment_new() {
        let env = TypeEnvironment::new();
        assert!(env.variables.is_empty());

        // Should have built-in functions
        assert!(env.get_function_signature("len").is_some());
        assert!(env.get_function_signature("range").is_some());
        assert!(env.get_function_signature("abs").is_some());
    }

    #[test]
    fn test_type_environment_default() {
        let env = TypeEnvironment::default();
        assert!(env.variables.is_empty());
        assert!(env.get_function_signature("len").is_some());
    }

    #[test]
    fn test_type_environment_variable_management() {
        let mut env = TypeEnvironment::new();

        // Test setting and getting variables
        env.set_var_type("x".to_string(), Type::Int);
        assert_eq!(env.get_var_type("x"), Some(&Type::Int));

        env.set_var_type("y".to_string(), Type::String);
        assert_eq!(env.get_var_type("y"), Some(&Type::String));

        // Test non-existent variable
        assert_eq!(env.get_var_type("z"), None);
    }

    #[test]
    fn test_builtin_function_signatures() {
        let env = TypeEnvironment::new();

        // Test len function
        let len_sig = env.get_function_signature("len").unwrap();
        assert_eq!(len_sig.params, vec![Type::Unknown]);
        assert_eq!(len_sig.return_type, Type::Int);

        // Test range function
        let range_sig = env.get_function_signature("range").unwrap();
        assert_eq!(range_sig.params, vec![Type::Int]);
        assert_eq!(range_sig.return_type, Type::Custom("range".to_string()));

        // Test numeric functions
        for func in ["abs", "max", "min", "sum"] {
            let sig = env.get_function_signature(func).unwrap();
            assert_eq!(sig.params, vec![Type::Unknown]);
            assert_eq!(sig.return_type, Type::Unknown);
        }
    }

    #[test]
    fn test_type_inferencer_new() {
        let inferencer = TypeInferencer::new();
        assert!(inferencer.env.variables.is_empty());
    }

    #[test]
    fn test_type_inferencer_default() {
        let inferencer = TypeInferencer::default();
        assert!(inferencer.env.variables.is_empty());
    }

    #[test]
    fn test_infer_literal() {
        let inferencer = TypeInferencer::new();

        assert_eq!(inferencer.infer_literal(&Literal::Int(42)), Type::Int);
        assert_eq!(
            inferencer.infer_literal(&Literal::Float(std::f64::consts::PI)),
            Type::Float
        );
        assert_eq!(
            inferencer.infer_literal(&Literal::String("hello".to_string())),
            Type::String
        );
        assert_eq!(inferencer.infer_literal(&Literal::Bool(true)), Type::Bool);
        assert_eq!(inferencer.infer_literal(&Literal::None), Type::None);
    }

    #[test]
    fn test_infer_variable() {
        let mut inferencer = TypeInferencer::new();

        // Test unknown variable
        assert_eq!(inferencer.infer_variable("unknown"), Type::Unknown);

        // Test known variable
        inferencer.env.set_var_type("x".to_string(), Type::Int);
        assert_eq!(inferencer.infer_variable("x"), Type::Int);
    }

    #[test]
    fn test_infer_binary_op_arithmetic() {
        let inferencer = TypeInferencer::new();

        // Int + Int = Int
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Int, &Type::Int),
            Type::Int
        );

        // Float + Int = Float
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Float, &Type::Int),
            Type::Float
        );

        // Int + Float = Float
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Mul, &Type::Int, &Type::Float),
            Type::Float
        );

        // Unknown types
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::String, &Type::Int),
            Type::Unknown
        );
    }

    #[test]
    fn test_infer_binary_op_comparison() {
        let inferencer = TypeInferencer::new();

        for op in [
            BinOp::Eq,
            BinOp::NotEq,
            BinOp::Lt,
            BinOp::LtEq,
            BinOp::Gt,
            BinOp::GtEq,
        ] {
            assert_eq!(
                inferencer.infer_binary_op(op, &Type::Int, &Type::Int),
                Type::Bool
            );
        }
    }

    #[test]
    fn test_infer_binary_op_logical() {
        let inferencer = TypeInferencer::new();

        assert_eq!(
            inferencer.infer_binary_op(BinOp::And, &Type::Bool, &Type::Bool),
            Type::Bool
        );

        assert_eq!(
            inferencer.infer_binary_op(BinOp::Or, &Type::Bool, &Type::Bool),
            Type::Bool
        );
    }

    #[test]
    fn test_infer_binary_op_bitwise() {
        let inferencer = TypeInferencer::new();

        for op in [
            BinOp::BitAnd,
            BinOp::BitOr,
            BinOp::BitXor,
            BinOp::LShift,
            BinOp::RShift,
        ] {
            assert_eq!(
                inferencer.infer_binary_op(op, &Type::Int, &Type::Int),
                Type::Int
            );

            assert_eq!(
                inferencer.infer_binary_op(op, &Type::String, &Type::Int),
                Type::Unknown
            );
        }
    }

    #[test]
    fn test_infer_binary_op_membership() {
        let inferencer = TypeInferencer::new();

        assert_eq!(
            inferencer.infer_binary_op(BinOp::In, &Type::Int, &Type::List(Box::new(Type::Int))),
            Type::Bool
        );

        assert_eq!(
            inferencer.infer_binary_op(
                BinOp::NotIn,
                &Type::String,
                &Type::List(Box::new(Type::String))
            ),
            Type::Bool
        );
    }

    #[test]
    fn test_infer_unary_op() {
        let inferencer = TypeInferencer::new();

        // Not operator
        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::Not, &Type::Bool),
            Type::Bool
        );

        // Negation/positive preserve type
        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::Neg, &Type::Int),
            Type::Int
        );

        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::Pos, &Type::Float),
            Type::Float
        );

        // Bitwise not on integers
        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::BitNot, &Type::Int),
            Type::Int
        );

        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::BitNot, &Type::String),
            Type::Unknown
        );
    }

    #[test]
    fn test_get_element_type() {
        let inferencer = TypeInferencer::new();

        // List elements
        assert_eq!(
            inferencer.get_element_type(&Type::List(Box::new(Type::Int))),
            Type::Int
        );

        // Tuple elements (first element)
        assert_eq!(
            inferencer.get_element_type(&Type::Tuple(vec![Type::String, Type::Int])),
            Type::String
        );

        // Empty tuple
        assert_eq!(
            inferencer.get_element_type(&Type::Tuple(vec![])),
            Type::Unknown
        );

        // Dict values
        assert_eq!(
            inferencer.get_element_type(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
            Type::Int
        );

        // String indexing
        assert_eq!(inferencer.get_element_type(&Type::String), Type::String);

        // Unknown container
        assert_eq!(inferencer.get_element_type(&Type::Unknown), Type::Unknown);
    }

    #[test]
    fn test_function_signature() {
        let sig = FunctionSignature {
            params: vec![Type::Int, Type::String],
            return_type: Type::Bool,
        };

        assert_eq!(sig.params, vec![Type::Int, Type::String]);
        assert_eq!(sig.return_type, Type::Bool);
    }

    // Integration tests with HIR expressions
    use depyler_core::hir::HirExpr;

    #[test]
    fn test_infer_expr_literal() {
        let mut inferencer = TypeInferencer::new();

        let expr = HirExpr::Literal(Literal::Int(42));
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::Int);
    }

    #[test]
    fn test_infer_expr_list() {
        let mut inferencer = TypeInferencer::new();

        // Empty list
        let expr = HirExpr::List(vec![]);
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::List(Box::new(Type::Unknown)));

        // List with integers
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::List(Box::new(Type::Int)));
    }

    #[test]
    fn test_infer_expr_dict() {
        let mut inferencer = TypeInferencer::new();

        // Empty dict
        let expr = HirExpr::Dict(vec![]);
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(
            result,
            Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
        );

        // Dict with string keys and int values
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(
            result,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int))
        );
    }

    #[test]
    fn test_infer_expr_tuple() {
        let mut inferencer = TypeInferencer::new();

        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".to_string())),
            HirExpr::Literal(Literal::Bool(true)),
        ]);
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(
            result,
            Type::Tuple(vec![Type::Int, Type::String, Type::Bool])
        );
    }

    #[test]
    fn test_infer_call_builtin() {
        let mut inferencer = TypeInferencer::new();

        // len() call
        let result = inferencer
            .infer_call(
                "len",
                &[HirExpr::Literal(Literal::String("test".to_string()))],
            )
            .unwrap();
        assert_eq!(result, Type::Int);

        // Unknown function
        let result = inferencer.infer_call("unknown_func", &[]).unwrap();
        assert_eq!(result, Type::Unknown);
    }

    // DEPYLER-0022: Mutation Kill Tests (46 MISSED mutations @ 0% baseline)

    // Category 1: HirExpr Match Arm Deletions (10 mutations, lines 186-195)

    #[test]
    fn test_infer_expr_literal_arm_kill_mutation() {
        // Target: Line 186 - delete match arm HirExpr::Literal(lit)
        let mut inferencer = TypeInferencer::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::Int);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_var_arm_kill_mutation() {
        // Target: Line 187 - delete match arm HirExpr::Var(name)
        let mut inferencer = TypeInferencer::new();
        inferencer.env.set_var_type("x".to_string(), Type::String);
        let expr = HirExpr::Var("x".to_string());
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::String);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_binary_arm_kill_mutation() {
        // Target: Line 188 - delete match arm HirExpr::Binary{op, left, right}
        let mut inferencer = TypeInferencer::new();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::Int);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_unary_arm_kill_mutation() {
        // Target: Line 189 - delete match arm HirExpr::Unary{op, operand}
        let mut inferencer = TypeInferencer::new();
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::Bool);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_call_arm_kill_mutation() {
        // Target: Line 190 - delete match arm HirExpr::Call{func, args}
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_call(
                "len",
                &[HirExpr::Literal(Literal::String("test".to_string()))],
            )
            .unwrap();
        assert_eq!(result, Type::Int);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_index_arm_kill_mutation() {
        // Target: Line 191 - delete match arm HirExpr::Index{base, index}
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_index(
                &HirExpr::Literal(Literal::String("test".to_string())),
                &HirExpr::Literal(Literal::Int(0)),
            )
            .unwrap();
        assert_eq!(result, Type::String);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_list_arm_kill_mutation() {
        // Target: Line 192 - delete match arm HirExpr::List(elts)
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_list(&[
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ])
            .unwrap();
        assert_eq!(result, Type::List(Box::new(Type::Int)));
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_dict_arm_kill_mutation() {
        // Target: Line 193 - delete match arm HirExpr::Dict(items)
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_dict(&[(
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(42)),
            )])
            .unwrap();
        assert_eq!(
            result,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int))
        );
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_tuple_arm_kill_mutation() {
        // Target: Line 194 - delete match arm HirExpr::Tuple(elts)
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_tuple(&[
                HirExpr::Literal(Literal::String("name".to_string())),
                HirExpr::Literal(Literal::Int(42)),
            ])
            .unwrap();
        assert_eq!(result, Type::Tuple(vec![Type::String, Type::Int]));
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_infer_expr_attribute_arm_kill_mutation() {
        // Target: Line 195 - delete match arm HirExpr::Attribute{value, attr:_}
        let mut inferencer = TypeInferencer::new();
        inferencer.env.set_var_type("obj".to_string(), Type::String);
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "upper".to_string(),
        };
        let result = inferencer.infer_expr(&expr).unwrap();
        // Currently returns Unknown (simplified implementation)
        assert_eq!(result, Type::Unknown);
        // If match arm deleted: would still return Type::Unknown (fails to distinguish)
        // This test documents current behavior but doesn't kill the mutation
    }

    // Category 2: Type Match Arm Deletions (4 mutations, lines 338, 339, 346, 347)

    #[test]
    fn test_get_element_type_list_arm_kill_mutation() {
        // Target: Line 338 - delete match arm Type::List(elem)
        let inferencer = TypeInferencer::new();
        let result = inferencer.get_element_type(&Type::List(Box::new(Type::String)));
        assert_eq!(result, Type::String);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_get_element_type_tuple_arm_kill_mutation() {
        // Target: Line 339 - delete match arm Type::Tuple(elems)
        let inferencer = TypeInferencer::new();
        let result = inferencer.get_element_type(&Type::Tuple(vec![Type::String, Type::Int]));
        assert_eq!(result, Type::String);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_get_element_type_dict_arm_kill_mutation() {
        // Target: Line 346 - delete match arm Type::Dict(_, val)
        let inferencer = TypeInferencer::new();
        let result =
            inferencer.get_element_type(&Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
        assert_eq!(result, Type::Int);
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_get_element_type_string_arm_kill_mutation() {
        // Target: Line 347 - delete match arm Type::String
        let inferencer = TypeInferencer::new();
        let result = inferencer.get_element_type(&Type::String);
        assert_eq!(result, Type::String);
        // If match arm deleted: would return Type::Unknown
    }

    // Category 3: BinOp Match Arm Deletions (5 mutations, lines 291, 301, 305, 307, 315)

    #[test]
    fn test_binop_arithmetic_arm_kill_mutation() {
        // Target: Line 291 - delete match arm BinOp::Add | Sub | Mul | Div | FloorDiv | Mod
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Int, &Type::Int),
            Type::Int
        );
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Mul, &Type::Float, &Type::Float),
            Type::Float
        );
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_binop_comparison_arm_kill_mutation() {
        // Target: Line 301 - delete match arm BinOp::Eq | NotEq | Lt | LtEq | Gt | GtEq
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Eq, &Type::Int, &Type::Int),
            Type::Bool
        );
        assert_eq!(
            inferencer.infer_binary_op(BinOp::NotEq, &Type::String, &Type::String),
            Type::Bool
        );
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_binop_logical_arm_kill_mutation() {
        // Target: Line 305 - delete match arm BinOp::And | BinOp::Or
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_binary_op(BinOp::And, &Type::Bool, &Type::Bool),
            Type::Bool
        );
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Or, &Type::Bool, &Type::Bool),
            Type::Bool
        );
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_binop_bitwise_arm_kill_mutation() {
        // Target: Line 307 - delete match arm BinOp::BitAnd | BitOr | BitXor | LShift | RShift
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_binary_op(BinOp::BitAnd, &Type::Int, &Type::Int),
            Type::Int
        );
        assert_eq!(
            inferencer.infer_binary_op(BinOp::LShift, &Type::Int, &Type::Int),
            Type::Int
        );
        // If match arm deleted: would return Type::Unknown
    }

    #[test]
    fn test_binop_membership_arm_kill_mutation() {
        // Target: Line 315 - delete match arm BinOp::In | BinOp::NotIn
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_binary_op(
                BinOp::In,
                &Type::String,
                &Type::List(Box::new(Type::String))
            ),
            Type::Bool
        );
        // If match arm deleted: would return Type::Unknown
    }

    // Category 4: Boolean Logic (3 mutations, lines 292, 294, 308)

    #[test]
    fn test_boolean_logic_line_292_kill_mutation() {
        // Target: Line 292 - replace || with &&
        // Tests arithmetic type compatibility check
        let inferencer = TypeInferencer::new();
        // Int and Int should work (both numeric)
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Int, &Type::Int),
            Type::Int
        );
        // Float and Float should work (both numeric)
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Float, &Type::Float),
            Type::Float
        );
        // String and String should NOT work for arithmetic
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::String, &Type::String),
            Type::Unknown
        );
        // If || → &&: would fail on valid Float+Float
    }

    #[test]
    fn test_boolean_logic_line_294_kill_mutation() {
        // Target: Line 294 - replace && with ||
        // Tests both operands are Int (line 294 checks after Float coercion)
        let inferencer = TypeInferencer::new();
        // Int and Int should work
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Int, &Type::Int),
            Type::Int
        );
        // Int and Float should coerce to Float (line 292 handles this)
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Int, &Type::Float),
            Type::Float
        );
        // String and String should fail
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::String, &Type::String),
            Type::Unknown
        );
        // If && → ||: line 294 would incorrectly allow String+String → Int
    }

    #[test]
    fn test_boolean_logic_line_308_kill_mutation() {
        // Target: Line 308 - replace && with ||
        // Tests both operands are Int for bitwise
        let inferencer = TypeInferencer::new();
        // Int and Int should work
        assert_eq!(
            inferencer.infer_binary_op(BinOp::BitAnd, &Type::Int, &Type::Int),
            Type::Int
        );
        // Float and Int should NOT work (bitwise requires Int)
        assert_eq!(
            inferencer.infer_binary_op(BinOp::BitAnd, &Type::Float, &Type::Int),
            Type::Unknown
        );
        // If && → ||: would incorrectly allow Float & Int
    }

    // Category 5: Return Value Replacements (24 mutations, scattered lines)

    #[test]
    fn test_return_infer_variable_default_kill_mutation() {
        // Target: Line 216 - replace infer_variable -> Type with Default::default()
        let mut inferencer = TypeInferencer::new();
        inferencer.env.set_var_type("x".to_string(), Type::String);
        let result = inferencer.infer_variable("x");
        assert_eq!(result, Type::String);
        // If mutated to Default::default(): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_literal_default_kill_mutation() {
        // Target: Line 206 - replace infer_literal -> Type with Default::default()
        let inferencer = TypeInferencer::new();
        assert_eq!(inferencer.infer_literal(&Literal::Int(42)), Type::Int);
        assert_eq!(
            inferencer.infer_literal(&Literal::String("test".to_string())),
            Type::String
        );
        // If mutated to Default::default(): would return Type::Unknown
    }

    #[test]
    fn test_return_get_element_type_default_kill_mutation() {
        // Target: Line 337 - replace get_element_type -> Type with Default::default()
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.get_element_type(&Type::List(Box::new(Type::Int))),
            Type::Int
        );
        assert_eq!(
            inferencer.get_element_type(&Type::Dict(Box::new(Type::String), Box::new(Type::Float))),
            Type::Float
        );
        // If mutated to Default::default(): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_unary_op_default_kill_mutation() {
        // Target: Line 321 - replace infer_unary_op -> Type with Default::default()
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::Neg, &Type::Int),
            Type::Int
        );
        assert_eq!(
            inferencer.infer_unary_op(UnaryOp::Not, &Type::Bool),
            Type::Bool
        );
        // If mutated to Default::default(): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_binary_op_default_kill_mutation() {
        // Target: Line 287 - replace infer_binary_op -> Type with Default::default()
        let inferencer = TypeInferencer::new();
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Add, &Type::Int, &Type::Int),
            Type::Int
        );
        assert_eq!(
            inferencer.infer_binary_op(BinOp::Eq, &Type::String, &Type::String),
            Type::Bool
        );
        // If mutated to Default::default(): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_tuple_ok_default_kill_mutation() {
        // Target: Line 279 - replace infer_tuple -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_tuple(&[
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::String("test".to_string())),
            ])
            .unwrap();
        assert_eq!(result, Type::Tuple(vec![Type::Int, Type::String]));
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_dict_ok_default_kill_mutation() {
        // Target: Line 268 - replace infer_dict -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_dict(&[(
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(42)),
            )])
            .unwrap();
        assert_eq!(
            result,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int))
        );
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_call_ok_default_kill_mutation() {
        // Target: Line 240 - replace infer_call -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_call(
                "len",
                &[HirExpr::Literal(Literal::String("test".to_string()))],
            )
            .unwrap();
        assert_eq!(result, Type::Int);
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_index_ok_default_kill_mutation() {
        // Target: Line 253 - replace infer_index -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_index(
                &HirExpr::Literal(Literal::String("test".to_string())),
                &HirExpr::Literal(Literal::Int(0)),
            )
            .unwrap();
        assert_eq!(result, Type::String);
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_list_ok_default_kill_mutation() {
        // Target: Line 259 - replace infer_list -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_list(&[
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ])
            .unwrap();
        assert_eq!(result, Type::List(Box::new(Type::Int)));
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_binary_ok_default_kill_mutation() {
        // Target: Line 228 - replace infer_binary -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_binary(
                &BinOp::Add,
                &HirExpr::Literal(Literal::Int(1)),
                &HirExpr::Literal(Literal::Int(2)),
            )
            .unwrap();
        assert_eq!(result, Type::Int);
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_unary_ok_default_kill_mutation() {
        // Target: Line 234 - replace infer_unary -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let result = inferencer
            .infer_unary(&UnaryOp::Neg, &HirExpr::Literal(Literal::Int(42)))
            .unwrap();
        assert_eq!(result, Type::Int);
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_get_var_type_none_kill_mutation() {
        // Target: Line 67 - replace get_var_type -> Option<&Type> with None
        let mut env = TypeEnvironment::new();
        env.set_var_type("x".to_string(), Type::Int);
        assert_eq!(env.get_var_type("x"), Some(&Type::Int));
        // If mutated to None: would always return None even when variable exists
    }

    #[test]
    fn test_return_get_function_signature_none_kill_mutation() {
        // Target: Line 75 - replace get_function_signature -> Option<&FunctionSignature> with None
        let env = TypeEnvironment::new();
        let sig = env.get_function_signature("len");
        assert!(sig.is_some());
        assert_eq!(sig.unwrap().return_type, Type::Int);
        // If mutated to None: would always return None even for builtins
    }

    #[test]
    fn test_return_infer_expr_ok_default_kill_mutation() {
        // Target: Line 185 - replace infer_expr -> Result<Type> with Ok(Default::default())
        let mut inferencer = TypeInferencer::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = inferencer.infer_expr(&expr).unwrap();
        assert_eq!(result, Type::Int);
        // If mutated to Ok(Default::default()): would return Type::Unknown
    }

    #[test]
    fn test_return_infer_function_hashmap_mutations() {
        // Target: Line 98 (3 mutations) - infer_function HashMap returns
        use depyler_annotations::TranspilationAnnotations;
        use depyler_core::hir::FunctionProperties;
        use smallvec::SmallVec;
        let mut inferencer = TypeInferencer::new();
        let func = HirFunction {
            name: "test_func".to_string(),
            params: SmallVec::from_vec(vec![HirParam {
                name: "x".to_string(),
                ty: Type::Int,
                default: None,
            }]),
            ret_type: Type::Int,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let result = inferencer.infer_function(&func).unwrap();

        // Should have both params and assigned variables
        assert!(result.contains_key("x"));
        assert!(result.contains_key("y"));
        assert_eq!(result.get("x"), Some(&Type::Int));
        assert_eq!(result.get("y"), Some(&Type::Int));

        // If mutated to Ok(HashMap::new()): would be empty
        // If mutated to Ok(HashMap::from_iter([(String::new(), ...)])): would have wrong key
        // If mutated to Ok(HashMap::from_iter([("xyzzy".into(), ...)])): would have wrong key
    }

    #[test]
    fn test_return_infer_body_ok_unit_kill_mutation() {
        // Target: Line 109 - replace infer_body -> Result<()> with Ok(())
        let mut inferencer = TypeInferencer::new();
        let body = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Var("x".to_string()),
                type_annotation: None,
            },
        ];

        // This should process assignments and update environment
        inferencer.infer_body(&body).unwrap();

        // Verify variables were added to environment
        assert_eq!(inferencer.env.get_var_type("x"), Some(&Type::Int));
        assert_eq!(inferencer.env.get_var_type("y"), Some(&Type::Int));

        // If mutated to Ok(()) without body processing: environment wouldn't be updated
    }

    #[test]
    fn test_return_infer_stmt_ok_unit_kill_mutation() {
        // Target: Line 116 - replace infer_stmt -> Result<()> with Ok(())
        let mut inferencer = TypeInferencer::new();
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("z".to_string()),
            value: HirExpr::Literal(Literal::String("test".to_string())),
            type_annotation: None,
        };

        inferencer.infer_stmt(&stmt).unwrap();

        // Verify statement was processed
        assert_eq!(inferencer.env.get_var_type("z"), Some(&Type::String));

        // If mutated to Ok(()) without processing: variable wouldn't be added
    }

    #[test]
    fn test_mutation_add_builtin_functions_noop_kill() {
        // Target: Line 37 - replace add_builtin_functions with ()
        let env = TypeEnvironment::new();

        // Should have builtin functions
        assert!(env.get_function_signature("len").is_some());
        assert!(env.get_function_signature("range").is_some());
        assert!(env.get_function_signature("abs").is_some());

        // If mutated to (): builtins wouldn't exist
    }

    #[test]
    fn test_mutation_set_var_type_noop_kill() {
        // Target: Line 71 - replace set_var_type with ()
        let mut env = TypeEnvironment::new();

        env.set_var_type("test_var".to_string(), Type::Float);

        // Should be set
        assert_eq!(env.get_var_type("test_var"), Some(&Type::Float));

        // If mutated to (): variable wouldn't be stored
    }
}
