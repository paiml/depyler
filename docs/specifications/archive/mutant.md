# Mutation Testing Specification for Depyler

**Version**: 1.0
**Status**: Active
**Target**: ≥90% Mutation Kill Rate
**Tool**: cargo-mutants

---

## Executive Summary

Mutation testing validates the effectiveness of Depyler's test suite by deliberately introducing bugs ("mutations") into the code and verifying that tests catch them. This specification adapts the proven pforge mutation testing methodology for Depyler's Python→Rust transpilation domain.

**Core Principle**: High test coverage (80%) means nothing if tests don't actually catch bugs. Mutation testing proves test quality.

---

## Strategic Goals

### Primary Objectives

1. **≥90% Mutation Kill Rate**: Achieve and maintain industry-leading mutation score
2. **Zero Weak Tests**: Eliminate tests without meaningful assertions
3. **Transpilation Correctness**: Validate Python→Rust conversion accuracy
4. **Regression Prevention**: Catch bugs before they reach production

### Integration with Existing Quality Gates

Mutation testing complements Depyler's quality framework:

| Metric | Current Target | Mutation Testing Impact |
|--------|---------------|------------------------|
| TDG Score | A+ (≥95/100) | Validates quality is real |
| Test Coverage | ≥80% | Proves coverage is effective |
| Complexity | ≤10 per function | Lower complexity = easier to test |
| SATD | 0 violations | No technical debt hiding test gaps |
| **Mutation Score** | **≥90%** | **Ultimate test validation** |

---

## Depyler-Specific Mutation Testing Strategy

### Critical Testing Areas

Depyler's transpilation pipeline has unique mutation testing requirements:

#### 1. AST → HIR Conversion (`depyler-core`)
**Risk**: Incorrect Python AST interpretation
**Mutation Focus**: Expression conversion, type inference, scope tracking

```rust
// Original: depyler-core/src/ast_bridge.rs
pub fn convert_expr(expr: &ast::Expr) -> Result<HirExpr> {
    match expr {
        ast::Expr::BinOp { left, op, right, .. } => {
            Ok(HirExpr::Binary {
                left: Box::new(convert_expr(left)?),
                op: convert_binop(op)?,  // Critical: operator mapping
                right: Box::new(convert_expr(right)?),
            })
        }
        _ => Err(ConversionError::UnsupportedExpr)
    }
}

// Mutations cargo-mutants will generate:
// 1. Replace convert_binop(op)? with constant operator (e.g., always Add)
// 2. Delete error handling (remove ?)
// 3. Invert match arms
```

**Required Tests to Kill These Mutations**:
```rust
#[test]
fn test_binop_conversion_all_operators() {
    // Kills mutation 1: Tests actual operator is preserved
    assert_eq!(convert_expr(py_add), hir_add);
    assert_eq!(convert_expr(py_sub), hir_sub);
    assert_eq!(convert_expr(py_mul), hir_mul);
    // ... all 12 binary operators
}

#[test]
fn test_unsupported_expr_returns_error() {
    // Kills mutation 2: Tests error handling
    let result = convert_expr(unsupported_walrus);
    assert!(matches!(result, Err(ConversionError::UnsupportedExpr)));
}
```

#### 2. Type Inference (`depyler-analyzer`)
**Risk**: Incorrect Rust type generation
**Mutation Focus**: Type resolution, ownership inference

```rust
// Original: depyler-analyzer/src/type_flow.rs
pub fn infer_type(&mut self, expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::I32,
            Literal::Float(_) => Type::F64,
            Literal::Str(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
        }
        _ => Type::Unknown
    }
}

// Mutations:
// 1. Replace Type::I32 with Type::I64
// 2. Replace Type::String with Type::Unit
// 3. Delete match arms (return Type::Unknown always)
```

