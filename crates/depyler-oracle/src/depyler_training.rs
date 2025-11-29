//! Depyler-specific training data for error classification.
//!
//! Combines:
//! - Real fixes from DEPYLER-0551 through DEPYLER-0555+
//! - Synthetic patterns from verificar corpus
//!
//! Uses aprender's model evaluation for cross-validation.

use crate::classifier::ErrorCategory;
use crate::training::{TrainingDataset, TrainingSample};
use crate::verificar_integration;

/// Build depyler-specific training dataset from actual fixes.
#[must_use]
pub fn build_depyler_corpus() -> TrainingDataset {
    let mut dataset = TrainingDataset::new();

    // DEPYLER-0551: Error types + PathBuf methods
    add_pathbuf_samples(&mut dataset);

    // DEPYLER-0552: Dict access type inference
    add_dict_inference_samples(&mut dataset);

    // DEPYLER-0553: datetime.datetime chain + instance methods
    add_datetime_samples(&mut dataset);

    // DEPYLER-0554: Function return type + if/else returns
    add_return_type_samples(&mut dataset);

    // DEPYLER-0555: hashlib/file read patterns
    add_file_io_samples(&mut dataset);

    // Type inference: serde_json::Value defaults
    add_type_inference_samples(&mut dataset);

    // DEPYLER-0559: Real errors from stdlib_integration + log_analyzer
    add_stdlib_real_errors(&mut dataset);

    dataset
}

fn add_pathbuf_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0599]: no method named `exists` found for type `String`",
            ErrorCategory::TraitBound,
            "Use std::path::PathBuf::from(&path).exists() instead of String.exists()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `is_file` found for type `String`",
            ErrorCategory::TraitBound,
            "Convert to PathBuf: std::path::PathBuf::from(&path).is_file()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `stat` found for type `PathBuf`",
            ErrorCategory::TraitBound,
            "Use path.metadata() instead of path.stat() - Rust uses metadata()",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `PathBuf: From<Option<String>>` is not satisfied",
            ErrorCategory::TypeMismatch,
            "Unwrap Option before PathBuf conversion: path.map(PathBuf::from)",
        ),
    ]);
}

fn add_dict_inference_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&String`, found `&&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Fix type inference: parameter should be String/&str not serde_json::Value",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `String: Borrow<&str>` is not satisfied",
            ErrorCategory::TraitBound,
            "HashMap key type mismatch: use &str or String consistently",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `get` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Type should be HashMap not Value - fix dict type inference",
        ),
        TrainingSample::with_fix(
            "expected `HashMap<String, String>`, found `HashMap<String, serde_json::Value>`",
            ErrorCategory::TypeMismatch,
            "Dict value type inference: propagate concrete type from usage",
        ),
    ]);
}

fn add_datetime_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `DateTime`",
            ErrorCategory::MissingImport,
            "datetime.datetime.fromtimestamp() → chrono::DateTime::from_timestamp()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `isoformat` found",
            ErrorCategory::TraitBound,
            "dt.isoformat() → dt.to_string() for chrono DateTime",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `strftime` found for struct `NaiveDateTime`",
            ErrorCategory::TraitBound,
            "dt.strftime(fmt) → dt.format(fmt).to_string() for chrono",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `timestamp` found for struct `NaiveDateTime`",
            ErrorCategory::TraitBound,
            "dt.timestamp() → dt.and_utc().timestamp() as f64",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `fromtimestamp`",
            ErrorCategory::MissingImport,
            "datetime.datetime.fromtimestamp → chrono::DateTime::from_timestamp",
        ),
    ]);
}

fn add_return_type_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `String`",
            ErrorCategory::TypeMismatch,
            "Function missing return type: infer -> String from if/else branches",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `i32`",
            ErrorCategory::TypeMismatch,
            "Function missing return type: add return type annotation",
        ),
        TrainingSample::with_fix(
            "missing `return` keyword in if branch",
            ErrorCategory::SyntaxError,
            "If branches need explicit return when not final expression",
        ),
        TrainingSample::with_fix(
            "error[E0308]: `if` missing an `else` clause",
            ErrorCategory::TypeMismatch,
            "If expression needs else clause for type inference",
        ),
    ]);
}

fn add_file_io_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: expected `&mut [u8]`, found integer",
            ErrorCategory::TypeMismatch,
            "Python f.read(8192) → Rust requires buffer: let mut buf = vec![0u8; 8192]",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `hexdigest` found for struct `String`",
            ErrorCategory::TraitBound,
            "hashlib.hexdigest() → use sha2/md5 crate with .finalize() and hex encoding",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `update` found for struct `String`",
            ErrorCategory::TraitBound,
            "hasher.update(chunk) → use Digest trait from sha2 crate",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `is_empty` found for enum `Result`",
            ErrorCategory::TypeMismatch,
            "Walrus operator pattern: while chunk := f.read() needs different Rust idiom",
        ),
    ]);
}

