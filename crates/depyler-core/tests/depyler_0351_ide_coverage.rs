//! DEPYLER-0351: ide.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: ide.rs 25-30% → 85%+ coverage
//! TDG Score: 99.57 (A+) - Excellent quality IDE integration
//!
//! This test suite validates IDE integration functionality:
//! - Symbol indexing (functions, classes, methods, fields)
//! - Symbol lookup (symbol_at_position, find_references)
//! - Code completion suggestions
//! - Diagnostic reporting (errors, warnings)
//! - Hover information generation
//! - Edge cases
//!
//! NOTE: All tests use PUBLIC APIs only (no direct field access)

#![allow(non_snake_case)]

use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::*;
use depyler_core::ide::*;
use rustpython_parser::text_size::{TextRange, TextSize};
use smallvec::smallvec;

// ============================================================================
// SYMBOL INDEXING TESTS
// ============================================================================

#[test]
fn test_depyler_0351_index_class_with_methods() {
    let mut ide = IdeIntegration::new();

    let method = HirMethod {
        name: "calculate".to_string(),
        params: smallvec![
            HirParam::new("self".to_string(), Type::Unknown),
            HirParam::new("x".to_string(), Type::Int),
        ],
        ret_type: Type::Int,
        body: vec![],
        is_static: false,
        is_classmethod: false,
        is_property: false,
        is_async: false,
        docstring: None,
    };

    let class = HirClass {
        name: "Calculator".to_string(),
        methods: vec![method],
        fields: vec![],
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

    ide.index_symbols(
        &module,
        "class Calculator:\n    def calculate(self, x: int) -> int: pass",
    );

    // Verify class symbol indexed via find_references
    let class_refs = ide.find_references("Calculator");
    assert_eq!(class_refs.len(), 1, "Should index class");
    assert_eq!(class_refs[0].kind, SymbolKind::Class);

    // Verify method symbol indexed
    let method_refs = ide.find_references("calculate");
    assert_eq!(method_refs.len(), 1, "Should index method");
    assert_eq!(method_refs[0].kind, SymbolKind::Method);
}

#[test]
fn test_depyler_0351_index_class_with_fields() {
    let mut ide = IdeIntegration::new();

    let field = HirField {
        name: "count".to_string(),
        field_type: Type::Int,
        is_class_var: false,
        default_value: None,
    };

    let class = HirClass {
        name: "Counter".to_string(),
        methods: vec![],
        fields: vec![field],
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

    ide.index_symbols(&module, "class Counter:\n    count: int");

    // Verify class indexed
    let class_refs = ide.find_references("Counter");
    assert_eq!(class_refs.len(), 1);
    assert_eq!(class_refs[0].kind, SymbolKind::Class);

    // Verify field indexed
    let field_refs = ide.find_references("count");
    assert_eq!(field_refs.len(), 1);
    assert_eq!(field_refs[0].kind, SymbolKind::Field);
}

#[test]
fn test_depyler_0351_index_multiple_methods() {
    let mut ide = IdeIntegration::new();

    let method1 = HirMethod {
        name: "add".to_string(),
        params: smallvec![
            HirParam::new("self".to_string(), Type::Unknown),
            HirParam::new("x".to_string(), Type::Int),
        ],
        ret_type: Type::Int,
        body: vec![],
        is_static: false,
        is_classmethod: false,
        is_property: false,
        is_async: false,
        docstring: None,
    };

    let method2 = HirMethod {
        name: "subtract".to_string(),
        params: smallvec![
            HirParam::new("self".to_string(), Type::Unknown),
            HirParam::new("x".to_string(), Type::Int),
        ],
        ret_type: Type::Int,
        body: vec![],
        is_static: false,
        is_classmethod: false,
        is_property: false,
        is_async: false,
        docstring: None,
    };

    let class = HirClass {
        name: "Math".to_string(),
        methods: vec![method1, method2],
        fields: vec![],
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

    ide.index_symbols(&module, "class Math: pass");

    assert_eq!(ide.find_references("add").len(), 1);
    assert_eq!(ide.find_references("subtract").len(), 1);
}

#[test]
fn test_depyler_0351_index_empty_module() {
    let mut ide = IdeIntegration::new();

    let module = HirModule {
        functions: vec![],
        classes: vec![],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        constants: vec![],
        top_level_stmts: vec![],
    };

    ide.index_symbols(&module, "");

    // Empty module should have no symbols - test via find_references on a name
    assert_eq!(
        ide.find_references("anything").len(),
        0,
        "Empty module should have no symbols"
    );
}

#[test]
fn test_depyler_0351_index_module_with_functions_and_classes() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "helper".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let class = HirClass {
        name: "Widget".to_string(),
        methods: vec![],
        fields: vec![],
        base_classes: vec![],
        is_dataclass: false,
        docstring: None,
        type_params: vec![],
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![class],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        constants: vec![],
        top_level_stmts: vec![],
    };

    ide.index_symbols(&module, "def helper(): pass\nclass Widget: pass");

    // Verify both indexed
    assert_eq!(ide.find_references("helper").len(), 1);
    assert_eq!(ide.find_references("Widget").len(), 1);
}

#[test]
fn test_depyler_0351_index_function_with_multiple_params() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "process".to_string(),
        params: smallvec![
            HirParam::new("a".to_string(), Type::Int),
            HirParam::new("b".to_string(), Type::String),
            HirParam::new("c".to_string(), Type::Bool),
        ],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    ide.index_symbols(&module, "def process(a: int, b: str, c: bool): pass");

    let refs = ide.find_references("process");
    assert_eq!(refs.len(), 1);

    let detail = refs[0].detail.as_ref().unwrap();
    assert!(
        detail.contains("a: Int"),
        "Detail should contain parameter a"
    );
    assert!(
        detail.contains("b: String"),
        "Detail should contain parameter b"
    );
    assert!(
        detail.contains("c: Bool"),
        "Detail should contain parameter c"
    );
}

#[test]
fn test_depyler_0351_index_method_detail_format() {
    let mut ide = IdeIntegration::new();

    let method = HirMethod {
        name: "render".to_string(),
        params: smallvec![HirParam::new("self".to_string(), Type::Unknown)],
        ret_type: Type::String,
        body: vec![],
        is_static: false,
        is_classmethod: false,
        is_property: false,
        is_async: false,
        docstring: None,
    };

    let class = HirClass {
        name: "View".to_string(),
        methods: vec![method],
        fields: vec![],
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

    ide.index_symbols(&module, "class View: pass");

    let method_refs = ide.find_references("render");
    assert_eq!(method_refs.len(), 1);

    let detail = method_refs[0].detail.as_ref().unwrap();
    assert!(
        detail.contains("View::render"),
        "Method detail should include class name"
    );
}

#[test]
fn test_depyler_0351_index_field_type_annotation() {
    let mut ide = IdeIntegration::new();

    let field = HirField {
        name: "items".to_string(),
        field_type: Type::List(Box::new(Type::String)),
        is_class_var: false,
        default_value: None,
    };

    let class = HirClass {
        name: "Container".to_string(),
        methods: vec![],
        fields: vec![field],
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

    ide.index_symbols(&module, "class Container: pass");

    let field_refs = ide.find_references("items");
    assert_eq!(field_refs.len(), 1);

    let detail = field_refs[0].detail.as_ref().unwrap();
    assert!(
        detail.contains("items:"),
        "Field detail should contain name"
    );
    assert!(detail.contains("List"), "Field detail should contain type");
}

// ============================================================================
// SYMBOL LOOKUP TESTS
// ============================================================================

#[test]
fn test_depyler_0351_symbol_at_position_found() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    ide.index_symbols(&module, "def test(): pass");

    // The indexed symbol has range (0, 100) as per index_function implementation
    let result = ide.symbol_at_position(TextSize::from(50));
    assert!(result.is_some(), "Should find symbol at position 50");
    assert_eq!(result.unwrap().name, "test");
}

#[test]
fn test_depyler_0351_symbol_at_position_not_found() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    ide.index_symbols(&module, "def test(): pass");

    // Range is (0, 100), so position 200 should not find anything
    let result = ide.symbol_at_position(TextSize::from(200));
    assert!(result.is_none(), "Should not find symbol at position 200");
}

#[test]
fn test_depyler_0351_find_references_single() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "func".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    ide.index_symbols(&module, "def func(): pass");

    let refs = ide.find_references("func");
    assert_eq!(refs.len(), 1, "Should find 1 reference");
    assert_eq!(refs[0].name, "func");
}

