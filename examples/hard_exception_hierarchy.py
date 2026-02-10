"""Hard exception patterns: custom hierarchies, chaining, groups, multiple except, finally with return."""

from typing import List, Dict, Optional, Tuple, Union


# --- Custom exception hierarchy ---

class AppError(Exception):
    """Base application error."""

    def __init__(self, message: str, code: int = 0) -> None:
        super().__init__(message)
        self.code = code
        self.context: Dict[str, str] = {}

    def with_context(self, key: str, value: str) -> "AppError":
        self.context[key] = value
        return self

    def full_message(self) -> str:
        base = f"[E{self.code}] {str(self)}"
        if self.context:
            ctx = ", ".join(f"{k}={v}" for k, v in self.context.items())
            return f"{base} ({ctx})"
        return base


class ValidationError(AppError):
    """Validation error with field information."""

    def __init__(self, field: str, message: str) -> None:
        super().__init__(f"Validation failed for '{field}': {message}", code=400)
        self.field = field

    def to_dict(self) -> Dict[str, str]:
        return {"field": self.field, "message": str(self), "code": str(self.code)}


class NotFoundError(AppError):
    """Resource not found error."""

    def __init__(self, resource_type: str, resource_id: str) -> None:
        super().__init__(f"{resource_type} '{resource_id}' not found", code=404)
        self.resource_type = resource_type
        self.resource_id = resource_id


class AuthenticationError(AppError):
    """Authentication failure."""

    def __init__(self, reason: str) -> None:
        super().__init__(f"Authentication failed: {reason}", code=401)
        self.reason = reason


class AuthorizationError(AppError):
    """Authorization failure."""

    def __init__(self, action: str, resource: str) -> None:
        super().__init__(f"Not authorized to {action} on {resource}", code=403)
        self.action = action
        self.resource = resource


class RateLimitError(AppError):
    """Rate limit exceeded."""

    def __init__(self, limit: int, window_seconds: int) -> None:
        super().__init__(
            f"Rate limit exceeded: {limit} requests per {window_seconds}s",
            code=429,
        )
        self.limit = limit
        self.window_seconds = window_seconds

    def retry_after(self) -> int:
        return self.window_seconds


class DatabaseError(AppError):
    """Database operation failure."""

    def __init__(self, operation: str, detail: str) -> None:
        super().__init__(f"Database error during {operation}: {detail}", code=500)
        self.operation = operation
        self.detail = detail


class RetryableError(AppError):
    """Error that can be retried."""

    def __init__(self, message: str, max_retries: int = 3) -> None:
        super().__init__(message, code=503)
        self.max_retries = max_retries
        self.attempts = 0

    def should_retry(self) -> bool:
        return self.attempts < self.max_retries

    def record_attempt(self) -> None:
        self.attempts += 1


class ErrorCollector:
    """Collects multiple errors during processing."""

    def __init__(self) -> None:
        self._errors: List[AppError] = []
        self._warnings: List[str] = []

    def add_error(self, error: AppError) -> None:
        self._errors.append(error)

    def add_warning(self, message: str) -> None:
        self._warnings.append(message)

    @property
    def has_errors(self) -> bool:
        return len(self._errors) > 0

    @property
    def error_count(self) -> int:
        return len(self._errors)

    @property
    def warning_count(self) -> int:
        return len(self._warnings)

    def errors_by_code(self) -> Dict[int, List[str]]:
        result: Dict[int, List[str]] = {}
        for err in self._errors:
            if err.code not in result:
                result[err.code] = []
            result[err.code].append(str(err))
        return result

    def summary(self) -> str:
        lines = [f"Errors: {self.error_count}, Warnings: {self.warning_count}"]
        for err in self._errors:
            lines.append(f"  [{err.code}] {err}")
        for w in self._warnings:
            lines.append(f"  [WARN] {w}")
        return "\n".join(lines)


# --- Exception chaining (from e) ---

def parse_config_value(raw: str) -> int:
    """Parse a config value, chaining any ValueError."""
    try:
        return int(raw)
    except ValueError as e:
        raise ValidationError("config_value", f"Expected integer, got '{raw}'") from e


