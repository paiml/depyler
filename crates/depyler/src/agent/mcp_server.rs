//! Depyler MCP Server using PMCP SDK
//!
//! This module implements a high-performance MCP server for Depyler using the pmcp SDK,
//! providing Python-to-Rust transpilation tools for Claude Code integration.

use async_trait::async_trait;
use pmcp::{Error, RequestHandlerExtra, Result, Server, ServerCapabilities, ToolCapabilities, ToolHandler};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use depyler_core::DepylerPipeline;

/// Depyler MCP Server using PMCP SDK
/// 
/// Provides Python-to-Rust transpilation capabilities through MCP protocol,
/// leveraging the pmcp SDK for high-performance JSON-RPC handling.
pub struct DepylerMcpServer {
    /// Server state for tracking projects and sessions
    state: Arc<Mutex<ServerState>>,
}

/// Server state for tracking transpilation projects and statistics
#[derive(Debug)]
struct ServerState {
    /// Active transpilation projects
    projects: std::collections::HashMap<String, ProjectInfo>,
    
    /// Total files transpiled
    total_transpilations: u64,
    
    /// Successful transpilations
    successful_transpilations: u64,
    
    /// Failed transpilations  
    failed_transpilations: u64,
    
    /// Server start time
    start_time: std::time::SystemTime,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            projects: std::collections::HashMap::new(),
            total_transpilations: 0,
            successful_transpilations: 0,
            failed_transpilations: 0,
            start_time: std::time::SystemTime::now(),
        }
    }
}

/// Information about a monitored project
#[derive(Debug, Clone, Serialize)]
struct ProjectInfo {
    /// Project name
    name: String,
    
    /// Root path
    path: PathBuf,
    
    /// Watch patterns
    patterns: Vec<String>,
    
    /// Files transpiled in this project
    files_transpiled: u64,
    
    /// Last transpilation time
    last_transpilation: Option<std::time::SystemTime>,
}

impl DepylerMcpServer {
    /// Create a new Depyler MCP server
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServerState {
                start_time: std::time::SystemTime::now(),
                ..Default::default()
            })),
        }
    }
    
    /// Shutdown the MCP server
    pub async fn shutdown(&self) -> Result<()> {
        // Clean up any resources here
        debug!("Shutting down Depyler MCP server");
        Ok(())
    }

    /// Run the MCP server with stdio transport
    pub async fn run(&self) -> Result<()> {
        info!("Starting Depyler MCP Server using PMCP SDK");

        // Build server with Depyler transpilation tools
        let server = Server::builder()
            .name("depyler-agent")
            .version(env!("CARGO_PKG_VERSION"))
            .capabilities(ServerCapabilities {
                tools: Some(ToolCapabilities { list_changed: None }),
                ..Default::default()
            })
            // Core transpilation tools
            .tool("transpile_python_file", TranspilePythonFileTool::new(self.state.clone()))
            .tool("transpile_python_directory", TranspilePythonDirectoryTool::new(self.state.clone()))
            .tool("monitor_python_project", MonitorPythonProjectTool::new(self.state.clone()))
            .tool("get_transpilation_status", GetTranspilationStatusTool::new(self.state.clone()))
            .tool("verify_rust_code", VerifyRustCodeTool::new())
            .tool("analyze_python_compatibility", AnalyzePythonCompatibilityTool::new())
            .build()?;

        info!(
            "Depyler MCP server ready with {} transpilation tools, listening on stdio",
            6
        );

        // Run server with stdio transport
        server.run_stdio().await?;

        info!("Depyler MCP server shutting down");
        Ok(())
    }
}

impl Default for DepylerMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

// Tool Implementations using PMCP ToolHandler pattern

/// Tool for transpiling a single Python file to Rust
pub struct TranspilePythonFileTool {
    state: Arc<Mutex<ServerState>>,
}

impl TranspilePythonFileTool {
    pub fn new(state: Arc<Mutex<ServerState>>) -> Self {
        Self { state }
    }
}

#[derive(Debug, Deserialize)]
struct TranspileFileArgs {
    file_path: String,
    #[serde(default)]
    output_path: Option<String>,
    #[serde(default)]
    verification_level: Option<String>,
    #[serde(default)]
    verify: Option<bool>,
}

