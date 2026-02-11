"""1D cellular automaton (Rule 30 and Rule 110)."""


def get_rule_bit(rule_number: int, pattern: int) -> int:
    """Get the output bit for a given 3-bit pattern from rule number."""
    return (rule_number >> pattern) & 1


def step_automaton(cells: list[int], rule_number: int) -> list[int]:
    """Compute one step of 1D cellular automaton."""
    length: int = len(cells)
    new_cells: list[int] = []
    i: int = 0
    while i < length:
        left: int = 0
        if i > 0:
            left = cells[i - 1]
        center: int = cells[i]
        right_idx: int = i + 1
        right: int = 0
        if right_idx < length:
            right = cells[right_idx]
        pattern: int = (left << 2) | (center << 1) | right
        new_val: int = get_rule_bit(rule_number, pattern)
        new_cells.append(new_val)
        i = i + 1
    return new_cells


def run_automaton(cells: list[int], rule_number: int, generations: int) -> list[int]:
    """Run automaton for multiple generations. Returns final state."""
    current: list[int] = []
    i: int = 0
    length: int = len(cells)
    while i < length:
        current.append(cells[i])
        i = i + 1
    gen: int = 0
    while gen < generations:
        current = step_automaton(current, rule_number)
        gen = gen + 1
    return current


def count_alive_cells(cells: list[int]) -> int:
    """Count the number of alive (1) cells."""
    total: int = 0
    i: int = 0
    length: int = len(cells)
    while i < length:
        total = total + cells[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test cellular automaton operations."""
    passed: int = 0

    if get_rule_bit(30, 0) == 0:
        passed = passed + 1

    if get_rule_bit(30, 1) == 1:
        passed = passed + 1

    if get_rule_bit(30, 4) == 1:
        passed = passed + 1

    initial: list[int] = [0, 0, 0, 1, 0, 0, 0]
    gen1: list[int] = step_automaton(initial, 30)
    if gen1[2] == 1 and gen1[3] == 1 and gen1[4] == 1:
        passed = passed + 1

    if count_alive_cells(initial) == 1:
        passed = passed + 1

    if count_alive_cells(gen1) > 1:
        passed = passed + 1

    final: list[int] = run_automaton(initial, 30, 3)
    if len(final) == 7:
        passed = passed + 1

    if get_rule_bit(110, 1) == 1:
        passed = passed + 1

    return passed
