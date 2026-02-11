"""Pushdown Automaton (PDA) simulation.

Simulates a PDA with explicit stack. Used for context-free language recognition.
Stack and transitions represented as integer arrays.
"""


def pda_create_stack() -> list[int]:
    """Create PDA stack with bottom marker (0)."""
    return [0]


def pda_push(stack: list[int], symbol: int) -> int:
    """Push symbol onto stack. Returns new size."""
    stack.append(symbol)
    return len(stack)


def pda_pop(stack: list[int]) -> int:
    """Pop symbol from stack. Returns popped symbol or -1 if empty."""
    if len(stack) == 0:
        return 0 - 1
    val: int = stack[len(stack) - 1]
    stack.pop()
    return val


def pda_top(stack: list[int]) -> int:
    """Peek at top of stack. Returns -1 if empty."""
    if len(stack) == 0:
        return 0 - 1
    return stack[len(stack) - 1]


def pda_balanced_parens(input_str: list[int]) -> int:
    """Check if input has balanced parentheses. 1=open, 2=close. Returns 1 if balanced."""
    stack: list[int] = pda_create_stack()
    i: int = 0
    while i < len(input_str):
        sym: int = input_str[i]
        if sym == 1:
            pda_push(stack, 1)
        if sym == 2:
            top: int = pda_top(stack)
            if top != 1:
                return 0
            pda_pop(stack)
        i = i + 1
    if len(stack) == 1:
        return 1
    return 0


def pda_anbn(input_str: list[int]) -> int:
    """Recognize language a^n b^n. 1=a, 2=b. Returns 1 if valid."""
    stack: list[int] = pda_create_stack()
    state: int = 0
    i: int = 0
    while i < len(input_str):
        sym: int = input_str[i]
        if state == 0:
            if sym == 1:
                pda_push(stack, 1)
            if sym == 2:
                top: int = pda_top(stack)
                if top != 1:
                    return 0
                pda_pop(stack)
                state = 1
        else:
            if sym == 1:
                return 0
            if sym == 2:
                top2: int = pda_top(stack)
                if top2 != 1:
                    return 0
                pda_pop(stack)
        i = i + 1
    if len(stack) == 1:
        return 1
    return 0


def pda_palindrome(input_str: list[int], mid_marker: int) -> int:
    """Recognize palindromes with center marker. Push first half, match second half."""
    stack: list[int] = pda_create_stack()
    phase: int = 0
    i: int = 0
    while i < len(input_str):
        sym: int = input_str[i]
        if phase == 0:
            if sym == mid_marker:
                phase = 1
            else:
                pda_push(stack, sym)
        else:
            top: int = pda_top(stack)
            if top != sym:
                return 0
            pda_pop(stack)
        i = i + 1
    if len(stack) == 1:
        return 1
    return 0


def pda_depth(input_str: list[int]) -> int:
    """Compute maximum nesting depth of parentheses. 1=open, 2=close."""
    max_d: int = 0
    cur_d: int = 0
    i: int = 0
    while i < len(input_str):
        sym: int = input_str[i]
        if sym == 1:
            cur_d = cur_d + 1
            if cur_d > max_d:
                max_d = cur_d
        if sym == 2:
            cur_d = cur_d - 1
        i = i + 1
    return max_d


def test_module() -> int:
    """Test PDA."""
    ok: int = 0
    if pda_balanced_parens([1, 2, 1, 2]) == 1:
        ok = ok + 1
    if pda_balanced_parens([1, 1, 2]) == 0:
        ok = ok + 1
    if pda_anbn([1, 1, 2, 2]) == 1:
        ok = ok + 1
    if pda_anbn([1, 2, 2]) == 0:
        ok = ok + 1
    if pda_depth([1, 1, 2, 1, 2, 2]) == 2:
        ok = ok + 1
    return ok
