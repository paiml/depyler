"""Lock-free stack simulation using compare-and-swap.

Simulates a Treiber stack with CAS operations on a simulated single-threaded model.
Stack nodes stored in flat arrays with next-pointer indices.
"""


def create_stack() -> list[int]:
    """Create empty stack. Returns [top_index] where -1 = empty."""
    return [0 - 1]


def create_node_pool(capacity: int) -> list[int]:
    """Create node pool. nodes[i*2] = value, nodes[i*2+1] = next_ptr."""
    pool: list[int] = []
    i: int = 0
    while i < capacity * 2:
        pool.append(0)
        i = i + 1
    return pool


def cas_push(stack_top: list[int], node_pool: list[int], alloc_idx: list[int], value: int) -> int:
    """CAS push: allocate node, set next to current top, CAS top. Returns 1 on success."""
    idx: int = alloc_idx[0]
    alloc_idx[0] = idx + 1
    node_pool[idx * 2] = value
    old_top: int = stack_top[0]
    node_pool[idx * 2 + 1] = old_top
    stack_top[0] = idx
    return 1


def cas_pop(stack_top: list[int], node_pool: list[int]) -> int:
    """CAS pop: read top, CAS top to top.next. Returns value or -1 if empty."""
    old_top: int = stack_top[0]
    if old_top < 0:
        return 0 - 1
    value: int = node_pool[old_top * 2]
    next_ptr: int = node_pool[old_top * 2 + 1]
    stack_top[0] = next_ptr
    return value


def stack_size(stack_top: list[int], node_pool: list[int]) -> int:
    """Count elements in stack by traversing."""
    cnt: int = 0
    current: int = stack_top[0]
    while current >= 0:
        cnt = cnt + 1
        current = node_pool[current * 2 + 1]
        if cnt > 10000:
            return cnt
    return cnt


def stack_to_list(stack_top: list[int], node_pool: list[int]) -> list[int]:
    """Convert stack to list (top to bottom)."""
    result: list[int] = []
    current: int = stack_top[0]
    while current >= 0:
        val: int = node_pool[current * 2]
        result.append(val)
        current = node_pool[current * 2 + 1]
        if len(result) > 10000:
            return result
    return result


def stack_contains(stack_top: list[int], node_pool: list[int], target: int) -> int:
    """Check if stack contains value. Returns 1 if found."""
    current: int = stack_top[0]
    while current >= 0:
        val: int = node_pool[current * 2]
        if val == target:
            return 1
        current = node_pool[current * 2 + 1]
    return 0


def test_module() -> int:
    """Test lock-free stack."""
    ok: int = 0
    st: list[int] = create_stack()
    pool: list[int] = create_node_pool(10)
    alloc: list[int] = [0]
    cas_push(st, pool, alloc, 10)
    cas_push(st, pool, alloc, 20)
    cas_push(st, pool, alloc, 30)
    if stack_size(st, pool) == 3:
        ok = ok + 1
    v: int = cas_pop(st, pool)
    if v == 30:
        ok = ok + 1
    if stack_size(st, pool) == 2:
        ok = ok + 1
    if stack_contains(st, pool, 10) == 1:
        ok = ok + 1
    items: list[int] = stack_to_list(st, pool)
    if len(items) == 2:
        ok = ok + 1
    return ok
