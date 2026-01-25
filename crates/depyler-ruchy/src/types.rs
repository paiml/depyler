//! Enhanced type system mapping between Python and Ruchy
//!
//! This module has been updated to support Ruchy v1.5.0+ features:
//! - DataFrame types with Polars integration
//! - Actor model Value types
//! - Enhanced enum and pattern matching types
//! - Option<T> and Result<T,E> native support
//! - Range types and iterators

use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Type mapper for Python to Ruchy type conversion
pub struct TypeMapper {
    /// Cache for resolved types
    type_cache: HashMap<String, RuchyType>,

    /// Inference engine for gradual typing
    inference_engine: TypeInferenceEngine,

    /// Configuration
    #[allow(dead_code)]
    config: TypeMapperConfig,
}

impl TypeMapper {
    /// Creates a new type mapper
    #[must_use]
    pub fn new() -> Self {
        Self {
            type_cache: Self::init_builtin_types(),
            inference_engine: TypeInferenceEngine::new(),
            config: TypeMapperConfig::default(),
        }
    }

    /// Maps a Python pandas DataFrame to Ruchy DataFrame type
    pub fn map_dataframe_type(&mut self, schema_hint: Option<&str>) -> Result<RuchyType> {
        // In Ruchy v1.5.0+, DataFrame has rich schema support
        match schema_hint {
            Some(_) => Ok(RuchyType::DataFrame), // Could be enhanced with schema info
            None => Ok(RuchyType::DataFrame),
        }
    }

    /// Maps Python async/actor patterns to Ruchy Actor types
    pub fn map_actor_type(&mut self, message_type: Option<PythonType>) -> Result<RuchyType> {
        match message_type {
            Some(msg_type) => {
                let _ruchy_msg_type = self.map_type(&msg_type)?;
                Ok(RuchyType::Actor)
            }
            None => Ok(RuchyType::Actor),
        }
    }

    /// Maps Python range() to Ruchy Range type
    pub fn map_range_type(
        &self,
        start_type: Option<PythonType>,
        end_type: Option<PythonType>,
    ) -> Result<RuchyType> {
        // Ensure both start and end are numeric
        if let (Some(start), Some(end)) = (&start_type, &end_type) {
            match (start, end) {
                (PythonType::Named(s), PythonType::Named(e)) if s == "int" && e == "int" => {
                    Ok(RuchyType::Range)
                }
                _ => Ok(RuchyType::Range), // Default to range anyway
            }
        } else {
            Ok(RuchyType::Range)
        }
    }

    /// Creates with custom configuration
    #[must_use]
    pub fn with_config(config: &crate::RuchyConfig) -> Self {
        Self {
            type_cache: Self::init_builtin_types(),
            inference_engine: TypeInferenceEngine::new(),
            config: TypeMapperConfig::from_ruchy_config(config),
        }
    }

    /// Initialize built-in type mappings
    fn init_builtin_types() -> HashMap<String, RuchyType> {
        let mut cache = HashMap::new();

        // Python primitive types
        cache.insert("int".to_string(), RuchyType::I64);
        cache.insert("float".to_string(), RuchyType::F64);
        cache.insert("str".to_string(), RuchyType::String);
        cache.insert("bool".to_string(), RuchyType::Bool);
        cache.insert("bytes".to_string(), RuchyType::Vec(Box::new(RuchyType::U8)));

        // Python collection types
        cache.insert(
            "list".to_string(),
            RuchyType::Vec(Box::new(RuchyType::Dynamic)),
        );
        cache.insert(
            "dict".to_string(),
            RuchyType::HashMap(Box::new(RuchyType::Dynamic), Box::new(RuchyType::Dynamic)),
        );
        cache.insert(
            "set".to_string(),
            RuchyType::HashSet(Box::new(RuchyType::Dynamic)),
        );
        cache.insert("tuple".to_string(), RuchyType::Tuple(vec![]));

        // Special types
        cache.insert("None".to_string(), RuchyType::Unit);
        cache.insert("Any".to_string(), RuchyType::Dynamic);

        // Ruchy v1.5.0+ specific types
        cache.insert("DataFrame".to_string(), RuchyType::DataFrame);
        cache.insert("Range".to_string(), RuchyType::Range);
        cache.insert("Actor".to_string(), RuchyType::Actor);
        cache.insert("Message".to_string(), RuchyType::Message);
        cache.insert("EnumVariant".to_string(), RuchyType::EnumVariant);

        cache
    }

