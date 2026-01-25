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

        let mut by_kind: Vec<(String, u64)> =
            stats.iter().map(|(k, v)| (k.clone(), v.count)).collect();
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
        TypeInferenceTelemetry::global()
            .record_unknown(UnknownTypeEvent::new($expr_kind, format!("{:?}", $expr)));
    }};
    ($expr_kind:expr, $expr:expr, context: $ctx:expr) => {{
        use $crate::type_inference_telemetry::{TypeInferenceTelemetry, UnknownTypeEvent};
        TypeInferenceTelemetry::global().record_unknown(
            UnknownTypeEvent::new($expr_kind, format!("{:?}", $expr)).with_context($ctx),
        );
    }};
    ($expr_kind:expr, $expr:expr, context: $ctx:expr, function: $func:expr) => {{
        use $crate::type_inference_telemetry::{TypeInferenceTelemetry, UnknownTypeEvent};
        TypeInferenceTelemetry::global().record_unknown(
            UnknownTypeEvent::new($expr_kind, format!("{:?}", $expr))
                .with_context($ctx)
                .with_function($func),
        );
    }};
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    // === UnknownTypeEvent tests ===

    #[test]
    fn test_unknown_type_event_new() {
        let event = UnknownTypeEvent::new("Attribute", "obj.field");
        assert_eq!(event.expr_kind, "Attribute");
        assert_eq!(event.expr_repr, "obj.field");
        assert!(event.context.is_none());
        assert!(event.source_location.is_none());
        assert!(event.parent_function.is_none());
        assert!(event.expected_type.is_none());
        assert!(event.timestamp > 0);
    }

    #[test]
    fn test_unknown_type_event_with_context() {
        let event = UnknownTypeEvent::new("Call", "func()").with_context("some_context");
        assert_eq!(event.context, Some("some_context".to_string()));
    }

    #[test]
    fn test_unknown_type_event_with_location() {
        let event = UnknownTypeEvent::new("Call", "func()").with_location("test.py:42");
        assert_eq!(event.source_location, Some("test.py:42".to_string()));
    }

    #[test]
    fn test_unknown_type_event_with_function() {
        let event = UnknownTypeEvent::new("Call", "func()").with_function("main");
        assert_eq!(event.parent_function, Some("main".to_string()));
    }

    #[test]
    fn test_unknown_type_event_with_expected() {
        let event = UnknownTypeEvent::new("Call", "func()").with_expected("int");
        assert_eq!(event.expected_type, Some("int".to_string()));
    }

    #[test]
    fn test_unknown_type_event_builder_chain() {
        let event = UnknownTypeEvent::new("MethodCall", "obj.method()")
            .with_context("method")
            .with_location("file.py:10")
            .with_function("process")
            .with_expected("str");

        assert_eq!(event.expr_kind, "MethodCall");
        assert_eq!(event.context, Some("method".to_string()));
        assert_eq!(event.source_location, Some("file.py:10".to_string()));
        assert_eq!(event.parent_function, Some("process".to_string()));
        assert_eq!(event.expected_type, Some("str".to_string()));
    }

    #[test]
    fn test_unknown_type_event_clone() {
        let event = UnknownTypeEvent::new("Attr", "x.y").with_context("y");
        let cloned = event.clone();
        assert_eq!(event.expr_kind, cloned.expr_kind);
        assert_eq!(event.context, cloned.context);
    }

    #[test]
    fn test_unknown_type_event_debug() {
        let event = UnknownTypeEvent::new("Test", "test_expr");
        let debug = format!("{:?}", event);
        assert!(debug.contains("UnknownTypeEvent"));
        assert!(debug.contains("Test"));
    }

    #[test]
    fn test_unknown_type_event_serialize() {
        let event = UnknownTypeEvent::new("Attribute", "x.y");
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Attribute"));
        assert!(json.contains("x.y"));
    }

    #[test]
    fn test_unknown_type_event_deserialize() {
        let json = r#"{"expr_kind":"Call","expr_repr":"foo()","context":null,"source_location":null,"parent_function":null,"expected_type":null,"timestamp":0}"#;
        let event: UnknownTypeEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.expr_kind, "Call");
        assert_eq!(event.expr_repr, "foo()");
    }

    // === ExprKindStats tests ===

    #[test]
    fn test_expr_kind_stats_default() {
        let stats = ExprKindStats::default();
        assert_eq!(stats.count, 0);
        assert!(stats.unique_contexts.is_empty());
        assert!(stats.sample_exprs.is_empty());
    }

    #[test]
    fn test_expr_kind_stats_clone() {
        let mut stats = ExprKindStats::default();
        stats.count = 5;
        stats.unique_contexts.push("ctx".to_string());
        let cloned = stats.clone();
        assert_eq!(cloned.count, 5);
        assert_eq!(cloned.unique_contexts.len(), 1);
    }

    #[test]
    fn test_expr_kind_stats_debug() {
        let stats = ExprKindStats::default();
        let debug = format!("{:?}", stats);
        assert!(debug.contains("ExprKindStats"));
    }

    #[test]
    fn test_expr_kind_stats_serialize() {
        let stats = ExprKindStats {
            count: 10,
            unique_contexts: vec!["a".to_string()],
            sample_exprs: vec!["expr".to_string()],
        };
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("10"));
    }

    // === TypeInferenceTelemetry tests ===

    #[test]
    fn test_telemetry_new() {
        let telemetry = TypeInferenceTelemetry::new();
        assert!(telemetry.events().is_empty());
        assert!(telemetry.stats().is_empty());
        assert!(telemetry.is_enabled());
    }

    #[test]
    fn test_telemetry_default() {
        let telemetry = TypeInferenceTelemetry::default();
        // Default derives use bool::default() = false for enabled field
        // This differs from new() which explicitly sets enabled = true
        assert!(!telemetry.is_enabled());
    }

    #[test]
    fn test_telemetry_enable_disable() {
        let telemetry = TypeInferenceTelemetry::new();
        assert!(telemetry.is_enabled());

        telemetry.set_enabled(false);
        assert!(!telemetry.is_enabled());

        telemetry.set_enabled(true);
        assert!(telemetry.is_enabled());
    }

    #[test]
    fn test_telemetry_disabled_no_record() {
        let telemetry = TypeInferenceTelemetry::new();
        telemetry.set_enabled(false);

        telemetry.record_unknown(UnknownTypeEvent::new("Test", "x"));

        assert!(telemetry.events().is_empty());
        assert!(telemetry.stats().is_empty());
    }

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
    fn test_record_multiple_same_kind() {
        let telemetry = TypeInferenceTelemetry::new();

        telemetry.record_unknown(UnknownTypeEvent::new("Attr", "a.b"));
        telemetry.record_unknown(UnknownTypeEvent::new("Attr", "c.d"));
        telemetry.record_unknown(UnknownTypeEvent::new("Attr", "e.f"));

        let stats = telemetry.stats();
        assert_eq!(stats.get("Attr").unwrap().count, 3);
        assert_eq!(stats.get("Attr").unwrap().sample_exprs.len(), 3);
    }

    #[test]
    fn test_record_unique_contexts() {
        let telemetry = TypeInferenceTelemetry::new();

        telemetry.record_unknown(UnknownTypeEvent::new("A", "x").with_context("ctx1"));
        telemetry.record_unknown(UnknownTypeEvent::new("A", "y").with_context("ctx2"));
        telemetry.record_unknown(UnknownTypeEvent::new("A", "z").with_context("ctx1")); // dup

        let stats = telemetry.stats();
        let a_stats = stats.get("A").unwrap();
        assert_eq!(a_stats.unique_contexts.len(), 2);
    }

    #[test]
    fn test_sample_exprs_limit() {
        let telemetry = TypeInferenceTelemetry::new();

        for i in 0..15 {
            telemetry.record_unknown(UnknownTypeEvent::new("X", format!("expr{}", i)));
        }

        let stats = telemetry.stats();
        assert_eq!(stats.get("X").unwrap().sample_exprs.len(), 10); // Max 10
    }

    #[test]
    fn test_telemetry_clear() {
        let telemetry = TypeInferenceTelemetry::new();
        telemetry.record_unknown(UnknownTypeEvent::new("A", "x"));
        telemetry.record_unknown(UnknownTypeEvent::new("B", "y"));

        assert!(!telemetry.events().is_empty());
        assert!(!telemetry.stats().is_empty());

        telemetry.clear();

        assert!(telemetry.events().is_empty());
        assert!(telemetry.stats().is_empty());
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
    fn test_summary_empty() {
        let telemetry = TypeInferenceTelemetry::new();
        let summary = telemetry.summary();
        assert_eq!(summary.total_unknowns, 0);
        assert_eq!(summary.unique_expr_kinds, 0);
        assert!(summary.top_unknown_kinds.is_empty());
    }

    #[test]
    fn test_export_json() {
        let telemetry = TypeInferenceTelemetry::new();
        telemetry.record_unknown(UnknownTypeEvent::new("Call", "foo()"));

        let json = telemetry.export_json().unwrap();
        assert!(json.contains("Call"));
        assert!(json.contains("foo()"));
    }

    #[test]
    fn test_export_json_empty() {
        let telemetry = TypeInferenceTelemetry::new();
        let json = telemetry.export_json().unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_export_stats_json() {
        let telemetry = TypeInferenceTelemetry::new();
        telemetry.record_unknown(UnknownTypeEvent::new("Test", "expr"));

        let json = telemetry.export_stats_json().unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_export_stats_json_empty() {
        let telemetry = TypeInferenceTelemetry::new();
        let json = telemetry.export_stats_json().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_telemetry_debug() {
        let telemetry = TypeInferenceTelemetry::new();
        let debug = format!("{:?}", telemetry);
        assert!(debug.contains("TypeInferenceTelemetry"));
    }

    // === TelemetrySummary tests ===

    #[test]
    fn test_telemetry_summary_display() {
        let summary = TelemetrySummary {
            total_unknowns: 100,
            unique_expr_kinds: 5,
            top_unknown_kinds: vec![("Attribute".to_string(), 50), ("Call".to_string(), 30)],
        };

        let display = format!("{}", summary);
        assert!(display.contains("Type Inference Telemetry Summary"));
        assert!(display.contains("100"));
        assert!(display.contains("Attribute"));
        assert!(display.contains("50 occurrences"));
    }

    #[test]
    fn test_telemetry_summary_clone() {
        let summary = TelemetrySummary {
            total_unknowns: 10,
            unique_expr_kinds: 2,
            top_unknown_kinds: vec![("A".to_string(), 5)],
        };
        let cloned = summary.clone();
        assert_eq!(summary.total_unknowns, cloned.total_unknowns);
    }

    #[test]
    fn test_telemetry_summary_debug() {
        let summary = TelemetrySummary {
            total_unknowns: 0,
            unique_expr_kinds: 0,
            top_unknown_kinds: vec![],
        };
        let debug = format!("{:?}", summary);
        assert!(debug.contains("TelemetrySummary"));
    }

    #[test]
    fn test_telemetry_summary_serialize() {
        let summary = TelemetrySummary {
            total_unknowns: 42,
            unique_expr_kinds: 3,
            top_unknown_kinds: vec![("X".to_string(), 20)],
        };
        let json = serde_json::to_string(&summary).unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("X"));
    }

    // === Global instance tests ===

    #[test]
    fn test_global_instance() {
        let global1 = TypeInferenceTelemetry::global();
        let global2 = TypeInferenceTelemetry::global();
        // Both should point to the same instance
        assert!(Arc::ptr_eq(&global1, &global2));
    }

    #[test]
    fn test_reset_global() {
        let global = TypeInferenceTelemetry::global();
        global.record_unknown(UnknownTypeEvent::new("Reset", "test"));

        TypeInferenceTelemetry::reset_global();

        // After reset, events should be cleared
        assert!(global.events().is_empty());
    }

    // === Edge cases ===

    #[test]
    fn test_empty_strings() {
        let event = UnknownTypeEvent::new("", "");
        assert_eq!(event.expr_kind, "");
        assert_eq!(event.expr_repr, "");
    }

    #[test]
    fn test_special_characters() {
        let event = UnknownTypeEvent::new("Test<T>", "obj.method(\"arg\")");
        assert_eq!(event.expr_kind, "Test<T>");
        assert!(event.expr_repr.contains("\"arg\""));
    }
}
