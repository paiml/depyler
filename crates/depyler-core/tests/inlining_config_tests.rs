//! EXTREME TDD: Tests for inlining.rs InliningConfig
//! Coverage: InliningConfig, default values, custom configurations

use depyler_core::inlining::{InliningAnalyzer, InliningConfig};

// ============ Default configuration tests ============

#[test]
fn test_config_default_max_inline_size() {
    let config = InliningConfig::default();
    assert_eq!(config.max_inline_size, 20);
}

#[test]
fn test_config_default_max_inline_depth() {
    let config = InliningConfig::default();
    assert_eq!(config.max_inline_depth, 3);
}

#[test]
fn test_config_default_inline_single_use() {
    let config = InliningConfig::default();
    assert!(config.inline_single_use);
}

#[test]
fn test_config_default_inline_trivial() {
    let config = InliningConfig::default();
    assert!(config.inline_trivial);
}

#[test]
fn test_config_default_cost_threshold() {
    let config = InliningConfig::default();
    assert_eq!(config.cost_threshold, 1.5);
}

#[test]
fn test_config_default_inline_loops() {
    let config = InliningConfig::default();
    assert!(!config.inline_loops);
}

// ============ Custom configuration tests ============

#[test]
fn test_config_custom_max_inline_size_small() {
    let config = InliningConfig {
        max_inline_size: 5,
        ..Default::default()
    };
    assert_eq!(config.max_inline_size, 5);
}

#[test]
fn test_config_custom_max_inline_size_large() {
    let config = InliningConfig {
        max_inline_size: 100,
        ..Default::default()
    };
    assert_eq!(config.max_inline_size, 100);
}

#[test]
fn test_config_custom_max_inline_depth_zero() {
    let config = InliningConfig {
        max_inline_depth: 0,
        ..Default::default()
    };
    assert_eq!(config.max_inline_depth, 0);
}

#[test]
fn test_config_custom_max_inline_depth_deep() {
    let config = InliningConfig {
        max_inline_depth: 10,
        ..Default::default()
    };
    assert_eq!(config.max_inline_depth, 10);
}

#[test]
fn test_config_custom_disable_single_use() {
    let config = InliningConfig {
        inline_single_use: false,
        ..Default::default()
    };
    assert!(!config.inline_single_use);
}

#[test]
fn test_config_custom_disable_trivial() {
    let config = InliningConfig {
        inline_trivial: false,
        ..Default::default()
    };
    assert!(!config.inline_trivial);
}

#[test]
fn test_config_custom_cost_threshold_low() {
    let config = InliningConfig {
        cost_threshold: 0.5,
        ..Default::default()
    };
    assert_eq!(config.cost_threshold, 0.5);
}

#[test]
fn test_config_custom_cost_threshold_high() {
    let config = InliningConfig {
        cost_threshold: 5.0,
        ..Default::default()
    };
    assert_eq!(config.cost_threshold, 5.0);
}

#[test]
fn test_config_custom_enable_inline_loops() {
    let config = InliningConfig {
        inline_loops: true,
        ..Default::default()
    };
    assert!(config.inline_loops);
}

// ============ Full custom configuration tests ============

#[test]
fn test_config_aggressive_inlining() {
    let config = InliningConfig {
        max_inline_size: 50,
        max_inline_depth: 5,
        inline_single_use: true,
        inline_trivial: true,
        cost_threshold: 0.5,
        inline_loops: true,
    };
    assert_eq!(config.max_inline_size, 50);
    assert_eq!(config.max_inline_depth, 5);
    assert!(config.inline_single_use);
    assert!(config.inline_trivial);
    assert_eq!(config.cost_threshold, 0.5);
    assert!(config.inline_loops);
}

#[test]
fn test_config_conservative_inlining() {
    let config = InliningConfig {
        max_inline_size: 10,
        max_inline_depth: 1,
        inline_single_use: false,
        inline_trivial: false,
        cost_threshold: 3.0,
        inline_loops: false,
    };
    assert_eq!(config.max_inline_size, 10);
    assert_eq!(config.max_inline_depth, 1);
    assert!(!config.inline_single_use);
    assert!(!config.inline_trivial);
    assert_eq!(config.cost_threshold, 3.0);
    assert!(!config.inline_loops);
}

#[test]
fn test_config_no_inlining() {
    let config = InliningConfig {
        max_inline_size: 0,
        max_inline_depth: 0,
        inline_single_use: false,
        inline_trivial: false,
        cost_threshold: 100.0,
        inline_loops: false,
    };
    assert_eq!(config.max_inline_size, 0);
    assert_eq!(config.max_inline_depth, 0);
    assert!(!config.inline_single_use);
    assert!(!config.inline_trivial);
}

// ============ Analyzer with config tests ============

#[test]
fn test_analyzer_with_default_config() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    // Analyzer should be created successfully
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

#[test]
fn test_analyzer_with_aggressive_config() {
    let config = InliningConfig {
        max_inline_size: 100,
        max_inline_depth: 10,
        inline_single_use: true,
        inline_trivial: true,
        cost_threshold: 0.1,
        inline_loops: true,
    };
    let analyzer = InliningAnalyzer::new(config);
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

#[test]
fn test_analyzer_with_conservative_config() {
    let config = InliningConfig {
        max_inline_size: 5,
        max_inline_depth: 1,
        inline_single_use: false,
        inline_trivial: false,
        cost_threshold: 10.0,
        inline_loops: false,
    };
    let analyzer = InliningAnalyzer::new(config);
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

// ============ Config clone tests ============

#[test]
fn test_config_clone() {
    let config = InliningConfig {
        max_inline_size: 25,
        max_inline_depth: 4,
        inline_single_use: true,
        inline_trivial: false,
        cost_threshold: 2.0,
        inline_loops: true,
    };
    let cloned = config.clone();
    assert_eq!(cloned.max_inline_size, config.max_inline_size);
    assert_eq!(cloned.max_inline_depth, config.max_inline_depth);
    assert_eq!(cloned.inline_single_use, config.inline_single_use);
    assert_eq!(cloned.inline_trivial, config.inline_trivial);
    assert_eq!(cloned.cost_threshold, config.cost_threshold);
    assert_eq!(cloned.inline_loops, config.inline_loops);
}

#[test]
fn test_config_debug() {
    let config = InliningConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("InliningConfig"));
    assert!(debug_str.contains("max_inline_size"));
    assert!(debug_str.contains("max_inline_depth"));
}

// ============ Edge case configurations ============

#[test]
fn test_config_zero_cost_threshold() {
    let config = InliningConfig {
        cost_threshold: 0.0,
        ..Default::default()
    };
    assert_eq!(config.cost_threshold, 0.0);
}

#[test]
fn test_config_negative_cost_threshold() {
    let config = InliningConfig {
        cost_threshold: -1.0,
        ..Default::default()
    };
    assert_eq!(config.cost_threshold, -1.0);
}

#[test]
fn test_config_very_large_size() {
    let config = InliningConfig {
        max_inline_size: usize::MAX,
        ..Default::default()
    };
    assert_eq!(config.max_inline_size, usize::MAX);
}

#[test]
fn test_config_very_large_depth() {
    let config = InliningConfig {
        max_inline_depth: usize::MAX,
        ..Default::default()
    };
    assert_eq!(config.max_inline_depth, usize::MAX);
}
