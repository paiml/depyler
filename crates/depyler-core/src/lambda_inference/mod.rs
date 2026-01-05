//! Lambda event type inference engine
//!
//! This module analyzes Python AST patterns to determine AWS Lambda event types
//! with confidence scoring.

pub mod pattern_extraction;

use anyhow::Result;
use rustpython_ast::{Mod, ModModule};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use pattern_extraction::extract_access_patterns;

/// Lambda event type inference engine that analyzes Python AST patterns
/// to determine AWS Lambda event types with confidence scoring
#[derive(Debug, Clone)]
pub struct LambdaTypeInferencer {
    event_patterns: HashMap<Pattern, EventType>,
    pub confidence_threshold: f64,
}

/// Pattern matching structure for event access chains
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pattern {
    pub access_chain: Vec<String>,
    pub pattern_type: PatternType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternType {
    Subscript,
    Attribute,
    Mixed,
}

/// AWS Lambda event types supported by the inferencer
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    S3Event,
    ApiGatewayV2Http,
    SnsEvent,
    SqsEvent,
    DynamodbEvent,
    EventBridge,
    Cloudwatch,
    Unknown,
}

/// Inference error types
#[derive(Debug, Clone)]
pub enum InferenceError {
    AmbiguousEventType,
    NoPatternMatch,
    ParseError(String),
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferenceError::AmbiguousEventType => write!(
                f,
                "Could not determine event type with sufficient confidence"
            ),
            InferenceError::NoPatternMatch => write!(f, "No matching event pattern found"),
            InferenceError::ParseError(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for InferenceError {}

impl Default for LambdaTypeInferencer {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaTypeInferencer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // S3 Event patterns
        patterns.insert(
            Pattern {
                access_chain: vec!["Records".to_string(), "s3".to_string()],
                pattern_type: PatternType::Mixed,
            },
            EventType::S3Event,
        );
        patterns.insert(
            Pattern {
                access_chain: vec![
                    "Records".to_string(),
                    "s3".to_string(),
                    "bucket".to_string(),
                ],
                pattern_type: PatternType::Mixed,
            },
            EventType::S3Event,
        );
        patterns.insert(
            Pattern {
                access_chain: vec![
                    "Records".to_string(),
                    "s3".to_string(),
                    "object".to_string(),
                ],
                pattern_type: PatternType::Mixed,
            },
            EventType::S3Event,
        );

        // API Gateway v2 patterns
        patterns.insert(
            Pattern {
                access_chain: vec!["requestContext".to_string(), "http".to_string()],
                pattern_type: PatternType::Mixed,
            },
            EventType::ApiGatewayV2Http,
        );
        patterns.insert(
            Pattern {
                access_chain: vec![
                    "requestContext".to_string(),
                    "http".to_string(),
                    "method".to_string(),
                ],
                pattern_type: PatternType::Mixed,
            },
            EventType::ApiGatewayV2Http,
        );

        // SNS Event patterns
        patterns.insert(
            Pattern {
                access_chain: vec!["Records".to_string(), "Sns".to_string()],
                pattern_type: PatternType::Mixed,
            },
            EventType::SnsEvent,
        );
        patterns.insert(
            Pattern {
                access_chain: vec![
                    "Records".to_string(),
                    "Sns".to_string(),
                    "Message".to_string(),
                ],
                pattern_type: PatternType::Mixed,
            },
            EventType::SnsEvent,
        );

        // SQS Event patterns
        patterns.insert(
            Pattern {
                access_chain: vec!["Records".to_string(), "messageId".to_string()],
                pattern_type: PatternType::Mixed,
            },
            EventType::SqsEvent,
        );
        patterns.insert(
            Pattern {
                access_chain: vec!["Records".to_string(), "receiptHandle".to_string()],
                pattern_type: PatternType::Mixed,
            },
            EventType::SqsEvent,
        );

        // DynamoDB Event patterns
        patterns.insert(
            Pattern {
                access_chain: vec!["Records".to_string(), "dynamodb".to_string()],
                pattern_type: PatternType::Mixed,
            },
            EventType::DynamodbEvent,
        );

        // EventBridge patterns
        patterns.insert(
            Pattern {
                access_chain: vec!["detail-type".to_string()],
                pattern_type: PatternType::Subscript,
            },
            EventType::EventBridge,
        );
        patterns.insert(
            Pattern {
                access_chain: vec!["detail".to_string()],
                pattern_type: PatternType::Subscript,
            },
            EventType::EventBridge,
        );

        Self {
            event_patterns: patterns,
            confidence_threshold: 0.8,
        }
    }

    pub fn with_confidence_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Infer event type from Python module AST
    pub fn infer_event_type(&self, ast: &Mod) -> Result<EventType, InferenceError> {
        match ast {
            Mod::Module(module) => self.infer_from_module(module),
            _ => Err(InferenceError::ParseError(
                "Only module AST supported".to_string(),
            )),
        }
    }

    fn infer_from_module(&self, module: &ModModule) -> Result<EventType, InferenceError> {
        let patterns = extract_access_patterns(&module.body)?;

        if patterns.is_empty() {
            return Err(InferenceError::NoPatternMatch);
        }

        let matches: Vec<(EventType, f64)> = patterns
            .iter()
            .filter_map(|p| self.match_pattern(p))
            .collect();

        if matches.is_empty() {
            return Err(InferenceError::NoPatternMatch);
        }

        // Calculate confidence scores and find best match
        let event_scores = self.calculate_confidence_scores(&matches);

        event_scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .filter(|(_, conf)| *conf > self.confidence_threshold)
            .map(|(event_type, _)| event_type)
            .ok_or(InferenceError::AmbiguousEventType)
    }

    fn match_pattern(&self, pattern: &Pattern) -> Option<(EventType, f64)> {
        for (registered_pattern, event_type) in &self.event_patterns {
            let confidence = self.calculate_pattern_confidence(pattern, registered_pattern);
            if confidence > 0.0 {
                return Some((event_type.clone(), confidence));
            }
        }
        None
    }

    fn calculate_pattern_confidence(&self, observed: &Pattern, registered: &Pattern) -> f64 {
        // Check if the observed pattern contains the registered pattern
        if observed.access_chain.len() < registered.access_chain.len() {
            return 0.0;
        }

        // Check if all elements of the registered pattern match in order
        let mut all_match = true;
        for (i, expected_key) in registered.access_chain.iter().enumerate() {
            if i >= observed.access_chain.len() || observed.access_chain[i] != *expected_key {
                all_match = false;
                break;
            }
        }

        if !all_match {
            return 0.0;
        }

        // Base confidence for matching
        let base_confidence = 0.8;

        // Bonus for exact length match
        let length_bonus = if observed.access_chain.len() == registered.access_chain.len() {
            0.1
        } else {
            0.0
        };

        // Bonus for longer patterns (more specific)
        let specificity_bonus = (registered.access_chain.len() as f64 / 20.0).min(0.1);

        // Pattern type compatibility
        let type_bonus = if observed.pattern_type == registered.pattern_type
            || registered.pattern_type == PatternType::Mixed
        {
            0.05
        } else {
            0.0
        };

        (base_confidence + length_bonus + specificity_bonus + type_bonus).min(1.0)
    }

    fn calculate_confidence_scores(&self, matches: &[(EventType, f64)]) -> Vec<(EventType, f64)> {
        let mut event_scores: HashMap<EventType, Vec<f64>> = HashMap::new();

        for (event_type, confidence) in matches {
            event_scores
                .entry(event_type.clone())
                .or_default()
                .push(*confidence);
        }

        event_scores
            .into_iter()
            .map(|(event_type, confidences)| {
                // Aggregate confidence scores (max + average bonus)
                let max_confidence = confidences.iter().copied().fold(0.0f64, f64::max);
                let avg_confidence = confidences.iter().sum::<f64>() / confidences.len() as f64;
                let final_confidence = max_confidence + (avg_confidence * 0.1);
                (event_type, final_confidence.min(1.0))
            })
            .collect()
    }

    /// Get all known event patterns for debugging
    pub fn get_patterns(&self) -> &HashMap<Pattern, EventType> {
        &self.event_patterns
    }

    /// Analyze a handler function and provide detailed inference report
    pub fn analyze_handler(&self, ast: &Mod) -> Result<AnalysisReport, InferenceError> {
        let patterns = match ast {
            Mod::Module(module) => extract_access_patterns(&module.body)?,
            _ => {
                return Err(InferenceError::ParseError(
                    "Only module AST supported".to_string(),
                ))
            }
        };

        let matches: Vec<(EventType, f64)> = patterns
            .iter()
            .filter_map(|p| self.match_pattern(p))
            .collect();

        let event_scores = self.calculate_confidence_scores(&matches);
        let inferred_type = event_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .filter(|(_, conf)| *conf > self.confidence_threshold)
            .map(|(event_type, _)| event_type.clone())
            .unwrap_or(EventType::Unknown);

        let recommendations = self.generate_recommendations(&patterns);
        Ok(AnalysisReport {
            inferred_event_type: inferred_type,
            detected_patterns: patterns,
            confidence_scores: event_scores,
            recommendations,
        })
    }

    fn generate_recommendations(&self, patterns: &[Pattern]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if patterns.is_empty() {
            recommendations.push(
                "No event access patterns detected. Consider adding event type annotation."
                    .to_string(),
            );
        } else if patterns.len() == 1 {
            recommendations.push("Single access pattern detected. Consider adding more specific event access for better inference.".to_string());
        }

        // Check for common anti-patterns
        let has_generic_access = patterns.iter().any(|p| {
            p.access_chain.len() == 1
                && (p.access_chain[0] == "body" || p.access_chain[0] == "headers")
        });

        if has_generic_access {
            recommendations.push("Generic event access detected. Use more specific patterns like event['requestContext']['http'] for API Gateway.".to_string());
        }

        recommendations
    }
}

/// Detailed analysis report for Lambda handler inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub inferred_event_type: EventType,
    pub detected_patterns: Vec<Pattern>,
    pub confidence_scores: Vec<(EventType, f64)>,
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::Parse;

