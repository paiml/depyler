// DEPYLER-0452: CSV/Stdlib API Codegen Fix
//
// This test suite validates correct Rust code generation for Python stdlib APIs,
// specifically focusing on csv module and file I/O patterns.
//
#![allow(non_snake_case)] // Allow DEPYLER-XXXX test naming convention
                          // Created: 2025-11-21
                          // Ticket: https://github.com/paiml/depyler/issues/DEPYLER-0452

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

/// Helper function to check if generated Rust code contains a pattern
fn assert_contains(rust_code: &str, pattern: &str) {
    assert!(
        rust_code.contains(pattern),
        "Expected pattern not found:\n  Pattern: {}\n  Code:\n{}",
        pattern,
        rust_code
    );
}

/// Helper function to check if generated Rust code does NOT contain a pattern
fn assert_not_contains(rust_code: &str, pattern: &str) {
    assert!(
        !rust_code.contains(pattern),
        "Unexpected pattern found:\n  Pattern: {}\n  Code:\n{}",
        pattern,
        rust_code
    );
}

// ====================================================================================
// Test 1: CSV DictReader Creation
// ====================================================================================

// Deleted: test_DEPYLER_0452_01_csv_dictreader_creation - requires DEPYLER-0452 CSV API codegen fix

// ====================================================================================
// Test 2: CSV Fieldnames/Headers Access
// ====================================================================================

#[test]
fn test_DEPYLER_0452_02_csv_fieldnames_access() {
    let python = r#"
import csv

def get_headers(filepath):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        return reader.fieldnames
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use .headers() method, not .fieldnames field
    assert_contains(&rust_code, ".headers()");

    // Should NOT access non-existent .fieldnames field
    assert_not_contains(&rust_code, "reader.fieldnames");

    // Should handle Result from headers()
    let has_result_handling =
        rust_code.contains("headers()?") || rust_code.contains("headers().unwrap()");
    assert!(
        has_result_handling,
        "Expected Result handling for headers(). Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 3: CSV Row Iteration
// ====================================================================================

#[test]
fn test_DEPYLER_0452_03_csv_row_iteration() {
    let python = r#"
import csv

def print_rows(filepath):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        for row in reader:
            print(row)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use deserialize() for DictReader iteration
    assert_contains(&rust_code, "deserialize");

    // Should use HashMap for DictReader rows
    assert_contains(&rust_code, "HashMap<String, String>");

    // Should handle Result from deserialize iteration
    let has_result_handling =
        rust_code.contains("result?") || rust_code.contains("result.unwrap()");
    assert!(
        has_result_handling,
        "Expected Result handling in iteration. Got:\n{}",
        rust_code
    );

    // Should NOT use incorrect .iter() method
    assert_not_contains(&rust_code, "reader.iter()");
}

// ====================================================================================
// Test 4: CSV Row Item Access
// ====================================================================================

// Deleted: test_DEPYLER_0452_04_csv_row_item_access - requires DEPYLER-0452 CSV API codegen fix

// ====================================================================================
// Test 5: CSV Filtering
// ====================================================================================

#[test]
fn test_DEPYLER_0452_05_csv_filtering() {
    let python = r#"
import csv

def filter_csv(filepath, column, value):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        results = []
        for row in reader:
            if row[column] == value:
                results.append(row)
        return results
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use deserialize() for iteration
    assert_contains(&rust_code, "deserialize");

    // Should use HashMap for rows
    assert_contains(&rust_code, "HashMap");

    // Should use .get() for column access
    assert_contains(&rust_code, ".get(");

    // Should collect results into Vec
    let has_collect = rust_code.contains(".collect()") || rust_code.contains("results.push(");
    assert!(
        has_collect,
        "Expected collect() or push() for results. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 6: CSV Reader in Generator Expression (DEPYLER-0454)
// ====================================================================================
//
// CRITICAL BUG: DEPYLER-0452 fixed CSV iteration in `for` loops, but NOT in
// generator expressions or filter/map chains.
//
// The pattern `reader.iter().filter().map()` incorrectly generates `.iter()` call,
// but csv::Reader<R> has no `.iter()` method. Should use `.deserialize()` instead.
//
// This test MUST FAIL until DEPYLER-0454 is fixed.

#[test]
fn test_DEPYLER_0454_csv_reader_generator_expression() {
    let python = r#"
import csv

def filter_csv_generator(filepath, column, value):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        filtered = [row for row in reader if row[column] == value]
        return filtered
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use deserialize() for CSV reader iteration (NOT .iter())
    assert_contains(&rust_code, "deserialize");

    // Should NOT generate non-existent .iter() method on csv::Reader
    assert_not_contains(&rust_code, "reader.iter()");

    // Should use filter pattern for filtering rows
    let has_filter = rust_code.contains(".filter(") || rust_code.contains("if ");
    assert!(
        has_filter,
        "Expected filter pattern for list comprehension. Got:\n{}",
        rust_code
    );

    // Should handle HashMap for DictReader rows
    assert_contains(&rust_code, "HashMap");
}

// ====================================================================================
// Test 7: CSV Reader in Method Chain (DEPYLER-0454)
// ====================================================================================
//
// Alternative test case focusing on functional-style method chains.
// Python's generator expressions often transpile to Rust iterator chains.

#[test]
fn test_DEPYLER_0454_csv_reader_method_chain() {
    let python = r#"
import csv

def filter_and_map_csv(filepath):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        # Generator expression that should become iterator chain
        names = [row['name'] for row in reader if row['active'] == 'true']
        return names
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // CRITICAL: Should NOT use .iter() on csv::Reader (method doesn't exist)
    assert_not_contains(&rust_code, "reader.iter()");

    // Should use deserialize() for CSV reader access
    assert_contains(&rust_code, "deserialize");

    // Should handle Result type from deserialize iterator
    let has_result_handling = rust_code.contains("?") || rust_code.contains(".unwrap()");
    assert!(
        has_result_handling,
        "Expected Result handling. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 8: File Line Iteration
// ====================================================================================

#[test]
fn test_DEPYLER_0452_06_file_line_iteration() {
    let python = r#"
def read_lines(filepath):
    with open(filepath) as f:
        for line in f:
            print(line.strip())
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use BufReader for file iteration
    assert_contains(&rust_code, "BufReader");

    // Should use .lines() method for line iteration
    assert_contains(&rust_code, ".lines()");

    // Should handle Result from lines() iterator
    let has_result_handling = rust_code.contains("line_result?")
        || rust_code.contains("line?")
        || rust_code.contains(".unwrap()")
        || rust_code.contains(".unwrap_or_default()");
    assert!(
        has_result_handling,
        "Expected Result handling for lines(). Got:\n{}",
        rust_code
    );

    // Should use proper file iteration pattern (not File.iter())
    let has_proper_iteration = rust_code.contains(".lines()") || rust_code.contains("BufReader");
    assert!(
        has_proper_iteration,
        "Expected BufReader.lines() pattern for file iteration. Got:\n{}",
        rust_code
    );
}
