# Generator Expressions (DEPYLER-TBD) - v3.13.0

**Status**: Planning
**Created**: 2025-10-09
**Target Release**: v3.13.0
**Dependencies**: Generators Phase 3 (DEPYLER-0115) ✅ Complete

## Overview

Generator expressions are Python's syntax for creating lazy iterators inline using comprehension-like syntax. They are similar to list comprehensions but produce values on-demand rather than building a complete list in memory.

```python
# List comprehension (eager)
lst = [x * 2 for x in range(1000000)]  # Creates 1M element list

# Generator expression (lazy)
gen = (x * 2 for x in range(1000000))  # Creates iterator, no values yet
```

## Current Status

**Unsupported**: Generator expressions currently fail with "Expression type not yet supported"

```python
def test_gen_exp():
    result = (x * 2 for x in range(5))  # FAILS
    return list(result)
```

**Workaround**: Use explicit generator functions
```python
def test_gen_exp():
    def gen():
        for x in range(5):
            yield x * 2
    return list(gen())  # WORKS (34/34 generator tests passing)
```

## Design Goals

1. **Lazy Evaluation**: Generate values on-demand, not all at once
2. **Memory Efficiency**: No intermediate collections
3. **Iterator Chains**: Leverage Rust's iterator combinators
4. **Zero Cost**: Compile to efficient Rust iterators
5. **Complete Coverage**: All generator expression patterns

## Python AST Structure

```python
# Python code
gen = (x * 2 for x in range(5) if x > 1)

# AST structure
GeneratorExp(
    elt=BinOp(left=Name(id='x'), op=Mult(), right=Constant(value=2)),
    generators=[
        comprehension(
            target=Name(id='x'),
            iter=Call(func=Name(id='range'), args=[Constant(value=5)]),
            ifs=[Compare(left=Name(id='x'), ops=[Gt()], comparators=[Constant(value=1)])],
            is_async=0
        )
    ]
)
```

**Key Difference from ListComp**:
- `ListComp` → `Vec::collect()` (eager)
- `GeneratorExp` → `impl Iterator` (lazy)

## Transpilation Strategy

### Option 1: Inline Iterator Chains (Preferred)

Convert generator expressions to Rust iterator chains:

```python
# Python
gen = (x * 2 for x in range(5))
```

```rust
// Rust - Iterator chain (no struct)
let gen = (0..5).map(|x| x * 2);
```

**Pros**:
- ✅ Idiomatic Rust
- ✅ Zero overhead
- ✅ Compiler optimizations (iterator fusion)
- ✅ Minimal code generation

**Cons**:
- ❌ Complex type signatures for multiple generators
- ❌ May require `Box<dyn Iterator>` for some cases

### Option 2: Anonymous Generator Functions

Convert to anonymous generator functions (reuse existing generator infrastructure):

```python
# Python
gen = (x * 2 for x in range(5))
```

```rust
// Rust - Anonymous generator function
let gen = {
    struct __GenExpr0 { iter: std::ops::Range<i64> }
    impl Iterator for __GenExpr0 {
        type Item = i64;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|x| x * 2)
        }
    }
    __GenExpr0 { iter: 0..5 }
};
```

**Pros**:
- ✅ Reuses existing generator infrastructure
- ✅ Consistent with generator functions
- ✅ Handles complex state easily

**Cons**:
- ❌ More verbose code generation
- ❌ Requires struct per generator expression
- ❌ Less idiomatic for simple cases

### Decision: Hybrid Approach

**Simple cases** (single generator, no complex state) → **Iterator chains** (Option 1)
**Complex cases** (nested generators, complex conditions) → **Anonymous functions** (Option 2)

## Test-Driven Development Plan

### Phase 1: Basic Generator Expressions (10 tests)

