#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::{ConstGeneric, Type as PythonType};
use crate::trace_decision;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntWidth {
    I32,
    I64,
    ISize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StringStrategy {
    AlwaysOwned,    // String everywhere (safe, simple)
    InferBorrowing, // &str where possible (V1.1)
    CowByDefault,   // Cow<'static, str> (V1.2)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMapper {
    pub width_preference: IntWidth,
    pub string_type: StringStrategy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RustType {
    Primitive(PrimitiveType),
    String,
    Str {
        lifetime: Option<String>,
    },
    Cow {
        lifetime: String,
    },
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    HashSet(Box<RustType>),
    Option(Box<RustType>),
    Result(Box<RustType>, Box<RustType>),
    Reference {
        lifetime: Option<String>,
        mutable: bool,
        inner: Box<RustType>,
    },
    Tuple(Vec<RustType>),
    Unit,
    Custom(String),
    Unsupported(String),
    /// Type parameter for generics
    TypeParam(String),
    /// Generic type with parameters
    Generic {
        base: String,
        params: Vec<RustType>,
    },
    /// Enum type for union types
    Enum {
        name: String,
        variants: Vec<(String, RustType)>,
    },
    /// Fixed-size array type
    Array {
        element_type: Box<RustType>,
        size: RustConstGeneric,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RustConstGeneric {
    /// Literal constant value (e.g., 5 in [T; 5])
    Literal(usize),
    /// Const generic parameter (e.g., N in [T; N])
    Parameter(String),
    /// Expression involving const generics (e.g., N + 1)
    Expression(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Bool,
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
}

impl Default for TypeMapper {
    fn default() -> Self {
        Self {
            width_preference: IntWidth::I32,
            string_type: StringStrategy::AlwaysOwned,
        }
    }
}

impl TypeMapper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_i64(mut self) -> Self {
        self.width_preference = IntWidth::I64;
        self
    }

    pub fn with_string_strategy(mut self, strategy: StringStrategy) -> Self {
        self.string_type = strategy;
        self
    }

    pub fn map_type(&self, py_type: &PythonType) -> RustType {
        // CITL: Trace Python→Rust type mapping decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "python_to_rust_type",
            chosen = &format!("{:?}", py_type),
            alternatives = ["primitive", "owned", "borrowed", "option", "result"],
            confidence = 0.90
        );

        match py_type {
            // DEPYLER-0264: Map Unknown to generic type T instead of serde_json::Value
            // DEPYLER-0705: Use generic T for single-shot compilation without external deps
            PythonType::Unknown => RustType::TypeParam("T".to_string()),
            PythonType::Int => RustType::Primitive(match self.width_preference {
                IntWidth::I32 => PrimitiveType::I32,
                IntWidth::I64 => PrimitiveType::I64,
                IntWidth::ISize => PrimitiveType::ISize,
            }),
            PythonType::Float => RustType::Primitive(PrimitiveType::F64),
            PythonType::String => match self.string_type {
                StringStrategy::AlwaysOwned => RustType::String,
                StringStrategy::InferBorrowing => RustType::String, // V1: Always owned
                StringStrategy::CowByDefault => RustType::String,   // V1: Always owned
            },
            PythonType::Bool => RustType::Primitive(PrimitiveType::Bool),
            PythonType::None => RustType::Unit,
            PythonType::List(inner) => RustType::Vec(Box::new(self.map_type(inner))),
            PythonType::Dict(k, v) => {
                RustType::HashMap(Box::new(self.map_type(k)), Box::new(self.map_type(v)))
            }
            PythonType::Tuple(types) => {
                let rust_types = types.iter().map(|t| self.map_type(t)).collect();
                RustType::Tuple(rust_types)
            }
            PythonType::Optional(inner) => RustType::Option(Box::new(self.map_type(inner))),
            PythonType::Final(inner) => self.map_type(inner), // Unwrap Final to get the actual type
            PythonType::Function { params: _, ret: _ } => {
                // For V1, we don't map function types directly
                RustType::Unsupported("function".to_string())
            }
            PythonType::Custom(name) => {
                // Check if this is a single uppercase letter (type parameter)
                if name.len() == 1 && name.chars().next().unwrap().is_uppercase() {
                    RustType::TypeParam(name.clone())
                } else {
                    // Handle common typing imports when used without parameters
                    // DEPYLER-0718: Use serde_json::Value for bare Dict/List to enable
                    // single-shot compilation. TypeParam("V"/"T") requires generics declaration
                    // which isn't generated, causing E0412 "cannot find type" errors.
                    match name.as_str() {
                        "Dict" => RustType::HashMap(
                            Box::new(RustType::String), // Default to String keys
                            Box::new(RustType::Custom("serde_json::Value".to_string())), // DEPYLER-0718: Use Value, not TypeParam
                        ),
                        // DEPYLER-0609: Handle both "List" (typing import) and "list" (builtin)
                        // DEPYLER-0718: Use serde_json::Value for bare List (no type params)
                        "List" | "list" => RustType::Vec(Box::new(RustType::Custom(
                            "serde_json::Value".to_string(),
                        ))),
                        "Set" => RustType::HashSet(Box::new(RustType::String)),
                        // DEPYLER-0379: Handle generic tuple annotation
                        // Python `-> tuple` (without type parameters) maps to empty Rust tuple `()`
                        // This is a fallback - ideally type should be inferred from return value
                        "tuple" => RustType::Tuple(vec![]),
                        // DEPYLER-0525/DEPYLER-0608: File-like objects that implement Write trait
                        // Map to mutable reference to impl Write for parameter positions
                        // This allows both File and Stdout to be passed as arguments
                        "File" => RustType::Reference {
                            lifetime: None,
                            mutable: true,
                            inner: Box::new(RustType::Custom("impl std::io::Write".to_string())),
                        },
                        // DEPYLER-0580: argparse.Namespace maps to Args struct in clap
                        // Python: def cmd_step(args: argparse.Namespace) → Rust: fn cmd_step(args: Args)
                        "Namespace" | "argparse.Namespace" => {
                            RustType::Custom("Args".to_string())
                        }
                        // DEPYLER-0584: Python bytes type maps to Vec<u8>
                        "bytes" => RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::U8))),
                        // DEPYLER-0674: Python bytearray type maps to Vec<u8>
                        "bytearray" => RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::U8))),
                        // DEPYLER-0584: Python Callable type maps to Box<dyn Fn>
                        "Callable" | "typing.Callable" | "callable" => {
                            RustType::Custom("Box<dyn Fn()>".to_string())
                        }
                        // DEPYLER-0589: Python Any type maps to serde_json::Value
                        // Both typing.Any and bare 'any' need to be handled
                        "Any" | "typing.Any" | "any" => {
                            RustType::Custom("serde_json::Value".to_string())
                        }
                        // DEPYLER-0628: Python object type maps to serde_json::Value
                        // Python's base object type needs dynamic typing in Rust
                        "object" | "builtins.object" => {
                            RustType::Custom("serde_json::Value".to_string())
                        }
                        // DEPYLER-0592: Python datetime module types map to chrono types
                        // This fixes E0412 errors where 'date', 'datetime', 'time' are unknown types
                        "date" | "datetime.date" => {
                            RustType::Custom("chrono::NaiveDate".to_string())
                        }
                        "datetime" | "datetime.datetime" => {
                            RustType::Custom("chrono::NaiveDateTime".to_string())
                        }
                        "time" | "datetime.time" => {
                            RustType::Custom("chrono::NaiveTime".to_string())
                        }
                        "timedelta" | "datetime.timedelta" => {
                            RustType::Custom("chrono::Duration".to_string())
                        }
                        // DEPYLER-197: Python Path types map to std::path::PathBuf
                        // pathlib.Path → PathBuf (owned path, can be modified)
                        // Path alone also maps to PathBuf for consistency
                        "Path" | "pathlib.Path" | "PurePath" | "pathlib.PurePath" => {
                            RustType::Custom("std::path::PathBuf".to_string())
                        }
                        // DEPYLER-0597: Python exception types map to Rust error types
                        // OSError maps to std::io::Error for file/system operations
                        "OSError" | "IOError" | "FileNotFoundError" | "PermissionError" => {
                            RustType::Custom("std::io::Error".to_string())
                        }
                        // General Python exceptions map to Box<dyn std::error::Error>
                        // Using Box<dyn Error> since it doesn't require external crates
                        "Exception" | "BaseException" | "ValueError" | "TypeError"
                        | "KeyError" | "IndexError" | "RuntimeError" | "AttributeError"
                        | "NotImplementedError" | "AssertionError" | "StopIteration"
                        | "ZeroDivisionError" | "OverflowError" | "ArithmeticError" => {
                            RustType::Custom("Box<dyn std::error::Error>".to_string())
                        }
                        _ => RustType::Custom(name.clone()),
                    }
                }
            }
            PythonType::TypeVar(name) => RustType::TypeParam(name.clone()),
            PythonType::Generic { base, params } => {
                // Map generic types like MyClass<T> to appropriate Rust types
                match base.as_str() {
                    "List" if params.len() == 1 => {
                        RustType::Vec(Box::new(self.map_type(&params[0])))
                    }
                    "Dict" if params.len() == 2 => RustType::HashMap(
                        Box::new(self.map_type(&params[0])),
                        Box::new(self.map_type(&params[1])),
                    ),
                    // DEPYLER-0188: Generator[YieldType, SendType, ReturnType] -> impl Iterator<Item=YieldType>
                    // Python generators map to Rust iterators for idiomatic code
                    "Generator" if !params.is_empty() => {
                        let yield_type = self.map_type(&params[0]);
                        RustType::Custom(format!("impl Iterator<Item={}>", yield_type.to_rust_string()))
                    }
                    // DEPYLER-0188: Iterator[YieldType] -> impl Iterator<Item=YieldType>
                    "Iterator" if params.len() == 1 => {
                        let yield_type = self.map_type(&params[0]);
                        RustType::Custom(format!("impl Iterator<Item={}>", yield_type.to_rust_string()))
                    }
                    // DEPYLER-0188: Iterable[T] -> impl IntoIterator<Item=T>
                    "Iterable" if params.len() == 1 => {
                        let item_type = self.map_type(&params[0]);
                        RustType::Custom(format!("impl IntoIterator<Item={}>", item_type.to_rust_string()))
                    }
                    // DEPYLER-197: Callable[[T1, T2, ...], R] -> Box<dyn Fn(T1, T2, ...) -> R>
                    // Python Callable types map to boxed trait objects for dynamic dispatch
                    // This enables type aliases like `EventHandler = Callable[[str], None]`
                    "Callable" if params.len() == 2 => {
                        // params[0] is the parameter list type (may be Tuple, List, or single type)
                        // params[1] is the return type
                        let param_types = match &params[0] {
                            PythonType::Tuple(inner) => {
                                inner.iter().map(|t| self.map_type(t).to_rust_string()).collect::<Vec<_>>()
                            }
                            PythonType::List(inner) => {
                                // Single param list: [[T]] -> [T]
                                vec![self.map_type(inner).to_rust_string()]
                            }
                            PythonType::None => vec![], // Empty param list
                            PythonType::Unknown => vec![], // Empty param list from []
                            _ => vec![self.map_type(&params[0]).to_rust_string()],
                        };
                        let return_type = self.map_type(&params[1]);
                        let return_str = return_type.to_rust_string();

                        // Format: Box<dyn Fn(T1, T2) -> R> or Box<dyn Fn()> for None return
                        let fn_str = if return_str == "()" || matches!(params[1], PythonType::None) {
                            format!("Box<dyn Fn({})>", param_types.join(", "))
                        } else {
                            format!("Box<dyn Fn({}) -> {}>", param_types.join(", "), return_str)
                        };
                        RustType::Custom(fn_str)
                    }
                    "Callable" if params.is_empty() => {
                        // Bare Callable without parameters -> Box<dyn Fn()>
                        RustType::Custom("Box<dyn Fn()>".to_string())
                    }
                    _ => RustType::Generic {
                        base: base.clone(),
                        params: params.iter().map(|t| self.map_type(t)).collect(),
                    },
                }
            }
            PythonType::Union(types) => {
                // For now, map Union to an enum or use dynamic typing
                if types.len() == 2 && types.iter().any(|t| matches!(t, PythonType::None)) {
                    // Union[T, None] is Optional[T]
                    let non_none = types
                        .iter()
                        .find(|t| !matches!(t, PythonType::None))
                        .unwrap();
                    RustType::Option(Box::new(self.map_type(non_none)))
                } else {
                    // For non-optional unions, we'll need to generate an enum
                    // The actual enum will be generated during code generation
                    RustType::Enum {
                        name: "UnionType".to_string(), // Placeholder, will be replaced
                        variants: types
                            .iter()
                            .enumerate()
                            .map(|(i, t)| {
                                let variant_name = match t {
                                    PythonType::Int => "Integer".to_string(),
                                    PythonType::Float => "Float".to_string(),
                                    PythonType::String => "Text".to_string(),
                                    PythonType::Bool => "Boolean".to_string(),
                                    PythonType::None => "None".to_string(),
                                    _ => format!("Variant{}", i),
                                };
                                (variant_name, self.map_type(t))
                            })
                            .collect(),
                    }
                }
            }
            PythonType::Array { element_type, size } => RustType::Array {
                element_type: Box::new(self.map_type(element_type)),
                size: self.map_const_generic(size),
            },
            PythonType::Set(inner) => RustType::HashSet(Box::new(self.map_type(inner))),
            PythonType::UnificationVar(id) => {
                // DEPYLER-0692: UnificationVar indicates incomplete type inference
                // Instead of panicking, fall back to a generic type
                // This allows compilation to proceed even when inference is incomplete
                tracing::warn!(
                    "UnificationVar({}) encountered in type mapper. Falling back to serde_json::Value.",
                    id
                );
                RustType::Custom("serde_json::Value".to_string())
            }
        }
    }

    pub fn map_return_type(&self, py_type: &PythonType) -> RustType {
        // CITL: Trace return type mapping decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "return_type_mapping",
            chosen = &format!("{:?}", py_type),
            alternatives = ["unit", "result", "option", "owned"],
            confidence = 0.92
        );

        match py_type {
            PythonType::None => RustType::Unit,
            PythonType::Unknown => RustType::Unit, // Functions without return annotation implicitly return None/()
            _ => self.map_type(py_type),
        }
    }

    pub fn needs_reference(&self, rust_type: &RustType) -> bool {
        // CITL: Trace reference need decision
        trace_decision!(
            category = DecisionCategory::BorrowStrategy,
            name = "needs_reference",
            chosen = &format!("{:?}", rust_type),
            alternatives = ["by_value", "by_ref", "by_mut_ref"],
            confidence = 0.88
        );
        match rust_type {
            RustType::String => false, // V1: Always owned
            RustType::Vec(_) | RustType::HashMap(_, _) | RustType::HashSet(_) => true,
            RustType::Primitive(_) => false,
            RustType::Array { .. } => true, // Arrays need references for large sizes
            _ => false,
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn can_copy(&self, rust_type: &RustType) -> bool {
        match rust_type {
            RustType::Primitive(_) | RustType::Unit => true,
            RustType::Tuple(types) => types.iter().all(|t| self.can_copy(t)),
            RustType::Array { element_type, size } => {
                // Arrays are copy if elements are copy and size is reasonable
                match size {
                    RustConstGeneric::Literal(n) if *n <= 32 => self.can_copy(element_type),
                    _ => false, // Large or unknown size arrays are not Copy
                }
            }
            _ => false,
        }
    }

    /// Map a const generic from HIR to Rust representation
    pub fn map_const_generic(&self, const_generic: &ConstGeneric) -> RustConstGeneric {
        match const_generic {
            ConstGeneric::Literal(value) => RustConstGeneric::Literal(*value),
            ConstGeneric::Parameter(name) => RustConstGeneric::Parameter(name.clone()),
            ConstGeneric::Expression(expr) => RustConstGeneric::Expression(expr.clone()),
        }
    }
}

impl RustType {
    pub fn to_rust_string(&self) -> String {
        match self {
            RustType::Primitive(p) => p.to_rust_string().to_string(),
            RustType::String => "String".to_string(),
            RustType::Str { lifetime } => {
                if let Some(lt) = lifetime {
                    format!("&{lt} str")
                } else {
                    "&str".to_string()
                }
            }
            RustType::Cow { lifetime } => format!("Cow<{lifetime}, str>"),
            RustType::Vec(inner) => format!("Vec<{}>", inner.to_rust_string()),
            RustType::HashMap(k, v) => {
                format!("HashMap<{}, {}>", k.to_rust_string(), v.to_rust_string())
            }
            RustType::HashSet(inner) => format!("HashSet<{}>", inner.to_rust_string()),
            RustType::Option(inner) => format!("Option<{}>", inner.to_rust_string()),
            RustType::Result(ok, err) => {
                format!("Result<{}, {}>", ok.to_rust_string(), err.to_rust_string())
            }
            RustType::Reference {
                lifetime,
                mutable,
                inner,
            } => {
                let mut_str = if *mutable { "mut " } else { "" };
                if let Some(lt) = lifetime {
                    format!("&{} {}{}", lt, mut_str, inner.to_rust_string())
                } else {
                    format!("&{}{}", mut_str, inner.to_rust_string())
                }
            }
            RustType::Tuple(types) => {
                if types.is_empty() {
                    "()".to_string()
                } else {
                    let type_strs: Vec<String> = types.iter().map(|t| t.to_rust_string()).collect();
                    format!("({})", type_strs.join(", "))
                }
            }
            RustType::Unit => "()".to_string(),
            RustType::Custom(name) => name.clone(),
            RustType::Unsupported(desc) => format!("/* unsupported: {desc} */"),
            RustType::TypeParam(name) => name.clone(),
            RustType::Generic { base, params } => {
                let param_strs: Vec<String> = params.iter().map(|p| p.to_rust_string()).collect();
                format!("{}<{}>", base, param_strs.join(", "))
            }
            RustType::Enum { name, .. } => name.clone(),
            RustType::Array { element_type, size } => {
                format!(
                    "[{}; {}]",
                    element_type.to_rust_string(),
                    size.to_rust_string()
                )
            }
        }
    }
}

impl RustConstGeneric {
    pub fn to_rust_string(&self) -> String {
        match self {
            RustConstGeneric::Literal(value) => value.to_string(),
            RustConstGeneric::Parameter(name) => name.clone(),
            RustConstGeneric::Expression(expr) => expr.clone(),
        }
    }
}

impl PrimitiveType {
    pub fn to_rust_string(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "bool",
            PrimitiveType::I8 => "i8",
            PrimitiveType::I16 => "i16",
            PrimitiveType::I32 => "i32",
            PrimitiveType::I64 => "i64",
            PrimitiveType::I128 => "i128",
            PrimitiveType::ISize => "isize",
            PrimitiveType::U8 => "u8",
            PrimitiveType::U16 => "u16",
            PrimitiveType::U32 => "u32",
            PrimitiveType::U64 => "u64",
            PrimitiveType::U128 => "u128",
            PrimitiveType::USize => "usize",
            PrimitiveType::F32 => "f32",
            PrimitiveType::F64 => "f64",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_type_mapper() {
        let mapper = TypeMapper::default();
        assert_eq!(mapper.width_preference, IntWidth::I32);
        assert_eq!(mapper.string_type, StringStrategy::AlwaysOwned);
    }

    #[test]
    fn test_type_mapper_creation() {
        let mapper = TypeMapper::new();
        assert_eq!(mapper.width_preference, IntWidth::I32);

        let mapper_i64 = mapper.with_i64();
        assert_eq!(mapper_i64.width_preference, IntWidth::I64);

        let mapper_borrowing =
            TypeMapper::new().with_string_strategy(StringStrategy::InferBorrowing);
        assert_eq!(mapper_borrowing.string_type, StringStrategy::InferBorrowing);
    }

    #[test]
    fn test_basic_type_mapping() {
        let mapper = TypeMapper::new();

        assert_eq!(
            mapper.map_type(&PythonType::Int),
            RustType::Primitive(PrimitiveType::I32)
        );

        assert_eq!(
            mapper.map_type(&PythonType::Float),
            RustType::Primitive(PrimitiveType::F64)
        );

        assert_eq!(mapper.map_type(&PythonType::String), RustType::String);

        assert_eq!(
            mapper.map_type(&PythonType::Bool),
            RustType::Primitive(PrimitiveType::Bool)
        );

        assert_eq!(mapper.map_type(&PythonType::None), RustType::Unit);
    }

    #[test]
    fn test_width_preference() {
        let mapper_i32 = TypeMapper::new();
        assert_eq!(
            mapper_i32.map_type(&PythonType::Int),
            RustType::Primitive(PrimitiveType::I32)
        );

        let mapper_i64 = TypeMapper::new().with_i64();
        assert_eq!(
            mapper_i64.map_type(&PythonType::Int),
            RustType::Primitive(PrimitiveType::I64)
        );

        let mut mapper_isize = TypeMapper::new();
        mapper_isize.width_preference = IntWidth::ISize;
        assert_eq!(
            mapper_isize.map_type(&PythonType::Int),
            RustType::Primitive(PrimitiveType::ISize)
        );
    }

    #[test]
    fn test_complex_type_mapping() {
        let mapper = TypeMapper::new();

        // List[int]
        let list_type = PythonType::List(Box::new(PythonType::Int));
        assert_eq!(
            mapper.map_type(&list_type),
            RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)))
        );

        // Optional[str]
        let optional_type = PythonType::Optional(Box::new(PythonType::String));
        assert_eq!(
            mapper.map_type(&optional_type),
            RustType::Option(Box::new(RustType::String))
        );

        // Dict[str, int]
        let dict_type = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::Int));
        assert_eq!(
            mapper.map_type(&dict_type),
            RustType::HashMap(
                Box::new(RustType::String),
                Box::new(RustType::Primitive(PrimitiveType::I32))
            )
        );
    }

    #[test]
    fn test_tuple_mapping() {
        let mapper = TypeMapper::new();

        let tuple_type =
            PythonType::Tuple(vec![PythonType::Int, PythonType::String, PythonType::Bool]);

        if let RustType::Tuple(types) = mapper.map_type(&tuple_type) {
            assert_eq!(types.len(), 3);
            assert_eq!(types[0], RustType::Primitive(PrimitiveType::I32));
            assert_eq!(types[1], RustType::String);
            assert_eq!(types[2], RustType::Primitive(PrimitiveType::Bool));
        } else {
            panic!("Expected tuple type");
        }
    }

    #[test]
    fn test_return_type_mapping() {
        let mapper = TypeMapper::new();

        assert_eq!(mapper.map_return_type(&PythonType::None), RustType::Unit);

        assert_eq!(
            mapper.map_return_type(&PythonType::Int),
            RustType::Primitive(PrimitiveType::I32)
        );
    }

    #[test]
    fn test_needs_reference() {
        let mapper = TypeMapper::new();

        assert!(!mapper.needs_reference(&RustType::String));
        assert!(!mapper.needs_reference(&RustType::Primitive(PrimitiveType::I32)));
        assert!(mapper.needs_reference(&RustType::Vec(Box::new(RustType::String))));
        assert!(mapper.needs_reference(&RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(PrimitiveType::I32))
        )));
    }

    #[test]
    fn test_can_copy() {
        let mapper = TypeMapper::new();

        assert!(mapper.can_copy(&RustType::Primitive(PrimitiveType::I32)));
        assert!(mapper.can_copy(&RustType::Unit));
        assert!(!mapper.can_copy(&RustType::String));
        assert!(
            !mapper.can_copy(&RustType::Vec(Box::new(RustType::Primitive(
                PrimitiveType::I32
            ))))
        );

        // Tuple of copyable types
        let copyable_tuple = RustType::Tuple(vec![
            RustType::Primitive(PrimitiveType::I32),
            RustType::Primitive(PrimitiveType::Bool),
        ]);
        assert!(mapper.can_copy(&copyable_tuple));

        // Tuple with non-copyable type
        let non_copyable_tuple = RustType::Tuple(vec![
            RustType::Primitive(PrimitiveType::I32),
            RustType::String,
        ]);
        assert!(!mapper.can_copy(&non_copyable_tuple));
    }

    #[test]
    fn test_rust_type_to_string() {
        assert_eq!(
            RustType::Primitive(PrimitiveType::I32).to_rust_string(),
            "i32"
        );
        assert_eq!(RustType::String.to_rust_string(), "String");
        assert_eq!(RustType::Unit.to_rust_string(), "()");

        let vec_type = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
        assert_eq!(vec_type.to_rust_string(), "Vec<i32>");

        let optional_type = RustType::Option(Box::new(RustType::String));
        assert_eq!(optional_type.to_rust_string(), "Option<String>");

        let hashmap_type = RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(PrimitiveType::I32)),
        );
        assert_eq!(hashmap_type.to_rust_string(), "HashMap<String, i32>");

        let tuple_type = RustType::Tuple(vec![
            RustType::Primitive(PrimitiveType::I32),
            RustType::String,
        ]);
        assert_eq!(tuple_type.to_rust_string(), "(i32, String)");

        let empty_tuple = RustType::Tuple(vec![]);
        assert_eq!(empty_tuple.to_rust_string(), "()");
    }

    #[test]
    fn test_primitive_type_to_string() {
        assert_eq!(PrimitiveType::Bool.to_rust_string(), "bool");
        assert_eq!(PrimitiveType::I32.to_rust_string(), "i32");
        assert_eq!(PrimitiveType::I64.to_rust_string(), "i64");
        assert_eq!(PrimitiveType::F64.to_rust_string(), "f64");
        assert_eq!(PrimitiveType::ISize.to_rust_string(), "isize");
    }

    #[test]
    fn test_custom_and_unsupported_types() {
        let mapper = TypeMapper::new();

        let custom_type = PythonType::Custom("MyClass".to_string());
        assert_eq!(
            mapper.map_type(&custom_type),
            RustType::Custom("MyClass".to_string())
        );

        assert_eq!(
            RustType::Custom("MyClass".to_string()).to_rust_string(),
            "MyClass"
        );

        // DEPYLER-0705: Unknown type now maps to TypeParam("T") for single-shot compilation
        let unknown_type = PythonType::Unknown;
        if let RustType::TypeParam(name) = mapper.map_type(&unknown_type) {
            assert_eq!(name, "T");
        } else {
            panic!("Expected TypeParam(T) for unknown type");
        }
    }

    #[test]
    fn test_function_type_unsupported() {
        let mapper = TypeMapper::new();

        let func_type = PythonType::Function {
            params: vec![PythonType::Int],
            ret: Box::new(PythonType::String),
        };

        if let RustType::Unsupported(desc) = mapper.map_type(&func_type) {
            assert_eq!(desc, "function");
        } else {
            panic!("Expected unsupported function type");
        }
    }

    #[test]
    fn test_depyler_0589_any_type_mapping() {
        // DEPYLER-0589: Python `any` and `Any` should map to serde_json::Value
        let mapper = TypeMapper::new();

        // Lowercase 'any'
        let any_lower = PythonType::Custom("any".to_string());
        if let RustType::Custom(name) = mapper.map_type(&any_lower) {
            assert_eq!(name, "serde_json::Value");
        } else {
            panic!("Expected Custom type for 'any'");
        }

        // Uppercase 'Any'
        let any_upper = PythonType::Custom("Any".to_string());
        if let RustType::Custom(name) = mapper.map_type(&any_upper) {
            assert_eq!(name, "serde_json::Value");
        } else {
            panic!("Expected Custom type for 'Any'");
        }

        // typing.Any
        let typing_any = PythonType::Custom("typing.Any".to_string());
        if let RustType::Custom(name) = mapper.map_type(&typing_any) {
            assert_eq!(name, "serde_json::Value");
        } else {
            panic!("Expected Custom type for 'typing.Any'");
        }
    }

    #[test]
    fn test_depyler_0589_callable_type_mapping() {
        // DEPYLER-0589: Python `callable` and `Callable` should map to Box<dyn Fn()>
        let mapper = TypeMapper::new();

        // Lowercase 'callable'
        let callable_lower = PythonType::Custom("callable".to_string());
        if let RustType::Custom(name) = mapper.map_type(&callable_lower) {
            assert_eq!(name, "Box<dyn Fn()>");
        } else {
            panic!("Expected Custom type for 'callable'");
        }

        // Uppercase 'Callable'
        let callable_upper = PythonType::Custom("Callable".to_string());
        if let RustType::Custom(name) = mapper.map_type(&callable_upper) {
            assert_eq!(name, "Box<dyn Fn()>");
        } else {
            panic!("Expected Custom type for 'Callable'");
        }
    }

    #[test]
    fn test_set_type_mapping() {
        let mapper = TypeMapper::new();

        // Set[int]
        let set_type = PythonType::Set(Box::new(PythonType::Int));
        assert_eq!(
            mapper.map_type(&set_type),
            RustType::HashSet(Box::new(RustType::Primitive(PrimitiveType::I32)))
        );

        // Set[str]
        let set_str_type = PythonType::Set(Box::new(PythonType::String));
        assert_eq!(
            mapper.map_type(&set_str_type),
            RustType::HashSet(Box::new(RustType::String))
        );

        assert_eq!(
            RustType::HashSet(Box::new(RustType::Primitive(PrimitiveType::I32))).to_rust_string(),
            "HashSet<i32>"
        );

        // Sets need references
        assert!(mapper.needs_reference(&RustType::HashSet(Box::new(RustType::String))));
    }
}
