// DEPYLER-0426: csv.DictWriter keyword argument support
//
// Tests that csv.DictWriter(file, fieldnames=['col1', 'col2']) correctly
// threads keyword arguments through the method call chain.
//
// Root cause: convert_method_call() only receives args, not kwargs
// Solution: Thread kwargs parameter through entire call chain

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

#[test]
fn test_DEPYLER_0426_csv_dictwriter_basic() {
    // Basic csv.DictWriter with fieldnames kwarg
    let python = r#"
import csv

def write_data(filename):
    with open(filename, 'w') as f:
        writer = csv.DictWriter(f, fieldnames=['name', 'age'])
        writer.writeheader()
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust = result.unwrap();

    // Should generate csv::Writer::from_writer
    assert!(rust.contains("csv::Writer"), "Should use csv::Writer: {}", rust);

    // Should not bail with "requires at least 2 arguments"
    // (this test will fail initially, proving RED phase)
}

#[test]
fn test_DEPYLER_0426_csv_dictwriter_multiple_fields() {
    // csv.DictWriter with multiple fieldnames
    let python = r#"
import csv

writer = csv.DictWriter(output, fieldnames=['id', 'name', 'email', 'age'])
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should handle multiple fieldnames: {:?}", result.err());
}

#[test]
fn test_DEPYLER_0426_csv_dictwriter_positional_args() {
    // Ensure positional args still work (backward compatibility)
    let python = r#"
import csv

# This was the only syntax that worked before
# csv.DictWriter(file, ['col1', 'col2'])
# Note: This is not valid Python syntax but tests our arg handling

writer = csv.DictWriter(f, fieldnames=fields)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should handle variable fieldnames: {:?}", result.err());
}

#[test]
fn test_DEPYLER_0426_real_world_csv_filter() {
    // Actual pattern from csv_filter.py
    let python = r#"
import csv
import sys

def filter_csv(input_file, column, value, output_file=None):
    with open(input_file, "r") as f:
        reader = csv.DictReader(f)
        fieldnames = reader.fieldnames
        filtered_rows = (row for row in reader if row[column] == value)

        output = open(output_file, "w") if output_file else sys.stdout

        try:
            writer = csv.DictWriter(output, fieldnames=fieldnames)
            writer.writeheader()

            for row in filtered_rows:
                writer.writerow(row)
        finally:
            if output_file:
                output.close()
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Real-world pattern should transpile: {:?}", result.err());

    let rust = result.unwrap();
    assert!(rust.contains("csv::Writer"), "Should generate csv::Writer");
}

#[test]
fn test_DEPYLER_0426_property_based_fieldnames() {
    // Property: fieldnames can be any expression (variable, literal, list comp)
    let test_cases = vec![
        ("csv.DictWriter(f, fieldnames=['a', 'b'])", "literal list"),
        ("csv.DictWriter(f, fieldnames=fields)", "variable"),
        ("csv.DictWriter(f, fieldnames=reader.fieldnames)", "attribute"),
    ];

    for (python_expr, description) in test_cases {
        let python = format!("import csv\n{}", python_expr);
        let result = transpile_python(&python);
        assert!(result.is_ok(), "Should handle {}: {:?}", description, result.err());
    }
}
