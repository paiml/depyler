use depyler_annotations::TranspilationAnnotations;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

pub type Symbol = String;

/// Helper for creating parameter SmallVecs in tests
#[cfg(test)]
#[macro_export]
macro_rules! params {
    // Empty params
    () => {
        smallvec::smallvec![]
    };
    // Params with HirParam structs
    ($($param:expr),* $(,)?) => {
        smallvec::smallvec![$($param),*]
    };
}

/// Helper for creating a required parameter (no default)
#[cfg(test)]
#[macro_export]
macro_rules! param {
    ($name:expr, $ty:expr) => {
        $crate::hir::HirParam::new($name.to_string(), $ty)
    };
}

/// Helper for creating a parameter with a default value
#[cfg(test)]
#[macro_export]
macro_rules! param_with_default {
    ($name:expr, $ty:expr, $default:expr) => {
        $crate::hir::HirParam::with_default($name.to_string(), $ty, $default)
    };
}

/// High-level Intermediate Representation of a Python module
///
/// `HirModule` represents a complete Python module after semantic analysis and type inference.
/// It contains all the declarations (functions, classes, imports, etc.) in a form that's
/// optimized for transpilation to Rust.
///
/// # Examples
///
/// Creating a HIR module manually:
///
/// ```rust
/// use depyler_core::hir::{HirModule, HirFunction, Type, FunctionProperties};
/// use depyler_annotations::TranspilationAnnotations;
/// use smallvec::smallvec;
///
/// let function = HirFunction {
///     name: "example".to_string(),
///     params: smallvec![HirParam::new("x".to_string(), Type::Int)],
///     ret_type: Type::Int,
///     body: vec![],
///     properties: FunctionProperties::default(),
///     annotations: TranspilationAnnotations::default(),
///     docstring: Some("Example function".to_string()),
/// };
///
/// let module = HirModule {
///     functions: vec![function],
///     imports: vec![],
///     type_aliases: vec![],
///     protocols: vec![],
///     classes: vec![],
/// };
///
/// assert_eq!(module.functions.len(), 1);
/// assert_eq!(module.functions[0].name, "example");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirModule {
    pub functions: Vec<HirFunction>,
    pub imports: Vec<Import>,
    pub type_aliases: Vec<TypeAlias>,
    pub protocols: Vec<Protocol>,
    pub classes: Vec<HirClass>,
}

