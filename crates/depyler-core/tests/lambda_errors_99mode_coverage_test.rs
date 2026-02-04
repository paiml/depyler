//! Coverage tests for lambda_errors.rs
//!
//! DEPYLER-99MODE-001: Targets lambda_errors.rs (1,617 lines)
//! Covers: exception handling patterns, error type mapping,
//! nested exception handling, multiple handler types,
//! error context propagation.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Basic exception handling
// ============================================================================

#[test]
fn test_lambda_err_keyerror() {
    let code = r#"
def handler(event: dict) -> str:
    return event["user_id"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_valueerror() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_typeerror() {
    let code = r#"
def handler(event: dict) -> int:
    x: int = event["count"]
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_indexerror() {
    let code = r#"
def handler(items: list) -> int:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Try/except with specific error types
// ============================================================================

#[test]
fn test_lambda_err_try_valueerror() {
    let code = r#"
def handler(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_try_keyerror() {
    let code = r#"
def handler(d: dict, key: str) -> str:
    try:
        return d[key]
    except KeyError:
        return "default"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_try_indexerror() {
    let code = r#"
def handler(items: list, idx: int) -> int:
    try:
        return items[idx]
    except IndexError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_try_attributeerror() {
    let code = r#"
def handler(obj: dict) -> str:
    try:
        return str(obj)
    except AttributeError:
        return "unknown"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple exception handlers
// ============================================================================

#[test]
fn test_lambda_err_multiple_handlers() {
    let code = r#"
def handler(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_three_handlers() {
    let code = r#"
def handler(data: dict, key: str) -> int:
    try:
        val = data[key]
        return int(val)
    except KeyError:
        return -1
    except ValueError:
        return -2
    except TypeError:
        return -3
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Exception with finally
// ============================================================================

#[test]
fn test_lambda_err_try_finally() {
    let code = r#"
def handler(items: list) -> int:
    result = 0
    try:
        result = items[0]
    except IndexError:
        result = -1
    finally:
        pass
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_try_else() {
    let code = r#"
def handler(s: str) -> int:
    try:
        val = int(s)
    except ValueError:
        val = 0
    else:
        val = val * 2
    return val
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested exception handling
// ============================================================================

#[test]
fn test_lambda_err_nested_try() {
    let code = r#"
def handler(data: dict) -> str:
    try:
        try:
            return data["key"]
        except KeyError:
            return "fallback"
    except TypeError:
        return "error"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Exception in loops
// ============================================================================

#[test]
fn test_lambda_err_try_in_loop() {
    let code = r#"
def handler(items: list) -> list:
    result = []
    for item in items:
        try:
            result.append(int(item))
        except ValueError:
            result.append(0)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generic exception
// ============================================================================

#[test]
fn test_lambda_err_bare_except() {
    let code = r#"
def handler(s: str) -> int:
    try:
        return int(s)
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_exception_base() {
    let code = r#"
def handler(s: str) -> int:
    try:
        return int(s)
    except Exception:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Error context patterns
// ============================================================================

#[test]
fn test_lambda_err_named_exception() {
    let code = r#"
def handler(s: str) -> str:
    try:
        val = int(s)
        return str(val)
    except ValueError as e:
        return str(e)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_reraise() {
    let code = r#"
def handler(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        raise
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex error handling patterns
// ============================================================================

#[test]
fn test_lambda_err_full_pipeline() {
    let code = r#"
def process(data: dict) -> dict:
    result = {}
    try:
        for key in data:
            try:
                result[key] = int(data[key])
            except (ValueError, TypeError):
                result[key] = 0
    except KeyError:
        pass
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_err_conditional_handling() {
    let code = r#"
def handler(items: list) -> int:
    total = 0
    for item in items:
        if isinstance(item, int):
            total += item
    return total
"#;
    assert!(transpile_ok(code));
}
