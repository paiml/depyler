//! Curriculum Scheduler (DEPYLER-0925)
//!
//! Implements curriculum learning for optimal error processing order.
//! Processes errors EASY→MEDIUM→HARD→EXPERT for fastest convergence.
//!
//! ## Algorithm
//!
//! Priority calculation:
//! - Base priority from difficulty level (Easy=100, Medium=50, Hard=25, Expert=10)
//! - Cluster bonus: +20 if example belongs to a cluster (fixes multiple examples)
//! - Dependency penalty: -5 per unmet dependency
//!
//! Reference: Bengio et al. (2009) - Curriculum Learning

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Compilation error from rustc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationError {
    pub code: String,
    pub message: String,
}

/// Difficulty level for a failing example
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Easy = 1,
    Medium = 2,
    Hard = 3,
    Expert = 4,
}

/// A failing example to be processed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailingExample {
    pub path: String,
    pub errors: Vec<CompilationError>,
    pub difficulty: DifficultyLevel,
    pub cluster_id: Option<u32>,
    pub dependencies: Vec<String>,
}

/// Internal wrapper for priority queue
#[derive(Debug, Clone)]
struct PrioritizedExample {
    example: FailingExample,
    priority: i32,
}

impl Eq for PrioritizedExample {}

impl PartialEq for PrioritizedExample {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Ord for PrioritizedExample {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority = processed first
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PrioritizedExample {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Curriculum scheduler for processing errors in optimal order
pub struct CurriculumScheduler {
    queue: BinaryHeap<PrioritizedExample>,
    graduated: Vec<String>,
    total_added: usize,
}

impl CurriculumScheduler {
    /// Create a new curriculum scheduler
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            graduated: Vec::new(),
            total_added: 0,
        }
    }

    /// Add an example to the queue
    pub fn add_example(&mut self, example: FailingExample) {
        let priority = Self::calculate_priority(&example);
        self.queue.push(PrioritizedExample { example, priority });
        self.total_added += 1;
    }

    /// Calculate priority for an example
    fn calculate_priority(example: &FailingExample) -> i32 {
        // Base priority from difficulty
        let base = match example.difficulty {
            DifficultyLevel::Easy => 100,
            DifficultyLevel::Medium => 50,
            DifficultyLevel::Hard => 25,
            DifficultyLevel::Expert => 10,
        };

        // Cluster bonus: +20 for clustered examples
        let cluster_bonus = if example.cluster_id.is_some() { 20 } else { 0 };

        // Dependency penalty: -5 per dependency
        let dependency_penalty = example.dependencies.len() as i32 * 5;

        base + cluster_bonus - dependency_penalty
    }

    /// Get next example to process (pops from priority queue)
    pub fn pop_next(&mut self) -> Option<FailingExample> {
        self.queue.pop().map(|p| p.example)
    }

    /// Mark an example as graduated (successfully compiled)
    pub fn graduate(&mut self, path: String) {
        self.graduated.push(path);
    }

    /// Get current progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        let total = self.queue.len() + self.graduated.len();
        if total == 0 {
            return 0.0;
        }
        self.graduated.len() as f32 / total as f32
    }

    /// Number of examples remaining
    pub fn remaining(&self) -> usize {
        self.queue.len()
    }

    /// Number of graduated examples
    pub fn graduated_count(&self) -> usize {
        self.graduated.len()
    }

    /// Check if scheduler is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for CurriculumScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === CompilationError tests ===

    #[test]
    fn test_compilation_error_new() {
        let err = CompilationError {
            code: "E0277".to_string(),
            message: "trait bound not satisfied".to_string(),
        };
        assert_eq!(err.code, "E0277");
        assert_eq!(err.message, "trait bound not satisfied");
    }

    #[test]
    fn test_compilation_error_clone() {
        let err = CompilationError {
            code: "E0308".to_string(),
            message: "mismatched types".to_string(),
        };
        let cloned = err.clone();
        assert_eq!(cloned.code, err.code);
        assert_eq!(cloned.message, err.message);
    }

    #[test]
    fn test_compilation_error_debug() {
        let err = CompilationError {
            code: "E0599".to_string(),
            message: "no method named".to_string(),
        };
        let debug = format!("{:?}", err);
        assert!(debug.contains("E0599"));
        assert!(debug.contains("no method named"));
    }

