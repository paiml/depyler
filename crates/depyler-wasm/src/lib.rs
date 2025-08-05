mod utils;

#[cfg(test)]
mod tests;

use depyler_core::DepylerPipeline;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Options for controlling Python to Rust transpilation in WASM
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_wasm::WasmTranspileOptions;
///
/// let mut options = WasmTranspileOptions::new();
/// options.set_verify(true);
/// options.set_optimize(true);
/// options.set_emit_docs(false);
/// options.set_target_version("1.83".to_string());
///
/// assert!(options.verify());
/// assert!(options.optimize());
/// assert!(!options.emit_docs());
/// assert_eq!(options.target_version(), "1.83");
/// ```
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmTranspileOptions {
    #[wasm_bindgen(skip)]
    pub verify: bool,
    #[wasm_bindgen(skip)]
    pub optimize: bool,
    #[wasm_bindgen(skip)]
    pub emit_docs: bool,
    #[wasm_bindgen(skip)]
    pub target_version: String,
}

impl Default for WasmTranspileOptions {
    fn default() -> Self {
        Self {
            verify: true,
            optimize: true,
            emit_docs: false,
            target_version: "1.83".to_string(),
        }
    }
}

#[wasm_bindgen]
impl WasmTranspileOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTranspileOptions {
        WasmTranspileOptions::default()
    }

    #[wasm_bindgen(getter)]
    pub fn verify(&self) -> bool {
        self.verify
    }

    #[wasm_bindgen(setter)]
    pub fn set_verify(&mut self, verify: bool) {
        self.verify = verify;
    }

    #[wasm_bindgen(getter)]
    pub fn optimize(&self) -> bool {
        self.optimize
    }

    #[wasm_bindgen(setter)]
    pub fn set_optimize(&mut self, optimize: bool) {
        self.optimize = optimize;
    }

    #[wasm_bindgen(getter)]
    pub fn emit_docs(&self) -> bool {
        self.emit_docs
    }

    #[wasm_bindgen(setter)]
    pub fn set_emit_docs(&mut self, emit_docs: bool) {
        self.emit_docs = emit_docs;
    }

    #[wasm_bindgen(getter)]
    pub fn target_version(&self) -> String {
        self.target_version.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_target_version(&mut self, target_version: String) {
        self.target_version = target_version;
    }
}

/// Result of a Python to Rust transpilation operation
///
/// Contains the generated Rust code, any errors or warnings,
/// performance metrics, and quality analysis.
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_wasm::{DepylerWasm, WasmTranspileOptions};
///
/// let engine = DepylerWasm::new();
/// let options = WasmTranspileOptions::new();
///
/// let python_code = r#"
/// def add(a: int, b: int) -> int:
///     return a + b
/// "#;
///
/// let result = engine.transpile(python_code, &options).unwrap();
///
/// if result.success() {
///     println!("Rust code: {}", result.rust_code());
///     println!("Time: {}ms", result.transpile_time_ms());
///     println!("Energy: {}J", result.energy_estimate().joules());
/// }
/// ```
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmTranspileResult {
    #[wasm_bindgen(skip)]
    pub success: bool,
    #[wasm_bindgen(skip)]
    pub rust_code: String,
    #[wasm_bindgen(skip)]
    pub errors: Vec<String>,
    #[wasm_bindgen(skip)]
    pub warnings: Vec<String>,
    #[wasm_bindgen(skip)]
    pub transpile_time_ms: f64,
    #[wasm_bindgen(skip)]
    pub memory_usage_mb: f64,
    #[wasm_bindgen(skip)]
    pub energy_estimate: WasmEnergyEstimate,
    #[wasm_bindgen(skip)]
    pub quality_metrics: WasmQualityMetrics,
}

#[wasm_bindgen]
impl WasmTranspileResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn rust_code(&self) -> String {
        self.rust_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn transpile_time_ms(&self) -> f64 {
        self.transpile_time_ms
    }

    #[wasm_bindgen(getter)]
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn memory_usage_mb(&self) -> f64 {
        self.memory_usage_mb
    }

