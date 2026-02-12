from typing import List, Tuple

def xorshift32(state: int) -> int:
    state = state ^ ((state << 13) & 0xFFFFFFFF)
    state = state ^ (state >> 17)
    state = state ^ ((state << 5) & 0xFFFFFFFF)
    return state & 0xFFFFFFFF

def xorshift_gen(seed: int, count: int) -> List[int]:
    results: List[int] = []
    state: int = seed
    if state == 0:
        state = 1
    for i in range(count):
        state = state ^ ((state << 13) & 0xFFFFFFFF)
        state = state ^ (state >> 17)
        state = state ^ ((state << 5) & 0xFFFFFFFF)
        state = state & 0xFFFFFFFF
        results.append(state)
    return results

def xorshift_uniform_scaled(state: int) -> Tuple[int, int]:
    s: int = state
    s = s ^ ((s << 13) & 0xFFFFFFFF)
    s = s ^ (s >> 17)
    s = s ^ ((s << 5) & 0xFFFFFFFF)
    s = s & 0xFFFFFFFF
    if s == 0:
        s = 1
    scaled: int = (s * 10000) // 4294967295
    return (scaled, s)

def test_period(seed: int, max_iter: int) -> int:
    s: int = seed
    if s == 0:
        s = 1
    s = s ^ ((s << 13) & 0xFFFFFFFF)
    s = s ^ (s >> 17)
    s = s ^ ((s << 5) & 0xFFFFFFFF)
    s = s & 0xFFFFFFFF
    initial: int = s
    count: int = 0
    while count < max_iter:
        s = s ^ ((s << 13) & 0xFFFFFFFF)
        s = s ^ (s >> 17)
        s = s ^ ((s << 5) & 0xFFFFFFFF)
        s = s & 0xFFFFFFFF
        count = count + 1
        if s == initial:
            return count
    return max_iter

def xorshift_bytes(seed: int, count: int) -> List[int]:
    result: List[int] = []
    s: int = seed
    if s == 0:
        s = 1
    for i in range(count):
        s = s ^ ((s << 13) & 0xFFFFFFFF)
        s = s ^ (s >> 17)
        s = s ^ ((s << 5) & 0xFFFFFFFF)
        s = s & 0xFFFFFFFF
        result.append(s & 0xFF)
    return result
