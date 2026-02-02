//! DEPYLER-DASH-001: Sovereign Stack Coverage Dashboard
//!
//! Tracks coverage of sovereign stack components vs their Python equivalents:
//! - aprender vs sklearn (ML algorithms)
//! - trueno vs numpy (array operations)
//! - pandas replacement via trueno (data manipulation)
//!
//! Provides visual feedback on Path B (Sovereign Fallback) progress.

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};

/// Sovereign stack component mapping status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCoverage {
    /// Name of the sovereign component (e.g., "aprender")
    pub component: String,
    /// Python library it replaces (e.g., "sklearn")
    pub replaces: String,
    /// Total functions in Python library (tracked)
    pub total_functions: usize,
    /// Functions with sovereign mapping
    pub mapped_functions: usize,
    /// Coverage percentage
    pub coverage_rate: f64,
    /// Individual function mappings
    pub mappings: Vec<FunctionMapping>,
}

/// Individual function mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMapping {
    /// Python function name
    pub python_fn: String,
    /// Sovereign equivalent (if mapped)
    pub sovereign_fn: Option<String>,
    /// Migration status
    pub status: MigrationStatus,
    /// Usage rank (1 = most used)
    pub rank: usize,
}

/// Migration status for a function
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MigrationStatus {
    /// Fully mapped with identical API
    Mapped,
    /// Mapped with API differences
    MappedWithChanges,
    /// Partial mapping (some features missing)
    Partial,
    /// Not yet mapped
    Unmapped,
    /// Cannot be mapped (incompatible with sovereign)
    Incompatible,
}

/// Complete dashboard report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardReport {
    /// Timestamp of the report
    pub timestamp: String,
    /// Overall Path B progress (0-100%)
    pub path_b_progress: f64,
    /// Component-level coverage
    pub components: Vec<ComponentCoverage>,
    /// Compile rate from latest corpus run
    pub compile_rate: Option<f64>,
    /// Number of files in corpus
    pub corpus_files: Option<usize>,
}

