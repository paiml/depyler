//! Background Agent Mode for Depyler
//!
//! Provides continuous monitoring and transpilation services for Python codebases,
//! integrating with Claude Code for seamless development workflow.

pub mod daemon;
pub mod mcp_server;
pub mod transpilation_monitor;

pub use daemon::{AgentDaemon, DaemonConfig, DaemonSettings, DaemonState, DaemonStatus};
pub use mcp_server::DepylerMcpServer;
pub use transpilation_monitor::{
    TranspilationEvent, TranspilationMonitorConfig, TranspilationMonitorEngine,
};
