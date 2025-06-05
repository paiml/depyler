# Deep Context Analysis

## Executive Summary

Generated: 2025-06-05 01:59:20.966124130 UTC
Version: 0.21.0
Analysis Time: 0.72s
Cache Hit Rate: 0.0%

## Quality Scorecard

- **Overall Health**: ⚠️ (75.0/100)
- **Maintainability Index**: 70.0
- **Technical Debt**: 40.0 hours estimated

## Project Structure

```
└── /
    ├── README.md
    ├── python-stdlib-stubs/
    ├── CLAUDE.md
    ├── RELEASE_SUMMARY.md
    ├── LICENSE
    ├── Cargo.toml
    ├── MCP_SUBMISSION.md
    ├── .gitignore
    ├── deep_context.md
    ├── docs/
    │   ├── cli-reference.md
    │   ├── user-guide.md
    │   ├── improvements-summary.md
    │   ├── v0-spec.md
    │   ├── mcp-integration.md
    │   ├── project-overview.md
    │   ├── mcpv1-spec.md
    │   ├── energy-efficiency.md
    │   └── enterprise/
    ├── .git/
    ├── RELEASE.md
    ├── cobertura.xml
    ├── tests/
    │   ├── validation/
    │   │   └── rustc_compilation.rs
    │   ├── transpilation/
    │   │   └── test_basic.rs
    │   ├── functional_tests.rs
    │   ├── integration/
    │   │   ├── semantic/
    │   │   ├── transpilation/
    │   │   ├── type_inference/
    │   │   ├── transpilation_tests.rs
    │   │   ├── semantic_equivalence.rs
    │   │   └── runtime/
    │   ├── fixtures/
    │   │   ├── edge_cases/
    │   │   ├── python_samples/
    │   │   │   ├── basic_functions.py
    │   │   │   ├── control_flow.py
    │   │   │   ├── dictionary_operations.py
    │   │   │   ├── edge_cases.py
    │   │   │   ├── string_operations.py
    │   │   │   └── list_operations.py
    │   │   └── expected_rust/
    │   │       ├── basic_functions.rs
    │   │       └── list_operations.rs
    │   ├── unit/
    │   └── semantics/
    ├── test_example.py
    ├── simple_test.rs
    ├── ROADMAP.md
    ├── benches/
    │   ├── memory_usage.rs
    │   ├── binary_size.rs
    │   └── transpilation.rs
    ├── .idea/
    │   ├── modules.xml
    │   ├── .gitignore
    │   ├── workspace.xml
    │   ├── vcs.xml
    │   └── depyler.iml
    ├── crates/
    │   ├── depyler-verify/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       ├── properties.rs
    │   │       ├── contracts.rs
    │   │       ├── quickcheck.rs
    │   │       └── lib.rs
    │   ├── depyler/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       └── main.rs
    │   ├── depyler-core/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       ├── codegen.rs
    │   │       ├── direct_rules.rs
    │   │       ├── rust_gen.rs
    │   │       ├── error.rs
    │   │       ├── type_mapper.rs
    │   │       ├── lib.rs
    │   │       ├── ast_bridge.rs
    │   │       └── hir.rs
    │   ├── depyler-analyzer/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       ├── complexity.rs
    │   │       ├── type_flow.rs
    │   │       ├── metrics.rs
    │   │       └── lib.rs
    │   └── depyler-mcp/
    │       ├── Cargo.toml
    │       └── src/
    │           ├── validator.rs
    │           ├── tests.rs
    │           ├── protocol.rs
    │           ├── error.rs
    │           ├── tools.rs
    │           ├── lib.rs
    │           └── server.rs
    ├── Makefile
    ├── coverage.lcov
    ├── examples/
    │   ├── validation/
    │   │   └── test_all.py
    │   ├── demo.rs
    │   ├── demo.py
    │   ├── showcase/
    │   │   ├── calculate_sum.py
    │   │   ├── process_config.py
    │   │   ├── classify_number.py
    │   │   └── binary_search.py
    │   └── mcp_usage.py
    ├── simple_test.py
    ├── Cargo.lock
    ├── CHANGELOG.md
    ├── scripts/
    │   ├── prepare-release.sh
    │   ├── track_binary_size.sh
    │   └── run_comprehensive_tests.sh
    ├── RELEASE_NOTES_v0.1.0.md
    ├── .github/
    │   └── workflows/
    │       ├── release.yml
    │       └── ci.yml
    ├── target/
    ├── test_mcp_functionality.rs
    └── .cargo/
        └── config.toml

📊 Total Files: 92, Total Size: 774604 bytes
```

## Enhanced AST Analysis

### ./benches/binary_size.rs

**Language:** rust
**Total Symbols:** 13
**Functions:** 7 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Functions:**
  - `measure_binary_size` (private) at line 1
  - `get_section_sizes` (private) at line 1
  - `bench_binary_size_profiles` (private) at line 1
  - `bench_feature_size_impact` (private) at line 1
  - `bench_compilation_speed_vs_size` (private) at line 1
  - `bench_strip_and_compression_impact` (private) at line 1
  - `bench_dependency_size_impact` (private) at line 1

