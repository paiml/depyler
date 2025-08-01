# Next-Gen Depyler Development Tasks

This document provides a granular, prioritized list of development tasks for the Depyler Python-to-Rust transpiler, based on the current codebase state and CLAUDE.md guidelines.

## Critical Fixes (Priority 1 - Broken Functionality)

### Type Inference & Ownership
- [ ] Fix incorrect ownership patterns in type inference (`type_mapper.rs`)
  - [ ] Implement proper borrowing inference for function parameters
    - [ ] Add `BorrowingContext` struct to track parameter usage
    - [ ] Analyze function body for parameter mutations
    - [ ] Detect parameter escaping (stored in structs/returned)
    - [ ] Generate `&T` vs `&mut T` vs `T` based on usage patterns
    - [ ] Add tests for complex borrowing scenarios
  - [ ] Fix lifetime annotations for string references
    - [ ] Create `LifetimeInference` module in `type_mapper.rs`
    - [ ] Track string origin (literal, parameter, return value)
    - [ ] Implement lifetime elision rules from Rust RFC 141
    - [ ] Add explicit lifetime annotations when needed
    - [ ] Handle string slicing and substring operations
  - [ ] Add ownership transfer validation for method calls
    - [ ] Track ownership state in HIR nodes
    - [ ] Detect move vs borrow for self parameters
    - [ ] Validate no use-after-move
    - [ ] Generate proper `self`, `&self`, `&mut self` signatures
    - [ ] Add error messages for ownership violations
  - [ ] Ensure mutable/immutable borrow checking aligns with Rust semantics
    - [ ] Implement `BorrowChecker` for HIR
    - [ ] Track active borrows with scope information
    - [ ] Detect conflicting borrows (mut + immut)
    - [ ] Add two-phase borrowing support
    - [ ] Generate helpful error messages with suggestions

### String Handling
- [ ] Optimize string allocations (`direct_rules.rs`, `codegen.rs`)
  - [ ] Implement `&str` inference where possible (currently always uses `String`)
    - [ ] Add `StringUsageAnalyzer` to track string usage patterns
    - [ ] Detect read-only string usage
    - [ ] Identify string literals that don't need allocation
    - [ ] Update `convert_literal` to return `&'static str` when safe
    - [ ] Add configuration option for string strategy preference
  - [ ] Add `Cow<'static, str>` support for mixed ownership scenarios
    - [ ] Detect functions that sometimes return literals, sometimes owned
    - [ ] Implement `CowInference` logic
    - [ ] Update type mapper to support `Cow` type generation
    - [ ] Add smart constructors for `Cow::Borrowed` vs `Cow::Owned`
    - [ ] Generate `.into_owned()` calls where necessary
  - [ ] Fix unnecessary `.to_string()` calls in literal conversions
    - [ ] Audit all uses of `.to_string()` in codegen
    - [ ] Replace with `&str` where receiver accepts it
    - [ ] Use `format!` macro instead of multiple allocations
    - [ ] Implement string concatenation optimization
    - [ ] Add benchmarks to measure allocation reduction
  - [ ] Implement string interning for repeated literals
    - [ ] Create `StringInterner` struct with `FxHashMap`
    - [ ] Track string literal usage frequency
    - [ ] Generate `lazy_static!` for frequently used strings
    - [ ] Use `Arc<str>` for shared string data
    - [ ] Add configuration threshold for interning

### Property Verification
- [ ] Fix lifetime violation detection gaps (`verify/memory_safety.rs`)
  - [ ] Add comprehensive lifetime analysis for all HIR nodes
    - [ ] Implement `LifetimeAnalyzer` visitor for HIR
    - [ ] Track lifetime constraints for each expression
    - [ ] Build lifetime dependency graph
    - [ ] Detect cyclic lifetime dependencies
    - [ ] Generate lifetime bounds for generic functions
  - [ ] Implement proper scope tracking for references
    - [ ] Create `ScopeTracker` with nested scope support
    - [ ] Track variable initialization points
    - [ ] Record last use of each variable
    - [ ] Implement non-lexical lifetime analysis
    - [ ] Add debug visualization for scope trees
  - [ ] Fix false negatives in borrow checker integration
    - [ ] Add test cases for known false negatives
    - [ ] Implement path-sensitive analysis
    - [ ] Track conditional borrowing patterns
    - [ ] Handle loop-carried dependencies
    - [ ] Add heuristics for common patterns
  - [ ] Add verification for iterator invalidation patterns
    - [ ] Detect collection modification during iteration
    - [ ] Track iterator lifetimes separately
    - [ ] Implement invalidation rules for each collection type
    - [ ] Generate safe iteration patterns (collect then iterate)
    - [ ] Add warnings for potential invalidation