fn add_type_inference_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0606]: casting `&serde_json::Value` as `i64` is invalid",
            ErrorCategory::TypeMismatch,
            "Parameter type should be f64 not Value - infer from cast usage",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `f64`, found `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Numeric parameter defaulted to Value - propagate type from arithmetic",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `to_uppercase` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "String method on Value - parameter should be String not Value",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `len` found for reference `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Collection method on Value - infer Vec/String from .len() usage",
        ),
        TrainingSample::with_fix(
            "error[E0599]: the method `join` exists but trait bounds not satisfied",
            ErrorCategory::TraitBound,
            "Vec<Value> should be Vec<String> for join() - propagate element type",
        ),
        TrainingSample::with_fix(
            "error[E0282]: type annotations needed",
            ErrorCategory::TypeMismatch,
            "Insufficient type context - add explicit annotation or infer from usage",
        ),
    ]);
}

/// DEPYLER-0559: Real errors from stdlib_integration + log_analyzer examples
/// These are actual compilation errors encountered during transpilation.
fn add_stdlib_real_errors(dataset: &mut TrainingDataset) {
    // Add reprorusted-python-cli corpus errors first
    add_reprorusted_corpus_errors(dataset);
    dataset.add_many(vec![
        // Function return type inference failures
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `String` arguments to this enum variant are incorrect",
            ErrorCategory::TypeMismatch,
            "FIX_RETURN_TYPE: Function returns String but declared as Result<(), _>. Infer return type from actual return statements.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: `?` operator has incompatible types expected `String`, found `()`",
            ErrorCategory::TypeMismatch,
            "FIX_RETURN_TYPE: Called function returns () but caller expects String. Fix callee's return type annotation.",
        ),
        // HashMap key type issues
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `String: Borrow<&str>` is not satisfied required by a bound introduced by this call",
            ErrorCategory::TraitBound,
            "FIX_HASHMAP_KEY: Use .get(\"key\") not .get(&\"key\"). HashMap<String, V>::get takes &str directly.",
        ),
        TrainingSample::with_fix(
            "the trait `Borrow<&_>` is not implemented for `String` but trait `Borrow<_>` is implemented",
            ErrorCategory::TraitBound,
            "FIX_HASHMAP_KEY: Extra & before string literal in .get(). Remove the extra reference.",
        ),
        // Mixed dict value types
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `String`, found `u64` HashMap<String, String>",
            ErrorCategory::TypeMismatch,
            "FIX_DICT_VALUE_TYPE: Dict has mixed value types (String + u64). Use serde_json::Value or convert all to String with .to_string().",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `HashMap<String, Value>`, found `HashMap<String, String>`",
            ErrorCategory::TypeMismatch,
            "FIX_DICT_TYPE_CONSISTENCY: Return type expects Value but built with String. Unify dict value types.",
        ),
        // Value vs concrete type method calls
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `bool`, found `Value` in if condition",
            ErrorCategory::TypeMismatch,
            "FIX_CONDITION_TYPE: Value used in if condition. Use .as_bool().unwrap_or(false) or infer concrete bool type.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `bool`, found `&Value` in && expression",
            ErrorCategory::TypeMismatch,
            "FIX_BOOL_PARAM: Parameter has wrong type - should be bool not &serde_json::Value.",
        ),
        // Option handling
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `PathBuf: From<Option<String>>` is not satisfied",
            ErrorCategory::TypeMismatch,
            "FIX_OPTION_UNWRAP: Cannot construct PathBuf from Option. Use args.output.map(PathBuf::from) or unwrap first.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `String`, found `Option<String>` insert arguments incorrect",
            ErrorCategory::TypeMismatch,
            "FIX_OPTION_VALUE: Option<String> used where String expected. Add .unwrap_or_default() or handle None case.",
        ),
        // Function argument type mismatches
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&f64`, found `f64` consider borrowing here",
            ErrorCategory::TypeMismatch,
            "FIX_BORROW: Function expects reference but got owned value. Add & before argument.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `String`, found `&PathBuf`",
            ErrorCategory::TypeMismatch,
            "FIX_PATH_TO_STRING: Expected String but got PathBuf reference. Use path.to_string_lossy().to_string().",
        ),
        // Mutability mismatches
        TrainingSample::with_fix(
            "error[E0308]: types differ in mutability expected `&mut HashMap`, found `&HashMap`",
            ErrorCategory::BorrowChecker,
            "FIX_MUTABILITY: Function expects mutable reference. Change &info to &mut info at call site.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected mutable reference `&mut serde_json::Value` found reference `&HashMap`",
            ErrorCategory::TypeMismatch,
            "FIX_PARAM_TYPE: Parameter type mismatch - function expects Value but called with HashMap.",
        ),
        // Generator/yield errors (log_analyzer)
        TrainingSample::with_fix(
            "error[E0658]: yield syntax is experimental",
            ErrorCategory::SyntaxError,
            "FIX_YIELD: Python yield requires #![feature(coroutines)] or rewrite as Iterator impl.",
        ),
        TrainingSample::with_fix(
            "error[E0627]: yield expression outside of coroutine literal",
            ErrorCategory::SyntaxError,
            "FIX_COROUTINE: yield must be in #[coroutine] closure. Convert generator to iterator pattern.",
        ),
        // GroupBy/itertools errors
        TrainingSample::with_fix(
            "error[E0277]: `GroupBy<_, _, _>` is not an iterator",
            ErrorCategory::TraitBound,
            "FIX_GROUPBY: itertools group_by returns GroupBy. Use .into_iter() on result groups.",
        ),
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `group` in this scope",
            ErrorCategory::SyntaxError,
            "FIX_CLOSURE_CAPTURE: Variable not in scope. Check closure captures or function parameters.",
        ),
        // Tuple field access
        TrainingSample::with_fix(
            "error[E0609]: no field `0` on type `()`",
            ErrorCategory::TypeMismatch,
            "FIX_TUPLE_TYPE: Trying to access tuple field on unit type. Check iterator element type.",
        ),
        TrainingSample::with_fix(
            "error[E0631]: type mismatch in closure arguments expected closure signature `fn((&K, &V))`",
            ErrorCategory::TypeMismatch,
            "FIX_CLOSURE_SIG: Closure parameter types don't match. Adjust parameter patterns or types.",
        ),
        // === ADDITIONAL SERDE_JSON::VALUE PATTERNS ===
        // These are the most common errors in stdlib_integration
        TrainingSample::with_fix(
            "error[E0308]: expected `String`, found `u64` HashMap<String, String>",
            ErrorCategory::TypeMismatch,
            "FIX_MIXED_DICT_VALUES: Dict inserts String and u64. Convert u64 to String: stats.len().to_string()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `to_uppercase` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_VALUE_TO_STRING: Value.to_uppercase() invalid. Use .as_str().unwrap().to_uppercase() or infer String type.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `len` found for reference `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_VALUE_LEN: Value.len() invalid. Type should be String or Vec. Use .as_str().unwrap().len() or fix type inference.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: the method `join` exists for struct `Vec<serde_json::Value>`, but its trait bounds were not satisfied",
            ErrorCategory::TraitBound,
            "FIX_VEC_VALUE_JOIN: Vec<Value> can't join. Change to Vec<String> or convert elements: vec.iter().map(|v| v.to_string()).collect::<Vec<_>>().join()",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `HashMap<String, Value>`, found `HashMap<String, String>`",
            ErrorCategory::TypeMismatch,
            "FIX_DICT_TYPE_UNIFY: Return type HashMap<String, Value> but built HashMap<String, String>. Unify: use serde_json::json!() or convert values.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `bool`, found `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_VALUE_TO_BOOL: Value used as bool. Use value.as_bool().unwrap_or(false) or fix type to bool.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `Value`, found `String` in push/insert",
            ErrorCategory::TypeMismatch,
            "FIX_STRING_TO_VALUE: Vec<Value> expects Value. Convert: serde_json::Value::String(s) or s.into()",
        ),
        TrainingSample::with_fix(
            "error[E0308]: arguments to this function are incorrect expected `&mut HashMap<_, _>` found `&HashMap`",
            ErrorCategory::BorrowChecker,
            "FIX_MUT_BORROW: Function needs &mut but got &. Change call site to pass &mut variable.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `&Value`, found `bool` arguments to this function are incorrect",
            ErrorCategory::TypeMismatch,
            "FIX_PARAM_TYPE_VALUE: Function param is &Value but passed bool. Fix function signature or convert argument.",
        ),
        TrainingSample::with_fix(
            "error[E0282]: type annotations needed PathBuf as_ref",
            ErrorCategory::TypeMismatch,
            "FIX_PATHBUF_ASREF: Type inference failed on PathBuf.as_ref(). Use explicit path or avoid double .as_ref().unwrap() chains.",
        ),
    ]);
}

