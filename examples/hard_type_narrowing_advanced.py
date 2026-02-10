"""Hard type narrowing: isinstance chains, TypeGuard, Literal, overloaded functions, Union narrowing."""

from typing import (
    Union,
    Optional,
    List,
    Dict,
    Tuple,
    Literal,
    Callable,
    TypeVar,
    overload,
)

T = TypeVar("T")

# --- Union types for narrowing ---

JsonValue = Union[str, int, float, bool, None, List["JsonValue"], Dict[str, "JsonValue"]]
Numeric = Union[int, float]
StringOrBytes = Union[str, bytes]
MaybeList = Union[int, List[int]]


class ParseError:
    """Error type for parsing failures."""

    def __init__(self, message: str, position: int) -> None:
        self.message = message
        self.position = position

    def describe(self) -> str:
        return f"ParseError at {self.position}: {self.message}"


class ParsedInt:
    """Successful integer parse result."""

    def __init__(self, value: int, consumed: int) -> None:
        self.value = value
        self.consumed = consumed


class ParsedFloat:
    """Successful float parse result."""

    def __init__(self, value: float, consumed: int) -> None:
        self.value = value
        self.consumed = consumed


class ParsedString:
    """Successful string parse result."""

    def __init__(self, value: str, consumed: int) -> None:
        self.value = value
        self.consumed = consumed


ParseResult = Union[ParsedInt, ParsedFloat, ParsedString, ParseError]


def is_numeric(value: object) -> bool:
    """TypeGuard-style check for numeric values."""
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def narrow_parse_result(result: ParseResult) -> str:
    """Narrow a union of parse results using isinstance chains."""
    if isinstance(result, ParsedInt):
        return f"int({result.value}), consumed {result.consumed} chars"
    elif isinstance(result, ParsedFloat):
        return f"float({result.value:.4f}), consumed {result.consumed} chars"
    elif isinstance(result, ParsedString):
        return f"string({result.value!r}), consumed {result.consumed} chars"
    elif isinstance(result, ParseError):
        return result.describe()
    return "unknown"


def coerce_to_float(value: Numeric) -> float:
    """Narrow Union[int, float] to float."""
    if isinstance(value, int):
        return float(value)
    return value


def process_string_or_bytes(data: StringOrBytes) -> str:
    """Narrow Union[str, bytes] based on type."""
    if isinstance(data, bytes):
        return data.decode("utf-8")
    return data


def normalize_maybe_list(value: MaybeList) -> List[int]:
    """Narrow Union[int, List[int]] to List[int]."""
    if isinstance(value, int):
        return [value]
    return list(value)


def deep_isinstance_chain(value: object) -> str:
    """Multi-level isinstance narrowing chain."""
    if isinstance(value, bool):
        return f"bool:{value}"
    elif isinstance(value, int):
        if value < 0:
            return f"negative_int:{value}"
        elif value == 0:
            return "zero"
        elif value <= 100:
            return f"small_int:{value}"
        else:
            return f"large_int:{value}"
    elif isinstance(value, float):
        if value != value:  # NaN check
            return "nan"
        elif value == float("inf"):
            return "positive_infinity"
        elif value == float("-inf"):
            return "negative_infinity"
        else:
            return f"float:{value:.4f}"
    elif isinstance(value, str):
        if len(value) == 0:
            return "empty_string"
        elif len(value) == 1:
            return f"char:{value}"
        elif value.isdigit():
            return f"numeric_string:{value}"
        else:
            return f"string:{value}"
    elif isinstance(value, list):
        return f"list[{len(value)}]"
    elif isinstance(value, dict):
        return f"dict[{len(value)}]"
    elif isinstance(value, tuple):
        return f"tuple[{len(value)}]"
    elif value is None:
        return "none"
    return "object"


# --- Literal type patterns ---

Color = Literal["red", "green", "blue", "yellow"]
LogLevel = Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]


def color_to_rgb(color: Color) -> Tuple[int, int, int]:
    """Map literal color names to RGB tuples."""
    if color == "red":
        return (255, 0, 0)
    elif color == "green":
        return (0, 255, 0)
    elif color == "blue":
        return (0, 0, 255)
    elif color == "yellow":
        return (255, 255, 0)
    return (0, 0, 0)


def log_level_severity(level: LogLevel) -> int:
    """Map literal log levels to numeric severity."""
    severity_map: Dict[str, int] = {
        "DEBUG": 10,
        "INFO": 20,
        "WARNING": 30,
        "ERROR": 40,
        "CRITICAL": 50,
    }
    return severity_map.get(level, 0)


def should_alert(level: LogLevel) -> bool:
    """Determine if a log level should trigger an alert."""
    return level in ("ERROR", "CRITICAL")


# --- Overloaded function patterns ---

@overload
def parse_value(text: str, as_type: Literal["int"]) -> int: ...

@overload
def parse_value(text: str, as_type: Literal["float"]) -> float: ...

