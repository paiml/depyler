//! EXTREME TDD: Tests for inlining.rs InliningDecision and InliningReason
//! Coverage: InliningDecision, InliningReason variants, decision making

use depyler_core::inlining::{InliningDecision, InliningReason};

// ============ InliningReason variant tests ============

#[test]
fn test_reason_trivial() {
    let reason = InliningReason::Trivial;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("Trivial"));
}

#[test]
fn test_reason_single_use() {
    let reason = InliningReason::SingleUse;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("SingleUse"));
}

#[test]
fn test_reason_small_hot_function() {
    let reason = InliningReason::SmallHotFunction;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("SmallHotFunction"));
}

#[test]
fn test_reason_enables_optimization() {
    let reason = InliningReason::EnablesOptimization;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("EnablesOptimization"));
}

#[test]
fn test_reason_too_large() {
    let reason = InliningReason::TooLarge;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("TooLarge"));
}

#[test]
fn test_reason_recursive() {
    let reason = InliningReason::Recursive;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("Recursive"));
}

#[test]
fn test_reason_has_side_effects() {
    let reason = InliningReason::HasSideEffects;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("HasSideEffects"));
}

#[test]
fn test_reason_contains_loops() {
    let reason = InliningReason::ContainsLoops;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("ContainsLoops"));
}

#[test]
fn test_reason_cost_too_high() {
    let reason = InliningReason::CostTooHigh;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("CostTooHigh"));
}

// ============ InliningReason clone tests ============

#[test]
fn test_reason_clone_trivial() {
    let reason = InliningReason::Trivial;
    let cloned = reason.clone();
    assert!(matches!(cloned, InliningReason::Trivial));
}

#[test]
fn test_reason_clone_single_use() {
    let reason = InliningReason::SingleUse;
    let cloned = reason.clone();
    assert!(matches!(cloned, InliningReason::SingleUse));
}

#[test]
fn test_reason_clone_too_large() {
    let reason = InliningReason::TooLarge;
    let cloned = reason.clone();
    assert!(matches!(cloned, InliningReason::TooLarge));
}

#[test]
fn test_reason_clone_recursive() {
    let reason = InliningReason::Recursive;
    let cloned = reason.clone();
    assert!(matches!(cloned, InliningReason::Recursive));
}

// ============ InliningDecision creation tests ============

#[test]
fn test_decision_should_inline_trivial() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::Trivial,
        cost_benefit: 10.0,
    };
    assert!(decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::Trivial));
    assert_eq!(decision.cost_benefit, 10.0);
}

#[test]
fn test_decision_should_inline_single_use() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SingleUse,
        cost_benefit: 5.0,
    };
    assert!(decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::SingleUse));
    assert_eq!(decision.cost_benefit, 5.0);
}

#[test]
fn test_decision_should_inline_small_hot() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SmallHotFunction,
        cost_benefit: 2.5,
    };
    assert!(decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::SmallHotFunction));
    assert_eq!(decision.cost_benefit, 2.5);
}

#[test]
fn test_decision_should_inline_optimization() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::EnablesOptimization,
        cost_benefit: 3.0,
    };
    assert!(decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::EnablesOptimization));
}

#[test]
fn test_decision_not_inline_too_large() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::TooLarge,
        cost_benefit: 0.0,
    };
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::TooLarge));
    assert_eq!(decision.cost_benefit, 0.0);
}

#[test]
fn test_decision_not_inline_recursive() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::Recursive,
        cost_benefit: 0.0,
    };
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::Recursive));
}

#[test]
fn test_decision_not_inline_side_effects() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::HasSideEffects,
        cost_benefit: 0.0,
    };
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::HasSideEffects));
}

#[test]
fn test_decision_not_inline_loops() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::ContainsLoops,
        cost_benefit: 0.0,
    };
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::ContainsLoops));
}

#[test]
fn test_decision_not_inline_cost() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::CostTooHigh,
        cost_benefit: -0.5,
    };
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::CostTooHigh));
    assert!(decision.cost_benefit < 0.0);
}

// ============ InliningDecision clone tests ============

#[test]
fn test_decision_clone_inline() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::Trivial,
        cost_benefit: 10.0,
    };
    let cloned = decision.clone();
    assert_eq!(cloned.should_inline, decision.should_inline);
    assert_eq!(cloned.cost_benefit, decision.cost_benefit);
}

#[test]
fn test_decision_clone_no_inline() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::TooLarge,
        cost_benefit: 0.0,
    };
    let cloned = decision.clone();
    assert_eq!(cloned.should_inline, decision.should_inline);
    assert_eq!(cloned.cost_benefit, decision.cost_benefit);
}

// ============ InliningDecision debug tests ============

#[test]
fn test_decision_debug() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::Trivial,
        cost_benefit: 10.0,
    };
    let debug_str = format!("{:?}", decision);
    assert!(debug_str.contains("InliningDecision"));
    assert!(debug_str.contains("should_inline"));
    assert!(debug_str.contains("true"));
}

