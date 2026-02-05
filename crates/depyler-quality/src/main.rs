use clap::{Arg, Command};
use depyler_quality::{QualityAnalyzer, QualityStatus};
use std::fs;
use std::process;

fn main() {
    env_logger::init();

    let matches = Command::new("depyler-quality")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Quality analysis and PMAT scoring for Depyler")
        .subcommand(
            Command::new("analyze")
                .about("Analyze code quality")
                .arg(
                    Arg::new("path")
                        .help("Path to analyze")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Output format")
                        .value_parser(["json", "text", "sarif"])
                        .default_value("text"),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Output file")
                        .value_name("FILE"),
                ),
        )
        .subcommand(
            Command::new("enforce")
                .about("Enforce quality gates")
                .arg(
                    Arg::new("path")
                        .help("Path to analyze")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("max-tdg")
                        .long("max-tdg")
                        .help("Maximum TDG score")
                        .value_name("SCORE")
                        .value_parser(clap::value_parser!(f64))
                        .default_value("2.0"),
                )
                .arg(
                    Arg::new("min-coverage")
                        .long("min-coverage")
                        .help("Minimum test coverage")
                        .value_name("PERCENT")
                        .value_parser(clap::value_parser!(f64))
                        .default_value("85.0"),
                )
                .arg(
                    Arg::new("max-complexity")
                        .long("max-complexity")
                        .help("Maximum complexity")
                        .value_name("COUNT")
                        .value_parser(clap::value_parser!(u32))
                        .default_value("15"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("analyze", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .expect("required argument");
            let format = sub_matches
                .get_one::<String>("format")
                .expect("required argument");
            let output = sub_matches.get_one::<String>("output");

            if let Err(e) = run_analyze(path, format, output) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Some(("enforce", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .expect("required argument");
            let max_tdg = *sub_matches
                .get_one::<f64>("max-tdg")
                .expect("required argument");
            let min_coverage = *sub_matches
                .get_one::<f64>("min-coverage")
                .expect("required argument");
            let max_complexity = *sub_matches
                .get_one::<u32>("max-complexity")
                .expect("required argument");

            if let Err(e) = run_enforce(path, max_tdg, min_coverage, max_complexity) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        _ => {
            eprintln!("No subcommand provided. Use --help for usage information.");
            process::exit(1);
        }
    }
}

fn run_analyze(
    path: &str,
    format: &str,
    output: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Analyzing path: {path}");

    // For now, analyze with empty functions (would parse actual code in real implementation)
    let analyzer = QualityAnalyzer::new();
    let functions = vec![]; // Would load actual HIR functions

    let report = analyzer.analyze_quality(&functions)?;

    let output_content = match format {
        "json" => serde_json::to_string_pretty(&report)?,
        "sarif" => generate_sarif_report(&report)?,
        "text" => {
            analyzer.print_quality_report(&report);
            return Ok(());
        }
        _ => return Err("Invalid format".into()),
    };

    if let Some(output_file) = output {
        fs::write(output_file, output_content)?;
        println!("Report written to {output_file}");
    } else {
        println!("{output_content}");
    }

    Ok(())
}

fn run_enforce(
    path: &str,
    max_tdg: f64,
    min_coverage: f64,
    max_complexity: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Enforcing quality gates for path: {path}");
    log::info!(
        "Max TDG: {max_tdg}, Min Coverage: {min_coverage}%, Max Complexity: {max_complexity}"
    );

    let analyzer = QualityAnalyzer::new();
    let functions = vec![]; // Would load actual HIR functions

    let report = analyzer.analyze_quality(&functions)?;

    // Check if quality gates pass
    let tdg_check = report.pmat_metrics.tdg <= max_tdg;
    let coverage_check = report.coverage_metrics.line_coverage >= (min_coverage / 100.0);
    let complexity_check = report.complexity_metrics.cyclomatic_complexity <= max_complexity;

    println!("Quality Gate Enforcement Results:");
    println!("=================================");
    println!(
        "TDG Score: {:.2} <= {} {}",
        report.pmat_metrics.tdg,
        max_tdg,
        if tdg_check { "✅" } else { "❌" }
    );
    println!(
        "Coverage: {:.1}% >= {}% {}",
        report.coverage_metrics.line_coverage * 100.0,
        min_coverage,
        if coverage_check { "✅" } else { "❌" }
    );
    println!(
        "Complexity: {} <= {} {}",
        report.complexity_metrics.cyclomatic_complexity,
        max_complexity,
        if complexity_check { "✅" } else { "❌" }
    );

    let all_passed = tdg_check && coverage_check && complexity_check;

    match report.overall_status {
        QualityStatus::Passed if all_passed => {
            println!("\n✅ All quality gates PASSED");
            Ok(())
        }
        _ => {
            println!("\n❌ Quality gates FAILED");
            if !tdg_check {
                println!(
                    "  - TDG score too high: {:.2} > {}",
                    report.pmat_metrics.tdg, max_tdg
                );
            }
            if !coverage_check {
                println!(
                    "  - Coverage too low: {:.1}% < {}%",
                    report.coverage_metrics.line_coverage * 100.0,
                    min_coverage
                );
            }
            if !complexity_check {
                println!(
                    "  - Complexity too high: {} > {}",
                    report.complexity_metrics.cyclomatic_complexity, max_complexity
                );
            }

            analyzer.print_quality_report(&report);
            process::exit(1);
        }
    }
}

fn generate_sarif_report(
    report: &depyler_quality::QualityReport,
) -> Result<String, Box<dyn std::error::Error>> {
    // Generate SARIF 2.1.0 compliant report
    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "depyler-quality",
                    "version": env!("CARGO_PKG_VERSION"),
                    "informationUri": "https://github.com/depyler/depyler",
                    "rules": [
                        {
                            "id": "PMAT_TDG",
                            "name": "PMAT TDG Score",
                            "shortDescription": {
                                "text": "PMAT Total Depyler Grade score validation"
                            }
                        },
                        {
                            "id": "COMPLEXITY",
                            "name": "Code Complexity",
                            "shortDescription": {
                                "text": "Cyclomatic and cognitive complexity validation"
                            }
                        }
                    ]
                }
            },
            "results": report.gates_failed.iter().map(|gate| {
                serde_json::json!({
                    "ruleId": match gate.gate_name.as_str() {
                        "PMAT TDG Range" => "PMAT_TDG",
                        "Complexity Limits" => "COMPLEXITY",
                        _ => "QUALITY_GATE"
                    },
                    "message": {
                        "text": format!("{}: {} (actual: {})", gate.gate_name,
                                      format!("{:?}", gate.requirement), gate.actual_value)
                    },
                    "level": match gate.severity {
                        depyler_quality::Severity::Error => "error",
                        depyler_quality::Severity::Warning => "warning",
                        depyler_quality::Severity::Info => "note"
                    }
                })
            }).collect::<Vec<_>>()
        }]
    });

    Ok(serde_json::to_string_pretty(&sarif)?)
}
