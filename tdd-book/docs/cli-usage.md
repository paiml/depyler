# CLI Usage Guide

Complete guide to using the Depyler command-line interface.

## Installation

Install Depyler from crates.io:

```bash
cargo install depyler
```

Verify installation:

```bash
depyler --version
# Output: depyler 3.20.0
```

## Quick Start

The fastest way to use Depyler is with the `compile` command:

```bash
# Compile Python to standalone binary
depyler compile script.py

# Run the compiled binary
./script
```

## Commands Overview

| Command | Purpose | Example |
|---------|---------|---------|
| `compile` | Python → Native binary | `depyler compile script.py` |
| `transpile` | Python → Rust source | `depyler transpile script.py` |
| `analyze` | Migration complexity | `depyler analyze script.py` |
| `check` | Type safety validation | `depyler check script.py` |
| `interactive` | REPL mode | `depyler interactive` |

---

## `depyler compile` - Single-Shot Compilation

**NEW in v3.20.0** - Compile Python scripts to standalone native executables.

### Basic Usage

```bash
# Compile Python script to binary
depyler compile hello.py

# Output: ./hello (or hello.exe on Windows)
```

### Options

**`-o, --output <PATH>`** - Custom output path
```bash
depyler compile script.py -o my_app
# Output: ./my_app
```

**`--profile <PROFILE>`** - Build profile (default: release)
```bash
# Release build (optimized, slower compile)
depyler compile script.py --profile release

# Debug build (faster compile, less optimization)
depyler compile script.py --profile debug
```

### Examples

**Hello World:**
```python
# hello.py
def main():
    print("Hello, World!")

if __name__ == '__main__':
    main()
```

```bash
depyler compile hello.py
./hello
# Output: Hello, World!
```

**With Command-Line Arguments:**
```python
# greet.py
import sys

def main():
    if len(sys.argv) > 1:
        print(f"Hello, {sys.argv[1]}!")
    else:
        print("Hello, World!")

if __name__ == '__main__':
    main()
```

```bash
depyler compile greet.py
./greet Alice
# Output: Hello, Alice!
```

**Optimized Release Build:**
```python
# fibonacci.py
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def main():
    result = fibonacci(10)
    print(f"fib(10) = {result}")

if __name__ == '__main__':
    main()
```

```bash
depyler compile fibonacci.py --profile release -o fib
./fib
# Output: fib(10) = 55
```

### How It Works

The `compile` command uses a 4-phase pipeline:

1. **Transpile**: Python → Rust using Depyler's transpiler
2. **Generate**: Creates temporary Cargo project structure
3. **Build**: Compiles with `cargo build --release`
4. **Finalize**: Copies binary to desired location with executable permissions

Visual progress bar shows each phase:
```
[00:00:05] ████████████████████████████████ 4/4 ✅ Compilation complete!
```

### Cross-Platform Support

- **Linux**: Produces ELF executable
- **macOS**: Produces Mach-O executable
- **Windows**: Produces PE executable (.exe)

Binary permissions set automatically (`chmod 755` on Unix).

---

## `depyler transpile` - Source-to-Source Translation

Transpile Python to Rust source code.

### Basic Usage

```bash
# Transpile Python file to Rust
depyler transpile example.py
# Output: example.rs
```

### Options

**`--verify`** - Run semantic verification tests
```bash
depyler transpile example.py --verify
```

**`--gen-tests`** - Generate property tests
```bash
depyler transpile example.py --gen-tests
```

**`--debug`** - Enable debug output
```bash
depyler transpile example.py --debug
```

**`--source-map`** - Generate source map
```bash
depyler transpile example.py --source-map
```

### Example

**Input (`fibonacci.py`):**
```python
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

**Command:**
```bash
depyler transpile fibonacci.py
```

**Output (`fibonacci.rs`):**
```rust
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

---

## `depyler analyze` - Migration Complexity Analysis

Analyze Python code to estimate transpilation complexity.

### Basic Usage

```bash
depyler analyze example.py
```

### Options

**`--format <FORMAT>`** - Output format (text, json, html)
```bash
depyler analyze example.py --format json
```

### Example Output

```
📊 Migration Analysis for example.py
=====================================

Complexity Score: 3.2/10 (Low)

✅ Supported Features:
  - Type annotations: 100%
  - Control flow: Simple
  - Functions: 5
  - Classes: 2

⚠️  Challenges:
  - Dynamic typing in 3 locations
  - Unpacking in 1 location

💡 Recommendations:
  1. Add type hints to variables
  2. Replace dynamic unpacking with explicit access

Estimated Effort: 2-4 hours
Success Probability: 95%
```

---

## `depyler check` - Type Safety Validation

Check Python code for type safety issues before transpilation.

### Basic Usage

```bash
depyler check example.py
```

### Example Output

```
✅ Type safety check passed

Summary:
  - Functions checked: 8
  - Type errors: 0
  - Warnings: 2

⚠️  Warnings:
  - line 42: Implicit Any type for variable 'data'
  - line 57: Missing return type annotation
```

---

## `depyler interactive` - REPL Mode

Interactive Python-to-Rust transpilation.

### Basic Usage

```bash
depyler interactive
```

### Options

**`--annotate`** - Show type annotations
```bash
depyler interactive --annotate
```

