//! IDE integration support for Depyler
//!
//! This module provides IDE integration features including:
//! - Symbol indexing for navigation
//! - Hover information generation
//! - Code completion suggestions
//! - Diagnostic reporting

use crate::error::ErrorKind;
use crate::hir::{HirClass, HirFunction, HirMethod, HirModule};
use rustpython_parser::text_size::{TextRange, TextSize};
use std::collections::HashMap;

/// Symbol information for IDE features
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: TextRange,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Class,
    Method,
    Variable,
    Parameter,
    Field,
    Module,
}

/// IDE integration provider
#[derive(Default)]
pub struct IdeIntegration {
    symbols: HashMap<String, Vec<Symbol>>,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: TextRange,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

impl IdeIntegration {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index symbols from HIR for navigation
    pub fn index_symbols(&mut self, module: &HirModule, source: &str) {
        // Index functions
        for func in &module.functions {
            self.index_function(func, source);
        }

        // Index classes
        for class in &module.classes {
            self.index_class(class, source);
        }
    }

    fn index_function(&mut self, func: &HirFunction, _source: &str) {
        // Create a placeholder range (would need actual source positions)
        let range = TextRange::new(TextSize::from(0), TextSize::from(100));

        let mut params = Vec::new();
        for (name, ty) in &func.params {
            params.push(format!("{}: {:?}", name, ty));
        }
        let detail = format!(
            "fn {}({}) -> {:?}",
            func.name,
            params.join(", "),
            func.ret_type
        );

        let symbol = Symbol {
            name: func.name.clone(),
            kind: SymbolKind::Function,
            range,
            detail: Some(detail),
            documentation: None, // Could extract from docstring if available
        };

        self.symbols
            .entry(func.name.clone())
            .or_default()
            .push(symbol);
    }

    fn index_class(&mut self, class: &HirClass, _source: &str) {
        // Create a placeholder range
        let range = TextRange::new(TextSize::from(0), TextSize::from(100));

        let symbol = Symbol {
            name: class.name.clone(),
            kind: SymbolKind::Class,
            range,
            detail: Some(format!("class {}", class.name)),
            documentation: None,
        };

        self.symbols
            .entry(class.name.clone())
            .or_default()
            .push(symbol);

        // Index class methods
        for method in &class.methods {
            self.index_method(method, &class.name, _source);
        }

        // Index fields
        for field in &class.fields {
            let field_symbol = Symbol {
                name: field.name.clone(),
                kind: SymbolKind::Field,
                range, // Use class range for now
                detail: Some(format!("{}: {:?}", field.name, field.field_type)),
                documentation: None,
            };
            self.symbols
                .entry(field.name.clone())
                .or_default()
                .push(field_symbol);
        }
    }

    fn index_method(&mut self, method: &HirMethod, class_name: &str, _source: &str) {
        // Create a placeholder range
        let range = TextRange::new(TextSize::from(0), TextSize::from(100));

        let mut params = Vec::new();
        for (name, ty) in &method.params {
            params.push(format!("{}: {:?}", name, ty));
        }
        let detail = format!(
            "{}::{}({}) -> {:?}",
            class_name,
            method.name,
            params.join(", "),
            method.ret_type
        );

        let symbol = Symbol {
            name: format!("{}::{}", class_name, method.name),
            kind: SymbolKind::Method,
            range,
            detail: Some(detail),
            documentation: None,
        };

        self.symbols
            .entry(method.name.clone())
            .or_default()
            .push(symbol);
    }

    /// Get symbol at position for hover/goto definition
    pub fn symbol_at_position(&self, position: TextSize) -> Option<&Symbol> {
        for symbols in self.symbols.values() {
            for symbol in symbols {
                if symbol.range.contains(position) {
                    return Some(symbol);
                }
            }
        }
        None
    }

    /// Find all references to a symbol
    pub fn find_references(&self, symbol_name: &str) -> Vec<&Symbol> {
        self.symbols
            .get(symbol_name)
            .map(|symbols| symbols.iter().collect())
            .unwrap_or_default()
    }

