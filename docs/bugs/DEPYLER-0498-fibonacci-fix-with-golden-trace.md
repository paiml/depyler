# DEPYLER-0498: Fibonacci.rs Fix Using Golden Trace + TypeEnvironment

**Method**: Golden Trace Validation + Five-Whys + Subtyping Constraints
**Status**: IN PROGRESS
**Golden Trace**: fibonacci_golden.json (9,293 syscalls captured)

---

## Error Summary

**Total**: 8 compilation errors
**Root Cause Category**: Type inference without subtyping constraints

| Error | Type | Line | Issue | SubtypeChecker Can Fix? |
|-------|------|------|-------|------------------------|
| 1 | E0432 | 1 | serde_json import | ❌ (dependency issue) |
| 2 | E0308 | 182 | i32 → i64 argument | ✅ (Int <: needs cast) |
| 3 | E0308 | 182 | i32 → i64 argument | ✅ (Int <: needs cast) |
| 4 | E0308 | 180 | i32 == i64 comparison | ✅ (subtyping check) |
| 5 | E0061 | 198 | Missing memo argument | ❌ (codegen issue) |
| 6 | E0277 | 202 | Vec Display trait | ✅ (use {:?}) |
| 7 | E0308 | 205 | i32 vs &Option<i32> | ✅ (borrow check) |
| 8 | E0277 | 214 | Option Display trait | ✅ (use {:?} or unwrap) |

**TypeEnvironment Can Fix**: 6/8 errors (75%)

---

## Five-Whys Analysis: Error #2-3 (i32 → i64 argument)

### Error
```rust
error[E0308]: mismatched types
   --> fibonacci.rs:182:24
182 |     (is_perfect_square(5 * num * num + 4)) || (is_perfect_square(5 * num * num - 4))
    |      ----------------- ^^^^^^^^^^^^^^^^^ expected `i64`, found `i32`
```

### Five-Whys

**Why #1**: Why does `5 * num * num + 4` produce i32 when function expects i64?
- **Answer**: `num: i32` (parameter type) × i32 literal = i32 result

**Why #2**: Why is `num` typed as i32?
- **Answer**: Python `num: int` transpiled to Rust i32 (literal size heuristic)

**Why #3**: Why does `is_perfect_square(x: i64)` expect i64?
- **Answer**: Python `x: int` on nested function transpiled to i64

**Why #4**: Why different int sizes for same Python `int` type?
- **Answer**: No subtyping constraints - each decision independent

**Why #5 (ROOT CAUSE)**: Why no subtyping?
- **Answer**: Old system used equality unification (T1 = T2), not subtyping (T1 <: T2)

### Golden Trace Evidence

```bash
$ jq '.events[] | select(.syscall_name == "write" and .args[1] | contains("True"))' fibonacci_golden.json | head -5
```

Python runtime shows:
- `is_fibonacci_number(21)` → calls `is_perfect_square()` with int values
- All int operations succeed (Python has arbitrary-precision ints)
- No type errors in Python execution

**Conclusion**: Python treats all `int` as single type, Rust needs explicit widening casts.

### SubtypeChecker Solution

Using DEPYLER-0499 SubtypeChecker:

```rust
// Check if subtyping allows implicit conversion
let checker = SubtypeChecker::new();
let arg_type = Type::Int;  // From 5 * num * num + 4
let param_type = Type::Int; // From x: i64

// In simplified Type enum, both are Type::Int
// But in reality: arg is i32, param is i64

// SubtypeChecker would say: Int <: Int (✓)
// BUT: Rust requires explicit cast for narrower → wider

// Solution: Insert cast when TypeEnvironment detects size mismatch
if inferred_size(arg) < declared_size(param) {
    insert_cast(arg, param_type);
}
```

**Fix**:
```rust
// BEFORE (transpiler output):
is_perfect_square(5 * num * num + 4)

// AFTER (with subtyping-aware cast insertion):
is_perfect_square((5 * num * num + 4) as i64)
```

---

## Five-Whys Analysis: Error #4 (i32 == i64 comparison)