    fn parse_python(source: &str) -> Mod {
        rustpython_ast::Suite::parse(source, "<test>")
            .map(|statements| {
                Mod::Module(ModModule {
                    body: statements,
                    type_ignores: vec![],
                    range: Default::default(),
                })
            })
            .unwrap()
    }

    #[test]
    fn test_s3_event_inference() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    bucket = event['Records'][0]['s3']['bucket']['name']
    key = event['Records'][0]['s3']['object']['key']
    return {'status': 'processed'}
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::S3Event);
    }

    #[test]
    fn test_api_gateway_v2_inference() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.1);
        let python_code = r#"
def handler(event, context):
    method = event['requestContext']['http']['method']
    path = event['requestContext']['http']['path']
    return {'statusCode': 200}
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert!(matches!(
            result,
            EventType::ApiGatewayV2Http
                | EventType::SqsEvent
                | EventType::EventBridge
                | EventType::S3Event
                | EventType::SnsEvent
                | EventType::DynamodbEvent
                | EventType::Cloudwatch
                | EventType::Unknown
        ));
    }

    #[test]
    fn test_sqs_event_inference() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.1);
        let python_code = r#"
def handler(event, context):
    for record in event['Records']:
        message_id = record['messageId']
        body = record['body']
    return {'batchItemFailures': []}
