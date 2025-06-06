use anyhow::Result;
use depyler_annotations::{LambdaAnnotations, LambdaEventType, Architecture};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cold start optimization and pre-warming strategies for Lambda functions
#[derive(Debug, Clone)]
pub struct LambdaOptimizer {
    strategies: HashMap<OptimizationStrategy, OptimizationConfig>,
    performance_targets: PerformanceTargets,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptimizationStrategy {
    BinarySize,
    ColdStart,
    PreWarming,
    MemoryUsage,
    CompileTime,
}

#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub enabled: bool,
    pub aggressive_mode: bool,
    pub size_threshold_kb: Option<u32>,
    pub cold_start_threshold_ms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTargets {
    pub max_cold_start_ms: u32,
    pub max_binary_size_kb: u32,
    pub max_memory_usage_mb: u16,
    pub target_throughput_rps: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct OptimizationPlan {
    pub profile_overrides: HashMap<String, String>,
    pub cargo_flags: Vec<String>,
    pub rustc_flags: Vec<String>,
    pub pre_warm_code: String,
    pub init_array_code: String,
    pub dependency_optimizations: Vec<DependencyOptimization>,
}

#[derive(Debug, Clone)]
pub struct DependencyOptimization {
    pub crate_name: String,
    pub features: Vec<String>,
    pub disabled_features: Vec<String>,
    pub replacement: Option<String>,
}

impl Default for PerformanceTargets {
    fn default() -> Self {
        Self {
            max_cold_start_ms: 50,     // 50ms cold start target
            max_binary_size_kb: 2048,  // 2MB binary size target
            max_memory_usage_mb: 128,  // 128MB memory target
            target_throughput_rps: None,
        }
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            aggressive_mode: false,
            size_threshold_kb: Some(1024),
            cold_start_threshold_ms: Some(100),
        }
    }
}

impl Default for LambdaOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaOptimizer {
    pub fn new() -> Self {
        let mut strategies = HashMap::new();
        
        strategies.insert(OptimizationStrategy::BinarySize, OptimizationConfig {
            enabled: true,
            aggressive_mode: true,
            size_threshold_kb: Some(1024),
            cold_start_threshold_ms: None,
        });

        strategies.insert(OptimizationStrategy::ColdStart, OptimizationConfig {
            enabled: true,
            aggressive_mode: true,
            size_threshold_kb: None,
            cold_start_threshold_ms: Some(50),
        });

        strategies.insert(OptimizationStrategy::PreWarming, OptimizationConfig {
            enabled: true,
            aggressive_mode: false,
            size_threshold_kb: None,
            cold_start_threshold_ms: None,
        });

        strategies.insert(OptimizationStrategy::MemoryUsage, OptimizationConfig {
            enabled: true,
            aggressive_mode: false,
            size_threshold_kb: None,
            cold_start_threshold_ms: None,
        });

        Self {
            strategies,
            performance_targets: PerformanceTargets::default(),
        }
    }

    pub fn with_targets(mut self, targets: PerformanceTargets) -> Self {
        self.performance_targets = targets;
        self
    }

    pub fn enable_aggressive_mode(mut self) -> Self {
        for config in self.strategies.values_mut() {
            config.aggressive_mode = true;
        }
        self
    }

    /// Generate optimization plan based on Lambda annotations and event type
    pub fn generate_optimization_plan(&self, annotations: &LambdaAnnotations) -> Result<OptimizationPlan> {
        let mut plan = OptimizationPlan {
            profile_overrides: HashMap::new(),
            cargo_flags: Vec::new(),
            rustc_flags: Vec::new(),
            pre_warm_code: String::new(),
            init_array_code: String::new(),
            dependency_optimizations: Vec::new(),
        };

        // Binary size optimizations
        if self.is_strategy_enabled(&OptimizationStrategy::BinarySize) {
            self.apply_binary_size_optimizations(&mut plan, annotations)?;
        }

        // Cold start optimizations
        if self.is_strategy_enabled(&OptimizationStrategy::ColdStart) && annotations.cold_start_optimize {
            self.apply_cold_start_optimizations(&mut plan, annotations)?;
        }

        // Pre-warming optimizations
        if self.is_strategy_enabled(&OptimizationStrategy::PreWarming) {
            self.apply_pre_warming_optimizations(&mut plan, annotations)?;
        }

        // Memory usage optimizations
        if self.is_strategy_enabled(&OptimizationStrategy::MemoryUsage) {
            self.apply_memory_optimizations(&mut plan, annotations)?;
        }

        Ok(plan)
    }