    #[test]
    fn test_compilation_error_serialize() {
        let err = CompilationError {
            code: "E0425".to_string(),
            message: "cannot find value".to_string(),
        };
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("E0425"));
        let deserialized: CompilationError = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.code, err.code);
    }

    // === DifficultyLevel tests ===

    #[test]
    fn test_difficulty_level_easy() {
        assert_eq!(DifficultyLevel::Easy as i32, 1);
    }

    #[test]
    fn test_difficulty_level_medium() {
        assert_eq!(DifficultyLevel::Medium as i32, 2);
    }

    #[test]
    fn test_difficulty_level_hard() {
        assert_eq!(DifficultyLevel::Hard as i32, 3);
    }

    #[test]
    fn test_difficulty_level_expert() {
        assert_eq!(DifficultyLevel::Expert as i32, 4);
    }

    #[test]
    fn test_difficulty_level_ordering() {
        assert!(DifficultyLevel::Easy < DifficultyLevel::Medium);
        assert!(DifficultyLevel::Medium < DifficultyLevel::Hard);
        assert!(DifficultyLevel::Hard < DifficultyLevel::Expert);
    }

    #[test]
    fn test_difficulty_level_clone() {
        let level = DifficultyLevel::Hard;
        assert_eq!(level.clone(), level);
    }

    #[test]
    fn test_difficulty_level_serialize() {
        let level = DifficultyLevel::Expert;
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: DifficultyLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, level);
    }

    // === FailingExample tests ===

    fn make_example(
        path: &str,
        difficulty: DifficultyLevel,
        cluster: Option<u32>,
        deps: Vec<&str>,
    ) -> FailingExample {
        FailingExample {
            path: path.to_string(),
            errors: vec![CompilationError {
                code: "E0001".to_string(),
                message: "error".to_string(),
            }],
            difficulty,
            cluster_id: cluster,
            dependencies: deps.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_failing_example_new() {
        let ex = make_example("test.py", DifficultyLevel::Easy, None, vec![]);
        assert_eq!(ex.path, "test.py");
        assert_eq!(ex.difficulty, DifficultyLevel::Easy);
        assert!(ex.cluster_id.is_none());
        assert!(ex.dependencies.is_empty());
    }

    #[test]
    fn test_failing_example_with_cluster() {
        let ex = make_example("test.py", DifficultyLevel::Medium, Some(42), vec![]);
        assert_eq!(ex.cluster_id, Some(42));
    }

    #[test]
    fn test_failing_example_with_dependencies() {
        let ex = make_example("test.py", DifficultyLevel::Hard, None, vec!["dep1", "dep2"]);
        assert_eq!(ex.dependencies.len(), 2);
        assert!(ex.dependencies.contains(&"dep1".to_string()));
        assert!(ex.dependencies.contains(&"dep2".to_string()));
    }

    #[test]
    fn test_failing_example_clone() {
        let ex = make_example("test.py", DifficultyLevel::Expert, Some(1), vec!["a"]);
        let cloned = ex.clone();
        assert_eq!(cloned.path, ex.path);
        assert_eq!(cloned.difficulty, ex.difficulty);
        assert_eq!(cloned.cluster_id, ex.cluster_id);
    }

    #[test]
    fn test_failing_example_serialize() {
        let ex = make_example("test.py", DifficultyLevel::Easy, None, vec![]);
        let json = serde_json::to_string(&ex).unwrap();
        assert!(json.contains("test.py"));
        let deserialized: FailingExample = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.path, ex.path);
    }

    // === PrioritizedExample tests ===

    #[test]
    fn test_prioritized_example_eq() {
        let ex1 = PrioritizedExample {
            example: make_example("a.py", DifficultyLevel::Easy, None, vec![]),
            priority: 100,
        };
        let ex2 = PrioritizedExample {
            example: make_example("b.py", DifficultyLevel::Hard, None, vec![]),
            priority: 100,
        };
        assert_eq!(ex1, ex2); // Equal by priority, not content
    }

    #[test]
    fn test_prioritized_example_ord() {
        let low = PrioritizedExample {
            example: make_example("a.py", DifficultyLevel::Expert, None, vec![]),
            priority: 10,
        };
        let high = PrioritizedExample {
            example: make_example("b.py", DifficultyLevel::Easy, None, vec![]),
            priority: 100,
        };
        assert!(high > low);
        assert!(low < high);
    }

    #[test]
    fn test_prioritized_example_partial_ord() {
        let ex1 = PrioritizedExample {
            example: make_example("a.py", DifficultyLevel::Easy, None, vec![]),
            priority: 50,
        };
        let ex2 = PrioritizedExample {
            example: make_example("b.py", DifficultyLevel::Easy, None, vec![]),
            priority: 75,
        };
        assert!(ex1.partial_cmp(&ex2) == Some(Ordering::Less));
    }

    // === CurriculumScheduler tests ===

    #[test]
    fn test_scheduler_new() {
        let scheduler = CurriculumScheduler::new();
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.remaining(), 0);
        assert_eq!(scheduler.graduated_count(), 0);
    }

    #[test]
    fn test_scheduler_default() {
        let scheduler = CurriculumScheduler::default();
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_scheduler_add_example() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.add_example(make_example("test.py", DifficultyLevel::Easy, None, vec![]));
        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.remaining(), 1);
    }

    #[test]
    fn test_scheduler_pop_next_empty() {
        let mut scheduler = CurriculumScheduler::new();
        assert!(scheduler.pop_next().is_none());
    }

    #[test]
    fn test_scheduler_pop_next() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.add_example(make_example("test.py", DifficultyLevel::Easy, None, vec![]));
        let ex = scheduler.pop_next();
        assert!(ex.is_some());
        assert_eq!(ex.unwrap().path, "test.py");
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_scheduler_priority_order() {
        let mut scheduler = CurriculumScheduler::new();
        // Add in wrong order
        scheduler.add_example(make_example("hard.py", DifficultyLevel::Hard, None, vec![]));
        scheduler.add_example(make_example("easy.py", DifficultyLevel::Easy, None, vec![]));
        scheduler.add_example(make_example(
            "medium.py",
            DifficultyLevel::Medium,
            None,
            vec![],
        ));

        // Should pop in priority order: Easy > Medium > Hard
        assert_eq!(scheduler.pop_next().unwrap().path, "easy.py");
        assert_eq!(scheduler.pop_next().unwrap().path, "medium.py");
        assert_eq!(scheduler.pop_next().unwrap().path, "hard.py");
    }

    #[test]
    fn test_scheduler_graduate() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.graduate("test.py".to_string());
        assert_eq!(scheduler.graduated_count(), 1);
    }

    #[test]
    fn test_scheduler_progress_empty() {
        let scheduler = CurriculumScheduler::new();
        assert_eq!(scheduler.progress(), 0.0);
    }

    #[test]
    fn test_scheduler_progress_half() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.add_example(make_example("a.py", DifficultyLevel::Easy, None, vec![]));
        scheduler.add_example(make_example("b.py", DifficultyLevel::Easy, None, vec![]));
        scheduler.graduate("c.py".to_string());
        scheduler.graduate("d.py".to_string());
        // 2 graduated, 2 remaining = 50%
        assert!((scheduler.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_scheduler_progress_complete() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.graduate("a.py".to_string());
        scheduler.graduate("b.py".to_string());
        // All graduated = 100%
        assert_eq!(scheduler.progress(), 1.0);
    }

    // === Priority calculation tests ===

    #[test]
    fn test_priority_easy_base() {
        let ex = make_example("test.py", DifficultyLevel::Easy, None, vec![]);
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 100);
    }

    #[test]
    fn test_priority_medium_base() {
        let ex = make_example("test.py", DifficultyLevel::Medium, None, vec![]);
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 50);
    }

    #[test]
    fn test_priority_hard_base() {
        let ex = make_example("test.py", DifficultyLevel::Hard, None, vec![]);
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 25);
    }

    #[test]
    fn test_priority_expert_base() {
        let ex = make_example("test.py", DifficultyLevel::Expert, None, vec![]);
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 10);
    }

    #[test]
    fn test_priority_cluster_bonus() {
        let ex = make_example("test.py", DifficultyLevel::Easy, Some(1), vec![]);
        // 100 (base) + 20 (cluster) = 120
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 120);
    }

    #[test]
    fn test_priority_dependency_penalty() {
        let ex = make_example("test.py", DifficultyLevel::Easy, None, vec!["a", "b", "c"]);
        // 100 (base) - 15 (3 deps * 5) = 85
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 85);
    }

    #[test]
    fn test_priority_combined() {
        let ex = make_example("test.py", DifficultyLevel::Medium, Some(5), vec!["x", "y"]);
        // 50 (base) + 20 (cluster) - 10 (2 deps * 5) = 60
        assert_eq!(CurriculumScheduler::calculate_priority(&ex), 60);
    }

    // === Integration tests ===

    #[test]
    fn test_full_workflow() {
        let mut scheduler = CurriculumScheduler::new();

        // Add various examples
        scheduler.add_example(make_example(
            "hard.py",
            DifficultyLevel::Hard,
            None,
            vec!["dep"],
        ));
        scheduler.add_example(make_example(
            "easy_cluster.py",
            DifficultyLevel::Easy,
            Some(1),
            vec![],
        ));
        scheduler.add_example(make_example("easy.py", DifficultyLevel::Easy, None, vec![]));

        // Process in priority order
        // easy_cluster: 100 + 20 = 120
        // easy: 100
        // hard: 25 - 5 = 20
        let first = scheduler.pop_next().unwrap();
        assert_eq!(first.path, "easy_cluster.py");
        scheduler.graduate(first.path);

        let second = scheduler.pop_next().unwrap();
        assert_eq!(second.path, "easy.py");
        scheduler.graduate(second.path);

        let third = scheduler.pop_next().unwrap();
        assert_eq!(third.path, "hard.py");
        scheduler.graduate(third.path);

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.graduated_count(), 3);
        assert_eq!(scheduler.progress(), 1.0);
    }

    #[test]
    fn test_total_added_tracking() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.add_example(make_example("a.py", DifficultyLevel::Easy, None, vec![]));
        scheduler.add_example(make_example("b.py", DifficultyLevel::Easy, None, vec![]));
        assert_eq!(scheduler.total_added, 2);
        scheduler.pop_next();
        assert_eq!(scheduler.total_added, 2); // total_added doesn't decrease
    }
}