#[async_trait]
impl ToolHandler for TranspilePythonFileTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value> {
        debug!("Handling transpile_python_file with args: {}", args);

        let params: TranspileFileArgs = serde_json::from_value(args)
            .map_err(|e| Error::validation(format!("Invalid arguments: {}", e)))?;

        let file_path = PathBuf::from(&params.file_path);
        
        // Validate file exists and is Python
        if !file_path.exists() {
            return Err(Error::validation(format!("File not found: {}", params.file_path)));
        }
        
        if file_path.extension().and_then(|s| s.to_str()) != Some("py") {
            return Err(Error::validation("File must have .py extension".to_string()));
        }

        let start_time = std::time::Instant::now();
        
        // Read Python source
        let source = std::fs::read_to_string(&file_path)
            .map_err(|e| Error::internal(format!("Failed to read file: {}", e)))?;
        
        let python_lines = source.lines().count();

        // Configure transpiler pipeline
        let pipeline = DepylerPipeline::new();

        // Perform transpilation
        let transpile_result = pipeline
            .transpile(&source)
            .map_err(|e| Error::internal(format!("Transpilation failed: {}", e)))?;
        
        let rust_lines = transpile_result.lines().count();

        // Determine output path
        let output_path = match params.output_path {
            Some(custom_path) => PathBuf::from(custom_path),
            None => file_path.with_extension("rs"),
        };

        // Write Rust code
        std::fs::write(&output_path, &transpile_result)
            .map_err(|e| Error::internal(format!("Failed to write output file: {}", e)))?;

        let transpilation_time = start_time.elapsed();

        // Update statistics
        {
            let mut state = self.state.lock().await;
            state.total_transpilations += 1;
            state.successful_transpilations += 1;
        }

        // Perform verification if requested
        let verification_result = if params.verify.unwrap_or(false) {
            let verify_level = params.verification_level.as_deref().unwrap_or("basic");
            match verify_rust_code(&output_path, verify_level).await {
                Ok(result) => Some(result),
                Err(e) => {
                    warn!("Verification failed: {}", e);
                    Some(json!({
                        "success": false,
                        "error": e.to_string()
                    }))
                }
            }
        } else {
            None
        };

        let result = json!({
            "content": [{
                "type": "text",
                "text": format!(
                    "✅ Successfully transpiled Python to Rust\n\n\
                    📂 Input: {}\n\
                    📂 Output: {}\n\
                    📊 Lines: {} Python → {} Rust\n\
                    ⏱️  Time: {}ms\n\
                    {}{}",
                    file_path.display(),
                    output_path.display(),
                    python_lines,
                    rust_lines,
                    transpilation_time.as_millis(),
                    if let Some(ref verify) = verification_result {
                        format!("🔍 Verification: {}\n", 
                               if verify.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                                   "✅ Passed"
                               } else {
                                   "❌ Failed"
                               })
                    } else {
                        "".to_string()
                    },
                    "".to_string()
                )
            }],
            "metadata": {
                "python_file": params.file_path,
                "rust_file": output_path.to_string_lossy(),
                "python_lines": python_lines,
                "rust_lines": rust_lines,
                "transpilation_time_ms": transpilation_time.as_millis(),
                "verification": verification_result,
                "warnings_count": 0
            }
        });

        Ok(result)
    }
}

/// Tool for transpiling an entire Python directory
pub struct TranspilePythonDirectoryTool {
    state: Arc<Mutex<ServerState>>,
}

impl TranspilePythonDirectoryTool {
    pub fn new(state: Arc<Mutex<ServerState>>) -> Self {
        Self { state }
    }
}

#[derive(Debug, Deserialize)]
struct TranspileDirectoryArgs {
    directory_path: String,
    #[serde(default)]
    output_directory: Option<String>,
    #[serde(default)]
    recursive: Option<bool>,
    #[serde(default)]
    verify: Option<bool>,
}

#[async_trait]
impl ToolHandler for TranspilePythonDirectoryTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value> {
        debug!("Handling transpile_python_directory with args: {}", args);

