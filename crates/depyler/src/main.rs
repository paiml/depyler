use anyhow::Result;
use clap::Parser;
use depyler::{
    analyze_command, check_command, inspect_command, interactive_command, quality_check_command,
    transpile_command, Cli, Commands,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(level).init();

    match cli.command {
        Commands::Transpile {
            input,
            output,
            verify,
            gen_tests,
        } => {
            transpile_command(input, output, verify, gen_tests)?;
        }
        Commands::Analyze { input, format } => {
            analyze_command(input, format)?;
        }
        Commands::Check { input } => {
            check_command(input)?;
        }
        Commands::QualityCheck {
            input,
            enforce,
            min_tdg,
            max_tdg,
            max_complexity,
            min_coverage,
        } => {
            quality_check_command(
                input,
                enforce,
                min_tdg,
                max_tdg,
                max_complexity,
                min_coverage,
            )?;
        }
        Commands::Interactive { input, annotate } => {
            interactive_command(input, annotate)?;
        }
        Commands::Inspect {
            input,
            repr,
            format,
            output,
        } => {
            inspect_command(input, repr, format, output)?;
        }
    }

    Ok(())
}
