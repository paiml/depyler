"""Expression evaluation: integer-encoded tokens, infix to postfix, evaluate.

Uses integer encoding: numbers are positive values, operators encoded as negative:
  -1 = +, -2 = -, -3 = *, -4 = /, -5 = (, -6 = )

Tests: tokenize, infix_to_postfix, eval_postfix, simple expression eval.
"""


def op_precedence(op_tok: int) -> int:
    """Get operator precedence by token code. Higher = binds tighter."""
    if op_tok == -1:
        return 1
    if op_tok == -2:
        return 1
    if op_tok == -3:
        return 2
    if op_tok == -4:
        return 2
    return 0


def is_op_token(tok: int) -> int:
    """Check if token is an operator (-1 to -4)."""
    if tok == -1:
        return 1
    if tok == -2:
        return 1
    if tok == -3:
        return 1
    if tok == -4:
        return 1
    return 0


def char_to_token(ch_code: int) -> int:
    """Convert character code to token. Digits return -100 (handled separately)."""
    if ch_code == 43:
        return -1
    if ch_code == 45:
        return -2
    if ch_code == 42:
        return -3
    if ch_code == 47:
        return -4
    if ch_code == 40:
        return -5
    if ch_code == 41:
        return -6
    if ch_code >= 48:
        if ch_code <= 57:
            return -100
    return -999


def tokenize_expr(expr: str) -> list[int]:
    """Tokenize expression string into integer tokens."""
    tokens: list[int] = []
    i: int = 0
    n: int = len(expr)
    while i < n:
        ch_code: int = ord(expr[i])
        if ch_code == 32:
            i = i + 1
        elif ch_code >= 48:
            if ch_code <= 57:
                num: int = 0
                while i < n:
                    c2: int = ord(expr[i])
                    if c2 >= 48:
                        if c2 <= 57:
                            num = num * 10 + (c2 - 48)
                            i = i + 1
                        else:
                            break
                    else:
                        break
                tokens.append(num)
            else:
                i = i + 1
        else:
            tok: int = char_to_token(ch_code)
            if tok != -999:
                tokens.append(tok)
            i = i + 1
    return tokens


def infix_to_postfix_int(tokens: list[int]) -> list[int]:
    """Convert infix int-tokens to postfix using Shunting Yard."""
    output: list[int] = []
    ops: list[int] = []
    i: int = 0
    n: int = len(tokens)
    while i < n:
        tok: int = tokens[i]
        if tok == -5:
            ops.append(tok)
        elif tok == -6:
            while len(ops) > 0:
                top: int = ops[len(ops) - 1]
                if top == -5:
                    ops.pop()
                    break
                output.append(ops.pop())
        elif is_op_token(tok) == 1:
            while len(ops) > 0:
                top2: int = ops[len(ops) - 1]
                if top2 == -5:
                    break
                p_top: int = op_precedence(top2)
                p_tok: int = op_precedence(tok)
                if p_top >= p_tok:
                    output.append(ops.pop())
                else:
                    break
            ops.append(tok)
        else:
            output.append(tok)
        i = i + 1
    while len(ops) > 0:
        output.append(ops.pop())
    return output


def eval_postfix_int(tokens: list[int]) -> int:
    """Evaluate postfix integer-token expression."""
    stack: list[int] = []
    i: int = 0
    n: int = len(tokens)
    while i < n:
        tok: int = tokens[i]
        if is_op_token(tok) == 1:
            b: int = stack.pop()
            a: int = stack.pop()
            if tok == -1:
                stack.append(a + b)
            elif tok == -2:
                stack.append(a - b)
            elif tok == -3:
                stack.append(a * b)
            elif tok == -4:
                if b != 0:
                    stack.append(a // b)
                else:
                    stack.append(0)
        else:
            stack.append(tok)
        i = i + 1
    if len(stack) == 0:
        return 0
    return stack[0]


def evaluate_expression(expr: str) -> int:
    """Evaluate simple arithmetic expression string."""
    tokens: list[int] = tokenize_expr(expr)
    postfix: list[int] = infix_to_postfix_int(tokens)
    return eval_postfix_int(postfix)


def count_ops_in_expr(expr: str) -> int:
    """Count number of operators in expression string."""
    count: int = 0
    i: int = 0
    n: int = len(expr)
    while i < n:
        ch_code: int = ord(expr[i])
        if ch_code == 43:
            count = count + 1
        elif ch_code == 45:
            count = count + 1
        elif ch_code == 42:
            count = count + 1
        elif ch_code == 47:
            count = count + 1
        i = i + 1
    return count


def apply_op(a: int, b: int, op_tok: int) -> int:
    """Apply operator token to two operands."""
    if op_tok == -1:
        return a + b
    if op_tok == -2:
        return a - b
    if op_tok == -3:
        return a * b
    if op_tok == -4:
        if b != 0:
            return a // b
        return 0
    return 0


def test_module() -> int:
    """Test expression evaluation."""
    passed: int = 0

    r1: int = evaluate_expression("3 + 4")
    if r1 == 7:
        passed = passed + 1

    r2: int = evaluate_expression("3 + 4 * 2")
    if r2 == 11:
        passed = passed + 1

    r3: int = evaluate_expression("( 3 + 4 ) * 2")
    if r3 == 14:
        passed = passed + 1

    r4: int = evaluate_expression("10 - 3 - 2")
    if r4 == 5:
        passed = passed + 1

    r5: int = evaluate_expression("100 / 10 / 2")
    if r5 == 5:
        passed = passed + 1

    if count_ops_in_expr("3 + 4 * 2") == 2:
        passed = passed + 1

    if apply_op(10, 3, -2) == 7:
        passed = passed + 1

    return passed
