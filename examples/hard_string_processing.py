# String manipulation patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def string_length(s: str) -> int:
    """Return length of string."""
    return len(s)


def repeat_string(s: str, n: int) -> str:
    """Repeat string s n times."""
    result: str = ""
    i: int = 0
    while i < n:
        result = result + s
        i = i + 1
    return result


def is_empty(s: str) -> bool:
    """Check if string is empty."""
    return len(s) == 0


def concat_two(a: str, b: str) -> str:
    """Concatenate two strings."""
    return a + b


def concat_three(a: str, b: str, c: str) -> str:
    """Concatenate three strings."""
    return a + b + c


def test_module() -> int:
    """Test all string processing functions."""
    assert string_length("hello") == 5
    assert string_length("") == 0
    assert string_length("abc") == 3
    assert repeat_string("ab", 3) == "ababab"
    assert repeat_string("x", 5) == "xxxxx"
    assert repeat_string("", 10) == ""
    assert is_empty("") == True
    assert is_empty("a") == False
    assert concat_two("hello", " world") == "hello world"
    assert concat_two("", "abc") == "abc"
    assert concat_three("a", "b", "c") == "abc"
    assert concat_three("", "", "") == ""
    assert concat_three("hello", " ", "world") == "hello world"
    return 0


if __name__ == "__main__":
    test_module()
