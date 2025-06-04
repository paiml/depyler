# MCP Integration Guide

[![Model Context Protocol](https://img.shields.io/badge/MCP-Compatible-brightgreen?style=for-the-badge&logo=ai)](https://modelcontextprotocol.io/)

Depyler provides full integration with the **Model Context Protocol (MCP)** v2024.11, enabling AI assistants to perform Python-to-Rust transpilation with formal verification capabilities.

## Overview

The Depyler MCP server exposes three core tools that enable AI assistants to:

1. **Transpile Python to Rust** with energy-efficient optimizations
2. **Analyze migration complexity** for entire codebases
3. **Verify transpilation correctness** with semantic equivalence checking

## Quick Start

### Installation

```bash
# Clone and build Depyler
git clone https://github.com/your-org/depyler.git
cd depyler
cargo build --release

# The MCP server is built into the main binary
./target/release/depyler mcp-server
```

### MCP Server Configuration

Add Depyler to your MCP client configuration:

```json
{
  "mcpServers": {
    "depyler": {
      "command": "./target/release/depyler",
      "args": ["mcp-server"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

## MCP Tools Reference

### 1. transpile_python

Converts Python code to verified Rust with optimization options.

#### Input Schema
```json
{
  "source": "def add(a: int, b: int) -> int:\n    return a + b",
  "mode": "inline",
  "options": {
    "optimization_level": "energy",
    "type_inference": "conservative", 
    "memory_safety": "strict",
    "verification_level": "basic"
  }
}
```

#### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `source` | string | required | Python source code to transpile |
| `mode` | enum | `"inline"` | Transpilation mode: `inline`, `file`, `project` |
| `options.optimization_level` | enum | `"balanced"` | `speed`, `size`, `energy`, `balanced` |
| `options.type_inference` | enum | `"conservative"` | `conservative`, `aggressive`, `minimal` |
| `options.memory_safety` | enum | `"strict"` | `strict`, `permissive` |
| `options.verification_level` | enum | `"basic"` | `none`, `basic`, `comprehensive` |

#### Output Schema
```json
{
  "rust_code": "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
  "compilation_command": "rustc --edition 2021 output.rs",
  "metrics": {
    "lines_of_code": 3,
    "cyclomatic_complexity": 1,
    "estimated_performance_gain": "15%",
    "memory_safety_score": 1.0,
    "energy_efficiency_rating": "A+"
  },
  "verification_status": {
    "passed": true,
    "warnings": [],
    "guarantees": ["memory_safe", "panic_free", "terminates"]
  }
}
```

### 2. analyze_migration_complexity

Analyzes an entire Python codebase to estimate migration effort and complexity.

#### Input Schema
```json
{
  "project_path": "/path/to/python/project",
  "analysis_depth": "standard",
  "options": {
    "include_patterns": ["*.py"],
    "exclude_patterns": ["test_*.py", "*_test.py"],
    "consider_dependencies": true
  }
}
```

#### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `project_path` | string | required | Path to Python project directory |
| `analysis_depth` | enum | `"standard"` | `quick`, `standard`, `deep` |
| `options.include_patterns` | array | `["*.py"]` | File patterns to include |
| `options.exclude_patterns` | array | `[]` | File patterns to exclude |
| `options.consider_dependencies` | boolean | `true` | Analyze external dependencies |

#### Output Schema
```json
{
  "complexity_score": 7.2,
  "total_python_loc": 15420,
  "estimated_rust_loc": 12850,
  "estimated_effort_hours": 180,
  "risk_assessment": {
    "overall_risk": "Medium",
    "risk_factors": [
      {
        "factor": "Dynamic typing usage",
        "severity": "High",
        "affected_files": 23,
        "mitigation": "Add comprehensive type hints"
      }
    ]
  },
  "migration_strategy": {
    "recommended_approach": "incremental",
    "phases": [
      {
        "phase": 1,
        "description": "Transpile core utilities",
        "estimated_hours": 40,
        "files": ["utils.py", "constants.py"]
      }
    ]
  },
  "compatibility_report": {
    "supported_features": 0.85,
    "unsupported_constructs": [
      "metaclasses",
      "exec/eval statements"
    ]
  }
}
```

### 3. verify_transpilation

Verifies that transpiled Rust code is semantically equivalent to the original Python.

#### Input Schema
```json
{
  "python_source": "def factorial(n):\n    return 1 if n <= 1 else n * factorial(n-1)",
  "rust_source": "fn factorial(n: i32) -> i32 {\n    if n <= 1 { 1 } else { n * factorial(n - 1) }\n}",
  "verification_level": "comprehensive",
  "options": {
    "property_checks": ["termination", "memory_safety", "overflow"],
    "test_cases": [
      {"input": [5], "expected_output": 120}
    ]
  }
}
```

#### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `python_source` | string | required | Original Python source code |
| `rust_source` | string | required | Transpiled Rust source code |
| `verification_level` | enum | `"basic"` | `basic`, `standard`, `comprehensive` |
| `options.property_checks` | array | `["basic"]` | Properties to verify |
| `options.test_cases` | array | `[]` | Custom test cases for validation |

#### Output Schema
```json
{
  "verification_passed": true,
  "semantic_equivalence_score": 0.98,
  "safety_guarantees": [
    "memory_safe",
    "panic_free", 
    "no_undefined_behavior",
    "terminates"
  ],
  "performance_comparison": {
    "rust_faster_by": "340%",
    "memory_usage_reduction": "45%",
    "energy_efficiency_improvement": "60%"
  },
  "property_verification_results": [
    {
      "property": "termination",
      "status": "proven",
      "method": "structural_recursion_analysis"
    }
  ],
  "test_results": {
    "total_tests": 25,
    "passed": 25,
    "failed": 0,
    "coverage": "100%"
  }
}
```

## Advanced Usage

### Batch Processing with MCP

```python
# Example AI assistant workflow using MCP
async def migrate_python_project(project_path: str):
    # 1. Analyze complexity first
    analysis = await mcp_call("analyze_migration_complexity", {
        "project_path": project_path,
        "analysis_depth": "deep"
    })
    
    # 2. Transpile files in recommended order
    for phase in analysis["migration_strategy"]["phases"]:
        for file_path in phase["files"]:
            with open(file_path) as f:
                python_code = f.read()
                
            result = await mcp_call("transpile_python", {
                "source": python_code,
                "mode": "file",
                "options": {
                    "optimization_level": "energy",
                    "verification_level": "comprehensive"
                }
            })
            
            # 3. Verify each transpilation
            verification = await mcp_call("verify_transpilation", {
                "python_source": python_code,
                "rust_source": result["rust_code"],
                "verification_level": "comprehensive"
            })
            
            if verification["verification_passed"]:
                # Write verified Rust code
                rust_path = file_path.replace(".py", ".rs")
                with open(rust_path, "w") as f:
                    f.write(result["rust_code"])
```

### Error Handling

The MCP server returns structured error responses:

```json
{
  "error": {
    "code": "TRANSPILATION_FAILED",
    "message": "Type inference failed for dynamic variable 'x'",
    "details": {
      "line": 15,
      "column": 8,
      "suggestion": "Add type annotation: x: int = ..."
    }
  }
}
```

### Performance Optimization

For large projects, use these optimization strategies:

1. **Incremental Analysis**: Use `analysis_depth: "quick"` for initial assessment
2. **Caching**: The MCP server caches analysis results between calls
3. **Parallel Processing**: Transpile independent modules simultaneously
4. **Verification Levels**: Use `basic` verification during development, `comprehensive` for production

## Integration Examples

### Claude Desktop Integration

```json
{
  "mcpServers": {
    "depyler": {
      "command": "cargo",
      "args": ["run", "--release", "--", "mcp-server"],
      "cwd": "/path/to/depyler"
    }
  }
}
```

### VS Code MCP Extension

```json
{
  "mcp.servers": [
    {
      "name": "Depyler Transpiler",
      "command": "./target/release/depyler mcp-server",
      "description": "Python to Rust transpilation with verification"
    }
  ]
}
```

### Custom AI Assistant Integration

```typescript
import { MCPClient } from '@modelcontextprotocol/sdk';

const client = new MCPClient();
await client.connect('stdio', {
  command: './target/release/depyler',
  args: ['mcp-server']
});

// Transpile Python function
const result = await client.callTool('transpile_python', {
  source: 'def greet(name: str) -> str:\n    return f"Hello, {name}!"',
  mode: 'inline',
  options: {
    optimization_level: 'energy',
    verification_level: 'comprehensive'
  }
});
```

## Protocol Compliance

Depyler's MCP implementation is fully compliant with **MCP v2024.11**:

- ✅ **Initialize Protocol**: Proper capability negotiation
- ✅ **Tool Discovery**: Dynamic tool listing with schemas
- ✅ **Tool Execution**: Async tool calls with structured I/O
- ✅ **Error Handling**: Standardized error responses
- ✅ **Progress Reporting**: Real-time progress updates for long operations
- ✅ **Resource Management**: Efficient memory and CPU usage
- ✅ **Security**: Sandboxed execution environment

## Troubleshooting

### Common Issues

1. **Transpilation Fails**
   - Check Python syntax and type annotations
   - Verify supported Python subset (see docs/python-support.md)
   - Use `analysis_depth: "deep"` to identify unsupported constructs

2. **Verification Errors**
   - Ensure both Python and Rust code compile correctly
   - Check for semantic differences in edge cases
   - Use custom test cases to validate specific scenarios

3. **Performance Issues**
   - Use `optimization_level: "speed"` for faster transpilation
   - Enable caching with `DEPYLER_CACHE=true`
   - Consider `analysis_depth: "quick"` for large projects

### Debug Mode

Enable verbose logging:

```bash
RUST_LOG=debug ./target/release/depyler mcp-server
```

## Contributing

To extend MCP functionality:

1. Add new tools in `crates/depyler-mcp/src/tools.rs`
2. Implement handlers in `crates/depyler-mcp/src/server.rs`
3. Add comprehensive tests in `crates/depyler-mcp/src/tests.rs`
4. Update this documentation

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines.

## License

Depyler MCP integration is licensed under the same terms as Depyler. See [LICENSE](../LICENSE) for details.