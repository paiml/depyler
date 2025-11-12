# DEPYLER-0365: Production-Ready argparse Transpilation Roadmap

**Status**: 📋 Planning Phase
**Priority**: P1 (High Value Feature)
**Ticket**: DEPYLER-0365
**Created**: 2025-11-11
**Assignee**: Claude Code

## Executive Summary

Transform argparse transpilation from **toy examples** (50% test pass rate) to **production-ready** (95%+ compatibility with real-world argparse scripts).

**Current State**: 5/10 tests passing (50%)
**Target State**: 19/20 tests passing (95%+)

## Current Limitations Analysis

### ✅ What Works (5 tests passing)
1. Basic `import argparse` → `use clap::Parser`
2. `ArgumentParser()` constructor detection
3. Simple positional arguments (String type only)
4. `parse_args()` → `Args::parse()` transformation
5. Basic attribute access (`args.name`)

### ❌ What Fails (5 tests failing)
1. **Keyword arguments** (nargs, type, action, help) - **BLOCKING**
2. **Type mapping** (type=int, type=Path) - Depends on #1
3. **Action mapping** (action="store_true") - Depends on #1
4. **Optional flags** (-v, --verbose) - Depends on #1
5. **try/except blocks** (exception handling in file I/O)

### 🚫 Not Implemented (out of scope for MVP)
- Subcommands / subparsers
- Mutually exclusive groups
- Custom validators
- Argument choices/constraints
- Environment variable fallbacks

## Dependency Chain

```
DEPYLER-0364 (HIR kwargs support)
    ↓ BLOCKS
DEPYLER-0363 (argparse transpilation)
    ↓ ENABLES
DEPYLER-0365 (production-ready argparse)
```

## Implementation Phases

### Phase 1: Foundation (DEPYLER-0364) - CRITICAL PATH

**Goal**: Add keyword argument support to HIR

**Tasks**:
1. ✅ Create spec document (DONE)
2. Extend HIR structure with kwargs field
3. Update AST bridge to preserve kwargs
4. Fix all pattern matches (30+ files)
5. Write 18 tests (10 unit + 3 property + 5 integration)

**Deliverables**:
- HIR preserves all keyword arguments
- All existing tests still pass
- New kwargs tests pass

**Estimated Time**: 8-9 hours

---

### Phase 2: Argument Type Mapping (DEPYLER-0366)

**Goal**: Map Python types to Rust types correctly

**File**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

**Mappings**:

| Python | Rust | Notes |
|--------|------|-------|
| `type=str` | `String` | Default |
| `type=int` | `i32` | Standard integer |
| `type=float` | `f64` | Standard float |
| `type=Path` | `PathBuf` | From pathlib |
| `type=bool` | `bool` | Rarely used directly |
| `type=File` | `std::fs::File` | Needs custom handling |

**Implementation**:

```rust
fn map_python_type_to_rust(type_expr: &HirExpr) -> syn::Type {
    match type_expr {
        HirExpr::Var(name) => match name.as_str() {
            "str" => parse_quote! { String },
            "int" => parse_quote! { i32 },
            "float" => parse_quote! { f64 },
            "Path" => parse_quote! { PathBuf },
            "bool" => parse_quote! { bool },
            _ => parse_quote! { String }, // Default fallback
        }
        _ => parse_quote! { String },
    }
}
```

**Tests**: 5 tests (one per type mapping)

**Estimated Time**: 2 hours

---

### Phase 3: Nargs Mapping (DEPYLER-0367)

**Goal**: Map Python nargs to Rust collection types

**Mappings**:

| Python | Rust | Notes |
|--------|------|-------|
| (default) | `T` | Single required value |
| `nargs="+"` | `Vec<T>` | One or more values |
| `nargs="*"` | `Vec<T>` | Zero or more values |
| `nargs="?"` | `Option<T>` | Zero or one value |
| `nargs=N` | `[T; N]` | Exactly N values (limited support) |

**Implementation**:

```rust
fn apply_nargs_wrapper(base_type: syn::Type, nargs: &HirExpr) -> syn::Type {
    match nargs {
        HirExpr::Literal(Literal::String(s)) => match s.as_str() {
            "+" | "*" => parse_quote! { Vec<#base_type> },
            "?" => parse_quote! { Option<#base_type> },
            _ => base_type,
        }
        HirExpr::Literal(Literal::Int(n)) if *n <= 16 => {
            // Fixed-size array for small N
            let size = syn::Index::from(*n as usize);
            parse_quote! { [#base_type; #size] }
        }
        _ => base_type,
    }
}
```

**Tests**: 5 tests (one per nargs pattern)

**Estimated Time**: 2 hours

---

### Phase 4: Action Mapping (DEPYLER-0368)

**Goal**: Map Python actions to Rust types and attributes

**Mappings**:

| Python | Rust Type | Clap Attribute | Notes |
|--------|-----------|----------------|-------|
| (default) | `T` | None | Store value |
| `action="store_true"` | `bool` | `#[arg(long)]` | Flag present = true |
| `action="store_false"` | `bool` | `#[arg(long)]` | Flag present = false |
| `action="count"` | `usize` | `#[arg(long, action = clap::ArgAction::Count)]` | Count occurrences |
| `action="append"` | `Vec<T>` | `#[arg(long)]` | Multiple uses |

**Implementation**:

```rust
fn determine_field_type(
    base_type: syn::Type,
    action: Option<&HirExpr>,
    nargs: Option<&HirExpr>
) -> (syn::Type, Vec<syn::Attribute>) {
    match action {
        Some(HirExpr::Literal(Literal::String(s))) => match s.as_str() {
            "store_true" | "store_false" => (
                parse_quote! { bool },
                vec![parse_quote! { #[arg(long)] }]
            ),
            "count" => (
                parse_quote! { usize },
                vec![parse_quote! { #[arg(long, action = clap::ArgAction::Count)] }]
            ),
            "append" => (
                parse_quote! { Vec<#base_type> },
                vec![parse_quote! { #[arg(long)] }]
            ),
            _ => (base_type, vec![]),
        }
        None => {
            // Apply nargs if present
            if let Some(nargs_expr) = nargs {
                let wrapped = apply_nargs_wrapper(base_type, nargs_expr);
                (wrapped, vec![])
            } else {
                (base_type, vec![])
            }
        }
    }
}
```

**Tests**: 6 tests (one per action type + combined cases)

**Estimated Time**: 3 hours

---

### Phase 5: Flag Mapping (DEPYLER-0369)

**Goal**: Map short/long flags to clap attributes

**Mappings**:

| Python | Clap Attribute |
|--------|----------------|
| `"-v"` | `#[arg(short = 'v')]` |
| `"--verbose"` | `#[arg(long = "verbose")]` |
| `"-v", "--verbose"` | `#[arg(short = 'v', long = "verbose")]` |

**Implementation**:

```rust
fn parse_argument_names(arg_name: &str) -> (Option<char>, Option<String>, bool) {
    if arg_name.starts_with("--") {
        // Long flag: --verbose
        (None, Some(arg_name[2..].to_string()), false)
    } else if arg_name.starts_with("-") && arg_name.len() == 2 {
        // Short flag: -v
        (Some(arg_name.chars().nth(1).unwrap()), None, false)
    } else {
        // Positional argument
        (None, None, true)
    }
}

fn generate_clap_attributes(
    short: Option<char>,
    long: Option<&str>,
    help: Option<&str>
) -> Vec<syn::Attribute> {
    let mut attrs = vec![];

    match (short, long) {
        (Some(s), Some(l)) => {
            attrs.push(parse_quote! { #[arg(short = #s, long = #l)] });
        }
        (Some(s), None) => {
            attrs.push(parse_quote! { #[arg(short = #s)] });
        }
        (None, Some(l)) => {
            attrs.push(parse_quote! { #[arg(long = #l)] });
        }
        (None, None) => {} // Positional
    }

    if let Some(help_text) = help {
        attrs.push(parse_quote! { #[arg(help = #help_text)] });
    }

    attrs
}
```

**Tests**: 4 tests (short only, long only, both, positional)

**Estimated Time**: 2 hours

---

### Phase 6: Integration & Polish (DEPYLER-0370)

**Goal**: Combine all features and handle edge cases

**Tasks**:
1. Update `ArgParserArgument` struct to store all kwargs
2. Refactor `generate_args_struct()` to use full kwargs info
3. Handle argument name conflicts (rename reserved words)
4. Add default values support
5. Generate proper documentation comments
6. Handle multiple ArgumentParser instances

**Implementation Example**:

```rust
pub fn generate_args_struct(parser_info: &ArgParserInfo) -> TokenStream {
    let struct_doc = parser_info.description
        .as_ref()
        .map(|desc| quote! { #[doc = #desc] });

    let fields = parser_info.arguments.iter().map(|arg| {
        let field_name = sanitize_field_name(&arg.name);
        let field_type = determine_field_type(
            map_python_type_to_rust(arg.arg_type.as_ref()),
            arg.action.as_ref(),
            arg.nargs.as_ref()
        );
        let attrs = generate_clap_attributes(
            arg.short,
            arg.long.as_deref(),
            arg.help.as_deref()
        );

        quote! {
            #( #attrs )*
            #field_name: #field_type
        }
    });

    quote! {
        #struct_doc
        #[derive(clap::Parser)]
        struct Args {
            #( #fields ),*
        }
    }
}
```

**Tests**: 8 integration tests covering realistic argparse scripts

**Estimated Time**: 4 hours

---

### Phase 7: Exception Handling (DEPYLER-0371)

**Goal**: Fix try/except code generation for file I/O

**Current Problem**:
```rust
// Generated (WRONG):
let content = std::fs::read_to_string(filepath).unwrap();
content  // orphaned
println!("Error: {}", e);  // orphaned except handler
return "".to_string();
```

