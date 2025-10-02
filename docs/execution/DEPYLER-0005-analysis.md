# DEPYLER-0005: Refactor expr_to_rust_tokens Function Analysis

**Ticket**: DEPYLER-0005
**Priority**: P0 - CRITICAL
**Sprint**: Sprint 2 - Critical Complexity Reduction
**Estimated**: 60-80 hours
**Status**: In Progress
**Date**: 2025-10-02

---

## 🎯 **Objective**

Refactor `expr_to_rust_tokens` function from cyclomatic complexity 39 to ≤10 using Extract Method pattern while maintaining all existing functionality and test coverage.

---

## 📊 **Current State**

**Location**: `crates/depyler-core/src/codegen.rs:427-673`
**Lines**: 246
**Cyclomatic Complexity**: 39
**Cognitive Complexity**: Unknown (likely high)
**Current Tests**: Likely existing integration tests
**Dependencies**: HirExpr enum, proc_macro2, quote

---

## 🔍 **Function Structure Analysis**

The function handles **19 HirExpr variants** via a large match statement:

### 1. **Simple Expressions** (Lines 429-433)
- `Literal`: Delegates to `literal_to_rust_tokens`
- `Var`: Creates identifier
**Complexity**: 2

### 2. **Binary Operations** (Lines 434-462)
- Regular binary operators
- Special case: `Sub` with `is_len_call` → `saturating_sub`
- Special case: `FloorDiv` → Python floor division semantics
**Complexity**: ~6-7 (nested match + special cases)

### 3. **Unary Operations** (Lines 463-467)
- Delegates to `unaryop_to_rust_tokens`
**Complexity**: 1

### 4. **Function Call** (Lines 468-475)
- Converts function name and arguments
**Complexity**: 1

### 5. **Index Access** (Lines 476-480)
- Simple array indexing
**Complexity**: 1

### 6. **Collection Literals** (Lines 481-509)
- `List`: vec! macro
- `Dict`: HashMap with insert loop
- `Tuple`: tuple syntax
**Complexity**: 3

### 7. **Attribute Access** (Lines 510-514)
- Member field access
**Complexity**: 1

### 8. **Borrow Expressions** (Lines 515-522)
- Mutable vs immutable borrow
**Complexity**: 2 (if-else)

### 9. **Method Calls** (Lines 523-535)
- Object method invocation
**Complexity**: 1

### 10. **Slice Operations** (Lines 536-564)
- Pattern matching on (start, stop, step):
  - `(None, None, None)`: clone
  - `(Some, Some, None)`: range slice
  - `(Some, None, None)`: start slice
  - `(None, Some, None)`: end slice
  - `_`: complex fallback
**Complexity**: 5 (match arms)

### 11. **List Comprehensions** (Lines 565-594)
- With condition: filter + map + collect
- Without condition: map + collect
**Complexity**: 2 (if-else)

### 12. **Lambda Expressions** (Lines 595-611)
- With params: closure with params
- Without params: nullary closure
**Complexity**: 2 (if-else)

### 13. **Set Literals** (Lines 612-624)
- HashSet construction with insert loop
**Complexity**: 1

### 14. **FrozenSet Literals** (Lines 625-637)
- HashSet wrapped in Arc
**Complexity**: 1

### 15. **Set Comprehensions** (Lines 638-667)
- With condition: filter + map + collect
- Without condition: map + collect
**Complexity**: 2 (if-else)

### 16. **Await Expressions** (Lines 668-671)
- Simple .await suffix
**Complexity**: 1

---

## 🎯 **Refactoring Strategy**

### **Apply Extract Method Pattern**

Extract 14 focused helper functions for different expression types:

