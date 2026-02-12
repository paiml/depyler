from typing import List, Tuple

def create_state(is_accept: int, char_val: int, next_st: int, eps_st: int) -> Tuple[int, int, int, int]:
    return (is_accept, char_val, next_st, eps_st)

def epsilon_closure(states: List[int], num_states: int, start_set: List[int]) -> List[int]:
    result: List[int] = []
    for s in start_set:
        result.append(s)
    changed: bool = True
    while changed:
        changed = False
        for i in range(len(result)):
            st: int = result[i]
            if st >= 0 and st < num_states:
                eps: int = states[st * 4 + 3]
                if eps >= 0:
                    found: bool = False
                    for r in result:
                        if r == eps:
                            found = True
                    if not found:
                        result.append(eps)
                        changed = True
    return result

def nfa_step_simple(states: List[int], num_states: int, current: List[int], ch: int) -> List[int]:
    next_set: List[int] = []
    for s in current:
        if s >= 0 and s < num_states:
            if states[s * 4 + 1] == ch and states[s * 4 + 2] >= 0:
                next_set.append(states[s * 4 + 2])
    return next_set

def nfa_match(states: List[int], num_states: int, input_str: List[int]) -> bool:
    current: List[int] = epsilon_closure(states, num_states, [0])
    for ch in input_str:
        next_set: List[int] = []
        for s in current:
            if s >= 0 and s < num_states:
                if states[s * 4 + 1] == ch and states[s * 4 + 2] >= 0:
                    next_set.append(states[s * 4 + 2])
        current: List[int] = []
        for s in next_set:
            current.append(s)
        changed: bool = True
        while changed:
            changed = False
            for i in range(len(current)):
                st: int = current[i]
                if st >= 0 and st < num_states:
                    eps: int = states[st * 4 + 3]
                    if eps >= 0:
                        found: bool = False
                        for r in current:
                            if r == eps:
                                found = True
                        if not found:
                            current.append(eps)
                            changed = True
        if len(current) == 0:
            return False
    for s in current:
        if s >= 0 and s < num_states and states[s * 4] == 1:
            return True
    return False

def state_count(num_states: int) -> int:
    return num_states

def count_accept_states(states: List[int], num_states: int) -> int:
    count: int = 0
    for i in range(num_states):
        if states[i * 4] == 1:
            count = count + 1
    return count