**Expected Output**:
```rust
match std::fs::read_to_string(filepath) {
    Ok(content) => content,
    Err(e) => {
        eprintln!("Error: {}", e);
        return "".to_string();
    }
}
```

**Implementation**: Refactor try/except codegen in `stmt_gen.rs`

**Tests**: 3 tests for try/except patterns

**Estimated Time**: 3 hours

---

## Testing Strategy

### Test Coverage Goals

| Component | Unit Tests | Property Tests | Integration Tests | Total |
|-----------|------------|----------------|-------------------|-------|
| HIR kwargs | 10 | 3 | 5 | 18 |
| Type mapping | 5 | 1 | - | 6 |
| Nargs mapping | 5 | 1 | - | 6 |
| Action mapping | 6 | 1 | - | 7 |
| Flag mapping | 4 | - | - | 4 |
| Integration | - | - | 8 | 8 |
| Exception handling | 3 | - | - | 3 |
| **TOTAL** | **33** | **6** | **13** | **52** |

### Property Test Examples

```rust
#[proptest]
fn prop_any_argparse_transpiles(
    #[strategy("[a-z]{3,10}")] arg_name: String,
    #[strategy(python_type_strategy())] py_type: String,
) {
    let python = format!(r#"
import argparse
parser = argparse.ArgumentParser()
parser.add_argument("{}", type={})
args = parser.parse_args()
    "#, arg_name, py_type);

    let rust = transpile(&python);

    // Should not panic
    assert!(!rust.is_empty());

    // Should contain clap import
    assert!(rust.contains("use clap::Parser"));
}
```

## Success Metrics

### Quantitative Goals
- ✅ **Test Pass Rate**: 19/20 (95%)
- ✅ **argparse Coverage**: Support 90% of common argparse patterns
- ✅ **Code Quality**: All functions ≤10 complexity
- ✅ **Test Coverage**: ≥85% line coverage
- ✅ **Performance**: No slowdown vs. current transpiler

### Qualitative Goals
- ✅ Generated Rust code is **idiomatic** (uses clap derive macros)
- ✅ Generated code **compiles** without manual fixes
- ✅ Error messages are **helpful** for unsupported patterns
- ✅ Documentation explains **limitations** clearly

## Risk Mitigation

### High Risk: Breaking Changes
- **Risk**: HIR structure change breaks existing code
- **Mitigation**:
  - Add kwargs as optional field with default
  - Use comprehensive test suite to catch regressions
  - Follow TDD workflow (RED-GREEN-REFACTOR)

### Medium Risk: Complexity Growth
- **Risk**: argparse_transform.rs becomes unmaintainable
- **Mitigation**:
  - Keep functions ≤10 complexity
  - Extract helpers for each mapping type
  - Add extensive documentation

### Low Risk: Unsupported Patterns
- **Risk**: Users try unsupported argparse features
- **Mitigation**:
  - Generate TODO comments for unsupported patterns
  - Document limitations in README
  - Provide helpful error messages

## Timeline

### Fast Track (20 hours over 3 days)
- **Day 1**: Phase 1 (HIR kwargs) - 8 hours
- **Day 2**: Phases 2-4 (Type/nargs/action mapping) - 7 hours
- **Day 3**: Phases 5-7 (Flags/integration/exceptions) - 9 hours

### Conservative (28 hours over 5 days)
- **Day 1**: Phase 1 (HIR kwargs) - 9 hours
- **Day 2**: Phases 2-3 (Type/nargs) - 5 hours
- **Day 3**: Phases 4-5 (Action/flags) - 6 hours
- **Day 4**: Phase 6 (Integration) - 5 hours
- **Day 5**: Phase 7 (Exceptions) + Polish - 3 hours

## Deliverables

### Code
- ✅ 7 new bug documents (DEPYLER-0364 through DEPYLER-0371)
- ✅ 52 new tests (all passing)
- ✅ Updated HIR structure with kwargs support
- ✅ Enhanced argparse_transform.rs with full feature set
- ✅ Fixed try/except code generation

### Documentation
- ✅ Updated CLAUDE.md with HIR extension process
- ✅ Updated README with argparse support status
- ✅ Added examples/argparse_cli with 5+ working examples
- ✅ API documentation for all new functions

### Validation
- ✅ All quality gates pass (clippy, coverage, complexity)
- ✅ No regressions in existing tests
- ✅ Performance metrics show no slowdown
- ✅ Manual testing with real-world argparse scripts

## Next Steps

1. ✅ **Immediate**: Begin Phase 1 (DEPYLER-0364 - HIR kwargs)
2. Update roadmap.yaml with all 7 tickets
3. Follow EXTREME TDD workflow for each phase
4. Commit after each GREEN phase
5. Update this roadmap as work progresses

---

**Note**: This roadmap is ambitious but achievable. The foundation (HIR kwargs) is the critical path - once that's done, the rest is mostly systematic mapping logic.
