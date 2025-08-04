# Depyler User Guide

## Getting Started

### Prerequisites

Before using Depyler, ensure you have:

- **Rust 1.70+** with Cargo toolchain
- **Python 3.8+** for source code analysis
- **Git** for version control and dependency management

### Installation

**Note**: Depyler is currently in active development. Until our first stable
release, installation requires building from source.

#### Building from Source

```bash
# Clone the repository
git clone https://github.com/paiml/depyler.git
cd depyler

# Build and install
make setup      # Install dependencies
make build      # Build the project
make install    # Install to ~/.cargo/bin

# Verify installation
depyler --version
```

#### Future Installation (Post-Release)

Once released, Depyler will be available through multiple channels:

```bash
# Via our installation script (coming soon)
curl -sSfL https://github.com/paiml/depyler/releases/latest/download/install.sh | sh

# Via Cargo
cargo install depyler

# Via package managers
brew install depyler                    # macOS/Linux
scoop install depyler                   # Windows
apt install depyler                     # Ubuntu/Debian
```

## Basic Usage

### Your First Transpilation

Let's start with a simple Python function:

**example.py**:

```python
def add_numbers(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b

def main():
    result = add_numbers(5, 3)
    print(f"Result: {result}")

if __name__ == "__main__":
    main()
```

Transpile to Rust:

```bash
depyler transpile example.py --output example.rs
```

**Generated example.rs**:

```rust
/// Add two numbers together.
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

pub fn main() {
    let result = add_numbers(5, 3);
    println!("Result: {}", result);
}
```

### Command-Line Interface

#### Basic Commands

```bash
# Transpile a single file
depyler transpile <input.py> --output <output.rs>

# Transpile an entire project
depyler transpile --project <python_dir> --output <rust_dir>

# Analyze Python code without transpiling
depyler analyze <input.py>

# Validate generated Rust code
depyler validate <output.rs>
```

#### Common Options

```bash
# Optimization levels
--optimize              # Enable all optimizations
--optimize-size         # Optimize for binary size
--optimize-speed        # Optimize for execution speed

# Target selection
--target <triple>       # Rust target triple (e.g., x86_64-unknown-linux-gnu)

# Type inference
--strict-types          # Require type annotations
--infer-types           # Enable aggressive type inference

# Output control
--format                # Format generated Rust code
--comments              # Preserve Python comments
--docstrings            # Convert docstrings to Rust docs
```

## Developer Tools Integration

### Language Server Protocol (LSP)

Depyler provides full IDE integration through LSP:

```bash
# Start the language server
depyler lsp

# Or configure your IDE to use it automatically
# VSCode: Install the Depyler extension
# Neovim: Add to your LSP config
# IntelliJ: Use the Depyler plugin
```

Features:

- **Symbol Navigation**: Go-to-definition, find references
- **Hover Information**: Type details and documentation on hover
- **Code Completions**: Context-aware completions
- **Real-time Diagnostics**: Instant error detection
- **Code Actions**: Quick fixes and refactoring suggestions

### Debugging Support

Depyler generates debug information for easier troubleshooting:

```bash
# Transpile with debug information
depyler transpile main.py --debug --source-map

# View debugging tips
depyler debug --tips

# Generate debugger scripts
depyler debug --generate-scripts main.rs
```

Debug levels:

- **None**: No debug information (production)
- **Basic**: Line number mapping from Python to Rust
- **Full**: Complete variable state tracking

### Performance Profiling

Analyze performance characteristics of your Python code:

```bash
# Basic profiling
depyler profile main.py

# Generate flamegraph
depyler profile main.py --flamegraph

# Compare Python vs Rust performance
depyler profile main.py --compare

# Export profiling data
depyler profile main.py --export profile.json
```

Profiling features:

- **Hot Path Detection**: Identify performance bottlenecks
- **Memory Analysis**: Track allocations and usage
- **Execution Time**: Function-level timing
- **Performance Predictions**: Estimate Rust performance gains

### Documentation Generation

Automatically generate documentation from Python code:

```bash
# Generate API documentation
depyler docs main.py --output ./docs

# Generate full documentation suite
depyler docs main.py --level full --format html

# Include usage examples
depyler docs main.py --include-examples
```

