//! Pure functions for Lambda command operations - EXTREME TDD
//!
//! DEPYLER-COVERAGE-95: Extracted for testability

use std::path::Path;

/// Optimization profile for Lambda functions
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaOptConfig {
    pub lto: bool,
    pub panic_abort: bool,
    pub codegen_units: u32,
    pub opt_level: String,
    pub strip: bool,
    pub mimalloc: bool,
}

impl Default for LambdaOptConfig {
    fn default() -> Self {
        Self {
            lto: false,
            panic_abort: false,
            codegen_units: 16,
            opt_level: "2".to_string(),
            strip: false,
            mimalloc: false,
        }
    }
}

impl LambdaOptConfig {
    /// Create an aggressive optimization config
    pub fn aggressive() -> Self {
        Self {
            lto: true,
            panic_abort: true,
            codegen_units: 1,
            opt_level: "z".to_string(),
            strip: true,
            mimalloc: true,
        }
    }

    /// Create a balanced optimization config
    pub fn balanced() -> Self {
        Self {
            lto: true,
            panic_abort: false,
            codegen_units: 4,
            opt_level: "2".to_string(),
            strip: true,
            mimalloc: false,
        }
    }

    /// Create a debug-friendly config
    pub fn debug() -> Self {
        Self {
            lto: false,
            panic_abort: false,
            codegen_units: 16,
            opt_level: "0".to_string(),
            strip: false,
            mimalloc: false,
        }
    }

    /// Get estimated binary size category
    pub fn estimated_size_category(&self) -> &'static str {
        if self.lto && self.strip && self.opt_level == "z" {
            "minimal (<1MB)"
        } else if self.lto && self.strip {
            "small (1-5MB)"
        } else if self.lto || self.strip {
            "medium (5-15MB)"
        } else {
            "large (>15MB)"
        }
    }
}

/// Lambda event type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCategory {
    HttpApi,
    Storage,
    Messaging,
    Database,
    Scheduled,
    Custom,
}

/// Categorize an event type string
pub fn categorize_event_type(event_type: &str) -> EventCategory {
    let event_lower = event_type.to_lowercase();
    if event_lower.contains("api") || event_lower.contains("http") {
        EventCategory::HttpApi
    } else if event_lower.contains("s3") || event_lower.contains("storage") {
        EventCategory::Storage
    } else if event_lower.contains("sns") || event_lower.contains("sqs") || event_lower.contains("kinesis") {
        EventCategory::Messaging
    } else if event_lower.contains("dynamo") || event_lower.contains("database") {
        EventCategory::Database
    } else if event_lower.contains("schedule") || event_lower.contains("cron") || event_lower.contains("eventbridge") {
        EventCategory::Scheduled
    } else {
        EventCategory::Custom
    }
}

/// Extract function name from a file path
pub fn extract_function_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("handler")
        .to_string()
}

/// Generate a valid Rust crate name from a function name
pub fn sanitize_crate_name(name: &str) -> String {
    let mut result = String::new();
    let mut prev_underscore = false;

    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            prev_underscore = false;
        } else if !prev_underscore && !result.is_empty() {
            result.push('_');
            prev_underscore = true;
        }
    }

    // Remove trailing underscore
    while result.ends_with('_') {
        result.pop();
    }

    // Handle empty result first
    if result.is_empty() {
        return "lambda_handler".to_string();
    }

    // Ensure it starts with a letter (not a number)
    if result.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        result = format!("fn_{result}");
    }

    result
}

/// Format memory size in MB
pub fn format_memory_mb(memory: u32) -> String {
    format!("{memory} MB")
}

/// Validate Lambda memory setting (128-10240 MB)
pub fn validate_memory(memory: u32) -> Result<u32, &'static str> {
    if memory < 128 {
        Err("Memory must be at least 128 MB")
    } else if memory > 10240 {
        Err("Memory cannot exceed 10240 MB")
    } else if memory % 64 != 0 {
        // Lambda requires memory in 64MB increments (after 128)
        Err("Memory must be a multiple of 64 MB")
    } else {
        Ok(memory)
    }
}