**Structs:**
  - `SectionSizes` (private) with 4 fields (derives: derive) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 2.66

**TDG Severity:** Critical

### ./benches/memory_usage.rs

**Language:** rust
**Total Symbols:** 15
**Functions:** 9 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Functions:**
  - `reset_memory_tracking` (public) at line 1
  - `get_current_allocation` (public) at line 1
  - `get_peak_allocation` (public) at line 1
  - `generate_memory_test_source` (private) at line 1
  - `bench_ast_memory_usage` (private) at line 1
  - `bench_hir_memory_usage` (private) at line 1
  - `bench_transpilation_memory_efficiency` (private) at line 1
  - `bench_memory_leaks_detection` (private) at line 1
  - `bench_memory_fragmentation` (private) at line 1

**Structs:**
  - `TrackingAllocator` (public) with 0 fields at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 2.09

**TDG Severity:** Warning

### ./benches/transpilation.rs

**Language:** rust
**Total Symbols:** 12
**Functions:** 9 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Functions:**
  - `generate_python_source` (private) at line 1
  - `bench_parsing_performance` (private) at line 1
  - `bench_ast_to_hir_conversion` (private) at line 1
  - `bench_type_inference` (private) at line 1
  - `bench_rust_codegen` (private) at line 1
  - `bench_end_to_end_transpilation` (private) at line 1
  - `bench_verification_overhead` (private) at line 1
  - `bench_real_world_scenarios` (private) at line 1
  - `bench_scalability_stress_test` (private) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.56

**TDG Severity:** Warning

### ./crates/depyler/src/main.rs

**Language:** rust
**Total Symbols:** 17
**Functions:** 5 | **Structs:** 1 | **Enums:** 1 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 10

**Functions:**
  - `main` (private) at line 1
  - `transpile_command` (private) at line 1
  - `analyze_command` (private) at line 1
  - `check_command` (private) at line 1
  - `complexity_rating` (private) at line 1

**Structs:**
  - `Cli` (private) with 2 fields (derives: derive) at line 1

**Enums:**
  - `Commands` (private) with 3 variants at line 1

**Imports:** 10 import statements

**Technical Debt Gradient:** 1.46

**TDG Severity:** Normal

### ./crates/depyler-analyzer/src/complexity.rs

**Language:** rust
**Total Symbols:** 11
**Functions:** 10 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 1

**Functions:**
  - `calculate_cyclomatic` (public) at line 1
  - `cyclomatic_stmt` (private) at line 1
  - `cyclomatic_body` (private) at line 1
  - `cyclomatic_expr` (private) at line 1
  - `calculate_cognitive` (public) at line 1
  - `cognitive_body` (private) at line 1
  - `cognitive_stmt` (private) at line 1
  - `cognitive_condition` (private) at line 1
  - `calculate_max_nesting` (public) at line 1
  - `count_statements` (public) at line 1

**Key Imports:**
  - `use statement` at line 1

**Technical Debt Gradient:** 1.36

**TDG Severity:** Normal

### ./crates/depyler-analyzer/src/lib.rs

**Language:** rust
**Total Symbols:** 18
**Functions:** 6 | **Structs:** 5 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 7

**Functions:**
  - `create_test_function` (private) at line 1
  - `test_analyzer_creation` (private) at line 1
  - `test_analyze_empty_module` (private) at line 1
  - `test_analyze_single_function` (private) at line 1
  - `test_type_coverage_calculation` (private) at line 1
  - `test_module_metrics_calculation` (private) at line 1

**Structs:**
  - `AnalysisResult` (public) with 3 fields (derives: derive) at line 1
  - `ModuleMetrics` (public) with 6 fields (derives: derive) at line 1
  - `FunctionMetrics` (public) with 8 fields (derives: derive) at line 1
  - `TypeCoverage` (public) with 5 fields (derives: derive) at line 1
  - `Analyzer` (public) with 1 field at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.23

**TDG Severity:** Normal

### ./crates/depyler-analyzer/src/metrics.rs

**Language:** rust
**Total Symbols:** 6
**Functions:** 0 | **Structs:** 4 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Structs:**
  - `TranspilationMetrics` (public) with 9 fields (derives: derive) at line 1
  - `QualityMetrics` (public) with 6 fields (derives: derive) at line 1
  - `ComplexityDistribution` (public) with 4 fields (derives: derive) at line 1
  - `PerformanceProfile` (public) with 4 fields (derives: derive) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.14

**TDG Severity:** Normal

### ./crates/depyler-analyzer/src/type_flow.rs

**Language:** rust
**Total Symbols:** 8
**Functions:** 0 | **Structs:** 3 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Structs:**
  - `TypeEnvironment` (public) with 2 fields (derives: derive) at line 1
  - `FunctionSignature` (public) with 2 fields (derives: derive) at line 1
  - `TypeInferencer` (public) with 1 field at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.97

**TDG Severity:** Warning

### ./crates/depyler-core/src/ast_bridge.rs

