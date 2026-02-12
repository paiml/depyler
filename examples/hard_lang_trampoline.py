from typing import List, Tuple

def trampoline_step(state: List[int], func_flat: List[int], func_len: int) -> Tuple[List[int], bool]:
    if len(state) == 0:
        result: List[int] = []
        return (result, True)
    func_id: int = state[0]
    if func_id < 0 or func_id * func_len >= len(func_flat):
        result2: List[int] = []
        for s in state:
            result2.append(s)
        return (result2, True)
    new_state: List[int] = []
    start: int = func_id * func_len
    for i in range(func_len):
        new_state.append(func_flat[start + i])
    done: bool = new_state[0] < 0
    return (new_state, done)

def trampoline_run(initial: List[int], func_flat: List[int], func_len: int, max_steps: int) -> List[int]:
    state: List[int] = []
    for s in initial:
        state.append(s)
    for i in range(max_steps):
        if len(state) == 0 or state[0] < 0:
            return state
        fid: int = state[0]
        if fid * func_len >= len(func_flat):
            return state
        state = []
        start: int = fid * func_len
        for j in range(func_len):
            state.append(func_flat[start + j])
    return state

def is_done(state: List[int]) -> bool:
    if len(state) == 0:
        return True
    return state[0] < 0

def trampoline_factorial(n: int) -> int:
    acc: int = 1
    current: int = n
    while current > 1:
        acc = acc * current
        current = current - 1
    return acc

def trampoline_fib(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for i in range(2, n + 1):
        temp: int = a + b
        a = b
        b = temp
    return b