**Required Tests**:
```rust
#[test]
fn test_literal_type_inference_exact_types() {
    // Kills mutations 1 & 2: Validates exact type mapping
    assert_eq!(infer_type(int_42), Type::I32);      // Not I64
    assert_eq!(infer_type(str_hello), Type::String); // Not Unit
    assert_eq!(infer_type(float_3_14), Type::F64);
    assert_eq!(infer_type(bool_true), Type::Bool);
}

#[test]
fn test_unknown_expr_returns_unknown() {
    // Kills mutation 3: Tests fallback behavior
    assert_eq!(infer_type(complex_expr), Type::Unknown);
}
```

#### 3. Code Generation (`depyler-core/codegen.rs`)
**Risk**: Generated Rust doesn't compile or has wrong semantics
**Mutation Focus**: Rust token generation, scope management

```rust
// Original: depyler-core/src/codegen.rs
fn generate_if_stmt(&self, test: &HirExpr, body: &HirBlock, orelse: &Option<HirBlock>) -> TokenStream {
    let test_tokens = self.expr_to_tokens(test);
    let body_tokens = self.block_to_tokens(body);

    if let Some(else_block) = orelse {
        let else_tokens = self.block_to_tokens(else_block);
        quote! {
            if #test_tokens {
                #body_tokens
            } else {
                #else_tokens
            }
        }
    } else {
        quote! {
            if #test_tokens {
                #body_tokens
            }
        }
    }
}

// Mutations:
// 1. Invert test: if !#test_tokens
// 2. Delete else branch handling
// 3. Swap body and else blocks
```

**Required Tests**:
```rust
#[test]
fn test_if_stmt_correct_condition_polarity() {
    // Kills mutation 1: Validates condition not inverted
    let code = generate_if_stmt(true_condition, then_block, None);
    assert!(code.to_string().contains("if true"));
    assert!(!code.to_string().contains("if !true"));
}

#[test]
fn test_if_else_both_branches_present() {
    // Kills mutation 2: Validates else branch exists
    let code = generate_if_stmt(condition, then_block, Some(else_block));
    assert!(code.to_string().contains("if"));
    assert!(code.to_string().contains("else"));
}

#[test]
fn test_if_else_correct_branch_order() {
    // Kills mutation 3: Validates then/else not swapped
    let code = generate_if_stmt(
        bool_true,
        block_with_marker("THEN"),
        Some(block_with_marker("ELSE"))
    );
    let s = code.to_string();
    let then_pos = s.find("THEN").unwrap();
    let else_pos = s.find("ELSE").unwrap();
    assert!(then_pos < else_pos, "Then must come before else");
}
```

#### 4. Lambda Conversion (`depyler/src/lib.rs`)
**Risk**: AWS Lambda generation fails or produces incorrect handler
**Mutation Focus**: Event type inference, runtime generation

```rust
// Original: depyler/src/lib.rs (after DEPYLER-0011 refactoring)
fn infer_and_map_event_type(
    inferred_type: depyler_core::lambda_inference::EventType,
) -> depyler_annotations::LambdaEventType {
    match inferred_type {
        depyler_core::lambda_inference::EventType::S3Event => {
            depyler_annotations::LambdaEventType::S3Event
        }
        depyler_core::lambda_inference::EventType::ApiGatewayEvent => {
            depyler_annotations::LambdaEventType::ApiGatewayEvent
        }
        // ... 5 more event types
    }
}

// Mutations:
// 1. Return wrong event type (e.g., always S3Event)
// 2. Swap event type mappings
```

**Required Tests** (already exist from DEPYLER-0011):
```rust
#[test]
fn test_s3_event_handler_generation() {
    // Kills mutation: Validates S3 events generate S3 handler
    let result = lambda_convert_command(s3_handler_input, ...);
    assert!(result.is_ok());

    let generated = read_generated_main_rs();
    assert!(generated.contains("aws_lambda_events::s3::S3Event"));
    assert!(!generated.contains("ApiGatewayEvent")); // Not wrong type
}

#[test]
fn test_api_gateway_event_handler_generation() {
    // Kills mutation: Validates API Gateway events generate API Gateway handler
    let result = lambda_convert_command(api_gw_handler_input, ...);

    let generated = read_generated_main_rs();
    assert!(generated.contains("ApiGatewayProxyRequest"));
    assert!(!generated.contains("S3Event")); // Not wrong type
}
```

