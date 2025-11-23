//! Function signature registry for interprocedural analysis
//!
//! This module maintains a registry of all function signatures in a module,
//! including parameter types, borrowing strategies, and mutability information.

use crate::hir::{HirFunction, HirModule, HirParam, Type as PythonType};
use crate::type_mapper::RustType;
use std::collections::HashMap;

/// Registry of function signatures for interprocedural analysis
#[derive(Debug, Clone)]
pub struct FunctionSignatureRegistry {
    /// Map from function name to its signature
    pub(crate) signatures: HashMap<String, FunctionSignature>,
}

impl FunctionSignatureRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }

    /// Build registry from a HIR module
    pub fn from_module(module: &HirModule) -> Self {
        let mut registry = Self::new();

        for func in &module.functions {
            let signature = FunctionSignature::from_hir_function(func);
            registry.register(signature);
        }

        registry
    }

    /// Register a function signature
    pub fn register(&mut self, signature: FunctionSignature) {
        self.signatures.insert(signature.name.clone(), signature);
    }

    /// Look up a function signature by name
    pub fn get(&self, name: &str) -> Option<&FunctionSignature> {
        self.signatures.get(name)
    }

    /// Get all function names
    pub fn function_names(&self) -> impl Iterator<Item = &String> {
        self.signatures.keys()
    }

    /// Check if a function exists
    pub fn contains(&self, name: &str) -> bool {
        self.signatures.contains_key(name)
    }

    /// Get number of registered functions
    pub fn len(&self) -> usize {
        self.signatures.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.signatures.is_empty()
    }
}

impl Default for FunctionSignatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete signature information for a function
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Parameter information
    pub params: Vec<ParamSignature>,
    /// Return type (Python type)
    pub return_type: PythonType,
    /// Whether function can fail
    pub can_fail: bool,
}

impl FunctionSignature {
    /// Create a signature from a HIR function
    pub fn from_hir_function(func: &HirFunction) -> Self {
        let params = func
            .params
            .iter()
            .map(ParamSignature::from_hir_param)
            .collect();

        Self {
            name: func.name.clone(),
            params,
            return_type: func.ret_type.clone(),
            can_fail: func.properties.can_fail,
        }
    }

    /// Get parameter by name
    pub fn get_param(&self, name: &str) -> Option<&ParamSignature> {
        self.params.iter().find(|p| p.name == name)
    }

    /// Get parameter by index
    pub fn get_param_by_index(&self, index: usize) -> Option<&ParamSignature> {
        self.params.get(index)
    }

    /// Get number of parameters
    pub fn param_count(&self) -> usize {
        self.params.len()
    }
}

/// Parameter signature with borrowing information
#[derive(Debug, Clone)]
pub struct ParamSignature {
    /// Parameter name
    pub name: String,
    /// Python type
    pub python_type: PythonType,
    /// Whether parameter is mutated (set by analysis)
    pub is_mutated: bool,
    /// Whether parameter has a default value
    pub has_default: bool,
}

impl ParamSignature {
    /// Create a parameter signature from a HIR parameter
    pub fn from_hir_param(param: &HirParam) -> Self {
        Self {
            name: param.name.clone(),
            python_type: param.ty.clone(),
            is_mutated: false, // Will be determined by analysis
            has_default: param.default.is_some(),
        }
    }

    /// Mark this parameter as mutated
    pub fn set_mutated(&mut self, mutated: bool) {
        self.is_mutated = mutated;
    }

    /// Check if this parameter needs mutable borrowing
    pub fn needs_mut_borrow(&self) -> bool {
        self.is_mutated && self.is_borrowable_type()
    }

    /// Check if this type can be borrowed (vs moved)
    fn is_borrowable_type(&self) -> bool {
        // Most types can be borrowed except simple Copy types
        match &self.python_type {
            PythonType::Int | PythonType::Float | PythonType::Bool => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{FunctionProperties, HirFunction};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_signature_from_hir_function() {
        let func = HirFunction {
            name: "test_func".to_string(),
            params: smallvec![
                HirParam::new("x".to_string(), PythonType::Int),
                HirParam::new("y".to_string(), PythonType::String),
            ],
            ret_type: PythonType::None,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let sig = FunctionSignature::from_hir_function(&func);

        assert_eq!(sig.name, "test_func");
        assert_eq!(sig.params.len(), 2);
        assert_eq!(sig.params[0].name, "x");
        assert_eq!(sig.params[1].name, "y");
    }

    #[test]
    fn test_registry_from_module() {
        let module = HirModule {
            functions: vec![
                HirFunction {
                    name: "func1".to_string(),
                    params: smallvec![],
                    ret_type: PythonType::None,
                    body: vec![],
                    properties: FunctionProperties::default(),
                    annotations: TranspilationAnnotations::default(),
                    docstring: None,
                },
                HirFunction {
                    name: "func2".to_string(),
                    params: smallvec![],
                    ret_type: PythonType::None,
                    body: vec![],
                    properties: FunctionProperties::default(),
                    annotations: TranspilationAnnotations::default(),
                    docstring: None,
                },
            ],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };

        let registry = FunctionSignatureRegistry::from_module(&module);

        assert_eq!(registry.len(), 2);
        assert!(registry.contains("func1"));
        assert!(registry.contains("func2"));
    }
}
