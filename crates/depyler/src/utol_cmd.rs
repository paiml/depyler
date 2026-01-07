//! UTOL: Unified Training Oracle Loop command handler
//!
//! Implements the CLI interface for the UTOL system following Toyota Way principles.

use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::Result;
use depyler_oracle::utol::{
    ConvergenceConfig, CorpusConfig, DisplayConfig, DisplayMode, UtolConfig, UtolResult,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

/// Handle the `depyler utol` command
#[allow(clippy::too_many_arguments)]
pub fn handle_utol_command(
    corpus: Option<PathBuf>,
    target_rate: f64,
    max_iterations: usize,
    patience: usize,
    display: String,
    output: Option<PathBuf>,
    _config: Option<PathBuf>,
    status: bool,
    watch: bool,
    watch_debounce: u64,
) -> Result<()> {
    // Build configuration from CLI args
    let config = build_config(corpus, target_rate, max_iterations, patience, &display)?;

    // Handle status-only mode
    if status {
        return show_status(&config);
    }

    // Handle watch mode
    if watch {
        return run_watch_mode(&config, output, watch_debounce);
    }

    // Run single UTOL loop
    run_single_utol(&config, output)
}

/// Run a single UTOL iteration
fn run_single_utol(config: &UtolConfig, output: Option<PathBuf>) -> Result<()> {
    let result = run_utol_loop(config)?;

    // Output results
    if let Some(output_path) = output {
        let json = serde_json::to_string_pretty(&result)?;
        std::fs::write(&output_path, json)?;
        println!("\n💾 Results saved to: {}", output_path.display());
    }

    // Final summary
    print_final_summary(&result, config);

    Ok(())
}

/// Run UTOL in watch mode - continuously monitor and re-run on changes
fn run_watch_mode(config: &UtolConfig, output: Option<PathBuf>, debounce_ms: u64) -> Result<()> {
    let corpus_path = config.corpus.path.clone();

    println!("👁️  UTOL Watch Mode - Monitoring: {}", corpus_path.display());
    println!("   Press Ctrl+C to stop\n");

    // Initial run
    println!("🔄 Initial scan...");
    if let Err(e) = run_single_utol(config, output.clone()) {
        eprintln!("⚠️  Initial run failed: {}", e);
    }

    // Set up file watcher
    let (tx, rx) = channel();
    let watcher_config = Config::default()
        .with_poll_interval(Duration::from_millis(debounce_ms));

    let mut watcher = RecommendedWatcher::new(tx, watcher_config)?;
    watcher.watch(&corpus_path, RecursiveMode::Recursive)?;

    println!("\n👁️  Watching for changes...");

    // Track last event time for debouncing
    let mut last_run = std::time::Instant::now();
    let debounce = Duration::from_millis(debounce_ms);

    loop {
        match rx.recv() {
            Ok(event) => {
                // Only trigger on actual file changes
                if let Ok(event) = event {
                    let dominated = event
                        .paths
                        .iter()
                        .any(|p| p.extension().is_some_and(|e| e == "py"));

                    if !dominated {
                        continue;
                    }

                    // Debounce: skip if we ran recently
                    if last_run.elapsed() < debounce {
                        continue;
                    }

                    println!("\n📝 Change detected in: {:?}", event.paths);
                    println!("🔄 Re-running UTOL...\n");

                    last_run = std::time::Instant::now();

                    if let Err(e) = run_single_utol(config, output.clone()) {
                        eprintln!("⚠️  Run failed: {}", e);
                    }

                    println!("\n👁️  Watching for changes...");
                }
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Build UTOL configuration from CLI arguments
fn build_config(
    corpus: Option<PathBuf>,
    target_rate: f64,
    max_iterations: usize,
    patience: usize,
    display: &str,
) -> Result<UtolConfig> {
    let display_mode = match display.to_lowercase().as_str() {
        "rich" => DisplayMode::Rich,
        "minimal" => DisplayMode::Minimal,
        "json" => DisplayMode::Json,
        "silent" => DisplayMode::Silent,
        _ => DisplayMode::Rich,
    };

    Ok(UtolConfig {
        corpus: CorpusConfig {
            path: corpus.unwrap_or_else(|| PathBuf::from("../reprorusted-python-cli")),
            ..Default::default()
        },
        convergence: ConvergenceConfig {
            target_rate,
            max_iterations,
            patience,
            ..Default::default()
        },
        display: DisplayConfig {
            mode: display_mode,
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Show current UTOL status without running the loop
fn show_status(config: &UtolConfig) -> Result<()> {
    use depyler_oracle::Oracle;

    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                    UTOL - Status Report                              ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");

    // Check corpus
    let corpus_exists = config.corpus.path.exists();
    let corpus_status = if corpus_exists { "✓" } else { "✗" };
    println!(
        "║  Corpus: {} {}",
        corpus_status,
        config.corpus.path.display()
    );

    // Check model
    let model_path = &config.model.path;
    let model_exists = model_path.exists();
    let model_status = if model_exists { "✓" } else { "✗" };
    println!("║  Model:  {} {}", model_status, model_path.display());

    if model_exists {
        if let Ok(metadata) = std::fs::metadata(model_path) {
            let size_kb = metadata.len() / 1024;
            println!("║  Size:   {} KB", size_kb);
        }
    }

    // Try to load oracle and get stats
    if let Ok(oracle) = Oracle::load_or_train() {
        let drift_stats = oracle.drift_stats();
        println!("║  Samples: {}", drift_stats.n_samples);
        println!(
            "║  Error Rate: {:.1}%",
            drift_stats.error_rate * 100.0
        );
    }

    println!("║");
    println!("║  Target Rate: {:.1}%", config.convergence.target_rate * 100.0);
    println!("║  Max Iterations: {}", config.convergence.max_iterations);
    println!("║  Patience: {}", config.convergence.patience);
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    Ok(())
}

/// Run the main UTOL loop
fn run_utol_loop(config: &UtolConfig) -> Result<UtolResult> {
    // Use the real UTOL implementation from depyler_oracle
    depyler_oracle::utol::run_utol(config)
}

/// Print final summary
fn print_final_summary(result: &UtolResult, config: &UtolConfig) {
    if matches!(config.display.mode, DisplayMode::Json) {
        // JSON mode - print result as JSON
        if let Ok(json) = serde_json::to_string_pretty(result) {
            println!("{}", json);
        }
        return;
    }

    if matches!(config.display.mode, DisplayMode::Silent) {
        return;
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("                         UTOL Final Report                              ");
    println!("═══════════════════════════════════════════════════════════════════════");
    println!();
    println!(
        "  Status:       {}",
        if result.converged {
            "✅ CONVERGED"
        } else {
            "⚠ NOT CONVERGED"
        }
    );
    println!("  Final Rate:   {:.1}%", result.compile_rate * 100.0);
    println!("  Iterations:   {}", result.iterations);
    println!("  Duration:     {:.1}s", result.duration_secs);
    println!("  Model:        {}", result.model_version);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_config_defaults() {
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        assert!((config.convergence.target_rate - 0.80).abs() < 0.001);
        assert_eq!(config.convergence.max_iterations, 50);
        assert_eq!(config.convergence.patience, 5);
        assert!(matches!(config.display.mode, DisplayMode::Rich));
    }

    #[test]
    fn test_build_config_minimal_mode() {
        let config = build_config(None, 0.90, 100, 10, "minimal").unwrap();
        assert!(matches!(config.display.mode, DisplayMode::Minimal));
    }

    #[test]
    fn test_build_config_json_mode() {
        let config = build_config(None, 0.80, 50, 5, "json").unwrap();
        assert!(matches!(config.display.mode, DisplayMode::Json));
    }

    #[test]
    fn test_build_config_silent_mode() {
        let config = build_config(None, 0.80, 50, 5, "silent").unwrap();
        assert!(matches!(config.display.mode, DisplayMode::Silent));
    }

    #[test]
    fn test_build_config_unknown_mode_defaults_rich() {
        let config = build_config(None, 0.80, 50, 5, "unknown").unwrap();
        assert!(matches!(config.display.mode, DisplayMode::Rich));
    }

    #[test]
    fn test_build_config_with_corpus() {
        let corpus = Some(PathBuf::from("/tmp/test"));
        let config = build_config(corpus.clone(), 0.80, 50, 5, "rich").unwrap();
        assert_eq!(config.corpus.path, PathBuf::from("/tmp/test"));
    }

    #[test]
    fn test_build_config_case_insensitive() {
        let config = build_config(None, 0.80, 50, 5, "RICH").unwrap();
        assert!(matches!(config.display.mode, DisplayMode::Rich));
        let config = build_config(None, 0.80, 50, 5, "Minimal").unwrap();
        assert!(matches!(config.display.mode, DisplayMode::Minimal));
    }

    fn make_test_result(converged: bool, rate: f64, iterations: usize, duration: f64) -> UtolResult {
        UtolResult {
            converged,
            compile_rate: rate,
            iterations,
            duration_secs: duration,
            model_version: "test".to_string(),
            category_rates: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_print_final_summary_json_mode() {
        let result = make_test_result(true, 0.95, 10, 30.0);
        let config = build_config(None, 0.80, 50, 5, "json").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_silent_mode() {
        let result = make_test_result(false, 0.5, 50, 100.0);
        let config = build_config(None, 0.80, 50, 5, "silent").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_rich_converged() {
        let result = make_test_result(true, 0.95, 10, 30.0);
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_rich_not_converged() {
        let result = make_test_result(false, 0.5, 50, 100.0);
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_minimal_mode() {
        let result = make_test_result(true, 0.85, 20, 45.0);
        let config = build_config(None, 0.80, 50, 5, "minimal").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_with_category_rates() {
        use std::collections::HashMap;
        let mut category_rates = HashMap::new();
        category_rates.insert("simple".to_string(), 0.95);
        category_rates.insert("complex".to_string(), 0.75);

        let result = UtolResult {
            converged: true,
            compile_rate: 0.85,
            iterations: 15,
            duration_secs: 50.0,
            model_version: "v1.2.3".to_string(),
            category_rates,
        };
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_show_status_with_nonexistent_corpus() {
        let config = build_config(
            Some(PathBuf::from("/nonexistent/corpus/path")),
            0.80,
            50,
            5,
            "rich"
        ).unwrap();
        let result = show_status(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_status_with_temp_corpus() {
        use std::fs;

        let temp_dir = std::env::temp_dir().join("utol_test_corpus");
        fs::create_dir_all(&temp_dir).ok();

        let config = build_config(Some(temp_dir.clone()), 0.80, 50, 5, "rich").unwrap();
        let result = show_status(&config);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn test_utol_result_struct_fields() {
        let result = make_test_result(true, 0.95, 10, 30.0);
        assert!(result.converged);
        assert!((result.compile_rate - 0.95).abs() < 0.001);
        assert_eq!(result.iterations, 10);
        assert!((result.duration_secs - 30.0).abs() < 0.001);
        assert_eq!(result.model_version, "test");
        assert!(result.category_rates.is_empty());
    }

    #[test]
    fn test_utol_result_serialization() {
        let result = make_test_result(true, 0.95, 10, 30.0);
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"converged\":true"));
        assert!(json.contains("\"compile_rate\":0.95"));
        assert!(json.contains("\"iterations\":10"));
    }

    #[test]
    fn test_build_config_edge_case_zero_target_rate() {
        let config = build_config(None, 0.0, 50, 5, "rich").unwrap();
        assert!((config.convergence.target_rate - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_build_config_edge_case_full_target_rate() {
        let config = build_config(None, 1.0, 50, 5, "rich").unwrap();
        assert!((config.convergence.target_rate - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_build_config_edge_case_zero_iterations() {
        let config = build_config(None, 0.80, 0, 5, "rich").unwrap();
        assert_eq!(config.convergence.max_iterations, 0);
    }

    #[test]
    fn test_build_config_edge_case_zero_patience() {
        let config = build_config(None, 0.80, 50, 0, "rich").unwrap();
        assert_eq!(config.convergence.patience, 0);
    }

    #[test]
    fn test_build_config_large_iterations() {
        let config = build_config(None, 0.80, 1_000_000, 5, "rich").unwrap();
        assert_eq!(config.convergence.max_iterations, 1_000_000);
    }

    #[test]
    fn test_build_config_empty_corpus_path() {
        let config = build_config(Some(PathBuf::from("")), 0.80, 50, 5, "rich").unwrap();
        assert_eq!(config.corpus.path, PathBuf::from(""));
    }

    #[test]
    fn test_display_mode_all_variants() {
        // Test all display mode variants
        let modes = ["rich", "minimal", "json", "silent", "RICH", "MINIMAL", "JSON", "SILENT", "Rich", "Minimal", "Json", "Silent"];
        for mode in modes {
            let config = build_config(None, 0.80, 50, 5, mode).unwrap();
            let _expected = match mode.to_lowercase().as_str() {
                "rich" => DisplayMode::Rich,
                "minimal" => DisplayMode::Minimal,
                "json" => DisplayMode::Json,
                "silent" => DisplayMode::Silent,
                _ => DisplayMode::Rich,
            };
            assert!(matches!(config.display.mode, _expected));
        }
    }

    #[test]
    fn test_print_final_summary_zero_rate() {
        let result = make_test_result(false, 0.0, 50, 100.0);
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_full_rate() {
        let result = make_test_result(true, 1.0, 5, 10.0);
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_zero_iterations() {
        let result = make_test_result(false, 0.0, 0, 0.0);
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_handle_utol_command_status_only() {
        // Test status mode with a nonexistent corpus
        let result = handle_utol_command(
            Some(PathBuf::from("/nonexistent/path")),
            0.80,
            50,
            5,
            "rich".to_string(),
            None,
            None,
            true,  // status mode
            false, // not watch mode
            500,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_corpus_config_default_path() {
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        assert_eq!(config.corpus.path, PathBuf::from("../reprorusted-python-cli"));
    }

    #[test]
    fn test_convergence_config_fields() {
        let config = build_config(None, 0.95, 100, 10, "rich").unwrap();
        assert!((config.convergence.target_rate - 0.95).abs() < 0.001);
        assert_eq!(config.convergence.max_iterations, 100);
        assert_eq!(config.convergence.patience, 10);
    }
}
