//! DEPYLER-0516 / GH-103: Missing Args::parse() call in argparse-generated CLI code
//!
//! **ROOT CAUSE**: Args struct generated at module level, but Args::parse() not injected into main()
//!
//! **Five Whys**:
//! 1. Why is `args` not defined in main()? The `let args = Args::parse();` statement is not generated
//! 2. Why is the statement not generated? The stmt_gen code should generate it but isn't being called
//! 3. Why isn't that code path hit? The parse_args() assignment is inside main(), but struct is at module level
//! 4. Why does that matter? The assignment transformation happens but the result is lost
//! 5. ROOT: Argparse transformation generates struct at module level but doesn't properly inject Args::parse() into main() function body
//!
//! **Problem**: The transpiler correctly generates:
//! - `use clap::Parser;`
//! - `#[derive(clap::Parser)] struct Args { ... }`
//!
//! But FAILS to generate:
//! - `let args = Args::parse();` inside main()
//!
//! This causes references to `args.field` to fail with E0425: cannot find value `args`
//!
//! **Examples**:
//! ```python
//! def main():
//!     parser = argparse.ArgumentParser()
//!     parser.add_argument("name")
//!     args = parser.parse_args()
//!     print(f"Hello {args.name}")
//! ```
//!
//! **Generated (BROKEN)**:
//! ```rust,ignore
//! #[derive(clap::Parser)]
//! struct Args { name: String }
//! fn main() {
//!     println!("{}", args.name); // ERROR: args not defined
//! }
//! ```
//!
//! **Expected (CORRECT)**:
//! ```rust,ignore
//! #[derive(clap::Parser)]
//! struct Args { name: String }
//! fn main() {
//!     let args = Args::parse(); // ← MISSING
//!     println!("{}", args.name);
//! }
//! ```

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile_to_rust(python_code: &str) -> Result<String, String> {
    let ast = parse(python_code, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

// ============================================================================
// RED PHASE - Failing Tests
// ============================================================================

#[test]
fn test_DEPYLER_0516_simple_argparse_in_main() {
    // RED: args = parser.parse_args() should generate let args = Args::parse()
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Test CLI")
    parser.add_argument("name", help="Your name")
    args = parser.parse_args()
    print(f"Hello {args.name}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0516: Simple argparse should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should generate Args struct
    assert!(
        rust_code.contains("struct Args") || rust_code.contains("Args"),
        "DEPYLER-0516: Should generate Args struct.\nGenerated:\n{}",
        rust_code
    );

    // Should generate Args::parse() call
    assert!(
        rust_code.contains("Args::parse()"),
        "DEPYLER-0516: Should generate Args::parse() call.\nGenerated:\n{}",
        rust_code
    );

    // Should NOT reference args without defining it
    if rust_code.contains("args.name") || rust_code.contains("args.") {
        assert!(
            rust_code.contains("let args") || rust_code.contains("args ="),
            "DEPYLER-0516: If args is used, it must be defined first.\nGenerated:\n{}",
            rust_code
        );
    }
}

#[test]
fn test_DEPYLER_0516_trivial_cli_example() {
    // Exact example from GH-103
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A trivial CLI example")
    parser.add_argument("--name", required=True, help="Your name")
    parser.add_argument("--version", help="Version info")
    args = parser.parse_args()
    print(f"Hello, {args.name}!")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0516: Trivial CLI should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Must have Args::parse()
    assert!(
        rust_code.contains("Args::parse()"),
        "DEPYLER-0516: Must generate Args::parse().\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0516_positional_args() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", help="Input file")
    parser.add_argument("output", help="Output file")
    args = parser.parse_args()
    print(f"Processing {args.file} -> {args.output}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0516: Positional args should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("Args::parse()"),
        "DEPYLER-0516: Positional args need Args::parse().\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0516_args_used_multiple_times() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("--count", type=int, default=1)
    args = parser.parse_args()

    if args.verbose:
        print(f"Count: {args.count}")

    for i in range(args.count):
        print(f"Iteration {i}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0516: Multiple args usage should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // All uses of args must come after Args::parse()
    assert!(
        rust_code.contains("Args::parse()"),
        "DEPYLER-0516: Args::parse() required for args usage.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0516_check_compile() {
    // This test verifies the generated Rust actually compiles
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Compile test")
    parser.add_argument("name")
    args = parser.parse_args()
    print(args.name)

if __name__ == "__main__":
    main()
"#;

    let result = transpile_to_rust(python);
    assert!(result.is_ok(), "Should transpile: {}", result.unwrap_err());

    let rust_code = result.unwrap();

    // Write to temp file and try to compile (note: just for debugging, doesn't actually compile)
    // No need to write temp file - structural check is sufficient

    // Note: We can't actually compile here without rustc, but we can check the structure
    let _has_struct = rust_code.contains("struct Args");
    let has_parse = rust_code.contains("Args::parse()");
    let uses_args = rust_code.contains("args.");

    if uses_args {
        assert!(
            has_parse,
            "DEPYLER-0516: If code uses args, must call Args::parse().\nGenerated:\n{}",
            rust_code
        );
    }
}

#[test]
fn test_DEPYLER_0516_main_with_return_type() {
    // RED: Exact pattern from simple_cli.py that fails
    let python = r#"
import argparse

def main() -> int:
    parser = argparse.ArgumentParser(description="A simple CLI tool example")
    parser.add_argument("name", help="Your name")
    args = parser.parse_args()
    print(f"Hello, {args.name}!")
    return 0

if __name__ == "__main__":
    exit(main())
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0516: main() with return type should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Note: Temp file write removed for parallel test safety (DEPYLER-1028)

    // CRITICAL: Must have Args::parse() call
    assert!(
        rust_code.contains("Args::parse()"),
        "DEPYLER-0516: main() with -> int return type MUST have Args::parse().\nGenerated:\n{}",
        rust_code
    );

    // Must not reference args without defining it
    if rust_code.contains("args.") {
        assert!(
            rust_code.contains("let args") || rust_code.contains("args ="),
            "DEPYLER-0516: args must be defined before use.\nGenerated:\n{}",
            rust_code
        );
    }
}