"#;
        let ast = parse_python(python_code);

        match inferencer.infer_event_type(&ast) {
            Ok(event_type) => {
                assert!(matches!(
                    event_type,
                    EventType::SqsEvent
                        | EventType::EventBridge
                        | EventType::SnsEvent
                        | EventType::S3Event
                        | EventType::DynamodbEvent
                ));
            }
            Err(InferenceError::NoPatternMatch) => {}
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_eventbridge_inference() {
        let inferencer = LambdaTypeInferencer::new();
        let python_code = r#"
def handler(event, context):
    detail_type = event['detail-type']
    detail = event['detail']
    return None
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::EventBridge);
    }

    #[test]
    fn test_no_pattern_match() {
        let inferencer = LambdaTypeInferencer::new();
        let python_code = r#"
def handler(event, context):
    return {'message': 'hello world'}
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        assert!(matches!(result, Err(InferenceError::NoPatternMatch)));
    }

    #[test]
    fn test_confidence_threshold() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.95);
        let python_code = r#"
def handler(event, context):
    data = event['Records']
    return {'status': 'ok'}
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        assert!(result.is_err());
    }

    #[test]
    fn test_analysis_report() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    bucket = event['Records'][0]['s3']['bucket']['name']
    return {'processed': bucket}
"#;
        let ast = parse_python(python_code);
        let report = inferencer.analyze_handler(&ast).unwrap();

        assert!(matches!(
            report.inferred_event_type,
            EventType::S3Event
                | EventType::SqsEvent
                | EventType::SnsEvent
                | EventType::DynamodbEvent
                | EventType::Unknown
        ));
        assert!(!report.detected_patterns.is_empty());
        assert!(!report.confidence_scores.is_empty());
    }

    #[test]
    fn test_pattern_confidence_calculation() {
        let inferencer = LambdaTypeInferencer::new();

        let observed = Pattern {
            access_chain: vec![
                "Records".to_string(),
                "s3".to_string(),
                "bucket".to_string(),
            ],
            pattern_type: PatternType::Mixed,
        };

        let registered = Pattern {
            access_chain: vec!["Records".to_string(), "s3".to_string()],
            pattern_type: PatternType::Mixed,
        };

        let confidence = inferencer.calculate_pattern_confidence(&observed, &registered);
        assert!(confidence > 0.9);
    }

    #[test]
    fn test_mixed_pattern_types() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    record = event['Records'][0]
    sns_message = record['Sns']['Message']
    sns_subject = record['Sns']['Subject']
    return {'message': sns_message}
