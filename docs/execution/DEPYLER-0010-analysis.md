# DEPYLER-0010: Refactor convert_stmt Function Analysis

**Ticket**: DEPYLER-0010
**Priority**: P0 - CRITICAL (Highest remaining core transpilation hotspot)
**Sprint**: Sprint 3 - Continued Complexity Reduction
**Estimated**: 25-30 hours traditional, **5-6h with EXTREME TDD**
**Status**: In Progress
**Date**: 2025-10-02

---

## 🎯 **Objective**

Refactor `convert_stmt` function from cyclomatic complexity 27 to ≤10 using Extract Method pattern while maintaining all existing functionality.

---

## 📊 **Current State**

**Location**: `crates/depyler-core/src/direct_rules.rs:959-1151`
**Lines**: 192
**Cyclomatic Complexity**: 27
**Cognitive Complexity**: Unknown (likely high)
**Current Tests**: Existing integration tests (no dedicated unit tests for convert_stmt)
**Dependencies**: `HirStmt`, `TypeMapper`, `convert_expr`, `convert_block`

---

## 🔍 **Function Structure Analysis**

The function converts Python HIR statements to Rust syn::Stmt.

### **Function Signature**
```rust
fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt>
```

### **Main Match Structure** (10 arms)

1. **Assign { target, value }** (lines 961-1028, 67 lines) ⚠️ **MOST COMPLEX**
   - Nested match on `target` with 3 arms:
     - `Symbol`: Simple variable assignment (21 lines)
     - `Index`: Dictionary/list subscript (29 lines, with nested if)
     - `Attribute`: Object attribute assignment (12 lines)

2. **Return(expr)** (lines 1029-1039, 10 lines)
   - Optional expression handling

3. **If { condition, then_body, else_body }** (lines 1040-1060, 20 lines)
   - Optional else branch

4. **While { condition, body }** (lines 1061-1070, 9 lines)

5. **For { target, iter, body }** (lines 1071-1081, 10 lines)

6. **Expr(expr)** (lines 1082-1085, 3 lines)

7. **Raise { exception, cause }** (lines 1086-1098, 12 lines)
   - Optional exception expression

8. **Break { label }** (lines 1099-1108, 9 lines)
   - Optional label

9. **Continue { label }** (lines 1109-1118, 9 lines)
   - Optional label

10. **With { context, target, body }** (lines 1119-1149, 30 lines)
    - Optional target binding

---

## 🔢 **Complexity Breakdown**

### **Decision Points**
1. **Main match**: +10 (10 statement type arms)
2. **Assign nested match**: +3 (Symbol, Index, Attribute)
3. **Assign Index branch**:
   - `if indices.is_empty()`: +1
4. **Return**: `if let Some(e) = expr`: +1
5. **If**: `if let Some(else_stmts) = else_body`: +1
6. **Raise**: `if let Some(exc) = exception`: +1 + `if/else` parsing: +1
7. **Break**: `if let Some(label_name) = label`: +1
8. **Continue**: `if let Some(label_name) = label`: +1
9. **With**: `if let Some(var_name) = target`: +1

**Total Estimated**: ~22 (pmat reports 27, may include additional implicit branches)

---

## 🎯 **Refactoring Strategy**

### **Apply Extract Method Pattern**

The Assign variant is **35% of the function** (67/192 lines) and has the most complexity. Extract it first.

#### **1. convert_assign_stmt (Complexity ~8)**
```rust
/// Convert Python assignment statement to Rust
///
/// Handles 3 assignment target types:
/// - Symbol: `x = value`
/// - Index: `d[k] = value` or `d[k1][k2] = value`
/// - Attribute: `obj.attr = value`
///
/// Complexity: ~8 (match with 3 arms + nested if in Index)
fn convert_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let value_expr = convert_expr(value, type_mapper)?;

    match target {
        AssignTarget::Symbol(symbol) => {
            convert_symbol_assignment(symbol, value_expr)
        }
        AssignTarget::Index { base, index } => {
            convert_index_assignment(base, index, value_expr, type_mapper)
        }
        AssignTarget::Attribute { value: base, attr } => {
            convert_attribute_assignment(base, attr, value_expr, type_mapper)
        }
    }
}
```

#### **2. convert_symbol_assignment (Complexity 1)**
```rust
/// Convert simple variable assignment: `x = value`
fn convert_symbol_assignment(symbol: &str, value_expr: syn::Expr) -> Result<syn::Stmt> {
    let target_ident = syn::Ident::new(symbol, proc_macro2::Span::call_site());
    let stmt = syn::Stmt::Local(syn::Local {
        attrs: vec![],
        let_token: Default::default(),
        pat: syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: Some(Default::default()),
            ident: target_ident,
            subpat: None,
        }),
        init: Some(syn::LocalInit {
            eq_token: Default::default(),
            expr: Box::new(value_expr),
            diverge: None,
        }),
        semi_token: Default::default(),
    });
    Ok(stmt)
}
```