/// Validate Lambda timeout (1-900 seconds)
pub fn validate_timeout(timeout: u32) -> Result<u32, &'static str> {
    if timeout < 1 {
        Err("Timeout must be at least 1 second")
    } else if timeout > 900 {
        Err("Timeout cannot exceed 900 seconds (15 minutes)")
    } else {
        Ok(timeout)
    }
}

/// Calculate estimated cold start time based on optimization config
pub fn estimate_cold_start_ms(config: &LambdaOptConfig, binary_size_kb: u32) -> u32 {
    let base = 100; // Base cold start time
    let size_factor = binary_size_kb / 100; // ~1ms per 100KB
    let lto_bonus = if config.lto { 0 } else { 50 }; // LTO improves cold start
    let mimalloc_bonus = if config.mimalloc { 0 } else { 20 }; // mimalloc is faster

    base + size_factor + lto_bonus + mimalloc_bonus
}

/// Format duration in human-readable form
pub fn format_duration_human(ms: u32) -> String {
    if ms < 1000 {
        format!("{ms}ms")
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        format!("{:.1}min", ms as f64 / 60000.0)
    }
}

/// Generate deployment region list
pub fn get_recommended_regions() -> Vec<&'static str> {
    vec![
        "us-east-1",      // N. Virginia
        "us-west-2",      // Oregon
        "eu-west-1",      // Ireland
        "ap-northeast-1", // Tokyo
        "ap-southeast-1", // Singapore
    ]
}

/// Check if a region is valid AWS region
pub fn is_valid_aws_region(region: &str) -> bool {
    let valid_prefixes = ["us-", "eu-", "ap-", "sa-", "ca-", "me-", "af-"];
    valid_prefixes.iter().any(|prefix| region.starts_with(prefix))
        && region.len() >= 9
        && region.len() <= 16
}

/// Generate Lambda ARN from components
pub fn generate_lambda_arn(
    region: &str,
    account_id: &str,
    function_name: &str,
) -> Result<String, &'static str> {
    if !is_valid_aws_region(region) {
        return Err("Invalid AWS region");
    }
    if account_id.len() != 12 || !account_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("Invalid AWS account ID (must be 12 digits)");
    }
    if function_name.is_empty() || function_name.len() > 140 {
        return Err("Invalid function name (1-140 characters)");
    }

    Ok(format!("arn:aws:lambda:{region}:{account_id}:function:{function_name}"))
}

/// Runtime selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LambdaRuntime {
    ProvidedAl2023,
    ProvidedAl2,
    Python312,
    Python311,
}

impl LambdaRuntime {
    /// Get runtime string for AWS
    pub fn as_str(&self) -> &'static str {
        match self {
            LambdaRuntime::ProvidedAl2023 => "provided.al2023",
            LambdaRuntime::ProvidedAl2 => "provided.al2",
            LambdaRuntime::Python312 => "python3.12",
            LambdaRuntime::Python311 => "python3.11",
        }
    }

    /// Is this a native Rust runtime?
    pub fn is_native(&self) -> bool {
        matches!(self, LambdaRuntime::ProvidedAl2023 | LambdaRuntime::ProvidedAl2)
    }

    /// Get recommended runtime for new projects
    pub fn recommended() -> Self {
        LambdaRuntime::ProvidedAl2023
    }
}

/// Architecture selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    Arm64,
}

