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
    /// DEPYLER-1015: NASA single-shot compile mode - use std-only types
    /// When true, uses String instead of serde_json::Value for unknown types
    #[serde(default = "default_nasa_mode")]
    pub nasa_mode: bool,
}

/// DEPYLER-1015: Default to NASA mode enabled for backward compatibility
fn default_nasa_mode() -> bool {
    true
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
            nasa_mode: true, // DEPYLER-1015: Default to NASA mode for single-shot compile
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

    /// DEPYLER-1015: Enable/disable NASA single-shot compile mode
    pub fn with_nasa_mode(mut self, enabled: bool) -> Self {
        self.nasa_mode = enabled;
        self
    }

    /// DEPYLER-1015: Get the fallback type for unknown values
    /// In NASA mode: String (std-only, always compiles with rustc)
    /// In normal mode: serde_json::Value (requires cargo/external crate)
    fn unknown_fallback(&self) -> RustType {
        if self.nasa_mode {
            RustType::Custom("DepylerValue".to_string())
        } else {
            RustType::Custom("serde_json::Value".to_string())
        }
    }

    /// DEPYLER-1015: Get the fallback type name as a string for format! usage
    fn unknown_fallback_str(&self) -> &'static str {
        if self.nasa_mode {
            "DepylerValue"
        } else {
            "serde_json::Value"
        }
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
            // DEPYLER-0781: Map Unknown to concrete fallback type
            // Previously used TypeParam("T") which caused E0283 errors when T
            // wasn't actually used in the function signature
            // DEPYLER-1015: Use unknown_fallback() for NASA mode compatibility
            PythonType::Unknown => self.unknown_fallback(),
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
            // DEPYLER-0750: Use fallback for bare list (no type params)
            // to avoid generating Vec<T> which requires generic parameter declaration
            // DEPYLER-1015: Use unknown_fallback() for NASA mode compatibility
            PythonType::List(inner) => match inner.as_ref() {
                PythonType::Unknown => RustType::Vec(Box::new(self.unknown_fallback())),
                _ => RustType::Vec(Box::new(self.map_type(inner))),
            },
            PythonType::Dict(k, v) => {
                // DEPYLER-1040b: Use DepylerValue for unknown dict keys (Point 14 falsification fix)
                // Python allows integer keys `{1: "a"}`, tuple keys `{(1,2): "b"}`, etc.
                // Using String would cause E0308 for non-string keys.
                // Principle: "If unsure, use DepylerValue. Never guess."
                let key_type = if matches!(**k, PythonType::Unknown) {
                    self.unknown_fallback() // DepylerValue instead of String
                } else {
                    self.map_type(k)
                };
                let val_type = if matches!(**v, PythonType::Unknown) {
                    self.unknown_fallback()
                } else {
                    self.map_type(v)
                };
                RustType::HashMap(Box::new(key_type), Box::new(val_type))
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
                    // DEPYLER-1015: Use unknown_fallback() for NASA mode compatibility
                    match name.as_str() {
                        // DEPYLER-1040b: Use DepylerValue for both keys and values
                        // Point 14: Dict with integer keys `{1: "a"}` must not assume String keys
                        "Dict" => RustType::HashMap(
                            Box::new(self.unknown_fallback()), // DepylerValue keys (not String)
                            Box::new(self.unknown_fallback()), // DEPYLER-0718/1015: Use fallback
                        ),
                        // DEPYLER-0609: Handle both "List" (typing import) and "list" (builtin)
                        // DEPYLER-0718/1015: Use fallback for bare List (no type params)
                        "List" | "list" => RustType::Vec(Box::new(self.unknown_fallback())),
                        // DEPYLER-1040b: Use DepylerValue for sets too
                        "Set" => RustType::HashSet(Box::new(self.unknown_fallback())),
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
                        // DEPYLER-0734: Python Callable type maps to impl Fn
                        "Callable" | "typing.Callable" | "callable" => {
                            RustType::Custom("impl Fn()".to_string())
                        }
                        // DEPYLER-0589/1015: Python Any type maps to fallback
                        // Both typing.Any and bare 'any' need to be handled
                        "Any" | "typing.Any" | "any" => self.unknown_fallback(),
                        // DEPYLER-0628/1015: Python object type maps to fallback
                        // Python's base object type needs dynamic typing in Rust
                        "object" | "builtins.object" => self.unknown_fallback(),
                        // DEPYLER-0592/1025: Python datetime module types
                        // NASA mode: use std::time types; otherwise use chrono
                        "date" | "datetime.date" => {
                            if self.nasa_mode {
                                RustType::Custom("(u32, u32, u32)".to_string()) // (year, month, day)
                            } else {
                                RustType::Custom("chrono::NaiveDate".to_string())
                            }
                        }
                        "datetime" | "datetime.datetime" => {
                            if self.nasa_mode {
                                RustType::Custom("std::time::SystemTime".to_string())
                            } else {
                                RustType::Custom("chrono::NaiveDateTime".to_string())
                            }
                        }
                        "time" | "datetime.time" => {
                            if self.nasa_mode {
                                RustType::Custom("(u32, u32, u32)".to_string()) // (hour, min, sec)
                            } else {
                                RustType::Custom("chrono::NaiveTime".to_string())
                            }
                        }
                        "timedelta" | "datetime.timedelta" => {
                            if self.nasa_mode {
                                RustType::Custom("std::time::Duration".to_string())
                            } else {
                                RustType::Custom("chrono::Duration".to_string())
                            }
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
                        // DEPYLER-0742/1015: Python collections.deque maps to std::collections::VecDeque
                        // VecDeque is the Rust equivalent of Python's deque (double-ended queue)
                        "deque" | "collections.deque" | "Deque" => {
                            RustType::Custom(format!("std::collections::VecDeque<{}>", self.unknown_fallback_str()))
                        }
                        // DEPYLER-0742: Python Counter maps to HashMap (for now)
                        // A proper Counter implementation would need a wrapper type
                        "Counter" | "collections.Counter" => {
                            RustType::HashMap(Box::new(RustType::String), Box::new(RustType::Primitive(PrimitiveType::I32)))
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
                    // DEPYLER-0742: deque[T] -> VecDeque<T>
                    // Python's collections.deque with type parameter
                    "deque" | "collections.deque" | "Deque" if params.len() == 1 => {
                        let inner_type = self.map_type(&params[0]);
                        RustType::Custom(format!("std::collections::VecDeque<{}>", inner_type.to_rust_string()))
                    }
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
                    // DEPYLER-0734: Callable[[T1, T2, ...], R] -> impl Fn(T1, T2, ...) -> R
                    // Python Callable types map to impl Fn for ergonomic closures without boxing
                    // This allows passing closures directly without Box::new() wrapping
                    // DEPYLER-0846: Detect nested Callable types and use &dyn Fn to avoid E0666
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

                        // DEPYLER-0846: Check if any param or return type contains impl Fn
                        // If so, we can't use impl Fn (E0666 nested impl Trait not allowed)
                        // Convert ALL impl Fn to &dyn Fn for proper higher-order function support
                        let has_nested_fn = param_types.iter().any(|s| s.contains("impl Fn"))
                            || return_str.contains("impl Fn");

                        // DEPYLER-0734: Format: impl Fn(T1, T2) -> R or impl Fn() for None return
                        // DEPYLER-0846: Use &dyn Fn for nested Callable types, and convert inner impl Fn too
                        let (fn_prefix, fixed_params, fixed_return) = if has_nested_fn {
                            // Convert all impl Fn to &dyn Fn recursively
                            let fixed_params: Vec<String> = param_types.iter()
                                .map(|s| s.replace("impl Fn", "&dyn Fn"))
                                .collect();
                            let fixed_return = return_str.replace("impl Fn", "&dyn Fn");
                            ("&dyn Fn", fixed_params, fixed_return)
                        } else {
                            ("impl Fn", param_types.clone(), return_str.clone())
                        };
                        let fn_str = if fixed_return == "()" || matches!(params[1], PythonType::None) {
                            format!("{}({})", fn_prefix, fixed_params.join(", "))
                        } else {
                            format!("{}({}) -> {}", fn_prefix, fixed_params.join(", "), fixed_return)
                        };
                        RustType::Custom(fn_str)
                    }
                    "Callable" if params.is_empty() => {
                        // DEPYLER-0734: Bare Callable without parameters -> impl Fn()
                        RustType::Custom("impl Fn()".to_string())
                    }
                    // DEPYLER-0845: type[T] -> std::marker::PhantomData<T>
                    // Python's type[T] represents a class object that instantiates to T.
                    // Rust doesn't have runtime type objects, so we use PhantomData<T>
                    // to carry the type parameter without storing any data.
                    "type" if params.len() == 1 => {
                        let inner_type = self.map_type(&params[0]);
                        RustType::Custom(format!("std::marker::PhantomData<{}>", inner_type.to_rust_string()))
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
            // DEPYLER-0750: Use String for bare set (no type params) since HashSet needs Hash
            PythonType::Set(inner) => match inner.as_ref() {
                PythonType::Unknown => RustType::HashSet(Box::new(RustType::String)),
                _ => RustType::HashSet(Box::new(self.map_type(inner))),
            },
            PythonType::UnificationVar(id) => {
                // DEPYLER-0692/1015: UnificationVar indicates incomplete type inference
                // Instead of panicking, fall back to a generic type
                // This allows compilation to proceed even when inference is incomplete
                tracing::warn!(
                    "UnificationVar({}) encountered in type mapper. Falling back to {}.",
                    id,
                    self.unknown_fallback_str()
                );
                self.unknown_fallback()
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

        // DEPYLER-0781/1015/1051: Unknown type now maps to DepylerValue in NASA mode (default)
        // This prevents unused generic parameter errors (E0283) and ensures single-shot compile
        // DEPYLER-1051: Hybrid Fallback Strategy - DepylerValue for all uncertain types
        let unknown_type = PythonType::Unknown;
        assert_eq!(
            mapper.map_type(&unknown_type),
            RustType::Custom("DepylerValue".to_string()) // NASA mode uses DepylerValue
        );
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
        // DEPYLER-0589/1015/1051: Python `any` and `Any` should map to DepylerValue in NASA mode
        let mapper = TypeMapper::new();

        // Lowercase 'any' - maps to DepylerValue in NASA mode (Hybrid Fallback)
        let any_lower = PythonType::Custom("any".to_string());
        assert_eq!(mapper.map_type(&any_lower), RustType::Custom("DepylerValue".to_string()));

        // Uppercase 'Any'
        let any_upper = PythonType::Custom("Any".to_string());
        assert_eq!(mapper.map_type(&any_upper), RustType::Custom("DepylerValue".to_string()));

        // typing.Any
        let typing_any = PythonType::Custom("typing.Any".to_string());
        assert_eq!(mapper.map_type(&typing_any), RustType::Custom("DepylerValue".to_string()));

        // Non-NASA mode should use serde_json::Value
        let non_nasa_mapper = TypeMapper::new().with_nasa_mode(false);
        assert_eq!(
            non_nasa_mapper.map_type(&any_lower),
            RustType::Custom("serde_json::Value".to_string())
        );
    }

    #[test]
    fn test_depyler_0734_callable_type_mapping() {
        // DEPYLER-0734: Python `callable` and `Callable` should map to impl Fn()
        // Using impl Fn allows closures to be passed directly without Box::new()
        let mapper = TypeMapper::new();

        // Lowercase 'callable'
        let callable_lower = PythonType::Custom("callable".to_string());
        if let RustType::Custom(name) = mapper.map_type(&callable_lower) {
            assert_eq!(name, "impl Fn()");
        } else {
            panic!("Expected Custom type for 'callable'");
        }

        // Uppercase 'Callable'
        let callable_upper = PythonType::Custom("Callable".to_string());
        if let RustType::Custom(name) = mapper.map_type(&callable_upper) {
            assert_eq!(name, "impl Fn()");
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

    // ============ datetime type mappings (DEPYLER-0592/1025) ============
    // DEPYLER-1025: Tests now test both NASA mode (std types) and non-NASA mode (chrono types)

    #[test]
    fn test_date_type_mapping() {
        // NASA mode (default) - uses tuple
        let nasa_mapper = TypeMapper::new();
        let date = PythonType::Custom("date".to_string());
        if let RustType::Custom(name) = nasa_mapper.map_type(&date) {
            assert_eq!(name, "(u32, u32, u32)");
        } else {
            panic!("Expected Custom type for 'date' in NASA mode");
        }

        // Non-NASA mode - uses chrono
        let non_nasa_mapper = TypeMapper::new().with_nasa_mode(false);
        let datetime_date = PythonType::Custom("datetime.date".to_string());
        if let RustType::Custom(name) = non_nasa_mapper.map_type(&datetime_date) {
            assert_eq!(name, "chrono::NaiveDate");
        } else {
            panic!("Expected Custom type for 'datetime.date' in non-NASA mode");
        }
    }

    #[test]
    fn test_datetime_type_mapping() {
        // NASA mode (default) - uses std::time::SystemTime
        let nasa_mapper = TypeMapper::new();
        let datetime = PythonType::Custom("datetime".to_string());
        if let RustType::Custom(name) = nasa_mapper.map_type(&datetime) {
            assert_eq!(name, "std::time::SystemTime");
        } else {
            panic!("Expected Custom type for 'datetime' in NASA mode");
        }

        // Non-NASA mode - uses chrono
        let non_nasa_mapper = TypeMapper::new().with_nasa_mode(false);
        let datetime_datetime = PythonType::Custom("datetime.datetime".to_string());
        if let RustType::Custom(name) = non_nasa_mapper.map_type(&datetime_datetime) {
            assert_eq!(name, "chrono::NaiveDateTime");
        } else {
            panic!("Expected Custom type for 'datetime.datetime' in non-NASA mode");
        }
    }

    #[test]
    fn test_time_type_mapping() {
        // NASA mode (default) - uses tuple
        let nasa_mapper = TypeMapper::new();
        let time = PythonType::Custom("time".to_string());
        if let RustType::Custom(name) = nasa_mapper.map_type(&time) {
            assert_eq!(name, "(u32, u32, u32)");
        } else {
            panic!("Expected Custom type for 'time' in NASA mode");
        }

        // Non-NASA mode - uses chrono
        let non_nasa_mapper = TypeMapper::new().with_nasa_mode(false);
        let datetime_time = PythonType::Custom("datetime.time".to_string());
        if let RustType::Custom(name) = non_nasa_mapper.map_type(&datetime_time) {
            assert_eq!(name, "chrono::NaiveTime");
        } else {
            panic!("Expected Custom type for 'datetime.time' in non-NASA mode");
        }
    }

    #[test]
    fn test_timedelta_type_mapping() {
        // NASA mode (default) - uses std::time::Duration
        let nasa_mapper = TypeMapper::new();
        let timedelta = PythonType::Custom("timedelta".to_string());
        if let RustType::Custom(name) = nasa_mapper.map_type(&timedelta) {
            assert_eq!(name, "std::time::Duration");
        } else {
            panic!("Expected Custom type for 'timedelta' in NASA mode");
        }

        // Non-NASA mode - uses chrono
        let non_nasa_mapper = TypeMapper::new().with_nasa_mode(false);
        let datetime_timedelta = PythonType::Custom("datetime.timedelta".to_string());
        if let RustType::Custom(name) = non_nasa_mapper.map_type(&datetime_timedelta) {
            assert_eq!(name, "chrono::Duration");
        } else {
            panic!("Expected Custom type for 'datetime.timedelta' in non-NASA mode");
        }
    }

    // ============ Path type mappings (DEPYLER-197) ============

    #[test]
    fn test_path_type_mapping() {
        let mapper = TypeMapper::new();

        let path = PythonType::Custom("Path".to_string());
        if let RustType::Custom(name) = mapper.map_type(&path) {
            assert_eq!(name, "std::path::PathBuf");
        } else {
            panic!("Expected Custom type for 'Path'");
        }

        let pathlib_path = PythonType::Custom("pathlib.Path".to_string());
        if let RustType::Custom(name) = mapper.map_type(&pathlib_path) {
            assert_eq!(name, "std::path::PathBuf");
        } else {
            panic!("Expected Custom type for 'pathlib.Path'");
        }
    }

    #[test]
    fn test_purepath_type_mapping() {
        let mapper = TypeMapper::new();

        let pure_path = PythonType::Custom("PurePath".to_string());
        if let RustType::Custom(name) = mapper.map_type(&pure_path) {
            assert_eq!(name, "std::path::PathBuf");
        } else {
            panic!("Expected Custom type for 'PurePath'");
        }

        let pathlib_purepath = PythonType::Custom("pathlib.PurePath".to_string());
        if let RustType::Custom(name) = mapper.map_type(&pathlib_purepath) {
            assert_eq!(name, "std::path::PathBuf");
        } else {
            panic!("Expected Custom type for 'pathlib.PurePath'");
        }
    }

    // ============ bytes/bytearray mappings (DEPYLER-0584, DEPYLER-0674) ============

    #[test]
    fn test_bytes_type_mapping() {
        let mapper = TypeMapper::new();

        let bytes = PythonType::Custom("bytes".to_string());
        assert_eq!(
            mapper.map_type(&bytes),
            RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::U8)))
        );
    }

    #[test]
    fn test_bytearray_type_mapping() {
        let mapper = TypeMapper::new();

        let bytearray = PythonType::Custom("bytearray".to_string());
        assert_eq!(
            mapper.map_type(&bytearray),
            RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::U8)))
        );
    }

    // ============ exception type mappings (DEPYLER-0597) ============

    #[test]
    fn test_oserror_type_mapping() {
        let mapper = TypeMapper::new();

        let oserror = PythonType::Custom("OSError".to_string());
        if let RustType::Custom(name) = mapper.map_type(&oserror) {
            assert_eq!(name, "std::io::Error");
        } else {
            panic!("Expected Custom type for 'OSError'");
        }
    }

    #[test]
    fn test_ioerror_type_mapping() {
        let mapper = TypeMapper::new();

        let ioerror = PythonType::Custom("IOError".to_string());
        if let RustType::Custom(name) = mapper.map_type(&ioerror) {
            assert_eq!(name, "std::io::Error");
        } else {
            panic!("Expected Custom type for 'IOError'");
        }
    }

    #[test]
    fn test_filenotfounderror_type_mapping() {
        let mapper = TypeMapper::new();

        let fnf = PythonType::Custom("FileNotFoundError".to_string());
        if let RustType::Custom(name) = mapper.map_type(&fnf) {
            assert_eq!(name, "std::io::Error");
        } else {
            panic!("Expected Custom type for 'FileNotFoundError'");
        }
    }

    #[test]
    fn test_permissionerror_type_mapping() {
        let mapper = TypeMapper::new();

        let perr = PythonType::Custom("PermissionError".to_string());
        if let RustType::Custom(name) = mapper.map_type(&perr) {
            assert_eq!(name, "std::io::Error");
        } else {
            panic!("Expected Custom type for 'PermissionError'");
        }
    }

    // ============ argparse/object type mappings ============

    #[test]
    fn test_namespace_type_mapping() {
        let mapper = TypeMapper::new();

        let ns = PythonType::Custom("Namespace".to_string());
        if let RustType::Custom(name) = mapper.map_type(&ns) {
            assert_eq!(name, "Args");
        } else {
            panic!("Expected Custom type for 'Namespace'");
        }

        let argparse_ns = PythonType::Custom("argparse.Namespace".to_string());
        if let RustType::Custom(name) = mapper.map_type(&argparse_ns) {
            assert_eq!(name, "Args");
        } else {
            panic!("Expected Custom type for 'argparse.Namespace'");
        }
    }

    #[test]
    fn test_object_type_mapping() {
        // DEPYLER-1015/1051: In NASA mode, object maps to DepylerValue (Hybrid Fallback)
        let mapper = TypeMapper::new();

        let object = PythonType::Custom("object".to_string());
        assert_eq!(mapper.map_type(&object), RustType::Custom("DepylerValue".to_string()));
    }

    // ============ bare collection type mappings (DEPYLER-0718/1015/1051) ============

    #[test]
    fn test_bare_dict_type_mapping() {
        // DEPYLER-1040b: In NASA mode, bare Dict uses DepylerValue for BOTH keys AND values
        // Point 14: Dict with integer keys `{1: "a"}` must not assume String keys
        let mapper = TypeMapper::new();

        let dict = PythonType::Custom("Dict".to_string());
        if let RustType::HashMap(k, v) = mapper.map_type(&dict) {
            assert_eq!(*k, RustType::Custom("DepylerValue".to_string())); // DEPYLER-1040b: DepylerValue keys
            assert_eq!(*v, RustType::Custom("DepylerValue".to_string())); // NASA mode: DepylerValue fallback
        } else {
            panic!("Expected HashMap for 'Dict'");
        }
    }

    #[test]
    fn test_bare_list_type_mapping() {
        // DEPYLER-1015/1051: In NASA mode, bare List uses DepylerValue for elements (Hybrid Fallback)
        let mapper = TypeMapper::new();

        let list = PythonType::Custom("List".to_string());
        if let RustType::Vec(inner) = mapper.map_type(&list) {
            assert_eq!(*inner, RustType::Custom("DepylerValue".to_string())); // NASA mode: DepylerValue fallback
        } else {
            panic!("Expected Vec for 'List'");
        }

        let list_lower = PythonType::Custom("list".to_string());
        if let RustType::Vec(inner) = mapper.map_type(&list_lower) {
            assert_eq!(*inner, RustType::Custom("DepylerValue".to_string())); // NASA mode: DepylerValue fallback
        } else {
            panic!("Expected Vec for 'list'");
        }
    }

    #[test]
    fn test_bare_set_type_mapping() {
        // DEPYLER-1040b: Bare Set uses DepylerValue for consistency
        let mapper = TypeMapper::new();

        let set = PythonType::Custom("Set".to_string());
        if let RustType::HashSet(inner) = mapper.map_type(&set) {
            assert_eq!(*inner, RustType::Custom("DepylerValue".to_string())); // DEPYLER-1040b
        } else {
            panic!("Expected HashSet for 'Set'");
        }
    }

    #[test]
    fn test_bare_tuple_type_mapping() {
        let mapper = TypeMapper::new();

        let tuple = PythonType::Custom("tuple".to_string());
        if let RustType::Tuple(types) = mapper.map_type(&tuple) {
            assert!(types.is_empty());
        } else {
            panic!("Expected empty Tuple for 'tuple'");
        }
    }

    // ============ type parameter mappings ============

    #[test]
    fn test_single_letter_type_param() {
        let mapper = TypeMapper::new();

        let t = PythonType::Custom("T".to_string());
        assert_eq!(mapper.map_type(&t), RustType::TypeParam("T".to_string()));

        let v = PythonType::Custom("V".to_string());
        assert_eq!(mapper.map_type(&v), RustType::TypeParam("V".to_string()));

        let k = PythonType::Custom("K".to_string());
        assert_eq!(mapper.map_type(&k), RustType::TypeParam("K".to_string()));
    }

    // ============ File type mapping (DEPYLER-0525) ============

    #[test]
    fn test_file_type_mapping() {
        let mapper = TypeMapper::new();

        let file = PythonType::Custom("File".to_string());
        if let RustType::Reference { mutable, inner, .. } = mapper.map_type(&file) {
            assert!(mutable);
            assert_eq!(*inner, RustType::Custom("impl std::io::Write".to_string()));
        } else {
            panic!("Expected Reference type for 'File'");
        }
    }

    // ============ RustConstGeneric tests ============

    #[test]
    fn test_const_generic_literal() {
        assert_eq!(RustConstGeneric::Literal(10).to_rust_string(), "10");
        assert_eq!(RustConstGeneric::Literal(0).to_rust_string(), "0");
    }

    #[test]
    fn test_const_generic_parameter() {
        assert_eq!(
            RustConstGeneric::Parameter("N".to_string()).to_rust_string(),
            "N"
        );
    }

    #[test]
    fn test_const_generic_expression() {
        assert_eq!(
            RustConstGeneric::Expression("N + 1".to_string()).to_rust_string(),
            "N + 1"
        );
    }

    // ============ Array type tests ============

    #[test]
    fn test_array_type_to_string() {
        let array = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::I32)),
            size: RustConstGeneric::Literal(5),
        };
        assert_eq!(array.to_rust_string(), "[i32; 5]");
    }

    #[test]
    fn test_array_type_with_param() {
        let array = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::F64)),
            size: RustConstGeneric::Parameter("N".to_string()),
        };
        assert_eq!(array.to_rust_string(), "[f64; N]");
    }

    // ============ Callable with parameters (DEPYLER-0734) ============

    #[test]
    fn test_callable_with_single_param() {
        let mapper = TypeMapper::new();

        // Callable[[int], str] -> impl Fn(i32) -> String
        let callable = PythonType::Generic {
            base: "Callable".to_string(),
            params: vec![
                PythonType::List(Box::new(PythonType::Int)),
                PythonType::String,
            ],
        };
        if let RustType::Custom(name) = mapper.map_type(&callable) {
            assert!(name.contains("impl Fn"));
            assert!(name.contains("i32"));
            assert!(name.contains("String"));
        } else {
            panic!("Expected Custom type for Callable with params");
        }
    }

    #[test]
    fn test_callable_with_tuple_params() {
        let mapper = TypeMapper::new();

        // Callable[[int, str], bool] -> impl Fn(i32, String) -> bool
        let callable = PythonType::Generic {
            base: "Callable".to_string(),
            params: vec![
                PythonType::Tuple(vec![PythonType::Int, PythonType::String]),
                PythonType::Bool,
            ],
        };
        if let RustType::Custom(name) = mapper.map_type(&callable) {
            assert!(name.contains("impl Fn"));
            assert!(name.contains("i32"));
            assert!(name.contains("String"));
            assert!(name.contains("bool"));
        } else {
            panic!("Expected Custom type for Callable with tuple params");
        }
    }

    #[test]
    fn test_callable_with_no_return() {
        let mapper = TypeMapper::new();

        // Callable[[int], None] -> impl Fn(i32)
        let callable = PythonType::Generic {
            base: "Callable".to_string(),
            params: vec![
                PythonType::Tuple(vec![PythonType::Int]),
                PythonType::None,
            ],
        };
        if let RustType::Custom(name) = mapper.map_type(&callable) {
            assert!(name.contains("impl Fn"));
            assert!(!name.contains("->"));
        } else {
            panic!("Expected Custom type for Callable with None return");
        }
    }

    #[test]
    fn test_callable_empty_params() {
        let mapper = TypeMapper::new();

        // Callable[[], int] -> impl Fn() -> i32
        let callable = PythonType::Generic {
            base: "Callable".to_string(),
            params: vec![
                PythonType::None, // Empty param list
                PythonType::Int,
            ],
        };
        if let RustType::Custom(name) = mapper.map_type(&callable) {
            assert!(name.contains("impl Fn()"));
            assert!(name.contains("i32"));
        } else {
            panic!("Expected Custom type for Callable with empty params");
        }
    }

    #[test]
    fn test_callable_unknown_params() {
        let mapper = TypeMapper::new();

        // Callable with Unknown params list
        let callable = PythonType::Generic {
            base: "Callable".to_string(),
            params: vec![
                PythonType::Unknown,
                PythonType::Int,
            ],
        };
        if let RustType::Custom(name) = mapper.map_type(&callable) {
            assert!(name.contains("impl Fn()"));
        } else {
            panic!("Expected Custom type for Callable with unknown params");
        }
    }

    #[test]
    fn test_bare_callable_generic() {
        let mapper = TypeMapper::new();

        // Bare Callable without params
        let callable = PythonType::Generic {
            base: "Callable".to_string(),
            params: vec![],
        };
        if let RustType::Custom(name) = mapper.map_type(&callable) {
            assert_eq!(name, "impl Fn()");
        } else {
            panic!("Expected Custom type for bare Callable");
        }
    }

    // ============ Generator/Iterator type mappings (DEPYLER-0188) ============

    #[test]
    fn test_generator_type_mapping() {
        let mapper = TypeMapper::new();

        // Generator[int, None, None] -> impl Iterator<Item=i32>
        let gen = PythonType::Generic {
            base: "Generator".to_string(),
            params: vec![PythonType::Int, PythonType::None, PythonType::None],
        };
        if let RustType::Custom(name) = mapper.map_type(&gen) {
            assert!(name.contains("impl Iterator"));
            assert!(name.contains("Item=i32"));
        } else {
            panic!("Expected Custom type for Generator");
        }
    }

    #[test]
    fn test_iterator_type_mapping() {
        let mapper = TypeMapper::new();

        // Iterator[str] -> impl Iterator<Item=String>
        let iter = PythonType::Generic {
            base: "Iterator".to_string(),
            params: vec![PythonType::String],
        };
        if let RustType::Custom(name) = mapper.map_type(&iter) {
            assert!(name.contains("impl Iterator"));
            assert!(name.contains("Item=String"));
        } else {
            panic!("Expected Custom type for Iterator");
        }
    }

    #[test]
    fn test_iterable_type_mapping() {
        let mapper = TypeMapper::new();

        // Iterable[int] -> impl IntoIterator<Item=i32>
        let iterable = PythonType::Generic {
            base: "Iterable".to_string(),
            params: vec![PythonType::Int],
        };
        if let RustType::Custom(name) = mapper.map_type(&iterable) {
            assert!(name.contains("impl IntoIterator"));
            assert!(name.contains("Item=i32"));
        } else {
            panic!("Expected Custom type for Iterable");
        }
    }

    // ============ Deque type mappings (DEPYLER-0742) ============

    #[test]
    fn test_deque_type_mapping() {
        let mapper = TypeMapper::new();

        // deque[int] -> VecDeque<i32>
        let deque = PythonType::Generic {
            base: "deque".to_string(),
            params: vec![PythonType::Int],
        };
        if let RustType::Custom(name) = mapper.map_type(&deque) {
            assert!(name.contains("VecDeque"));
            assert!(name.contains("i32"));
        } else {
            panic!("Expected Custom type for deque");
        }
    }

    #[test]
    fn test_bare_deque_mapping() {
        let mapper = TypeMapper::new();

        let deque = PythonType::Custom("deque".to_string());
        if let RustType::Custom(name) = mapper.map_type(&deque) {
            assert!(name.contains("VecDeque"));
        } else {
            panic!("Expected Custom type for bare deque");
        }
    }

    #[test]
    fn test_counter_mapping() {
        let mapper = TypeMapper::new();

        let counter = PythonType::Custom("Counter".to_string());
        if let RustType::HashMap(k, v) = mapper.map_type(&counter) {
            assert_eq!(*k, RustType::String);
            assert_eq!(*v, RustType::Primitive(PrimitiveType::I32));
        } else {
            panic!("Expected HashMap for Counter");
        }
    }

    // ============ type[T] mappings (DEPYLER-0845) ============

    #[test]
    fn test_type_param_mapping() {
        let mapper = TypeMapper::new();

        // type[MyClass] -> PhantomData<MyClass>
        let type_t = PythonType::Generic {
            base: "type".to_string(),
            params: vec![PythonType::Custom("MyClass".to_string())],
        };
        if let RustType::Custom(name) = mapper.map_type(&type_t) {
            assert!(name.contains("PhantomData"));
            assert!(name.contains("MyClass"));
        } else {
            panic!("Expected Custom type for type[T]");
        }
    }

    // ============ Union type mappings ============

    #[test]
    fn test_union_type_mapping() {
        let mapper = TypeMapper::new();

        // Union[int, str] -> Enum with variants
        let union = PythonType::Union(vec![PythonType::Int, PythonType::String]);
        if let RustType::Enum { name: _, variants } = mapper.map_type(&union) {
            assert_eq!(variants.len(), 2);
        } else {
            panic!("Expected Enum for Union");
        }
    }

    #[test]
    fn test_union_with_none_is_optional() {
        let mapper = TypeMapper::new();

        // Union[int, None] -> Option<int>
        let union = PythonType::Union(vec![PythonType::Int, PythonType::None]);
        if let RustType::Option(inner) = mapper.map_type(&union) {
            assert_eq!(*inner, RustType::Primitive(PrimitiveType::I32));
        } else {
            panic!("Expected Option for Union[T, None]");
        }

        // Union[None, str] -> Option<str> (None first)
        let union2 = PythonType::Union(vec![PythonType::None, PythonType::String]);
        if let RustType::Option(inner) = mapper.map_type(&union2) {
            assert_eq!(*inner, RustType::String);
        } else {
            panic!("Expected Option for Union[None, T]");
        }
    }

    // ============ Exception type mappings (DEPYLER-0597) ============

    #[test]
    fn test_os_error_mapping() {
        let mapper = TypeMapper::new();

        for err in ["OSError", "IOError", "FileNotFoundError", "PermissionError"] {
            let error = PythonType::Custom(err.to_string());
            if let RustType::Custom(name) = mapper.map_type(&error) {
                assert_eq!(name, "std::io::Error");
            } else {
                panic!("Expected std::io::Error for {}", err);
            }
        }
    }

    #[test]
    fn test_general_exception_mapping() {
        let mapper = TypeMapper::new();

        for exc in ["ValueError", "TypeError", "KeyError", "IndexError", "RuntimeError"] {
            let error = PythonType::Custom(exc.to_string());
            if let RustType::Custom(name) = mapper.map_type(&error) {
                assert_eq!(name, "Box<dyn std::error::Error>");
            } else {
                panic!("Expected Box<dyn Error> for {}", exc);
            }
        }
    }

    // ============ Object type mapping (DEPYLER-0628/1051) ============

    #[test]
    fn test_object_builtins_type_mapping() {
        let mapper = TypeMapper::new();

        // DEPYLER-1015/1051: In NASA mode, builtins.object maps to DepylerValue (Hybrid Fallback)
        let obj = PythonType::Custom("builtins.object".to_string());
        assert_eq!(mapper.map_type(&obj), RustType::Custom("DepylerValue".to_string()));
    }

    // ============ Additional RustType tests ============

    #[test]
    fn test_str_type_to_string() {
        let str_type = RustType::Str { lifetime: Some("'a".to_string()) };
        assert_eq!(str_type.to_rust_string(), "&'a str");

        let str_no_lt = RustType::Str { lifetime: None };
        assert_eq!(str_no_lt.to_rust_string(), "&str");
    }

    #[test]
    fn test_cow_type_to_string() {
        let cow = RustType::Cow { lifetime: "'static".to_string() };
        assert_eq!(cow.to_rust_string(), "Cow<'static, str>");
    }

    #[test]
    fn test_reference_type_to_string() {
        let ref_type = RustType::Reference {
            lifetime: Some("'a".to_string()),
            mutable: false,
            inner: Box::new(RustType::String),
        };
        assert_eq!(ref_type.to_rust_string(), "&'a String");

        let mut_ref = RustType::Reference {
            lifetime: None,
            mutable: true,
            inner: Box::new(RustType::Primitive(PrimitiveType::I32)),
        };
        assert_eq!(mut_ref.to_rust_string(), "&mut i32");
    }

    #[test]
    fn test_result_type_to_string() {
        let result = RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("Error".to_string())),
        );
        assert_eq!(result.to_rust_string(), "Result<String, Error>");
    }

    #[test]
    fn test_generic_type_to_string() {
        let generic = RustType::Generic {
            base: "MyType".to_string(),
            params: vec![RustType::Primitive(PrimitiveType::I32), RustType::String],
        };
        assert_eq!(generic.to_rust_string(), "MyType<i32, String>");
    }

    #[test]
    fn test_enum_type_to_string() {
        let enum_type = RustType::Enum {
            name: "MyUnion".to_string(),
            variants: vec![
                ("Int".to_string(), RustType::Primitive(PrimitiveType::I32)),
                ("Str".to_string(), RustType::String),
            ],
        };
        // Enum just returns its name
        assert_eq!(enum_type.to_rust_string(), "MyUnion");
    }

    #[test]
    fn test_unsupported_type_to_string() {
        let unsup = RustType::Unsupported("SomeType".to_string());
        assert_eq!(unsup.to_rust_string(), "/* unsupported: SomeType */");
    }

    #[test]
    fn test_type_param_to_string() {
        let param = RustType::TypeParam("T".to_string());
        assert_eq!(param.to_rust_string(), "T");
    }

    // ============ PrimitiveType tests ============

    #[test]
    fn test_all_primitive_types() {
        let primitives = [
            (PrimitiveType::Bool, "bool"),
            (PrimitiveType::I8, "i8"),
            (PrimitiveType::I16, "i16"),
            (PrimitiveType::I32, "i32"),
            (PrimitiveType::I64, "i64"),
            (PrimitiveType::I128, "i128"),
            (PrimitiveType::ISize, "isize"),
            (PrimitiveType::U8, "u8"),
            (PrimitiveType::U16, "u16"),
            (PrimitiveType::U32, "u32"),
            (PrimitiveType::U64, "u64"),
            (PrimitiveType::U128, "u128"),
            (PrimitiveType::USize, "usize"),
            (PrimitiveType::F32, "f32"),
            (PrimitiveType::F64, "f64"),
        ];

        for (prim, expected) in primitives {
            let rust_type = RustType::Primitive(prim);
            assert_eq!(rust_type.to_rust_string(), expected);
        }
    }

    // ============ TypeVar and GenericList tests ============

    #[test]
    fn test_type_var_mapping() {
        let mapper = TypeMapper::new();

        let type_var = PythonType::TypeVar("T".to_string());
        assert_eq!(mapper.map_type(&type_var), RustType::TypeParam("T".to_string()));
    }

    #[test]
    fn test_generic_list_mapping() {
        let mapper = TypeMapper::new();

        let list = PythonType::Generic {
            base: "List".to_string(),
            params: vec![PythonType::String],
        };
        assert_eq!(
            mapper.map_type(&list),
            RustType::Vec(Box::new(RustType::String))
        );
    }

    #[test]
    fn test_generic_dict_mapping() {
        let mapper = TypeMapper::new();

        let dict = PythonType::Generic {
            base: "Dict".to_string(),
            params: vec![PythonType::String, PythonType::Int],
        };
        assert_eq!(
            mapper.map_type(&dict),
            RustType::HashMap(
                Box::new(RustType::String),
                Box::new(RustType::Primitive(PrimitiveType::I32))
            )
        );
    }

    // ============ Array type mapping with ConstGeneric ============

    #[test]
    fn test_array_type_mapping() {
        let mapper = TypeMapper::new();

        let array = PythonType::Array {
            element_type: Box::new(PythonType::Int),
            size: ConstGeneric::Literal(10),
        };
        if let RustType::Array { element_type, size } = mapper.map_type(&array) {
            assert_eq!(*element_type, RustType::Primitive(PrimitiveType::I32));
            assert_eq!(size, RustConstGeneric::Literal(10));
        } else {
            panic!("Expected Array type");
        }
    }

    #[test]
    fn test_array_with_const_param() {
        let mapper = TypeMapper::new();

        let array = PythonType::Array {
            element_type: Box::new(PythonType::Float),
            size: ConstGeneric::Parameter("N".to_string()),
        };
        if let RustType::Array { element_type, size } = mapper.map_type(&array) {
            assert_eq!(*element_type, RustType::Primitive(PrimitiveType::F64));
            assert_eq!(size, RustConstGeneric::Parameter("N".to_string()));
        } else {
            panic!("Expected Array type");
        }
    }

    #[test]
    fn test_array_with_expression() {
        let mapper = TypeMapper::new();

        let array = PythonType::Array {
            element_type: Box::new(PythonType::Bool),
            size: ConstGeneric::Expression("N + 1".to_string()),
        };
        if let RustType::Array { element_type, size } = mapper.map_type(&array) {
            assert_eq!(*element_type, RustType::Primitive(PrimitiveType::Bool));
            assert_eq!(size, RustConstGeneric::Expression("N + 1".to_string()));
        } else {
            panic!("Expected Array type");
        }
    }

    // ============ needs_reference edge cases ============

    #[test]
    fn test_needs_reference_option() {
        let mapper = TypeMapper::new();

        // Option doesn't need reference (falls through to default false)
        assert!(!mapper.needs_reference(&RustType::Option(Box::new(RustType::Primitive(PrimitiveType::I32)))));
    }

    #[test]
    fn test_needs_reference_result() {
        let mapper = TypeMapper::new();

        // Result doesn't need reference (falls through to default false)
        let result = RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("Error".to_string())),
        );
        assert!(!mapper.needs_reference(&result));
    }

    #[test]
    fn test_needs_reference_array() {
        let mapper = TypeMapper::new();

        // Arrays need references
        let array = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::I32)),
            size: RustConstGeneric::Literal(100),
        };
        assert!(mapper.needs_reference(&array));
    }

    // ============ can_copy edge cases ============

    #[test]
    fn test_can_copy_option_not_copy() {
        let mapper = TypeMapper::new();

        // Option is NOT Copy in this implementation
        assert!(!mapper.can_copy(&RustType::Option(Box::new(RustType::Primitive(PrimitiveType::I32)))));
    }

    #[test]
    fn test_can_copy_reference_not_copy() {
        let mapper = TypeMapper::new();

        // References are NOT Copy in this implementation
        let ref_type = RustType::Reference {
            lifetime: None,
            mutable: false,
            inner: Box::new(RustType::String),
        };
        assert!(!mapper.can_copy(&ref_type));
    }

    #[test]
    fn test_can_copy_small_array() {
        let mapper = TypeMapper::new();

        // Small array of Copy elements is Copy
        let small_array = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::I32)),
            size: RustConstGeneric::Literal(16),
        };
        assert!(mapper.can_copy(&small_array));

        // Large array is NOT Copy
        let large_array = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::I32)),
            size: RustConstGeneric::Literal(100),
        };
        assert!(!mapper.can_copy(&large_array));

        // Array with parameter size is NOT Copy
        let param_array = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::I32)),
            size: RustConstGeneric::Parameter("N".to_string()),
        };
        assert!(!mapper.can_copy(&param_array));
    }
}