```rust
// crates/depyler-core/tests/generator_expression_test.rs

#[test]
fn test_simple_generator_expression() {
    let python = r#"
def use_gen() -> list:
    gen = (x for x in range(5))
    return list(gen)
"#;
    // Should transpile and compile
    // Expected: Iterator chain (0..5)
}

#[test]
fn test_generator_expression_with_transform() {
    let python = r#"
def use_gen() -> list:
    gen = (x * 2 for x in range(5))
    return list(gen)
"#;
    // Expected: (0..5).map(|x| x * 2)
}

#[test]
fn test_generator_expression_with_filter() {
    let python = r#"
def use_gen() -> list:
    gen = (x for x in range(10) if x % 2 == 0)
    return list(gen)
"#;
    // Expected: (0..10).filter(|x| x % 2 == 0)
}

#[test]
fn test_generator_expression_map_and_filter() {
    let python = r#"
def use_gen() -> list:
    gen = (x * 2 for x in range(10) if x > 5)
    return list(gen)
"#;
    // Expected: (0..10).filter(|x| x > 5).map(|x| x * 2)
}

#[test]
fn test_generator_expression_in_sum() {
    let python = r#"
def calculate() -> int:
    return sum(x**2 for x in range(5))
"#;
    // Expected: (0..5).map(|x| x.pow(2)).sum()
}

#[test]
fn test_generator_expression_in_max() {
    let python = r#"
def find_max(nums: list) -> int:
    return max(x * 2 for x in nums)
"#;
    // Expected: nums.iter().map(|x| x * 2).max()
}

#[test]
fn test_generator_expression_with_list_source() {
    let python = r#"
def use_gen(nums: list) -> list:
    gen = (x + 1 for x in nums)
    return list(gen)
"#;
    // Expected: nums.iter().map(|x| x + 1).collect()
}

#[test]
fn test_generator_expression_string_transform() {
    let python = r#"
def use_gen(words: list) -> list:
    gen = (w.upper() for w in words)
    return list(gen)
"#;
    // Expected: words.iter().map(|w| w.to_uppercase())
}

#[test]
fn test_generator_expression_tuple_result() {
    let python = r#"
def use_gen() -> list:
    gen = ((x, x*2) for x in range(3))
    return list(gen)
"#;
    // Expected: (0..3).map(|x| (x, x * 2))
}

#[test]
fn test_generator_expression_immediate_consume() {
    let python = r#"
def calculate() -> int:
    # Generator expression consumed without assignment
    return sum(x for x in range(100))
"#;
    // Expected: (0..100).sum()
}
```

### Phase 2: Nested Generator Expressions (5 tests)

```rust
#[test]
fn test_nested_generator_expression() {
    let python = r#"
def use_gen() -> list:
    gen = (x + y for x in range(3) for y in range(3))
    return list(gen)
"#;
    // Expected: Nested iterator or flat_map
}

#[test]
fn test_nested_generator_with_condition() {
    let python = r#"
def use_gen() -> list:
    gen = ((x, y) for x in range(3) for y in range(x))
    return list(gen)
"#;
    // Expected: Conditional nested iteration
}

#[test]
fn test_nested_generator_with_filter() {
    let python = r#"
def use_gen() -> list:
    gen = (x * y for x in range(3) for y in range(3) if x != y)
    return list(gen)
"#;
    // Expected: Nested with filter
}

#[test]
fn test_generator_of_generator_expressions() {
    let python = r#"
def use_gen() -> list:
    outer = ((x, (y for y in range(x))) for x in range(3))
    return list((x, list(gen)) for x, gen in outer)
"#;
    // Expected: Nested generator expressions
}

#[test]
fn test_cartesian_product_generator() {
    let python = r#"
def use_gen(a: list, b: list) -> list:
    gen = ((x, y) for x in a for y in b)
    return list(gen)
"#;
    // Expected: Cartesian product via nested iterators
}
```

### Phase 3: Edge Cases (5 tests)

```rust
#[test]
fn test_generator_expression_with_complex_condition() {
    let python = r#"
def use_gen(nums: list) -> list:
    gen = (x for x in nums if x > 0 and x < 100)
    return list(gen)
"#;
    // Expected: Multiple filter conditions
}

#[test]
fn test_generator_expression_with_function_call() {
    let python = r#"
def double(x: int) -> int:
    return x * 2

def use_gen() -> list:
    gen = (double(x) for x in range(5))
    return list(gen)
"#;
    // Expected: Function call in map
}

#[test]
fn test_generator_expression_variable_capture() {
    let python = r#"
def use_gen(multiplier: int) -> list:
    gen = (x * multiplier for x in range(5))
    return list(gen)
"#;
    // Expected: Closure capturing multiplier
}

#[test]
fn test_generator_expression_enumerate_pattern() {
    let python = r#"
def use_gen(items: list) -> list:
    gen = ((i, item) for i, item in enumerate(items))
    return list(gen)
"#;
    // Expected: enumerate() → .enumerate()
}

#[test]
fn test_generator_expression_zip_pattern() {
    let python = r#"
def use_gen(a: list, b: list) -> list:
    gen = ((x, y) for x, y in zip(a, b))
    return list(gen)
"#;
    // Expected: zip() → .zip()
}
```

## Implementation Plan

### Step 1: Add HIR Support (Week 1, Day 1-2)

**File**: `crates/depyler-core/src/hir.rs`

```rust
pub enum HirExpr {
    // ... existing variants ...

    /// Generator expression (lazy iterator)
    GeneratorExp {
        element: Box<HirExpr>,
        generators: Vec<HirComprehension>,
        span: Span,
    },
}

pub struct HirComprehension {
    pub target: HirPattern,
    pub iter: Box<HirExpr>,
    pub conditions: Vec<HirExpr>,
    pub is_async: bool,
}
```