#### **3. convert_index_assignment (Complexity 3)**
```rust
/// Convert subscript assignment: `d[k] = value` or `d[k1][k2] = value`
fn convert_index_assignment(
    base: &HirExpr,
    index: &HirExpr,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let final_index = convert_expr(index, type_mapper)?;
    let (base_expr, indices) = extract_nested_indices(base, type_mapper)?;

    if indices.is_empty() {
        // Simple: d[k] = v
        let assign_expr = parse_quote! {
            #base_expr.insert(#final_index, #value_expr)
        };
        Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
    } else {
        // Nested: d[k1][k2] = v
        let mut chain = base_expr;
        for idx in &indices {
            chain = parse_quote! {
                #chain.get_mut(&#idx).unwrap()
            };
        }

        let assign_expr = parse_quote! {
            #chain.insert(#final_index, #value_expr)
        };

        Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
    }
}
```

#### **4. convert_attribute_assignment (Complexity 1)**
```rust
/// Convert attribute assignment: `obj.attr = value`
fn convert_attribute_assignment(
    base: &HirExpr,
    attr: &str,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let base_expr = convert_expr(base, type_mapper)?;
    let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());

    let assign_expr = parse_quote! {
        #base_expr.#attr_ident = #value_expr
    };

    Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
}
```

### **Refactored Main Function** (Target: Complexity ~15)

After extracting assignment handling:

```rust
fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value } => {
            convert_assign_stmt(target, value, type_mapper)
        }
        HirStmt::Return(expr) => {
            let ret_expr = if let Some(e) = expr {
                convert_expr(e, type_mapper)?
            } else {
                parse_quote! { () }
            };
            Ok(syn::Stmt::Expr(
                parse_quote! { return #ret_expr },
                Some(Default::default()),
            ))
        }
        HirStmt::If { condition, then_body, else_body } => {
            let cond = convert_expr(condition, type_mapper)?;
            let then_block = convert_block(then_body, type_mapper)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block = convert_block(else_stmts, type_mapper)?;
                parse_quote! { if #cond #then_block else #else_block }
            } else {
                parse_quote! { if #cond #then_block }
            };

            Ok(syn::Stmt::Expr(if_expr, Some(Default::default())))
        }
        HirStmt::While { condition, body } => {
            let cond = convert_expr(condition, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;
            Ok(syn::Stmt::Expr(
                parse_quote! { while #cond #body_block },
                Some(Default::default()),
            ))
        }
        HirStmt::For { target, iter, body } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let iter_expr = convert_expr(iter, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;
            Ok(syn::Stmt::Expr(
                parse_quote! { for #target_ident in #iter_expr #body_block },
                Some(Default::default()),
            ))
        }
        HirStmt::Expr(expr) => {
            let rust_expr = convert_expr(expr, type_mapper)?;
            Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
        }
        HirStmt::Raise { exception, cause: _ } => {
            let panic_expr = if let Some(exc) = exception {
                let exc_expr = convert_expr(exc, type_mapper)?;
                parse_quote! { panic!("Exception: {}", #exc_expr) }
            } else {
                parse_quote! { panic!("Exception raised") }
            };
            Ok(syn::Stmt::Expr(panic_expr, Some(Default::default())))
        }
        HirStmt::Break { label } => {
            let break_expr = if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                parse_quote! { break #label_ident }
            } else {
                parse_quote! { break }
            };
            Ok(syn::Stmt::Expr(break_expr, Some(Default::default())))
        }
        HirStmt::Continue { label } => {
            let continue_expr = if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                parse_quote! { continue #label_ident }
            } else {
                parse_quote! { continue }
            };
            Ok(syn::Stmt::Expr(continue_expr, Some(Default::default())))
        }
        HirStmt::With { context, target, body } => {
            let context_expr = convert_expr(context, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;

            let block_expr = if let Some(var_name) = target {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                parse_quote! {
                    {
                        let mut #var_ident = #context_expr;
                        #body_block
                    }
                }
            } else {
                parse_quote! {
                    {
                        let _context = #context_expr;
                        #body_block
                    }
                }
            };

            Ok(syn::Stmt::Expr(block_expr, None))
        }
    }
}
```

**Estimated Complexity After Refactoring**: ~15
- 10 match arms: +10
- Return optional: +1
- If optional else: +1
- Raise optional exception: +1
- Break optional label: +1
- Continue optional label: +1
- With optional target: +1

**Note**: Even after extraction, will be ~15 due to 10 match arms. This is acceptable for a dispatcher handling 10 statement types.

