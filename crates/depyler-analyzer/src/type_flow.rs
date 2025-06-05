use anyhow::Result;
use depyler_core::hir::{HirExpr, HirFunction, HirStmt, Type};
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
        for (name, ty) in &func.params {
            self.env.set_var_type(name.clone(), ty.clone());
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
            HirStmt::Assign { target, value } => {
                let value_type = self.infer_expr(value)?;
                self.env.set_var_type(target.clone(), value_type);
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

    fn infer_binary(&mut self, op: &depyler_core::hir::BinOp, left: &HirExpr, right: &HirExpr) -> Result<Type> {
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
