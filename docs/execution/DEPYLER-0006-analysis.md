# DEPYLER-0006: Refactor main Function Analysis

**Ticket**: DEPYLER-0006
**Priority**: P0 - CRITICAL
**Sprint**: Sprint 2 - Critical Complexity Reduction
**Estimated**: 20-30 hours
**Status**: In Progress
**Date**: 2025-10-02

---

## 🎯 **Objective**

Refactor `main` function from cyclomatic complexity 25 to ≤10 using Command Pattern while maintaining all existing functionality and CLI behavior.

---

## 📊 **Current State**

**Location**: `crates/depyler/src/main.rs:13-219`
**Lines**: 207
**Cyclomatic Complexity**: 25
**Cognitive Complexity**: 56
**Current Tests**: CLI integration tests exist
**Dependencies**: Many command handler functions already extracted

---

## 🔍 **Function Structure Analysis**

The function has **27 command variants** (14 top-level + 13 nested):

### **Top-Level Commands** (14 variants)
1. `Commands::Transpile` - Lines 21-30
2. `Commands::Analyze` - Lines 31-33
3. `Commands::Check` - Lines 34-36
4. `Commands::QualityCheck` - Lines 37-53
5. `Commands::Interactive` - Lines 54-56
6. `Commands::Inspect` - Lines 57-64
7. `Commands::Lambda` (with 5 nested subcommands) - Lines 65-107
8. `Commands::Lsp` - Lines 108-113
9. `Commands::Debug` - Lines 114-122
10. `Commands::Docs` - Lines 123-148
11. `Commands::Profile` - Lines 149-172
12. `Commands::Agent` (with 8 nested subcommands) - Lines 173-215

### **Lambda Subcommands** (5 variants, Lines 65-107)
1. `LambdaCommands::Analyze` - Lines 66-72
2. `LambdaCommands::Convert` - Lines 73-81
3. `LambdaCommands::Test` - Lines 82-89
4. `LambdaCommands::Build` - Lines 90-97
5. `LambdaCommands::Deploy` - Lines 98-106

### **Agent Subcommands** (8 variants, Lines 173-215)
1. `AgentCommands::Start` - Lines 174-181 (async)
2. `AgentCommands::Stop` - Lines 182-184
3. `AgentCommands::Status` - Lines 185-187
4. `AgentCommands::Restart` - Lines 188-190 (async)
5. `AgentCommands::AddProject` - Lines 191-202 (inline implementation)
6. `AgentCommands::RemoveProject` - Lines 203-207 (inline implementation)
7. `AgentCommands::ListProjects` - Lines 208-211 (inline implementation)
8. `AgentCommands::Logs` - Lines 212-214

### **Complexity Sources**
- **27 match arms**: Each adds 1 to cyclomatic complexity
- **Nested matches**: Lambda and Agent subcommands add nesting
- **Inline implementations**: AddProject, RemoveProject, ListProjects have inline logic
- **Argument extraction**: Many commands require extracting multiple fields

---

## 🎯 **Refactoring Strategy**

### **Apply Command Pattern with Dispatcher Functions**

Create three dispatcher functions for command categories:

```rust
/// Handle top-level command dispatch
/// Complexity: ~12 (one per top-level command)
async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Transpile { input, output, verify, gen_tests, debug, source_map } =>
            transpile_command(input, output, verify, gen_tests, debug, source_map),
        Commands::Analyze { input, format } =>
            analyze_command(input, format),
        Commands::Check { input } =>
            check_command(input),
        Commands::QualityCheck { input, enforce, min_tdg, max_tdg, max_complexity, min_coverage } =>
            quality_check_command(input, enforce, min_tdg, max_tdg, max_complexity, min_coverage),
        Commands::Interactive { input, annotate } =>
            interactive_command(input, annotate),
        Commands::Inspect { input, repr, format, output } =>
            inspect_command(input, repr, format, output),
        Commands::Lambda(lambda_cmd) =>
            handle_lambda_command(lambda_cmd),
        Commands::Lsp { port, verbose } =>
            lsp_command(port, verbose),
        Commands::Debug { tips, gen_script, debugger, source, output } =>
            debug_command(tips, gen_script, debugger, source, output),
        Commands::Docs { input, output, format, include_source, examples, migration_notes,
                        performance_notes, api_reference, usage_guide, index } => {
            let args = DocsArgs {
                input, output, format, include_source, examples, migration_notes,
                performance_notes, api_reference, usage_guide, index,
            };
            handle_docs_command(args)
        }
        Commands::Profile { file, count_instructions, track_allocations, detect_hot_paths,
                           hot_path_threshold, flamegraph, hints, flamegraph_output, perf_output } => {
            let args = ProfileArgs {
                file, count_instructions, track_allocations, detect_hot_paths,
                hot_path_threshold, flamegraph, hints, flamegraph_output, perf_output,
            };
            handle_profile_command(args)
        }
        Commands::Agent(agent_cmd) =>
            handle_agent_command(agent_cmd).await,
    }
}

/// Handle Lambda subcommands
/// Complexity: 5 (one per lambda subcommand)
fn handle_lambda_command(lambda_cmd: LambdaCommands) -> Result<()> {
    match lambda_cmd {
        LambdaCommands::Analyze { input, format, confidence } =>
            lambda_analyze_command(input, format, confidence),
        LambdaCommands::Convert { input, output, optimize, tests, deploy } =>
            lambda_convert_command(input, output, optimize, tests, deploy),
        LambdaCommands::Test { input, event, benchmark, load_test } =>
            lambda_test_command(input, event, benchmark, load_test),
        LambdaCommands::Build { input, arch, optimize_size, optimize_cold_start } =>
            lambda_build_command(input, arch, optimize_size, optimize_cold_start),
        LambdaCommands::Deploy { input, region, function_name, role, dry_run } =>
            lambda_deploy_command(input, region, function_name, role, dry_run),
    }
}

/// Handle Agent subcommands
/// Complexity: 8 (one per agent subcommand)
async fn handle_agent_command(agent_cmd: AgentCommands) -> Result<()> {
    match agent_cmd {
        AgentCommands::Start { port, debug, config, foreground } =>
            agent_start_command(port, debug, config, foreground).await,
        AgentCommands::Stop =>
            agent_stop_command(),
        AgentCommands::Status =>
            agent_status_command(),
        AgentCommands::Restart { port, debug, config } =>
            agent_restart_command(port, debug, config).await,
        AgentCommands::AddProject { path, id, patterns } =>
            agent_add_project_command(path, id, patterns),
        AgentCommands::RemoveProject { project } =>
            agent_remove_project_command(project),
        AgentCommands::ListProjects =>
            agent_list_projects_command(),
        AgentCommands::Logs { lines, follow } =>
            agent_logs_command(lines, follow),
    }
}
```