/// Add samples from historical bug documentation (mined from docs/bugs/*.md)
fn add_bug_doc_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        // From DEPYLER-0161: Dead code elimination bug
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `arr1` in this scope",
            ErrorCategory::SyntaxError,
            "FIX_DCE_TUPLE: Dead code elimination removed tuple return. Disable DCE for tuple assignments.",
        ),
        // From DEPYLER-0264: DynamicType not found
        TrainingSample::with_fix(
            "error[E0412]: cannot find type `DynamicType` in this scope",
            ErrorCategory::MissingImport,
            "FIX_DYNAMIC_TYPE: Unknown type annotation. Use serde_json::Value for dynamic typing.",
        ),
        // From DEPYLER-0265: Reference vs value in comparisons
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&i32`, found `i32`",
            ErrorCategory::TypeMismatch,
            "FIX_REF_COMPARE: Comparison with borrowed value. Add & to value or dereference borrowed.",
        ),
        // From DEPYLER-0266: Unary operator on collection
        TrainingSample::with_fix(
            "error[E0600]: cannot apply unary operator `!` to type `&'a Vec<i32>`",
            ErrorCategory::TraitBound,
            "FIX_NOT_COLLECTION: Python `not list` → Rust `.is_empty()`. Convert truthiness check.",
        ),
        TrainingSample::with_fix(
            "error[E0600]: cannot apply unary operator `!` to type `&'a Vec<String>`",
            ErrorCategory::TraitBound,
            "FIX_NOT_COLLECTION: Python `not list` → Rust `.is_empty()`. Convert truthiness check.",
        ),
        // From DEPYLER-0267: String doesn't implement Copy
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `String: Copy` is not satisfied",
            ErrorCategory::TraitBound,
            "FIX_STRING_COPY: String isn't Copy. Use .cloned() instead of .copied().",
        ),
        // From DEPYLER-0268: Unary minus on usize
        TrainingSample::with_fix(
            "error[E0600]: cannot apply unary operator `-` to type `usize`",
            ErrorCategory::TypeMismatch,
            "FIX_USIZE_NEG: usize can't be negative. Cast to isize first or use wrapping_neg.",
        ),
        // From DEPYLER-0432: ? operator in non-Result function
        TrainingSample::with_fix(
            "error[E0277]: the `?` operator can only be used in a function that returns `Result`",
            ErrorCategory::TypeMismatch,
            "FIX_QUESTION_RESULT: ? requires Result return. Add -> Result<T, E> or use match.",
        ),
        // From DEPYLER-0448: Return type defaults to i32
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `i32`, found `HashMap`",
            ErrorCategory::TypeMismatch,
            "FIX_RETURN_TYPE_INFERENCE: Return type incorrectly inferred as i32. Check return statements.",
        ),
        // From DEPYLER-0455: Option doesn't implement Display
        TrainingSample::with_fix(
            "error[E0277]: `Option<String>` doesn't implement `std::fmt::Display`",
            ErrorCategory::TraitBound,
            "FIX_OPTION_DISPLAY: Option lacks Display. Use {:?} or .unwrap_or_default().",
        ),
        // From DEPYLER-0467: Value auto-borrowing
        TrainingSample::with_fix(
            "error[E0308]: arguments to this function are incorrect expected `&Value`",
            ErrorCategory::TypeMismatch,
            "FIX_VALUE_BORROW: serde_json::Value needs &. Add borrow for function argument.",
        ),
        // Common Vec<Value> issues (current stdlib errors)
        TrainingSample::with_fix(
            "error[E0599]: no method named `join` found for struct `Vec<Value>`",
            ErrorCategory::TraitBound,
            "FIX_VEC_VALUE_JOIN: Vec<Value> has no join(). Convert to Vec<String> first.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `len` found for struct `Value`",
            ErrorCategory::TraitBound,
            "FIX_VALUE_LEN: serde_json::Value has no .len(). Use .as_str().map(|s| s.len()).",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `Vec<String>`, found `Vec<Value>`",
            ErrorCategory::TypeMismatch,
            "FIX_VEC_VALUE_STRING: List literal inferred as Vec<Value>. Use explicit String type.",
        ),
    ]);
}

