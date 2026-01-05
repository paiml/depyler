//! Five Whys Root Cause Analysis
//!
//! Implements Toyota's "Ask 'why' five times about every matter" technique
//! for identifying the true root cause of compilation failures.
//!
//! Reference: Ohno, T. (1988). Toyota Production System, pp. 17-20

use super::hansei::CycleOutcome;

/// A single "Why" step in the analysis chain
#[derive(Debug, Clone)]
pub struct WhyStep {
    /// The depth of this why (1-5)
    pub depth: u8,
    /// The question asked
    pub question: String,
    /// The answer/cause identified
    pub description: String,
    /// Whether this is the root cause
    pub is_root_cause: bool,
    /// Suggested preventive measure
    pub preventive_measure: String,
    /// Deeper cause (if not root)
    pub deeper_cause: Option<String>,
}

impl WhyStep {
    /// Create a new why step
    pub fn new(depth: u8, description: &str) -> Self {
        Self {
            depth,
            question: format!("Why #{}?", depth),
            description: description.to_string(),
            is_root_cause: false,
            preventive_measure: String::new(),
            deeper_cause: None,
        }
    }

    /// Mark this as the root cause with a preventive measure
    pub fn mark_as_root(mut self, preventive_measure: &str) -> Self {
        self.is_root_cause = true;
        self.preventive_measure = preventive_measure.to_string();
        self
    }

    /// Set the deeper cause
    pub fn with_deeper_cause(mut self, cause: &str) -> Self {
        self.deeper_cause = Some(cause.to_string());
        self
    }
}

/// Chain of Why steps forming a root cause analysis
#[derive(Debug, Clone, Default)]
pub struct RootCauseChain {
    /// The why steps in order
    pub whys: Vec<WhyStep>,
}

impl RootCauseChain {
    /// Create a new empty chain
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a why step
    pub fn add_why(&mut self, step: WhyStep) {
        self.whys.push(step);
    }

    /// Get the root cause (last step marked as root)
    pub fn root_cause(&self) -> Option<&WhyStep> {
        self.whys.iter().find(|w| w.is_root_cause)
    }

    /// Get depth of analysis
    pub fn depth(&self) -> usize {
        self.whys.len()
    }

    /// Check if root cause was found
    pub fn found_root_cause(&self) -> bool {
        self.whys.iter().any(|w| w.is_root_cause)
    }

    /// Export to markdown
    pub fn to_markdown(&self) -> String {
        let mut md = String::from("## Five Whys Analysis\n\n");

        for why in &self.whys {
            let marker = if why.is_root_cause { "🎯 " } else { "" };
            md.push_str(&format!(
                "### {}Why #{}: {}\n\n",
                marker, why.depth, why.description
            ));

            if why.is_root_cause {
                md.push_str(&format!(
                    "**Root Cause Identified**\n\n**Preventive Measure:** {}\n\n",
                    why.preventive_measure
                ));
            } else if let Some(ref deeper) = why.deeper_cause {
                md.push_str(&format!("*Leads to:* {}\n\n", deeper));
            }
        }

        md
    }
}

/// Five Whys Analyzer: Performs root cause analysis on failures
#[derive(Debug, Default)]
pub struct FiveWhysAnalyzer {
    /// Knowledge base of common root causes
    known_patterns: Vec<RootCausePattern>,
}

/// A known pattern with associated root cause chain
#[derive(Debug, Clone)]
struct RootCausePattern {
    /// Error code this pattern matches
    error_code: String,
    /// Keywords in error message
    keywords: Vec<String>,
    /// Pre-built why chain for this pattern
    chain: RootCauseChain,
}

impl FiveWhysAnalyzer {
    /// Create a new analyzer with built-in knowledge
    pub fn new() -> Self {
        let mut analyzer = Self::default();
        analyzer.register_known_patterns();
        analyzer
    }