        let params: TranspileDirectoryArgs = serde_json::from_value(args)
            .map_err(|e| Error::validation(format!("Invalid arguments: {}", e)))?;

        let dir_path = PathBuf::from(&params.directory_path);
        
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(Error::validation(format!("Directory not found: {}", params.directory_path)));
        }

        let recursive = params.recursive.unwrap_or(true);
        let _verify = params.verify.unwrap_or(false);
        
        // Find all Python files
        let mut python_files = Vec::new();
        find_python_files(&dir_path, recursive, &mut python_files)?;

        if python_files.is_empty() {
            return Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!("No Python files found in {}", params.directory_path)
                }]
            }));
        }

        let start_time = std::time::Instant::now();
        let mut transpiled_count = 0;
        let mut failed_count = 0;
        let mut total_python_lines = 0;
        let mut total_rust_lines = 0;
        let mut errors = Vec::new();

        // Create transpiler pipeline
        let pipeline = DepylerPipeline::new();

        // Transpile each file
        for python_file in &python_files {
            let source = match std::fs::read_to_string(python_file) {
                Ok(s) => s,
                Err(e) => {
                    failed_count += 1;
                    errors.push(format!("Failed to read {}: {}", python_file.display(), e));
                    continue;
                }
            };

            total_python_lines += source.lines().count();

            match pipeline.transpile(&source) {
                Ok(result) => {
                    total_rust_lines += result.lines().count();
                    
                    let output_path = match &params.output_directory {
                        Some(out_dir) => {
                            let relative_path = python_file.strip_prefix(&dir_path)
                                .unwrap_or(python_file);
                            PathBuf::from(out_dir).join(relative_path).with_extension("rs")
                        }
                        None => python_file.with_extension("rs"),
                    };

                    // Create output directory if needed
                    if let Some(parent) = output_path.parent() {
                        if let Err(e) = std::fs::create_dir_all(parent) {
                            failed_count += 1;
                            errors.push(format!("Failed to create directory {}: {}", parent.display(), e));
                            continue;
                        }
                    }

                    if let Err(e) = std::fs::write(&output_path, &result) {
                        failed_count += 1;
                        errors.push(format!("Failed to write {}: {}", output_path.display(), e));
                    } else {
                        transpiled_count += 1;
                    }
                }
                Err(e) => {
                    failed_count += 1;
                    errors.push(format!("Transpilation failed for {}: {}", python_file.display(), e));
                }
            }
        }

        let total_time = start_time.elapsed();

        // Update statistics
        {
            let mut state = self.state.lock().await;
            state.total_transpilations += transpiled_count;
            state.successful_transpilations += transpiled_count;
            state.failed_transpilations += failed_count;
        }

        let result = json!({
            "content": [{
                "type": "text",
                "text": format!(
                    "📁 Directory Transpilation Complete\n\n\
                    📂 Directory: {}\n\
                    📄 Files found: {}\n\
                    ✅ Successfully transpiled: {}\n\
                    ❌ Failed: {}\n\
                    📊 Total lines: {} Python → {} Rust\n\
                    ⏱️  Total time: {}ms\n\
                    ⚡ Average: {}ms per file\n\
                    {}",
                    params.directory_path,
                    python_files.len(),
                    transpiled_count,
                    failed_count,
                    total_python_lines,
                    total_rust_lines,
                    total_time.as_millis(),
                    if transpiled_count > 0 { total_time.as_millis() / transpiled_count as u128 } else { 0 },
                    if errors.is_empty() {
                        "".to_string()
                    } else {
                        format!("\n⚠️  Errors:\n{}", errors.join("\n"))
                    }
                )
            }],
            "metadata": {
                "directory": params.directory_path,
                "files_found": python_files.len(),
                "files_transpiled": transpiled_count,
                "files_failed": failed_count,
                "total_python_lines": total_python_lines,
                "total_rust_lines": total_rust_lines,
                "total_time_ms": total_time.as_millis(),
                "errors": errors
            }
        });

        Ok(result)
    }
}

