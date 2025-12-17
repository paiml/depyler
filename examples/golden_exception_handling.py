"""Golden Exception Handling Example - DEPYLER-0983

A fully type-annotated Python file specifically designed to test
exception handling transformations (try/except → Result).

Purpose: Falsify hypothesis that exception handling codegen is sound.
Method: Every function has explicit type annotations, isolating
exception-specific bugs from type inference issues.

Exception patterns tested:
1. Simple try/except with early return
2. try/except with exception variable binding
3. try/except/finally (resource cleanup)
4. try/except/else pattern (full suite)
5. Multiple exception handlers
6. Nested try/except blocks
7. Exception re-raising patterns
8. Custom Exception classes
9. raise statements with and without arguments
10. Return value propagation through Result

Expected Rust transformations:
- try/except → match on Result/closure
- Exception types → custom error structs or Box<dyn Error>
- finally → defer-like cleanup (drop guards or explicit)
- else → executed when no exception occurs
- raise → Err(...) or panic!
- Custom exceptions → custom error types with Display/Error traits
"""

from typing import Optional, Dict, List


# =============================================================================
# Custom Exception Classes (Pattern 8)
# =============================================================================

class ValidationError(Exception):
    """Custom exception for validation failures.

    Rust: struct ValidationError { message: String }
    impl std::error::Error for ValidationError {}
    """
    def __init__(self, message: str) -> None:
        self.message: str = message
        super().__init__(message)


class RangeError(Exception):
    """Custom exception for out-of-range values.

    Rust: struct RangeError { value: i64, min_val: i64, max_val: i64 }
    """
    def __init__(self, value: int, min_val: int, max_val: int) -> None:
        self.value: int = value
        self.min_val: int = min_val
        self.max_val: int = max_val
        super().__init__(f"Value {value} not in range [{min_val}, {max_val}]")


def parse_int_safe(s: str) -> int:
    """Simple try/except with fallback return.

    Python: try/except ValueError → return default
    Rust: s.parse::<i64>().unwrap_or(0)
    """
    try:
        return int(s)
    except ValueError:
        return 0


def parse_int_with_error(s: str) -> Optional[int]:
    """try/except returning Optional.

    Python: try/except → None
    Rust: s.parse::<i64>().ok()
    """
    try:
        return int(s)
    except ValueError:
        return None


def divide_safe(a: int, b: int) -> int:
    """try/except with ZeroDivisionError.

    Python: try/except ZeroDivisionError
    Rust: if b == 0 { 0 } else { a / b }
    """
    try:
        return a // b
    except ZeroDivisionError:
        return 0


def get_with_key_error(d: Dict[str, int], key: str) -> int:
    """try/except with KeyError.

    Python: try d[key] except KeyError
    Rust: d.get(&key).copied().unwrap_or(-1)
    """
    try:
        return d[key]
    except KeyError:
        return -1


def get_with_bound_exception(d: Dict[str, int], key: str) -> str:
    """try/except with exception variable binding.

    Python: except KeyError as e → use e
    Rust: Err(e) => format!("Error: {}", e)
    """
    try:
        value: int = d[key]
        return str(value)
    except KeyError as e:
        return f"Missing key: {e}"


def multiple_handlers(s: str, d: Dict[str, int]) -> int:
    """Multiple exception type handlers.

    Python: except ValueError, except KeyError
    Rust: match with multiple Err patterns
    """
    try:
        num: int = int(s)
        return d[str(num)]
    except ValueError:
        return -1
    except KeyError:
        return -2


def nested_try_except(x: int) -> int:
    """Nested try/except blocks.

    Python: outer try wrapping inner try
    Rust: nested match expressions
    """
    outer: int = 0
    inner: int = 0
    try:
        outer = x + 1
        try:
            inner = outer * 2
            if inner > 100:
                raise ValueError("Too large")
            return inner
        except ValueError:
            return outer
    except Exception:
        return 0


def try_except_finally_pattern(filename: str) -> str:
    """try/except/finally for resource cleanup.

    Python: finally block always executes
    Rust: Drop guard or explicit cleanup

    Note: This tests cleanup semantics, not file I/O.
    """
    result: str = ""
    opened: bool = False
    try:
        opened = True
        result = f"Processing {filename}"
        if filename == "":
            raise ValueError("Empty filename")
        return result
    except ValueError as e:
        result = f"Error: {e}"
        return result
    finally:
        # Cleanup - this should always execute
        if opened:
            pass  # Close would happen here


def propagate_result(values: List[str]) -> int:
    """Exception propagation through multiple operations.

    Python: Multiple operations that can fail
    Rust: ? operator or explicit Result handling
    """
    total: int = 0
    try:
        for v in values:
            num: int = int(v)
            total += num
        return total
    except ValueError:
        return -1


def early_return_in_try(x: int) -> int:
    """Early return within try block.

    Python: return before try block ends
    Rust: Ok(value) propagation
    """
    try:
        if x < 0:
            return -1
        result: int = x * 2
        if result > 100:
            return 100
        return result
    except Exception:
        return 0


def exception_with_computation(a: int, b: int, c: int) -> int:
    """Complex computation in try with multiple failure points.

    Python: Chain of operations that can fail
    Rust: Result chain with ? or match
    """
    try:
        step1: int = a // b  # Can raise ZeroDivisionError
        step2: int = step1 * c
        if step2 < 0:
            raise ValueError("Negative result")
        return step2
    except ZeroDivisionError:
        return -1
    except ValueError:
        return -2


