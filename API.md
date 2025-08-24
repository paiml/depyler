# Depyler API Documentation

## Core Library API

### depyler-core

The core transpilation engine providing Python AST to Rust HIR conversion.

#### `DepylerPipeline`

Main transpilation pipeline orchestrating the conversion process.

```rust
use depyler_core::DepylerPipeline;

let pipeline = DepylerPipeline::new();
let rust_code = pipeline.transpile(python_source)?;
```

**Methods:**

- `new() -> Self` - Create new pipeline with default configuration
- `transpile(&self, source: &str) -> Result<String>` - Transpile Python source to Rust
- `transpile_with_options(&self, source: &str, options: TranspileOptions) -> Result<String>` - Transpile with custom options
- `verify(&self, rust_code: &str) -> Result<VerificationResult>` - Verify generated Rust code

#### `TranspileOptions`

Configuration for transpilation behavior.

```rust
use depyler_core::TranspileOptions;

let options = TranspileOptions::builder()
    .verification_level(VerificationLevel::Strict)
    .optimize_level(OptimizeLevel::Release)
    .target_edition(Edition::Rust2021)
    .build();
```

**Fields:**

- `verification_level: VerificationLevel` - Verification strictness (None, Basic, Strict)
- `optimize_level: OptimizeLevel` - Optimization level (Debug, Release, Size)
- `target_edition: Edition` - Target Rust edition
- `preserve_comments: bool` - Preserve Python comments in output
- `generate_tests: bool` - Generate test modules for functions

### depyler-hir

High-level Intermediate Representation for Python-to-Rust conversion.

#### `HirModule`

Root HIR structure representing a Python module.

```rust
use depyler_hir::{HirModule, HirFunction, HirClass};

let module = HirModule {
    name: "my_module".to_string(),
    imports: vec![],
    functions: vec![function],
    classes: vec![],
    globals: vec![],
};
```

#### `HirFunction`

HIR representation of a Python function.

```rust
use depyler_hir::{HirFunction, HirParam, Type};

let function = HirFunction {
    name: "calculate".to_string(),
    params: vec![
        HirParam {
            name: "x".to_string(),
            ty: Type::Int,
            default: None,
        }
    ],
    ret_type: Type::Int,
    body: HirBlock { statements: vec![] },
    properties: FunctionProperties::default(),
};
```

#### `Type`

Type system for HIR nodes.

```rust
use depyler_hir::Type;

// Basic types
Type::Int           // i64
Type::Float         // f64
Type::String        // String
Type::Bool          // bool
Type::Unit          // ()

// Container types
Type::List(Box::new(Type::Int))              // Vec<i64>
Type::Dict(Box::new(Type::String), Box::new(Type::Int))  // HashMap<String, i64>
Type::Option(Box::new(Type::String))          // Option<String>
Type::Result(Box::new(Type::Int), Box::new(Type::String)) // Result<i64, String>
```

### depyler-analyzer

Static analysis and type inference for Python code.

#### `Analyzer`

Performs static analysis on Python AST.

```rust
use depyler_analyzer::{Analyzer, AnalysisResult};

let analyzer = Analyzer::new();
let result = analyzer.analyze(python_ast)?;

// Access inferred types
let var_type = result.get_type("variable_name");

// Check for errors
for error in result.errors() {
    println!("Analysis error: {}", error);
}
```

#### `TypeInferencer`

Type inference engine for Python code.

```rust
use depyler_analyzer::TypeInferencer;

let mut inferencer = TypeInferencer::new();
inferencer.add_annotation("x", Type::Int);
let inferred = inferencer.infer_expr(expr)?;
```

### depyler-verify

Property verification for generated Rust code.

#### `PropertyVerifier`

Verifies safety properties of generated code.

```rust
use depyler_verify::{PropertyVerifier, Property};

let verifier = PropertyVerifier::new();
let properties = vec![
    Property::NoUseAfterFree,
    Property::NoDataRaces,
    Property::BorrowCheckerCompliant,
];

let result = verifier.verify_properties(&hir_module, &properties)?;
if result.is_safe() {
    println!("All properties verified!");
}
```

#### `VerificationLevel`

Strictness levels for verification.

```rust
use depyler_verify::VerificationLevel;

VerificationLevel::None    // Skip verification
VerificationLevel::Basic   // Type checking only
VerificationLevel::Strict  // Full property verification
```

## Agent Mode API

### depyler-agent

Background daemon for continuous transpilation.

#### `Agent`

Main agent controller.

```rust
use depyler_agent::{Agent, AgentConfig};

let config = AgentConfig::default();
let agent = Agent::new(config)?;

// Start agent
agent.start().await?;

// Add project to monitor
agent.add_project("/path/to/project", vec!["**/*.py"]).await?;

// Get status
let status = agent.status().await?;

// Stop agent
agent.stop().await?;
```

#### `AgentConfig`

Agent configuration structure.

```rust
use depyler_agent::AgentConfig;

let config = AgentConfig {
    port: 3000,
    host: "127.0.0.1".to_string(),
    auto_transpile: true,
    watch_debounce: Duration::from_millis(500),
    max_workers: 4,
    log_level: "info".to_string(),
};
```

### depyler-mcp

Model Context Protocol server implementation.

#### `McpServer`

MCP server for Claude Code integration.

```rust
use depyler_mcp::{McpServer, McpTool};

let server = McpServer::new(agent)?;

// Register tools
server.register_tool(McpTool::TranspilePythonFile);
server.register_tool(McpTool::MonitorPythonProject);

// Start server
server.listen("127.0.0.1:3000").await?;
```

