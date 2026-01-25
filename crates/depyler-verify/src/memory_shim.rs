//! Memory Safety Shim - pure logic separated from I/O
//!
//! Extracts testable logic from memory_safety.rs

use std::collections::{HashMap, HashSet};

/// Memory allocation tracker
#[derive(Debug, Clone, Default)]
pub struct AllocationTracker {
    allocated: HashSet<String>,
    freed: HashSet<String>,
    stack_allocations: HashMap<String, usize>,
}

impl AllocationTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self, name: &str) {
        self.allocated.insert(name.to_string());
        self.freed.remove(name);
    }

    pub fn free(&mut self, name: &str) {
        if self.allocated.contains(name) {
            self.allocated.remove(name);
            self.freed.insert(name.to_string());
        }
    }

    pub fn is_allocated(&self, name: &str) -> bool {
        self.allocated.contains(name)
    }

    pub fn is_freed(&self, name: &str) -> bool {
        self.freed.contains(name)
    }

    pub fn is_use_after_free(&self, name: &str) -> bool {
        self.freed.contains(name)
    }

    pub fn push_stack_frame(&mut self, name: &str, depth: usize) {
        self.stack_allocations.insert(name.to_string(), depth);
        self.allocate(name);
    }

    pub fn pop_stack_frame(&mut self, depth: usize) -> Vec<String> {
        let popped: Vec<String> = self
            .stack_allocations
            .iter()
            .filter(|(_, &d)| d == depth)
            .map(|(name, _)| name.clone())
            .collect();

        for name in &popped {
            self.stack_allocations.remove(name);
            self.free(name);
        }

        popped
    }
}

/// Memory safety violation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryViolation {
    UseAfterFree(String),
    DoubleFree(String),
    BufferOverflow {
        index: i64,
        size: usize,
    },
    BufferUnderflow {
        index: i64,
    },
    NullDereference,
    UninitializedAccess(String),
    DataRace {
        var: String,
        op1: String,
        op2: String,
    },
}

impl MemoryViolation {
    pub fn severity(&self) -> u8 {
        match self {
            Self::UseAfterFree(_) => 5,
            Self::DoubleFree(_) => 5,
            Self::BufferOverflow { .. } => 5,
            Self::BufferUnderflow { .. } => 5,
            Self::NullDereference => 4,
            Self::UninitializedAccess(_) => 4,
            Self::DataRace { .. } => 5,
        }
    }

    pub fn is_exploitable(&self) -> bool {
        matches!(
            self,
            Self::UseAfterFree(_) | Self::BufferOverflow { .. } | Self::DoubleFree(_)
        )
    }

    pub fn cwe_id(&self) -> u32 {
        match self {
            Self::UseAfterFree(_) => 416,
            Self::DoubleFree(_) => 415,
            Self::BufferOverflow { .. } => 787,
            Self::BufferUnderflow { .. } => 125,
            Self::NullDereference => 476,
            Self::UninitializedAccess(_) => 908,
            Self::DataRace { .. } => 362,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::UseAfterFree(var) => format!("Use after free of variable '{}'", var),
            Self::DoubleFree(var) => format!("Double free of variable '{}'", var),
            Self::BufferOverflow { index, size } => {
                format!("Buffer overflow: index {} >= size {}", index, size)
            }
            Self::BufferUnderflow { index } => {
                format!("Buffer underflow: negative index {}", index)
            }
            Self::NullDereference => "Null pointer dereference".to_string(),
            Self::UninitializedAccess(var) => format!("Access to uninitialized variable '{}'", var),
            Self::DataRace { var, op1, op2 } => {
                format!("Data race on '{}' between {} and {}", var, op1, op2)
            }
        }
    }
}

/// Bounds checker for arrays
#[derive(Debug, Clone)]
pub struct BoundsChecker {
    array_sizes: HashMap<String, usize>,
}

impl BoundsChecker {
    pub fn new() -> Self {
        Self {
            array_sizes: HashMap::new(),
        }
    }

    pub fn register_array(&mut self, name: &str, size: usize) {
        self.array_sizes.insert(name.to_string(), size);
    }

    pub fn check_bounds(&self, name: &str, index: i64) -> Option<MemoryViolation> {
        if index < 0 {
            return Some(MemoryViolation::BufferUnderflow { index });
        }

        if let Some(&size) = self.array_sizes.get(name) {
            if index as usize >= size {
                return Some(MemoryViolation::BufferOverflow { index, size });
            }
        }

        None
    }

    pub fn get_size(&self, name: &str) -> Option<usize> {
        self.array_sizes.get(name).copied()
    }
}

