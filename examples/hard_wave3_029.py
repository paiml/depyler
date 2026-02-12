"""Text processing: Expression parsing and evaluation.

Tests: recursive descent parsing simulation, operator precedence,
parenthesized expressions, stack-based evaluation.
"""

from typing import Dict, List, Tuple


def eval_simple_add(expr: str) -> int:
    """Evaluate simple addition expression like '3+5+2'."""
    result: int = 0
    current: int = 0
    for ch in expr:
        if ch >= "0" and ch <= "9":
            current = current * 10 + (ord(ch) - ord("0"))
        elif ch == "+":
            result = result + current
            current = 0
    result = result + current
    return result


def eval_add_sub(expr: str) -> int:
    """Evaluate expression with + and - operators."""
    result: int = 0
    current: int = 0
    op: int = 1
    for ch in expr:
        if ch >= "0" and ch <= "9":
            current = current * 10 + (ord(ch) - ord("0"))
        elif ch == "+":
            result = result + op * current
            current = 0
            op = 1
        elif ch == "-":
            result = result + op * current
            current = 0
            op = -1
    result = result + op * current
    return result


def postfix_eval(tokens: List[str]) -> int:
    """Evaluate postfix (RPN) expression."""
    stack: List[int] = []
    for token in tokens:
        if token == "+":
            b: int = stack[len(stack) - 1]
            stack.pop()
            a: int = stack[len(stack) - 1]
            stack.pop()
            stack.append(a + b)
        elif token == "-":
            b2: int = stack[len(stack) - 1]
            stack.pop()
            a2: int = stack[len(stack) - 1]
            stack.pop()
            stack.append(a2 - b2)
        elif token == "*":
            b3: int = stack[len(stack) - 1]
            stack.pop()
            a3: int = stack[len(stack) - 1]
            stack.pop()
            stack.append(a3 * b3)
        else:
            stack.append(int(token))
    if len(stack) > 0:
        return stack[0]
    return 0


def infix_to_postfix_simple(expr: str) -> List[str]:
    """Convert simple infix (digits and +-) to postfix tokens."""
    output: List[str] = []
    ops: List[str] = []
    current: List[str] = []
    for ch in expr:
        if ch >= "0" and ch <= "9":
            current.append(ch)
        elif ch == "+" or ch == "-":
            if len(current) > 0:
                output.append("".join(current))
                current = []
            while len(ops) > 0:
                output.append(ops[len(ops) - 1])
                ops.pop()
            ops.append(ch)
    if len(current) > 0:
        output.append("".join(current))
    while len(ops) > 0:
        output.append(ops[len(ops) - 1])
        ops.pop()
    return output


def count_operators(expr: str) -> int:
    """Count arithmetic operators in expression."""
    count: int = 0
    for ch in expr:
        if ch == "+" or ch == "-" or ch == "*" or ch == "/":
            count += 1
    return count


def extract_numbers(expr: str) -> List[int]:
    """Extract all integers from an expression string."""
    result: List[int] = []
    current: int = 0
    has_digit: bool = False
    for ch in expr:
        if ch >= "0" and ch <= "9":
            current = current * 10 + (ord(ch) - ord("0"))
            has_digit = True
        else:
            if has_digit:
                result.append(current)
                current = 0
                has_digit = False
    if has_digit:
        result.append(current)
    return result


def test_eval() -> bool:
    """Test expression evaluation."""
    ok: bool = True
    r1: int = eval_simple_add("3+5+2")
    if r1 != 10:
        ok = False
    r2: int = eval_add_sub("10-3+2")
    if r2 != 9:
        ok = False
    r3: int = postfix_eval(["3", "4", "+", "2", "*"])
    if r3 != 14:
        ok = False
    nums: List[int] = extract_numbers("12+34-5")
    if len(nums) != 3:
        ok = False
    return ok