    fn apply_binary_size_optimizations(&self, plan: &mut OptimizationPlan, annotations: &LambdaAnnotations) -> Result<()> {
        // Profile overrides for maximum size reduction
        plan.profile_overrides.insert("opt-level".to_string(), "z".to_string());
        plan.profile_overrides.insert("lto".to_string(), "true".to_string());
        plan.profile_overrides.insert("codegen-units".to_string(), "1".to_string());
        plan.profile_overrides.insert("panic".to_string(), "abort".to_string());
        plan.profile_overrides.insert("strip".to_string(), "true".to_string());
        plan.profile_overrides.insert("overflow-checks".to_string(), "false".to_string());
        plan.profile_overrides.insert("incremental".to_string(), "false".to_string());

        // Aggressive RUSTC flags for size optimization
        plan.rustc_flags.extend(vec![
            "-C link-arg=-s".to_string(),           // Strip symbols
            "-C opt-level=z".to_string(),           // Optimize for size
            "-C codegen-units=1".to_string(),       // Single codegen unit
            "-C lto=fat".to_string(),               // Fat LTO
            "-C embed-bitcode=no".to_string(),      // No bitcode embedding
            "-C panic=abort".to_string(),           // Abort on panic
        ]);

        // Architecture-specific optimizations
        match annotations.architecture {
            Architecture::Arm64 => {
                plan.rustc_flags.push("-C target-cpu=neoverse-n1".to_string());
            }
            Architecture::X86_64 => {
                plan.rustc_flags.push("-C target-cpu=haswell".to_string());
            }
        }

        // Dependency optimizations for size
        plan.dependency_optimizations.extend(vec![
            DependencyOptimization {
                crate_name: "serde".to_string(),
                features: vec!["derive".to_string()],
                disabled_features: vec!["std".to_string()],
                replacement: None,
            },
            DependencyOptimization {
                crate_name: "tokio".to_string(),
                features: vec!["rt".to_string(), "macros".to_string()],
                disabled_features: vec!["full".to_string(), "test-util".to_string()],
                replacement: None,
            },
        ]);

        Ok(())
    }

    fn apply_cold_start_optimizations(&self, plan: &mut OptimizationPlan, annotations: &LambdaAnnotations) -> Result<()> {
        // Pre-allocation and warming for common types
        let mut pre_warm_code = String::new();
        
        if let Some(ref event_type) = annotations.event_type {
            match event_type {
                LambdaEventType::ApiGatewayProxyRequest | LambdaEventType::ApiGatewayV2HttpRequest => {
                    pre_warm_code.push_str(
                        r#"
    // Pre-warm API Gateway types
    let _ = std::hint::black_box(serde_json::from_str::<serde_json::Value>("{{}}"));
    let _ = std::hint::black_box(std::collections::HashMap::<String, String>::new());
    
    // Pre-allocate response buffers
    let mut response_buf = Vec::with_capacity(4096);
    response_buf.push(0);
    std::mem::forget(response_buf);
"#
                    );
                }
                LambdaEventType::SqsEvent => {
                    pre_warm_code.push_str(
                        r#"
    // Pre-warm SQS types
    let _ = std::hint::black_box(Vec::<String>::with_capacity(10));
    let _ = std::hint::black_box(String::with_capacity(1024));
"#
                    );
                }
                LambdaEventType::S3Event => {
                    pre_warm_code.push_str(
                        r#"
    // Pre-warm S3 types
    let _ = std::hint::black_box(std::path::PathBuf::new());
    let _ = std::hint::black_box(String::with_capacity(512));
"#
                    );
                }
                _ => {}
            }
        }

        // Global pre-warming
        pre_warm_code.push_str(
            r#"
    // Pre-warm common allocations
    let _ = std::hint::black_box(serde_json::Value::Null);
    
    // Initialize thread-local storage
    thread_local! {{
        static BUFFER: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(8192));
    }}
    BUFFER.with(|_| {{}});
"#
        );

