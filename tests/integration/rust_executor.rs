use anyhow::{Context, Result};
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

/// Execute Rust code and return the result
pub fn execute_rust_code(rust_code: &str, function_name: &str, args: &[i32]) -> Result<i32> {
    // Create a temporary Rust file with a main function that calls our test function
    let wrapper_code = format!(
        r#"
{}

fn main() {{
    let result = {}({});
    println!("{{}}", result);
}}
"#,
        rust_code,
        function_name,
        args.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Write to temporary file with .rs extension
    let temp_file = NamedTempFile::new()?;
    let temp_rs_path = temp_file.path().with_extension("rs");
    fs::write(&temp_rs_path, wrapper_code)?;

    // Compile the Rust code with a valid crate name
    let output = Command::new("rustc")
        .arg("-O")
        .arg("--crate-name")
        .arg("temp_test")
        .arg("-o")
        .arg(temp_file.path().with_extension("exe"))
        .arg(&temp_rs_path)
        .output()
        .context("Failed to compile Rust code")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rust compilation failed: {}", stderr);
    }

    // Execute the compiled binary
    let output = Command::new(temp_file.path().with_extension("exe"))
        .output()
        .context("Failed to execute Rust binary")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rust execution failed: {}", stderr);
    }

    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = stdout
        .trim()
        .parse::<i32>()
        .context("Failed to parse output as i32")?;

    Ok(result)
}

/// Execute Python code and return the result
pub fn execute_python_code(python_code: &str, function_name: &str, args: &[i32]) -> Result<i32> {
    // Create a wrapper that calls the function and prints the result
    let wrapper_code = format!(
        r#"
{}

if __name__ == "__main__":
    result = {}({})
    print(result)
"#,
        python_code,
        function_name,
        args.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Write to temporary file
    let temp_file = NamedTempFile::new()?;
    fs::write(temp_file.path(), wrapper_code)?;

    // Execute Python code
    let output = Command::new("python3")
        .arg(temp_file.path())
        .output()
        .context("Failed to execute Python code")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Python execution failed: {}", stderr);
    }

    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = stdout
        .trim()
        .parse::<i32>()
        .context("Failed to parse output as i32")?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_execution() {
        let code = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let result = execute_rust_code(code, "add", &[2, 3]).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_python_execution() {
        let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

        let result = execute_python_code(code, "add", &[2, 3]).unwrap();
        assert_eq!(result, 5);
    }
}