Documentation options:

- **API Reference**: Function signatures and docstrings
- **Usage Guide**: How to use the transpiled code
- **Migration Notes**: Python to Rust transition guide
- **Module Overview**: Dependency analysis and structure

## Language Support Matrix

### Supported Python Features ✅

| Feature           | Support Level | Example                    |
| ----------------- | ------------- | -------------------------- |
| **Functions**     | Full          | `def func(x: int) -> str:` |
| **Basic Types**   | Full          | `int, float, str, bool`    |
| **Control Flow**  | Full          | `if/else, for, while`      |
| **Lists**         | Full          | `[1, 2, 3]`                |
| **Tuples**        | Full          | `(1, "hello", True)`       |
| **Dictionaries**  | Basic         | `{"key": "value"}`         |
| **Type Hints**    | Full          | `List[int], Optional[str]` |
| **Basic Classes** | Partial       | Simple classes only        |

### Planned Features 🚧

| Feature               | Timeline | Notes                            |
| --------------------- | -------- | -------------------------------- |
| **Async/Await**       | Q2 2025  | Tokio integration                |
| **Class Inheritance** | Q2 2025  | Multiple inheritance limitations |
| **Decorators**        | Q3 2025  | Common decorators first          |
| **Context Managers**  | Q3 2025  | `with` statements                |
| **Generators**        | Q4 2025  | Iterator trait mapping           |

### Unsupported Features ❌

- Dynamic typing without hints
- `eval()` and `exec()`
- Monkey patching
- Multiple inheritance (complex cases)
- Python's introspection features

## Type System Mapping

### Primitive Types

| Python  | Rust          | Notes                    |
| ------- | ------------- | ------------------------ |
| `int`   | `i32` / `i64` | Configurable width       |
| `float` | `f64`         | Double precision default |
| `str`   | `String`      | Owned strings (V1)       |
| `bool`  | `bool`        | Direct mapping           |
| `None`  | `()`          | Unit type                |

### Collection Types

| Python          | Rust            | Notes             |
| --------------- | --------------- | ----------------- |
| `List[T]`       | `Vec<T>`        | Dynamic arrays    |
| `Dict[K, V]`    | `HashMap<K, V>` | Hash maps         |
| `Tuple[T1, T2]` | `(T1, T2)`      | Fixed-size tuples |
| `Set[T]`        | `HashSet<T>`    | Hash sets         |

### Advanced Types

| Python             | Rust         | Notes             |
| ------------------ | ------------ | ----------------- |
| `Optional[T]`      | `Option<T>`  | Null safety       |
| `Union[T1, T2]`    | `enum`       | Tagged unions     |
| `Callable[[T], R]` | `fn(T) -> R` | Function pointers |

## Configuration

### Project Configuration

Create a `depyler.toml` file in your project root:

```toml
[project]
name = "my-python-project"
version = "0.1.0"
python_version = "3.9"

[transpilation]
target = "x86_64-unknown-linux-gnu"
optimize = true
strict_types = false

[type_mapping]
int_width = "i32"          # or "i64", "isize"
string_strategy = "owned"   # or "borrowed", "cow"

[output]
format_code = true
preserve_comments = true
generate_docs = true
```

### Per-File Configuration

Use comment directives in Python files:

```python
# depyler: optimize-speed
# depyler: target=wasm32-unknown-unknown
# depyler: strict-types

def performance_critical_function(data: List[int]) -> int:
    return sum(data)
```

## Migration Strategies

### Incremental Migration

**Strategy 1: Bottom-Up (Recommended)**

1. Start with utility functions and data structures
2. Move to computational kernels and algorithms
3. Gradually replace higher-level application logic
4. Maintain Python interfaces during transition

**Strategy 2: Hot Path Optimization**

1. Profile your application to identify bottlenecks
2. Transpile performance-critical functions first
3. Use Python/Rust interop during transition
4. Expand to related components

**Strategy 3: Service-by-Service**

1. Identify self-contained microservices
2. Transpile entire services to Rust
3. Update interfaces and deployment
4. Repeat for next service

### Large Codebase Approach

For codebases >100k lines of Python:

