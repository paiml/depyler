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
            function_pattern: Regex::new(r"(?m)^def\s+(\w+)\s*\(").unwrap(),
            class_pattern: Regex::new(r"(?m)^class\s+(\w+)\s*[\(:]").unwrap(),
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
                if captures.get(1).unwrap().as_str() == function_name {
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
                if captures.get(1).unwrap().as_str() == class_name {
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
                let key = captures.get(1).unwrap().as_str().to_string();
                let value = captures.get(2).unwrap().as_str().trim_matches('"').trim();
                parsed_values.insert(key, value.to_string());
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
                "optimization_level" | "performance_critical" | "vectorize" | "unroll_loops"
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
                "lambda_runtime" | "event_type" | "cold_start_optimize" | "memory_size"
                | "architecture" | "batch_failure_reporting" | "custom_serialization"
                | "timeout" | "tracing" => {
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
            "cold_start_optimize" | "batch_failure_reporting" | "custom_serialization" | "tracing" => {
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
            "S3Event" | "SqsEvent" | "SnsEvent" | "DynamodbEvent" | "CloudwatchEvent" | "KinesisEvent" => {
                self.parse_aws_service_event(value)
            }
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
    fn test_default_annotations() {
        let annotations = TranspilationAnnotations::default();
        assert_eq!(annotations.type_strategy, TypeStrategy::Conservative);
        assert_eq!(annotations.ownership_model, OwnershipModel::Owned);
        assert_eq!(annotations.safety_level, SafetyLevel::Safe);
        assert_eq!(annotations.fallback_strategy, FallbackStrategy::Error);
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
}