**Language:** rust
**Total Symbols:** 43
**Functions:** 37 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 6

**Functions:**
  - `python_to_hir` (public) at line 1
  - `convert_module` (private) at line 1
  - `convert_function` (private) at line 1
  - `convert_parameters` (private) at line 1
  - `extract_return_type` (private) at line 1
  - `extract_type` (private) at line 1
  - `convert_body` (private) at line 1
  - `convert_stmt` (private) at line 1
  - `extract_assign_target` (private) at line 1
  - `convert_expr` (private) at line 1
  - ... and 27 more functions

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 2.62

**TDG Severity:** Critical

### ./crates/depyler-core/src/codegen.rs

**Language:** rust
**Total Symbols:** 28
**Functions:** 23 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Functions:**
  - `generate_rust` (public) at line 1
  - `hir_to_rust` (public) at line 1
  - `needs_std_collections` (private) at line 1
  - `uses_hashmap` (private) at line 1
  - `function_body_uses_hashmap` (private) at line 1
  - `stmt_uses_hashmap` (private) at line 1
  - `expr_uses_hashmap` (private) at line 1
  - `convert_function_to_rust` (private) at line 1
  - `type_to_rust_type` (private) at line 1
  - `stmt_to_rust_tokens` (private) at line 1
  - ... and 13 more functions

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.89

**TDG Severity:** Warning

### ./crates/depyler-core/src/direct_rules.rs

**Language:** rust
**Total Symbols:** 35
**Functions:** 27 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 7

**Functions:**
  - `apply_rules` (public) at line 1
  - `convert_function` (private) at line 1
  - `rust_type_to_syn` (private) at line 1
  - `convert_body` (private) at line 1
  - `convert_stmt` (private) at line 1
  - `convert_block` (private) at line 1
  - `convert_expr` (private) at line 1
  - `convert_literal` (private) at line 1
  - `convert_binop` (private) at line 1
  - `create_test_type_mapper` (private) at line 1
  - ... and 17 more functions

**Structs:**
  - `ExprConverter` (private) with 1 field at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.82

**TDG Severity:** Warning

### ./crates/depyler-core/src/error.rs

**Language:** rust
**Total Symbols:** 12
**Functions:** 5 | **Structs:** 2 | **Enums:** 1 | **Traits:** 1 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Functions:**
  - `test_error_creation` (private) at line 1
  - `test_error_with_location` (private) at line 1
  - `test_error_with_context` (private) at line 1
  - `test_error_display` (private) at line 1
  - `test_transpile_error_macro` (private) at line 1

**Structs:**
  - `SourceLocation` (public) with 3 fields (derives: derive) at line 1
  - `TranspileError` (public) with 4 fields (derives: derive) at line 1

**Enums:**
  - `ErrorKind` (public) with 7 variants at line 1

**Traits:**
  - `ResultExt` (public) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.21

**TDG Severity:** Normal

### ./crates/depyler-core/src/hir.rs

**Language:** rust
**Total Symbols:** 13
**Functions:** 0 | **Structs:** 4 | **Enums:** 7 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Structs:**
  - `HirModule` (public) with 2 fields (derives: derive) at line 1
  - `Import` (public) with 2 fields (derives: derive) at line 1
  - `HirFunction` (public) with 5 fields (derives: derive) at line 1
  - `FunctionProperties` (public) with 4 fields (derives: derive) at line 1

**Enums:**
  - `ImportItem` (public) with 2 variants at line 1
  - `HirStmt` (public) with 6 variants at line 1
  - `HirExpr` (public) with 11 variants at line 1
  - `Literal` (public) with 5 variants at line 1
  - `BinOp` (public) with 22 variants at line 1
  - ... and 2 more enums

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.08

**TDG Severity:** Normal

### ./crates/depyler-core/src/lib.rs

**Language:** rust
**Total Symbols:** 25
**Functions:** 10 | **Structs:** 8 | **Enums:** 1 | **Traits:** 1 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Functions:**
  - `test_pipeline_creation` (private) at line 1
  - `test_pipeline_with_verification` (private) at line 1
  - `test_config_creation` (private) at line 1
  - `test_simple_transpilation` (private) at line 1
  - `test_parse_to_hir` (private) at line 1
  - `test_validation_result` (private) at line 1
  - `test_invalid_python_syntax` (private) at line 1
  - `test_analyzable_stage_trait` (private) at line 1
  - `test_complex_function_transpilation` (private) at line 1
  - `test_type_annotations` (private) at line 1

**Structs:**
  - `DepylerPipeline` (public) with 4 fields (derives: derive) at line 1
  - `CoreAnalyzer` (public) with 2 fields (derives: derive) at line 1
  - `DirectTranspiler` (public) with 1 field (derives: derive) at line 1
  - `PropertyVerifier` (public) with 2 fields (derives: derive) at line 1
  - `LazyMcpClient` (public) with 1 field (derives: derive) at line 1
  - ... and 3 more structs

**Enums:**
  - `OptimizationLevel` (public) with 3 variants at line 1

**Traits:**
  - `AnalyzableStage` (public) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.24

