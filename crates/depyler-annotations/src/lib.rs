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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeStrategy {
    Conservative,
    Aggressive,
    ZeroCopy,
    AlwaysOwned,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OwnershipModel {
    Owned,
    Borrowed,
    Shared,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    UnsafeAllowed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceHint {
    Vectorize,
    UnrollLoops(u32),
    OptimizeForLatency,
    OptimizeForThroughput,
    PerformanceCritical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FallbackStrategy {
    Mcp,
    Manual,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BoundsChecking {
    Explicit,
    Implicit,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Standard,
    Aggressive,
    Conservative,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreadSafety {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteriorMutability {
    None,
    ArcMutex,
    RefCell,
    Cell,
}

pub struct AnnotationParser {
    pattern: Regex,
}

impl Default for AnnotationParser {
    fn default() -> Self {
        Self::new()
    }
}

impl AnnotationParser {
    pub fn new() -> Self {
        let pattern = Regex::new(r"#\s*@depyler:\s*(\w+)\s*=\s*(.+)")
            .expect("Invalid regex pattern");
        Self { pattern }
    }

    pub fn parse_annotations(&self, source: &str) -> Result<TranspilationAnnotations, AnnotationError> {
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

    pub fn parse_function_annotations(&self, function_source: &str) -> Result<TranspilationAnnotations, AnnotationError> {
        self.parse_annotations(function_source)
    }

    fn apply_annotations(
        &self,
        annotations: &mut TranspilationAnnotations,
        values: HashMap<String, String>,
    ) -> Result<(), AnnotationError> {
        for (key, value) in values {
            match key.as_str() {
                "type_strategy" => {
                    annotations.type_strategy = self.parse_type_strategy(&value)?;
                }
                "ownership" => {
                    annotations.ownership_model = self.parse_ownership_model(&value)?;
                }
                "safety_level" => {
                    annotations.safety_level = self.parse_safety_level(&value)?;
                }
                "fallback" => {
                    annotations.fallback_strategy = self.parse_fallback_strategy(&value)?;
                }
                "bounds_checking" => {
                    annotations.bounds_checking = self.parse_bounds_checking(&value)?;
                }
                "optimization_level" => {
                    annotations.optimization_level = self.parse_optimization_level(&value)?;
                }
                "thread_safety" => {
                    annotations.thread_safety = self.parse_thread_safety(&value)?;
                }
                "interior_mutability" => {
                    annotations.interior_mutability = self.parse_interior_mutability(&value)?;
                }
                "performance_critical" => {
                    if value == "true" {
                        annotations.performance_hints.push(PerformanceHint::PerformanceCritical);
                    }
                }
                "vectorize" => {
                    if value == "true" {
                        annotations.performance_hints.push(PerformanceHint::Vectorize);
                    }
                }
                "unroll_loops" => {
                    let count: u32 = value.parse().map_err(|_| AnnotationError::InvalidValue {
                        key: key.clone(),
                        value: value.clone(),
                    })?;
                    annotations.performance_hints.push(PerformanceHint::UnrollLoops(count));
                }
                "optimization_hint" => {
                    match value.as_str() {
                        "vectorize" => annotations.performance_hints.push(PerformanceHint::Vectorize),
                        "latency" => annotations.performance_hints.push(PerformanceHint::OptimizeForLatency),
                        "throughput" => annotations.performance_hints.push(PerformanceHint::OptimizeForThroughput),
                        _ => return Err(AnnotationError::InvalidValue { key, value }),
                    }
                }
                _ => return Err(AnnotationError::UnknownKey(key)),
            }
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

    fn parse_interior_mutability(&self, value: &str) -> Result<InteriorMutability, AnnotationError> {
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
        assert!(annotations.performance_hints.contains(&PerformanceHint::PerformanceCritical));
        assert!(annotations.performance_hints.contains(&PerformanceHint::Vectorize));
        assert!(annotations.performance_hints.contains(&PerformanceHint::UnrollLoops(4)));
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
        assert_eq!(annotations.interior_mutability, InteriorMutability::ArcMutex);
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
        assert!(annotations.performance_hints.contains(&PerformanceHint::Vectorize));
        assert_eq!(annotations.optimization_level, OptimizationLevel::Aggressive);
    }
}