"""Postfix (Reverse Polish Notation) expression evaluation."""


def is_digit_string(s: str) -> int:
    """Return 1 if string represents an integer (possibly negative), else 0."""
    length: int = len(s)
    if length == 0:
        return 0
    start: int = 0
    if s[0] == "-":
        if length == 1:
            return 0
        start = 1
    idx: int = start
    while idx < length:
        ch: str = s[idx]
        if ch < "0" or ch > "9":
            return 0
        idx = idx + 1
    return 1


def parse_int_simple(s: str) -> int:
    """Parse a string to integer. Assumes valid integer string."""
    negative: int = 0
    start: int = 0
    if s[0] == "-":
        negative = 1
        start = 1
    result: int = 0
    idx: int = start
    length: int = len(s)
    while idx < length:
        digit: int = ord(s[idx]) - ord("0")
        result = result * 10 + digit
        idx = idx + 1
    if negative == 1:
        result = -result
    return result


def eval_postfix(tokens: list[str]) -> int:
    """Evaluate postfix expression given as list of tokens.
    Supports +, -, *, /."""
    stack: list[int] = []
    idx: int = 0
    length: int = len(tokens)
    while idx < length:
        token: str = tokens[idx]
        if is_digit_string(token) == 1:
            stack.append(parse_int_simple(token))
        else:
            stack_len: int = len(stack)
            b_pos: int = stack_len - 1
            a_pos: int = stack_len - 2
            b: int = stack[b_pos]
            a: int = stack[a_pos]
            stack.pop()
            stack.pop()
            if token == "+":
                stack.append(a + b)
            elif token == "-":
                stack.append(a - b)
            elif token == "*":
                stack.append(a * b)
            elif token == "/":
                stack.append(a // b)
        idx = idx + 1
    result_pos: int = len(stack) - 1
    return stack[result_pos]


def test_module() -> int:
    passed: int = 0

    if eval_postfix(["2", "3", "+"]) == 5:
        passed = passed + 1
    if eval_postfix(["5", "1", "2", "+", "4", "*", "+", "3", "-"]) == 14:
        passed = passed + 1
    if eval_postfix(["4", "2", "*"]) == 8:
        passed = passed + 1
    if eval_postfix(["10", "3", "/"]) == 3:
        passed = passed + 1

    if is_digit_string("123") == 1:
        passed = passed + 1
    if is_digit_string("+") == 0:
        passed = passed + 1
    if parse_int_simple("-42") == -42:
        passed = passed + 1

    return passed