**TDG Severity:** Normal

### ./crates/depyler-core/src/rust_gen.rs

**Language:** rust
**Total Symbols:** 23
**Functions:** 12 | **Structs:** 1 | **Enums:** 0 | **Traits:** 2 | **Impls:** 0 | **Modules:** 0 | **Imports:** 8

**Functions:**
  - `generate_rust_file` (public) at line 1
  - `literal_to_rust_expr` (private) at line 1
  - `convert_binop` (private) at line 1
  - `rust_type_to_syn` (private) at line 1
  - `format_rust_code` (private) at line 1
  - `create_test_context` (private) at line 1
  - `test_simple_function_generation` (private) at line 1
  - `test_control_flow_generation` (private) at line 1
  - `test_list_generation` (private) at line 1
  - `test_dict_generation_sets_needs_hashmap` (private) at line 1
  - ... and 2 more functions

**Structs:**
  - `CodeGenContext` (public) with 2 fields at line 1

**Traits:**
  - `RustCodeGen` (public) at line 1
  - `ToRustExpr` (private) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 2.18

**TDG Severity:** Warning

### ./crates/depyler-core/src/type_mapper.rs

**Language:** rust
**Total Symbols:** 21
**Functions:** 13 | **Structs:** 1 | **Enums:** 4 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Functions:**
  - `test_default_type_mapper` (private) at line 1
  - `test_type_mapper_creation` (private) at line 1
  - `test_basic_type_mapping` (private) at line 1
  - `test_width_preference` (private) at line 1
  - `test_complex_type_mapping` (private) at line 1
  - `test_tuple_mapping` (private) at line 1
  - `test_return_type_mapping` (private) at line 1
  - `test_needs_reference` (private) at line 1
  - `test_can_copy` (private) at line 1
  - `test_rust_type_to_string` (private) at line 1
  - ... and 3 more functions

**Structs:**
  - `TypeMapper` (public) with 2 fields (derives: derive) at line 1

**Enums:**
  - `IntWidth` (public) with 3 variants at line 1
  - `StringStrategy` (public) with 3 variants at line 1
  - `RustType` (public) with 13 variants at line 1
  - `PrimitiveType` (public) with 15 variants at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.62

**TDG Severity:** Warning

### ./crates/depyler-mcp/src/error.rs

**Language:** rust
**Total Symbols:** 3
**Functions:** 0 | **Structs:** 0 | **Enums:** 1 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Enums:**
  - `DepylerMcpError` (public) with 8 variants at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.07

**TDG Severity:** Normal

### ./crates/depyler-mcp/src/lib.rs

**Language:** rust
**Total Symbols:** 15
**Functions:** 0 | **Structs:** 8 | **Enums:** 1 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 6

**Structs:**
  - `McpClient` (public) with 2 fields (derives: derive) at line 1
  - `McpTranspilationRequest` (public) with 4 fields (derives: derive) at line 1
  - `ErrorContext` (public) with 3 fields (derives: derive) at line 1
  - `Location` (public) with 2 fields (derives: derive) at line 1
  - `QualityHints` (public) with 3 fields (derives: derive) at line 1
  - ... and 3 more structs

**Enums:**
  - `StyleLevel` (public) with 3 variants at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.09

**TDG Severity:** Normal

### ./crates/depyler-mcp/src/protocol.rs

**Language:** rust
**Total Symbols:** 14
**Functions:** 0 | **Structs:** 12 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Structs:**
  - `McpMessage` (public) with 3 fields (derives: derive) at line 1
  - `McpResponse` (public) with 3 fields (derives: derive) at line 1
  - `McpError` (public) with 3 fields (derives: derive) at line 1
  - `InitializeParams` (public) with 3 fields (derives: derive) at line 1
  - `ClientCapabilities` (public) with 1 field (derives: derive) at line 1
  - ... and 7 more structs

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.10

**TDG Severity:** Normal

### ./crates/depyler-mcp/src/server.rs

**Language:** rust
**Total Symbols:** 11
**Functions:** 0 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 10

**Structs:**
  - `DepylerMcpServer` (public) with 2 fields at line 1

**Imports:** 10 import statements

**Technical Debt Gradient:** 1.44

**TDG Severity:** Normal

### ./crates/depyler-mcp/src/tests.rs

**Language:** rust
**Total Symbols:** 19
**Functions:** 13 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 6

**Functions:**
  - `test_initialize (async)` (private) at line 1
  - `test_tools_list (async)` (private) at line 1
  - `test_transpile_python_inline (async)` (private) at line 1
  - `test_analyze_migration_complexity (async)` (private) at line 1
  - `test_verify_transpilation (async)` (private) at line 1
  - `test_invalid_tool_call (async)` (private) at line 1
  - `test_invalid_method (async)` (private) at line 1
  - `test_transpile_request_deserialization` (private) at line 1
  - `test_analyze_request_deserialization` (private) at line 1
  - `test_verify_request_deserialization` (private) at line 1
  - ... and 3 more functions

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.19

**TDG Severity:** Normal

