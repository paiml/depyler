# Ruchy Script Format Transpilation Specification
## Depyler v3.0 - Multi-Target Transpilation Architecture

### Executive Summary

This specification defines the integration of Ruchy script format as an alternative transpilation target for Depyler, enabling Python code to be transpiled to either idiomatic Rust or Ruchy script format. Ruchy provides a more Python-like syntax with functional programming features, making it an ideal intermediate language for gradual migration strategies.

### Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Python-to-Ruchy Mapping](#python-to-ruchy-mapping)
3. [Type System Translation](#type-system-translation)
4. [Implementation Plan](#implementation-plan)
5. [Quality Gates](#quality-gates)
6. [Performance Targets](#performance-targets)
7. [Release Strategy](#release-strategy)

---

## Architecture Overview

### Multi-Target Transpilation Pipeline

```rust
pub enum TranspilationTarget {
    Rust,      // Default: Generate idiomatic Rust code
    Ruchy,     // New: Generate Ruchy script format
}

pub struct DepylerCore {
    target: TranspilationTarget,
    python_parser: PythonParser,
    hir_builder: HirBuilder,
    backend: Box<dyn TranspilationBackend>,
}

pub trait TranspilationBackend {
    fn transpile(&self, hir: &Hir) -> Result<String, TranspileError>;
    fn validate_output(&self, code: &str) -> Result<(), ValidationError>;
    fn optimize(&self, hir: &Hir) -> Hir;
}
```

### Ruchy Backend Architecture

```rust
pub struct RuchyBackend {
    // AST builder for Ruchy expressions
    ast_builder: RuchyAstBuilder,
    
    // Type mapping engine
    type_mapper: PythonToRuchyTypeMapper,
    
    // Optimization passes
    optimizer: RuchyOptimizer,
    
    // Code formatter
    formatter: RuchyFormatter,
}

impl TranspilationBackend for RuchyBackend {
    fn transpile(&self, hir: &Hir) -> Result<String, TranspileError> {
        // Phase 1: Convert HIR to Ruchy AST
        let ruchy_ast = self.ast_builder.build(hir)?;
        
        // Phase 2: Apply Ruchy-specific optimizations
        let optimized = self.optimizer.optimize(ruchy_ast)?;
        
        // Phase 3: Format and serialize
        Ok(self.formatter.format(optimized))
    }
}
```

---

## Python-to-Ruchy Mapping

### Core Language Constructs

| Python Construct | Ruchy Equivalent | Notes |
|-----------------|------------------|-------|
| `def func():` | `fun func()` | Direct mapping |
| `class MyClass:` | `struct MyClass { }` | With impl blocks |
| `lambda x: x + 1` | `\|x\| x + 1` | Rust-style lambdas |
| `async def` | `async fun` | Direct async support |
| `await expr` | `expr.await` | Postfix await |
| `try/except` | `try { } catch { }` | Error handling |
| `with ctx:` | `defer { }` | Resource management |
| `@decorator` | `#[decorator]` | Attribute syntax |

### Data Structures

```python
# Python
my_list = [1, 2, 3]
my_dict = {"key": "value"}
my_set = {1, 2, 3}

# Ruchy
let my_list = [1, 2, 3]
let my_dict = {"key": "value"}  
let my_set = {1, 2, 3}
```

### Control Flow

```python
# Python
for item in items:
    if condition:
        process(item)
    else:
        skip(item)

# Ruchy
for item in items {
    if condition {
        process(item)
    } else {
        skip(item)
    }
}
```

### Advanced Features

#### List Comprehensions → Pipeline Operators

```python
# Python
result = [x * 2 for x in items if x > 0]

# Ruchy
let result = items |> filter(|x| x > 0) |> map(|x| x * 2)
```

#### Generators → Lazy Iterators

```python
# Python
def generate():
    for i in range(10):
        yield i * 2

# Ruchy
fun generate() -> impl Iterator<i32> {
    (0..10) |> map(|i| i * 2)
}
```

#### Async/Await → Actor System

```python
# Python
async def fetch_data():
    data = await api_call()
    return process(data)

# Ruchy
async fun fetch_data() {
    let data = api_call().await
    process(data)
}
```

---

## Type System Translation

### Type Mapping Table

| Python Type | Ruchy Type | Notes |
|------------|------------|-------|
| `int` | `i64` | Default to 64-bit |
| `float` | `f64` | IEEE 754 double |
| `str` | `String` | UTF-8 strings |
| `bool` | `bool` | Direct mapping |
| `List[T]` | `Vec<T>` or `[T]` | Mutable vs slice |
| `Dict[K, V]` | `HashMap<K, V>` | Hash map |
| `Set[T]` | `HashSet<T>` | Hash set |
| `Optional[T]` | `T?` | Option type |
| `Union[A, B]` | `enum { A(A), B(B) }` | Sum types |
| `Tuple[A, B]` | `(A, B)` | Product types |
| `Callable[[A], B]` | `fun(A) -> B` | Function types |
| `Any` | `dyn Any` | Dynamic typing |

### Type Inference Strategy

```rust
pub struct TypeInferencer {
    // Hindley-Milner type inference
    unifier: TypeUnifier,
    
    // Python type hints parser
    hint_parser: TypeHintParser,
    
    // Usage-based inference
    usage_analyzer: UsageAnalyzer,
}

impl TypeInferencer {
    pub fn infer_types(&self, ast: &PythonAst) -> TypedHir {
        // 1. Parse explicit type hints
        let hints = self.hint_parser.parse(ast);
        
        // 2. Analyze usage patterns
        let usage = self.usage_analyzer.analyze(ast);
        
        // 3. Unify and resolve types
        self.unifier.unify(hints, usage)
    }
}
```

---

## Implementation Plan

### Phase 1: Foundation (Week 1)

#### Core Infrastructure
```rust
// crates/depyler-ruchy/src/lib.rs
pub mod ast;
pub mod backend;
pub mod formatter;
pub mod optimizer;
pub mod types;

pub struct RuchyTranspiler {
    parser: RuchyParser,
    validator: RuchyValidator,
}
```

#### AST Definition
```rust
// crates/depyler-ruchy/src/ast.rs
#[derive(Debug, Clone, PartialEq)]
pub enum RuchyExpr {
    Literal(Literal),
    Identifier(String),
    Binary { left: Box<RuchyExpr>, op: BinaryOp, right: Box<RuchyExpr> },
    Pipeline { expr: Box<RuchyExpr>, stages: Vec<PipelineStage> },
    Function { name: String, params: Vec<Param>, body: Box<RuchyExpr> },
    // ... complete AST nodes
}
```

### Phase 2: Type System (Week 2)

#### Type Mapper Implementation
```rust
// crates/depyler-ruchy/src/types.rs
pub struct TypeMapper {
    python_types: HashMap<String, PythonType>,
    ruchy_types: HashMap<String, RuchyType>,
    mappings: BiMap<PythonType, RuchyType>,
}

impl TypeMapper {
    pub fn map_type(&self, py_type: &PythonType) -> Result<RuchyType> {
        match py_type {
            PythonType::Primitive(p) => self.map_primitive(p),
            PythonType::Collection(c) => self.map_collection(c),
            PythonType::Function(f) => self.map_function(f),
            PythonType::Class(c) => self.map_class_to_struct(c),
        }
    }
}
```

### Phase 3: Transformations (Week 3)

#### Pattern-Based Transformations
```rust
// crates/depyler-ruchy/src/transforms.rs
pub struct Transformer {
    patterns: Vec<TransformPattern>,
}

impl Transformer {
    pub fn transform(&self, hir: &Hir) -> RuchyAst {
        let mut ast = self.base_transform(hir);
        
        // Apply pattern-based optimizations
        for pattern in &self.patterns {
            ast = pattern.apply(ast);
        }
        
        ast
    }
}

// Transformation patterns
pub enum TransformPattern {
    ListCompToPipeline,
    AsyncToActor,
    GeneratorToIterator,
    DecoratorToAttribute,
}
```

### Phase 4: Optimization (Week 4)

#### Ruchy-Specific Optimizations
```rust
pub struct RuchyOptimizer {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl RuchyOptimizer {
    pub fn optimize(&self, ast: RuchyAst) -> RuchyAst {
        let mut optimized = ast;
        
        for pass in &self.passes {
            optimized = pass.run(optimized);
        }
        
        optimized
    }
}

// Optimization passes
pub struct PipelineFusion;
pub struct DeadCodeElimination;
pub struct InlineSimpleFunctions;
pub struct CommonSubexpressionElimination;
```

### Phase 5: Code Generation (Week 5)

#### Formatter Implementation
```rust
pub struct RuchyFormatter {
    indent_width: usize,
    max_line_length: usize,
    style: FormatStyle,
}

impl RuchyFormatter {
    pub fn format(&self, ast: &RuchyAst) -> String {
        let mut output = String::new();
        self.format_node(ast, 0, &mut output);
        output
    }
    
    fn format_node(&self, node: &RuchyExpr, indent: usize, out: &mut String) {
        match node {
            RuchyExpr::Function { name, params, body, .. } => {
                write!(out, "fun {}(", name);
                self.format_params(params, out);
                writeln!(out, ") {{");
                self.format_node(body, indent + self.indent_width, out);
                writeln!(out, "}}");
            }
            // ... other node types
        }
    }
}
```

---

## Quality Gates

### Testing Requirements

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_transpilation() {
        let python = "def add(x, y): return x + y";
        let expected = "fun add(x, y) { x + y }";
        assert_eq!(transpile_to_ruchy(python).unwrap(), expected);
    }
    
    #[test]
    fn test_pipeline_transformation() {
        let python = "[x * 2 for x in items if x > 0]";
        let expected = "items |> filter(|x| x > 0) |> map(|x| x * 2)";
        assert_eq!(transpile_to_ruchy(python).unwrap(), expected);
    }
}
```

#### Property-Based Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn ruchy_output_is_valid(python_code in python_ast_strategy()) {
        let ruchy_code = transpile_to_ruchy(&python_code)?;
        let parsed = ruchy::parse(&ruchy_code);
        prop_assert!(parsed.is_ok());
    }
    
    #[test]
    fn type_mapping_is_sound(py_type in python_type_strategy()) {
        let ruchy_type = map_type(&py_type)?;
        prop_assert!(is_compatible(&py_type, &ruchy_type));
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_ruchy_runtime_execution() {
    let python = r#"
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;
    
    let ruchy_code = transpile_to_ruchy(python).unwrap();
    let result = ruchy::execute(&ruchy_code, &["fibonacci", "10"]);
    assert_eq!(result.unwrap(), "55");
}
```

### Coverage Requirements

- **Line Coverage**: ≥ 95%
- **Branch Coverage**: ≥ 90%
- **Property Test Cases**: ≥ 100
- **Integration Tests**: All example programs must transpile and execute correctly

### Performance Benchmarks

```rust
#[bench]
fn bench_ruchy_transpilation(b: &mut Bencher) {
    let python = load_test_file("binary_search.py");
    b.iter(|| {
        transpile_to_ruchy(&python)
    });
}

// Performance targets:
// - Transpilation speed: > 50K LOC/sec
// - Memory usage: < 100 MB for 10K LOC
// - Output size: < 1.5x Python source
```

---

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Transpilation Speed | > 50K LOC/sec | Real-time IDE support |
| Memory Usage | < 100 MB / 10K LOC | Resource efficiency |
| Output Size | < 1.5x source | Readable output |
| Type Inference | > 85% coverage | Minimal annotations |
| Optimization Impact | > 20% reduction | Worthwhile transforms |

---

## Release Strategy

### Version 3.0.0 Release Plan

#### Pre-Release Checklist
- [ ] All unit tests passing (100%)
- [ ] Property tests passing (100%)
- [ ] Integration tests with Ruchy runtime
- [ ] Performance benchmarks meet targets
- [ ] Documentation complete
- [ ] Examples for all features
- [ ] CLI integration tested
- [ ] Backwards compatibility verified

#### Release Process
1. **Version Bump**: Update Cargo.toml to 3.0.0
2. **Changelog**: Document all new features
3. **Testing**: Run full regression suite
4. **Documentation**: Update README and guides
5. **Publishing**: Release to crates.io
6. **Announcement**: Blog post and social media

### Feature Flags

```toml
[features]
default = ["rust-backend"]
ruchy = ["dep:ruchy-parser", "dep:ruchy-formatter"]
all-backends = ["rust-backend", "ruchy"]
```

### CLI Interface

```bash
# Default: Transpile to Rust
depyler transpile input.py -o output.rs

# New: Transpile to Ruchy
depyler transpile input.py --target=ruchy -o output.ruchy

# With optimizations
depyler transpile input.py --target=ruchy --optimize -o output.ruchy

# Validate output
depyler validate output.ruchy --format=ruchy
```

---

## Implementation Timeline

### Week 1: Foundation
- [x] Create specification
- [ ] Set up project structure
- [ ] Define Ruchy AST types
- [ ] Implement basic transpiler

### Week 2: Type System
- [ ] Type mapping implementation
- [ ] Type inference integration
- [ ] Generic type handling
- [ ] Union type resolution

### Week 3: Transformations
- [ ] List comprehension → pipeline
- [ ] Generator → iterator
- [ ] Async → actor system
- [ ] Pattern matching

### Week 4: Optimization
- [ ] Pipeline fusion
- [ ] Dead code elimination
- [ ] Function inlining
- [ ] CSE optimization

### Week 5: Quality & Release
- [ ] Complete test suite
- [ ] Performance benchmarks
- [ ] Documentation
- [ ] Release preparation

---

## Risk Mitigation

### Technical Risks
1. **Semantic Differences**: Some Python features may not map cleanly
   - Mitigation: Provide fallback to Rust backend
   
2. **Performance Regression**: Ruchy output might be slower
   - Mitigation: Extensive benchmarking and optimization

3. **Type Inference Failures**: Complex Python code may resist inference
   - Mitigation: Support gradual typing with annotations

### Quality Assurance
- Automated testing on every commit
- Property-based testing for edge cases
- Fuzzing for parser robustness
- Performance regression detection

---

## Success Criteria

1. **Functional**: All Python examples transpile to valid Ruchy
2. **Performance**: Meets or exceeds performance targets
3. **Quality**: 100% test coverage, zero known bugs
4. **Usability**: Clear documentation and examples
5. **Integration**: Seamless CLI and IDE support

---

## Appendix: Example Transformations

### Complex Example: Data Processing Pipeline

```python
# Python
import pandas as pd

def process_data(df):
    return (df[df['age'] > 18]
           .groupby('category')
           .agg({'value': 'sum'})
           .sort_values('value', ascending=False))

# Ruchy
fun process_data(df: DataFrame) -> DataFrame {
    df |> filter(|row| row.age > 18)
       |> group_by("category")
       |> agg({"value": sum})
       |> sort_by("value", descending: true)
}
```

### Actor System Example

```python
# Python
import asyncio

class DataProcessor:
    async def process(self, data):
        result = await self.transform(data)
        return await self.validate(result)

# Ruchy
actor DataProcessor {
    state data: Vec<Data>
    
    receive {
        Process(data) => {
            let result = self.transform(data).await
            self.validate(result).await
        }
    }
}
```

---

*This specification defines a comprehensive approach to adding Ruchy script format as a transpilation target for Depyler, ensuring high quality, performance, and maintainability.*