```bash
# 1. Analyze the entire codebase
depyler analyze --project . --output analysis.json

# 2. Generate migration plan
depyler plan --analysis analysis.json --strategy incremental

# 3. Start with dependency-free modules
depyler transpile --batch --dependencies=none

# 4. Progress through dependency layers
depyler transpile --batch --layer=1
depyler transpile --batch --layer=2
```

## Testing Strategy

### Validation Workflow

```bash
# 1. Transpile with validation
depyler transpile main.py --output main.rs --validate

# 2. Run Rust compilation check
cargo check

# 3. Compare Python and Rust behavior
depyler test-equivalence main.py main.rs

# 4. Performance benchmark
depyler benchmark main.py main.rs
```

### Integration Testing

```python
# test_transpilation.py
import subprocess
import pytest

def test_transpiled_output():
    # Run Python version
    py_result = subprocess.run(['python', 'main.py'], capture_output=True)
    
    # Compile and run Rust version
    subprocess.run(['cargo', 'build', '--release'])
    rs_result = subprocess.run(['./target/release/main'], capture_output=True)
    
    # Compare outputs
    assert py_result.stdout == rs_result.stdout
```

## Performance Optimization

### Optimization Flags

```bash
# Size optimization
depyler transpile --optimize-size main.py

# Speed optimization  
depyler transpile --optimize-speed main.py

# Custom optimization
depyler transpile --llvm-args="-O3 -march=native" main.py
```

### Manual Optimization Hints

```python
# depyler: inline
def small_function(x: int) -> int:
    return x * 2

# depyler: no-bounds-check
def unsafe_array_access(arr: List[int], idx: int) -> int:
    return arr[idx]  # Generates unchecked access

# depyler: simd
def vector_operation(data: List[float]) -> List[float]:
    return [x * 2.0 for x in data]  # Uses SIMD instructions
```

## Troubleshooting

### Common Issues

**Issue**: Type inference failures

```
Error: Cannot infer type for variable 'x'
```

**Solution**: Add explicit type hints

```python
x: int = some_function()  # Instead of: x = some_function()
```

**Issue**: Unsupported Python feature

```
Error: Dynamic attribute access not supported
```

**Solution**: Refactor to use static attribute access or dictionaries

```python
# Instead of: getattr(obj, attr_name)
# Use: obj.known_attribute or obj_dict[attr_name]
```

**Issue**: Memory safety violation

```
Error: Cannot borrow value as mutable
```

**Solution**: Review ownership patterns, may need code restructuring

### Debug Options

```bash
# Verbose output
depyler transpile --verbose main.py

# Debug AST transformation
depyler transpile --debug-ast main.py

# Show type inference steps
depyler transpile --debug-types main.py

# Generate intermediate representations
depyler transpile --emit-hir --emit-llvm main.py
```

### Getting Help

- **Documentation**: Available in this repository's [docs directory](../docs/)
- **GitHub Issues**: Report bugs and request features
- **Discord Community**: Real-time help and discussions
- **Stack Overflow**: Tag questions with `depyler`

## Examples Gallery

### Web Server Migration

**Python (FastAPI)**:

```python
from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class Item(BaseModel):
    name: str
    price: float

@app.post("/items/")
async def create_item(item: Item):
    return {"message": f"Created {item.name}"}
```

**Rust (Axum)**:

```rust
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Item {
    name: String,
    price: f64,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn create_item(Json(item): Json<Item>) -> Json<Response> {
    Json(Response {
        message: format!("Created {}", item.name),
    })
}

pub fn app() -> Router {
    Router::new().route("/items/", post(create_item))
}
```

### Data Processing Pipeline

**Python (Pandas-style)**:

```python
def process_data(data: List[Dict[str, float]]) -> Dict[str, float]:
    totals = {}
    for record in data:
        for key, value in record.items():
            totals[key] = totals.get(key, 0.0) + value
    return totals
```

**Rust (Generated)**:

```rust
use std::collections::HashMap;

pub fn process_data(data: Vec<HashMap<String, f64>>) -> HashMap<String, f64> {
    let mut totals = HashMap::new();
    for record in data {
        for (key, value) in record {
            *totals.entry(key).or_insert(0.0) += value;
        }
    }
    totals
}
```

