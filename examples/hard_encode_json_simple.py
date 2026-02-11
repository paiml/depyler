def json_int(val: int) -> str:
    if val == 0:
        return "0"
    result: str = ""
    neg: int = 0
    v: int = val
    if v < 0:
        neg = 1
        v = 0 - v
    while v > 0:
        digit: int = v % 10
        digits: str = "0123456789"
        result = digits[digit] + result
        v = v // 10
    if neg == 1:
        result = "-" + result
    return result

def json_null() -> str:
    return "null"

def json_string(val: str) -> str:
    return "\"" + val + "\""

def json_pair(name: str, val: str) -> str:
    return json_string(name) + ":" + val

def json_object(pairs: list[str]) -> str:
    result: str = "{"
    n: int = len(pairs)
    i: int = 0
    while i < n:
        if i > 0:
            result = result + ","
        result = result + pairs[i]
        i = i + 1
    result = result + "}"
    return result

def json_array(items: list[str]) -> str:
    result: str = "["
    n: int = len(items)
    i: int = 0
    while i < n:
        if i > 0:
            result = result + ","
        result = result + items[i]
        i = i + 1
    result = result + "]"
    return result

def json_bool(val: int) -> str:
    if val == 1:
        return "true"
    return "false"

def test_module() -> int:
    passed: int = 0
    s: str = json_int(42)
    if s == "42":
        passed = passed + 1
    s2: str = json_int(0 - 5)
    if s2 == "-5":
        passed = passed + 1
    s3: str = json_string("hello")
    if s3 == "\"hello\"":
        passed = passed + 1
    arr: list[str] = ["1", "2", "3"]
    s4: str = json_array(arr)
    if s4 == "[1,2,3]":
        passed = passed + 1
    s5: str = json_bool(1)
    if s5 == "true":
        passed = passed + 1
    return passed
