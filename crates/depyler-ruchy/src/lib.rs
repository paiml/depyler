//! Ruchy backend for Depyler Python-to-Rust transpiler
//!
//! This crate provides an alternative transpilation target that generates
//! Ruchy script format instead of direct Rust code. Ruchy offers a more
//! Python-like syntax with functional programming features.

#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod ast;
pub mod formatter;
pub mod optimizer;
pub mod transformer;
pub mod types;

use anyhow::Result;
use depyler_core::{TranspilationBackend, TranspileError, ValidationError};
use depyler_core::hir::HirModule;
use std::fmt;

/// The main Ruchy backend implementation
pub struct RuchyBackend {
    /// AST builder for constructing Ruchy expressions
    ast_builder: ast::RuchyAstBuilder,
    
    /// Type mapping engine for Python to Ruchy types
    #[allow(dead_code)]
    type_mapper: types::TypeMapper,
    
    /// Optimization passes for generated code
    optimizer: optimizer::RuchyOptimizer,
    
    /// Code formatter for pretty-printing
    formatter: formatter::RuchyFormatter,
    
    /// Pattern transformer for Pythonic to functional style
    transformer: transformer::PatternTransformer,
}

impl RuchyBackend {
    /// Creates a new Ruchy backend with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            ast_builder: ast::RuchyAstBuilder::new(),
            type_mapper: types::TypeMapper::new(),
            optimizer: optimizer::RuchyOptimizer::new(),
            formatter: formatter::RuchyFormatter::new(),
            transformer: transformer::PatternTransformer::new(),
        }
    }
    
    /// Creates a Ruchy backend with custom configuration
    #[must_use]
    pub fn with_config(config: RuchyConfig) -> Self {
        Self {
            ast_builder: ast::RuchyAstBuilder::with_config(&config),
            type_mapper: types::TypeMapper::with_config(&config),
            optimizer: optimizer::RuchyOptimizer::with_config(&config),
            formatter: formatter::RuchyFormatter::with_config(&config),
            transformer: transformer::PatternTransformer::with_config(&config),
        }
    }
}

impl Default for RuchyBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl TranspilationBackend for RuchyBackend {
    fn transpile(&self, hir: &HirModule) -> Result<String, TranspileError> {
        // Phase 1: Transform HIR to Ruchy AST
        let mut ruchy_ast = self.ast_builder.build(hir)
            .map_err(|e| TranspileError::backend_error(e.to_string()))?;
        
        // Phase 2: Apply pattern transformations (e.g., list comp → pipeline)
        ruchy_ast = self.transformer.transform(ruchy_ast)
            .map_err(|e| TranspileError::transform_error(e.to_string()))?;
        
        // Phase 3: Apply optimizations
        ruchy_ast = self.optimizer.optimize(ruchy_ast)
            .map_err(|e| TranspileError::optimization_error(e.to_string()))?;
        
        // Phase 4: Format and serialize to string
        Ok(self.formatter.format(&ruchy_ast))
    }
    
    fn validate_output(&self, code: &str) -> Result<(), ValidationError> {
        // Validation is optional - Ruchy parser integration would go here
        // For now, basic syntax checks
        if code.is_empty() {
            return Err(ValidationError::InvalidSyntax("Empty code".to_string()));
        }
        
        // Basic bracket matching
        let mut paren_count = 0;
        let mut brace_count = 0;
        let mut bracket_count = 0;
        
        for ch in code.chars() {
            match ch {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => {}
            }
        }
        
        if paren_count != 0 || brace_count != 0 || bracket_count != 0 {
            return Err(ValidationError::InvalidSyntax("Unmatched brackets".to_string()));
        }
        
        Ok(())
    }
    
    fn optimize(&self, hir: &HirModule) -> HirModule {
        // Apply HIR-level optimizations before transpilation
        self.optimizer.optimize_hir(hir.clone())
    }
    
    fn target_name(&self) -> &str {
        "ruchy"
    }
    
    fn file_extension(&self) -> &str {
        "ruchy"
    }
}

/// Configuration for the Ruchy backend
#[derive(Debug, Clone)]
pub struct RuchyConfig {
    /// Enable pipeline operator transformations
    pub use_pipelines: bool,
    
    /// Convert async/await to actor system
    pub use_actors: bool,
    
    /// Enable DataFrame optimizations
    pub optimize_dataframes: bool,
    
    /// Use string interpolation
    pub use_string_interpolation: bool,
    
    /// Maximum line length for formatting
    pub max_line_length: usize,
    
    /// Indentation width
    pub indent_width: usize,
    
    /// Optimization level (0-3)
    pub optimization_level: u8,
}

impl Default for RuchyConfig {
    fn default() -> Self {
        Self {
            use_pipelines: true,
            use_actors: false,
            optimize_dataframes: true,
            use_string_interpolation: true,
            max_line_length: 100,
            indent_width: 4,
            optimization_level: 2,
        }
    }
}

impl fmt::Display for RuchyBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ruchy Script Backend v{}", env!("CARGO_PKG_VERSION"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_backend_creation() {
        let backend = RuchyBackend::new();
        assert_eq!(backend.target_name(), "ruchy");
        assert_eq!(backend.file_extension(), "ruchy");
    }
    
    #[test]
    fn test_config_defaults() {
        let config = RuchyConfig::default();
        assert!(config.use_pipelines);
        assert!(config.use_string_interpolation);
        assert_eq!(config.optimization_level, 2);
    }
}