# Depyler CLI Reference

Complete command-line interface documentation for the Depyler Python-to-Rust transpiler.

## Installation

```bash
# From source (recommended for development)
git clone https://github.com/paiml/depyler
cd depyler
cargo install --path crates/depyler

# Verify installation
depyler --version
```

## Global Options

```bash
depyler [OPTIONS] <COMMAND>

Options:
  -v, --verbose          Enable verbose output
  -q, --quiet           Suppress non-error output
  -h, --help            Print help information
  -V, --version         Print version information
```

## Commands

### `transpile` - Convert Python to Rust

Convert Python source files to idiomatic Rust code with optional verification.

```bash
depyler transpile [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file (.py)

Options:
  -o, --output <FILE>   Output Rust file (default: INPUT.rs)
  --verify              Enable verification during transpilation
  --verify-level <LEVEL>
                        Verification level [default: basic]
                        [possible values: none, basic, strict, paranoid]
  --no-format          Don't format generated Rust code
  --emit-hir           Also emit HIR intermediate representation
  --target <TARGET>    Target Rust edition [default: 2021]
  -f, --force          Overwrite existing output files
```

#### Examples

```bash
# Basic transpilation
depyler transpile examples/showcase/binary_search.py

# Transpile with strict verification
depyler transpile input.py --verify-level strict -o output.rs

# Transpile multiple files
depyler transpile examples/showcase/*.py --verify

# Generate HIR for debugging
depyler transpile input.py --emit-hir --output debug/
```

#### Verification Levels

- **none**: No verification, fastest transpilation
- **basic**: Type safety and basic property checks
- **strict**: Full property verification with contracts
- **paranoid**: Formal verification with proof generation

### `verify` - Verify Python Code Properties

Analyze Python code for transpilation compatibility and safety properties.

```bash
depyler verify [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  --property-tests      Generate property-based tests
  --contracts          Verify pre/post conditions
  --quickcheck         Enable QuickCheck integration
  --termination        Verify loop termination
  --bounds-check       Verify array bounds safety
  --memory-safety      Verify memory usage patterns
  --report <FORMAT>    Output format [default: text]
                       [possible values: text, json, markdown]
```

#### Examples

```bash
# Basic verification
depyler verify examples/showcase/

# Full verification suite
depyler verify input.py --property-tests --contracts --quickcheck

# Generate verification report
depyler verify project/ --report json > verification_report.json
```

### `analyze` - Code Analysis and Metrics

Analyze Python code for complexity, energy efficiency, and performance characteristics.

```bash
depyler analyze [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  --complexity         Calculate complexity metrics
  --energy             Estimate energy consumption
  --performance        Analyze performance characteristics
  --types              Show inferred type information
  --dependencies       Analyze dependency structure
  --format <FORMAT>    Output format [default: text]
                       [possible values: text, json, csv]
```

#### Examples

```bash
# Complexity analysis
depyler analyze src/ --complexity

# Energy efficiency analysis
depyler analyze compute_heavy.py --energy --performance

# Type analysis for debugging
depyler analyze input.py --types --format json
```

### `check` - Compatibility Check

Check Python code for transpilation compatibility without full conversion.

```bash
depyler check [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  --strict             Use strict compatibility rules
  --show-unsupported   List unsupported constructs
  --suggest-fixes      Suggest code modifications
  --report <FORMAT>    Output format [default: text]
```

#### Examples

```bash
# Quick compatibility check
depyler check legacy_code.py

# Detailed compatibility report
depyler check project/ --show-unsupported --suggest-fixes
```

### `init` - Initialize New Project

Create a new Depyler-compatible Python project with templates.

```bash
depyler init [OPTIONS] <NAME>

Arguments:
  <NAME>                Project name

Options:
  --template <TEMPLATE> Project template [default: basic]
                        [possible values: basic, scientific, web, cli]
  --rust-target         Include Rust target configuration
  --git                 Initialize git repository
  --license <LICENSE>   Add license file [default: MIT]
```

#### Examples

```bash
# Create basic project
depyler init my-project

# Create scientific computing project
depyler init simulation --template scientific --rust-target

# Create CLI tool project
depyler init cli-tool --template cli --git --license Apache-2.0
```

## Configuration

### Project Configuration File

Create a `depyler.toml` file in your project root:

```toml
[project]
name = \"my-project\"
version = \"0.1.0\"
edition = \"2021\"

[transpilation]
verify_level = \"strict\"
target_edition = \"2021\"
format_output = true
emit_hir = false

[verification]
property_tests = true
contracts = true
quickcheck = true
termination_analysis = true

[analysis]
complexity_threshold = 10
energy_analysis = true
performance_profiling = false

[output]
preserve_comments = true
generate_docs = true
include_source_map = true
```

### Environment Variables

```bash
# Enable debug logging
export DEPYLER_LOG=debug

# Set custom verification timeout
export DEPYLER_VERIFY_TIMEOUT=30

# Configure parallel processing
export DEPYLER_THREADS=8

# Set cache directory
export DEPYLER_CACHE_DIR=\"~/.cache/depyler\"
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0    | Success |
| 1    | General error |
| 2    | Parse error |
| 3    | Type error |
| 4    | Verification failure |
| 5    | Unsupported construct |
| 6    | I/O error |
| 7    | Configuration error |

## Error Handling

### Common Errors and Solutions

#### Parse Errors
```bash
Error: Failed to parse Python source
  --> input.py:15:23
   |
