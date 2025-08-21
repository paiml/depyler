# Ruchy Interpreter Integration Specification

## Executive Summary

This specification defines the integration of Ruchy as an immediate execution backend for Depyler, enabling Python code to be transpiled to Ruchy intermediate representation and executed without compilation to native code. This provides a rapid prototyping and development workflow with sub-10ms startup times while maintaining type safety and property verification.

## Architecture Overview

### Pipeline Stages

```
Python Source → AST → HIR → Ruchy IR → Ruchy REPL → Execution Result
     ↓           ↓      ↓       ↓           ↓            ↓
  Parser     Type    HIR→    Ruchy     Runtime      Value/Error
            Infer   Ruchy    Parser    Evaluator
```

### Key Components

1. **HIR-to-Ruchy Transpiler**: Converts Depyler's HIR to Ruchy source code
2. **Ruchy Runtime Integration**: Embeds Ruchy REPL for immediate execution
3. **Value Bridge**: Bidirectional conversion between Python and Ruchy values
4. **Error Mapping**: Translates Ruchy runtime errors to Python-like messages

## Implementation Design

### HIR to Ruchy Transpilation

```rust
pub struct RuchyTranspiler {
    type_mapper: TypeMapper,
    name_mangler: NameMangler,
}

impl RuchyTranspiler {
    pub fn transpile(&self, hir: &HirModule) -> Result<String> {
        let mut output = String::new();
        
        // Transpile imports
        for import in &hir.imports {
            writeln!(output, "import {}", self.map_import(import))?;
        }
        
        // Transpile functions
        for func in &hir.functions {
            self.transpile_function(&mut output, func)?;
        }
        
        // Transpile main expression if present
        if let Some(main) = &hir.main_expr {
            writeln!(output, "{}", self.transpile_expr(main)?)?;
        }
        
        Ok(output)
    }
    
    fn transpile_function(&self, out: &mut String, func: &HirFunction) -> Result<()> {
        write!(out, "fun {}(", func.name)?;
        
        // Parameters with type annotations
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 { write!(out, ", ")?; }
            write!(out, "{}: {}", param.name, self.map_type(&param.ty)?)?;
        }
        
        writeln!(out, ") -> {} {{", self.map_type(&func.ret_type)?)?;
        
        // Function body
        writeln!(out, "    {}", self.transpile_expr(&func.body)?)?;
        writeln!(out, "}}")?;
        
        Ok(())
    }
}
```

### Type Mapping

| Python/HIR Type | Ruchy Type | Notes |
|-----------------|------------|-------|
| `int` | `i64` | Default integer type |
| `float` | `f64` | IEEE 754 double |
| `str` | `String` | UTF-8 string |
| `bool` | `bool` | Direct mapping |
| `list[T]` | `Vec<T>` | Dynamic array |
| `dict[K, V]` | `HashMap<K, V>` | Hash map |
| `set[T]` | `HashSet<T>` | Hash set |
| `tuple[T...]` | `(T...)` | Product type |
| `Optional[T]` | `Option<T>` | Nullable type |
| `Callable[[A], R]` | `fun(A) -> R` | Function type |
| `class` | `struct` + `impl` | Object-oriented mapping |

### Runtime Integration

```rust
pub struct RuchyInterpreter {
    repl: ruchy::runtime::Repl,
    value_bridge: ValueBridge,
}

impl RuchyInterpreter {
    pub fn new() -> Result<Self> {
        let mut repl = ruchy::runtime::Repl::new()?;
        
        // Configure for embedded use
        repl.set_config(ReplConfig {
            print_results: false,
            save_history: false,
            timeout: Duration::from_millis(100),
        });
        
        Ok(Self {
            repl,
            value_bridge: ValueBridge::new(),
        })
    }
    
    pub fn execute(&mut self, ruchy_code: &str) -> Result<PythonValue> {
        // Execute in Ruchy REPL
        let ruchy_value = self.repl.eval(ruchy_code)?;
        
        // Convert to Python-compatible value
        self.value_bridge.from_ruchy(ruchy_value)
    }
    
    pub fn execute_file(&mut self, path: &Path) -> Result<PythonValue> {
        let code = fs::read_to_string(path)?;
        self.execute(&code)
    }
}
```

### Value Bridge

