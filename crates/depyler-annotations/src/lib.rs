#![allow(clippy::missing_errors_doc)] // Parse methods have obvious error conditions

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnnotationError {
    #[error("Invalid annotation syntax: {0}")]
    InvalidSyntax(String),
    #[error("Unknown annotation key: {0}")]
    UnknownKey(String),
    #[error("Invalid value for key {key}: {value}")]
    InvalidValue { key: String, value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)] // Configuration struct requires multiple boolean flags
pub struct TranspilationAnnotations {
    pub type_strategy: TypeStrategy,
    pub ownership_model: OwnershipModel,
    pub safety_level: SafetyLevel,
    pub performance_hints: Vec<PerformanceHint>,
    pub fallback_strategy: FallbackStrategy,
    pub bounds_checking: BoundsChecking,
    pub optimization_level: OptimizationLevel,
    pub thread_safety: ThreadSafety,
    pub interior_mutability: InteriorMutability,
    pub string_strategy: StringStrategy,
    pub hash_strategy: HashStrategy,
    pub panic_behavior: PanicBehavior,
    pub error_strategy: ErrorStrategy,
    pub global_strategy: GlobalStrategy,
    pub termination: Termination,
    pub invariants: Vec<String>,
    pub verify_bounds: bool,
    pub service_type: Option<ServiceType>,
    pub migration_strategy: Option<MigrationStrategy>,
    pub compatibility_layer: Option<CompatibilityLayer>,
    pub pattern: Option<String>,
    // Lambda-specific annotations
    pub lambda_annotations: Option<LambdaAnnotations>,
    pub custom_attributes: Vec<String>,
}