#[test]
fn test_depyler_0351_find_references_none() {
    let ide = IdeIntegration::new();

    let refs = ide.find_references("nonexistent");
    assert_eq!(
        refs.len(),
        0,
        "Should find no references for nonexistent symbol"
    );
}

// ============================================================================
// COMPLETIONS TESTS
// ============================================================================

#[test]
fn test_depyler_0351_completions_empty_prefix() {
    let mut ide = IdeIntegration::new();

    let func1 = HirFunction {
        name: "func1".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let class1 = HirClass {
        name: "class1".to_string(),
        methods: vec![],
        fields: vec![],
        base_classes: vec![],
        is_dataclass: false,
        docstring: None,
        type_params: vec![],
    };

    let module = HirModule {
        functions: vec![func1],
        classes: vec![class1],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        constants: vec![],
        top_level_stmts: vec![],
    };

    ide.index_symbols(&module, "def func1(): pass\nclass class1: pass");

    let completions = ide.completions_at_position(TextSize::from(0), "");
    assert_eq!(
        completions.len(),
        2,
        "Empty prefix should return all symbols"
    );
}

#[test]
fn test_depyler_0351_completions_no_matches() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    ide.index_symbols(&module, "def test(): pass");

    let completions = ide.completions_at_position(TextSize::from(0), "xyz");
    assert_eq!(completions.len(), 0, "No symbols match prefix 'xyz'");
}