```rust
pub struct ValueBridge;

impl ValueBridge {
    pub fn from_ruchy(&self, value: ruchy::runtime::Value) -> Result<PythonValue> {
        match value {
            Value::Int(i) => Ok(PythonValue::Int(i)),
            Value::Float(f) => Ok(PythonValue::Float(f)),
            Value::String(s) => Ok(PythonValue::String(s)),
            Value::Bool(b) => Ok(PythonValue::Bool(b)),
            Value::List(items) => {
                let python_items = items.into_iter()
                    .map(|v| self.from_ruchy(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(PythonValue::List(python_items))
            }
            Value::Function { .. } => Ok(PythonValue::Callable),
            Value::Unit => Ok(PythonValue::None),
            _ => Err(anyhow!("Unsupported Ruchy value type"))
        }
    }
    
    pub fn to_ruchy(&self, value: &PythonValue) -> Result<ruchy::runtime::Value> {
        match value {
            PythonValue::Int(i) => Ok(Value::Int(*i)),
            PythonValue::Float(f) => Ok(Value::Float(*f)),
            PythonValue::String(s) => Ok(Value::String(s.clone())),
            PythonValue::Bool(b) => Ok(Value::Bool(*b)),
            PythonValue::List(items) => {
                let ruchy_items = items.iter()
                    .map(|v| self.to_ruchy(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Value::List(ruchy_items))
            }
            PythonValue::None => Ok(Value::Unit),
            _ => Err(anyhow!("Cannot convert Python value to Ruchy"))
        }
    }
}
```

## CLI Integration

### New Command: `depyler interpret`

```bash
# Interpret Python file using Ruchy backend
depyler interpret script.py

# Interactive REPL mode
depyler interpret --repl

# With verification
depyler interpret script.py --verify

# Output intermediate Ruchy code
depyler interpret script.py --emit-ruchy > output.ruchy
```

### CLI Implementation

```rust
#[derive(Parser)]
pub struct InterpretCommand {
    /// Python file to interpret
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,
    
    /// Start interactive REPL
    #[arg(long)]
    pub repl: bool,
    
    /// Enable property verification
    #[arg(long)]
    pub verify: bool,
    
    /// Output Ruchy intermediate code
    #[arg(long)]
    pub emit_ruchy: bool,
    
    /// Timeout for execution (ms)
    #[arg(long, default_value = "5000")]
    pub timeout: u64,
}

impl InterpretCommand {
    pub fn run(&self) -> Result<()> {
        if self.repl {
            self.run_repl()
        } else if let Some(file) = &self.file {
            self.interpret_file(file)
        } else {
            self.interpret_stdin()
        }
    }
    
    fn interpret_file(&self, path: &Path) -> Result<()> {
        // Read Python source
        let source = fs::read_to_string(path)?;
        
        // Parse to AST
        let ast = parse_python(&source)?;
        
        // Convert to HIR
        let hir = ast_to_hir(&ast)?;
        
        // Verify if requested
        if self.verify {
            verify_properties(&hir)?;
        }
        
        // Transpile to Ruchy
        let transpiler = RuchyTranspiler::new();
        let ruchy_code = transpiler.transpile(&hir)?;
        
        if self.emit_ruchy {
            println!("{}", ruchy_code);
            return Ok(());
        }
        
        // Execute in Ruchy interpreter
        let mut interpreter = RuchyInterpreter::new()?;
        let result = interpreter.execute(&ruchy_code)?;
        
        // Display result
        println!("{}", format_python_value(&result));
        
        Ok(())
    }
}
```

## Example Workflows

### 1. Simple Script Execution

```python
# script.py
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(5))
```

```bash
$ depyler interpret script.py
120
```

Generated Ruchy code:
```rust
fun factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

print(factorial(5))
```

### 2. Data Processing Pipeline

```python
# process.py
from typing import List

def process_data(numbers: List[int]) -> List[int]:
    return [x * 2 for x in numbers if x > 0]

data = [1, -2, 3, -4, 5]
result = process_data(data)
print(result)
```

```bash
$ depyler interpret process.py --emit-ruchy
```

Generated Ruchy:
```rust
fun process_data(numbers: Vec<i64>) -> Vec<i64> {
    numbers
        |> filter(|x| x > 0)
        |> map(|x| x * 2)
        |> collect()
}

let data = vec![1, -2, 3, -4, 5];
let result = process_data(data);
print(result)
```

### 3. Interactive REPL

```bash
$ depyler interpret --repl
Depyler-Ruchy REPL v3.0.0
Type :help for commands

>>> def add(x: int, y: int) -> int:
...     return x + y
...
>>> add(3, 4)
7

>>> numbers = [1, 2, 3, 4, 5]
>>> sum(x * x for x in numbers)
55

>>> :show add
fun add(x: i64, y: i64) -> i64 {
    x + y
}

>>> :exit
```

## Performance Characteristics

### Execution Modes

| Mode | Startup Time | Throughput | Use Case |
|------|--------------|------------|----------|
| Interpret (tree-walk) | <10ms | 1M ops/s | REPL, prototyping |
| Bytecode VM | <20ms | 10M ops/s | Scripts, testing |
| JIT (future) | <100ms | 50M ops/s | Hot loops |
| AOT (via rustc) | >1s | 100M ops/s | Production |

### Memory Usage

- Base interpreter: ~10MB
- Per binding overhead: ~4KB
- AST retention: ~100KB per KLOC
- Type environment: ~50KB per KLOC

## Error Handling

### Error Mapping