## Core Features (Priority 2 - Incomplete V1.0)

### Control Flow & Operators
- [ ] Implement `range()` with step parameter support (`direct_rules.rs:355`)
  - [ ] Update `convert_builtin_call` to handle 3-argument range
    - [ ] Parse step parameter from args[2]
    - [ ] Generate `(start..end).step_by(step)` for positive steps
    - [ ] Handle negative steps with `.rev()` and bounds adjustment
    - [ ] Add validation for zero step (should panic like Python)
    - [ ] Test edge cases: negative ranges, large steps
  - [ ] Add range type inference
    - [ ] Infer integer type from arguments
    - [ ] Handle mixed integer types (cast to common type)
    - [ ] Support range over custom types with Step trait
    - [ ] Generate appropriate type annotations
- [ ] Add proper floor division handling (`direct_rules.rs:480`, `rust_gen.rs:532`)
  - [ ] Implement `FloorDiv` operator conversion
    - [ ] Generate `(a / b).floor()` for floating point
    - [ ] Use integer division for integer types
    - [ ] Handle mixed numeric types correctly
    - [ ] Add tests for negative operands
    - [ ] Document Python vs Rust semantics differences
  - [ ] Add optimization pass
    - [ ] Detect integer-only floor division
    - [ ] Avoid unnecessary `.floor()` calls
    - [ ] Use bit shifting for power-of-2 divisors
- [ ] Implement power operator (`**`) transpilation
  - [ ] Add `pow` method selection based on types
    - [ ] Use `.pow()` for integer exponents
    - [ ] Use `.powf()` for floating point
    - [ ] Handle negative exponents (convert to float)
    - [ ] Support modular exponentiation (3-arg pow)
    - [ ] Add overflow checking for integer powers
  - [ ] Import requirements
    - [ ] Add `num_traits::Pow` for generic code
    - [ ] Conditionally import based on usage
    - [ ] Generate type-specific implementations
- [ ] Add break/continue with labels support
  - [ ] Implement label generation for nested loops
    - [ ] Track loop nesting depth
    - [ ] Generate unique labels for each loop level
    - [ ] Map Python loop names to Rust labels
    - [ ] Handle break/continue with count argument
    - [ ] Add scope validation for labels
  - [ ] Update control flow analysis
    - [ ] Track reachability after labeled breaks
    - [ ] Ensure all paths are covered
    - [ ] Generate appropriate return types
- [ ] Implement match/case statement transpilation
  - [ ] Pattern compilation
    - [ ] Convert Python patterns to Rust match arms
    - [ ] Handle literal patterns
    - [ ] Support sequence patterns with `..`
    - [ ] Implement guard clauses (`if` conditions)
    - [ ] Add wildcard and capture patterns
  - [ ] Advanced patterns
    - [ ] Class patterns with attribute matching
    - [ ] OR patterns (`|` in match arms)
    - [ ] AS patterns (capture and match)
    - [ ] Mapping patterns for dictionaries

### Error Handling
- [ ] Complete error propagation patterns
  - [ ] Map Python exceptions to Rust `Result<T, E>` types
    - [ ] Create exception hierarchy mapping
    - [ ] Generate error enums for each module
    - [ ] Map built-in exceptions (ValueError, KeyError, etc.)
    - [ ] Support exception inheritance chains
    - [ ] Add conversion traits between error types
  - [ ] Implement try/except/finally transpilation
    - [ ] Convert try blocks to Result-returning closures
    - [ ] Map except clauses to match arms
    - [ ] Implement finally with Drop trait or defer pattern
    - [ ] Handle multiple except clauses with proper ordering
    - [ ] Support exception binding and re-raising
  - [ ] Add custom error type generation
    - [ ] Generate error structs from Python exception classes
    - [ ] Include error context and backtrace support
    - [ ] Implement Display and Error traits
    - [ ] Add #[derive(thiserror::Error)] when available
    - [ ] Support error wrapping and downcasting
  - [ ] Support error chaining and context
    - [ ] Use anyhow/eyre for context propagation
    - [ ] Generate `.context()` calls from Python comments
    - [ ] Map `raise from` to error sources
    - [ ] Preserve stack traces across boundaries
    - [ ] Add structured error reporting

