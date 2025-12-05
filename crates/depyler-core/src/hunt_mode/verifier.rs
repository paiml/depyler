//! Andon Verifier - Visual Control and Stop-the-Line Signaling
//!
//! Implements Andon (行灯) - Visual Control / Stop the Line
//! Provides immediate visibility into system state and automatic
//! escalation on failure.
//!
//! Reference: Baudin, M. (2007). Working with Machines

use super::isolator::ReproCase;
use super::repair::Fix;

/// Result of verification
#[derive(Debug, Clone)]
pub enum VerifyResult {
    /// Fix successfully applied and verified
    Success,
    /// Fix needs human review before committing
    NeedsReview {
        fix: Fix,
        confidence: f64,
        reason: String,
    },
    /// Fix failed verification
    FixFailed(String),
    /// No fix was available to verify
    NoFixAvailable,
}

/// Andon status indicator
///
/// Visual representation of system health for the dashboard.
#[derive(Debug, Clone)]
pub enum AndonStatus {
    /// All systems operational, compilation rate on target
    Green {
        compilation_rate: f64,
        message: String,
    },
    /// Warning condition, needs attention but not blocking
    Yellow {
        warnings: Vec<String>,
        needs_attention: bool,
    },
    /// Critical issue, cycle halted
    Red {
        error: String,
        cycle_halted: bool,
    },
    /// System idle, no active work
    Idle,
}

impl AndonStatus {
    /// Check if status indicates a problem
    pub fn is_problem(&self) -> bool {
        matches!(self, AndonStatus::Yellow { needs_attention: true, .. } | AndonStatus::Red { .. })
    }

    /// Check if cycle should halt
    pub fn should_halt(&self) -> bool {
        matches!(self, AndonStatus::Red { cycle_halted: true, .. })
    }

    /// Get status color as string (for CLI display)
    pub fn color(&self) -> &'static str {
        match self {
            AndonStatus::Green { .. } => "green",
            AndonStatus::Yellow { .. } => "yellow",
            AndonStatus::Red { .. } => "red",
            AndonStatus::Idle => "gray",
        }
    }

    /// Get status emoji
    pub fn emoji(&self) -> &'static str {
        match self {
            AndonStatus::Green { .. } => "🟢",
            AndonStatus::Yellow { .. } => "🟡",
            AndonStatus::Red { .. } => "🔴",
            AndonStatus::Idle => "⚪",
        }
    }
}

impl std::fmt::Display for AndonStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AndonStatus::Green { compilation_rate, message } => {
                write!(f, "{} GREEN ({:.1}%): {}", self.emoji(), compilation_rate * 100.0, message)
            }
            AndonStatus::Yellow { warnings, .. } => {
                write!(f, "{} YELLOW: {} warning(s)", self.emoji(), warnings.len())
            }
            AndonStatus::Red { error, .. } => {
                write!(f, "{} RED: {}", self.emoji(), error)
            }
            AndonStatus::Idle => {
                write!(f, "{} IDLE", self.emoji())
            }
        }
    }
}

/// Andon Verifier: Validates fixes and provides visual status
///
/// Andon: Immediate visibility and escalation on failure.
#[derive(Debug)]
pub struct AndonVerifier {
    /// Current status
    status: AndonStatus,
    /// History of status changes
    status_history: Vec<AndonStatus>,
    /// Total fixes verified
    total_verified: u32,
    /// Successful verifications
    successful_verifications: u32,
}

impl AndonVerifier {
    /// Create a new verifier
    pub fn new() -> Self {
        Self {
            status: AndonStatus::Idle,
            status_history: Vec::new(),
            total_verified: 0,
            successful_verifications: 0,
        }
    }

    /// Get current Andon status
    pub fn status(&self) -> &AndonStatus {
        &self.status
    }