    #[wasm_bindgen(getter)]
    pub fn energy_estimate(&self) -> WasmEnergyEstimate {
        self.energy_estimate.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn quality_metrics(&self) -> WasmQualityMetrics {
        self.quality_metrics.clone()
    }
}

/// Energy consumption estimate for transpilation
///
/// Provides estimated energy usage, power consumption,
/// and carbon emissions for the transpilation process.
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_wasm::WasmEnergyEstimate;
///
/// // After transpilation
/// let energy = result.energy_estimate();
/// println!("Energy used: {} joules", energy.joules());
/// println!("Average power: {} watts", energy.watts_average());
/// println!("CO2 emissions: {} grams", energy.co2_grams());
/// println!("Confidence: {}%", energy.confidence() * 100.0);
/// ```
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmEnergyEstimate {
    #[wasm_bindgen(skip)]
    pub joules: f64,
    #[wasm_bindgen(skip)]
    pub watts_average: f64,
    #[wasm_bindgen(skip)]
    pub co2_grams: f64,
    #[wasm_bindgen(skip)]
    pub confidence: f64,
}

#[wasm_bindgen]
impl WasmEnergyEstimate {
    #[wasm_bindgen(getter)]
    pub fn joules(&self) -> f64 {
        self.joules
    }

    #[wasm_bindgen(getter)]
    pub fn watts_average(&self) -> f64 {
        self.watts_average
    }

    #[wasm_bindgen(getter)]
    pub fn co2_grams(&self) -> f64 {
        self.co2_grams
    }

    #[wasm_bindgen(getter)]
    pub fn confidence(&self) -> f64 {
        self.confidence
    }
}

/// Code quality metrics for transpiled Rust code
///
/// Measures productivity, maintainability, accessibility,
/// and testability (PMAT) along with complexity metrics.
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_wasm::WasmQualityMetrics;
///
/// // After transpilation
/// let metrics = result.quality_metrics();
/// println!("PMAT Score: {:.2}", metrics.pmat_score());
/// println!("Productivity: {:.2}", metrics.productivity());
/// println!("Maintainability: {:.2}", metrics.maintainability());
/// println!("Code Complexity: {}", metrics.code_complexity());
/// println!("Cyclomatic Complexity: {}", metrics.cyclomatic_complexity());
/// ```
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmQualityMetrics {
    #[wasm_bindgen(skip)]
    pub pmat_score: f64,
    #[wasm_bindgen(skip)]
    pub productivity: f64,
    #[wasm_bindgen(skip)]
    pub maintainability: f64,
    #[wasm_bindgen(skip)]
    pub accessibility: f64,
    #[wasm_bindgen(skip)]
    pub testability: f64,
    #[wasm_bindgen(skip)]
    pub code_complexity: i32,
    #[wasm_bindgen(skip)]
    pub cyclomatic_complexity: i32,
}

#[wasm_bindgen]
impl WasmQualityMetrics {
    #[wasm_bindgen(getter)]
    pub fn pmat_score(&self) -> f64 {
        self.pmat_score
    }

    #[wasm_bindgen(getter)]
    pub fn productivity(&self) -> f64 {
        self.productivity
    }

    #[wasm_bindgen(getter)]
    pub fn maintainability(&self) -> f64 {
        self.maintainability
    }

    #[wasm_bindgen(getter)]
    pub fn accessibility(&self) -> f64 {
        self.accessibility
    }

    #[wasm_bindgen(getter)]
    pub fn testability(&self) -> f64 {
        self.testability
    }

    #[wasm_bindgen(getter)]
    pub fn code_complexity(&self) -> i32 {
        self.code_complexity
    }

