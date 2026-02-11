def markov_step(state: list[float], transition: list[list[float]]) -> list[float]:
    n: int = len(state)
    result: list[float] = []
    i: int = 0
    while i < n:
        val: float = 0.0
        j: int = 0
        while j < n:
            sj: float = state[j]
            row_j: list[float] = transition[j]
            tji: float = row_j[i]
            val = val + sj * tji
            j = j + 1
        result.append(val)
        i = i + 1
    return result

def markov_n_steps(state: list[float], transition: list[list[float]], steps: int) -> list[float]:
    current: list[float] = state
    i: int = 0
    while i < steps:
        current = markov_step(current, transition)
        i = i + 1
    return current

def is_absorbing(transition: list[list[float]], state_idx: int) -> int:
    row: list[float] = transition[state_idx]
    val: float = row[state_idx]
    if val == 1.0:
        return 1
    return 0

def steady_state_approx(state: list[float], transition: list[list[float]]) -> list[float]:
    return markov_n_steps(state, transition, 100)

def state_entropy(state: list[float]) -> float:
    entropy: float = 0.0
    n: int = len(state)
    i: int = 0
    while i < n:
        p: float = state[i]
        if p > 0.0001:
            log_p: float = (p - 1.0) - 0.5 * (p - 1.0) * (p - 1.0)
            entropy = entropy - p * log_p
        i = i + 1
    return entropy

def test_module() -> int:
    passed: int = 0
    s: list[float] = [1.0, 0.0]
    t: list[list[float]] = [[0.5, 0.5], [0.3, 0.7]]
    r: list[float] = markov_step(s, t)
    r0: float = r[0]
    if r0 == 0.5:
        passed = passed + 1
    r1: float = r[1]
    if r1 == 0.5:
        passed = passed + 1
    r2: list[float] = markov_n_steps(s, t, 2)
    n: int = len(r2)
    if n == 2:
        passed = passed + 1
    t2: list[list[float]] = [[1.0, 0.0], [0.5, 0.5]]
    ab: int = is_absorbing(t2, 0)
    if ab == 1:
        passed = passed + 1
    ab2: int = is_absorbing(t2, 1)
    if ab2 == 0:
        passed = passed + 1
    return passed