    /// Register built-in patterns
    fn register_known_patterns(&mut self) {
        // E0308: Type mismatch
        self.known_patterns.push(RootCausePattern {
            error_code: "E0308".to_string(),
            keywords: vec!["expected".to_string(), "found".to_string()],
            chain: Self::build_type_mismatch_chain(),
        });

        // E0432: Unresolved import
        self.known_patterns.push(RootCausePattern {
            error_code: "E0432".to_string(),
            keywords: vec!["unresolved".to_string(), "import".to_string()],
            chain: Self::build_import_chain(),
        });

        // E0277: Trait bound not satisfied
        self.known_patterns.push(RootCausePattern {
            error_code: "E0277".to_string(),
            keywords: vec!["trait".to_string(), "bound".to_string()],
            chain: Self::build_trait_bound_chain(),
        });

        // E0502/E0503: Borrowing conflicts
        self.known_patterns.push(RootCausePattern {
            error_code: "E0502".to_string(),
            keywords: vec!["borrow".to_string(), "mutable".to_string()],
            chain: Self::build_borrow_chain(),
        });
    }

    /// Build chain for type mismatch errors
    fn build_type_mismatch_chain() -> RootCauseChain {
        let mut chain = RootCauseChain::new();

        chain.add_why(WhyStep::new(1, "Type mismatch between expected and actual type")
            .with_deeper_cause("Type inference produced wrong type"));

        chain.add_why(WhyStep::new(2, "Type inference produced wrong type")
            .with_deeper_cause("Insufficient type context in Python source"));

        chain.add_why(WhyStep::new(3, "Insufficient type context in Python source")
            .with_deeper_cause("Python's dynamic typing allows implicit conversions"));

        chain.add_why(WhyStep::new(4, "Python's dynamic typing allows implicit conversions")
            .with_deeper_cause("Transpiler doesn't add explicit conversions"));

        chain.add_why(WhyStep::new(5, "Transpiler codegen missing type coercion logic")
            .mark_as_root("Add .into(), .to_string(), or explicit type conversion in expr_gen.rs"));

        chain
    }

    /// Build chain for import errors
    fn build_import_chain() -> RootCauseChain {
        let mut chain = RootCauseChain::new();

        chain.add_why(WhyStep::new(1, "External crate not found")
            .with_deeper_cause("Cargo.toml missing dependency"));

        chain.add_why(WhyStep::new(2, "Cargo.toml missing dependency")
            .with_deeper_cause("Python import not mapped to Rust crate"));

        chain.add_why(WhyStep::new(3, "Python import not mapped to Rust crate")
            .mark_as_root("Add mapping in module_mapper.rs and cargo_toml_gen.rs"));

        chain
    }

    /// Build chain for trait bound errors
    fn build_trait_bound_chain() -> RootCauseChain {
        let mut chain = RootCauseChain::new();

        chain.add_why(WhyStep::new(1, "Trait bound not satisfied")
            .with_deeper_cause("Generated type doesn't implement required trait"));

        chain.add_why(WhyStep::new(2, "Generated type doesn't implement required trait")
            .with_deeper_cause("Wrong type chosen for dynamic data"));

        chain.add_why(WhyStep::new(3, "Wrong type chosen for dynamic data")
            .with_deeper_cause("Type inference defaults to concrete type instead of trait object"));

        chain.add_why(WhyStep::new(4, "Type inference defaults to concrete type")
            .mark_as_root("Use serde_json::Value or Box<dyn Trait> for heterogeneous data"));

        chain
    }

    /// Build chain for borrowing errors
    fn build_borrow_chain() -> RootCauseChain {
        let mut chain = RootCauseChain::new();

        chain.add_why(WhyStep::new(1, "Cannot borrow as mutable while borrowed as immutable")
            .with_deeper_cause("Multiple references to same data"));

        chain.add_why(WhyStep::new(2, "Multiple references to same data")
            .with_deeper_cause("Python code pattern doesn't translate directly to Rust"));

        chain.add_why(WhyStep::new(3, "Python pattern incompatible with Rust ownership")
            .with_deeper_cause("Transpiler generates naive translation"));

        chain.add_why(WhyStep::new(4, "Transpiler generates naive translation")
            .mark_as_root("Add .clone(), restructure to avoid aliasing, or use RefCell"));

        chain
    }

