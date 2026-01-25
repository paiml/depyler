//! Curriculum Learning for Error Processing (Strategy #3 - DEPYLER-0633)
//!
//! Applies progressive difficulty ordering to fix easy errors first, building momentum.
//! Based on the StepCoder paper's curriculum learning approach.
//!
//! # Difficulty Levels
//!
//! | Level | Score | Error Categories | Fix Approach |
//! |-------|-------|------------------|--------------|
//! | EASY | 0.25 | SyntaxError, MissingImport | Rule-based |
//! | MEDIUM | 0.50 | TypeMismatch, MethodNotFound | Oracle lookup |
//! | HARD | 0.75 | TraitBound, Ownership | Oracle + LLM |
//! | EXPERT | 1.00 | Lifetime, Async, Complex Borrow | Human review |
//!
//! # References
//!
//! - DEPYLER-0633: Strategy #3 implementation
//! - docs/specifications/single-shot-80-percentage-review.md Section 10.4

use crate::classifier::ErrorCategory;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Difficulty level for an error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DifficultyLevel {
    /// Easy errors: syntax, missing imports (0.25)
    Easy,
    /// Medium errors: type mismatch, method not found (0.50)
    Medium,
    /// Hard errors: trait bounds, ownership (0.75)
    Hard,
    /// Expert errors: lifetimes, async, complex borrow (1.00)
    Expert,
}

impl DifficultyLevel {
    /// Get the difficulty score (0.0-1.0)
    #[must_use]
    pub fn score(&self) -> f32 {
        match self {
            Self::Easy => 0.25,
            Self::Medium => 0.50,
            Self::Hard => 0.75,
            Self::Expert => 1.00,
        }
    }

    /// Get display name
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Easy => "EASY",
            Self::Medium => "MEDIUM",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    /// All difficulty levels in order
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[Self::Easy, Self::Medium, Self::Hard, Self::Expert]
    }

    /// Get processing priority (lower = process first)
    #[must_use]
    pub fn priority(&self) -> u8 {
        match self {
            Self::Easy => 0,
            Self::Medium => 1,
            Self::Hard => 2,
            Self::Expert => 3,
        }
    }
}

impl PartialOrd for DifficultyLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DifficultyLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl std::fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// An error entry in the curriculum
#[derive(Debug, Clone)]
pub struct CurriculumEntry {
    /// Unique identifier
    pub id: String,
    /// File path
    pub file: String,
    /// Error code (e.g., "E0308")
    pub error_code: String,
    /// Error message
    pub error_message: String,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    /// Estimated fix complexity (for ordering within a level)
    pub complexity: f32,
}

impl CurriculumEntry {
    /// Create a new curriculum entry
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        file: impl Into<String>,
        error_code: impl Into<String>,
        error_message: impl Into<String>,
        difficulty: DifficultyLevel,
    ) -> Self {
        Self {
            id: id.into(),
            file: file.into(),
            error_code: error_code.into(),
            error_message: error_message.into(),
            difficulty,
            complexity: difficulty.score(),
        }
    }

    /// Set custom complexity
    #[must_use]
    pub fn with_complexity(mut self, complexity: f32) -> Self {
        self.complexity = complexity;
        self
    }
}

/// Wrapper for priority queue ordering (min-heap by difficulty)
#[derive(Debug)]
struct PrioritizedEntry {
    entry: CurriculumEntry,
}

impl PartialEq for PrioritizedEntry {
    fn eq(&self, other: &Self) -> bool {
        self.entry.difficulty == other.entry.difficulty
            && (self.entry.complexity - other.entry.complexity).abs() < f32::EPSILON
    }
}

impl Eq for PrioritizedEntry {}

impl PartialOrd for PrioritizedEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap behavior (lower difficulty first)
        match other.entry.difficulty.cmp(&self.entry.difficulty) {
            Ordering::Equal => {
                // Within same difficulty, lower complexity first
                other
                    .entry
                    .complexity
                    .partial_cmp(&self.entry.complexity)
                    .unwrap_or(Ordering::Equal)
            }
            ord => ord,
        }
    }
}