impl Default for TranspilationAnnotations {
    fn default() -> Self {
        Self {
            type_strategy: TypeStrategy::Conservative,
            ownership_model: OwnershipModel::Owned,
            safety_level: SafetyLevel::Safe,
            performance_hints: Vec::new(),
            fallback_strategy: FallbackStrategy::Error,
            bounds_checking: BoundsChecking::Explicit,
            optimization_level: OptimizationLevel::Standard,
            thread_safety: ThreadSafety::NotRequired,
            interior_mutability: InteriorMutability::None,
            string_strategy: StringStrategy::Conservative,
            hash_strategy: HashStrategy::Standard,
            panic_behavior: PanicBehavior::Propagate,
            error_strategy: ErrorStrategy::Panic,
            global_strategy: GlobalStrategy::None,
            termination: Termination::Unknown,
            invariants: Vec::new(),
            verify_bounds: false,
            service_type: None,
            migration_strategy: None,
            compatibility_layer: None,
            pattern: None,
            lambda_annotations: None,
            custom_attributes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)] // Lambda configuration requires many boolean flags
pub struct LambdaAnnotations {
    pub runtime: LambdaRuntime,
    pub event_type: Option<LambdaEventType>,
    pub cold_start_optimize: bool,
    pub memory_size: u16,
    pub architecture: Architecture,
    pub pre_warm_paths: Vec<String>,
    pub custom_serialization: bool,
    pub batch_failure_reporting: bool,
    pub timeout: Option<u16>,
    pub tracing_enabled: bool,
    pub environment_variables: Vec<(String, String)>,
}

impl Default for LambdaAnnotations {
    fn default() -> Self {
        Self {
            runtime: LambdaRuntime::ProvidedAl2,
            event_type: None,
            cold_start_optimize: true,
            memory_size: 128,
            architecture: Architecture::Arm64,
            pre_warm_paths: vec![],
            custom_serialization: false,
            batch_failure_reporting: false,
            timeout: None,
            tracing_enabled: false,
            environment_variables: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LambdaRuntime {
    ProvidedAl2,
    ProvidedAl2023,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LambdaEventType {
    Auto,
    S3Event,
    ApiGatewayProxyRequest,
    ApiGatewayV2HttpRequest,
    SqsEvent,
    SnsEvent,
    DynamodbEvent,
    EventBridgeEvent(Option<String>),
    CloudwatchEvent,
    KinesisEvent,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    X86_64,
    Arm64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeStrategy {
    Conservative,
    Aggressive,
    ZeroCopy,
    AlwaysOwned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipModel {
    Owned,
    Borrowed,
    Shared,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    UnsafeAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceHint {
    Vectorize,
    UnrollLoops(u32),
    OptimizeForLatency,
    OptimizeForThroughput,
    PerformanceCritical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallbackStrategy {
    Mcp,
    Manual,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundsChecking {
    Explicit,
    Implicit,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Standard,
    Aggressive,
    Conservative,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadSafety {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteriorMutability {
    None,
    ArcMutex,
    RefCell,
    Cell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StringStrategy {
    Conservative,
    AlwaysOwned,
    ZeroCopy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashStrategy {
    Standard,
    Fnv,
    AHash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanicBehavior {
    Propagate,
    ReturnError,
    Abort,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorStrategy {
    Panic,
    ResultType,
    OptionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlobalStrategy {
    None,
    LazyStatic,
    OnceCell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Termination {
    Unknown,
    Proven,
    BoundedLoop(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    WebApi,
    Cli,
    Library,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStrategy {
    Incremental,
    BigBang,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityLayer {
    PyO3,
    CTypes,
    None,
}

pub struct AnnotationParser {
    pattern: Regex,
}

#[derive(Debug, Clone, Default)]
pub struct AnnotationValidator;

impl AnnotationValidator {
    /// Creates a new annotation validator.
    pub fn new() -> Self {
        Self
    }

    /// Validates the consistency of annotation settings.
    ///
    /// # Errors
    ///
    /// Returns a vector of error messages if any validation rules are violated.
    pub fn validate(&self, annotations: &TranspilationAnnotations) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate conflicting strategies
        if annotations.string_strategy == StringStrategy::ZeroCopy
            && annotations.ownership_model == OwnershipModel::Owned
        {
            errors
                .push("Zero-copy string strategy conflicts with owned ownership model".to_string());
        }

        if annotations.thread_safety == ThreadSafety::Required
            && annotations.interior_mutability == InteriorMutability::RefCell
        {
            errors.push("RefCell is not thread-safe, use Arc<Mutex<T>> instead".to_string());
        }

        if annotations.panic_behavior == PanicBehavior::ReturnError
            && annotations.error_strategy == ErrorStrategy::Panic
        {
            errors.push("Conflicting panic behavior and error strategy".to_string());
        }

        if annotations.optimization_level == OptimizationLevel::Aggressive
            && annotations.bounds_checking == BoundsChecking::Explicit
        {
            errors.push(
                "Aggressive optimization may conflict with explicit bounds checking".to_string(),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn suggest_improvements(&self, annotations: &TranspilationAnnotations) -> Vec<String> {
        let mut suggestions = Vec::new();

        if annotations
            .performance_hints
            .contains(&PerformanceHint::PerformanceCritical)
            && annotations.optimization_level != OptimizationLevel::Aggressive
        {
            suggestions.push(
                "Consider using optimization_level = \"aggressive\" for performance critical code"
                    .to_string(),
            );
        }

        if annotations.thread_safety == ThreadSafety::Required
            && annotations.ownership_model != OwnershipModel::Shared
        {
            suggestions
                .push("Consider using ownership = \"shared\" for thread-safe code".to_string());
        }

        if annotations.service_type == Some(ServiceType::WebApi)
            && !annotations
                .performance_hints
                .contains(&PerformanceHint::OptimizeForLatency)
        {
            suggestions
                .push("Consider adding optimization_hint = \"latency\" for web APIs".to_string());
        }

        suggestions
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationExtractor {
    function_pattern: Regex,
    class_pattern: Regex,
}

impl Default for AnnotationExtractor {
    fn default() -> Self {
        Self {
            function_pattern: Regex::new(r"(?m)^def\s+(\w+)\s*\(").expect("static regex"),
            class_pattern: Regex::new(r"(?m)^class\s+(\w+)\s*[\(:]").expect("static regex"),
        }
    }
}

impl AnnotationExtractor {
    /// Creates a new annotation extractor with pre-compiled regex patterns.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts annotations for a specific function from source code.
    ///
    /// # Panics
    ///
    /// Panics if regex patterns fail to match (should not happen with valid regex).
    pub fn extract_function_annotations(
        &self,
        source: &str,
        function_name: &str,
    ) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if let Some(captures) = self.function_pattern.captures(line) {
                if captures.get(1).expect("capture group 1 exists").as_str() == function_name {
                    // Collect annotations above the function
                    let mut annotations = Vec::new();
                    let mut j = i.saturating_sub(1);

                    while j < i && (lines[j].trim().starts_with('#') || lines[j].trim().is_empty())
                    {
                        if lines[j].contains("@depyler:") {
                            annotations.push(lines[j]);
                        }
                        if j == 0 {
                            break;
                        }
                        j = j.saturating_sub(1);
                    }

                    if !annotations.is_empty() {
                        annotations.reverse();
                        return Some(annotations.join("\n"));
                    }
                }
            }
        }
        None
    }

    /// Extracts annotations for a specific class from source code.
    ///
    /// # Panics
    ///
    /// Panics if regex patterns fail to match (should not happen with valid regex).
    pub fn extract_class_annotations(&self, source: &str, class_name: &str) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if let Some(captures) = self.class_pattern.captures(line) {
                if captures.get(1).expect("capture group 1 exists").as_str() == class_name {
                    // Collect annotations above the class
                    let mut annotations = Vec::new();
                    let mut j = i.saturating_sub(1);

                    while j < i && (lines[j].trim().starts_with('#') || lines[j].trim().is_empty())
                    {
                        if lines[j].contains("@depyler:") {
                            annotations.push(lines[j]);
                        }
                        if j == 0 {
                            break;
                        }
                        j = j.saturating_sub(1);
                    }

                    if !annotations.is_empty() {
                        annotations.reverse();
                        return Some(annotations.join("\n"));
                    }
                }
            }
        }
        None
    }
}

impl Default for AnnotationParser {
    fn default() -> Self {
        Self::new()
    }
}

impl AnnotationParser {
    /// Creates a new annotation parser.
    ///
    /// # Panics
    ///
    /// Panics if the internal regex pattern fails to compile (should never happen).
    pub fn new() -> Self {
        let pattern =
            // This regex is statically known to be valid
            Regex::new(r"#\s*@depyler:\s*(\w+)\s*=\s*(.+)")
                .unwrap_or_else(|e| panic!("Failed to compile annotation regex: {e}"));
        Self { pattern }
    }

    /// Parses annotations from source code comments.
    ///
    /// # Errors
    ///
    /// Returns `AnnotationError` if unknown keys or invalid values are encountered.
    ///
    /// # Panics
    ///
    /// Panics if the regex fails to capture groups (should not happen with valid regex).
    pub fn parse_annotations(
        &self,
        source: &str,
    ) -> Result<TranspilationAnnotations, AnnotationError> {
        let mut annotations = TranspilationAnnotations::default();
        let mut parsed_values: HashMap<String, String> = HashMap::new();

        for line in source.lines() {
            if let Some(captures) = self.pattern.captures(line) {
                let key = captures
                    .get(1)
                    .expect("capture group 1 exists")
                    .as_str()
                    .to_string();
                let value = captures
                    .get(2)
                    .expect("capture group 2 exists")
                    .as_str()
                    .trim_matches('"')
                    .trim();

                // Special handling for custom_attribute - accumulate instead of replace
                if key == "custom_attribute" {
                    annotations.custom_attributes.push(value.to_string());
                } else {
                    parsed_values.insert(key, value.to_string());
                }
            }
        }

        self.apply_annotations(&mut annotations, parsed_values)?;
        Ok(annotations)
    }

    /// Parses annotations from function-specific source code.
    ///
    /// # Errors
    ///
    /// Returns `AnnotationError` if parsing fails.
    pub fn parse_function_annotations(
        &self,
        function_source: &str,
    ) -> Result<TranspilationAnnotations, AnnotationError> {
        self.parse_annotations(function_source)
    }

    fn apply_annotations(
        &self,
        annotations: &mut TranspilationAnnotations,
        values: HashMap<String, String>,
    ) -> Result<(), AnnotationError> {
        for (key, value) in values {
            // Dispatch to category handlers
            match key.as_str() {
                // Core annotations (5)
                "type_strategy" | "ownership" | "safety_level" | "fallback" | "bounds_checking" => {
                    self.apply_core_annotation(annotations, &key, &value)?;
                }

                // Optimization annotations (5)
                "optimization_level"
                | "performance_critical"
                | "vectorize"
                | "unroll_loops"
                | "optimization_hint" => {
                    self.apply_optimization_annotation(annotations, &key, &value)?;
                }

                // Thread safety annotations (2)
                "thread_safety" | "interior_mutability" => {
                    self.apply_thread_safety_annotation(annotations, &key, &value)?;
                }

                // String/Hash strategy (2)
                "string_strategy" | "hash_strategy" => {
                    self.apply_string_hash_annotation(annotations, &key, &value)?;
                }

                // Error handling (2)
                "panic_behavior" | "error_strategy" => {
                    self.apply_error_handling_annotation(annotations, &key, &value)?;
                }

                // Global strategy (1)
                "global_strategy" => {
                    self.apply_global_strategy_annotation(annotations, &value)?;
                }

                // Verification (3)
                "termination" | "invariant" | "verify_bounds" => {
                    self.apply_verification_annotation(annotations, &key, &value)?;
                }

                // Service metadata (4)
                "service_type" | "migration_strategy" | "compatibility_layer" | "pattern" => {
                    self.apply_service_metadata_annotation(annotations, &key, &value)?;
                }

                // Lambda-specific annotations (9)
                "lambda_runtime"
                | "event_type"
                | "cold_start_optimize"
                | "memory_size"
                | "architecture"
                | "batch_failure_reporting"
                | "custom_serialization"
                | "timeout"
                | "tracing" => {
                    self.apply_lambda_annotation(annotations, &key, &value)?;
                }

                _ => return Err(AnnotationError::UnknownKey(key)),
            }
        }
        Ok(())
    }

    /// Apply core annotation (type_strategy, ownership, safety_level, fallback, bounds_checking)
    #[inline]
    fn apply_core_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "type_strategy" => {
                annotations.type_strategy = self.parse_type_strategy(value)?;
            }
            "ownership" => {
                annotations.ownership_model = self.parse_ownership_model(value)?;
            }
            "safety_level" => {
                annotations.safety_level = self.parse_safety_level(value)?;
            }
            "fallback" => {
                annotations.fallback_strategy = self.parse_fallback_strategy(value)?;
            }
            "bounds_checking" => {
                annotations.bounds_checking = self.parse_bounds_checking(value)?;
            }
            _ => unreachable!("apply_core_annotation called with non-core key"),
        }
        Ok(())
    }

    /// Apply optimization annotation (optimization_level, performance_critical, vectorize, unroll_loops, optimization_hint)
    #[inline]
    fn apply_optimization_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "optimization_level" => {
                annotations.optimization_level = self.parse_optimization_level(value)?;
            }
            "performance_critical" => {
                if value == "true" {
                    annotations
                        .performance_hints
                        .push(PerformanceHint::PerformanceCritical);
                }
            }
            "vectorize" => {
                if value == "true" {
                    annotations
                        .performance_hints
                        .push(PerformanceHint::Vectorize);
                }
            }
            "unroll_loops" => {
                let count: u32 = value.parse().map_err(|_| AnnotationError::InvalidValue {
                    key: key.to_string(),
                    value: value.to_string(),
                })?;
                annotations
                    .performance_hints
                    .push(PerformanceHint::UnrollLoops(count));
            }
            "optimization_hint" => {
                self.apply_optimization_hint(annotations, value)?;
            }
            _ => unreachable!("apply_optimization_annotation called with non-optimization key"),
        }
        Ok(())
    }

    /// Apply optimization hint sub-handler
    #[inline]
    fn apply_optimization_hint(
        &self,
        annotations: &mut TranspilationAnnotations,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match value {
            "vectorize" => annotations
                .performance_hints
                .push(PerformanceHint::Vectorize),
            "latency" => annotations
                .performance_hints
                .push(PerformanceHint::OptimizeForLatency),
            "throughput" => annotations
                .performance_hints
                .push(PerformanceHint::OptimizeForThroughput),
            "async_ready" => {
                eprintln!("Warning: async_ready is experimental and not yet fully supported");
            }
            _ => {
                return Err(AnnotationError::InvalidValue {
                    key: "optimization_hint".to_string(),
                    value: value.to_string(),
                })
            }
        }
        Ok(())
    }

    /// Apply thread safety annotation (thread_safety, interior_mutability)
    #[inline]
    fn apply_thread_safety_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "thread_safety" => {
                annotations.thread_safety = self.parse_thread_safety(value)?;
            }
            "interior_mutability" => {
                annotations.interior_mutability = self.parse_interior_mutability(value)?;
            }
            _ => unreachable!("apply_thread_safety_annotation called with non-thread-safety key"),
        }
        Ok(())
    }

    /// Apply global strategy annotation
    #[inline]
    fn apply_global_strategy_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        value: &str,
    ) -> Result<(), AnnotationError> {
        annotations.global_strategy = self.parse_global_strategy(value)?;
        Ok(())
    }

    /// Apply string/hash strategy annotation (string_strategy, hash_strategy)
    #[inline]
    fn apply_string_hash_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "string_strategy" => {
                annotations.string_strategy = self.parse_string_strategy(value)?;
            }
            "hash_strategy" => {
                annotations.hash_strategy = self.parse_hash_strategy(value)?;
            }
            _ => unreachable!("apply_string_hash_annotation called with non-string/hash key"),
        }
        Ok(())
    }

    /// Apply error handling annotation (panic_behavior, error_strategy)
    #[inline]
    fn apply_error_handling_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "panic_behavior" => {
                annotations.panic_behavior = self.parse_panic_behavior(value)?;
            }
            "error_strategy" => {
                annotations.error_strategy = self.parse_error_strategy(value)?;
            }
            _ => unreachable!("apply_error_handling_annotation called with non-error key"),
        }
        Ok(())
    }

    /// Apply verification annotation (termination, invariant, verify_bounds)
    #[inline]
    fn apply_verification_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "termination" => {
                annotations.termination = self.parse_termination(value)?;
            }
            "invariant" => {
                annotations.invariants.push(value.to_string());
            }
            "verify_bounds" => {
                annotations.verify_bounds = value == "true";
            }
            _ => unreachable!("apply_verification_annotation called with non-verification key"),
        }
        Ok(())
    }

    /// Apply service metadata annotation (service_type, migration_strategy, compatibility_layer, pattern)
    #[inline]
    fn apply_service_metadata_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "service_type" => {
                annotations.service_type = Some(self.parse_service_type(value)?);
            }
            "migration_strategy" => {
                annotations.migration_strategy = Some(self.parse_migration_strategy(value)?);
            }
            "compatibility_layer" => {
                annotations.compatibility_layer = Some(self.parse_compatibility_layer(value)?);
            }
            "pattern" => {
                annotations.pattern = Some(value.to_string());
            }
            _ => unreachable!("apply_service_metadata_annotation called with non-service key"),
        }
        Ok(())
    }

    /// Apply lambda-specific annotation (9 lambda keys) - dispatcher with ≤10 complexity
    #[inline]
    fn apply_lambda_annotation(
        &self,
        annotations: &mut TranspilationAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        let lambda_annotations = annotations
            .lambda_annotations
            .get_or_insert_with(LambdaAnnotations::default);

        match key {
            "lambda_runtime" | "event_type" | "architecture" => {
                self.apply_lambda_config(lambda_annotations, key, value)?;
            }
            "cold_start_optimize"
            | "batch_failure_reporting"
            | "custom_serialization"
            | "tracing" => {
                self.apply_lambda_flags(lambda_annotations, key, value);
            }
            "memory_size" | "timeout" => {
                self.apply_lambda_numeric(lambda_annotations, key, value)?;
            }
            _ => unreachable!("apply_lambda_annotation called with non-lambda key"),
        }
        Ok(())
    }

    /// Apply lambda configuration (runtime, event_type, architecture)
    #[inline]
    fn apply_lambda_config(
        &self,
        lambda_annotations: &mut LambdaAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "lambda_runtime" => {
                lambda_annotations.runtime = self.parse_lambda_runtime(value)?;
            }
            "event_type" => {
                lambda_annotations.event_type = Some(self.parse_lambda_event_type(value)?);
            }
            "architecture" => {
                lambda_annotations.architecture = self.parse_architecture(value)?;
            }
            _ => unreachable!("apply_lambda_config called with non-config key"),
        }
        Ok(())
    }

    /// Apply lambda feature flags (cold_start_optimize, batch_failure_reporting, custom_serialization, tracing)
    #[inline]
    fn apply_lambda_flags(
        &self,
        lambda_annotations: &mut LambdaAnnotations,
        key: &str,
        value: &str,
    ) {
        match key {
            "cold_start_optimize" => {
                lambda_annotations.cold_start_optimize = value == "true";
            }
            "batch_failure_reporting" => {
                lambda_annotations.batch_failure_reporting = value == "true";
            }
            "custom_serialization" => {
                lambda_annotations.custom_serialization = value == "true";
            }
            "tracing" => {
                lambda_annotations.tracing_enabled = value == "true" || value == "Active";
            }
            _ => unreachable!("apply_lambda_flags called with non-flag key"),
        }
    }

    /// Apply lambda numeric settings (memory_size, timeout)
    #[inline]
    fn apply_lambda_numeric(
        &self,
        lambda_annotations: &mut LambdaAnnotations,
        key: &str,
        value: &str,
    ) -> Result<(), AnnotationError> {
        match key {
            "memory_size" => {
                lambda_annotations.memory_size =
                    value.parse().map_err(|_| AnnotationError::InvalidValue {
                        key: key.to_string(),
                        value: value.to_string(),
                    })?;
            }
            "timeout" => {
                lambda_annotations.timeout =
                    Some(value.parse().map_err(|_| AnnotationError::InvalidValue {
                        key: key.to_string(),
                        value: value.to_string(),
                    })?);
            }
            _ => unreachable!("apply_lambda_numeric called with non-numeric key"),
        }
        Ok(())
    }

    fn parse_type_strategy(&self, value: &str) -> Result<TypeStrategy, AnnotationError> {
        match value {
            "conservative" => Ok(TypeStrategy::Conservative),
            "aggressive" => Ok(TypeStrategy::Aggressive),
            "zero_copy" => Ok(TypeStrategy::ZeroCopy),
            "always_owned" => Ok(TypeStrategy::AlwaysOwned),
            _ => Err(AnnotationError::InvalidValue {
                key: "type_strategy".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_ownership_model(&self, value: &str) -> Result<OwnershipModel, AnnotationError> {
        match value {
            "owned" => Ok(OwnershipModel::Owned),
            "borrowed" => Ok(OwnershipModel::Borrowed),
            "shared" => Ok(OwnershipModel::Shared),
            _ => Err(AnnotationError::InvalidValue {
                key: "ownership".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_safety_level(&self, value: &str) -> Result<SafetyLevel, AnnotationError> {
        match value {
            "safe" => Ok(SafetyLevel::Safe),
            "unsafe_allowed" => Ok(SafetyLevel::UnsafeAllowed),
            _ => Err(AnnotationError::InvalidValue {
                key: "safety_level".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_fallback_strategy(&self, value: &str) -> Result<FallbackStrategy, AnnotationError> {
        match value {
            "mcp" => Ok(FallbackStrategy::Mcp),
            "manual" => Ok(FallbackStrategy::Manual),
            "error" => Ok(FallbackStrategy::Error),
            _ => Err(AnnotationError::InvalidValue {
                key: "fallback".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_bounds_checking(&self, value: &str) -> Result<BoundsChecking, AnnotationError> {
        match value {
            "explicit" => Ok(BoundsChecking::Explicit),
            "implicit" => Ok(BoundsChecking::Implicit),
            "disabled" => Ok(BoundsChecking::Disabled),
            _ => Err(AnnotationError::InvalidValue {
                key: "bounds_checking".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_optimization_level(&self, value: &str) -> Result<OptimizationLevel, AnnotationError> {
        match value {
            "standard" => Ok(OptimizationLevel::Standard),
            "aggressive" => Ok(OptimizationLevel::Aggressive),
            "conservative" => Ok(OptimizationLevel::Conservative),
            _ => Err(AnnotationError::InvalidValue {
                key: "optimization_level".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_thread_safety(&self, value: &str) -> Result<ThreadSafety, AnnotationError> {
        match value {
            "required" => Ok(ThreadSafety::Required),
            "not_required" => Ok(ThreadSafety::NotRequired),
            _ => Err(AnnotationError::InvalidValue {
                key: "thread_safety".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_interior_mutability(
        &self,
        value: &str,
    ) -> Result<InteriorMutability, AnnotationError> {
        match value {
            "none" => Ok(InteriorMutability::None),
            "arc_mutex" => Ok(InteriorMutability::ArcMutex),
            "ref_cell" => Ok(InteriorMutability::RefCell),
            "cell" => Ok(InteriorMutability::Cell),
            _ => Err(AnnotationError::InvalidValue {
                key: "interior_mutability".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_string_strategy(&self, value: &str) -> Result<StringStrategy, AnnotationError> {
        match value {
            "conservative" => Ok(StringStrategy::Conservative),
            "always_owned" => Ok(StringStrategy::AlwaysOwned),
            "zero_copy" => Ok(StringStrategy::ZeroCopy),
            _ => Err(AnnotationError::InvalidValue {
                key: "string_strategy".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_hash_strategy(&self, value: &str) -> Result<HashStrategy, AnnotationError> {
        match value {
            "standard" => Ok(HashStrategy::Standard),
            "fnv" => Ok(HashStrategy::Fnv),
            "ahash" => Ok(HashStrategy::AHash),
            _ => Err(AnnotationError::InvalidValue {
                key: "hash_strategy".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_panic_behavior(&self, value: &str) -> Result<PanicBehavior, AnnotationError> {
        match value {
            "propagate" => Ok(PanicBehavior::Propagate),
            "return_error" => Ok(PanicBehavior::ReturnError),
            "abort" => Ok(PanicBehavior::Abort),
            _ => Err(AnnotationError::InvalidValue {
                key: "panic_behavior".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_error_strategy(&self, value: &str) -> Result<ErrorStrategy, AnnotationError> {
        match value {
            "panic" => Ok(ErrorStrategy::Panic),
            "result_type" => Ok(ErrorStrategy::ResultType),
            "option_type" => Ok(ErrorStrategy::OptionType),
            _ => Err(AnnotationError::InvalidValue {
                key: "error_strategy".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_global_strategy(&self, value: &str) -> Result<GlobalStrategy, AnnotationError> {
        match value {
            "none" => Ok(GlobalStrategy::None),
            "lazy_static" => Ok(GlobalStrategy::LazyStatic),
            "once_cell" => Ok(GlobalStrategy::OnceCell),
            _ => Err(AnnotationError::InvalidValue {
                key: "global_strategy".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_termination(&self, value: &str) -> Result<Termination, AnnotationError> {
        match value {
            "unknown" => Ok(Termination::Unknown),
            "proven" => Ok(Termination::Proven),
            _ => {
                if value.starts_with("bounded_") {
                    if let Some(num_str) = value.strip_prefix("bounded_") {
                        if let Ok(bound) = num_str.parse::<u32>() {
                            return Ok(Termination::BoundedLoop(bound));
                        }
                    }
                }
                Err(AnnotationError::InvalidValue {
                    key: "termination".to_string(),
                    value: value.to_string(),
                })
            }
        }
    }

    fn parse_service_type(&self, value: &str) -> Result<ServiceType, AnnotationError> {
        match value {
            "web_api" => Ok(ServiceType::WebApi),
            "cli" => Ok(ServiceType::Cli),
            "library" => Ok(ServiceType::Library),
            _ => Err(AnnotationError::InvalidValue {
                key: "service_type".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_migration_strategy(&self, value: &str) -> Result<MigrationStrategy, AnnotationError> {
        match value {
            "incremental" => Ok(MigrationStrategy::Incremental),
            "big_bang" => Ok(MigrationStrategy::BigBang),
            "hybrid" => Ok(MigrationStrategy::Hybrid),
            _ => Err(AnnotationError::InvalidValue {
                key: "migration_strategy".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_compatibility_layer(
        &self,
        value: &str,
    ) -> Result<CompatibilityLayer, AnnotationError> {
        match value {
            "pyo3" => Ok(CompatibilityLayer::PyO3),
            "ctypes" => Ok(CompatibilityLayer::CTypes),
            "none" => Ok(CompatibilityLayer::None),
            _ => Err(AnnotationError::InvalidValue {
                key: "compatibility_layer".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn parse_lambda_runtime(&self, value: &str) -> Result<LambdaRuntime, AnnotationError> {
        match value {
            "provided.al2" => Ok(LambdaRuntime::ProvidedAl2),
            "provided.al2023" => Ok(LambdaRuntime::ProvidedAl2023),
            _ => Ok(LambdaRuntime::Custom(value.to_string())),
        }
    }

    fn parse_lambda_event_type(&self, value: &str) -> Result<LambdaEventType, AnnotationError> {
        // Quick path for common types
        let event_type = match value {
            "auto" => LambdaEventType::Auto,
            "S3Event" | "SqsEvent" | "SnsEvent" | "DynamodbEvent" | "CloudwatchEvent"
            | "KinesisEvent" => self.parse_aws_service_event(value),
            "APIGatewayProxyRequest" | "APIGatewayV2HttpRequest" => {
                self.parse_api_gateway_event(value)
            }
            _ => self.parse_custom_event_type(value),
        };
        Ok(event_type)
    }

    /// Parse AWS service events (S3, SQS, SNS, DynamoDB, CloudWatch, Kinesis)
    #[inline]
    fn parse_aws_service_event(&self, value: &str) -> LambdaEventType {
        match value {
            "S3Event" => LambdaEventType::S3Event,
            "SqsEvent" => LambdaEventType::SqsEvent,
            "SnsEvent" => LambdaEventType::SnsEvent,
            "DynamodbEvent" => LambdaEventType::DynamodbEvent,
            "CloudwatchEvent" => LambdaEventType::CloudwatchEvent,
            "KinesisEvent" => LambdaEventType::KinesisEvent,
            _ => unreachable!("parse_aws_service_event called with non-AWS-service event"),
        }
    }

    /// Parse API Gateway events (v1 and v2)
    #[inline]
    fn parse_api_gateway_event(&self, value: &str) -> LambdaEventType {
        match value {
            "APIGatewayProxyRequest" => LambdaEventType::ApiGatewayProxyRequest,
            "APIGatewayV2HttpRequest" => LambdaEventType::ApiGatewayV2HttpRequest,
            _ => unreachable!("parse_api_gateway_event called with non-API-Gateway event"),
        }
    }

    /// Parse custom or EventBridge event types
    #[inline]
    fn parse_custom_event_type(&self, value: &str) -> LambdaEventType {
        if value.starts_with("EventBridgeEvent<") && value.ends_with('>') {
            let inner = &value[17..value.len() - 1];
            LambdaEventType::EventBridgeEvent(Some(inner.to_string()))
        } else if value == "EventBridgeEvent" {
            LambdaEventType::EventBridgeEvent(None)
        } else {
            LambdaEventType::Custom(value.to_string())
        }
    }

    fn parse_architecture(&self, value: &str) -> Result<Architecture, AnnotationError> {
        match value {
            "x86_64" | "x64" => Ok(Architecture::X86_64),
            "arm64" | "aarch64" => Ok(Architecture::Arm64),
            _ => Err(AnnotationError::InvalidValue {
                key: "architecture".to_string(),
                value: value.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===================================================================
    // Default Implementation Tests
    // ===================================================================

    #[test]
    fn test_default_annotations() {
        let annotations = TranspilationAnnotations::default();
        assert_eq!(annotations.type_strategy, TypeStrategy::Conservative);
        assert_eq!(annotations.ownership_model, OwnershipModel::Owned);
        assert_eq!(annotations.safety_level, SafetyLevel::Safe);
        assert_eq!(annotations.fallback_strategy, FallbackStrategy::Error);
    }

    #[test]
    fn test_default_annotations_all_fields() {
        let a = TranspilationAnnotations::default();
        assert_eq!(a.bounds_checking, BoundsChecking::Explicit);
        assert_eq!(a.optimization_level, OptimizationLevel::Standard);
        assert_eq!(a.thread_safety, ThreadSafety::NotRequired);
        assert_eq!(a.interior_mutability, InteriorMutability::None);
        assert_eq!(a.string_strategy, StringStrategy::Conservative);
        assert_eq!(a.hash_strategy, HashStrategy::Standard);
        assert_eq!(a.panic_behavior, PanicBehavior::Propagate);
        assert_eq!(a.error_strategy, ErrorStrategy::Panic);
        assert_eq!(a.global_strategy, GlobalStrategy::None);
        assert_eq!(a.termination, Termination::Unknown);
        assert!(a.invariants.is_empty());
        assert!(!a.verify_bounds);
        assert!(a.service_type.is_none());
        assert!(a.migration_strategy.is_none());
        assert!(a.compatibility_layer.is_none());
        assert!(a.pattern.is_none());
        assert!(a.lambda_annotations.is_none());
        assert!(a.custom_attributes.is_empty());
        assert!(a.performance_hints.is_empty());
    }

    #[test]
    fn test_default_lambda_annotations() {
        let la = LambdaAnnotations::default();
        assert_eq!(la.runtime, LambdaRuntime::ProvidedAl2);
        assert!(la.event_type.is_none());
        assert!(la.cold_start_optimize);
        assert_eq!(la.memory_size, 128);
        assert_eq!(la.architecture, Architecture::Arm64);
        assert!(la.pre_warm_paths.is_empty());
        assert!(!la.custom_serialization);
        assert!(!la.batch_failure_reporting);
        assert!(la.timeout.is_none());
        assert!(!la.tracing_enabled);
        assert!(la.environment_variables.is_empty());
    }

    #[test]
    fn test_annotation_validator_default() {
        let v = AnnotationValidator;
        let a = TranspilationAnnotations::default();
        assert!(v.validate(&a).is_ok());
    }

    #[test]
    fn test_annotation_extractor_default() {
        let e = AnnotationExtractor::default();
        let result = e.extract_function_annotations("def foo():\n    pass", "foo");
        assert!(result.is_none());
    }

    #[test]
    fn test_annotation_parser_default() {
        let p = AnnotationParser::default();
        let a = p.parse_annotations("# no annotations here").unwrap();
        assert_eq!(a, TranspilationAnnotations::default());
    }

    // ===================================================================
    // Core Parser Tests
    // ===================================================================

    #[test]
    fn test_parse_basic_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: type_strategy = "conservative"
# @depyler: ownership = "borrowed"
def test_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.type_strategy, TypeStrategy::Conservative);
        assert_eq!(annotations.ownership_model, OwnershipModel::Borrowed);
    }

    #[test]
    fn test_parse_empty_source() {
        let parser = AnnotationParser::new();
        let annotations = parser.parse_annotations("").unwrap();
        assert_eq!(annotations, TranspilationAnnotations::default());
    }

    #[test]
    fn test_parse_no_annotations_in_source() {
        let parser = AnnotationParser::new();
        let source = "def foo():\n    return 42\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations, TranspilationAnnotations::default());
    }

    #[test]
    fn test_parse_function_annotations_delegates() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: ownership = \"shared\"\ndef foo(): pass";
        let a = parser.parse_function_annotations(source).unwrap();
        assert_eq!(a.ownership_model, OwnershipModel::Shared);
    }

    // ===================================================================
    // Type Strategy Parser Tests
    // ===================================================================

    #[test]
    fn test_parse_type_strategy_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("conservative", TypeStrategy::Conservative),
            ("aggressive", TypeStrategy::Aggressive),
            ("zero_copy", TypeStrategy::ZeroCopy),
            ("always_owned", TypeStrategy::AlwaysOwned),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: type_strategy = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.type_strategy, *expected);
        }
    }

    #[test]
    fn test_parse_type_strategy_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: type_strategy = \"bogus\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    // ===================================================================
    // Ownership Model Parser Tests
    // ===================================================================

    #[test]
    fn test_parse_ownership_model_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("owned", OwnershipModel::Owned),
            ("borrowed", OwnershipModel::Borrowed),
            ("shared", OwnershipModel::Shared),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: ownership = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.ownership_model, *expected);
        }
    }

    #[test]
    fn test_parse_ownership_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: ownership = \"moved\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    // ===================================================================
    // Safety Level Parser Tests
    // ===================================================================

    #[test]
    fn test_parse_safety_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: safety_level = "unsafe_allowed"
# @depyler: bounds_checking = "disabled"
def unsafe_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.safety_level, SafetyLevel::UnsafeAllowed);
        assert_eq!(annotations.bounds_checking, BoundsChecking::Disabled);
    }

    #[test]
    fn test_parse_safety_level_safe() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: safety_level = \"safe\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.safety_level, SafetyLevel::Safe);
    }

    #[test]
    fn test_parse_safety_level_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: safety_level = \"yolo\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    // ===================================================================
    // Fallback Strategy Tests
    // ===================================================================

    #[test]
    fn test_parse_fallback_strategy() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: fallback = "mcp"
def complex_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.fallback_strategy, FallbackStrategy::Mcp);
    }

    #[test]
    fn test_parse_fallback_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("mcp", FallbackStrategy::Mcp),
            ("manual", FallbackStrategy::Manual),
            ("error", FallbackStrategy::Error),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: fallback = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.fallback_strategy, *expected);
        }
    }

    #[test]
    fn test_parse_fallback_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: fallback = \"skip\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    // ===================================================================
    // Bounds Checking Tests
    // ===================================================================

    #[test]
    fn test_parse_bounds_checking_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("explicit", BoundsChecking::Explicit),
            ("implicit", BoundsChecking::Implicit),
            ("disabled", BoundsChecking::Disabled),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: bounds_checking = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.bounds_checking, *expected);
        }
    }

    // ===================================================================
    // Performance Hints Tests
    // ===================================================================

    #[test]
    fn test_parse_performance_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: performance_critical = "true"
# @depyler: vectorize = "true"
# @depyler: unroll_loops = "4"
def fast_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert!(annotations
            .performance_hints
            .contains(&PerformanceHint::PerformanceCritical));
        assert!(annotations
            .performance_hints
            .contains(&PerformanceHint::Vectorize));
        assert!(annotations
            .performance_hints
            .contains(&PerformanceHint::UnrollLoops(4)));
    }

    #[test]
    fn test_optimization_hints() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: optimization_hint = "vectorize"
# @depyler: optimization_level = "aggressive"
def optimized_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert!(annotations
            .performance_hints
            .contains(&PerformanceHint::Vectorize));
        assert_eq!(
            annotations.optimization_level,
            OptimizationLevel::Aggressive
        );
    }

    #[test]
    fn test_parse_optimization_hint_latency() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: optimization_hint = \"latency\"";
        let a = parser.parse_annotations(source).unwrap();
        assert!(a
            .performance_hints
            .contains(&PerformanceHint::OptimizeForLatency));
    }

    #[test]
    fn test_parse_optimization_hint_throughput() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: optimization_hint = \"throughput\"";
        let a = parser.parse_annotations(source).unwrap();
        assert!(a
            .performance_hints
            .contains(&PerformanceHint::OptimizeForThroughput));
    }

    #[test]
    fn test_parse_optimization_hint_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: optimization_hint = \"magic\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_parse_unroll_loops_invalid_value() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: unroll_loops = \"not_a_number\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_parse_performance_critical_false() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: performance_critical = \"false\"";
        let a = parser.parse_annotations(source).unwrap();
        assert!(!a
            .performance_hints
            .contains(&PerformanceHint::PerformanceCritical));
    }

    #[test]
    fn test_parse_vectorize_false() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: vectorize = \"false\"";
        let a = parser.parse_annotations(source).unwrap();
        assert!(!a
            .performance_hints
            .contains(&PerformanceHint::Vectorize));
    }

    #[test]
    fn test_parse_optimization_level_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("standard", OptimizationLevel::Standard),
            ("aggressive", OptimizationLevel::Aggressive),
            ("conservative", OptimizationLevel::Conservative),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: optimization_level = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.optimization_level, *expected);
        }
    }

    // ===================================================================
    // Thread Safety Tests
    // ===================================================================

    #[test]
    fn test_parse_thread_safety() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: thread_safety = "required"
# @depyler: interior_mutability = "arc_mutex"
def thread_safe_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.thread_safety, ThreadSafety::Required);
        assert_eq!(
            annotations.interior_mutability,
            InteriorMutability::ArcMutex
        );
    }

    #[test]
    fn test_parse_interior_mutability_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("none", InteriorMutability::None),
            ("arc_mutex", InteriorMutability::ArcMutex),
            ("ref_cell", InteriorMutability::RefCell),
            ("cell", InteriorMutability::Cell),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: interior_mutability = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.interior_mutability, *expected);
        }
    }

    #[test]
    fn test_parse_thread_safety_not_required() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: thread_safety = \"not_required\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.thread_safety, ThreadSafety::NotRequired);
    }

    // ===================================================================
    // String and Hash Strategy Tests
    // ===================================================================

    #[test]
    fn test_string_and_hash_strategies() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: string_strategy = "zero_copy"
# @depyler: hash_strategy = "fnv"
def string_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.string_strategy, StringStrategy::ZeroCopy);
        assert_eq!(annotations.hash_strategy, HashStrategy::Fnv);
    }

    #[test]
    fn test_parse_string_strategy_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("conservative", StringStrategy::Conservative),
            ("always_owned", StringStrategy::AlwaysOwned),
            ("zero_copy", StringStrategy::ZeroCopy),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: string_strategy = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.string_strategy, *expected);
        }
    }

    #[test]
    fn test_parse_hash_strategy_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("standard", HashStrategy::Standard),
            ("fnv", HashStrategy::Fnv),
            ("ahash", HashStrategy::AHash),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: hash_strategy = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.hash_strategy, *expected);
        }
    }

    // ===================================================================
    // Error Handling Annotations Tests
    // ===================================================================

    #[test]
    fn test_error_handling_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: panic_behavior = "return_error"
# @depyler: error_strategy = "result_type"
def error_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.panic_behavior, PanicBehavior::ReturnError);
        assert_eq!(annotations.error_strategy, ErrorStrategy::ResultType);
    }

    #[test]
    fn test_parse_panic_behavior_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("propagate", PanicBehavior::Propagate),
            ("return_error", PanicBehavior::ReturnError),
            ("abort", PanicBehavior::Abort),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: panic_behavior = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.panic_behavior, *expected);
        }
    }

    #[test]
    fn test_parse_error_strategy_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("panic", ErrorStrategy::Panic),
            ("result_type", ErrorStrategy::ResultType),
            ("option_type", ErrorStrategy::OptionType),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: error_strategy = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.error_strategy, *expected);
        }
    }

    // ===================================================================
    // Global Strategy Tests
    // ===================================================================

    #[test]
    fn test_global_strategy() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: global_strategy = "lazy_static"
def global_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.global_strategy, GlobalStrategy::LazyStatic);
    }

    #[test]
    fn test_parse_global_strategy_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("none", GlobalStrategy::None),
            ("lazy_static", GlobalStrategy::LazyStatic),
            ("once_cell", GlobalStrategy::OnceCell),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: global_strategy = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.global_strategy, *expected);
        }
    }

    // ===================================================================
    // Termination Tests
    // ===================================================================

    #[test]
    fn test_parse_termination_unknown() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = \"unknown\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.termination, Termination::Unknown);
    }

    #[test]
    fn test_parse_termination_proven() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = \"proven\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.termination, Termination::Proven);
    }

    #[test]
    fn test_parse_termination_bounded_loop() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = \"bounded_100\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.termination, Termination::BoundedLoop(100));
    }

    #[test]
    fn test_parse_termination_bounded_loop_zero() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = \"bounded_0\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.termination, Termination::BoundedLoop(0));
    }

    #[test]
    fn test_parse_termination_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = \"infinite\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_parse_termination_bounded_non_numeric() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = \"bounded_abc\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    // ===================================================================
    // Verification Annotations Tests
    // ===================================================================

    #[test]
    fn test_verification_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: termination = "proven"
# @depyler: invariant = "left <= right"
# @depyler: verify_bounds = "true"
def verified_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.termination, Termination::Proven);
        assert!(annotations
            .invariants
            .contains(&"left <= right".to_string()));
        assert!(annotations.verify_bounds);
    }

    #[test]
    fn test_parse_verify_bounds_false() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: verify_bounds = \"false\"";
        let a = parser.parse_annotations(source).unwrap();
        assert!(!a.verify_bounds);
    }

    // ===================================================================
    // Service and Migration Tests
    // ===================================================================

    #[test]
    fn test_service_and_migration_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: service_type = "web_api"
# @depyler: migration_strategy = "incremental"
# @depyler: compatibility_layer = "pyo3"
def service_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.service_type, Some(ServiceType::WebApi));
        assert_eq!(
            annotations.migration_strategy,
            Some(MigrationStrategy::Incremental)
        );
        assert_eq!(
            annotations.compatibility_layer,
            Some(CompatibilityLayer::PyO3)
        );
    }

    #[test]
    fn test_parse_service_type_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("web_api", ServiceType::WebApi),
            ("cli", ServiceType::Cli),
            ("library", ServiceType::Library),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: service_type = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.service_type, Some(expected.clone()));
        }
    }

    #[test]
    fn test_parse_migration_strategy_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("incremental", MigrationStrategy::Incremental),
            ("big_bang", MigrationStrategy::BigBang),
            ("hybrid", MigrationStrategy::Hybrid),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: migration_strategy = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.migration_strategy, Some(expected.clone()));
        }
    }

    #[test]
    fn test_parse_compatibility_layer_all_variants() {
        let parser = AnnotationParser::new();
        let cases = [
            ("pyo3", CompatibilityLayer::PyO3),
            ("ctypes", CompatibilityLayer::CTypes),
            ("none", CompatibilityLayer::None),
        ];
        for (input, expected) in &cases {
            let source = format!("# @depyler: compatibility_layer = \"{input}\"");
            let a = parser.parse_annotations(&source).unwrap();
            assert_eq!(a.compatibility_layer, Some(expected.clone()));
        }
    }

    #[test]
    fn test_parse_pattern_annotation() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: pattern = \"observer\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.pattern, Some("observer".to_string()));
    }