@overload
def parse_value(text: str, as_type: Literal["bool"]) -> bool: ...

def parse_value(text: str, as_type: str) -> Union[int, float, bool]:
    """Parse a string value based on the requested type."""
    if as_type == "int":
        return int(text)
    elif as_type == "float":
        return float(text)
    elif as_type == "bool":
        return text.lower() in ("true", "1", "yes")
    raise ValueError(f"Unknown type: {as_type}")


@overload
def first_element(items: List[int]) -> int: ...

@overload
def first_element(items: List[str]) -> str: ...

def first_element(items: Union[List[int], List[str]]) -> Union[int, str]:
    """Get the first element with type-specific handling."""
    if not items:
        raise ValueError("Empty list")
    return items[0]


# --- Complex Union narrowing in if/elif ---

class TokenLiteral:
    """Literal token (number or string)."""
    def __init__(self, value: Union[int, float, str]) -> None:
        self.value = value

class TokenOperator:
    """Operator token."""
    def __init__(self, op: str) -> None:
        self.op = op

class TokenKeyword:
    """Keyword token."""
    def __init__(self, keyword: str) -> None:
        self.keyword = keyword

class TokenEOF:
    """End-of-file token."""
    pass

ExprToken = Union[TokenLiteral, TokenOperator, TokenKeyword, TokenEOF]


def format_token(token: ExprToken) -> str:
    """Format a token for display, narrowing the union."""
    if isinstance(token, TokenLiteral):
        if isinstance(token.value, int):
            return f"INT({token.value})"
        elif isinstance(token.value, float):
            return f"FLOAT({token.value:.4f})"
        elif isinstance(token.value, str):
            return f"STR({token.value!r})"
        return "LITERAL(?)"
    elif isinstance(token, TokenOperator):
        return f"OP({token.op})"
    elif isinstance(token, TokenKeyword):
        return f"KW({token.keyword})"
    elif isinstance(token, TokenEOF):
        return "EOF"
    return "UNKNOWN"


def evaluate_simple_expr(tokens: List[ExprToken]) -> Optional[float]:
    """Evaluate a simple expression by narrowing token types."""
    values: List[float] = []
    current_op: Optional[str] = None

    for token in tokens:
        if isinstance(token, TokenLiteral):
            val: float
            if isinstance(token.value, (int, float)):
                val = float(token.value)
            else:
                return None  # Can't evaluate string literals

            if current_op is None:
                values.append(val)
            elif current_op == "+":
                prev = values.pop() if values else 0.0
                values.append(prev + val)
                current_op = None
            elif current_op == "-":
                prev = values.pop() if values else 0.0
                values.append(prev - val)
                current_op = None
            elif current_op == "*":
                prev = values.pop() if values else 1.0
                values.append(prev * val)
                current_op = None
            elif current_op == "/":
                prev = values.pop() if values else 0.0
                if val != 0.0:
                    values.append(prev / val)
                else:
                    return None
                current_op = None
        elif isinstance(token, TokenOperator):
            current_op = token.op
        elif isinstance(token, TokenEOF):
            break

    return values[-1] if values else None


def optional_chain(
    value: Optional[str],
    transforms: List[Callable[[str], Optional[str]]],
) -> Optional[str]:
    """Apply a chain of transforms, short-circuiting on None."""
    current = value
    for transform in transforms:
        if current is None:
            return None
        current = transform(current)
    return current


def safe_index(items: List[T], index: int) -> Optional[T]:
    """Safe list indexing returning Optional[T]."""
    if 0 <= index < len(items):
        return items[index]
    return None


def classify_json_value(value: object) -> str:
    """Classify a JSON-like value through type narrowing."""
    if value is None:
        return "null"
    elif isinstance(value, bool):
        return "boolean"
    elif isinstance(value, int):
        return "integer"
    elif isinstance(value, float):
        return "number"
    elif isinstance(value, str):
        return "string"
    elif isinstance(value, list):
        if len(value) == 0:
            return "empty_array"
        types = {classify_json_value(item) for item in value}
        if len(types) == 1:
            return f"array<{types.pop()}>"
        return f"array<mixed:{len(types)}>"
    elif isinstance(value, dict):
        return f"object<{len(value)} keys>"
    return "unknown"


# Untyped function 1: test inference on union narrowing
def auto_convert(value):
    if isinstance(value, str):
        stripped = value.strip()
        if stripped.isdigit():
            return int(stripped)
        try:
            return float(stripped)
        except ValueError:
            return stripped
    elif isinstance(value, bool):
        return 1 if value else 0
    elif isinstance(value, (int, float)):
        return value
    elif isinstance(value, list):
        return [auto_convert(item) for item in value]
    elif isinstance(value, dict):
        return {str(k): auto_convert(v) for k, v in value.items()}
    return str(value)


