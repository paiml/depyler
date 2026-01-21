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
/// use depyler_core::hir::{HirModule, HirFunction, HirParam, Type, FunctionProperties};
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
///     constants: vec![],
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
    pub constants: Vec<HirConstant>,
}

/// Module-level constant declaration
///
/// Represents a constant value defined at module scope in Python,
/// which will be transpiled to Rust `const` or `pub const` declarations.
///
/// # Examples
///
/// ```python
/// MAX_SIZE = 100
/// PI: float = 3.14159
/// NAME = "MyApp"
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirConstant {
    pub name: String,
    pub value: HirExpr,
    pub type_annotation: Option<Type>,
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

/// DEPYLER-1136: Import with optional module-level alias
/// For `import xml.etree.ElementTree as ET`:
/// - module = "xml.etree.ElementTree"
/// - alias = Some("ET")
/// - items = []
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    /// Module-level alias for `import X as Y` patterns
    pub alias: Option<String>,
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
    /// DEPYLER-0739: Generic type parameters from Generic[T, U, ...] base class
    pub type_params: Vec<String>,
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
    /// DEPYLER-0477: True for varargs parameters (*args in Python)
    /// Transpiles to Vec<T> instead of regular parameter type
    pub is_vararg: bool,
}

impl HirParam {
    /// Create a required parameter (no default value)
    pub fn new(name: Symbol, ty: Type) -> Self {
        Self {
            name,
            ty,
            default: None,
            is_vararg: false, // DEPYLER-0477: Regular parameter
        }
    }

