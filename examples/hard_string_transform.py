"""String transformation: camelCase, snake_case, capitalize words."""


def to_snake_case(s: str) -> str:
    """Convert camelCase or PascalCase to snake_case."""
    result: str = ""
    i: int = 0
    while i < len(s):
        c: str = s[i]
        if c >= "A" and c <= "Z":
            if len(result) > 0:
                result = result + "_"
            # Convert uppercase to lowercase by adding 32 to ASCII
            code: int = ord(c) + 32
            result = result + chr(code)
        else:
            result = result + c
        i = i + 1
    return result


def to_camel_case(s: str) -> str:
    """Convert snake_case to camelCase."""
    result: str = ""
    capitalize_next: int = 0
    i: int = 0
    while i < len(s):
        if s[i] == "_":
            capitalize_next = 1
        else:
            if capitalize_next == 1 and s[i] >= "a" and s[i] <= "z":
                code: int = ord(s[i]) - 32
                result = result + chr(code)
                capitalize_next = 0
            else:
                result = result + s[i]
                capitalize_next = 0
        i = i + 1
    return result


def capitalize_words(s: str) -> str:
    """Capitalize the first letter of each word."""
    result: str = ""
    cap_next: int = 1
    i: int = 0
    while i < len(s):
        if s[i] == " ":
            result = result + " "
            cap_next = 1
        else:
            if cap_next == 1 and s[i] >= "a" and s[i] <= "z":
                code: int = ord(s[i]) - 32
                result = result + chr(code)
                cap_next = 0
            else:
                result = result + s[i]
                cap_next = 0
        i = i + 1
    return result


def count_words(s: str) -> int:
    """Count number of words separated by spaces."""
    count: int = 0
    in_word: int = 0
    i: int = 0
    while i < len(s):
        if s[i] == " ":
            in_word = 0
        else:
            if in_word == 0:
                count = count + 1
                in_word = 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0

    if to_snake_case("camelCase") == "camel_case":
        passed = passed + 1

    if to_snake_case("PascalCase") == "pascal_case":
        passed = passed + 1

    if to_camel_case("hello_world") == "helloWorld":
        passed = passed + 1

    if to_camel_case("one") == "one":
        passed = passed + 1

    if capitalize_words("hello world") == "Hello World":
        passed = passed + 1

    if capitalize_words("foo bar baz") == "Foo Bar Baz":
        passed = passed + 1

    if count_words("hello world foo") == 3:
        passed = passed + 1

    if count_words("") == 0:
        passed = passed + 1

    return passed
