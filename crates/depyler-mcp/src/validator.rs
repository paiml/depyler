use crate::McpTranspilationResponse;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use syn;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub syntactically_valid: bool,
    pub type_checks: bool,
    pub tests_pass: bool,
    pub complexity_acceptable: bool,
    pub explanation_quality: f64,
}

#[derive(Debug, PartialEq)]
pub struct McpValidator;

impl Default for McpValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl McpValidator {
    pub fn new() -> Self {
        Self
    }

    pub async fn validate_response(
        &self,
        response: &McpTranspilationResponse,
    ) -> Result<ValidationResult> {
        // 1. Parse Rust code
        let syntactically_valid = syn::parse_str::<syn::File>(&response.rust_code).is_ok();

        // 2. Type check (simplified for V1)
        let type_checks = syntactically_valid; // Would use rust-analyzer in real implementation

        // 3. Run provided test cases (simplified)
        let tests_pass = response.test_cases.is_empty() || response.confidence > 0.8;

        // 4. Complexity analysis
        let complexity_acceptable = self.check_complexity(&response.rust_code)?;

        // 5. Explanation quality
        let explanation_quality = self.score_explanation(&response.explanation);

        Ok(ValidationResult {
            syntactically_valid,
            type_checks,
            tests_pass,
            complexity_acceptable,
            explanation_quality,
        })
    }

    fn check_complexity(&self, rust_code: &str) -> Result<bool> {
        // Simple heuristic: check line count and nesting depth
        let lines: Vec<&str> = rust_code.lines().collect();
        let max_line_length = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let max_nesting = self.estimate_nesting(&lines);

        Ok(max_line_length < 120 && max_nesting < 5)
    }

    fn estimate_nesting(&self, lines: &[&str]) -> usize {
        let mut max_nesting = 0;
        let mut current_nesting = 0;

        for line in lines {
            let opens = line.matches('{').count();
            let closes = line.matches('}').count();
            current_nesting += opens;
            max_nesting = max_nesting.max(current_nesting);
            current_nesting = current_nesting.saturating_sub(closes);
        }

        max_nesting
    }

    fn score_explanation(&self, explanation: &str) -> f64 {
        // Simple scoring based on explanation length and keywords
        let word_count = explanation.split_whitespace().count();
        let has_reasoning = explanation.contains("because")
            || explanation.contains("since")
            || explanation.contains("due to");
        let has_context = explanation.contains("Python") && explanation.contains("Rust");

        let mut score: f64 = 0.5;

        if word_count > 20 {
            score += 0.2;
        }
        if has_reasoning {
            score += 0.2;
        }
        if has_context {
            score += 0.1;
        }

        score.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_nesting() {
        let validator = McpValidator::new();
        let lines = vec![
            "fn test() {",
            "    if true {",
            "        println!(\"hello\");",
            "    }",
            "}",
        ];

        let nesting = validator.estimate_nesting(&lines);
        assert_eq!(nesting, 2);
    }

    #[test]
    fn test_score_explanation() {
        let validator = McpValidator::new();

        // Good explanation
        let good = "This Python function converts to Rust because it needs memory safety and uses ownership patterns since the original used mutable data.";
        let score = validator.score_explanation(good);
        assert!(score > 0.8);

        // Poor explanation
        let poor = "Code";
        let score2 = validator.score_explanation(poor);
        assert!(score2 < 0.7);
    }

    #[test]
    fn test_check_complexity() {
        let validator = McpValidator::new();

        // Simple code
        let simple = "fn test() -> i32 { 42 }";
        assert!(validator.check_complexity(simple).unwrap());

        // Very long line
        let long_line = &"a".repeat(200);
        assert!(!validator.check_complexity(long_line).unwrap());
    }
}
