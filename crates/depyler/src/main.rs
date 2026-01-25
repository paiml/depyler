//! Depyler CLI - Minimal Python-to-Rust transpiler
//!
//! Focused on transpilation and single-shot compilation.

use anyhow::Result;
use clap::Parser;
use depyler::{
    analyze_command, check_command, compile_command, converge, graph_cmd, repair_command,
    report_cmd::{handle_report_command, ReportArgs},
    transpile_command,
    utol_cmd::handle_utol_command,
    CacheCommands, Cli, Commands, GraphCommands,
};
use std::path::PathBuf;

/// Handle converge command
#[allow(clippy::too_many_arguments)]
async fn handle_converge_command(
    input_dir: PathBuf,
    target_rate: f64,
    max_iterations: usize,
    auto_fix: bool,
    dry_run: bool,
    fix_confidence: f64,
    checkpoint_dir: Option<PathBuf>,
    parallel_jobs: usize,
    display: String,
    oracle: bool,
    explain: bool,
    use_cache: bool,
    patch_transpiler: bool,
    apr_file: Option<PathBuf>,
) -> Result<()> {
    let display_mode = converge::DisplayMode::parse(&display);

    let config = converge::ConvergenceConfig {
        input_dir,
        target_rate,
        max_iterations,
        auto_fix,
        dry_run,
        verbose: !matches!(display_mode, converge::DisplayMode::Silent),
        fix_confidence_threshold: fix_confidence,
        checkpoint_dir,
        parallel_jobs,
        display_mode,
        oracle,
        explain,
        use_cache,
        patch_transpiler,
        apr_file,
    };

    config.validate()?;

    let state = converge::run_convergence_loop(config).await?;

    if state.compilation_rate >= state.config.target_rate {
        if !matches!(
            state.config.display_mode,
            converge::DisplayMode::Silent | converge::DisplayMode::Json
        ) {
            println!("Target rate reached: {:.1}%", state.compilation_rate);
        }
        Ok(())
    } else {
        anyhow::bail!(
            "Target rate not reached: {:.1}% < {:.1}%",
            state.compilation_rate,
            state.config.target_rate
        )
    }
}

/// Handle Cache subcommands
fn handle_cache_command(cache_cmd: CacheCommands) -> Result<()> {
    use depyler::converge::{CacheConfig, SqliteCache};

    let get_cache_dir = || -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".depyler")
            .join("cache")
    };

    match cache_cmd {
        CacheCommands::Stats { format } => {
            let cache_path = get_cache_dir();
            let config = CacheConfig {
                cache_dir: cache_path.clone(),
                ..Default::default()
            };

            match SqliteCache::open(config) {
                Ok(cache) => {
                    let stats = cache.stats()?;
                    if format == "json" {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "entries": stats.total_entries,
                                "total_size_bytes": stats.total_size_bytes,
                                "hits": stats.hit_count,
                                "misses": stats.miss_count,
                                "hit_rate": stats.hit_rate(),
                                "cache_dir": cache_path.display().to_string()
                            }))?
                        );
                    } else {
                        println!("Cache Statistics");
                        println!("================");
                        println!("Cache directory: {}", cache_path.display());
                        println!("Entries: {}", stats.total_entries);
                        println!(
                            "Total size: {:.2} MB",
                            stats.total_size_bytes as f64 / (1024.0 * 1024.0)
                        );
                        println!("Cache hits: {}", stats.hit_count);
                        println!("Cache misses: {}", stats.miss_count);
                        println!("Hit rate: {:.1}%", stats.hit_rate() * 100.0);
                    }
                    Ok(())
                }
                Err(e) => {
                    if format == "json" {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "error": format!("Cache not found: {}", e),
                                "cache_dir": cache_path.display().to_string()
                            }))?
                        );
                    } else {
                        println!("No cache found at {}", cache_path.display());
                        println!("Run `depyler transpile` to populate the cache.");
                    }
                    Ok(())
                }
            }
        }
        CacheCommands::Gc {
            max_age_days,
            dry_run,
        } => {
            let cache_path = get_cache_dir();
            let config = CacheConfig {
                cache_dir: cache_path.clone(),
                max_age_secs: (max_age_days as u64) * 24 * 3600,
                ..Default::default()
            };

            if dry_run {
                println!("Dry run - no files will be deleted");
            }

            match SqliteCache::open(config) {
                Ok(cache) => {
                    let result = cache.gc()?;
                    println!("Garbage Collection Results");
                    println!("==========================");
                    println!("Entries removed: {}", result.evicted);
                    println!(
                        "Space reclaimed: {:.2} MB",
                        result.freed_bytes as f64 / (1024.0 * 1024.0)
                    );
                    Ok(())
                }
                Err(e) => {
                    anyhow::bail!("Failed to open cache: {}", e)
                }
            }
        }
        CacheCommands::Clear { force } => {
            let cache_path = get_cache_dir();

            if !force {
                println!("This will delete all cached transpilation results.");
                println!("Path: {}", cache_path.display());
                print!("Continue? [y/N] ");
                use std::io::{self, Write};
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled");
                    return Ok(());
                }
            }

            if cache_path.exists() {
                std::fs::remove_dir_all(&cache_path)?;
                println!("Cache cleared: {}", cache_path.display());
            } else {
                println!("Cache directory does not exist: {}", cache_path.display());
            }
            Ok(())
        }
        CacheCommands::Warm {
            input_dir,
            jobs: _jobs,
        } => {
            use depyler::converge::cache_warmer::CacheWarmer;

            let cache_path = get_cache_dir();
            println!("Warming cache from {}", input_dir.display());
            println!("Cache directory: {}", cache_path.display());

            let config = CacheConfig {
                cache_dir: cache_path,
                ..Default::default()
            };

            let warmer = CacheWarmer::new(config);
            let stats = warmer.warm_directory(&input_dir)?;

            println!("==============================");
            println!("Compiled & cached: {}", stats.compiled);
            println!("Already cached: {}", stats.cached);
            println!("Transpile failed: {}", stats.transpile_failed);
            println!("Compile failed: {}", stats.compile_failed);
            println!("Single-shot compile rate: {:.1}%", stats.compile_rate());
            Ok(())
        }
    }
}

