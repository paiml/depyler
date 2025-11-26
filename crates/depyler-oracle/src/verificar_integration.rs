//! Integration with verificar for synthetic training data generation.
//!
//! Extracts error patterns from verificar-generated test corpus to expand oracle training.

use crate::classifier::ErrorCategory;
use crate::training::{TrainingDataset, TrainingSample};
use std::collections::HashMap;

/// Known error code to category mapping based on verificar corpus analysis.
const ERROR_CATEGORY_MAP: &[(&str, ErrorCategory, &str)] = &[
    // E0308 - Type mismatches (most common in depyler output)
    ("E0308", ErrorCategory::TypeMismatch, "Check type inference for parameters and return types"),
    // E0432 - Unresolved imports (serde_json, chrono, etc.)
    ("E0432", ErrorCategory::MissingImport, "Add missing crate dependency to Cargo.toml"),
    // E0277 - Trait bounds not satisfied
    ("E0277", ErrorCategory::TraitBound, "Implement required trait or change type"),
    // E0425 - Cannot find value/function in scope
    ("E0425", ErrorCategory::MissingImport, "Variable not declared or out of scope"),
    // E0599 - No method found on type
    ("E0599", ErrorCategory::TraitBound, "Method doesn't exist on this type - check stdlib mapping"),
    // E0609 - No field on type
    ("E0609", ErrorCategory::TypeMismatch, "Struct field access failed - check class codegen"),
    // E0282 - Type annotations needed
    ("E0282", ErrorCategory::TypeMismatch, "Add explicit type annotation"),
    // E0061 - Wrong number of arguments
    ("E0061", ErrorCategory::SyntaxError, "Function call has wrong argument count"),
    // E0596 - Cannot borrow as mutable
    ("E0596", ErrorCategory::BorrowChecker, "Add mut to variable declaration"),
];

/// Synthetic error patterns from verificar corpus analysis.
#[must_use]
pub fn build_verificar_corpus() -> TrainingDataset {
    let mut dataset = TrainingDataset::new();

    // Add patterns from verificar verification runs
    add_type_mismatch_patterns(&mut dataset);
    add_import_patterns(&mut dataset);
    add_trait_bound_patterns(&mut dataset);
    add_scope_patterns(&mut dataset);
    add_borrow_patterns(&mut dataset);
    add_extended_patterns(&mut dataset);

    dataset
}

fn add_type_mismatch_patterns(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        // Option/PathBuf patterns from test_000-003
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `PathBuf`, found `Option<PathBuf>`",
            ErrorCategory::TypeMismatch,
            "Unwrap Option: path.unwrap_or_else(|| PathBuf::from(\"default\"))",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&Path`, found `PathBuf`",
            ErrorCategory::TypeMismatch,
            "Add reference: &path or path.as_path()",
        ),
        TrainingSample::with_fix(
            "error[E0308]: `?` operator has incompatible types expected `io::Error`, found `serde_json::Error`",
            ErrorCategory::TypeMismatch,
            "Use .map_err() or anyhow for error conversion",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `String`, found `&str`",
            ErrorCategory::TypeMismatch,
            "Add .to_string() for owned String",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&str`, found `String`",
            ErrorCategory::TypeMismatch,
            "Add & or .as_str() for string slice",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `i32`, found `i64`",
            ErrorCategory::TypeMismatch,
            "Add explicit cast: value as i32",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `Vec<String>`, found `Vec<&str>`",
            ErrorCategory::TypeMismatch,
            "Map to owned strings: .iter().map(|s| s.to_string()).collect()",
        ),
        TrainingSample::with_fix(
            "error[E0308]: arguments to this function are incorrect",
            ErrorCategory::TypeMismatch,
            "Check function signature and argument types",
        ),
    ]);
}

fn add_import_patterns(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `serde_json`",
            ErrorCategory::MissingImport,
            "Add serde_json = \"1.0\" to [dependencies] in Cargo.toml",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `chrono`",
            ErrorCategory::MissingImport,
            "Add chrono = \"0.4\" to [dependencies] in Cargo.toml",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `regex`",
            ErrorCategory::MissingImport,
            "Add regex = \"1\" to [dependencies] in Cargo.toml",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `sha2`",
            ErrorCategory::MissingImport,
            "Add sha2 = \"0.10\" to [dependencies] in Cargo.toml",
        ),
        TrainingSample::with_fix(
            "error[E0425]: cannot find function `open` in this scope",
            ErrorCategory::MissingImport,
            "Use std::fs::File::open() or std::fs::read_to_string()",
        ),
    ]);
}

