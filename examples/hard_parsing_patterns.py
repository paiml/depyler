"""String parsing patterns.

Tests: simple tokenizer, expression evaluator, bracket matching,
nesting depth, and string splitting.
"""


def tokenize_expression(expr: str) -> list[str]:
    """Tokenize a simple math expression into numbers and operators."""
    tokens: list[str] = []
    i: int = 0
    n: int = len(expr)
    while i < n:
        c: str = expr[i]
        if c == " ":
            i = i + 1
            continue
        if c == "+" or c == "-" or c == "*" or c == "/" or c == "(" or c == ")":
            tokens.append(c)
            i = i + 1
        elif c >= "0" and c <= "9":
            num_str: str = ""
            while i < n and expr[i] >= "0" and expr[i] <= "9":
                num_str = num_str + expr[i]
                i = i + 1
            tokens.append(num_str)
        else:
            i = i + 1
    return tokens


def parse_int(s: str) -> int:
    """Convert numeric string to integer."""
    result: int = 0
    i: int = 0
    while i < len(s):
        result = result * 10 + (ord(s[i]) - ord("0"))
        i = i + 1
    return result


def eval_simple_addition(expr: str) -> int:
    """Evaluate expression with only + and - (left to right)."""
    tokens: list[str] = tokenize_expression(expr)
    if len(tokens) == 0:
        return 0
    result: int = parse_int(tokens[0])
    i: int = 1
    while i < len(tokens) - 1:
        op: str = tokens[i]
        val: int = parse_int(tokens[i + 1])
        if op == "+":
            result = result + val
        elif op == "-":
            result = result - val
        i = i + 2
    return result


def is_balanced_brackets(s: str) -> bool:
    """Check if brackets are balanced: (), [], {}."""
    stack: list[str] = []
    i: int = 0
    while i < len(s):
        c: str = s[i]
        if c == "(" or c == "[" or c == "{":
            stack.append(c)
        elif c == ")":
            if len(stack) == 0:
                return False
            top: str = stack[len(stack) - 1]
            stack = stack[0:len(stack) - 1]
            if top != "(":
                return False
        elif c == "]":
            if len(stack) == 0:
                return False
            top2: str = stack[len(stack) - 1]
            stack = stack[0:len(stack) - 1]
            if top2 != "[":
                return False
        elif c == "}":
            if len(stack) == 0:
                return False
            top3: str = stack[len(stack) - 1]
            stack = stack[0:len(stack) - 1]
            if top3 != "{":
                return False
        i = i + 1
    return len(stack) == 0


def max_nesting_depth(s: str) -> int:
    """Find maximum nesting depth of parentheses."""
    max_depth: int = 0
    current: int = 0
    i: int = 0
    while i < len(s):
        if s[i] == "(":
            current = current + 1
            if current > max_depth:
                max_depth = current
        elif s[i] == ")":
            current = current - 1
        i = i + 1
    return max_depth


def split_by_comma(s: str) -> list[str]:
    """Split string by comma delimiter."""
    result: list[str] = []
    current: str = ""
    i: int = 0
    while i < len(s):
        if s[i] == ",":
            result.append(current)
            current = ""
        else:
            current = current + s[i]
        i = i + 1
    result.append(current)
    return result


def test_module() -> bool:
    """Test all parsing functions."""
    ok: bool = True

    tokens: list[str] = tokenize_expression("12 + 34 * 5")
    if len(tokens) != 5:
        ok = False
    if tokens[0] != "12":
        ok = False

    if eval_simple_addition("3 + 4") != 7:
        ok = False
    if eval_simple_addition("10 - 3") != 7:
        ok = False

    if not is_balanced_brackets("({[]})"):
        ok = False
    if is_balanced_brackets("({[})"):
        ok = False
    if not is_balanced_brackets(""):
        ok = False

    if max_nesting_depth("((()))") != 3:
        ok = False
    if max_nesting_depth("()()") != 1:
        ok = False

    parts: list[str] = split_by_comma("a,b,c,d")
    if len(parts) != 4:
        ok = False
    if parts[0] != "a":
        ok = False

    return ok