def load_user_by_id(user_id: str, db: Dict[str, Dict[str, str]]) -> Dict[str, str]:
    """Load a user, chaining KeyError as NotFoundError."""
    try:
        return db[user_id]
    except KeyError as e:
        raise NotFoundError("User", user_id) from e


def safe_divide(a: float, b: float) -> float:
    """Division with chained exception on zero."""
    try:
        return a / b
    except ZeroDivisionError as e:
        raise ValidationError("divisor", "Cannot divide by zero") from e


# --- Multiple except blocks ---

def resilient_parse(data: str) -> Union[int, float, str]:
    """Try multiple parse strategies with different exception handlers."""
    try:
        return int(data)
    except ValueError:
        pass

    try:
        return float(data)
    except ValueError:
        pass

    return data


def process_record(record: Dict[str, str]) -> Dict[str, Union[int, float, str]]:
    """Process a record with multiple exception handling paths."""
    result: Dict[str, Union[int, float, str]] = {}
    for key, value in record.items():
        try:
            result[key] = int(value)
        except ValueError:
            try:
                result[key] = float(value)
            except ValueError:
                result[key] = value
    return result


def multi_except_handler(operation: str, value: str) -> str:
    """Handler with multiple except clauses catching different error types."""
    try:
        if operation == "parse_int":
            n = int(value)
            return f"parsed: {n}"
        elif operation == "parse_float":
            f = float(value)
            return f"parsed: {f:.4f}"
        elif operation == "index":
            items = value.split(",")
            return items[10]  # Likely IndexError
        elif operation == "lookup":
            d: Dict[str, int] = {"a": 1, "b": 2}
            return str(d[value])  # Likely KeyError
        elif operation == "divide":
            parts = value.split("/")
            a, b = int(parts[0]), int(parts[1])
            return str(a // b)
        else:
            return "unknown operation"
    except ValueError as e:
        return f"ValueError: {e}"
    except IndexError:
        return "IndexError: index out of range"
    except KeyError as e:
        return f"KeyError: {e}"
    except ZeroDivisionError:
        return "ZeroDivisionError: division by zero"


# --- Finally with complex control flow ---

def resource_with_finally(name: str, should_fail: bool) -> Tuple[str, bool]:
    """Simulate resource acquisition with finally cleanup."""
    acquired = False
    result = "none"
    try:
        acquired = True
        if should_fail:
            raise DatabaseError("open", f"Cannot open resource '{name}'")
        result = f"success:{name}"
    except DatabaseError:
        result = f"failed:{name}"
    finally:
        if acquired:
            result = result + ":cleaned"
    return (result, acquired)


def nested_try_finally(depth: int, fail_at: int) -> List[str]:
    """Nested try/finally blocks tracking execution order."""
    log: List[str] = []

    try:
        log.append("outer_try")
        try:
            log.append("middle_try")
            try:
                log.append("inner_try")
                if fail_at == 3:
                    raise ValueError("fail at inner")
            except ValueError as e:
                log.append(f"inner_except:{e}")
            finally:
                log.append("inner_finally")

            if fail_at == 2:
                raise ValueError("fail at middle")
        except ValueError as e:
            log.append(f"middle_except:{e}")
        finally:
            log.append("middle_finally")

        if fail_at == 1:
            raise ValueError("fail at outer")
    except ValueError as e:
        log.append(f"outer_except:{e}")
    finally:
        log.append("outer_finally")

    return log


def try_else_finally(values: List[int], index: int) -> Dict[str, str]:
    """Try/except/else/finally with tracking."""
    status: Dict[str, str] = {}
    try:
        value = values[index]
        status["try"] = "success"
    except IndexError:
        status["try"] = "index_error"
        value = -1
    else:
        status["else"] = f"value={value}"
    finally:
        status["finally"] = "executed"
        status["value"] = str(value)
    return status


# --- Retry pattern with exceptions ---

def retry_operation(
    func_name: str, max_attempts: int, fail_until: int
) -> Tuple[bool, int, List[str]]:
    """Retry an operation that fails for the first N attempts."""
    log: List[str] = []
    attempt = 0

    while attempt < max_attempts:
        attempt += 1
        try:
            if attempt <= fail_until:
                raise RetryableError(f"Attempt {attempt} failed for {func_name}")
            log.append(f"success on attempt {attempt}")
            return (True, attempt, log)
        except RetryableError as e:
            e.record_attempt()
            log.append(f"retry {attempt}: {e}")
            if not e.should_retry():
                log.append("max retries exhausted")
                return (False, attempt, log)

    return (False, attempt, log)


# --- Error collection and batch validation ---

def validate_user_data(data: Dict[str, str]) -> ErrorCollector:
    """Validate user data, collecting all errors."""
    collector = ErrorCollector()

    # Validate name
    name = data.get("name", "")
    if not name:
        collector.add_error(ValidationError("name", "Name is required"))
    elif len(name) < 2:
        collector.add_error(ValidationError("name", "Name too short"))

    # Validate age
    age_str = data.get("age", "")
    if not age_str:
        collector.add_error(ValidationError("age", "Age is required"))
    else:
        try:
            age = int(age_str)
            if age < 0 or age > 150:
                collector.add_error(ValidationError("age", f"Invalid age: {age}"))
        except ValueError:
            collector.add_error(ValidationError("age", f"Not a number: {age_str}"))

    # Validate email
    email = data.get("email", "")
    if email and "@" not in email:
        collector.add_error(ValidationError("email", "Invalid email format"))
    elif not email:
        collector.add_warning("Email not provided")

    return collector


def batch_process(
    items: List[Dict[str, str]],
) -> Tuple[List[Dict[str, str]], ErrorCollector]:
    """Process multiple items, collecting errors without stopping."""
    collector = ErrorCollector()
    results: List[Dict[str, str]] = []

    for i, item in enumerate(items):
        try:
            processed = process_record(item)
            results.append({k: str(v) for k, v in processed.items()})
        except Exception as e:
            collector.add_error(
                AppError(f"Failed to process item {i}: {e}", code=500)
            )

    return (results, collector)


# --- Error classification and handling ---

def classify_error(error: AppError) -> str:
    """Classify an error using isinstance chain."""
    if isinstance(error, ValidationError):
        return f"validation:{error.field}"
    elif isinstance(error, NotFoundError):
        return f"not_found:{error.resource_type}"
    elif isinstance(error, AuthenticationError):
        return f"auth:{error.reason}"
    elif isinstance(error, AuthorizationError):
        return f"authz:{error.action}"
    elif isinstance(error, RateLimitError):
        return f"rate_limit:{error.limit}"
    elif isinstance(error, DatabaseError):
        return f"db:{error.operation}"
    elif isinstance(error, RetryableError):
        return f"retryable:{error.attempts}/{error.max_retries}"
    return f"unknown:{error.code}"


def error_to_http_response(error: AppError) -> Dict[str, Union[int, str]]:
    """Convert an error to an HTTP-like response dict."""
    return {
        "status": error.code,
        "error": type(error).__name__,
        "message": str(error),
        "detail": error.full_message(),
    }


# Untyped function 1: test inference on exception handling patterns
def safe_batch_convert(values):
    results = []
    errors = []
    for i, val in enumerate(values):
        try:
            if isinstance(val, str):
                results.append(int(val))
            elif isinstance(val, float):
                results.append(int(val))
            else:
                results.append(val)
        except (ValueError, TypeError) as e:
            errors.append((i, str(e)))
    return results, errors


# Untyped function 2: test inference on nested exception flow
def cascading_fallback(primary, secondary, default):
    try:
        result = primary()
        if result is None:
            raise ValueError("primary returned None")
        return result
    except Exception:
        try:
            result = secondary()
            if result is None:
                raise ValueError("secondary returned None")
            return result
        except Exception:
            return default


def main() -> None:
    # Test custom exception hierarchy
    err = ValidationError("email", "invalid format")
    assert err.code == 400
    assert err.field == "email"
    d = err.to_dict()
    assert d["field"] == "email"

    nf = NotFoundError("User", "123")
    assert nf.code == 404
    msg = nf.full_message()
    assert "404" in msg

    # Test exception context
    ae = AppError("test", 500).with_context("request_id", "abc123")
    assert "abc123" in ae.full_message()

    # Test exception chaining
    try:
        parse_config_value("not_a_number")
        assert False, "Should have raised"
    except ValidationError as e:
        assert e.__cause__ is not None
        assert isinstance(e.__cause__, ValueError)

    # Test safe_divide chaining
    try:
        safe_divide(10.0, 0.0)
        assert False, "Should have raised"
    except ValidationError as e:
        assert "zero" in str(e).lower()
        assert isinstance(e.__cause__, ZeroDivisionError)

    # Test resilient parse
    assert resilient_parse("42") == 42
    assert resilient_parse("3.14") == 3.14
    assert resilient_parse("hello") == "hello"

    # Test multi_except_handler
    assert "parsed: 42" == multi_except_handler("parse_int", "42")
    assert "ValueError" in multi_except_handler("parse_int", "abc")
    assert "IndexError" in multi_except_handler("index", "a,b,c")
    assert "KeyError" in multi_except_handler("lookup", "z")
    assert "ZeroDivision" in multi_except_handler("divide", "10/0")

    # Test resource_with_finally
    result, acquired = resource_with_finally("db", False)
    assert "success" in result and "cleaned" in result
    assert acquired

    result2, acquired2 = resource_with_finally("db", True)
    assert "failed" in result2 and "cleaned" in result2

    # Test nested_try_finally
    log = nested_try_finally(3, 3)
    assert "inner_finally" in log
    assert "middle_finally" in log
    assert "outer_finally" in log

    log2 = nested_try_finally(3, 2)
    assert "middle_except:fail at middle" in log2

    # Test try_else_finally
    status = try_else_finally([10, 20, 30], 1)
    assert status["try"] == "success"
    assert status["else"] == "value=20"
    assert status["finally"] == "executed"

    status2 = try_else_finally([10], 5)
    assert status2["try"] == "index_error"
    assert "else" not in status2

    # Test retry pattern
    success, attempts, log = retry_operation("test_op", 5, 2)
    assert success
    assert attempts == 3

    fail_success, fail_attempts, fail_log = retry_operation("test_op", 2, 5)
    assert not fail_success

    # Test error collection
    collector = validate_user_data({"name": "", "age": "abc", "email": "bad"})
    assert collector.has_errors
    assert collector.error_count >= 2

    good_collector = validate_user_data({"name": "Alice", "age": "30", "email": "a@b.com"})
    assert not good_collector.has_errors

    # Test batch_process
    items = [
        {"x": "1", "y": "2.5"},
        {"a": "hello", "b": "3"},
    ]
    results, batch_errors = batch_process(items)
    assert len(results) == 2

    # Test error classification
    assert classify_error(ValidationError("x", "bad")).startswith("validation:")
    assert classify_error(NotFoundError("User", "1")).startswith("not_found:")
    assert classify_error(RateLimitError(100, 60)).startswith("rate_limit:")

    # Test error_to_http_response
    resp = error_to_http_response(NotFoundError("Item", "42"))
    assert resp["status"] == 404

    # Test untyped functions
    converted, errs = safe_batch_convert(["1", "2", "bad", 4.5])
    assert len(converted) == 3
    assert len(errs) == 1

    result = cascading_fallback(
        lambda: None,
        lambda: 42,
        0,
    )
    assert result == 42

    result2 = cascading_fallback(
        lambda: None,
        lambda: None,
        99,
    )
    assert result2 == 99

    # Test ErrorCollector summary
    ec = ErrorCollector()
    ec.add_error(ValidationError("name", "required"))
    ec.add_error(NotFoundError("Role", "admin"))
    ec.add_warning("Some warning")
    summary = ec.summary()
    assert "Errors: 2" in summary
    assert "Warnings: 1" in summary

    by_code = ec.errors_by_code()
    assert 400 in by_code
    assert 404 in by_code


if __name__ == "__main__":
    main()
