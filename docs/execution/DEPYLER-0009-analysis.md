# DEPYLER-0009: Refactor process_module_imports Function Analysis

**Ticket**: DEPYLER-0009
**Priority**: P0 - CRITICAL
**Sprint**: Sprint 2 - Critical Complexity Reduction
**Estimated**: 15-20 hours
**Status**: In Progress
**Date**: 2025-10-02

---

## 🎯 **Objective**

Refactor `process_module_imports` function from cyclomatic complexity 15 to ≤10 using Extract Method pattern while maintaining all existing functionality.

---

## 📊 **Current State**

**Location**: `crates/depyler-core/src/rust_gen.rs:73-128`
**Lines**: 56
**Cyclomatic Complexity**: 15
**Cognitive Complexity**: 72 (VERY HIGH!)
**Current Tests**: No dedicated unit tests for this function
**Dependencies**:
- `crate::module_mapper::ModuleMapper`
- `Import`, `ImportItem` types

---

## 🔍 **Function Structure Analysis**

The function processes Python imports and maps them to Rust module paths.

### **Function Signature**
```rust
fn process_module_imports(
    imports: &[Import],
    module_mapper: &ModuleMapper,
) -> (
    HashMap<String, ModuleMapping>,
    HashMap<String, String>,
)
```

### **High-Level Logic**
1. Initialize two HashMaps: `imported_modules` and `imported_items`
2. Loop through all imports
3. For each import:
   - **Case A**: Whole module import (no specific items)
   - **Case B**: Specific items import
     - Loop through each item
     - Match on ImportItem (Named vs Aliased)
     - Apply special handling for "typing" module
     - Build qualified Rust paths

### **Complexity Breakdown**

#### **Control Flow**
1. **Outer loop** (line 83): `for import in imports` - +1
2. **Empty items check** (line 84): `if import.items.is_empty()` - +1
3. **Whole module mapping** (line 86): `if let Some(mapping)` - +1
4. **Specific items mapping** (line 91): `if let Some(mapping)` - +1
5. **Items loop** (line 92): `for item in &import.items` - +1
6. **Match on ImportItem** (line 93): `match item` - +1
7. **Named arm - rust_name check** (line 95): `if let Some(rust_name)` - +1
8. **Named arm - typing module check** (line 97): `if import.module == "typing"` - +1
9. **Named arm - empty path check** (line 100): `else if !mapping.rust_path.is_empty()` - +1
10. **Aliased arm - rust_name check** (line 109): `if let Some(rust_name)` - +1
11. **Aliased arm - typing module check** (line 111): `if import.module == "typing"` - +1
12. **Aliased arm - empty path check** (line 113): `else if !mapping.rust_path.is_empty()` - +1

**Total Estimated**: ~12-15 complexity

### **Code Duplication**

The logic for Named and Aliased items is nearly identical:
```rust
// Named item (lines 94-107)
if let Some(rust_name) = mapping.item_map.get(name) {
    if import.module == "typing" && !rust_name.is_empty() {
        imported_items.insert(name.clone(), rust_name.clone());
    } else if !mapping.rust_path.is_empty() {
        imported_items.insert(
            name.clone(),
            format!("{}::{}", mapping.rust_path, rust_name),
        );
    }
}

// Aliased item (lines 108-120) - SAME LOGIC, different key
if let Some(rust_name) = mapping.item_map.get(name) {
    if import.module == "typing" && !rust_name.is_empty() {
        imported_items.insert(alias.clone(), rust_name.clone());
    } else if !mapping.rust_path.is_empty() {
        imported_items.insert(
            alias.clone(),
            format!("{}::{}", mapping.rust_path, rust_name),
        );
    }
}
```

---

## 🎯 **Refactoring Strategy**

### **Apply Extract Method Pattern**

Create 3 helper functions:

#### **1. process_whole_module_import (Complexity: 2)**
```rust
/// Process a whole module import (e.g., `import math`)
/// Complexity: 2 (if let)
fn process_whole_module_import(
    import: &Import,
    module_mapper: &ModuleMapper,
    imported_modules: &mut HashMap<String, ModuleMapping>,
) {
    if let Some(mapping) = module_mapper.get_mapping(&import.module) {
        imported_modules.insert(import.module.clone(), mapping.clone());
    }
}
```

#### **2. process_import_item (Complexity: 4)**
```rust
/// Process a single import item and add to imported_items
/// Complexity: 4 (2 if checks + typing special case)
fn process_import_item(
    import_module: &str,
    item_name: &str,
    import_key: &str,  // Either name or alias
    mapping: &ModuleMapping,
    imported_items: &mut HashMap<String, String>,
) {
    if let Some(rust_name) = mapping.item_map.get(item_name) {
        if import_module == "typing" && !rust_name.is_empty() {
            // Types from typing module don't need full paths
            imported_items.insert(import_key.to_string(), rust_name.clone());
        } else if !mapping.rust_path.is_empty() {
            imported_items.insert(
                import_key.to_string(),
                format!("{}::{}", mapping.rust_path, rust_name),
            );
        }
    }
}
```

#### **3. process_specific_items_import (Complexity: 5)**
```rust
/// Process specific items import (e.g., `from typing import List, Dict`)
/// Complexity: 5 (if let + loop + match)
fn process_specific_items_import(
    import: &Import,
    module_mapper: &ModuleMapper,
    imported_items: &mut HashMap<String, String>,
) {
    if let Some(mapping) = module_mapper.get_mapping(&import.module) {
        for item in &import.items {
            match item {
                ImportItem::Named(name) => {
                    process_import_item(&import.module, name, name, &mapping, imported_items);
                }
                ImportItem::Aliased { name, alias } => {
                    process_import_item(&import.module, name, alias, &mapping, imported_items);
                }
            }
        }
    }
}
```

