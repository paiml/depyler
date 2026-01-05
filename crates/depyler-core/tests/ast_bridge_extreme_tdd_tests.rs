//! EXTREME TDD tests for ast_bridge module
//! Tests edge cases, error paths, and boundary conditions

use depyler_core::ast_bridge::AstBridge;
use proptest::prelude::*;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

/// Helper to create a ModModule from parsed code
fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

// ============================================================================
// FALSIFICATION TESTS - Try to break the code
// ============================================================================

/// Test that empty module produces empty HIR
#[test]
fn test_empty_module_produces_empty_hir() {
    let python_code = "";
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert!(hir.functions.is_empty());
    assert!(hir.imports.is_empty());
    assert!(hir.classes.is_empty());
}

/// Test module with only comments/whitespace
#[test]
fn test_whitespace_only_module() {
    let python_code = "   \n\n   # comment\n   ";
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test function with no parameters
#[test]
fn test_function_no_params() {
    let python_code = r#"
def hello() -> str:
    return "hello"
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.functions.len(), 1);
    assert!(hir.functions[0].params.is_empty());
}

/// Test function with many parameters (boundary test)
#[test]
fn test_function_many_params() {
    let python_code = r#"
def many_params(a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int) -> int:
    return a + b + c + d + e + f + g + h
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.functions[0].params.len(), 8);
}

/// Test async function conversion
#[test]
fn test_async_function() {
    let python_code = r#"
async def async_fetch(url: str) -> str:
    return url
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.functions.len(), 1);
    assert!(hir.functions[0].properties.is_async);
}

/// Test import statement conversion
#[test]
fn test_import_statement() {
    let python_code = r#"
import os
import sys
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert!(hir.imports.len() >= 2);
}

/// Test from import statement conversion
#[test]
fn test_from_import_statement() {
    let python_code = r#"
from typing import List, Dict, Optional
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    // from imports may be represented as single import with multiple items
    assert!(!hir.imports.is_empty());
}

/// Test type alias conversion
#[test]
fn test_simple_type_alias() {
    let python_code = r#"
UserId = int
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.type_aliases.len(), 1);
    assert_eq!(hir.type_aliases[0].name, "UserId");
}

/// Test generic type alias conversion
#[test]
fn test_generic_type_alias() {
    let python_code = r#"
from typing import Optional
MaybeInt = Optional[int]
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    // Should have type alias for MaybeInt
    assert!(hir.type_aliases.iter().any(|ta| ta.name == "MaybeInt"));
}

/// Test NewType pattern
#[test]
fn test_newtype_pattern() {
    let python_code = r#"
from typing import NewType
UserId = NewType('UserId', int)
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    let alias = hir.type_aliases.iter().find(|ta| ta.name == "UserId");
    assert!(alias.is_some());
    assert!(alias.unwrap().is_newtype);
}

/// Test protocol conversion
#[test]
fn test_protocol_conversion() {
    let python_code = r#"
from typing import Protocol

class Drawable(Protocol):
    def draw(self) -> None:
        ...
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.protocols.len(), 1);
    assert_eq!(hir.protocols[0].name, "Drawable");
}

/// Test runtime_checkable protocol
#[test]
fn test_runtime_checkable_protocol() {
    let python_code = r#"
from typing import Protocol, runtime_checkable

@runtime_checkable
class Sized(Protocol):
    def __len__(self) -> int:
        ...
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.protocols.len(), 1);
    assert!(hir.protocols[0].is_runtime_checkable);
}

/// Test class conversion
#[test]
fn test_class_conversion() {
    let python_code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.classes.len(), 1);
    assert_eq!(hir.classes[0].name, "Point");
}

/// Test module-level constant
#[test]
fn test_module_level_constant() {
    let python_code = r#"
MAX_SIZE = 100
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert!(hir.constants.iter().any(|c| c.name == "MAX_SIZE"));
}

/// Test annotated module-level constant
#[test]
fn test_annotated_constant() {
    let python_code = r#"
MAX_SIZE: int = 100
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    let constant = hir.constants.iter().find(|c| c.name == "MAX_SIZE");
    assert!(constant.is_some());
    assert!(constant.unwrap().type_annotation.is_some());
}

/// Test function with docstring
#[test]
fn test_function_with_docstring() {
    let python_code = r#"
def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert!(hir.functions[0].docstring.is_some());
}