impl Default for BoundsChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Null safety checker
#[derive(Debug, Clone, Default)]
pub struct NullChecker {
    nullable_vars: HashSet<String>,
    checked_vars: HashSet<String>,
}

impl NullChecker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_nullable(&mut self, var: &str) {
        self.nullable_vars.insert(var.to_string());
    }

    pub fn mark_checked(&mut self, var: &str) {
        self.checked_vars.insert(var.to_string());
    }

    pub fn unmark_checked(&mut self, var: &str) {
        self.checked_vars.remove(var);
    }

    pub fn is_nullable(&self, var: &str) -> bool {
        self.nullable_vars.contains(var)
    }

    pub fn is_safe_to_dereference(&self, var: &str) -> bool {
        !self.nullable_vars.contains(var) || self.checked_vars.contains(var)
    }

    pub fn check_dereference(&self, var: &str) -> Option<MemoryViolation> {
        if self.nullable_vars.contains(var) && !self.checked_vars.contains(var) {
            Some(MemoryViolation::NullDereference)
        } else {
            None
        }
    }
}

/// Initialization tracker
#[derive(Debug, Clone, Default)]
pub struct InitializationTracker {
    initialized: HashSet<String>,
    partially_initialized: HashMap<String, HashSet<String>>,
}

impl InitializationTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self, var: &str) {
        self.initialized.insert(var.to_string());
    }

    pub fn initialize_field(&mut self, var: &str, field: &str) {
        self.partially_initialized
            .entry(var.to_string())
            .or_default()
            .insert(field.to_string());
    }

    pub fn is_initialized(&self, var: &str) -> bool {
        self.initialized.contains(var)
    }

    pub fn is_field_initialized(&self, var: &str, field: &str) -> bool {
        self.initialized.contains(var)
            || self
                .partially_initialized
                .get(var)
                .map(|fields| fields.contains(field))
                .unwrap_or(false)
    }

    pub fn check_access(&self, var: &str) -> Option<MemoryViolation> {
        if !self.initialized.contains(var) && !self.partially_initialized.contains_key(var) {
            Some(MemoryViolation::UninitializedAccess(var.to_string()))
        } else {
            None
        }
    }
}

/// Data race detector
#[derive(Debug, Clone, Default)]
pub struct DataRaceDetector {
    concurrent_reads: HashMap<String, Vec<String>>,
    concurrent_writes: HashMap<String, Vec<String>>,
}

impl DataRaceDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_read(&mut self, var: &str, thread: &str) {
        self.concurrent_reads
            .entry(var.to_string())
            .or_default()
            .push(thread.to_string());
    }

    pub fn record_write(&mut self, var: &str, thread: &str) {
        self.concurrent_writes
            .entry(var.to_string())
            .or_default()
            .push(thread.to_string());
    }

    pub fn check_races(&self) -> Vec<MemoryViolation> {
        let mut violations = Vec::new();

        for (var, writers) in &self.concurrent_writes {
            // Multiple writers
            if writers.len() > 1 {
                violations.push(MemoryViolation::DataRace {
                    var: var.clone(),
                    op1: format!("write from {}", writers[0]),
                    op2: format!("write from {}", writers[1]),
                });
            }

            // Writer + Reader
            if let Some(readers) = self.concurrent_reads.get(var) {
                if !writers.is_empty() && !readers.is_empty() {
                    violations.push(MemoryViolation::DataRace {
                        var: var.clone(),
                        op1: format!("write from {}", writers[0]),
                        op2: format!("read from {}", readers[0]),
                    });
                }
            }
        }

        violations
    }

    pub fn clear(&mut self) {
        self.concurrent_reads.clear();
        self.concurrent_writes.clear();
    }
}

/// Check if a type is Copy (can be safely duplicated)
pub fn is_copy_type(ty: &str) -> bool {
    matches!(
        ty,
        "i8" | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
            | "bool"
            | "char"
    )
}

/// Check if a pointer operation is safe
pub fn is_safe_pointer_op(op: &str) -> bool {
    matches!(op, "as_ref" | "as_mut" | "is_null" | "is_some" | "is_none")
}

#[cfg(test)]
mod tests {
    use super::*;

    // AllocationTracker tests
    #[test]
    fn test_allocation_tracker_new() {
        let tracker = AllocationTracker::new();
        assert!(!tracker.is_allocated("x"));
        assert!(!tracker.is_freed("x"));
    }

    #[test]
    fn test_allocation_tracker_allocate() {
        let mut tracker = AllocationTracker::new();
        tracker.allocate("x");
        assert!(tracker.is_allocated("x"));
        assert!(!tracker.is_freed("x"));
    }

    #[test]
    fn test_allocation_tracker_free() {
        let mut tracker = AllocationTracker::new();
        tracker.allocate("x");
        tracker.free("x");
        assert!(!tracker.is_allocated("x"));
        assert!(tracker.is_freed("x"));
    }

