# DEPYLER-0383: Walrus Operator (:=) Assignment Expression Not Supported

**Ticket**: DEPYLER-0383
**Severity**: P1 (HIGH - Blocks modern Python code)
**Status**: 🛑 STOP THE LINE
**Created**: 2025-11-17
**Component**: Expression Generator / HIR Converter
**Affects**: Assignment expressions (PEP 572 - Python 3.8+)

## Problem Statement

The transpiler fails with "Expression type not yet supported" when encountering the walrus operator (`:=`) used in assignment expressions. This is a Python 3.8+ feature that allows assignments within expressions.

**Common Use Cases**:
- `while chunk := f.read(8192):` - Read file in chunks
- `if match := pattern.search(text):` - Conditional with match capture
- `[y for x in data if (y := transform(x)) is not None]` - List comprehension with assignment

## Reproduction

**Input** (`test_walrus.py`):
```python
def read_file_chunks(file_path):
    with open(file_path, "rb") as f:
        while chunk := f.read(8192):
            process(chunk)
```

**Current Error**:
```
Error: Expression type not yet supported
```

**Expected Behavior**: Should transpile to Rust equivalent using loop + assignment pattern.

## Root Cause Analysis

### Location
Expression generator in HIR conversion phase does not recognize or handle the `NamedExpr` AST node type (walrus operator).

### Python AST Structure
```python
# Python code: while chunk := f.read(8192):
# AST structure:
While(
    test=NamedExpr(
        target=Name(id='chunk'),  # Variable being assigned
        value=Call(...)           # Expression being evaluated
    ),
    body=[...],
    orelse=[]
)
```

### Why It's Wrong
1. The `NamedExpr` expression type was added in Python 3.8 (PEP 572)
2. Common idiom for efficient file reading and pattern matching
3. Reduces code duplication (no need to assign twice)
4. Required for example_stdlib transpilation

### Semantic Differences: Python vs Rust

**Python (Walrus Operator)**:
```python
# Assign AND use in condition
while chunk := f.read(8192):
    process(chunk)

# Equivalent to:
chunk = f.read(8192)
while chunk:
    process(chunk)
    chunk = f.read(8192)  # Repeated assignment
```

**Rust Equivalent**:
```rust
// Option 1: loop with break
loop {
    let chunk = f.read(8192);
    if chunk.is_empty() {
        break;
    }
    process(chunk);
}

// Option 2: while let with iterator
while let Some(chunk) = read_chunk(&mut f) {
    process(chunk);
}
```

**Key Difference**: Python's walrus operator combines assignment + expression evaluation. Rust requires explicit loop + conditional.

## Impact Assessment

**Affected Code Patterns**:
1. `while chunk := f.read(size):` - Buffered file reading (most common)
2. `if match := re.search(pattern, text):` - Regex with capture
3. `if value := compute_expensive():` - Avoid recomputation
4. `[y for x in data if (y := transform(x))]` - List comprehension with assignment
5. `while line := file.readline().strip():` - Line-by-line processing

**Severity**: **P1 HIGH**
- Blocks transpilation of example_stdlib
- Python 3.8+ standard idiom (released 2019)
- Common in modern Python codebases (~20% usage in 3.8+ code)
- Required for efficient I/O operations

**Blocked Examples**:
- example_stdlib: Uses `while chunk := f.read(8192):` on line 34

## Solution Design

### Approach Options

#### Option 1: Convert to Loop + Break Pattern
Most general solution:

```python
# Python: while chunk := f.read(8192):
# Rust:
loop {
    let chunk = f.read(8192);
    if chunk.is_empty() || !chunk_is_truthy() {
        break;
    }
    // ... use chunk
}
```

**Pros**: Works for all walrus use cases
**Cons**: Verbose, need to determine truthiness condition

#### Option 2: Function-Specific Optimization
Special-case common patterns:

```python
# Python: while chunk := f.read(size):
# Rust: (optimized iterator pattern)
for chunk in std::io::Read::by_ref(&mut f)
    .bytes()
    .chunks(8192) {
    // ... use chunk
}
```

**Pros**: Idiomatic Rust, efficient
**Cons**: Only works for specific patterns

#### Option 3: Desugar to Traditional While Loop
Convert to equivalent non-walrus form first:

```python
# Python: while chunk := f.read(8192):
# Desugar to:
chunk = f.read(8192)
while chunk:
    # ... body
    chunk = f.read(8192)
```

**Pros**: Reuses existing transpilation logic
**Cons**: Code duplication, less efficient

### Recommended Approach: Option 1 (Loop + Break)

**Rationale**:
- Most general - handles all walrus use cases
- Clean transpilation to Rust `loop { if !cond { break; } }`
- Idiomatic Rust pattern
- No function-specific mappings needed

### Algorithm

**Step 1: Detect NamedExpr in While Condition**
```rust
fn convert_while(&mut self, test: &Expr, body: &[Stmt]) -> Result<syn::Stmt> {
    // Check if test is NamedExpr (walrus operator)
    if let Expr::NamedExpr { target, value } = test {
        return self.convert_while_walrus(target, value, body);
    }
    // ... normal while handling
}
```

**Step 2: Convert to Loop + Break Pattern**
```rust
fn convert_while_walrus(
    &mut self,
    target: &Expr,  // Variable being assigned
    value: &Expr,   // Expression to evaluate
    body: &[Stmt]
) -> Result<syn::Stmt> {
    let var_name = extract_variable_name(target)?;
    let value_expr = self.convert_expr(value)?;
    let body_stmts = self.convert_statements(body)?;

    // Determine truthiness check based on type
    let truthiness_check = self.get_truthiness_check(&var_name, value)?;

    Ok(parse_quote! {
        loop {
            let #var_name = #value_expr;
            if !(#truthiness_check) {
                break;
            }
            #(#body_stmts)*
        }
    })
}
```

**Step 3: Truthiness Check Logic**
```rust
fn get_truthiness_check(&self, var: &Ident, expr: &Expr) -> Result<syn::Expr> {
    // Infer type from expression
    let expr_type = self.infer_type(expr)?;

    match expr_type {
        RustType::Vec(_) | RustType::String => {
            parse_quote! { !#var.is_empty() }
        }
        RustType::Option(_) => {
            parse_quote! { #var.is_some() }
        }
        RustType::Bool => {
            parse_quote! { #var }
        }
        RustType::Int | RustType::Float => {
            parse_quote! { #var != 0 }
        }
        _ => {
            // Default: check for None/zero
            parse_quote! { #var != Default::default() }
        }
    }
}
```

### Implementation Strategy

**Phase 1: Core Infrastructure** (1-2 hours)
- Add `NamedExpr` recognition in HIR converter
- Implement basic loop + break pattern
- Handle simple `while chunk := expr:` cases

**Phase 2: Truthiness Logic** (1-2 hours)
- Implement type-based truthiness checks
- Handle Vec, String, Option, primitives
- Add tests for each type

**Phase 3: Extended Support** (2-3 hours)
- `if value := expr:` - conditional assignment
- List comprehensions with walrus
- Nested NamedExpr (edge case)

**Phase 4: Edge Cases** (1 hour)
- Multiple walrus operators in single expression
- Walrus in function arguments
- Scope handling for assigned variables

### Complexity Considerations
- Loop conversion: ≤ 15 lines, complexity ≤ 8
- Truthiness logic: ≤ 20 lines, complexity ≤ 10
- Target: TDG ≤ 2.0 (A- or better)

## Test Plan

### Test Case 1: File Reading with Walrus
**Input**:
```python
def read_chunks(file_path: str) -> None:
    with open(file_path, "rb") as f:
        while chunk := f.read(8192):
            process(chunk)
```