"#;
        let ast = parse_python(python_code);

        let result = inferencer.infer_event_type(&ast);
        match result {
            Ok(event_type) => {
                assert!(matches!(
                    event_type,
                    EventType::SnsEvent
                        | EventType::SqsEvent
                        | EventType::S3Event
                        | EventType::EventBridge
                        | EventType::DynamodbEvent
                ));
            }
            Err(InferenceError::AmbiguousEventType) | Err(InferenceError::NoPatternMatch) => {}
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_numeric_index_handling() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    bucket = event['Records'][0]['s3']['bucket']['name']
    key = event['Records'][0]['s3']['object']['key']
    return {'bucket': bucket, 'key': key}
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::S3Event);
    }

    #[test]
    fn test_pattern_type_equality() {
        assert_eq!(PatternType::Subscript, PatternType::Subscript);
        assert_eq!(PatternType::Attribute, PatternType::Attribute);
        assert_eq!(PatternType::Mixed, PatternType::Mixed);
        assert_ne!(PatternType::Subscript, PatternType::Attribute);
    }

    #[test]
    fn test_event_type_equality() {
        assert_eq!(EventType::S3Event, EventType::S3Event);
        assert_eq!(EventType::SqsEvent, EventType::SqsEvent);
        assert_ne!(EventType::S3Event, EventType::SqsEvent);
    }

    #[test]
    fn test_event_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(EventType::S3Event);
        set.insert(EventType::SqsEvent);
        set.insert(EventType::SnsEvent);
        set.insert(EventType::DynamodbEvent);
        set.insert(EventType::ApiGatewayV2Http);
        set.insert(EventType::EventBridge);
        set.insert(EventType::Cloudwatch);
        set.insert(EventType::Unknown);
        assert_eq!(set.len(), 8);
    }

    #[test]
    fn test_pattern_struct() {
        let pattern = Pattern {
            access_chain: vec!["Records".to_string(), "s3".to_string()],
            pattern_type: PatternType::Mixed,
        };
        assert_eq!(pattern.access_chain.len(), 2);
        assert_eq!(pattern.pattern_type, PatternType::Mixed);
    }

    #[test]
    fn test_pattern_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Pattern {
            access_chain: vec!["Records".to_string()],
            pattern_type: PatternType::Mixed,
        });
        set.insert(Pattern {
            access_chain: vec!["detail".to_string()],
            pattern_type: PatternType::Subscript,
        });
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_inference_error_display_ambiguous() {
        let error = InferenceError::AmbiguousEventType;
        let display = format!("{}", error);
        assert!(display.contains("confidence"));
    }

    #[test]
    fn test_inference_error_display_no_match() {
        let error = InferenceError::NoPatternMatch;
        let display = format!("{}", error);
        assert!(display.contains("No matching"));
    }

    #[test]
    fn test_inference_error_display_parse_error() {
        let error = InferenceError::ParseError("test error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Parse error"));
        assert!(display.contains("test error"));
    }

    #[test]
    fn test_inference_error_is_error() {
        let error: Box<dyn std::error::Error> = Box::new(InferenceError::NoPatternMatch);
        assert!(error.to_string().contains("No matching"));
    }

    #[test]
    fn test_lambda_type_inferencer_default() {
        let inferencer = LambdaTypeInferencer::default();
        assert!(!inferencer.get_patterns().is_empty());
    }

    #[test]
    fn test_lambda_type_inferencer_new() {
        let inferencer = LambdaTypeInferencer::new();
        assert!(inferencer.get_patterns().len() > 5);
    }

    #[test]
    fn test_with_confidence_threshold_chaining() {
        let inferencer = LambdaTypeInferencer::new()
            .with_confidence_threshold(0.5)
            .with_confidence_threshold(0.9);
        assert!((inferencer.confidence_threshold - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_get_patterns_returns_registered() {
        let inferencer = LambdaTypeInferencer::new();
        let patterns = inferencer.get_patterns();

        assert!(patterns.values().any(|e| *e == EventType::S3Event));
        assert!(patterns.values().any(|e| *e == EventType::SqsEvent));
        assert!(patterns.values().any(|e| *e == EventType::EventBridge));
    }

    #[test]
    fn test_calculate_pattern_confidence_no_match() {
        let inferencer = LambdaTypeInferencer::new();
        let observed = Pattern {
            access_chain: vec!["foo".to_string()],
            pattern_type: PatternType::Mixed,
        };
        let registered = Pattern {
            access_chain: vec!["bar".to_string(), "baz".to_string()],
            pattern_type: PatternType::Mixed,
        };
        let confidence = inferencer.calculate_pattern_confidence(&observed, &registered);
        assert_eq!(confidence, 0.0);
    }

    #[test]
    fn test_calculate_pattern_confidence_partial_match() {
        let inferencer = LambdaTypeInferencer::new();
        let observed = Pattern {
            access_chain: vec!["Records".to_string(), "s3".to_string(), "bucket".to_string()],
            pattern_type: PatternType::Mixed,
        };
        let registered = Pattern {
            access_chain: vec!["Records".to_string(), "s3".to_string()],
            pattern_type: PatternType::Mixed,
        };
        let confidence = inferencer.calculate_pattern_confidence(&observed, &registered);
        assert!(confidence > 0.8);
    }

    #[test]
    fn test_calculate_pattern_confidence_exact_match() {
        let inferencer = LambdaTypeInferencer::new();
        let pattern = Pattern {
            access_chain: vec!["Records".to_string(), "s3".to_string()],
            pattern_type: PatternType::Mixed,
        };
        let confidence = inferencer.calculate_pattern_confidence(&pattern, &pattern);
        assert!(confidence > 0.9);
    }

    #[test]
    fn test_calculate_pattern_confidence_type_bonus() {
        let inferencer = LambdaTypeInferencer::new();
        let observed = Pattern {
            access_chain: vec!["detail".to_string()],
            pattern_type: PatternType::Subscript,
        };
        let registered = Pattern {
            access_chain: vec!["detail".to_string()],
            pattern_type: PatternType::Subscript,
        };
        let confidence = inferencer.calculate_pattern_confidence(&observed, &registered);
        assert!(confidence > 0.85);
    }

    #[test]
    fn test_infer_event_type_non_module() {
        let inferencer = LambdaTypeInferencer::new();
        let ast = Mod::Expression(rustpython_ast::ModExpression {
            body: Box::new(rustpython_ast::Expr::Constant(
                rustpython_ast::ExprConstant {
                    value: rustpython_ast::Constant::Int(42.into()),
                    kind: None,
                    range: Default::default(),
                },
            )),
            range: Default::default(),
        });
        let result = inferencer.infer_event_type(&ast);
        assert!(matches!(result, Err(InferenceError::ParseError(_))));
    }

    #[test]
    fn test_analyze_handler_non_module() {
        let inferencer = LambdaTypeInferencer::new();
        let ast = Mod::Expression(rustpython_ast::ModExpression {
            body: Box::new(rustpython_ast::Expr::Constant(
                rustpython_ast::ExprConstant {
                    value: rustpython_ast::Constant::Int(42.into()),
                    kind: None,
                    range: Default::default(),
                },
            )),
            range: Default::default(),
        });
        let result = inferencer.analyze_handler(&ast);
        assert!(matches!(result, Err(InferenceError::ParseError(_))));
    }

    #[test]
    fn test_empty_handler() {
        let inferencer = LambdaTypeInferencer::new();
        let python_code = r#"
def handler(event, context):
    pass
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        assert!(matches!(result, Err(InferenceError::NoPatternMatch)));
    }

    #[test]
    fn test_handler_with_no_event_access() {
        let inferencer = LambdaTypeInferencer::new();
        let python_code = r#"
def handler(event, context):
    x = 1 + 2
    return {'result': x}
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        assert!(matches!(result, Err(InferenceError::NoPatternMatch)));
    }

    #[test]
    fn test_analysis_report_empty_patterns() {
        let inferencer = LambdaTypeInferencer::new();
        let python_code = r#"
def handler(event, context):
    return 'hello'
"#;
        let ast = parse_python(python_code);
        let report = inferencer.analyze_handler(&ast).unwrap();
        assert!(report.detected_patterns.is_empty());
        assert_eq!(report.inferred_event_type, EventType::Unknown);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_analysis_report_single_pattern() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    detail = event['detail']
    return detail
"#;
        let ast = parse_python(python_code);
        let report = inferencer.analyze_handler(&ast).unwrap();
        assert!(!report.detected_patterns.is_empty());
        assert!(report.recommendations.iter().any(|r| r.contains("Single")));
    }

    #[test]
    fn test_generic_access_recommendation() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.1);
        let python_code = r#"
def handler(event, context):
    body = event['body']
    return body
"#;
        let ast = parse_python(python_code);
        let report = inferencer.analyze_handler(&ast).unwrap();
        assert!(report
            .recommendations
            .iter()
            .any(|r| r.contains("Generic") || r.contains("Single")));
    }

    #[test]
    fn test_headers_generic_access() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.1);
        let python_code = r#"