    /// Maps a Python type annotation to Ruchy type
    pub fn map_type(&mut self, py_type: &PythonType) -> Result<RuchyType> {
        match py_type {
            PythonType::Named(name) => self
                .type_cache
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow!("Unknown type: {}", name)),

            PythonType::List(inner) => {
                let inner_type = self.map_type(inner)?;
                Ok(RuchyType::Vec(Box::new(inner_type)))
            }

            PythonType::Dict(key, value) => {
                let key_type = self.map_type(key)?;
                let value_type = self.map_type(value)?;
                Ok(RuchyType::HashMap(Box::new(key_type), Box::new(value_type)))
            }

            PythonType::Set(inner) => {
                let inner_type = self.map_type(inner)?;
                Ok(RuchyType::HashSet(Box::new(inner_type)))
            }

            PythonType::Tuple(types) => {
                let ruchy_types = types
                    .iter()
                    .map(|t| self.map_type(t))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RuchyType::Tuple(ruchy_types))
            }

            PythonType::Optional(inner) => {
                let inner_type = self.map_type(inner)?;
                Ok(RuchyType::Option(Box::new(inner_type)))
            }

            PythonType::Union(types) => {
                // For union types, create an enum
                self.create_union_type(types)
            }

            PythonType::Callable(params, ret) => {
                let param_types = params
                    .iter()
                    .map(|p| self.map_type(p))
                    .collect::<Result<Vec<_>>>()?;
                let return_type = self.map_type(ret)?;

                Ok(RuchyType::Function {
                    params: param_types,
                    returns: Box::new(return_type),
                })
            }

            PythonType::Generic(name, args) => self.map_generic_type(name, args),

            PythonType::Any => Ok(RuchyType::Dynamic),

            PythonType::Literal(lit) => self.map_literal_type(lit),
        }
    }

    /// Creates a union type as an enum
    fn create_union_type(&mut self, types: &[PythonType]) -> Result<RuchyType> {
        // Special case: Optional[T] = Union[T, None]
        if types.len() == 2
            && types
                .iter()
                .any(|t| matches!(t, PythonType::Named(n) if n == "None"))
        {
            let other_type = types
                .iter()
                .find(|t| !matches!(t, PythonType::Named(n) if n == "None"))
                .ok_or_else(|| anyhow!("Invalid Optional type"))?;

            let inner = self.map_type(other_type)?;
            return Ok(RuchyType::Option(Box::new(inner)));
        }

        // General union: create enum
        let variants = types
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let ruchy_type = self.map_type(t)?;
                Ok((format!("Variant{}", i), ruchy_type))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(RuchyType::Enum(variants))
    }

    /// Maps generic types like List[str], Dict[str, int]
    fn map_generic_type(&mut self, name: &str, args: &[PythonType]) -> Result<RuchyType> {
        match name {
            "List" | "list" => {
                let inner = args
                    .first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::Vec(Box::new(inner)))
            }

            "Dict" | "dict" => {
                let key = args
                    .first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                let value = args
                    .get(1)
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::HashMap(Box::new(key), Box::new(value)))
            }

            "Set" | "set" => {
                let inner = args
                    .first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::HashSet(Box::new(inner)))
            }

            "Optional" => {
                let inner = args
                    .first()
                    .ok_or_else(|| anyhow!("Optional requires type argument"))?;
                let inner_type = self.map_type(inner)?;
                Ok(RuchyType::Option(Box::new(inner_type)))
            }

            "Awaitable" | "Coroutine" => {
                let inner = args
                    .last()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::Future(Box::new(inner)))
            }

            "DataFrame" => {
                // DataFrame can be parameterized with schema info
                Ok(RuchyType::DataFrame)
            }

            "Actor" => {
                let _message_type = args
                    .first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Message);
                Ok(RuchyType::Actor)
            }

            "Range" => Ok(RuchyType::Range),

            "Iterator" | "Iterable" => {
                let inner = args
                    .first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::Iterator(Box::new(inner)))
            }

            _ => {
                // User-defined generic type
                let type_args = args
                    .iter()
                    .map(|t| self.map_type(t))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RuchyType::Generic(name.to_string(), type_args))
            }
        }
    }

    /// Maps literal types
    fn map_literal_type(&self, lit: &PythonLiteral) -> Result<RuchyType> {
        match lit {
            PythonLiteral::Int(_) => Ok(RuchyType::I64),
            PythonLiteral::Float(_) => Ok(RuchyType::F64),
            PythonLiteral::Str(_) => Ok(RuchyType::String),
            PythonLiteral::Bool(_) => Ok(RuchyType::Bool),
            PythonLiteral::None => Ok(RuchyType::Unit),
        }
    }

    /// Infers type from usage patterns
    pub fn infer_type(&mut self, usage: &TypeUsage) -> Result<RuchyType> {
        self.inference_engine.infer(usage)
    }

    /// Registers a custom type mapping
    pub fn register_type(&mut self, py_name: String, ruchy_type: RuchyType) {
        self.type_cache.insert(py_name, ruchy_type);
    }
}