### ./crates/depyler-mcp/src/tools.rs

**Language:** rust
**Total Symbols:** 32
**Functions:** 7 | **Structs:** 16 | **Enums:** 7 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `default_mode` (private) at line 1
  - `default_optimization` (private) at line 1
  - `default_type_inference` (private) at line 1
  - `default_memory_model` (private) at line 1
  - `default_analysis_depth` (private) at line 1
  - `default_include_patterns` (private) at line 1
  - `default_verification_level` (private) at line 1

**Structs:**
  - `TranspileRequest` (public) with 3 fields (derives: derive) at line 1
  - `TranspileOptions` (public) with 3 fields (derives: derive) at line 1
  - `TranspileResponse` (public) with 4 fields (derives: derive) at line 1
  - `TranspileMetrics` (public) with 4 fields (derives: derive) at line 1
  - `AnalyzeRequest` (public) with 3 fields (derives: derive) at line 1
  - ... and 11 more structs

**Enums:**
  - `Mode` (public) with 3 variants at line 1
  - `OptimizationLevel` (public) with 3 variants at line 1
  - `TypeInference` (public) with 3 variants at line 1
  - `MemoryModel` (public) with 3 variants at line 1
  - `AnalysisDepth` (public) with 3 variants at line 1
  - ... and 2 more enums

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.07

**TDG Severity:** Normal

### ./crates/depyler-mcp/src/validator.rs

**Language:** rust
**Total Symbols:** 10
**Functions:** 3 | **Structs:** 2 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Functions:**
  - `test_estimate_nesting` (private) at line 1
  - `test_score_explanation` (private) at line 1
  - `test_check_complexity` (private) at line 1

**Structs:**
  - `ValidationResult` (public) with 5 fields (derives: derive) at line 1
  - `McpValidator` (public) with 0 fields (derives: derive) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.25

**TDG Severity:** Normal

### ./crates/depyler-verify/src/contracts.rs

**Language:** rust
**Total Symbols:** 7
**Functions:** 2 | **Structs:** 3 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `check_stmt_contracts` (private) at line 1
  - `check_expr_contracts` (private) at line 1

**Structs:**
  - `Contract` (public) with 3 fields (derives: derive) at line 1
  - `Condition` (public) with 3 fields (derives: derive) at line 1
  - `ContractChecker` (public) with 0 fields at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.86

**TDG Severity:** Warning

### ./crates/depyler-verify/src/lib.rs

**Language:** rust
**Total Symbols:** 8
**Functions:** 0 | **Structs:** 3 | **Enums:** 2 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Structs:**
  - `PropertyVerifier` (public) with 3 fields (derives: derive) at line 1
  - `VerificationResult` (public) with 5 fields (derives: derive) at line 1
  - `TestCase` (public) with 4 fields (derives: derive) at line 1

**Enums:**
  - `PropertyStatus` (public) with 5 variants at line 1
  - `VerificationMethod` (public) with 5 variants at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.30

**TDG Severity:** Normal

### ./crates/depyler-verify/src/properties.rs

**Language:** rust
**Total Symbols:** 9
**Functions:** 7 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `generate_quickcheck_tests` (public) at line 1
  - `has_numeric_types` (private) at line 1
  - `has_container_params` (private) at line 1
  - `generate_numeric_property_test` (private) at line 1
  - `generate_bounds_property_test` (private) at line 1
  - `generate_termination_test` (private) at line 1
  - `type_to_rust_string` (private) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.66

**TDG Severity:** Warning

### ./crates/depyler-verify/src/quickcheck.rs

**Language:** rust
**Total Symbols:** 4
**Functions:** 1 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `shrink_value` (public) at line 1

**Structs:**
  - `TypedValue` (public) with 2 fields at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.70

**TDG Severity:** Warning

### ./examples/demo.py

**Language:** python
**Total Symbols:** 5
**Functions:** 4 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 1

**Functions:**
  - `fibonacci` (public) at line 1
  - `factorial` (public) at line 1
  - `is_prime` (public) at line 1
  - `process_list` (public) at line 1

**Key Imports:**
  - `typing.List` at line 1

**Technical Debt Gradient:** 1.14

**TDG Severity:** Normal

### ./examples/demo.rs

**Language:** rust
**Total Symbols:** 4
**Functions:** 4 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `fibonacci` (public) at line 1
  - `factorial` (public) at line 1
  - `is_prime` (public) at line 1
  - `process_list` (public) at line 1

**Technical Debt Gradient:** 1.22

**TDG Severity:** Normal

### ./examples/mcp_usage.py

**Language:** python
**Total Symbols:** 16
**Functions:** 9 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 6

**Functions:**
  - `__init__` (private) at line 1
  - `call_tool (async)` (public) at line 1
  - `_mock_response (async)` (private) at line 1
  - `example_1_simple_transpilation (async)` (public) at line 1
  - `example_2_project_analysis (async)` (public) at line 1
  - `example_3_verification (async)` (public) at line 1
  - `example_4_batch_processing (async)` (public) at line 1
  - `example_5_ai_assistant_integration (async)` (public) at line 1
  - `main (async)` (public) at line 1

