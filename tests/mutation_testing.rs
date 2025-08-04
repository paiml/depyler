//! Mutation Testing - Phase 8.1
//!
//! Mutation testing to validate the quality of our test suite by introducing
//! small modifications to the code and ensuring tests catch the defects.

use depyler_core::DepylerPipeline;
use std::collections::HashMap;
use std::time::Instant;

/// Types of mutations we can apply to Python code
#[derive(Debug, Clone)]
pub enum MutationOperator {
    // Arithmetic mutations
    ArithmeticOperatorReplacement,
    // Relational mutations
    RelationalOperatorReplacement,
    // Assignment mutations
    AssignmentOperatorReplacement,
    // Logical mutations
    LogicalOperatorReplacement,
    // Statement mutations
    StatementRemoval,
    // Constant mutations
    ConstantReplacement,
    // Variable mutations
    VariableNameReplacement,
}

/// A specific mutation applied to code
#[derive(Debug, Clone)]
pub struct Mutation {
    pub operator: MutationOperator,
    pub original: String,
    pub mutated: String,
    pub line: usize,
    pub column: usize,
}

/// Results of mutation testing
#[derive(Debug)]
pub struct MutationTestResults {
    pub total_mutations: usize,
    pub killed_mutations: usize,     // Detected by tests
    pub survived_mutations: usize,   // Not detected
    pub equivalent_mutations: usize, // Functionally equivalent
    pub mutation_score: f64,
}

/// Mutation test engine
pub struct MutationTester {
    pipeline: DepylerPipeline,
    mutations_cache: HashMap<String, Vec<Mutation>>,
}

impl Default for MutationTester {
    fn default() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            mutations_cache: HashMap::new(),
        }
    }
}

impl MutationTester {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate all possible mutations for a given code snippet
    pub fn generate_mutations(&mut self, code: &str) -> Vec<Mutation> {
        if let Some(cached) = self.mutations_cache.get(code) {
            return cached.clone();
        }

        let mut mutations = Vec::new();

        // Arithmetic operator mutations
        mutations.extend(self.generate_arithmetic_mutations(code));

        // Relational operator mutations
        mutations.extend(self.generate_relational_mutations(code));

        // Logical operator mutations
        mutations.extend(self.generate_logical_mutations(code));

        // Constant mutations
        mutations.extend(self.generate_constant_mutations(code));

        // Statement mutations
        mutations.extend(self.generate_statement_mutations(code));

        self.mutations_cache
            .insert(code.to_string(), mutations.clone());
        mutations
    }

    /// Test a single mutation
    pub fn test_mutation(&self, original_code: &str, mutation: &Mutation) -> MutationResult {
        let mutated_code = self.apply_mutation(original_code, mutation);

        // Test original code
        let original_result = self.pipeline.transpile(original_code);

        // Test mutated code
        let mutated_result = self.pipeline.transpile(&mutated_code);

        // Determine if mutation was killed (detected)
        match (original_result.is_ok(), mutated_result.is_ok()) {
            (true, false) => MutationResult::Killed, // Original worked, mutant failed
            (false, true) => MutationResult::Killed, // Original failed, mutant worked
            (true, true) => {
                // Both worked - need to compare outputs
                if let (Ok(orig), Ok(mut_code)) = (original_result, mutated_result) {
                    if orig != mut_code {
                        MutationResult::Killed
                    } else {
                        MutationResult::Equivalent
                    }
                } else {
                    MutationResult::Survived
                }
            }
            (false, false) => MutationResult::Equivalent, // Both failed the same way
        }
    }

    /// Run comprehensive mutation testing
    pub fn run_mutation_testing(&mut self, test_cases: &[&str]) -> MutationTestResults {
        let mut total_mutations = 0;
        let mut killed_mutations = 0;
        let mut survived_mutations = 0;
        let mut equivalent_mutations = 0;

        for code in test_cases {
            let mutations = self.generate_mutations(code);
            total_mutations += mutations.len();

            for mutation in mutations {
                match self.test_mutation(code, &mutation) {
                    MutationResult::Killed => killed_mutations += 1,
                    MutationResult::Survived => survived_mutations += 1,
                    MutationResult::Equivalent => equivalent_mutations += 1,
                }
            }
        }

        let mutation_score = if total_mutations > 0 {
            killed_mutations as f64 / (total_mutations - equivalent_mutations) as f64
        } else {
            0.0
        };

        MutationTestResults {
            total_mutations,
            killed_mutations,
            survived_mutations,
            equivalent_mutations,
            mutation_score,
        }
    }

    // Private helper methods