## Running the Interactive Playground (🧪 EXPERIMENTAL - UNSTABLE)

> ⚠️ **WARNING**: The Interactive Playground is currently **EXPERIMENTAL** and
> **UNSTABLE**. It is not recommended for production use. Features may change or
> break without notice. Use `DEPYLER_EXPERIMENTAL=true depyler playground` to
> acknowledge this status.

The Depyler Interactive Playground provides a zero-configuration, browser-based
environment for exploring Python-to-Rust transpilation. It's the fastest way to
understand Depyler's capabilities and see real-time energy efficiency
comparisons.

### Building the WASM Module

Before running the playground locally, you need to build the WebAssembly module:

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the WASM module from the repository root
cd crates/depyler-wasm
wasm-pack build --target web --out-dir ../../playground/public/wasm

# Optimize the WASM module for size
wasm-opt -Oz -o ../../playground/public/wasm/optimized.wasm \
  ../../playground/public/wasm/depyler_bg.wasm

# Verify the build (should be < 1.5MB gzipped)
gzip -9 < ../../playground/public/wasm/optimized.wasm > ../../playground/public/wasm/depyler.wasm.gz
ls -lh ../../playground/public/wasm/depyler.wasm.gz
```

### Running Locally

#### Quick Start

```bash
# Navigate to the playground directory
cd playground

# Install dependencies
npm install  # or yarn install

# Start the development server
npm run dev  # or yarn dev

# Open in browser
# Navigate to http://localhost:5173
```

#### Using Deno (Alternative)

If you prefer Deno for a more secure runtime:

```bash
# Ensure Deno is installed
curl -fsSL https://deno.land/x/install/install.sh | sh

# Run with Deno
cd playground
deno task dev

# Build for production
deno task build
deno task preview
```

### Playground Features

#### 1. **Real-Time Transpilation**

- As you type Python code, see the Rust output update instantly
- Syntax highlighting for both Python and Rust
- Error indicators with helpful suggestions

#### 2. **Intelli-Sensei Assistant**

The built-in AI assistant provides:

- **Annotation Suggestions**: Type `@` to see available Depyler annotations
- **Type Hints**: Automatic type inference with override suggestions
- **Anti-Pattern Detection**: Warnings for Python patterns that don't transpile
  well
- **Optimization Tips**: Inline hints for better performance

Example with Intelli-Sensei:

```python
# Type '@' to see annotation options
@depyler:optimize-speed
def process_data(items):  # <- Intelli-Sensei suggests: items: List[int]
    total = 0
    for item in items:
        total += item  # <- Hint: Consider using sum() for better optimization
    return total
```

#### 3. **Side-by-Side Execution**

- Run both Python and Rust versions in sandboxed environments
- Compare output, execution time, and memory usage
- See actual energy consumption estimates

#### 4. **Energy Efficiency Visualization**

- Interactive gauge showing energy savings (0-100%)
- Detailed breakdown of CPU vs memory energy usage
- Real-world equivalents (e.g., "Saves energy equivalent to 5 Google searches")
- Confidence indicators based on code complexity

#### 5. **Deep Dive Mode**

View the complete transpilation pipeline:

- **Python Source** → **HIR (High-level IR)** → **Rust Output**
- Hover over code to see mappings between stages
- Understand how Python constructs map to Rust idioms

### Example Workflows

#### Testing a Simple Function

1. Enter Python code:

```python
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

# Test the function
print(fibonacci(10))
```

2. Click "Run Comparison" to see:
   - Rust output with proper recursion
   - Execution time comparison
   - Energy savings visualization

#### Exploring Annotations

1. Use Depyler annotations for optimization:

```python
@depyler:memo
def fibonacci(n: int) -> int:
    """Memoized fibonacci for better performance"""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

2. See how annotations affect:
   - Generated Rust code (adds memoization)
   - Performance metrics
   - Energy efficiency

#### Working with Complex Types

```python
from typing import List, Dict, Optional

@depyler:strict-types
def process_records(
    records: List[Dict[str, float]], 
    threshold: Optional[float] = None
) -> Dict[str, float]:
    """Aggregate records by key with optional filtering"""
    result: Dict[str, float] = {}
    
    for record in records:
        for key, value in record.items():
            if threshold is None or value >= threshold:
                result[key] = result.get(key, 0.0) + value
    
    return result
```

