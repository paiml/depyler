"""Bytecode virtual machine with stack-based execution.

Opcodes (integers):
  0 = PUSH (next word is value)
  1 = POP
  2 = ADD
  3 = SUB
  4 = MUL
  5 = DIV
  6 = DUP
  7 = SWAP
  8 = JMP (next word is target)
  9 = JZ (next word is target, jump if top == 0)
  10 = HALT
  11 = LOAD (next word is local var index)
  12 = STORE (next word is local var index)
  13 = CMP_LT (push 1 if a < b)
"""


def vm_create(stack_size: int, num_locals: int) -> list[int]:
    """Create VM state: stack + locals + [sp, ip]."""
    state: list[int] = []
    i: int = 0
    while i < stack_size:
        state.append(0)
        i = i + 1
    j: int = 0
    while j < num_locals:
        state.append(0)
        j = j + 1
    state.append(0)
    state.append(0)
    return state


def vm_push(state: list[int], stack_size: int, value: int) -> int:
    """Push value onto stack. Returns new sp."""
    sp_idx: int = len(state) - 2
    sp: int = state[sp_idx]
    state[sp] = value
    state[sp_idx] = sp + 1
    return sp + 1


def vm_pop(state: list[int], stack_size: int) -> int:
    """Pop value from stack. Returns popped value."""
    sp_idx: int = len(state) - 2
    sp: int = state[sp_idx]
    state[sp_idx] = sp - 1
    return state[sp - 1]


def vm_run(bytecode: list[int], stack_size: int, num_locals: int, max_steps: int) -> list[int]:
    """Execute bytecode. Returns [top_of_stack, steps_executed]."""
    state: list[int] = vm_create(stack_size, num_locals)
    sp_idx: int = len(state) - 2
    ip_idx: int = len(state) - 1
    locals_start: int = stack_size
    steps: int = 0
    while steps < max_steps:
        ip: int = state[ip_idx]
        if ip >= len(bytecode):
            sp: int = state[sp_idx]
            if sp > 0:
                return [state[sp - 1], steps]
            return [0, steps]
        op: int = bytecode[ip]
        if op == 10:
            sp2: int = state[sp_idx]
            if sp2 > 0:
                return [state[sp2 - 1], steps]
            return [0, steps]
        if op == 0:
            val: int = bytecode[ip + 1]
            vm_push(state, stack_size, val)
            state[ip_idx] = ip + 2
        if op == 1:
            vm_pop(state, stack_size)
            state[ip_idx] = ip + 1
        if op == 2:
            b: int = vm_pop(state, stack_size)
            a: int = vm_pop(state, stack_size)
            vm_push(state, stack_size, a + b)
            state[ip_idx] = ip + 1
        if op == 3:
            b2: int = vm_pop(state, stack_size)
            a2: int = vm_pop(state, stack_size)
            vm_push(state, stack_size, a2 - b2)
            state[ip_idx] = ip + 1
        if op == 4:
            b3: int = vm_pop(state, stack_size)
            a3: int = vm_pop(state, stack_size)
            vm_push(state, stack_size, a3 * b3)
            state[ip_idx] = ip + 1
        if op == 5:
            b4: int = vm_pop(state, stack_size)
            a4: int = vm_pop(state, stack_size)
            if b4 != 0:
                vm_push(state, stack_size, a4 // b4)
            else:
                vm_push(state, stack_size, 0)
            state[ip_idx] = ip + 1
        if op == 6:
            sp3: int = state[sp_idx]
            top: int = state[sp3 - 1]
            vm_push(state, stack_size, top)
            state[ip_idx] = ip + 1
        if op == 8:
            target: int = bytecode[ip + 1]
            state[ip_idx] = target
        if op == 9:
            cond: int = vm_pop(state, stack_size)
            if cond == 0:
                state[ip_idx] = bytecode[ip + 1]
            else:
                state[ip_idx] = ip + 2
        if op == 11:
            loc_idx: int = bytecode[ip + 1]
            lv: int = state[locals_start + loc_idx]
            vm_push(state, stack_size, lv)
            state[ip_idx] = ip + 2
        if op == 12:
            loc_idx2: int = bytecode[ip + 1]
            sv: int = vm_pop(state, stack_size)
            state[locals_start + loc_idx2] = sv
            state[ip_idx] = ip + 2
        steps = steps + 1
    sp_final: int = state[sp_idx]
    if sp_final > 0:
        return [state[sp_final - 1], steps]
    return [0, steps]


def test_module() -> int:
    """Test bytecode VM."""
    ok: int = 0
    code1: list[int] = [0, 3, 0, 4, 2, 10]
    res1: list[int] = vm_run(code1, 16, 4, 100)
    v1: int = res1[0]
    if v1 == 7:
        ok = ok + 1
    code2: list[int] = [0, 10, 0, 3, 3, 10]
    res2: list[int] = vm_run(code2, 16, 4, 100)
    v2: int = res2[0]
    if v2 == 7:
        ok = ok + 1
    code3: list[int] = [0, 5, 0, 6, 4, 10]
    res3: list[int] = vm_run(code3, 16, 4, 100)
    v3: int = res3[0]
    if v3 == 30:
        ok = ok + 1
    code4: list[int] = [0, 42, 12, 0, 11, 0, 10]
    res4: list[int] = vm_run(code4, 16, 4, 100)
    v4: int = res4[0]
    if v4 == 42:
        ok = ok + 1
    s1: int = res1[1]
    if s1 > 0:
        ok = ok + 1
    return ok