fn add_trait_bound_patterns(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `String: Borrow<&str>` is not satisfied",
            ErrorCategory::TraitBound,
            "HashMap key type issue: use consistent &str or String",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `bool: AsRef<Path>` is not satisfied",
            ErrorCategory::TraitBound,
            "Conditional path: use if/else to return PathBuf not bool",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the `?` operator can only be used in a function that returns `Result`",
            ErrorCategory::TraitBound,
            "Change return type to Result<T, E> or use .unwrap()/.expect()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `write_all` found for struct `String`",
            ErrorCategory::TraitBound,
            "String is not a file - use File or BufWriter",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `read_to_string` found",
            ErrorCategory::TraitBound,
            "Use std::io::Read trait or std::fs::read_to_string(path)",
        ),
    ]);
}

fn add_scope_patterns(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `r#override` in this scope",
            ErrorCategory::MissingImport,
            "Python keyword 'override' needs raw identifier r# prefix",
        ),
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `verbose` in this scope",
            ErrorCategory::MissingImport,
            "Variable not passed to function - add as parameter",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `output` on type `&Logger`",
            ErrorCategory::TypeMismatch,
            "Struct field not defined - check class __init__ translation",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `start` on type `&mut Timer`",
            ErrorCategory::TypeMismatch,
            "Context manager __enter__ fields not in struct definition",
        ),
    ]);
}

fn add_borrow_patterns(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0596]: cannot borrow `_context` as mutable, as it is not declared as mutable",
            ErrorCategory::BorrowChecker,
            "Add 'mut' to variable declaration: let mut _context = ...",
        ),
        TrainingSample::with_fix(
            "error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable",
            ErrorCategory::BorrowChecker,
            "Restructure code to avoid overlapping borrows",
        ),
        TrainingSample::with_fix(
            "error[E0499]: cannot borrow `*self` as mutable more than once",
            ErrorCategory::BorrowChecker,
            "Use Cell/RefCell for interior mutability or restructure",
        ),
        TrainingSample::with_fix(
            "error[E0507]: cannot move out of borrowed content",
            ErrorCategory::BorrowChecker,
            "Use .clone() or change to reference",
        ),
        TrainingSample::with_fix(
            "error[E0382]: borrow of moved value",
            ErrorCategory::BorrowChecker,
            "Clone value before move or restructure ownership",
        ),
    ]);
}