### Step 2: AST Bridge Conversion (Week 1, Day 2-3)

**File**: `crates/depyler-core/src/ast_bridge/converters.rs`

```rust
impl AstConverter {
    fn convert_generator_exp(
        &mut self,
        elt: &ast::Expr,
        generators: &[ast::Comprehension],
    ) -> Result<HirExpr> {
        let element = self.convert_expr(elt)?;
        let hir_generators = generators
            .iter()
            .map(|gen| self.convert_comprehension(gen))
            .collect::<Result<Vec<_>>>()?;

        Ok(HirExpr::GeneratorExp {
            element: Box::new(element),
            generators: hir_generators,
            span: self.current_span(),
        })
    }

    fn convert_comprehension(
        &mut self,
        comp: &ast::Comprehension,
    ) -> Result<HirComprehension> {
        Ok(HirComprehension {
            target: self.convert_pattern(&comp.target)?,
            iter: Box::new(self.convert_expr(&comp.iter)?),
            conditions: comp.ifs.iter()
                .map(|e| self.convert_expr(e))
                .collect::<Result<Vec<_>>>()?,
            is_async: comp.is_async != 0,
        })
    }
}
```

### Step 3: Code Generation (Week 1, Day 3-5)

**File**: `crates/depyler-core/src/codegen.rs`

```rust
impl CodeGenerator {
    fn generate_generator_expr(
        &mut self,
        element: &HirExpr,
        generators: &[HirComprehension],
    ) -> Result<TokenStream> {
        // Decision: Simple vs Complex
        if self.is_simple_generator_expr(generators) {
            self.generate_iterator_chain(element, generators)
        } else {
            self.generate_anonymous_generator(element, generators)
        }
    }

    fn is_simple_generator_expr(&self, generators: &[HirComprehension]) -> bool {
        // Single generator, no complex conditions
        generators.len() == 1 && generators[0].conditions.len() <= 1
    }

    fn generate_iterator_chain(
        &mut self,
        element: &HirExpr,
        generators: &[HirComprehension],
    ) -> Result<TokenStream> {
        let gen = &generators[0];
        let iter_expr = self.generate_expr(&gen.iter)?;
        let target = self.generate_pattern(&gen.target)?;
        let elem_expr = self.generate_expr(element)?;

        let mut chain = quote! { #iter_expr };

        // Add filters
        for condition in &gen.conditions {
            let cond_expr = self.generate_expr(condition)?;
            chain = quote! { #chain.filter(|#target| #cond_expr) };
        }

        // Add map
        chain = quote! { #chain.map(|#target| #elem_expr) };

        Ok(chain)
    }

    fn generate_anonymous_generator(
        &mut self,
        element: &HirExpr,
        generators: &[HirComprehension],
    ) -> Result<TokenStream> {
        // Generate struct + impl Iterator similar to generator functions
        // Reuse existing generator infrastructure
        todo!("Complex generator expressions")
    }
}
```

## Success Criteria

1. ✅ All 20 tests passing (10 basic + 5 nested + 5 edge cases)
2. ✅ Zero clippy warnings
3. ✅ Complexity ≤10 per function
4. ✅ Generated code compiles without errors
5. ✅ Performance competitive with hand-written iterators
6. ✅ PMAT TDG: A- grade maintained

## Performance Goals

Generator expressions should compile to efficient Rust code:

```python
# Python
result = sum(x**2 for x in range(1000000))
```

```rust
// Rust (target) - Zero overhead
let result: i64 = (0..1000000).map(|x| x.pow(2)).sum();
```

**Benchmark target**: Within 5% of hand-written Rust iterator performance.

## Implementation Timeline

**Total**: 1 week (5 days, 8 hours/day = 40 hours)

- **Day 1**: Write all 20 tests (TDD), design HIR structure (8h)
- **Day 2**: AST bridge conversion, simple cases (8h)
- **Day 3**: Code generation - iterator chains (8h)
- **Day 4**: Code generation - complex cases (8h)
- **Day 5**: Edge cases, optimization, documentation (8h)

## Dependencies

- ✅ Generators Phase 3 (DEPYLER-0115) - Complete
- ✅ Iterator trait implementation - Complete
- ✅ Comprehensions (DEPYLER-0116) - Complete (8/8 tests)

## Future Extensions (Not v3.13.0)

1. **Async generator expressions**: `(await f(x) async for x in async_iter)`
2. **Generator .send()**: Bidirectional communication
3. **Generator delegation**: `yield from gen_expr`

## References

- Python PEP 289: Generator Expressions
- Rust Iterator trait documentation
- Existing generator function implementation (DEPYLER-0115)
- Comprehensions implementation (DEPYLER-0116)
