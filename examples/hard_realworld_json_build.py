"""Real-world JSON-like string building without imports.

Mimics: json.dumps, API response construction patterns.
Manual serialization of nested structures to JSON strings.
"""


def int_to_str(num: int) -> str:
    """Convert integer to string representation."""
    if num == 0:
        return "0"
    is_neg: int = 0
    val: int = num
    if val < 0:
        is_neg = 1
        val = 0 - val
    digits: str = ""
    while val > 0:
        remainder: int = val % 10
        digits = chr(ord("0") + remainder) + digits
        val = val // 10
    if is_neg == 1:
        return "-" + digits
    return digits


def escape_json_str(text: str) -> str:
    """Escape special characters for JSON string encoding."""
    result: str = ""
    idx: int = 0
    while idx < len(text):
        ch: str = text[idx]
        if ch == '"':
            result = result + '\\"'
        elif ch == "\\":
            result = result + "\\\\"
        elif ch == "\n":
            result = result + "\\n"
        elif ch == "\t":
            result = result + "\\t"
        else:
            result = result + ch
        idx = idx + 1
    return result


def wrap_json_str(value: str) -> str:
    """Wrap a value as a JSON string with quotes."""
    escaped: str = escape_json_str(value)
    return '"' + escaped + '"'


def json_number(value: int) -> str:
    """Serialize an integer as JSON number."""
    return int_to_str(value)


def json_bool_str(value: int) -> str:
    """Serialize a boolean (0/1) as JSON string."""
    if value == 1:
        return "true"
    return "false"


def json_null() -> str:
    """Return JSON null."""
    return "null"


def json_array_ints(values: list[int]) -> str:
    """Serialize a list of ints as JSON array."""
    result: str = "["
    idx: int = 0
    while idx < len(values):
        if idx > 0:
            result = result + ", "
        num_str: str = int_to_str(values[idx])
        result = result + num_str
        idx = idx + 1
    result = result + "]"
    return result


def json_array_strs(values: list[str]) -> str:
    """Serialize a list of strings as JSON array."""
    result: str = "["
    idx: int = 0
    while idx < len(values):
        if idx > 0:
            result = result + ", "
        wrapped: str = wrap_json_str(values[idx])
        result = result + wrapped
        idx = idx + 1
    result = result + "]"
    return result


def build_kv(field_name: str, field_value: str) -> str:
    """Build a JSON key-value pair (value already serialized)."""
    wrapped_name: str = wrap_json_str(field_name)
    return wrapped_name + ": " + field_value


def build_object(names: list[str], values: list[str]) -> str:
    """Build a JSON object from parallel name/value lists."""
    result: str = "{"
    idx: int = 0
    while idx < len(names):
        if idx > 0:
            result = result + ", "
        pair: str = build_kv(names[idx], values[idx])
        result = result + pair
        idx = idx + 1
    result = result + "}"
    return result


def build_user_json(username: str, age: int, active: int) -> str:
    """Build a JSON user object like an API response."""
    names: list[str] = ["username", "age", "active"]
    v_user: str = wrap_json_str(username)
    v_age: str = json_number(age)
    v_active: str = json_bool_str(active)
    values: list[str] = [v_user, v_age, v_active]
    return build_object(names, values)


def test_module() -> int:
    """Test JSON building module."""
    passed: int = 0

    # Test 1: int_to_str
    if int_to_str(42) == "42" and int_to_str(0) == "0":
        passed = passed + 1

    # Test 2: negative number
    neg_str: str = int_to_str(0 - 7)
    if neg_str == "-7":
        passed = passed + 1

    # Test 3: json string escaping
    escaped: str = wrap_json_str('hello "world"')
    if '"hello \\"world\\""' == escaped:
        passed = passed + 1

    # Test 4: json array of ints
    arr: str = json_array_ints([1, 2, 3])
    if arr == "[1, 2, 3]":
        passed = passed + 1

    # Test 5: json array of strings
    sarr: str = json_array_strs(["a", "b"])
    if sarr == '["a", "b"]':
        passed = passed + 1

    # Test 6: json bool
    t_str: str = json_bool_str(1)
    f_str: str = json_bool_str(0)
    if t_str == "true" and f_str == "false":
        passed = passed + 1

    # Test 7: json null
    if json_null() == "null":
        passed = passed + 1

    # Test 8: build user json contains expected fields
    user: str = build_user_json("alice", 30, 1)
    has_name: int = 0
    if '"username": "alice"' in user:
        has_name = 1
    has_age: int = 0
    if '"age": 30' in user:
        has_age = 1
    if has_name == 1 and has_age == 1:
        passed = passed + 1

    return passed
