"""Systems programming patterns: file 91.
Tests: integer arithmetic, bitwise operations, state machines.
"""
from typing import Dict, List, Tuple

def func_a_91(x: int, y: int) -> int:
    """Compute combined hash of two integers."""
    h: int = x
    h = ((h << 5) + h) ^ y
    h = h & 0xFFFFFFFF
    return h

def func_b_91(data: List[int]) -> int:
    """Accumulate values with mixing."""
    result: int = 0
    for val in data:
        result = result ^ val
        result = ((result << 3) | (result >> 29)) & 0xFFFFFFFF
    return result

def func_c_91(n: int) -> List[int]:
    """Generate pseudorandom sequence."""
    result: List[int] = []
    state: int = n
    i: int = 0
    while i < 10:
        state = (state * 1103515245 + 12345) & 0x7FFFFFFF
        result.append(state % 100)
        i += 1
    return result

def test_91() -> bool:
    ok: bool = True
    h: int = func_a_91(42, 99)
    if h == 0:
        ok = False
    acc: int = func_b_91([1, 2, 3, 4, 5])
    if acc < 0:
        ok = False
    seq: List[int] = func_c_91(7)
    if len(seq) != 10:
        ok = False
    return ok