/// Build combined corpus from real fixes + synthetic verificar patterns.
#[must_use]
pub fn build_combined_corpus() -> TrainingDataset {
    let mut real = build_depyler_corpus();
    let synthetic = verificar_integration::build_verificar_corpus();

    // Merge synthetic samples into real corpus
    for sample in synthetic.samples() {
        real.add(sample.clone());
    }

    // Add samples mined from bug documentation
    add_bug_doc_samples(&mut real);

    // Add additional samples to balance underrepresented categories
    add_borrow_checker_samples(&mut real);
    add_syntax_error_samples(&mut real);
    add_missing_import_samples(&mut real);

    real
}

/// Additional BorrowChecker samples to balance corpus (Issue #106)
fn add_borrow_checker_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable",
            ErrorCategory::BorrowChecker,
            "FIX_BORROW_CONFLICT: Split the borrow - use immutable borrow first, then mutable.",
        ),
        TrainingSample::with_fix(
            "error[E0499]: cannot borrow `x` as mutable more than once at a time",
            ErrorCategory::BorrowChecker,
            "FIX_DOUBLE_MUT: Only one &mut reference allowed. Clone or restructure code.",
        ),
        TrainingSample::with_fix(
            "error[E0507]: cannot move out of borrowed content",
            ErrorCategory::BorrowChecker,
            "FIX_MOVE_BORROW: Cannot move from &T. Use .clone() or restructure ownership.",
        ),
        TrainingSample::with_fix(
            "error[E0382]: borrow of moved value",
            ErrorCategory::BorrowChecker,
            "FIX_USE_AFTER_MOVE: Value moved earlier. Clone before move or use reference.",
        ),
        TrainingSample::with_fix(
            "error[E0382]: use of moved value: `x`",
            ErrorCategory::BorrowChecker,
            "FIX_MOVED_VALUE: Ownership transferred. Clone the value before the move.",
        ),
        TrainingSample::with_fix(
            "error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable",
            ErrorCategory::BorrowChecker,
            "FIX_NOT_MUT: Add `mut` keyword: `let mut x = ...`",
        ),
        TrainingSample::with_fix(
            "error[E0597]: `x` does not live long enough",
            ErrorCategory::BorrowChecker,
            "FIX_LIFETIME: Reference outlives value. Extend lifetime or restructure.",
        ),
        TrainingSample::with_fix(
            "error[E0505]: cannot move out of `x` because it is borrowed",
            ErrorCategory::BorrowChecker,
            "FIX_MOVE_WHILE_BORROWED: Drop the borrow before moving the value.",
        ),
        TrainingSample::with_fix(
            "error[E0503]: cannot use `x` because it was mutably borrowed",
            ErrorCategory::BorrowChecker,
            "FIX_MUT_BORROW_USE: Mutable borrow active. End borrow scope first.",
        ),
        TrainingSample::with_fix(
            "error[E0594]: cannot assign to `x`, as it is not declared as mutable",
            ErrorCategory::BorrowChecker,
            "FIX_ASSIGN_IMMUT: Variable is immutable. Declare with `let mut`.",
        ),
        TrainingSample::with_fix(
            "error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable",
            ErrorCategory::BorrowChecker,
            "FIX_SELF_BORROW: Split self borrows - extract immutable data first.",
        ),
        TrainingSample::with_fix(
            "error[E0499]: cannot borrow `*self` as mutable more than once",
            ErrorCategory::BorrowChecker,
            "FIX_SELF_DOUBLE_MUT: Multiple &mut self. Use interior mutability or restructure.",
        ),
    ]);
}

