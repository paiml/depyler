# Depyler Single-Shot Compile Specification

**Document ID**: DEPYLER-COMPILE-001
**Version**: 1.0.0
**Status**: Implementation - EXTREME TDD
**Author**: Depyler Team
**Date**: 2025-11-12
**Related Specs**:
- [DEPYLER-PERF-001](./optimized-python-to-rust-compilation.md)
- [Convert Python to Optimized Rust Binary Examples](./convert-python-to-optimized-rust-binary-examples.md)

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [User Experience Goals](#user-experience-goals)
3. [Command Interface](#command-interface)
4. [Implementation Architecture](#implementation-architecture)
5. [EXTREME TDD Protocol](#extreme-tdd-protocol)
6. [Testing Strategy](#testing-strategy)
7. [Performance Measurement](#performance-measurement)
8. [Quality Gates](#quality-gates)
9. [Implementation Phases](#implementation-phases)
10. [Reference Implementations](#reference-implementations)

---

## Executive Summary

### Vision

Enable Python developers to compile ANY Python script to a highly optimized native binary with a **single command**, just like `deno compile`, `uv compile`, or `ruchy compile`.

```bash
# User writes Python
cat > wordcount.py << 'EOF'
#!/usr/bin/env python3
import argparse
import sys

def count_words(filename):
    with open(filename) as f:
        content = f.read()
    return len(content.split())

def main():
    parser = argparse.ArgumentParser(description='Count words in files')
    parser.add_argument('files', nargs='+', help='Files to process')
    parser.add_argument('-v', '--verbose', action='store_true')
    args = parser.parse_args()

    for filename in args.files:
        count = count_words(filename)
        print(f"{filename}: {count} words")

if __name__ == '__main__':
    main()
EOF

# Single command produces optimized binary
depyler compile wordcount.py

# Binary ready to use - 10-50x faster than Python
./wordcount README.md LICENSE
```

### Key Results

| Metric | Target | Measured |
|--------|--------|----------|
| **Command Simplicity** | 1 command | `depyler compile script.py` |
| **Success Rate** | ≥95% Python scripts | TBD (via matrix testing) |
| **Performance Gain** | 10-50x vs CPython | Validated via benchmarks |
| **Binary Size** | <5 MB (CLI tools) | Profile-dependent |
| **Compilation Time** | <60s (typical CLI) | Measured per workload |
| **Test Coverage** | ≥85% | Enforced via cargo-llvm-cov |
| **Mutation Kill Rate** | ≥80% | Enforced via cargo-mutants |

---

## User Experience Goals

### Design Principles

1. **"Just Works"** - User shouldn't think about transpilation, Cargo, or Rust
2. **Intelligent Defaults** - Optimal profile selected based on workload detection
3. **Escape Hatches** - Advanced users can override everything
4. **Fast Feedback** - Progress indicators, clear error messages
5. **Reproducible** - Same input always produces same binary

### User Personas

#### Persona 1: Python Developer (No Rust Knowledge)

**Goal**: Make my CLI tool faster without learning Rust

```bash
# Current workflow (slow)
python3 my_tool.py input.csv

# Desired workflow (fast)
depyler compile my_tool.py
./my_tool input.csv  # 10-50x faster
```

**Requirements**:
- No Rust/Cargo knowledge needed
- Clear error messages referencing Python concepts
- Automatic dependency detection

#### Persona 2: Performance Engineer

**Goal**: Maximum performance for specific workload

```bash
# Profile the Python code
depyler compile --profile perf-ultra --pgo my_tool.py

# Result: 25-50x faster with PGO
```

**Requirements**:
- Control over optimization profiles
- Benchmarking integration
- Size vs speed trade-offs

#### Persona 3: Embedded/IoT Developer

**Goal**: Smallest possible binary for resource-constrained environments

```bash
# Optimize for size
depyler compile --profile min-size my_sensor.py

# Result: <500KB binary, still 2-5x faster than Python
```

**Requirements**:
- Ultra-small binaries
- Cross-compilation support
- Static linking

---

## Command Interface

### Primary Command

```bash
depyler compile [OPTIONS] <PYTHON_SCRIPT>
```

### Options

```
USAGE:
    depyler compile [OPTIONS] <SCRIPT>

ARGS:
    <SCRIPT>    Python script to compile (.py file)

OPTIONS:
    -o, --output <OUTPUT>         Output binary name [default: script name without .py]
    --profile <PROFILE>           Optimization profile [default: auto]
                                  Values: auto, release, perf-ultra, min-size, debug
    --target <TARGET>             Rust target triple [default: native]
    --pgo                         Enable Profile-Guided Optimization (slower build, faster binary)
    --strip                       Strip debug symbols (smaller binary)
    --upx                         Compress binary with UPX (smallest size)
    --keep-rust                   Keep generated Rust source after compilation
    --cargo-args <ARGS>           Additional arguments to pass to cargo build
    --benchmark                   Run benchmarks comparing to Python
    --verify                      Run verification tests comparing Rust and Python output
    -v, --verbose                 Verbose output
    -q, --quiet                   Minimal output
    -h, --help                    Print help
    -V, --version                 Print version

PROFILES:
    auto         Automatically detect workload and select optimal profile (default)
    release      Balanced optimization (lto=fat, opt-level=3) - 15x speedup, 1.5MB
    perf-ultra   Maximum performance (PGO + native CPU) - 25-50x speedup, 500KB
    min-size     Smallest binary (opt-level=z, lto=fat, panic=abort) - 2-5x speedup, 300KB
    debug        Fast compilation, debug symbols - For development only

EXAMPLES:
    # Basic compilation (automatic optimization)
    depyler compile wordcount.py

    # Maximum performance
    depyler compile --profile perf-ultra --pgo compute_intensive.py

    # Minimum size for embedded
    depyler compile --profile min-size --strip sensor_reader.py

    # Cross-compile for Raspberry Pi
    depyler compile --target aarch64-unknown-linux-gnu server.py

    # Benchmark against Python
    depyler compile --benchmark --verify my_tool.py
```

### Output

```bash
$ depyler compile wordcount.py
🔬 Analyzing wordcount.py...
   ├─ 45 lines of Python
   ├─ Detected patterns: argparse CLI, file I/O
   └─ Recommended profile: release

🦀 Transpiling to Rust...
   └─ Generated 127 lines of idiomatic Rust

⚙️  Compiling with profile 'release' (lto=fat)...
   Compiling wordcount v0.1.0
    Finished release [optimized] target(s) in 8.3s

📦 Binary ready: ./wordcount
   ├─ Size: 1.4 MB
   ├─ Estimated speedup: 10-20x
   └─ Run: ./wordcount --help

✨ Done in 9.1s
```

### Error Handling

```bash
$ depyler compile broken.py
❌ Error: Python transpilation failed

  Python syntax error at line 15:
    def broken_function(
                       ^
  SyntaxError: unexpected EOF while parsing

  Fix the Python code and try again.

$ depyler compile unsupported.py
⚠️  Warning: Partial support

   The following features are not yet fully supported:
   - async/await generators (line 23)
   - metaclasses (line 45)

   Generated Rust may not match Python behavior exactly.
   Use --verify to check output equivalence.

   Continue anyway? [y/N]
```

---

## Implementation Architecture

### High-Level Flow

```
┌─────────────────┐
│ Python Script   │
│ (wordcount.py)  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ depyler compile                     │
│                                     │
│ 1. Validate Python syntax          │
│ 2. Detect workload type             │
│ 3. Select optimization profile      │
│ 4. Transpile to Rust                │
│ 5. Create Cargo project             │
│ 6. Compile with optimal flags       │
│ 7. Strip/compress (if requested)    │
│ 8. Benchmark (if requested)         │
│ 9. Verify correctness (if requested)│
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────┐
│ Native Binary   │
│ (wordcount)     │
│ 10-50x faster!  │
└─────────────────┘
```

### Component Design

#### 1. Workload Detector

**Purpose**: Analyze Python code to recommend optimal compilation profile

```rust
pub struct WorkloadDetector {
    ast: PythonAST,
}

impl WorkloadDetector {
    pub fn detect(&self) -> WorkloadType {
        // Heuristics:
        // - Heavy loops + math ops → perf-ultra
        // - Argparse + file I/O → release
        // - Simple script → min-size
        // - Complex logic → release (balanced)
    }
}

pub enum WorkloadType {
    CpuIntensive,    // Numeric computation, tight loops
    MemoryIntensive, // Large data structures, sorting
    IoIntensive,     // File/network operations
    CliTool,         // Argparse + light processing
    Simple,          // <100 lines, minimal logic
}
```

#### 2. Profile Manager

**Purpose**: Map workload types to optimal Cargo profiles

```rust
pub struct CompilationProfile {
    name: String,
    cargo_profile: String,
    target_cpu: Option<String>,
    enable_pgo: bool,
    enable_strip: bool,
    expected_speedup_range: (f64, f64),
    expected_size_kb: u64,
}

impl CompilationProfile {
    pub fn auto_select(workload: WorkloadType) -> Self {
        match workload {
            WorkloadType::CpuIntensive => Self::perf_ultra(),
            WorkloadType::CliTool => Self::release(),
            WorkloadType::Simple => Self::min_size(),
            _ => Self::release(),
        }
    }

    pub fn release() -> Self {
        CompilationProfile {
            name: "release".into(),
            cargo_profile: "release".into(),
            target_cpu: None,
            enable_pgo: false,
            enable_strip: false,
            expected_speedup_range: (10.0, 20.0),
            expected_size_kb: 1500,
        }
    }

    pub fn perf_ultra() -> Self {
        CompilationProfile {
            name: "perf-ultra".into(),
            cargo_profile: "perf-ultra".into(),
            target_cpu: Some("native".into()),
            enable_pgo: true,
            enable_strip: true,
            expected_speedup_range: (25.0, 50.0),
            expected_size_kb: 500,
        }
    }

    pub fn min_size() -> Self {
        CompilationProfile {
            name: "min-size".into(),
            cargo_profile: "min-size".into(),
            target_cpu: None,
            enable_pgo: false,
            enable_strip: true,
            expected_speedup_range: (2.0, 5.0),
            expected_size_kb: 300,
        }
    }
}
```

#### 3. Cargo Project Generator

**Purpose**: Create complete Cargo project from transpiled Rust

```rust
pub struct CargoProjectGenerator {
    project_name: String,
    rust_code: String,
    profile: CompilationProfile,
}

impl CargoProjectGenerator {
    pub fn generate(&self, output_dir: &Path) -> Result<()> {
        // 1. Create Cargo.toml with optimized profiles
        self.write_cargo_toml(output_dir)?;

        // 2. Write Rust source to src/main.rs
        self.write_main_rs(output_dir)?;

        // 3. Add .cargo/config.toml for target-specific flags
        self.write_cargo_config(output_dir)?;

        // 4. Copy runtime dependencies if needed
        self.copy_dependencies(output_dir)?;

        Ok(())
    }

    fn cargo_toml_template(&self) -> String {
        format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{}"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = false
panic = "unwind"

[profile.perf-ultra]
inherits = "release"
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[profile.min-size]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[dependencies]
# Depyler runtime (if needed for advanced features)
# depyler-runtime = "0.1"
"#,
        self.project_name, self.project_name)
    }
}
```

#### 4. PGO (Profile-Guided Optimization) Runner

**Purpose**: Two-phase compilation for maximum performance

```rust
pub struct PgoRunner {
    project_dir: PathBuf,
    training_inputs: Vec<PathBuf>,
}

impl PgoRunner {
    pub fn run_pgo(&self) -> Result<()> {
        // Phase 1: Instrumented build
        self.build_instrumented()?;

        // Phase 2: Collect profile data
        self.run_training_workload()?;

        // Phase 3: Optimized build with profile data
        self.build_optimized()?;

        Ok(())
    }

    fn build_instrumented(&self) -> Result<()> {
        // RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data"
        // cargo build --release
        todo!()
    }

    fn run_training_workload(&self) -> Result<()> {
        // Execute binary with representative inputs
        // to collect profile data
        todo!()
    }

    fn build_optimized(&self) -> Result<()> {
        // RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata"
        // cargo build --release
        todo!()
    }
}
```

#### 5. Verification Runner

**Purpose**: Ensure Rust binary produces same output as Python

```rust
pub struct VerificationRunner {
    python_script: PathBuf,
    rust_binary: PathBuf,
    test_inputs: Vec<TestCase>,
}

pub struct TestCase {
    args: Vec<String>,
    stdin: Option<String>,
    expected_stdout: Option<String>,
    expected_stderr: Option<String>,
    expected_exit_code: i32,
}

impl VerificationRunner {
    pub fn verify(&self) -> Result<VerificationReport> {
        let mut report = VerificationReport::new();

        for test_case in &self.test_inputs {
            // Run Python version
            let python_output = self.run_python(test_case)?;

            // Run Rust version
            let rust_output = self.run_rust(test_case)?;

            // Compare outputs
            if python_output == rust_output {
                report.add_pass(test_case);
            } else {
                report.add_fail(test_case, python_output, rust_output);
            }
        }

        Ok(report)
    }
}
```

#### 6. Benchmark Runner

**Purpose**: Measure performance improvement vs Python

```rust
pub struct BenchmarkRunner {
    python_script: PathBuf,
    rust_binary: PathBuf,
    workload: WorkloadType,
}

impl BenchmarkRunner {
    pub fn benchmark(&self) -> Result<BenchmarkReport> {
        // Generate representative workload
        let inputs = self.generate_benchmark_inputs();

        // Benchmark Python (5 iterations, median)
        let python_time = self.benchmark_python(&inputs)?;

        // Benchmark Rust (5 iterations, median)
        let rust_time = self.benchmark_rust(&inputs)?;

        // Calculate speedup
        let speedup = python_time / rust_time;

        Ok(BenchmarkReport {
            python_time_ms: python_time,
            rust_time_ms: rust_time,
            speedup,
            binary_size_kb: self.get_binary_size()?,
        })
    }
}
```

---

## EXTREME TDD Protocol

### Red-Green-Refactor Cycle

Following CLAUDE.md EXTREME TDD requirements:

#### Phase 1: RED (Write Failing Tests First)

```rust
// tests/test_compile_command.rs

#[test]
fn test_compile_simple_hello_world() {
    // RED: This test will fail initially
    let python_code = r#"
def main():
    print("Hello, World!")

if __name__ == '__main__':
    main()
"#;

    let temp_file = write_temp_file("hello.py", python_code);

    // Run depyler compile
    let output = Command::new("depyler")
        .args(&["compile", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to run depyler compile");

    assert!(output.status.success());

    // Binary should exist
    let binary = temp_file.path().with_extension("");
    assert!(binary.exists());

    // Binary should run and produce correct output
    let run_output = Command::new(&binary)
        .output()
        .expect("Failed to run compiled binary");

    assert_eq!(
        String::from_utf8_lossy(&run_output.stdout),
        "Hello, World!\n"
    );
}

#[test]
fn test_compile_argparse_cli() {
    // RED: Tests argparse transpilation + compilation
    let python_code = include_str!("fixtures/wordcount.py");
    let temp_file = write_temp_file("wordcount.py", python_code);

    // Compile
    let compile_output = Command::new("depyler")
        .args(&["compile", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to compile");

    assert!(compile_output.status.success());

    // Test binary with arguments
    let binary = temp_file.path().with_extension("");
    let test_file = write_temp_file("test.txt", "hello world test");

    let run_output = Command::new(&binary)
        .arg(test_file.path())
        .output()
        .expect("Failed to run");

    assert!(run_output.stdout.contains(b"3 words"));
}

#[test]
fn test_compile_with_profile_perf_ultra() {
    let python_code = r#"
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

def main():
    print(fibonacci(30))

if __name__ == '__main__':
    main()
"#;

    let temp_file = write_temp_file("fib.py", python_code);

    let output = Command::new("depyler")
        .args(&[
            "compile",
            "--profile", "perf-ultra",
            temp_file.path().to_str().unwrap()
        ])
        .output()
        .expect("Failed to compile");

    assert!(output.status.success());

    // Binary should be smaller due to stripping
    let binary = temp_file.path().with_extension("");
    let size = std::fs::metadata(&binary).unwrap().len();
    assert!(size < 1_000_000, "Binary should be < 1MB with perf-ultra profile");
}

#[test]
fn test_compile_keep_rust_source() {
    let python_code = "print('test')";
    let temp_file = write_temp_file("test.py", python_code);

    let output = Command::new("depyler")
        .args(&[
            "compile",
            "--keep-rust",
            temp_file.path().to_str().unwrap()
        ])
        .output()
        .expect("Failed to compile");

    assert!(output.status.success());

    // Rust source should be kept
    let rust_dir = temp_file.path().with_extension(".rust");
    assert!(rust_dir.exists());
    assert!(rust_dir.join("src/main.rs").exists());
    assert!(rust_dir.join("Cargo.toml").exists());
}
```

#### Phase 2: GREEN (Minimal Implementation)

```rust
// crates/depyler/src/commands/compile.rs

pub struct CompileCommand {
    pub script: PathBuf,
    pub output: Option<PathBuf>,
    pub profile: CompilationProfile,
    pub keep_rust: bool,
    pub benchmark: bool,
    pub verify: bool,
}

impl CompileCommand {
    pub fn execute(&self) -> Result<()> {
        // Step 1: Validate Python
        self.validate_python()?;

        // Step 2: Transpile to Rust
        let rust_code = self.transpile()?;

        // Step 3: Create Cargo project
        let project_dir = self.create_cargo_project(&rust_code)?;

        // Step 4: Compile
        self.compile_rust(&project_dir)?;

        // Step 5: Move binary to output location
        self.install_binary(&project_dir)?;

        // Step 6: Optional verification
        if self.verify {
            self.run_verification()?;
        }

        // Step 7: Optional benchmarking
        if self.benchmark {
            self.run_benchmark()?;
        }

        // Step 8: Cleanup (unless --keep-rust)
        if !self.keep_rust {
            std::fs::remove_dir_all(&project_dir)?;
        }

        Ok(())
    }
}
```

#### Phase 3: REFACTOR (Quality Enforcement)

```bash
# After GREEN phase, enforce quality gates

# Complexity ≤10
pmat analyze complexity --file crates/depyler/src/commands/compile.rs \
    --max-cyclomatic 10 --fail-on-violation

# TDG ≤2.0
pmat analyze tdg --path crates/depyler/src/commands/compile.rs \
    --threshold 2.0 --critical-only

# Coverage ≥85%
cargo llvm-cov --package depyler --lib --tests --fail-under-lines 85

# Zero SATD
pmat analyze satd --path crates/depyler/src/commands/compile.rs \
    --fail-on-violation

# Mutation testing ≥80%
cargo mutants --file crates/depyler/src/commands/compile.rs
```

### Quality Gates (MANDATORY - BLOCKING)

All tests must pass before proceeding:

1. ✅ **Unit Tests**: 100% pass rate
2. ✅ **Integration Tests**: All CLI scenarios
3. ✅ **Property Tests**: Fuzz inputs
4. ✅ **Mutation Tests**: ≥80% kill rate
5. ✅ **Coverage**: ≥85% line coverage
6. ✅ **Complexity**: ≤10 cyclomatic
7. ✅ **TDG Score**: ≤2.0 (Grade A-)
8. ✅ **Clippy**: Zero warnings
9. ✅ **SATD**: Zero TODO/FIXME

---

## Testing Strategy

### 1. Unit Tests

Test each component in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workload_detector_cpu_intensive() {
        let python = r#"
for i in range(1000000):
    result = i * i + i * 2
"#;
        let detector = WorkloadDetector::new(python);
        assert_eq!(detector.detect(), WorkloadType::CpuIntensive);
    }

    #[test]
    fn test_workload_detector_cli_tool() {
        let python = r#"
import argparse
parser = argparse.ArgumentParser()
parser.add_argument('--input')
"#;
        let detector = WorkloadDetector::new(python);
        assert_eq!(detector.detect(), WorkloadType::CliTool);
    }

    #[test]
    fn test_profile_manager_auto_select() {
        assert_eq!(
            CompilationProfile::auto_select(WorkloadType::CpuIntensive).name,
            "perf-ultra"
        );
        assert_eq!(
            CompilationProfile::auto_select(WorkloadType::CliTool).name,
            "release"
        );
    }
}
```

### 2. Integration Tests

Test complete compile workflows:

```rust
// tests/integration/compile_tests.rs

#[test]
fn test_compile_hello_world_e2e() {
    let workspace = TempWorkspace::new();
    workspace.write_python("hello.py", "print('Hello')");

    let output = workspace.compile("hello.py");
    assert!(output.success);

    let result = workspace.run_binary("hello");
    assert_eq!(result.stdout, "Hello\n");
}

#[test]
fn test_compile_argparse_cli_e2e() {
    let workspace = TempWorkspace::new();
    workspace.write_python("cli.py", WORDCOUNT_SOURCE);
    workspace.write_file("test.txt", "hello world");

    let output = workspace.compile("cli.py");
    assert!(output.success);

    let result = workspace.run_binary_with_args("cli", &["test.txt"]);
    assert!(result.stdout.contains("2 words"));
}

#[test]
fn test_compile_with_all_profiles() {
    let profiles = ["release", "perf-ultra", "min-size", "debug"];

    for profile in &profiles {
        let workspace = TempWorkspace::new();
        workspace.write_python("script.py", SIMPLE_SCRIPT);

        let output = workspace.compile_with_profile("script.py", profile);
        assert!(output.success, "Profile {} failed", profile);

        let result = workspace.run_binary("script");
        assert!(result.success);
    }
}
```

### 3. Property Tests

Fuzz test various Python patterns:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_compile_any_valid_python(python_code in python_script_generator()) {
        // If Python compiles, transpilation should work
        if validate_python_syntax(&python_code).is_ok() {
            let result = compile_python(&python_code);

            // Either succeeds or gives clear unsupported feature error
            match result {
                Ok(_) => {
                    // Compiled binary should run
                    let output = run_binary();
                    assert!(output.status.success());
                }
                Err(e) => {
                    // Error should be actionable
                    assert!(e.is_unsupported_feature() || e.is_transpile_error());
                }
            }
        }
    }

    #[test]
    fn prop_compiled_binary_matches_python_output(
        script in simple_python_generator(),
        inputs in test_input_generator()
    ) {
        let python_output = run_python(&script, &inputs);

        if let Ok(binary) = compile_python(&script) {
            let rust_output = run_binary(&binary, &inputs);

            // Outputs should match
            assert_eq!(python_output.stdout, rust_output.stdout);
            assert_eq!(python_output.stderr, rust_output.stderr);
            assert_eq!(python_output.exit_code, rust_output.exit_code);
        }
    }

    #[test]
    fn prop_binary_size_within_expected_range(
        script in python_script_generator(),
        profile in profile_generator()
    ) {
        if let Ok(binary) = compile_with_profile(&script, &profile) {
            let size = get_binary_size(&binary);

            // Size should be within profile's expected range
            assert!(
                size >= profile.min_expected_size(),
                "Binary too small: {} < {}",
                size,
                profile.min_expected_size()
            );
            assert!(
                size <= profile.max_expected_size(),
                "Binary too large: {} > {}",
                size,
                profile.max_expected_size()
            );
        }
    }
}

fn python_script_generator() -> impl Strategy<Value = String> {
    prop::collection::vec(python_statement_generator(), 1..50)
        .prop_map(|stmts| stmts.join("\n"))
}

fn simple_python_generator() -> impl Strategy<Value = String> {
    // Generate simple Python scripts that are guaranteed to work
    prop_oneof![
        Just("print('hello')".to_string()),
        Just("x = 42\nprint(x)".to_string()),
        Just("for i in range(10):\n    print(i)".to_string()),
    ]
}
```

### 4. Matrix Testing

Test multiple CLI patterns (inspired by https://github.com/paiml/ruchy):

```rust
// tests/matrix/cli_patterns.rs

const CLI_PATTERNS: &[(&str, &str)] = &[
    ("argparse_basic", include_str!("fixtures/argparse_basic.py")),
    ("argparse_subcommands", include_str!("fixtures/argparse_subcommands.py")),
    ("argparse_file_io", include_str!("fixtures/argparse_file_io.py")),
    ("click_cli", include_str!("fixtures/click_cli.py")),
    ("typer_cli", include_str!("fixtures/typer_cli.py")),
    ("fire_cli", include_str!("fixtures/fire_cli.py")),
];

#[test]
fn test_all_cli_patterns() {
    for (name, source) in CLI_PATTERNS {
        println!("Testing CLI pattern: {}", name);

        let workspace = TempWorkspace::new();
        workspace.write_python(&format!("{}.py", name), source);

        let compile_result = workspace.compile(&format!("{}.py", name));

        // All patterns should compile successfully
        assert!(
            compile_result.success,
            "CLI pattern '{}' failed to compile:\n{}",
            name,
            compile_result.stderr
        );

        // Run basic smoke test
        let run_result = workspace.run_binary_with_args(name, &["--help"]);
        assert!(run_result.success, "CLI pattern '{}' failed to run", name);
    }
}
```

### 5. Mutation Tests

Verify test quality with cargo-mutants:

```bash
# Mutation testing on compile command implementation
cargo mutants --file crates/depyler/src/commands/compile.rs \
    --baseline skip \
    --timeout 300 \
    --jobs 4

# Expected: ≥80% mutation kill rate
# Mutants should be caught by:
# - Unit tests (component behavior)
# - Integration tests (E2E workflows)
# - Property tests (edge cases)
```

---

## Performance Measurement

### Benchmark Infrastructure

Following patterns from https://github.com/paiml/compiled-rust-benchmarking:

#### 1. Workload Suite

```rust
// benchmarks/workloads.rs

pub struct Workload {
    pub name: &'static str,
    pub python_source: &'static str,
    pub input_generator: fn() -> Vec<String>,
    pub expected_speedup_range: (f64, f64),
}

pub const WORKLOADS: &[Workload] = &[
    Workload {
        name: "fibonacci_recursive",
        python_source: include_str!("../fixtures/fibonacci.py"),
        input_generator: || vec!["30".to_string()],
        expected_speedup_range: (15.0, 30.0),
    },
    Workload {
        name: "quicksort",
        python_source: include_str!("../fixtures/quicksort.py"),
        input_generator: generate_random_array,
        expected_speedup_range: (30.0, 60.0),
    },
    Workload {
        name: "wordcount",
        python_source: include_str!("../fixtures/wordcount.py"),
        input_generator: generate_text_file,
        expected_speedup_range: (8.0, 15.0),
    },
    Workload {
        name: "json_parser",
        python_source: include_str!("../fixtures/json_parser.py"),
        input_generator: generate_json_file,
        expected_speedup_range: (10.0, 25.0),
    },
    Workload {
        name: "regex_matcher",
        python_source: include_str!("../fixtures/regex_matcher.py"),
        input_generator: generate_text_corpus,
        expected_speedup_range: (5.0, 12.0),
    },
];
```

#### 2. Benchmark Runner

```rust
pub struct BenchmarkSuite {
    workloads: Vec<Workload>,
    profiles: Vec<CompilationProfile>,
    iterations: usize,
}

impl BenchmarkSuite {
    pub fn run(&self) -> BenchmarkResults {
        let mut results = BenchmarkResults::new();

        for workload in &self.workloads {
            println!("Benchmarking: {}", workload.name);

            // Baseline: Python performance
            let python_time = self.benchmark_python(workload);

            for profile in &self.profiles {
                // Compile with profile
                let binary = compile_with_profile(
                    workload.python_source,
                    profile
                )?;

                // Benchmark Rust binary
                let rust_time = self.benchmark_rust(&binary, workload);

                // Calculate metrics
                let speedup = python_time / rust_time;
                let binary_size = get_binary_size(&binary);

                results.add(BenchmarkResult {
                    workload: workload.name,
                    profile: profile.name.clone(),
                    python_time_ms: python_time,
                    rust_time_ms: rust_time,
                    speedup,
                    binary_size_kb: binary_size / 1024,
                    within_expected_range: workload.expected_speedup_range.0 <= speedup
                        && speedup <= workload.expected_speedup_range.1,
                });
            }
        }

        results
    }

    fn benchmark_python(&self, workload: &Workload) -> f64 {
        let inputs = (workload.input_generator)();

        let mut times = Vec::new();
        for _ in 0..self.iterations {
            let start = Instant::now();

            let _ = Command::new("python3")
                .args(&inputs)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .output();

            times.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        // Return median time
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        times[times.len() / 2]
    }

    fn benchmark_rust(&self, binary: &Path, workload: &Workload) -> f64 {
        let inputs = (workload.input_generator)();

        let mut times = Vec::new();
        for _ in 0..self.iterations {
            let start = Instant::now();

            let _ = Command::new(binary)
                .args(&inputs)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .output();

            times.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        times[times.len() / 2]
    }
}
```

#### 3. Statistical Analysis

```rust
pub struct BenchmarkResults {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkResults {
    pub fn analyze(&self) -> StatisticalReport {
        // Calculate statistics
        let speedups: Vec<f64> = self.results.iter().map(|r| r.speedup).collect();

        let mean_speedup = speedups.iter().sum::<f64>() / speedups.len() as f64;
        let median_speedup = {
            let mut sorted = speedups.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted[sorted.len() / 2]
        };
        let min_speedup = speedups.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_speedup = speedups.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // ANOVA: Test if profile choice significantly affects performance
        let anova_result = self.run_anova();

        StatisticalReport {
            mean_speedup,
            median_speedup,
            min_speedup,
            max_speedup,
            anova_f_statistic: anova_result.f,
            anova_p_value: anova_result.p,
            effect_size_eta_squared: anova_result.eta_squared,
            all_within_expected: self.results.iter().all(|r| r.within_expected_range),
        }
    }

    pub fn print_report(&self) {
        println!("\n=== Depyler Compile Benchmark Results ===\n");
        println!("| Workload | Profile | Speedup | Binary Size | Within Expected |");
        println!("|----------|---------|---------|-------------|-----------------|");

        for result in &self.results {
            println!(
                "| {} | {} | {:.2}x | {} KB | {} |",
                result.workload,
                result.profile,
                result.speedup,
                result.binary_size_kb,
                if result.within_expected_range { "✅" } else { "❌" }
            );
        }

        let analysis = self.analyze();
        println!("\n=== Statistical Analysis ===\n");
        println!("Mean speedup: {:.2}x", analysis.mean_speedup);
        println!("Median speedup: {:.2}x", analysis.median_speedup);
        println!("Min speedup: {:.2}x", analysis.min_speedup);
        println!("Max speedup: {:.2}x", analysis.max_speedup);
        println!("ANOVA F-statistic: {:.2}", analysis.anova_f_statistic);
        println!("p-value: {:.4}", analysis.anova_p_value);
        println!("Effect size (η²): {:.3}", analysis.effect_size_eta_squared);

        if analysis.all_within_expected {
            println!("\n✅ All workloads achieved expected speedup ranges");
        } else {
            println!("\n⚠️ Some workloads outside expected ranges - investigation needed");
        }
    }
}
```

### Performance Validation Tests

```rust
#[test]
fn test_benchmark_fibonacci_meets_target() {
    let workload = WORKLOADS.iter()
        .find(|w| w.name == "fibonacci_recursive")
        .unwrap();

    let binary = compile_python(workload.python_source, "perf-ultra").unwrap();

    let python_time = benchmark_python(workload);
    let rust_time = benchmark_rust(&binary, workload);
    let speedup = python_time / rust_time;

    assert!(
        speedup >= workload.expected_speedup_range.0,
        "Speedup {:.2}x below expected minimum {:.2}x",
        speedup,
        workload.expected_speedup_range.0
    );
}

#[test]
fn test_all_workloads_compile_and_run() {
    for workload in WORKLOADS {
        let binary = compile_python(workload.python_source, "release")
            .expect(&format!("Failed to compile {}", workload.name));

        let inputs = (workload.input_generator)();
        let output = run_binary(&binary, &inputs)
            .expect(&format!("Failed to run {}", workload.name));

        assert!(output.status.success());
    }
}
```

---

## Quality Gates

### Pre-Commit Quality Gates (BLOCKING)

All gates must pass before commit:

```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "🔍 Running quality gates..."

# 1. Format check
echo "  ├─ Format check..."
cargo fmt --check

# 2. Clippy (zero warnings)
echo "  ├─ Clippy (zero warnings)..."
cargo clippy --all-targets --all-features -- -D warnings

# 3. Unit tests
echo "  ├─ Unit tests..."
cargo test --lib

# 4. Integration tests
echo "  ├─ Integration tests..."
cargo test --test compile_tests

# 5. Coverage ≥85%
echo "  ├─ Coverage ≥85%..."
cargo llvm-cov --package depyler --lib --tests --fail-under-lines 85

# 6. Complexity ≤10
echo "  ├─ Complexity ≤10..."
pmat analyze complexity \
    --file crates/depyler/src/commands/compile.rs \
    --max-cyclomatic 10 \
    --fail-on-violation

# 7. TDG ≤2.0
echo "  ├─ TDG ≤2.0..."
pmat analyze tdg \
    --path crates/depyler/src/commands/compile.rs \
    --threshold 2.0 \
    --critical-only

# 8. Zero SATD
echo "  └─ Zero SATD..."
pmat analyze satd \
    --path crates/depyler/src/commands/compile.rs \
    --fail-on-violation

echo "✅ All quality gates passed!"
```

### CI/CD Quality Gates (BLOCKING)

```yaml
# .github/workflows/quality.yml

name: Quality Gates

on: [push, pull_request]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          override: true

      - name: Run all tests
        run: cargo test --workspace

      - name: Matrix testing (CLI patterns)
        run: cargo test --test matrix_cli_patterns

      - name: Property testing
        run: cargo test --test property_tests

      - name: Mutation testing
        run: |
          cargo install cargo-mutants
          cargo mutants --file crates/depyler/src/commands/compile.rs \
            --baseline skip \
            --timeout 300

      - name: Benchmark regression check
        run: cargo bench --bench compile_benchmarks -- --test

      - name: Coverage report
        run: |
          cargo install cargo-llvm-cov
          cargo llvm-cov --package depyler --html
          cargo llvm-cov --package depyler --fail-under-lines 85

      - name: PMAT Quality Analysis
        run: |
          pip install pmat
          pmat quality-gate --fail-on-violation
```

---

## Implementation Phases

### Phase 1: Core Compile Command (Week 1)

**Goal**: Basic `depyler compile` works for simple scripts

**Tasks**:
1. ✅ Add `compile` subcommand to CLI
2. ✅ Implement basic transpile → cargo project → build flow
3. ✅ Support `--profile release` only
4. ✅ Unit tests for all components
5. ✅ Integration test: hello world

**Acceptance Criteria**:
- `depyler compile hello.py` produces working binary
- Binary runs and prints correct output
- All unit tests pass
- Coverage ≥85%

**TDD Workflow**:
```bash
# RED: Write failing test
cargo test test_compile_hello_world
# Status: FAIL

# GREEN: Implement minimal code
# ... implement compile command ...
cargo test test_compile_hello_world
# Status: PASS

# REFACTOR: Meet quality standards
pmat quality-gate --fail-on-violation
cargo llvm-cov --fail-under-lines 85
cargo mutants --file crates/depyler/src/commands/compile.rs
```

### Phase 2: ArgParse Support (Week 2)

**Goal**: Full argparse CLI compilation support

**Tasks**:
1. ✅ Test argparse pattern detection
2. ✅ Enhance clap generation for complex patterns
3. ✅ Support all action types (store_true, append, etc.)
4. ✅ Integration tests for CLI tools
5. ✅ Property tests for argument parsing

**Acceptance Criteria**:
- Comprehensive CLI example compiles and runs
- All 13 argparse patterns work (from matrix testing)
- Generated binary matches Python behavior
- Coverage ≥85%

### Phase 3: Profile Management (Week 3)

**Goal**: Multiple optimization profiles with auto-detection

**Tasks**:
1. ✅ Implement workload detector
2. ✅ Add perf-ultra, min-size profiles
3. ✅ Support --profile flag
4. ✅ Auto-select based on workload type
5. ✅ Benchmark each profile

**Acceptance Criteria**:
- All 4 profiles work (debug, release, perf-ultra, min-size)
- Auto-selection chooses optimal profile
- Benchmarks show expected speedup ranges
- Coverage ≥85%

### Phase 4: PGO Support (Week 4)

**Goal**: Profile-guided optimization for maximum performance

**Tasks**:
1. ✅ Implement PGO runner
2. ✅ Support --pgo flag
3. ✅ Generate training workload
4. ✅ Two-phase compilation
5. ✅ Benchmark PGO gains

**Acceptance Criteria**:
- `depyler compile --pgo` works
- PGO provides 1.5-2x additional speedup
- Training workload covers representative inputs
- Coverage ≥85%

### Phase 5: Verification & Benchmarking (Week 5)

**Goal**: Automated correctness checking and performance measurement

**Tasks**:
1. ✅ Implement verification runner
2. ✅ Support --verify flag
3. ✅ Implement benchmark runner
4. ✅ Support --benchmark flag
5. ✅ Statistical analysis

**Acceptance Criteria**:
- `--verify` catches output mismatches
- `--benchmark` produces detailed report
- Statistical validation (ANOVA, effect sizes)
- Coverage ≥85%

### Phase 6: Matrix Testing & Polish (Week 6)

**Goal**: Comprehensive CLI pattern support and production readiness

**Tasks**:
1. ✅ Test 20+ CLI patterns (argparse, click, typer, fire)
2. ✅ Property testing with fuzzing
3. ✅ Mutation testing ≥80%
4. ✅ Documentation and examples
5. ✅ Performance regression tests

**Acceptance Criteria**:
- ≥95% CLI patterns compile successfully
- All quality gates pass
- Mutation kill rate ≥80%
- Comprehensive documentation

---

## Reference Implementations

### Similar Tools

#### 1. Deno Compile

```bash
# Deno's approach
deno compile --output my_app main.ts

# Produces single binary
./my_app
```

**Key Features**:
- Single command
- Embeds dependencies
- Cross-compilation support
- Optimized by default

#### 2. UV Compile (Hypothetical)

```bash
# UV's potential approach
uv compile script.py

# Fast Rust-based compilation
./script
```

**Key Features**:
- Fast compilation (Rust-based tooling)
- Automatic dependency resolution
- Smart caching

#### 3. Ruchy Compile

Reference: https://github.com/paiml/ruchy

```bash
# Ruchy's approach
ruchy compile cli.py

# Produces optimized Rust binary
./cli --help
```

**Key Features**:
- Python → Rust transpilation
- ArgParse → Clap conversion
- Lambda/Docker deployment
- Performance benchmarking

### Depyler Compile (Our Implementation)

```bash
# Depyler's approach - combines best of all
depyler compile \
    --profile perf-ultra \
    --pgo \
    --verify \
    --benchmark \
    wordcount.py

# Produces:
# ✅ Optimized binary (25-50x faster)
# ✅ Correctness verification
# ✅ Performance report
# ✅ Ready for deployment
```

**Unique Features**:
- **Intelligent Profiles**: Auto-detect workload type
- **Scientific Validation**: Statistical benchmarking
- **EXTREME TDD**: ≥85% coverage, ≥80% mutation kill rate
- **Quality Enforcement**: PMAT gates, TDG ≤2.0
- **ArgParse Excellence**: Comprehensive CLI support

---

## Success Metrics

### Quantitative Metrics

| Metric | Target | Measured | Status |
|--------|--------|----------|--------|
| CLI Success Rate | ≥95% | TBD | 🟡 |
| Mean Speedup (release) | 10-20x | TBD | 🟡 |
| Mean Speedup (perf-ultra) | 25-50x | TBD | 🟡 |
| Binary Size (min-size) | <500 KB | TBD | 🟡 |
| Compilation Time | <60s (typical) | TBD | 🟡 |
| Test Coverage | ≥85% | TBD | 🟡 |
| Mutation Kill Rate | ≥80% | TBD | 🟡 |
| TDG Score | ≤2.0 (A-) | TBD | 🟡 |
| User Satisfaction | ≥4.5/5 | TBD | 🟡 |

### Qualitative Goals

- ✅ "Just works" experience for Python developers
- ✅ Clear, actionable error messages
- ✅ Production-ready binaries
- ✅ Comprehensive documentation
- ✅ Active community support

---

## Appendix

### A. Cargo Profile Configurations

Complete profile definitions for Cargo.toml:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = false
panic = "unwind"

[profile.perf-ultra]
inherits = "release"
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[profile.min-size]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[profile.debug]
opt-level = 0
debug = true
```

### B. RUSTFLAGS for Target-Specific Optimization

```bash
# Native CPU optimization
export RUSTFLAGS="-C target-cpu=native"

# PGO instrumentation
export RUSTFLAGS="-C profile-generate=/tmp/pgo-data"

# PGO optimization
export RUSTFLAGS="-C profile-use=/tmp/pgo-data/merged.profdata -C llvm-args=-pgo-warn-missing-function"
```

### C. Test Fixtures Directory Structure

```
tests/
├── fixtures/
│   ├── argparse/
│   │   ├── basic.py
│   │   ├── subcommands.py
│   │   ├── file_io.py
│   │   └── complex.py
│   ├── algorithms/
│   │   ├── fibonacci.py
│   │   ├── quicksort.py
│   │   ├── primes.py
│   │   └── dijkstra.py
│   ├── generators/
│   │   ├── pipeline.py
│   │   ├── lazy_eval.py
│   │   └── streaming.py
│   └── real_world/
│       ├── wordcount.py
│       ├── json_parser.py
│       ├── log_analyzer.py
│       └── data_processor.py
├── integration/
│   ├── compile_tests.rs
│   ├── profile_tests.rs
│   └── verification_tests.rs
├── property/
│   ├── fuzz_compile.rs
│   └── output_equivalence.rs
└── benchmarks/
    ├── workloads.rs
    └── regression.rs
```

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2025-11-12 | Depyler Team | Initial specification |

---

**END OF SPECIFICATION**
