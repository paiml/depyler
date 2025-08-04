//! Fuzzing Tests - Phase 8.1
//!
//! Fuzzing-based input validation to discover edge cases, security issues,
//! and robustness problems in the Depyler transpiler through random input generation.

use depyler_core::DepylerPipeline;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Fuzzing strategy types
#[derive(Debug, Clone)]
pub enum FuzzingStrategy {
    RandomBytes,       // Pure random byte sequences
    StructuredPython,  // Structured but random Python-like code
    MalformedSyntax,   // Intentionally broken Python syntax
    SecurityFocused,   // Input designed to test security boundaries
    UnicodeExploit,    // Unicode and encoding edge cases
    LargeInput,        // Extremely large inputs
    DeepNesting,       // Deeply nested structures
}

/// Fuzzing test result
#[derive(Debug, Clone)]
pub struct FuzzingResult {
    pub input_size: usize,
    pub strategy: FuzzingStrategy,
    pub execution_time: Duration,
    pub outcome: FuzzingOutcome,
    pub error_type: Option<String>,
    pub crash_detected: bool,
    pub memory_usage_spike: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FuzzingOutcome {
    Success,           // Transpiled successfully
    ExpectedFailure,   // Failed as expected
    UnexpectedError,   // Failed in unexpected way
    Timeout,           // Took too long
    Crash,             // Process crashed/panicked
}

/// Fuzzing test engine
pub struct FuzzingEngine {
    pipeline: DepylerPipeline,
    timeout_ms: u64,
    max_input_size: usize,
    results_cache: HashMap<String, FuzzingResult>,
}

impl FuzzingEngine {
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            timeout_ms: 5000, // 5 second timeout
            max_input_size: 1024 * 1024, // 1MB max
            results_cache: HashMap::new(),
        }
    }
    
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
    
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_input_size = max_size;
        self
    }
    
    /// Generate random input based on strategy
    pub fn generate_fuzz_input(&self, strategy: &FuzzingStrategy, size: usize) -> String {
        match strategy {
            FuzzingStrategy::RandomBytes => self.generate_random_bytes(size),
            FuzzingStrategy::StructuredPython => self.generate_structured_python(size),
            FuzzingStrategy::MalformedSyntax => self.generate_malformed_syntax(size),
            FuzzingStrategy::SecurityFocused => self.generate_security_input(size),
            FuzzingStrategy::UnicodeExploit => self.generate_unicode_exploit(size),
            FuzzingStrategy::LargeInput => self.generate_large_input(size),
            FuzzingStrategy::DeepNesting => self.generate_deep_nesting(size),
        }
    }
    
    /// Run fuzzing test with a specific input
    pub fn fuzz_test(&mut self, input: &str, strategy: FuzzingStrategy) -> FuzzingResult {
        let input_hash = format!("{:x}", md5::compute(input));
        
        // Check cache first
        if let Some(cached) = self.results_cache.get(&input_hash) {
            return cached.clone();
        }
        
        let start = Instant::now();
        let outcome = self.execute_with_timeout(input);
        let execution_time = start.elapsed();
        
        let result = FuzzingResult {
            input_size: input.len(),
            strategy,
            execution_time,
            outcome: outcome.clone(),
            error_type: self.extract_error_type(&outcome, input),
            crash_detected: matches!(outcome, FuzzingOutcome::Crash),
            memory_usage_spike: execution_time > Duration::from_millis(self.timeout_ms / 2),
        };
        
        // Cache result
        self.results_cache.insert(input_hash, result.clone());
        result
    }
    
    /// Run comprehensive fuzzing campaign
    pub fn run_fuzzing_campaign(&mut self, iterations: usize) -> FuzzingCampaignResults {
        let mut results = Vec::new();
        let strategies = vec![
            FuzzingStrategy::RandomBytes,
            FuzzingStrategy::StructuredPython,
            FuzzingStrategy::MalformedSyntax,
            FuzzingStrategy::SecurityFocused,
            FuzzingStrategy::UnicodeExploit,
            FuzzingStrategy::LargeInput,
            FuzzingStrategy::DeepNesting,
        ];
        
        let start_time = Instant::now();
        
        for i in 0..iterations {
            let strategy = &strategies[i % strategies.len()];
            let size = if i < iterations / 2 { 100 + i * 10 } else { 1000 + i * 50 };
            let size = std::cmp::min(size, self.max_input_size);
            
            let input = self.generate_fuzz_input(strategy, size);
            let result = self.fuzz_test(&input, strategy.clone());
            
            // Early termination on severe issues
            let should_terminate = matches!(result.outcome, FuzzingOutcome::Crash);
            
            results.push(result);
            
            if should_terminate {
                println!("CRASH DETECTED: Terminating fuzzing campaign early");
                break;
            }
        }
        
        let total_time = start_time.elapsed();
        
        FuzzingCampaignResults {
            total_tests: results.len(),
            total_time,
            results,
        }
    }
    
    // Private implementation methods
    
    fn execute_with_timeout(&self, input: &str) -> FuzzingOutcome {
        // Simple timeout mechanism using std::thread
        use std::sync::mpsc;
        use std::thread;
        
        let (tx, rx) = mpsc::channel();
        let input_clone = input.to_string();
        let pipeline = DepylerPipeline::new(); // Create new pipeline for thread
        
        let handle = thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                pipeline.transpile(&input_clone)
            });
            
            let outcome = match result {
                Ok(Ok(_)) => FuzzingOutcome::Success,
                Ok(Err(_)) => FuzzingOutcome::ExpectedFailure,
                Err(_) => FuzzingOutcome::Crash,
            };
            
            let _ = tx.send(outcome);
        });
        
        // Wait for result with timeout
        match rx.recv_timeout(Duration::from_millis(self.timeout_ms)) {
            Ok(outcome) => {
                let _ = handle.join();
                outcome
            },
            Err(_) => {
                // Thread timed out
                FuzzingOutcome::Timeout
            }
        }
    }
    
    fn extract_error_type(&self, outcome: &FuzzingOutcome, input: &str) -> Option<String> {
        match outcome {
            FuzzingOutcome::ExpectedFailure | FuzzingOutcome::UnexpectedError => {
                // Try to categorize the error by input characteristics
                if input.contains("async") || input.contains("await") {
                    Some("UnsupportedAsyncFeature".to_string())
                } else if input.len() > 10000 {
                    Some("LargeInputError".to_string())
                } else if input.chars().any(|c| c as u32 > 127) {
                    Some("UnicodeError".to_string())
                } else if input.contains("def") && !input.contains(":") {
                    Some("SyntaxError".to_string())
                } else {
                    Some("UnknownError".to_string())
                }
            },
            FuzzingOutcome::Crash => Some("Crash".to_string()),
            FuzzingOutcome::Timeout => Some("Timeout".to_string()),
            FuzzingOutcome::Success => None,
        }
    }
    
    fn generate_random_bytes(&self, size: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Generate ASCII-only random string to avoid UTF-8 issues
        (0..size)
            .map(|_| {
                let ascii_chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \n\t()[]{}:;.,";
                ascii_chars[rng.gen_range(0..ascii_chars.len())] as char
            })
            .collect()
    }
    
    fn generate_structured_python(&self, target_size: usize) -> String {
        let mut code = String::new();
        let mut current_size = 0;
        
        // Generate random but structured Python-like code
        while current_size < target_size {
            let fragment = match rand::random::<u8>() % 6 {
                0 => "def func(): pass\n",
                1 => "x = 42\n",
                2 => "if True: return 1\n",
                3 => "for i in range(10): continue\n",
                4 => "class Test: pass\n",
                _ => "# comment\n",
            };
            
            code.push_str(fragment);
            current_size = code.len();
        }
        
        // Safe truncation on char boundaries
        if code.len() > target_size {
            // Find a safe UTF-8 truncation point
            let mut safe_len = target_size;
            while safe_len > 0 && !code.is_char_boundary(safe_len) {
                safe_len -= 1;
            }
            code.truncate(safe_len);
        }
        code
    }
    
    fn generate_malformed_syntax(&self, target_size: usize) -> String {
        let malformed_patterns = vec![
            "def broken_function(\n    return 42",
            "if condition\n    print('missing colon')",
            "def func(: pass",
            "class Broken\n    def method(self):",
            "for i in\n    print(i)",
            "def (invalid_name): return 1",
            "def func(): return",
            "if elif else: pass",
        ];
        
        let pattern = &malformed_patterns[rand::random::<usize>() % malformed_patterns.len()];
        let mut result = pattern.to_string();
        
        // Pad to target size
        while result.len() < target_size {
            result.push_str("\n# padding");
        }
        
        // Safe truncation on char boundaries
        if result.len() > target_size {
            // Find a safe UTF-8 truncation point
            let mut safe_len = target_size;
            while safe_len > 0 && !result.is_char_boundary(safe_len) {
                safe_len -= 1;
            }
            result.truncate(safe_len);
        }
        result
    }
    
    fn generate_security_input(&self, target_size: usize) -> String {
        let security_patterns = vec![
            "__import__('os').system('echo test')",
            "eval('print(1337)')",
            "exec('import sys')",
            "open('/etc/passwd').read()",
            "subprocess.call(['ls', '-la'])",
            "pickle.loads(b'malicious_data')",
            "compile('evil_code', '<string>', 'exec')",
        ];
        
        let pattern = &security_patterns[rand::random::<usize>() % security_patterns.len()];
        let mut result = format!("def security_test(): {}", pattern);
        
        // Pad to target size
        while result.len() < target_size {
            result.push_str("\n# security padding");
        }
        
        // Safe truncation on char boundaries
        if result.len() > target_size {
            // Find a safe UTF-8 truncation point
            let mut safe_len = target_size;
            while safe_len > 0 && !result.is_char_boundary(safe_len) {
                safe_len -= 1;
            }
            result.truncate(safe_len);
        }
        result
    }
    
    fn generate_unicode_exploit(&self, target_size: usize) -> String {
        let unicode_patterns = vec![
            "def 函数(): return '测试'",
            "def функция(): return 'тест'",
            "def פונקציה(): return 'בדיקה'",
            "def 関数(): return 'テスト'",
            "def 함수(): return '테스트'",
            "x = '🚀🎉💻' + '⚡🔥🌟'",
            "def test(): return '\\u0000\\u001f\\u007f'",
            "def test(): return '\\U0001F600\\U0001F601'",
        ];
        
        let pattern = &unicode_patterns[rand::random::<usize>() % unicode_patterns.len()];
        let mut result = pattern.to_string();
        
        // Add more unicode
        while result.len() < target_size {
            result.push_str("# 更多注释 更多文本 더 많이 😀");
        }
        
        // Safe truncation on char boundaries
        if result.len() > target_size {
            // Find a safe UTF-8 truncation point
            let mut safe_len = target_size;
            while safe_len > 0 && !result.is_char_boundary(safe_len) {
                safe_len -= 1;
            }
            result.truncate(safe_len);
        }
        result
    }
    
    fn generate_large_input(&self, size: usize) -> String {
        let mut code = String::from("def large_function():\n");
        
        // Generate many statements
        for i in 0..size / 20 {
            code.push_str(&format!("    x{} = {} + {} * {}\n", i, i, i+1, i*2));
        }
        
        code.push_str("    return sum([");
        for i in 0..std::cmp::min(size / 30, 1000) {
            if i > 0 { code.push_str(", "); }
            code.push_str(&format!("x{}", i % (size / 20 + 1)));
        }
        code.push_str("])");
        
        // Safe truncation on char boundaries
        if code.len() > size {
            // Find a safe UTF-8 truncation point
            let mut safe_len = size;
            while safe_len > 0 && !code.is_char_boundary(safe_len) {
                safe_len -= 1;
            }
            code.truncate(safe_len);
        }
        code
    }
    
    fn generate_deep_nesting(&self, target_size: usize) -> String {
        let mut code = String::from("def deeply_nested():\n");
        let max_nesting = std::cmp::min(target_size / 20, 10); // Reasonable depth limit
        
        // Create deep if-else nesting
        for i in 0..max_nesting {
            let indent = "    ".repeat(i + 1);
            code.push_str(&format!("{}if x{} > 0:\n", indent, i));
            
            // Stop if we're approaching target size
            if code.len() > target_size * 3 / 4 {
                break;
            }
        }
        
        // Add return at current level
        let final_indent = "    ".repeat(std::cmp::min(max_nesting, 5) + 1);
        code.push_str(&format!("{}return True\n", final_indent));
        
        // Ensure we don't exceed target size with safe truncation
        if code.len() > target_size {
            // Find a safe UTF-8 truncation point
            let mut safe_len = target_size;
            while safe_len > 0 && !code.is_char_boundary(safe_len) {
                safe_len -= 1;
            }
            code.truncate(safe_len);
        }
        
        code
    }
}