    /// Get completion suggestions at position
    pub fn completions_at_position(
        &self,
        _position: TextSize,
        prefix: &str,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        for (name, symbols) in &self.symbols {
            if name.starts_with(prefix) {
                for symbol in symbols {
                    completions.push(CompletionItem {
                        label: name.clone(),
                        kind: match symbol.kind {
                            SymbolKind::Function => CompletionKind::Function,
                            SymbolKind::Class => CompletionKind::Class,
                            SymbolKind::Method => CompletionKind::Method,
                            SymbolKind::Variable => CompletionKind::Variable,
                            SymbolKind::Parameter => CompletionKind::Variable,
                            SymbolKind::Field => CompletionKind::Field,
                            SymbolKind::Module => CompletionKind::Module,
                        },
                        detail: symbol.detail.clone(),
                        documentation: symbol.documentation.clone(),
                    });
                }
            }
        }

        completions
    }

    /// Add a diagnostic
    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Convert errors to diagnostics
    pub fn add_error(&mut self, error: &ErrorKind, range: TextRange) {
        let diagnostic = Diagnostic {
            range,
            severity: DiagnosticSeverity::Error,
            message: error.to_string(),
            code: None,
            source: "depyler".to_string(),
        };
        self.add_diagnostic(diagnostic);
    }

    /// Add a warning diagnostic
    pub fn add_warning(&mut self, message: String, range: TextRange) {
        let diagnostic = Diagnostic {
            range,
            severity: DiagnosticSeverity::Warning,
            message,
            code: None,
            source: "depyler".to_string(),
        };
        self.add_diagnostic(diagnostic);
    }
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompletionKind {
    Function,
    Class,
    Method,
    Variable,
    Field,
    Module,
}

/// Generate hover information for a symbol
pub fn generate_hover_info(symbol: &Symbol) -> String {
    let mut hover = String::new();

    // Add symbol type and signature
    if let Some(detail) = &symbol.detail {
        hover.push_str(&format!("```rust\n{}\n```\n\n", detail));
    }

    // Add documentation if available
    if let Some(doc) = &symbol.documentation {
        hover.push_str(doc);
    }

    hover
}

/// IDE-specific context extensions
pub trait IdeContext {
    fn get_symbol_at(&self, position: TextSize) -> Option<&Symbol>;
    fn get_completions(&self, position: TextSize, prefix: &str) -> Vec<CompletionItem>;
    fn get_diagnostics(&self) -> &[Diagnostic];
}

/// Create IDE integration from transpilation result
pub fn create_ide_integration(module: &HirModule, source: &str) -> IdeIntegration {
    let mut ide = IdeIntegration::new();
    ide.index_symbols(module, source);
    ide
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{FunctionProperties, Type};
    use smallvec::smallvec;

    #[test]
    fn test_symbol_indexing() {
        let mut ide = IdeIntegration::new();

        let func = HirFunction {
            name: "test_func".to_string(),
            params: smallvec![("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![],
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
            properties: FunctionProperties::default(),
        };

        let module = HirModule {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
        };

        ide.index_symbols(&module, "def test_func(x: int) -> int:\n    pass");

        assert_eq!(ide.symbols.len(), 1);
        assert!(ide.symbols.contains_key("test_func"));

        let symbols = &ide.symbols["test_func"];
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].kind, SymbolKind::Function);
    }

    #[test]
    fn test_hover_generation() {
        let symbol = Symbol {
            name: "my_func".to_string(),
            kind: SymbolKind::Function,
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            detail: Some("fn my_func(x: int) -> int".to_string()),
            documentation: Some("Calculates something".to_string()),
        };

        let hover = generate_hover_info(&symbol);
        assert!(hover.contains("```rust"));
        assert!(hover.contains("fn my_func(x: int) -> int"));
        assert!(hover.contains("Calculates something"));
    }

    #[test]
    fn test_completions() {
        let mut ide = IdeIntegration::new();

        // Add some test symbols
        ide.symbols.insert(
            "test_func".to_string(),
            vec![Symbol {
                name: "test_func".to_string(),
                kind: SymbolKind::Function,
                range: TextRange::new(TextSize::from(0), TextSize::from(10)),
                detail: Some("fn test_func()".to_string()),
                documentation: None,
            }],
        );

        ide.symbols.insert(
            "test_class".to_string(),
            vec![Symbol {
                name: "test_class".to_string(),
                kind: SymbolKind::Class,
                range: TextRange::new(TextSize::from(20), TextSize::from(30)),
                detail: Some("class test_class".to_string()),
                documentation: None,
            }],
        );

        let completions = ide.completions_at_position(TextSize::from(0), "test");
        assert_eq!(completions.len(), 2);

        let completions = ide.completions_at_position(TextSize::from(0), "test_f");
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].label, "test_func");
    }
}