/// Get the sklearn → aprender coverage data
pub fn get_sklearn_coverage() -> ComponentCoverage {
    let mappings = vec![
        FunctionMapping {
            python_fn: "train_test_split()".to_string(),
            sovereign_fn: Some("aprender::model_selection::train_test_split()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 1,
        },
        FunctionMapping {
            python_fn: "LinearRegression.fit()".to_string(),
            sovereign_fn: Some("LinearRegression::new().fit()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 2,
        },
        FunctionMapping {
            python_fn: "StandardScaler.fit_transform()".to_string(),
            sovereign_fn: Some("StandardScaler::new().fit_transform()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 3,
        },
        FunctionMapping {
            python_fn: "KMeans.fit_predict()".to_string(),
            sovereign_fn: Some("KMeans::new(k).fit_predict()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 4,
        },
        FunctionMapping {
            python_fn: "RandomForestClassifier.fit()".to_string(),
            sovereign_fn: Some("RandomForest::new().fit()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 5,
        },
        FunctionMapping {
            python_fn: "accuracy_score()".to_string(),
            sovereign_fn: Some("aprender::metrics::accuracy()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 6,
        },
        FunctionMapping {
            python_fn: "cross_val_score()".to_string(),
            sovereign_fn: Some("aprender::model_selection::cross_val_score()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 7,
        },
        FunctionMapping {
            python_fn: "confusion_matrix()".to_string(),
            sovereign_fn: Some("aprender::metrics::confusion_matrix()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 8,
        },
        FunctionMapping {
            python_fn: "PCA.fit_transform()".to_string(),
            sovereign_fn: Some("PCA::new(n_components).fit_transform()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 9,
        },
        FunctionMapping {
            python_fn: "LogisticRegression.fit()".to_string(),
            sovereign_fn: Some("LogisticRegression::new().fit()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 10,
        },
    ];

    let mapped = mappings
        .iter()
        .filter(|m| m.status == MigrationStatus::Mapped || m.status == MigrationStatus::MappedWithChanges)
        .count();

    ComponentCoverage {
        component: "aprender".to_string(),
        replaces: "sklearn".to_string(),
        total_functions: mappings.len(),
        mapped_functions: mapped,
        coverage_rate: (mapped as f64 / mappings.len() as f64) * 100.0,
        mappings,
    }
}

/// Get the numpy → trueno coverage data
pub fn get_numpy_coverage() -> ComponentCoverage {
    let mappings = vec![
        FunctionMapping {
            python_fn: "np.array()".to_string(),
            sovereign_fn: Some("Vector::from_slice()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 1,
        },
        FunctionMapping {
            python_fn: "np.zeros()".to_string(),
            sovereign_fn: Some("Vector::zeros()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 2,
        },
        FunctionMapping {
            python_fn: "np.ones()".to_string(),
            sovereign_fn: Some("Vector::ones()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 3,
        },
        FunctionMapping {
            python_fn: "np.dot()".to_string(),
            sovereign_fn: Some("Vector::dot()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 4,
        },
        FunctionMapping {
            python_fn: "np.sum()".to_string(),
            sovereign_fn: Some("Vector::sum()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 5,
        },
        FunctionMapping {
            python_fn: "np.mean()".to_string(),
            sovereign_fn: Some("Vector::mean()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 6,
        },
        FunctionMapping {
            python_fn: "np.std()".to_string(),
            sovereign_fn: Some("Vector::std()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 7,
        },
        FunctionMapping {
            python_fn: "np.reshape()".to_string(),
            sovereign_fn: Some("Matrix::reshape()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 8,
        },
        FunctionMapping {
            python_fn: "np.transpose()".to_string(),
            sovereign_fn: Some("Matrix::transpose()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 9,
        },
        FunctionMapping {
            python_fn: "np.matmul()".to_string(),
            sovereign_fn: Some("Matrix::matmul()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 10,
        },
    ];

    let mapped = mappings
        .iter()
        .filter(|m| m.status == MigrationStatus::Mapped || m.status == MigrationStatus::MappedWithChanges)
        .count();

    ComponentCoverage {
        component: "trueno".to_string(),
        replaces: "numpy".to_string(),
        total_functions: mappings.len(),
        mapped_functions: mapped,
        coverage_rate: (mapped as f64 / mappings.len() as f64) * 100.0,
        mappings,
    }
}

/// Get the pandas → trueno/realizar coverage data
pub fn get_pandas_coverage() -> ComponentCoverage {
    let mappings = vec![
        FunctionMapping {
            python_fn: "pd.DataFrame()".to_string(),
            sovereign_fn: Some("realizar::DataFrame::new()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 1,
        },
        FunctionMapping {
            python_fn: "df.read_csv()".to_string(),
            sovereign_fn: Some("realizar::io::read_csv()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 2,
        },
        FunctionMapping {
            python_fn: "df.to_csv()".to_string(),
            sovereign_fn: Some("realizar::io::write_csv()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 3,
        },
        FunctionMapping {
            python_fn: "df.head()".to_string(),
            sovereign_fn: Some("DataFrame::head()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 4,
        },
        FunctionMapping {
            python_fn: "df.tail()".to_string(),
            sovereign_fn: Some("DataFrame::tail()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 5,
        },
        FunctionMapping {
            python_fn: "df.describe()".to_string(),
            sovereign_fn: Some("DataFrame::describe()".to_string()),
            status: MigrationStatus::Partial,
            rank: 6,
        },
        FunctionMapping {
            python_fn: "df.groupby()".to_string(),
            sovereign_fn: Some("DataFrame::group_by()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 7,
        },
        FunctionMapping {
            python_fn: "df.merge()".to_string(),
            sovereign_fn: Some("DataFrame::join()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 8,
        },
        FunctionMapping {
            python_fn: "df.fillna()".to_string(),
            sovereign_fn: Some("DataFrame::fill_null()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 9,
        },
        FunctionMapping {
            python_fn: "df.dropna()".to_string(),
            sovereign_fn: Some("DataFrame::drop_nulls()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 10,
        },
    ];

    let mapped = mappings
        .iter()
        .filter(|m| {
            m.status == MigrationStatus::Mapped
                || m.status == MigrationStatus::MappedWithChanges
                || m.status == MigrationStatus::Partial
        })
        .count();

    ComponentCoverage {
        component: "realizar".to_string(),
        replaces: "pandas".to_string(),
        total_functions: mappings.len(),
        mapped_functions: mapped,
        coverage_rate: (mapped as f64 / mappings.len() as f64) * 100.0,
        mappings,
    }
}

/// Get scipy → trueno coverage data
pub fn get_scipy_coverage() -> ComponentCoverage {
    let mappings = vec![
        FunctionMapping {
            python_fn: "scipy.linalg.inv()".to_string(),
            sovereign_fn: Some("trueno::linalg::inverse()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 1,
        },
        FunctionMapping {
            python_fn: "scipy.linalg.det()".to_string(),
            sovereign_fn: Some("trueno::linalg::determinant()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 2,
        },
        FunctionMapping {
            python_fn: "scipy.linalg.eig()".to_string(),
            sovereign_fn: Some("trueno::linalg::eigen()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 3,
        },
        FunctionMapping {
            python_fn: "scipy.linalg.svd()".to_string(),
            sovereign_fn: Some("trueno::linalg::svd()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 4,
        },
        FunctionMapping {
            python_fn: "scipy.optimize.minimize()".to_string(),
            sovereign_fn: None,
            status: MigrationStatus::Unmapped,
            rank: 5,
        },
        FunctionMapping {
            python_fn: "scipy.integrate.quad()".to_string(),
            sovereign_fn: None,
            status: MigrationStatus::Unmapped,
            rank: 6,
        },
        FunctionMapping {
            python_fn: "scipy.stats.norm()".to_string(),
            sovereign_fn: Some("trueno::stats::Normal::new()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 7,
        },
        FunctionMapping {
            python_fn: "scipy.sparse.csr_matrix()".to_string(),
            sovereign_fn: Some("trueno::sparse::CsrMatrix::new()".to_string()),
            status: MigrationStatus::MappedWithChanges,
            rank: 8,
        },
        FunctionMapping {
            python_fn: "scipy.fft.fft()".to_string(),
            sovereign_fn: Some("trueno::fft::fft()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 9,
        },
        FunctionMapping {
            python_fn: "scipy.signal.convolve()".to_string(),
            sovereign_fn: Some("trueno::signal::convolve()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 10,
        },
    ];

    let mapped = mappings
        .iter()
        .filter(|m| {
            m.status == MigrationStatus::Mapped
                || m.status == MigrationStatus::MappedWithChanges
        })
        .count();

    ComponentCoverage {
        component: "trueno".to_string(),
        replaces: "scipy".to_string(),
        total_functions: mappings.len(),
        mapped_functions: mapped,
        coverage_rate: (mapped as f64 / mappings.len() as f64) * 100.0,
        mappings,
    }
}

/// Generate the full dashboard report
pub fn generate_dashboard() -> DashboardReport {
    let sklearn = get_sklearn_coverage();
    let numpy = get_numpy_coverage();
    let pandas = get_pandas_coverage();
    let scipy = get_scipy_coverage();

    // Calculate overall Path B progress
    let total_mapped: usize = [&sklearn, &numpy, &pandas, &scipy]
        .iter()
        .map(|c| c.mapped_functions)
        .sum();
    let total_functions: usize = [&sklearn, &numpy, &pandas, &scipy]
        .iter()
        .map(|c| c.total_functions)
        .sum();

    let path_b_progress = if total_functions > 0 {
        (total_mapped as f64 / total_functions as f64) * 100.0
    } else {
        0.0
    };

    // Try to load compile rate from metrics file
    let (compile_rate, corpus_files) = load_corpus_metrics();

    DashboardReport {
        timestamp: chrono::Utc::now().to_rfc3339(),
        path_b_progress,
        components: vec![sklearn, numpy, pandas, scipy],
        compile_rate,
        corpus_files,
    }
}

/// Load corpus metrics from oracle_roi_metrics.json if available
fn load_corpus_metrics() -> (Option<f64>, Option<usize>) {
    let metrics_path = std::path::Path::new("docs/oracle_roi_metrics.json");
    if !metrics_path.exists() {
        return (None, None);
    }

    if let Ok(content) = std::fs::read_to_string(metrics_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let compile_rate = json["baseline"]["transpile_rate"]
                .as_f64()
                .map(|r| r * 100.0);
            let corpus_files = json["baseline"]["files_processed"].as_u64().map(|f| f as usize);
            return (compile_rate, corpus_files);
        }
    }

    (None, None)
}

/// Display dashboard in text format
pub fn display_text(report: &DashboardReport) {
    println!();
    println!("{}", "═══════════════════════════════════════════════════════════════".bright_blue());
    println!("{}", "           DEPYLER SOVEREIGN STACK COVERAGE DASHBOARD          ".bright_blue().bold());
    println!("{}", "═══════════════════════════════════════════════════════════════".bright_blue());
    println!();

    // Path B Progress
    let progress_bar = create_progress_bar(report.path_b_progress, 40);
    println!(
        "{} {} [{:.1}%]",
        "Path B Progress:".bold(),
        progress_bar,
        report.path_b_progress
    );
    println!();

    // Compile rate if available
    if let Some(rate) = report.compile_rate {
        let rate_color = if rate >= 80.0 {
            format!("{:.1}%", rate).green()
        } else if rate >= 50.0 {
            format!("{:.1}%", rate).yellow()
        } else {
            format!("{:.1}%", rate).red()
        };
        print!("{} {}", "Compile Rate:".bold(), rate_color);
        if let Some(files) = report.corpus_files {
            print!(" ({} files)", files);
        }
        println!();
        println!();
    }

    // Component coverage table
    println!("┌─────────────┬─────────────┬─────────┬─────────────────────┐");
    println!(
        "│ {} │ {} │ {} │ {} │",
        "Component".bold(),
        "Replaces".bold(),
        "Rate".bold(),
        "Status".bold()
    );
    println!("├─────────────┼─────────────┼─────────┼─────────────────────┤");

    for component in &report.components {
        let rate_str = format!("{:.0}%", component.coverage_rate);
        let rate_colored = if component.coverage_rate >= 80.0 {
            rate_str.green()
        } else if component.coverage_rate >= 50.0 {
            rate_str.yellow()
        } else {
            rate_str.red()
        };

        let status = if component.coverage_rate >= 100.0 {
            "Complete".green()
        } else if component.coverage_rate >= 80.0 {
            "Good".green()
        } else if component.coverage_rate >= 50.0 {
            "Partial".yellow()
        } else {
            "Needs Work".red()
        };

        println!(
            "│ {:11} │ {:11} │ {:7} │ {:19} │",
            component.component,
            component.replaces,
            rate_colored,
            status
        );
    }
    println!("└─────────────┴─────────────┴─────────┴─────────────────────┘");
    println!();

    // Detailed mappings
    println!("─────────────────────────────────────────────────────────────────");
    println!("{}", "                    DETAILED FUNCTION MAPPINGS                   ".bold());
    println!("─────────────────────────────────────────────────────────────────");
    println!();

    for component in &report.components {
        println!(
            "{} → {} ({}/{} mapped)",
            component.replaces.yellow(),
            component.component.green(),
            component.mapped_functions,
            component.total_functions
        );
        println!();

        for mapping in &component.mappings {
            let status_icon = match mapping.status {
                MigrationStatus::Mapped => "✓".green(),
                MigrationStatus::MappedWithChanges => "~".yellow(),
                MigrationStatus::Partial => "◐".yellow(),
                MigrationStatus::Unmapped => "✗".red(),
                MigrationStatus::Incompatible => "⊘".red(),
            };

            let sovereign = mapping
                .sovereign_fn
                .as_deref()
                .unwrap_or("(unmapped)");

            println!(
                "  {} #{:<2} {} → {}",
                status_icon,
                mapping.rank,
                mapping.python_fn,
                sovereign
            );
        }
        println!();
    }

    // Legend
    println!("{}", "Legend:".bold());
    println!("  {} Mapped (identical API)", "✓".green());
    println!("  {} Mapped (API changes)", "~".yellow());
    println!("  {} Partial mapping", "◐".yellow());
    println!("  {} Unmapped", "✗".red());
    println!("  {} Incompatible", "⊘".red());
    println!();

    println!("{}", format!("Generated: {}", report.timestamp).dimmed());
}

/// Create a text progress bar
fn create_progress_bar(percent: f64, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    let filled_str: String = "█".repeat(filled);
    let empty_str: String = "░".repeat(empty);

    format!("{}{}", filled_str.green(), empty_str.dimmed())
}

/// Main dashboard command entry point
pub fn dashboard_command(format: &str, component: Option<&str>) -> Result<()> {
    let report = generate_dashboard();

    match format {
        "json" => {
            let json = if let Some(comp) = component {
                // Filter to specific component
                let filtered: Vec<_> = report
                    .components
                    .iter()
                    .filter(|c| c.component == comp || c.replaces == comp)
                    .cloned()
                    .collect();
                serde_json::to_string_pretty(&filtered)?
            } else {
                serde_json::to_string_pretty(&report)?
            };
            println!("{}", json);
        }
        _ => {
            display_text(&report);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sklearn_coverage_complete() {
        let coverage = get_sklearn_coverage();
        assert_eq!(coverage.component, "aprender");
        assert_eq!(coverage.replaces, "sklearn");
        assert_eq!(coverage.total_functions, 10);
        assert!(coverage.coverage_rate >= 80.0);
    }

    #[test]
    fn test_numpy_coverage_complete() {
        let coverage = get_numpy_coverage();
        assert_eq!(coverage.component, "trueno");
        assert_eq!(coverage.replaces, "numpy");
        assert_eq!(coverage.total_functions, 10);
        assert!(coverage.coverage_rate >= 80.0);
    }

    #[test]
    fn test_pandas_coverage_complete() {
        let coverage = get_pandas_coverage();
        assert_eq!(coverage.component, "realizar");
        assert_eq!(coverage.replaces, "pandas");
        assert_eq!(coverage.total_functions, 10);
    }

    #[test]
    fn test_scipy_coverage_complete() {
        let coverage = get_scipy_coverage();
        assert_eq!(coverage.component, "trueno");
        assert_eq!(coverage.replaces, "scipy");
        assert_eq!(coverage.total_functions, 10);
    }

    #[test]
    fn test_generate_dashboard() {
        let report = generate_dashboard();
        assert_eq!(report.components.len(), 4);
        assert!(report.path_b_progress >= 0.0);
        assert!(report.path_b_progress <= 100.0);
    }

    #[test]
    fn test_progress_bar_empty() {
        let bar = create_progress_bar(0.0, 10);
        assert!(!bar.is_empty());
    }

    #[test]
    fn test_progress_bar_full() {
        let bar = create_progress_bar(100.0, 10);
        assert!(!bar.is_empty());
    }

    #[test]
    fn test_progress_bar_half() {
        let bar = create_progress_bar(50.0, 10);
        assert!(!bar.is_empty());
    }

    #[test]
    fn test_migration_status_variants() {
        assert_eq!(MigrationStatus::Mapped, MigrationStatus::Mapped);
        assert_ne!(MigrationStatus::Mapped, MigrationStatus::Unmapped);
    }

    #[test]
    fn test_function_mapping_structure() {
        let mapping = FunctionMapping {
            python_fn: "test()".to_string(),
            sovereign_fn: Some("test()".to_string()),
            status: MigrationStatus::Mapped,
            rank: 1,
        };
        assert_eq!(mapping.rank, 1);
        assert!(mapping.sovereign_fn.is_some());
    }

    #[test]
    fn test_component_coverage_calculation() {
        let coverage = get_sklearn_coverage();
        let expected_rate = (coverage.mapped_functions as f64 / coverage.total_functions as f64) * 100.0;
        assert!((coverage.coverage_rate - expected_rate).abs() < 0.01);
    }
}