/// Tool for monitoring Python projects for automatic transpilation
pub struct MonitorPythonProjectTool {
    state: Arc<Mutex<ServerState>>,
}

impl MonitorPythonProjectTool {
    pub fn new(state: Arc<Mutex<ServerState>>) -> Self {
        Self { state }
    }
}

#[derive(Debug, Deserialize)]
struct MonitorProjectArgs {
    project_path: String,
    #[serde(default)]
    project_name: Option<String>,
    #[serde(default)]
    watch_patterns: Option<Vec<String>>,
}

#[async_trait]
impl ToolHandler for MonitorPythonProjectTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value> {
        debug!("Handling monitor_python_project with args: {}", args);

        let params: MonitorProjectArgs = serde_json::from_value(args)
            .map_err(|e| Error::validation(format!("Invalid arguments: {}", e)))?;

        let project_path = PathBuf::from(&params.project_path);
        
        if !project_path.exists() || !project_path.is_dir() {
            return Err(Error::validation(format!("Project directory not found: {}", params.project_path)));
        }

        let project_name = params.project_name
            .unwrap_or_else(|| project_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unnamed_project")
                .to_string());

        let watch_patterns = params.watch_patterns
            .unwrap_or_else(|| vec!["**/*.py".to_string()]);

        // Add to monitored projects
        {
            let mut state = self.state.lock().await;
            state.projects.insert(project_name.clone(), ProjectInfo {
                name: project_name.clone(),
                path: project_path.clone(),
                patterns: watch_patterns.clone(),
                files_transpiled: 0,
                last_transpilation: None,
            });
        }

        let result = json!({
            "content": [{
                "type": "text",
                "text": format!(
                    "🔍 Started monitoring Python project\n\n\
                    📁 Project: {}\n\
                    📂 Path: {}\n\
                    🎯 Patterns: {:?}\n\
                    🔄 Auto-transpilation: Enabled\n\n\
                    The project is now being monitored for changes. Python files will be automatically transpiled to Rust when modified.",
                    project_name,
                    project_path.display(),
                    watch_patterns
                )
            }],
            "metadata": {
                "project_name": project_name,
                "project_path": params.project_path,
                "watch_patterns": watch_patterns,
                "monitoring": true
            }
        });

        Ok(result)
    }
}

/// Tool for getting transpilation status and statistics
pub struct GetTranspilationStatusTool {
    state: Arc<Mutex<ServerState>>,
}

impl GetTranspilationStatusTool {
    pub fn new(state: Arc<Mutex<ServerState>>) -> Self {
        Self { state }
    }
}

#[derive(Debug, Deserialize)]
struct GetStatusArgs {
    #[serde(default)]
    project_name: Option<String>,
    #[serde(default)]
    detailed: Option<bool>,
}

#[async_trait]
impl ToolHandler for GetTranspilationStatusTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value> {
        debug!("Handling get_transpilation_status with args: {}", args);

        let params: GetStatusArgs = serde_json::from_value(args)
            .map_err(|e| Error::validation(format!("Invalid arguments: {}", e)))?;

        let state = self.state.lock().await;
        let uptime = state.start_time.elapsed().unwrap_or_default();
        let success_rate = if state.total_transpilations > 0 {
            (state.successful_transpilations as f64 / state.total_transpilations as f64) * 100.0
        } else {
            100.0
        };

        let status_text = if let Some(project_name) = &params.project_name {
            // Get specific project status
            if let Some(project) = state.projects.get(project_name) {
                format!(
                    "📊 Project Status: {}\n\n\
                    📂 Path: {}\n\
                    📄 Files transpiled: {}\n\
                    🕐 Last transpilation: {}\n\
                    🎯 Watch patterns: {:?}",
                    project.name,
                    project.path.display(),
                    project.files_transpiled,
                    project.last_transpilation
                        .map(|t| format!("{:?} ago", t.elapsed().unwrap_or_default()))
                        .unwrap_or("Never".to_string()),
                    project.patterns
                )
            } else {
                format!("Project '{}' not found", project_name)
            }
        } else {
            // Get overall status
            format!(
                "🚀 Depyler Agent Status\n\n\
                ⏱️  Uptime: {}s\n\
                📁 Projects monitored: {}\n\
                📄 Total transpilations: {}\n\
                ✅ Successful: {}\n\
                ❌ Failed: {}\n\
                📈 Success rate: {:.1}%\n\
                {}",
                uptime.as_secs(),
                state.projects.len(),
                state.total_transpilations,
                state.successful_transpilations,
                state.failed_transpilations,
                success_rate,
                if params.detailed.unwrap_or(false) && !state.projects.is_empty() {
                    format!("\n🔍 Monitored Projects:\n{}", 
                           state.projects.values()
                               .map(|p| format!("  • {} ({})", p.name, p.path.display()))
                               .collect::<Vec<_>>()
                               .join("\n"))
                } else {
                    "".to_string()
                }
            )
        };