**Structs:**
  - `DepylerMCPClient` (public) with 0 fields at line 1

**Key Imports:**
  - `asyncio` at line 1
  - `json` at line 1
  - `pathlib.Path` at line 1
  - `typing.Dict` at line 1
  - `typing.Any` at line 1
  - `typing.List` at line 1

**Technical Debt Gradient:** 1.29

**TDG Severity:** Normal

### ./examples/showcase/binary_search.py

**Language:** python
**Total Symbols:** 2
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 1

**Functions:**
  - `binary_search` (public) at line 1

**Key Imports:**
  - `typing.List` at line 1

**Technical Debt Gradient:** 1.10

**TDG Severity:** Normal

### ./examples/showcase/calculate_sum.py

**Language:** python
**Total Symbols:** 2
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 1

**Functions:**
  - `calculate_sum` (public) at line 1

**Key Imports:**
  - `typing.List` at line 1

**Technical Debt Gradient:** 1.07

**TDG Severity:** Normal

### ./examples/showcase/classify_number.py

**Language:** python
**Total Symbols:** 1
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `classify_number` (public) at line 1

**Technical Debt Gradient:** 1.07

**TDG Severity:** Normal

### ./examples/showcase/process_config.py

**Language:** python
**Total Symbols:** 3
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `process_config` (public) at line 1

**Key Imports:**
  - `typing.Dict` at line 1
  - `typing.Optional` at line 1

**Technical Debt Gradient:** 1.07

**TDG Severity:** Normal

### ./examples/validation/test_all.py

**Language:** python
**Total Symbols:** 8
**Functions:** 4 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 4

**Functions:**
  - `test_binary_search` (public) at line 1
  - `test_calculate_sum` (public) at line 1
  - `test_process_config` (public) at line 1
  - `test_classify_number` (public) at line 1

**Key Imports:**
  - `subprocess` at line 1
  - `sys` at line 1
  - `typing.List` at line 1
  - `typing.Any` at line 1

**Technical Debt Gradient:** 1.16

**TDG Severity:** Normal

### ./simple_test.py

**Language:** python
**Total Symbols:** 1
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `add` (public) at line 1

**Technical Debt Gradient:** 1.05

**TDG Severity:** Normal

### ./simple_test.rs

**Language:** rust
**Total Symbols:** 1
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `add` (public) at line 1

**Technical Debt Gradient:** 1.05

**TDG Severity:** Normal

### ./test_example.py

**Language:** python
**Total Symbols:** 2
**Functions:** 2 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `add_numbers` (public) at line 1
  - `main` (public) at line 1

**Technical Debt Gradient:** 1.06

**TDG Severity:** Normal

### ./test_mcp_functionality.rs

**Language:** rust
**Total Symbols:** 4
**Functions:** 1 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Functions:**
  - `main (async)` (private) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.51

**TDG Severity:** Warning

### ./tests/fixtures/expected_rust/basic_functions.rs

**Language:** rust
**Total Symbols:** 10
**Functions:** 10 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `add_two_numbers` (public) at line 1
  - `subtract_numbers` (public) at line 1
  - `multiply_numbers` (public) at line 1
  - `divide_numbers` (public) at line 1
  - `modulo_operation` (public) at line 1
  - `power_operation` (public) at line 1
  - `absolute_value` (public) at line 1
  - `max_two_numbers` (public) at line 1
  - `min_two_numbers` (public) at line 1
  - `sign_function` (public) at line 1

**Technical Debt Gradient:** 1.15

**TDG Severity:** Normal

### ./tests/fixtures/expected_rust/list_operations.rs

**Language:** rust
**Total Symbols:** 10
**Functions:** 10 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `sum_list` (public) at line 1
  - `find_max` (public) at line 1
  - `count_elements` (public) at line 1
  - `filter_positive` (public) at line 1
  - `get_element` (public) at line 1
  - `reverse_list` (public) at line 1
  - `contains_element` (public) at line 1
  - `first_element` (public) at line 1
  - `last_element` (public) at line 1
  - `average_numbers` (public) at line 1

**Technical Debt Gradient:** 1.42

**TDG Severity:** Normal

### ./tests/fixtures/python_samples/basic_functions.py

**Language:** python
**Total Symbols:** 10
**Functions:** 10 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 0

**Functions:**
  - `add_two_numbers` (public) at line 1
  - `subtract_numbers` (public) at line 1
  - `multiply_numbers` (public) at line 1
  - `divide_numbers` (public) at line 1
  - `modulo_operation` (public) at line 1
  - `power_operation` (public) at line 1
  - `absolute_value` (public) at line 1
  - `max_two_numbers` (public) at line 1
  - `min_two_numbers` (public) at line 1
  - `sign_function` (public) at line 1

**Technical Debt Gradient:** 1.11

**TDG Severity:** Normal

### ./tests/fixtures/python_samples/control_flow.py

**Language:** python
**Total Symbols:** 16
**Functions:** 15 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 1

