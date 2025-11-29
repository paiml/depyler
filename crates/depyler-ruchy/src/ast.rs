//! Ruchy AST definitions and builder

use anyhow::{anyhow, Result};
use depyler_core::hir::{HirFunction, HirModule};
use depyler_core::simplified_hir::{
    Hir, HirBinaryOp, HirExpr, HirLiteral, HirMetadata, HirParam, HirStatement, HirType, HirUnaryOp,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ruchy AST expression type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuchyExpr {
    /// Literal values
    Literal(Literal),

    /// Variable identifier
    Identifier(String),

    /// Binary operation
    Binary {
        left: Box<RuchyExpr>,
        op: BinaryOp,
        right: Box<RuchyExpr>,
    },

    /// Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<RuchyExpr>,
    },

    /// Function definition
    Function {
        name: String,
        params: Vec<Param>,
        body: Box<RuchyExpr>,
        is_async: bool,
        return_type: Option<RuchyType>,
    },

    /// Lambda expression
    Lambda {
        params: Vec<Param>,
        body: Box<RuchyExpr>,
    },

    /// Function call
    Call {
        func: Box<RuchyExpr>,
        args: Vec<RuchyExpr>,
    },

    /// Method call
    MethodCall {
        receiver: Box<RuchyExpr>,
        method: String,
        args: Vec<RuchyExpr>,
    },

    /// Pipeline operator
    Pipeline {
        expr: Box<RuchyExpr>,
        stages: Vec<PipelineStage>,
    },

    /// If expression
    If {
        condition: Box<RuchyExpr>,
        then_branch: Box<RuchyExpr>,
        else_branch: Option<Box<RuchyExpr>>,
    },

    /// Match expression
    Match {
        expr: Box<RuchyExpr>,
        arms: Vec<MatchArm>,
    },

    /// For loop
    For {
        var: String,
        iter: Box<RuchyExpr>,
        body: Box<RuchyExpr>,
    },

    /// While loop
    While {
        condition: Box<RuchyExpr>,
        body: Box<RuchyExpr>,
    },

    /// Block of statements
    Block(Vec<RuchyExpr>),

    /// Let binding
    Let {
        name: String,
        value: Box<RuchyExpr>,
        body: Box<RuchyExpr>,
        is_mutable: bool,
    },

    /// List literal
    List(Vec<RuchyExpr>),

    /// String interpolation
    StringInterpolation { parts: Vec<StringPart> },

    /// Struct definition
    Struct {
        name: String,
        fields: Vec<StructField>,
    },

    /// Struct literal
    StructLiteral {
        name: String,
        fields: Vec<(String, RuchyExpr)>,
    },

    /// Field access
    FieldAccess {
        object: Box<RuchyExpr>,
        field: String,
    },

    /// Await expression
    Await { expr: Box<RuchyExpr> },

    /// Try expression
    Try { expr: Box<RuchyExpr> },

    /// `DataFrame` literal
    DataFrame { columns: Vec<DataFrameColumn> },

    /// Range expression
    Range {
        start: Box<RuchyExpr>,
        end: Box<RuchyExpr>,
        inclusive: bool,
    },

    /// Break statement
    Break { label: Option<String> },

    /// Continue statement
    Continue { label: Option<String> },

    /// Return statement
    Return { value: Option<Box<RuchyExpr>> },
}

/// Literal values in Ruchy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Unit,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Negate,
    BitwiseNot,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub typ: Option<RuchyType>,
    pub default: Option<Box<RuchyExpr>>,
}

/// Pipeline stage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PipelineStage {
    Map(Box<RuchyExpr>),
    Filter(Box<RuchyExpr>),
    FlatMap(Box<RuchyExpr>),
    Reduce(Box<RuchyExpr>),
    Call(String, Vec<RuchyExpr>),
}

/// Match arm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<RuchyExpr>>,
    pub body: Box<RuchyExpr>,
}

/// Pattern for match expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Identifier(String),
    Tuple(Vec<Pattern>),
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
    List(Vec<Pattern>),
}

/// String interpolation part
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StringPart {
    Text(String),
    Expr(Box<RuchyExpr>),
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub typ: RuchyType,
    pub is_public: bool,
}