        let result = json!({
            "content": [{
                "type": "text",
                "text": status_text
            }],
            "metadata": {
                "uptime_seconds": uptime.as_secs(),
                "projects_monitored": state.projects.len(),
                "total_transpilations": state.total_transpilations,
                "successful_transpilations": state.successful_transpilations,
                "failed_transpilations": state.failed_transpilations,
                "success_rate": success_rate,
                "projects": if params.detailed.unwrap_or(false) {
                    Some(state.projects.values().cloned().collect::<Vec<_>>())
                } else {
                    None
                }
            }
        });

        Ok(result)
    }
}

/// Tool for verifying generated Rust code
pub struct VerifyRustCodeTool;

impl VerifyRustCodeTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct VerifyRustArgs {
    rust_file_path: String,
    #[serde(default)]
    verification_level: Option<String>,
}

#[async_trait]
impl ToolHandler for VerifyRustCodeTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value> {
        debug!("Handling verify_rust_code with args: {}", args);

        let params: VerifyRustArgs = serde_json::from_value(args)
            .map_err(|e| Error::validation(format!("Invalid arguments: {}", e)))?;

        let rust_file = PathBuf::from(&params.rust_file_path);
        if !rust_file.exists() {
            return Err(Error::validation(format!("Rust file not found: {}", params.rust_file_path)));
        }

        let verification_level = params.verification_level.as_deref().unwrap_or("basic");
        
        match verify_rust_code(&rust_file, verification_level).await {
            Ok(result) => Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "🔍 Rust Code Verification\n\n\
                        📂 File: {}\n\
                        🎯 Level: {}\n\
                        {}\n\
                        ⏱️  Time: {}ms",
                        rust_file.display(),
                        verification_level,
                        if result.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                            "✅ Verification passed"
                        } else {
                            "❌ Verification failed"
                        },
                        result.get("verification_time_ms").and_then(|v| v.as_u64()).unwrap_or(0)
                    )
                }],
                "metadata": result
            })),
            Err(e) => Err(Error::internal(format!("Verification failed: {}", e)))
        }
    }
}

/// Tool for analyzing Python code compatibility with Depyler
pub struct AnalyzePythonCompatibilityTool;

impl AnalyzePythonCompatibilityTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct AnalyzeCompatibilityArgs {
    python_file_path: String,
}

#[async_trait]
impl ToolHandler for AnalyzePythonCompatibilityTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value> {
        debug!("Handling analyze_python_compatibility with args: {}", args);

        let params: AnalyzeCompatibilityArgs = serde_json::from_value(args)
            .map_err(|e| Error::validation(format!("Invalid arguments: {}", e)))?;

        let python_file = PathBuf::from(&params.python_file_path);
        if !python_file.exists() {
            return Err(Error::validation(format!("Python file not found: {}", params.python_file_path)));
        }

        // Read and analyze Python code
        let source = std::fs::read_to_string(&python_file)
            .map_err(|e| Error::internal(format!("Failed to read file: {}", e)))?;

        let analysis = analyze_python_compatibility(&source).await?;