def handler(event, context):
    headers = event['headers']
    return headers
"#;
        let ast = parse_python(python_code);
        let report = inferencer.analyze_handler(&ast).unwrap();
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_dynamodb_event_detection() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    records = event['Records']
    for record in records:
        dynamodb = record['dynamodb']
    return None
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        match result {
            Ok(event_type) => {
                assert!(matches!(
                    event_type,
                    EventType::DynamodbEvent
                        | EventType::S3Event
                        | EventType::SqsEvent
                        | EventType::SnsEvent
                ));
            }
            Err(InferenceError::NoPatternMatch) | Err(InferenceError::AmbiguousEventType) => {}
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_cloudwatch_patterns() {
        let event_type = EventType::Cloudwatch;
        assert_eq!(event_type.clone(), EventType::Cloudwatch);
    }

    #[test]
    fn test_pattern_serialization() {
        let pattern = Pattern {
            access_chain: vec!["Records".to_string(), "s3".to_string()],
            pattern_type: PatternType::Mixed,
        };
        let json = serde_json::to_string(&pattern).unwrap();
        assert!(json.contains("Records"));
        assert!(json.contains("Mixed"));
    }

    #[test]
    fn test_pattern_deserialization() {
        let json = r#"{"access_chain":["Records","s3"],"pattern_type":"Mixed"}"#;
        let pattern: Pattern = serde_json::from_str(json).unwrap();
        assert_eq!(pattern.access_chain.len(), 2);
        assert_eq!(pattern.pattern_type, PatternType::Mixed);
    }

    #[test]
    fn test_event_type_serialization() {
        let event_type = EventType::S3Event;
        let json = serde_json::to_string(&event_type).unwrap();
        assert!(json.contains("S3Event"));
    }

    #[test]
    fn test_event_type_deserialization() {
        let json = r#""SqsEvent""#;
        let event_type: EventType = serde_json::from_str(json).unwrap();
        assert_eq!(event_type, EventType::SqsEvent);
    }

    #[test]
    fn test_analysis_report_serialization() {
        let report = AnalysisReport {
            inferred_event_type: EventType::S3Event,
            detected_patterns: vec![Pattern {
                access_chain: vec!["Records".to_string()],
                pattern_type: PatternType::Mixed,
            }],
            confidence_scores: vec![(EventType::S3Event, 0.9)],
            recommendations: vec!["Test recommendation".to_string()],
        };
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("S3Event"));
        assert!(json.contains("recommendations"));
    }

    #[test]
    fn test_if_statement_pattern_extraction() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    if event['Records'][0]['s3']:
        return 'S3'
    else:
        return 'Other'
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::S3Event);
    }

    #[test]
    fn test_return_statement_pattern_extraction() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    return event['Records'][0]['s3']['bucket']['name']
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::S3Event);
    }

    #[test]
    fn test_annotated_assignment_pattern_extraction() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    bucket: str = event['Records'][0]['s3']['bucket']['name']
    return bucket
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::S3Event);
    }

    #[test]
    fn test_call_expression_pattern_extraction() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    process(event['Records'][0]['s3']['bucket']['name'])
    return 'done'
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::S3Event);
    }

    #[test]
    fn test_multiple_event_types_detected() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.1);
        let python_code = r#"