    // ===================================================================
    // Error Type Tests
    // ===================================================================

    #[test]
    fn test_invalid_annotation_key() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: invalid_key = "value"
def test_function():
    pass
        "#;

        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::UnknownKey(_))));
    }

    #[test]
    fn test_invalid_annotation_value() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: type_strategy = "invalid_value"
def test_function():
    pass
        "#;

        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_annotation_error_display_invalid_syntax() {
        let err = AnnotationError::InvalidSyntax("bad line".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("Invalid annotation syntax"));
        assert!(msg.contains("bad line"));
    }

    #[test]
    fn test_annotation_error_display_unknown_key() {
        let err = AnnotationError::UnknownKey("foo_bar".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("Unknown annotation key"));
        assert!(msg.contains("foo_bar"));
    }

    #[test]
    fn test_annotation_error_display_invalid_value() {
        let err = AnnotationError::InvalidValue {
            key: "type_strategy".to_string(),
            value: "bogus".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("Invalid value for key type_strategy"));
        assert!(msg.contains("bogus"));
    }

    #[test]
    fn test_annotation_error_is_debug() {
        let err = AnnotationError::UnknownKey("test".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("UnknownKey"));
    }

    // ===================================================================
    // Lambda Annotations Tests
    // ===================================================================

    #[test]
    fn test_lambda_annotations_basic() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: lambda_runtime = "provided.al2"