    #[test]
    fn test_allocation_tracker_use_after_free() {
        let mut tracker = AllocationTracker::new();
        tracker.allocate("x");
        tracker.free("x");
        assert!(tracker.is_use_after_free("x"));
    }

    #[test]
    fn test_allocation_tracker_free_not_allocated() {
        let mut tracker = AllocationTracker::new();
        tracker.free("x"); // Should not panic
        assert!(!tracker.is_freed("x")); // Wasn't allocated
    }

    #[test]
    fn test_allocation_tracker_stack_frame() {
        let mut tracker = AllocationTracker::new();
        tracker.push_stack_frame("x", 1);
        assert!(tracker.is_allocated("x"));

        let popped = tracker.pop_stack_frame(1);
        assert_eq!(popped, vec!["x"]);
        assert!(!tracker.is_allocated("x"));
    }

    #[test]
    fn test_allocation_tracker_reallocate() {
        let mut tracker = AllocationTracker::new();
        tracker.allocate("x");
        tracker.free("x");
        tracker.allocate("x"); // Reallocate
        assert!(tracker.is_allocated("x"));
        assert!(!tracker.is_freed("x"));
    }

    // MemoryViolation tests
    #[test]
    fn test_memory_violation_severity() {
        assert_eq!(MemoryViolation::UseAfterFree("x".to_string()).severity(), 5);
        assert_eq!(MemoryViolation::DoubleFree("x".to_string()).severity(), 5);
        assert_eq!(
            MemoryViolation::BufferOverflow { index: 10, size: 5 }.severity(),
            5
        );
        assert_eq!(MemoryViolation::NullDereference.severity(), 4);
    }

    #[test]
    fn test_memory_violation_is_exploitable() {
        assert!(MemoryViolation::UseAfterFree("x".to_string()).is_exploitable());
        assert!(MemoryViolation::BufferOverflow { index: 10, size: 5 }.is_exploitable());
        assert!(!MemoryViolation::NullDereference.is_exploitable());
        assert!(!MemoryViolation::UninitializedAccess("x".to_string()).is_exploitable());
    }

    #[test]
    fn test_memory_violation_cwe_id() {
        assert_eq!(MemoryViolation::UseAfterFree("x".to_string()).cwe_id(), 416);
        assert_eq!(MemoryViolation::DoubleFree("x".to_string()).cwe_id(), 415);
        assert_eq!(
            MemoryViolation::BufferOverflow { index: 10, size: 5 }.cwe_id(),
            787
        );
        assert_eq!(MemoryViolation::BufferUnderflow { index: -1 }.cwe_id(), 125);
        assert_eq!(MemoryViolation::NullDereference.cwe_id(), 476);
        assert_eq!(
            MemoryViolation::DataRace {
                var: "x".to_string(),
                op1: "a".to_string(),
                op2: "b".to_string()
            }
            .cwe_id(),
            362
        );
    }

    #[test]
    fn test_memory_violation_description() {
        let uaf = MemoryViolation::UseAfterFree("x".to_string());
        assert!(uaf.description().contains("x"));

        let overflow = MemoryViolation::BufferOverflow { index: 10, size: 5 };
        assert!(overflow.description().contains("10"));
        assert!(overflow.description().contains("5"));
    }

    // BoundsChecker tests
    #[test]
    fn test_bounds_checker_new() {
        let checker = BoundsChecker::new();
        assert_eq!(checker.get_size("arr"), None);
    }

    #[test]
    fn test_bounds_checker_register_array() {
        let mut checker = BoundsChecker::new();
        checker.register_array("arr", 10);
        assert_eq!(checker.get_size("arr"), Some(10));
    }

    #[test]
    fn test_bounds_checker_valid_index() {
        let mut checker = BoundsChecker::new();
        checker.register_array("arr", 10);
        assert!(checker.check_bounds("arr", 0).is_none());
        assert!(checker.check_bounds("arr", 9).is_none());
    }

    #[test]
    fn test_bounds_checker_overflow() {
        let mut checker = BoundsChecker::new();
        checker.register_array("arr", 10);
        let violation = checker.check_bounds("arr", 10);
        assert!(matches!(
            violation,
            Some(MemoryViolation::BufferOverflow {
                index: 10,
                size: 10
            })
        ));
    }

    #[test]
    fn test_bounds_checker_underflow() {
        let checker = BoundsChecker::new();
        let violation = checker.check_bounds("arr", -1);
        assert!(matches!(
            violation,
            Some(MemoryViolation::BufferUnderflow { index: -1 })
        ));
    }