/// Simplified program representation for optimization passes
///
/// `HirProgram` is a streamlined version of `HirModule` used during optimization passes.
/// It focuses on the core elements needed for analysis and transformation.
///
/// # Examples
///
/// ```rust
/// use depyler_core::hir::{HirProgram, HirFunction, Type, FunctionProperties};
/// use depyler_annotations::TranspilationAnnotations;
/// use smallvec::smallvec;
///
/// let program = HirProgram {
///     functions: vec![
///         HirFunction {
///             name: "main".to_string(),
///             params: smallvec![],
///             ret_type: Type::None,
///             body: vec![],
///             properties: FunctionProperties::default(),
///             annotations: TranspilationAnnotations::default(),
///             docstring: None,
///         }
///     ],
///     classes: vec![],
///     imports: vec![],
/// };
///
/// assert_eq!(program.functions.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirProgram {
    pub functions: Vec<HirFunction>,
    pub classes: Vec<HirClass>,
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
pub struct TypeAlias {
    pub name: String,
    pub target_type: Type,
    pub is_newtype: bool, // true for NewType, false for simple alias
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Protocol {
    pub name: String,
    pub type_params: Vec<String>, // Generic type parameters like T, U
    pub methods: Vec<ProtocolMethod>,
    pub is_runtime_checkable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMethod {
    pub name: String,
    pub params: SmallVec<[HirParam; 4]>,
    pub ret_type: Type,
    pub is_optional: bool,
    pub has_default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirClass {
    pub name: String,
    pub base_classes: Vec<String>, // For inheritance, empty for now
    pub methods: Vec<HirMethod>,
    pub fields: Vec<HirField>,
    pub is_dataclass: bool,
    pub docstring: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirMethod {
    pub name: String,
    pub params: SmallVec<[HirParam; 4]>,
    pub ret_type: Type,
    pub body: Vec<HirStmt>,
    pub is_static: bool,
    pub is_classmethod: bool,
    pub is_property: bool,
    pub is_async: bool,
    pub docstring: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirField {
    pub name: String,
    pub field_type: Type,
    pub default_value: Option<HirExpr>,
    pub is_class_var: bool,
}

/// Function parameter with optional default value
///
/// Represents a single parameter in a function signature, including its name,
/// type, and an optional default value expression.
///
/// # Examples
///
/// ```rust
/// use depyler_core::hir::{HirParam, Type, HirExpr, Literal};
///
/// // Required parameter (no default)
/// let param = HirParam::new("x".to_string(), Type::Int);
/// assert_eq!(param.default, None);
///
/// // Parameter with default value
/// let param_with_default = HirParam::with_default(
///     "count".to_string(),
///     Type::Int,
///     HirExpr::Literal(Literal::Int(0)),
/// );
/// assert!(param_with_default.default.is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParam {
    pub name: Symbol,
    pub ty: Type,
    pub default: Option<HirExpr>,
}

impl HirParam {
    /// Create a required parameter (no default value)
    pub fn new(name: Symbol, ty: Type) -> Self {
        Self {
            name,
            ty,
            default: None,
        }
    }

    /// Create a parameter with a default value
    pub fn with_default(name: Symbol, ty: Type, default: HirExpr) -> Self {
        Self {
            name,
            ty,
            default: Some(default),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirFunction {
    pub name: Symbol,
    pub params: SmallVec<[HirParam; 4]>, // Most functions have < 4 params
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
    pub is_async: bool,
    pub is_generator: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssignTarget {
    /// Simple variable assignment: x = value
    Symbol(Symbol),
    /// Subscript assignment: x[key] = value
    Index {
        base: Box<HirExpr>,
        index: Box<HirExpr>,
    },
    /// Attribute assignment: x.attr = value (for future use)
    Attribute { value: Box<HirExpr>, attr: Symbol },
    /// Tuple unpacking: (a, b) = value or a, b = value
    Tuple(Vec<AssignTarget>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirStmt {
    Assign {
        target: AssignTarget,
        value: HirExpr,
        type_annotation: Option<Type>,
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
        target: AssignTarget,
        iter: HirExpr,
        body: Vec<HirStmt>,
    },
    Expr(HirExpr),
    // Basic exception support
    Raise {
        exception: Option<HirExpr>,
        cause: Option<HirExpr>,
    },
    Break {
        label: Option<Symbol>,
    },
    Continue {
        label: Option<Symbol>,
    },
    With {
        context: HirExpr,
        target: Option<Symbol>,
        body: Vec<HirStmt>,
    },
    Try {
        body: Vec<HirStmt>,
        handlers: Vec<ExceptHandler>,
        orelse: Option<Vec<HirStmt>>,
        finalbody: Option<Vec<HirStmt>>,
    },
    Assert {
        test: HirExpr,
        msg: Option<HirExpr>,
    },
    Pass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptHandler {
    pub exception_type: Option<String>,
    pub name: Option<Symbol>,
    pub body: Vec<HirStmt>,
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
    Set(Vec<HirExpr>),
    FrozenSet(Vec<HirExpr>),
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
    // Set comprehension
    SetComp {
        element: Box<HirExpr>,
        target: Symbol,
        iter: Box<HirExpr>,
        condition: Option<Box<HirExpr>>,
    },
    // Dict comprehension
    DictComp {
        key: Box<HirExpr>,
        value: Box<HirExpr>,
        target: Symbol,
        iter: Box<HirExpr>,
        condition: Option<Box<HirExpr>>,
    },
    // Lambda function
    Lambda {
        params: Vec<Symbol>,
        body: Box<HirExpr>,
    },
    // Await expression
    Await {
        value: Box<HirExpr>,
    },
    // F-string (format string)
    FString {
        parts: Vec<FStringPart>,
    },
    // Yield expression for generators
    Yield {
        value: Option<Box<HirExpr>>,
    },
    // Ternary/conditional expression (Python: x if cond else y)
    IfExpr {
        test: Box<HirExpr>,
        body: Box<HirExpr>,
        orelse: Box<HirExpr>,
    },
    // sorted() with key parameter (Python: sorted(iterable, key=lambda x: ..., reverse=True))
    SortByKey {
        iterable: Box<HirExpr>,
        key_params: Vec<Symbol>,
        key_body: Box<HirExpr>,
        reverse: bool,
    },
    // Generator expression (Python: (x * 2 for x in range(5)))
    GeneratorExp {
        element: Box<HirExpr>,
        generators: Vec<HirComprehension>,
    },
}

/// Comprehension generator (used in list/set/dict/generator comprehensions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirComprehension {
    pub target: Symbol,
    pub iter: Box<HirExpr>,
    pub conditions: Vec<HirExpr>,
}

/// Part of an f-string - either literal text or an expression to interpolate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FStringPart {
    /// Literal text in the f-string
    Literal(String),
    /// Expression to be formatted and inserted
    Expr(Box<HirExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstGeneric {
    /// Literal constant value (e.g., 5 in [T; 5])
    Literal(usize),
    /// Const generic parameter (e.g., N in [T; N])
    Parameter(String),
    /// Expression involving const generics (e.g., N + 1)
    Expression(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Set(Box<Type>),
    Optional(Box<Type>),
    Function {
        params: Vec<Type>,
        ret: Box<Type>,
    },
    Custom(String),
    /// Type variable for generics (e.g., T, U)
    TypeVar(String),
    /// Generic type with parameters (e.g., List<T>, Dict<K, V>)
    Generic {
        base: String,
        params: Vec<Type>,
    },
    /// Union type (e.g., Union[int, str])
    Union(Vec<Type>),
    /// Fixed-size array with const generic size (e.g., [T; N])
    Array {
        element_type: Box<Type>,
        size: ConstGeneric,
    },
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    pub fn is_container(&self) -> bool {
        matches!(
            self,
            Type::List(_) | Type::Dict(_, _) | Type::Tuple(_) | Type::Set(_) | Type::Array { .. }
        )
    }
}