15 |     result = [x for x in range(10) if x % 2 == 0
   |                                               ^ Expected ']'

Solution: Fix Python syntax errors before transpilation
```

#### Type Inference Errors
```bash
Error: Cannot infer type for variable 'data'
  --> input.py:8:5
   |
8  |     data = get_data()
   |     ^^^^ Type annotation required

Solution: Add type hints or use --verify-level none
```

#### Unsupported Construct Errors
```bash
Error: Unsupported construct: async function
  --> input.py:12:1
   |
12 | async def fetch_data():
   | ^^^^^ async/await not supported in current version

Solution: Use synchronous alternatives or wait for async support
```

## Performance Tips

### Optimizing Transpilation Speed

1. **Use appropriate verification levels**:
   ```bash
   # Fast development iteration
   depyler transpile --verify-level none
   
   # Production builds
   depyler transpile --verify-level strict
   ```

2. **Parallel processing**:
   ```bash
   # Process multiple files in parallel
   export DEPYLER_THREADS=$(nproc)
   depyler transpile src/*.py
   ```

3. **Caching**:
   ```bash
   # Enable persistent caching
   export DEPYLER_CACHE_DIR=\"~/.cache/depyler\"
   ```

### Memory Usage Optimization

```bash
# For large codebases
export DEPYLER_MAX_MEMORY=8G
export DEPYLER_STREAMING=true

# Process files individually for very large projects
find src -name \"*.py\" -exec depyler transpile {} \\;
```

## Integration Examples

### Makefile Integration

```makefile
# Makefile
.PHONY: transpile verify clean

transpile:
\t@echo \"Transpiling Python to Rust...\"
\tdepyler transpile src/ --verify-level strict

verify:
\t@echo \"Verifying Python code...\"
\tdepyler verify src/ --property-tests --contracts

check:
\t@echo \"Checking compatibility...\"
\tdepyler check src/ --strict --show-unsupported

clean:
\t@echo \"Cleaning generated files...\"
\trm -rf target/ *.rs

# Continuous integration
ci: check verify transpile
\tcargo test --all
\tcargo clippy -- -D warnings
```

### GitHub Actions Integration

```yaml
# .github/workflows/depyler.yml
name: Depyler CI

on: [push, pull_request]

jobs:
  transpile:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Depyler
      run: |
        curl -sSf https://install.depyler.rs | sh
        echo \"$HOME/.depyler/bin\" >> $GITHUB_PATH
    
    - name: Check Python compatibility
      run: depyler check src/ --strict
    
    - name: Verify Python properties
      run: depyler verify src/ --property-tests --contracts
    
    - name: Transpile to Rust
      run: depyler transpile src/ --verify-level strict
    
    - name: Test generated Rust
      run: cargo test --all
```

### VS Code Integration

Create `.vscode/tasks.json`:

```json
{
    \"version\": \"2.0.0\",
    \"tasks\": [
        {
            \"label\": \"Depyler: Transpile\",
            \"type\": \"shell\",
            \"command\": \"depyler\",
            \"args\": [\"transpile\", \"${file}\", \"--verify\"],
            \"group\": \"build\",
            \"presentation\": {
                \"echo\": true,
                \"reveal\": \"always\",
                \"panel\": \"new\"
            }
        },
        {
            \"label\": \"Depyler: Verify\",
            \"type\": \"shell\",
            \"command\": \"depyler\",
            \"args\": [\"verify\", \"${file}\", \"--property-tests\"],
            \"group\": \"test\"
        }
    ]
}
```

## Troubleshooting

### Common Issues

1. **Installation Problems**:
   ```bash
   # Ensure Rust is installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Update PATH
   source ~/.cargo/env
   ```

2. **Memory Issues**:
   ```bash
   # Increase memory limit
   export DEPYLER_MAX_MEMORY=16G
   
   # Use streaming mode for large files
   export DEPYLER_STREAMING=true
   ```

3. **Verification Timeout**:
   ```bash
   # Increase timeout for complex verification
   export DEPYLER_VERIFY_TIMEOUT=300
   
   # Or disable problematic checks
   depyler verify --no-termination-analysis
   ```

### Debug Mode

```bash
# Enable comprehensive debugging
export DEPYLER_LOG=trace
export RUST_BACKTRACE=1

# Run with debug output
depyler transpile input.py --verbose 2> debug.log
```

### Reporting Issues

When reporting issues, include:

1. **Version information**:
   ```bash
   depyler --version
   rustc --version
   python --version
   ```

2. **Minimal reproduction case**:
   ```python
   # minimal_example.py
   def problematic_function():
       # Code that causes issues
       pass
   ```

3. **Full command and output**:
   ```bash
   depyler transpile minimal_example.py --verbose 2>&1 | tee issue_report.txt
   ```

## See Also

- [User Guide](user-guide.md) - Comprehensive usage documentation
- [Project Overview](project-overview.md) - Architecture and design
- [Energy Efficiency](energy-efficiency.md) - Sustainability focus
- [V0 Specification](v0-spec.md) - Technical specification

---

*Generated: 2025-01-06*  
*Version: 0.1.0*  
*For issues and contributions: https://github.com/paiml/depyler*