**Functions:**
  - `factorial` (public) at line 1
  - `fibonacci` (public) at line 1
  - `binary_search` (public) at line 1
  - `count_down` (public) at line 1
  - `is_prime` (public) at line 1
  - `gcd` (public) at line 1
  - `power_iterative` (public) at line 1
  - `sum_of_digits` (public) at line 1
  - `reverse_integer` (public) at line 1
  - `linear_search` (public) at line 1
  - ... and 5 more functions

**Key Imports:**
  - `typing.List` at line 1

**Technical Debt Gradient:** 1.43

**TDG Severity:** Normal

### ./tests/fixtures/python_samples/dictionary_operations.py

**Language:** python
**Total Symbols:** 18
**Functions:** 15 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Functions:**
  - `get_dict_value` (public) at line 1
  - `set_dict_value` (public) at line 1
  - `count_dict_keys` (public) at line 1
  - `dict_contains_key` (public) at line 1
  - `get_dict_keys` (public) at line 1
  - `get_dict_values` (public) at line 1
  - `sum_dict_values` (public) at line 1
  - `find_key_by_value` (public) at line 1
  - `merge_dicts` (public) at line 1
  - `filter_dict_by_value` (public) at line 1
  - ... and 5 more functions

**Key Imports:**
  - `typing.Dict` at line 1
  - `typing.List` at line 1
  - `typing.Optional` at line 1

**Technical Debt Gradient:** 1.30

**TDG Severity:** Normal

### ./tests/fixtures/python_samples/edge_cases.py

**Language:** python
**Total Symbols:** 18
**Functions:** 15 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 3

**Functions:**
  - `safe_max` (public) at line 1
  - `safe_divide` (public) at line 1
  - `safe_get_last` (public) at line 1
  - `handle_large_numbers` (public) at line 1
  - `safe_substring` (public) at line 1
  - `count_nested_items` (public) at line 1
  - `complex_conditions` (public) at line 1
  - `safe_range_sum` (public) at line 1
  - `get_nested_value` (public) at line 1
  - `validate_input` (public) at line 1
  - ... and 5 more functions

**Key Imports:**
  - `typing.List` at line 1
  - `typing.Optional` at line 1
  - `typing.Dict` at line 1

**Technical Debt Gradient:** 1.48

**TDG Severity:** Normal

### ./tests/fixtures/python_samples/list_operations.py

**Language:** python
**Total Symbols:** 12
**Functions:** 10 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `sum_list` (public) at line 1
  - `find_max` (public) at line 1
  - `count_elements` (public) at line 1
  - `filter_positive` (public) at line 1
  - `get_element` (public) at line 1
  - `reverse_list` (public) at line 1
  - `contains_element` (public) at line 1
  - `first_element` (public) at line 1
  - `last_element` (public) at line 1
  - `average_numbers` (public) at line 1

**Key Imports:**
  - `typing.List` at line 1
  - `typing.Optional` at line 1

**Technical Debt Gradient:** 1.29

**TDG Severity:** Normal

### ./tests/fixtures/python_samples/string_operations.py

**Language:** python
**Total Symbols:** 17
**Functions:** 15 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 2

**Functions:**
  - `string_length` (public) at line 1
  - `concat_strings` (public) at line 1
  - `repeat_string` (public) at line 1
  - `to_uppercase` (public) at line 1
  - `to_lowercase` (public) at line 1
  - `contains_substring` (public) at line 1
  - `starts_with` (public) at line 1
  - `ends_with` (public) at line 1
  - `replace_substring` (public) at line 1
  - `split_string` (public) at line 1
  - ... and 5 more functions

**Key Imports:**
  - `typing.List` at line 1
  - `typing.Optional` at line 1

**Technical Debt Gradient:** 1.14

**TDG Severity:** Normal

### ./tests/functional_tests.rs

**Language:** rust
**Total Symbols:** 8
**Functions:** 3 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 5

**Functions:**
  - `test_mcp_server_functionality (async)` (private) at line 1
  - `test_cli_functionality` (private) at line 1
  - `test_integration_transpilation_pipeline` (private) at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.19

**TDG Severity:** Normal

### ./tests/integration/semantic_equivalence.rs

**Language:** rust
**Total Symbols:** 17
**Functions:** 11 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 6

**Functions:**
  - `arb_simple_expr` (private) at line 1
  - `arb_simple_literal` (private) at line 1
  - `arb_binary_op` (private) at line 1
  - `arb_python_function` (private) at line 1
  - `arb_function_body` (private) at line 1
  - `verify_rust_syntax` (private) at line 1
  - `eval_python_arithmetic` (private) at line 1
  - `eval_rust_arithmetic` (private) at line 1
  - `eval_rust_conditional` (private) at line 1
  - `test_simple_arithmetic_functions` (private) at line 1
  - ... and 1 more functions

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 1.69

**TDG Severity:** Warning

### ./tests/integration/transpilation_tests.rs

**Language:** rust
**Total Symbols:** 15
**Functions:** 4 | **Structs:** 1 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 10