/// Additional synthetic patterns for edge cases.
fn add_extended_patterns(dataset: &mut TrainingDataset) {
    // More type mismatch variants
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `String`",
            ErrorCategory::TypeMismatch,
            "Function missing return type - add -> String",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `i32`",
            ErrorCategory::TypeMismatch,
            "Function missing return type - add -> i32",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `bool`",
            ErrorCategory::TypeMismatch,
            "Function missing return type - add -> bool",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `Vec<String>`, found `&[String]`",
            ErrorCategory::TypeMismatch,
            "Convert slice to Vec: .to_vec()",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&mut`, found `&`",
            ErrorCategory::TypeMismatch,
            "Need mutable reference: change & to &mut",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected tuple, found struct",
            ErrorCategory::TypeMismatch,
            "Destructuring mismatch - check pattern syntax",
        ),
    ]);

    // Iterator and collection errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0599]: no method named `iter` found for type `HashMap`",
            ErrorCategory::TraitBound,
            "Use .iter() on HashMap or convert to iterator",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `map` found for struct `Vec`",
            ErrorCategory::TraitBound,
            "Call .iter().map() instead of .map() directly on Vec",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `collect` found",
            ErrorCategory::TraitBound,
            "Need iterator - call .iter() first or check type",
        ),
        TrainingSample::with_fix(
            "error[E0277]: `()` is not an iterator",
            ErrorCategory::TraitBound,
            "Expression doesn't return iterator - check for loop source",
        ),
    ]);

    // String/str errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0599]: no method named `push` found for type `&str`",
            ErrorCategory::TraitBound,
            "&str is immutable - use String::from() then .push()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `clear` found for type `&str`",
            ErrorCategory::TraitBound,
            "&str is immutable - convert to String first",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the size for values of type `str` cannot be known",
            ErrorCategory::TraitBound,
            "Use &str or String instead of bare str",
        ),
    ]);

    // Closure and function errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected closure, found fn pointer",
            ErrorCategory::TypeMismatch,
            "Use closure |args| body or Box<dyn Fn>",
        ),
        TrainingSample::with_fix(
            "error[E0277]: expected a `Fn<()>` closure, found",
            ErrorCategory::TraitBound,
            "Closure signature doesn't match - check arguments",
        ),
        TrainingSample::with_fix(
            "error[E0373]: closure may outlive the current function",
            ErrorCategory::BorrowChecker,
            "Add 'move' keyword: move |args| body",
        ),
    ]);

    // Option/Result errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0599]: no method named `unwrap` found for type `()`",
            ErrorCategory::TypeMismatch,
            "Expression doesn't return Option/Result",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `Option<T>`, found `T`",
            ErrorCategory::TypeMismatch,
            "Wrap value in Some(): Some(value)",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `Result<T, E>`, found `T`",
            ErrorCategory::TypeMismatch,
            "Wrap value in Ok(): Ok(value)",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the `?` operator can only be applied to values that implement `Try`",
            ErrorCategory::TraitBound,
            "Expression doesn't return Option/Result - remove ?",
        ),
    ]);

    // Async errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0728]: `await` is only allowed inside `async` functions",
            ErrorCategory::SyntaxError,
            "Mark function as async: async fn name()",
        ),
        TrainingSample::with_fix(
            "error[E0277]: `impl Future` is not a `Future`",
            ErrorCategory::TraitBound,
            "Missing .await on async function call",
        ),
    ]);

    // Derive and attribute errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0277]: `MyStruct` doesn't implement `Debug`",
            ErrorCategory::TraitBound,
            "Add #[derive(Debug)] to struct definition",
        ),
        TrainingSample::with_fix(
            "error[E0277]: `MyStruct` doesn't implement `Clone`",
            ErrorCategory::TraitBound,
            "Add #[derive(Clone)] to struct definition",
        ),
        TrainingSample::with_fix(
            "error[E0277]: `MyStruct` doesn't implement `Default`",
            ErrorCategory::TraitBound,
            "Add #[derive(Default)] or impl Default manually",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `MyStruct: Serialize` is not satisfied",
            ErrorCategory::TraitBound,
            "Add #[derive(Serialize)] and serde dependency",
        ),
    ]);

    // Numeric type errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `usize`, found `i32`",
            ErrorCategory::TypeMismatch,
            "Cast with 'as usize' or use .try_into()",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `f32`, found `f64`",
            ErrorCategory::TypeMismatch,
            "Cast with 'as f32' (may lose precision)",
        ),
        TrainingSample::with_fix(
            "error[E0277]: cannot multiply `f64` by `i32`",
            ErrorCategory::TypeMismatch,
            "Convert operands to same type: value as f64",
        ),
        TrainingSample::with_fix(
            "error[E0277]: cannot add `String` to `&str`",
            ErrorCategory::TypeMismatch,
            "Use format!() or convert both to String",
        ),
    ]);

    // Lifetime errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0106]: missing lifetime specifier",
            ErrorCategory::BorrowChecker,
            "Add lifetime parameter: fn foo<'a>(x: &'a str)",
        ),
        TrainingSample::with_fix(
            "error[E0597]: borrowed value does not live long enough",
            ErrorCategory::BorrowChecker,
            "Extend lifetime or clone the value",
        ),
        TrainingSample::with_fix(
            "error[E0515]: cannot return value referencing local variable",
            ErrorCategory::BorrowChecker,
            "Return owned value instead of reference",
        ),
        TrainingSample::with_fix(
            "error[E0621]: explicit lifetime required",
            ErrorCategory::BorrowChecker,
            "Add lifetime annotation to struct/function",
        ),
    ]);

    // Module and visibility errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0603]: module `internal` is private",
            ErrorCategory::MissingImport,
            "Make module public: pub mod internal",
        ),
        TrainingSample::with_fix(
            "error[E0603]: function `helper` is private",
            ErrorCategory::MissingImport,
            "Make function public: pub fn helper()",
        ),
        TrainingSample::with_fix(
            "error[E0412]: cannot find type `Config` in this scope",
            ErrorCategory::MissingImport,
            "Add use statement: use crate::config::Config",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: could not find `module`",
            ErrorCategory::MissingImport,
            "Add mod declaration or check module path",
        ),
    ]);

    // Pattern matching errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0004]: non-exhaustive patterns: `None` not covered",
            ErrorCategory::SyntaxError,
            "Add missing match arm: None => { ... }",
        ),
        TrainingSample::with_fix(
            "error[E0004]: non-exhaustive patterns: `Err(_)` not covered",
            ErrorCategory::SyntaxError,
            "Add error handling arm or use if let",
        ),
        TrainingSample::with_fix(
            "error[E0005]: refutable pattern in local binding",
            ErrorCategory::SyntaxError,
            "Use if let or match instead of let",
        ),
        TrainingSample::with_fix(
            "error[E0026]: struct does not have a field named `foo`",
            ErrorCategory::TypeMismatch,
            "Check struct definition for correct field names",
        ),
    ]);

    // Macro and const errors
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0658]: `const fn` is not yet stable",
            ErrorCategory::SyntaxError,
            "Add #![feature(const_fn)] or wait for stabilization",
        ),
        TrainingSample::with_fix(
            "error[E0015]: calls in constants are limited",
            ErrorCategory::SyntaxError,
            "Use const fn or compute value at runtime",
        ),
        TrainingSample::with_fix(
            "error: cannot find macro `println` in this scope",
            ErrorCategory::MissingImport,
            "println! is in std prelude - check macro name",
        ),
    ]);
}

/// Get category for a rustc error code.
#[must_use]
pub fn categorize_error(error_code: &str) -> Option<(ErrorCategory, &'static str)> {
    ERROR_CATEGORY_MAP
        .iter()
        .find(|(code, _, _)| *code == error_code)
        .map(|(_, cat, fix)| (*cat, *fix))
}

/// Parse rustc error output and extract error patterns.
#[must_use]
pub fn parse_rustc_errors(output: &str) -> Vec<(String, ErrorCategory, String)> {
    let mut results = Vec::new();

    for line in output.lines() {
        if let Some(start) = line.find("error[E") {
            if let Some(end) = line[start..].find(']') {
                let code = &line[start + 6..start + end];
                if let Some((category, fix_hint)) = categorize_error(code) {
                    results.push((line.to_string(), category, fix_hint.to_string()));
                }
            }
        }
    }

    results
}

/// Statistics about error distribution.
#[must_use]
pub fn verificar_corpus_stats() -> HashMap<ErrorCategory, usize> {
    let corpus = build_verificar_corpus();
    let mut stats = HashMap::new();

    for sample in corpus.samples() {
        *stats.entry(sample.category).or_insert(0) += 1;
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verificar_corpus_size() {
        let corpus = build_verificar_corpus();
        assert!(corpus.len() >= 25, "Should have at least 25 synthetic samples");
    }

    #[test]
    fn test_error_categorization() {
        assert_eq!(
            categorize_error("E0308").map(|(c, _)| c),
            Some(ErrorCategory::TypeMismatch)
        );
        assert_eq!(
            categorize_error("E0432").map(|(c, _)| c),
            Some(ErrorCategory::MissingImport)
        );
        assert_eq!(
            categorize_error("E0596").map(|(c, _)| c),
            Some(ErrorCategory::BorrowChecker)
        );
    }

    #[test]
    fn test_parse_rustc_errors() {
        let output = r#"error[E0308]: mismatched types
   --> src/main.rs:10:5
error[E0432]: unresolved import `serde_json`
   --> src/lib.rs:1:5"#;

        let errors = parse_rustc_errors(output);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].1, ErrorCategory::TypeMismatch);
        assert_eq!(errors[1].1, ErrorCategory::MissingImport);
    }

    #[test]
    fn test_category_distribution() {
        let stats = verificar_corpus_stats();
        let total: usize = stats.values().sum();
        assert!(total >= 25);

        // TypeMismatch should be well-represented
        assert!(stats.get(&ErrorCategory::TypeMismatch).unwrap_or(&0) >= &5);
    }
}
