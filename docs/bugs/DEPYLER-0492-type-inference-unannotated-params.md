# DEPYLER-0492: Type Inference for Unannotated Parameters

**Status**: 🔴 IN PROGRESS
**Priority**: P0 (HIGHEST IMPACT)
**Affects**: 6/13 examples (example_subprocess and likely others)
**Session**: 2025-11-23

## Problem Statement

Function parameters without type annotations default to `serde_json::Value` instead of being inferred from usage, causing widespread compilation failures.

### Example Error (example_subprocess)

**Python Source** (task_runner.py:21):
```python
def run_command(cmd, capture=False, check=False, cwd=None):
    # cmd used as list: cmd[0], cmd[1..]
    result = subprocess.run(cmd, ...)
```

**Generated Rust** (task_runner.rs:29-34):
```rust
pub fn run_command(
    cmd: &serde_json::Value,  // ❌ Should be &[String]
    capture: bool,             // ✅ Correct
    check: serde_json::Value,  // ❌ Should be bool
    cwd: Option<String>,       // ✅ Correct
) -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    //  ❌ Should return (i32, String, String)
    let cmd_list = cmd;
    let mut cmd = std::process::Command::new(&cmd_list[0]);  // ← ERROR HERE
    cmd.args(&cmd_list[1..]);  // ← AND HERE
}
```

**Compilation Errors**:
```
error[E0277]: the trait bound `serde_json::Value: AsRef<std::ffi::OsStr>` is not satisfied
  --> task_runner.rs:39:55
39 |             let mut cmd = std::process::Command::new(&cmd_list[0]);
   |                                                       ^^^^^^^^^^^ the trait `AsRef<std::ffi::OsStr>` is not implemented for `serde_json::Value`

error[E0277]: the trait bound `std::ops::RangeFrom<{integer}>: serde_json::value::Index` is not satisfied
  --> task_runner.rs:40:32
40 |             cmd.args(&cmd_list[1..]);
   |                                ^^^ the trait `serde_json::value::Index` is not implemented for `std::ops::RangeFrom<{integer}>`
```

---

## Root Cause Analysis (Five Whys)

### Five Whys

**1. Why does `cmd` have type `serde_json::Value`?**
→ Because `param.ty` in HIR is `Type::Unknown`, which maps to `serde_json::Value`

**2. Why is `param.ty` set to `Type::Unknown`?**
→ Because there's no type annotation in Python source, and no type inference happens

**3. Why doesn't type inference happen?**
→ The Hindley-Milner type system exists but ISN'T INTEGRATED into the HIR construction

**4. Why isn't it integrated?**
→ Type inference was implemented (DEPYLER-0451, 0455) but not connected to AST→HIR conversion

**5. ROOT CAUSE: Type inference system exists but is never invoked during transpilation pipeline**

### Call Stack Analysis

**Flow through codebase**:

1. **AST → HIR** (ast_bridge/converters.rs:1063-1067):
```rust
fn convert_nested_function_params(args: &ast::Arguments) -> Result<Vec<HirParam>> {
    for (i, arg) in args.args.iter().enumerate() {
        let ty = if let Some(annotation) = &arg.def.annotation {
            TypeExtractor::extract_type(annotation)?
        } else {
            Type::Unknown  // ← NO INFERENCE HAPPENS HERE
        };
        // ...
    }
}
```

2. **HIR → Lifetime Analysis** (lifetime_analysis.rs:581):
```rust
fn infer_parameter_lifetimes(...) -> IndexMap<String, InferredParam> {
    for param in &func.params {
        let rust_type = type_mapper.map_type(&param.ty);  // ← param.ty is Unknown
        // ...
    }
}
```

3. **Type Mapping** (type_mapper.rs:124):
```rust
pub fn map_type(&self, py_type: &PythonType) -> RustType {
    match py_type {
        PythonType::Unknown => RustType::Custom("serde_json::Value".to_string()),  // ← DEFAULT
        // ...
    }
}
```

4. **Rust Code Generation** (rust_gen/func_gen.rs:434-440):
```rust
let ty = apply_param_borrowing_strategy(
    &param.name,
    &actual_rust_type,  // ← RustType::Custom("serde_json::Value")
    &inferred_with_mut,
    lifetime_result,
    ctx,
)?;
```

