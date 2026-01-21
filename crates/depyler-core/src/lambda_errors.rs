use anyhow::Result;
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Lambda-specific error handling pipeline with automatic conversion from Python errors
#[derive(Debug, Clone)]
pub struct LambdaErrorHandler {
    error_mappings: HashMap<PythonErrorPattern, LambdaErrorMapping>,
    error_strategy: ErrorHandlingStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PythonErrorPattern {
    pub error_type: String,
    pub message_pattern: Option<String>,
    pub context: Option<ErrorContext>,
}

#[derive(Debug, Clone)]
pub struct LambdaErrorMapping {
    pub rust_error_type: String,
    pub status_code: Option<u16>,
    pub error_message_template: String,
    pub include_stack_trace: bool,
    pub retry_strategy: RetryStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorContext {
    Handler,
    Serialization,
    EventProcessing,
    ResponseGeneration,
    Initialization,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ErrorHandlingStrategy {
    Panic,
    #[default]
    ReturnError,
    LogAndContinue,
    CustomHandler(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetryStrategy {
    None,
    Immediate,
    ExponentialBackoff,
    Custom(String),
}

/// Lambda runtime errors that can occur during transpilation and execution
#[derive(Error, Debug)]
pub enum LambdaError {
    #[error("Serialization failed: {message}")]
    Serialization {
        message: String,
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Handler error: {message}")]
    Handler {
        message: String,
        context: Option<String>,
    },

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("HTTP error: {status} - {message}")]
    Http { status: u16, message: String },

    #[error("Missing parameter: {parameter}")]
    MissingParameter { parameter: String },

    #[error("Invalid event format: {message}")]
    InvalidEvent {
        message: String,
        event_type: Option<String>,
    },

    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    #[error("Authorization failed: {message}")]
    Authorization { message: String },

    #[error("Timeout occurred: {operation} took {duration_ms}ms")]
    Timeout { operation: String, duration_ms: u64 },

    #[error("Resource limit exceeded: {resource} - {limit}")]
    ResourceLimit { resource: String, limit: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },
}

impl LambdaError {
    pub fn status_code(&self) -> u16 {
        match self {
            LambdaError::MissingParameter { .. } => 400,
            LambdaError::Handler { .. } => 400,
            LambdaError::InvalidEvent { .. } => 400,
            LambdaError::Authentication { .. } => 401,
            LambdaError::Authorization { .. } => 403,
            LambdaError::Timeout { .. } => 504,
            LambdaError::ExternalService { .. } => 502,
            LambdaError::Http { status, .. } => *status,
            _ => 500,
        }
    }

    pub fn should_retry(&self) -> bool {
        match self {
            LambdaError::Timeout { .. } => true,
            LambdaError::ExternalService { .. } => true,
            LambdaError::Http { status, .. } => *status >= 500,
            _ => false,
        }
    }
}

/// Generated error conversion code for common Python error patterns
#[derive(Debug, Clone)]
pub struct ErrorConversionCode {
    pub conversion_functions: String,
    pub error_enum: String,
    pub helper_traits: String,
}

impl Default for LambdaErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaErrorHandler {
    pub fn new() -> Self {
        let mut error_mappings = HashMap::new();

        // Python KeyError mappings
        error_mappings.insert(
            PythonErrorPattern {
                error_type: "KeyError".to_string(),
                message_pattern: None,
                context: Some(ErrorContext::EventProcessing),
            },
            LambdaErrorMapping {
                rust_error_type: "LambdaError::MissingParameter".to_string(),
                status_code: Some(400),
                error_message_template: "Missing required parameter: {parameter}".to_string(),
                include_stack_trace: false,
                retry_strategy: RetryStrategy::None,
            },
        );

        // Python ValueError mappings
        error_mappings.insert(
            PythonErrorPattern {
                error_type: "ValueError".to_string(),
                message_pattern: None,
                context: Some(ErrorContext::Handler),
            },
            LambdaErrorMapping {
                rust_error_type: "LambdaError::Handler".to_string(),
                status_code: Some(400),
                error_message_template: "Invalid value: {message}".to_string(),
                include_stack_trace: false,
                retry_strategy: RetryStrategy::None,
            },
        );

        // Python TypeError mappings
        error_mappings.insert(
            PythonErrorPattern {
                error_type: "TypeError".to_string(),
                message_pattern: None,
                context: Some(ErrorContext::Serialization),
            },
            LambdaErrorMapping {
                rust_error_type: "LambdaError::Serialization".to_string(),
                status_code: Some(500),
                error_message_template: "Type conversion error: {message}".to_string(),
                include_stack_trace: true,
                retry_strategy: RetryStrategy::None,
            },
        );

        // JSON decode errors
        error_mappings.insert(
            PythonErrorPattern {
                error_type: "json.JSONDecodeError".to_string(),
                message_pattern: None,
                context: Some(ErrorContext::Serialization),
            },
            LambdaErrorMapping {
                rust_error_type: "LambdaError::Serialization".to_string(),
                status_code: Some(400),
                error_message_template: "Invalid JSON: {message}".to_string(),
                include_stack_trace: false,
                retry_strategy: RetryStrategy::None,
            },
        );

        // HTTP-related errors
        error_mappings.insert(
            PythonErrorPattern {
                error_type: "requests.HTTPError".to_string(),
                message_pattern: None,
                context: Some(ErrorContext::Handler),
            },
            LambdaErrorMapping {
                rust_error_type: "LambdaError::ExternalService".to_string(),
                status_code: Some(502),
                error_message_template: "External service error: {message}".to_string(),
                include_stack_trace: false,
                retry_strategy: RetryStrategy::ExponentialBackoff,
            },
        );

        // Timeout errors
        error_mappings.insert(
            PythonErrorPattern {
                error_type: "TimeoutError".to_string(),
                message_pattern: None,
                context: Some(ErrorContext::Handler),
            },
            LambdaErrorMapping {
                rust_error_type: "LambdaError::Timeout".to_string(),
                status_code: Some(504),
                error_message_template: "Operation timed out: {message}".to_string(),
                include_stack_trace: false,
                retry_strategy: RetryStrategy::Immediate,
            },
        );

        Self {
            error_mappings,
            error_strategy: ErrorHandlingStrategy::default(),
        }
    }

    pub fn with_strategy(mut self, strategy: ErrorHandlingStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Generate error handling code for Lambda functions
    pub fn generate_error_handling_code(&self) -> Result<ErrorConversionCode> {
        let conversion_functions = self.generate_conversion_functions();
        let error_enum = self.generate_error_enum();
        let helper_traits = self.generate_helper_traits();

        Ok(ErrorConversionCode {
            conversion_functions,
            error_enum,
            helper_traits,
        })
    }

    /// Generate Rust error enum definition
    fn generate_error_enum(&self) -> String {
        r#"#[derive(Debug, thiserror::Error)]
pub enum LambdaError {
    #[error("Serialization failed: {message}")]
    Serialization {
        message: String,
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    #[error("Handler error: {message}")]
    Handler {
        message: String,
        context: Option<String>,
    },
    
    #[error("Runtime error: {0}")]
    Runtime(#[from] lambda_runtime::Error),
    
    #[error("HTTP error: {status} - {message}")]
    Http {
        status: u16,
        message: String,
    },
    
    #[error("Missing parameter: {parameter}")]
    MissingParameter {
        parameter: String,
    },
    
    #[error("Invalid event format: {message}")]
    InvalidEvent {
        message: String,
        event_type: Option<String>,
    },
    
    #[error("Authentication failed: {message}")]
    Authentication {
        message: String,
    },
    
    #[error("Authorization failed: {message}")]
    Authorization {
        message: String,
    },
    
    #[error("Timeout occurred: {operation} took {duration_ms}ms")]
    Timeout {
        operation: String,
        duration_ms: u64,
    },
    
    #[error("Resource limit exceeded: {resource} - {limit}")]
    ResourceLimit {
        resource: String,
        limit: String,
    },
    
    #[error("Configuration error: {message}")]
    Configuration {
        message: String,
    },
    
    #[error("External service error: {service} - {message}")]
    ExternalService {
        service: String,
        message: String,
    },
}

impl LambdaError {
    pub fn status_code(&self) -> u16 {
        match self {
            LambdaError::MissingParameter { .. } => 400,
            LambdaError::Handler { .. } => 400,
            LambdaError::InvalidEvent { .. } => 400,
            LambdaError::Authentication { .. } => 401,
            LambdaError::Authorization { .. } => 403,
            LambdaError::Timeout { .. } => 504,
            LambdaError::ExternalService { .. } => 502,
            LambdaError::Http { status, .. } => *status,
            _ => 500,
        }
    }
    
    pub fn should_retry(&self) -> bool {
        match self {
            LambdaError::Timeout { .. } => true,
            LambdaError::ExternalService { .. } => true,
            LambdaError::Http { status, .. } => *status >= 500,
            _ => false,
        }
    }
}
"#
        .to_string()
    }

    /// Generate conversion functions from Python error patterns
    fn generate_conversion_functions(&self) -> String {
        let mut functions = String::new();

        functions.push_str(
            r#"// Automatic error conversion functions
impl From<serde_json::Error> for LambdaError {{
    fn from(err: serde_json::Error) -> Self {{
        LambdaError::Serialization {{
            message: err.to_string(),
            cause: Some(Box::new(err)),
        }}
    }}
}}

impl From<&str> for LambdaError {{
    fn from(msg: &str) -> Self {{
        // Pattern matching on common Python error messages
        if msg.contains("KeyError") {{
            let parameter = extract_key_error_parameter(msg).unwrap_or_else(|| "unknown".to_string());
            LambdaError::MissingParameter {{ parameter }}
        }} else if msg.contains("ValueError") {{
            LambdaError::Handler {{
                message: msg.to_string(),
                context: Some("ValueError".to_string()),
            }}
        }} else if msg.contains("TypeError") {{
            LambdaError::Serialization {{
                message: msg.to_string(),
                cause: None,
            }}
        }} else if msg.contains("TimeoutError") {{
            LambdaError::Timeout {{
                operation: "unknown".to_string(),
                duration_ms: 0,
            }}
        }} else {{
            LambdaError::Handler {{
                message: msg.to_string(),
                context: None,
            }}
        }}
    }}
}}

fn extract_key_error_parameter(error_msg: &str) -> Option<String> {{
    // Extract parameter name from KeyError messages like "KeyError: 'param_name'"
    if let Some(start) = error_msg.find("'") {{
        if let Some(end) = error_msg[start + 1..].find("'") {{
            return Some(error_msg[start + 1..start + 1 + end].to_string());
        }}
    }}
    None
}}

"#
        );

        // Generate API Gateway specific error handling
        functions.push_str(
            r#"// API Gateway specific error handling
impl From<LambdaError> for aws_lambda_events::apigw::ApiGatewayProxyResponse {{
    fn from(err: LambdaError) -> Self {{
        let status_code = err.status_code();
        let error_body = serde_json::json!({{
            "error": {{
                "message": err.to_string(),
                "type": match &err {{
                    LambdaError::MissingParameter {{ .. }} => "MissingParameter",
                    LambdaError::Handler {{ .. }} => "HandlerError",
                    LambdaError::Serialization {{ .. }} => "SerializationError",
                    LambdaError::Timeout {{ .. }} => "TimeoutError",
                    _ => "InternalError",
                }},
                "retryable": err.should_retry(),
            }}
        }});
        
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        aws_lambda_events::apigw::ApiGatewayProxyResponse {{
            status_code,
            headers,
            multi_value_headers: std::collections::HashMap::new(),
            body: Some(error_body.to_string()),
            is_base64_encoded: false,
        }}
    }}
}}

// API Gateway v2 specific error handling
impl From<LambdaError> for aws_lambda_events::apigw::ApiGatewayV2httpResponse {{
    fn from(err: LambdaError) -> Self {{
        let status_code = err.status_code();
        let error_body = serde_json::json!({{
            "error": {{
                "message": err.to_string(),
                "type": match &err {{
                    LambdaError::MissingParameter {{ .. }} => "MissingParameter",
                    LambdaError::Handler {{ .. }} => "HandlerError",
                    LambdaError::Serialization {{ .. }} => "SerializationError",
                    LambdaError::Timeout {{ .. }} => "TimeoutError",
                    _ => "InternalError",
                }},
                "retryable": err.should_retry(),
            }}
        }});
        
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        aws_lambda_events::apigw::ApiGatewayV2httpResponse {{
            status_code,
            headers,
            body: Some(error_body.to_string()),
            is_base64_encoded: Some(false),
            cookies: vec![],
        }}
    }}
}}

"#,
        );

        functions
    }

    /// Generate helper traits for error handling
    fn generate_helper_traits(&self) -> String {
        r#"// Helper traits for error handling
pub trait LambdaErrorExt {
    fn with_context(self, context: &str) -> LambdaError;
    fn with_parameter(self, parameter: &str) -> LambdaError;
    fn with_status(self, status: u16) -> LambdaError;
}

impl LambdaErrorExt for String {{
    fn with_context(self, context: &str) -> LambdaError {{
        LambdaError::Handler {{
            message: self,
            context: Some(context.to_string()),
        }}
    }}
    
    fn with_parameter(self, parameter: &str) -> LambdaError {{
        LambdaError::MissingParameter {{
            parameter: parameter.to_string(),
        }}
    }}
    
    fn with_status(self, status: u16) -> LambdaError {{
        LambdaError::Http {{
            status,
            message: self,
        }}
    }}
}}

impl LambdaErrorExt for &str {{
    fn with_context(self, context: &str) -> LambdaError {{
        self.to_string().with_context(context)
    }}
    
    fn with_parameter(self, parameter: &str) -> LambdaError {{
        self.to_string().with_parameter(parameter)
    }}
    
    fn with_status(self, status: u16) -> LambdaError {{
        self.to_string().with_status(status)
    }}
}}

// Result type alias for Lambda functions
pub type LambdaResult<T> = std::result::Result<T, LambdaError>;

// Macro for easy error creation
#[macro_export]
macro_rules! lambda_error {{
    ($msg:expr) => {{
        LambdaError::Handler {{
            message: $msg.to_string(),
            context: None,
        }}
    }};
    ($msg:expr, $context:expr) => {{
        LambdaError::Handler {{
            message: $msg.to_string(),
            context: Some($context.to_string()),
        }}
    }};
}}

// Macro for parameter validation
#[macro_export]
macro_rules! require_param {{
    ($event:expr, $key:expr) => {{
        $event.get($key).ok_or_else(|| {{
            LambdaError::MissingParameter {{
                parameter: $key.to_string(),
            }}
        }})
    }};
}}

"#
        .to_string()
    }

    /// Generate error handling wrapper for handler functions
    pub fn generate_handler_wrapper(&self, handler_name: &str) -> String {
        match &self.error_strategy {
            ErrorHandlingStrategy::ReturnError => {
                format!(
                    r#"// Error handling wrapper for {handler_name}
async fn {handler_name}_with_error_handling(
    event: LambdaEvent<serde_json::Value>
) -> Result<serde_json::Value, LambdaError> {{
    match {handler_name}(event).await {{
        Ok(response) => Ok(response),
        Err(err) => {{
            // Log the error for debugging
            eprintln!("Handler error: {{:?}}", err);
            
            // Return appropriate error response
            Err(err.into())
        }}
    }}
}}
"#
                )
            }
            ErrorHandlingStrategy::LogAndContinue => {
                format!(
                    r#"// Error handling wrapper for {handler_name} (log and continue)
async fn {handler_name}_with_error_handling(
    event: LambdaEvent<serde_json::Value>
) -> Result<serde_json::Value, LambdaError> {{
    match {handler_name}(event).await {{
        Ok(response) => Ok(response),
        Err(err) => {{
            // Log the error
            eprintln!("Handler error (continuing): {{:?}}", err);
            
            // Return default response
            Ok(serde_json::json!({{
                "status": "error_logged",
                "message": "An error occurred but was handled"
            }}))
        }}
    }}
}}
"#
                )
            }
            ErrorHandlingStrategy::Panic => {
                format!(
                    r#"// Error handling wrapper for {handler_name} (panic on error)
async fn {handler_name}_with_error_handling(
    event: LambdaEvent<serde_json::Value>
) -> Result<serde_json::Value, LambdaError> {{
    match {handler_name}(event).await {{
        Ok(response) => Ok(response),
        Err(err) => {{
            eprintln!("Handler error (panicking): {{:?}}", err);
            panic!("Handler failed: {{}}", err);
        }}
    }}
}}
"#
                )
            }
            ErrorHandlingStrategy::CustomHandler(custom_code) => {
                format!(
                    r#"// Custom error handling wrapper for {handler_name}
async fn {handler_name}_with_error_handling(
    event: LambdaEvent<serde_json::Value>
) -> Result<serde_json::Value, LambdaError> {{
    match {handler_name}(event).await {{
        Ok(response) => Ok(response),
        Err(err) => {{
            {custom_code}
        }}
    }}
}}
"#
                )
            }
        }
    }

    /// Generate retry logic for Lambda functions
    pub fn generate_retry_logic(&self) -> String {
        r#"// Retry logic for Lambda functions
pub struct RetryConfig {{
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}}

impl Default for RetryConfig {{
    fn default() -> Self {{
        Self {{
            max_attempts: 3,
            base_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }}
    }}
}}

pub async fn retry_with_backoff<F, T, E>(
    config: &RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
    E: std::fmt::Debug,
{{
    let mut last_error = None;
    let mut delay = config.base_delay_ms;
    
    for attempt in 1..=config.max_attempts {{
        match operation().await {{
            Ok(result) => return Ok(result),
            Err(err) => {{
                eprintln!("Attempt {{}} failed: {{:?}}", attempt, err);
                last_error = Some(err);
                
                if attempt < config.max_attempts {{
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                    delay = ((delay as f64 * config.backoff_multiplier) as u64).min(config.max_delay_ms);
                }}
            }}
        }}
    }}
    
    Err(last_error.unwrap())
}}

"#.to_string()
    }

    /// Add custom error mapping
    pub fn add_error_mapping(&mut self, pattern: PythonErrorPattern, mapping: LambdaErrorMapping) {
        self.error_mappings.insert(pattern, mapping);
    }

    /// Get error mapping for a Python error pattern
    pub fn get_error_mapping(&self, pattern: &PythonErrorPattern) -> Option<&LambdaErrorMapping> {
        self.error_mappings.get(pattern)
    }
}

impl fmt::Display for PythonErrorPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_type)?;
        if let Some(ref message) = self.message_pattern {
            write!(f, " ({message})")?;
        }
        if let Some(ref context) = self.context {
            write!(f, " in {context:?}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === PythonErrorPattern tests ===

    #[test]
    fn test_python_error_pattern_fields() {
        let pattern = PythonErrorPattern {
            error_type: "ValueError".to_string(),
            message_pattern: Some("invalid input".to_string()),
            context: Some(ErrorContext::Handler),
        };
        assert_eq!(pattern.error_type, "ValueError");
        assert_eq!(pattern.message_pattern, Some("invalid input".to_string()));
        assert_eq!(pattern.context, Some(ErrorContext::Handler));
    }

    #[test]
    fn test_python_error_pattern_clone() {
        let pattern = PythonErrorPattern {
            error_type: "KeyError".to_string(),
            message_pattern: None,
            context: None,
        };
        let cloned = pattern.clone();
        assert_eq!(cloned.error_type, "KeyError");
    }

    #[test]
    fn test_python_error_pattern_debug() {
        let pattern = PythonErrorPattern {
            error_type: "TypeError".to_string(),
            message_pattern: None,
            context: None,
        };
        let debug = format!("{:?}", pattern);
        assert!(debug.contains("PythonErrorPattern"));
        assert!(debug.contains("TypeError"));
    }

    #[test]
    fn test_python_error_pattern_eq() {
        let p1 = PythonErrorPattern {
            error_type: "KeyError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Handler),
        };
        let p2 = PythonErrorPattern {
            error_type: "KeyError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Handler),
        };
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_python_error_pattern_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let p1 = PythonErrorPattern {
            error_type: "ValueError".to_string(),
            message_pattern: None,
            context: None,
        };
        set.insert(p1.clone());
        assert!(set.contains(&p1));
    }

    #[test]
    fn test_python_error_pattern_display() {
        let pattern = PythonErrorPattern {
            error_type: "KeyError".to_string(),
            message_pattern: Some("missing key".to_string()),
            context: Some(ErrorContext::EventProcessing),
        };
        let display = format!("{}", pattern);
        assert!(display.contains("KeyError"));
        assert!(display.contains("missing key"));
        assert!(display.contains("EventProcessing"));
    }

    #[test]
    fn test_python_error_pattern_display_no_optionals() {
        let pattern = PythonErrorPattern {
            error_type: "RuntimeError".to_string(),
            message_pattern: None,
            context: None,
        };
        let display = format!("{}", pattern);
        assert_eq!(display, "RuntimeError");
    }

    // === LambdaErrorMapping tests ===

    #[test]
    fn test_lambda_error_mapping_fields() {
        let mapping = LambdaErrorMapping {
            rust_error_type: "LambdaError::Handler".to_string(),
            status_code: Some(400),
            error_message_template: "Error: {message}".to_string(),
            include_stack_trace: true,
            retry_strategy: RetryStrategy::None,
        };
        assert_eq!(mapping.rust_error_type, "LambdaError::Handler");
        assert_eq!(mapping.status_code, Some(400));
        assert!(mapping.include_stack_trace);
    }

    #[test]
    fn test_lambda_error_mapping_clone() {
        let mapping = LambdaErrorMapping {
            rust_error_type: "LambdaError::Timeout".to_string(),
            status_code: Some(504),
            error_message_template: "Timeout".to_string(),
            include_stack_trace: false,
            retry_strategy: RetryStrategy::Immediate,
        };
        let cloned = mapping.clone();
        assert_eq!(cloned.status_code, Some(504));
    }

    #[test]
    fn test_lambda_error_mapping_debug() {
        let mapping = LambdaErrorMapping {
            rust_error_type: "LambdaError::Runtime".to_string(),
            status_code: None,
            error_message_template: "Runtime error".to_string(),
            include_stack_trace: false,
            retry_strategy: RetryStrategy::ExponentialBackoff,
        };
        let debug = format!("{:?}", mapping);
        assert!(debug.contains("LambdaErrorMapping"));
    }

    // === ErrorContext tests ===

    #[test]
    fn test_error_context_variants() {
        assert_eq!(ErrorContext::Handler, ErrorContext::Handler);
        assert_eq!(ErrorContext::Serialization, ErrorContext::Serialization);
        assert_eq!(ErrorContext::EventProcessing, ErrorContext::EventProcessing);
        assert_eq!(
            ErrorContext::ResponseGeneration,
            ErrorContext::ResponseGeneration
        );
        assert_eq!(ErrorContext::Initialization, ErrorContext::Initialization);
    }

    #[test]
    fn test_error_context_clone() {
        let ctx = ErrorContext::Handler;
        let cloned = ctx.clone();
        assert_eq!(cloned, ErrorContext::Handler);
    }

    #[test]
    fn test_error_context_debug() {
        let ctx = ErrorContext::Serialization;
        let debug = format!("{:?}", ctx);
        assert!(debug.contains("Serialization"));
    }

    #[test]
    fn test_error_context_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ErrorContext::Handler);
        set.insert(ErrorContext::Serialization);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&ErrorContext::Handler));
    }

    #[test]
    fn test_error_context_ne() {
        assert_ne!(ErrorContext::Handler, ErrorContext::Serialization);
    }

    // === ErrorHandlingStrategy tests ===

    #[test]
    fn test_error_handling_strategy_variants() {
        let strategies = [ErrorHandlingStrategy::Panic,
            ErrorHandlingStrategy::ReturnError,
            ErrorHandlingStrategy::LogAndContinue,
            ErrorHandlingStrategy::CustomHandler("custom".to_string())];
        assert_eq!(strategies.len(), 4);
    }

    #[test]
    fn test_error_handling_strategy_default() {
        let default = ErrorHandlingStrategy::default();
        assert_eq!(default, ErrorHandlingStrategy::ReturnError);
    }

    #[test]
    fn test_error_handling_strategy_clone() {
        let strategy = ErrorHandlingStrategy::Panic;
        let cloned = strategy.clone();
        assert_eq!(cloned, ErrorHandlingStrategy::Panic);
    }

    #[test]
    fn test_error_handling_strategy_debug() {
        let strategy = ErrorHandlingStrategy::LogAndContinue;
        let debug = format!("{:?}", strategy);
        assert!(debug.contains("LogAndContinue"));
    }

    #[test]
    fn test_error_handling_strategy_eq() {
        assert_eq!(ErrorHandlingStrategy::Panic, ErrorHandlingStrategy::Panic);
        assert_ne!(
            ErrorHandlingStrategy::Panic,
            ErrorHandlingStrategy::ReturnError
        );
    }

    #[test]
    fn test_error_handling_strategy_custom_handler() {
        let custom = ErrorHandlingStrategy::CustomHandler("my_handler()".to_string());
        if let ErrorHandlingStrategy::CustomHandler(code) = custom {
            assert_eq!(code, "my_handler()");
        } else {
            panic!("Expected CustomHandler");
        }
    }

    // === RetryStrategy tests ===

    #[test]
    fn test_retry_strategy_variants() {
        let strategies = [RetryStrategy::None,
            RetryStrategy::Immediate,
            RetryStrategy::ExponentialBackoff,
            RetryStrategy::Custom("custom".to_string())];
        assert_eq!(strategies.len(), 4);
    }

    #[test]
    fn test_retry_strategy_clone() {
        let strategy = RetryStrategy::ExponentialBackoff;
        let cloned = strategy.clone();
        assert_eq!(cloned, RetryStrategy::ExponentialBackoff);
    }

    #[test]
    fn test_retry_strategy_debug() {
        let strategy = RetryStrategy::Immediate;
        let debug = format!("{:?}", strategy);
        assert!(debug.contains("Immediate"));
    }

    #[test]
    fn test_retry_strategy_eq() {
        assert_eq!(RetryStrategy::None, RetryStrategy::None);
        assert_ne!(RetryStrategy::None, RetryStrategy::Immediate);
    }

    #[test]
    fn test_retry_strategy_custom() {
        let custom = RetryStrategy::Custom("retry_with_jitter".to_string());
        if let RetryStrategy::Custom(code) = custom {
            assert_eq!(code, "retry_with_jitter");
        } else {
            panic!("Expected Custom");
        }
    }

    // === LambdaError tests ===

    #[test]
    fn test_lambda_error_serialization() {
        let err = LambdaError::Serialization {
            message: "JSON parse failed".to_string(),
            cause: None,
        };
        assert_eq!(err.status_code(), 500);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_handler() {
        let err = LambdaError::Handler {
            message: "Handler failed".to_string(),
            context: Some("validation".to_string()),
        };
        assert_eq!(err.status_code(), 400);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_runtime() {
        let err = LambdaError::Runtime("runtime crashed".to_string());
        assert_eq!(err.status_code(), 500);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_http() {
        let err = LambdaError::Http {
            status: 404,
            message: "Not found".to_string(),
        };
        assert_eq!(err.status_code(), 404);
        assert!(!err.should_retry());

        let err_500 = LambdaError::Http {
            status: 503,
            message: "Service unavailable".to_string(),
        };
        assert!(err_500.should_retry());
    }

    #[test]
    fn test_lambda_error_invalid_event() {
        let err = LambdaError::InvalidEvent {
            message: "Invalid format".to_string(),
            event_type: Some("S3Event".to_string()),
        };
        assert_eq!(err.status_code(), 400);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_authentication() {
        let err = LambdaError::Authentication {
            message: "Invalid token".to_string(),
        };
        assert_eq!(err.status_code(), 401);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_authorization() {
        let err = LambdaError::Authorization {
            message: "Access denied".to_string(),
        };
        assert_eq!(err.status_code(), 403);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_resource_limit() {
        let err = LambdaError::ResourceLimit {
            resource: "memory".to_string(),
            limit: "128MB".to_string(),
        };
        assert_eq!(err.status_code(), 500);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_configuration() {
        let err = LambdaError::Configuration {
            message: "Missing env var".to_string(),
        };
        assert_eq!(err.status_code(), 500);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_lambda_error_external_service() {
        let err = LambdaError::ExternalService {
            service: "DynamoDB".to_string(),
            message: "Throttled".to_string(),
        };
        assert_eq!(err.status_code(), 502);
        assert!(err.should_retry());
    }

    #[test]
    fn test_lambda_error_display_serialization() {
        let err = LambdaError::Serialization {
            message: "parse error".to_string(),
            cause: None,
        };
        let display = err.to_string();
        assert!(display.contains("Serialization failed"));
        assert!(display.contains("parse error"));
    }

    #[test]
    fn test_lambda_error_display_handler() {
        let err = LambdaError::Handler {
            message: "invalid input".to_string(),
            context: None,
        };
        let display = err.to_string();
        assert!(display.contains("Handler error"));
    }

    #[test]
    fn test_lambda_error_display_runtime() {
        let err = LambdaError::Runtime("panic occurred".to_string());
        let display = err.to_string();
        assert!(display.contains("Runtime error"));
        assert!(display.contains("panic occurred"));
    }

    #[test]
    fn test_lambda_error_display_http() {
        let err = LambdaError::Http {
            status: 500,
            message: "Internal error".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("HTTP error"));
        assert!(display.contains("500"));
    }

    #[test]
    fn test_lambda_error_display_missing_parameter() {
        let err = LambdaError::MissingParameter {
            parameter: "user_id".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("Missing parameter"));
        assert!(display.contains("user_id"));
    }

    #[test]
    fn test_lambda_error_display_invalid_event() {
        let err = LambdaError::InvalidEvent {
            message: "bad format".to_string(),
            event_type: Some("SQS".to_string()),
        };
        let display = err.to_string();
        assert!(display.contains("Invalid event format"));
    }

    #[test]
    fn test_lambda_error_display_authentication() {
        let err = LambdaError::Authentication {
            message: "expired token".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("Authentication failed"));
    }

    #[test]
    fn test_lambda_error_display_authorization() {
        let err = LambdaError::Authorization {
            message: "insufficient permissions".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("Authorization failed"));
    }

    #[test]
    fn test_lambda_error_display_timeout() {
        let err = LambdaError::Timeout {
            operation: "db_query".to_string(),
            duration_ms: 30000,
        };
        let display = err.to_string();
        assert!(display.contains("Timeout"));
        assert!(display.contains("db_query"));
        assert!(display.contains("30000"));
    }

    #[test]
    fn test_lambda_error_display_resource_limit() {
        let err = LambdaError::ResourceLimit {
            resource: "CPU".to_string(),
            limit: "100%".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("Resource limit exceeded"));
    }

    #[test]
    fn test_lambda_error_display_configuration() {
        let err = LambdaError::Configuration {
            message: "invalid config".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("Configuration error"));
    }

    #[test]
    fn test_lambda_error_display_external_service() {
        let err = LambdaError::ExternalService {
            service: "S3".to_string(),
            message: "bucket not found".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("External service error"));
        assert!(display.contains("S3"));
    }

    // === ErrorConversionCode tests ===

    #[test]
    fn test_error_conversion_code_fields() {
        let code = ErrorConversionCode {
            conversion_functions: "fn convert()".to_string(),
            error_enum: "enum Error {}".to_string(),
            helper_traits: "trait Helper {}".to_string(),
        };
        assert!(code.conversion_functions.contains("convert"));
        assert!(code.error_enum.contains("enum"));
        assert!(code.helper_traits.contains("trait"));
    }

    #[test]
    fn test_error_conversion_code_clone() {
        let code = ErrorConversionCode {
            conversion_functions: "fn a()".to_string(),
            error_enum: "enum E {}".to_string(),
            helper_traits: "trait T {}".to_string(),
        };
        let cloned = code.clone();
        assert_eq!(cloned.conversion_functions, "fn a()");
    }

    #[test]
    fn test_error_conversion_code_debug() {
        let code = ErrorConversionCode {
            conversion_functions: "code".to_string(),
            error_enum: "enum".to_string(),
            helper_traits: "trait".to_string(),
        };
        let debug = format!("{:?}", code);
        assert!(debug.contains("ErrorConversionCode"));
    }

    // === LambdaErrorHandler tests ===

    #[test]
    fn test_lambda_error_handler_new() {
        let handler = LambdaErrorHandler::new();
        // Should have default error mappings
        let pattern = PythonErrorPattern {
            error_type: "KeyError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::EventProcessing),
        };
        assert!(handler.get_error_mapping(&pattern).is_some());
    }

    #[test]
    fn test_lambda_error_handler_default() {
        let handler = LambdaErrorHandler::default();
        let code = handler.generate_error_handling_code().unwrap();
        assert!(!code.error_enum.is_empty());
    }

    #[test]
    fn test_lambda_error_handler_clone() {
        let handler = LambdaErrorHandler::new();
        let cloned = handler.clone();
        let code = cloned.generate_error_handling_code().unwrap();
        assert!(code.error_enum.contains("LambdaError"));
    }

    #[test]
    fn test_lambda_error_handler_debug() {
        let handler = LambdaErrorHandler::new();
        let debug = format!("{:?}", handler);
        assert!(debug.contains("LambdaErrorHandler"));
    }

    #[test]
    fn test_lambda_error_handler_with_strategy() {
        let handler = LambdaErrorHandler::new().with_strategy(ErrorHandlingStrategy::Panic);
        let wrapper = handler.generate_handler_wrapper("test");
        assert!(wrapper.contains("panicking"));
    }

    #[test]
    fn test_lambda_error_handler_get_mapping_not_found() {
        let handler = LambdaErrorHandler::new();
        let pattern = PythonErrorPattern {
            error_type: "UnknownError".to_string(),
            message_pattern: None,
            context: None,
        };
        assert!(handler.get_error_mapping(&pattern).is_none());
    }

    #[test]
    fn test_lambda_error_handler_default_mappings() {
        let handler = LambdaErrorHandler::new();

        // Test ValueError mapping
        let value_error = PythonErrorPattern {
            error_type: "ValueError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Handler),
        };
        let mapping = handler.get_error_mapping(&value_error).unwrap();
        assert_eq!(mapping.status_code, Some(400));

        // Test TypeError mapping
        let type_error = PythonErrorPattern {
            error_type: "TypeError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Serialization),
        };
        let mapping = handler.get_error_mapping(&type_error).unwrap();
        assert_eq!(mapping.status_code, Some(500));
    }

    #[test]
    fn test_lambda_error_handler_json_decode_error_mapping() {
        let handler = LambdaErrorHandler::new();
        let pattern = PythonErrorPattern {
            error_type: "json.JSONDecodeError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Serialization),
        };
        let mapping = handler.get_error_mapping(&pattern).unwrap();
        assert_eq!(mapping.status_code, Some(400));
    }

    #[test]
    fn test_lambda_error_handler_http_error_mapping() {
        let handler = LambdaErrorHandler::new();
        let pattern = PythonErrorPattern {
            error_type: "requests.HTTPError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Handler),
        };
        let mapping = handler.get_error_mapping(&pattern).unwrap();
        assert_eq!(mapping.retry_strategy, RetryStrategy::ExponentialBackoff);
    }

    #[test]
    fn test_lambda_error_handler_timeout_error_mapping() {
        let handler = LambdaErrorHandler::new();
        let pattern = PythonErrorPattern {
            error_type: "TimeoutError".to_string(),
            message_pattern: None,
            context: Some(ErrorContext::Handler),
        };
        let mapping = handler.get_error_mapping(&pattern).unwrap();
        assert_eq!(mapping.retry_strategy, RetryStrategy::Immediate);
    }

    // === Handler wrapper generation tests ===

    #[test]
    fn test_handler_wrapper_return_error() {
        let handler = LambdaErrorHandler::new().with_strategy(ErrorHandlingStrategy::ReturnError);
        let wrapper = handler.generate_handler_wrapper("my_func");
        assert!(wrapper.contains("my_func_with_error_handling"));
        assert!(wrapper.contains("Err(err.into())"));
    }

    #[test]
    fn test_handler_wrapper_log_and_continue() {
        let handler =
            LambdaErrorHandler::new().with_strategy(ErrorHandlingStrategy::LogAndContinue);
        let wrapper = handler.generate_handler_wrapper("my_func");
        assert!(wrapper.contains("log and continue"));
        assert!(wrapper.contains("error_logged"));
    }

    #[test]
    fn test_handler_wrapper_panic() {
        let handler = LambdaErrorHandler::new().with_strategy(ErrorHandlingStrategy::Panic);
        let wrapper = handler.generate_handler_wrapper("my_func");
        assert!(wrapper.contains("panic!"));
    }

    #[test]
    fn test_handler_wrapper_custom() {
        let custom_code = "log_error(&err); return Err(err)".to_string();
        let handler = LambdaErrorHandler::new()
            .with_strategy(ErrorHandlingStrategy::CustomHandler(custom_code.clone()));
        let wrapper = handler.generate_handler_wrapper("my_func");
        assert!(wrapper.contains(&custom_code));
    }

    // === Code generation tests ===

    #[test]
    fn test_generate_error_enum_content() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        // Check all error variants are present
        assert!(code.error_enum.contains("Serialization"));
        assert!(code.error_enum.contains("Handler"));
        assert!(code.error_enum.contains("Runtime"));
        assert!(code.error_enum.contains("Http"));
        assert!(code.error_enum.contains("MissingParameter"));
        assert!(code.error_enum.contains("InvalidEvent"));
        assert!(code.error_enum.contains("Authentication"));
        assert!(code.error_enum.contains("Authorization"));
        assert!(code.error_enum.contains("Timeout"));
        assert!(code.error_enum.contains("ResourceLimit"));
        assert!(code.error_enum.contains("Configuration"));
        assert!(code.error_enum.contains("ExternalService"));
    }

    #[test]
    fn test_generate_conversion_functions_content() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        // Check From implementations
        assert!(code.conversion_functions.contains("From<serde_json::Error>"));
        assert!(code.conversion_functions.contains("From<&str>"));
        assert!(code.conversion_functions.contains("KeyError"));
        assert!(code.conversion_functions.contains("ValueError"));
        assert!(code.conversion_functions.contains("TypeError"));
        assert!(code.conversion_functions.contains("TimeoutError"));
    }

    #[test]
    fn test_generate_conversion_functions_api_gateway() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        // Check API Gateway conversions
        assert!(code
            .conversion_functions
            .contains("ApiGatewayProxyResponse"));
        assert!(code
            .conversion_functions
            .contains("ApiGatewayV2httpResponse"));
    }

    #[test]
    fn test_generate_helper_traits_content() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        // Check helper traits
        assert!(code.helper_traits.contains("LambdaErrorExt"));
        assert!(code.helper_traits.contains("with_context"));
        assert!(code.helper_traits.contains("with_parameter"));
        assert!(code.helper_traits.contains("with_status"));
        assert!(code.helper_traits.contains("LambdaResult"));
    }

    #[test]
    fn test_generate_helper_traits_macros() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        // Check macros
        assert!(code.helper_traits.contains("macro_rules! lambda_error"));
        assert!(code.helper_traits.contains("macro_rules! require_param"));
    }

    #[test]
    fn test_generate_retry_logic_content() {
        let handler = LambdaErrorHandler::new();
        let retry = handler.generate_retry_logic();

        // Check retry config
        assert!(retry.contains("RetryConfig"));
        assert!(retry.contains("max_attempts"));
        assert!(retry.contains("base_delay_ms"));
        assert!(retry.contains("max_delay_ms"));
        assert!(retry.contains("backoff_multiplier"));

        // Check retry function
        assert!(retry.contains("retry_with_backoff"));
        assert!(retry.contains("tokio::time::sleep"));
    }

    // === Original tests ===

    #[test]
    fn test_error_enum_generation() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        assert!(code.error_enum.contains("enum LambdaError"));
        assert!(code.error_enum.contains("MissingParameter"));
        assert!(code.error_enum.contains("status_code"));
    }

    #[test]
    fn test_conversion_functions() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        assert!(code
            .conversion_functions
            .contains("impl From<serde_json::Error>"));
        assert!(code
            .conversion_functions
            .contains("extract_key_error_parameter"));
    }

    #[test]
    fn test_helper_traits() {
        let handler = LambdaErrorHandler::new();
        let code = handler.generate_error_handling_code().unwrap();

        assert!(code.helper_traits.contains("trait LambdaErrorExt"));
        assert!(code.helper_traits.contains("with_context"));
        assert!(code.helper_traits.contains("LambdaResult"));
    }

    #[test]
    fn test_handler_wrapper_generation() {
        let handler = LambdaErrorHandler::new().with_strategy(ErrorHandlingStrategy::ReturnError);
        let wrapper = handler.generate_handler_wrapper("my_handler");

        assert!(wrapper.contains("my_handler_with_error_handling"));
        assert!(wrapper.contains("match my_handler(event).await"));
    }

    #[test]
    fn test_retry_logic_generation() {
        let handler = LambdaErrorHandler::new();
        let retry_code = handler.generate_retry_logic();

        assert!(retry_code.contains("struct RetryConfig"));
        assert!(retry_code.contains("retry_with_backoff"));
        assert!(retry_code.contains("tokio::time::sleep"));
    }

    #[test]
    fn test_custom_error_mapping() {
        let mut handler = LambdaErrorHandler::new();

        let pattern = PythonErrorPattern {
            error_type: "CustomError".to_string(),
            message_pattern: Some("custom pattern".to_string()),
            context: Some(ErrorContext::Handler),
        };

        let mapping = LambdaErrorMapping {
            rust_error_type: "LambdaError::Custom".to_string(),
            status_code: Some(422),
            error_message_template: "Custom error: {message}".to_string(),
            include_stack_trace: true,
            retry_strategy: RetryStrategy::Custom("custom_retry".to_string()),
        };

        handler.add_error_mapping(pattern.clone(), mapping);

        let retrieved = handler.get_error_mapping(&pattern).unwrap();
        assert_eq!(retrieved.status_code, Some(422));
    }

    #[test]
    fn test_error_strategies() {
        let strategies = vec![
            ErrorHandlingStrategy::ReturnError,
            ErrorHandlingStrategy::LogAndContinue,
            ErrorHandlingStrategy::Panic,
            ErrorHandlingStrategy::CustomHandler("custom".to_string()),
        ];

        for strategy in strategies {
            let handler = LambdaErrorHandler::new().with_strategy(strategy);
            let wrapper = handler.generate_handler_wrapper("test_handler");
            assert!(wrapper.contains("test_handler_with_error_handling"));
        }
    }

    #[test]
    fn test_lambda_error_methods() {
        let err = LambdaError::MissingParameter {
            parameter: "test_param".to_string(),
        };

        assert_eq!(err.status_code(), 400);
        assert!(!err.should_retry());

        let timeout_err = LambdaError::Timeout {
            operation: "test_op".to_string(),
            duration_ms: 5000,
        };

        assert_eq!(timeout_err.status_code(), 504);
        assert!(timeout_err.should_retry());
    }
}