```rust
// 1. Binary operations with special handling
fn binary_expr_to_rust_tokens(
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Result<proc_macro2::TokenStream> {
    // Lines 434-462
    // Complexity: ~6-7 → Target: ≤10 ✅
}

// 2. Function call expressions
fn call_expr_to_rust_tokens(
    func: &str,
    args: &[HirExpr],
) -> Result<proc_macro2::TokenStream> {
    // Lines 468-475
    // Complexity: 1 ✅
}

// 3. Collection literals (List, Dict, Tuple)
fn collection_literal_to_rust_tokens(
    expr: &HirExpr,
) -> Result<proc_macro2::TokenStream> {
    // Lines 481-509
    // Complexity: 3 ✅
}

// 4. Borrow expressions
fn borrow_expr_to_rust_tokens(
    expr: &HirExpr,
    mutable: bool,
) -> Result<proc_macro2::TokenStream> {
    // Lines 515-522
    // Complexity: 2 ✅
}

// 5. Method call expressions
fn method_call_to_rust_tokens(
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
) -> Result<proc_macro2::TokenStream> {
    // Lines 523-535
    // Complexity: 1 ✅
}

// 6. Slice expressions
fn slice_expr_to_rust_tokens(
    base: &HirExpr,
    start: &Option<Box<HirExpr>>,
    stop: &Option<Box<HirExpr>>,
    step: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    // Lines 536-564
    // Complexity: 5 ✅
}

// 7. List comprehensions
fn list_comp_to_rust_tokens(
    element: &HirExpr,
    target: &str,
    iter: &HirExpr,
    condition: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    // Lines 565-594
    // Complexity: 2 ✅
}

// 8. Lambda expressions
fn lambda_to_rust_tokens(
    params: &[String],
    body: &HirExpr,
) -> Result<proc_macro2::TokenStream> {
    // Lines 595-611
    // Complexity: 2 ✅
}

// 9. Set literals
fn set_literal_to_rust_tokens(
    items: &[HirExpr],
) -> Result<proc_macro2::TokenStream> {
    // Lines 612-624
    // Complexity: 1 ✅
}

// 10. FrozenSet literals
fn frozen_set_to_rust_tokens(
    items: &[HirExpr],
) -> Result<proc_macro2::TokenStream> {
    // Lines 625-637
    // Complexity: 1 ✅
}

// 11. Set comprehensions
fn set_comp_to_rust_tokens(
    element: &HirExpr,
    target: &str,
    iter: &HirExpr,
    condition: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    // Lines 638-667
    // Complexity: 2 ✅
}
```

### **Refactored expr_to_rust_tokens (Target: Complexity ≤10)**

```rust
fn expr_to_rust_tokens(expr: &HirExpr) -> Result<proc_macro2::TokenStream> {
    match expr {
        // Simple cases (complexity: 1 each)
        HirExpr::Literal(lit) => literal_to_rust_tokens(lit),
        HirExpr::Var(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            Ok(quote! { #ident })
        }
        HirExpr::Index { base, index } => {
            let base_tokens = expr_to_rust_tokens(base)?;
            let index_tokens = expr_to_rust_tokens(index)?;
            Ok(quote! { #base_tokens[#index_tokens] })
        }
        HirExpr::Attribute { value, attr } => {
            let value_tokens = expr_to_rust_tokens(value)?;
            let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
            Ok(quote! { #value_tokens.#attr_ident })
        }
        HirExpr::Await { value } => {
            let value_tokens = expr_to_rust_tokens(value)?;
            Ok(quote! { #value_tokens.await })
        }
        HirExpr::Unary { op, operand } => {
            let operand_tokens = expr_to_rust_tokens(operand)?;
            let op_tokens = unaryop_to_rust_tokens(op);
            Ok(quote! { (#op_tokens #operand_tokens) })
        }

        // Delegated to helper functions (complexity: 1 each)
        HirExpr::Binary { op, left, right } =>
            binary_expr_to_rust_tokens(op, left, right),
        HirExpr::Call { func, args } =>
            call_expr_to_rust_tokens(func, args),
        HirExpr::List(items) =>
            list_literal_to_rust_tokens(items),
        HirExpr::Dict(items) =>
            dict_literal_to_rust_tokens(items),
        HirExpr::Tuple(items) =>
            tuple_literal_to_rust_tokens(items),
        HirExpr::Borrow { expr, mutable } =>
            borrow_expr_to_rust_tokens(expr, *mutable),
        HirExpr::MethodCall { object, method, args } =>
            method_call_to_rust_tokens(object, method, args),
        HirExpr::Slice { base, start, stop, step } =>
            slice_expr_to_rust_tokens(base, start, stop, step),
        HirExpr::ListComp { element, target, iter, condition } =>
            list_comp_to_rust_tokens(element, target, iter, condition),
        HirExpr::Lambda { params, body } =>
            lambda_to_rust_tokens(params, body),
        HirExpr::Set(items) =>
            set_literal_to_rust_tokens(items),
        HirExpr::FrozenSet(items) =>
            frozen_set_to_rust_tokens(items),
        HirExpr::SetComp { element, target, iter, condition } =>
            set_comp_to_rust_tokens(element, target, iter, condition),
    }
}
```

