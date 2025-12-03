//! Oracle Query Loop Demo (Issue #172)
//!
//! Demonstrates pattern-based error resolution using entrenar CITL patterns.
//!
//! # Usage
//! ```bash
//! cargo run -p depyler-oracle --example oracle_query_loop_demo
//! ```

use depyler_oracle::{
    auto_fix_loop, AutoFixResult, ErrorContext, OracleMetrics, OracleQueryLoop, OracleStats,
    QueryLoopConfig, RustErrorCode,
};
use std::path::PathBuf;

fn main() {
    println!("=== Oracle Query Loop Demo (Issue #172) ===\n");

    // Phase 1: Configuration
    println!("Phase 1: Configuration");
    let config = QueryLoopConfig {
        threshold: 0.7,
        max_suggestions: 3,
        boost_recent: true,
        max_retries: 3,
        llm_fallback: false,
    };
    println!("  - Confidence threshold: {}", config.threshold);
    println!("  - Max suggestions: {}", config.max_suggestions);
    println!("  - Max retries: {}", config.max_retries);
    println!("  - LLM fallback: {}", config.llm_fallback);

    // Phase 2: Create Oracle Query Loop
    println!("\nPhase 2: Oracle Query Loop");
    let mut oracle = OracleQueryLoop::with_config(config);
    println!("  - Oracle created with custom config");
    println!(
        "  - Default pattern path: {:?}",
        OracleQueryLoop::default_pattern_path()
    );

    // Phase 3: Error Code Parsing
    println!("\nPhase 3: Error Code Parsing");
    let error_codes = ["E0308", "E0382", "E0277", "E0599", "E9999"];
    for code_str in &error_codes {
        let code: RustErrorCode = code_str.parse().unwrap();
        println!("  - {} -> {:?} -> {}", code_str, code, code.as_str());
    }

    // Phase 4: Simulate Error Resolution
    println!("\nPhase 4: Auto-Fix Loop Simulation");
    let mut source = r#"let x: i32 = "hello";"#.to_string();
    let context = ErrorContext {
        file: PathBuf::from("test.rs"),
        line: 1,
        column: 14,
        source_snippet: source.clone(),
        surrounding_lines: vec![source.clone()],
    };

    // Simulate auto-fix (without actual patterns loaded)
    let result = auto_fix_loop(
        &mut oracle,
        &mut source,
        RustErrorCode::E0308,
        "mismatched types: expected i32, found &str",
        &context,
        |_| false, // Verification function (always fails in demo)
    );

    match result {
        AutoFixResult::NoSuggestion => {
            println!("  - Result: NoSuggestion (no patterns loaded in demo)");
        }
        AutoFixResult::Success {
            pattern_id,
            attempts,
            ..
        } => {
            println!(
                "  - Result: Success with pattern {} after {} attempts",
                pattern_id, attempts
            );
        }
        AutoFixResult::Exhausted {
            error_code,
            attempts,
        } => {
            println!(
                "  - Result: Exhausted {} attempts for {}",
                attempts, error_code
            );
        }
    }

    // Phase 5: Stats and Metrics
    println!("\nPhase 5: Oracle Statistics");
    let stats = oracle.stats();
    println!("  - Queries: {}", stats.queries);
    println!("  - Hits: {}", stats.hits);
    println!("  - Misses: {}", stats.misses);
    println!("  - Hit Rate: {:.1}%", stats.hit_rate() * 100.0);

    // Simulate some usage for metrics
    let demo_stats = OracleStats {
        queries: 100,
        hits: 85,
        misses: 15,
        fixes_applied: 72,
        fixes_verified: 68,
        llm_fallbacks: 8,
    };

    println!("\nPhase 6: Prometheus Metrics Export");
    let metrics = OracleMetrics::from_stats(&demo_stats);
    println!("  - queries_total: {}", metrics.queries_total);
    println!("  - hits_total: {}", metrics.hits_total);
    println!("  - hit_rate: {:.2}", metrics.hit_rate());
    println!("  - fix_success_rate: {:.2}", metrics.fix_success_rate());

    println!("\n--- Prometheus Format ---");
    println!("{}", metrics.to_prometheus());

    println!("=== Demo Complete ===");
}
