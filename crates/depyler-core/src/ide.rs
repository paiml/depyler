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
        for param in &func.params {
            params.push(format!("{}: {:?}", param.name, param.ty));
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
        for param in &method.params {
            params.push(format!("{}: {:?}", param.name, param.ty));
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
    use crate::hir::{FunctionProperties, HirClass, HirField, HirMethod, HirParam, Type};
    use smallvec::smallvec;

    // === Symbol tests ===

    #[test]
    fn test_symbol_new() {
        let symbol = Symbol {
            name: "my_symbol".to_string(),
            kind: SymbolKind::Function,
            range: TextRange::new(TextSize::from(0), TextSize::from(50)),
            detail: Some("fn my_symbol()".to_string()),
            documentation: Some("A test symbol".to_string()),
        };
        assert_eq!(symbol.name, "my_symbol");
        assert_eq!(symbol.kind, SymbolKind::Function);
        assert!(symbol.detail.is_some());
        assert!(symbol.documentation.is_some());
    }

    #[test]
    fn test_symbol_clone() {
        let symbol = Symbol {
            name: "test".to_string(),
            kind: SymbolKind::Class,
            range: TextRange::new(TextSize::from(10), TextSize::from(20)),
            detail: None,
            documentation: None,
        };
        let cloned = symbol.clone();
        assert_eq!(cloned.name, symbol.name);
        assert_eq!(cloned.kind, symbol.kind);
    }

    #[test]
    fn test_symbol_debug() {
        let symbol = Symbol {
            name: "debug_test".to_string(),
            kind: SymbolKind::Variable,
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            detail: None,
            documentation: None,
        };
        let debug = format!("{:?}", symbol);
        assert!(debug.contains("debug_test"));
        assert!(debug.contains("Variable"));
    }

    // === SymbolKind tests ===

    #[test]
    fn test_symbol_kind_function() {
        assert_eq!(SymbolKind::Function, SymbolKind::Function);
        assert_ne!(SymbolKind::Function, SymbolKind::Class);
    }

    #[test]
    fn test_symbol_kind_class() {
        let kind = SymbolKind::Class;
        assert_eq!(kind.clone(), SymbolKind::Class);
    }

    #[test]
    fn test_symbol_kind_method() {
        let kind = SymbolKind::Method;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Method"));
    }

    #[test]
    fn test_symbol_kind_variable() {
        assert_eq!(SymbolKind::Variable, SymbolKind::Variable);
    }

    #[test]
    fn test_symbol_kind_parameter() {
        assert_eq!(SymbolKind::Parameter, SymbolKind::Parameter);
    }

    #[test]
    fn test_symbol_kind_field() {
        assert_eq!(SymbolKind::Field, SymbolKind::Field);
    }

    #[test]
    fn test_symbol_kind_module() {
        assert_eq!(SymbolKind::Module, SymbolKind::Module);
    }

    // === DiagnosticSeverity tests ===

    #[test]
    fn test_diagnostic_severity_error() {
        assert_eq!(DiagnosticSeverity::Error, DiagnosticSeverity::Error);
        assert_ne!(DiagnosticSeverity::Error, DiagnosticSeverity::Warning);
    }

    #[test]
    fn test_diagnostic_severity_warning() {
        assert_eq!(DiagnosticSeverity::Warning, DiagnosticSeverity::Warning);
    }

    #[test]
    fn test_diagnostic_severity_information() {
        assert_eq!(
            DiagnosticSeverity::Information,
            DiagnosticSeverity::Information
        );
    }

    #[test]
    fn test_diagnostic_severity_hint() {
        let severity = DiagnosticSeverity::Hint;
        let cloned = severity;
        assert_eq!(cloned, DiagnosticSeverity::Hint);
    }

    // === Diagnostic tests ===

    #[test]
    fn test_diagnostic_new() {
        let diag = Diagnostic {
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            severity: DiagnosticSeverity::Error,
            message: "Test error".to_string(),
            code: Some("E0001".to_string()),
            source: "depyler".to_string(),
        };
        assert_eq!(diag.message, "Test error");
        assert_eq!(diag.severity, DiagnosticSeverity::Error);
        assert_eq!(diag.code, Some("E0001".to_string()));
    }

    #[test]
    fn test_diagnostic_clone() {
        let diag = Diagnostic {
            range: TextRange::new(TextSize::from(0), TextSize::from(5)),
            severity: DiagnosticSeverity::Warning,
            message: "Warning".to_string(),
            code: None,
            source: "test".to_string(),
        };
        let cloned = diag.clone();
        assert_eq!(cloned.message, diag.message);
        assert_eq!(cloned.severity, diag.severity);
    }

    #[test]
    fn test_diagnostic_debug() {
        let diag = Diagnostic {
            range: TextRange::new(TextSize::from(0), TextSize::from(1)),
            severity: DiagnosticSeverity::Information,
            message: "Info".to_string(),
            code: None,
            source: "test".to_string(),
        };
        let debug = format!("{:?}", diag);
        assert!(debug.contains("Information"));
        assert!(debug.contains("Info"));
    }

    // === CompletionKind tests ===

    #[test]
    fn test_completion_kind_function() {
        assert_eq!(CompletionKind::Function, CompletionKind::Function);
    }

    #[test]
    fn test_completion_kind_class() {
        assert_eq!(CompletionKind::Class, CompletionKind::Class);
    }

    #[test]
    fn test_completion_kind_method() {
        assert_eq!(CompletionKind::Method, CompletionKind::Method);
    }

    #[test]
    fn test_completion_kind_variable() {
        assert_eq!(CompletionKind::Variable, CompletionKind::Variable);
    }

    #[test]
    fn test_completion_kind_field() {
        assert_eq!(CompletionKind::Field, CompletionKind::Field);
    }

    #[test]
    fn test_completion_kind_module() {
        let kind = CompletionKind::Module;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Module"));
    }

    // === CompletionItem tests ===

    #[test]
    fn test_completion_item_new() {
        let item = CompletionItem {
            label: "my_func".to_string(),
            kind: CompletionKind::Function,
            detail: Some("fn my_func()".to_string()),
            documentation: Some("Does something".to_string()),
        };
        assert_eq!(item.label, "my_func");
        assert_eq!(item.kind, CompletionKind::Function);
    }

    #[test]
    fn test_completion_item_clone() {
        let item = CompletionItem {
            label: "test".to_string(),
            kind: CompletionKind::Variable,
            detail: None,
            documentation: None,
        };
        let cloned = item.clone();
        assert_eq!(cloned.label, item.label);
    }

    // === IdeIntegration tests ===

    #[test]
    fn test_ide_integration_new() {
        let ide = IdeIntegration::new();
        assert!(ide.diagnostics.is_empty());
        assert!(ide.symbols.is_empty());
    }

    #[test]
    fn test_ide_integration_default() {
        let ide = IdeIntegration::default();
        assert!(ide.diagnostics.is_empty());
    }

    #[test]
    fn test_add_diagnostic() {
        let mut ide = IdeIntegration::new();
        let diag = Diagnostic {
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            severity: DiagnosticSeverity::Error,
            message: "Test".to_string(),
            code: None,
            source: "test".to_string(),
        };
        ide.add_diagnostic(diag);
        assert_eq!(ide.diagnostics().len(), 1);
    }

    #[test]
    fn test_diagnostics_getter() {
        let mut ide = IdeIntegration::new();
        assert!(ide.diagnostics().is_empty());

        ide.add_diagnostic(Diagnostic {
            range: TextRange::new(TextSize::from(0), TextSize::from(1)),
            severity: DiagnosticSeverity::Warning,
            message: "Warn".to_string(),
            code: None,
            source: "test".to_string(),
        });
        assert_eq!(ide.diagnostics().len(), 1);
        assert_eq!(ide.diagnostics()[0].message, "Warn");
    }

    #[test]
    fn test_add_warning() {
        let mut ide = IdeIntegration::new();
        let range = TextRange::new(TextSize::from(5), TextSize::from(15));
        ide.add_warning("Unused variable".to_string(), range);

        assert_eq!(ide.diagnostics().len(), 1);
        assert_eq!(ide.diagnostics()[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(ide.diagnostics()[0].message, "Unused variable");
        assert_eq!(ide.diagnostics()[0].source, "depyler");
    }

    #[test]
    fn test_add_error() {
        let mut ide = IdeIntegration::new();
        let range = TextRange::new(TextSize::from(0), TextSize::from(10));
        let error = crate::error::ErrorKind::ParseError;
        ide.add_error(&error, range);

        assert_eq!(ide.diagnostics().len(), 1);
        assert_eq!(ide.diagnostics()[0].severity, DiagnosticSeverity::Error);
    }

    #[test]
    fn test_symbol_at_position_found() {
        let mut ide = IdeIntegration::new();
        ide.symbols.insert(
            "test".to_string(),
            vec![Symbol {
                name: "test".to_string(),
                kind: SymbolKind::Function,
                range: TextRange::new(TextSize::from(10), TextSize::from(50)),
                detail: None,
                documentation: None,
            }],
        );

        let symbol = ide.symbol_at_position(TextSize::from(25));
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().name, "test");
    }

    #[test]
    fn test_symbol_at_position_not_found() {
        let mut ide = IdeIntegration::new();
        ide.symbols.insert(
            "test".to_string(),
            vec![Symbol {
                name: "test".to_string(),
                kind: SymbolKind::Function,
                range: TextRange::new(TextSize::from(10), TextSize::from(20)),
                detail: None,
                documentation: None,
            }],
        );

        let symbol = ide.symbol_at_position(TextSize::from(100));
        assert!(symbol.is_none());
    }

    #[test]
    fn test_find_references_found() {
        let mut ide = IdeIntegration::new();
        ide.symbols.insert(
            "my_func".to_string(),
            vec![
                Symbol {
                    name: "my_func".to_string(),
                    kind: SymbolKind::Function,
                    range: TextRange::new(TextSize::from(0), TextSize::from(10)),
                    detail: None,
                    documentation: None,
                },
                Symbol {
                    name: "my_func".to_string(),
                    kind: SymbolKind::Function,
                    range: TextRange::new(TextSize::from(50), TextSize::from(60)),
                    detail: None,
                    documentation: None,
                },
            ],
        );

        let refs = ide.find_references("my_func");
        assert_eq!(refs.len(), 2);
    }

    #[test]
    fn test_find_references_not_found() {
        let ide = IdeIntegration::new();
        let refs = ide.find_references("nonexistent");
        assert!(refs.is_empty());
    }

    #[test]
    fn test_completions_empty_prefix() {
        let mut ide = IdeIntegration::new();
        ide.symbols.insert(
            "abc".to_string(),
            vec![Symbol {
                name: "abc".to_string(),
                kind: SymbolKind::Variable,
                range: TextRange::new(TextSize::from(0), TextSize::from(5)),
                detail: None,
                documentation: None,
            }],
        );

        let completions = ide.completions_at_position(TextSize::from(0), "");
        // Empty prefix matches nothing since "abc".starts_with("") is true
        assert!(!completions.is_empty());
    }

    #[test]
    fn test_completions_no_match() {
        let mut ide = IdeIntegration::new();
        ide.symbols.insert(
            "foo".to_string(),
            vec![Symbol {
                name: "foo".to_string(),
                kind: SymbolKind::Function,
                range: TextRange::new(TextSize::from(0), TextSize::from(5)),
                detail: None,
                documentation: None,
            }],
        );

        let completions = ide.completions_at_position(TextSize::from(0), "bar");
        assert!(completions.is_empty());
    }

    #[test]
    fn test_index_class_with_methods() {
        let mut ide = IdeIntegration::new();

        let class = HirClass {
            name: "MyClass".to_string(),
            fields: vec![HirField {
                name: "value".to_string(),
                field_type: Type::Int,
                default_value: None,
                is_class_var: false,
            }],
            methods: vec![HirMethod {
                name: "get_value".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![],
                docstring: None,
                is_static: false,
                is_classmethod: false,
                is_property: false,
                is_async: false,
            }],
            base_classes: vec![],
            is_dataclass: false,
            docstring: None,
            type_params: vec![],
        };

        let module = HirModule {
            functions: vec![],
            classes: vec![class],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        ide.index_symbols(&module, "");

        // Should have class, method, and field indexed
        assert!(ide.symbols.contains_key("MyClass"));
        assert!(ide.symbols.contains_key("get_value"));
        assert!(ide.symbols.contains_key("value"));
    }

    #[test]
    fn test_create_ide_integration() {
        let module = HirModule {
            functions: vec![HirFunction {
                name: "main".to_string(),
                params: smallvec![],
                ret_type: Type::None,
                body: vec![],
                annotations: depyler_annotations::TranspilationAnnotations::default(),
                docstring: None,
                properties: FunctionProperties::default(),
            }],
            classes: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let ide = create_ide_integration(&module, "");
        assert!(ide.symbols.contains_key("main"));
    }

    // === generate_hover_info tests ===

    #[test]
    fn test_generate_hover_info_with_detail_only() {
        let symbol = Symbol {
            name: "func".to_string(),
            kind: SymbolKind::Function,
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            detail: Some("fn func() -> i32".to_string()),
            documentation: None,
        };

        let hover = generate_hover_info(&symbol);
        assert!(hover.contains("```rust"));
        assert!(hover.contains("fn func() -> i32"));
        assert!(hover.contains("```"));
    }

    #[test]
    fn test_generate_hover_info_with_doc_only() {
        let symbol = Symbol {
            name: "var".to_string(),
            kind: SymbolKind::Variable,
            range: TextRange::new(TextSize::from(0), TextSize::from(5)),
            detail: None,
            documentation: Some("A counter variable".to_string()),
        };

        let hover = generate_hover_info(&symbol);
        assert!(hover.contains("A counter variable"));
    }

    #[test]
    fn test_generate_hover_info_empty() {
        let symbol = Symbol {
            name: "empty".to_string(),
            kind: SymbolKind::Variable,
            range: TextRange::new(TextSize::from(0), TextSize::from(5)),
            detail: None,
            documentation: None,
        };

        let hover = generate_hover_info(&symbol);
        assert!(hover.is_empty());
    }

    // === Original tests ===

    #[test]
    fn test_symbol_indexing() {
        let mut ide = IdeIntegration::new();

        let func = HirFunction {
            name: "test_func".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
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
            constants: vec![],
            top_level_stmts: vec![],
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