### What SHOULD Happen

**Expected constraints from usage**:

1. **List construction at call site** (task_runner.py:73):
```python
cmd = [args.command] + args.args  # List[str]
run_command(cmd, ...)  # ← Should constrain cmd: List[str]
```

2. **Indexing operations** (task_runner.py:37):
```python
result = subprocess.run(cmd, ...)
# Inside subprocess.run:
#   command = cmd[0]      # ← Requires Index<usize>
#   args = cmd[1..]       # ← Requires slicing
```

3. **Stdlib function signature** (subprocess.run):
```python
def run(cmd: List[str], ...) -> CompletedProcess:
    # ← Should propagate List[str] constraint backward
```

**Hindley-Milner should collect**:
- Constraint 1: `cmd == List[T]` (from indexing `cmd[0]`)
- Constraint 2: `T == str` (from subprocess.run signature)
- Constraint 3: `capture == bool` (from default value `False`)
- Constraint 4: `check == bool` (from default value `False`)
- Unification: `cmd: List[str]`

---

## Evidence

### Hindley-Milner System Exists But Unused

**Implementation exists** (type_system/hindley_milner.rs:78-89):
```rust
pub struct TypeConstraintSolver {
    constraints: Vec<Constraint>,
    substitutions: HashMap<VarId, Type>,
    next_type_var: VarId,
}

impl TypeConstraintSolver {
    pub fn new() -> Self { ... }
    pub fn add_constraint(&mut self, constraint: Constraint) { ... }
    pub fn solve(&mut self) -> Result<HashMap<VarId, Type>, TypeError> { ... }
}
```

**But NEVER invoked**:
```bash
$ grep -r "TypeConstraintSolver" crates/depyler-core/src --exclude-dir=type_system
# NO RESULTS - Only referenced in type_system module itself
```

### Golden Trace Analysis

**Python execution** (captured with Renacer):
```bash
$ cd /home/noah/src/reprorusted-python-cli/examples/example_subprocess
$ renacer -c -- python task_runner.py echo "hello"
# Shows: execve("echo", ["echo", "hello"], ...) - string array
```

**Insight**: Python runtime uses string array for subprocess, confirming `cmd: List[str]`.

---

## Impact Assessment

**Affected Examples** (estimated 6/13):
1. ✅ example_subprocess - `cmd: Value` should be `Vec<String>`
2. ❓ example_io_streams - Likely has similar issues
3. ❓ example_stdlib - Likely has similar issues
4. ❓ example_csv_filter - Likely has similar issues
5. ❓ example_log_analyzer - Likely has similar issues
6. ❓ example_regex - Likely has similar issues

**Severity**: HIGHEST - This is a systemic type inference gap affecting ALL unannotated parameters.

---

## Solution Options

### Option 1: Integrate Hindley-Milner (CORRECT - Systematic)

**Approach**: Connect type inference to AST→HIR conversion

**Implementation**:
1. Collect constraints during HIR construction:
   - Indexing: `expr[i]` → `expr: List[T]` or `expr: Dict[K,V]`
   - Function calls: Propagate constraints from stdlib signatures
   - List construction: `[a] + b` → `result: List[T]`
2. Invoke `TypeConstraintSolver::solve()` after HIR construction
3. Update `HirParam.ty` with solved types
4. Proceed to code generation

**Complexity**: Medium (2-4 hours)
**Impact**: Fixes ALL examples with unannotated parameters

**Files to modify**:
- `ast_bridge/converters.rs` - Collect constraints during conversion
- `lib.rs` or `rust_gen.rs` - Invoke solver before codegen
- `type_system/hindley_milner.rs` - Add constraint collection helpers

### Option 2: Ad-hoc List Inference (WRONG - Not Systematic)

**Approach**: Detect list operations and hardcode List type

**Issues**:
- ❌ Doesn't solve Dict, Tuple, or other types
- ❌ Doesn't propagate constraints from function calls
- ❌ Creates technical debt
- ❌ Violates "fix transpiler systematically" principle

**NOT RECOMMENDED**

### Option 3: Require Type Annotations (WRONG - User Burden)