### Error
```rust
error[E0308]: mismatched types
   --> fibonacci.rs:180:31
180 |         return root * root == x;
    |                -----------    ^ expected `i32`, found `i64`
```

### Five-Whys

**Why #1**: Why is `root * root` i32 but `x` is i64?
- **Answer**: `root: i32` (from `int(x ** 0.5) as i32`) but `x: i64` (parameter)

**Why #2**: Why is root i32?
- **Answer**: `int(x ** 0.5)` result assigned to i32 (heuristic: small result)

**Why #3**: Why doesn't comparison work?
- **Answer**: Rust requires both sides of `==` to have same type

**Why #4**: Why didn't transpiler insert cast?
- **Answer**: No constraint propagation from comparison operator

**Why #5 (ROOT CAUSE)**: Why no constraint propagation?
- **Answer**: Fragmented type tracking (var_types doesn't talk to expression inference)

### TypeEnvironment Solution

```rust
// TypeEnvironment tracks constraint:
env.add_constraint(TypeConstraint {
    lhs: typeof(root * root),  // i32
    rhs: typeof(x),             // i64
    kind: ConstraintKind::Eq,   // == requires equality
    reason: "Comparison in return".to_string(),
});

// SubtypeChecker says: i32 NOT <: i64 for equality (needs explicit cast)
// WorklistSolver propagates: root should be i64 OR insert cast
```

**Fix Options**:
1. Cast root to i64: `let root = ((x as f64).powf(0.5)) as i64`
2. Cast comparison: `return i64::from(root * root) == x`

**Golden Trace Validation**:
```bash
$ jq '.events[] | select(.syscall_name == "write" and (.args[1] | contains("21: True")))' fibonacci_golden.json
```
Shows: Python `is_fibonacci_number(21)` returns `True` - comparison succeeds in Python.

---

## Five-Whys Analysis: Error #7 (i32 vs &Option<i32>)

### Error
```rust
error[E0308]: mismatched types
   --> fibonacci.rs:205:41
205 |     for (i, fib) in fibonacci_generator(n).into_iter().enumerate() {
    |                     ------------------- ^ expected `&Option<i32>`, found `i32`
```

### Five-Whys

**Why #1**: Why does `fibonacci_generator` expect `&Option<i32>` when called with `i32`?
- **Answer**: Function signature: `fn fibonacci_generator(limit: &Option<i32>)`

**Why #2**: Why is parameter borrowed (`&Option`) instead of owned (`Option`)?
- **Answer**: Borrow heuristic applied to Option parameters

**Why #3**: Why is call site passing `n: i32` not `&Some(n)`?
- **Answer**: Python code: `fibonacci_generator(n)` - n is int, not Optional

**Why #4**: Why mismatch between Python Optional type and Rust call?
- **Answer**: Python signature: `limit: Optional[int]` but call site passes required int

**Why #5 (ROOT CAUSE)**: Why didn't transpiler lift `int` to `Option<int>`?
- **Answer**: No Option lifting rule (T <: Option<T>) in old type system

### SubtypeChecker Solution

```rust
// SubtypeChecker implements Option lifting:
let result = checker.check_subtype(&Type::Int, &Type::Optional(Box::new(Type::Int)));
// Returns: Ok(()) because T <: Option<T>

// TypeEnvironment inserts lift:
if param_is_option && arg_is_t {
    wrap_in_some(arg);
}
```

**Fix**:
```rust
// BEFORE:
for (i, fib) in fibonacci_generator(n).into_iter().enumerate()

// AFTER:
for (i, fib) in fibonacci_generator(&Some(n)).into_iter().enumerate()
```

**Golden Trace Validation**:
```bash
$ jq '.events[] | select(.syscall_name == "write" and (.args[1] | contains("F(")))' fibonacci_golden.json | head -3
```
Shows generator produces output - lifting succeeds in Python.

---

## Implementation Plan

### Phase 1: Update Transpiler with TypeEnvironment (RED → GREEN)

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Change 1**: Use SubtypeChecker for function arguments
```rust
fn convert_generic_call(&mut self, func: &str, args: &[HirExpr]) -> TokenStream {
    let checker = SubtypeChecker::new();

    for (arg, param_type) in args.iter().zip(function_params) {
        let arg_type = infer_expr_type(arg);

        // Check subtyping relationship
        if checker.check_subtype(&arg_type, &param_type).is_err() {
            // Not subtype - need explicit cast
            if can_cast(&arg_type, &param_type) {
                insert_cast(arg, param_type);
            }
        }
    }
}
```

**Change 2**: Use TypeEnvironment for comparison type checking
```rust
fn convert_binary(&mut self, left: &HirExpr, right: &HirExpr, op: BinOp) -> TokenStream {
    if matches!(op, BinOp::Eq | BinOp::NotEq) {
        let left_type = type_env.synthesize_type(left)?;
        let right_type = type_env.synthesize_type(right)?;

        // Equality requires same type (not just subtyping)
        if left_type != right_type {
            // Insert cast to common type
            let common = common_type(&left_type, &right_type);
            insert_cast(smaller_type_side, common);
        }
    }
}
```

### Phase 2: Fix Specific Errors

**Error #1** (E0432): Remove serde_json dependency
```rust
// Replace HashMap<serde_json::Value, serde_json::Value>
// With: HashMap<i32, i32>
```

**Error #2-3** (E0308): Insert i32 → i64 casts
```rust
is_perfect_square((5 * num * num + 4) as i64)
```

**Error #4** (E0308): Cast comparison operand
```rust
return i64::from(root * root) == x;
```

**Error #5** (E0061): Generate correct function call
```rust
fibonacci_memoized(n, None)  // Add missing argument
```

**Error #6** (E0277): Use Debug formatter
```rust
format!("\nFirst {} Fibonacci numbers: {:?}", n, fibonacci_sequence(n))
```

**Error #7** (E0308): Lift to Option
```rust
fibonacci_generator(&Some(n))
```

**Error #8** (E0277): Unwrap Option for Display
```rust
format!("\n{} is at index {} in Fibonacci sequence", target, index.unwrap())
```

### Phase 3: Validate with Golden Trace

```bash
# Re-transpile with fixes
cargo run -- transpile fibonacci.py -o fibonacci_fixed.rs

# Compile Rust
rustc --crate-type bin fibonacci_fixed.rs -o fibonacci

# Run and capture trace
renacer --format json -- ./fibonacci > fibonacci_rust.json

# Compare traces
diff <(jq -S '.events[] | select(.syscall_name == "write") | .args[1]' fibonacci_golden.json) \
     <(jq -S '.events[] | select(.syscall_name == "write") | .args[1]' fibonacci_rust.json)

# Expected: Identical output (semantic equivalence)
```

---

## Success Criteria

1. ✅ 8 errors → 0 errors (fibonacci.rs compiles)
2. ✅ Golden trace validation passes (Python ≡ Rust output)
3. ✅ TypeEnvironment + SubtypeChecker used for type decisions
4. ✅ No ad-hoc heuristics (all decisions justified by constraints)
5. ✅ Quality gates pass (complexity ≤10, TDG A-)

---

## Key Innovation: Subtyping Eliminates Heuristics

**BEFORE** (DEPYLER-0498 pre-TypeEnvironment):
```rust
// Heuristic: "cast if not builtin"
let is_builtin = matches!(func, "len" | "range" | ...);
if !is_builtin && arg_is_i32 {
    insert_cast(arg, i64);  // GUESS!
}
```

**AFTER** (with SubtypeChecker):
```rust
// Constraint-based: check subtyping relation
if !checker.check_subtype(&arg_type, &param_type).is_ok() {
    if can_cast(&arg_type, &param_type) {
        insert_cast(arg, param_type);  // JUSTIFIED!
    }
}
```

**Impact**: O(N) decisions vs O(exp) heuristic guessing.

---

**Next Action**: Implement Phase 1 changes in expr_gen.rs