# Untyped function 2: test inference on complex conditional narrowing
def smart_merge(left, right):
    if isinstance(left, dict) and isinstance(right, dict):
        result = dict(left)
        for k, v in right.items():
            if k in result:
                result[k] = smart_merge(result[k], v)
            else:
                result[k] = v
        return result
    elif isinstance(left, list) and isinstance(right, list):
        return left + right
    elif isinstance(left, str) and isinstance(right, str):
        return left + " " + right
    elif isinstance(left, (int, float)) and isinstance(right, (int, float)):
        return left + right
    return right


class TypeNarrowingValidator:
    """Class that validates values using type narrowing."""

    def __init__(self) -> None:
        self._rules: Dict[str, Callable[[object], bool]] = {}
        self._errors: List[str] = []

    def add_rule(self, name: str, rule: Callable[[object], bool]) -> None:
        self._rules[name] = rule

    def validate(self, value: object) -> Tuple[bool, List[str]]:
        self._errors = []
        for name, rule in self._rules.items():
            if not rule(value):
                self._errors.append(f"Rule '{name}' failed")
        return (len(self._errors) == 0, list(self._errors))

    def error_count(self) -> int:
        return len(self._errors)


def main() -> None:
    # Test isinstance narrowing
    assert narrow_parse_result(ParsedInt(42, 2)) == "int(42), consumed 2 chars"
    assert narrow_parse_result(ParsedFloat(3.14, 4)) == "float(3.1400), consumed 4 chars"
    assert narrow_parse_result(ParseError("bad", 0)) == "ParseError at 0: bad"

    # Test coerce_to_float
    assert coerce_to_float(42) == 42.0
    assert coerce_to_float(3.14) == 3.14

    # Test process_string_or_bytes
    assert process_string_or_bytes("hello") == "hello"
    assert process_string_or_bytes(b"world") == "world"

    # Test normalize_maybe_list
    assert normalize_maybe_list(5) == [5]
    assert normalize_maybe_list([1, 2, 3]) == [1, 2, 3]

    # Test deep isinstance chain
    assert deep_isinstance_chain(True) == "bool:True"
    assert deep_isinstance_chain(-5) == "negative_int:-5"
    assert deep_isinstance_chain(0) == "zero"
    assert deep_isinstance_chain(50) == "small_int:50"
    assert deep_isinstance_chain("") == "empty_string"
    assert deep_isinstance_chain("a") == "char:a"
    assert deep_isinstance_chain(None) == "none"

    # Test Literal type functions
    assert color_to_rgb("red") == (255, 0, 0)
    assert color_to_rgb("blue") == (0, 0, 255)
    assert log_level_severity("ERROR") == 40
    assert should_alert("CRITICAL")
    assert not should_alert("INFO")

    # Test overloaded parse_value
    assert parse_value("42", "int") == 42
    assert abs(parse_value("3.14", "float") - 3.14) < 0.01
    assert parse_value("yes", "bool") is True

    # Test token formatting with union narrowing
    assert format_token(TokenLiteral(42)) == "INT(42)"
    assert format_token(TokenOperator("+")) == "OP(+)"
    assert format_token(TokenEOF()) == "EOF"

    # Test expression evaluation
    tokens: List[ExprToken] = [
        TokenLiteral(3),
        TokenOperator("+"),
        TokenLiteral(4),
        TokenOperator("*"),
        TokenLiteral(2),
        TokenEOF(),
    ]
    result = evaluate_simple_expr(tokens)
    assert result is not None
    assert abs(result - 14.0) < 0.01

    # Test optional chain
    result_chain = optional_chain("  hello  ", [str.strip, str.upper])
    assert result_chain == "HELLO"
    assert optional_chain(None, [str.strip]) is None

    # Test classify_json_value
    assert classify_json_value(None) == "null"
    assert classify_json_value(42) == "integer"
    assert classify_json_value("hi") == "string"
    assert classify_json_value([1, 2, 3]) == "array<integer>"
    assert classify_json_value({"a": 1}) == "object<1 keys>"

    # Test untyped functions
    assert auto_convert("42") == 42
    assert auto_convert("3.14") == 3.14
    assert auto_convert("hello") == "hello"
    assert auto_convert([1, "2"]) == [1, 2]

    merged = smart_merge({"a": 1}, {"b": 2})
    assert merged == {"a": 1, "b": 2}
    assert smart_merge([1, 2], [3, 4]) == [1, 2, 3, 4]
    assert smart_merge("hello", "world") == "hello world"

    # Test TypeNarrowingValidator
    validator = TypeNarrowingValidator()
    validator.add_rule("positive", lambda x: isinstance(x, (int, float)) and x > 0)
    validator.add_rule("small", lambda x: isinstance(x, (int, float)) and x < 100)
    ok, errors = validator.validate(50)
    assert ok
    assert len(errors) == 0

    ok2, errors2 = validator.validate(-5)
    assert not ok2
    assert len(errors2) == 1


if __name__ == "__main__":
    main()