### Collections
- [ ] Dictionary subscript assignment (`lib.rs:405`)
  - [ ] Implement assignment desugaring
    - [ ] Convert `d[k] = v` to `d.insert(k, v)`
    - [ ] Handle nested assignments `d[k1][k2] = v`
    - [ ] Support tuple key assignments
    - [ ] Add get_mut for update operations
    - [ ] Generate entry API calls for efficiency
  - [ ] Type inference for dictionary operations
    - [ ] Infer key and value types from usage
    - [ ] Handle heterogeneous dictionaries
    - [ ] Support type narrowing after checks
- [ ] List slicing with step parameter
  - [ ] Full slice implementation
    - [ ] Parse Python slice syntax `[start:stop:step]`
    - [ ] Generate iterator chains for positive steps
    - [ ] Handle negative indices correctly
    - [ ] Implement negative step with rev()
    - [ ] Support slice assignment operations
  - [ ] Optimization for common patterns
    - [ ] Detect full reversal `[::-1]`
    - [ ] Use chunks() for regular steps
    - [ ] Avoid allocation when possible
- [ ] Set operations and methods
  - [ ] Implement set type and operations
    - [ ] Map Python set to HashSet/BTreeSet
    - [ ] Implement set operators (&, |, -, ^)
    - [ ] Add set methods (add, remove, discard)
    - [ ] Support set comprehensions
    - [ ] Handle frozen sets as immutable
  - [ ] Set-specific optimizations
    - [ ] Use bitsets for enum sets
    - [ ] Implement small-set optimizations
    - [ ] Cache hash values when beneficial
- [ ] Dictionary comprehensions
  - [ ] Parse and transform comprehensions
    - [ ] Convert to iterator chains with collect()
    - [ ] Handle conditional clauses
    - [ ] Support nested comprehensions
    - [ ] Optimize key/value expressions
    - [ ] Generate type annotations
- [ ] Tuple unpacking in all contexts
  - [ ] Implement full unpacking support
    - [ ] Function parameters unpacking
    - [ ] Assignment unpacking with patterns
    - [ ] For loop unpacking
    - [ ] Support starred expressions
    - [ ] Handle nested unpacking

### Testing Infrastructure
- [ ] Fix linker errors in test suite (missing `ld`)
  - [ ] Environment setup
    - [ ] Add linker installation to CI
    - [ ] Document development prerequisites
    - [ ] Create Docker image with tools
    - [ ] Add automatic tool detection
    - [ ] Provide helpful error messages
- [ ] Add QuickCheck property tests (`verify/quickcheck.rs`)
  - [ ] Property definitions
    - [ ] Transpilation preserves semantics
    - [ ] Type safety is maintained
    - [ ] No panics on valid input
    - [ ] Ownership rules are satisfied
    - [ ] Performance bounds are met
  - [ ] Custom generators
    - [ ] Generate valid Python AST
    - [ ] Create type-annotated functions
    - [ ] Produce nested data structures
    - [ ] Generate edge case values
- [ ] Implement semantic equivalence testing
  - [ ] Execution comparison framework
    - [ ] Run Python and Rust versions
    - [ ] Compare outputs for equality
    - [ ] Handle floating point tolerance
    - [ ] Test side effects and mutations
    - [ ] Measure performance differences
- [ ] Add fuzzing for edge cases
  - [ ] AFL/LibFuzzer integration
    - [ ] Create fuzzing harnesses
    - [ ] Seed with real Python code
    - [ ] Target error paths
    - [ ] Monitor coverage expansion
    - [ ] Add crash minimization
- [ ] Create regression test suite from bug reports
  - [ ] Bug tracking integration
    - [ ] Extract test cases from issues
    - [ ] Add minimized reproductions
    - [ ] Tag tests with issue numbers
    - [ ] Prevent regression with CI
    - [ ] Generate test status reports

## Type System Enhancements (Priority 3 - V1.1 Features)