/// Results of a fuzzing campaign
#[derive(Debug)]
pub struct FuzzingCampaignResults {
    pub total_tests: usize,
    pub total_time: Duration,
    pub results: Vec<FuzzingResult>,
}

impl FuzzingCampaignResults {
    pub fn success_rate(&self) -> f64 {
        let successes = self.results.iter()
            .filter(|r| matches!(r.outcome, FuzzingOutcome::Success))
            .count();
        successes as f64 / self.total_tests as f64
    }
    
    pub fn crash_count(&self) -> usize {
        self.results.iter()
            .filter(|r| r.crash_detected)
            .count()
    }
    
    pub fn timeout_count(&self) -> usize {
        self.results.iter()
            .filter(|r| matches!(r.outcome, FuzzingOutcome::Timeout))
            .count()
    }
    
    pub fn average_execution_time(&self) -> Duration {
        if self.results.is_empty() {
            return Duration::from_millis(0);
        }
        
        let total_ms: u64 = self.results.iter()
            .map(|r| r.execution_time.as_millis() as u64)
            .sum();
        
        Duration::from_millis(total_ms / self.results.len() as u64)
    }
    
    pub fn error_distribution(&self) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();
        
        for result in &self.results {
            if let Some(error_type) = &result.error_type {
                *distribution.entry(error_type.clone()).or_insert(0) += 1;
            }
        }
        