**Expected Rust**:
```rust
pub fn read_chunks(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open(file_path)?;
    loop {
        let mut chunk = vec![0u8; 8192];
        let n = f.read(&mut chunk)?;
        chunk.truncate(n);
        if chunk.is_empty() {
            break;
        }
        process(chunk);
    }
    Ok(())
}
```

### Test Case 2: If with Walrus
**Input**:
```python
def search_pattern(text: str, pattern: str) -> bool:
    if match := find_match(pattern, text):
        print(f"Found: {match}")
        return True
    return False
```

**Expected Rust**:
```rust
pub fn search_pattern(text: String, pattern: String) -> bool {
    let match_result = find_match(pattern, text);
    if match_result.is_some() {
        let match_val = match_result.unwrap();
        println!("Found: {}", match_val);
        return true;
    }
    false
}
```

### Test Case 3: Walrus in Comprehension
**Input**:
```python
def transform_filter(data: list) -> list:
    return [y for x in data if (y := transform(x)) is not None]
```

**Expected Rust** (may defer to future):
```rust
pub fn transform_filter(data: Vec<Value>) -> Vec<Value> {
    data.into_iter()
        .filter_map(|x| transform(x))
        .collect()
}
```

### Test Case 4: String Truthiness
**Input**:
```python
def read_lines(file_path: str) -> None:
    with open(file_path) as f:
        while line := f.readline().strip():
            print(line)
```

**Expected Rust**:
```rust
pub fn read_lines(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open(file_path)?;
    let reader = BufReader::new(f);
    loop {
        let line = reader.read_line()?.trim().to_string();
        if line.is_empty() {
            break;
        }
        println!("{}", line);
    }
    Ok(())
}
```

## Verification Steps

1. **Unit Tests**: Add tests for NamedExpr detection (≥5 tests)
2. **Integration Tests**: Test while/if with walrus (≥3 tests)
3. **Property Tests**: QuickCheck truthiness logic (1000 iterations)
4. **Compilation Test**: Verify generated Rust compiles with `rustc --deny warnings`
5. **Runtime Test**: Execute transpiled binary with sample file
6. **Regression Test**: Ensure example_stdlib transpiles after fix

## Related Issues

- **example_stdlib**: Blocked by this issue (line 34: `while chunk := f.read(8192):`)
- **PEP 572**: Python Enhancement Proposal for assignment expressions
- **Future**: Walrus in comprehensions (may be separate ticket)

## Implementation Checklist

- [ ] Add `NamedExpr` AST node type to HIR
- [ ] Implement walrus detection in while/if statements
- [ ] Create convert_while_walrus() function
- [ ] Implement type-based truthiness checks
- [ ] Handle Vec/String/Option/primitives
- [ ] Support `if value := expr:` pattern
- [ ] Add comprehensive unit tests (≥10 tests)
- [ ] Add property-based tests (QuickCheck)
- [ ] Test with example_stdlib
- [ ] Verify transpiled code compiles
- [ ] Run full test suite for regressions
- [ ] Update documentation

## Estimated Complexity

**TDG Target**: ≤ 2.0 (A- or better)
**Cyclomatic Complexity**: ≤ 10 per function
**Test Coverage**: ≥ 85%
**Implementation Time**: 6-8 hours (EXTREME TDD + multiple truthiness patterns)

## Notes

**Key Insight**: Python's walrus operator is syntactic sugar that can always be desugared to traditional assignment + loop. The challenge is generating idiomatic Rust that matches Python's semantics.

**Truthiness Complexity**: Different types have different truthiness rules in Python:
- Empty containers → False
- Zero numbers → False
- None → False
- Empty strings → False
Rust requires explicit checks for each type.

**Performance**: Rust's `loop { if !cond { break; } }` pattern is typically faster than Python's walrus operator due to:
- No dynamic dispatch
- Better loop optimization
- Compile-time type checking

**PEP 572 Rationale**: The walrus operator was controversial in Python community but is now standard in modern Python (3.8+). Supporting it is essential for transpiling contemporary Python code.
