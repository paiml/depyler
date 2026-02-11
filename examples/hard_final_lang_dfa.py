"""DFA (Deterministic Finite Automaton) with minimization.

Transition table stored as flat array. Includes DFA simulation,
state reachability, and equivalence checking.
"""


def dfa_create_trans(num_states: int, num_symbols: int) -> list[int]:
    """Create transition table: trans[state * num_symbols + symbol] = next_state.

    -1 means dead state (no transition).
    """
    result: list[int] = []
    total: int = num_states * num_symbols
    i: int = 0
    while i < total:
        result.append(0 - 1)
        i = i + 1
    return result


def dfa_set_trans(trans: list[int], num_symbols: int, from_st: int, symbol: int, to_st: int) -> int:
    """Set transition. Returns 1."""
    trans[from_st * num_symbols + symbol] = to_st
    return 1


def dfa_next(trans: list[int], num_symbols: int, state: int, symbol: int) -> int:
    """Get next state for given state and symbol. -1 if no transition."""
    return trans[state * num_symbols + symbol]


def dfa_run(trans: list[int], num_symbols: int, start: int, accept_states: list[int], input_str: list[int]) -> int:
    """Run DFA on input string. Returns 1 if accepts."""
    state: int = start
    i: int = 0
    while i < len(input_str):
        sym: int = input_str[i]
        state = dfa_next(trans, num_symbols, state, sym)
        if state < 0:
            return 0
        i = i + 1
    j: int = 0
    while j < len(accept_states):
        ac: int = accept_states[j]
        if state == ac:
            return 1
        j = j + 1
    return 0


def dfa_reachable(trans: list[int], num_states: int, num_symbols: int, start: int) -> list[int]:
    """Find all reachable states from start."""
    visited: list[int] = []
    stack: list[int] = [start]
    while len(stack) > 0:
        current: int = stack[len(stack) - 1]
        stack.pop()
        found: int = 0
        vi: int = 0
        while vi < len(visited):
            vv: int = visited[vi]
            if vv == current:
                found = 1
            vi = vi + 1
        if found == 0:
            visited.append(current)
            sym: int = 0
            while sym < num_symbols:
                nxt: int = dfa_next(trans, num_symbols, current, sym)
                if nxt >= 0:
                    stack.append(nxt)
                sym = sym + 1
    return visited


def dfa_complement_accepts(trans: list[int], num_symbols: int, start: int, accept_states: list[int], num_states: int, input_str: list[int]) -> int:
    """Run complement DFA (accept iff original rejects). Returns 1 if complement accepts."""
    result: int = dfa_run(trans, num_symbols, start, accept_states, input_str)
    if result == 1:
        return 0
    return 1


def dfa_num_accepting_reachable(trans: list[int], num_states: int, num_symbols: int, start: int, accept_states: list[int]) -> int:
    """Count how many reachable states are accepting."""
    reachable: list[int] = dfa_reachable(trans, num_states, num_symbols, start)
    cnt: int = 0
    i: int = 0
    while i < len(reachable):
        rs: int = reachable[i]
        j: int = 0
        while j < len(accept_states):
            ac: int = accept_states[j]
            if rs == ac:
                cnt = cnt + 1
            j = j + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test DFA. DFA for strings ending in 'b' (symbol 1). States: 0(start), 1(accept)."""
    ok: int = 0
    ns: int = 2
    nsym: int = 2
    trans: list[int] = dfa_create_trans(ns, nsym)
    dfa_set_trans(trans, nsym, 0, 0, 0)
    dfa_set_trans(trans, nsym, 0, 1, 1)
    dfa_set_trans(trans, nsym, 1, 0, 0)
    dfa_set_trans(trans, nsym, 1, 1, 1)
    accepts: list[int] = [1]
    if dfa_run(trans, nsym, 0, accepts, [1]) == 1:
        ok = ok + 1
    if dfa_run(trans, nsym, 0, accepts, [0, 1]) == 1:
        ok = ok + 1
    if dfa_run(trans, nsym, 0, accepts, [1, 0]) == 0:
        ok = ok + 1
    reach: list[int] = dfa_reachable(trans, ns, nsym, 0)
    if len(reach) == 2:
        ok = ok + 1
    nac: int = dfa_num_accepting_reachable(trans, ns, nsym, 0, accepts)
    if nac == 1:
        ok = ok + 1
    return ok