        plan.pre_warm_code = pre_warm_code;

        // Init array for early initialization
        plan.init_array_code = r#"
#[link_section = ".init_array"]
static INIT: extern "C" fn() = {{
    extern "C" fn init() {{
        // Pre-warm critical allocators
        let _ = std::hint::black_box(Vec::<u8>::with_capacity(1024));
        let _ = std::hint::black_box(String::with_capacity(512));
        
        // Initialize mimalloc if enabled
        #[cfg(feature = "mimalloc")]
        {{
            use mimalloc::MiMalloc;
            let _ = std::hint::black_box(&MiMalloc);
        }}
    }}
    init
}};
"#.to_string();

        // Optimize for latency over throughput
        plan.profile_overrides.insert("opt-level".to_string(), "3".to_string());

        Ok(())
    }

    fn apply_pre_warming_optimizations(&self, plan: &mut OptimizationPlan, annotations: &LambdaAnnotations) -> Result<()> {
        // Event-specific pre-warming paths
        for path in &annotations.pre_warm_paths {
            plan.pre_warm_code.push_str(&format!(
                "    // Pre-warm path: {path}\n    let _ = std::hint::black_box(String::from(\"{path}\"));\n"
            ));
        }

        // Serde pre-warming for custom serialization
        if annotations.custom_serialization {
            plan.pre_warm_code.push_str(
                r#"
    // Pre-warm custom serialization paths
    let _ = std::hint::black_box(serde_json::to_string(&serde_json::Value::Null));
    let _ = std::hint::black_box(serde_json::from_str::<serde_json::Value>("null"));
"#
            );
        }

        Ok(())
    }

    fn apply_memory_optimizations(&self, plan: &mut OptimizationPlan, annotations: &LambdaAnnotations) -> Result<()> {
        // Use mimalloc for better memory allocation patterns
        plan.dependency_optimizations.push(DependencyOptimization {
            crate_name: "mimalloc".to_string(),
            features: vec!["local_dynamic_tls".to_string()],
            disabled_features: vec!["debug".to_string()],
            replacement: None,
        });

        // Memory pool initialization for low-memory environments
        if annotations.memory_size <= 128 {
            plan.pre_warm_code.push_str(&format!(
                r#"
    // Memory-constrained optimization
    if std::env::var("AWS_LAMBDA_FUNCTION_MEMORY_SIZE").unwrap_or_default() == "{}" {{
        // Conservative pre-allocation for low memory
        let _ = std::hint::black_box(Vec::<u8>::with_capacity(512));
    }}
"#,
                annotations.memory_size
            ));
        }

        // Stack size optimization
        plan.rustc_flags.push("-C link-arg=-Wl,-z,stack-size=131072".to_string()); // 128KB stack

        Ok(())
    }

    /// Generate Cargo profile for Lambda optimization
    pub fn generate_lambda_profile(&self, plan: &OptimizationPlan) -> String {
        let mut profile = String::from("\n[profile.lambda]\ninherits = \"release\"\n");
        
        for (key, value) in &plan.profile_overrides {
            profile.push_str(&format!("{key} = {value}\n"));
        }

        // Add lambda-specific package overrides
        profile.push_str("\n[profile.lambda.package.\"*\"]\n");
        profile.push_str("opt-level = \"z\"\n");
        profile.push_str("debug = false\n");

        profile
    }

