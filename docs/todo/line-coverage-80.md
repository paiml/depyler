# Line Coverage 80% Target Plan

## Current Status
- **Overall Line Coverage**: 63.86% (18,333/28,710 lines)
- **Target**: 80% coverage
- **Lines Needed**: ~4,635 additional lines to reach 80%

## Priority Modules (Lowest Coverage)

### Critical Modules (<35% coverage)
1. **depyler-wasm/src/lib.rs** - 0% (452 lines missed)
2. **depyler/src/interactive.rs** - 0% (570 lines missed)
3. **depyler-mcp/src/error.rs** - 0% (17 lines missed)
4. **depyler-verify/src/lib.rs** - 0% (154 lines missed)
5. **depyler/src/debug_cmd.rs** - 0% (59 lines missed)
6. **depyler-core/src/module_mapper.rs** - 2.63% (73 lines missed)
7. **depyler-core/src/direct_rules.rs** - 34.42% (1,124 lines missed)
8. **depyler-core/src/rust_gen.rs** - 34.33% (1,236 lines missed)
9. **depyler-core/src/ast_bridge/type_extraction.rs** - 32.03% (87 lines missed)
10. **depyler-verify/src/lifetime_analysis.rs** - 32.37% (257 lines missed)

### Medium Priority (35-50% coverage)
1. **depyler-core/src/ast_bridge.rs** - 56.05% (458 lines missed)
2. **depyler-core/src/ast_bridge/converters.rs** - 41.37% (197 lines missed)
3. **depyler/src/lib.rs** - 43.95% (537 lines missed)
4. **depyler-core/src/migration_suggestions.rs** - 43.89% (349 lines missed)
5. **depyler-core/src/lsp.rs** - 45.34% (129 lines missed)
6. **depyler-verify/src/memory_safety.rs** - 46.04% (177 lines missed)
7. **depyler-core/src/lifetime_analysis.rs** - 49.35% (272 lines missed)

## Implementation Strategy

### Phase 1: Zero Coverage Modules (Quick Wins)
These modules have 0% coverage and are relatively small, making them ideal starting points.

#### Task 1: depyler-mcp/src/error.rs (17 lines)
- [ ] Add unit tests for error types
- [ ] Add doctest example showing error usage
- [ ] Create property test for error conversion
- [ ] Add example demonstrating error handling

#### Task 2: depyler/src/debug_cmd.rs (59 lines)
- [ ] Add unit tests for debug command
- [ ] Add doctest for public functions
- [ ] Create integration test
- [ ] Add example showing debug usage

#### Task 3: depyler-verify/src/lib.rs (154 lines)
- [ ] Add unit tests for public API
- [ ] Add doctests for verification functions
- [ ] Create property tests for verification logic
- [ ] Add example demonstrating verification

### Phase 2: WASM Module (452 lines)
The WASM module is completely untested but critical for web deployment.

#### Task 4: depyler-wasm/src/lib.rs (452 lines)
- [ ] Add WASM-specific unit tests
- [ ] Add doctests for WASM API
- [ ] Create integration tests with wasm-bindgen-test
- [ ] Add example showing WASM usage

#### Task 5: depyler-wasm/src/tests.rs (241 lines)
- [ ] Enable existing tests
- [ ] Add comprehensive WASM tests
- [ ] Add property tests for WASM bindings
- [ ] Document WASM testing approach

### Phase 3: Interactive Module (570 lines)
The interactive module is important for user experience.

#### Task 6: depyler/src/interactive.rs (570 lines)
- [ ] Add unit tests for REPL functionality
- [ ] Add doctests for interactive commands
- [ ] Create integration tests for user flows
- [ ] Add example showing interactive usage

### Phase 4: Core Transpilation (High Impact)
These are the largest uncovered areas in core functionality.

#### Task 7: depyler-core/src/direct_rules.rs (1,124 lines)
- [ ] Add tests for each direct rule
- [ ] Add doctests for rule application
- [ ] Create property tests for rule correctness
- [ ] Add examples for complex rules

#### Task 8: depyler-core/src/rust_gen.rs (1,236 lines)
- [ ] Add tests for Rust code generation
- [ ] Add doctests for generation functions
- [ ] Create property tests for code generation
- [ ] Add examples of generated code

### Phase 5: AST Bridge Components
These modules handle Python to Rust AST conversion.

#### Task 9: depyler-core/src/ast_bridge.rs (458 lines)
- [ ] Add tests for AST conversion
- [ ] Add doctests for bridge functions
- [ ] Create property tests for AST mapping
- [ ] Add examples of AST transformations

#### Task 10: depyler-core/src/ast_bridge/converters.rs (197 lines)
- [ ] Add tests for each converter
- [ ] Add doctests for conversion functions
- [ ] Create property tests for conversions
- [ ] Add examples of conversions

#### Task 11: depyler-core/src/ast_bridge/type_extraction.rs (87 lines)
- [ ] Add tests for type extraction
- [ ] Add doctests for extraction functions
- [ ] Create property tests for type inference
- [ ] Add examples of type extraction

### Phase 6: Verification and Analysis
These modules handle code verification and analysis.

#### Task 12: depyler-verify/src/lifetime_analysis.rs (257 lines)
- [ ] Add tests for lifetime analysis
- [ ] Add doctests for analysis functions
- [ ] Create property tests for lifetime rules
- [ ] Add examples of lifetime analysis

#### Task 13: depyler-verify/src/memory_safety.rs (177 lines)
- [ ] Add tests for memory safety checks
- [ ] Add doctests for safety functions
- [ ] Create property tests for safety rules
- [ ] Add examples of memory safety

### Phase 7: Supporting Features
These modules provide supporting functionality.

#### Task 14: depyler-core/src/migration_suggestions.rs (349 lines)
- [ ] Add tests for migration suggestions
- [ ] Add doctests for suggestion functions
- [ ] Create property tests for suggestions
- [ ] Add examples of migrations

#### Task 15: depyler-core/src/lsp.rs (129 lines)
- [ ] Add tests for LSP functionality
- [ ] Add doctests for LSP handlers
- [ ] Create integration tests for LSP
- [ ] Add examples of LSP usage

## Test Implementation Guidelines

### For Each Module:
1. **Unit Tests**: Cover all public functions and important private ones
2. **Doctests**: Add examples to all public APIs
3. **Property Tests**: Create at least 2-3 property tests per module
4. **Examples**: Create realistic usage examples
5. **Integration Tests**: Test module interactions

### Test Quality Requirements:
- Each test should be meaningful (no trivial tests)
- Tests should cover edge cases
- Property tests should use realistic inputs
- Examples should demonstrate real-world usage
- Doctests should be educational

### Coverage Targets by Phase:
- Phase 1: +230 lines (~0.8% increase)
- Phase 2: +693 lines (~2.4% increase)
- Phase 3: +570 lines (~2.0% increase)
- Phase 4: +2,360 lines (~8.2% increase)
- Phase 5: +742 lines (~2.6% increase)
- Phase 6: +434 lines (~1.5% increase)
- Phase 7: +478 lines (~1.7% increase)

**Total Expected Coverage Increase**: ~19.2% (from 63.86% to ~83%)

## Execution Order
1. Start with Phase 1 (quick wins) to build momentum
2. Move to Phase 2-3 (WASM and Interactive) for user-facing features
3. Tackle Phase 4 (Core) for maximum impact
4. Complete Phase 5-7 to reach and exceed 80%

## Success Metrics
- Line coverage ≥ 80%
- All new tests pass
- No regression in existing tests
- Documentation coverage increases
- Examples compile and run