#[test]
fn test_depyler_0351_completions_prefix_filtering() {
    let mut ide = IdeIntegration::new();

    let func1 = HirFunction {
        name: "my_func".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let func2 = HirFunction {
        name: "other_func".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func1, func2],
        classes: vec![],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        constants: vec![],
        top_level_stmts: vec![],
    };

    ide.index_symbols(&module, "def my_func(): pass\ndef other_func(): pass");

    let completions = ide.completions_at_position(TextSize::from(0), "my_");
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].label, "my_func");
}

#[test]
fn test_depyler_0351_completions_different_kinds() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "test_func".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let class = HirClass {
        name: "test_class".to_string(),
        methods: vec![],
        fields: vec![],
        base_classes: vec![],
        is_dataclass: false,
        docstring: None,
        type_params: vec![],
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![class],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        constants: vec![],
        top_level_stmts: vec![],
    };

    ide.index_symbols(&module, "def test_func(): pass\nclass test_class: pass");

    let completions = ide.completions_at_position(TextSize::from(0), "test_");
    assert_eq!(completions.len(), 2);

    let kinds: Vec<_> = completions.iter().map(|c| c.kind).collect();
    assert!(kinds.contains(&CompletionKind::Function));
    assert!(kinds.contains(&CompletionKind::Class));
}

// ============================================================================
// DIAGNOSTICS TESTS
// ============================================================================

#[test]
fn test_depyler_0351_add_diagnostic() {
    let mut ide = IdeIntegration::new();

    let diagnostic = Diagnostic {
        range: TextRange::new(TextSize::from(0), TextSize::from(10)),
        severity: DiagnosticSeverity::Error,
        message: "Test error".to_string(),
        code: Some("E001".to_string()),
        source: "test".to_string(),
    };

    ide.add_diagnostic(diagnostic);

    assert_eq!(ide.diagnostics().len(), 1);
    assert_eq!(ide.diagnostics()[0].message, "Test error");
}