---

## Mutation Operators Relevant to Depyler

### 1. Replace Function Return Values

**High Risk in**: Type inference, error handling, code generation

```rust
// Original
fn get_rust_type(&self, py_type: &str) -> RustType {
    match py_type {
        "int" => RustType::I32,
        "str" => RustType::String,
        _ => RustType::Unknown,
    }
}

// Mutations cargo-mutants generates:
fn get_rust_type(&self, py_type: &str) -> RustType {
    RustType::I32  // Always return I32
}

fn get_rust_type(&self, py_type: &str) -> RustType {
    RustType::Unknown  // Always return Unknown
}
```

**Kill Strategy**: Test all branches with specific assertions
```rust
#[test]
fn test_rust_type_mapping_all_types() {
    assert_eq!(get_rust_type("int"), RustType::I32);
    assert_eq!(get_rust_type("str"), RustType::String);
    assert_eq!(get_rust_type("unknown_type"), RustType::Unknown);
}
```

### 2. Negate Boolean Conditions

**High Risk in**: AST conversion, validation logic

```rust
// Original
if expr.has_type_annotation() {
    self.use_annotated_type(expr)
} else {
    self.infer_type(expr)
}

// Mutation
if !expr.has_type_annotation() {  // Inverted!
    self.use_annotated_type(expr)
} else {
    self.infer_type(expr)
}
```

**Kill Strategy**: Test both branches
```rust
#[test]
fn test_annotated_expr_uses_annotation() {
    let expr = create_annotated_expr("x: i32");
    assert_eq!(process_expr(expr), uses_annotation_path);
}

#[test]
fn test_unannotated_expr_infers_type() {
    let expr = create_unannotated_expr("x");
    assert_eq!(process_expr(expr), uses_inference_path);
}
```

### 3. Change Comparison Operators

**High Risk in**: Scope validation, boundary checks

```rust
// Original
if scope_depth > 0 {
    self.add_local_variable(name)
}

// Mutations
if scope_depth >= 0 { }  // > → >=
if scope_depth < 0 { }   // > → <
if scope_depth == 0 { }  // > → ==
```

**Kill Strategy**: Test boundary conditions
```rust
#[test]
fn test_scope_depth_boundary() {
    assert!(!should_add_local(0));   // depth == 0
    assert!(should_add_local(1));    // depth > 0
    assert!(!should_add_local(-1));  // depth < 0 (invalid)
}
```

### 4. Delete Statements

**High Risk in**: Validation, error handling, scope tracking

```rust
// Original
fn transpile_function(&mut self, func: &ast::Function) -> Result<String> {
    self.validate_function_signature(func)?;  // Validation
    self.enter_scope();                       // Scope tracking
    let body = self.transpile_body(&func.body)?;
    self.exit_scope();
    Ok(format!("fn {} {{ {} }}", func.name, body))
}

// Mutation: Delete validation
fn transpile_function(&mut self, func: &ast::Function) -> Result<String> {
    // self.validate_function_signature(func)?;  // DELETED!
    self.enter_scope();
    let body = self.transpile_body(&func.body)?;
    self.exit_scope();
    Ok(format!("fn {} {{ {} }}", func.name, body))
}
```

**Kill Strategy**: Test that invalid input fails
```rust
#[test]
fn test_invalid_function_rejected() {
    let invalid_func = create_function_without_return_type();

    let result = transpile_function(invalid_func);

    assert!(result.is_err(), "Should fail validation");
    assert!(matches!(result, Err(TranspileError::MissingReturnType)));
}
```