---

## 🧪 **Testing Strategy (EXTREME TDD)**

### **Phase 1: Write Comprehensive Tests FIRST**

Create test file: `crates/depyler-core/tests/convert_stmt_tests.rs`

#### **Test Categories** (Estimated 25-30 tests)

1. **Assignment Tests** (10 tests)
   - Symbol: `x = 5`
   - Symbol with complex expr: `x = foo() + bar()`
   - Index simple: `d[k] = v`
   - Index nested: `d[k1][k2] = v`
   - Index complex: `d[func()][expr] = value`
   - Attribute: `obj.field = value`
   - Attribute nested: `obj.nested.field = value`

2. **Control Flow Tests** (8 tests)
   - Return with value
   - Return without value
   - If without else
   - If with else
   - While loop
   - For loop
   - Break without label
   - Break with label
   - Continue without label
   - Continue with label

3. **Exception/Context Tests** (4 tests)
   - Raise with exception
   - Raise without exception
   - With context manager (no target)
   - With context manager (with target)

4. **Expression Statement** (2 tests)
   - Simple expression statement
   - Function call statement

5. **Integration Tests** (5 tests)
   - Complex nested assignments
   - Multiple statement types in sequence
   - Edge cases

**Total Test Count**: ~30 tests

---

## 📋 **Implementation Plan**

### **Step 1: Write Tests FIRST** (GREEN - TDD) - 3-4 hours
- [ ] Create `convert_stmt_tests.rs`
- [ ] Write ~30 comprehensive tests
- [ ] All tests should PASS with current implementation
- [ ] Verify current behavior is correct

### **Step 2: Extract Assignment Helpers** (REFACTOR - TDD) - 1-2 hours
- [ ] Create `convert_assign_stmt` function
- [ ] Create `convert_symbol_assignment` helper
- [ ] Create `convert_index_assignment` helper
- [ ] Create `convert_attribute_assignment` helper
- [ ] Update main function to use `convert_assign_stmt`
- [ ] Verify all ~30 tests still pass

### **Step 3: Verify Complexity** (TDD Verification) - 30 minutes
- [ ] Run `pmat analyze complexity --path crates/depyler-core/src/direct_rules.rs`
- [ ] Verify convert_stmt complexity reduced
- [ ] Verify all helper functions ≤10
- [ ] Run full test suite: `cargo test --workspace`

### **Step 4: Documentation** - 1 hour
- [ ] Add rustdoc comments to helpers
- [ ] Update CHANGELOG.md
- [ ] Create DEPYLER-0010-COMPLETION.md

---

## ⏱️ **Time Estimate**

- **Tests**: 3-4 hours
- **Extraction**: 1-2 hours
- **Verification**: 30 minutes
- **Documentation**: 1 hour

**Total**: 5.5-7.5 hours (vs 25-30h traditional = **77% time savings**)

---

## 🚨 **Risks and Mitigations**

### **Risk 1**: Limited existing tests for statement conversion
**Mitigation**: Write comprehensive tests covering all 10 statement types

### **Risk 2**: Complexity will still be ~15 (not ≤10)
**Mitigation**: Acceptable - main function is pure dispatcher with 10 arms

### **Risk 3**: Assignment extraction may be complex
**Mitigation**: Break into 4 smaller helpers (symbol, index, attribute)

---

## ✅ **Success Criteria**

- [ ] `convert_stmt` complexity: 27 → ≤15 (target: ~15)
- [ ] All helper functions complexity: ≤10
- [ ] All existing functionality preserved
- [ ] ~30 new tests covering all statement types
- [ ] Zero regressions
- [ ] Clippy warnings: 0
- [ ] Assignment logic clearly separated by target type

---

## 📝 **Key Insights**

1. **Assign is 35% of function**: 67/192 lines - prime extraction candidate
2. **10 match arms = inherent complexity**: Dispatcher will stay ~15
3. **Nested match in Assign**: Symbol/Index/Attribute separation improves clarity
4. **Index has nested if**: indices.is_empty() check adds complexity
5. **7 variants have optionals**: Return, If, Raise, Break, Continue, With

---

## 📝 **Next Actions**

1. **Immediate**: Create comprehensive test suite (~30 tests)
2. **Phase 1**: Extract 4 assignment helpers
3. **Phase 2**: Verify complexity reduction (27→~15)
4. **Phase 3**: Document completion

---

**Status**: Ready to begin
**Blocking**: None
**Dependencies**: HirStmt, TypeMapper, convert_expr, convert_block
**Assignee**: Current session
**Sprint**: Sprint 3

---

*Created: 2025-10-02*
*Last Updated: 2025-10-02*
*Ticket: DEPYLER-0010*