**Functions:**
  - `test_simple_function_transpilation` (private) at line 1
  - `test_list_operations` (private) at line 1
  - `test_conditional_logic` (private) at line 1
  - `test_mcp_functionality` (private) at line 1

**Structs:**
  - `TranspilationTestHarness` (public) with 3 fields (derives: derive) at line 1

**Imports:** 10 import statements

**Technical Debt Gradient:** 1.76

**TDG Severity:** Warning

### ./tests/transpilation/test_basic.rs

**Language:** rust
**Total Symbols:** 5
**Functions:** 4 | **Structs:** 0 | **Enums:** 0 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 1

**Functions:**
  - `test_binary_search` (private) at line 1
  - `test_calculate_sum` (private) at line 1
  - `test_process_config` (private) at line 1
  - `test_classify_number` (private) at line 1

**Key Imports:**
  - `use statement` at line 1

**Technical Debt Gradient:** 1.06

**TDG Severity:** Normal

### ./tests/validation/rustc_compilation.rs

**Language:** rust
**Total Symbols:** 14
**Functions:** 3 | **Structs:** 3 | **Enums:** 1 | **Traits:** 0 | **Impls:** 0 | **Modules:** 0 | **Imports:** 7

**Functions:**
  - `test_exhaustive_compilation_validation` (private) at line 1
  - `test_individual_function_validation` (private) at line 1
  - `test_complex_function_validation` (private) at line 1

**Structs:**
  - `CompilationValidator` (public) with 4 fields at line 1
  - `CompilationReport` (public) with 4 fields (derives: derive) at line 1
  - `TestResult` (public) with 4 fields (derives: derive) at line 1

**Enums:**
  - `TestFailure` (public) with 5 variants at line 1

**Key Imports:**
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1
  - `use statement` at line 1

**Technical Debt Gradient:** 2.33

**TDG Severity:** Warning

## Complexity Hotspots

| Function | File | Cyclomatic | Cognitive |
|----------|------|------------|-----------|
| `HirExpr::to_rust_expr` | `./crates/depyler-core/src/rust_gen.rs` | 42 | 61 |
| `convert_expr` | `./crates/depyler-core/src/ast_bridge.rs` | 39 | 51 |
| `TypeInferencer::infer_expr` | `./crates/depyler-analyzer/src/type_flow.rs` | 31 | 40 |
| `convert_stmt` | `./crates/depyler-core/src/ast_bridge.rs` | 27 | 30 |
| `expr_to_rust_tokens` | `./crates/depyler-core/src/codegen.rs` | 26 | 28 |
| `extract_type` | `./crates/depyler-core/src/ast_bridge.rs` | 23 | 55 |
| `binop_to_rust_tokens` | `./crates/depyler-core/src/codegen.rs` | 23 | 23 |
| `convert_binop` | `./crates/depyler-core/src/direct_rules.rs` | 22 | 22 |
| `convert_binop` | `./crates/depyler-core/src/rust_gen.rs` | 22 | 22 |
| `TypeMapper::map_type` | `./crates/depyler-core/src/type_mapper.rs` | 21 | 27 |

## Code Churn Analysis

**Summary:**
- Total Commits: 131
- Files Changed: 88

**Top Changed Files:**
| File | Commits | Authors |
|------|---------|---------|
| `.github/workflows/ci.yml` | 10 | 1 |
| `crates/depyler-core/src/direct_rules.rs` | 5 | 1 |
| `README.md` | 4 | 1 |
| `Cargo.toml` | 4 | 1 |
| `crates/depyler-mcp/src/server.rs` | 3 | 1 |
| `docs/project-overview.md` | 2 | 1 |
| `crates/depyler-core/src/rust_gen.rs` | 2 | 1 |
| `docs/cli-reference.md` | 2 | 1 |
| `docs/user-guide.md` | 2 | 1 |
| `crates/depyler-analyzer/src/lib.rs` | 2 | 1 |

## Technical Debt Analysis

**SATD Summary:**

## Dead Code Analysis

**Summary:**
- Dead Functions: 0
- Total Dead Lines: 20

**Top Files with Dead Code:**
| File | Dead Lines | Dead Functions |
|------|------------|----------------|
| `./benches/memory_usage.rs` | 0 | 0 |
| `./benches/transpilation.rs` | 0 | 0 |
| `./crates/depyler-analyzer/src/complexity.rs` | 0 | 0 |
| `./crates/depyler-analyzer/src/lib.rs` | 0 | 0 |
| `./crates/depyler-analyzer/src/metrics.rs` | 0 | 0 |
| `./crates/depyler-analyzer/src/type_flow.rs` | 0 | 0 |
| `./crates/depyler-core/src/ast_bridge.rs` | 0 | 0 |
| `./crates/depyler-core/src/codegen.rs` | 0 | 0 |
| `./crates/depyler-core/src/error.rs` | 0 | 0 |
| `./crates/depyler-core/src/hir.rs` | 0 | 0 |

## Defect Probability Analysis

**Risk Assessment:**
- Total Defects Predicted: 63
- Defect Density: 19.49 defects per 1000 lines

---
Generated by deep-context v0.21.0