### 5. Replace Binary Operators

**High Risk in**: Expression conversion, optimization

```rust
// Original
let result = a + b;

// Mutations
let result = a - b;  // + → -
let result = a * b;  // + → *
let result = a / b;  // + → /
```

**Kill Strategy**: Test with specific values where operators differ
```rust
#[test]
fn test_addition_operator_correct() {
    let expr = create_binop(5, Add, 3);
    let result = evaluate_expr(expr);
    assert_eq!(result, 8);   // Not 2 (sub), 15 (mul), 1 (div)
}
```

---

## Configuration

### `.cargo/mutants.toml`

```toml
# Depyler Mutation Testing Configuration

# Timeout per mutant (5 minutes - transpiler tests can be slow)
timeout = 300

# Exclude patterns
exclude_globs = [
    "**/tests/**",           # Don't mutate test code
    "**/*_test.rs",          # Don't mutate test helpers
    "**/examples/**",        # Don't mutate examples
    "**/target/**",          # Don't mutate build artifacts
    "**/.git/**",            # Don't mutate git metadata
]

# Focus on critical crates
crates = [
    "depyler-core",          # Core transpilation logic (HIGHEST PRIORITY)
    "depyler-analyzer",      # Type inference and analysis
    "depyler",               # CLI and Lambda conversion
    "depyler-verify",        # Contract verification
]

# Skip crates with external dependencies or low risk
skip_crates = [
    "depyler-mcp",           # MCP protocol (external API)
    "depyler-ruchy",         # Ruchy interpreter (separate project)
]

# Additional test arguments
test_args = ["--release"]  # Faster test execution

# Parallel execution (use all cores)
test_threads = 0  # 0 = auto-detect CPU count
```

### Running Mutation Tests

```bash
# Full mutation test suite
make mutants

# Or manually with all cores
cargo mutants --test-threads=0

# Run on specific crate (faster iteration)
cargo mutants -p depyler-core

# Run on specific file (EXTREME TDD workflow)
cargo mutants --file crates/depyler-core/src/codegen.rs

# Show what would be mutated without running tests
cargo mutants --list

# Generate mutation report
cargo mutants --json > mutation-report.json
```

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/mutation-testing.yml
name: Mutation Testing

on:
  # Run on PRs touching core transpilation logic
  pull_request:
    paths:
      - 'crates/depyler-core/**'
      - 'crates/depyler-analyzer/**'
      - 'crates/depyler/src/**'

  # Weekly full run
  schedule:
    - cron: '0 0 * * 0'  # Sunday midnight UTC

  # Manual trigger
  workflow_dispatch:

jobs:
  mutation-test-core:
    runs-on: ubuntu-latest
    timeout-minutes: 90  # Mutation testing is slow

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-mutants
        run: cargo install cargo-mutants --locked

      - name: Run mutation tests on depyler-core
        run: |
          cargo mutants -p depyler-core \
            --test-threads=4 \
            --timeout=300 \
            --json > mutation-core.json

      - name: Check mutation score (≥90% required)
        run: |
          SCORE=$(jq '.mutation_score' mutation-core.json)
          echo "Mutation score: $SCORE%"

          if (( $(echo "$SCORE < 90" | bc -l) )); then
            echo "❌ FAILED: Mutation score $SCORE% below 90% target"
            exit 1
          fi

          echo "✅ PASSED: Mutation score $SCORE% meets 90% target"

      - name: Upload mutation report
        uses: actions/upload-artifact@v4
        with:
          name: mutation-report-core
          path: mutation-core.json

  mutation-test-analyzer:
    runs-on: ubuntu-latest
    timeout-minutes: 60

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-mutants
        run: cargo install cargo-mutants --locked

      - name: Run mutation tests on depyler-analyzer
        run: |
          cargo mutants -p depyler-analyzer \
            --test-threads=4 \
            --timeout=300 \
            --json > mutation-analyzer.json

      - name: Check mutation score
        run: |
          SCORE=$(jq '.mutation_score' mutation-analyzer.json)
          echo "Mutation score: $SCORE%"

          if (( $(echo "$SCORE < 90" | bc -l) )); then
            echo "❌ FAILED: Mutation score $SCORE% below 90% target"
            exit 1
          fi

      - name: Upload mutation report
        uses: actions/upload-artifact@v4
        with:
          name: mutation-report-analyzer
          path: mutation-analyzer.json
