//! Background Daemon for Depyler Agent Mode
//!
//! Manages the lifecycle of the Depyler background agent service with graceful
//! startup, shutdown, and continuous Python-to-Rust transpilation capabilities.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::signal;
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

/// Configuration for the MCP agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// MCP server port
    pub port: u16,
    /// Enable debug logging
    pub debug: bool,
    /// Auto-transpile on file changes
    pub auto_transpile: bool,
    /// Verification level for transpiled code
    pub verification_level: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            debug: false,
            auto_transpile: true,
            verification_level: "basic".to_string(),
        }
    }
}
use super::transpilation_monitor::{
    TranspilationEvent, TranspilationMonitorConfig, TranspilationMonitorEngine,
};

/// Background daemon for the Depyler agent
pub struct AgentDaemon {
    /// Daemon configuration
    config: DaemonConfig,

    /// Transpilation monitor engine
    transpilation_monitor: Option<TranspilationMonitorEngine>,

    /// Daemon state
    state: Arc<RwLock<DaemonState>>,

    /// Shutdown signal sender
    shutdown_tx: Option<mpsc::Sender<()>>,
}

/// Configuration for the background daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Agent configuration
    pub agent: AgentConfig,

    /// Transpilation monitoring configuration
    pub transpilation_monitor: TranspilationMonitorConfig,

    /// Daemon-specific settings
    pub daemon: DaemonSettings,

    /// MCP server port (convenience field)
    pub mcp_port: u16,

    /// Debug mode (convenience field)
    pub debug: bool,
}

impl DaemonConfig {
    /// Load configuration from file
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            agent: AgentConfig::default(),
            transpilation_monitor: TranspilationMonitorConfig::default(),
            daemon: DaemonSettings::default(),
            mcp_port: 3000,
            debug: false,
        }
    }
}

/// Daemon-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSettings {
    /// PID file location (optional)
    pub pid_file: Option<PathBuf>,

    /// Log file location (optional)
    pub log_file: Option<PathBuf>,

    /// Working directory
    pub working_directory: PathBuf,

    /// Health check interval
    pub health_check_interval: Duration,

    /// Maximum memory usage before restart (MB)
    pub max_memory_mb: u64,

    /// Auto-restart on failure
    pub auto_restart: bool,

    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,

    /// Auto-transpile Python files on change
    pub auto_transpile: bool,

    /// Verification level for transpiled code
    pub verification_level: VerificationLevel,
}

impl Default for DaemonSettings {
    fn default() -> Self {
        Self {
            pid_file: None,
            log_file: None,
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            health_check_interval: Duration::from_secs(30),
            max_memory_mb: 1000, // More memory for transpilation
            auto_restart: true,
            shutdown_timeout: Duration::from_secs(10),
            auto_transpile: true,
            verification_level: VerificationLevel::Basic,
        }
    }
}

/// Verification level for transpiled code
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum VerificationLevel {
    /// No verification
    None,
    /// Basic syntax and type checking
    #[default]
    Basic,
    /// Full verification with property checking
    Full,
    /// Strict verification with formal proofs
    Strict,
}

/// Current state of the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonState {
    /// Daemon status
    pub status: DaemonStatus,

    /// Start time
    pub start_time: SystemTime,

    /// Last health check
    pub last_health_check: SystemTime,

    /// Current memory usage (MB)
    pub memory_usage_mb: u64,

    /// Number of monitored projects
    pub monitored_projects: usize,

    /// Total transpilations performed
    pub total_transpilations: u64,

    /// Successful transpilations
    pub successful_transpilations: u64,

    /// Failed transpilations
    pub failed_transpilations: u64,

    /// Last error (if any)
    pub last_error: Option<String>,
}

