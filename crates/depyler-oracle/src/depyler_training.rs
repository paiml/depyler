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
        (ErrorCategory::TypeMismatch, dataset.samples_for_category(ErrorCategory::TypeMismatch).len()),
        (ErrorCategory::TraitBound, dataset.samples_for_category(ErrorCategory::TraitBound).len()),
        (ErrorCategory::MissingImport, dataset.samples_for_category(ErrorCategory::MissingImport).len()),
        (ErrorCategory::BorrowChecker, dataset.samples_for_category(ErrorCategory::BorrowChecker).len()),
        (ErrorCategory::SyntaxError, dataset.samples_for_category(ErrorCategory::SyntaxError).len()),
    ]
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
        let type_mismatch_count = stats.iter()
            .find(|(cat, _)| *cat == ErrorCategory::TypeMismatch)
            .map(|(_, c)| *c)
            .unwrap_or(0);
        assert!(type_mismatch_count >= 8, "TypeMismatch should have most samples");
    }

    #[test]
    fn test_training_pairs_format() {
        let pairs = get_training_pairs();
        for (error, fix, _category) in &pairs {
            assert!(!error.is_empty(), "Error should not be empty");
            assert!(!fix.is_empty(), "Fix should not be empty");
        }
    }
}