```

---

## EXTREME TDD Integration

Mutation testing fits perfectly into Depyler's EXTREME TDD workflow:

### Development Flow with Mutation Testing

```bash
# STEP 1: Baseline (before starting new feature/refactoring)
pmat tdg . --min-grade A- --format=table
cargo mutants --list --file src/target_file.rs  # See what will be tested

# STEP 2: Write comprehensive tests FIRST
cargo test new_feature_tests -- --nocapture  # Should fail

# STEP 3: Implement minimal code to pass tests
# ... write code ...

# STEP 4: Verify tests pass
cargo test new_feature_tests -- --nocapture  # Should pass

# STEP 5: Run mutation testing on new code
cargo mutants --file src/target_file.rs

# STEP 6: If mutations survive, strengthen tests
# Example surviving mutation:
#   src/codegen.rs:123: replace foo() -> i32 with 0
#   SURVIVED - tests didn't catch this!

# Add test to kill mutation:
#[test]
fn test_foo_returns_nonzero() {
    assert_eq!(foo(), 42);  // Not 0
}

# STEP 7: Re-run mutations until ≥90% killed
cargo mutants --file src/target_file.rs

# STEP 8: Full quality gate
pmat tdg . --min-grade A- --fail-on-violation
pmat quality-gate --fail-on-violation
cargo llvm-cov report --fail-under-lines 80
```

### Pre-commit Hook Enhancement

Add mutation testing to high-risk files:

```bash
#!/bin/bash
# scripts/pre-commit (enhanced with mutation testing)

# ... existing quality gates ...

# Mutation testing on critical files (if changed)
CRITICAL_FILES=(
    "crates/depyler-core/src/codegen.rs"
    "crates/depyler-core/src/ast_bridge.rs"
    "crates/depyler-analyzer/src/type_flow.rs"
)

for file in $(git diff --cached --name-only); do
    for critical in "${CRITICAL_FILES[@]}"; do
        if [[ "$file" == "$critical" ]]; then
            echo "🧬 Critical file changed: $file - Running mutation tests..."

            cargo mutants --file "$file" --timeout=60 || {
                echo "❌ BLOCKED: Mutations survived in critical file"
                echo "Run: cargo mutants --file $file"
                echo "Add tests to kill surviving mutations"
                exit 1
            }
        fi
    done
done
```

---

## Performance Optimization

Mutation testing is computationally expensive. Optimize:

### 1. Parallel Execution

```bash
# Use all CPU cores
cargo mutants --test-threads=0  # Auto-detect

# Or specify explicitly
cargo mutants --test-threads=16
```

### 2. Incremental Testing

```bash
# Only test changed files in PR
git diff --name-only main...HEAD | grep '.rs$' | while read file; do
    cargo mutants --file "$file"
done

# Or specific crate
cargo mutants -p depyler-core
```

### 3. Baseline Filtering

```bash
# Skip low-value mutations
cargo mutants \
    --exclude-globs '**/tests/**' \
    --exclude-globs '**/*_generated.rs'
```

### 4. Timeout Tuning

```bash
# Shorter timeout for fast tests (unit tests)
cargo mutants --timeout=30 -p depyler-core

# Longer timeout for integration tests
cargo mutants --timeout=300 -p depyler
```

### 5. Caching Strategy

```bash
# Use sccache for faster compilation
export RUSTC_WRAPPER=sccache
cargo mutants --test-threads=8

