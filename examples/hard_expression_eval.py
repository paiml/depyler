"""Expression evaluation for postfix expressions using integer tokens.

Tests: postfix evaluation, parenthesis validation, operator precedence.
"""


def apply_op(a: int, b: int, op: int) -> int:
    """Apply operator: 1=add, 2=sub, 3=mul."""
    if op == 1:
        return a + b
    if op == 2:
        return a - b
    if op == 3:
        return a * b
    return 0


def eval_two_operands(stack: list[int], op: int) -> list[int]:
    """Pop two operands, apply op, push result. Returns new stack."""
    n: int = len(stack)
    b: int = stack[n - 1]
    a: int = stack[n - 2]
    result: list[int] = []
    i: int = 0
    while i < n - 2:
        result.append(stack[i])
        i = i + 1
    val: int = apply_op(a, b, op)
    result.append(val)
    return result


def eval_postfix_simple(tokens: list[int], ops: list[int]) -> int:
    """Evaluate a postfix expression.

    ops[i] == 0 means tokens[i] is a number to push.
    ops[i] != 0 means apply that operator to top two stack elements.
    """
    stack: list[int] = []
    i: int = 0
    while i < len(tokens):
        if ops[i] == 0:
            stack.append(tokens[i])
        else:
            stack = eval_two_operands(stack, ops[i])
        i = i + 1
    return stack[0]


def check_balanced_parens_val(parens: list[int]) -> int:
    """Check if parentheses are balanced. 1=open, -1=close. Returns 1 balanced, 0 not."""
    depth: int = 0
    i: int = 0
    while i < len(parens):
        depth = depth + parens[i]
        if depth < 0:
            return 0
        i = i + 1
    if depth == 0:
        return 1
    return 0


def operator_precedence(op: int) -> int:
    """Return precedence: 1=add/sub, 2=mul/div, 0=unknown."""
    if op == 1 or op == 2:
        return 1
    if op == 3 or op == 4:
        return 2
    return 0


def eval_simple_chain(values: list[int], operators: list[int]) -> int:
    """Evaluate left-to-right: v0 op0 v1 op1 v2..."""
    if len(values) == 0:
        return 0
    result: int = values[0]
    i: int = 0
    while i < len(operators):
        result = apply_op(result, values[i + 1], operators[i])
        i = i + 1
    return result


def test_module() -> None:
    tokens: list[int] = [3, 4, 0, 5, 0]
    ops: list[int] = [0, 0, 1, 0, 3]
    assert eval_postfix_simple(tokens, ops) == 35
    tokens2: list[int] = [2, 3, 0, 0, 0]
    ops2: list[int] = [0, 0, 1, 0, 2]
    assert eval_postfix_simple(tokens2, ops2) == 5
    assert check_balanced_parens_val([1, 1, -1, -1]) == 1
    assert check_balanced_parens_val([1, -1, -1]) == 0
    assert check_balanced_parens_val([]) == 1
    assert operator_precedence(1) == 1
    assert operator_precedence(3) == 2
    vals: list[int] = [10, 3, 2]
    operators: list[int] = [2, 3]
    assert eval_simple_chain(vals, operators) == 14
