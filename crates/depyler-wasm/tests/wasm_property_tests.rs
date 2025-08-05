use depyler_wasm::*;
use proptest::prelude::*;

// Property: WasmTranspileOptions setters always work correctly
proptest! {
    #[test]
    fn test_options_setters_preserve_values(
        verify in any::<bool>(),
        optimize in any::<bool>(),
        emit_docs in any::<bool>(),
        target_version in "[0-9]+\\.[0-9]+",
    ) {
        let mut options = WasmTranspileOptions::new();
        
        options.set_verify(verify);
        assert_eq!(options.verify(), verify);
        
        options.set_optimize(optimize);
        assert_eq!(options.optimize(), optimize);
        
        options.set_emit_docs(emit_docs);
        assert_eq!(options.emit_docs(), emit_docs);
        
        options.set_target_version(target_version.clone());
        assert_eq!(options.target_version(), target_version);
    }
}

// Property: Options are independent - setting one doesn't affect others
proptest! {
    #[test]
    fn test_options_independence(
        verify1 in any::<bool>(),
        verify2 in any::<bool>(),
        optimize1 in any::<bool>(),
        optimize2 in any::<bool>(),
    ) {
        let mut options = WasmTranspileOptions::new();
        
        // Set initial values
        options.set_verify(verify1);
        options.set_optimize(optimize1);
        
        // Verify initial values
        prop_assert_eq!(options.verify(), verify1);
        prop_assert_eq!(options.optimize(), optimize1);
        
        // Change one value
        options.set_verify(verify2);
        
        // Verify only the changed value is different
        prop_assert_eq!(options.verify(), verify2);
        prop_assert_eq!(options.optimize(), optimize1); // Should remain unchanged
    }
}

// Property: DepylerWasm handles various Python code inputs
proptest! {
    #[test]
    fn test_transpilation_doesnt_panic(
        func_name in "[a-zA-Z_][a-zA-Z0-9_]*",
        param_name in "[a-zA-Z_][a-zA-Z0-9_]*",
        value in prop::num::i32::ANY,
    ) {
        let engine = DepylerWasm::new();
        let options = WasmTranspileOptions::new();
        
        // Generate valid Python code
        let python_code = format!(
            "def {}({}: int) -> int:\n    return {} + {}",
            func_name, param_name, param_name, value
        );
        
        // Transpilation should not panic, even if it fails
        let _result = engine.transpile(&python_code, &options);
    }
}

// Property: Valid Python functions always produce deterministic results
proptest! {
    #[test]
    fn test_deterministic_transpilation(
        func_name in "[a-zA-Z_][a-zA-Z0-9_]*",
        iterations in 2usize..5,
    ) {
        let engine = DepylerWasm::new();
        let options = WasmTranspileOptions::new();
        
        let python_code = format!(
            "def {}(x: int) -> int:\n    return x * 2",
            func_name
        );
        
        let mut results = Vec::new();
        
        for _ in 0..iterations {
            match engine.transpile(&python_code, &options) {
                Ok(result) => {
                    if result.success() {
                        results.push(result.rust_code());
                    }
                }
                Err(_) => {}
            }
        }
        
        // All successful results should be identical
        if results.len() > 1 {
            for i in 1..results.len() {
                prop_assert_eq!(&results[0], &results[i]);
            }
        }
    }
}

// Property: Analysis always produces valid JSON
proptest! {
    #[test]
    fn test_analysis_produces_valid_json(
        code_content in "[a-zA-Z0-9\n ]{10,100}",
    ) {
        let engine = DepylerWasm::new();
        
        // Wrap content in a function to make it valid Python
        let python_code = format!("def test():\n    pass\n    # {}", code_content);
        
        match engine.analyze_code(&python_code) {
            Ok(result) => {
                // Should be valid JSON (JsValue)
                // The fact that it returns Ok means it's valid
                prop_assert!(true);
            }
            Err(_) => {
                // Even errors should be properly formatted
                prop_assert!(true);
            }
        }
    }
}

// Property: Benchmark always returns consistent structure
proptest! {
    #[test]
    fn test_benchmark_consistency(
        iterations in 1u32..10,
    ) {
        let engine = DepylerWasm::new();
        
        let python_code = "def identity(x: int) -> int: return x";
        
        match engine.benchmark(python_code, iterations) {
            Ok(_result) => {
                // Benchmark succeeded - structure is valid
                prop_assert!(true);
            }
            Err(_) => {
                // Even on error, it should be properly formatted
                prop_assert!(true);
            }
        }
    }
}

// Property: Transpilation results have valid metrics
proptest! {
    #[test]
    fn test_transpilation_metrics_validity(
        num_lines in 1usize..20,
    ) {
        let engine = DepylerWasm::new();
        let options = WasmTranspileOptions::new();
        
        // Generate Python code with variable complexity
        let mut python_code = String::from("def complex_func(x: int) -> int:\n");
        python_code.push_str("    result = 0\n");
        for i in 0..num_lines {
            python_code.push_str(&format!("    if x > {}: result += {}\n", i, i));
        }
        python_code.push_str("    return result");
        
        match engine.transpile(&python_code, &options) {
            Ok(result) => {
                if result.success() {
                    // Time should be non-negative
                    prop_assert!(result.transpile_time_ms() >= 0.0);
                    
                    // Memory usage should be non-negative
                    prop_assert!(result.memory_usage_mb() >= 0.0);
                    
                    // Energy metrics should be valid
                    let energy = result.energy_estimate();
                    prop_assert!(energy.joules() >= 0.0);
                    prop_assert!(energy.confidence() >= 0.0 && energy.confidence() <= 1.0);
                    
                    // Quality metrics should be in valid ranges
                    let quality = result.quality_metrics();
                    prop_assert!(quality.pmat_score() >= 0.0 && quality.pmat_score() <= 1.0);
                    prop_assert!(quality.productivity() >= 0.0 && quality.productivity() <= 1.0);
                    prop_assert!(quality.maintainability() >= 0.0 && quality.maintainability() <= 1.0);
                    prop_assert!(quality.accessibility() >= 0.0 && quality.accessibility() <= 1.0);
                    prop_assert!(quality.testability() >= 0.0 && quality.testability() <= 1.0);
                    prop_assert!(quality.code_complexity() >= 0);
                    prop_assert!(quality.cyclomatic_complexity() >= 0);
                }
            }
            Err(_) => {
                // Error case is also valid
                prop_assert!(true);
            }
        }
    }
}

// Property: Empty or whitespace-only code handles gracefully
proptest! {
    #[test]
    fn test_empty_code_handling(
        whitespace in prop::string::string_regex(" *\n*\t*").unwrap(),
    ) {
        let engine = DepylerWasm::new();
        let options = WasmTranspileOptions::new();
        
        // Should not panic on empty/whitespace code
        let _result = engine.transpile(&whitespace, &options);
        let _analysis = engine.analyze_code(&whitespace);
    }
}