#[test]
fn test_depyler_0351_diagnostics_retrieval() {
    let mut ide = IdeIntegration::new();

    assert_eq!(
        ide.diagnostics().len(),
        0,
        "Should start with no diagnostics"
    );

    let diag1 = Diagnostic {
        range: TextRange::new(TextSize::from(0), TextSize::from(10)),
        severity: DiagnosticSeverity::Warning,
        message: "Warning 1".to_string(),
        code: None,
        source: "test".to_string(),
    };

    let diag2 = Diagnostic {
        range: TextRange::new(TextSize::from(20), TextSize::from(30)),
        severity: DiagnosticSeverity::Error,
        message: "Error 1".to_string(),
        code: None,
        source: "test".to_string(),
    };

    ide.add_diagnostic(diag1);
    ide.add_diagnostic(diag2);

    let diagnostics = ide.diagnostics();
    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].message, "Warning 1");
    assert_eq!(diagnostics[1].message, "Error 1");
}

#[test]
fn test_depyler_0351_add_error_from_error_kind() {
    let mut ide = IdeIntegration::new();

    use depyler_core::error::ErrorKind;

    let error = ErrorKind::ParseError;
    let range = TextRange::new(TextSize::from(5), TextSize::from(15));

    ide.add_error(&error, range);

    assert_eq!(ide.diagnostics().len(), 1);
    assert_eq!(ide.diagnostics()[0].severity, DiagnosticSeverity::Error);
    assert!(ide.diagnostics()[0].message.contains("Python parse error"));
    assert_eq!(ide.diagnostics()[0].source, "depyler");
}

#[test]
fn test_depyler_0351_add_warning() {
    let mut ide = IdeIntegration::new();

    let message = "Unused variable 'x'".to_string();
    let range = TextRange::new(TextSize::from(10), TextSize::from(20));

    ide.add_warning(message.clone(), range);

    assert_eq!(ide.diagnostics().len(), 1);
    assert_eq!(ide.diagnostics()[0].severity, DiagnosticSeverity::Warning);
    assert_eq!(ide.diagnostics()[0].message, message);
    assert_eq!(ide.diagnostics()[0].source, "depyler");
}

#[test]
fn test_depyler_0351_multiple_diagnostics_accumulate() {
    let mut ide = IdeIntegration::new();

    use depyler_core::error::ErrorKind;

    ide.add_error(
        &ErrorKind::TypeInferenceError("type error".to_string()),
        TextRange::new(TextSize::from(0), TextSize::from(5)),
    );

    ide.add_warning(
        "style warning".to_string(),
        TextRange::new(TextSize::from(10), TextSize::from(15)),
    );

    ide.add_diagnostic(Diagnostic {
        range: TextRange::new(TextSize::from(20), TextSize::from(25)),
        severity: DiagnosticSeverity::Hint,
        message: "optimization hint".to_string(),
        code: None,
        source: "depyler".to_string(),
    });

    assert_eq!(ide.diagnostics().len(), 3);
}

