"""Postfix (reverse Polish notation) calculator using stack operations.

Tests: basic arithmetic, multi-operand expressions, stack push/pop.
"""


def stack_push(stack: list[int], val: int) -> list[int]:
    """Push a value onto the stack."""
    result: list[int] = stack[:]
    result.append(val)
    return result


def stack_pop(stack: list[int]) -> list[int]:
    """Pop top value from stack, returning [remaining..., popped_value]."""
    if len(stack) == 0:
        return [0]
    result: list[int] = []
    i: int = 0
    while i < len(stack) - 1:
        result.append(stack[i])
        i = i + 1
    result.append(stack[len(stack) - 1])
    return result


def eval_postfix(tokens: list[str]) -> int:
    """Evaluate a postfix expression. Numbers and operators as string tokens."""
    stack: list[int] = []
    i: int = 0
    while i < len(tokens):
        token: str = tokens[i]
        if token == "+" or token == "-" or token == "*" or token == "/":
            b: int = stack[len(stack) - 1]
            stack = stack[:len(stack) - 1]
            a: int = stack[len(stack) - 1]
            stack = stack[:len(stack) - 1]
            result: int = 0
            if token == "+":
                result = a + b
            elif token == "-":
                result = a - b
            elif token == "*":
                result = a * b
            elif token == "/":
                if b != 0:
                    result = a // b
                else:
                    result = 0
            stack.append(result)
        else:
            stack.append(int(token))
        i = i + 1
    if len(stack) > 0:
        return stack[0]
    return 0


def stack_peek(stack: list[int]) -> int:
    """Return top element without removing it."""
    if len(stack) == 0:
        return 0
    return stack[len(stack) - 1]


def test_module() -> int:
    """Test postfix calculator."""
    ok: int = 0

    if eval_postfix(["3", "4", "+"]) == 7:
        ok = ok + 1

    if eval_postfix(["5", "3", "-"]) == 2:
        ok = ok + 1

    if eval_postfix(["2", "3", "*"]) == 6:
        ok = ok + 1

    if eval_postfix(["10", "2", "/"]) == 5:
        ok = ok + 1

    if eval_postfix(["3", "4", "+", "2", "*"]) == 14:
        ok = ok + 1

    if eval_postfix(["5", "1", "2", "+", "4", "*", "+", "3", "-"]) == 14:
        ok = ok + 1

    s: list[int] = stack_push([], 42)
    if stack_peek(s) == 42:
        ok = ok + 1

    return ok