**Approach**: Force users to add type hints to Python code

**Issues**:
- ❌ Breaks compatibility with existing Python code
- ❌ User shouldn't need to change code for transpilation
- ❌ Python community doesn't universally use type hints

**NOT RECOMMENDED**

---

## Recommended Solution: Option 1 (Hindley-Milner Integration)

### Phase 1: Constraint Collection

**Add to ast_bridge/converters.rs**:
```rust
pub struct ConstraintCollector {
    constraints: Vec<Constraint>,
    next_var: VarId,
}

impl ConstraintCollector {
    pub fn collect_function_constraints(func: &HirFunction) -> Vec<Constraint> {
        let mut collector = Self::new();
        
        // Collect from function body
        for stmt in &func.body {
            collector.visit_stmt(stmt);
        }
        
        collector.constraints
    }
    
    fn visit_expr(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Index { value, index } => {
                // value[index] → value must be indexable (List, Dict, etc.)
                self.constraints.push(Constraint::Indexable(value.type_id()));
            }
            HirExpr::Call { func, args } => {
                // Propagate constraints from stdlib function signatures
                if let Some(sig) = self.get_stdlib_signature(func) {
                    for (arg, param_ty) in args.iter().zip(&sig.params) {
                        self.constraints.push(Constraint::Equality(
                            arg.type_id(),
                            param_ty.clone()
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}
```

### Phase 2: Solver Integration

**Add to lib.rs or rust_gen.rs**:
```rust
pub fn transpile_with_inference(module: ast::Module) -> Result<String> {
    // 1. AST → HIR (parameters default to UnificationVar)
    let hir = ast_bridge::convert_module(module)?;
    
    // 2. Collect type constraints
    let mut solver = TypeConstraintSolver::new();
    for func in &hir.functions {
        let constraints = ConstraintCollector::collect_function_constraints(func);
        for c in constraints {
            solver.add_constraint(c);
        }
    }
    
    // 3. Solve constraints
    let solution = solver.solve()?;
    
    // 4. Update HIR with solved types
    let hir_typed = apply_solution(hir, solution)?;
    
    // 5. Generate Rust code
    let rust_code = rust_gen::generate(hir_typed)?;
    
    Ok(rust_code)
}
```

### Phase 3: Testing

**Add test** (tests/type_inference_test.rs):
```rust
#[test]
fn test_subprocess_cmd_type_inference() {
    let python = r#"
def run_command(cmd, capture=False):
    result = subprocess.run(cmd, capture_output=capture)
    return result.returncode
"#;
    
    let rust = transpile(python).unwrap();
    
    // Should infer cmd: Vec<String>, capture: bool
    assert!(rust.contains("cmd: Vec<String>"));
    assert!(rust.contains("capture: bool"));
    assert!(!rust.contains("serde_json::Value"));
}
```

---

## Success Criteria

- [ ] Hindley-Milner solver integrated into transpilation pipeline
- [ ] Constraints collected from indexing, slicing, function calls
- [ ] example_subprocess compiles without errors
- [ ] Parameter types: `cmd: Vec<String>`, `check: bool` (not Value)
- [ ] Test coverage: 5+ property tests for constraint collection
- [ ] No regression in existing examples
- [ ] Performance: <50ms overhead for type inference

---

## Next Steps

1. **Create failing test** - test_subprocess_cmd_type_inference() (TDD)
2. **Implement constraint collection** - ast_bridge/converters.rs
3. **Integrate solver** - lib.rs transpilation pipeline
4. **Re-transpile example_subprocess** - Verify compilation
5. **Run golden trace validation** - Semantic equivalence check
6. **Measure impact** - Check if other failing examples now compile

---

## Related Issues

- DEPYLER-0451: Hindley-Milner type system implementation (DONE)
- DEPYLER-0455: Type system bugs (DONE)
- DEPYLER-0478: Result type inference (DONE)
- DEPYLER-0479: Type conversion analysis (DONE)

**Key Difference**: Those fixes addressed type *mapping*, this addresses type *inference* (collecting constraints from usage).

---

**Session Summary**: Root cause identified via Golden Tracing + Five Whys analysis. Solution path clear: integrate existing Hindley-Milner system into HIR construction pipeline.
