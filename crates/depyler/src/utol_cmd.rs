//! UTOL: Unified Training Oracle Loop command handler
//!
//! Simplified implementation focused on testable pure functions.

use std::path::PathBuf;

use anyhow::Result;
use depyler_oracle::utol::{
    ConvergenceConfig, CorpusConfig, DisplayConfig, DisplayMode, UtolConfig, UtolResult,
};

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
    _watch: bool,
    _watch_debounce: u64,
) -> Result<()> {
    let config = build_config(corpus, target_rate, max_iterations, patience, &display)?;

    if status {
        return show_status(&config);
    }

    run_single_utol(&config, output)
}

/// Run a single UTOL iteration
fn run_single_utol(config: &UtolConfig, output: Option<PathBuf>) -> Result<()> {
    let result = run_utol_loop(config)?;

    if let Some(output_path) = output {
        let json = serde_json::to_string_pretty(&result)?;
        std::fs::write(&output_path, json)?;
        println!("Results saved to: {}", output_path.display());
    }

    print_final_summary(&result, config);
    Ok(())
}

/// Build UTOL configuration from CLI arguments
pub fn build_config(
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
pub fn show_status(config: &UtolConfig) -> Result<()> {
    println!("UTOL Status Report");
    println!("==================");

    let corpus_status = if config.corpus.path.exists() {
        "OK"
    } else {
        "Missing"
    };
    println!(
        "Corpus: {} ({})",
        config.corpus.path.display(),
        corpus_status
    );

    let model_status = if config.model.path.exists() {
        "OK"
    } else {
        "Missing"
    };
    println!("Model:  {} ({})", config.model.path.display(), model_status);

    println!();
    println!(
        "Target Rate:    {:.1}%",
        config.convergence.target_rate * 100.0
    );
    println!("Max Iterations: {}", config.convergence.max_iterations);
    println!("Patience:       {}", config.convergence.patience);

    Ok(())
}

/// Run the main UTOL loop
fn run_utol_loop(config: &UtolConfig) -> Result<UtolResult> {
    depyler_oracle::utol::run_utol(config)
}

/// Print final summary
pub fn print_final_summary(result: &UtolResult, config: &UtolConfig) {
    if matches!(config.display.mode, DisplayMode::Json) {
        if let Ok(json) = serde_json::to_string_pretty(result) {
            println!("{}", json);
        }
        return;
    }

    if matches!(config.display.mode, DisplayMode::Silent) {
        return;
    }

    println!();
    println!("UTOL Final Report");
    println!("=================");
    println!(
        "Status:     {}",
        if result.converged {
            "CONVERGED"
        } else {
            "NOT CONVERGED"
        }
    );
    println!("Final Rate: {:.1}%", result.compile_rate * 100.0);
    println!("Iterations: {}", result.iterations);
    println!("Duration:   {:.1}s", result.duration_secs);
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
    fn test_build_config_empty_corpus() {
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        assert!(!config.corpus.path.as_os_str().is_empty());
    }

    #[test]
    fn test_build_config_large_iterations() {
        let config = build_config(None, 0.99, 1000, 100, "rich").unwrap();
        assert_eq!(config.convergence.max_iterations, 1000);
        assert_eq!(config.convergence.patience, 100);
    }

    #[test]
    fn test_show_status_with_temp_corpus() {
        let config = build_config(Some(PathBuf::from("/tmp")), 0.80, 50, 5, "rich").unwrap();
        let result = show_status(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_status_with_nonexistent_corpus() {
        let config =
            build_config(Some(PathBuf::from("/nonexistent")), 0.80, 50, 5, "rich").unwrap();
        let result = show_status(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_final_summary_json_mode() {
        let result = UtolResult {
            converged: true,
            compile_rate: 0.95,
            iterations: 10,
            duration_secs: 5.0,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 0.80, 50, 5, "json").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_silent_mode() {
        let result = UtolResult {
            converged: false,
            compile_rate: 0.50,
            iterations: 50,
            duration_secs: 100.0,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 0.80, 50, 5, "silent").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_rich_converged() {
        let result = UtolResult {
            converged: true,
            compile_rate: 0.85,
            iterations: 20,
            duration_secs: 30.0,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_rich_not_converged() {
        let result = UtolResult {
            converged: false,
            compile_rate: 0.60,
            iterations: 50,
            duration_secs: 200.0,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_minimal_mode() {
        let result = UtolResult {
            converged: true,
            compile_rate: 0.90,
            iterations: 15,
            duration_secs: 25.0,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 0.80, 50, 5, "minimal").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_zero_rate() {
        let result = UtolResult {
            converged: false,
            compile_rate: 0.0,
            iterations: 1,
            duration_secs: 0.1,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 0.80, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_print_final_summary_full_rate() {
        let result = UtolResult {
            converged: true,
            compile_rate: 1.0,
            iterations: 5,
            duration_secs: 10.0,
            model_version: "v1".to_string(),
            category_rates: Default::default(),
        };
        let config = build_config(None, 1.0, 50, 5, "rich").unwrap();
        print_final_summary(&result, &config);
    }

    #[test]
    fn test_handle_utol_command_status_only() {
        let result = handle_utol_command(
            Some(PathBuf::from("/tmp")),
            0.80,
            50,
            5,
            "rich".to_string(),
            None,
            None,
            true,
            false,
            500,
        );
        assert!(result.is_ok());
    }
}