    /// Create a parameter with a default value
    pub fn with_default(name: Symbol, ty: Type, default: HirExpr) -> Self {
        Self {
            name,
            ty,
            default: Some(default),
            is_vararg: false, // DEPYLER-0477: Regular parameter
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
        /// DEPYLER-0188: True for `async with` statements
        is_async: bool,
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
    /// Block of statements (used for multi-target assignment: i = j = 0)
    /// DEPYLER-0614: Support Python chained assignment
    Block(Vec<HirStmt>),
    /// Nested function definition (inner functions)
    FunctionDef {
        name: Symbol,
        params: Box<SmallVec<[HirParam; 4]>>,
        ret_type: Type,
        body: Vec<HirStmt>,
        docstring: Option<String>,
    },
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
        /// DEPYLER-0364: Keyword arguments preserved from Python AST
        /// Format: Vec<(arg_name, value_expr)>
        /// Empty for calls without kwargs
        kwargs: Vec<(Symbol, HirExpr)>,
    },
    MethodCall {
        object: Box<HirExpr>,
        method: Symbol,
        args: Vec<HirExpr>,
        /// DEPYLER-0364: Keyword arguments preserved from Python AST
        /// Format: Vec<(arg_name, value_expr)>
        /// Empty for calls without kwargs
        kwargs: Vec<(Symbol, HirExpr)>,
    },
    /// DEPYLER-0188: Dynamic function call where the callee is an expression
    /// E.g., `handlers[name](args)` where `handlers[name]` is the callee
    /// Converts to Rust: `(handlers[&name])(args)` or `handlers.get(&name).unwrap()(args)`
    DynamicCall {
        callee: Box<HirExpr>,
        args: Vec<HirExpr>,
        kwargs: Vec<(Symbol, HirExpr)>,
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
    // List comprehension (DEPYLER-0504: Support multiple generators)
    ListComp {
        element: Box<HirExpr>,
        generators: Vec<HirComprehension>,
    },
    // Set comprehension (DEPYLER-0504: Support multiple generators)
    SetComp {
        element: Box<HirExpr>,
        generators: Vec<HirComprehension>,
    },
    // Dict comprehension (DEPYLER-0504: Support multiple generators)
    DictComp {
        key: Box<HirExpr>,
        value: Box<HirExpr>,
        generators: Vec<HirComprehension>,
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
    // DEPYLER-0502: reverse_expr supports dynamic boolean expressions, not just constants
    SortByKey {
        iterable: Box<HirExpr>,
        key_params: Vec<Symbol>,
        key_body: Box<HirExpr>,
        reverse_expr: Option<Box<HirExpr>>, // None = false (default), Some = dynamic expression
    },
    // Generator expression (Python: (x * 2 for x in range(5)))
    GeneratorExp {
        element: Box<HirExpr>,
        generators: Vec<HirComprehension>,
    },
    // Walrus operator / Named expression (Python: x := expr)
    // DEPYLER-0188: Assignment expression that evaluates to the assigned value
    // Rust equivalent: { let x = expr; x } or inline assignment in patterns
    NamedExpr {
        target: Symbol,
        value: Box<HirExpr>,
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

/// DEPYLER-0333: Exception scope tracking for try/except blocks
///
/// Tracks whether code is executing inside a try/except block to determine
/// appropriate error handling strategy:
/// - Unhandled: Exceptions propagate to caller (use ? operator or Result return)
/// - TryCaught: Exceptions are caught by handlers (use .unwrap_or() or control flow)
/// - Handler: Inside except/finally block (exceptions may propagate)
///
/// # Complexity
/// N/A (enum definition)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExceptionScope {
    /// Code outside any try/except block - exceptions propagate to caller
    /// Functions with unhandled exceptions should return Result<T, E>
    Unhandled,

    /// Code inside try block - exceptions are caught by handlers
    /// Contains list of exception types that are handled
    /// e.g., `try: ... except ValueError: ...` → TryCaught { handled_types: ["ValueError"] }
    TryCaught {
        /// Exception types caught by handlers (e.g., ["ValueError", "ZeroDivisionError"])
        /// Empty list means bare except clause (catches all)
        handled_types: Vec<String>,
    },

    /// Code inside except or finally block
    /// Exceptions in handlers may propagate to outer scope
    Handler,
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
    /// Unification variable for Hindley-Milner type inference
    /// Used during constraint solving, replaced with concrete types after inference
    UnificationVar(usize),
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
    /// Final type annotation from typing.Final[T] - marks constants
    Final(Box<Type>),
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

    /// DEPYLER-1154: Check if this type maps to a Rust Copy type
    ///
    /// Copy types don't need borrowing - they can be passed by value without
    /// moving ownership, eliminating unnecessary `&` references.
    ///
    /// # Rust Copy types
    /// - `i32`/`i64` (Int)
    /// - `f64` (Float)
    /// - `bool` (Bool)
    /// - `()` (None/unit)
    /// - Tuples where all elements are Copy
    ///
    /// # Non-Copy types (need borrowing or clone)
    /// - `String`
    /// - `Vec<T>`
    /// - `HashMap<K, V>`
    /// - `HashSet<T>`
    pub fn is_copy(&self) -> bool {
        match self {
            // Scalar types map to Copy Rust primitives
            Type::Int | Type::Float | Type::Bool | Type::None => true,

            // Tuples are Copy if all elements are Copy
            Type::Tuple(elems) => elems.iter().all(|t| t.is_copy()),

            // String, collections are NOT Copy
            Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => false,

            // Optional<Copy> is still Copy in Rust
            Type::Optional(inner) => inner.is_copy(),

            // Arrays of Copy elements are Copy
            Type::Array { element_type, .. } => element_type.is_copy(),

            // Union types are Copy only if ALL variants are Copy
            Type::Union(types) => types.iter().all(|t| t.is_copy()),

            // Custom types - assume not Copy unless we know otherwise
            Type::Custom(_) => false,

            // Unknown, Function, TypeVar, etc. - assume not Copy (defensive)
            _ => false,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn test_type_is_numeric() {
        assert!(Type::Int.is_numeric());
        assert!(Type::Float.is_numeric());
        assert!(!Type::String.is_numeric());
        assert!(!Type::Bool.is_numeric());
        assert!(!Type::None.is_numeric());
    }

    #[test]
    fn test_type_is_container() {
        assert!(Type::List(Box::new(Type::Int)).is_container());
        assert!(Type::Dict(Box::new(Type::String), Box::new(Type::Int)).is_container());
        assert!(Type::Tuple(vec![Type::Int, Type::String]).is_container());
        assert!(Type::Set(Box::new(Type::Int)).is_container());
        assert!(!Type::Int.is_container());
        assert!(!Type::String.is_container());
    }

    // ========================================================================
    // DEPYLER-1154: TYPE IS_COPY TESTS
    // ========================================================================

    #[test]
    fn test_DEPYLER_1154_type_is_copy_primitives() {
        // Primitive Copy types
        assert!(Type::Int.is_copy(), "Int should be Copy");
        assert!(Type::Float.is_copy(), "Float should be Copy");
        assert!(Type::Bool.is_copy(), "Bool should be Copy");
        assert!(Type::None.is_copy(), "None should be Copy");
    }

    #[test]
    fn test_DEPYLER_1154_type_is_copy_non_copy() {
        // Non-Copy types
        assert!(!Type::String.is_copy(), "String should NOT be Copy");
        assert!(
            !Type::List(Box::new(Type::Int)).is_copy(),
            "List should NOT be Copy"
        );
        assert!(
            !Type::Dict(Box::new(Type::String), Box::new(Type::Int)).is_copy(),
            "Dict should NOT be Copy"
        );
        assert!(
            !Type::Set(Box::new(Type::Int)).is_copy(),
            "Set should NOT be Copy"
        );
    }

    #[test]
    fn test_DEPYLER_1154_type_is_copy_tuple() {
        // Tuple of Copy types is Copy
        assert!(
            Type::Tuple(vec![Type::Int, Type::Float]).is_copy(),
            "Tuple(Int, Float) should be Copy"
        );
        // Tuple with non-Copy element is NOT Copy
        assert!(
            !Type::Tuple(vec![Type::Int, Type::String]).is_copy(),
            "Tuple(Int, String) should NOT be Copy"
        );
    }

    #[test]
    fn test_DEPYLER_1154_type_is_copy_optional() {
        // Optional<Copy> is Copy
        assert!(
            Type::Optional(Box::new(Type::Int)).is_copy(),
            "Optional<Int> should be Copy"
        );
        // Optional<non-Copy> is NOT Copy
        assert!(
            !Type::Optional(Box::new(Type::String)).is_copy(),
            "Optional<String> should NOT be Copy"
        );
    }

    #[test]
    fn test_DEPYLER_1154_type_is_copy_array() {
        // Array of Copy elements is Copy
        assert!(
            Type::Array {
                element_type: Box::new(Type::Int),
                size: ConstGeneric::Literal(10)
            }
            .is_copy(),
            "Array<Int> should be Copy"
        );
        // Array of non-Copy elements is NOT Copy
        assert!(
            !Type::Array {
                element_type: Box::new(Type::String),
                size: ConstGeneric::Literal(10)
            }
            .is_copy(),
            "Array<String> should NOT be Copy"
        );
    }

    #[test]
    fn test_type_equality() {
        assert_eq!(Type::Int, Type::Int);
        assert_eq!(Type::String, Type::String);
        assert_ne!(Type::Int, Type::Float);
    }

    #[test]
    fn test_type_clone() {
        let ty = Type::List(Box::new(Type::Int));
        let cloned = ty.clone();
        assert_eq!(ty, cloned);
    }

    #[test]
    fn test_hir_module_creation() {
        let module = HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };
        assert!(module.functions.is_empty());
        assert!(module.imports.is_empty());
    }

    #[test]
    fn test_hir_function_creation() {
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        assert_eq!(func.name, "test");
        assert_eq!(func.ret_type, Type::Int);
    }

    #[test]
    fn test_hir_param_new() {
        let param = HirParam::new("x".to_string(), Type::Int);
        assert_eq!(param.name, "x");
        assert_eq!(param.ty, Type::Int);
        assert!(param.default.is_none());
    }

    #[test]
    fn test_hir_param_with_default() {
        let param = HirParam::with_default("y".to_string(), Type::Int, HirExpr::Literal(Literal::Int(42)));
        assert_eq!(param.name, "y");
        assert!(param.default.is_some());
    }

    #[test]
    fn test_function_properties_default() {
        let props = FunctionProperties::default();
        assert!(!props.is_generator);
        assert!(!props.is_async);
        assert!(!props.is_pure);
        assert!(!props.can_fail);
    }

    #[test]
    fn test_literal_types() {
        assert_eq!(Literal::Int(42), Literal::Int(42));
        assert_eq!(Literal::Float(3.15), Literal::Float(3.15));
        assert_eq!(Literal::String("test".to_string()), Literal::String("test".to_string()));
        assert_eq!(Literal::Bool(true), Literal::Bool(true));
        assert_eq!(Literal::None, Literal::None);
    }

    #[test]
    fn test_assign_target_symbol() {
        let target = AssignTarget::Symbol("x".to_string());
        assert!(matches!(target, AssignTarget::Symbol(_)));
    }

    #[test]
    fn test_assign_target_tuple() {
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ]);
        if let AssignTarget::Tuple(items) = target {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected Tuple");
        }
    }

    #[test]
    fn test_binop_variants() {
        let ops = [
            BinOp::Add, BinOp::Sub, BinOp::Mul, BinOp::Div,
            BinOp::Mod, BinOp::Pow, BinOp::Eq, BinOp::NotEq,
            BinOp::Lt, BinOp::LtEq, BinOp::Gt, BinOp::GtEq,
            BinOp::And, BinOp::Or, BinOp::FloorDiv,
            BinOp::BitAnd, BinOp::BitOr, BinOp::BitXor,
            BinOp::LShift, BinOp::RShift, BinOp::In, BinOp::NotIn,
        ];
        for op in ops {
            assert_eq!(op, op);
        }
    }

    #[test]
    fn test_unaryop_variants() {
        assert_eq!(UnaryOp::Not, UnaryOp::Not);
        assert_eq!(UnaryOp::Neg, UnaryOp::Neg);
    }

    #[test]
    fn test_hir_expr_var() {
        let expr = HirExpr::Var("x".to_string());
        if let HirExpr::Var(name) = expr {
            assert_eq!(name, "x");
        } else {
            panic!("Expected Var");
        }
    }

    #[test]
    fn test_hir_expr_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        if let HirExpr::Literal(Literal::Int(n)) = expr {
            assert_eq!(n, 42);
        } else {
            panic!("Expected Int literal");
        }
    }

    #[test]
    fn test_hir_expr_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        if let HirExpr::Binary { op, .. } = expr {
            assert_eq!(op, BinOp::Add);
        } else {
            panic!("Expected Binary");
        }
    }

    #[test]
    fn test_hir_expr_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert!(matches!(expr, HirExpr::Unary { op: UnaryOp::Neg, .. }));
    }