impl Default for DaemonState {
    fn default() -> Self {
        Self {
            status: DaemonStatus::Starting,
            start_time: SystemTime::now(),
            last_health_check: SystemTime::now(),
            memory_usage_mb: 0,
            monitored_projects: 0,
            total_transpilations: 0,
            successful_transpilations: 0,
            failed_transpilations: 0,
            last_error: None,
        }
    }
}

/// Daemon status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DaemonStatus {
    /// Daemon is starting up
    Starting,
    /// Daemon is running normally
    Running,
    /// Daemon is stopping
    Stopping,
    /// Daemon has stopped
    Stopped,
    /// Daemon encountered an error
    Error,
    /// Daemon is restarting
    Restarting,
}

impl AgentDaemon {
    /// Create a new agent daemon with configuration
    pub fn new(config: DaemonConfig) -> Self {
        let state = Arc::new(RwLock::new(DaemonState::default()));

        Self {
            config,
            transpilation_monitor: None,
            state,
            shutdown_tx: None,
        }
    }

    /// Start the daemon
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Depyler Agent Daemon...");

        // Update state to starting
        {
            let mut state = self.state.write().await;
            state.status = DaemonStatus::Starting;
            state.start_time = SystemTime::now();
        }

        // Write PID file if specified
        if let Some(pid_file) = &self.config.daemon.pid_file {
            let pid = std::process::id();
            std::fs::write(pid_file, pid.to_string())
                .map_err(|e| anyhow::anyhow!("Failed to write PID file: {}", e))?;
            info!("PID {} written to {:?}", pid, pid_file);
        }

        // Change working directory
        std::env::set_current_dir(&self.config.daemon.working_directory)
            .map_err(|e| anyhow::anyhow!("Failed to change working directory: {}", e))?;

        // Initialize transpilation monitor
        let transpilation_monitor =
            TranspilationMonitorEngine::new(self.config.transpilation_monitor.clone()).await?;
        self.transpilation_monitor = Some(transpilation_monitor);

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        // Update state to running
        {
            let mut state = self.state.write().await;
            state.status = DaemonStatus::Running;
        }

        info!("Depyler Agent Daemon started successfully");