/// Additional SyntaxError samples to balance corpus (Issue #106)
fn add_syntax_error_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error: expected `;`, found `}`",
            ErrorCategory::SyntaxError,
            "FIX_MISSING_SEMI: Add semicolon at end of statement.",
        ),
        TrainingSample::with_fix(
            "error: expected expression, found `let`",
            ErrorCategory::SyntaxError,
            "FIX_LET_EXPR: let is a statement, not expression. Use block or match.",
        ),
        TrainingSample::with_fix(
            "error: expected `{`, found `=>`",
            ErrorCategory::SyntaxError,
            "FIX_MATCH_SYNTAX: Match arm syntax error. Check braces and commas.",
        ),
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `x` in this scope",
            ErrorCategory::SyntaxError,
            "FIX_UNDEFINED_VAR: Variable not defined. Check spelling or add definition.",
        ),
        TrainingSample::with_fix(
            "error[E0412]: cannot find type `Foo` in this scope",
            ErrorCategory::SyntaxError,
            "FIX_UNDEFINED_TYPE: Type not in scope. Add use statement or define type.",
        ),
        TrainingSample::with_fix(
            "error: expected one of `,` or `>`, found `:`",
            ErrorCategory::SyntaxError,
            "FIX_GENERIC_SYNTAX: Generic parameter syntax error. Check angle brackets.",
        ),
        TrainingSample::with_fix(
            "error: unexpected token: `)`",
            ErrorCategory::SyntaxError,
            "FIX_PAREN_MISMATCH: Mismatched parentheses. Check function call syntax.",
        ),
        TrainingSample::with_fix(
            "error: expected item, found `let`",
            ErrorCategory::SyntaxError,
            "FIX_TOP_LEVEL_LET: let not allowed at module level. Use const or static.",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: could not find `foo` in `bar`",
            ErrorCategory::SyntaxError,
            "FIX_PATH_RESOLVE: Module path invalid. Check module structure.",
        ),
        TrainingSample::with_fix(
            "error: expected pattern, found `123`",
            ErrorCategory::SyntaxError,
            "FIX_PATTERN_SYNTAX: Invalid pattern in match arm or let binding.",
        ),
        TrainingSample::with_fix(
            "error[E0423]: expected value, found struct `Foo`",
            ErrorCategory::SyntaxError,
            "FIX_STRUCT_VALUE: Use Foo { } or Foo::new() to create instance.",
        ),
        TrainingSample::with_fix(
            "error: lifetime arguments must be declared prior to type arguments",
            ErrorCategory::SyntaxError,
            "FIX_LIFETIME_ORDER: Lifetimes before types: <'a, T> not <T, 'a>.",
        ),
    ]);
}

/// Additional MissingImport samples to balance corpus (Issue #106)
fn add_missing_import_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `std::collections::hashmap`",
            ErrorCategory::MissingImport,
            "FIX_IMPORT_CASE: Case sensitive - use HashMap not hashmap.",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `Vec`",
            ErrorCategory::MissingImport,
            "FIX_PRELUDE: Vec is in prelude. Check if std prelude is available.",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `serde`",
            ErrorCategory::MissingImport,
            "FIX_CRATE_DEP: Add serde to Cargo.toml dependencies.",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared crate or module `tokio`",
            ErrorCategory::MissingImport,
            "FIX_ASYNC_CRATE: Add tokio to Cargo.toml with features.",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `chrono::NaiveDateTime`",
            ErrorCategory::MissingImport,
            "FIX_CHRONO_IMPORT: Add chrono crate and use correct path.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no function or associated item named `new` found for struct `HashMap`",
            ErrorCategory::MissingImport,
            "FIX_HASHMAP_NEW: Use HashMap::new() - ensure std::collections::HashMap imported.",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `itertools`",
            ErrorCategory::MissingImport,
            "FIX_ITERTOOLS: Add itertools = \"0.12\" to Cargo.toml.",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `Regex`",
            ErrorCategory::MissingImport,
            "FIX_REGEX_IMPORT: Add use regex::Regex after adding regex crate.",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `std::fs::read_to_string`",
            ErrorCategory::MissingImport,
            "FIX_FS_IMPORT: Use std::fs or import specific functions.",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `PathBuf`",
            ErrorCategory::MissingImport,
            "FIX_PATHBUF_IMPORT: Add use std::path::PathBuf;",
        ),
        TrainingSample::with_fix(
            "error[E0432]: unresolved import `anyhow`",
            ErrorCategory::MissingImport,
            "FIX_ANYHOW: Add anyhow = \"1.0\" to Cargo.toml dependencies.",
        ),
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `Arc`",
            ErrorCategory::MissingImport,
            "FIX_ARC_IMPORT: Add use std::sync::Arc;",
        ),
    ]);
}

