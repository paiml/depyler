//! Hunt Planner - Failure Pattern Classification and Prioritization
//!
//! Implements Heijunka (平準化) - Level the Workload
//! Processes errors in frequency order to ensure maximum impact per cycle.
//!
//! Uses the Pareto principle: 20% of patterns cause 80% of failures.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// A cluster of similar compilation errors
#[derive(Debug, Clone)]
pub struct ErrorCluster {
    /// Unique identifier for this cluster
    pub id: String,
    /// Error code (e.g., "E0308", "E0432")
    pub error_code: String,
    /// Human-readable description
    pub description: String,
    /// Number of occurrences in the corpus
    pub frequency: u32,
    /// Estimated severity (1-10)
    pub severity: u8,
    /// Example error messages in this cluster
    pub examples: Vec<String>,
}

/// A failure pattern that can be targeted for fixing
#[derive(Debug, Clone)]
pub struct FailurePattern {
    /// Unique pattern identifier
    pub id: String,
    /// Error code this pattern addresses
    pub error_code: String,
    /// Description of the pattern
    pub description: String,
    /// Category (e.g., "type_inference", "external_deps", "borrowing")
    pub category: PatternCategory,
    /// How many files exhibit this pattern
    pub affected_count: u32,
    /// Estimated complexity to fix (1-10)
    pub fix_complexity: u8,
    /// Example Python code that triggers this
    pub trigger_example: String,
}

/// Categories of failure patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternCategory {
    /// Type inference failures (15% of total)
    TypeInference,
    /// External dependency issues (68% of total)
    ExternalDeps,
    /// Borrowing and lifetime issues (10% of total)
    Borrowing,
    /// Control flow (try/except, match) issues (5% of total)
    ControlFlow,
    /// Other/miscellaneous (2% of total)
    Miscellaneous,
}

impl std::fmt::Display for PatternCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternCategory::TypeInference => write!(f, "Type Inference"),
            PatternCategory::ExternalDeps => write!(f, "External Dependencies"),
            PatternCategory::Borrowing => write!(f, "Borrowing"),
            PatternCategory::ControlFlow => write!(f, "Control Flow"),
            PatternCategory::Miscellaneous => write!(f, "Miscellaneous"),
        }
    }
}

/// Prioritized pattern for the work queue
#[derive(Debug, Clone)]
pub struct PrioritizedPattern {
    pub pattern: FailurePattern,
    /// Priority score: frequency × severity / complexity
    pub priority: f64,
}

impl PartialEq for PrioritizedPattern {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PrioritizedPattern {}

impl PartialOrd for PrioritizedPattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedPattern {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first (max heap)
        self.priority.partial_cmp(&other.priority)
            .unwrap_or(Ordering::Equal)
    }
}

/// Hunt Planner: Classifies and prioritizes compilation failures
///
/// Implements Heijunka by processing highest-impact patterns first.
#[derive(Debug)]
pub struct HuntPlanner {
    /// Clustered errors from analysis
    error_clusters: Vec<ErrorCluster>,
    /// Priority queue of patterns to fix
    priority_queue: BinaryHeap<PrioritizedPattern>,
    /// Patterns already processed
    processed: HashMap<String, bool>,
}

impl HuntPlanner {
    /// Create a new hunt planner
    pub fn new() -> Self {
        Self {
            error_clusters: Vec::new(),
            priority_queue: BinaryHeap::new(),
            processed: HashMap::new(),
        }
    }

    /// Add error clusters from corpus analysis
    pub fn add_clusters(&mut self, clusters: Vec<ErrorCluster>) {
        self.error_clusters.extend(clusters);
    }

    /// Analyze clusters and build priority queue
    ///
    /// Heijunka: Sort by frequency × severity / complexity for maximum impact
    pub fn build_priority_queue(&mut self) {
        for cluster in &self.error_clusters {
            let pattern = self.cluster_to_pattern(cluster);
            let priority = self.calculate_priority(&pattern);

            self.priority_queue.push(PrioritizedPattern { pattern, priority });
        }
    }