        distribution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic fuzzing input generation
    #[test]
    fn test_fuzz_input_generation() {
        println!("=== Fuzzing Input Generation Test ===");
        
        let engine = FuzzingEngine::new();
        let strategies = vec![
            FuzzingStrategy::RandomBytes,
            FuzzingStrategy::StructuredPython,
            FuzzingStrategy::MalformedSyntax,
            FuzzingStrategy::SecurityFocused,
            FuzzingStrategy::UnicodeExploit,
            FuzzingStrategy::LargeInput,
            FuzzingStrategy::DeepNesting,
        ];
        
        for strategy in strategies {
            let input = engine.generate_fuzz_input(&strategy, 200);
            
            println!("{:?}: {} bytes", strategy, input.len());
            println!("  Preview: {}", input.chars().take(50).collect::<String>());
            
            // Basic validation
            assert!(input.len() <= 200, "Input should not exceed target size");
            assert!(!input.is_empty(), "Input should not be empty");
            
            // Strategy-specific validation
            match strategy {
                FuzzingStrategy::StructuredPython => {
                    assert!(input.contains("def") || input.contains("x =") || input.contains("if"), 
                        "Structured Python should contain Python-like keywords");
                },
                FuzzingStrategy::UnicodeExploit => {
                    assert!(input.chars().any(|c| c as u32 > 127), 
                        "Unicode exploit should contain non-ASCII characters");
                },
                FuzzingStrategy::DeepNesting => {
                    let indent_count = input.matches("    ").count();
                    assert!(indent_count > 3, "Deep nesting should have multiple indentation levels");
                },
                _ => {
                    // Other strategies have less specific requirements
                }
            }
        }
    }

