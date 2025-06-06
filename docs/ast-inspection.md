# AST Inspection Guide

## Overview

Depyler provides powerful AST inspection capabilities to help you understand how Python code is transformed into Rust. The `inspect` command allows you to examine intermediate representations at different stages of the transpilation pipeline.

## Available Representations

### 1. Python AST (`python-ast`)
The original Python Abstract Syntax Tree as parsed by rustpython-parser.

```bash
depyler inspect example.py --repr python-ast --format pretty
```

**Use cases:**
- Understanding how Python code is parsed
- Debugging Python syntax issues
- Learning AST structure

### 2. HIR - High-level IR (`hir`)
Depyler's high-level intermediate representation that includes:
- Type information
- Function properties (pure, terminates, panic-free)
- Depyler annotations
- Simplified control flow

```bash
depyler inspect example.py --repr hir --format pretty
```

**Use cases:**
- Understanding type inference results
- Checking annotation extraction
- Verifying function properties

### 3. Typed HIR (`typed-hir`)
Enhanced HIR with additional type analysis (currently same as HIR, will be extended).

```bash
depyler inspect example.py --repr typed-hir --format json
```

## Output Formats

### Pretty Format (`--format pretty`)
Human-readable, structured output with colors and formatting.

```
🦀 Depyler HIR Structure
=========================

🔧 Functions (3):

1. Function: calculate_score
   Parameters: attempts: Int, rounds: Int -> Int
   Body: 7 statements
   Properties: pure, terminates, panic-free
   Annotations:
     • optimization_level: Standard
     • bounds_checking: Explicit
   Body:
     1: Expression statement
     2: If statement
     3: Assignment to 'base_score'
     ...
```

### JSON Format (`--format json`)
Machine-readable JSON for programmatic processing.

```json
{
  "functions": [
    {
      "name": "calculate_score",
      "params": [
        ["attempts", "Int"],
        ["rounds", "Int"]
      ],
      "ret_type": "Int",
      "body": [...],
      "properties": {
        "is_pure": true,
        "always_terminates": true,
        "panic_free": true
      },
      "annotations": {
        "optimization_level": "Standard",
        "bounds_checking": "Explicit"
      }
    }
  ]
}
```

### Debug Format (`--format debug`)
Raw Rust debug output showing all internal details.

## Command Examples

### Basic Inspection
```bash
# Inspect HIR with pretty formatting
depyler inspect marco_polo_simple.py

# Inspect Python AST
depyler inspect marco_polo_simple.py --repr python-ast

# Get JSON output
depyler inspect marco_polo_simple.py --format json
```

### Advanced Usage
```bash
# Save to file
depyler inspect code.py --repr hir --format json -o analysis.json

# Compare representations
depyler inspect code.py --repr python-ast > python.ast
depyler inspect code.py --repr hir > depyler.hir
diff python.ast depyler.hir
```

### Integration with Other Tools
```bash
# Process with jq
depyler inspect code.py --format json | jq '.functions[0].name'

# Count functions
depyler inspect code.py --format json | jq '.functions | length'

# Extract annotations
depyler inspect code.py --format json | jq '.functions[].annotations'
```

## Understanding HIR Output

### Function Properties
- **pure**: Function has no side effects
- **terminates**: Function guaranteed to terminate
- **panic-free**: Function cannot panic

### Type Information
- **Int**: i32 in Rust
- **Float**: f64 in Rust  
- **String**: String in Rust
- **Bool**: bool in Rust
- **List(T)**: Vec<T> in Rust
- **Dict(K,V)**: HashMap<K,V> in Rust

### Annotations
Extracted from `# @depyler:` comments:
- **optimization_level**: Conservative, Standard, Aggressive
- **string_strategy**: Conservative, AlwaysOwned, ZeroCopy
- **ownership_model**: Owned, Borrowed, Shared
- **bounds_checking**: Runtime, Explicit, Disabled

## Debugging Workflow

### 1. Check Python Parsing
```bash
depyler inspect broken.py --repr python-ast
```

### 2. Verify HIR Generation
```bash
depyler inspect working.py --repr hir --format pretty
```

### 3. Examine Annotations
```bash
depyler inspect annotated.py --repr hir --format json | jq '.functions[].annotations'
```

### 4. Compare Before/After
```bash
# Before adding annotations
depyler inspect code.py --repr hir -o before.json

# After adding annotations  
depyler inspect annotated_code.py --repr hir -o after.json

# Compare
diff before.json after.json
```

## Integration with IDEs

### VS Code
Create a task in `.vscode/tasks.json`:

```json
{
  "label": "Inspect HIR",
  "type": "shell",
  "command": "depyler",
  "args": ["inspect", "${file}", "--repr", "hir", "--format", "pretty"],
  "group": "build",
  "presentation": {
    "panel": "new"
  }
}
```

### Vim/Neovim
Add to your config:

```vim
command! DepylerInspect !depyler inspect % --repr hir --format pretty
```

## Performance Analysis

Use inspection to optimize code:

```bash
# Check if function properties are detected
depyler inspect slow.py --repr hir --format json | jq '.functions[] | select(.properties.is_pure == false)'

# Find functions without optimization annotations
depyler inspect code.py --repr hir --format json | jq '.functions[] | select(.annotations.optimization_level == "Conservative")'
```

## Troubleshooting

### Common Issues

1. **File not found**
   ```bash
   # Use absolute paths
   depyler inspect /full/path/to/file.py
   ```

2. **Invalid representation**
   ```bash
   # Valid options: python-ast, hir, typed-hir
   depyler inspect file.py --repr hir
   ```

3. **Large output**
   ```bash
   # Use file output
   depyler inspect large.py --format json -o output.json
   ```

## Tips and Tricks

1. **Quick annotation check**:
   ```bash
   depyler inspect file.py --format json | jq '.functions[].annotations' | grep -v '{}'
   ```

2. **Function complexity analysis**:
   ```bash
   depyler inspect file.py --format json | jq '.functions[] | {name, body_size: (.body | length)}'
   ```

3. **Type usage statistics**:
   ```bash
   depyler inspect file.py --format json | jq '.functions[].ret_type' | sort | uniq -c
   ```

---

The inspect command is essential for understanding Depyler's transpilation process and optimizing your Python code for better Rust output.