# =============================================================================
# try/except/else Pattern (Pattern 4)
# =============================================================================

def try_except_else(s: str) -> int:
    """Full try/except/else pattern.

    Python: else block runs when try succeeds (no exception)
    Rust: Separate success path in match arm
    """
    result: int = 0
    try:
        value: int = int(s)
    except ValueError:
        result = -1
    else:
        # This runs only if no exception occurred
        result = value * 2
    return result


def try_except_else_finally(s: str) -> str:
    """Complete try/except/else/finally suite.

    Python: Full exception handling pattern
    Rust: Complex Result handling with cleanup
    """
    status: str = "init"
    result: int = 0
    try:
        result = int(s)
        status = "parsed"
    except ValueError:
        status = "error"
        result = -1
    else:
        # Runs when try succeeds
        result = result * 10
        status = "success"
    finally:
        # Always runs
        status = f"{status}_done"
    return f"{status}:{result}"


# =============================================================================
# Raise Statement Patterns (Pattern 9)
# =============================================================================

def raise_without_args(x: int) -> int:
    """Raise without arguments (re-raise current exception).

    Python: bare raise re-raises the current exception
    Rust: return Err(e) in catch block
    """
    try:
        if x < 0:
            raise ValueError("Negative")
        return x
    except ValueError:
        # Re-raise the same exception
        raise


def raise_with_message(x: int) -> int:
    """Raise with explicit message.

    Python: raise ValueError("message")
    Rust: Err(Box::new(ValueError { message: "...".to_string() }))
    """
    if x < 0:
        raise ValueError("Value must be non-negative")
    if x > 100:
        raise ValueError("Value must be <= 100")
    return x


def raise_custom_exception(value: int, min_val: int, max_val: int) -> int:
    """Raise custom exception type.

    Python: raise RangeError(value, min, max)
    Rust: Err(Box::new(RangeError { value, min_val, max_val }))
    """
    if value < min_val or value > max_val:
        raise RangeError(value, min_val, max_val)
    return value


def raise_from_exception(s: str) -> int:
    """Exception chaining with raise ... from.

    Python: raise NewError from original_error
    Rust: Error chaining with source()
    """
    try:
        return int(s)
    except ValueError as e:
        raise ValidationError(f"Invalid input: {s}") from e


def validate_and_transform(value: int) -> int:
    """Multiple raise points in one function.

    Python: Different exceptions at different validation stages
    Rust: Multiple Err() returns with different error types
    """
    if value < 0:
        raise ValidationError("Value cannot be negative")
    if value > 1000:
        raise RangeError(value, 0, 1000)
    if value % 2 != 0:
        raise ValueError("Value must be even")
    return value // 2


# =============================================================================
# Exception Handling with Custom Exceptions
# =============================================================================

def catch_custom_exception(value: int) -> str:
    """Catch custom exception types.

    Python: except ValidationError as e
    Rust: match on specific error types via downcast
    """
    try:
        result: int = validate_and_transform(value)
        return f"Result: {result}"
    except ValidationError as e:
        return f"Validation failed: {e.message}"
    except RangeError as e:
        return f"Range error: {e.value} not in [{e.min_val}, {e.max_val}]"
    except ValueError as e:
        return f"Value error: {e}"


def main() -> int:
    """Main function exercising all exception patterns."""
    # Test parse_int_safe
    assert parse_int_safe("42") == 42
    assert parse_int_safe("invalid") == 0

    # Test parse_int_with_error
    assert parse_int_with_error("42") == 42
    assert parse_int_with_error("invalid") is None

    # Test divide_safe
    assert divide_safe(10, 2) == 5
    assert divide_safe(10, 0) == 0

    # Test get_with_key_error
    d: Dict[str, int] = {"a": 1, "b": 2}
    assert get_with_key_error(d, "a") == 1
    assert get_with_key_error(d, "missing") == -1

    # Test multiple_handlers
    assert multiple_handlers("1", {"1": 100}) == 100
    assert multiple_handlers("invalid", {"1": 100}) == -1
    assert multiple_handlers("99", {"1": 100}) == -2

    # Test nested_try_except
    assert nested_try_except(10) == 22
    assert nested_try_except(100) == 101  # inner exceeds 100

    # Test propagate_result
    assert propagate_result(["1", "2", "3"]) == 6
    assert propagate_result(["1", "invalid"]) == -1

    # Test early_return_in_try
    assert early_return_in_try(-5) == -1
    assert early_return_in_try(10) == 20
    assert early_return_in_try(100) == 100

    # Test exception_with_computation
    assert exception_with_computation(10, 2, 3) == 15
    assert exception_with_computation(10, 0, 3) == -1
    assert exception_with_computation(10, 2, -3) == -2

    # Test try_except_else
    assert try_except_else("5") == 10  # 5 * 2
    assert try_except_else("invalid") == -1

    # Test try_except_else_finally
    assert try_except_else_finally("5") == "success_done:50"
    assert try_except_else_finally("invalid") == "error_done:-1"

    # Test raise_with_message (should work for valid input)
    assert raise_with_message(50) == 50

    # Test raise_custom_exception (valid range)
    assert raise_custom_exception(50, 0, 100) == 50

    # Test validate_and_transform (valid even number)
    assert validate_and_transform(100) == 50

    # Test catch_custom_exception
    assert catch_custom_exception(100) == "Result: 50"
    assert "Validation failed" in catch_custom_exception(-1)
    assert "Range error" in catch_custom_exception(2000)
    assert "Value error" in catch_custom_exception(3)  # odd number

    return 0