**Estimated Complexity**: 19 match arms, but each arm is trivial → **~8-10 ✅**

---

## 🧪 **Testing Strategy (EXTREME TDD)**

### **Phase 1: Property Tests (MUST WRITE FIRST)**

Create comprehensive property tests BEFORE refactoring:

```rust
#[cfg(test)]
mod expr_to_rust_tests {
    use super::*;

    // Helper to create simple expressions
    fn create_int_literal(value: i64) -> HirExpr {
        HirExpr::Literal(Literal::Int(value))
    }

    fn create_var(name: &str) -> HirExpr {
        HirExpr::Var(name.to_string())
    }

    #[test]
    fn test_literal_never_panics() {
        let expr = create_int_literal(42);
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_var_produces_valid_rust() {
        let expr = create_var("x");
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());

        if let Ok(tokens) = result {
            let code = tokens.to_string();
            assert!(code.contains("x"));
        }
    }

    #[test]
    fn test_binary_add() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(create_int_literal(1)),
            right: Box::new(create_int_literal(2)),
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_binary_sub_with_len() {
        let len_call = HirExpr::MethodCall {
            object: Box::new(create_var("arr")),
            method: "len".to_string(),
            args: vec![],
        };
        let expr = HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(len_call),
            right: Box::new(create_int_literal(1)),
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
        if let Ok(tokens) = result {
            let code = tokens.to_string();
            assert!(code.contains("saturating_sub"));
        }
    }

    #[test]
    fn test_floor_div() {
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(create_int_literal(7)),
            right: Box::new(create_int_literal(3)),
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_literal() {
        let expr = HirExpr::List(vec![
            create_int_literal(1),
            create_int_literal(2),
            create_int_literal(3),
        ]);
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
        if let Ok(tokens) = result {
            let code = tokens.to_string();
            assert!(code.contains("vec"));
        }
    }

    #[test]
    fn test_dict_literal() {
        let expr = HirExpr::Dict(vec![
            (create_int_literal(1), create_int_literal(10)),
            (create_int_literal(2), create_int_literal(20)),
        ]);
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_slice_full() {
        let expr = HirExpr::Slice {
            base: Box::new(create_var("arr")),
            start: Some(Box::new(create_int_literal(1))),
            stop: Some(Box::new(create_int_literal(3))),
            step: None,
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_comp_without_condition() {
        let expr = HirExpr::ListComp {
            element: Box::new(create_var("x")),
            target: "x".to_string(),
            iter: Box::new(create_var("items")),
            condition: None,
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_comp_with_condition() {
        let expr = HirExpr::ListComp {
            element: Box::new(create_var("x")),
            target: "x".to_string(),
            iter: Box::new(create_var("items")),
            condition: Some(Box::new(HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(create_var("x")),
                right: Box::new(create_int_literal(5)),
            })),
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lambda_no_params() {
        let expr = HirExpr::Lambda {
            params: vec![],
            body: Box::new(create_int_literal(42)),
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lambda_with_params() {
        let expr = HirExpr::Lambda {
            params: vec!["x".to_string(), "y".to_string()],
            body: Box::new(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(create_var("x")),
                right: Box::new(create_var("y")),
            }),
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(create_var("obj")),
            method: "foo".to_string(),
            args: vec![create_int_literal(42)],
        };
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_literal() {
        let expr = HirExpr::Set(vec![
            create_int_literal(1),
            create_int_literal(2),
        ]);
        let result = expr_to_rust_tokens(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deterministic_output() {
        let expr = create_var("test_var");
        let output1 = expr_to_rust_tokens(&expr);
        let output2 = expr_to_rust_tokens(&expr);
        assert_eq!(output1.is_ok(), output2.is_ok());
        if let (Ok(tokens1), Ok(tokens2)) = (output1, output2) {
            assert_eq!(tokens1.to_string(), tokens2.to_string());
        }
    }
}
```

### **Phase 2: Regression Tests**

Ensure refactoring doesn't break existing behavior:
- Run full test suite
- Test with all existing examples
- Verify output identical pre/post refactoring

---

## 📋 **Implementation Plan**

