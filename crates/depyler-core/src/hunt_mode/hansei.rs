//! Hansei (反省) - Reflection and Lessons Learned
//!
//! Implements Toyota's practice of honest self-reflection to
//! identify improvement opportunities.
//!
//! "Hansei is about being honest about your own weaknesses."
//! - Jeffrey Liker, The Toyota Way

use super::kaizen::KaizenMetrics;
use super::planner::FailurePattern;
use super::isolator::ReproCase;
use super::verifier::VerifyResult;

/// A lesson learned from a Hunt Mode cycle
#[derive(Debug, Clone)]
pub struct Lesson {
    /// Category of lesson (e.g., "type_system", "imports", "borrowing")
    pub category: String,
    /// What was observed
    pub observation: String,
    /// Recommended action to prevent recurrence
    pub action: String,
    /// Confidence in this lesson (0.0 - 1.0)
    pub confidence: f64,
    /// Number of times this pattern was observed
    pub occurrences: u32,
}

impl Lesson {
    /// Create a new lesson
    pub fn new(category: &str, observation: &str, action: &str) -> Self {
        Self {
            category: category.to_string(),
            observation: observation.to_string(),
            action: action.to_string(),
            confidence: 0.5,
            occurrences: 1,
        }
    }

    /// Reinforce this lesson (increase confidence and occurrences)
    pub fn reinforce(&mut self) {
        self.occurrences += 1;
        self.confidence = (self.confidence + 0.1).min(1.0);
    }
}

/// Outcome of a Hunt Mode cycle
#[derive(Debug, Clone)]
pub struct CycleOutcome {
    /// The failure pattern that was targeted
    pub pattern: FailurePattern,
    /// The reproduction case used
    pub repro: ReproCase,
    /// Result of verification
    pub verify_result: VerifyResult,
    /// Metrics snapshot at end of cycle
    pub metrics_snapshot: KaizenMetrics,
}

impl CycleOutcome {
    /// Check if this cycle resulted in a successful fix
    pub fn was_successful(&self) -> bool {
        matches!(self.verify_result, VerifyResult::Success)
    }

    /// Check if this cycle needs human review
    pub fn needs_review(&self) -> bool {
        matches!(self.verify_result, VerifyResult::NeedsReview { .. })
    }

    /// Get the fix type category
    pub fn fix_category(&self) -> &str {
        match self.pattern.category {
            super::planner::PatternCategory::TypeInference => "type_inference",
            super::planner::PatternCategory::ExternalDeps => "external_deps",
            super::planner::PatternCategory::Borrowing => "borrowing",
            super::planner::PatternCategory::ControlFlow => "control_flow",
            super::planner::PatternCategory::Miscellaneous => "misc",
        }
    }
}

/// Hansei Reflector: Extracts lessons from cycle outcomes
///
/// Hansei: Reflect on what worked and what didn't.
#[derive(Debug, Default)]
pub struct HanseiReflector {
    /// History of all cycle outcomes
    cycle_history: Vec<CycleOutcome>,
    /// Lessons learned across all cycles
    lessons_learned: Vec<Lesson>,
    /// Five Whys analyzer for root cause analysis
    five_whys: super::five_whys::FiveWhysAnalyzer,
}

impl HanseiReflector {
    /// Create a new reflector
    pub fn new() -> Self {
        Self::default()
    }

    /// Reflect on a completed cycle and extract lessons
    ///
    /// Hansei: Be honest about weaknesses to improve.
    pub fn reflect_on_cycle(&mut self, outcome: &CycleOutcome) -> Vec<Lesson> {
        let mut new_lessons = Vec::new();

        // Pattern-based lessons
        new_lessons.extend(self.analyze_pattern_lessons(outcome));

        // Success/failure lessons
        if outcome.was_successful() {
            new_lessons.extend(self.analyze_success_lessons(outcome));
        } else {
            new_lessons.extend(self.analyze_failure_lessons(outcome));
        }

        // Five Whys analysis for failures
        if !outcome.was_successful() {
            if let Some(root_cause_lesson) = self.five_whys_lesson(outcome) {
                new_lessons.push(root_cause_lesson);
            }
        }

        // Record cycle in history
        self.cycle_history.push(outcome.clone());

        // Merge new lessons with existing (reinforce duplicates)
        for lesson in &new_lessons {
            self.add_or_reinforce_lesson(lesson.clone());
        }

        new_lessons
    }