# Check cache stats
sccache --show-stats
```

---

## Acceptable Mutations (Exceptions)

Some mutations are acceptable to miss (90% target, not 100%):

### 1. Logging Statements

```rust
// Original
fn transpile(&self, input: &str) -> Result<String> {
    log::debug!("Transpiling {} bytes", input.len());
    // ... actual logic
}

// Mutation: Delete log statement
fn transpile(&self, input: &str) -> Result<String> {
    // log::debug!("Transpiling {} bytes", input.len());  // DELETED
    // ... actual logic
}
```

**Acceptable**: Tests shouldn't depend on logging.

### 2. Performance Optimizations

```rust
// Original
fn get_cached_type(&self, key: &str) -> Type {
    self.type_cache.get(key)
        .cloned()
        .unwrap_or_else(|| self.compute_type(key))
}

// Mutation: Remove caching
fn get_cached_type(&self, key: &str) -> Type {
    self.compute_type(key)  // Always compute
}
```

**Acceptable**: Result is same, just slower. Performance tests would catch this, not unit tests.

### 3. Error Messages

```rust
// Original
return Err(TranspileError::UnsupportedFeature(
    format!("Walrus operator := not supported in Python {}", version)
));

// Mutation
return Err(TranspileError::UnsupportedFeature(String::new()));
```

**Acceptable if**: Tests only check error variant, not message content.

### 4. Debug Assertions

```rust
// Original
debug_assert!(scope_depth >= 0, "Scope depth cannot be negative");

// Mutation: Delete debug_assert
// (no assertion)
```

**Acceptable**: Debug assertions are development aids, not production logic.

---

## Mutation Testing Best Practices

### 1. Run Regularly, Not Every Commit

```bash
# Local development: Manual runs on changed files
cargo mutants --file src/my_changes.rs

# CI: Weekly full runs
schedule:
  - cron: '0 0 * * 0'  # Sunday

# CI: PR runs on critical crates only
paths:
  - 'crates/depyler-core/**'
```

### 2. Focus on Critical Code

Prioritize mutation testing on high-risk areas:

**Critical Priority**:
- `depyler-core/src/codegen.rs` - Code generation (bugs ship to users)
- `depyler-core/src/ast_bridge.rs` - AST conversion (correctness critical)
- `depyler-analyzer/src/type_flow.rs` - Type inference (safety critical)

**Medium Priority**:
- `depyler/src/lib.rs` - CLI and Lambda conversion
- `depyler-verify/src/contracts.rs` - Contract verification

**Low Priority**:
- `depyler-mcp/src/server.rs` - MCP protocol (external API)
- `depyler-ruchy/src/interpreter.rs` - Ruchy runtime (separate project)

### 3. Track Metrics Over Time

```bash
# Save mutation scores
cargo mutants --json > reports/mutation-$(date +%Y%m%d).json

# Compare over time
jq -r '.mutation_score' reports/mutation-*.json |
    awk '{sum+=$1; count++} END {print "Avg:", sum/count "%"}'
```

### 4. Don't Aim for 100%

90% is excellent. Diminishing returns above that:

- **90%**: ✅ Excellent test quality (TARGET)
- **95%**: ⚠️ Very good, but significant effort
- **100%**: ❌ Not worth the effort (acceptable mutations exist)

### 5. Use with Other Metrics

Mutation testing validates but doesn't replace other metrics:

```bash
# Comprehensive quality check
make quality-gate  # Runs all checks:
  # - TDG ≥A- (95/100)
  # - Coverage ≥80%
  # - Complexity ≤10
  # - SATD = 0
  # - Mutation score ≥90%
```

---

## Interpreting Results

### Example Mutation Run Output

```
Testing mutants in depyler-core:

crates/depyler-core/src/codegen.rs:142:5: replace generate_function -> String with String::new()
    CAUGHT in 0.3s (test_function_generation_nonempty)