    #[test]
    fn test_bounds_checker_default() {
        let checker = BoundsChecker::default();
        assert_eq!(checker.get_size("arr"), None);
    }

    // NullChecker tests
    #[test]
    fn test_null_checker_new() {
        let checker = NullChecker::new();
        assert!(!checker.is_nullable("x"));
    }

    #[test]
    fn test_null_checker_mark_nullable() {
        let mut checker = NullChecker::new();
        checker.mark_nullable("x");
        assert!(checker.is_nullable("x"));
        assert!(!checker.is_safe_to_dereference("x"));
    }

    #[test]
    fn test_null_checker_mark_checked() {
        let mut checker = NullChecker::new();
        checker.mark_nullable("x");
        checker.mark_checked("x");
        assert!(checker.is_safe_to_dereference("x"));
    }

    #[test]
    fn test_null_checker_unmark_checked() {
        let mut checker = NullChecker::new();
        checker.mark_nullable("x");
        checker.mark_checked("x");
        checker.unmark_checked("x");
        assert!(!checker.is_safe_to_dereference("x"));
    }

    #[test]
    fn test_null_checker_dereference_safe() {
        let checker = NullChecker::new();
        assert!(checker.check_dereference("x").is_none());
    }

    #[test]
    fn test_null_checker_dereference_unsafe() {
        let mut checker = NullChecker::new();
        checker.mark_nullable("x");
        assert!(matches!(
            checker.check_dereference("x"),
            Some(MemoryViolation::NullDereference)
        ));
    }

    // InitializationTracker tests
    #[test]
    fn test_initialization_tracker_new() {
        let tracker = InitializationTracker::new();
        assert!(!tracker.is_initialized("x"));
    }

    #[test]
    fn test_initialization_tracker_initialize() {
        let mut tracker = InitializationTracker::new();
        tracker.initialize("x");
        assert!(tracker.is_initialized("x"));
    }

    #[test]
    fn test_initialization_tracker_field() {
        let mut tracker = InitializationTracker::new();
        tracker.initialize_field("obj", "field1");
        assert!(!tracker.is_initialized("obj"));
        assert!(tracker.is_field_initialized("obj", "field1"));
        assert!(!tracker.is_field_initialized("obj", "field2"));
    }

    #[test]
    fn test_initialization_tracker_check_access_uninit() {
        let tracker = InitializationTracker::new();
        assert!(matches!(
            tracker.check_access("x"),
            Some(MemoryViolation::UninitializedAccess(_))
        ));
    }

    #[test]
    fn test_initialization_tracker_check_access_init() {
        let mut tracker = InitializationTracker::new();
        tracker.initialize("x");
        assert!(tracker.check_access("x").is_none());
    }

    // DataRaceDetector tests
    #[test]
    fn test_data_race_detector_new() {
        let detector = DataRaceDetector::new();
        assert!(detector.check_races().is_empty());
    }

    #[test]
    fn test_data_race_detector_no_race_reads() {
        let mut detector = DataRaceDetector::new();
        detector.record_read("x", "thread1");
        detector.record_read("x", "thread2");
        assert!(detector.check_races().is_empty());
    }

    #[test]
    fn test_data_race_detector_race_write_write() {
        let mut detector = DataRaceDetector::new();
        detector.record_write("x", "thread1");
        detector.record_write("x", "thread2");
        let races = detector.check_races();
        assert!(!races.is_empty());
    }

    #[test]
    fn test_data_race_detector_race_read_write() {
        let mut detector = DataRaceDetector::new();
        detector.record_read("x", "thread1");
        detector.record_write("x", "thread2");
        let races = detector.check_races();
        assert!(!races.is_empty());
    }

    #[test]
    fn test_data_race_detector_clear() {
        let mut detector = DataRaceDetector::new();
        detector.record_write("x", "thread1");
        detector.clear();
        assert!(detector.check_races().is_empty());
    }

    // Helper function tests
    #[test]
    fn test_is_copy_type() {
        assert!(is_copy_type("i32"));
        assert!(is_copy_type("u64"));
        assert!(is_copy_type("f64"));
        assert!(is_copy_type("bool"));
        assert!(is_copy_type("char"));
        assert!(!is_copy_type("String"));
        assert!(!is_copy_type("Vec"));
    }

    #[test]
    fn test_is_safe_pointer_op() {
        assert!(is_safe_pointer_op("as_ref"));
        assert!(is_safe_pointer_op("as_mut"));
        assert!(is_safe_pointer_op("is_null"));
        assert!(is_safe_pointer_op("is_some"));
        assert!(is_safe_pointer_op("is_none"));
        assert!(!is_safe_pointer_op("offset"));
        assert!(!is_safe_pointer_op("add"));
    }
}