/// Statistics for curriculum processing
#[derive(Debug, Clone, Default)]
pub struct CurriculumStats {
    /// Total entries added
    pub total_added: usize,
    /// Entries processed
    pub processed: usize,
    /// Successful fixes by difficulty
    pub successes: HashMap<DifficultyLevel, usize>,
    /// Failed fixes by difficulty
    pub failures: HashMap<DifficultyLevel, usize>,
    /// Current processing level
    pub current_level: Option<DifficultyLevel>,
}

impl CurriculumStats {
    /// Get success rate for a difficulty level
    #[must_use]
    pub fn success_rate(&self, level: DifficultyLevel) -> f64 {
        let successes = *self.successes.get(&level).unwrap_or(&0);
        let failures = *self.failures.get(&level).unwrap_or(&0);
        let total = successes + failures;

        if total == 0 {
            0.0
        } else {
            successes as f64 / total as f64
        }
    }

    /// Get overall success rate
    #[must_use]
    pub fn overall_success_rate(&self) -> f64 {
        let total_successes: usize = self.successes.values().sum();
        let total_failures: usize = self.failures.values().sum();
        let total = total_successes + total_failures;

        if total == 0 {
            0.0
        } else {
            total_successes as f64 / total as f64
        }
    }

    /// Get processing progress (0.0-1.0)
    #[must_use]
    pub fn progress(&self) -> f64 {
        if self.total_added == 0 {
            0.0
        } else {
            self.processed as f64 / self.total_added as f64
        }
    }
}

/// Curriculum Scheduler for error processing
///
/// Schedules errors for processing in difficulty order: EASY → MEDIUM → HARD → EXPERT
pub struct CurriculumScheduler {
    /// Priority queue of entries
    queue: BinaryHeap<PrioritizedEntry>,
    /// Statistics
    stats: CurriculumStats,
    /// Whether to skip to harder levels if current level has high success rate
    adaptive: bool,
    /// Minimum success rate to advance (when adaptive)
    advance_threshold: f64,
}