#### MCP Tools

Available MCP tools for transpilation operations.

```rust
use depyler_mcp::tools::*;

// Transpile single file
let result = transpile_python_file(TranspileFileRequest {
    file_path: "/path/to/file.py".to_string(),
    verify: true,
    optimize: false,
}).await?;

// Monitor project
let result = monitor_python_project(MonitorProjectRequest {
    project_path: "/path/to/project".to_string(),
    patterns: vec!["**/*.py".to_string()],
    auto_transpile: true,
}).await?;

// Get status
let status = get_transpilation_status(StatusRequest {
    project_path: Some("/path/to/project".to_string()),
}).await?;

// Verify Rust code
let result = verify_rust_code(VerifyRequest {
    rust_code: generated_code,
    level: VerificationLevel::Strict,
}).await?;

// Analyze compatibility
let result = analyze_python_compatibility(AnalyzeRequest {
    file_path: "/path/to/file.py".to_string(),
}).await?;
```

## CLI API

### Command Line Interface

```bash
# Core transpilation
depyler transpile <file.py> [options]
  --output, -o <file.rs>    Output file (default: stdout)
  --verify                   Enable verification
  --optimize                 Enable optimizations
  --quiet, -q               Suppress output

# Agent commands
depyler agent start [options]
  --port <port>             Server port (default: 3000)
  --foreground              Run in foreground
  --debug                   Enable debug logging

depyler agent stop
depyler agent status
depyler agent restart

depyler agent add-project <path> [options]
  --patterns <patterns>     File patterns to watch
  --auto                    Enable auto-transpilation

depyler agent remove-project <path>
depyler agent list-projects

# Utility commands
depyler analyze <file.py>    Analyze Python compatibility
depyler verify <file.rs>     Verify Rust code
depyler version              Show version information
```

## Error Types

### `TranspileError`

Main error type for transpilation failures.

```rust
use depyler_core::TranspileError;

match pipeline.transpile(source) {
    Ok(rust_code) => println!("{}", rust_code),
    Err(TranspileError::UnsupportedFeature(feature)) => {
        eprintln!("Unsupported Python feature: {}", feature);
    }
    Err(TranspileError::TypeError(msg)) => {
        eprintln!("Type error: {}", msg);
    }
    Err(e) => eprintln!("Transpilation failed: {}", e),
}
```

### Error Variants

- `TranspileError::ParseError(String)` - Python parsing failed
- `TranspileError::UnsupportedFeature(String)` - Unsupported Python construct
- `TranspileError::TypeError(String)` - Type inference/checking error
- `TranspileError::OwnershipError(String)` - Ownership inference failed
- `TranspileError::VerificationError(String)` - Property verification failed
- `TranspileError::IoError(std::io::Error)` - File I/O error

## Examples

### Basic Transpilation

```rust
use depyler_core::DepylerPipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let python_code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python_code)?;
    
    println!("{}", rust_code);
    Ok(())
}
```

### Agent Integration

```rust
use depyler_agent::{Agent, AgentConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AgentConfig {
        port: 3000,
        auto_transpile: true,
        watch_debounce: Duration::from_millis(500),
        ..Default::default()
    };

    let agent = Agent::new(config)?;
    agent.start().await?;

    // Monitor Python project
    agent.add_project(
        "/home/user/my_python_project",
        vec!["**/*.py", "!**/__pycache__/**"]
    ).await?;

    // Keep running
    tokio::signal::ctrl_c().await?;
    agent.stop().await?;
    
    Ok(())
}
```

### Custom Verification

```rust
use depyler_verify::{PropertyVerifier, Property};
use depyler_hir::HirModule;

fn verify_module(module: &HirModule) -> Result<(), Box<dyn std::error::Error>> {
    let verifier = PropertyVerifier::new();
    
    let properties = vec![
        Property::NoUseAfterFree,
        Property::NoDataRaces,
        Property::BorrowCheckerCompliant,
        Property::NoMemoryLeaks,
    ];

    let result = verifier.verify_properties(module, &properties)?;
    
    for violation in result.violations() {
        eprintln!("Property violation: {}", violation);
    }

    if result.is_safe() {
        println!("Module verified as safe!");
    }

    Ok(())
}
```

## Performance Considerations

### Transpilation Performance

- **Throughput**: >10MB/s for typical Python code
- **Memory**: O(n) where n is source size
- **Parallelization**: Multiple files transpiled concurrently

### Agent Performance

- **File watching**: Uses efficient OS-specific APIs (inotify/FSEvents/ReadDirectoryChangesW)
- **Debouncing**: Configurable delay to batch rapid changes
- **Worker pool**: Configurable number of transpilation workers

### Optimization Tips

1. **Use type annotations** - Reduces inference overhead
2. **Enable caching** - Agent caches transpilation results
3. **Tune debounce interval** - Balance responsiveness vs CPU usage
4. **Selective monitoring** - Use specific file patterns
5. **Batch operations** - Process multiple files together

## Thread Safety

All public APIs are thread-safe and can be used from multiple threads:

- `DepylerPipeline` - Immutable, safe to share
- `Agent` - Uses Arc<Mutex<>> internally
- `McpServer` - Async-safe with Tokio
- `PropertyVerifier` - Stateless, safe to share

## Version Compatibility

- **Rust**: 1.70.0 or later required
- **Python**: Targets Python 3.8+ syntax
- **MCP**: Compatible with MCP 1.0 specification
- **Claude Code**: Tested with Claude Desktop 1.0+

---

For more examples and detailed usage, see the [examples directory](./examples/) in the repository.