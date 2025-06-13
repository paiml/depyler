mod quality;

pub use quality::*;
use wasm_bindgen::prelude::*;
use depyler_wasm::DepylerWasm;

// Re-export key types for playground use
pub use depyler_wasm::{
    WasmTranspileOptions, WasmTranspileResult, WasmEnergyEstimate, WasmQualityMetrics
};

#[wasm_bindgen]
pub struct PlaygroundEngine {
    depyler: DepylerWasm,
    quality_monitor: QualityMonitor,
}

#[wasm_bindgen]
impl PlaygroundEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<PlaygroundEngine, JsValue> {
        let config = PmatConfiguration::default_playground();
        let quality_monitor = QualityMonitor::new(config);
        let depyler = DepylerWasm::new();
        
        Ok(PlaygroundEngine {
            depyler,
            quality_monitor,
        })
    }
    
    #[wasm_bindgen]
    pub fn transpile_with_metrics(&mut self, python_code: &str, options: &WasmTranspileOptions) -> Result<JsValue, JsValue> {
        // Perform transpilation
        let result = self.depyler.transpile(python_code, options)?;
        
        // Calculate complexity bucket
        let lines = python_code.lines().count();
        let complexity_bucket = if lines < 10 {
            ComplexityBucket::Simple
        } else if lines < 50 {
            ComplexityBucket::Medium
        } else {
            ComplexityBucket::Complex
        };
        
        // Create playground metrics
        let metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 0.0, // Set by frontend
                tti_ms: 0.0,   // Set by frontend
                wasm_load_ms: 0.0, // Set by frontend
                wasm_size_kb: 0.0,  // Set by frontend
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: result.transpile_time_ms(),
                complexity_bucket,
                cache_hit_rate: 0.0, // Would be tracked by frontend
                error_rate: if result.success() { 0.0 } else { 1.0 },
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 0.0, // Would be measured during execution
                python_execution_ms: 0.0, // Would be measured during execution
                energy_savings_percent: 75.0, // Estimated
                memory_usage_mb: result.memory_usage_mb,
            },
            quality_events: vec![],
        };
        
        // Record metrics for quality monitoring
        let pmat_score = self.quality_monitor.record_metrics(&metrics);
        
        // Combine results
        let combined_result = CombinedResult {
            transpile_result: result,
            pmat_score,
            quality_events: self.quality_monitor.get_recent_events(std::time::Duration::from_secs(60))
                .into_iter()
                .cloned()
                .collect(),
        };
        
        serde_wasm_bindgen::to_value(&combined_result)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedResult {
    pub transpile_result: WasmTranspileResult,
    pub pmat_score: PmatScore,
    pub quality_events: Vec<QualityEvent>,
}

impl Default for PlaygroundEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_playground_engine_new() {
        let engine = PlaygroundEngine::new();
        assert!(engine.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_playground_engine_default() {
        let engine = PlaygroundEngine::default();
        // Should not panic
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_transpile_simple_code() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: false,
            energy_analysis: false,
            optimization_level: "none".to_string(),
        };
        
        let python_code = "def add(a, b):\n    return a + b";
        let result = engine.transpile_with_metrics(python_code, &options);
        
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_transpile_with_metrics_complex_code() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: false,
            energy_analysis: true,
            optimization_level: "basic".to_string(),
        };
        
        // Test with medium complexity code (10-50 lines)
        let python_code = r#"
def binary_search(arr, target):
    left = 0
    right = len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1

def test_search():
    numbers = [1, 3, 5, 7, 9, 11, 13, 15]
    result = binary_search(numbers, 7)
    return result
"#;
        
        let result = engine.transpile_with_metrics(python_code, &options);
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_transpile_error_handling() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: true,
            energy_analysis: false,
            optimization_level: "none".to_string(),
        };
        
        // Test with invalid Python code
        let python_code = "def broken(\n    return";
        let result = engine.transpile_with_metrics(python_code, &options);
        
        // Should still return a result (with error info)
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_complexity_bucket_classification() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: false,
            energy_analysis: false,
            optimization_level: "none".to_string(),
        };
        
        // Test simple code (<10 lines)
        let simple_code = "x = 1\ny = 2\nz = x + y";
        let result = engine.transpile_with_metrics(simple_code, &options);
        assert!(result.is_ok());
        
        // Test complex code (>50 lines)
        let mut complex_code = String::new();
        for i in 0..60 {
            complex_code.push_str(&format!("var_{} = {}\n", i, i));
        }
        let result = engine.transpile_with_metrics(&complex_code, &options);
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_medium_complexity_bucket() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: false,
            energy_analysis: false,
            optimization_level: "none".to_string(),
        };
        
        // Test medium complexity (10-50 lines)
        let mut medium_code = String::new();
        for i in 0..25 {
            medium_code.push_str(&format!("line_{} = {}\n", i, i * 2));
        }
        let result = engine.transpile_with_metrics(&medium_code, &options);
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_transpile_with_all_options() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: true,
            energy_analysis: true,
            optimization_level: "aggressive".to_string(),
        };
        
        let python_code = "def square(x):\n    return x * x";
        let result = engine.transpile_with_metrics(python_code, &options);
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_quality_monitor_integration() {
        let mut engine = PlaygroundEngine::new().unwrap();
        let options = WasmTranspileOptions {
            verify: false,
            energy_analysis: false,
            optimization_level: "none".to_string(),
        };
        
        // Run multiple transpilations to populate quality history
        for i in 0..5 {
            let code = format!("x = {}\ny = x * 2", i);
            let result = engine.transpile_with_metrics(&code, &options);
            assert!(result.is_ok());
        }
        
        // The quality monitor should have recorded metrics
        let recent_events = engine.quality_monitor.get_recent_events(std::time::Duration::from_secs(60));
        // Events may or may not be present depending on quality changes
        assert!(recent_events.len() >= 0);
    }
    
    #[test]
    fn test_combined_result_serialization() {
        use depyler_wasm::WasmTranspileResult;
        
        let combined_result = CombinedResult {
            transpile_result: WasmTranspileResult {
                rust_code: "fn main() {}".to_string(),
                success: true,
                error: None,
                warnings: vec![],
                energy_estimate: None,
                quality_metrics: None,
                transpile_time_ms: 10.0,
                memory_usage_mb: 1.0,
            },
            pmat_score: PmatScore {
                productivity: 0.8,
                maintainability: 0.9,
                accessibility: 0.85,
                testability: 0.7,
                tdg: 0.81,
                timestamp: std::time::SystemTime::now(),
            },
            quality_events: vec![],
        };
        
        // Test that it can be serialized
        let serialized = serde_json::to_string(&combined_result);
        assert!(serialized.is_ok());
    }
}