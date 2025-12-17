//! DEPYLER-0970: Type Inference Telemetry for Oracle Training
//!
//! This module captures ALL type inference events where we fall back to Unknown,
//! storing them for oracle training. Every unknown type is a training opportunity.
//!
//! # Philosophy
//!
//! "If you don't measure it, you can't improve it."
//!
//! Silent failures (`_ => Type::Unknown`) are the enemy of progress. This module
//! ensures every unhandled expression type is:
//! 1. Logged at runtime
//! 2. Stored for later analysis
//! 3. Available for oracle training
//!
//! # Usage
//!
//! ```ignore
//! use depyler_core::type_inference_telemetry::{TypeInferenceTelemetry, UnknownTypeEvent};
//!
//! let telemetry = TypeInferenceTelemetry::global();
//! telemetry.record_unknown(UnknownTypeEvent {
//!     expr_kind: "Attribute",
//!     context: "result.returncode",
//!     source_location: Some("task_runner.py:38"),
//!     parent_function: Some("run_command"),
//!     expected_type: None,
//! });
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Global telemetry instance
static GLOBAL_TELEMETRY: OnceLock<Arc<TypeInferenceTelemetry>> = OnceLock::new();

/// Event recorded when type inference returns Unknown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownTypeEvent {
    /// The HIR expression kind (e.g., "Attribute", "MethodCall", "Call")
    pub expr_kind: String,
    /// String representation of the expression
    pub expr_repr: String,
    /// Optional context (e.g., attribute name, method name)
    pub context: Option<String>,
    /// Source file location if available
    pub source_location: Option<String>,
    /// Parent function name if available
    pub parent_function: Option<String>,
    /// What type we expected (if known from context)
    pub expected_type: Option<String>,
    /// Timestamp of the event
    pub timestamp: u64,
}

impl UnknownTypeEvent {
    /// Create a new event
    pub fn new(expr_kind: impl Into<String>, expr_repr: impl Into<String>) -> Self {
        Self {
            expr_kind: expr_kind.into(),
            expr_repr: expr_repr.into(),
            context: None,
            source_location: None,
            parent_function: None,
            expected_type: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    /// Add context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Add source location
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.source_location = Some(location.into());
        self
    }

    /// Add parent function
    pub fn with_function(mut self, function: impl Into<String>) -> Self {
        self.parent_function = Some(function.into());
        self
    }

    /// Add expected type
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected_type = Some(expected.into());
        self
    }
}

/// Statistics for a particular expression kind
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExprKindStats {
    /// Total count of unknown events for this kind
    pub count: u64,
    /// Unique contexts seen
    pub unique_contexts: Vec<String>,
    /// Sample expressions (up to 10)
    pub sample_exprs: Vec<String>,
}

/// Telemetry collector for type inference unknowns
#[derive(Debug, Default)]
pub struct TypeInferenceTelemetry {
    /// All recorded events
    events: Mutex<Vec<UnknownTypeEvent>>,
    /// Aggregated stats by expression kind
    stats: Mutex<HashMap<String, ExprKindStats>>,
    /// Whether logging is enabled
    enabled: Mutex<bool>,
}

impl TypeInferenceTelemetry {
    /// Create a new telemetry collector
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            stats: Mutex::new(HashMap::new()),
            enabled: Mutex::new(true),
        }
    }

    /// Get the global telemetry instance
    pub fn global() -> Arc<TypeInferenceTelemetry> {
        GLOBAL_TELEMETRY
            .get_or_init(|| Arc::new(TypeInferenceTelemetry::new()))
            .clone()
    }

    /// Reset the global instance (useful for tests)
    pub fn reset_global() {
        if let Some(telemetry) = GLOBAL_TELEMETRY.get() {
            telemetry.clear();
        }
    }

    /// Enable or disable telemetry
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock().unwrap() = enabled;
    }

    /// Check if telemetry is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.lock().unwrap()
    }

    /// Record an unknown type event
    pub fn record_unknown(&self, event: UnknownTypeEvent) {
        if !self.is_enabled() {
            return;
        }

        // Log at debug level
        tracing::debug!(
            expr_kind = %event.expr_kind,
            expr = %event.expr_repr,
            context = ?event.context,
            location = ?event.source_location,
            function = ?event.parent_function,
            "Type inference returned Unknown"
        );

        // Update stats
        {
            let mut stats = self.stats.lock().unwrap();
            let entry = stats.entry(event.expr_kind.clone()).or_default();
            entry.count += 1;

            if let Some(ref ctx) = event.context {
                if !entry.unique_contexts.contains(ctx) && entry.unique_contexts.len() < 100 {
                    entry.unique_contexts.push(ctx.clone());
                }
            }

            if entry.sample_exprs.len() < 10 {
                entry.sample_exprs.push(event.expr_repr.clone());
            }
        }

        // Store event
        {
            let mut events = self.events.lock().unwrap();
            // Keep last 10000 events
            if events.len() >= 10000 {
                events.remove(0);
            }
            events.push(event);
        }
    }

    /// Get all events
    pub fn events(&self) -> Vec<UnknownTypeEvent> {
        self.events.lock().unwrap().clone()
    }

    /// Get stats by expression kind
    pub fn stats(&self) -> HashMap<String, ExprKindStats> {
        self.stats.lock().unwrap().clone()
    }

    /// Get summary report
    pub fn summary(&self) -> TelemetrySummary {
        let stats = self.stats.lock().unwrap();
        let events = self.events.lock().unwrap();

        let mut by_kind: Vec<(String, u64)> = stats
            .iter()
            .map(|(k, v)| (k.clone(), v.count))
            .collect();
        by_kind.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending

        TelemetrySummary {
            total_unknowns: events.len() as u64,
            unique_expr_kinds: stats.len(),
            top_unknown_kinds: by_kind.into_iter().take(10).collect(),
        }
    }

    /// Clear all recorded data
    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
        self.stats.lock().unwrap().clear();
    }

    /// Export events to JSON for oracle training
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        let events = self.events.lock().unwrap();
        serde_json::to_string_pretty(&*events)
    }

    /// Export stats to JSON
    pub fn export_stats_json(&self) -> Result<String, serde_json::Error> {
        let stats = self.stats.lock().unwrap();
        serde_json::to_string_pretty(&*stats)
    }
}

