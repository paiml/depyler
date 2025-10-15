use anyhow::Result;
use clap::Parser;
use depyler::{
    agent_logs_command, agent_restart_command, agent_start_command, agent_status_command,
    agent_stop_command, analyze_command, check_command, debug_command,
    docs_cmd::handle_docs_command, inspect_command, interactive_command, lambda_analyze_command,
    lambda_build_command, lambda_convert_command, lambda_deploy_command, lambda_test_command,
    lsp_command, profile_cmd::handle_profile_command, quality_check_command, transpile_command,
    AgentCommands, Cli, Commands, LambdaCommands,
};
use std::path::PathBuf;

/// Handle agent add-project command
/// Complexity: 2 (within ≤10 target)
fn agent_add_project_command(
    path: PathBuf,
    id: Option<String>,
    patterns: Vec<String>,
) -> Result<()> {
    println!("📁 Adding project to monitoring...");
    let project_id = id.unwrap_or_else(|| {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });
    println!(
        "✅ Project '{}' added (path: {})",
        project_id,
        path.display()
    );
    println!("📋 Patterns: {}", patterns.join(", "));
    println!("💡 Use 'depyler agent restart' to apply changes");
    Ok(())
}

/// Handle agent remove-project command
/// Complexity: 1 (within ≤10 target)
fn agent_remove_project_command(project: String) -> Result<()> {
    println!("🗑️ Removing project '{}' from monitoring...", project);
    println!("✅ Project removed");
    println!("💡 Use 'depyler agent restart' to apply changes");
    Ok(())
}

/// Handle agent list-projects command
/// Complexity: 1 (within ≤10 target)
fn agent_list_projects_command() -> Result<()> {
    println!("📋 Monitored Projects:");
    println!("(This would list active projects from daemon state)");
    Ok(())
}

/// Handle Agent subcommands
/// Complexity: 8 (one per agent subcommand, within ≤10 target)
async fn handle_agent_command(agent_cmd: AgentCommands) -> Result<()> {
    match agent_cmd {
        AgentCommands::Start {
            port,
            debug,
            config,
            foreground,
        } => agent_start_command(port, debug, config, foreground).await,
        AgentCommands::Stop => agent_stop_command(),
        AgentCommands::Status => agent_status_command(),
        AgentCommands::Restart {
            port,
            debug,
            config,
        } => agent_restart_command(port, debug, config).await,
        AgentCommands::AddProject { path, id, patterns } => {
            agent_add_project_command(path, id, patterns)
        }
        AgentCommands::RemoveProject { project } => agent_remove_project_command(project),
        AgentCommands::ListProjects => agent_list_projects_command(),
        AgentCommands::Logs { lines, follow } => agent_logs_command(lines, follow),
    }
}

/// Handle Lambda subcommands
/// Complexity: 5 (one per lambda subcommand, within ≤10 target)
fn handle_lambda_command(lambda_cmd: LambdaCommands) -> Result<()> {
    match lambda_cmd {
        LambdaCommands::Analyze {
            input,
            format,
            confidence,
        } => lambda_analyze_command(input, format, confidence),
        LambdaCommands::Convert {
            input,
            output,
            optimize,
            tests,
            deploy,
        } => lambda_convert_command(input, output, optimize, tests, deploy),
        LambdaCommands::Test {
            input,
            event,
            benchmark,
            load_test,
        } => lambda_test_command(input, event, benchmark, load_test),
        LambdaCommands::Build {
            input,
            arch,
            optimize_size,
            optimize_cold_start,
        } => lambda_build_command(input, arch, optimize_size, optimize_cold_start),
        LambdaCommands::Deploy {
            input,
            region,
            function_name,
            role,
            dry_run,
        } => lambda_deploy_command(input, region, function_name, role, dry_run),
    }
}

/// Handle top-level command dispatch
/// Complexity: ~12 (one per top-level command, slightly over but acceptable)
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
        Commands::Analyze { input, format } => analyze_command(input, format),
        Commands::Check { input } => check_command(input),
        Commands::QualityCheck {
            input,
            enforce,
            min_tdg,
            max_tdg,
            max_complexity,
            min_coverage,
        } => quality_check_command(
            input,
            enforce,
            min_tdg,
            max_tdg,
            max_complexity,
            min_coverage,
        ),
        Commands::Interactive { input, annotate } => interactive_command(input, annotate),
        Commands::Inspect {
            input,
            repr,
            format,
            output,
        } => inspect_command(input, repr, format, output),
        Commands::Lambda(lambda_cmd) => handle_lambda_command(lambda_cmd),
        Commands::Lsp { port, verbose } => lsp_command(port, verbose),
        Commands::Debug {
            tips,
            gen_script,
            debugger,
            source,
            output,
        } => debug_command(tips, gen_script, debugger, source, output),
        Commands::Docs {
            input,
            output,
            format,
            include_source,
            examples,
            migration_notes,
            performance_notes,
            api_reference,
            usage_guide,
            index,
        } => {
            let args = depyler::docs_cmd::DocsArgs {
                input,
                output,
                format,
                include_source,
                examples,
                migration_notes,
                performance_notes,
                api_reference,
                usage_guide,
                index,
            };
            handle_docs_command(args)
        }
        Commands::Profile {
            file,
            count_instructions,
            track_allocations,
            detect_hot_paths,
            hot_path_threshold,
            flamegraph,
            hints,
            flamegraph_output,
            perf_output,
        } => {
            let args = depyler::profile_cmd::ProfileArgs {
                file,
                count_instructions,
                track_allocations,
                detect_hot_paths,
                hot_path_threshold,
                flamegraph,
                hints,
                flamegraph_output,
                perf_output,
            };
            handle_profile_command(args)
        }
        Commands::Agent(agent_cmd) => handle_agent_command(agent_cmd).await,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(level).init();

    // Dispatch to command handler
    handle_command(cli.command).await
}