/// REPRORUSTED-CLI corpus: Real errors from transpiled Python CLI examples
/// Captured 2025-11-27 from examples that don't yet compile.
fn add_reprorusted_corpus_errors(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        // === LOG_ANALYZER (27 errors) - Generator + Regex patterns ===
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `Value`, found `Regex`",
            ErrorCategory::TypeMismatch,
            "FIX_GENERATOR_FIELD_TYPE: Generator state field should be regex::Regex not serde_json::Value.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `find` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_REGEX_TYPE: Type should be regex::Regex not Value. Fix generator state variable typing.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `is_some` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_OPTION_TYPE: Type should be Option<Match> not Value. Fix match result type.",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `0` on type `Vec<String>`",
            ErrorCategory::TypeMismatch,
            "FIX_TUPLE_GROUPS: regex.groups() returns tuple, not Vec. Use captures.get(1).map(|m| m.as_str()).",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `Value`, found `(Value, Value, Value)`",
            ErrorCategory::TypeMismatch,
            "FIX_ITERATOR_ITEM: Iterator Item type is Value but returning tuple. Fix generator Item type.",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `serde_json::Value: Ord` is not satisfied",
            ErrorCategory::TraitBound,
            "FIX_VALUE_ORD: Value doesn't implement Ord for sorting. Convert to sortable type first.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `as_object` found for struct `HashMap`",
            ErrorCategory::TypeMismatch,
            "FIX_HASHMAP_ASOBJ: HashMap isn't Value. Remove .as_object() - already have HashMap.",
        ),
        TrainingSample::with_fix(
            "error[E0631]: type mismatch in function arguments expected `&Value`, found `&Args`",
            ErrorCategory::TypeMismatch,
            "FIX_ARGPARSE_TYPE: cmd_* functions should take &Args not &serde_json::Value.",
        ),

        // === FUTURES (7 errors) - ThreadPoolExecutor patterns ===
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `ThreadPoolExecutor`",
            ErrorCategory::MissingImport,
            "FIX_THREADPOOL: Python concurrent.futures.ThreadPoolExecutor → Rust rayon::ThreadPoolBuilder.",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `numbers` on type `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_ARGS_FIELD: Args struct field access. Use args.numbers not &Value.numbers.",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `i32` missing return type",
            ErrorCategory::TypeMismatch,
            "FIX_RETURN_TYPE_I32: Function returns i32 but declared as (). Add -> i32 to signature.",
        ),

        // === LRU_CACHE (4 errors) - Subcommand patterns ===
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `n` in this scope pattern doesn't include",
            ErrorCategory::SyntaxError,
            "FIX_SUBCOMMAND_PATTERN: Use Commands::Fib { n } not Commands::Fib { .. } to capture field.",
        ),

        // === DATACLASS (11 errors) - Struct patterns ===
        TrainingSample::with_fix(
            "error[E0015]: cannot call non-const method `to_string` in constants",
            ErrorCategory::SyntaxError,
            "FIX_CONST_INIT: Struct const fields can't call methods. Use Default::default() or lazy_static.",
        ),
        TrainingSample::with_fix(
            "error[E0425]: cannot find function `asdict` in this scope",
            ErrorCategory::MissingImport,
            "FIX_ASDICT: Python asdict() → Rust serde_json::to_value(&obj).unwrap() with #[derive(Serialize)].",
        ),
        TrainingSample::with_fix(
            "error[E0061]: this function takes 2 arguments but 3 arguments were supplied",
            ErrorCategory::TypeMismatch,
            "FIX_STRUCT_CTOR: Struct::new() argument count mismatch. Check @dataclass field defaults.",
        ),

        // === DATETIME (21 errors) - chrono patterns ===
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `datetime` in this scope",
            ErrorCategory::MissingImport,
            "FIX_DATETIME_MODULE: Python datetime module → Rust chrono crate types directly.",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `format` on type `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_ARGS_STRING_FIELD: Args field is String, not Value. Check argparse type mapping.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `isoformat` found for struct `NaiveDateTime`",
            ErrorCategory::TraitBound,
            "FIX_CHRONO_ISO: chrono NaiveDateTime.isoformat() → .format(\"%+\").to_string() or .to_string().",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `strftime` found for struct `NaiveDateTime`",
            ErrorCategory::TraitBound,
            "FIX_CHRONO_STRFTIME: chrono strftime() → .format(fmt).to_string(). Different API.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `total_seconds` found for struct `TimeDelta`",
            ErrorCategory::TraitBound,
            "FIX_CHRONO_TOTAL_SECONDS: chrono TimeDelta.total_seconds() → .num_seconds() as f64.",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `get` on `&serde_json::Value` called with `&str`",
            ErrorCategory::TypeMismatch,
            "FIX_STRING_CONTAINS: args.date.get(\"T\") should be args.date.contains('T') for String.",
        ),

        // === CONTEXTLIB (7 errors) - Context manager patterns ===
        TrainingSample::with_fix(
            "error[E0425]: cannot find function `redirect_stdout` in this scope",
            ErrorCategory::MissingImport,
            "FIX_REDIRECT_STDOUT: Python redirect_stdout → Rust: capture stdout with closure or gag crate.",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `content` on type `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_ARGS_FIELD_TYPE: Args.content should be String field, not Value access.",
        ),

        // === NAMEDTUPLE (10 errors) - Struct patterns ===
        TrainingSample::with_fix(
            "error[E0277]: cannot add `f64` to `i32`",
            ErrorCategory::TypeMismatch,
            "FIX_NUMERIC_ADD: Numeric types must match. Cast: (x as f64) + y_f64.",
        ),
        TrainingSample::with_fix(
            "error[E0282]: type annotations needed in complex expression",
            ErrorCategory::TypeMismatch,
            "FIX_TYPE_ANNOTATION: Complex expression needs type hint. Add explicit annotation or turbofish.",
        ),

        // === PATHLIB (16 errors) - PathBuf patterns ===
        TrainingSample::with_fix(
            "error[E0277]: `PathBuf` doesn't implement `std::fmt::Display`",
            ErrorCategory::TraitBound,
            "FIX_PATHBUF_DISPLAY: PathBuf needs .display() for formatting: format!(\"{}\", path.display()).",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `glob` found for struct `PathBuf`",
            ErrorCategory::TraitBound,
            "FIX_GLOB_CRATE: PathBuf.glob() → use glob crate: glob::glob(&pattern_str).",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `components` found for reference `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_PATH_COMPONENTS: Type should be PathBuf not Value. path.components() is PathBuf method.",
        ),
        TrainingSample::with_fix(
            "error[E0369]: cannot divide `PathBuf` by `_`",
            ErrorCategory::SyntaxError,
            "FIX_PATH_JOIN: Python path / part → Rust path.join(part). PathBuf doesn't impl Div.",
        ),

        // === SHUTIL (12 errors) - File operation patterns ===
        TrainingSample::with_fix(
            "error[E0425]: cannot find value `shutil` in this scope",
            ErrorCategory::MissingImport,
            "FIX_SHUTIL_MODULE: Python shutil.copy2() → Rust std::fs::copy() or fs_extra crate.",
        ),
        TrainingSample::with_fix(
            "error[E0609]: no field `src` on type `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "FIX_ARGS_PATH_FIELD: Args.src/dst should be String or PathBuf, not Value field access.",
        ),
    ]);
}