/// Summary of telemetry data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySummary {
    /// Total unknown type events
    pub total_unknowns: u64,
    /// Number of unique expression kinds that returned unknown
    pub unique_expr_kinds: usize,
    /// Top expression kinds by frequency
    pub top_unknown_kinds: Vec<(String, u64)>,
}

impl std::fmt::Display for TelemetrySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Type Inference Telemetry Summary")?;
        writeln!(f, "================================")?;
        writeln!(f, "Total Unknown Events: {}", self.total_unknowns)?;
        writeln!(f, "Unique Expression Kinds: {}", self.unique_expr_kinds)?;
        writeln!(f)?;
        writeln!(f, "Top Unknown Expression Kinds:")?;
        for (kind, count) in &self.top_unknown_kinds {
            writeln!(f, "  {}: {} occurrences", kind, count)?;
        }
        Ok(())
    }
}

/// Helper macro to record unknown type with minimal boilerplate
#[macro_export]
macro_rules! record_unknown_type {
    ($expr_kind:expr, $expr:expr) => {{
        use $crate::type_inference_telemetry::{TypeInferenceTelemetry, UnknownTypeEvent};
        TypeInferenceTelemetry::global().record_unknown(
            UnknownTypeEvent::new($expr_kind, format!("{:?}", $expr))
        );
    }};
    ($expr_kind:expr, $expr:expr, context: $ctx:expr) => {{
        use $crate::type_inference_telemetry::{TypeInferenceTelemetry, UnknownTypeEvent};
        TypeInferenceTelemetry::global().record_unknown(
            UnknownTypeEvent::new($expr_kind, format!("{:?}", $expr))
                .with_context($ctx)
        );
    }};
    ($expr_kind:expr, $expr:expr, context: $ctx:expr, function: $func:expr) => {{
        use $crate::type_inference_telemetry::{TypeInferenceTelemetry, UnknownTypeEvent};
        TypeInferenceTelemetry::global().record_unknown(
            UnknownTypeEvent::new($expr_kind, format!("{:?}", $expr))
                .with_context($ctx)
                .with_function($func)
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_unknown() {
        let telemetry = TypeInferenceTelemetry::new();

        telemetry.record_unknown(
            UnknownTypeEvent::new("Attribute", "result.returncode")
                .with_context("returncode")
                .with_function("run_command"),
        );

        let events = telemetry.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].expr_kind, "Attribute");

        let stats = telemetry.stats();
        assert_eq!(stats.get("Attribute").unwrap().count, 1);
    }

    #[test]
    fn test_summary() {
        let telemetry = TypeInferenceTelemetry::new();

        for _ in 0..5 {
            telemetry.record_unknown(UnknownTypeEvent::new("Attribute", "x.y"));
        }
        for _ in 0..3 {
            telemetry.record_unknown(UnknownTypeEvent::new("MethodCall", "x.foo()"));
        }

        let summary = telemetry.summary();
        assert_eq!(summary.total_unknowns, 8);
        assert_eq!(summary.unique_expr_kinds, 2);
        assert_eq!(summary.top_unknown_kinds[0], ("Attribute".to_string(), 5));
    }

    #[test]
    fn test_export_json() {
        let telemetry = TypeInferenceTelemetry::new();
        telemetry.record_unknown(UnknownTypeEvent::new("Call", "foo()"));

        let json = telemetry.export_json().unwrap();
        assert!(json.contains("Call"));
        assert!(json.contains("foo()"));
    }
}