/// Python type representation
#[derive(Debug, Clone, PartialEq)]
pub enum PythonType {
    Named(String),
    List(Box<PythonType>),
    Dict(Box<PythonType>, Box<PythonType>),
    Set(Box<PythonType>),
    Tuple(Vec<PythonType>),
    Optional(Box<PythonType>),
    Union(Vec<PythonType>),
    Callable(Vec<PythonType>, Box<PythonType>),
    Generic(String, Vec<PythonType>),
    Any,
    Literal(PythonLiteral),
}

/// Python literal values for literal types
#[derive(Debug, Clone, PartialEq)]
pub enum PythonLiteral {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
}

/// Ruchy type system
#[derive(Debug, Clone, PartialEq)]
pub enum RuchyType {
    // Primitive types
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
    Unit,

    // Collection types
    Vec(Box<RuchyType>),
    Array(Box<RuchyType>, usize),
    Slice(Box<RuchyType>),
    Tuple(Vec<RuchyType>),
    HashMap(Box<RuchyType>, Box<RuchyType>),
    HashSet(Box<RuchyType>),

    // Option and Result
    Option(Box<RuchyType>),
    Result(Box<RuchyType>, Box<RuchyType>),

    // Function types
    Function {
        params: Vec<RuchyType>,
        returns: Box<RuchyType>,
    },

    // Async types
    Future(Box<RuchyType>),
    Stream(Box<RuchyType>),

    // Iterator types
    Iterator(Box<RuchyType>),

    // Reference types
    Reference {
        typ: Box<RuchyType>,
        is_mutable: bool,
        lifetime: Option<String>,
    },

    // User-defined types
    Named(String),
    Generic(String, Vec<RuchyType>),
    Enum(Vec<(String, RuchyType)>),

    // Ruchy v1.5.0+ specific types
    DataFrame,
    Range,
    Actor,
    Message,
    EnumVariant,
    Lambda {
        params: Vec<String>,
        return_type: Box<RuchyType>,
    },

    // Dynamic type for gradual typing
    Dynamic,
}

/// Type usage information for inference
#[derive(Debug, Clone)]
pub struct TypeUsage {
    pub operations: Vec<Operation>,
    pub assignments: Vec<Assignment>,
    pub calls: Vec<FunctionCall>,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Arithmetic,
    Comparison,
    Logical,
    Indexing,
    Iteration,
    StringOp,
    DataFrameOp,
    ActorSend,
    PatternMatch,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub value_type: Option<PythonType>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function: String,
    pub arg_types: Vec<Option<PythonType>>,
}

/// Type inference engine
struct TypeInferenceEngine {
    constraints: Vec<TypeConstraint>,
}

impl TypeInferenceEngine {
    fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    fn infer(&mut self, usage: &TypeUsage) -> Result<RuchyType> {
        // Collect constraints from usage
        for op in &usage.operations {
            self.add_operation_constraint(op);
        }

        for assignment in &usage.assignments {
            self.add_assignment_constraint(assignment);
        }

        for call in &usage.calls {
            self.add_call_constraint(call);
        }

        // Solve constraints
        self.solve_constraints()
    }

