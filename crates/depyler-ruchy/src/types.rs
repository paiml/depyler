//! Type system mapping between Python and Ruchy

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
        cache.insert("list".to_string(), RuchyType::Vec(Box::new(RuchyType::Dynamic)));
        cache.insert("dict".to_string(), RuchyType::HashMap(
            Box::new(RuchyType::Dynamic),
            Box::new(RuchyType::Dynamic),
        ));
        cache.insert("set".to_string(), RuchyType::HashSet(Box::new(RuchyType::Dynamic)));
        cache.insert("tuple".to_string(), RuchyType::Tuple(vec![]));
        
        // Special types
        cache.insert("None".to_string(), RuchyType::Unit);
        cache.insert("Any".to_string(), RuchyType::Dynamic);
        
        cache
    }
    
    /// Maps a Python type annotation to Ruchy type
    pub fn map_type(&mut self, py_type: &PythonType) -> Result<RuchyType> {
        match py_type {
            PythonType::Named(name) => {
                self.type_cache.get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Unknown type: {}", name))
            }
            
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
                let ruchy_types = types.iter()
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
                let param_types = params.iter()
                    .map(|p| self.map_type(p))
                    .collect::<Result<Vec<_>>>()?;
                let return_type = self.map_type(ret)?;
                
                Ok(RuchyType::Function {
                    params: param_types,
                    returns: Box::new(return_type),
                })
            }
            
            PythonType::Generic(name, args) => {
                self.map_generic_type(name, args)
            }
            
            PythonType::Any => Ok(RuchyType::Dynamic),
            
            PythonType::Literal(lit) => self.map_literal_type(lit),
        }
    }
    
    /// Creates a union type as an enum
    fn create_union_type(&mut self, types: &[PythonType]) -> Result<RuchyType> {
        // Special case: Optional[T] = Union[T, None]
        if types.len() == 2 {
            if types.iter().any(|t| matches!(t, PythonType::Named(n) if n == "None")) {
                let other_type = types.iter()
                    .find(|t| !matches!(t, PythonType::Named(n) if n == "None"))
                    .ok_or_else(|| anyhow!("Invalid Optional type"))?;
                
                let inner = self.map_type(other_type)?;
                return Ok(RuchyType::Option(Box::new(inner)));
            }
        }
        
        // General union: create enum
        let variants = types.iter()
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
                let inner = args.first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::Vec(Box::new(inner)))
            }
            
            "Dict" | "dict" => {
                let key = args.get(0)
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                let value = args.get(1)
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::HashMap(Box::new(key), Box::new(value)))
            }
            
            "Set" | "set" => {
                let inner = args.first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::HashSet(Box::new(inner)))
            }
            
            "Optional" => {
                let inner = args.first()
                    .ok_or_else(|| anyhow!("Optional requires type argument"))?;
                let inner_type = self.map_type(inner)?;
                Ok(RuchyType::Option(Box::new(inner_type)))
            }
            
            "Awaitable" | "Coroutine" => {
                let inner = args.last()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::Future(Box::new(inner)))
            }
            
            "Iterator" | "Iterable" => {
                let inner = args.first()
                    .map(|t| self.map_type(t))
                    .transpose()?
                    .unwrap_or(RuchyType::Dynamic);
                Ok(RuchyType::Iterator(Box::new(inner)))
            }
            
            _ => {
                // User-defined generic type
                let type_args = args.iter()
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
    I8, I16, I32, I64, I128, ISize,
    U8, U16, U32, U64, U128, USize,
    F32, F64,
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
        if self.constraints.iter().any(|c| matches!(c, TypeConstraint::StringLike)) {
            return Ok(RuchyType::String);
        }
        
        if self.constraints.iter().any(|c| matches!(c, TypeConstraint::Numeric)) {
            // Check if float operations are present
            if self.constraints.iter().any(|c| matches!(c, TypeConstraint::FloatingPoint)) {
                return Ok(RuchyType::F64);
            }
            return Ok(RuchyType::I64);
        }
        
        if self.constraints.iter().any(|c| matches!(c, TypeConstraint::Boolean)) {
            return Ok(RuchyType::Bool);
        }
        
        if self.constraints.iter().any(|c| matches!(c, TypeConstraint::Iterable)) {
            return Ok(RuchyType::Vec(Box::new(RuchyType::Dynamic)));
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
            mapper.map_type(&PythonType::Named("int".to_string())).unwrap(),
            RuchyType::I64
        );
        
        assert_eq!(
            mapper.map_type(&PythonType::Named("float".to_string())).unwrap(),
            RuchyType::F64
        );
        
        assert_eq!(
            mapper.map_type(&PythonType::Named("str".to_string())).unwrap(),
            RuchyType::String
        );
        
        assert_eq!(
            mapper.map_type(&PythonType::Named("bool".to_string())).unwrap(),
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
}