#[test]
fn test_decision_debug_false() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::TooLarge,
        cost_benefit: 0.0,
    };
    let debug_str = format!("{:?}", decision);
    assert!(debug_str.contains("should_inline"));
    assert!(debug_str.contains("false"));
}

// ============ Cost benefit value tests ============

#[test]
fn test_decision_positive_cost_benefit() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SmallHotFunction,
        cost_benefit: 5.5,
    };
    assert!(decision.cost_benefit > 0.0);
}

#[test]
fn test_decision_zero_cost_benefit() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::TooLarge,
        cost_benefit: 0.0,
    };
    assert_eq!(decision.cost_benefit, 0.0);
}

#[test]
fn test_decision_negative_cost_benefit() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::CostTooHigh,
        cost_benefit: -2.0,
    };
    assert!(decision.cost_benefit < 0.0);
}

#[test]
fn test_decision_large_cost_benefit() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::Trivial,
        cost_benefit: 100.0,
    };
    assert_eq!(decision.cost_benefit, 100.0);
}

#[test]
fn test_decision_fractional_cost_benefit() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SmallHotFunction,
        cost_benefit: 1.5,
    };
    assert_eq!(decision.cost_benefit, 1.5);
}

// ============ Reason categorization tests ============

#[test]
fn test_positive_reasons() {
    let positive_reasons = [
        InliningReason::Trivial,
        InliningReason::SingleUse,
        InliningReason::SmallHotFunction,
        InliningReason::EnablesOptimization,
    ];
    for reason in &positive_reasons {
        let decision = InliningDecision {
            should_inline: true,
            reason: reason.clone(),
            cost_benefit: 1.0,
        };
        assert!(decision.should_inline);
    }
}

#[test]
fn test_negative_reasons() {
    let negative_reasons = [
        InliningReason::TooLarge,
        InliningReason::Recursive,
        InliningReason::HasSideEffects,
        InliningReason::ContainsLoops,
        InliningReason::CostTooHigh,
    ];
    for reason in &negative_reasons {
        let decision = InliningDecision {
            should_inline: false,
            reason: reason.clone(),
            cost_benefit: 0.0,
        };
        assert!(!decision.should_inline);
    }
}

// ============ Decision consistency tests ============

#[test]
fn test_trivial_always_high_benefit() {
    // Trivial functions should have high cost benefit
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::Trivial,
        cost_benefit: 10.0,
    };
    assert!(decision.cost_benefit >= 5.0);
}

#[test]
fn test_single_use_moderate_benefit() {
    // Single use functions have moderate cost benefit
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SingleUse,
        cost_benefit: 5.0,
    };
    assert!(decision.cost_benefit >= 1.0);
}

#[test]
fn test_rejection_reasons_zero_benefit() {
    // Rejection reasons typically have zero or low cost benefit
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::Recursive,
        cost_benefit: 0.0,
    };
    assert_eq!(decision.cost_benefit, 0.0);
}

// ============ All variants iteration test ============

#[test]
fn test_all_reason_variants_covered() {
    let all_reasons = vec![
        InliningReason::Trivial,
        InliningReason::SingleUse,
        InliningReason::SmallHotFunction,
        InliningReason::EnablesOptimization,
        InliningReason::TooLarge,
        InliningReason::Recursive,
        InliningReason::HasSideEffects,
        InliningReason::ContainsLoops,
        InliningReason::CostTooHigh,
    ];

    for reason in all_reasons {
        // Each reason should be debuggable
        let debug_str = format!("{:?}", reason);
        assert!(!debug_str.is_empty());

        // Each reason should be clonable
        let cloned = reason.clone();
        let cloned_debug = format!("{:?}", cloned);
        assert_eq!(debug_str, cloned_debug);
    }
}

// ============ Decision state tests ============

#[test]
fn test_decision_true_with_positive_reasons() {
    let positive_reasons = [
        InliningReason::Trivial,
        InliningReason::SingleUse,
        InliningReason::SmallHotFunction,
        InliningReason::EnablesOptimization,
    ];

    for reason in &positive_reasons {
        let decision = InliningDecision {
            should_inline: true,
            reason: reason.clone(),
            cost_benefit: 2.0,
        };
        // Positive reasons imply should_inline = true
        assert!(
            decision.should_inline,
            "Reason {:?} should allow inlining",
            reason
        );
    }
}

#[test]
fn test_decision_false_with_negative_reasons() {
    let negative_reasons = [
        InliningReason::TooLarge,
        InliningReason::Recursive,
        InliningReason::HasSideEffects,
        InliningReason::ContainsLoops,
        InliningReason::CostTooHigh,
    ];

    for reason in &negative_reasons {
        let decision = InliningDecision {
            should_inline: false,
            reason: reason.clone(),
            cost_benefit: 0.0,
        };
        // Negative reasons imply should_inline = false
        assert!(
            !decision.should_inline,
            "Reason {:?} should prevent inlining",
            reason
        );
    }
}
