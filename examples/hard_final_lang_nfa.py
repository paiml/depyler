"""NFA (Nondeterministic Finite Automaton) simulation.

States and transitions stored as flat arrays. Supports epsilon transitions.
Uses subset construction for simulation (powerset of active states).
"""


def create_nfa(num_states: int) -> list[int]:
    """Create NFA metadata: [num_states, start_state, num_accept_states, accept0, accept1, ...]."""
    return [num_states, 0]


def add_transition(trans_table: list[int], from_state: int, symbol: int, to_state: int) -> int:
    """Add transition: [from, symbol, to]. Symbol -1 = epsilon. Returns new size."""
    trans_table.append(from_state)
    trans_table.append(symbol)
    trans_table.append(to_state)
    return len(trans_table) // 3


def epsilon_closure(trans_table: list[int], states: list[int]) -> list[int]:
    """Compute epsilon closure of a set of states."""
    result: list[int] = []
    i: int = 0
    while i < len(states):
        sv: int = states[i]
        found: int = 0
        j: int = 0
        while j < len(result):
            rv: int = result[j]
            if rv == sv:
                found = 1
            j = j + 1
        if found == 0:
            result.append(sv)
        i = i + 1
    changed: int = 1
    while changed == 1:
        changed = 0
        ri: int = 0
        while ri < len(result):
            cur: int = result[ri]
            ti: int = 0
            while ti < len(trans_table) // 3:
                fs: int = trans_table[ti * 3]
                sym: int = trans_table[ti * 3 + 1]
                ts: int = trans_table[ti * 3 + 2]
                if fs == cur:
                    if sym == 0 - 1:
                        exists: int = 0
                        k: int = 0
                        while k < len(result):
                            rk: int = result[k]
                            if rk == ts:
                                exists = 1
                            k = k + 1
                        if exists == 0:
                            result.append(ts)
                            changed = 1
                ti = ti + 1
            ri = ri + 1
    return result


def move_states(trans_table: list[int], states: list[int], symbol: int) -> list[int]:
    """Compute states reachable from states on given symbol."""
    result: list[int] = []
    i: int = 0
    while i < len(states):
        cur: int = states[i]
        ti: int = 0
        while ti < len(trans_table) // 3:
            fs: int = trans_table[ti * 3]
            sym: int = trans_table[ti * 3 + 1]
            ts: int = trans_table[ti * 3 + 2]
            if fs == cur:
                if sym == symbol:
                    exists: int = 0
                    j: int = 0
                    while j < len(result):
                        rj: int = result[j]
                        if rj == ts:
                            exists = 1
                        j = j + 1
                    if exists == 0:
                        result.append(ts)
            ti = ti + 1
        i = i + 1
    return result


def nfa_accepts(trans_table: list[int], start: int, accept_states: list[int], input_str: list[int]) -> int:
    """Simulate NFA on input. Returns 1 if accepts."""
    current: list[int] = epsilon_closure(trans_table, [start])
    ii: int = 0
    while ii < len(input_str):
        sym: int = input_str[ii]
        moved: list[int] = move_states(trans_table, current, sym)
        current = epsilon_closure(trans_table, moved)
        ii = ii + 1
    i: int = 0
    while i < len(current):
        cs: int = current[i]
        j: int = 0
        while j < len(accept_states):
            ac: int = accept_states[j]
            if cs == ac:
                return 1
            j = j + 1
        i = i + 1
    return 0


def test_module() -> int:
    """Test NFA simulation. NFA for a*b: states 0->0(a), 0->1(b), 1 accept."""
    ok: int = 0
    tt: list[int] = []
    add_transition(tt, 0, 97, 0)
    add_transition(tt, 0, 98, 1)
    accepts: list[int] = [1]
    if nfa_accepts(tt, 0, accepts, [98]) == 1:
        ok = ok + 1
    if nfa_accepts(tt, 0, accepts, [97, 98]) == 1:
        ok = ok + 1
    if nfa_accepts(tt, 0, accepts, [97, 97, 98]) == 1:
        ok = ok + 1
    if nfa_accepts(tt, 0, accepts, [97]) == 0:
        ok = ok + 1
    ec: list[int] = epsilon_closure(tt, [0])
    if len(ec) == 1:
        ok = ok + 1
    return ok
