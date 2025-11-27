//! Test the AutoFixer on real compilation errors.

use depyler_oracle::AutoFixer;
use std::fs;
use std::process::Command;

fn main() {
    println!("=== AutoFixer Test ===\n");

    // Test 1: stdlib_integration borrow-after-move
    test_stdlib_integration();
}

fn test_stdlib_integration() {
    println!("Test: stdlib_integration.rs (E0382 borrow-after-move)");

    let source_path = "/home/noah/src/reprorusted-python-cli/examples/example_stdlib/stdlib_integration.rs";
    let source = fs::read_to_string(source_path).expect("Failed to read source");

    // Simulate the compilation error
    let errors = r#"error[E0382]: borrow of moved value: `args.hash`
   --> stdlib_integration.rs:387:56
    |
379 |         let mut info = get_file_info(args.file, args.hash, args.time_format)?;
    |                                                 --------- value moved here
...
387 |                 output = format_output_text(&mut info, args.hash.is_some())?;
    |                                                        ^^^^^^^^^ value borrowed here after move
    |
    = note: move occurs because `args.hash` has type `Option<std::string::String>`, which does not implement the `Copy` trait"#;

    // Create autofixer
    println!("  Loading oracle...");
    let fixer = AutoFixer::new().expect("Failed to create AutoFixer");

    // Attempt fix
    println!("  Applying fixes...");
    let result = fixer.fix(&source, errors);

    if result.fixed {
        println!("  FIXED! Confidence: {:.1}%", result.confidence * 100.0);
        println!("  Changes:\n{}", result.description);

        // Write fixed file
        let fixed_path = "/tmp/stdlib_integration_fixed.rs";
        fs::write(fixed_path, &result.source).expect("Failed to write fixed source");
        println!("\n  Fixed source written to: {}", fixed_path);

        // Try to compile
        println!("\n  Testing compilation...");
        let output = Command::new("rustc")
            .args(["--crate-type", "lib", "--emit=metadata", "-o", "/tmp/test.rmeta", fixed_path])
            .output()
            .expect("Failed to run rustc");

        if output.status.success() {
            println!("  COMPILATION SUCCESS!");
        } else {
            println!("  Compilation failed (may need more fixes)");
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                println!("  Remaining errors:\n{}", stderr.lines().take(10).collect::<Vec<_>>().join("\n"));
            }
        }
    } else {
        println!("  No fixes applied");
    }
}