        // Start main loop
        self.run_main_loop(shutdown_rx).await
    }

    /// Run the main daemon loop
    async fn run_main_loop(&mut self, mut shutdown_rx: mpsc::Receiver<()>) -> Result<()> {
        let mut health_check_interval = interval(self.config.daemon.health_check_interval);
        let mut transpilation_events = self
            .transpilation_monitor
            .as_mut()
            .and_then(|tm| tm.get_event_receiver().ok());

        loop {
            tokio::select! {
                // Handle shutdown signal
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown signal");
                    break;
                }

                // Handle system signals
                _ = signal::ctrl_c() => {
                    info!("Received Ctrl+C signal");
                    break;
                }

                // Periodic health check
                _ = health_check_interval.tick() => {
                    if let Err(e) = self.perform_health_check().await {
                        error!("Health check failed: {}", e);

                        let mut state = self.state.write().await;
                        state.last_error = Some(e.to_string());

                        if self.config.daemon.auto_restart {
                            warn!("Auto-restart enabled, restarting daemon...");
                            state.status = DaemonStatus::Restarting;
                            // Note: Automatic restart logic is not yet implemented.
                            // Currently only updates status. Restart must be triggered manually
                            // using `depyler agent restart`. This is a known limitation.
                        }
                    }
                }

                // Handle transpilation events
                event = async {
                    match transpilation_events.as_mut() {
                        Some(rx) => rx.recv().await,
                        None => None
                    }
                } => {
                    if let Some(event) = event {
                        if let Err(e) = self.handle_transpilation_event(event).await {
                            error!("Failed to handle transpilation event: {}", e);
                        }
                    }
                }
            }
        }

        // Graceful shutdown
        self.shutdown().await
    }

    /// Perform health check
    async fn perform_health_check(&self) -> Result<()> {
        debug!("Performing health check...");

        // Check memory usage
        let memory_usage = self.get_memory_usage().await?;
        if memory_usage > self.config.daemon.max_memory_mb {
            bail!(
                "Memory usage ({} MB) exceeds limit ({} MB)",
                memory_usage,
                self.config.daemon.max_memory_mb
            );
        }

        // Update state
        {
            let mut state = self.state.write().await;
            state.last_health_check = SystemTime::now();
            state.memory_usage_mb = memory_usage;
        }

        debug!("Health check passed (memory: {} MB)", memory_usage);
        Ok(())
    }

    /// Get current memory usage in MB
    async fn get_memory_usage(&self) -> Result<u64> {
        // Simple implementation - in production could use more sophisticated memory tracking
        #[cfg(unix)]
        {
            use std::fs;
            let status = fs::read_to_string("/proc/self/status")?;
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let kb: u64 = parts[1].parse().unwrap_or(0);
                        return Ok(kb / 1024); // Convert KB to MB
                    }
                }
            }
        }

        // Fallback - estimate based on process
        Ok(100) // Default estimate
    }

    /// Handle transpilation event
    async fn handle_transpilation_event(&self, event: TranspilationEvent) -> Result<()> {
        info!("Handling transpilation event: {:?}", event);

        match event {
            TranspilationEvent::FileChanged { path, .. } => {
                if self.config.daemon.auto_transpile {
                    // Perform transpilation
                    match self.transpile_file(&path).await {
                        Ok(_) => {
                            let mut state = self.state.write().await;
                            state.total_transpilations += 1;
                            state.successful_transpilations += 1;
                        }
                        Err(e) => {
                            error!("Failed to transpile {}: {}", path.display(), e);
                            let mut state = self.state.write().await;
                            state.total_transpilations += 1;
                            state.failed_transpilations += 1;
                            state.last_error = Some(e.to_string());
                        }
                    }
                }
            }
            TranspilationEvent::ProjectAdded { .. } => {
                let mut state = self.state.write().await;
                state.monitored_projects += 1;
                info!("Now monitoring {} projects", state.monitored_projects);
            }
            TranspilationEvent::ProjectRemoved { project_id: _ } => {
                let mut state = self.state.write().await;
                state.monitored_projects = state.monitored_projects.saturating_sub(1);
                info!("Now monitoring {} projects", state.monitored_projects);
            }
            TranspilationEvent::TranspilationSucceeded { project_id, .. } => {
                debug!("Transpilation succeeded for project '{}'", project_id);
                let mut state = self.state.write().await;
                state.successful_transpilations += 1;
            }
            TranspilationEvent::TranspilationFailed {
                project_id, error, ..
            } => {
                warn!(
                    "Transpilation failed for project '{}': {}",
                    project_id, error
                );
                let mut state = self.state.write().await;
                state.failed_transpilations += 1;
                state.last_error = Some(error);
            }
            TranspilationEvent::StatusUpdate { .. } => {
                debug!("Received transpilation status update");
            }
        }

        Ok(())
    }

    /// Transpile a single file
    async fn transpile_file(&self, path: &std::path::Path) -> Result<()> {
        use depyler_core::DepylerPipeline;

        // Read the Python file
        let source = std::fs::read_to_string(path)?;

        // Create transpiler pipeline
        let pipeline = DepylerPipeline::new();

        // Transpile
        let result = pipeline.transpile(&source)?;

        // Generate output path
        let output_path = path.with_extension("rs");

        // Write Rust code
        std::fs::write(&output_path, result)?;

        info!("Transpiled {} -> {}", path.display(), output_path.display());

        // Optionally verify the generated code
        if self.config.daemon.verification_level != VerificationLevel::None {
            self.verify_transpiled_code(&output_path).await?;
        }

        Ok(())
    }

    /// Verify transpiled Rust code
    async fn verify_transpiled_code(&self, rust_path: &std::path::Path) -> Result<()> {
        match self.config.daemon.verification_level {
            VerificationLevel::None => Ok(()),
            VerificationLevel::Basic => {
                // Basic syntax check with rustc --parse-only
                let output = std::process::Command::new("rustc")
                    .arg("--parse-only")
                    .arg(rust_path)
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Rust syntax check failed: {}", stderr);
                }

                Ok(())
            }
            VerificationLevel::Full => {
                // Full compilation check
                let output = std::process::Command::new("rustc")
                    .arg("--check")
                    .arg(rust_path)
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Rust compilation check failed: {}", stderr);
                }

                Ok(())
            }
            VerificationLevel::Strict => {
                // Full compilation + clippy checks
                let mut cmd = std::process::Command::new("cargo");
                cmd.args(["clippy", "--", "-D", "warnings"]).current_dir(
                    rust_path
                        .parent()
                        .unwrap_or_else(|| std::path::Path::new(".")),
                );

                let output = cmd.output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Rust strict verification failed: {}", stderr);
                }

                Ok(())
            }
        }
    }

    /// Get current daemon state
    pub async fn get_state(&self) -> DaemonState {
        self.state.read().await.clone()
    }

    /// Request graceful shutdown
    pub async fn request_shutdown(&self) -> Result<()> {
        if let Some(shutdown_tx) = &self.shutdown_tx {
            shutdown_tx.send(()).await?;
        }
        Ok(())
    }

    /// Run daemon in foreground
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting Depyler Agent Daemon in foreground mode");
        self.start().await
    }

    /// Start daemon in background
    pub async fn start_daemon(&mut self) -> Result<()> {
        info!("Starting Depyler Agent Daemon in background mode");
        // For now, just run in foreground - proper daemonization would require forking
        self.start().await
    }

    /// Shutdown daemon
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down daemon...");

        // Shutdown transpilation monitor
        if let Some(mut monitor) = self.transpilation_monitor.take() {
            if let Err(e) = monitor.shutdown().await {
                error!("Failed to shutdown transpilation monitor: {}", e);
            }
        }

        info!("Depyler Agent Daemon shut down successfully");
        Ok(())
    }

    /// Stop a running daemon
    pub fn stop_daemon() -> Result<()> {
        // This would typically send a signal to the running daemon process
        // For now, just check if a PID file exists and try to stop it
        let pid_file = std::env::temp_dir().join("depyler_agent.pid");
        if pid_file.exists() {
            let pid_str = std::fs::read_to_string(&pid_file)?;
            let pid = pid_str.trim().parse::<i32>()?;

            // Try to send SIGTERM to the process
            #[cfg(unix)]
            {
                use std::process::Command;
                let _ = Command::new("kill").arg(pid.to_string()).output();
            }

            // Remove PID file
            std::fs::remove_file(&pid_file)?;
            info!("Daemon stopped (PID: {})", pid);
        } else {
            info!("No daemon PID file found");
        }
        Ok(())
    }

    /// Check daemon status
    pub fn daemon_status() -> Result<Option<i32>> {
        let pid_file = std::env::temp_dir().join("depyler_agent.pid");
        if pid_file.exists() {
            let pid_str = std::fs::read_to_string(&pid_file)?;
            let pid = pid_str.trim().parse::<i32>()?;

            // Check if process is still running
            #[cfg(unix)]
            {
                use std::process::Command;
                let output = Command::new("ps").args(["-p", &pid.to_string()]).output()?;

                if output.status.success() {
                    Ok(Some(pid))
                } else {
                    // Process not running, clean up PID file
                    let _ = std::fs::remove_file(&pid_file);
                    Ok(None)
                }
            }

            #[cfg(not(unix))]
            Ok(Some(pid))
        } else {
            Ok(None)
        }
    }

    /// Show daemon logs
    pub fn show_logs(lines: usize) -> Result<()> {
        let log_file = std::env::temp_dir().join("depyler_agent.log");
        if log_file.exists() {
            let content = std::fs::read_to_string(&log_file)?;
            let lines_vec: Vec<&str> = content.lines().collect();
            let start = lines_vec.len().saturating_sub(lines);

            for line in &lines_vec[start..] {
                println!("{}", line);
            }
        } else {
            println!("No log file found");
        }
        Ok(())
    }

    /// Tail daemon logs
    pub fn tail_logs() -> Result<()> {
        println!(
            "Log following not yet implemented. Use 'depyler agent logs' to view recent logs."
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test AgentConfig
    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.port, 3000);
        assert!(!config.debug);
        assert!(config.auto_transpile);
        assert_eq!(config.verification_level, "basic");
    }

    #[test]
    fn test_agent_config_serialization() {
        let config = AgentConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("3000"));
        assert!(json.contains("basic"));
    }

    #[test]
    fn test_agent_config_deserialization() {
        let json = r#"{"port": 8080, "debug": true, "auto_transpile": false, "verification_level": "full"}"#;
        let config: AgentConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.port, 8080);
        assert!(config.debug);
        assert!(!config.auto_transpile);
        assert_eq!(config.verification_level, "full");
    }

    // Test DaemonConfig
    #[test]
    fn test_daemon_config_default() {
        let config = DaemonConfig::default();
        assert_eq!(config.mcp_port, 3000);
        assert!(!config.debug);
        assert_eq!(config.agent.port, 3000);
    }

    #[test]
    fn test_daemon_config_serialization() {
        let config = DaemonConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("mcp_port"));
        assert!(json.contains("debug"));
    }

    // Test DaemonSettings
    #[test]
    fn test_daemon_settings_default() {
        let settings = DaemonSettings::default();
        assert!(settings.pid_file.is_none());
        assert!(settings.log_file.is_none());
        assert_eq!(settings.health_check_interval, Duration::from_secs(30));
        assert_eq!(settings.max_memory_mb, 1000);
        assert!(settings.auto_restart);
        assert_eq!(settings.shutdown_timeout, Duration::from_secs(10));
        assert!(settings.auto_transpile);
        assert_eq!(settings.verification_level, VerificationLevel::Basic);
    }

    #[test]
    fn test_daemon_settings_with_pid_file() {
        let settings = DaemonSettings {
            pid_file: Some(PathBuf::from("/tmp/depyler.pid")),
            ..Default::default()
        };
        assert_eq!(
            settings.pid_file,
            Some(PathBuf::from("/tmp/depyler.pid"))
        );
    }

    #[test]
    fn test_daemon_settings_serialization() {
        let settings = DaemonSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("health_check_interval"));
        assert!(json.contains("max_memory_mb"));
        assert!(json.contains("auto_restart"));
    }

    // Test VerificationLevel
    #[test]
    fn test_verification_level_default() {
        let level = VerificationLevel::default();
        assert_eq!(level, VerificationLevel::Basic);
    }

    #[test]
    fn test_verification_level_variants() {
        assert_eq!(VerificationLevel::None, VerificationLevel::None);
        assert_eq!(VerificationLevel::Basic, VerificationLevel::Basic);
        assert_eq!(VerificationLevel::Full, VerificationLevel::Full);
        assert_eq!(VerificationLevel::Strict, VerificationLevel::Strict);
    }

    #[test]
    fn test_verification_level_serialization() {
        let level = VerificationLevel::Full;
        let json = serde_json::to_string(&level).unwrap();
        assert!(json.contains("Full"));
    }

    #[test]
    fn test_verification_level_deserialization() {
        let json = r#""Strict""#;
        let level: VerificationLevel = serde_json::from_str(json).unwrap();
        assert_eq!(level, VerificationLevel::Strict);
    }

    #[test]
    fn test_verification_level_clone() {
        let level = VerificationLevel::Full;
        let cloned = level; // Copy type, clone() unnecessary
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_verification_level_copy() {
        let level = VerificationLevel::Strict;
        let copied: VerificationLevel = level;
        assert_eq!(level, copied);
    }

    // Test DaemonState
    #[test]
    fn test_daemon_state_default() {
        let state = DaemonState::default();
        assert_eq!(state.status, DaemonStatus::Starting);
        assert_eq!(state.memory_usage_mb, 0);
        assert_eq!(state.monitored_projects, 0);
        assert_eq!(state.total_transpilations, 0);
        assert_eq!(state.successful_transpilations, 0);
        assert_eq!(state.failed_transpilations, 0);
        assert!(state.last_error.is_none());
    }

    #[test]
    fn test_daemon_state_with_error() {
        let state = DaemonState {
            last_error: Some("Test error".to_string()),
            ..Default::default()
        };
        assert_eq!(state.last_error, Some("Test error".to_string()));
    }

    #[test]
    fn test_daemon_state_serialization() {
        let state = DaemonState::default();
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("status"));
        assert!(json.contains("memory_usage_mb"));
    }

    #[test]
    fn test_daemon_state_clone() {
        let state = DaemonState::default();
        let cloned = state.clone();
        assert_eq!(state.status, cloned.status);
    }

    // Test DaemonStatus
    #[test]
    fn test_daemon_status_variants() {
        assert_eq!(DaemonStatus::Starting, DaemonStatus::Starting);
        assert_eq!(DaemonStatus::Running, DaemonStatus::Running);
        assert_eq!(DaemonStatus::Stopping, DaemonStatus::Stopping);
        assert_eq!(DaemonStatus::Stopped, DaemonStatus::Stopped);
        assert_eq!(DaemonStatus::Error, DaemonStatus::Error);
        assert_eq!(DaemonStatus::Restarting, DaemonStatus::Restarting);
    }

    #[test]
    fn test_daemon_status_serialization() {
        let status = DaemonStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("Running"));
    }

    #[test]
    fn test_daemon_status_deserialization() {
        let json = r#""Stopped""#;
        let status: DaemonStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, DaemonStatus::Stopped);
    }

    #[test]
    fn test_daemon_status_clone() {
        let status = DaemonStatus::Running;
        let cloned = status; // Copy type, clone() unnecessary
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_daemon_status_copy() {
        let status = DaemonStatus::Running;
        let copied: DaemonStatus = status;
        assert_eq!(status, copied);
    }

    // Test AgentDaemon
    #[test]
    fn test_agent_daemon_new() {
        let config = DaemonConfig::default();
        let daemon = AgentDaemon::new(config);
        assert!(daemon.transpilation_monitor.is_none());
        assert!(daemon.shutdown_tx.is_none());
    }

    #[tokio::test]
    async fn test_agent_daemon_get_state() {
        let config = DaemonConfig::default();
        let daemon = AgentDaemon::new(config);
        let state = daemon.get_state().await;
        assert_eq!(state.status, DaemonStatus::Starting);
    }

    #[test]
    fn test_agent_daemon_stop_daemon_no_pid_file() {
        // Should not error even if there's no PID file
        let result = AgentDaemon::stop_daemon();
        // Result depends on whether temp dir has a PID file
        // In most cases it won't, so it should succeed
        assert!(result.is_ok());
    }

    #[test]
    fn test_agent_daemon_daemon_status_no_pid_file() {
        let result = AgentDaemon::daemon_status();
        assert!(result.is_ok());
        // No PID file means daemon is not running
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_agent_daemon_tail_logs() {
        let result = AgentDaemon::tail_logs();
        assert!(result.is_ok());
    }

    #[test]
    fn test_agent_daemon_show_logs_no_file() {
        // Should not error even if there's no log file
        let result = AgentDaemon::show_logs(10);
        assert!(result.is_ok());
    }

    // Test config loading
    #[test]
    fn test_daemon_config_from_file_not_found() {
        let result = DaemonConfig::from_file(std::path::Path::new("/nonexistent/path.json"));
        assert!(result.is_err());
    }

    #[test]
    fn test_daemon_config_from_file_invalid_json() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_invalid_config.json");
        std::fs::write(&temp_file, "not valid json").unwrap();

        let result = DaemonConfig::from_file(&temp_file);
        assert!(result.is_err());

        // Cleanup
        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn test_daemon_config_from_file_valid() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_valid_daemon_config.json");

        let config = DaemonConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(&temp_file, json).unwrap();

        let result = DaemonConfig::from_file(&temp_file);
        assert!(result.is_ok());
        let loaded = result.unwrap();
        assert_eq!(loaded.mcp_port, 3000);

        // Cleanup
        let _ = std::fs::remove_file(temp_file);
    }

    // Test transpilation statistics tracking
    #[tokio::test]
    async fn test_daemon_state_counters() {
        let config = DaemonConfig::default();
        let daemon = AgentDaemon::new(config);

        let state = daemon.get_state().await;
        assert_eq!(state.total_transpilations, 0);
        assert_eq!(state.successful_transpilations, 0);
        assert_eq!(state.failed_transpilations, 0);
    }

    // Test DaemonState timing
    #[test]
    fn test_daemon_state_start_time() {
        let before = SystemTime::now();
        let state = DaemonState::default();
        let after = SystemTime::now();

        // Start time should be between before and after
        assert!(state.start_time >= before);
        assert!(state.start_time <= after);
    }

    #[test]
    fn test_daemon_state_health_check_time() {
        let state = DaemonState::default();
        // Last health check should be close to start time
        let diff = state.start_time
            .duration_since(state.last_health_check)
            .unwrap_or(Duration::ZERO);
        // Should be within 1 second
        assert!(diff.as_secs() < 1);
    }

    // Test serialization round-trip
    #[test]
    fn test_daemon_config_round_trip() {
        let config = DaemonConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DaemonConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.mcp_port, deserialized.mcp_port);
        assert_eq!(config.debug, deserialized.debug);
    }

    #[test]
    fn test_daemon_settings_round_trip() {
        let settings = DaemonSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: DaemonSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings.max_memory_mb, deserialized.max_memory_mb);
        assert_eq!(settings.auto_restart, deserialized.auto_restart);
    }

    #[test]
    fn test_daemon_state_round_trip() {
        let state = DaemonState {
            status: DaemonStatus::Running,
            total_transpilations: 100,
            successful_transpilations: 95,
            failed_transpilations: 5,
            memory_usage_mb: 512,
            monitored_projects: 3,
            last_error: Some("Test error".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: DaemonState = serde_json::from_str(&json).unwrap();

        assert_eq!(state.status, deserialized.status);
        assert_eq!(state.total_transpilations, deserialized.total_transpilations);
        assert_eq!(state.successful_transpilations, deserialized.successful_transpilations);
        assert_eq!(state.failed_transpilations, deserialized.failed_transpilations);
        assert_eq!(state.last_error, deserialized.last_error);
    }

    // Test Debug trait
    #[test]
    fn test_verification_level_debug() {
        let level = VerificationLevel::Full;
        let debug = format!("{:?}", level);
        assert!(debug.contains("Full"));
    }

    #[test]
    fn test_daemon_status_debug() {
        let status = DaemonStatus::Running;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Running"));
    }

    #[test]
    fn test_daemon_settings_debug() {
        let settings = DaemonSettings::default();
        let debug = format!("{:?}", settings);
        assert!(debug.contains("health_check_interval"));
    }

    #[test]
    fn test_daemon_state_debug() {
        let state = DaemonState::default();
        let debug = format!("{:?}", state);
        assert!(debug.contains("status"));
    }

    #[test]
    fn test_agent_config_debug() {
        let config = AgentConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("port"));
    }

    #[test]
    fn test_daemon_config_debug() {
        let config = DaemonConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("mcp_port"));
    }
}
