"""Array and queue operations using pure functions with lists.

No classes, no tuples, no imports. All state is explicit list/dict params.
"""


def make_array() -> list[int]:
    """Create a sample array."""
    arr: list[int] = [1, 2, 3, 4, 5]
    return arr


def array_append_test() -> list[int]:
    """Test appending to array."""
    arr: list[int] = [1, 2, 3]
    arr.append(4)
    arr.append(5)
    return arr


def array_extend_test() -> list[int]:
    """Test extending array manually."""
    arr: list[int] = [1, 2, 3]
    ext: list[int] = [4, 5, 6]
    i: int = 0
    while i < len(ext):
        val: int = ext[i]
        arr.append(val)
        i = i + 1
    return arr


def array_insert_test() -> list[int]:
    """Test inserting into array."""
    arr: list[int] = [1, 2, 4, 5]
    arr.insert(2, 3)
    return arr


def array_remove_test() -> list[int]:
    """Test removing from array."""
    arr: list[int] = [1, 2, 3, 4, 5]
    arr.remove(3)
    return arr


def array_pop_value(arr: list[int]) -> int:
    """Pop last element from array and return it."""
    if len(arr) == 0:
        return -1
    return arr.pop()


def array_index_test() -> int:
    """Find index of value 30."""
    arr: list[int] = [10, 20, 30, 40, 50]
    idx: int = arr.index(30)
    return idx


def array_count_test() -> int:
    """Count occurrences of 2."""
    arr: list[int] = [1, 2, 2, 3, 2, 4]
    c: int = arr.count(2)
    return c


def array_reverse_test() -> list[int]:
    """Reverse array."""
    arr: list[int] = [1, 2, 3, 4, 5]
    arr.reverse()
    return arr


def queue_put(items: list[int], val: int) -> list[int]:
    """Enqueue: append to end."""
    items.append(val)
    return items


def queue_get(items: list[int]) -> int:
    """Dequeue: remove and return first element. Returns -1 if empty."""
    if len(items) == 0:
        return -1
    item: int = items[0]
    new_items: list[int] = []
    i: int = 1
    while i < len(items):
        v: int = items[i]
        new_items.append(v)
        i = i + 1
    j: int = 0
    while j < len(new_items):
        if j == 0:
            items.clear()
        val: int = new_items[j]
        items.append(val)
        j = j + 1
    if len(new_items) == 0:
        items.clear()
    return item


def queue_size(items: list[int]) -> int:
    """Return queue size."""
    return len(items)


def queue_is_empty(items: list[int]) -> int:
    """Returns 1 if empty, 0 otherwise."""
    if len(items) == 0:
        return 1
    return 0


def stack_push(items: list[int], val: int) -> list[int]:
    """Push onto stack."""
    items.append(val)
    return items


def stack_pop(items: list[int]) -> int:
    """Pop from stack. Returns -1 if empty."""
    if len(items) == 0:
        return -1
    return items.pop()


def stack_peek(items: list[int]) -> int:
    """Peek at top of stack. Returns -1 if empty."""
    if len(items) == 0:
        return -1
    idx: int = len(items) - 1
    return items[idx]


def test_fifo_order() -> int:
    """Test FIFO: put 1,2,3 then get should return 1,2,3 in order."""
    q: list[int] = []
    q.append(1)
    q.append(2)
    q.append(3)
    results: list[int] = []
    first: int = queue_get(q)
    results.append(first)
    second: int = queue_get(q)
    results.append(second)
    third: int = queue_get(q)
    results.append(third)
    ok: int = 0
    if results[0] == 1:
        ok = ok + 1
    if results[1] == 2:
        ok = ok + 1
    if results[2] == 3:
        ok = ok + 1
    return ok


def test_lifo_order() -> int:
    """Test LIFO: push 1,2,3 then pop should return 3,2,1."""
    s: list[int] = []
    s.append(1)
    s.append(2)
    s.append(3)
    results: list[int] = []
    v1: int = stack_pop(s)
    results.append(v1)
    v2: int = stack_pop(s)
    results.append(v2)
    v3: int = stack_pop(s)
    results.append(v3)
    ok: int = 0
    if results[0] == 3:
        ok = ok + 1
    if results[1] == 2:
        ok = ok + 1
    if results[2] == 1:
        ok = ok + 1
    return ok


def circular_buffer(max_size: int) -> list[int]:
    """Simulate circular buffer: insert 1..8, keep only last max_size."""
    buf: list[int] = []
    val: int = 1
    while val <= 8:
        buf.append(val)
        if len(buf) > max_size:
            new_buf: list[int] = []
            k: int = 1
            while k < len(buf):
                v: int = buf[k]
                new_buf.append(v)
                k = k + 1
            buf = new_buf
        val = val + 1
    return buf


def priority_insert(priorities: list[int], vals: list[int], pri: int, val: int) -> int:
    """Insert into priority queue (two parallel lists). Returns new length."""
    priorities.append(pri)
    vals.append(val)
    n: int = len(priorities)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            pi: int = priorities[j]
            pii: int = priorities[i]
            if pi < pii:
                tmp_p: int = priorities[i]
                priorities[i] = priorities[j]
                priorities[j] = tmp_p
                tmp_v: int = vals[i]
                vals[i] = vals[j]
                vals[j] = tmp_v
            j = j + 1
        i = i + 1
    return n


def deque_sim() -> list[int]:
    """Simulate double-ended queue operations."""
    dq: list[int] = []
    dq.append(1)
    dq.append(2)
    dq.append(3)
    dq.insert(0, 0)
    dq.pop()
    if len(dq) > 0:
        new_dq: list[int] = []
        idx: int = 1
        while idx < len(dq):
            v: int = dq[idx]
            new_dq.append(v)
            idx = idx + 1
        dq = new_dq
    return dq


def test_module() -> int:
    """Test all array and queue features."""
    ok: int = 0

    arr: list[int] = make_array()
    if len(arr) == 5:
        ok = ok + 1
    if arr[0] == 1:
        ok = ok + 1

    app: list[int] = array_append_test()
    if len(app) == 5:
        ok = ok + 1
    if app[4] == 5:
        ok = ok + 1

    ext: list[int] = array_extend_test()
    if len(ext) == 6:
        ok = ok + 1
    if ext[5] == 6:
        ok = ok + 1

    ins: list[int] = array_insert_test()
    if ins[2] == 3:
        ok = ok + 1

    rem: list[int] = array_remove_test()
    if len(rem) == 4:
        ok = ok + 1

    pop_arr: list[int] = [10, 20, 30]
    popped: int = array_pop_value(pop_arr)
    if popped == 30:
        ok = ok + 1

    idx: int = array_index_test()
    if idx == 2:
        ok = ok + 1

    cnt: int = array_count_test()
    if cnt == 3:
        ok = ok + 1

    rev: list[int] = array_reverse_test()
    if rev[0] == 5:
        ok = ok + 1

    fifo: int = test_fifo_order()
    ok = ok + fifo

    lifo: int = test_lifo_order()
    ok = ok + lifo

    s: list[int] = []
    s.append(1)
    s.append(2)
    s.append(3)
    top: int = stack_peek(s)
    if top == 3:
        ok = ok + 1

    sz: int = queue_size(s)
    if sz == 3:
        ok = ok + 1

    circ: list[int] = circular_buffer(3)
    if len(circ) == 3:
        ok = ok + 1
    if circ[0] == 6:
        ok = ok + 1

    dsim: list[int] = deque_sim()
    if len(dsim) == 2:
        ok = ok + 1

    return ok