### **Step 1: Write Tests** (RED - TDD) - 8-10 hours
- [x] Analyze function structure
- [ ] Create test file: `crates/depyler-core/tests/expr_to_rust_tests.rs`
- [ ] Write 15+ integration tests for each expression type
- [ ] All tests should PASS (function already works)

### **Step 2: Extract Helper Functions** (GREEN - TDD) - 30-40 hours
- [ ] Extract `binary_expr_to_rust_tokens` (Lines 434-462)
- [ ] Extract `call_expr_to_rust_tokens` (Lines 468-475)
- [ ] Extract `list_literal_to_rust_tokens` (Lines 481-487)
- [ ] Extract `dict_literal_to_rust_tokens` (Lines 488-502)
- [ ] Extract `tuple_literal_to_rust_tokens` (Lines 503-509)
- [ ] Extract `borrow_expr_to_rust_tokens` (Lines 515-522)
- [ ] Extract `method_call_to_rust_tokens` (Lines 523-535)
- [ ] Extract `slice_expr_to_rust_tokens` (Lines 536-564)
- [ ] Extract `list_comp_to_rust_tokens` (Lines 565-594)
- [ ] Extract `lambda_to_rust_tokens` (Lines 595-611)
- [ ] Extract `set_literal_to_rust_tokens` (Lines 612-624)
- [ ] Extract `frozen_set_to_rust_tokens` (Lines 625-637)
- [ ] Extract `set_comp_to_rust_tokens` (Lines 638-667)

### **Step 3: Refactor Main Function** (REFACTOR - TDD) - 10-15 hours
- [ ] Replace inline code with helper function calls
- [ ] Verify complexity ≤10 via `pmat analyze complexity`
- [ ] Verify all tests PASS
- [ ] Run `cargo test --workspace`

### **Step 4: Verify Quality** (TDD Verification) - 5-10 hours
- [ ] Run `pmat tdg crates/depyler-core/src/codegen.rs`
- [ ] Verify TDG score maintains A+ (99+)
- [ ] Run full test suite
- [ ] Verify no regressions
- [ ] Run clippy: `cargo clippy -- -D warnings`

### **Step 5: Documentation** - 2-3 hours
- [ ] Add rustdoc comments to all new helper functions
- [ ] Add examples in doctests
- [ ] Update CHANGELOG.md
- [ ] Update roadmap.md
- [ ] Create DEPYLER-0005-COMPLETION.md

---

## ⏱️ **Time Estimate**

- **Tests**: 8-10 hours
- **Refactoring**: 40-55 hours
- **Verification**: 5-10 hours
- **Documentation**: 2-3 hours

**Total**: 55-78 hours (within 60-80h estimate ✅)

---

## 🚨 **Risks and Mitigations**

### **Risk 1**: Many recursive calls to `expr_to_rust_tokens`
**Mitigation**: Each helper will call back to main function, maintaining recursion

### **Risk 2**: Tests may not catch all edge cases
**Mitigation**: Comprehensive test suite with 15+ tests covering all variants

### **Risk 3**: Helper functions might still be complex
**Mitigation**: Each helper targets single responsibility, should be ≤7 complexity

### **Risk 4**: Refactoring breaks existing transpilation
**Mitigation**: Test-first approach + regression test suite

---

## ✅ **Success Criteria**

- [ ] `expr_to_rust_tokens` complexity: 39 → ≤10
- [ ] All helper functions complexity: ≤10
- [ ] Integration tests: 15+ covering all expression types
- [ ] TDG score: Maintains A+ (99+)
- [ ] All existing tests pass
- [ ] Clippy warnings: 0
- [ ] SATD comments: 0
- [ ] Rustdoc coverage: 100% for new functions

---

## 📝 **Next Actions**

1. **Immediate**: Create test file `expr_to_rust_tests.rs`
2. **Phase 1**: Write 15+ integration tests (8-10h)
3. **Phase 2**: Begin extraction (start with simplest helpers)
4. **Phase 3**: Refactor main function
5. **Phase 4**: Verify and document

---

**Status**: Ready to begin
**Blocking**: None
**Dependencies**: None (all exist)
**Assignee**: Current session
**Sprint**: Sprint 2

---

*Created: 2025-10-02*
*Last Updated: 2025-10-02*
*Ticket: DEPYLER-0005*