    /// Select the next highest-priority failure pattern
    ///
    /// Heijunka: Process highest-impact patterns first.
    /// Pareto principle: 20% of patterns cause 80% of failures.
    pub fn select_next_target(&mut self) -> Option<FailurePattern> {
        while let Some(prioritized) = self.priority_queue.pop() {
            let pattern_id = &prioritized.pattern.id;

            // Skip already processed patterns
            if self.processed.get(pattern_id).copied().unwrap_or(false) {
                continue;
            }

            self.processed.insert(pattern_id.clone(), true);
            return Some(prioritized.pattern);
        }
        None
    }

    /// Calculate priority score for a pattern
    ///
    /// Formula: (frequency × severity) / complexity
    /// Higher score = higher priority
    fn calculate_priority(&self, pattern: &FailurePattern) -> f64 {
        let frequency = pattern.affected_count as f64;
        let complexity = pattern.fix_complexity as f64;

        // Avoid division by zero
        let complexity = complexity.max(1.0);

        (frequency * 10.0) / complexity
    }

    /// Convert error cluster to failure pattern
    fn cluster_to_pattern(&self, cluster: &ErrorCluster) -> FailurePattern {
        let category = self.categorize_error(&cluster.error_code);

        FailurePattern {
            id: format!("pattern_{}", cluster.id),
            error_code: cluster.error_code.clone(),
            description: cluster.description.clone(),
            category,
            affected_count: cluster.frequency,
            fix_complexity: self.estimate_complexity(&cluster.error_code),
            trigger_example: cluster.examples.first()
                .cloned()
                .unwrap_or_default(),
        }
    }

    /// Categorize error code into pattern category
    fn categorize_error(&self, error_code: &str) -> PatternCategory {
        match error_code {
            // Type mismatch errors
            "E0308" | "E0277" | "E0282" => PatternCategory::TypeInference,
            // Unresolved import errors
            "E0432" | "E0433" => PatternCategory::ExternalDeps,
            // Borrowing errors
            "E0502" | "E0503" | "E0505" | "E0506" | "E0507" => PatternCategory::Borrowing,
            // Move errors (related to borrowing)
            "E0382" | "E0383" => PatternCategory::Borrowing,
            // Lifetime errors
            "E0106" | "E0621" | "E0623" => PatternCategory::Borrowing,
            // Control flow (not exhaustive match, etc.)
            "E0004" | "E0005" => PatternCategory::ControlFlow,
            // Default to miscellaneous
            _ => PatternCategory::Miscellaneous,
        }
    }

    /// Estimate fix complexity based on error code
    fn estimate_complexity(&self, error_code: &str) -> u8 {
        match error_code {
            // Easy: just add imports
            "E0432" | "E0433" => 2,
            // Medium: type coercion
            "E0308" => 4,
            // Medium-hard: trait bounds
            "E0277" => 5,
            // Hard: borrowing issues
            "E0502" | "E0503" | "E0505" | "E0506" | "E0507" => 7,
            // Very hard: lifetime issues
            "E0106" | "E0621" | "E0623" => 8,
            // Default medium complexity
            _ => 5,
        }
    }

    /// Get remaining patterns count
    pub fn remaining_count(&self) -> usize {
        self.priority_queue.len()
    }

    /// Get all clusters
    pub fn clusters(&self) -> &[ErrorCluster] {
        &self.error_clusters
    }
}

