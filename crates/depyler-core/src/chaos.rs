// Chaos Engineering Configuration (from renacer Sprint 29)
// Source: renacer v0.4.1 (https://github.com/paiml/renacer)
//
// Provides chaos testing infrastructure for systematic fault injection
// and stress testing of the Depyler transpiler.

use std::time::Duration;

/// Chaos engineering configuration for stress testing the transpiler
///
/// Supports:
/// - Memory limit enforcement
/// - CPU throttling
/// - Timeout controls
/// - Signal injection for fault simulation
///
/// # Examples
///
/// ```
/// use depyler_core::chaos::ChaosConfig;
/// use std::time::Duration;
///
/// // Gentle chaos testing (development)
/// let gentle = ChaosConfig::gentle();
///
/// // Aggressive chaos testing (CI/CD)
/// let aggressive = ChaosConfig::aggressive();
///
/// // Custom configuration
/// let custom = ChaosConfig::new()
///     .with_memory_limit(100 * 1024 * 1024)
///     .with_cpu_limit(0.5)
///     .with_timeout(Duration::from_secs(30))
///     .with_signal_injection(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ChaosConfig {
    /// Maximum memory usage in bytes (0 = unlimited)
    pub memory_limit: usize,
    /// CPU limit as fraction 0.0-1.0 (0.0 = unlimited)
    pub cpu_limit: f64,
    /// Maximum execution timeout
    pub timeout: Duration,
    /// Enable random signal injection for fault testing
    pub signal_injection: bool,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self {
            memory_limit: 0,
            cpu_limit: 0.0,
            timeout: Duration::from_secs(60),
            signal_injection: false,
        }
    }
}

impl ChaosConfig {
    /// Create a new chaos configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set memory limit in bytes
    ///
    /// # Example
    /// ```
    /// # use depyler_core::chaos::ChaosConfig;
    /// let config = ChaosConfig::new().with_memory_limit(512 * 1024 * 1024); // 512 MB
    /// assert_eq!(config.memory_limit, 512 * 1024 * 1024);
    /// ```
    pub fn with_memory_limit(mut self, bytes: usize) -> Self {
        self.memory_limit = bytes;
        self
    }

    /// Set CPU limit as fraction (0.0-1.0, automatically clamped)
    ///
    /// # Example
    /// ```
    /// # use depyler_core::chaos::ChaosConfig;
    /// let config = ChaosConfig::new().with_cpu_limit(0.8); // 80% CPU
    /// assert_eq!(config.cpu_limit, 0.8);
    ///
    /// // Out-of-range values are clamped
    /// let clamped = ChaosConfig::new().with_cpu_limit(1.5);
    /// assert_eq!(clamped.cpu_limit, 1.0);
    /// ```
    pub fn with_cpu_limit(mut self, fraction: f64) -> Self {
        self.cpu_limit = fraction.clamp(0.0, 1.0);
        self
    }

    /// Set execution timeout
    ///
    /// # Example
    /// ```
    /// # use depyler_core::chaos::ChaosConfig;
    /// # use std::time::Duration;
    /// let config = ChaosConfig::new().with_timeout(Duration::from_secs(30));
    /// assert_eq!(config.timeout, Duration::from_secs(30));
    /// ```
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Enable or disable signal injection
    ///
    /// # Example
    /// ```
    /// # use depyler_core::chaos::ChaosConfig;
    /// let config = ChaosConfig::new().with_signal_injection(true);
    /// assert_eq!(config.signal_injection, true);
    /// ```
    pub fn with_signal_injection(mut self, enabled: bool) -> Self {
        self.signal_injection = enabled;
        self
    }

    /// Build the final configuration (consumes self)
    pub fn build(self) -> Self {
        self
    }

    /// Gentle chaos preset for development testing
    ///
    /// - Memory: 512 MB limit
    /// - CPU: 80% throttle
    /// - Timeout: 120 seconds
    /// - Signals: Disabled
    ///
    /// # Example
    /// ```
    /// # use depyler_core::chaos::ChaosConfig;
    /// # use std::time::Duration;
    /// let config = ChaosConfig::gentle();
    /// assert_eq!(config.memory_limit, 512 * 1024 * 1024);
    /// assert_eq!(config.cpu_limit, 0.8);
    /// assert_eq!(config.timeout, Duration::from_secs(120));
    /// assert_eq!(config.signal_injection, false);
    /// ```
    pub fn gentle() -> Self {
        Self::new()
            .with_memory_limit(512 * 1024 * 1024)
            .with_cpu_limit(0.8)
            .with_timeout(Duration::from_secs(120))
    }