crates/depyler-core/src/codegen.rs:156:9: replace if condition with true
    CAUGHT in 0.2s (test_if_stmt_both_branches)

crates/depyler-core/src/ast_bridge.rs:89:20: replace convert_binop -> BinOp with BinOp::Add
    MISSED in 0.4s

crates/depyler-core/src/type_flow.rs:234:12: replace infer_type -> Type with Type::Unknown
    CAUGHT in 0.1s (test_type_inference_exact_types)

Summary:
  Tested:  47 mutants
  Caught:  43 mutants (91.5%) ✅
  Missed:   3 mutants (6.4%)
  Timeout:  1 mutant  (2.1%)
```

### Result Interpretation

- **CAUGHT**: ✅ Test suite detected the mutation (good!)
- **MISSED**: ❌ Test suite didn't detect mutation (add test!)
- **TIMEOUT**: ⚠️ Test took too long (possibly infinite loop - add timeout test)
- **UNVIABLE**: Mutation wouldn't compile (ignored, not counted)

### Addressing Missed Mutations

**Example**: `replace convert_binop -> BinOp with BinOp::Add` MISSED

**Root Cause**: Tests don't verify all operators are correctly mapped

**Fix**: Add comprehensive operator test
```rust
#[test]
fn test_binop_conversion_all_operators() {
    assert_eq!(convert_binop(ast::Add), BinOp::Add);
    assert_eq!(convert_binop(ast::Sub), BinOp::Sub);
    assert_eq!(convert_binop(ast::Mul), BinOp::Mul);
    assert_eq!(convert_binop(ast::Div), BinOp::Div);
    // ... all operators
}
```

**Re-run**: `cargo mutants --file crates/depyler-core/src/ast_bridge.rs`

**Expected**: CAUGHT (mutation score improves)

---

## Makefile Integration

Add mutation testing to project Makefile:

```makefile
# Makefile (add to existing file)

.PHONY: mutants mutants-core mutants-quick mutants-report

# Full mutation testing suite (slow - use for releases)
mutants:
	@echo "🧬 Running comprehensive mutation tests..."
	cargo mutants --test-threads=0 --json > mutation-report.json
	@echo "📊 Mutation score:"
	@jq -r '.mutation_score' mutation-report.json

# Core crates only (faster - use for regular development)
mutants-core:
	@echo "🧬 Running mutation tests on core crates..."
	cargo mutants -p depyler-core -p depyler-analyzer --test-threads=0

# Quick check on changed files (use during TDD)
mutants-quick:
	@echo "🧬 Running mutation tests on changed files..."
	@for file in $$(git diff --name-only main...HEAD | grep '.rs$$'); do \
		echo "Testing $$file..."; \
		cargo mutants --file "$$file" --timeout=60; \
	done

# Generate HTML report
mutants-report:
	cargo mutants --json > mutation-report.json
	@echo "📊 Mutation Testing Report" > mutation-report.md
	@echo "=========================" >> mutation-report.md
	@echo "" >> mutation-report.md
	@jq -r '"Mutation Score: " + (.mutation_score | tostring) + "%"' mutation-report.json >> mutation-report.md
	@jq -r '"Tested: " + (.tested | tostring) + " mutants"' mutation-report.json >> mutation-report.md
	@jq -r '"Caught: " + (.caught | tostring) + " mutants"' mutation-report.json >> mutation-report.md
	@jq -r '"Missed: " + (.missed | tostring) + " mutants"' mutation-report.json >> mutation-report.md
	@echo "" >> mutation-report.md
	@echo "## Missed Mutations" >> mutation-report.md
	@jq -r '.missed_mutations[] | "- " + .file + ":" + (.line | tostring) + " - " + .description' mutation-report.json >> mutation-report.md
