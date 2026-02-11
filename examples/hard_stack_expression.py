"""Expression evaluation using stack-based approaches."""


def evaluate_postfix(tokens: list[int], ops: list[int]) -> int:
    """Evaluate a postfix expression.
    tokens: operands (numbers), ops: 0=push, 1=add, 2=sub, 3=mul, 4=div.
    When ops[i]=0, push tokens[i]. Otherwise apply operator to top two stack elements."""
    stack: list[int] = []
    i: int = 0
    while i < len(ops):
        op: int = ops[i]
        if op == 0:
            stack.append(tokens[i])
        else:
            top_idx: int = len(stack) - 1
            b: int = stack[top_idx]
            stack.pop()
            next_idx: int = len(stack) - 1
            a: int = stack[next_idx]
            stack.pop()
            if op == 1:
                stack.append(a + b)
            elif op == 2:
                stack.append(a - b)
            elif op == 3:
                stack.append(a * b)
            elif op == 4:
                if b != 0:
                    stack.append(a // b)
                else:
                    stack.append(0)
        i = i + 1
    if len(stack) == 0:
        return 0
    result_idx: int = len(stack) - 1
    return stack[result_idx]


def check_balanced_parens(parens: list[int]) -> int:
    """Check if parentheses are balanced.
    1 = open paren, -1 = close paren.
    Returns 1 if balanced, 0 otherwise."""
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


def max_nesting_depth(parens: list[int]) -> int:
    """Find maximum nesting depth of parentheses.
    1 = open, -1 = close."""
    max_depth: int = 0
    depth: int = 0
    i: int = 0
    while i < len(parens):
        depth = depth + parens[i]
        if depth > max_depth:
            max_depth = depth
        i = i + 1
    return max_depth


def evaluate_prefix_simple(values: list[int], operators: list[int]) -> int:
    """Evaluate simple prefix expression: op a b.
    operators: 1=add, 2=sub, 3=mul. values has two operands per operator."""
    if len(operators) == 0:
        if len(values) > 0:
            return values[0]
        return 0
    op: int = operators[0]
    a: int = values[0]
    b: int = values[1]
    if op == 1:
        return a + b
    elif op == 2:
        return a - b
    elif op == 3:
        return a * b
    return 0


def test_module() -> int:
    """Test stack expression evaluation."""
    ok: int = 0

    # 3 4 + => 7
    tokens1: list[int] = [3, 4, 0]
    ops1: list[int] = [0, 0, 1]
    if evaluate_postfix(tokens1, ops1) == 7:
        ok = ok + 1

    # 5 3 - => 2
    tokens2: list[int] = [5, 3, 0]
    ops2: list[int] = [0, 0, 2]
    if evaluate_postfix(tokens2, ops2) == 2:
        ok = ok + 1

    # 2 3 * 4 + => 10
    tokens3: list[int] = [2, 3, 0, 4, 0]
    ops3: list[int] = [0, 0, 3, 0, 1]
    if evaluate_postfix(tokens3, ops3) == 10:
        ok = ok + 1

    balanced: list[int] = [1, 1, -1, -1]
    if check_balanced_parens(balanced) == 1:
        ok = ok + 1

    unbalanced: list[int] = [1, -1, -1]
    if check_balanced_parens(unbalanced) == 0:
        ok = ok + 1

    nested: list[int] = [1, 1, 1, -1, -1, -1]
    if max_nesting_depth(nested) == 3:
        ok = ok + 1

    return ok
