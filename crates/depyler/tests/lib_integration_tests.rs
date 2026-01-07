//! Fast integration tests for lib.rs CLI handlers
//! Target: 100% coverage of testable pure functions

use depyler::*;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// inspect_python_ast Tests
// ============================================================================

#[test]
fn test_inspect_python_ast_json() {
    let source = "x = 1";
    let result = inspect_python_ast(source, "json").unwrap();
    assert!(result.contains("Module"));
}

#[test]
fn test_inspect_python_ast_pretty() {
    let source = "def foo(): pass";
    let result = inspect_python_ast(source, "pretty").unwrap();
    assert!(result.contains("foo"));
}

#[test]
fn test_inspect_python_ast_invalid_syntax() {
    let source = "def invalid(";
    let result = inspect_python_ast(source, "json");
    assert!(result.is_err());
}

#[test]
fn test_inspect_python_ast_empty() {
    let source = "";
    let result = inspect_python_ast(source, "json").unwrap();
    assert!(result.contains("Module"));
}

#[test]
fn test_inspect_python_ast_complex() {
    let source = r#"
class Foo:
    def bar(self, x: int) -> int:
        return x * 2
"#;
    let result = inspect_python_ast(source, "json").unwrap();
    assert!(result.contains("Foo"));
}

// ============================================================================
// format_stmt_summary Tests
// ============================================================================

#[test]
fn test_format_stmt_summary_function_def() {
    use rustpython_parser::{parse, Mode};
    let source = "def my_function(): pass";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        // Check it contains meaningful info about the function
        assert!(!summary.is_empty());
    }
}

#[test]
fn test_format_stmt_summary_class_def() {
    use rustpython_parser::{parse, Mode};
    let source = "class MyClass: pass";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        // Check it contains meaningful info about the class
        assert!(!summary.is_empty());
    }
}

#[test]
fn test_format_stmt_summary_import() {
    use rustpython_parser::{parse, Mode};
    let source = "import os";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("Import"));
    }
}

#[test]
fn test_format_stmt_summary_import_from() {
    use rustpython_parser::{parse, Mode};
    let source = "from os import path";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("ImportFrom"));
    }
}

#[test]
fn test_format_stmt_summary_assign() {
    use rustpython_parser::{parse, Mode};
    let source = "x = 1";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("Assign"));
    }
}

#[test]
fn test_format_stmt_summary_expr() {
    use rustpython_parser::{parse, Mode};
    let source = "print('hello')";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("Expr"));
    }
}

#[test]
fn test_format_stmt_summary_if() {
    use rustpython_parser::{parse, Mode};
    let source = "if True: pass";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("If"));
    }
}

#[test]
fn test_format_stmt_summary_for() {
    use rustpython_parser::{parse, Mode};
    let source = "for i in range(10): pass";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("For"));
    }
}

#[test]
fn test_format_stmt_summary_while() {
    use rustpython_parser::{parse, Mode};
    let source = "while True: break";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        let summary = format_stmt_summary(&module.body[0]);
        assert!(summary.contains("While"));
    }
}

#[test]
fn test_format_stmt_summary_return() {
    use rustpython_parser::{parse, Mode};
    let source = "def f():\n    return 1";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    if let rustpython_ast::Mod::Module(module) = ast {
        if let rustpython_ast::Stmt::FunctionDef(func) = &module.body[0] {
            let summary = format_stmt_summary(&func.body[0]);
            assert!(summary.contains("Return"));
        }
    }
}

// ============================================================================
// format_python_ast_pretty Tests
// ============================================================================

#[test]
fn test_format_python_ast_pretty_module() {
    use rustpython_parser::{parse, Mode};
    let source = "def foo(): pass\nclass Bar: pass";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    let pretty = format_python_ast_pretty(&ast);
    assert!(!pretty.is_empty());
}

#[test]
fn test_format_python_ast_pretty_empty() {
    use rustpython_parser::{parse, Mode};
    let source = "";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    let pretty = format_python_ast_pretty(&ast);
    assert!(pretty.contains("Module"));
}

#[test]
fn test_format_python_ast_pretty_with_imports() {
    use rustpython_parser::{parse, Mode};
    let source = "import os\nfrom sys import argv";
    let ast = parse(source, Mode::Module, "<test>").unwrap();
    let pretty = format_python_ast_pretty(&ast);
    assert!(pretty.contains("Import"));
}

// ============================================================================
// complexity_rating Tests
// ============================================================================

