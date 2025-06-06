use anyhow::Result;
use clap::Parser;
use depyler::{
    analyze_command, check_command, inspect_command, interactive_command, lambda_analyze_command,
    lambda_build_command, lambda_convert_command, lambda_deploy_command, lambda_test_command,
    quality_check_command, transpile_command, Cli, Commands, LambdaCommands,
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
        Commands::Lambda(lambda_cmd) => match lambda_cmd {
            LambdaCommands::Analyze {
                input,
                format,
                confidence,
            } => {
                lambda_analyze_command(input, format, confidence)?;
            }
            LambdaCommands::Convert {
                input,
                output,
                optimize,
                tests,
                deploy,
            } => {
                lambda_convert_command(input, output, optimize, tests, deploy)?;
            }
            LambdaCommands::Test {
                input,
                event,
                benchmark,
                load_test,
            } => {
                lambda_test_command(input, event, benchmark, load_test)?;
            }
            LambdaCommands::Build {
                input,
                arch,
                optimize_size,
                optimize_cold_start,
            } => {
                lambda_build_command(input, arch, optimize_size, optimize_cold_start)?;
            }
            LambdaCommands::Deploy {
                input,
                region,
                function_name,
                role,
                dry_run,
            } => {
                lambda_deploy_command(input, region, function_name, role, dry_run)?;
            }
        },
    }

    Ok(())
}