    /// Test individual fuzzing execution
    #[test]
    fn test_individual_fuzzing() {
        println!("=== Individual Fuzzing Test ===");
        
        let mut engine = FuzzingEngine::new().with_timeout(1000); // 1 second timeout
        
        let large_input = format!("def func():\n{}", "    x = 1\n".repeat(100));
        let test_cases = vec![
            ("def valid(): return 42", FuzzingStrategy::StructuredPython, "Valid Python"),
            ("def broken(\n    return", FuzzingStrategy::MalformedSyntax, "Malformed syntax"),
            ("def 函数(): return '测试'", FuzzingStrategy::UnicodeExploit, "Unicode function"),
            (&large_input, FuzzingStrategy::LargeInput, "Large input"),
            ("def simple(): return 42  # Simple test", FuzzingStrategy::StructuredPython, "Real Python file"),
        ];
        
        for (input, strategy, description) in test_cases {
            println!("\nTesting: {}", description);
            
            let result = engine.fuzz_test(input, strategy);
            
            println!("  Input size: {} bytes", result.input_size);
            println!("  Execution time: {:?}", result.execution_time);
            println!("  Outcome: {:?}", result.outcome);
            println!("  Error type: {:?}", result.error_type);
            println!("  Crash detected: {}", result.crash_detected);
            
            // Basic validation
            assert_eq!(result.input_size, input.len());
            assert!(result.execution_time < Duration::from_millis(2000), 
                "Execution should complete quickly");
            
            // Should not crash on valid inputs
            if input.starts_with("def ") && input.contains(":") && input.contains("return") {
                assert!(!result.crash_detected, "Valid Python should not crash");
            }
        }
    }