    /// Analyze pattern-specific lessons
    fn analyze_pattern_lessons(&self, outcome: &CycleOutcome) -> Vec<Lesson> {
        let mut lessons = Vec::new();
        let category = outcome.fix_category();

        match category {
            "type_inference" => {
                lessons.push(Lesson::new(
                    "type_system",
                    &format!(
                        "Type inference failure for error {}: {}",
                        outcome.pattern.error_code,
                        outcome.pattern.description
                    ),
                    "Consider adding explicit type annotations or using serde_json::Value fallback",
                ));
            }
            "external_deps" => {
                lessons.push(Lesson::new(
                    "imports",
                    &format!(
                        "External dependency required: {}",
                        outcome.pattern.description
                    ),
                    "Ensure Cargo.toml includes required crates (serde_json, regex, etc.)",
                ));
            }
            "borrowing" => {
                lessons.push(Lesson::new(
                    "ownership",
                    &format!(
                        "Borrowing conflict: {}",
                        outcome.pattern.description
                    ),
                    "Review borrow checker rules; consider .clone() or restructuring",
                ));
            }
            _ => {}
        }

        lessons
    }

    /// Analyze lessons from successful fixes
    fn analyze_success_lessons(&self, outcome: &CycleOutcome) -> Vec<Lesson> {
        vec![Lesson::new(
            "success_pattern",
            &format!(
                "Successfully fixed {} pattern",
                outcome.fix_category()
            ),
            "Apply similar fix strategy to related patterns",
        )]
    }

    /// Analyze lessons from failures
    fn analyze_failure_lessons(&self, outcome: &CycleOutcome) -> Vec<Lesson> {
        let reason = match &outcome.verify_result {
            VerifyResult::FixFailed(e) => e.clone(),
            VerifyResult::NeedsReview { reason, .. } => reason.clone(),
            _ => "Unknown failure".to_string(),
        };

        vec![Lesson::new(
            "failure_analysis",
            &format!("Fix attempt failed: {}", reason),
            "Review fix strategy; may need different approach or manual intervention",
        )]
    }

    /// Generate lesson from Five Whys analysis
    fn five_whys_lesson(&self, outcome: &CycleOutcome) -> Option<Lesson> {
        let root_cause = self.five_whys.analyze_from_outcome(outcome);

        root_cause.root_cause().map(|root| Lesson::new(
            "root_cause",
            &root.description,
            &root.preventive_measure,
        ))
    }

    /// Add a lesson or reinforce existing one
    fn add_or_reinforce_lesson(&mut self, lesson: Lesson) {
        // Check if similar lesson exists
        if let Some(existing) = self.lessons_learned.iter_mut().find(|l| {
            l.category == lesson.category && l.observation.contains(&lesson.observation[..20.min(lesson.observation.len())])
        }) {
            existing.reinforce();
        } else {
            self.lessons_learned.push(lesson);
        }
    }

    /// Get all lessons learned
    pub fn lessons(&self) -> &[Lesson] {
        &self.lessons_learned
    }

    /// Get lessons by category
    pub fn lessons_by_category(&self, category: &str) -> Vec<&Lesson> {
        self.lessons_learned
            .iter()
            .filter(|l| l.category == category)
            .collect()
    }

    /// Get high-confidence lessons (confidence > 0.7)
    pub fn high_confidence_lessons(&self) -> Vec<&Lesson> {
        self.lessons_learned
            .iter()
            .filter(|l| l.confidence > 0.7)
            .collect()
    }

    /// Export lessons to markdown
    pub fn export_markdown(&self) -> String {
        let mut md = String::from("# Hansei Report: Lessons Learned\n\n");

        // Group by category
        let mut categories: std::collections::HashMap<&str, Vec<&Lesson>> =
            std::collections::HashMap::new();

        for lesson in &self.lessons_learned {
            categories
                .entry(&lesson.category)
                .or_default()
                .push(lesson);
        }

        for (category, lessons) in categories {
            md.push_str(&format!("## {}\n\n", category));

            for lesson in lessons {
                md.push_str(&format!(
                    "### Observation (confidence: {:.0}%, occurrences: {})\n",
                    lesson.confidence * 100.0,
                    lesson.occurrences
                ));
                md.push_str(&format!("{}\n\n", lesson.observation));
                md.push_str(&format!("**Recommended Action:** {}\n\n", lesson.action));
            }
        }

        md.push_str(&format!(
            "\n---\n*Generated from {} cycles*\n",
            self.cycle_history.len()
        ));

        md
    }

    /// Get cycle history
    pub fn history(&self) -> &[CycleOutcome] {
        &self.cycle_history
    }