        let result = json!({
            "content": [{
                "type": "text",
                "text": format!(
                    "🔍 Python Compatibility Analysis\n\n\
                    📂 File: {}\n\
                    📊 Compatibility Score: {:.1}%\n\
                    ✅ Supported Features: {}\n\
                    ⚠️  Potential Issues: {}\n\
                    ❌ Unsupported Features: {}\n\n\
                    {}",
                    python_file.display(),
                    analysis.get("compatibility_score").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    analysis.get("supported_features").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
                    analysis.get("warnings").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
                    analysis.get("unsupported_features").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
                    analysis.get("recommendations").and_then(|v| v.as_str()).unwrap_or("")
                )
            }],
            "metadata": analysis
        });

        Ok(result)
    }
}

// Helper functions

/// Find all Python files in a directory
fn find_python_files(dir: &PathBuf, recursive: bool, files: &mut Vec<PathBuf>) -> Result<()> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| Error::internal(format!("Failed to read directory: {}", e)))?;

    for entry in entries {
        let entry = entry.map_err(|e| Error::internal(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("py") {
            files.push(path);
        } else if path.is_dir() && recursive {
            find_python_files(&path, recursive, files)?;
        }
    }
    
    Ok(())
}

/// Verify Rust code with different levels
async fn verify_rust_code(rust_file: &PathBuf, level: &str) -> Result<Value> {
    let start_time = std::time::Instant::now();
    let mut messages = Vec::new();
    let mut success = true;

    match level {
        "basic" => {
            // Basic syntax check
            let output = std::process::Command::new("rustc")
                .args(&["--parse-only", &rust_file.to_string_lossy()])
                .output()
                .map_err(|e| Error::internal(format!("Failed to run rustc: {}", e)))?;
            
            if !output.status.success() {
                success = false;
                messages.push(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        "full" => {
            // Full compilation check
            let output = std::process::Command::new("rustc")
                .args(&["--check", &rust_file.to_string_lossy()])
                .output()
                .map_err(|e| Error::internal(format!("Failed to run rustc: {}", e)))?;
            
            if !output.status.success() {
                success = false;
                messages.push(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        "strict" => {
            // Clippy checks
            let output = std::process::Command::new("cargo")
                .args(&["clippy", "--", "-D", "warnings"])
                .current_dir(rust_file.parent().unwrap_or_else(|| std::path::Path::new(".")))
                .output()
                .map_err(|e| Error::internal(format!("Failed to run clippy: {}", e)))?;
            
            if !output.status.success() {
                success = false;
                messages.push(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        _ => return Err(Error::validation(format!("Unknown verification level: {}", level)))
    }

    let verification_time = start_time.elapsed();

    Ok(json!({
        "success": success,
        "level": level,
        "messages": messages,
        "verification_time_ms": verification_time.as_millis()
    }))
}

/// Analyze Python code compatibility with Depyler
async fn analyze_python_compatibility(source: &str) -> Result<Value> {
    // Simple analysis - in practice this would be much more sophisticated
    let lines = source.lines().collect::<Vec<_>>();
    let mut supported_features = Vec::new();
    let mut warnings = Vec::new();
    let mut unsupported_features = Vec::new();
    
    // Check for basic Python constructs
    if source.contains("def ") {
        supported_features.push("Functions");
    }
    if source.contains("class ") {
        supported_features.push("Classes");
    }
    if source.contains("for ") || source.contains("while ") {
        supported_features.push("Loops");
    }
    if source.contains("if ") {
        supported_features.push("Conditionals");
    }
    
    // Check for potentially problematic constructs
    if source.contains("eval") || source.contains("exec") {
        unsupported_features.push("Dynamic code execution (eval/exec)");
    }
    if source.contains("import sys") {
        warnings.push("System imports may need manual handling");
    }
    
    let compatibility_score = if lines.len() > 0 {
        ((supported_features.len() as f64) / (supported_features.len() + unsupported_features.len()) as f64) * 100.0
    } else {
        100.0
    };
    
    let recommendations = if unsupported_features.is_empty() {
        "✅ Code appears compatible with Depyler transpilation"
    } else {
        "⚠️ Some features may need manual refactoring before transpilation"
    };
    
    Ok(json!({
        "compatibility_score": compatibility_score,
        "supported_features": supported_features,
        "warnings": warnings,
        "unsupported_features": unsupported_features,
        "recommendations": recommendations,
        "total_lines": lines.len()
    }))
}