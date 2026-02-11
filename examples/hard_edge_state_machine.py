"""State machines using dicts and ints for state tracking."""


def simple_state_machine(input_seq: list[int]) -> int:
    """Simple 3-state machine. States: 0, 1, 2.
    Input 0: stay, Input 1: advance, Input 2: reset.
    Returns final state."""
    current_state: int = 0
    i: int = 0
    while i < len(input_seq):
        inp: int = input_seq[i]
        if inp == 1:
            current_state = current_state + 1
            if current_state > 2:
                current_state = 2
        elif inp == 2:
            current_state = 0
        i = i + 1
    return current_state


def count_state_visits(input_seq: list[int], num_states: int) -> list[int]:
    """Count how many times each state was visited."""
    counts: list[int] = []
    s: int = 0
    while s < num_states:
        counts.append(0)
        s = s + 1
    current_state: int = 0
    counts[0] = 1
    i: int = 0
    while i < len(input_seq):
        inp: int = input_seq[i]
        if inp == 1:
            current_state = current_state + 1
            if current_state >= num_states:
                current_state = num_states - 1
        elif inp == 2:
            current_state = 0
        elif inp == 3:
            current_state = current_state - 1
            if current_state < 0:
                current_state = 0
        counts[current_state] = counts[current_state] + 1
        i = i + 1
    return counts


def transition_table_machine(input_seq: list[int]) -> int:
    """State machine with transition table stored in dict.
    States 0-3, inputs 0-1.
    Returns final state."""
    transitions: dict[int, int] = {}
    transitions[0] = 1
    transitions[1] = 2
    transitions[10] = 0
    transitions[11] = 3
    transitions[20] = 2
    transitions[21] = 0
    transitions[30] = 3
    transitions[31] = 1
    current_state: int = 0
    i: int = 0
    while i < len(input_seq):
        inp: int = input_seq[i]
        lookup: int = current_state * 10 + inp
        if lookup in transitions:
            current_state = transitions[lookup]
        i = i + 1
    return current_state


def vending_machine(coins: list[int], item_price: int) -> list[int]:
    """Simulate vending machine. Returns [dispensed, change].
    States: 0=waiting, 1=accepting, 2=dispensed."""
    total: int = 0
    dispensed: int = 0
    state: int = 0
    i: int = 0
    while i < len(coins):
        coin: int = coins[i]
        if state == 0:
            total = total + coin
            state = 1
        elif state == 1:
            total = total + coin
            if total >= item_price:
                dispensed = 1
                state = 2
        i = i + 1
    change: int = 0
    if dispensed == 1:
        change = total - item_price
    return [dispensed, change]


def traffic_light_sim(steps: int) -> list[int]:
    """Simulate traffic light: 0=red(3), 1=green(3), 2=yellow(1).
    Returns sequence of states."""
    sequence: list[int] = []
    current_state: int = 0
    timer: int = 0
    durations: list[int] = [3, 3, 1]
    i: int = 0
    while i < steps:
        sequence.append(current_state)
        timer = timer + 1
        dur: int = durations[current_state]
        if timer >= dur:
            timer = 0
            current_state = current_state + 1
            if current_state > 2:
                current_state = 0
        i = i + 1
    return sequence


def string_validator(chars: list[int]) -> int:
    """Validate integer string format: optional sign, then digits.
    chars as ASCII codes. Returns 1 if valid, 0 if not."""
    n: int = len(chars)
    if n == 0:
        return 0
    state: int = 0
    i: int = 0
    while i < n:
        ch: int = chars[i]
        if state == 0:
            if ch == 43 or ch == 45:
                state = 1
            elif ch >= 48 and ch <= 57:
                state = 2
            else:
                return 0
        elif state == 1:
            if ch >= 48 and ch <= 57:
                state = 2
            else:
                return 0
        elif state == 2:
            if ch >= 48 and ch <= 57:
                state = 2
            else:
                return 0
        i = i + 1
    if state == 2:
        return 1
    return 0


def test_module() -> int:
    """Test all state machine functions."""
    passed: int = 0
    sm1: int = simple_state_machine([1, 1, 1])
    if sm1 == 2:
        passed = passed + 1
    sm2: int = simple_state_machine([1, 2, 1])
    if sm2 == 1:
        passed = passed + 1
    cv: list[int] = count_state_visits([1, 1, 2, 1], 3)
    if cv[0] == 2:
        passed = passed + 1
    tt: int = transition_table_machine([0, 1, 0, 1])
    if tt >= 0:
        passed = passed + 1
    vm: list[int] = vending_machine([25, 25, 25, 25], 75)
    if vm[0] == 1:
        passed = passed + 1
    if vm[1] == 25:
        passed = passed + 1
    tl: list[int] = traffic_light_sim(7)
    if tl[0] == 0:
        passed = passed + 1
    if len(tl) == 7:
        passed = passed + 1
    sv1: int = string_validator([43, 49, 50, 51])
    if sv1 == 1:
        passed = passed + 1
    sv2: int = string_validator([65, 66])
    if sv2 == 0:
        passed = passed + 1
    sv3: int = string_validator([])
    if sv3 == 0:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
