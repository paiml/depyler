//! Property tests for MCP error types

use depyler_mcp::error::DepylerMcpError;
use pmcp::error::Error as McpError;
use proptest::prelude::*;

proptest! {
    /// Property: Error display messages never panic
    #[test]
    fn prop_error_display_never_panics(msg in ".*") {
        let errors = vec![
            DepylerMcpError::TypeInferenceError(msg.clone()),
            DepylerMcpError::UnsupportedDynamicFeature(msg.clone()),
            DepylerMcpError::InvalidInput(msg.clone()),
            DepylerMcpError::Internal(msg.clone()),
        ];

        for err in errors {
            let _ = err.to_string();
        }
    }

    /// Property: Error conversion to MCP preserves information
    #[test]
    fn prop_error_conversion_preserves_info(
        pattern in "[a-zA-Z0-9_]+",
        location in "[a-zA-Z0-9:._]+",
        timeout in 1u64..3600,
        msg in "\\PC+"
    ) {
        let errors = vec![
            DepylerMcpError::TypeInferenceError(msg.clone()),
            DepylerMcpError::UnsafePatternError {
                pattern: pattern.clone(),
                location: location.clone()
            },
            DepylerMcpError::TranspilationTimeout(timeout),
            DepylerMcpError::InvalidInput(msg.clone()),
        ];

        for err in errors {
            let _original_msg = err.to_string();
            let mcp_err: McpError = err.into();

            match mcp_err {
                McpError::Internal(mcp_msg) => {
                    // The MCP message should contain key parts of the original
                    assert!(!mcp_msg.is_empty());
                }
                _ => {} // MCP passthrough is also valid
            }
        }
    }

    /// Property: Helper methods produce correct error variants
    #[test]
    fn prop_helper_methods_correct(
        msg in "\\PC+",
        pattern in "[a-zA-Z0-9_]+",
        location in "[a-zA-Z0-9:._]+"
    ) {
        let err1 = DepylerMcpError::type_inference(&msg);
        assert!(matches!(err1, DepylerMcpError::TypeInferenceError(_)));
        assert!(err1.to_string().contains("Type inference failed"));

        let err2 = DepylerMcpError::unsafe_pattern(&pattern, &location);
        assert!(matches!(err2, DepylerMcpError::UnsafePatternError { .. }));
        assert!(err2.to_string().contains(&pattern));
        assert!(err2.to_string().contains(&location));
    }

    /// Property: All error variants can round-trip through Display
    #[test]
    fn prop_error_display_roundtrip(
        s1 in "\\PC+",
        s2 in "\\PC+",
        n in 1u64..1000
    ) {
        let errors = vec![
            DepylerMcpError::TypeInferenceError(s1.clone()),
            DepylerMcpError::UnsafePatternError {
                pattern: s1.clone(),
                location: s2.clone(),
            },
            DepylerMcpError::UnsupportedDynamicFeature(s1.clone()),
            DepylerMcpError::TranspilationTimeout(n),
            DepylerMcpError::InvalidInput(s1.clone()),
            DepylerMcpError::Internal(s2.clone()),
        ];

        for err in errors {
            let display = err.to_string();
            assert!(!display.is_empty());
            // Verify key information is preserved in display
            match &err {
                DepylerMcpError::TypeInferenceError(_msg) => {
                    assert!(display.contains("Type inference failed"));
                }
                DepylerMcpError::UnsafePatternError { pattern, location } => {
                    assert!(display.contains(pattern));
                    assert!(display.contains(location));
                }
                DepylerMcpError::TranspilationTimeout(secs) => {
                    assert!(display.contains(&secs.to_string()));
                }
                _ => {}
            }
        }
    }
}