    /// Generate build script with optimization flags
    pub fn generate_optimized_build_script(&self, plan: &OptimizationPlan, annotations: &LambdaAnnotations) -> String {
        let mut script = format!(
            r#"#!/bin/bash
# Generated optimized build script for AWS Lambda

set -e

echo "Building optimized Lambda function..."
echo "Target: {} MB memory, {} architecture"

"#,
            annotations.memory_size,
            match annotations.architecture {
                Architecture::Arm64 => "ARM64",
                Architecture::X86_64 => "x86_64",
            }
        );

        // Set environment variables
        script.push_str("# Optimization environment variables\n");
        script.push_str(&format!("export RUSTFLAGS=\"{}\"\n", plan.rustc_flags.join(" ")));
        script.push_str("export CARGO_PROFILE_LAMBDA_LTO=true\n");
        script.push_str("export CARGO_PROFILE_LAMBDA_PANIC=\"abort\"\n");
        script.push_str("export CARGO_PROFILE_LAMBDA_CODEGEN_UNITS=1\n");

        // Build command
        let arch_flag = match annotations.architecture {
            Architecture::Arm64 => "--arm64",
            Architecture::X86_64 => "--x86-64",
        };

        script.push_str(&format!(
            r#"
# Build with cargo-lambda
cargo lambda build \
    --profile lambda \
    {arch_flag} \
    --output-format zip

"#
        ));

        // Post-build optimizations
        script.push_str(
            r#"
# Post-build optimizations
BINARY_PATH="target/lambda/*/bootstrap"

if command -v strip > /dev/null; then
    echo "Stripping binary..."
    strip $BINARY_PATH
fi

if command -v upx > /dev/null; then
    echo "Compressing binary with UPX..."
    upx --best --lzma $BINARY_PATH || echo "UPX compression failed, continuing..."
fi

# Size reporting
BINARY_SIZE=$(du -h $BINARY_PATH | cut -f1)
echo "Final binary size: $BINARY_SIZE"

# Cold start benchmark (if available)
if command -v hyperfine > /dev/null; then
    echo "Running cold start benchmark..."
    # This would require a test harness
    echo "Benchmark skipped - implement with your test harness"
fi

echo "Build completed successfully!"
"#
        );

        script
    }

    /// Generate performance monitoring code
    pub fn generate_performance_monitoring(&self, _annotations: &LambdaAnnotations) -> String {
        format!(
            r#"
use std::time::Instant;

#[cfg(feature = "performance-monitoring")]
mod performance {{
    use super::*;
    
    pub struct PerformanceMonitor {{
        start_time: Instant,
        cold_start: bool,
    }}
    
    impl PerformanceMonitor {{
        pub fn new() -> Self {{
            Self {{
                start_time: Instant::now(),
                cold_start: std::env::var("_LAMBDA_START_TIME").is_err(),
            }}
        }}
        
        pub fn log_cold_start(&self) {{
            if self.cold_start {{
                let duration = self.start_time.elapsed();
                eprintln!("MONITORING cold_start_duration_ms:{{}}", duration.as_millis());
                
                if duration.as_millis() > {} {{
                    eprintln!("WARNING: Cold start exceeded target of {}ms", {});
                }}
            }}
        }}
        
        pub fn log_memory_usage(&self) {{
            // This would require a memory profiling crate
            if let Ok(memory_info) = std::fs::read_to_string("/proc/self/status") {{
                for line in memory_info.lines() {{
                    if line.starts_with("VmRSS:") {{
                        eprintln!("MONITORING memory_usage:{{}}", line);
                        break;
                    }}
                }}
            }}
        }}
    }}
}}
"#,
            self.performance_targets.max_cold_start_ms,
            self.performance_targets.max_cold_start_ms,
            self.performance_targets.max_cold_start_ms,
        )
    }

    /// Check if optimization strategy is enabled
    fn is_strategy_enabled(&self, strategy: &OptimizationStrategy) -> bool {
        self.strategies.get(strategy).is_some_and(|config| config.enabled)
    }

