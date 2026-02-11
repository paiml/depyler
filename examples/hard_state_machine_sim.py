"""Simple state machine simulation using integer states.

Tests: state transitions, accept/reject, run count.
"""


def run_state_machine(transitions_from: list[int], transitions_input: list[int], transitions_to: list[int], start: int, inputs: list[int]) -> int:
    """Run state machine and return final state. Returns -1 if stuck."""
    state: int = start
    idx: int = 0
    while idx < len(inputs):
        inp: int = inputs[idx]
        found: int = 0
        t: int = 0
        while t < len(transitions_from):
            if transitions_from[t] == state and transitions_input[t] == inp:
                state = transitions_to[t]
                found = 1
                t = len(transitions_from)
            else:
                t = t + 1
        if found == 0:
            return -1
        idx = idx + 1
    return state


def is_accepted_val(transitions_from: list[int], transitions_input: list[int], transitions_to: list[int], start: int, accept_states: list[int], inputs: list[int]) -> int:
    """Check if input sequence is accepted. Returns 1 if accepted, 0 otherwise."""
    final: int = run_state_machine(transitions_from, transitions_input, transitions_to, start, inputs)
    if final == -1:
        return 0
    i: int = 0
    while i < len(accept_states):
        if accept_states[i] == final:
            return 1
        i = i + 1
    return 0


def count_transitions(transitions_from: list[int], transitions_input: list[int], transitions_to: list[int], start: int, inputs: list[int]) -> int:
    """Count number of transitions taken."""
    state: int = start
    count: int = 0
    idx: int = 0
    while idx < len(inputs):
        inp: int = inputs[idx]
        found: int = 0
        t: int = 0
        while t < len(transitions_from):
            if transitions_from[t] == state and transitions_input[t] == inp:
                state = transitions_to[t]
                found = 1
                count = count + 1
                t = len(transitions_from)
            else:
                t = t + 1
        if found == 0:
            return count
        idx = idx + 1
    return count


def reachable_states_count(transitions_from: list[int], transitions_to: list[int], start: int, num_states: int) -> int:
    """Count states reachable from start via BFS."""
    visited: list[int] = [0] * num_states
    visited[start] = 1
    queue: list[int] = [start]
    count: int = 1
    while len(queue) > 0:
        current: int = queue[0]
        queue = queue[1:]
        t: int = 0
        while t < len(transitions_from):
            if transitions_from[t] == current:
                nxt: int = transitions_to[t]
                if visited[nxt] == 0:
                    visited[nxt] = 1
                    queue.append(nxt)
                    count = count + 1
            t = t + 1
    return count


def test_module() -> None:
    tf: list[int] = [0, 0, 1, 1]
    ti: list[int] = [0, 1, 0, 1]
    tt: list[int] = [0, 1, 0, 1]
    assert run_state_machine(tf, ti, tt, 0, [0, 1, 1]) == 1
    assert run_state_machine(tf, ti, tt, 0, [0, 0]) == 0
    acc: list[int] = [1]
    assert is_accepted_val(tf, ti, tt, 0, acc, [1]) == 1
    assert is_accepted_val(tf, ti, tt, 0, acc, [0]) == 0
    assert count_transitions(tf, ti, tt, 0, [0, 1, 1]) == 3
    assert reachable_states_count(tf, tt, 0, 2) == 2
