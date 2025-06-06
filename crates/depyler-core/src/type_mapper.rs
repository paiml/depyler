use crate::hir::Type as PythonType;
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
        match py_type {
            PythonType::Unknown => RustType::Custom("serde_json::Value".to_string()), // Default to dynamic type for unknown
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
            PythonType::Function { params: _, ret: _ } => {
                // For V1, we don't map function types directly
                RustType::Unsupported("function".to_string())
            }
            PythonType::Custom(name) => RustType::Custom(name.clone()),
        }
    }

    pub fn map_return_type(&self, py_type: &PythonType) -> RustType {
        match py_type {
            PythonType::None => RustType::Unit,
            _ => self.map_type(py_type),
        }
    }

    pub fn needs_reference(&self, rust_type: &RustType) -> bool {
        match rust_type {
            RustType::String => false, // V1: Always owned
            RustType::Vec(_) | RustType::HashMap(_, _) => true,
            RustType::Primitive(_) => false,
            _ => false,
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn can_copy(&self, rust_type: &RustType) -> bool {
        match rust_type {
            RustType::Primitive(_) | RustType::Unit => true,
            RustType::Tuple(types) => types.iter().all(|t| self.can_copy(t)),
            _ => false,
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

        let unknown_type = PythonType::Unknown;
        if let RustType::Custom(name) = mapper.map_type(&unknown_type) {
            assert_eq!(name, "serde_json::Value");
        } else {
            panic!("Expected custom type serde_json::Value for unknown type");
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
}