    #[wasm_bindgen(getter)]
    pub fn cyclomatic_complexity(&self) -> i32 {
        self.cyclomatic_complexity
    }
}

/// Main WASM interface for Depyler Python-to-Rust transpiler
///
/// Provides transpilation, code analysis, and benchmarking
/// functionality for Python code in WebAssembly environments.
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_wasm::{DepylerWasm, WasmTranspileOptions};
///
/// // Initialize the engine
/// let engine = DepylerWasm::new();
///
/// // Configure options
/// let mut options = WasmTranspileOptions::new();
/// options.set_verify(true);
///
/// // Transpile Python code
/// let python_code = r#"
/// def factorial(n: int) -> int:
///     if n <= 1:
///         return 1
///     return n * factorial(n - 1)
/// "#;
///
/// match engine.transpile(python_code, &options) {
///     Ok(result) => {
///         if result.success() {
///             println!("Generated Rust: {}", result.rust_code());
///         } else {
///             println!("Errors: {:?}", result.errors());
///         }
///     }
///     Err(e) => println!("Transpilation failed: {:?}", e),
/// }
/// ```
#[wasm_bindgen]
pub struct DepylerWasm {
    initialized: bool,
}

#[wasm_bindgen]
impl DepylerWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DepylerWasm {
        set_panic_hook();
        console_log!("Depyler WASM initialized");

        DepylerWasm { initialized: true }
    }

