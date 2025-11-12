# DEPYLER-0364: HIR Keyword Arguments Support

**Status**: 🔴 RED - Specification Phase
**Priority**: P0 (BLOCKING - Required for DEPYLER-0363)
**Ticket**: DEPYLER-0364
**Created**: 2025-11-11
**Assignee**: Claude Code

## Problem Statement

The HIR (High-level Intermediate Representation) currently **loses keyword argument information** during AST→HIR lowering. This prevents proper transpilation of any Python code that uses keyword arguments, including:

- `argparse.ArgumentParser(description="...", epilog="...")`
- `parser.add_argument("name", nargs="+", type=Path, help="...")`
- `open(filename, mode="r", encoding="utf-8")`
- Any function call with kwargs

### Current HIR Structure

```rust
pub enum HirExpr {
    // ...
    Call {
        func: String,
        args: Vec<HirExpr>,  // ❌ Only positional args
    },
    MethodCall {
        object: Box<HirExpr>,
        method: String,
        args: Vec<HirExpr>,  // ❌ Only positional args
    },
    // ...
}
```

### Impact on DEPYLER-0363 (argparse)

**Python:**
```python
parser.add_argument("files", nargs="+", type=Path, help="Files to process")
```

**Current HIR (WRONG):**
```rust
MethodCall {
    object: Var("parser"),
    method: "add_argument",
    args: [Literal(String("files"))]  // ❌ nargs/type/help LOST
}
```

**Expected HIR (CORRECT):**
```rust
MethodCall {
    object: Var("parser"),
    method: "add_argument",
    args: [Literal(String("files"))],
    kwargs: [
        ("nargs", Literal(String("+"))),
        ("type", Var("Path")),
        ("help", Literal(String("Files to process")))
    ]
}
```

## Root Cause Analysis

### 1. AST Bridge Drops Kwargs

**File**: `crates/depyler-core/src/ast_bridge.rs`

The AST→HIR lowering in `convert_expr` only processes positional arguments:

```rust
Expr::Call(call) => {
    let args = call.args.iter()
        .map(|arg| self.convert_expr(arg))
        .collect::<Result<Vec<_>>>()?;

    // ❌ call.keywords is IGNORED

    HirExpr::Call { func, args }
}
```

### 2. Python AST Has Keywords

Python's rustpython_parser AST structure DOES include keywords:

```rust
pub struct ExprCall {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
    pub keywords: Vec<Keyword>,  // ✅ Available but unused
}

pub struct Keyword {
    pub arg: Option<Identifier>,  // None for **kwargs
    pub value: Expr,
}
```

## Solution Design

### Phase 1: Extend HIR Structure

Add `kwargs` field to Call and MethodCall variants:

```rust
pub enum HirExpr {
    Call {
        func: String,
        args: Vec<HirExpr>,
        kwargs: Vec<(String, HirExpr)>,  // ✅ NEW
    },
    MethodCall {
        object: Box<HirExpr>,
        method: String,
        args: Vec<HirExpr>,
        kwargs: Vec<(String, HirExpr)>,  // ✅ NEW
    },
    // ...
}
```

**Rationale for Vec<(String, HirExpr)>:**
- Preserves keyword argument order (important for Python semantics)
- Simple to pattern match in code generation
- Can represent both `foo=bar` and `**kwargs` (empty string for arg name)

### Phase 2: Update AST Bridge

**File**: `crates/depyler-core/src/ast_bridge.rs`

```rust
Expr::Call(call) => {
    let args = call.args.iter()
        .map(|arg| self.convert_expr(arg))
        .collect::<Result<Vec<_>>>()?;

    // ✅ NEW: Convert keywords
    let kwargs = call.keywords.iter()
        .filter_map(|kw| {
            if let Some(arg_name) = &kw.arg {
                let value = self.convert_expr(&kw.value).ok()?;
                Some((arg_name.to_string(), value))
            } else {
                // **kwargs unpacking - skip for now
                None
            }
        })
        .collect::<Vec<_>>();

    HirExpr::Call { func, args, kwargs }
}
```

### Phase 3: Update All Pattern Matches

**Impact Analysis**: Every pattern match on `HirExpr::Call` and `HirExpr::MethodCall` must be updated:

**Files to modify:**
1. `crates/depyler-core/src/rust_gen/expr_gen.rs` (primary codegen)
2. `crates/depyler-core/src/optimizer.rs` (dead code analysis)
3. `crates/depyler-core/src/borrowing_context.rs` (lifetime analysis)
4. `crates/depyler-core/src/inlining.rs` (function inlining)
5. `crates/depyler-core/src/type_hints.rs` (type inference)
6. `crates/depyler-core/src/profiling.rs` (performance analysis)
7. `crates/depyler-core/src/const_generic_inference.rs` (const generics)
8. Any tests that construct HIR expressions

**Migration Strategy:**
- Add kwargs with default empty vec: `kwargs: vec![]`
- Use `..` pattern to ignore new field during migration
- Then incrementally add kwargs handling where needed

### Phase 4: Code Generation Support

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

For method calls with kwargs, generate appropriate Rust syntax:

**Simple case (struct literal):**
```rust
// Python: Point(x=10, y=20)
// Rust: Point { x: 10, y: 20 }
```

**Builder pattern case:**
```rust
// Python: ArgumentParser(description="foo", epilog="bar")
// Rust: clap::Parser struct with attributes
```