### Troubleshooting Common Issues

#### 1. **WASM Module Loading Errors**

**Problem**: "Failed to load WASM module"

**Solutions**:

- Ensure WASM module is built: `ls playground/public/wasm/`
- Check browser console for CORS errors
- Verify your browser supports WebAssembly

#### 2. **Slow Initial Load**

**Problem**: First run takes several seconds

**Explanation**: The playground downloads:

- WASM transpiler (~1.5MB gzipped)
- Pyodide runtime (~15MB) for Python execution
- Rust toolchain (~21MB) for compilation

**Solutions**:

- These are cached after first download
- Use a fast internet connection for initial setup
- The loading indicator shows progress

#### 3. **Transpilation Errors**

**Problem**: "Unsupported Python feature"

**Solutions**:

- Check the [Language Support Matrix](#language-support-matrix)
- Simplify dynamic features
- Add type hints for better inference
- Use `@depyler:skip` to exclude problematic functions

#### 4. **Performance Warnings**

**Problem**: "Transpilation timeout" or slow performance

**Solutions**:

- Break large functions into smaller ones
- Avoid deeply nested structures
- Use the complexity indicator as a guide
- Enable `@depyler:optimize` annotations

### Sharing and Collaboration

#### Share Your Code

1. Click the "Share" button to generate a unique URL
2. The URL includes your code and configuration
3. Share with colleagues for review or debugging

#### Export Options

- **Download Rust**: Get the generated `.rs` file
- **Copy as Gist**: Create a GitHub Gist
- **Export Report**: PDF with code, metrics, and analysis

### Keyboard Shortcuts

| Shortcut               | Action                  |
| ---------------------- | ----------------------- |
| `Ctrl/Cmd + Enter`     | Run transpilation       |
| `Ctrl/Cmd + S`         | Save to browser storage |
| `Ctrl/Cmd + Shift + F` | Format code             |
| `Ctrl/Cmd + D`         | Toggle deep dive view   |
| `Ctrl/Cmd + /`         | Toggle comment          |
| `F1`                   | Command palette         |

### Privacy and Security

The playground runs entirely in your browser:

- **No code leaves your machine** - all processing is local
- **Sandboxed execution** - Python and Rust run in isolated environments
- **No network requests** - after initial resource loading
- **No tracking** - anonymous quality metrics only (opt-out available)

### Advanced Configuration

For power users, access advanced settings via the gear icon:

```javascript
{
  "transpilation": {
    "target": "wasm32-unknown-unknown",
    "optimizationLevel": 2,
    "enableAnnotations": true
  },
  "editor": {
    "fontSize": 14,
    "theme": "vs-dark",
    "minimap": false
  },
  "execution": {
    "pythonTimeout": 5000,
    "rustTimeout": 10000,
    "memoryLimit": 256  // MB
  }
}
```

### Playground Limitations

Current limitations in the playground environment:

1. **File System**: No access to local files
2. **Network**: No external API calls
3. **Dependencies**: Limited to standard library
4. **Execution Time**: 5s timeout for safety
5. **Memory**: 256MB limit per execution

For full features, use the Depyler CLI tool.

## Best Practices

### Code Organization

1. **Start with pure functions** - easier to transpile and test
2. **Use explicit type hints** - improves transpilation accuracy
3. **Minimize global state** - Rust prefers explicit ownership
4. **Prefer composition over inheritance** - maps better to Rust

### Performance Tips

1. **Profile before optimizing** - focus on actual bottlenecks
2. **Use appropriate data structures** - Vec vs HashMap vs BTreeMap
3. **Consider memory layout** - struct packing and cache locality
4. **Benchmark iteratively** - measure each optimization

### Security Considerations

1. **Review generated unsafe blocks** - rare but requires attention
2. **Validate external inputs** - especially in web applications
3. **Use secure random number generation** - replace Python's `random`
4. **Audit dependencies** - `cargo audit` for security issues

---

This guide will continue to evolve as Depyler develops. For the latest
information, see our [documentation](../) and [release notes](../CHANGELOG.md).