impl CurriculumScheduler {
    /// Create a new curriculum scheduler
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            stats: CurriculumStats::default(),
            adaptive: true,
            advance_threshold: 0.8,
        }
    }

    /// Create with custom settings
    #[must_use]
    pub fn with_adaptive(mut self, adaptive: bool) -> Self {
        self.adaptive = adaptive;
        self
    }

    /// Set advance threshold
    #[must_use]
    pub fn with_advance_threshold(mut self, threshold: f64) -> Self {
        self.advance_threshold = threshold;
        self
    }

    /// Add an error to the curriculum
    pub fn add(&mut self, entry: CurriculumEntry) {
        self.stats.total_added += 1;
        self.queue.push(PrioritizedEntry { entry });
    }

    /// Add an error with automatic difficulty classification
    pub fn add_classified(
        &mut self,
        id: impl Into<String>,
        file: impl Into<String>,
        error_code: impl Into<String>,
        error_message: impl Into<String>,
    ) {
        let error_code_str = error_code.into();
        let error_message_str = error_message.into();
        let difficulty = classify_error_difficulty(&error_code_str, &error_message_str);

        let entry = CurriculumEntry::new(id, file, error_code_str, error_message_str, difficulty);

        self.add(entry);
    }

    /// Get the next entry to process
    pub fn pop_next(&mut self) -> Option<CurriculumEntry> {
        let entry = self.queue.pop().map(|p| p.entry)?;
        self.stats.current_level = Some(entry.difficulty);
        Some(entry)
    }

    /// Peek at the next entry without removing it
    #[must_use]
    pub fn peek(&self) -> Option<&CurriculumEntry> {
        self.queue.peek().map(|p| &p.entry)
    }

    /// Record a successful fix
    pub fn record_success(&mut self, difficulty: DifficultyLevel) {
        self.stats.processed += 1;
        *self.stats.successes.entry(difficulty).or_insert(0) += 1;
    }

    /// Record a failed fix
    pub fn record_failure(&mut self, difficulty: DifficultyLevel) {
        self.stats.processed += 1;
        *self.stats.failures.entry(difficulty).or_insert(0) += 1;
    }

    /// Check if the curriculum is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get the number of remaining entries
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> &CurriculumStats {
        &self.stats
    }

    /// Get count by difficulty level
    #[must_use]
    pub fn count_by_level(&self) -> HashMap<DifficultyLevel, usize> {
        let mut counts = HashMap::new();
        for entry in self.queue.iter() {
            *counts.entry(entry.entry.difficulty).or_insert(0) += 1;
        }
        counts
    }

    /// Generate a summary report
    #[must_use]
    pub fn summary(&self) -> String {
        let mut report = String::new();

        report.push_str("# Curriculum Summary\n\n");
        report.push_str(&format!(
            "Total: {} | Processed: {} | Remaining: {}\n\n",
            self.stats.total_added,
            self.stats.processed,
            self.queue.len()
        ));

        report.push_str("## By Difficulty\n\n");
        report.push_str("| Level | Count | Success | Failure | Rate |\n");
        report.push_str("|-------|-------|---------|---------|------|\n");

        let counts = self.count_by_level();
        for level in DifficultyLevel::all() {
            let count = counts.get(level).unwrap_or(&0);
            let successes = *self.stats.successes.get(level).unwrap_or(&0);
            let failures = *self.stats.failures.get(level).unwrap_or(&0);
            let rate = self.stats.success_rate(*level);

            report.push_str(&format!(
                "| {} | {} | {} | {} | {:.1}% |\n",
                level,
                count,
                successes,
                failures,
                rate * 100.0
            ));
        }

        report.push_str(&format!(
            "\nOverall success rate: {:.1}%\n",
            self.stats.overall_success_rate() * 100.0
        ));

        report
    }
}

impl Default for CurriculumScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CurriculumScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CurriculumScheduler")
            .field("pending", &self.queue.len())
            .field("processed", &self.stats.processed)
            .finish()
    }
}

/// Classify an error's difficulty based on error code and message
#[must_use]
pub fn classify_error_difficulty(error_code: &str, error_message: &str) -> DifficultyLevel {
    let msg_lower = error_message.to_lowercase();

    // EASY: Syntax errors, missing imports
    match error_code {
        "E0433" | "E0412" | "E0405" | "E0425" => return DifficultyLevel::Easy, // Import/resolution
        "E0601" | "E0602" => return DifficultyLevel::Easy,                     // Main function
        _ => {}
    }

    if msg_lower.contains("cannot find") && msg_lower.contains("in this scope") {
        return DifficultyLevel::Easy;
    }
    if msg_lower.contains("unresolved import") {
        return DifficultyLevel::Easy;
    }

    // MEDIUM: Type mismatch, method not found
    match error_code {
        "E0308" => return DifficultyLevel::Medium, // Type mismatch
        "E0599" => return DifficultyLevel::Medium, // Method not found
        "E0609" => return DifficultyLevel::Medium, // Field not found
        "E0061" | "E0060" => return DifficultyLevel::Medium, // Wrong number of args
        _ => {}
    }

    if msg_lower.contains("mismatched types") {
        return DifficultyLevel::Medium;
    }
    if msg_lower.contains("method not found") {
        return DifficultyLevel::Medium;
    }

    // HARD: Trait bounds, ownership
    match error_code {
        "E0277" => return DifficultyLevel::Hard, // Trait bound not satisfied
        "E0382" | "E0505" | "E0507" => return DifficultyLevel::Hard, // Borrow checker
        "E0502" | "E0499" => return DifficultyLevel::Hard, // Mutable borrow
        _ => {}
    }

    if msg_lower.contains("trait") && msg_lower.contains("not implemented") {
        return DifficultyLevel::Hard;
    }
    if msg_lower.contains("moved") || msg_lower.contains("borrow") {
        return DifficultyLevel::Hard;
    }

    // EXPERT: Lifetimes, async, complex patterns
    match error_code {
        "E0106" | "E0495" | "E0621" => return DifficultyLevel::Expert, // Lifetime
        "E0597" | "E0716" => return DifficultyLevel::Expert,           // Borrowed value
        "E0728" | "E0746" => return DifficultyLevel::Expert,           // Async
        _ => {}
    }

    if msg_lower.contains("lifetime") || msg_lower.contains("'a") {
        return DifficultyLevel::Expert;
    }
    if msg_lower.contains("async") || msg_lower.contains("future") {
        return DifficultyLevel::Expert;
    }

    // Default to MEDIUM for unknown errors
    DifficultyLevel::Medium
}

