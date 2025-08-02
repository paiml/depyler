use depyler_annotations::TranspilationAnnotations;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

pub type Symbol = String;

/// Helper for creating parameter SmallVecs in tests
#[cfg(test)]
#[macro_export]
macro_rules! params {
    ($($param:expr),* $(,)?) => {
        smallvec::smallvec![$($param),*]
    };
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirModule {
    pub functions: Vec<HirFunction>,
    pub imports: Vec<Import>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportItem {
    Named(String),
    Aliased { name: String, alias: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirFunction {
    pub name: Symbol,
    pub params: SmallVec<[(Symbol, Type); 4]>, // Most functions have < 4 params
    pub ret_type: Type,
    pub body: Vec<HirStmt>,
    pub properties: FunctionProperties,
    pub annotations: TranspilationAnnotations,
    pub docstring: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionProperties {
    pub is_pure: bool,
    pub max_stack_depth: Option<usize>,
    pub always_terminates: bool,
    pub panic_free: bool,
    pub can_fail: bool,
    pub error_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirStmt {
    Assign {
        target: Symbol,
        value: HirExpr,
    },
    Return(Option<HirExpr>),
    If {
        condition: HirExpr,
        then_body: Vec<HirStmt>,
        else_body: Option<Vec<HirStmt>>,
    },
    While {
        condition: HirExpr,
        body: Vec<HirStmt>,
    },
    For {
        target: Symbol,
        iter: HirExpr,
        body: Vec<HirStmt>,
    },
    Expr(HirExpr),
    // Basic exception support
    Raise {
        exception: Option<HirExpr>,
        cause: Option<HirExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirExpr {
    Literal(Literal),
    Var(Symbol),
    Binary {
        op: BinOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<HirExpr>,
    },
    Call {
        func: Symbol,
        args: Vec<HirExpr>,
    },
    MethodCall {
        object: Box<HirExpr>,
        method: Symbol,
        args: Vec<HirExpr>,
    },
    Index {
        base: Box<HirExpr>,
        index: Box<HirExpr>,
    },
    Slice {
        base: Box<HirExpr>,
        start: Option<Box<HirExpr>>,
        stop: Option<Box<HirExpr>>,
        step: Option<Box<HirExpr>>,
    },
    Attribute {
        value: Box<HirExpr>,
        attr: Symbol,
    },
    List(Vec<HirExpr>),
    Dict(Vec<(HirExpr, HirExpr)>),
    Tuple(Vec<HirExpr>),
    // Ownership hints from analysis
    Borrow {
        expr: Box<HirExpr>,
        mutable: bool,
    },
    // List comprehension
    ListComp {
        element: Box<HirExpr>,
        target: Symbol,
        iter: Box<HirExpr>,
        condition: Option<Box<HirExpr>>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
    In,
    NotIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
    Pos,
    BitNot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Unknown,
    Int,
    Float,
    String,
    Bool,
    None,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Optional(Box<Type>),
    Function { params: Vec<Type>, ret: Box<Type> },
    Custom(String),
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    pub fn is_container(&self) -> bool {
        matches!(self, Type::List(_) | Type::Dict(_, _) | Type::Tuple(_))
    }
}