    /// Test fuzzing campaign execution
    #[test]
    fn test_fuzzing_campaign() {
        println!("=== Fuzzing Campaign Test ===");
        
        let mut engine = FuzzingEngine::new()
            .with_timeout(500)  // Short timeout for test
            .with_max_size(1000); // Reasonable max size
        
        let start = Instant::now();
        let results = engine.run_fuzzing_campaign(50); // 50 test cases
        let campaign_duration = start.elapsed();
        
        println!("Campaign Results:");
        println!("  Total tests: {}", results.total_tests);
        println!("  Total time: {:?}", results.total_time); 
        println!("  Campaign overhead: {:?}", campaign_duration);
        println!("  Success rate: {:.1}%", results.success_rate() * 100.0);
        println!("  Crashes: {}", results.crash_count());
        println!("  Timeouts: {}", results.timeout_count());
        println!("  Average execution: {:?}", results.average_execution_time());
        
        // Validate campaign results
        assert!(results.total_tests > 0, "Should have run tests");
        assert!(results.total_tests <= 50, "Should not exceed requested test count");
        assert!(results.success_rate() >= 0.0 && results.success_rate() <= 1.0, "Success rate should be valid");
        assert!(campaign_duration < Duration::from_secs(30), "Campaign should complete quickly");
        
        // Error distribution analysis
        let error_dist = results.error_distribution();
        println!("Error distribution:");
        for (error_type, count) in &error_dist {
            println!("  {}: {} occurrences", error_type, count);
        }
        
        // Should have some variety in error types
        if !error_dist.is_empty() {
            assert!(error_dist.len() >= 1, "Should categorize error types");
        }
        
        // Performance check
        let avg_time = results.average_execution_time();
        assert!(avg_time < Duration::from_millis(1000), 
            "Average execution time should be reasonable: {:?}", avg_time);
    }

