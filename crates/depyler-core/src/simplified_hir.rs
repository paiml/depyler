//! Simplified HIR for backend usage

use serde::{Deserialize, Serialize};

/// Simplified HIR structure for backend transpilation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hir {
    pub root: HirExpr,
    pub metadata: HirMetadata,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HirMetadata {
    pub source_file: Option<String>,
    pub module_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirExpr {
    Literal(HirLiteral),
    Identifier(String),
    Binary {
        left: Box<HirExpr>,
        op: HirBinaryOp,
        right: Box<HirExpr>,
    },
    Unary {
        op: HirUnaryOp,
        operand: Box<HirExpr>,
    },
    Call {
        func: Box<HirExpr>,
        args: Vec<HirExpr>,
    },
    If {
        condition: Box<HirExpr>,
        then_branch: Box<HirExpr>,
        else_branch: Option<Box<HirExpr>>,
    },
    Block(Vec<HirStatement>),
    List(Vec<HirExpr>),
    Function {
        name: String,
        params: Vec<HirParam>,
        body: Box<HirExpr>,
        is_async: bool,
        return_type: Option<HirType>,
    },
    Lambda {
        params: Vec<HirParam>,
        body: Box<HirExpr>,
    },
    For {
        var: String,
        iter: Box<HirExpr>,
        body: Box<HirExpr>,
    },
    While {
        condition: Box<HirExpr>,
        body: Box<HirExpr>,
    },
    Return(Option<Box<HirExpr>>),
    Break(Option<String>),
    Continue(Option<String>),
    Await(Box<HirExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirStatement {
    Let {
        name: String,
        value: Box<HirExpr>,
        is_mutable: bool,
    },
    Expression(Box<HirExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParam {
    pub name: String,
    pub typ: Option<HirType>,
    pub default: Option<Box<HirExpr>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirLiteral {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirBinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    And, Or,
    BitwiseAnd, BitwiseOr, BitwiseXor,
    LeftShift, RightShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirUnaryOp {
    Not,
    Negate,
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirType {
    Int,
    Float,
    String,
    Bool,
    List(Box<HirType>),
    Optional(Box<HirType>),
    Named(String),
    Any,
}