    /// Analyze a compilation failure
    pub fn analyze(&self, error_code: &str, error_message: &str) -> RootCauseChain {
        // Try to find a matching known pattern
        for pattern in &self.known_patterns {
            if pattern.error_code == error_code {
                // Check keywords
                let matches_keywords = pattern.keywords.iter()
                    .any(|kw| error_message.to_lowercase().contains(&kw.to_lowercase()));

                if matches_keywords {
                    return pattern.chain.clone();
                }
            }
        }

        // Build generic chain for unknown errors
        self.build_generic_chain(error_code, error_message)
    }

    /// Analyze from a cycle outcome
    pub fn analyze_from_outcome(&self, outcome: &CycleOutcome) -> RootCauseChain {
        self.analyze(&outcome.pattern.error_code, &outcome.pattern.description)
    }

    /// Build a generic chain for unknown errors
    fn build_generic_chain(&self, error_code: &str, error_message: &str) -> RootCauseChain {
        let mut chain = RootCauseChain::new();

        chain.add_why(WhyStep::new(1, &format!("Compilation error {}: {}", error_code, error_message))
            .with_deeper_cause("Generated Rust code invalid"));

        chain.add_why(WhyStep::new(2, "Generated Rust code invalid")
            .with_deeper_cause("Transpiler codegen incomplete for this pattern"));

        chain.add_why(WhyStep::new(3, "Codegen incomplete for pattern")
            .mark_as_root("Investigate expr_gen.rs/stmt_gen.rs for missing case handling"));

        chain
    }

    /// Interactive Five Whys session (for CLI)
    pub fn interactive_session(&self, initial_problem: &str) -> RootCauseChain {
        let mut chain = RootCauseChain::new();

        // In a real implementation, this would prompt the user
        // For now, use the initial problem and build a basic chain
        chain.add_why(WhyStep::new(1, initial_problem)
            .mark_as_root("Requires manual investigation"));

        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_new() {
        let analyzer = FiveWhysAnalyzer::new();
        assert!(!analyzer.known_patterns.is_empty());
    }

    #[test]
    fn test_analyze_type_mismatch() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.analyze("E0308", "expected String, found i32");

        assert!(chain.found_root_cause());
        assert_eq!(chain.depth(), 5);

        let root = chain.root_cause().unwrap();
        assert!(root.preventive_measure.contains("expr_gen"));
    }

    #[test]
    fn test_analyze_import_error() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.analyze("E0432", "unresolved import serde_json");

        assert!(chain.found_root_cause());
        assert!(chain.depth() <= 5);

