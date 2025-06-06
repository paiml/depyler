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
        let start_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);
        
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
            quality_events: self.quality_monitor.get_recent_events(std::time::Duration::from_secs(60)),
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
    pub quality_events: Vec<&QualityEvent>,
}

impl Default for PlaygroundEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}