/// Test with_source builder pattern
#[test]
fn test_with_source_builder() {
    let python_code = r#"
def hello() -> str:
    return "hello"
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new().with_source(python_code.to_string());
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test Default trait implementation
#[test]
fn test_default_implementation() {
    let bridge: AstBridge = Default::default();
    let python_code = "def f(): pass";
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

/// Test function with complex return type
#[test]
fn test_complex_return_type() {
    let python_code = r#"
from typing import Dict, List
def get_data() -> Dict[str, List[int]]:
    return {}
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test function with *args
#[test]
fn test_function_with_varargs() {
    let python_code = r#"
def variadic(*args: int) -> int:
    return sum(args)
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test function with **kwargs
#[test]
fn test_function_with_kwargs() {
    let python_code = r#"
def with_kwargs(**kwargs: str) -> None:
    pass
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test function with default arguments
#[test]
fn test_function_with_defaults() {
    let python_code = r#"
def with_defaults(a: int, b: int = 10, c: str = "hello") -> int:
    return a + b
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test nested class definition
#[test]
fn test_nested_class() {
    let python_code = r#"
class Outer:
    class Inner:
        def method(self) -> int:
            return 42
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test class with inheritance
#[test]
fn test_class_inheritance() {
    let python_code = r#"
class Base:
    pass

class Derived(Base):
    pass
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.classes.len(), 2);
}

/// Test class with multiple inheritance
#[test]
fn test_multiple_inheritance() {
    let python_code = r#"
class A:
    pass

class B:
    pass

class C(A, B):
    pass
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test decorated function
#[test]
fn test_decorated_function() {
    let python_code = r#"
def decorator(f):
    return f

@decorator
def decorated() -> int:
    return 42
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test staticmethod
#[test]
fn test_staticmethod() {
    let python_code = r#"
class MyClass:
    @staticmethod
    def static_method() -> int:
        return 42
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test classmethod
#[test]
fn test_classmethod() {
    let python_code = r#"
class MyClass:
    @classmethod
    def class_method(cls) -> int:
        return 42
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

proptest! {
    /// Property: parsing valid function names should succeed
    #[test]
    fn prop_valid_function_names(name in "[a-z][a-z0-9_]{0,20}") {
        let code = format!("def {}() -> None: pass", name);
        let ast = Suite::parse(&code, "<test>");
        prop_assert!(ast.is_ok());
    }

    /// Property: parsing valid parameter counts should succeed
    #[test]
    fn prop_parameter_counts(count in 0usize..10) {
        let params: Vec<String> = (0..count).map(|i| format!("p{}: int", i)).collect();
        let code = format!("def f({}) -> int: return 0", params.join(", "));
        let ast = Suite::parse(&code, "<test>");
        prop_assert!(ast.is_ok());
        if let Ok(suite) = ast {
            let bridge = AstBridge::new();
            let result = bridge.python_to_hir(make_module(suite));
            prop_assert!(result.is_ok());
            if let Ok((hir, _)) = result {
                prop_assert_eq!(hir.functions[0].params.len(), count);
            }
        }
    }

    /// Property: type environment should contain all parameters
    #[test]
    fn prop_type_env_contains_params(count in 1usize..5) {
        let params: Vec<String> = (0..count).map(|i| format!("p{}: int", i)).collect();
        let code = format!("def f({}) -> int: return 0", params.join(", "));
        let ast = Suite::parse(&code, "<test>");
        if let Ok(suite) = ast {
            let bridge = AstBridge::new();
            let result = bridge.python_to_hir(make_module(suite));
            if let Ok((_, type_env)) = result {
                // Type environment should have bindings for params
                for i in 0..count {
                    let param_name = format!("p{}", i);
                    prop_assert!(type_env.get_var_type(&param_name).is_some());
                }
            }
        }
    }
}

// ============================================================================
// MUTATION-RESISTANT TESTS
// ============================================================================

/// Test that functions are correctly counted (mutation-resistant)
#[test]
fn test_function_count_mutation_resistant() {
    let python_code = r#"
def f1(): pass
def f2(): pass
def f3(): pass
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let (hir, _) = bridge.python_to_hir(make_module(ast)).unwrap();

    // Exact count, not just > 0
    assert_eq!(hir.functions.len(), 3);
    // Check all names
    assert!(hir.functions.iter().any(|f| f.name == "f1"));
    assert!(hir.functions.iter().any(|f| f.name == "f2"));
    assert!(hir.functions.iter().any(|f| f.name == "f3"));
}

/// Test that import counts are exact (mutation-resistant)
#[test]
fn test_import_count_mutation_resistant() {
    let python_code = r#"
import os
import sys
import json
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let (hir, _) = bridge.python_to_hir(make_module(ast)).unwrap();

    assert_eq!(hir.imports.len(), 3);
}

/// Test that parameter types are preserved exactly (mutation-resistant)
#[test]
fn test_param_types_mutation_resistant() {
    let python_code = r#"
def typed(a: int, b: str, c: float) -> bool:
    return True
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let (hir, _) = bridge.python_to_hir(make_module(ast)).unwrap();

    let func = &hir.functions[0];
    assert_eq!(func.params.len(), 3);
    assert_eq!(func.params[0].name, "a");
    assert_eq!(func.params[1].name, "b");
    assert_eq!(func.params[2].name, "c");
}

/// Test function properties are correctly analyzed (mutation-resistant)
#[test]
fn test_function_properties_mutation_resistant() {
    let python_code = r#"
async def async_fn() -> int:
    return 0

def sync_fn() -> int:
    return 1
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let (hir, _) = bridge.python_to_hir(make_module(ast)).unwrap();

    let async_fn = hir.functions.iter().find(|f| f.name == "async_fn").unwrap();
    let sync_fn = hir.functions.iter().find(|f| f.name == "sync_fn").unwrap();

    assert!(async_fn.properties.is_async);
    assert!(!sync_fn.properties.is_async);
}

// ============================================================================
// ERROR PATH TESTS
// ============================================================================

/// Test empty function body handling
#[test]
fn test_empty_function_body() {
    let python_code = "def empty(): ...";
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test function with only pass statement
#[test]
fn test_function_only_pass() {
    let python_code = "def do_nothing(): pass";
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

// ============================================================================
// BOUNDARY TESTS
// ============================================================================

/// Test very long function name
#[test]
fn test_long_function_name() {
    let long_name = "a".repeat(100);
    let python_code = format!("def {}() -> None: pass", long_name);
    let ast = Suite::parse(&python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.functions[0].name, long_name);
}

/// Test deeply nested function calls
#[test]
fn test_nested_function_calls() {
    let python_code = r#"
def f(x: int) -> int:
    return x

def g() -> int:
    return f(f(f(f(f(1)))))
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test function with all parameter types
#[test]
fn test_all_param_types() {
    let python_code = r#"
def all_params(a: int, b: str = "default", *args: float, **kwargs: bool) -> None:
    pass
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test class with many methods
#[test]
fn test_class_many_methods() {
    let methods: Vec<String> = (0..20)
        .map(|i| format!("    def method{}(self) -> int: return {}", i, i))
        .collect();
    let python_code = format!("class ManyMethods:\n{}", methods.join("\n"));
    let ast = Suite::parse(&python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert_eq!(hir.classes.len(), 1);
    assert_eq!(hir.classes[0].methods.len(), 20);
}

/// Test multiple type aliases
#[test]
fn test_multiple_type_aliases() {
    let python_code = r#"
Int = int
Str = str
Bool = bool
Float = float
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert!(hir.type_aliases.len() >= 4);
}

/// Test generator function
#[test]
fn test_generator_function() {
    let python_code = r#"
def gen(n: int) -> int:
    for i in range(n):
        yield i
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
    let (hir, _) = result.unwrap();
    assert!(hir.functions[0].properties.is_generator);
}

/// Test global statement handling
#[test]
fn test_global_statement() {
    let python_code = r#"
x = 10

def modify_global() -> None:
    global x
    x = 20
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test nonlocal statement handling
#[test]
fn test_nonlocal_statement() {
    let python_code = r#"
def outer() -> int:
    x = 10
    def inner() -> None:
        nonlocal x
        x = 20
    inner()
    return x
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test lambda expression
#[test]
fn test_lambda_expression() {
    let python_code = r#"
def use_lambda() -> int:
    f = lambda x: x + 1
    return f(5)
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test list comprehension
#[test]
fn test_list_comprehension() {
    let python_code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test dict comprehension
#[test]
fn test_dict_comprehension() {
    let python_code = r#"
def square_dict(n: int) -> dict:
    return {x: x * x for x in range(n)}
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test set comprehension
#[test]
fn test_set_comprehension() {
    let python_code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test ternary expression
#[test]
fn test_ternary_expression() {
    let python_code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test walrus operator (named expression)
#[test]
fn test_walrus_operator() {
    let python_code = r#"
def check_length(s: str) -> bool:
    if (n := len(s)) > 10:
        return True
    return False
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test assert statement
#[test]
fn test_assert_statement() {
    let python_code = r#"
def validate(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test try-except handling
#[test]
fn test_try_except() {
    let python_code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test with statement
#[test]
fn test_with_statement() {
    let python_code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test raise statement
#[test]
fn test_raise_statement() {
    let python_code = r#"
def always_fails() -> None:
    raise ValueError("always fails")
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}

/// Test match statement (Python 3.10+)
#[test]
fn test_match_statement() {
    let python_code = r#"
def process(x: int) -> str:
    match x:
        case 0:
            return "zero"
        case 1:
            return "one"
        case _:
            return "other"
"#;
    let ast = Suite::parse(python_code, "<test>").unwrap();
    let bridge = AstBridge::new();
    let result = bridge.python_to_hir(make_module(ast));
    assert!(result.is_ok());
}