        let root = chain.root_cause().unwrap();
        assert!(root.preventive_measure.contains("module_mapper"));
    }

    #[test]
    fn test_analyze_unknown_error() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.analyze("E9999", "unknown error");

        // Should still produce a chain
        assert!(!chain.whys.is_empty());
        assert!(chain.found_root_cause());
    }

    #[test]
    fn test_why_step_creation() {
        let step = WhyStep::new(1, "Test description");
        assert_eq!(step.depth, 1);
        assert!(!step.is_root_cause);
    }

    #[test]
    fn test_why_step_mark_as_root() {
        let step = WhyStep::new(3, "Root cause")
            .mark_as_root("Fix this thing");

        assert!(step.is_root_cause);
        assert_eq!(step.preventive_measure, "Fix this thing");
    }

    #[test]
    fn test_root_cause_chain_to_markdown() {
        let mut chain = RootCauseChain::new();
        chain.add_why(WhyStep::new(1, "Problem A")
            .with_deeper_cause("Leads to B"));
        chain.add_why(WhyStep::new(2, "Problem B")
            .mark_as_root("Fix B"));

        let md = chain.to_markdown();
        assert!(md.contains("Five Whys Analysis"));
        assert!(md.contains("Why #1"));
        assert!(md.contains("Why #2"));
        assert!(md.contains("Root Cause"));
    }

    #[test]
    fn test_chain_root_cause() {
        let mut chain = RootCauseChain::new();
        chain.add_why(WhyStep::new(1, "Not root"));
        chain.add_why(WhyStep::new(2, "Is root").mark_as_root("Fix it"));

        let root = chain.root_cause();
        assert!(root.is_some());
        assert_eq!(root.unwrap().depth, 2);
    }

    #[test]
    fn test_chain_no_root_cause() {
        let mut chain = RootCauseChain::new();
        chain.add_why(WhyStep::new(1, "Not root"));

        assert!(!chain.found_root_cause());
        assert!(chain.root_cause().is_none());
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_why_step_debug() {
        let step = WhyStep::new(2, "Test step");
        let debug_str = format!("{:?}", step);
        assert!(debug_str.contains("WhyStep"));
        assert!(debug_str.contains("depth"));
        assert!(debug_str.contains("Test step"));
    }

    #[test]
    fn test_why_step_clone() {
        let step = WhyStep::new(3, "Original")
            .mark_as_root("Fix it")
            .with_deeper_cause("Deeper");
        let cloned = step.clone();
        assert_eq!(cloned.depth, 3);
        assert!(cloned.is_root_cause);
        assert_eq!(cloned.preventive_measure, "Fix it");
    }

    #[test]
    fn test_why_step_with_deeper_cause() {
        let step = WhyStep::new(1, "Problem")
            .with_deeper_cause("Root issue");
        assert_eq!(step.deeper_cause, Some("Root issue".to_string()));
    }

    #[test]
    fn test_why_step_question_format() {
        let step = WhyStep::new(4, "desc");
        assert_eq!(step.question, "Why #4?");
    }

    #[test]
    fn test_root_cause_chain_default() {
        let chain: RootCauseChain = Default::default();
        assert!(chain.whys.is_empty());
        assert_eq!(chain.depth(), 0);
    }

    #[test]
    fn test_root_cause_chain_debug() {
        let chain = RootCauseChain::new();
        let debug_str = format!("{:?}", chain);
        assert!(debug_str.contains("RootCauseChain"));
    }

    #[test]
    fn test_root_cause_chain_clone() {
        let mut chain = RootCauseChain::new();
        chain.add_why(WhyStep::new(1, "Step 1"));
        chain.add_why(WhyStep::new(2, "Step 2").mark_as_root("Fix"));

        let cloned = chain.clone();
        assert_eq!(cloned.depth(), 2);
        assert!(cloned.found_root_cause());
    }

    #[test]
    fn test_root_cause_chain_depth() {
        let mut chain = RootCauseChain::new();
        assert_eq!(chain.depth(), 0);

        chain.add_why(WhyStep::new(1, "First"));
        assert_eq!(chain.depth(), 1);

        chain.add_why(WhyStep::new(2, "Second"));
        chain.add_why(WhyStep::new(3, "Third"));
        assert_eq!(chain.depth(), 3);
    }

    #[test]
    fn test_five_whys_analyzer_default() {
        let analyzer: FiveWhysAnalyzer = Default::default();
        // Default has no patterns
        assert!(analyzer.known_patterns.is_empty());
    }

    #[test]
    fn test_five_whys_analyzer_debug() {
        let analyzer = FiveWhysAnalyzer::new();
        let debug_str = format!("{:?}", analyzer);
        assert!(debug_str.contains("FiveWhysAnalyzer"));
    }

    #[test]
    fn test_analyze_trait_bound_error() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.analyze("E0277", "the trait bound is not satisfied");

        assert!(chain.found_root_cause());
        assert!(chain.depth() >= 3);

        let root = chain.root_cause().unwrap();
        assert!(root.preventive_measure.contains("serde_json::Value") ||
                root.preventive_measure.contains("Box<dyn Trait>"));
    }

    #[test]
    fn test_analyze_borrow_error() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.analyze("E0502", "cannot borrow as mutable");

        assert!(chain.found_root_cause());
        assert!(chain.depth() >= 3);

        let root = chain.root_cause().unwrap();
        assert!(root.preventive_measure.contains("clone") ||
                root.preventive_measure.contains("RefCell"));
    }

    #[test]
    fn test_analyze_from_outcome() {
        use super::super::hansei::CycleOutcome;
        use super::super::planner::{FailurePattern, PatternCategory};
        use super::super::isolator::ReproCase;
        use super::super::verifier::VerifyResult;
        use super::super::kaizen::KaizenMetrics;

        let analyzer = FiveWhysAnalyzer::new();
        let outcome = CycleOutcome {
            pattern: FailurePattern {
                id: "test".to_string(),
                error_code: "E0308".to_string(),
                description: "expected String found i32".to_string(),
                category: PatternCategory::TypeInference,
                affected_count: 10,
                fix_complexity: 5,
                trigger_example: String::new(),
            },
            repro: ReproCase::new("test".to_string(), "E0308".to_string(), "test".to_string()),
            verify_result: VerifyResult::Success,
            metrics_snapshot: KaizenMetrics::default(),
        };

        let chain = analyzer.analyze_from_outcome(&outcome);
        assert!(chain.found_root_cause());
    }

    #[test]
    fn test_interactive_session() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.interactive_session("Code doesn't compile");

        // Interactive session creates a basic chain
        assert!(!chain.whys.is_empty());
        assert!(chain.found_root_cause());
    }

    #[test]
    fn test_to_markdown_with_deeper_cause() {
        let mut chain = RootCauseChain::new();
        chain.add_why(WhyStep::new(1, "Surface problem")
            .with_deeper_cause("Leads to deeper issue"));

        let md = chain.to_markdown();
        assert!(md.contains("Leads to:"));
        assert!(md.contains("deeper issue"));
    }

    #[test]
    fn test_to_markdown_empty_chain() {
        let chain = RootCauseChain::new();
        let md = chain.to_markdown();
        assert!(md.contains("Five Whys Analysis"));
        // No whys in empty chain
        assert!(!md.contains("Why #1"));
    }

    #[test]
    fn test_known_patterns_count() {
        let analyzer = FiveWhysAnalyzer::new();
        // Should have patterns for E0308, E0432, E0277, E0502
        assert_eq!(analyzer.known_patterns.len(), 4);
    }

    #[test]
    fn test_generic_chain_structure() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain = analyzer.build_generic_chain("E1234", "unknown error message");

        assert!(chain.found_root_cause());
        assert_eq!(chain.depth(), 3);

        let root = chain.root_cause().unwrap();
        assert!(root.preventive_measure.contains("expr_gen") ||
                root.preventive_measure.contains("stmt_gen"));
    }

    #[test]
    fn test_analyze_no_keyword_match() {
        let analyzer = FiveWhysAnalyzer::new();
        // E0308 pattern but no matching keywords
        let chain = analyzer.analyze("E0308", "completely different message");

        // Should fall back to generic chain
        assert!(chain.found_root_cause());
    }

    #[test]
    fn test_root_cause_pattern_clone() {
        let analyzer = FiveWhysAnalyzer::new();
        let chain1 = analyzer.analyze("E0308", "expected String, found i32");
        let chain2 = analyzer.analyze("E0308", "expected String, found bool");

        // Both should return cloned chains from the same pattern
        assert_eq!(chain1.depth(), chain2.depth());
    }
}