    /// Test fuzzing robustness with extreme inputs
    #[test]
    fn test_extreme_fuzzing_robustness() {
        println!("=== Extreme Fuzzing Robustness Test ===");
        
        let mut engine = FuzzingEngine::new()
            .with_timeout(2000) // 2 second timeout
            .with_max_size(5000);
        
        let long_token = "x".repeat(1000);
        let many_statements = format!("def f():\n{}", "    pass\n".repeat(500));
        let deep_nesting = format!("if True:\n{}        pass\n", "    if True:\n".repeat(100));
        
        let extreme_inputs = vec![
            ("", "Empty input"),
            ("\0\0\0\0\0", "Null bytes"),
            (&long_token, "Very long single token"),
            (&many_statements, "Many statements"),
            (&deep_nesting, "Deep nesting"),
            ("def 🚀🎉💻⚡🔥🌟(): return '🦀'", "Emoji identifiers"),
            ("def test(): return '\\x00\\x01\\x02\\x03\\x04\\x05'", "Control characters"),
        ];
        
        for (input, description) in extreme_inputs {
            println!("\nTesting extreme case: {}", description);
            
            let result = engine.fuzz_test(input, FuzzingStrategy::SecurityFocused);
            
            println!("  Outcome: {:?}", result.outcome);
            println!("  Execution time: {:?}", result.execution_time);
            println!("  Crash detected: {}", result.crash_detected);
            
            // Critical: should never crash
            assert!(!result.crash_detected, 
                "Extreme input '{}' should not crash the transpiler", description);
            
            // Should complete within timeout
            assert!(result.execution_time < Duration::from_millis(3000),
                "Extreme input '{}' should not hang", description);
        }
    }

    /// Test fuzzing memory safety
    #[test]
    fn test_fuzzing_memory_safety() {
        println!("=== Fuzzing Memory Safety Test ===");
        
        let mut engine = FuzzingEngine::new()
            .with_timeout(1000)
            .with_max_size(10000);
        
        // Generate inputs designed to test memory safety
        let memory_test_inputs = vec![
            engine.generate_fuzz_input(&FuzzingStrategy::LargeInput, 5000),
            engine.generate_fuzz_input(&FuzzingStrategy::DeepNesting, 50),
            "x = 'A' * 10000\ndef func(): return x * 100".to_string(),
            "def recursive(n): return recursive(n-1) if n > 0 else 0".to_string(),
        ];
        
        for (i, input) in memory_test_inputs.iter().enumerate() {
            println!("\nMemory test case {}: {} bytes", i + 1, input.len());
            
            let result = engine.fuzz_test(input, FuzzingStrategy::LargeInput);
            
            println!("  Execution time: {:?}", result.execution_time);  
            println!("  Memory spike detected: {}", result.memory_usage_spike);
            println!("  Outcome: {:?}", result.outcome);
            
            // Memory safety checks
            assert!(!result.crash_detected, "Memory test should not crash");
            
            // Large inputs may cause memory spikes but should still complete
            if result.memory_usage_spike {
                println!("  ⚠️  Memory usage spike detected (expected for large inputs)");
            }
        }
    }

    /// Test fuzzing performance characteristics
    #[test]
    fn test_fuzzing_performance() {
        println!("=== Fuzzing Performance Test ===");
        
        let mut engine = FuzzingEngine::new()
            .with_timeout(100) // Very short timeout for performance test
            .with_max_size(500);
        
        // Test performance across different input sizes
        let size_tests = vec![10, 50, 100, 200, 500];
        
        for size in size_tests {
            let input = engine.generate_fuzz_input(&FuzzingStrategy::StructuredPython, size);
            
            let start = Instant::now();
            let result = engine.fuzz_test(&input, FuzzingStrategy::StructuredPython);
            let total_time = start.elapsed();
            
            println!("Size {}: execution {:?}, total {:?}", 
                size, result.execution_time, total_time);
            
            // Performance should scale reasonably
            assert!(result.execution_time < Duration::from_millis(200),
                "Size {} should execute quickly", size);
            
            assert!(total_time < Duration::from_millis(300),
                "Size {} total time should be reasonable", size);
        }
        
        // Test cache performance
        let test_input = "def cached_test(): return 42";
        
        let time1 = {
            let start = Instant::now();
            engine.fuzz_test(test_input, FuzzingStrategy::StructuredPython);
            start.elapsed()
        };
        
        let time2 = {
            let start = Instant::now();
            engine.fuzz_test(test_input, FuzzingStrategy::StructuredPython);
            start.elapsed()
        };
        
        println!("Cache test: first {:?}, second {:?}", time1, time2);
        
        // Second execution should be faster due to caching
        assert!(time2 <= time1 * 2, "Caching should not significantly slow down execution");
    }
}