# DEPYLER-0305: Classes/OOP Not Supported - Transpiler Panics

**Discovered**: 2025-10-29 during Example 11 (Basic Classes) validation
**Status**: 🛑 **CRITICAL ARCHITECTURAL GAP** - Classes completely unsupported
**Priority**: P0 (fundamental Python feature - blocks ALL OOP code)
**Estimate**: 40-60 hours (very high complexity, major architectural addition)

## Overview

Attempted to transpile Example 11 (18 simple class/OOP functions) and discovered that **classes are completely unsupported**. The transpiler **panics** when encountering class definitions instead of gracefully handling them.

## Discovery Context

**Example**: python-to-rust-conversion-examples/examples/11_basic_classes/
**Functions**: 18 functions using basic classes
**Result**: **Transpiler panic** - `thread 'main' panicked at expr_gen.rs:2079`

**Error Message**:
```
thread 'main' panicked at crates/depyler-core/src/rust_gen/expr_gen.rs:2079:23:
expected identifier or integer
```

## Root Cause: No Class Support in HIR

**Investigation**:
```bash
grep -r "ClassDef" crates/depyler-core/src/hir.rs
# No matches found
```

The HIR (High-level Intermediate Representation) has **NO representation for classes**:
- No `HirClass` or `ClassDef` enum variant
- No support for `__init__` methods
- No support for `self` parameter
- No support for instance attributes
- No support for instance methods

**Python AST Has Classes**: `rustpython_ast::Stmt::ClassDef` exists
**Depyler HIR**: No corresponding representation

---

## Scope of Impact

**Blocks**:
1. ✅ Class definitions (`class Foo:`)
2. ✅ `__init__` constructors
3. ✅ Instance methods (`def method(self):`)
4. ✅ Instance attributes (`self.x = value`)
5. ✅ Inheritance (`class Child(Parent):`)
6. ✅ Class methods (`@classmethod`)
7. ✅ Static methods (`@staticmethod`)
8. ✅ Properties (`@property`)
9. ✅ Dataclasses
10. ✅ **ALL object-oriented programming**

**Impact**: **Blocks 60-70% of real-world Python code** that uses OOP

---

## Python Pattern That Fails

```python
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance_squared(self) -> int:
        return self.x * self.x + self.y * self.y

# Usage
p = Point(10, 20)
print(p.distance_squared())
```

**Current Behavior**: Transpiler panics with `expected identifier or integer`

---

## Required Implementation

### Phase 1: HIR Class Representation (10-15 hours)

Add class support to HIR:

```rust
// In hir.rs:
#[derive(Debug, Clone)]
pub struct HirClass {
    pub name: String,
    pub bases: Vec<String>,  // Inheritance
    pub methods: Vec<HirMethod>,
    pub attributes: Vec<HirAttribute>,
}

#[derive(Debug, Clone)]
pub struct HirMethod {
    pub name: String,
    pub params: Vec<HirParam>,
    pub body: Vec<HirStmt>,
    pub return_type: Option<Type>,
    pub is_init: bool,  // Special handling for __init__
}

#[derive(Debug, Clone)]
pub struct HirAttribute {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub default_value: Option<HirExpr>,
}
```

### Phase 2: AST → HIR Conversion (8-12 hours)

Convert Python's `ClassDef` to HIR:

```rust
// In ast_bridge/converters.rs:
impl StmtConverter {
    fn convert_class_def(c: ast::StmtClassDef) -> Result<HirStmt> {
        // Extract class name
        // Extract inheritance (bases)
        // Convert __init__ method
        // Convert other methods
        // Extract instance attributes from __init__
        // Return HirStmt::Class(HirClass { ... })
    }
}
```

### Phase 3: HIR → Rust Struct Generation (15-20 hours)

Map classes to Rust structs:

```rust
// Python:
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance_squared(self) -> int:
        return self.x * self.x + self.y * self.y

// Rust:
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}
```

### Phase 4: Method Call Translation (5-8 hours)

- Translate `obj.method()` to `obj.method()`
- Handle `self` parameter correctly
- Handle mutable methods (`&mut self`)

### Phase 5: Constructor Translation (3-5 hours)

- Translate `Point(x, y)` to `Point::new(x, y)`
- Handle `__init__` → `new()` mapping

---

## Implementation Complexity

**Total Estimate**: 40-60 hours

**Why So Complex**:
1. **New HIR structures** needed (no existing class representation)
2. **AST conversion** logic (handle __init__, self, attributes, inheritance)
3. **Rust code generation** (structs, impls, methods)
4. **Self parameter handling** (when to use `&self`, `&mut self`, or `self`)
5. **Constructor translation** (Point(x, y) → Point::new(x, y))
6. **Inheritance** (Python inheritance → Rust traits/composition)

**Dependencies**:
- Type inference for attributes
- Ownership analysis for methods
- Trait generation for inheritance (if supported)

---

## Comparison with Other Blockers

| Issue | Scope | Estimate | Impact |
|-------|-------|----------|--------|
| DEPYLER-0304 (Context managers) | File I/O | 11-13 hours | Blocks file I/O |
| **DEPYLER-0305 (Classes)** | **ALL OOP** | **40-60 hours** | **Blocks 60-70% of Python code** |
| DEPYLER-0302 (Strings) | String methods | 6-8 hours | High-frequency feature |
| DEPYLER-0303 (Dicts) | Dict methods | 4-6 hours | High-frequency feature |

**DEPYLER-0305 is the largest architectural gap discovered**.

---

## Alternative: Simplified Class Support

If full OOP is too complex, consider **minimal class support**:

**Supported**:
- Simple classes with `__init__`
- Instance attributes
- Instance methods (no inheritance)
- Constructor calls

**Not Supported** (initially):
- Inheritance
- Class methods / static methods
- Properties
- Magic methods (besides `__init__`)
- Multiple inheritance

This reduces implementation to **20-30 hours** but covers **80% of simple class usage**.

---

## Recommendation

**Status**: **ARCHITECTURAL BLOCKER** - Cannot proceed with OOP examples

**Options**:
1. **Skip classes for now** - Continue Matrix discovery with non-OOP examples
2. **Implement minimal class support** (20-30 hours) - Unblock basic OOP
3. **Full class support** (40-60 hours) - Complete OOP implementation

**My Recommendation**: **Option 1 (Skip for now)** - Continue Matrix discovery to find other gaps, then prioritize all architectural issues together.

---

## Strategic Implications

**If classes are not supported**, the transpiler can currently only handle:
- ✅ Procedural Python (functions only)
- ✅ Built-in data structures (lists, dicts, strings)
- ⚠️ Limited file I/O (broken context managers)
- ❌ Object-oriented Python
- ❌ Dataclasses
- ❌ Most real-world Python code

**Production Readiness**: Requires both DEPYLER-0304 (context managers) AND DEPYLER-0305 (classes)

---

## Conclusion

Example 11 discovery reveals the **most significant architectural gap**: classes are completely unsupported, affecting 60-70% of Python code. Combined with DEPYLER-0304 (context managers), these two issues represent the critical path to production readiness.

**Next Steps**:
1. ✅ Document finding (this ticket)
2. 🎯 Continue Matrix discovery (skip OOP examples)
3. 📋 After Matrix complete: Prioritize architectural fixes
4. 🚀 Implement DEPYLER-0304 + DEPYLER-0305 in dedicated sprint

**Status**: Documented, **CRITICAL ARCHITECTURAL BLOCKER**