    /// Calculate overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.cycle_history.is_empty() {
            return 0.0;
        }
        let successes = self.cycle_history.iter().filter(|c| c.was_successful()).count();
        successes as f64 / self.cycle_history.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::planner::PatternCategory;

    fn create_test_outcome(success: bool) -> CycleOutcome {
        CycleOutcome {
            pattern: FailurePattern {
                id: "test".to_string(),
                error_code: "E0308".to_string(),
                description: "Test pattern".to_string(),
                category: PatternCategory::TypeInference,
                affected_count: 10,
                fix_complexity: 5,
                trigger_example: String::new(),
            },
            repro: ReproCase::new(
                "test".to_string(),
                "E0308".to_string(),
                "test".to_string(),
            ),
            verify_result: if success {
                VerifyResult::Success
            } else {
                VerifyResult::FixFailed("Test failure".to_string())
            },
            metrics_snapshot: KaizenMetrics::default(),
        }
    }

    #[test]
    fn test_reflector_new() {
        let reflector = HanseiReflector::new();
        assert!(reflector.lessons().is_empty());
        assert!(reflector.history().is_empty());
    }

    #[test]
    fn test_reflect_on_successful_cycle() {
        let mut reflector = HanseiReflector::new();
        let outcome = create_test_outcome(true);

        let lessons = reflector.reflect_on_cycle(&outcome);
        assert!(!lessons.is_empty());
        assert_eq!(reflector.history().len(), 1);
    }

    #[test]
    fn test_reflect_on_failed_cycle() {
        let mut reflector = HanseiReflector::new();
        let outcome = create_test_outcome(false);

        let lessons = reflector.reflect_on_cycle(&outcome);
        assert!(!lessons.is_empty());

        // Should have failure analysis lesson
        assert!(lessons.iter().any(|l| l.category == "failure_analysis"));
    }

    #[test]
    fn test_lesson_reinforcement() {
        let mut reflector = HanseiReflector::new();

        // Add same type of outcome twice
        let outcome1 = create_test_outcome(true);
        let outcome2 = create_test_outcome(true);

        reflector.reflect_on_cycle(&outcome1);
        reflector.reflect_on_cycle(&outcome2);

        // Should have reinforced existing lessons
        let _high_conf = reflector.high_confidence_lessons();
        // At least some lessons should have been reinforced
        assert!(reflector.lessons().iter().any(|l| l.occurrences > 1));
    }

    #[test]
    fn test_success_rate() {
        let mut reflector = HanseiReflector::new();

        reflector.reflect_on_cycle(&create_test_outcome(true));
        reflector.reflect_on_cycle(&create_test_outcome(true));
        reflector.reflect_on_cycle(&create_test_outcome(false));

        assert!((reflector.success_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_export_markdown() {
        let mut reflector = HanseiReflector::new();
        reflector.reflect_on_cycle(&create_test_outcome(true));

        let md = reflector.export_markdown();
        assert!(md.contains("# Hansei Report"));
        assert!(md.contains("Observation"));
    }

    #[test]
    fn test_lessons_by_category() {
        let mut reflector = HanseiReflector::new();
        reflector.reflect_on_cycle(&create_test_outcome(true));

        let type_lessons = reflector.lessons_by_category("type_system");
        assert!(!type_lessons.is_empty());
    }

    #[test]
    fn test_lesson_new() {
        let lesson = Lesson::new("test", "observation", "action");
        assert_eq!(lesson.category, "test");
        assert_eq!(lesson.occurrences, 1);
        assert!((lesson.confidence - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_lesson_reinforce() {
        let mut lesson = Lesson::new("test", "obs", "act");
        lesson.reinforce();

        assert_eq!(lesson.occurrences, 2);
        assert!((lesson.confidence - 0.6).abs() < f64::EPSILON);
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_lesson_debug() {
        let lesson = Lesson::new("type_system", "observation", "action");
        let debug_str = format!("{:?}", lesson);
        assert!(debug_str.contains("Lesson"));
        assert!(debug_str.contains("type_system"));
        assert!(debug_str.contains("observation"));
    }

    #[test]
    fn test_lesson_clone() {
        let lesson = Lesson {
            category: "test".to_string(),
            observation: "obs".to_string(),
            action: "act".to_string(),
            confidence: 0.75,
            occurrences: 5,
        };
        let cloned = lesson.clone();
        assert_eq!(cloned.category, "test");
        assert_eq!(cloned.confidence, 0.75);
        assert_eq!(cloned.occurrences, 5);
    }

    #[test]
    fn test_lesson_reinforce_max_confidence() {
        let mut lesson = Lesson {
            category: "test".to_string(),
            observation: "obs".to_string(),
            action: "act".to_string(),
            confidence: 0.95,
            occurrences: 10,
        };
        lesson.reinforce();
        lesson.reinforce();

        // Confidence should cap at 1.0
        assert!(lesson.confidence <= 1.0);
        assert_eq!(lesson.occurrences, 12);
    }

    #[test]
    fn test_cycle_outcome_debug() {
        let outcome = create_test_outcome(true);
        let debug_str = format!("{:?}", outcome);
        assert!(debug_str.contains("CycleOutcome"));
        assert!(debug_str.contains("pattern"));
    }

    #[test]
    fn test_cycle_outcome_clone() {
        let outcome = create_test_outcome(false);
        let cloned = outcome.clone();
        assert_eq!(cloned.pattern.id, outcome.pattern.id);
        assert!(!cloned.was_successful());
    }

    #[test]
    fn test_cycle_outcome_was_successful() {
        let success = create_test_outcome(true);
        assert!(success.was_successful());

        let failure = create_test_outcome(false);
        assert!(!failure.was_successful());
    }

    #[test]
    fn test_cycle_outcome_needs_review() {
        let outcome = CycleOutcome {
            pattern: FailurePattern {
                id: "test".to_string(),
                error_code: "E0308".to_string(),
                description: "Test".to_string(),
                category: PatternCategory::TypeInference,
                affected_count: 1,
                fix_complexity: 1,
                trigger_example: String::new(),
            },
            repro: ReproCase::new("test".to_string(), "E0308".to_string(), "test".to_string()),
            verify_result: VerifyResult::NeedsReview {
                fix: super::super::repair::Fix {
                    id: "fix".to_string(),
                    ticket_id: "DEPYLER-001".to_string(),
                    description: "desc".to_string(),
                    mutator_name: "mut".to_string(),
                    confidence: 0.5,
                    rust_output: String::new(),
                    patch_location: None,
                },
                confidence: 0.5,
                reason: "review".to_string(),
            },
            metrics_snapshot: KaizenMetrics::default(),
        };
        assert!(outcome.needs_review());
        assert!(!outcome.was_successful());
    }

    #[test]
    fn test_cycle_outcome_fix_category_type_inference() {
        let outcome = create_test_outcome(true);
        assert_eq!(outcome.fix_category(), "type_inference");
    }

    #[test]
    fn test_cycle_outcome_fix_category_external_deps() {
        let mut outcome = create_test_outcome(true);
        outcome.pattern.category = PatternCategory::ExternalDeps;
        assert_eq!(outcome.fix_category(), "external_deps");
    }

    #[test]
    fn test_cycle_outcome_fix_category_borrowing() {
        let mut outcome = create_test_outcome(true);
        outcome.pattern.category = PatternCategory::Borrowing;
        assert_eq!(outcome.fix_category(), "borrowing");
    }

    #[test]
    fn test_cycle_outcome_fix_category_control_flow() {
        let mut outcome = create_test_outcome(true);
        outcome.pattern.category = PatternCategory::ControlFlow;
        assert_eq!(outcome.fix_category(), "control_flow");
    }

    #[test]
    fn test_cycle_outcome_fix_category_misc() {
        let mut outcome = create_test_outcome(true);
        outcome.pattern.category = PatternCategory::Miscellaneous;
        assert_eq!(outcome.fix_category(), "misc");
    }

    #[test]
    fn test_hansei_reflector_default() {
        let reflector: HanseiReflector = Default::default();
        assert!(reflector.lessons().is_empty());
        assert!(reflector.history().is_empty());
    }

    #[test]
    fn test_hansei_reflector_debug() {
        let reflector = HanseiReflector::new();
        let debug_str = format!("{:?}", reflector);
        assert!(debug_str.contains("HanseiReflector"));
    }

    #[test]
    fn test_success_rate_empty() {
        let reflector = HanseiReflector::new();
        assert_eq!(reflector.success_rate(), 0.0);
    }

    #[test]
    fn test_high_confidence_lessons_empty() {
        let reflector = HanseiReflector::new();
        assert!(reflector.high_confidence_lessons().is_empty());
    }

    #[test]
    fn test_lessons_by_category_empty() {
        let reflector = HanseiReflector::new();
        let lessons = reflector.lessons_by_category("nonexistent");
        assert!(lessons.is_empty());
    }

    #[test]
    fn test_export_markdown_empty() {
        let reflector = HanseiReflector::new();
        let md = reflector.export_markdown();
        assert!(md.contains("# Hansei Report"));
        assert!(md.contains("0 cycles"));
    }

    #[test]
    fn test_lessons_by_category_success() {
        let mut reflector = HanseiReflector::new();
        reflector.reflect_on_cycle(&create_test_outcome(true));

        let success_lessons = reflector.lessons_by_category("success_pattern");
        assert!(!success_lessons.is_empty());
    }

    #[test]
    fn test_high_confidence_after_reinforcement() {
        let mut reflector = HanseiReflector::new();

        // Add many similar outcomes to reinforce lessons
        for _ in 0..10 {
            reflector.reflect_on_cycle(&create_test_outcome(true));
        }

        let high_conf = reflector.high_confidence_lessons();
        assert!(!high_conf.is_empty());
    }
}