### Advanced Type Mapping
- [ ] Implement generic type parameter inference
  - [ ] Type variable tracking
    - [ ] Create `TypeVarRegistry` to track TypeVar definitions
    - [ ] Map Python TypeVar to Rust generic parameters
    - [ ] Track constraints and bounds on type variables
    - [ ] Handle variance annotations (covariant, contravariant)
    - [ ] Support default type parameters
  - [ ] Generic function inference
    - [ ] Analyze function body for type constraints
    - [ ] Implement Hindley-Milner type inference
    - [ ] Generate appropriate generic bounds
    - [ ] Handle generic method calls
    - [ ] Support generic return types
  - [ ] Generic class support
    - [ ] Map Generic[T] to Rust struct<T>
    - [ ] Infer type parameters from usage
    - [ ] Handle inheritance with generics
    - [ ] Support multiple type parameters
    - [ ] Generate phantom data for unused parameters
- [ ] Add Union type support with enum generation
  - [ ] Union type analysis
    - [ ] Parse Union[A, B, C] annotations
    - [ ] Generate Rust enum with variants
    - [ ] Implement discriminant inference
    - [ ] Handle Optional[T] as Union[T, None]
    - [ ] Support nested unions
  - [ ] Pattern matching generation
    - [ ] Generate match expressions for union handling
    - [ ] Add exhaustiveness checking
    - [ ] Implement type narrowing after matching
    - [ ] Support if-let patterns for simple cases
    - [ ] Generate helper methods for common patterns
  - [ ] Serialization support
    - [ ] Generate serde tags for variants
    - [ ] Handle untagged unions
    - [ ] Support custom discriminators
    - [ ] Add JSON compatibility
- [ ] Support type aliases and NewType patterns
  - [ ] Type alias handling
    - [ ] Track type alias definitions
    - [ ] Expand aliases during type checking
    - [ ] Generate Rust type aliases
    - [ ] Support generic type aliases
    - [ ] Handle recursive type aliases
  - [ ] NewType implementation
    - [ ] Detect NewType pattern usage
    - [ ] Generate newtype structs
    - [ ] Implement Deref for transparency
    - [ ] Add From/Into conversions
    - [ ] Support pattern matching
- [ ] Implement Protocol to Trait mapping
  - [ ] Protocol detection
    - [ ] Identify Protocol subclasses
    - [ ] Extract required methods
    - [ ] Map to Rust trait definitions
    - [ ] Handle optional methods
    - [ ] Support generic protocols
  - [ ] Trait implementation
    - [ ] Generate trait impls for conforming types
    - [ ] Add blanket implementations
    - [ ] Support associated types
    - [ ] Handle default methods
    - [ ] Generate extension traits
- [ ] Add const generic inference for fixed-size arrays
  - [ ] Array size inference
    - [ ] Detect fixed-size list literals
    - [ ] Track array size constraints
    - [ ] Generate const generic parameters
    - [ ] Support array operations
    - [ ] Handle multi-dimensional arrays
  - [ ] Const evaluation
    - [ ] Evaluate compile-time constants
    - [ ] Support const expressions
    - [ ] Handle const propagation
    - [ ] Generate const assertions

### Lifetime System
- [ ] Implement lifetime elision rules
  - [ ] Basic elision rules
    - [ ] Single input lifetime → output lifetime
    - [ ] Multiple inputs → explicit annotation
    - [ ] Self reference rules for methods
    - [ ] Static lifetime detection
    - [ ] Trait object lifetime defaults
  - [ ] Context-aware elision
    - [ ] Track method context
    - [ ] Apply struct lifetime parameters
    - [ ] Handle closure captures
    - [ ] Support async function lifetimes
- [ ] Add support for multiple lifetime parameters
  - [ ] Lifetime relationship analysis
    - [ ] Build lifetime constraint graph
    - [ ] Detect outlives relationships
    - [ ] Generate minimal lifetime sets
    - [ ] Handle lifetime intersection
    - [ ] Support lifetime bounds
  - [ ] Lifetime naming
    - [ ] Generate meaningful lifetime names
    - [ ] Avoid conflicts with user names
    - [ ] Use conventional patterns ('a, 'b)
    - [ ] Document lifetime purposes
- [ ] Implement lifetime bounds inference
  - [ ] Bound detection
    - [ ] Analyze type usage patterns
    - [ ] Infer required lifetime bounds
    - [ ] Generate where clauses
    - [ ] Handle transitive bounds
    - [ ] Support for<'a> bounds
  - [ ] Bound optimization
    - [ ] Remove redundant bounds
    - [ ] Simplify bound expressions
    - [ ] Use implied bounds
    - [ ] Generate minimal bound sets