# @depyler: event_type = "APIGatewayProxyRequest"
# @depyler: cold_start_optimize = "true"
def handler(event, context):
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert!(annotations.lambda_annotations.is_some());

        let lambda_annotations = annotations.lambda_annotations.unwrap();
        assert_eq!(lambda_annotations.runtime, LambdaRuntime::ProvidedAl2);
        assert_eq!(
            lambda_annotations.event_type,
            Some(LambdaEventType::ApiGatewayProxyRequest)
        );
        assert!(lambda_annotations.cold_start_optimize);
    }

    #[test]
    fn test_lambda_annotations_memory_and_architecture() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: memory_size = "256"
# @depyler: architecture = "arm64"
# @depyler: timeout = "30"
def handler(event, context):
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        let lambda_annotations = annotations.lambda_annotations.unwrap();
        assert_eq!(lambda_annotations.memory_size, 256);
        assert_eq!(lambda_annotations.architecture, Architecture::Arm64);
        assert_eq!(lambda_annotations.timeout, Some(30));
    }

    #[test]
    fn test_lambda_eventbridge_with_custom_type() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: event_type = "EventBridgeEvent<OrderEvent>"
# @depyler: custom_serialization = "true"
def handler(event, context):
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        let lambda_annotations = annotations.lambda_annotations.unwrap();
        assert_eq!(
            lambda_annotations.event_type,
            Some(LambdaEventType::EventBridgeEvent(Some(
                "OrderEvent".to_string()
            )))
        );
        assert!(lambda_annotations.custom_serialization);
    }

    #[test]
    fn test_lambda_eventbridge_without_type_parameter() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"EventBridgeEvent\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(
            la.event_type,
            Some(LambdaEventType::EventBridgeEvent(None))
        );
    }

    #[test]
    fn test_lambda_sqs_batch_processing() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: event_type = "SqsEvent"