/// Classify using ErrorCategory from the classifier module
#[must_use]
pub fn classify_from_category(category: ErrorCategory) -> DifficultyLevel {
    match category {
        ErrorCategory::SyntaxError => DifficultyLevel::Easy,
        ErrorCategory::MissingImport => DifficultyLevel::Easy,
        ErrorCategory::TypeMismatch => DifficultyLevel::Medium,
        ErrorCategory::BorrowChecker => DifficultyLevel::Hard,
        ErrorCategory::LifetimeError => DifficultyLevel::Expert,
        ErrorCategory::TraitBound => DifficultyLevel::Hard,
        ErrorCategory::Other => DifficultyLevel::Medium,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_level_score() {
        assert!((DifficultyLevel::Easy.score() - 0.25).abs() < 0.001);
        assert!((DifficultyLevel::Medium.score() - 0.50).abs() < 0.001);
        assert!((DifficultyLevel::Hard.score() - 0.75).abs() < 0.001);
        assert!((DifficultyLevel::Expert.score() - 1.00).abs() < 0.001);
    }

    #[test]
    fn test_difficulty_level_ordering() {
        assert!(DifficultyLevel::Easy < DifficultyLevel::Medium);
        assert!(DifficultyLevel::Medium < DifficultyLevel::Hard);
        assert!(DifficultyLevel::Hard < DifficultyLevel::Expert);
    }

    #[test]
    fn test_curriculum_entry_creation() {
        let entry = CurriculumEntry::new(
            "1",
            "test.rs",
            "E0308",
            "type mismatch",
            DifficultyLevel::Medium,
        );

        assert_eq!(entry.error_code, "E0308");
        assert_eq!(entry.difficulty, DifficultyLevel::Medium);
    }

    #[test]
    fn test_curriculum_scheduler_ordering() {
        let mut scheduler = CurriculumScheduler::new();

        // Add in random order
        scheduler.add(CurriculumEntry::new(
            "1",
            "test.rs",
            "E0277",
            "trait bound",
            DifficultyLevel::Hard,
        ));
        scheduler.add(CurriculumEntry::new(
            "2",
            "test.rs",
            "E0433",
            "unresolved",
            DifficultyLevel::Easy,
        ));
        scheduler.add(CurriculumEntry::new(
            "3",
            "test.rs",
            "E0106",
            "lifetime",
            DifficultyLevel::Expert,
        ));
        scheduler.add(CurriculumEntry::new(
            "4",
            "test.rs",
            "E0308",
            "mismatch",
            DifficultyLevel::Medium,
        ));

        // Should come out in EASY, MEDIUM, HARD, EXPERT order
        assert_eq!(
            scheduler.pop_next().unwrap().difficulty,
            DifficultyLevel::Easy
        );
        assert_eq!(
            scheduler.pop_next().unwrap().difficulty,
            DifficultyLevel::Medium
        );
        assert_eq!(
            scheduler.pop_next().unwrap().difficulty,
            DifficultyLevel::Hard
        );
        assert_eq!(
            scheduler.pop_next().unwrap().difficulty,
            DifficultyLevel::Expert
        );
    }

    #[test]
    fn test_curriculum_scheduler_stats() {
        let mut scheduler = CurriculumScheduler::new();

        scheduler.add(CurriculumEntry::new(
            "1",
            "test.rs",
            "E0433",
            "easy",
            DifficultyLevel::Easy,
        ));
        scheduler.add(CurriculumEntry::new(
            "2",
            "test.rs",
            "E0308",
            "medium",
            DifficultyLevel::Medium,
        ));

        assert_eq!(scheduler.stats().total_added, 2);
        assert_eq!(scheduler.len(), 2);

        scheduler.pop_next();
        scheduler.record_success(DifficultyLevel::Easy);

        assert_eq!(scheduler.stats().processed, 1);
        assert!((scheduler.stats().success_rate(DifficultyLevel::Easy) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_classify_error_difficulty_easy() {
        assert_eq!(
            classify_error_difficulty("E0433", "unresolved import"),
            DifficultyLevel::Easy
        );
        assert_eq!(
            classify_error_difficulty("E0425", "cannot find value"),
            DifficultyLevel::Easy
        );
    }

    #[test]
    fn test_classify_error_difficulty_medium() {
        assert_eq!(
            classify_error_difficulty("E0308", "mismatched types"),
            DifficultyLevel::Medium
        );
        assert_eq!(
            classify_error_difficulty("E0599", "method not found"),
            DifficultyLevel::Medium
        );
    }

    #[test]
    fn test_classify_error_difficulty_hard() {
        assert_eq!(
            classify_error_difficulty("E0277", "trait not implemented"),
            DifficultyLevel::Hard
        );
        assert_eq!(
            classify_error_difficulty("E0382", "use of moved value"),
            DifficultyLevel::Hard
        );
    }

    #[test]
    fn test_classify_error_difficulty_expert() {
        assert_eq!(
            classify_error_difficulty("E0106", "missing lifetime"),
            DifficultyLevel::Expert
        );
        assert_eq!(
            classify_error_difficulty("E0728", "async fn"),
            DifficultyLevel::Expert
        );
    }

    #[test]
    fn test_classify_from_category() {
        assert_eq!(
            classify_from_category(ErrorCategory::SyntaxError),
            DifficultyLevel::Easy
        );
        assert_eq!(
            classify_from_category(ErrorCategory::TypeMismatch),
            DifficultyLevel::Medium
        );
        assert_eq!(
            classify_from_category(ErrorCategory::BorrowChecker),
            DifficultyLevel::Hard
        );
        assert_eq!(
            classify_from_category(ErrorCategory::LifetimeError),
            DifficultyLevel::Expert
        );
    }

    #[test]
    fn test_curriculum_stats_overall_success_rate() {
        let mut scheduler = CurriculumScheduler::new();

        scheduler.record_success(DifficultyLevel::Easy);
        scheduler.record_success(DifficultyLevel::Easy);
        scheduler.record_failure(DifficultyLevel::Easy);
        scheduler.record_success(DifficultyLevel::Medium);

        // 3 successes, 1 failure = 75%
        assert!((scheduler.stats().overall_success_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_curriculum_scheduler_add_classified() {
        let mut scheduler = CurriculumScheduler::new();

        scheduler.add_classified("1", "test.rs", "E0433", "unresolved import");
        scheduler.add_classified("2", "test.rs", "E0106", "lifetime annotation");

        let first = scheduler.pop_next().unwrap();
        assert_eq!(first.difficulty, DifficultyLevel::Easy);

        let second = scheduler.pop_next().unwrap();
        assert_eq!(second.difficulty, DifficultyLevel::Expert);
    }

    #[test]
    fn test_curriculum_summary() {
        let mut scheduler = CurriculumScheduler::new();
        scheduler.add(CurriculumEntry::new(
            "1",
            "test.rs",
            "E0433",
            "easy",
            DifficultyLevel::Easy,
        ));

        let summary = scheduler.summary();
        assert!(summary.contains("Curriculum Summary"));
        assert!(summary.contains("EASY"));
    }
}