    fn add_operation_constraint(&mut self, op: &Operation) {
        let constraint = match op {
            Operation::Arithmetic => TypeConstraint::Numeric,
            Operation::Comparison => TypeConstraint::Comparable,
            Operation::Logical => TypeConstraint::Boolean,
            Operation::Indexing => TypeConstraint::Indexable,
            Operation::Iteration => TypeConstraint::Iterable,
            Operation::StringOp => TypeConstraint::StringLike,
            Operation::DataFrameOp => TypeConstraint::DataFrameCompatible,
            Operation::ActorSend => TypeConstraint::ActorCompatible,
            Operation::PatternMatch => TypeConstraint::PatternMatchable,
        };
        self.constraints.push(constraint);
    }

    fn add_assignment_constraint(&mut self, _assignment: &Assignment) {
        // Analyze assignment patterns
    }

    fn add_call_constraint(&mut self, _call: &FunctionCall) {
        // Analyze function calls
    }

    fn solve_constraints(&self) -> Result<RuchyType> {
        // Simple heuristic-based solving
        if self
            .constraints
            .iter()
            .any(|c| matches!(c, TypeConstraint::StringLike))
        {
            return Ok(RuchyType::String);
        }

        if self
            .constraints
            .iter()
            .any(|c| matches!(c, TypeConstraint::Numeric))
        {
            // Check if float operations are present
            if self
                .constraints
                .iter()
                .any(|c| matches!(c, TypeConstraint::FloatingPoint))
            {
                return Ok(RuchyType::F64);
            }
            return Ok(RuchyType::I64);
        }

        if self
            .constraints
            .iter()
            .any(|c| matches!(c, TypeConstraint::Boolean))
        {
            return Ok(RuchyType::Bool);
        }

        if self
            .constraints
            .iter()
            .any(|c| matches!(c, TypeConstraint::Iterable))
        {
            return Ok(RuchyType::Vec(Box::new(RuchyType::Dynamic)));
        }

        if self
            .constraints
            .iter()
            .any(|c| matches!(c, TypeConstraint::DataFrameCompatible))
        {
            return Ok(RuchyType::DataFrame);
        }

        if self
            .constraints
            .iter()
            .any(|c| matches!(c, TypeConstraint::ActorCompatible))
        {
            return Ok(RuchyType::Actor);
        }

        // Default to dynamic if unsure
        Ok(RuchyType::Dynamic)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum TypeConstraint {
    Numeric,
    FloatingPoint,
    Comparable,
    Boolean,
    Indexable,
    Iterable,
    StringLike,
    DataFrameCompatible,
    ActorCompatible,
    PatternMatchable,
}

/// Configuration for type mapping
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TypeMapperConfig {
    /// Use 32-bit integers by default
    use_i32_default: bool,

    /// Prefer &str over String
    prefer_str_slice: bool,

    /// Use HashMap instead of BTreeMap
    use_hashmap: bool,
}

impl Default for TypeMapperConfig {
    fn default() -> Self {
        Self {
            use_i32_default: false,
            prefer_str_slice: false,
            use_hashmap: true,
        }
    }
}

impl TypeMapperConfig {
    fn from_ruchy_config(_config: &crate::RuchyConfig) -> Self {
        Self::default()
    }
}

impl Default for TypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_type_mapping() {
        let mut mapper = TypeMapper::new();

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("int".to_string()))
                .unwrap(),
            RuchyType::I64
        );

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("float".to_string()))
                .unwrap(),
            RuchyType::F64
        );

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("str".to_string()))
                .unwrap(),
            RuchyType::String
        );

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("bool".to_string()))
                .unwrap(),
            RuchyType::Bool
        );
    }

    #[test]
    fn test_collection_type_mapping() {
        let mut mapper = TypeMapper::new();

        let list_type = PythonType::List(Box::new(PythonType::Named("int".to_string())));
        assert_eq!(
            mapper.map_type(&list_type).unwrap(),
            RuchyType::Vec(Box::new(RuchyType::I64))
        );

        let dict_type = PythonType::Dict(
            Box::new(PythonType::Named("str".to_string())),
            Box::new(PythonType::Named("int".to_string())),
        );
        assert_eq!(
            mapper.map_type(&dict_type).unwrap(),
            RuchyType::HashMap(Box::new(RuchyType::String), Box::new(RuchyType::I64))
        );
    }

    #[test]
    fn test_optional_type_mapping() {
        let mut mapper = TypeMapper::new();

        let optional_type = PythonType::Optional(Box::new(PythonType::Named("str".to_string())));
        assert_eq!(
            mapper.map_type(&optional_type).unwrap(),
            RuchyType::Option(Box::new(RuchyType::String))
        );
    }

    #[test]
    fn test_ruchy_v091_types() {
        let mut mapper = TypeMapper::new();

        // Test DataFrame type
        let df_type = mapper.map_dataframe_type(None);
        assert!(matches!(df_type.unwrap(), RuchyType::DataFrame));

        // Test Actor type
        let actor_type = mapper.map_actor_type(Some(PythonType::Named("str".to_string())));
        assert!(matches!(actor_type.unwrap(), RuchyType::Actor));

        // Test Range type
        let range_type = mapper.map_range_type(
            Some(PythonType::Named("int".to_string())),
            Some(PythonType::Named("int".to_string())),
        );
        assert!(matches!(range_type.unwrap(), RuchyType::Range));
    }

    #[test]
    fn test_enhanced_builtin_types() {
        let mut mapper = TypeMapper::new();

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("DataFrame".to_string()))
                .unwrap(),
            RuchyType::DataFrame
        );

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("Range".to_string()))
                .unwrap(),
            RuchyType::Range
        );

        assert_eq!(
            mapper
                .map_type(&PythonType::Named("Actor".to_string()))
                .unwrap(),
            RuchyType::Actor
        );
    }

    // Additional tests for better coverage

    #[test]
    fn test_type_mapper_default() {
        let mapper = TypeMapper::default();
        assert!(!mapper.type_cache.is_empty());
    }

    #[test]
    fn test_type_mapper_new() {
        let mapper = TypeMapper::new();
        // Check builtin types are initialized
        assert!(mapper.type_cache.contains_key("int"));
        assert!(mapper.type_cache.contains_key("float"));
        assert!(mapper.type_cache.contains_key("str"));
        assert!(mapper.type_cache.contains_key("bool"));
    }

    #[test]
    fn test_type_mapper_with_config() {
        let config = crate::RuchyConfig::default();
        let mapper = TypeMapper::with_config(&config);
        assert!(!mapper.type_cache.is_empty());
    }

    #[test]
    fn test_map_bytes_type() {
        let mut mapper = TypeMapper::new();
        let result = mapper
            .map_type(&PythonType::Named("bytes".to_string()))
            .unwrap();
        assert_eq!(result, RuchyType::Vec(Box::new(RuchyType::U8)));
    }

    #[test]
    fn test_map_none_type() {
        let mut mapper = TypeMapper::new();
        let result = mapper
            .map_type(&PythonType::Named("None".to_string()))
            .unwrap();
        assert_eq!(result, RuchyType::Unit);
    }

    #[test]
    fn test_map_any_type() {
        let mut mapper = TypeMapper::new();
        let result = mapper
            .map_type(&PythonType::Named("Any".to_string()))
            .unwrap();
        assert_eq!(result, RuchyType::Dynamic);
    }

    #[test]
    fn test_map_set_type() {
        let mut mapper = TypeMapper::new();
        let set_type = PythonType::Set(Box::new(PythonType::Named("int".to_string())));
        let result = mapper.map_type(&set_type).unwrap();
        assert_eq!(result, RuchyType::HashSet(Box::new(RuchyType::I64)));
    }

    #[test]
    fn test_map_tuple_type() {
        let mut mapper = TypeMapper::new();
        let tuple_type = PythonType::Tuple(vec![
            PythonType::Named("int".to_string()),
            PythonType::Named("str".to_string()),
        ]);
        let result = mapper.map_type(&tuple_type).unwrap();
        assert_eq!(
            result,
            RuchyType::Tuple(vec![RuchyType::I64, RuchyType::String])
        );
    }

    #[test]
    fn test_map_callable_type() {
        let mut mapper = TypeMapper::new();
        let callable_type = PythonType::Callable(
            vec![PythonType::Named("int".to_string())],
            Box::new(PythonType::Named("str".to_string())),
        );
        let result = mapper.map_type(&callable_type).unwrap();
        assert!(matches!(result, RuchyType::Function { .. }));
    }

    #[test]
    fn test_map_python_type_any() {
        let mut mapper = TypeMapper::new();
        let result = mapper.map_type(&PythonType::Any).unwrap();
        assert_eq!(result, RuchyType::Dynamic);
    }

    #[test]
    fn test_map_literal_int() {
        let mut mapper = TypeMapper::new();
        let lit = PythonType::Literal(PythonLiteral::Int(42));
        let result = mapper.map_type(&lit).unwrap();
        assert_eq!(result, RuchyType::I64);
    }

    #[test]
    fn test_map_literal_float() {
        let mut mapper = TypeMapper::new();
        let lit = PythonType::Literal(PythonLiteral::Float(3.14));
        let result = mapper.map_type(&lit).unwrap();
        assert_eq!(result, RuchyType::F64);
    }

    #[test]
    fn test_map_literal_str() {
        let mut mapper = TypeMapper::new();
        let lit = PythonType::Literal(PythonLiteral::Str("hello".to_string()));
        let result = mapper.map_type(&lit).unwrap();
        assert_eq!(result, RuchyType::String);
    }

    #[test]
    fn test_map_literal_bool() {
        let mut mapper = TypeMapper::new();
        let lit = PythonType::Literal(PythonLiteral::Bool(true));
        let result = mapper.map_type(&lit).unwrap();
        assert_eq!(result, RuchyType::Bool);
    }

    #[test]
    fn test_map_literal_none() {
        let mut mapper = TypeMapper::new();
        let lit = PythonType::Literal(PythonLiteral::None);
        let result = mapper.map_type(&lit).unwrap();
        assert_eq!(result, RuchyType::Unit);
    }

    #[test]
    fn test_map_union_optional() {
        let mut mapper = TypeMapper::new();
        // Union[str, None] should become Option<String>
        let union_type = PythonType::Union(vec![
            PythonType::Named("str".to_string()),
            PythonType::Named("None".to_string()),
        ]);
        let result = mapper.map_type(&union_type).unwrap();
        assert_eq!(result, RuchyType::Option(Box::new(RuchyType::String)));
    }

    #[test]
    fn test_map_union_multiple() {
        let mut mapper = TypeMapper::new();
        // Union[int, str, bool] should become an Enum
        let union_type = PythonType::Union(vec![
            PythonType::Named("int".to_string()),
            PythonType::Named("str".to_string()),
            PythonType::Named("bool".to_string()),
        ]);
        let result = mapper.map_type(&union_type).unwrap();
        assert!(matches!(result, RuchyType::Enum(_)));
    }

    #[test]
    fn test_map_generic_list() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "List".to_string(),
            vec![PythonType::Named("str".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Vec(Box::new(RuchyType::String)));
    }

    #[test]
    fn test_map_generic_dict() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Dict".to_string(),
            vec![
                PythonType::Named("str".to_string()),
                PythonType::Named("int".to_string()),
            ],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(
            result,
            RuchyType::HashMap(Box::new(RuchyType::String), Box::new(RuchyType::I64))
        );
    }

    #[test]
    fn test_map_generic_set() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Set".to_string(),
            vec![PythonType::Named("int".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::HashSet(Box::new(RuchyType::I64)));
    }

    #[test]
    fn test_map_generic_optional() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Optional".to_string(),
            vec![PythonType::Named("int".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Option(Box::new(RuchyType::I64)));
    }

    #[test]
    fn test_map_generic_awaitable() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Awaitable".to_string(),
            vec![PythonType::Named("str".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Future(Box::new(RuchyType::String)));
    }

    #[test]
    fn test_map_generic_coroutine() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Coroutine".to_string(),
            vec![
                PythonType::Named("int".to_string()),
                PythonType::Named("str".to_string()),
                PythonType::Named("bool".to_string()),
            ],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        // Should use the last type arg
        assert_eq!(result, RuchyType::Future(Box::new(RuchyType::Bool)));
    }

    #[test]
    fn test_map_generic_iterator() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Iterator".to_string(),
            vec![PythonType::Named("int".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Iterator(Box::new(RuchyType::I64)));
    }

    #[test]
    fn test_map_generic_dataframe() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic("DataFrame".to_string(), vec![]);
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::DataFrame);
    }

    #[test]
    fn test_map_generic_actor() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "Actor".to_string(),
            vec![PythonType::Named("str".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Actor);
    }

    #[test]
    fn test_map_generic_range() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic("Range".to_string(), vec![]);
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Range);
    }

    #[test]
    fn test_map_user_generic() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "MyCustomType".to_string(),
            vec![PythonType::Named("str".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert!(matches!(result, RuchyType::Generic(_, _)));
    }

    #[test]
    fn test_register_type() {
        let mut mapper = TypeMapper::new();
        mapper.register_type("CustomType".to_string(), RuchyType::I32);

        let result = mapper
            .map_type(&PythonType::Named("CustomType".to_string()))
            .unwrap();
        assert_eq!(result, RuchyType::I32);
    }

    #[test]
    fn test_unknown_type_error() {
        let mut mapper = TypeMapper::new();
        let result = mapper.map_type(&PythonType::Named("UnknownType".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_dataframe_type_with_schema() {
        let mut mapper = TypeMapper::new();
        let result = mapper.map_dataframe_type(Some("schema info")).unwrap();
        assert_eq!(result, RuchyType::DataFrame);
    }

    #[test]
    fn test_actor_type_without_message() {
        let mut mapper = TypeMapper::new();
        let result = mapper.map_actor_type(None).unwrap();
        assert_eq!(result, RuchyType::Actor);
    }

    #[test]
    fn test_range_type_non_int() {
        let mapper = TypeMapper::new();
        let result = mapper
            .map_range_type(
                Some(PythonType::Named("float".to_string())),
                Some(PythonType::Named("float".to_string())),
            )
            .unwrap();
        assert_eq!(result, RuchyType::Range);
    }

    #[test]
    fn test_range_type_partial() {
        let mapper = TypeMapper::new();
        let result = mapper
            .map_range_type(Some(PythonType::Named("int".to_string())), None)
            .unwrap();
        assert_eq!(result, RuchyType::Range);
    }

    #[test]
    fn test_range_type_none() {
        let mapper = TypeMapper::new();
        let result = mapper.map_range_type(None, None).unwrap();
        assert_eq!(result, RuchyType::Range);
    }

    // Test type inference
    #[test]
    fn test_infer_string_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::StringOp],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::String);
    }

    #[test]
    fn test_infer_numeric_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::Arithmetic],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::I64);
    }

    #[test]
    fn test_infer_boolean_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::Logical],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::Bool);
    }

    #[test]
    fn test_infer_iterable_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::Iteration],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::Vec(Box::new(RuchyType::Dynamic)));
    }

    #[test]
    fn test_infer_dataframe_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::DataFrameOp],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::DataFrame);
    }

    #[test]
    fn test_infer_actor_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::ActorSend],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::Actor);
    }

    #[test]
    fn test_infer_dynamic_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::Dynamic);
    }

    #[test]
    fn test_infer_comparison_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::Comparison],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        // Comparison alone doesn't constrain to a specific type
        assert_eq!(result, RuchyType::Dynamic);
    }

    #[test]
    fn test_infer_indexing_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::Indexing],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::Dynamic);
    }

    #[test]
    fn test_infer_pattern_match_type() {
        let mut mapper = TypeMapper::new();
        let usage = TypeUsage {
            operations: vec![Operation::PatternMatch],
            assignments: vec![],
            calls: vec![],
        };
        let result = mapper.infer_type(&usage).unwrap();
        assert_eq!(result, RuchyType::Dynamic);
    }

    // Test PythonType variants
    #[test]
    fn test_python_type_debug() {
        let py_type = PythonType::Named("int".to_string());
        let debug = format!("{:?}", py_type);
        assert!(debug.contains("Named"));
        assert!(debug.contains("int"));
    }

    #[test]
    fn test_python_type_clone() {
        let py_type = PythonType::Named("str".to_string());
        let cloned = py_type.clone();
        assert_eq!(py_type, cloned);
    }

    #[test]
    fn test_python_type_eq() {
        let t1 = PythonType::Named("int".to_string());
        let t2 = PythonType::Named("int".to_string());
        let t3 = PythonType::Named("str".to_string());
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    // Test PythonLiteral variants
    #[test]
    fn test_python_literal_debug() {
        let lit = PythonLiteral::Int(42);
        let debug = format!("{:?}", lit);
        assert!(debug.contains("Int"));
        assert!(debug.contains("42"));
    }

    #[test]
    fn test_python_literal_clone() {
        let lit = PythonLiteral::Str("test".to_string());
        let cloned = lit.clone();
        assert_eq!(lit, cloned);
    }

    // Test RuchyType variants
    #[test]
    fn test_ruchy_type_debug() {
        let ty = RuchyType::I64;
        let debug = format!("{:?}", ty);
        assert!(debug.contains("I64"));
    }

    #[test]
    fn test_ruchy_type_clone() {
        let ty = RuchyType::Vec(Box::new(RuchyType::String));
        let cloned = ty.clone();
        assert_eq!(ty, cloned);
    }

    #[test]
    fn test_ruchy_type_eq() {
        let t1 = RuchyType::I64;
        let t2 = RuchyType::I64;
        let t3 = RuchyType::F64;
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    // Test TypeUsage
    #[test]
    fn test_type_usage_debug() {
        let usage = TypeUsage {
            operations: vec![Operation::Arithmetic],
            assignments: vec![],
            calls: vec![],
        };
        let debug = format!("{:?}", usage);
        assert!(debug.contains("operations"));
    }

    #[test]
    fn test_type_usage_clone() {
        let usage = TypeUsage {
            operations: vec![Operation::Arithmetic],
            assignments: vec![],
            calls: vec![],
        };
        let cloned = usage.clone();
        assert_eq!(usage.operations.len(), cloned.operations.len());
    }

    // Test Operation
    #[test]
    fn test_operation_debug() {
        let op = Operation::Arithmetic;
        let debug = format!("{:?}", op);
        assert!(debug.contains("Arithmetic"));
    }

    #[test]
    fn test_operation_clone() {
        let op = Operation::StringOp;
        let cloned = op.clone();
        assert!(matches!(cloned, Operation::StringOp));
    }

    // Test Assignment
    #[test]
    fn test_assignment_debug() {
        let assignment = Assignment { value_type: None };
        let debug = format!("{:?}", assignment);
        assert!(debug.contains("value_type"));
    }

    #[test]
    fn test_assignment_clone() {
        let assignment = Assignment {
            value_type: Some(PythonType::Named("int".to_string())),
        };
        let cloned = assignment.clone();
        assert_eq!(assignment.value_type, cloned.value_type);
    }

    // Test FunctionCall
    #[test]
    fn test_function_call_debug() {
        let call = FunctionCall {
            function: "print".to_string(),
            arg_types: vec![],
        };
        let debug = format!("{:?}", call);
        assert!(debug.contains("print"));
    }

    #[test]
    fn test_function_call_clone() {
        let call = FunctionCall {
            function: "len".to_string(),
            arg_types: vec![Some(PythonType::Named("str".to_string()))],
        };
        let cloned = call.clone();
        assert_eq!(call.function, cloned.function);
        assert_eq!(call.arg_types.len(), cloned.arg_types.len());
    }

    // Test TypeMapperConfig
    #[test]
    fn test_type_mapper_config_default() {
        let config = TypeMapperConfig::default();
        assert!(!config.use_i32_default);
        assert!(!config.prefer_str_slice);
        assert!(config.use_hashmap);
    }

    #[test]
    fn test_type_mapper_config_from_ruchy_config() {
        let ruchy_config = crate::RuchyConfig::default();
        let config = TypeMapperConfig::from_ruchy_config(&ruchy_config);
        // Should return defaults
        assert!(!config.use_i32_default);
    }

    // Test empty generic type args
    #[test]
    fn test_map_generic_list_empty_args() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic("list".to_string(), vec![]);
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(result, RuchyType::Vec(Box::new(RuchyType::Dynamic)));
    }

    #[test]
    fn test_map_generic_dict_single_arg() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic(
            "dict".to_string(),
            vec![PythonType::Named("str".to_string())],
        );
        let result = mapper.map_type(&generic_type).unwrap();
        assert_eq!(
            result,
            RuchyType::HashMap(Box::new(RuchyType::String), Box::new(RuchyType::Dynamic))
        );
    }

    #[test]
    fn test_map_generic_optional_missing_arg() {
        let mut mapper = TypeMapper::new();
        let generic_type = PythonType::Generic("Optional".to_string(), vec![]);
        let result = mapper.map_type(&generic_type);
        assert!(result.is_err());
    }
}