# @depyler: batch_failure_reporting = "true"
# @depyler: tracing = "Active"
def handler(event, context):
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        let lambda_annotations = annotations.lambda_annotations.unwrap();
        assert_eq!(
            lambda_annotations.event_type,
            Some(LambdaEventType::SqsEvent)
        );
        assert!(lambda_annotations.batch_failure_reporting);
        assert!(lambda_annotations.tracing_enabled);
    }

    #[test]
    fn test_lambda_auto_event_type() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: event_type = "auto"
# @depyler: cold_start_optimize = "true"
def handler(event, context):
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        let lambda_annotations = annotations.lambda_annotations.unwrap();
        assert_eq!(lambda_annotations.event_type, Some(LambdaEventType::Auto));
        assert!(lambda_annotations.cold_start_optimize);
    }

    #[test]
    fn test_lambda_custom_runtime() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: lambda_runtime = "rust-runtime-1.0"
def handler(event, context):
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        let lambda_annotations = annotations.lambda_annotations.unwrap();
        assert_eq!(
            lambda_annotations.runtime,
            LambdaRuntime::Custom("rust-runtime-1.0".to_string())
        );
    }

    #[test]
    fn test_lambda_runtime_provided_al2023() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: lambda_runtime = \"provided.al2023\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.runtime, LambdaRuntime::ProvidedAl2023);
    }

    #[test]
    fn test_lambda_architecture_x86_64() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: architecture = \"x86_64\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.architecture, Architecture::X86_64);
    }

    #[test]
    fn test_lambda_architecture_x64_alias() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: architecture = \"x64\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.architecture, Architecture::X86_64);
    }

    #[test]
    fn test_lambda_architecture_aarch64_alias() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: architecture = \"aarch64\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.architecture, Architecture::Arm64);
    }

    #[test]
    fn test_lambda_architecture_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: architecture = \"mips\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_lambda_memory_size_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: memory_size = \"lots\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_lambda_timeout_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: timeout = \"forever\"";
        let result = parser.parse_annotations(source);
        assert!(matches!(result, Err(AnnotationError::InvalidValue { .. })));
    }

    #[test]
    fn test_lambda_tracing_true() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: tracing = \"true\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert!(la.tracing_enabled);
    }

    #[test]
    fn test_lambda_tracing_false() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: tracing = \"false\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert!(!la.tracing_enabled);
    }

    #[test]
    fn test_lambda_cold_start_optimize_false() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: cold_start_optimize = \"false\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert!(!la.cold_start_optimize);
    }

    #[test]
    fn test_lambda_event_type_s3() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"S3Event\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.event_type, Some(LambdaEventType::S3Event));
    }

    #[test]
    fn test_lambda_event_type_sns() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"SnsEvent\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.event_type, Some(LambdaEventType::SnsEvent));
    }

    #[test]
    fn test_lambda_event_type_dynamodb() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"DynamodbEvent\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.event_type, Some(LambdaEventType::DynamodbEvent));
    }

    #[test]
    fn test_lambda_event_type_cloudwatch() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"CloudwatchEvent\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.event_type, Some(LambdaEventType::CloudwatchEvent));
    }

    #[test]
    fn test_lambda_event_type_kinesis() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"KinesisEvent\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(la.event_type, Some(LambdaEventType::KinesisEvent));
    }

    #[test]
    fn test_lambda_event_type_api_gateway_v2() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"APIGatewayV2HttpRequest\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(
            la.event_type,
            Some(LambdaEventType::ApiGatewayV2HttpRequest)
        );
    }

    #[test]
    fn test_lambda_event_type_custom() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: event_type = \"MyCustomEvent\"";
        let a = parser.parse_annotations(source).unwrap();
        let la = a.lambda_annotations.unwrap();
        assert_eq!(
            la.event_type,
            Some(LambdaEventType::Custom("MyCustomEvent".to_string()))
        );
    }

    // ===================================================================
    // Custom Attributes Tests
    // ===================================================================

    #[test]
    fn test_custom_custom_attribute_single() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: custom_attribute = "inline"