    /// Verify a fix and commit if successful
    ///
    /// Andon: Immediate visibility and escalation on failure.
    pub fn verify_and_commit(&mut self, fix: &Fix, repro: &ReproCase) -> anyhow::Result<VerifyResult> {
        self.total_verified += 1;

        // Step 1: Compile the fixed output
        let compile_result = self.try_compile(&fix.rust_output);

        match compile_result {
            Ok(()) => {
                // Step 2: Run property tests (if applicable)
                if let Err(prop_failure) = self.run_property_tests(fix) {
                    self.update_status(AndonStatus::Yellow {
                        warnings: vec![format!("Property test failed: {}", prop_failure)],
                        needs_attention: true,
                    });
                    return Ok(VerifyResult::NeedsReview {
                        fix: fix.clone(),
                        confidence: fix.confidence * 0.8, // Reduce confidence
                        reason: format!("Property test failed: {}", prop_failure),
                    });
                }

                // Step 3: Check for regressions
                if let Err(regression) = self.check_regressions(fix) {
                    self.update_status(AndonStatus::Red {
                        error: regression.clone(),
                        cycle_halted: true,
                    });
                    return Ok(VerifyResult::FixFailed(regression));
                }

                // Success!
                self.successful_verifications += 1;
                let new_rate = self.calculate_compilation_rate();

                self.update_status(AndonStatus::Green {
                    compilation_rate: new_rate,
                    message: format!("Fix {} verified successfully", fix.ticket_id),
                });

                // Commit the fix (in real impl, would update config or patch code)
                self.commit_fix(fix)?;

                Ok(VerifyResult::Success)
            }
            Err(compile_error) => {
                // STOP THE LINE - fix did not work
                self.update_status(AndonStatus::Red {
                    error: compile_error.clone(),
                    cycle_halted: true,
                });
                Ok(VerifyResult::FixFailed(compile_error))
            }
        }
    }

    /// Try to compile Rust code
    fn try_compile(&self, rust_code: &str) -> Result<(), String> {
        // In real implementation:
        // 1. Write rust_code to temp file
        // 2. Run rustc --crate-type lib --edition 2021
        // 3. Check exit code

        // For now, simulate compilation
        if rust_code.is_empty() {
            // Empty code always "passes" (would be filled in real impl)
            Ok(())
        } else if rust_code.contains("COMPILE_ERROR") {
            Err("Simulated compilation error".to_string())
        } else {
            Ok(())
        }
    }

    /// Run property tests
    fn run_property_tests(&self, _fix: &Fix) -> Result<(), String> {
        // In real implementation:
        // 1. Generate proptest tests
        // 2. Run cargo test
        // 3. Check results

        Ok(()) // Simulate success
    }

    /// Check for regressions in existing examples
    fn check_regressions(&self, _fix: &Fix) -> Result<(), String> {
        // In real implementation:
        // 1. Re-transpile all examples
        // 2. Verify previously passing code still passes
        // 3. Report any regressions

        Ok(()) // Simulate no regressions
    }

    /// Calculate current compilation rate
    fn calculate_compilation_rate(&self) -> f64 {
        // In real implementation, would measure actual rate
        // For now, estimate based on verification success rate
        if self.total_verified == 0 {
            return 0.0;
        }
        self.successful_verifications as f64 / self.total_verified as f64
    }

    /// Commit a verified fix
    fn commit_fix(&self, fix: &Fix) -> anyhow::Result<()> {
        // In real implementation:
        // 1. Update .depyler/config.toml with new rule
        // 2. Or patch depyler-core source code
        // 3. Run git commit

        tracing::info!("Committing fix: {} - {}", fix.ticket_id, fix.description);
        Ok(())
    }

    /// Update status and record in history
    fn update_status(&mut self, new_status: AndonStatus) {
        self.status_history.push(self.status.clone());
        self.status = new_status;
    }

    /// Get status history
    pub fn history(&self) -> &[AndonStatus] {
        &self.status_history
    }

    /// Get verification statistics
    pub fn stats(&self) -> (u32, u32) {
        (self.total_verified, self.successful_verifications)
    }