```

Usage:
```bash
make mutants           # Full run (slow, use for releases)
make mutants-core      # Core crates only (regular development)
make mutants-quick     # Changed files only (TDD workflow)
make mutants-report    # Generate readable report
```

---

## Quality Metrics Dashboard

Mutation testing integrates into Depyler's quality dashboard:

```
Depyler Quality Metrics (v3.2.0)
================================

Code Quality:
  TDG Score:        99.1/100 (A+)    ✅ Target: ≥95
  Max Complexity:   20               🟡 Target: ≤10
  SATD Violations:  0                ✅ Target: 0
  Clippy Warnings:  0                ✅ Target: 0

Testing:
  Test Count:       596+             ✅ Growing
  Coverage:         70.16%           🟡 Target: 80%
  Mutation Score:   TBD              🎯 Target: 90%

Transpilation Correctness:
  Examples Pass:    100%             ✅ All transpile
  Generated Code:   Compiles         ✅ cargo check passes
  Property Tests:   Passing          ✅ Verification active
```

---

## Roadmap Integration

Add mutation testing tasks to `docs/execution/roadmap.md`:

### Sprint 5 (Q4 2025): Mutation Testing Implementation

**DEPYLER-0020**: Mutation Testing Infrastructure
- Priority: High
- Complexity: Medium
- Time: 8-12h
- Tasks:
  - [ ] Install cargo-mutants
  - [ ] Create .cargo/mutants.toml configuration
  - [ ] Add mutation testing to CI/CD
  - [ ] Integrate with pre-commit hooks (critical files only)
  - [ ] Update Makefile with mutation testing targets

**DEPYLER-0021**: Achieve 90% Mutation Score - Core Transpilation
- Priority: Critical
- Complexity: High
- Time: 16-24h (EXTREME TDD)
- Dependencies: DEPYLER-0020
- Tasks:
  - [ ] Baseline: Run cargo-mutants on depyler-core
  - [ ] Identify all missed mutations
  - [ ] Write tests to kill missed mutations (TDD)
  - [ ] Achieve ≥90% kill rate on depyler-core
  - [ ] Document remaining acceptable mutations

**DEPYLER-0022**: Achieve 90% Mutation Score - Type Analysis
- Priority: High
- Complexity: Medium
- Time: 8-12h (EXTREME TDD)
- Dependencies: DEPYLER-0020
- Tasks:
  - [ ] Baseline: Run cargo-mutants on depyler-analyzer
  - [ ] Write tests to kill missed mutations
  - [ ] Achieve ≥90% kill rate on depyler-analyzer

**DEPYLER-0023**: Mutation Testing Documentation
- Priority: Medium
- Complexity: Low
- Time: 2-4h
- Dependencies: DEPYLER-0021, DEPYLER-0022
- Tasks:
  - [ ] Update developer guide with mutation testing workflow
  - [ ] Document acceptable mutation patterns
  - [ ] Create mutation testing troubleshooting guide
  - [ ] Add mutation metrics to quality dashboard

---

## Success Criteria

Mutation testing implementation is complete when:

- [x] Specification created (this document)
- [ ] cargo-mutants installed and configured
- [ ] CI/CD integration active
- [ ] ≥90% mutation score achieved on depyler-core
- [ ] ≥90% mutation score achieved on depyler-analyzer
- [ ] Pre-commit hooks include mutation testing on critical files
- [ ] Documentation updated
- [ ] Baseline metrics tracked

---

## References

- **pforge Mutation Testing**: `../pforge/pforge-book/src/ch09-04-mutation-testing.md`
- **cargo-mutants Documentation**: https://mutants.rs/
- **PIT Mutation Testing** (Java): https://pitest.org/
- **Depyler Quality Gates**: `CLAUDE.md`
- **EXTREME TDD Methodology**: `docs/execution/SPRINT-4-COMPLETION.md`

---

**Document Status**: ACTIVE
**Last Updated**: 2025-10-03
**Next Review**: After DEPYLER-0021 completion
**Owner**: Depyler Project