### Example Session

```python
>>> def add(a: int, b: int) -> int:
...     return a + b
...
Rust:
fn add(a: i32, b: i32) -> i32 {
    a + b
}

>>> [x * 2 for x in range(5)]
Rust:
(0..5).map(|x| x * 2).collect::<Vec<_>>()
```

---

## Quality Check Commands

### `depyler quality-check` - Quality Gate Validation

Run quality gates on Python code before transpilation.

```bash
depyler quality-check example.py \
  --enforce \
  --min-tdg 80 \
  --max-tdg 2.0 \
  --max-complexity 10 \
  --min-coverage 85
```

---

## Debug Commands

### `depyler debug` - Debug Information

Generate debugging information for transpilation issues.

```bash
# Show debug tips
depyler debug --tips

# Generate debug script
depyler debug --gen-script example.py

# Launch debugger
depyler debug --debugger example.py --source python
```

---

## Advanced Commands

### `depyler inspect` - Code Inspection

Inspect Python code structure and representations.

```bash
# Show HIR representation
depyler inspect example.py --repr hir

# JSON output
depyler inspect example.py --format json
```

### `depyler docs` - Generate Documentation

Generate Rust documentation from Python code.

```bash
depyler docs example.py --output docs/

# Include migration notes
depyler docs example.py \
  --migration-notes \
  --performance-notes \
  --examples
```

### `depyler profile` - Performance Profiling

Profile transpiled Rust code performance.

```bash
depyler profile example.py \
  --count-instructions \
  --track-allocations \
  --detect-hot-paths \
  --flamegraph
```

---

## Lambda-Specific Commands

### `depyler lambda` - AWS Lambda Integration

Convert Python Lambda functions to optimized Rust.

```bash
# Analyze Lambda function
depyler lambda analyze handler.py

# Convert to Rust Lambda
depyler lambda convert handler.py --optimize --tests

# Test Lambda
depyler lambda test handler.py --event event.json

# Build for Lambda
depyler lambda build handler.py --arch arm64

# Deploy Lambda
depyler lambda deploy handler.py \
  --region us-east-1 \
  --function-name my-function \
  --role arn:aws:iam::123456789:role/lambda-role
```

---

## Language Server Protocol

### `depyler lsp` - LSP Server

Start Depyler LSP server for IDE integration.

```bash
# Start LSP server
depyler lsp --port 9257 --verbose
```

Use with VS Code, Vim, Emacs, or any LSP-compatible editor.

---

## Common Workflows

### Workflow 1: Quick Compilation

```bash
# Single-shot compile
depyler compile script.py && ./script
```

### Workflow 2: Verify Before Compile

```bash
# Check → Analyze → Compile
depyler check script.py && \
depyler analyze script.py && \
depyler compile script.py
```

### Workflow 3: Transpile with Verification

```bash
# Transpile with tests and verification
depyler transpile script.py \
  --verify \
  --gen-tests \
  --source-map

# Compile the generated Rust
rustc script.rs -o script
```

### Workflow 4: Production Lambda Deployment

```bash
# Full Lambda workflow
depyler lambda analyze handler.py
depyler lambda convert handler.py --optimize --tests --deploy
depyler lambda test handler.py --event event.json --benchmark
depyler lambda build handler.py --arch arm64 --optimize-cold-start
depyler lambda deploy handler.py \
  --region us-east-1 \
  --function-name production-handler \
  --role arn:aws:iam::123456789:role/lambda-role
```

---

## Troubleshooting

### Common Issues

**Issue: `command not found: depyler`**

Solution:
```bash
# Ensure cargo bin is in PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Reinstall if needed
cargo install --force depyler
```

**Issue: Compilation fails with type errors**

Solution:
```bash
# Check types first
depyler check script.py

# Add missing type annotations
# Then retry
depyler compile script.py
```

**Issue: Binary doesn't execute**

Solution (Unix):
```bash
# Ensure executable permissions
chmod +x ./script
./script
```

Solution (Windows):
```powershell
# Run with .exe extension
.\script.exe
```

**Issue: `cargo` not found during compile**

Solution:
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload PATH
source $HOME/.cargo/env
```

**Issue: Slow compilation**

Solution:
```bash
# Use debug profile for faster builds
depyler compile script.py --profile debug

# Or install faster linker (mold/lld)
sudo apt install mold  # Linux
cargo install lld      # Cross-platform
```

---

## Environment Variables

- `DEPYLER_LOG=debug` - Enable debug logging
- `DEPYLER_CACHE_DIR` - Custom cache directory
- `DEPYLER_NO_COLOR` - Disable colored output
- `RUST_BACKTRACE=1` - Show Rust backtraces on panic

---

## Getting Help

```bash
# General help
depyler --help

# Command-specific help
depyler compile --help
depyler transpile --help
depyler analyze --help

# Version information
depyler --version
```

---

## See Also

- [Python Standard Library Modules](./modules/) - Supported stdlib modules
- [Contributing Guide](./contributing.md) - How to contribute
- [Quality Standards](./quality.md) - Code quality requirements
- [GitHub Repository](https://github.com/paiml/depyler) - Source code
- [Crates.io](https://crates.io/crates/depyler) - Package page