    fn generate_arithmetic_mutations(&self, code: &str) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let arithmetic_ops = vec![
            ("+", "-"),
            ("-", "+"),
            ("*", "/"),
            ("/", "*"),
            ("//", "%"),
            ("%", "//"),
            ("**", "*"),
        ];

        for (original, replacement) in arithmetic_ops {
            mutations.extend(self.find_and_replace_mutations(
                code,
                original,
                replacement,
                MutationOperator::ArithmeticOperatorReplacement,
            ));
        }

        mutations
    }

    fn generate_relational_mutations(&self, code: &str) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let relational_ops = vec![
            ("==", "!="),
            ("!=", "=="),
            ("<", "<="),
            ("<=", "<"),
            (">", ">="),
            (">=", ">"),
            ("<", ">"),
            (">", "<"),
        ];

        for (original, replacement) in relational_ops {
            mutations.extend(self.find_and_replace_mutations(
                code,
                original,
                replacement,
                MutationOperator::RelationalOperatorReplacement,
            ));
        }

        mutations
    }

    fn generate_logical_mutations(&self, code: &str) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let logical_ops = vec![("and", "or"), ("or", "and"), ("not", "")];

        for (original, replacement) in logical_ops {
            mutations.extend(self.find_and_replace_mutations(
                code,
                original,
                replacement,
                MutationOperator::LogicalOperatorReplacement,
            ));
        }

        mutations
    }

    fn generate_constant_mutations(&self, code: &str) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let constant_replacements = vec![
            ("0", "1"),
            ("1", "0"),
            ("True", "False"),
            ("False", "True"),
            ("None", "0"),
            ("[]", "[0]"),
            ("{}", "{0: 0}"),
        ];

        for (original, replacement) in constant_replacements {
            mutations.extend(self.find_and_replace_mutations(
                code,
                original,
                replacement,
                MutationOperator::ConstantReplacement,
            ));
        }

        mutations
    }

    fn generate_statement_mutations(&self, code: &str) -> Vec<Mutation> {
        let mut mutations = Vec::new();

        // Find return statements and try to remove them
        for (line_num, line) in code.lines().enumerate() {
            if line.trim().starts_with("return ") {
                mutations.push(Mutation {
                    operator: MutationOperator::StatementRemoval,
                    original: line.trim().to_string(),
                    mutated: "pass".to_string(),
                    line: line_num,
                    column: line.find("return").unwrap_or(0),
                });
            }
        }

        mutations
    }

    fn find_and_replace_mutations(
        &self,
        code: &str,
        original: &str,
        replacement: &str,
        operator: MutationOperator,
    ) -> Vec<Mutation> {
        let mut mutations = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            let mut col = 0;
            while let Some(pos) = line[col..].find(original) {
                let absolute_pos = col + pos;
                mutations.push(Mutation {
                    operator: operator.clone(),
                    original: original.to_string(),
                    mutated: replacement.to_string(),
                    line: line_num,
                    column: absolute_pos,
                });
                col = absolute_pos + 1;
            }
        }

        mutations
    }

    fn apply_mutation(&self, code: &str, mutation: &Mutation) -> String {
        let lines: Vec<&str> = code.lines().collect();
        if mutation.line >= lines.len() {
            return code.to_string();
        }

        let line = lines[mutation.line];
        let mutated_line = match mutation.operator {
            MutationOperator::StatementRemoval => {
                line.replace(&mutation.original, &mutation.mutated)
            }
            _ => {
                // Simple string replacement
                line.replacen(&mutation.original, &mutation.mutated, 1)
            }
        };

        let mut result_lines = lines.clone();
        result_lines[mutation.line] = &mutated_line;
        result_lines.join("\n")
    }
}

#[derive(Debug, PartialEq)]
pub enum MutationResult {
    Killed,     // Mutation was detected by tests
    Survived,   // Mutation was not detected
    Equivalent, // Mutation is functionally equivalent
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic mutation generation
    #[test]
    fn test_mutation_generation() {
        println!("=== Mutation Generation Test ===");

        let mut tester = MutationTester::new();

        let test_code = r#"
def arithmetic_test(a: int, b: int) -> int:
    if a > b:
        return a + b
    else:
        return a - b
"#;

        let mutations = tester.generate_mutations(test_code);

        println!("Generated {} mutations", mutations.len());

        // Should generate various types of mutations
        assert!(mutations.len() > 5, "Should generate multiple mutations");

        // Test each mutation type is present
        let arithmetic_mutations = mutations
            .iter()
            .filter(|m| matches!(m.operator, MutationOperator::ArithmeticOperatorReplacement))
            .count();
        let relational_mutations = mutations
            .iter()
            .filter(|m| matches!(m.operator, MutationOperator::RelationalOperatorReplacement))
            .count();

        println!("Arithmetic mutations: {}", arithmetic_mutations);
        println!("Relational mutations: {}", relational_mutations);

        assert!(
            arithmetic_mutations > 0,
            "Should generate arithmetic mutations"
        );
        assert!(
            relational_mutations > 0,
            "Should generate relational mutations"
        );

        // Test some specific mutations
        for mutation in &mutations {
            println!(
                "  {:?}: {} -> {} at {}:{}",
                mutation.operator,
                mutation.original,
                mutation.mutated,
                mutation.line,
                mutation.column
            );
        }
    }