```rust
pub struct ErrorMapper;

impl ErrorMapper {
    pub fn map_ruchy_error(&self, error: RuchyError) -> PythonError {
        match error {
            RuchyError::ParseError { line, col, msg } => {
                PythonError::SyntaxError {
                    line,
                    col,
                    msg: self.pythonize_message(msg),
                }
            }
            RuchyError::TypeError { expected, found } => {
                PythonError::TypeError {
                    message: format!("Expected {}, got {}", 
                        self.python_type(&expected),
                        self.python_type(&found))
                }
            }
            RuchyError::RuntimeError(msg) => {
                PythonError::RuntimeError { message: msg }
            }
            _ => PythonError::Unknown(error.to_string())
        }
    }
}
```

### Example Error Messages

```python
# Type error
>>> def add(x: int, y: int) -> int:
...     return x + y
...
>>> add("hello", 5)
TypeError: Expected int, got str
  in function add, parameter x
  at line 1, column 5

# Runtime error
>>> numbers = [1, 2, 3]
>>> numbers[10]
IndexError: list index out of range
  index: 10, list length: 3
  at line 1, column 1
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_basic_transpilation() {
    let python = "def add(x: int, y: int) -> int: return x + y";
    let ruchy = transpile_to_ruchy(python).unwrap();
    assert!(ruchy.contains("fun add"));
    assert!(ruchy.contains("i64"));
}

#[test]
fn test_value_roundtrip() {
    let python_val = PythonValue::List(vec![
        PythonValue::Int(1),
        PythonValue::Int(2),
    ]);
    let bridge = ValueBridge::new();
    let ruchy_val = bridge.to_ruchy(&python_val).unwrap();
    let back = bridge.from_ruchy(ruchy_val).unwrap();
    assert_eq!(python_val, back);
}
```

### Integration Tests

```rust
#[test]
fn test_fibonacci_execution() {
    let python = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n-1) + fib(n-2)

result = fib(10)
"#;
    
    let mut interpreter = create_interpreter();
    let result = interpreter.execute_python(python).unwrap();
    
    assert_eq!(result.get("result"), Some(&PythonValue::Int(55)));
}
```

### Property Tests

```rust
#[proptest]
fn test_type_preservation(python_expr: ValidPythonExpr) {
    let hir = parse_and_type_check(&python_expr).unwrap();
    let ruchy = transpile_to_ruchy(&hir).unwrap();
    let ruchy_type = infer_ruchy_type(&ruchy).unwrap();
    
    prop_assert!(types_compatible(&hir.ty, &ruchy_type));
}
```

## Quality Gates

### Mandatory Checks

1. **Transpilation Correctness**: All Python constructs map to valid Ruchy
2. **Type Safety**: No type errors in generated Ruchy code
3. **Value Fidelity**: Python values round-trip without loss
4. **Error Clarity**: All errors map to Python-like messages
5. **Performance**: <10ms startup, >1M ops/s for basic operations

### Pre-commit Validation

```bash
#!/bin/bash
# Verify Ruchy integration works
echo 'print("Hello")' | cargo run -- interpret --stdin || {
    echo "❌ FATAL: Ruchy interpreter broken"
    exit 1
}

# Test type checking
echo 'def f(x: int) -> str: return x' | cargo run -- interpret --stdin 2>&1 | grep -q "TypeError" || {
    echo "❌ FATAL: Type checking not working"
    exit 1
}

# Performance check
time echo '2 + 2' | timeout 0.1s cargo run -- interpret --stdin || {
    echo "❌ FATAL: Interpreter too slow"
    exit 1
}
```

## Implementation Milestones

### Phase 1: Core Integration (Week 1)
- [ ] HIR to Ruchy transpiler for basic types
- [ ] Value bridge implementation
- [ ] Basic CLI command
- [ ] Simple expression evaluation

### Phase 2: Type System (Week 2)
- [ ] Complex type mapping
- [ ] Generic type handling
- [ ] Error message mapping
- [ ] Type verification

### Phase 3: Full Language Support (Week 3)
- [ ] Control flow structures
- [ ] Function definitions
- [ ] Class transpilation
- [ ] Module imports

### Phase 4: Polish and Performance (Week 4)
- [ ] REPL mode
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Documentation

## Future Enhancements

1. **JIT Compilation**: Detect hot paths and JIT-compile to native code
2. **Incremental Execution**: Cache transpiled Ruchy for faster re-execution
3. **Debugger Integration**: Step through Python code with Ruchy backend
4. **Property Synthesis**: Generate property tests from type annotations
5. **MCP Tool Export**: Expose Python functions as MCP tools via Ruchy

## Conclusion

The Ruchy interpreter integration provides Depyler with a fast, type-safe execution backend that bridges the gap between Python's ease of use and Rust's performance. By leveraging Ruchy's REPL and runtime, we achieve sub-10ms startup times while maintaining the safety guarantees and verification capabilities that are core to Depyler's value proposition.

This integration follows the Ruchy project management style:
- **Zero tolerance for technical debt**: No TODOs in implementation
- **Quality gates enforced**: All code passes clippy with zero warnings
- **Property testing**: Every component has property tests
- **Performance validated**: Benchmarks confirm <10ms startup