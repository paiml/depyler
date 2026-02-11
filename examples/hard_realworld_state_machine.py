"""Real-world protocol state machine simulation.

Mimics: TCP connection states, HTTP request lifecycle, game engine states.
Uses integer-encoded states and transition tables.
"""


def state_name(state_code: int) -> str:
    """Convert state code to human-readable name."""
    if state_code == 0:
        return "CLOSED"
    if state_code == 1:
        return "LISTEN"
    if state_code == 2:
        return "SYN_SENT"
    if state_code == 3:
        return "SYN_RECV"
    if state_code == 4:
        return "ESTABLISHED"
    if state_code == 5:
        return "FIN_WAIT_1"
    if state_code == 6:
        return "FIN_WAIT_2"
    if state_code == 7:
        return "CLOSE_WAIT"
    if state_code == 8:
        return "LAST_ACK"
    if state_code == 9:
        return "TIME_WAIT"
    return "UNKNOWN"


def transition(current: int, event: int) -> int:
    """Apply a transition event to current state.
    Events: 0=open, 1=syn, 2=ack, 3=fin, 4=close, 5=timeout."""
    if current == 0:
        if event == 0:
            return 1
        if event == 1:
            return 2
    elif current == 1:
        if event == 1:
            return 3
    elif current == 2:
        if event == 2:
            return 4
        if event == 1:
            return 3
    elif current == 3:
        if event == 2:
            return 4
    elif current == 4:
        if event == 3:
            return 5
        if event == 4:
            return 7
    elif current == 5:
        if event == 2:
            return 6
        if event == 3:
            return 9
    elif current == 6:
        if event == 3:
            return 9
    elif current == 7:
        if event == 4:
            return 8
    elif current == 8:
        if event == 2:
            return 0
    elif current == 9:
        if event == 5:
            return 0
    return current


def run_protocol(events: list[int]) -> list[int]:
    """Run a sequence of events, returning state history."""
    state: int = 0
    history: list[int] = [state]
    idx: int = 0
    while idx < len(events):
        state = transition(state, events[idx])
        history.append(state)
        idx = idx + 1
    return history


def count_time_in(history: list[int], target_state: int) -> int:
    """Count steps spent in a given state."""
    count: int = 0
    idx: int = 0
    while idx < len(history):
        if history[idx] == target_state:
            count = count + 1
        idx = idx + 1
    return count


def find_first(history: list[int], target_state: int) -> int:
    """Find first step where target state was reached. Returns -1 if never."""
    idx: int = 0
    while idx < len(history):
        if history[idx] == target_state:
            return idx
        idx = idx + 1
    return -1


def is_terminal(state_code: int) -> int:
    """Check if state is terminal. Returns 1 if terminal, 0 otherwise."""
    if state_code == 0 or state_code == 9:
        return 1
    return 0


def validate_sequence(events: list[int]) -> int:
    """Check if event sequence leads to terminal. Returns 1 if valid."""
    state: int = 0
    idx: int = 0
    while idx < len(events):
        state = transition(state, events[idx])
        idx = idx + 1
    return is_terminal(state)


def count_transitions(history: list[int]) -> int:
    """Count actual state transitions (changes)."""
    if len(history) <= 1:
        return 0
    changes: int = 0
    idx: int = 1
    while idx < len(history):
        if history[idx] != history[idx - 1]:
            changes = changes + 1
        idx = idx + 1
    return changes


def history_to_names(history: list[int]) -> list[str]:
    """Convert state code history to list of state names."""
    names: list[str] = []
    idx: int = 0
    while idx < len(history):
        nm: str = state_name(history[idx])
        names.append(nm)
        idx = idx + 1
    return names


def test_module() -> int:
    """Test state machine module."""
    passed: int = 0

    # Test 1: state names
    n0: str = state_name(0)
    n4: str = state_name(4)
    if n0 == "CLOSED" and n4 == "ESTABLISHED":
        passed = passed + 1

    # Test 2: basic transition
    s: int = transition(0, 0)
    if s == 1:
        passed = passed + 1

    # Test 3: full connection lifecycle
    events: list[int] = [1, 2, 3, 2, 3, 5]
    history: list[int] = run_protocol(events)
    last_state: int = history[len(history) - 1]
    if last_state == 0:
        passed = passed + 1

    # Test 4: count time in established
    time_est: int = count_time_in(history, 4)
    if time_est >= 1:
        passed = passed + 1

    # Test 5: find first established
    first_est: int = find_first(history, 4)
    if first_est > 0:
        passed = passed + 1

    # Test 6: terminal state check
    if is_terminal(0) == 1 and is_terminal(4) == 0:
        passed = passed + 1

    # Test 7: validate sequence
    valid_seq: list[int] = [1, 2, 3, 2, 3, 5]
    if validate_sequence(valid_seq) == 1:
        passed = passed + 1

    # Test 8: count transitions
    changes: int = count_transitions(history)
    if changes >= 4:
        passed = passed + 1

    return passed