    #[test]
    fn test_hir_expr_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            kwargs: vec![],
        };
        if let HirExpr::Call { func, args, .. } = expr {
            assert_eq!(func, "print");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected Call");
        }
    }

    #[test]
    fn test_hir_expr_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        if let HirExpr::List(items) = expr {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected List");
        }
    }

    #[test]
    fn test_hir_expr_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("test".to_string())),
        ]);
        if let HirExpr::Tuple(items) = expr {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected Tuple");
        }
    }

    #[test]
    fn test_hir_expr_dict() {
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);
        if let HirExpr::Dict(pairs) = expr {
            assert_eq!(pairs.len(), 1);
        } else {
            panic!("Expected Dict");
        }
    }

    #[test]
    fn test_hir_stmt_assign() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };
        assert!(matches!(stmt, HirStmt::Assign { .. }));
    }

    #[test]
    fn test_hir_stmt_return() {
        let stmt1 = HirStmt::Return(None);
        let stmt2 = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))));
        assert!(matches!(stmt1, HirStmt::Return(None)));
        assert!(matches!(stmt2, HirStmt::Return(Some(_))));
    }

    #[test]
    fn test_hir_stmt_if() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![],
            else_body: Some(vec![]),
        };
        if let HirStmt::If { else_body, .. } = stmt {
            assert!(else_body.is_some());
        } else {
            panic!("Expected If");
        }
    }

    #[test]
    fn test_hir_stmt_for() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![],
        };
        if let HirStmt::For { target, .. } = stmt {
            assert!(matches!(target, AssignTarget::Symbol(_)));
        } else {
            panic!("Expected For");
        }
    }

    #[test]
    fn test_hir_stmt_while() {
        let stmt = HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![],
        };
        assert!(matches!(stmt, HirStmt::While { .. }));
    }

    #[test]
    fn test_hir_stmt_expr() {
        let stmt = HirStmt::Expr(HirExpr::Literal(Literal::Int(42)));
        if let HirStmt::Expr(e) = stmt {
            assert!(matches!(e, HirExpr::Literal(Literal::Int(42))));
        } else {
            panic!("Expected Expr");
        }
    }

    #[test]
    fn test_import_and_import_item() {
        let import = Import {
            module: "os".to_string(),
            alias: None,
            items: vec![ImportItem::Named("path".to_string())],
        };
        assert_eq!(import.module, "os");
        assert_eq!(import.items.len(), 1);

        let aliased = ImportItem::Aliased {
            name: "path".to_string(),
            alias: "p".to_string(),
        };
        assert!(matches!(aliased, ImportItem::Aliased { .. }));
    }

    #[test]
    fn test_const_generic() {
        assert_eq!(ConstGeneric::Literal(5), ConstGeneric::Literal(5));
        assert_eq!(ConstGeneric::Parameter("N".to_string()), ConstGeneric::Parameter("N".to_string()));
        assert_eq!(ConstGeneric::Expression("N + 1".to_string()), ConstGeneric::Expression("N + 1".to_string()));
    }

    #[test]
    fn test_type_with_generics() {
        let ty = Type::Generic {
            base: "List".to_string(),
            params: vec![Type::Int],
        };
        if let Type::Generic { base, params } = ty {
            assert_eq!(base, "List");
            assert_eq!(params.len(), 1);
        } else {
            panic!("Expected Generic");
        }
    }

    #[test]
    fn test_type_union() {
        let ty = Type::Union(vec![Type::Int, Type::String]);
        if let Type::Union(types) = ty {
            assert_eq!(types.len(), 2);
        } else {
            panic!("Expected Union");
        }
    }

    #[test]
    fn test_type_optional() {
        let ty = Type::Optional(Box::new(Type::Int));
        assert!(matches!(ty, Type::Optional(_)));
    }

    #[test]
    fn test_type_function() {
        let ty = Type::Function {
            params: vec![Type::Int, Type::Int],
            ret: Box::new(Type::Int),
        };
        if let Type::Function { params, ret } = ty {
            assert_eq!(params.len(), 2);
            assert_eq!(*ret, Type::Int);
        } else {
            panic!("Expected Function");
        }
    }

    #[test]
    fn test_hir_expr_borrow() {
        let expr = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: false,
        };
        assert!(matches!(expr, HirExpr::Borrow { mutable: false, .. }));

        let mut_expr = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: true,
        };
        assert!(matches!(mut_expr, HirExpr::Borrow { mutable: true, .. }));
    }

    #[test]
    fn test_hir_expr_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(matches!(expr, HirExpr::Index { .. }));
    }

    #[test]
    fn test_hir_expr_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        if let HirExpr::Attribute { attr, .. } = expr {
            assert_eq!(attr, "field");
        } else {
            panic!("Expected Attribute");
        }
    }

    #[test]
    fn test_hir_expr_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        if let HirExpr::MethodCall { method, args, .. } = expr {
            assert_eq!(method, "append");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected MethodCall");
        }
    }
}