### **Refactored Main Function** (Target: Complexity ≤7)

```rust
fn process_module_imports(
    imports: &[Import],
    module_mapper: &ModuleMapper,
) -> (
    HashMap<String, ModuleMapping>,
    HashMap<String, String>,
) {
    let mut imported_modules = HashMap::new();
    let mut imported_items = HashMap::new();

    for import in imports {
        if import.items.is_empty() {
            process_whole_module_import(import, module_mapper, &mut imported_modules);
        } else {
            process_specific_items_import(import, module_mapper, &mut imported_items);
        }
    }

    (imported_modules, imported_items)
}
```

**Estimated Complexity**: ~3 (loop + if/else)

---

## 🧪 **Testing Strategy (EXTREME TDD)**

### **Phase 1: Write Comprehensive Tests FIRST**

Create test file: `crates/depyler-core/tests/process_module_imports_tests.rs`

#### **Test Categories** (Estimated 15-20 tests)

1. **Whole Module Imports** (3 tests)
   - Test: `import math` → Maps to Rust module
   - Test: `import sys` → Maps to Rust module
   - Test: Import unknown module → No mapping

2. **Specific Item Imports - Named** (5 tests)
   - Test: `from typing import List` → Type import
   - Test: `from math import sqrt` → Function import
   - Test: `from collections import HashMap` → With full path
   - Test: Import from unknown module → Empty result
   - Test: Import unknown item from known module → No mapping

3. **Specific Item Imports - Aliased** (5 tests)
   - Test: `from typing import List as L` → Aliased type
   - Test: `from math import sqrt as square_root` → Aliased function
   - Test: Alias with full path
   - Test: Alias from unknown module
   - Test: Alias of unknown item

4. **Edge Cases** (4 tests)
   - Test: Empty imports list → Empty HashMaps
   - Test: Mixed whole + specific imports
   - Test: Multiple items from same module
   - Test: typing module special handling (no full path)

5. **Integration Tests** (2 tests)
   - Test: Complex import scenario with all types
   - Test: Verify HashMap contents match expected mappings

**Total Test Count**: ~19 tests

---

## 📋 **Implementation Plan**

### **Step 1: Write Tests FIRST** (GREEN - TDD) - 2-3 hours
- [ ] Create `process_module_imports_tests.rs`
- [ ] Write ~19 comprehensive tests
- [ ] Create test fixtures (mock ModuleMapper)
- [ ] All tests should PASS with current implementation

### **Step 2: Extract Helper Functions** (REFACTOR - TDD) - 2-3 hours
- [ ] Create `process_whole_module_import` helper
- [ ] Create `process_import_item` helper (eliminates duplication!)
- [ ] Create `process_specific_items_import` helper
- [ ] Update main function to use helpers
- [ ] Verify all ~19 tests still pass

### **Step 3: Verify Complexity** (TDD Verification) - 1 hour
- [ ] Run `pmat analyze complexity --path crates/depyler-core/src/rust_gen.rs`
- [ ] Verify process_module_imports complexity reduced to ≤7
- [ ] Verify all helper functions ≤10
- [ ] Run full test suite: `cargo test --workspace`

### **Step 4: Documentation** - 1-2 hours
- [ ] Add rustdoc comments to all helpers
- [ ] Update CHANGELOG.md
- [ ] Create DEPYLER-0009-COMPLETION.md

---

## ⏱️ **Time Estimate**

- **Tests**: 2-3 hours
- **Extraction**: 2-3 hours
- **Verification**: 1 hour
- **Documentation**: 1-2 hours

**Total**: 6-9 hours (significantly less than 15-20h estimate with EXTREME TDD!)

---

## 🚨 **Risks and Mitigations**

### **Risk 1**: No existing tests for this function
**Mitigation**: Write comprehensive tests first to establish baseline behavior

### **Risk 2**: ModuleMapper dependency makes testing harder
**Mitigation**: Create mock/fixture ModuleMapper in tests

### **Risk 3**: Code duplication between Named/Aliased
**Mitigation**: Extract common logic to `process_import_item` helper

---

## ✅ **Success Criteria**

- [ ] `process_module_imports` complexity: 15 → ≤7 (target 3)
- [ ] All helper functions complexity: ≤10
- [ ] All existing functionality preserved
- [ ] ~19 new tests covering all scenarios
- [ ] Zero regressions
- [ ] Clippy warnings: 0
- [ ] Code duplication eliminated

---

## 📝 **Key Insights**

1. **Duplication**: Named vs Aliased logic is identical except for the key used
2. **Special Case**: "typing" module needs special handling (no full path)
3. **Two Distinct Paths**: Whole module vs specific items are cleanly separable
4. **HashMap Building**: Function builds state via side effects (mutations)

---

## 📝 **Next Actions**

1. **Immediate**: Create comprehensive test suite (~19 tests)
2. **Phase 1**: Extract 3 helper functions
3. **Phase 2**: Verify complexity reduction
4. **Phase 3**: Document completion

---

**Status**: Ready to begin
**Blocking**: None
**Dependencies**: ModuleMapper, Import types
**Assignee**: Current session
**Sprint**: Sprint 2

---

*Created: 2025-10-02*
*Last Updated: 2025-10-02*
*Ticket: DEPYLER-0009*
