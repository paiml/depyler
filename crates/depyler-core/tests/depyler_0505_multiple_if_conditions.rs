//! DEPYLER-0505: Multiple if conditions in list comprehension
//!
//! RED → GREEN Phase: List comprehension with multiple if clauses on single generator
//!
//! Root Cause (Five-Whys):
//! 1. Why? "Multiple conditions in list comprehension not yet supported"
//! 2. Why? convert_list_comp() only handles generator.ifs.len() <= 1
//! 3. Why? Code explicitly checks `if generator.ifs.len() == 1` then bails otherwise
//! 4. Why? Original implementation assumed single condition
//! 5. ROOT: Missing logical chaining of multiple conditions with && operator
//!
//! Python semantics: `[x for x in range(10) if x > 5 if x % 2 == 0]`
//! Equivalent to: `[x for x in range(10) if (x > 5) and (x % 2 == 0)]`
//! Rust: filter(|x| x > 5 && x % 2 == 0)

use depyler_core::ast_bridge;
use rustpython_ast::Suite;
use rustpython_parser::{ast, Parse};

fn parse_and_generate(python: &str) -> depyler_core::hir::HirModule {
    let statements = Suite::parse(python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });
    let (hir, _type_env) = ast_bridge::AstBridge::new()
        .python_to_hir(ast)
        .expect("Should generate HIR");
    hir
}

#[test]
fn test_double_if_condition() {
    // Minimal reproducer: two if conditions
    let python = r#"
result = [x for x in range(10) if x > 5 if x % 2 == 0]
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

#[test]
fn test_triple_if_condition() {
    // Three if conditions
    let python = r#"
result = [x for x in range(20) if x > 5 if x < 15 if x % 2 == 0]
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

#[test]
fn test_multiple_conditions_with_complex_expression() {
    // Multiple if with complex expressions
    let python = r#"
data = [{"value": i} for i in range(10)]
filtered = [d["value"] for d in data if d["value"] > 3 if d["value"] < 8]
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 2);
}