- [ ] Support higher-ranked trait bounds (HRTB)
  - [ ] HRTB detection
    - [ ] Identify higher-ranked lifetimes
    - [ ] Generate for<'a> syntax
    - [ ] Handle closure arguments
    - [ ] Support Fn trait bounds
    - [ ] Map to appropriate traits
- [ ] Add self-referential struct support
  - [ ] Safety analysis
    - [ ] Detect self-referential patterns
    - [ ] Generate pin-based solutions
    - [ ] Use rental/owning_ref patterns
    - [ ] Add safety documentation
    - [ ] Warn about limitations

### Dataclass Support
- [ ] Convert `@dataclass` to Rust structs
  - [ ] Dataclass detection and parsing
    - [ ] Identify @dataclass decorator
    - [ ] Parse class attributes with types
    - [ ] Handle default values
    - [ ] Support field() specifications
    - [ ] Extract metadata (frozen, order, etc.)
  - [ ] Struct generation
    - [ ] Generate Rust struct definition
    - [ ] Add visibility modifiers
    - [ ] Include documentation comments
    - [ ] Handle generic dataclasses
    - [ ] Support inheritance
- [ ] Implement field ownership inference
  - [ ] Ownership analysis
    - [ ] Analyze field usage patterns
    - [ ] Determine owned vs borrowed fields
    - [ ] Handle circular references
    - [ ] Infer Arc/Rc for shared data
    - [ ] Support weak references
  - [ ] Smart pointer selection
    - [ ] Choose Box for single ownership
    - [ ] Use Rc for shared ownership
    - [ ] Apply Arc for thread-safe sharing
    - [ ] Add RefCell for interior mutability
    - [ ] Generate accessor methods
- [ ] Add builder pattern generation
  - [ ] Builder struct creation
    - [ ] Generate builder with optional fields
    - [ ] Add type-state for required fields
    - [ ] Implement fluent interface
    - [ ] Support default values
    - [ ] Handle validation
  - [ ] Builder methods
    - [ ] Generate setter methods
    - [ ] Add batch update methods
    - [ ] Implement build() with validation
    - [ ] Support conditional fields
    - [ ] Add convenience constructors
- [ ] Support frozen dataclasses as immutable structs
  - [ ] Immutability enforcement
    - [ ] Generate all fields as immutable
    - [ ] Remove setter methods
    - [ ] Add const constructors where possible
    - [ ] Implement structural sharing
    - [ ] Support persistent data structures
  - [ ] Optimization opportunities
    - [ ] Use Copy trait for small types
    - [ ] Implement hash caching
    - [ ] Add memory pooling
    - [ ] Support zero-copy cloning
- [ ] Generate appropriate derive macros
  - [ ] Standard derives
    - [ ] Debug for all dataclasses
    - [ ] Clone based on field types
    - [ ] PartialEq/Eq when ordered=False
    - [ ] PartialOrd/Ord when ordered=True
    - [ ] Hash for hashable types
  - [ ] Serde integration
    - [ ] Add Serialize/Deserialize
    - [ ] Configure field renaming
    - [ ] Handle skip conditions
    - [ ] Support custom serializers
    - [ ] Generate schema information

## Verification System (Priority 4 - Quality Gates)

### Contract Implementation
- [ ] Replace TODO placeholders in contract checks (`contracts.rs:94,108`)
- [ ] Implement SMT solver integration for preconditions
- [ ] Add postcondition verification with symbolic execution
- [ ] Support invariant checking across function calls
- [ ] Generate runtime assertion code when verification fails

### Memory Safety
- [ ] Implement complete alias analysis
- [ ] Add use-after-move detection
- [ ] Verify absence of data races
- [ ] Check for memory leaks in cyclic structures
- [ ] Validate unsafe block usage

### Performance Verification
- [ ] Add complexity analysis verification
- [ ] Implement termination checking for loops
- [ ] Verify bounded recursion
- [ ] Add worst-case memory usage analysis
- [ ] Generate performance contracts

## Advanced Features (Priority 5 - V1.2+)