/// Handle top-level command dispatch
async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Transpile {
            input,
            output,
            verify,
            gen_tests,
            debug,
            source_map,
        } => transpile_command(input, output, verify, gen_tests, debug, source_map),
        Commands::Compile {
            input,
            output,
            profile,
        } => compile_command(input, output, profile),
        Commands::Analyze { input, format } => analyze_command(input, format),
        Commands::Check { input } => check_command(input),
        Commands::Cache(cache_cmd) => handle_cache_command(cache_cmd),
        Commands::Converge {
            input_dir,
            target_rate,
            max_iterations,
            auto_fix,
            dry_run,
            fix_confidence,
            checkpoint,
            jobs,
            display,
            oracle,
            explain,
            cache,
            patch_transpiler,
            apr_file,
        } => {
            handle_converge_command(
                input_dir,
                target_rate,
                max_iterations,
                auto_fix,
                dry_run,
                fix_confidence,
                checkpoint,
                jobs,
                display,
                oracle,
                explain,
                cache,
                patch_transpiler,
                apr_file,
            )
            .await
        }
        Commands::Report {
            input_dir,
            format,
            output,
            filter_error,
            filter_file,
            failures_only,
            verbose,
        } => {
            let args = ReportArgs {
                corpus: Some(input_dir),
                format,
                output,
                skip_clean: false,
                target_rate: 80.0,
                filter: filter_error,
                tag: filter_file,
                limit: None,
                sample: None,
                bisect: false,
                fail_fast: failures_only || verbose,
            };
            handle_report_command(args)
        }
        Commands::Utol {
            corpus,
            target_rate,
            max_iterations,
            patience,
            display,
            status,
        } => handle_utol_command(
            corpus,
            target_rate,
            max_iterations,
            patience,
            display,
            None, // output
            None, // config
            status,
            false, // watch
            500,   // watch_debounce
        ),
        Commands::Repair {
            input,
            output,
            max_iterations,
            verbose,
        } => repair_command(input, output, max_iterations, verbose),
        Commands::Graph(graph_cmd) => match graph_cmd {
            GraphCommands::Analyze {
                corpus,
                top,
                output,
            } => graph_cmd::analyze_corpus(&corpus, top, output.as_deref()),
            GraphCommands::Vectorize {
                corpus,
                output,
                format,
            } => graph_cmd::vectorize_corpus(&corpus, &output, &format),
        },
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(level).init();

    // DEPYLER-1148: Initialize CITL Flight Recorder for decision tracing
    // When enabled, captures transpiler decisions to /tmp/depyler_decisions.msgpack
    if let Err(e) = depyler_core::decision_trace::init_decision_tracing() {
        tracing::warn!("Failed to initialize decision tracing: {}", e);
    }

    handle_command(cli.command).await
}