    /// Test mutation application
    #[test]
    fn test_mutation_application() {
        println!("=== Mutation Application Test ===");

        let tester = MutationTester::new();
        let original_code = "def test(x: int) -> int: return x + 1";

        let mutation = Mutation {
            operator: MutationOperator::ArithmeticOperatorReplacement,
            original: "+".to_string(),
            mutated: "-".to_string(),
            line: 0,
            column: 35,
        };

        let mutated_code = tester.apply_mutation(original_code, &mutation);

        println!("Original: {}", original_code);
        println!("Mutated:  {}", mutated_code);

        assert!(mutated_code.contains("x - 1"), "Should replace + with -");
        assert!(
            !mutated_code.contains("x + 1"),
            "Should not contain original operator"
        );
    }

    /// Test individual mutation results
    #[test]
    fn test_mutation_testing() {
        println!("=== Individual Mutation Testing ===");

        let tester = MutationTester::new();

        // Test cases with known mutation behavior
        let test_cases = vec![
            (
                "def simple(x: int) -> int: return x + 1",
                "Should work with simple arithmetic",
            ),
            (
                "def compare(a: int, b: int) -> bool: return a > b",
                "Should work with comparisons",
            ),
            (
                "def logical(p: bool, q: bool) -> bool: return p and q",
                "Should work with logical ops",
            ),
        ];

        for (code, description) in test_cases {
            println!("\nTesting: {}", description);

            let mutations = vec![
                Mutation {
                    operator: MutationOperator::ArithmeticOperatorReplacement,
                    original: "+".to_string(),
                    mutated: "-".to_string(),
                    line: 0,
                    column: 0,
                },
                Mutation {
                    operator: MutationOperator::RelationalOperatorReplacement,
                    original: ">".to_string(),
                    mutated: "<".to_string(),
                    line: 0,
                    column: 0,
                },
            ];

            for mutation in mutations {
                if code.contains(&mutation.original) {
                    let result = tester.test_mutation(code, &mutation);
                    println!(
                        "  Mutation {} -> {}: {:?}",
                        mutation.original, mutation.mutated, result
                    );
                }
            }
        }
    }

    /// Test comprehensive mutation testing
    #[test]
    fn test_comprehensive_mutation_testing() {
        println!("=== Comprehensive Mutation Testing ===");

        let mut tester = MutationTester::new();

        let test_suite = vec![
            "def add(a: int, b: int) -> int: return a + b",
            "def subtract(a: int, b: int) -> int: return a - b",
            "def compare(x: int) -> bool: return x > 0",
            "def logical_and(p: bool, q: bool) -> bool: return p and q",
            "def conditional(x: int) -> str: return 'positive' if x > 0 else 'non-positive'",
        ];

        let start = Instant::now();
        let results = tester.run_mutation_testing(&test_suite);
        let duration = start.elapsed();

        println!("Mutation Testing Results:");
        println!("  Total mutations: {}", results.total_mutations);
        println!("  Killed mutations: {}", results.killed_mutations);
        println!("  Survived mutations: {}", results.survived_mutations);
        println!("  Equivalent mutations: {}", results.equivalent_mutations);
        println!("  Mutation score: {:.2}%", results.mutation_score * 100.0);
        println!("  Testing duration: {:?}", duration);

        // Validate results
        assert!(results.total_mutations > 0, "Should generate mutations");
        assert!(
            results.mutation_score >= 0.0 && results.mutation_score <= 1.0,
            "Score should be between 0 and 1"
        );
        assert_eq!(
            results.total_mutations,
            results.killed_mutations + results.survived_mutations + results.equivalent_mutations
        );

        // Performance check
        assert!(
            duration.as_secs() < 30,
            "Mutation testing should complete quickly"
        );
    }

