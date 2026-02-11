"""Bracket matching and validation using integer codes.

Bracket encoding: 1=open_paren, 2=close_paren, 3=open_square, 4=close_square,
5=open_curly, 6=close_curly, 0=other
"""


def char_code(ch: str) -> int:
    """Map bracket character to integer code."""
    if ch == "[":
        return 3
    if ch == "]":
        return 4
    if ch == "{":
        return 5
    if ch == "}":
        return 6
    return 0


def is_open_bracket(code: int) -> int:
    """Check if code is an opening bracket. Returns 1/0."""
    if code == 1 or code == 3 or code == 5:
        return 1
    return 0


def is_close_bracket(code: int) -> int:
    """Check if code is a closing bracket. Returns 1/0."""
    if code == 2 or code == 4 or code == 6:
        return 1
    return 0


def is_matching_code(open_code: int, close_code: int) -> int:
    """Check if open/close codes form a matching pair. Returns 1/0."""
    if open_code == 1 and close_code == 2:
        return 1
    if open_code == 3 and close_code == 4:
        return 1
    if open_code == 5 and close_code == 6:
        return 1
    return 0


def encode_string(text: str) -> list[int]:
    """Encode bracket string to list of integer codes."""
    codes: list[int] = []
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        c: int = char_code(ch)
        codes.append(c)
        i = i + 1
    return codes


def is_balanced(text: str) -> int:
    """Check if brackets in text are balanced. Returns 1 if yes, 0 if no."""
    codes: list[int] = encode_string(text)
    stack: list[int] = []
    i: int = 0
    length: int = len(codes)
    while i < length:
        code: int = codes[i]
        if is_open_bracket(code) == 1:
            stack.append(code)
        elif is_close_bracket(code) == 1:
            if len(stack) == 0:
                return 0
            top_idx: int = len(stack) - 1
            top: int = stack[top_idx]
            if is_matching_code(top, code) == 0:
                return 0
            stack.pop()
        i = i + 1
    if len(stack) == 0:
        return 1
    return 0


def max_nesting_depth(text: str) -> int:
    """Find the maximum nesting depth of brackets."""
    codes: list[int] = encode_string(text)
    max_depth: int = 0
    current: int = 0
    i: int = 0
    length: int = len(codes)
    while i < length:
        code: int = codes[i]
        if is_open_bracket(code) == 1:
            current = current + 1
            if current > max_depth:
                max_depth = current
        elif is_close_bracket(code) == 1:
            current = current - 1
        i = i + 1
    return max_depth


def count_bracket_pairs(text: str) -> int:
    """Count the number of bracket pairs in balanced text."""
    codes: list[int] = encode_string(text)
    count: int = 0
    i: int = 0
    length: int = len(codes)
    while i < length:
        code: int = codes[i]
        if is_close_bracket(code) == 1:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test bracket matching operations."""
    passed: int = 0

    if is_balanced("[]{}") == 1:
        passed = passed + 1

    if is_balanced("[{}]") == 1:
        passed = passed + 1

    if is_balanced("[}") == 0:
        passed = passed + 1

    if is_balanced("[") == 0:
        passed = passed + 1

    if is_balanced("") == 1:
        passed = passed + 1

    if max_nesting_depth("[[[]]]") == 3:
        passed = passed + 1

    if max_nesting_depth("[][]") == 1:
        passed = passed + 1

    if count_bracket_pairs("[[][]]") == 3:
        passed = passed + 1

    return passed