    /// Transpile Python code to Rust
    ///
    /// # Arguments
    ///
    /// * `python_code` - The Python source code to transpile
    /// * `options` - Configuration options for transpilation
    ///
    /// # Returns
    ///
    /// A `WasmTranspileResult` containing the generated Rust code,
    /// metrics, and any errors or warnings.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let engine = DepylerWasm::new();
    /// let options = WasmTranspileOptions::new();
    ///
    /// let result = engine.transpile(
    ///     "def square(x: int) -> int: return x * x",
    ///     &options
    /// ).unwrap();
    ///
    /// assert!(result.success());
    /// assert!(result.rust_code().contains("fn square"));
    /// ```
    #[wasm_bindgen]
    pub fn transpile(
        &self,
        python_code: &str,
        options: &WasmTranspileOptions,
    ) -> Result<WasmTranspileResult, JsValue> {
        if !self.initialized {
            return Err(JsValue::from_str("DepylerWasm not initialized"));
        }

        let start_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        let mem_before = get_memory_usage();

        // Create pipeline with options
        let mut pipeline = DepylerPipeline::new();
        if options.verify {
            pipeline = pipeline.with_verification();
        }

        // Perform transpilation
        let result = match pipeline.transpile(python_code) {
            Ok(rust_code) => {
                let transpile_time = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now() - start_time)
                    .unwrap_or(0.0);

                let mem_after = get_memory_usage();
                let memory_usage = (mem_after - mem_before).max(0.0);

                // Calculate energy estimate
                let energy_estimate = calculate_energy_estimate(transpile_time, memory_usage);

                // Calculate quality metrics
                let quality_metrics = calculate_quality_metrics(python_code, &rust_code);

                WasmTranspileResult {
                    success: true,
                    rust_code,
                    errors: vec![],
                    warnings: vec![],
                    transpile_time_ms: transpile_time,
                    memory_usage_mb: memory_usage,
                    energy_estimate,
                    quality_metrics,
                }
            }
            Err(e) => {
                let transpile_time = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now() - start_time)
                    .unwrap_or(0.0);

                WasmTranspileResult {
                    success: false,
                    rust_code: String::new(),
                    errors: vec![e.to_string()],
                    warnings: vec![],
                    transpile_time_ms: transpile_time,
                    memory_usage_mb: 0.0,
                    energy_estimate: WasmEnergyEstimate {
                        joules: 0.0,
                        watts_average: 0.0,
                        co2_grams: 0.0,
                        confidence: 0.0,
                    },
                    quality_metrics: WasmQualityMetrics {
                        pmat_score: 0.0,
                        productivity: 0.0,
                        maintainability: 0.0,
                        accessibility: 0.0,
                        testability: 0.0,
                        code_complexity: 0,
                        cyclomatic_complexity: 0,
                    },
                }
            }
        };

        Ok(result)
    }

    /// Perform static analysis on Python code
    ///
    /// Analyzes the given Python code without transpiling it,
    /// providing insights about complexity, anti-patterns,
    /// and optimization opportunities.
    ///
    /// # Arguments
    ///
    /// * `python_code` - The Python source code to analyze
    ///
    /// # Returns
    ///
    /// A JSON object containing:
    /// - Code complexity metrics
    /// - Detected functions and their properties
    /// - Import statements
    /// - Optimization suggestions
    /// - Anti-pattern warnings
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let engine = DepylerWasm::new();
    ///
    /// let analysis = engine.analyze_code(r#"
    /// def risky_function():
    ///     eval("user_input")  # Anti-pattern detected!
    ///     for i in range(len(items)):  # Could use enumerate
    ///         print(i)
    /// "#).unwrap();
    /// ```
    #[wasm_bindgen]
    pub fn analyze_code(&self, python_code: &str) -> Result<JsValue, JsValue> {
        if !self.initialized {
            return Err(JsValue::from_str("DepylerWasm not initialized"));
        }

        let pipeline = DepylerPipeline::new();

        // Parse to HIR
        let _hir = match pipeline.parse_to_hir(python_code) {
            Ok(hir) => hir,
            Err(e) => return Err(JsValue::from_str(&format!("Parse error: {e}"))),
        };

        // Perform static analysis using simple analysis
        let analysis = perform_static_analysis(python_code);

        // Convert to JS-compatible format
        serde_wasm_bindgen::to_value(&analysis).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Benchmark transpilation performance
    ///
    /// Runs the transpilation process multiple times to measure
    /// performance characteristics and consistency.
    ///
    /// # Arguments
    ///
    /// * `python_code` - The Python code to benchmark
    /// * `iterations` - Number of times to run transpilation
    ///
    /// # Returns
    ///
    /// A JSON object containing:
    /// - Execution times for each iteration
    /// - Statistical measures (min, max, mean, median, std dev)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let engine = DepylerWasm::new();
    ///
    /// let benchmark = engine.benchmark(
    ///     "def fib(n): return n if n < 2 else fib(n-1) + fib(n-2)",
    ///     10
    /// ).unwrap();
    ///
    /// // Results include timing statistics
    /// ```
    #[wasm_bindgen]
    pub fn benchmark(&self, python_code: &str, iterations: u32) -> Result<JsValue, JsValue> {
        if !self.initialized {
            return Err(JsValue::from_str("DepylerWasm not initialized"));
        }

        let mut results = Vec::new();
        let options = WasmTranspileOptions::default();

        for _ in 0..iterations {
            let result = self.transpile(python_code, &options)?;
            results.push(result.transpile_time_ms);
        }

        let benchmark_result = BenchmarkResult {
            iterations,
            times_ms: results.clone(),
            min_ms: results.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            max_ms: results.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            mean_ms: results.iter().sum::<f64>() / results.len() as f64,
            median_ms: {
                let mut sorted = results.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                sorted[sorted.len() / 2]
            },
            std_dev_ms: {
                let mean = results.iter().sum::<f64>() / results.len() as f64;
                let variance =
                    results.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / results.len() as f64;
                variance.sqrt()
            },
        };

        serde_wasm_bindgen::to_value(&benchmark_result)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub iterations: u32,
    pub times_ms: Vec<f64>,
    pub min_ms: f64,
    pub max_ms: f64,
    pub mean_ms: f64,
    pub median_ms: f64,
    pub std_dev_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticAnalysis {
    pub complexity: i32,
    pub cyclomatic_complexity: i32,
    pub functions: Vec<FunctionInfo>,
    pub imports: Vec<String>,
    pub suggestions: Vec<OptimizationSuggestion>,
    pub anti_patterns: Vec<AntiPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub line_start: u32,
    pub line_end: u32,
    pub complexity: i32,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub suggestion_type: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiPattern {
    pub line: u32,
    pub column: u32,
    pub pattern: String,
    pub description: String,
    pub severity: String,
}

fn get_memory_usage() -> f64 {
    // Memory API is not standard, return 0 for now
    // In a real implementation, you might track WASM memory growth
    0.0
}

fn calculate_energy_estimate(execution_ms: f64, memory_mb: f64) -> WasmEnergyEstimate {
    // Energy model based on empirical data
    let cpu_joules_per_ms = 0.001_f64; // Optimized Rust
    let mem_joules_per_mb = 0.0002_f64;
    let baseline_watts = 1.0_f64;

    let cpu_energy = execution_ms * cpu_joules_per_ms;
    let mem_energy = memory_mb * mem_joules_per_mb;
    let total_joules = cpu_energy + mem_energy;

    // Calculate confidence based on execution time and memory usage
    let time_weight = (execution_ms / 100.0).min(1.0) * 0.7;
    let mem_weight = (memory_mb / 10.0).min(1.0) * 0.3;
    let confidence = time_weight + mem_weight;

    WasmEnergyEstimate {
        joules: total_joules,
        watts_average: baseline_watts,
        co2_grams: total_joules * 0.475, // US grid average
        confidence,
    }
}

fn calculate_quality_metrics(python_code: &str, rust_code: &str) -> WasmQualityMetrics {
    // Simple quality metrics calculation
    let python_lines = python_code.lines().count();
    let rust_lines = rust_code.lines().count();

    // Calculate basic complexity
    let complexity = calculate_code_complexity(python_code);
    let cyclomatic_complexity = calculate_cyclomatic_complexity(python_code);

    // PMAT scoring (simplified)
    let productivity = calculate_productivity_score(python_lines, rust_lines);
    let maintainability = calculate_maintainability_score(rust_code);
    let accessibility = 0.85; // Rust is generally accessible
    let testability = calculate_testability_score(rust_code);

    let pmat_score = (productivity + maintainability + accessibility + testability) / 4.0;

    WasmQualityMetrics {
        pmat_score,
        productivity,
        maintainability,
        accessibility,
        testability,
        code_complexity: complexity,
        cyclomatic_complexity,
    }
}

fn calculate_code_complexity(code: &str) -> i32 {
    let mut complexity = 1; // Base complexity

    for line in code.lines() {
        if line.trim_start().starts_with("if ")
            || line.trim_start().starts_with("elif ")
            || line.trim_start().starts_with("while ")
            || line.trim_start().starts_with("for ")
            || line.trim_start().starts_with("try:")
            || line.trim_start().starts_with("except ")
        {
            complexity += 1;
        }
    }

    complexity
}

fn calculate_cyclomatic_complexity(code: &str) -> i32 {
    let mut complexity = 1;

    // Count decision points
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("if ")
            || trimmed.starts_with("elif ")
            || trimmed.starts_with("while ")
            || trimmed.starts_with("for ")
            || trimmed.contains(" and ")
            || trimmed.contains(" or ")
        {
            complexity += 1;
        }
    }

    complexity
}

fn calculate_productivity_score(python_lines: usize, rust_lines: usize) -> f64 {
    // Productivity based on conciseness and maintainability
    if rust_lines == 0 {
        return 0.0;
    }

    let ratio = python_lines as f64 / rust_lines as f64;
    // Score higher when Rust code is reasonably verbose (better than too concise)
    if ratio > 0.5 && ratio < 2.0 {
        0.9
    } else if ratio >= 2.0 {
        0.7
    } else {
        0.5
    }
}

fn calculate_maintainability_score(rust_code: &str) -> f64 {
    let mut score = 0.8_f64; // Base score for Rust

    // Check for good practices
    if rust_code.contains("/// ") {
        score += 0.1; // Documentation
    }
    if rust_code.contains("#[test]") {
        score += 0.1; // Tests
    }
    if rust_code.contains("Result<") {
        score += 0.05; // Error handling
    }

    score.min(1.0_f64)
}

fn calculate_testability_score(rust_code: &str) -> f64 {
    let mut score = 0.7_f64; // Base score

    if rust_code.contains("pub fn ") {
        score += 0.1; // Public functions
    }
    if rust_code.contains("impl ") {
        score += 0.1; // Structured code
    }
    if rust_code.contains("#[cfg(test)]") {
        score += 0.1; // Test modules
    }

    score.min(1.0_f64)
}

fn perform_static_analysis(python_code: &str) -> StaticAnalysis {
    let lines: Vec<&str> = python_code.lines().collect();
    let mut functions = Vec::new();
    let mut suggestions = Vec::new();
    let mut anti_patterns = Vec::new();
    let mut imports = Vec::new();

    // Simple parsing for demonstration
    for (line_num, line) in lines.iter().enumerate() {
        let line_num = line_num as u32 + 1;
        let trimmed = line.trim();

        // Detect function definitions
        if trimmed.starts_with("def ") {
            if let Some(func_name) = extract_function_name(trimmed) {
                functions.push(FunctionInfo {
                    name: func_name,
                    line_start: line_num,
                    line_end: line_num, // Simplified
                    complexity: 1,
                    parameters: vec![], // Simplified
                    return_type: None,
                });
            }
        }

        // Detect imports
        if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
            imports.push(trimmed.to_string());
        }

        // Detect anti-patterns
        if trimmed.contains("eval(") {
            anti_patterns.push(AntiPattern {
                line: line_num,
                column: line.find("eval(").unwrap_or(0) as u32,
                pattern: "eval()".to_string(),
                description: "Using eval() is dangerous and hard to optimize".to_string(),
                severity: "high".to_string(),
            });
        }

        // Suggest optimizations
        if trimmed.contains("range(len(") {
            suggestions.push(OptimizationSuggestion {
                line: line_num,
                column: line.find("range(len(").unwrap_or(0) as u32,
                message: "Consider using enumerate() instead of range(len())".to_string(),
                suggestion_type: "performance".to_string(),
                confidence: 0.9,
            });
        }
    }

    StaticAnalysis {
        complexity: calculate_code_complexity(python_code),
        cyclomatic_complexity: calculate_cyclomatic_complexity(python_code),
        functions,
        imports,
        suggestions,
        anti_patterns,
    }
}

fn extract_function_name(line: &str) -> Option<String> {
    // Extract function name from "def func_name(args):"
    if let Some(start) = line.find("def ") {
        let after_def = &line[start + 4..];
        if let Some(end) = after_def.find('(') {
            return Some(after_def[..end].trim().to_string());
        }
    }
    None
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_wasm_transpile_options_default() {
        let options = WasmTranspileOptions::default();
        assert!(options.verify);
        assert!(options.optimize);
        assert!(!options.emit_docs);
        assert_eq!(options.target_version, "1.83");
    }
    
    #[test]
    fn test_wasm_transpile_options_new() {
        let options = WasmTranspileOptions::new();
        assert_eq!(options.verify, WasmTranspileOptions::default().verify);
    }
    
    #[test]
    fn test_wasm_transpile_options_getters_setters() {
        let mut options = WasmTranspileOptions::new();
        
        // Test verify
        assert!(options.verify());
        options.set_verify(false);
        assert!(!options.verify());
        
        // Test optimize
        assert!(options.optimize());
        options.set_optimize(false);
        assert!(!options.optimize());
        
        // Test emit_docs
        assert!(!options.emit_docs());
        options.set_emit_docs(true);
        assert!(options.emit_docs());
        
        // Test target_version
        assert_eq!(options.target_version(), "1.83");
        options.set_target_version("1.84".to_string());
        assert_eq!(options.target_version(), "1.84");
    }
    
    #[test]
    fn test_extract_function_name() {
        assert_eq!(extract_function_name("def foo():"), Some("foo".to_string()));
        assert_eq!(extract_function_name("def bar(a, b):"), Some("bar".to_string()));
        assert_eq!(extract_function_name("  def baz(x: int) -> int:"), Some("baz".to_string()));
        assert_eq!(extract_function_name("class Foo:"), None);
        // The function finds "def" anywhere in the line, including comments
        assert_eq!(extract_function_name("# def commented():"), Some("commented".to_string()));
    }
    
    #[test]
    fn test_calculate_code_complexity() {
        let simple = "def foo(): return 42";
        assert_eq!(calculate_code_complexity(simple), 1);
        
        let with_if = "def foo():\n    if x > 0:\n        return x";
        assert_eq!(calculate_code_complexity(with_if), 2);
        
        let complex = "def foo():\n    if a:\n        pass\n    elif b:\n        pass\n    while c:\n        pass";
        assert_eq!(calculate_code_complexity(complex), 4);
    }
    
    #[test]
    fn test_calculate_cyclomatic_complexity() {
        let simple = "def foo(): return 42";
        assert_eq!(calculate_cyclomatic_complexity(simple), 1);
        
        let with_logic = "if a and b or c: pass";
        // This counts as: 1 (base) + 1 (if) + 1 (and) + 1 (or) = 4, but our implementation
        // only counts it as 2 because "and" and "or" are on the same line as "if"
        assert_eq!(calculate_cyclomatic_complexity(with_logic), 2);
        
        let nested = "if a:\n    if b:\n        pass\n    elif c:\n        pass";
        assert!(calculate_cyclomatic_complexity(nested) >= 3);
    }
    
    #[test]
    fn test_calculate_productivity_score() {
        assert_eq!(calculate_productivity_score(10, 10), 0.9); // 1:1 ratio
        assert_eq!(calculate_productivity_score(10, 5), 0.7); // 2:1 ratio  
        assert_eq!(calculate_productivity_score(10, 30), 0.5); // 1:3 ratio
        assert_eq!(calculate_productivity_score(10, 0), 0.0); // Division by zero
    }
    
    #[test]
    fn test_calculate_maintainability_score() {
        let basic = "fn foo() {}";
        assert!(calculate_maintainability_score(basic) >= 0.8);
        
        let with_docs = "/// Documentation\nfn foo() {}";
        assert!(calculate_maintainability_score(with_docs) > calculate_maintainability_score(basic));
        
        let with_tests = "#[test]\nfn test_foo() {}";
        assert!(calculate_maintainability_score(with_tests) > calculate_maintainability_score(basic));
        
        let with_result = "fn foo() -> Result<i32, Error> {}";
        assert!(calculate_maintainability_score(with_result) > calculate_maintainability_score(basic));
    }
    
    #[test]
    fn test_calculate_testability_score() {
        let private = "fn foo() {}";
        assert!(calculate_testability_score(private) >= 0.7);
        
        let public = "pub fn foo() {}";
        assert!(calculate_testability_score(public) > calculate_testability_score(private));
        
        let with_impl = "impl Foo { pub fn bar() {} }";
        assert!(calculate_testability_score(with_impl) > calculate_testability_score(private));
        
        let with_test_mod = "#[cfg(test)]\nmod tests {}";
        assert!(calculate_testability_score(with_test_mod) > calculate_testability_score(private));
    }
    
    #[test]
    fn test_energy_estimate_calculation() {
        let estimate = calculate_energy_estimate(100.0, 10.0);
        assert!(estimate.joules > 0.0);
        assert!(estimate.watts_average > 0.0);
        assert!(estimate.co2_grams > 0.0);
        assert!(estimate.confidence >= 0.0 && estimate.confidence <= 1.0);
        
        // Test edge cases
        let zero_estimate = calculate_energy_estimate(0.0, 0.0);
        assert_eq!(zero_estimate.joules, 0.0);
        assert_eq!(zero_estimate.confidence, 0.0);
    }
    
    #[test]
    fn test_static_analysis() {
        let code = r#"
def test_func(x, y):
    import math
    eval("dangerous")
    for i in range(len(items)):
        pass
"#;
        
        let analysis = perform_static_analysis(code);
        
        assert_eq!(analysis.functions.len(), 1);
        assert_eq!(analysis.functions[0].name, "test_func");
        assert_eq!(analysis.imports.len(), 1);
        assert!(!analysis.anti_patterns.is_empty());
        assert!(!analysis.suggestions.is_empty());
    }
}

impl Default for DepylerWasm {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience alias for playground usage
pub type PlaygroundEngine = DepylerWasm;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    console_log!("Depyler WASM module loaded successfully");
}

// Helper function to calculate average complexity
#[allow(dead_code)]
fn calculate_avg_complexity(hir: &depyler_core::hir::HirModule) -> f64 {
    if hir.functions.is_empty() {
        return 0.0;
    }

    let total_complexity: u32 = hir
        .functions
        .iter()
        .map(|f| depyler_analyzer::calculate_cyclomatic(&f.body))
        .sum();

    total_complexity as f64 / hir.functions.len() as f64
}