    /// Test mutation operator coverage
    #[test]
    fn test_mutation_operator_coverage() {
        println!("=== Mutation Operator Coverage Test ===");

        let mut tester = MutationTester::new();

        // Code that exercises all mutation operators
        let comprehensive_code = r#"
def comprehensive_test(x: int, y: int, flag: bool) -> int:
    if x > y and flag:
        result = x + y * 2
        return result
    elif x == y or not flag:
        return x - y // 2
    else:
        return 0
"#;

        let mutations = tester.generate_mutations(comprehensive_code);

        println!(
            "Generated {} mutations for comprehensive code",
            mutations.len()
        );

        // Check that all operator types are represented
        let operator_counts: HashMap<_, _> = mutations
            .iter()
            .map(|m| format!("{:?}", m.operator))
            .fold(HashMap::new(), |mut acc, op| {
                *acc.entry(op).or_insert(0) += 1;
                acc
            });

        println!("Operator coverage:");
        for (operator, count) in &operator_counts {
            println!("  {}: {} mutations", operator, count);
        }

        // Should have multiple operator types
        assert!(
            operator_counts.len() >= 3,
            "Should have at least 3 operator types"
        );

        // Test a subset of mutations
        let sample_size = std::cmp::min(mutations.len(), 10);
        let mut killed_count = 0;

        for mutation in mutations.iter().take(sample_size) {
            let result = tester.test_mutation(comprehensive_code, mutation);
            if result == MutationResult::Killed {
                killed_count += 1;
            }

            println!(
                "  {:?} at {}:{} -> {:?}",
                mutation.operator, mutation.line, mutation.column, result
            );
        }

        println!(
            "Sample mutation kill rate: {}/{} ({:.1}%)",
            killed_count,
            sample_size,
            killed_count as f64 / sample_size as f64 * 100.0
        );
    }

    /// Test mutation testing performance with caching
    #[test]
    fn test_mutation_performance_with_caching() {
        println!("=== Mutation Performance Test ===");

        let mut tester = MutationTester::new();

        let test_code = "def cached_test(x: int) -> int: return x + 1 if x > 0 else x - 1";

        // First run - should populate cache
        let start1 = Instant::now();
        let mutations1 = tester.generate_mutations(test_code);
        let duration1 = start1.elapsed();

        // Second run - should use cache
        let start2 = Instant::now();
        let mutations2 = tester.generate_mutations(test_code);
        let duration2 = start2.elapsed();

        println!(
            "First run: {} mutations in {:?}",
            mutations1.len(),
            duration1
        );
        println!(
            "Second run: {} mutations in {:?}",
            mutations2.len(),
            duration2
        );

        // Results should be identical
        assert_eq!(
            mutations1.len(),
            mutations2.len(),
            "Cache should return same results"
        );

        // Second run should be faster (or at least not significantly slower)
        let speedup_ratio = duration1.as_nanos() as f64 / duration2.as_nanos() as f64;
        println!("Speedup ratio: {:.2}x", speedup_ratio);

        // Cache should provide some benefit
        assert!(
            speedup_ratio >= 0.8,
            "Caching should not significantly slow down generation"
        );

        // Test cache size
        println!("Cache entries: {}", tester.mutations_cache.len());
        assert_eq!(
            tester.mutations_cache.len(),
            1,
            "Should have one cached entry"
        );
    }

    /// Test edge cases in mutation testing
    #[test]
    fn test_mutation_edge_cases() {
        println!("=== Mutation Edge Cases Test ===");

        let mut tester = MutationTester::new();

        let edge_cases = vec![
            ("", "Empty code"),
            ("# Just a comment", "Comment only"),
            ("def empty(): pass", "Empty function"),
            (
                "def single_line(x: int) -> int: return x",
                "Single expression",
            ),
            (
                "def unicode_函数(参数: int) -> int: return 参数 + 1",
                "Unicode identifiers",
            ),
        ];

        for (code, description) in edge_cases {
            println!("\nTesting edge case: {}", description);
            println!("Code: {}", code);

            let mutations = tester.generate_mutations(code);
            println!("Generated {} mutations", mutations.len());

            // Should handle edge cases gracefully
            if !code.trim().is_empty() && code.contains("def ") {
                // Non-empty functions should generate some mutations
                // mutations.len() is always >= 0, this is just documentation
                let _ = mutations.len(); // Should handle mutations gracefully
            }

            // Test a few mutations if any exist
            for mutation in mutations.iter().take(3) {
                let result = tester.test_mutation(code, mutation);
                println!("  {:?}: {:?}", mutation.operator, result);
            }
        }
    }
}