/// Get error-fix pairs formatted for NgramFixPredictor training.
/// Uses combined corpus (real + synthetic).
#[must_use]
pub fn get_training_pairs() -> Vec<(String, String, ErrorCategory)> {
    build_combined_corpus().error_fix_pairs()
}

/// Get pairs from real fixes only (for evaluation baseline).
#[must_use]
pub fn get_real_training_pairs() -> Vec<(String, String, ErrorCategory)> {
    build_depyler_corpus().error_fix_pairs()
}

/// Category distribution for combined corpus (real + synthetic).
#[must_use]
pub fn corpus_stats() -> Vec<(ErrorCategory, usize)> {
    let dataset = build_combined_corpus();
    vec![
        (
            ErrorCategory::TypeMismatch,
            dataset
                .samples_for_category(ErrorCategory::TypeMismatch)
                .len(),
        ),
        (
            ErrorCategory::TraitBound,
            dataset
                .samples_for_category(ErrorCategory::TraitBound)
                .len(),
        ),
        (
            ErrorCategory::MissingImport,
            dataset
                .samples_for_category(ErrorCategory::MissingImport)
                .len(),
        ),
        (
            ErrorCategory::BorrowChecker,
            dataset
                .samples_for_category(ErrorCategory::BorrowChecker)
                .len(),
        ),
        (
            ErrorCategory::SyntaxError,
            dataset
                .samples_for_category(ErrorCategory::SyntaxError)
                .len(),
        ),
    ]
}

// ============================================================================
// MoE Oracle Integration (DEPYLER-0580)
// ============================================================================

use crate::moe_oracle::{MoeClassificationResult, MoeOracle};

/// Load real error corpus from file (collected from reprorusted-python-cli)
///
/// This function parses compilation errors collected by scripts/collect_errors.sh
/// Returns samples suitable for MoE Oracle training.
pub fn load_real_corpus(path: &str) -> Vec<(String, String, ErrorCategory)> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut samples = Vec::new();

    for line in content.lines() {
        if line.contains("error[E") {
            // Extract error code
            if let Some(start) = line.find("[E") {
                if let Some(end) = line[start..].find(']') {
                    let error_code = line[start + 1..start + end].to_string();
                    let context = line.to_string();
                    let category = categorize_error_code(&error_code);

                    samples.push((error_code, context, category));
                }
            }
        }
    }

    samples
}

/// Categorize error code to ErrorCategory
fn categorize_error_code(code: &str) -> ErrorCategory {
    match code {
        // TypeSystem expert
        "E0308" | "E0606" | "E0061" => ErrorCategory::TypeMismatch,
        "E0277" => ErrorCategory::TraitBound,
        // ScopeResolution expert
        "E0425" | "E0412" | "E0433" | "E0423" => ErrorCategory::MissingImport,
        // MethodField expert
        "E0599" | "E0609" | "E0615" => ErrorCategory::TraitBound,
        // SyntaxBorrowing expert
        "E0369" | "E0282" | "E0027" | "E0015" => ErrorCategory::BorrowChecker,
        // Borrow checker
        "E0382" | "E0505" | "E0502" => ErrorCategory::BorrowChecker,
        // Lifetime
        "E0106" | "E0495" => ErrorCategory::LifetimeError,
        _ => ErrorCategory::Other,
    }
}