impl Default for HuntPlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cluster(id: &str, code: &str, freq: u32) -> ErrorCluster {
        ErrorCluster {
            id: id.to_string(),
            error_code: code.to_string(),
            description: format!("Test error {}", code),
            frequency: freq,
            severity: 5,
            examples: vec!["example".to_string()],
        }
    }

    #[test]
    fn test_planner_new() {
        let planner = HuntPlanner::new();
        assert_eq!(planner.remaining_count(), 0);
        assert!(planner.clusters().is_empty());
    }

    #[test]
    fn test_add_clusters() {
        let mut planner = HuntPlanner::new();
        planner.add_clusters(vec![
            create_test_cluster("1", "E0308", 10),
            create_test_cluster("2", "E0432", 20),
        ]);
        assert_eq!(planner.clusters().len(), 2);
    }

    #[test]
    fn test_build_priority_queue() {
        let mut planner = HuntPlanner::new();
        planner.add_clusters(vec![
            create_test_cluster("1", "E0308", 10),
            create_test_cluster("2", "E0432", 20),
        ]);
        planner.build_priority_queue();
        assert_eq!(planner.remaining_count(), 2);
    }

    #[test]
    fn test_select_next_target_priority_order() {
        let mut planner = HuntPlanner::new();
        planner.add_clusters(vec![
            create_test_cluster("low", "E0308", 5),   // Lower freq, higher complexity
            create_test_cluster("high", "E0432", 50), // Higher freq, lower complexity
        ]);
        planner.build_priority_queue();

        // Should select high-frequency, low-complexity first
        let first = planner.select_next_target().unwrap();
        assert_eq!(first.error_code, "E0432"); // External deps, easier to fix
    }

    #[test]
    fn test_select_next_target_no_duplicates() {
        let mut planner = HuntPlanner::new();
        planner.add_clusters(vec![create_test_cluster("1", "E0308", 10)]);
        planner.build_priority_queue();

        assert!(planner.select_next_target().is_some());
        assert!(planner.select_next_target().is_none()); // Already processed
    }

    #[test]
    fn test_categorize_error() {
        let planner = HuntPlanner::new();

        assert_eq!(planner.categorize_error("E0308"), PatternCategory::TypeInference);
        assert_eq!(planner.categorize_error("E0432"), PatternCategory::ExternalDeps);
        assert_eq!(planner.categorize_error("E0502"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0004"), PatternCategory::ControlFlow);
        assert_eq!(planner.categorize_error("E9999"), PatternCategory::Miscellaneous);
    }

    #[test]
    fn test_estimate_complexity() {
        let planner = HuntPlanner::new();

        // Import errors should be easy
        assert!(planner.estimate_complexity("E0432") < 5);
        // Lifetime errors should be hard
        assert!(planner.estimate_complexity("E0106") > 5);
    }

    #[test]
    fn test_pattern_category_display() {
        assert_eq!(format!("{}", PatternCategory::TypeInference), "Type Inference");
        assert_eq!(format!("{}", PatternCategory::ExternalDeps), "External Dependencies");
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_error_cluster_debug() {
        let cluster = create_test_cluster("1", "E0308", 10);
        let debug_str = format!("{:?}", cluster);
        assert!(debug_str.contains("ErrorCluster"));
        assert!(debug_str.contains("E0308"));
    }

    #[test]
    fn test_error_cluster_clone() {
        let cluster = create_test_cluster("orig", "E0432", 20);
        let cloned = cluster.clone();
        assert_eq!(cloned.id, "orig");
        assert_eq!(cloned.error_code, "E0432");
        assert_eq!(cloned.frequency, 20);
    }

    #[test]
    fn test_failure_pattern_debug() {
        let planner = HuntPlanner::new();
        let cluster = create_test_cluster("1", "E0308", 10);
        let pattern = planner.cluster_to_pattern(&cluster);

        let debug_str = format!("{:?}", pattern);
        assert!(debug_str.contains("FailurePattern"));
        assert!(debug_str.contains("E0308"));
    }

    #[test]
    fn test_failure_pattern_clone() {
        let planner = HuntPlanner::new();
        let cluster = create_test_cluster("1", "E0277", 15);
        let pattern = planner.cluster_to_pattern(&cluster);
        let cloned = pattern.clone();

        assert_eq!(cloned.error_code, "E0277");
        assert_eq!(cloned.affected_count, 15);
    }

    #[test]
    fn test_pattern_category_debug() {
        let cat = PatternCategory::TypeInference;
        let debug_str = format!("{:?}", cat);
        assert!(debug_str.contains("TypeInference"));
    }

    #[test]
    fn test_pattern_category_copy() {
        let cat = PatternCategory::Borrowing;
        let copied = cat;
        assert_eq!(copied, PatternCategory::Borrowing);
    }

    #[test]
    fn test_pattern_category_eq() {
        assert_eq!(PatternCategory::ControlFlow, PatternCategory::ControlFlow);
        assert_ne!(PatternCategory::ControlFlow, PatternCategory::Borrowing);
    }

    #[test]
    fn test_pattern_category_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(PatternCategory::TypeInference);
        set.insert(PatternCategory::TypeInference);
        set.insert(PatternCategory::ExternalDeps);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_pattern_category_display_all() {
        assert_eq!(format!("{}", PatternCategory::Borrowing), "Borrowing");
        assert_eq!(format!("{}", PatternCategory::ControlFlow), "Control Flow");
        assert_eq!(format!("{}", PatternCategory::Miscellaneous), "Miscellaneous");
    }

    #[test]
    fn test_prioritized_pattern_debug() {
        let planner = HuntPlanner::new();
        let cluster = create_test_cluster("1", "E0308", 10);
        let pattern = planner.cluster_to_pattern(&cluster);
        let prioritized = PrioritizedPattern { pattern, priority: 5.0 };

        let debug_str = format!("{:?}", prioritized);
        assert!(debug_str.contains("PrioritizedPattern"));
        assert!(debug_str.contains("priority"));
    }

    #[test]
    fn test_prioritized_pattern_clone() {
        let planner = HuntPlanner::new();
        let cluster = create_test_cluster("1", "E0432", 25);
        let pattern = planner.cluster_to_pattern(&cluster);
        let prioritized = PrioritizedPattern { pattern, priority: 12.5 };
        let cloned = prioritized.clone();

        assert_eq!(cloned.priority, 12.5);
        assert_eq!(cloned.pattern.error_code, "E0432");
    }

    #[test]
    fn test_prioritized_pattern_eq() {
        let planner = HuntPlanner::new();
        let cluster1 = create_test_cluster("1", "E0308", 10);
        let cluster2 = create_test_cluster("2", "E0432", 20);
        let pattern1 = planner.cluster_to_pattern(&cluster1);
        let pattern2 = planner.cluster_to_pattern(&cluster2);

        let p1 = PrioritizedPattern { pattern: pattern1, priority: 5.0 };
        let p2 = PrioritizedPattern { pattern: pattern2, priority: 5.0 };

        assert_eq!(p1, p2); // Same priority = equal
    }

    #[test]
    fn test_prioritized_pattern_ord() {
        let planner = HuntPlanner::new();
        let cluster = create_test_cluster("1", "E0308", 10);
        let pattern = planner.cluster_to_pattern(&cluster);

        let low = PrioritizedPattern { pattern: pattern.clone(), priority: 1.0 };
        let high = PrioritizedPattern { pattern, priority: 10.0 };

        assert!(high > low);
        assert!(low < high);
    }

    #[test]
    fn test_hunt_planner_default() {
        let planner: HuntPlanner = Default::default();
        assert!(planner.clusters().is_empty());
        assert_eq!(planner.remaining_count(), 0);
    }

    #[test]
    fn test_hunt_planner_debug() {
        let planner = HuntPlanner::new();
        let debug_str = format!("{:?}", planner);
        assert!(debug_str.contains("HuntPlanner"));
    }

    #[test]
    fn test_remaining_count_after_selection() {
        let mut planner = HuntPlanner::new();
        planner.add_clusters(vec![
            create_test_cluster("1", "E0308", 10),
            create_test_cluster("2", "E0432", 20),
        ]);
        planner.build_priority_queue();

        assert_eq!(planner.remaining_count(), 2);
        planner.select_next_target();
        assert_eq!(planner.remaining_count(), 1);
        planner.select_next_target();
        assert_eq!(planner.remaining_count(), 0);
    }

    #[test]
    fn test_calculate_priority() {
        let planner = HuntPlanner::new();
        let mut cluster = create_test_cluster("1", "E0308", 100);
        cluster.frequency = 100;
        let pattern = planner.cluster_to_pattern(&cluster);

        // Priority = (frequency * 10) / complexity
        // E0308 has complexity 4, so priority = (100 * 10) / 4 = 250
        let priority = planner.calculate_priority(&pattern);
        assert!(priority > 200.0);
    }

    #[test]
    fn test_calculate_priority_zero_complexity() {
        let planner = HuntPlanner::new();
        let pattern = FailurePattern {
            id: "test".to_string(),
            error_code: "E0000".to_string(),
            description: "test".to_string(),
            category: PatternCategory::Miscellaneous,
            affected_count: 10,
            fix_complexity: 0, // Zero complexity (edge case)
            trigger_example: String::new(),
        };

        // Should not panic, complexity clamped to 1.0
        let priority = planner.calculate_priority(&pattern);
        assert!(priority > 0.0);
    }

    #[test]
    fn test_categorize_all_error_codes() {
        let planner = HuntPlanner::new();

        // Type inference
        assert_eq!(planner.categorize_error("E0282"), PatternCategory::TypeInference);

        // External deps
        assert_eq!(planner.categorize_error("E0433"), PatternCategory::ExternalDeps);

        // Borrowing - various codes
        assert_eq!(planner.categorize_error("E0503"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0505"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0506"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0507"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0382"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0383"), PatternCategory::Borrowing);

        // Lifetime errors
        assert_eq!(planner.categorize_error("E0106"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0621"), PatternCategory::Borrowing);
        assert_eq!(planner.categorize_error("E0623"), PatternCategory::Borrowing);

        // Control flow
        assert_eq!(planner.categorize_error("E0005"), PatternCategory::ControlFlow);
    }

    #[test]
    fn test_estimate_complexity_all_codes() {
        let planner = HuntPlanner::new();

        // Easy imports
        assert!(planner.estimate_complexity("E0433") <= 3);

        // Medium type mismatch
        let e0308_complexity = planner.estimate_complexity("E0308");
        assert!(e0308_complexity >= 3 && e0308_complexity <= 5);

        // Hard borrowing
        assert!(planner.estimate_complexity("E0503") >= 6);
        assert!(planner.estimate_complexity("E0505") >= 6);

        // Very hard lifetime
        assert!(planner.estimate_complexity("E0621") >= 7);
        assert!(planner.estimate_complexity("E0623") >= 7);
    }

    #[test]
    fn test_cluster_to_pattern_with_examples() {
        let planner = HuntPlanner::new();
        let mut cluster = create_test_cluster("1", "E0308", 10);
        cluster.examples = vec!["first example".to_string(), "second".to_string()];

        let pattern = planner.cluster_to_pattern(&cluster);
        assert_eq!(pattern.trigger_example, "first example");
    }

    #[test]
    fn test_cluster_to_pattern_no_examples() {
        let planner = HuntPlanner::new();
        let mut cluster = create_test_cluster("1", "E0308", 10);
        cluster.examples.clear();

        let pattern = planner.cluster_to_pattern(&cluster);
        assert!(pattern.trigger_example.is_empty());
    }

    #[test]
    fn test_select_empty_queue() {
        let mut planner = HuntPlanner::new();
        assert!(planner.select_next_target().is_none());
    }
}