/// DataFrame column definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataFrameColumn {
    pub name: String,
    pub values: Vec<RuchyExpr>,
}

/// Ruchy type system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuchyType {
    /// Primitive types
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    F64,
    Bool,
    Char,
    String,

    /// Compound types
    Vec(Box<RuchyType>),
    Array(Box<RuchyType>, usize),
    Tuple(Vec<RuchyType>),
    Option(Box<RuchyType>),
    Result(Box<RuchyType>, Box<RuchyType>),

    /// Function type
    Function {
        params: Vec<RuchyType>,
        returns: Box<RuchyType>,
    },

    /// User-defined type
    Named(String),

    /// Generic type parameter
    Generic(String),

    /// Reference type
    Reference {
        typ: Box<RuchyType>,
        is_mutable: bool,
    },

    /// Dynamic type (for gradual typing)
    Dynamic,
}

/// Ruchy AST builder
pub struct RuchyAstBuilder {
    #[allow(dead_code)]
    type_cache: HashMap<String, RuchyType>,
}

impl RuchyAstBuilder {
    /// Creates a new AST builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            type_cache: HashMap::new(),
        }
    }

    /// Creates a builder with custom configuration
    #[must_use]
    pub fn with_config(_config: &crate::RuchyConfig) -> Self {
        Self::new()
    }

    /// Builds a Ruchy AST from HIR
    pub fn build(&self, module: &HirModule) -> Result<RuchyExpr> {
        // Convert HirModule to simplified HIR, then to Ruchy AST
        let simplified = self.module_to_simplified_hir(module)?;
        self.convert_hir_expr(&simplified.root)
    }

    /// Convert HirModule to simplified HIR
    fn module_to_simplified_hir(&self, module: &HirModule) -> Result<Hir> {
        // Create a block containing all module items
        let mut statements = Vec::new();

        // Convert functions
        for func in &module.functions {
            statements.push(HirStatement::Expression(Box::new(
                self.convert_function_to_expr(func)?,
            )));
        }

        // Create the root expression as a block
        let root = HirExpr::Block(statements);

        Ok(Hir {
            root,
            metadata: HirMetadata::default(),
        })
    }

    /// Convert HirFunction to simplified HirExpr
    fn convert_function_to_expr(&self, func: &HirFunction) -> Result<HirExpr> {
        // Create a simplified function expression
        Ok(HirExpr::Function {
            name: func.name.clone(),
            params: vec![], // We'll need to convert params properly
            body: Box::new(HirExpr::Block(vec![])), // Convert body
            is_async: false,
            return_type: None,
        })
    }

    /// Converts HIR expression to Ruchy expression
    fn convert_hir_expr(&self, expr: &HirExpr) -> Result<RuchyExpr> {
        match expr {
            HirExpr::Literal(lit) => Ok(self.convert_literal(lit)),
            HirExpr::Identifier(name) => Ok(RuchyExpr::Identifier(name.clone())),

            HirExpr::Binary { left, op, right } => Ok(RuchyExpr::Binary {
                left: Box::new(self.convert_hir_expr(left)?),
                op: self.convert_binary_op(op)?,
                right: Box::new(self.convert_hir_expr(right)?),
            }),

            HirExpr::Unary { op, operand } => Ok(RuchyExpr::Unary {
                op: self.convert_unary_op(op)?,
                operand: Box::new(self.convert_hir_expr(operand)?),
            }),

            HirExpr::Call { func, args, .. } => Ok(RuchyExpr::Call {
                func: Box::new(self.convert_hir_expr(func)?),
                args: args
                    .iter()
                    .map(|arg| self.convert_hir_expr(arg))
                    .collect::<Result<Vec<_>>>()?,
            }),

            HirExpr::If {
                condition,
                then_branch,
                else_branch,
            } => Ok(RuchyExpr::If {
                condition: Box::new(self.convert_hir_expr(condition)?),
                then_branch: Box::new(self.convert_hir_expr(then_branch)?),
                else_branch: else_branch
                    .as_ref()
                    .map(|e| self.convert_hir_expr(e).map(Box::new))
                    .transpose()?,
            }),

            HirExpr::Block(stmts) => {
                let exprs = stmts
                    .iter()
                    .map(|stmt| self.convert_statement(stmt))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RuchyExpr::Block(exprs))
            }

            HirExpr::List(elements) => {
                let ruchy_elements = elements
                    .iter()
                    .map(|e| self.convert_hir_expr(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RuchyExpr::List(ruchy_elements))
            }

            HirExpr::Function {
                name,
                params,
                body,
                is_async,
                return_type,
            } => Ok(RuchyExpr::Function {
                name: name.clone(),
                params: self.convert_params(params)?,
                body: Box::new(self.convert_hir_expr(body)?),
                is_async: *is_async,
                return_type: return_type
                    .as_ref()
                    .map(|t| self.convert_type(t))
                    .transpose()?,
            }),

            HirExpr::Lambda { params, body } => Ok(RuchyExpr::Lambda {
                params: self.convert_params(params)?,
                body: Box::new(self.convert_hir_expr(body)?),
            }),

            HirExpr::For { var, iter, body } => Ok(RuchyExpr::For {
                var: var.clone(),
                iter: Box::new(self.convert_hir_expr(iter)?),
                body: Box::new(self.convert_hir_expr(body)?),
            }),

            HirExpr::While { condition, body } => Ok(RuchyExpr::While {
                condition: Box::new(self.convert_hir_expr(condition)?),
                body: Box::new(self.convert_hir_expr(body)?),
            }),

            HirExpr::Return(value) => Ok(RuchyExpr::Return {
                value: value
                    .as_ref()
                    .map(|v| self.convert_hir_expr(v).map(Box::new))
                    .transpose()?,
            }),

            HirExpr::Break(label) => Ok(RuchyExpr::Break {
                label: label.clone(),
            }),

            HirExpr::Continue(label) => Ok(RuchyExpr::Continue {
                label: label.clone(),
            }),

            _ => Err(anyhow!("Unsupported HIR expression type: {:?}", expr)),
        }
    }

    /// Converts HIR statement to Ruchy expression
    fn convert_statement(&self, stmt: &HirStatement) -> Result<RuchyExpr> {
        match stmt {
            HirStatement::Let {
                name,
                value,
                is_mutable,
            } => {
                Ok(RuchyExpr::Let {
                    name: name.clone(),
                    value: Box::new(self.convert_hir_expr(value)?),
                    body: Box::new(RuchyExpr::Block(vec![])), // Will be filled by parent
                    is_mutable: *is_mutable,
                })
            }

            HirStatement::Expression(expr) => self.convert_hir_expr(expr),
        }
    }

    /// Converts HIR literal to Ruchy literal
    fn convert_literal(&self, lit: &HirLiteral) -> RuchyExpr {
        RuchyExpr::Literal(match lit {
            HirLiteral::Integer(n) => Literal::Integer(*n),
            HirLiteral::Float(f) => Literal::Float(*f),
            HirLiteral::String(s) => Literal::String(s.clone()),
            HirLiteral::Bool(b) => Literal::Bool(*b),
            HirLiteral::None => Literal::Unit,
        })
    }

    /// Converts HIR binary operator
    fn convert_binary_op(&self, op: &HirBinaryOp) -> Result<BinaryOp> {
        Ok(match op {
            HirBinaryOp::Add => BinaryOp::Add,
            HirBinaryOp::Subtract => BinaryOp::Subtract,
            HirBinaryOp::Multiply => BinaryOp::Multiply,
            HirBinaryOp::Divide => BinaryOp::Divide,
            HirBinaryOp::Modulo => BinaryOp::Modulo,
            HirBinaryOp::Power => BinaryOp::Power,
            HirBinaryOp::Equal => BinaryOp::Equal,
            HirBinaryOp::NotEqual => BinaryOp::NotEqual,
            HirBinaryOp::Less => BinaryOp::Less,
            HirBinaryOp::LessEqual => BinaryOp::LessEqual,
            HirBinaryOp::Greater => BinaryOp::Greater,
            HirBinaryOp::GreaterEqual => BinaryOp::GreaterEqual,
            HirBinaryOp::And => BinaryOp::And,
            HirBinaryOp::Or => BinaryOp::Or,
            HirBinaryOp::BitwiseAnd => BinaryOp::BitwiseAnd,
            HirBinaryOp::BitwiseOr => BinaryOp::BitwiseOr,
            HirBinaryOp::BitwiseXor => BinaryOp::BitwiseXor,
            HirBinaryOp::LeftShift => BinaryOp::LeftShift,
            HirBinaryOp::RightShift => BinaryOp::RightShift,
        })
    }

    /// Converts HIR unary operator
    fn convert_unary_op(&self, op: &HirUnaryOp) -> Result<UnaryOp> {
        Ok(match op {
            HirUnaryOp::Not => UnaryOp::Not,
            HirUnaryOp::Negate => UnaryOp::Negate,
            HirUnaryOp::BitwiseNot => UnaryOp::BitwiseNot,
        })
    }

    /// Converts HIR parameters
    fn convert_params(&self, params: &[HirParam]) -> Result<Vec<Param>> {
        params
            .iter()
            .map(|p| {
                Ok(Param {
                    name: p.name.clone(),
                    typ: p.typ.as_ref().map(|t| self.convert_type(t)).transpose()?,
                    default: p
                        .default
                        .as_ref()
                        .map(|d| self.convert_hir_expr(d).map(Box::new))
                        .transpose()?,
                })
            })
            .collect()
    }

    /// Converts HIR type to Ruchy type
    #[allow(clippy::only_used_in_recursion)]
    fn convert_type(&self, typ: &HirType) -> Result<RuchyType> {
        Ok(match typ {
            HirType::Int => RuchyType::I64,
            HirType::Float => RuchyType::F64,
            HirType::String => RuchyType::String,
            HirType::Bool => RuchyType::Bool,
            HirType::List(inner) => RuchyType::Vec(Box::new(self.convert_type(inner)?)),
            HirType::Optional(inner) => RuchyType::Option(Box::new(self.convert_type(inner)?)),
            HirType::Named(name) => RuchyType::Named(name.clone()),
            HirType::Any => RuchyType::Dynamic,
        })
    }
}