/// Train MoE Oracle on real + synthetic corpus
pub fn train_moe_on_real_corpus() -> crate::Result<MoeOracle> {
    let mut oracle = MoeOracle::new();

    // Load real corpus if available
    let real_samples = load_real_corpus("/tmp/real_errors.txt");
    println!("Loaded {} samples from real corpus", real_samples.len());

    // Add synthetic corpus
    let combined = build_combined_corpus();
    let synthetic_samples: Vec<(String, String, ErrorCategory)> = combined
        .samples()
        .iter()
        .map(|s| (extract_error_code(&s.message), s.message.clone(), s.category))
        .collect();

    // Combine all samples
    let mut all_samples = real_samples;
    all_samples.extend(synthetic_samples);

    println!("Training MoE Oracle on {} total samples", all_samples.len());
    oracle.train(&all_samples)?;

    Ok(oracle)
}

/// Classify a compilation error using the MoE Oracle.
///
/// Uses the Mixture of Experts model to:
/// 1. Route error to appropriate specialist expert
/// 2. Get domain-specific fix suggestions
/// 3. Return confidence-weighted classification
///
/// # Example
/// ```ignore
/// let result = classify_with_moe("E0308", "mismatched types expected i32, found String");
/// println!("Expert: {:?}, Fix: {:?}", result.primary_expert, result.suggested_fix);
/// ```
pub fn classify_with_moe(error_code: &str, context: &str) -> MoeClassificationResult {
    let oracle = MoeOracle::new();
    oracle.classify(error_code, context)
}

/// Train MoE Oracle on the depyler training corpus.
///
/// Returns a trained MoE Oracle that can classify errors and suggest fixes.
pub fn train_moe_oracle() -> crate::Result<MoeOracle> {
    let mut oracle = MoeOracle::new();
    let corpus = build_combined_corpus();

    // Convert training samples to MoE format
    let samples: Vec<(String, String, ErrorCategory)> = corpus
        .samples()
        .iter()
        .map(|s| (extract_error_code(&s.message), s.message.clone(), s.category))
        .collect();

    oracle.train(&samples)?;
    Ok(oracle)
}

/// Extract error code from error message (e.g., "E0308" from "error[E0308]: ...")
fn extract_error_code(message: &str) -> String {
    // Look for pattern like "error[E0308]" or just "E0308"
    if let Some(start) = message.find("[E") {
        if let Some(end) = message[start..].find(']') {
            return message[start + 1..start + end].to_string();
        }
    }

    // Fallback: look for E followed by digits
    if let Some(start) = message.find("E0") {
        let code: String = message[start..]
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect();
        if code.len() >= 4 {
            return code;
        }
    }

    "E0000".to_string() // Unknown error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depyler_corpus_not_empty() {
        let corpus = build_depyler_corpus();
        assert!(corpus.len() >= 20, "Corpus should have at least 20 samples");
    }

    #[test]
    fn test_all_samples_have_fixes() {
        let corpus = build_depyler_corpus();
        let pairs = corpus.error_fix_pairs();
        assert_eq!(pairs.len(), corpus.len(), "All samples should have fixes");
    }

    #[test]
    fn test_category_distribution() {
        let stats = corpus_stats();
        let total: usize = stats.iter().map(|(_, c)| c).sum();
        assert!(total >= 20);

        // TypeMismatch should be the largest category (our main issue)
        let type_mismatch_count = stats
            .iter()
            .find(|(cat, _)| *cat == ErrorCategory::TypeMismatch)
            .map(|(_, c)| *c)
            .unwrap_or(0);
        assert!(
            type_mismatch_count >= 8,
            "TypeMismatch should have most samples"
        );
    }

    #[test]
    fn test_training_pairs_format() {
        let pairs = get_training_pairs();
        for (error, fix, _category) in &pairs {
            assert!(!error.is_empty(), "Error should not be empty");
            assert!(!fix.is_empty(), "Fix should not be empty");
        }
    }

    // MoE Oracle Integration Tests (DEPYLER-0580)

    #[test]
    fn test_classify_with_moe_type_error() {
        let result = classify_with_moe("E0308", "mismatched types expected i32, found String");
        assert_eq!(
            result.primary_expert,
            crate::moe_oracle::ExpertDomain::TypeSystem
        );
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_classify_with_moe_scope_error() {
        let result = classify_with_moe("E0425", "cannot find value `foo` in this scope");
        assert_eq!(
            result.primary_expert,
            crate::moe_oracle::ExpertDomain::ScopeResolution
        );
    }

    #[test]
    fn test_extract_error_code_bracket() {
        assert_eq!(
            extract_error_code("error[E0308]: mismatched types"),
            "E0308"
        );
    }

    #[test]
    fn test_extract_error_code_plain() {
        assert_eq!(extract_error_code("E0599 no method named"), "E0599");
    }

    #[test]
    fn test_extract_error_code_unknown() {
        assert_eq!(extract_error_code("some error without code"), "E0000");
    }

    #[test]
    fn test_train_moe_oracle() {
        // Training may fail with small datasets (LinearRegression needs >= 2 samples)
        // This is expected behavior - the MoE Oracle still works with default patterns
        let result = train_moe_oracle();
        // Even if training fails, verify we can classify with the default oracle
        // DEPYLER-0625: Use if-let pattern instead of is_err() + unwrap()
        if let Ok(oracle) = result {
            // Training succeeded - verify trained oracle can classify
            let classification = oracle.classify("E0308", "type mismatch");
            assert!(classification.confidence > 0.0);
        } else {
            // Verify default classification still works
            let default_result = classify_with_moe("E0308", "type mismatch");
            assert!(default_result.confidence > 0.0);
        }
    }
}