    /// Aggressive chaos preset for CI/CD stress testing
    ///
    /// - Memory: 64 MB limit
    /// - CPU: 25% throttle
    /// - Timeout: 10 seconds
    /// - Signals: Enabled
    ///
    /// # Example
    /// ```
    /// # use depyler_core::chaos::ChaosConfig;
    /// # use std::time::Duration;
    /// let config = ChaosConfig::aggressive();
    /// assert_eq!(config.memory_limit, 64 * 1024 * 1024);
    /// assert_eq!(config.cpu_limit, 0.25);
    /// assert_eq!(config.timeout, Duration::from_secs(10));
    /// assert_eq!(config.signal_injection, true);
    /// ```
    pub fn aggressive() -> Self {
        Self::new()
            .with_memory_limit(64 * 1024 * 1024)
            .with_cpu_limit(0.25)
            .with_timeout(Duration::from_secs(10))
            .with_signal_injection(true)
    }
}

/// Result type for chaos testing operations
pub type ChaosResult<T> = Result<T, ChaosError>;

/// Errors that can occur during chaos testing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChaosError {
    /// Memory limit exceeded during test
    MemoryLimitExceeded {
        /// Configured memory limit
        limit: usize,
        /// Actual memory used
        used: usize,
    },
    /// Execution timeout exceeded
    Timeout {
        /// Actual elapsed time
        elapsed: Duration,
        /// Configured timeout limit
        limit: Duration,
    },
    /// Signal injection failed
    SignalInjectionFailed {
        /// Signal number that failed
        signal: i32,
        /// Reason for failure
        reason: String,
    },
}

impl std::fmt::Display for ChaosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChaosError::MemoryLimitExceeded { limit, used } => {
                write!(f, "Memory limit exceeded: {} > {} bytes", used, limit)
            }
            ChaosError::Timeout { elapsed, limit } => {
                write!(f, "Timeout: {:?} > {:?}", elapsed, limit)
            }
            ChaosError::SignalInjectionFailed { signal, reason } => {
                write!(f, "Signal injection failed ({}): {}", signal, reason)
            }
        }
    }
}

impl std::error::Error for ChaosError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ChaosConfig::default();
        assert_eq!(config.memory_limit, 0);
        assert_eq!(config.cpu_limit, 0.0);
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.signal_injection, false);
    }

    #[test]
    fn test_gentle_preset() {
        let config = ChaosConfig::gentle();
        assert_eq!(config.memory_limit, 512 * 1024 * 1024);
        assert_eq!(config.cpu_limit, 0.8);
        assert_eq!(config.timeout, Duration::from_secs(120));
        assert_eq!(config.signal_injection, false);
    }

    #[test]
    fn test_aggressive_preset() {
        let config = ChaosConfig::aggressive();
        assert_eq!(config.memory_limit, 64 * 1024 * 1024);
        assert_eq!(config.cpu_limit, 0.25);
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.signal_injection, true);
    }

    #[test]
    fn test_cpu_limit_clamping() {
        let over = ChaosConfig::new().with_cpu_limit(1.5);
        assert_eq!(over.cpu_limit, 1.0);

        let under = ChaosConfig::new().with_cpu_limit(-0.5);
        assert_eq!(under.cpu_limit, 0.0);

        let valid = ChaosConfig::new().with_cpu_limit(0.75);
        assert_eq!(valid.cpu_limit, 0.75);
    }

    #[test]
    fn test_builder_pattern() {
        let config = ChaosConfig::new()
            .with_memory_limit(100 * 1024 * 1024)
            .with_cpu_limit(0.5)
            .with_timeout(Duration::from_secs(30))
            .with_signal_injection(true)
            .build();

        assert_eq!(config.memory_limit, 100 * 1024 * 1024);
        assert_eq!(config.cpu_limit, 0.5);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.signal_injection, true);
    }

    #[test]
    fn test_chaos_error_display() {
        let mem_err = ChaosError::MemoryLimitExceeded {
            limit: 1000,
            used: 2000,
        };
        assert_eq!(
            mem_err.to_string(),
            "Memory limit exceeded: 2000 > 1000 bytes"
        );

        let timeout_err = ChaosError::Timeout {
            elapsed: Duration::from_secs(5),
            limit: Duration::from_secs(3),
        };
        assert!(timeout_err.to_string().contains("Timeout"));

        let signal_err = ChaosError::SignalInjectionFailed {
            signal: 9,
            reason: "Permission denied".to_string(),
        };
        assert_eq!(
            signal_err.to_string(),
            "Signal injection failed (9): Permission denied"
        );
    }
}