**Named arguments case (not directly supported in Rust):**
```rust
// Python: open(filename, mode="r", encoding="utf-8")
// Rust: open_with_options(filename, OpenOptions::new().mode("r").encoding("utf-8"))
// OR: std::fs::OpenOptions::new().read(true).open(filename)
```

## Implementation Plan

### RED Phase: Write Failing Tests

**File**: `crates/depyler-core/tests/depyler_0364_hir_kwargs.rs`

```rust
#[test]
fn test_depyler_0364_call_with_kwargs() {
    let python = r#"
result = foo(10, 20, bar=30, baz="hello")
"#;
    let hir = transpile_to_hir(python);

    // Verify kwargs are preserved
    match &hir.functions[0].body[0] {
        HirStmt::Assign { value, .. } => {
            match value {
                HirExpr::Call { func, args, kwargs } => {
                    assert_eq!(func, "foo");
                    assert_eq!(args.len(), 2);
                    assert_eq!(kwargs.len(), 2);
                    assert_eq!(kwargs[0].0, "bar");
                    assert_eq!(kwargs[1].0, "baz");
                }
                _ => panic!("Expected Call"),
            }
        }
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_method_call_with_kwargs() {
    let python = r#"
parser.add_argument("name", nargs="+", type=str, help="Name")
"#;
    let hir = transpile_to_hir(python);

    // Verify kwargs are preserved
    match &hir.functions[0].body[0] {
        HirStmt::Expr(HirExpr::MethodCall { kwargs, .. }) => {
            assert_eq!(kwargs.len(), 3);
            assert_eq!(kwargs[0].0, "nargs");
            assert_eq!(kwargs[1].0, "type");
            assert_eq!(kwargs[2].0, "help");
        }
        _ => panic!("Expected MethodCall"),
    }
}

#[test]
fn test_depyler_0364_argparse_full_transpilation() {
    let python = r#"
import argparse
parser = argparse.ArgumentParser(description="Test")
parser.add_argument("files", nargs="+", type=str)
args = parser.parse_args()
"#;
    let rust = transpile(python);

    // Should extract nargs="+" and generate Vec<String>
    assert!(rust.contains("files: Vec<String>"));
}
```

### GREEN Phase: Implement HIR Changes

**Step 1**: Update HIR structure in `crates/depyler-core/src/hir.rs`

**Step 2**: Update AST bridge in `crates/depyler-core/src/ast_bridge.rs`

**Step 3**: Fix all compilation errors by adding empty kwargs fields

**Step 4**: Run tests - should now pass HIR preservation tests

### REFACTOR Phase: Code Generation

**Step 1**: Update argparse transformation to use kwargs

**Step 2**: Add type mapping for common kwargs patterns

**Step 3**: Meet quality standards (complexity ≤10, coverage ≥80%)

## Testing Strategy

### Unit Tests (10 tests)
1. Call with single kwarg
2. Call with multiple kwargs
3. MethodCall with kwargs
4. Mixed positional and keyword args
5. kwargs with complex expressions
6. kwargs with nested calls
7. Empty kwargs (backward compat)
8. argparse ArgumentParser kwargs
9. argparse add_argument kwargs
10. open() with mode/encoding kwargs

### Property Tests (3 tests)
1. Any valid Python call with kwargs transpiles without panic
2. Kwargs order is preserved in HIR
3. Transpiled code compiles (for supported patterns)

### Integration Tests (5 tests)
1. Full argparse program with all kwargs
2. File I/O with open() kwargs
3. JSON parsing with kwargs
4. Custom class instantiation with kwargs
5. Multiple functions using kwargs

## Success Criteria

✅ All 13 DEPYLER-0364 tests pass
✅ DEPYLER-0363 argparse tests increase from 5/10 to 9/10 passing
✅ No regressions in existing tests
✅ Coverage ≥85% for new code
✅ Complexity ≤10 for all functions
✅ Zero clippy warnings

## Dependencies

**Blocked By**: None
**Blocks**: DEPYLER-0363 (argparse transpilation)

## Risk Analysis

### Low Risk
- HIR structure change is additive (backward compatible with empty vec)
- AST already has keyword information
- Python semantics are well-defined

### Medium Risk
- Many files need pattern match updates (mechanical but tedious)
- Code generation for kwargs needs design decisions
- May expose bugs in dead code elimination

### Mitigation
- Use compiler to find all pattern matches that need updating
- Add `#[non_exhaustive]` to prevent future breakage
- Update CLAUDE.md with HIR change process

## Timeline Estimate

- **Phase 1 (HIR structure)**: 1 hour
- **Phase 2 (AST bridge)**: 1 hour
- **Phase 3 (Pattern matches)**: 2-3 hours
- **Phase 4 (Code generation)**: 2 hours
- **Testing & refinement**: 2 hours

**Total**: 8-9 hours of focused work

## References

- Python AST: https://docs.python.org/3/library/ast.html#ast.Call
- rustpython_parser: https://github.com/RustPython/Parser
- DEPYLER-0363: argparse transpilation (blocked by this)
- DEPYLER-0161: Dead code elimination bugs

## Notes

This is a **fundamental** improvement to HIR that will enable:
- Full argparse support
- Better file I/O transpilation
- Class initialization with named fields
- More idiomatic Rust code generation

The kwargs information is ALREADY in the Python AST - we're just not using it yet.

---

**Next Steps**: Update DEPYLER-0363 status, then begin RED phase implementation.