impl Architecture {
    /// Get architecture string for AWS
    pub fn as_str(&self) -> &'static str {
        match self {
            Architecture::X86_64 => "x86_64",
            Architecture::Arm64 => "arm64",
        }
    }

    /// Get Rust target triple
    pub fn rust_target(&self) -> &'static str {
        match self {
            Architecture::X86_64 => "x86_64-unknown-linux-musl",
            Architecture::Arm64 => "aarch64-unknown-linux-musl",
        }
    }

    /// Get recommended architecture for new projects
    pub fn recommended() -> Self {
        Architecture::Arm64 // Graviton2 is cheaper and often faster
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ==================== LambdaOptConfig tests ====================

    #[test]
    fn test_opt_config_default() {
        let config = LambdaOptConfig::default();
        assert!(!config.lto);
        assert!(!config.strip);
        assert_eq!(config.opt_level, "2");
    }

    #[test]
    fn test_opt_config_aggressive() {
        let config = LambdaOptConfig::aggressive();
        assert!(config.lto);
        assert!(config.panic_abort);
        assert!(config.strip);
        assert!(config.mimalloc);
        assert_eq!(config.codegen_units, 1);
        assert_eq!(config.opt_level, "z");
    }

    #[test]
    fn test_opt_config_balanced() {
        let config = LambdaOptConfig::balanced();
        assert!(config.lto);
        assert!(!config.panic_abort);
        assert!(config.strip);
        assert!(!config.mimalloc);
        assert_eq!(config.codegen_units, 4);
    }

    #[test]
    fn test_opt_config_debug() {
        let config = LambdaOptConfig::debug();
        assert!(!config.lto);
        assert!(!config.strip);
        assert_eq!(config.opt_level, "0");
        assert_eq!(config.codegen_units, 16);
    }

    #[test]
    fn test_estimated_size_minimal() {
        let config = LambdaOptConfig::aggressive();
        assert_eq!(config.estimated_size_category(), "minimal (<1MB)");
    }

    #[test]
    fn test_estimated_size_small() {
        let mut config = LambdaOptConfig::aggressive();
        config.opt_level = "2".to_string();
        assert_eq!(config.estimated_size_category(), "small (1-5MB)");
    }

    #[test]
    fn test_estimated_size_medium() {
        let config = LambdaOptConfig::balanced();
        assert_eq!(config.estimated_size_category(), "small (1-5MB)");
    }

    #[test]
    fn test_estimated_size_large() {
        let config = LambdaOptConfig::debug();
        assert_eq!(config.estimated_size_category(), "large (>15MB)");
    }

    // ==================== Event categorization tests ====================

    #[test]
    fn test_categorize_http_api() {
        assert_eq!(categorize_event_type("ApiGatewayV2Http"), EventCategory::HttpApi);
        assert_eq!(categorize_event_type("http_request"), EventCategory::HttpApi);
    }

    #[test]
    fn test_categorize_storage() {
        assert_eq!(categorize_event_type("S3Event"), EventCategory::Storage);
        assert_eq!(categorize_event_type("storage_trigger"), EventCategory::Storage);
    }

    #[test]
    fn test_categorize_messaging() {
        assert_eq!(categorize_event_type("SnsEvent"), EventCategory::Messaging);
        assert_eq!(categorize_event_type("SqsEvent"), EventCategory::Messaging);
        assert_eq!(categorize_event_type("kinesis_stream"), EventCategory::Messaging);
    }

    #[test]
    fn test_categorize_database() {
        assert_eq!(categorize_event_type("DynamodbEvent"), EventCategory::Database);
        assert_eq!(categorize_event_type("database_change"), EventCategory::Database);
    }

    #[test]
    fn test_categorize_scheduled() {
        assert_eq!(categorize_event_type("EventBridge"), EventCategory::Scheduled);
        assert_eq!(categorize_event_type("schedule_event"), EventCategory::Scheduled);
        assert_eq!(categorize_event_type("cron_job"), EventCategory::Scheduled);
    }

    #[test]
    fn test_categorize_custom() {
        assert_eq!(categorize_event_type("MyCustomEvent"), EventCategory::Custom);
        assert_eq!(categorize_event_type("unknown"), EventCategory::Custom);
    }

    // ==================== Function name tests ====================

    #[test]
    fn test_extract_function_name() {
        assert_eq!(extract_function_name(Path::new("/tmp/handler.py")), "handler");
        assert_eq!(extract_function_name(Path::new("my_lambda.py")), "my_lambda");
    }

    #[test]
    fn test_extract_function_name_no_extension() {
        assert_eq!(extract_function_name(Path::new("/tmp/handler")), "handler");
    }

    #[test]
    fn test_extract_function_name_fallback() {
        assert_eq!(extract_function_name(Path::new("")), "handler");
    }

    // ==================== Crate name sanitization tests ====================

    #[test]
    fn test_sanitize_crate_name_simple() {
        assert_eq!(sanitize_crate_name("my_handler"), "my_handler");
    }

    #[test]
    fn test_sanitize_crate_name_uppercase() {
        assert_eq!(sanitize_crate_name("MyHandler"), "myhandler");
    }

    #[test]
    fn test_sanitize_crate_name_special_chars() {
        assert_eq!(sanitize_crate_name("my-handler.py"), "my_handler_py");
    }

    #[test]
    fn test_sanitize_crate_name_starts_with_number() {
        assert_eq!(sanitize_crate_name("123handler"), "fn_123handler");
    }

    #[test]
    fn test_sanitize_crate_name_empty() {
        assert_eq!(sanitize_crate_name(""), "lambda_handler");
    }

    #[test]
    fn test_sanitize_crate_name_all_special() {
        assert_eq!(sanitize_crate_name("---"), "lambda_handler");
    }

    #[test]
    fn test_sanitize_crate_name_trailing_underscore() {
        assert_eq!(sanitize_crate_name("handler_"), "handler");
    }

    // ==================== Memory validation tests ====================

    #[test]
    fn test_validate_memory_valid() {
        assert_eq!(validate_memory(128), Ok(128));
        assert_eq!(validate_memory(512), Ok(512));
        assert_eq!(validate_memory(1024), Ok(1024));
        assert_eq!(validate_memory(10240), Ok(10240));
    }

    #[test]
    fn test_validate_memory_too_low() {
        assert!(validate_memory(64).is_err());
        assert!(validate_memory(0).is_err());
    }

    #[test]
    fn test_validate_memory_too_high() {
        assert!(validate_memory(10304).is_err());
        assert!(validate_memory(20000).is_err());
    }

    #[test]
    fn test_validate_memory_not_multiple() {
        assert!(validate_memory(150).is_err()); // Not multiple of 64
        assert!(validate_memory(300).is_err());
    }

    // ==================== Timeout validation tests ====================

    #[test]
    fn test_validate_timeout_valid() {
        assert_eq!(validate_timeout(1), Ok(1));
        assert_eq!(validate_timeout(30), Ok(30));
        assert_eq!(validate_timeout(900), Ok(900));
    }

    #[test]
    fn test_validate_timeout_too_low() {
        assert!(validate_timeout(0).is_err());
    }

    #[test]
    fn test_validate_timeout_too_high() {
        assert!(validate_timeout(901).is_err());
        assert!(validate_timeout(3600).is_err());
    }

    // ==================== Cold start estimation tests ====================

    #[test]
    fn test_estimate_cold_start_optimized() {
        let config = LambdaOptConfig::aggressive();
        let estimate = estimate_cold_start_ms(&config, 500); // 500KB
        assert!(estimate < 200); // Should be fast
    }

    #[test]
    fn test_estimate_cold_start_unoptimized() {
        let config = LambdaOptConfig::debug();
        let estimate = estimate_cold_start_ms(&config, 5000); // 5MB
        assert!(estimate > 150); // Should be slower
    }

    // ==================== Duration formatting tests ====================

    #[test]
    fn test_format_duration_ms() {
        assert_eq!(format_duration_human(50), "50ms");
        assert_eq!(format_duration_human(999), "999ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration_human(1000), "1.0s");
        assert_eq!(format_duration_human(1500), "1.5s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration_human(60000), "1.0min");
        assert_eq!(format_duration_human(90000), "1.5min");
    }

    // ==================== Region tests ====================

    #[test]
    fn test_recommended_regions() {
        let regions = get_recommended_regions();
        assert!(regions.contains(&"us-east-1"));
        assert!(regions.contains(&"eu-west-1"));
    }

    #[test]
    fn test_is_valid_region_valid() {
        assert!(is_valid_aws_region("us-east-1"));
        assert!(is_valid_aws_region("eu-west-1"));
        assert!(is_valid_aws_region("ap-southeast-1"));
    }

    #[test]
    fn test_is_valid_region_invalid() {
        assert!(!is_valid_aws_region("invalid"));
        assert!(!is_valid_aws_region("xx-east-1"));
        assert!(!is_valid_aws_region("us"));
    }

    // ==================== ARN generation tests ====================

    #[test]
    fn test_generate_arn_valid() {
        let arn = generate_lambda_arn("us-east-1", "123456789012", "my-function");
        assert!(arn.is_ok());
        assert_eq!(arn.unwrap(), "arn:aws:lambda:us-east-1:123456789012:function:my-function");
    }

    #[test]
    fn test_generate_arn_invalid_region() {
        let arn = generate_lambda_arn("invalid", "123456789012", "my-function");
        assert!(arn.is_err());
    }

    #[test]
    fn test_generate_arn_invalid_account() {
        let arn = generate_lambda_arn("us-east-1", "123", "my-function");
        assert!(arn.is_err());
    }

    #[test]
    fn test_generate_arn_invalid_name() {
        let arn = generate_lambda_arn("us-east-1", "123456789012", "");
        assert!(arn.is_err());
    }

    // ==================== Runtime tests ====================

    #[test]
    fn test_runtime_as_str() {
        assert_eq!(LambdaRuntime::ProvidedAl2023.as_str(), "provided.al2023");
        assert_eq!(LambdaRuntime::ProvidedAl2.as_str(), "provided.al2");
        assert_eq!(LambdaRuntime::Python312.as_str(), "python3.12");
    }

    #[test]
    fn test_runtime_is_native() {
        assert!(LambdaRuntime::ProvidedAl2023.is_native());
        assert!(LambdaRuntime::ProvidedAl2.is_native());
        assert!(!LambdaRuntime::Python312.is_native());
    }

    #[test]
    fn test_runtime_recommended() {
        assert_eq!(LambdaRuntime::recommended(), LambdaRuntime::ProvidedAl2023);
    }

    // ==================== Architecture tests ====================

    #[test]
    fn test_arch_as_str() {
        assert_eq!(Architecture::X86_64.as_str(), "x86_64");
        assert_eq!(Architecture::Arm64.as_str(), "arm64");
    }

    #[test]
    fn test_arch_rust_target() {
        assert_eq!(Architecture::X86_64.rust_target(), "x86_64-unknown-linux-musl");
        assert_eq!(Architecture::Arm64.rust_target(), "aarch64-unknown-linux-musl");
    }

    #[test]
    fn test_arch_recommended() {
        assert_eq!(Architecture::recommended(), Architecture::Arm64);
    }

    // ==================== Memory formatting tests ====================

    #[test]
    fn test_format_memory() {
        assert_eq!(format_memory_mb(128), "128 MB");
        assert_eq!(format_memory_mb(1024), "1024 MB");
    }

    // ==================== Edge case tests ====================

    #[test]
    fn test_extract_name_from_path_buf() {
        let path = PathBuf::from("/home/user/projects/my_handler.py");
        assert_eq!(extract_function_name(&path), "my_handler");
    }

    #[test]
    fn test_sanitize_unicode() {
        // Non-ASCII chars are treated as separators (like special chars)
        assert_eq!(sanitize_crate_name("héllo_wörld"), "h_llo_w_rld");
    }

    #[test]
    fn test_opt_config_clone() {
        let config = LambdaOptConfig::aggressive();
        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    #[test]
    fn test_event_category_equality() {
        assert_eq!(EventCategory::HttpApi, EventCategory::HttpApi);
        assert_ne!(EventCategory::HttpApi, EventCategory::Storage);
    }

    #[test]
    fn test_categorize_case_insensitive() {
        assert_eq!(categorize_event_type("API"), EventCategory::HttpApi);
        assert_eq!(categorize_event_type("api"), EventCategory::HttpApi);
        assert_eq!(categorize_event_type("Api"), EventCategory::HttpApi);
    }
}