    /// Signal that human review is needed
    pub fn request_human_review(&mut self, reason: &str) {
        self.update_status(AndonStatus::Yellow {
            warnings: vec![reason.to_string()],
            needs_attention: true,
        });
    }

    /// Signal critical error (stop the line)
    pub fn halt(&mut self, error: &str) {
        self.update_status(AndonStatus::Red {
            error: error.to_string(),
            cycle_halted: true,
        });
    }

    /// Reset to idle state
    pub fn reset(&mut self) {
        self.update_status(AndonStatus::Idle);
    }
}

impl Default for AndonVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::repair::PatchLocation;

    fn create_test_fix() -> Fix {
        Fix {
            id: "fix_test".to_string(),
            ticket_id: "DEPYLER-TEST".to_string(),
            description: "Test fix".to_string(),
            mutator_name: "TestMutator".to_string(),
            confidence: 0.9,
            rust_output: "fn test() {}".to_string(),
            patch_location: None,
        }
    }

    fn create_test_repro() -> ReproCase {
        ReproCase::new(
            "test source".to_string(),
            "E0308".to_string(),
            "test_pattern".to_string(),
        )
    }

    #[test]
    fn test_verifier_new() {
        let verifier = AndonVerifier::new();
        assert!(matches!(verifier.status(), AndonStatus::Idle));
        assert_eq!(verifier.total_verified, 0);
    }

    #[test]
    fn test_verify_success() {
        let mut verifier = AndonVerifier::new();
        let fix = create_test_fix();
        let repro = create_test_repro();

        let result = verifier.verify_and_commit(&fix, &repro).unwrap();
        assert!(matches!(result, VerifyResult::Success));
        assert!(matches!(verifier.status(), AndonStatus::Green { .. }));
    }

    #[test]
    fn test_verify_compile_failure() {
        let mut verifier = AndonVerifier::new();
        let mut fix = create_test_fix();
        fix.rust_output = "COMPILE_ERROR".to_string();
        let repro = create_test_repro();

        let result = verifier.verify_and_commit(&fix, &repro).unwrap();
        assert!(matches!(result, VerifyResult::FixFailed(_)));
        assert!(matches!(verifier.status(), AndonStatus::Red { .. }));
    }

    #[test]
    fn test_andon_status_display() {
        let green = AndonStatus::Green {
            compilation_rate: 0.85,
            message: "All good".to_string(),
        };
        let display = format!("{}", green);
        assert!(display.contains("GREEN"));
        assert!(display.contains("85.0%"));
    }

    #[test]
    fn test_andon_status_emoji() {
        assert_eq!(AndonStatus::Idle.emoji(), "⚪");
        assert_eq!(AndonStatus::Green { compilation_rate: 0.0, message: String::new() }.emoji(), "🟢");
        assert_eq!(AndonStatus::Yellow { warnings: vec![], needs_attention: false }.emoji(), "🟡");
        assert_eq!(AndonStatus::Red { error: String::new(), cycle_halted: false }.emoji(), "🔴");
    }

    #[test]
    fn test_should_halt() {
        let red_halted = AndonStatus::Red {
            error: "error".to_string(),
            cycle_halted: true,
        };
        assert!(red_halted.should_halt());

        let red_not_halted = AndonStatus::Red {
            error: "error".to_string(),
            cycle_halted: false,
        };
        assert!(!red_not_halted.should_halt());
    }

    #[test]
    fn test_status_history() {
        let mut verifier = AndonVerifier::new();
        verifier.request_human_review("Test warning");
        verifier.reset();

        assert_eq!(verifier.history().len(), 2);
    }

    #[test]
    fn test_halt() {
        let mut verifier = AndonVerifier::new();
        verifier.halt("Critical error");

        assert!(verifier.status().should_halt());
        assert!(matches!(verifier.status(), AndonStatus::Red { .. }));
    }
}
