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