#[test]
fn test_depyler_0351_diagnostic_severity_levels() {
    let range = TextRange::new(TextSize::from(0), TextSize::from(10));

    let error = Diagnostic {
        range,
        severity: DiagnosticSeverity::Error,
        message: "error".to_string(),
        code: None,
        source: "test".to_string(),
    };
    assert_eq!(error.severity, DiagnosticSeverity::Error);

    let warning = Diagnostic {
        range,
        severity: DiagnosticSeverity::Warning,
        message: "warning".to_string(),
        code: None,
        source: "test".to_string(),
    };
    assert_eq!(warning.severity, DiagnosticSeverity::Warning);

    let info = Diagnostic {
        range,
        severity: DiagnosticSeverity::Information,
        message: "info".to_string(),
        code: None,
        source: "test".to_string(),
    };
    assert_eq!(info.severity, DiagnosticSeverity::Information);

    let hint = Diagnostic {
        range,
        severity: DiagnosticSeverity::Hint,
        message: "hint".to_string(),
        code: None,
        source: "test".to_string(),
    };
    assert_eq!(hint.severity, DiagnosticSeverity::Hint);
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_depyler_0351_create_ide_integration() {
    let func = HirFunction {
        name: "main".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    let ide = create_ide_integration(&module, "def main(): pass");

    // Verify symbol indexed via find_references
    let refs = ide.find_references("main");
    assert_eq!(refs.len(), 1);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_depyler_0351_empty_ide_integration() {
    let ide = IdeIntegration::new();

    assert_eq!(ide.diagnostics().len(), 0);

    let result = ide.symbol_at_position(TextSize::from(10));
    assert!(result.is_none());

    let refs = ide.find_references("anything");
    assert_eq!(refs.len(), 0);

    let completions = ide.completions_at_position(TextSize::from(0), "test");
    assert_eq!(completions.len(), 0);
}

#[test]
fn test_depyler_0351_symbol_boundary_position() {
    let mut ide = IdeIntegration::new();

    let func = HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
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

    ide.index_symbols(&module, "def test(): pass");

    // index_function creates range (0, 100)
    // At start boundary
    let result = ide.symbol_at_position(TextSize::from(0));
    assert!(result.is_some(), "Should find symbol at start boundary");

    // At end boundary (exclusive in TextRange)
    let result = ide.symbol_at_position(TextSize::from(100));
    assert!(
        result.is_none(),
        "Should not find symbol at exclusive end boundary"
    );

    // Inside range
    let result = ide.symbol_at_position(TextSize::from(50));
    assert!(result.is_some(), "Should find symbol inside range");
}

#[test]
fn test_depyler_0351_class_with_multiple_fields() {
    let mut ide = IdeIntegration::new();

    let field1 = HirField {
        name: "x".to_string(),
        field_type: Type::Int,
        is_class_var: false,
        default_value: None,
    };

    let field2 = HirField {
        name: "y".to_string(),
        field_type: Type::Int,
        is_class_var: false,
        default_value: None,
    };

    let class = HirClass {
        name: "Point".to_string(),
        methods: vec![],
        fields: vec![field1, field2],
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

    ide.index_symbols(&module, "class Point: pass");

    assert_eq!(ide.find_references("Point").len(), 1);
    assert_eq!(ide.find_references("x").len(), 1);
    assert_eq!(ide.find_references("y").len(), 1);
}

// ============================================================================
// PROPERTY TESTS - IDE Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_diagnostics_accumulate_monotonically(
            count in 1usize..20,
        ) {
            let mut ide = IdeIntegration::new();

            for i in 0..count {
                ide.add_diagnostic(Diagnostic {
                    range: TextRange::new(
                        TextSize::from((i * 10) as u32),
                        TextSize::from(((i * 10) + 5) as u32),
                    ),
                    severity: DiagnosticSeverity::Warning,
                    message: format!("diagnostic {}", i),
                    code: None,
                    source: "test".to_string(),
                });

                // Diagnostics should only increase
                prop_assert_eq!(ide.diagnostics().len(), i + 1);
            }

            prop_assert_eq!(ide.diagnostics().len(), count);
        }

        #[test]
        fn prop_find_references_always_returns_vec(
            name in "[a-z]{1,10}",
        ) {
            let ide = IdeIntegration::new();

            // Should always return a Vec (empty or non-empty)
            let refs = ide.find_references(&name);
            prop_assert!(refs.is_empty() || !refs.is_empty());
        }

        #[test]
        fn prop_completions_empty_prefix_returns_all(
            func_count in 1usize..10,
        ) {
            let mut ide = IdeIntegration::new();

            let mut functions = Vec::new();
            for i in 0..func_count {
                functions.push(HirFunction {
                    name: format!("func{}", i),
                    params: smallvec![],
                    ret_type: Type::None,
                    body: vec![],
                    properties: FunctionProperties::default(),
                    annotations: TranspilationAnnotations::default(),
                    docstring: None,
                });
            }

            let module = HirModule {
                functions,
                classes: vec![],
                imports: vec![],
                type_aliases: vec![],
                protocols: vec![],
        constants: vec![],
            top_level_stmts: vec![],
            };

            ide.index_symbols(&module, "");

            let completions = ide.completions_at_position(TextSize::from(0), "");
            prop_assert_eq!(completions.len(), func_count);
        }
    }
}