def my_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.custom_attributes.len(), 1);
        assert_eq!(annotations.custom_attributes[0], "inline");
    }

    #[test]
    fn test_custom_custom_attribute_multiple() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: custom_attribute = "inline"
# @depyler: custom_attribute = "must_use"
# @depyler: custom_attribute = "cold"
def my_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.custom_attributes.len(), 3);
        assert_eq!(annotations.custom_attributes[0], "inline");
        assert_eq!(annotations.custom_attributes[1], "must_use");
        assert_eq!(annotations.custom_attributes[2], "cold");
    }

    #[test]
    fn test_custom_custom_attribute_with_other_annotations() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: optimization_level = "aggressive"
# @depyler: custom_attribute = "inline(always)"
# @depyler: performance_critical = "true"
def hot_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(
            annotations.optimization_level,
            OptimizationLevel::Aggressive
        );
        assert_eq!(annotations.custom_attributes.len(), 1);
        assert_eq!(annotations.custom_attributes[0], "inline(always)");
        assert!(annotations
            .performance_hints
            .contains(&PerformanceHint::PerformanceCritical));
    }

    #[test]
    fn test_custom_custom_attribute_empty() {
        let parser = AnnotationParser::new();
        let source = r#"
def my_function():
    pass
        "#;

        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.custom_attributes.len(), 0);
    }

    // ===================================================================
    // AnnotationValidator Tests
    // ===================================================================

    #[test]
    fn test_validator_new() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations::default();
        assert!(v.validate(&a).is_ok());
    }

    #[test]
    fn test_validator_zero_copy_string_with_owned_conflict() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            string_strategy: StringStrategy::ZeroCopy,
            ownership_model: OwnershipModel::Owned,
            ..Default::default()
        };
        let result = v.validate(&a);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("Zero-copy string strategy")));
    }

    #[test]
    fn test_validator_refcell_with_thread_safety_conflict() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            thread_safety: ThreadSafety::Required,
            interior_mutability: InteriorMutability::RefCell,
            ..Default::default()
        };
        let result = v.validate(&a);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("RefCell is not thread-safe")));
    }

    #[test]
    fn test_validator_conflicting_panic_and_error_strategy() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            panic_behavior: PanicBehavior::ReturnError,
            error_strategy: ErrorStrategy::Panic,
            ..Default::default()
        };
        let result = v.validate(&a);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("Conflicting panic behavior")));
    }

    #[test]
    fn test_validator_aggressive_opt_with_explicit_bounds_conflict() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            optimization_level: OptimizationLevel::Aggressive,
            bounds_checking: BoundsChecking::Explicit,
            ..Default::default()
        };
        let result = v.validate(&a);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("Aggressive optimization")));
    }

    #[test]
    fn test_validator_multiple_conflicts() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            string_strategy: StringStrategy::ZeroCopy,
            ownership_model: OwnershipModel::Owned,
            thread_safety: ThreadSafety::Required,
            interior_mutability: InteriorMutability::RefCell,
            ..Default::default()
        };
        let result = v.validate(&a);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2);
    }

    #[test]
    fn test_validator_no_conflict_with_valid_combination() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            string_strategy: StringStrategy::AlwaysOwned,
            ownership_model: OwnershipModel::Owned,
            thread_safety: ThreadSafety::Required,
            interior_mutability: InteriorMutability::ArcMutex,
            ..Default::default()
        };
        assert!(v.validate(&a).is_ok());
    }

    // ===================================================================
    // Suggest Improvements Tests
    // ===================================================================

    #[test]
    fn test_suggest_improvements_perf_critical_not_aggressive() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            performance_hints: vec![PerformanceHint::PerformanceCritical],
            optimization_level: OptimizationLevel::Standard,
            ..Default::default()
        };
        let suggestions = v.suggest_improvements(&a);
        assert!(suggestions.iter().any(|s| s.contains("aggressive")));
    }

    #[test]
    fn test_suggest_improvements_thread_safe_not_shared() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            thread_safety: ThreadSafety::Required,
            ownership_model: OwnershipModel::Owned,
            ..Default::default()
        };
        let suggestions = v.suggest_improvements(&a);
        assert!(suggestions.iter().any(|s| s.contains("shared")));
    }

    #[test]
    fn test_suggest_improvements_web_api_no_latency() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            service_type: Some(ServiceType::WebApi),
            ..Default::default()
        };
        let suggestions = v.suggest_improvements(&a);
        assert!(suggestions.iter().any(|s| s.contains("latency")));
    }

    #[test]
    fn test_suggest_improvements_none_when_optimal() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations::default();
        let suggestions = v.suggest_improvements(&a);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_suggest_improvements_web_api_with_latency_no_suggestion() {
        let v = AnnotationValidator::new();
        let a = TranspilationAnnotations {
            service_type: Some(ServiceType::WebApi),
            performance_hints: vec![PerformanceHint::OptimizeForLatency],
            ..Default::default()
        };
        let suggestions = v.suggest_improvements(&a);
        assert!(!suggestions.iter().any(|s| s.contains("latency")));
    }

    // ===================================================================
    // AnnotationExtractor Tests
    // ===================================================================

    #[test]
    fn test_extractor_new() {
        let e = AnnotationExtractor::new();
        let result = e.extract_function_annotations("def foo():\n    pass", "bar");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_function_with_annotation() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: ownership = \"borrowed\"\ndef my_func(x):\n    return x";
        let result = e.extract_function_annotations(source, "my_func");
        assert!(result.is_some());
        assert!(result.unwrap().contains("@depyler:"));
    }

    #[test]
    fn test_extractor_function_no_annotation() {
        let e = AnnotationExtractor::new();
        let source = "# just a regular comment\ndef my_func(x):\n    return x";
        let result = e.extract_function_annotations(source, "my_func");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_function_not_found() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: ownership = \"borrowed\"\ndef foo():\n    pass";
        let result = e.extract_function_annotations(source, "bar");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_class_with_annotation() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: service_type = \"web_api\"\nclass MyService:\n    pass";
        let result = e.extract_class_annotations(source, "MyService");
        assert!(result.is_some());
        assert!(result.unwrap().contains("@depyler:"));
    }

    #[test]
    fn test_extractor_class_no_annotation() {
        let e = AnnotationExtractor::new();
        let source = "# regular comment\nclass MyClass:\n    pass";
        let result = e.extract_class_annotations(source, "MyClass");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_class_not_found() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: ownership = \"borrowed\"\nclass Foo:\n    pass";
        let result = e.extract_class_annotations(source, "Bar");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_class_with_parentheses() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: ownership = \"shared\"\nclass Child(Parent):\n    pass";
        let result = e.extract_class_annotations(source, "Child");
        assert!(result.is_some());
    }

    // ===================================================================
    // Serialization Tests
    // ===================================================================

    #[test]
    fn test_serialize_transpilation_annotations() {
        let a = TranspilationAnnotations::default();
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("Conservative"));
        assert!(json.contains("Owned"));
    }

    #[test]
    fn test_deserialize_transpilation_annotations() {
        let a = TranspilationAnnotations::default();
        let json = serde_json::to_string(&a).unwrap();
        let deserialized: TranspilationAnnotations = serde_json::from_str(&json).unwrap();
        assert_eq!(a, deserialized);
    }

    #[test]
    fn test_serialize_lambda_annotations() {
        let la = LambdaAnnotations::default();
        let json = serde_json::to_string(&la).unwrap();
        assert!(json.contains("ProvidedAl2"));
        assert!(json.contains("Arm64"));
    }

    #[test]
    fn test_deserialize_lambda_annotations() {
        let la = LambdaAnnotations::default();
        let json = serde_json::to_string(&la).unwrap();
        let deserialized: LambdaAnnotations = serde_json::from_str(&json).unwrap();
        assert_eq!(la, deserialized);
    }

    #[test]
    fn test_serialize_roundtrip_all_event_types() {
        let event_types = vec![
            LambdaEventType::Auto,
            LambdaEventType::S3Event,
            LambdaEventType::ApiGatewayProxyRequest,
            LambdaEventType::ApiGatewayV2HttpRequest,
            LambdaEventType::SqsEvent,
            LambdaEventType::SnsEvent,
            LambdaEventType::DynamodbEvent,
            LambdaEventType::EventBridgeEvent(None),
            LambdaEventType::EventBridgeEvent(Some("Order".to_string())),
            LambdaEventType::CloudwatchEvent,
            LambdaEventType::KinesisEvent,
            LambdaEventType::Custom("MyEvent".to_string()),
        ];
        for et in event_types {
            let json = serde_json::to_string(&et).unwrap();
            let deserialized: LambdaEventType = serde_json::from_str(&json).unwrap();
            assert_eq!(et, deserialized);
        }
    }

    #[test]
    fn test_serialize_roundtrip_with_lambda_annotations() {
        let la = LambdaAnnotations {
            event_type: Some(LambdaEventType::SqsEvent),
            memory_size: 512,
            timeout: Some(60),
            tracing_enabled: true,
            batch_failure_reporting: true,
            ..Default::default()
        };
        let a = TranspilationAnnotations {
            lambda_annotations: Some(la),
            service_type: Some(ServiceType::WebApi),
            pattern: Some("microservice".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&a).unwrap();
        let deserialized: TranspilationAnnotations = serde_json::from_str(&json).unwrap();
        assert_eq!(a, deserialized);
    }

    // ===================================================================
    // Clone and Debug Trait Tests
    // ===================================================================

    #[test]
    fn test_transpilation_annotations_clone() {
        let a = TranspilationAnnotations {
            type_strategy: TypeStrategy::Aggressive,
            invariants: vec!["x > 0".to_string()],
            ..Default::default()
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_lambda_annotations_clone() {
        let la = LambdaAnnotations {
            memory_size: 1024,
            event_type: Some(LambdaEventType::S3Event),
            ..Default::default()
        };
        let cloned = la.clone();
        assert_eq!(la, cloned);
    }

    #[test]
    fn test_annotation_extractor_clone() {
        let e = AnnotationExtractor::new();
        let _cloned = e.clone();
        // If this compiles and runs, Clone works
    }

    #[test]
    fn test_annotation_parser_new_and_default_equivalent() {
        let p1 = AnnotationParser::new();
        let p2 = AnnotationParser::default();
        // Both should produce identical results on the same input
        let source = "# @depyler: ownership = \"shared\"";
        let a1 = p1.parse_annotations(source).unwrap();
        let a2 = p2.parse_annotations(source).unwrap();
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_annotation_validator_debug() {
        let v = AnnotationValidator::new();
        let debug_str = format!("{v:?}");
        assert!(debug_str.contains("AnnotationValidator"));
    }

    #[test]
    fn test_annotation_extractor_debug() {
        let e = AnnotationExtractor::new();
        let debug_str = format!("{e:?}");
        assert!(debug_str.contains("AnnotationExtractor"));
    }

    // ===================================================================
    // Enum Variant Equality and Hash Tests
    // ===================================================================

    #[test]
    fn test_lambda_event_type_hash_in_collections() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(LambdaEventType::Auto);
        set.insert(LambdaEventType::S3Event);
        set.insert(LambdaEventType::Auto); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(TypeStrategy::Conservative, TypeStrategy::Conservative);
        assert_ne!(TypeStrategy::Conservative, TypeStrategy::Aggressive);
        assert_eq!(OwnershipModel::Owned, OwnershipModel::Owned);
        assert_ne!(OwnershipModel::Owned, OwnershipModel::Borrowed);
        assert_eq!(
            Termination::BoundedLoop(10),
            Termination::BoundedLoop(10)
        );
        assert_ne!(Termination::BoundedLoop(10), Termination::BoundedLoop(20));
    }

    // ===================================================================
    // Parser Edge Cases
    // ===================================================================

    #[test]
    fn test_parse_annotation_with_extra_whitespace_before_key() {
        let parser = AnnotationParser::new();
        // Extra spaces between # and @depyler are handled by \s* in the regex
        let source = "#   @depyler: ownership = \"shared\"";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.ownership_model, OwnershipModel::Shared);
    }

    #[test]
    fn test_parse_annotation_mixed_with_plain_comments() {
        let parser = AnnotationParser::new();
        let source = r#"
# This is a regular comment
# @depyler: ownership = "borrowed"
# Another plain comment
def foo():
    pass
        "#;
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.ownership_model, OwnershipModel::Borrowed);
    }

    #[test]
    fn test_parse_all_annotations_combined() {
        let parser = AnnotationParser::new();
        let source = r#"
# @depyler: type_strategy = "aggressive"
# @depyler: ownership = "shared"
# @depyler: safety_level = "unsafe_allowed"
# @depyler: fallback = "manual"
# @depyler: bounds_checking = "implicit"
# @depyler: optimization_level = "conservative"
# @depyler: thread_safety = "required"
# @depyler: interior_mutability = "arc_mutex"
# @depyler: string_strategy = "always_owned"
# @depyler: hash_strategy = "ahash"
# @depyler: panic_behavior = "abort"
# @depyler: error_strategy = "option_type"
# @depyler: global_strategy = "once_cell"
# @depyler: termination = "bounded_50"
# @depyler: verify_bounds = "true"
# @depyler: service_type = "cli"
# @depyler: migration_strategy = "hybrid"
# @depyler: compatibility_layer = "ctypes"
# @depyler: pattern = "singleton"
def full_function():
    pass
        "#;

        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a.type_strategy, TypeStrategy::Aggressive);
        assert_eq!(a.ownership_model, OwnershipModel::Shared);
        assert_eq!(a.safety_level, SafetyLevel::UnsafeAllowed);
        assert_eq!(a.fallback_strategy, FallbackStrategy::Manual);
        assert_eq!(a.bounds_checking, BoundsChecking::Implicit);
        assert_eq!(a.optimization_level, OptimizationLevel::Conservative);
        assert_eq!(a.thread_safety, ThreadSafety::Required);
        assert_eq!(a.interior_mutability, InteriorMutability::ArcMutex);
        assert_eq!(a.string_strategy, StringStrategy::AlwaysOwned);
        assert_eq!(a.hash_strategy, HashStrategy::AHash);
        assert_eq!(a.panic_behavior, PanicBehavior::Abort);
        assert_eq!(a.error_strategy, ErrorStrategy::OptionType);
        assert_eq!(a.global_strategy, GlobalStrategy::OnceCell);
        assert_eq!(a.termination, Termination::BoundedLoop(50));
        assert!(a.verify_bounds);
        assert_eq!(a.service_type, Some(ServiceType::Cli));
        assert_eq!(a.migration_strategy, Some(MigrationStrategy::Hybrid));
        assert_eq!(a.compatibility_layer, Some(CompatibilityLayer::CTypes));
        assert_eq!(a.pattern, Some("singleton".to_string()));
    }

    #[test]
    fn test_parse_only_comments_no_depyler_prefix() {
        let parser = AnnotationParser::new();
        let source = "# just a comment\n# another one\n";
        let a = parser.parse_annotations(source).unwrap();
        assert_eq!(a, TranspilationAnnotations::default());
    }

    #[test]
    fn test_extractor_function_at_first_line() {
        let e = AnnotationExtractor::new();
        let source = "def first_func():\n    pass";
        let result = e.extract_function_annotations(source, "first_func");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_empty_source() {
        let e = AnnotationExtractor::new();
        let result = e.extract_function_annotations("", "anything");
        assert!(result.is_none());
    }

    #[test]
    fn test_extractor_class_empty_source() {
        let e = AnnotationExtractor::new();
        let result = e.extract_class_annotations("", "anything");
        assert!(result.is_none());
    }

    // ========================================================================
    // S9B7: Coverage tests for annotations
    // ========================================================================

    #[test]
    fn test_s9b7_annotation_error_display_invalid_syntax() {
        let err = AnnotationError::InvalidSyntax("bad format".to_string());
        assert_eq!(err.to_string(), "Invalid annotation syntax: bad format");
    }

    #[test]
    fn test_s9b7_annotation_error_display_unknown_key() {
        let err = AnnotationError::UnknownKey("foobar".to_string());
        assert_eq!(err.to_string(), "Unknown annotation key: foobar");
    }

    #[test]
    fn test_s9b7_annotation_error_display_invalid_value() {
        let err = AnnotationError::InvalidValue {
            key: "type_strategy".to_string(),
            value: "bad_val".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("type_strategy"));
        assert!(msg.contains("bad_val"));
    }

    #[test]
    fn test_s9b7_annotation_error_debug() {
        let err = AnnotationError::UnknownKey("x".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("UnknownKey"));
    }

    #[test]
    fn test_s9b7_validator_no_conflicts_default() {
        let validator = AnnotationValidator::new();
        let annotations = TranspilationAnnotations::default();
        assert!(validator.validate(&annotations).is_ok());
    }

    #[test]
    fn test_s9b7_validator_thread_safety_refcell_conflict() {
        let validator = AnnotationValidator::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations.thread_safety = ThreadSafety::Required;
        annotations.interior_mutability = InteriorMutability::RefCell;
        let result = validator.validate(&annotations);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("RefCell")));
    }

    #[test]
    fn test_s9b7_validator_panic_error_conflict() {
        let validator = AnnotationValidator::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations.panic_behavior = PanicBehavior::ReturnError;
        annotations.error_strategy = ErrorStrategy::Panic;
        let result = validator.validate(&annotations);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Conflicting panic")));
    }

    #[test]
    fn test_s9b7_validator_aggressive_opt_bounds_conflict() {
        let validator = AnnotationValidator::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations.optimization_level = OptimizationLevel::Aggressive;
        annotations.bounds_checking = BoundsChecking::Explicit;
        let result = validator.validate(&annotations);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("bounds checking")));
    }

    #[test]
    fn test_s9b7_suggest_improvements_performance_critical() {
        let validator = AnnotationValidator::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::PerformanceCritical);
        let suggestions = validator.suggest_improvements(&annotations);
        assert!(suggestions.iter().any(|s| s.contains("aggressive")));
    }

    #[test]
    fn test_s9b7_suggest_improvements_thread_safety_shared() {
        let validator = AnnotationValidator::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations.thread_safety = ThreadSafety::Required;
        annotations.ownership_model = OwnershipModel::Owned;
        let suggestions = validator.suggest_improvements(&annotations);
        assert!(suggestions.iter().any(|s| s.contains("shared")));
    }

    #[test]
    fn test_s9b7_suggest_improvements_web_api_latency() {
        let validator = AnnotationValidator::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations.service_type = Some(ServiceType::WebApi);
        let suggestions = validator.suggest_improvements(&annotations);
        assert!(suggestions.iter().any(|s| s.contains("latency")));
    }

    #[test]
    fn test_s9b7_suggest_improvements_no_suggestions() {
        let validator = AnnotationValidator::new();
        let annotations = TranspilationAnnotations::default();
        let suggestions = validator.suggest_improvements(&annotations);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_s9b7_lambda_annotations_default() {
        let la = LambdaAnnotations::default();
        assert!(matches!(la.runtime, LambdaRuntime::ProvidedAl2));
        assert!(la.event_type.is_none());
        assert!(la.cold_start_optimize);
        assert_eq!(la.memory_size, 128);
        assert!(matches!(la.architecture, Architecture::Arm64));
        assert!(!la.custom_serialization);
        assert!(!la.batch_failure_reporting);
        assert!(la.timeout.is_none());
        assert!(!la.tracing_enabled);
        assert!(la.environment_variables.is_empty());
    }

    #[test]
    fn test_s9b7_parse_termination_bounded_loop() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = bounded_100\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.termination, Termination::BoundedLoop(100));
    }

    #[test]
    fn test_s9b7_parse_termination_invalid() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: termination = bounded_abc\n";
        let result = parser.parse_annotations(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_parse_invariant() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: invariant = x_greater_zero\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert!(annotations.invariants.contains(&"x_greater_zero".to_string()));
    }

    #[test]
    fn test_s9b7_parse_verify_bounds_true() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: verify_bounds = true\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert!(annotations.verify_bounds);
    }

    #[test]
    fn test_s9b7_parse_verify_bounds_false() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: verify_bounds = false\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert!(!annotations.verify_bounds);
    }

    #[test]
    fn test_s9b7_parse_pattern() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: pattern = singleton\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.pattern, Some("singleton".to_string()));
    }

    #[test]
    fn test_s9b7_parse_custom_attribute() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: custom_attribute = my_attr\n# @depyler: custom_attribute = another\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.custom_attributes.len(), 2);
        assert!(annotations.custom_attributes.contains(&"my_attr".to_string()));
        assert!(annotations.custom_attributes.contains(&"another".to_string()));
    }

    #[test]
    fn test_s9b7_parse_global_strategy_lazy_static() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: global_strategy = lazy_static\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.global_strategy, GlobalStrategy::LazyStatic);
    }

    #[test]
    fn test_s9b7_parse_global_strategy_once_cell() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: global_strategy = once_cell\n";
        let annotations = parser.parse_annotations(source).unwrap();
        assert_eq!(annotations.global_strategy, GlobalStrategy::OnceCell);
    }

    #[test]
    fn test_s9b7_parse_function_annotations_delegates() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: ownership = borrowed\n";
        let annotations = parser.parse_function_annotations(source).unwrap();
        assert_eq!(annotations.ownership_model, OwnershipModel::Borrowed);
    }

    #[test]
    fn test_s9b7_extractor_no_annotation_above_function() {
        let e = AnnotationExtractor::new();
        let source = "# just a comment\ndef my_func():\n    pass";
        let result = e.extract_function_annotations(source, "my_func");
        assert!(result.is_none());
    }

    #[test]
    fn test_s9b7_extractor_no_matching_function() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: ownership = owned\ndef other():\n    pass";
        let result = e.extract_function_annotations(source, "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_s9b7_extractor_class_no_annotation() {
        let e = AnnotationExtractor::new();
        let source = "# just a comment\nclass MyClass:\n    pass";
        let result = e.extract_class_annotations(source, "MyClass");
        assert!(result.is_none());
    }

    #[test]
    fn test_s9b7_extractor_class_no_match() {
        let e = AnnotationExtractor::new();
        let source = "# @depyler: ownership = shared\nclass Other:\n    pass";
        let result = e.extract_class_annotations(source, "NotHere");
        assert!(result.is_none());
    }

    #[test]
    fn test_s9b7_annotation_validator_default() {
        let v = AnnotationValidator::default();
        let debug = format!("{:?}", v);
        assert!(debug.contains("AnnotationValidator"));
    }

    #[test]
    fn test_s9b7_lambda_event_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(LambdaEventType::Auto);
        set.insert(LambdaEventType::S3Event);
        set.insert(LambdaEventType::SqsEvent);
        assert_eq!(set.len(), 3);
    }

    // ========================================================================
    // DEPYLER-99MODE-S11: Coverage tests for untested annotation paths
    // ========================================================================

    #[test]
    fn test_s11_parse_optimization_hint_async_ready() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: optimization_hint = \"async_ready\"";
        let result = parser.parse_annotations(source);
        // async_ready is experimental - it prints a warning but does NOT error
        assert!(result.is_ok());
        // No performance hints should be added (async_ready just warns)
        let annotations = result.unwrap();
        assert!(!annotations
            .performance_hints
            .contains(&PerformanceHint::Vectorize));
    }

    #[test]
    fn test_s11_parse_optimization_hint_vectorize() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: optimization_hint = \"vectorize\"";
        let result = parser.parse_annotations(source).unwrap();
        assert!(result
            .performance_hints
            .contains(&PerformanceHint::Vectorize));
    }

    #[test]
    fn test_s11_parse_custom_attribute_serde_roundtrip() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: custom_attribute = \"#[derive(Clone)]\"";
        let result = parser.parse_annotations(source).unwrap();
        assert!(result
            .custom_attributes
            .contains(&"#[derive(Clone)]".to_string()));

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: TranspilationAnnotations = serde_json::from_str(&json).unwrap();
        assert_eq!(result.custom_attributes, deserialized.custom_attributes);
    }

    #[test]
    fn test_s11_parse_invariant_annotation() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: invariant = \"x > 0\"";
        let result = parser.parse_annotations(source).unwrap();
        assert_eq!(result.invariants.len(), 1);
        assert!(result.invariants.contains(&"x > 0".to_string()));
    }

    #[test]
    fn test_s11_parse_batch_failure_reporting() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: batch_failure_reporting = \"true\"";
        let result = parser.parse_annotations(source).unwrap();
        let lambda = result.lambda_annotations.unwrap();
        assert!(lambda.batch_failure_reporting);
    }

    #[test]
    fn test_s11_parse_custom_serialization() {
        let parser = AnnotationParser::new();
        let source = "# @depyler: custom_serialization = \"true\"";
        let result = parser.parse_annotations(source).unwrap();
        let lambda = result.lambda_annotations.unwrap();
        assert!(lambda.custom_serialization);
    }
}