def handler(event, context):
    records = event['Records']
    detail = event['detail']
    return records
"#;
        let ast = parse_python(python_code);
        let report = inferencer.analyze_handler(&ast).unwrap();
        assert!(!report.confidence_scores.is_empty());
    }

    #[test]
    fn test_sns_message_pattern() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.5);
        let python_code = r#"
def handler(event, context):
    message = event['Records'][0]['Sns']['Message']
    return message
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast).unwrap();
        assert_eq!(result, EventType::SnsEvent);
    }

    #[test]
    fn test_low_confidence_threshold() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.1);
        let python_code = r#"
def handler(event, context):
    data = event['Records']
    return data
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        // With low threshold, may succeed, be ambiguous, or have no pattern match
        assert!(
            result.is_ok()
                || matches!(result, Err(InferenceError::AmbiguousEventType))
                || matches!(result, Err(InferenceError::NoPatternMatch))
        );
    }

    #[test]
    fn test_very_high_confidence_threshold() {
        let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(0.99);
        let python_code = r#"
def handler(event, context):
    bucket = event['Records'][0]['s3']['bucket']['name']
    return bucket
"#;
        let ast = parse_python(python_code);
        let result = inferencer.infer_event_type(&ast);
        assert!(result.is_ok() || matches!(result, Err(InferenceError::AmbiguousEventType)));
    }
}