### **Extract Inline Agent Command Implementations**

Create dedicated handler functions for the three inline agent commands:

```rust
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
    println!("✅ Project '{}' added (path: {})", project_id, path.display());
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
```

### **Refactored main (Target: Complexity ≤10)**

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(level).init();

    // Dispatch to command handler
    handle_command(cli.command).await
}
```

**Estimated Complexity**: 3 (parse + conditional + dispatch) ✅

---

## 🧪 **Testing Strategy (EXTREME TDD)**

### **Phase 1: Integration Tests (Already Exist)**

The CLI already has integration tests. We need to ensure all tests still pass after refactoring.

### **Phase 2: Unit Tests for New Handler Functions**

```rust
#[cfg(test)]
mod handler_tests {
    use super::*;

    #[test]
    fn test_agent_add_project_command() {
        let path = PathBuf::from("/test/path");
        let id = Some("test-project".to_string());
        let patterns = vec!["*.py".to_string()];

        let result = agent_add_project_command(path, id, patterns);
        assert!(result.is_ok());
    }

    #[test]
    fn test_agent_remove_project_command() {
        let result = agent_remove_project_command("test-project".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_agent_list_projects_command() {
        let result = agent_list_projects_command();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_agent_command_stop() {
        let cmd = AgentCommands::Stop;
        // This would test the dispatcher
        // (actual implementation would require mocking)
    }
}
```

### **Phase 3: Regression Tests**

Ensure all existing CLI tests pass:
- Run `cargo test --test cli_tests`
- Verify all commands work as expected

---

## 📋 **Implementation Plan**

### **Step 1: Extract Inline Implementations** (GREEN - TDD) - 2-3 hours
- [ ] Create `agent_add_project_command` function
- [ ] Create `agent_remove_project_command` function
- [ ] Create `agent_list_projects_command` function
- [ ] Move implementations from main.rs to dedicated functions
- [ ] Verify CLI behavior unchanged

### **Step 2: Create Dispatcher Functions** (GREEN - TDD) - 5-8 hours
- [ ] Create `handle_command` async function
- [ ] Create `handle_lambda_command` function
- [ ] Create `handle_agent_command` async function
- [ ] Move all match arms to dispatcher functions
- [ ] Verify all commands still work

### **Step 3: Simplify main Function** (REFACTOR - TDD) - 3-5 hours
- [ ] Replace large match with `handle_command` call
- [ ] Verify complexity ≤10 via `pmat analyze complexity`
- [ ] Run `cargo test --workspace`
- [ ] Test all CLI commands manually

### **Step 4: Verify Quality** (TDD Verification) - 3-5 hours
- [ ] Run `pmat tdg crates/depyler/src/main.rs`
- [ ] Verify TDG score improves
- [ ] Run full test suite
- [ ] Verify no regressions
- [ ] Run clippy: `cargo clippy -- -D warnings`

### **Step 5: Documentation** - 2-3 hours
- [ ] Add rustdoc comments to dispatcher functions
- [ ] Update CHANGELOG.md
- [ ] Update roadmap.md
- [ ] Create DEPYLER-0006-COMPLETION.md

---

## ⏱️ **Time Estimate**

- **Extraction**: 2-3 hours
- **Dispatchers**: 5-8 hours
- **Refactoring**: 3-5 hours
- **Verification**: 3-5 hours
- **Documentation**: 2-3 hours

**Total**: 15-24 hours (within 20-30h estimate ✅)

---

## 🚨 **Risks and Mitigations**

### **Risk 1**: Breaking CLI behavior
**Mitigation**: Extensive integration tests already exist, run after each change

### **Risk 2**: Async/await complications
**Mitigation**: Keep async boundaries clear, use `.await` only where needed

### **Risk 3**: Import organization
**Mitigation**: Use modules or keep all handlers in main.rs for now

---

## ✅ **Success Criteria**

- [ ] `main` function complexity: 25 → ≤10
- [ ] All dispatcher functions complexity: ≤10
- [ ] All existing CLI tests pass
- [ ] TDG score: Maintains or improves
- [ ] Clippy warnings: 0
- [ ] SATD comments: 0
- [ ] Manual CLI testing: All commands work

---

## 📝 **Next Actions**

1. **Immediate**: Extract inline agent command implementations
2. **Phase 1**: Create dispatcher functions (8-10h)
3. **Phase 2**: Simplify main function
4. **Phase 3**: Verify and document

---

**Status**: Ready to begin
**Blocking**: None
**Dependencies**: None (all exist)
**Assignee**: Current session
**Sprint**: Sprint 2

---

*Created: 2025-10-02*
*Last Updated: 2025-10-02*
*Ticket: DEPYLER-0006*
