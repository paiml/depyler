def is_unreserved(ch: str) -> int:
    code: int = ord(ch)
    if code >= 97 and code <= 122:
        return 1
    if code >= 65 and code <= 90:
        return 1
    if code >= 48 and code <= 57:
        return 1
    if ch == "-" or ch == "_" or ch == "." or ch == "~":
        return 1
    return 0

def hex_digit(val: int) -> str:
    digits: str = "0123456789ABCDEF"
    return digits[val]

def char_to_percent(ch: str) -> str:
    code: int = ord(ch)
    hi: int = code // 16
    lo: int = code % 16
    return "%" + hex_digit(hi) + hex_digit(lo)

def url_encode(text: str) -> str:
    result: str = ""
    n: int = len(text)
    i: int = 0
    while i < n:
        ch: str = text[i]
        safe: int = is_unreserved(ch)
        if safe == 1:
            result = result + ch
        elif ch == " ":
            result = result + "%20"
        else:
            result = result + char_to_percent(ch)
        i = i + 1
    return result

def count_encoded(text: str) -> int:
    encoded: str = url_encode(text)
    count: int = 0
    n: int = len(encoded)
    i: int = 0
    while i < n:
        if encoded[i] == "%":
            count = count + 1
        i = i + 1
    return count

def encoded_length(text: str) -> int:
    encoded: str = url_encode(text)
    return len(encoded)

def test_module() -> int:
    passed: int = 0
    r: int = is_unreserved("a")
    if r == 1:
        passed = passed + 1
    r2: int = is_unreserved(" ")
    if r2 == 0:
        passed = passed + 1
    e: str = url_encode("hello world")
    if e == "hello%20world":
        passed = passed + 1
    e2: str = url_encode("abc")
    if e2 == "abc":
        passed = passed + 1
    c: int = count_encoded("a b c")
    if c == 2:
        passed = passed + 1
    return passed