### Async/Await Support
- [ ] Implement async function detection (`grep` shows stubs)
- [ ] Map Python async/await to Rust async syntax
- [ ] Handle async context managers
- [ ] Support async generators
- [ ] Add tokio/async-std runtime selection

### Iterator Protocol
- [ ] Map Python iterators to Rust Iterator trait
- [ ] Implement generator to iterator conversion
- [ ] Support yield expressions
- [ ] Add lazy evaluation patterns
- [ ] Optimize iterator chains

### Lambda & Closures
- [ ] Complete lambda inference (`lambda_*.rs` files)
- [ ] Support capturing variables by value/reference
- [ ] Implement closure lifetime inference
- [ ] Add move semantics for closures
- [ ] Support higher-order functions

### Class Support (Future)
- [ ] Basic class to struct+impl conversion
- [ ] Single inheritance support
- [ ] Method resolution order (MRO) handling
- [ ] Property decorators (@property)
- [ ] Class method and static method support

## Tooling & Integration (Priority 6)

### MCP Integration
- [ ] Leverage completed pmcp upgrade (see `pmcp-upgrade-tasks.md`)
- [ ] Add resource support for code artifacts
- [ ] Implement prompt-based workflows
- [ ] Add progress notifications for long operations
- [ ] Support collaborative transpilation sessions

### CLI Improvements
- [ ] Add incremental transpilation support
- [ ] Implement watch mode for continuous transpilation
- [ ] Add project-wide transpilation commands
- [ ] Support configuration profiles
- [ ] Add migration wizard for large codebases

### IDE Support
- [ ] Implement Language Server Protocol (LSP)
- [ ] Add real-time transpilation preview
- [ ] Support inline error annotations
- [ ] Add quick-fix suggestions
- [ ] Implement refactoring support

## Quality & Performance (Priority 7)

### Code Generation Quality
- [ ] Integrate rustfmt properly (`rust_gen.rs:635`)
- [ ] Generate idiomatic Rust patterns
- [ ] Add clippy lint compliance
- [ ] Optimize generated code size
- [ ] Support `#[no_std]` environments

### Benchmarking
- [ ] Fix benchmark compilation issues
- [ ] Add micro-benchmarks for each transpilation phase
- [ ] Implement performance regression testing
- [ ] Add memory usage profiling
- [ ] Create performance comparison matrix

### Documentation
- [ ] Generate API documentation from transpiled code
- [ ] Add inline documentation preservation
- [ ] Create transpilation report generation
- [ ] Add migration guides for common patterns
- [ ] Implement example showcase

## Testing Strategy

### Immediate Actions
1. Fix linker configuration for test environment
2. Run existing test suite and catalog failures
3. Add missing test coverage for core features
4. Implement property-based testing framework

### Test Categories
- Unit tests for each transpilation rule
- Integration tests for full programs
- Property tests for correctness invariants
- Performance benchmarks
- Fuzzing for edge cases
- Semantic equivalence validation

## Success Metrics

### V1.0 Release Criteria
- [ ] 90% test coverage on safe Python subset
- [ ] All generated Rust code passes `cargo check`
- [ ] No panics in transpiler for valid Python input
- [ ] Binary size under 5MB
- [ ] Transpilation speed > 20MB/s

### Quality Gates
- [ ] Zero unsoundness bugs
- [ ] All code passes `clippy::pedantic`
- [ ] Full PMAT validation compliance
- [ ] Documentation coverage > 95%
- [ ] CI/CD pipeline fully green

## Development Workflow

### For Each Feature
1. Write comprehensive tests first
2. Implement with complete error handling
3. Add property verification
4. Ensure idiomatic Rust output
5. Document with examples
6. Benchmark performance impact

### Code Review Checklist
- [ ] No `unimplemented!()` or `todo!()` macros
- [ ] Complete error handling paths
- [ ] Property verification included
- [ ] Tests pass without warnings
- [ ] Documentation updated
- [ ] Benchmarks show no regression

## Next Steps

1. **Fix test environment** - Resolve linker errors
2. **Complete type inference** - Fix ownership/borrowing bugs
3. **Implement missing operators** - Floor div, power, etc.
4. **Add verification tests** - Property-based testing
5. **String optimization** - Reduce allocations
6. **Document patterns** - Migration guides

This roadmap follows the 自働化 (Jidoka) principle: build quality in, never ship incomplete transpilation.