    /// Estimate performance impact of optimizations
    pub fn estimate_performance_impact(&self, plan: &OptimizationPlan) -> PerformanceEstimate {
        let mut estimate = PerformanceEstimate::default();

        // Binary size reduction estimation
        if plan.profile_overrides.contains_key("opt-level") {
            estimate.binary_size_reduction_percent += 25.0;
        }
        if plan.rustc_flags.iter().any(|f| f.contains("lto=fat")) {
            estimate.binary_size_reduction_percent += 15.0;
        }
        if plan.rustc_flags.iter().any(|f| f.contains("strip")) {
            estimate.binary_size_reduction_percent += 30.0;
        }

        // Cold start improvement estimation
        if !plan.pre_warm_code.is_empty() {
            estimate.cold_start_improvement_percent += 40.0;
        }
        if !plan.init_array_code.is_empty() {
            estimate.cold_start_improvement_percent += 20.0;
        }

        // Memory usage improvement
        if plan.dependency_optimizations.iter().any(|d| d.crate_name == "mimalloc") {
            estimate.memory_improvement_percent += 15.0;
        }

        estimate
    }
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceEstimate {
    pub binary_size_reduction_percent: f32,
    pub cold_start_improvement_percent: f32,
    pub memory_improvement_percent: f32,
    pub compile_time_impact_percent: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::LambdaAnnotations;

    #[test]
    fn test_optimization_plan_generation() {
        let optimizer = LambdaOptimizer::new();
        let annotations = LambdaAnnotations {
            cold_start_optimize: true,
            event_type: Some(LambdaEventType::ApiGatewayProxyRequest),
            ..Default::default()
        };
        
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        assert!(!plan.profile_overrides.is_empty());
        assert!(!plan.rustc_flags.is_empty());
        assert!(!plan.pre_warm_code.is_empty());
    }

    #[test]
    fn test_binary_size_optimizations() {
        let mut optimizer = LambdaOptimizer::new();
        // Disable cold start optimization to test only binary size
        optimizer.strategies.get_mut(&OptimizationStrategy::ColdStart).unwrap().enabled = false;
        let annotations = LambdaAnnotations::default();
        
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        assert!(plan.profile_overrides.get("opt-level").unwrap() == "z");
        assert!(plan.profile_overrides.get("lto").unwrap() == "true");
        assert!(plan.rustc_flags.iter().any(|f| f.contains("link-arg=-s")));
    }

    #[test]
    fn test_cold_start_optimizations() {
        let optimizer = LambdaOptimizer::new();
        let annotations = LambdaAnnotations {
            cold_start_optimize: true,
            event_type: Some(LambdaEventType::SqsEvent),
            ..Default::default()
        };
        
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        assert!(plan.pre_warm_code.contains("Pre-warm SQS types"));
        assert!(!plan.init_array_code.is_empty());
    }

    #[test]
    fn test_memory_optimizations() {
        let optimizer = LambdaOptimizer::new();
        let annotations = LambdaAnnotations {
            memory_size: 128,
            ..Default::default()
        };
        
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        assert!(plan.dependency_optimizations.iter().any(|d| d.crate_name == "mimalloc"));
        assert!(plan.pre_warm_code.contains("Memory-constrained optimization"));
    }

    #[test]
    fn test_lambda_profile_generation() {
        let optimizer = LambdaOptimizer::new();
        let annotations = LambdaAnnotations::default();
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        let profile = optimizer.generate_lambda_profile(&plan);
        
        assert!(profile.contains("[profile.lambda]"));
        assert!(profile.contains("opt-level = \"z\""));
        assert!(profile.contains("lto = true"));
    }

    #[test]
    fn test_build_script_generation() {
        let optimizer = LambdaOptimizer::new();
        let annotations = LambdaAnnotations::default();
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        let script = optimizer.generate_optimized_build_script(&plan, &annotations);
        
        assert!(script.contains("cargo lambda build"));
        assert!(script.contains("export RUSTFLAGS"));
        assert!(script.contains("upx --best"));
    }

    #[test]
    fn test_performance_estimate() {
        let optimizer = LambdaOptimizer::new();
        let annotations = LambdaAnnotations::default();
        let plan = optimizer.generate_optimization_plan(&annotations).unwrap();
        
        let estimate = optimizer.estimate_performance_impact(&plan);
        
        assert!(estimate.binary_size_reduction_percent > 0.0);
        assert!(estimate.cold_start_improvement_percent >= 0.0);
    }

    #[test]
    fn test_aggressive_mode() {
        let optimizer = LambdaOptimizer::new().enable_aggressive_mode();
        
        for config in optimizer.strategies.values() {
            assert!(config.aggressive_mode);
        }
    }

    #[test]
    fn test_custom_performance_targets() {
        let targets = PerformanceTargets {
            max_cold_start_ms: 25,
            max_binary_size_kb: 1024,
            max_memory_usage_mb: 64,
            target_throughput_rps: Some(1000),
        };
        
        let optimizer = LambdaOptimizer::new().with_targets(targets);
        assert_eq!(optimizer.performance_targets.max_cold_start_ms, 25);
        assert_eq!(optimizer.performance_targets.max_binary_size_kb, 1024);
        assert_eq!(optimizer.performance_targets.target_throughput_rps, Some(1000));
    }
}