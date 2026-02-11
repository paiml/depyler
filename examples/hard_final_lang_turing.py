"""Turing machine simulation with finite tape.

Simulates a single-tape Turing machine. Tape is a list[int].
Transitions stored as flat array: [state, read_sym, write_sym, move, next_state, ...].
Move: 0=left, 1=right, 2=stay.
"""


def tm_create_tape(size: int, blank: int) -> list[int]:
    """Create tape filled with blank symbol."""
    tape: list[int] = []
    i: int = 0
    while i < size:
        tape.append(blank)
        i = i + 1
    return tape


def tm_write_input(tape: list[int], input_data: list[int], start_pos: int) -> int:
    """Write input data to tape starting at position. Returns length written."""
    i: int = 0
    while i < len(input_data):
        iv: int = input_data[i]
        tape[start_pos + i] = iv
        i = i + 1
    return len(input_data)


def tm_find_transition(trans: list[int], state: int, read_sym: int) -> int:
    """Find transition index for given state and read symbol. Returns -1 if not found."""
    n: int = len(trans) // 5
    i: int = 0
    while i < n:
        ts: int = trans[i * 5]
        tr: int = trans[i * 5 + 1]
        if ts == state:
            if tr == read_sym:
                return i
        i = i + 1
    return 0 - 1


def tm_step(tape: list[int], trans: list[int], state: int, head: int) -> list[int]:
    """Execute one TM step. Returns [new_state, new_head, halted].

    halted: 1 if no transition found (halt).
    """
    read_sym: int = tape[head]
    ti: int = tm_find_transition(trans, state, read_sym)
    if ti < 0:
        return [state, head, 1]
    write_sym: int = trans[ti * 5 + 2]
    move_dir: int = trans[ti * 5 + 3]
    next_state: int = trans[ti * 5 + 4]
    tape[head] = write_sym
    new_head: int = head
    if move_dir == 0:
        new_head = head - 1
        if new_head < 0:
            new_head = 0
    if move_dir == 1:
        new_head = head + 1
        if new_head >= len(tape):
            new_head = len(tape) - 1
    return [next_state, new_head, 0]


def tm_run(tape: list[int], trans: list[int], start_state: int, head: int, max_steps: int) -> list[int]:
    """Run TM for at most max_steps. Returns [final_state, final_head, steps_taken]."""
    state: int = start_state
    steps: int = 0
    while steps < max_steps:
        result: list[int] = tm_step(tape, trans, state, head)
        state = result[0]
        head = result[1]
        halted: int = result[2]
        if halted == 1:
            return [state, head, steps]
        steps = steps + 1
    return [state, head, steps]


def tm_read_output(tape: list[int], start: int, length: int) -> list[int]:
    """Read a section of tape."""
    result: list[int] = []
    i: int = 0
    while i < length:
        tv: int = tape[start + i]
        result.append(tv)
        i = i + 1
    return result


def test_module() -> int:
    """Test Turing machine. Simple TM that replaces all 1s with 2s, then halts."""
    ok: int = 0
    tape: list[int] = tm_create_tape(10, 0)
    tm_write_input(tape, [1, 1, 1], 0)
    trans: list[int] = [0, 1, 2, 1, 0, 0, 0, 0, 2, 1]
    result: list[int] = tm_run(tape, trans, 0, 0, 100)
    final_state: int = result[0]
    if final_state == 1:
        ok = ok + 1
    t0: int = tape[0]
    t1: int = tape[1]
    t2: int = tape[2]
    if t0 == 2:
        ok = ok + 1
    if t1 == 2:
        ok = ok + 1
    if t2 == 2:
        ok = ok + 1
    steps: int = result[2]
    if steps > 0:
        ok = ok + 1
    return ok