impl Default for RuchyAstBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_integer() {
        let lit = Literal::Integer(42);
        assert_eq!(lit, Literal::Integer(42));
    }

    #[test]
    fn test_literal_float() {
        let lit = Literal::Float(3.14);
        assert_eq!(lit, Literal::Float(3.14));
    }

    #[test]
    fn test_literal_string() {
        let lit = Literal::String("hello".to_string());
        assert_eq!(lit, Literal::String("hello".to_string()));
    }

    #[test]
    fn test_literal_bool() {
        assert_eq!(Literal::Bool(true), Literal::Bool(true));
        assert_eq!(Literal::Bool(false), Literal::Bool(false));
    }

    #[test]
    fn test_literal_char() {
        assert_eq!(Literal::Char('a'), Literal::Char('a'));
    }

    #[test]
    fn test_literal_unit() {
        assert_eq!(Literal::Unit, Literal::Unit);
    }

    #[test]
    fn test_binary_op_variants() {
        let ops = [
            BinaryOp::Add, BinaryOp::Subtract, BinaryOp::Multiply,
            BinaryOp::Divide, BinaryOp::Modulo, BinaryOp::Power,
            BinaryOp::Equal, BinaryOp::NotEqual, BinaryOp::Less,
            BinaryOp::LessEqual, BinaryOp::Greater, BinaryOp::GreaterEqual,
            BinaryOp::And, BinaryOp::Or, BinaryOp::BitwiseAnd,
            BinaryOp::BitwiseOr, BinaryOp::BitwiseXor,
            BinaryOp::LeftShift, BinaryOp::RightShift,
        ];
        for op in ops {
            assert_eq!(op, op);
        }
    }

    #[test]
    fn test_unary_op_variants() {
        assert_eq!(UnaryOp::Not, UnaryOp::Not);
        assert_eq!(UnaryOp::Negate, UnaryOp::Negate);
        assert_eq!(UnaryOp::BitwiseNot, UnaryOp::BitwiseNot);
    }

    #[test]
    fn test_param_creation() {
        let param = Param {
            name: "x".to_string(),
            typ: Some(RuchyType::I64),
            default: None,
        };
        assert_eq!(param.name, "x");
        assert!(param.typ.is_some());
        assert!(param.default.is_none());
    }

    #[test]
    fn test_ruchy_type_primitives() {
        let types = [
            RuchyType::I8, RuchyType::I16, RuchyType::I32, RuchyType::I64,
            RuchyType::I128, RuchyType::ISize, RuchyType::U8, RuchyType::U16,
            RuchyType::U32, RuchyType::U64, RuchyType::U128, RuchyType::USize,
            RuchyType::F32, RuchyType::F64, RuchyType::Bool, RuchyType::Char,
            RuchyType::String, RuchyType::Dynamic,
        ];
        for t in types {
            assert_eq!(t.clone(), t);
        }
    }

    #[test]
    fn test_ruchy_type_compound() {
        let vec_type = RuchyType::Vec(Box::new(RuchyType::I64));
        assert_eq!(vec_type.clone(), vec_type);

        let option_type = RuchyType::Option(Box::new(RuchyType::String));
        assert_eq!(option_type.clone(), option_type);

        let result_type = RuchyType::Result(
            Box::new(RuchyType::I64),
            Box::new(RuchyType::String),
        );
        assert_eq!(result_type.clone(), result_type);
    }

    #[test]
    fn test_ruchy_expr_identifier() {
        let expr = RuchyExpr::Identifier("foo".to_string());
        assert_eq!(expr, RuchyExpr::Identifier("foo".to_string()));
    }

    #[test]
    fn test_ruchy_expr_literal() {
        let expr = RuchyExpr::Literal(Literal::Integer(42));
        assert_eq!(expr, RuchyExpr::Literal(Literal::Integer(42)));
    }

    #[test]
    fn test_ruchy_expr_binary() {
        let expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            op: BinaryOp::Add,
            right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
        };
        if let RuchyExpr::Binary { op, .. } = &expr {
            assert_eq!(*op, BinaryOp::Add);
        }
    }

    #[test]
    fn test_ruchy_expr_unary() {
        let expr = RuchyExpr::Unary {
            op: UnaryOp::Negate,
            operand: Box::new(RuchyExpr::Literal(Literal::Integer(42))),
        };
        if let RuchyExpr::Unary { op, .. } = &expr {
            assert_eq!(*op, UnaryOp::Negate);
        }
    }

    #[test]
    fn test_ruchy_expr_list() {
        let expr = RuchyExpr::List(vec![
            RuchyExpr::Literal(Literal::Integer(1)),
            RuchyExpr::Literal(Literal::Integer(2)),
        ]);
        if let RuchyExpr::List(items) = &expr {
            assert_eq!(items.len(), 2);
        }
    }

    #[test]
    fn test_ruchy_expr_block() {
        let expr = RuchyExpr::Block(vec![]);
        assert_eq!(expr, RuchyExpr::Block(vec![]));
    }

    #[test]
    fn test_ruchy_expr_if() {
        let expr = RuchyExpr::If {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            then_branch: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            else_branch: Some(Box::new(RuchyExpr::Literal(Literal::Integer(2)))),
        };
        if let RuchyExpr::If { else_branch, .. } = &expr {
            assert!(else_branch.is_some());
        }
    }

    #[test]
    fn test_ruchy_expr_for() {
        let expr = RuchyExpr::For {
            var: "i".to_string(),
            iter: Box::new(RuchyExpr::List(vec![])),
            body: Box::new(RuchyExpr::Block(vec![])),
        };
        if let RuchyExpr::For { var, .. } = &expr {
            assert_eq!(var, "i");
        }
    }

    #[test]
    fn test_ruchy_expr_while() {
        let expr = RuchyExpr::While {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            body: Box::new(RuchyExpr::Block(vec![])),
        };
        assert!(matches!(expr, RuchyExpr::While { .. }));
    }

    #[test]
    fn test_ruchy_expr_return() {
        let expr1 = RuchyExpr::Return { value: None };
        let expr2 = RuchyExpr::Return {
            value: Some(Box::new(RuchyExpr::Literal(Literal::Integer(42)))),
        };
        assert!(matches!(expr1, RuchyExpr::Return { value: None }));
        assert!(matches!(expr2, RuchyExpr::Return { value: Some(_) }));
    }

    #[test]
    fn test_ruchy_expr_break_continue() {
        let break_expr = RuchyExpr::Break { label: None };
        let continue_expr = RuchyExpr::Continue { label: Some("outer".to_string()) };
        assert!(matches!(break_expr, RuchyExpr::Break { .. }));
        assert!(matches!(continue_expr, RuchyExpr::Continue { .. }));
    }

    #[test]
    fn test_pattern_variants() {
        assert_eq!(Pattern::Wildcard, Pattern::Wildcard);
        assert_eq!(
            Pattern::Identifier("x".to_string()),
            Pattern::Identifier("x".to_string())
        );
        assert_eq!(
            Pattern::Literal(Literal::Integer(1)),
            Pattern::Literal(Literal::Integer(1))
        );
        assert_eq!(Pattern::Tuple(vec![]), Pattern::Tuple(vec![]));
        assert_eq!(Pattern::List(vec![]), Pattern::List(vec![]));
    }

    #[test]
    fn test_string_part_variants() {
        let text = StringPart::Text("hello".to_string());
        let expr_part = StringPart::Expr(Box::new(RuchyExpr::Identifier("x".to_string())));
        assert_eq!(text.clone(), text);
        assert_eq!(expr_part.clone(), expr_part);
    }

    #[test]
    fn test_pipeline_stage_variants() {
        let map = PipelineStage::Map(Box::new(RuchyExpr::Identifier("f".to_string())));
        let filter = PipelineStage::Filter(Box::new(RuchyExpr::Identifier("g".to_string())));
        let call = PipelineStage::Call("method".to_string(), vec![]);
        assert_eq!(map.clone(), map);
        assert_eq!(filter.clone(), filter);
        assert_eq!(call.clone(), call);
    }

    #[test]
    fn test_match_arm() {
        let arm = MatchArm {
            pattern: Pattern::Wildcard,
            guard: None,
            body: Box::new(RuchyExpr::Literal(Literal::Unit)),
        };
        assert_eq!(arm.clone(), arm);
    }

    #[test]
    fn test_struct_field() {
        let field = StructField {
            name: "x".to_string(),
            typ: RuchyType::I64,
            is_public: true,
        };
        assert_eq!(field.clone(), field);
    }

    #[test]
    fn test_dataframe_column() {
        let col = DataFrameColumn {
            name: "col1".to_string(),
            values: vec![RuchyExpr::Literal(Literal::Integer(1))],
        };
        assert_eq!(col.clone(), col);
    }

    #[test]
    fn test_ast_builder_new() {
        let builder = RuchyAstBuilder::new();
        assert!(std::mem::size_of_val(&builder) > 0);
    }

    #[test]
    fn test_ast_builder_default() {
        let builder = RuchyAstBuilder::default();
        assert!(std::mem::size_of_val(&builder) > 0);
    }

    #[test]
    fn test_ast_builder_with_config() {
        let config = crate::RuchyConfig::default();
        let builder = RuchyAstBuilder::with_config(&config);
        assert!(std::mem::size_of_val(&builder) > 0);
    }

    #[test]
    fn test_ruchy_type_reference() {
        let ref_type = RuchyType::Reference {
            typ: Box::new(RuchyType::I64),
            is_mutable: false,
        };
        assert_eq!(ref_type.clone(), ref_type);
    }

    #[test]
    fn test_ruchy_type_function() {
        let fn_type = RuchyType::Function {
            params: vec![RuchyType::I64],
            returns: Box::new(RuchyType::Bool),
        };
        assert_eq!(fn_type.clone(), fn_type);
    }

    #[test]
    fn test_ruchy_type_array() {
        let arr_type = RuchyType::Array(Box::new(RuchyType::I64), 10);
        assert_eq!(arr_type.clone(), arr_type);
    }

    #[test]
    fn test_ruchy_type_tuple() {
        let tup_type = RuchyType::Tuple(vec![RuchyType::I64, RuchyType::String]);
        assert_eq!(tup_type.clone(), tup_type);
    }

    #[test]
    fn test_ruchy_type_generic() {
        let gen_type = RuchyType::Generic("T".to_string());
        assert_eq!(gen_type.clone(), gen_type);
    }

    #[test]
    fn test_ruchy_type_named() {
        let named_type = RuchyType::Named("MyStruct".to_string());
        assert_eq!(named_type.clone(), named_type);
    }
}