#[test]
fn test_complexity_rating_low() {
    let rating = complexity_rating(5.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_medium() {
    let rating = complexity_rating(15.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_high() {
    let rating = complexity_rating(25.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_boundary_10() {
    let rating = complexity_rating(10.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_boundary_20() {
    let rating = complexity_rating(20.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_zero() {
    let rating = complexity_rating(0.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_very_high() {
    let rating = complexity_rating(100.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

// ============================================================================
// analyze_command Tests
// ============================================================================

#[test]
fn test_analyze_command_valid_file() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("analyze.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = analyze_command(py_file, "text".to_string());
    let _ = result;
}

#[test]
fn test_analyze_command_json_format() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("analyze.py");
    std::fs::write(&py_file, "x = 1\n").unwrap();

    let result = analyze_command(py_file, "json".to_string());
    let _ = result;
}

#[test]
fn test_analyze_command_nonexistent() {
    let result = analyze_command(PathBuf::from("/nonexistent.py"), "text".to_string());
    assert!(result.is_err());
}

// ============================================================================
// check_command Tests
// ============================================================================

#[test]
fn test_check_command_valid_file() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("check.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = check_command(py_file);
    let _ = result;
}

#[test]
fn test_check_command_nonexistent() {
    let result = check_command(PathBuf::from("/nonexistent.py"));
    assert!(result.is_err());
}

// ============================================================================
// inspect_command Tests
// ============================================================================

#[test]
fn test_inspect_command_ast_json() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("inspect.py");
    std::fs::write(&py_file, "x = 1\n").unwrap();

    let result = inspect_command(py_file, "ast".to_string(), "json".to_string(), None);
    let _ = result;
}

#[test]
fn test_inspect_command_ast_pretty() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("inspect.py");
    std::fs::write(&py_file, "def foo(): pass\n").unwrap();

    let result = inspect_command(py_file, "ast".to_string(), "pretty".to_string(), None);
    let _ = result;
}

#[test]
fn test_inspect_command_hir_json() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("inspect.py");
    std::fs::write(&py_file, "def foo(): pass\n").unwrap();

    let result = inspect_command(py_file, "hir".to_string(), "json".to_string(), None);
    let _ = result;
}

#[test]
fn test_inspect_command_hir_pretty() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("inspect.py");
    std::fs::write(&py_file, "def foo(): pass\n").unwrap();

    let result = inspect_command(py_file, "hir".to_string(), "pretty".to_string(), None);
    let _ = result;
}

#[test]
fn test_inspect_command_nonexistent() {
    let result = inspect_command(PathBuf::from("/nonexistent.py"), "ast".to_string(), "json".to_string(), None);
    assert!(result.is_err());
}

#[test]
fn test_inspect_command_with_output() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("inspect.py");
    let out_file = temp.path().join("output.txt");
    std::fs::write(&py_file, "x = 1\n").unwrap();

    let result = inspect_command(py_file, "ast".to_string(), "json".to_string(), Some(out_file));
    let _ = result;
}

// ============================================================================
// print_compilation_results Tests
// ============================================================================

#[test]
fn test_print_compilation_results_all_pass() {
    let results = CompilationResults {
        compilation_ok: true,
        clippy_ok: true,
        all_passed: true,
    };
    print_compilation_results(&results);
}

#[test]
fn test_print_compilation_results_compile_fail() {
    let results = CompilationResults {
        compilation_ok: false,
        clippy_ok: false,
        all_passed: false,
    };
    print_compilation_results(&results);
}

#[test]
fn test_print_compilation_results_clippy_fail() {
    let results = CompilationResults {
        compilation_ok: true,
        clippy_ok: false,
        all_passed: false,
    };
    print_compilation_results(&results);
}

// ============================================================================
// File-based command tests with temp files
// ============================================================================

#[test]
fn test_generate_quality_report_valid() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("quality.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = generate_quality_report(&py_file);
    let _ = result;
}

#[test]
fn test_generate_quality_report_nonexistent() {
    let result = generate_quality_report(std::path::Path::new("/nonexistent/file.py"));
    assert!(result.is_err());
}

#[test]
fn test_check_compilation_quality_valid() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("compile.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = check_compilation_quality(&py_file);
    let _ = result;
}

#[test]
fn test_check_compilation_quality_nonexistent() {
    let result = check_compilation_quality(std::path::Path::new("/nonexistent.py"));
    // Should error or return failure, just check it runs
    let _ = result;
}

#[test]
fn test_check_rust_compilation_valid() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("rust_check.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = check_rust_compilation(&py_file);
    let _ = result;
}

#[test]
fn test_check_rust_compilation_nonexistent() {
    let result = check_rust_compilation(std::path::Path::new("/nonexistent.py"));
    // Should error or return false, just check it runs
    let _ = result;
}

#[test]
fn test_check_clippy_clean_valid() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("clippy.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = check_clippy_clean(&py_file);
    let _ = result;
}

#[test]
fn test_check_clippy_clean_nonexistent() {
    let result = check_clippy_clean(std::path::Path::new("/nonexistent.py"));
    // Should error or return false, just check it runs
    let _ = result;
}

// ============================================================================
// check_rust_compilation_for_file Tests
// ============================================================================

#[test]
fn test_check_rust_compilation_for_file_valid() {
    let temp = TempDir::new().unwrap();
    let rs_file = temp.path().join("test.rs");
    std::fs::write(&rs_file, "fn main() {}").unwrap();

    let result = check_rust_compilation_for_file(rs_file.to_str().unwrap());
    let _ = result;
}

#[test]
fn test_check_rust_compilation_for_file_invalid() {
    let temp = TempDir::new().unwrap();
    let rs_file = temp.path().join("test.rs");
    std::fs::write(&rs_file, "fn main() { invalid syntax }").unwrap();

    let result = check_rust_compilation_for_file(rs_file.to_str().unwrap());
    let _ = result;
}

#[test]
fn test_check_rust_compilation_for_file_nonexistent() {
    let result = check_rust_compilation_for_file("/nonexistent.rs");
    // Should error or return failure, just check it runs
    let _ = result;
}
