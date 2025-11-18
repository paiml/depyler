// Chaos Engineering Property Tests (renacer pattern)
// Source: renacer v0.4.1 (https://github.com/paiml/renacer)

use depyler_core::chaos::{ChaosConfig, ChaosError};
use proptest::prelude::*;
use std::time::Duration;

// Property: CPU limit is always clamped to [0.0, 1.0]
proptest! {
    #[test]
    fn test_cpu_limit_clamping(limit in any::<f64>()) {
        let config = ChaosConfig::new().with_cpu_limit(limit);
        prop_assert!(config.cpu_limit >= 0.0 && config.cpu_limit <= 1.0);
    }

    #[test]
    fn test_memory_limit_nonnegative(limit in any::<usize>()) {
        let config = ChaosConfig::new().with_memory_limit(limit);
        prop_assert_eq!(config.memory_limit, limit);
    }

    #[test]
    fn test_timeout_preserved(secs in 1u64..3600) {
        let timeout = Duration::from_secs(secs);
        let config = ChaosConfig::new().with_timeout(timeout);
        prop_assert_eq!(config.timeout, timeout);
    }

    #[test]
    fn test_signal_injection_toggle(enabled in any::<bool>()) {
        let config = ChaosConfig::new().with_signal_injection(enabled);
        prop_assert_eq!(config.signal_injection, enabled);
    }
}

// Property: Builder pattern preserves all values
proptest! {
    #[test]
    fn test_builder_preserves_values(
        mem in any::<usize>(),
        cpu in any::<f64>(),
        timeout_secs in 1u64..3600,
        signals in any::<bool>(),
    ) {
        let config = ChaosConfig::new()
            .with_memory_limit(mem)
            .with_cpu_limit(cpu)
            .with_timeout(Duration::from_secs(timeout_secs))
            .with_signal_injection(signals)
            .build();

        prop_assert_eq!(config.memory_limit, mem);
        prop_assert!(config.cpu_limit >= 0.0 && config.cpu_limit <= 1.0);
        prop_assert_eq!(config.timeout, Duration::from_secs(timeout_secs));
        prop_assert_eq!(config.signal_injection, signals);
    }
}

// Property: Gentle preset has expected invariants
#[test]
fn test_gentle_preset_invariants() {
    let gentle = ChaosConfig::gentle();

    assert!(gentle.memory_limit > 0, "Gentle should limit memory");
    assert!(
        gentle.cpu_limit > 0.5,
        "Gentle should allow >50% CPU (actual: {})",
        gentle.cpu_limit
    );
    assert!(
        gentle.timeout >= Duration::from_secs(60),
        "Gentle should have >=60s timeout"
    );
    assert!(!gentle.signal_injection, "Gentle should not inject signals");
}

// Property: Aggressive preset has expected invariants
#[test]
fn test_aggressive_preset_invariants() {
    let aggressive = ChaosConfig::aggressive();

    assert!(
        aggressive.memory_limit > 0,
        "Aggressive should limit memory"
    );
    assert!(
        aggressive.memory_limit < 128 * 1024 * 1024,
        "Aggressive should have tight memory limit (<128MB)"
    );
    assert!(
        aggressive.cpu_limit < 0.5,
        "Aggressive should throttle CPU (<50%, actual: {})",
        aggressive.cpu_limit
    );
    assert!(
        aggressive.timeout <= Duration::from_secs(30),
        "Aggressive should have short timeout (<=30s)"
    );
    assert!(
        aggressive.signal_injection,
        "Aggressive should inject signals"
    );
}

// Property: ChaosError display is never empty
proptest! {
    #[test]
    fn test_chaos_error_display_nonempty(
        limit in any::<usize>(),
        used in any::<usize>(),
        elapsed_secs in 1u64..100,
        limit_secs in 1u64..100,
    ) {
        let mem_err = ChaosError::MemoryLimitExceeded { limit, used };
        prop_assert!(!mem_err.to_string().is_empty());

        let timeout_err = ChaosError::Timeout {
            elapsed: Duration::from_secs(elapsed_secs),
            limit: Duration::from_secs(limit_secs),
        };
        prop_assert!(!timeout_err.to_string().is_empty());

        let signal_err = ChaosError::SignalInjectionFailed {
            signal: 9,
            reason: "test".to_string(),
        };
        prop_assert!(!signal_err.to_string().is_empty());
    }
}

// Property: Extreme CPU limits are clamped correctly
#[test]
fn test_extreme_cpu_limits() {
    let over = ChaosConfig::new().with_cpu_limit(f64::MAX);
    assert_eq!(over.cpu_limit, 1.0, "MAX should clamp to 1.0");

    let under = ChaosConfig::new().with_cpu_limit(f64::MIN);
    assert_eq!(under.cpu_limit, 0.0, "MIN should clamp to 0.0");

    let inf = ChaosConfig::new().with_cpu_limit(f64::INFINITY);
    assert_eq!(inf.cpu_limit, 1.0, "INFINITY should clamp to 1.0");

    let neg_inf = ChaosConfig::new().with_cpu_limit(f64::NEG_INFINITY);
    assert_eq!(neg_inf.cpu_limit, 0.0, "NEG_INFINITY should clamp to 0.0");

    let nan = ChaosConfig::new().with_cpu_limit(f64::NAN);
    assert!(
        nan.cpu_limit.is_nan() || (nan.cpu_limit >= 0.0 && nan.cpu_limit <= 1.0),
        "NAN should be handled safely"
    );
}

// Property: Default config is safe for general use
#[test]
fn test_default_config_safety() {
    let default = ChaosConfig::default();

    assert_eq!(default.memory_limit, 0, "Default should not limit memory");
    assert_eq!(default.cpu_limit, 0.0, "Default should not limit CPU");
    assert!(
        default.timeout >= Duration::from_secs(1),
        "Default should have reasonable timeout"
    );
    assert!(
        !default.signal_injection,
        "Default should not inject signals"
    );
}
