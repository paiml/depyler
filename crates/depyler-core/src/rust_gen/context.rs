//! Code generation context and core traits
//!
//! This module provides the central CodeGenContext struct that maintains
//! state during Rust code generation, along with core traits used across
//! the code generation pipeline.

use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::hir::Type;
use crate::string_optimization::StringOptimizer;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Code generation context
///
/// Maintains all state needed during Rust code generation including:
/// - Type mapping and string optimization
/// - Import tracking (needs_hashmap, needs_cow, etc.)
/// - Variable scoping and mutability analysis
/// - Generator state management
///
/// # Complexity
/// N/A (data structure)
pub struct CodeGenContext<'a> {
    pub type_mapper: &'a crate::type_mapper::TypeMapper,
    pub annotation_aware_mapper: AnnotationAwareTypeMapper,
    pub string_optimizer: StringOptimizer,
    pub union_enum_generator: crate::union_enum_gen::UnionEnumGenerator,
    pub generated_enums: Vec<proc_macro2::TokenStream>,
    pub needs_hashmap: bool,
    pub needs_hashset: bool,
    pub needs_vecdeque: bool,
    pub needs_fnv_hashmap: bool,
    pub needs_ahash_hashmap: bool,
    pub needs_arc: bool,
    pub needs_rc: bool,
    pub needs_cow: bool,
    pub declared_vars: Vec<HashSet<String>>,
    pub current_function_can_fail: bool,
    pub current_return_type: Option<Type>,
    pub module_mapper: crate::module_mapper::ModuleMapper,
    pub imported_modules: std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
    pub imported_items: std::collections::HashMap<String, String>,
    pub mutable_vars: HashSet<String>,
    pub needs_zerodivisionerror: bool,
    pub needs_indexerror: bool,
    pub is_classmethod: bool,
    pub in_generator: bool,
    pub generator_state_vars: HashSet<String>,
    pub var_types: HashMap<String, Type>,
    pub class_names: HashSet<String>,
    pub mutating_methods: HashMap<String, HashSet<String>>,
    /// DEPYLER-0307 Fix #9: Track variables that iterate over tuples (from zip())
    /// Used to generate tuple field access syntax (tuple.0, tuple.1) instead of vector indexing
    pub tuple_iter_vars: HashSet<String>,
    /// DEPYLER-0271: Tracks if current statement is the final statement in its block
    /// Used to generate idiomatic expression-based returns (no `return` keyword)
    pub is_final_statement: bool,
}

impl<'a> CodeGenContext<'a> {
    /// Enter a new lexical scope
    ///
    /// # Complexity
    /// 1 (simple push)
    pub fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    /// Exit the current lexical scope
    ///
    /// # Complexity
    /// 1 (simple pop)
    pub fn exit_scope(&mut self) {
        self.declared_vars.pop();
    }

    /// Check if a variable is declared in any scope
    ///
    /// # Complexity
    /// 2 (iterator + any)
    pub fn is_declared(&self, var_name: &str) -> bool {
        self.declared_vars
            .iter()
            .any(|scope| scope.contains(var_name))
    }

    /// Declare a variable in the current scope
    ///
    /// # Complexity
    /// 2 (if let + insert)
    pub fn declare_var(&mut self, var_name: &str) {
        if let Some(current_scope) = self.declared_vars.last_mut() {
            current_scope.insert(var_name.to_string());
        }
    }

    /// Process a Union type and generate an enum if needed
    ///
    /// Returns the enum name and optionally generates an enum definition
    /// that is added to generated_enums.
    ///
    /// # Complexity
    /// 2 (if + push)
    pub fn process_union_type(&mut self, types: &[crate::hir::Type]) -> String {
        let (enum_name, enum_def) = self.union_enum_generator.generate_union_enum(types);
        if !enum_def.is_empty() {
            self.generated_enums.push(enum_def);
        }
        enum_name
    }
}

/// Trait for converting HIR elements to Rust tokens
///
/// This is the main trait for code generation. All HIR types that can
/// be converted to Rust code implement this trait.
pub trait RustCodeGen {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream>;
}

/// Extension trait for converting expressions to Rust syn::Expr
///
/// Used internally for expression-to-expression conversions where
/// we need syn::Expr specifically rather than TokenStream.
pub trait ToRustExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr>;
}
