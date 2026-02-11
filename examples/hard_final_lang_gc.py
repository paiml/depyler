"""Mark-and-sweep garbage collector simulation.

Objects stored in a heap as flat arrays. Each object has:
  - A mark bit
  - Reference count
  - List of references to other objects (indices)
Root set determines reachable objects.
"""


def create_heap(num_objects: int) -> list[int]:
    """Create heap: marks[0..n-1], alive[n..2n-1]. All alive, unmarked."""
    result: list[int] = []
    i: int = 0
    while i < num_objects:
        result.append(0)
        i = i + 1
    j: int = 0
    while j < num_objects:
        result.append(1)
        j = j + 1
    return result


def add_reference(ref_table: list[int], from_obj: int, to_obj: int) -> int:
    """Add reference edge. ref_table is flat: [from, to, from, to, ...]."""
    ref_table.append(from_obj)
    ref_table.append(to_obj)
    return len(ref_table) // 2


def mark_phase(heap: list[int], ref_table: list[int], roots: list[int], num_objects: int) -> int:
    """Mark all reachable objects from roots. Returns count of marked objects."""
    i: int = 0
    while i < num_objects:
        heap[i] = 0
        i = i + 1
    worklist: list[int] = []
    ri: int = 0
    while ri < len(roots):
        rv: int = roots[ri]
        heap[rv] = 1
        worklist.append(rv)
        ri = ri + 1
    while len(worklist) > 0:
        current: int = worklist[len(worklist) - 1]
        worklist.pop()
        ti: int = 0
        while ti < len(ref_table) // 2:
            from_obj: int = ref_table[ti * 2]
            to_obj: int = ref_table[ti * 2 + 1]
            if from_obj == current:
                mark_val: int = heap[to_obj]
                if mark_val == 0:
                    heap[to_obj] = 1
                    worklist.append(to_obj)
            ti = ti + 1
    marked: int = 0
    k: int = 0
    while k < num_objects:
        mk: int = heap[k]
        if mk == 1:
            marked = marked + 1
        k = k + 1
    return marked


def sweep_phase(heap: list[int], num_objects: int) -> int:
    """Sweep unmarked objects. Returns count of freed objects."""
    freed: int = 0
    i: int = 0
    while i < num_objects:
        mark_val: int = heap[i]
        alive_val: int = heap[num_objects + i]
        if mark_val == 0:
            if alive_val == 1:
                heap[num_objects + i] = 0
                freed = freed + 1
        i = i + 1
    return freed


def collect(heap: list[int], ref_table: list[int], roots: list[int], num_objects: int) -> list[int]:
    """Full GC cycle. Returns [marked, freed]."""
    marked: int = mark_phase(heap, ref_table, roots, num_objects)
    freed: int = sweep_phase(heap, num_objects)
    return [marked, freed]


def count_alive(heap: list[int], num_objects: int) -> int:
    """Count living objects."""
    cnt: int = 0
    i: int = 0
    while i < num_objects:
        av: int = heap[num_objects + i]
        if av == 1:
            cnt = cnt + 1
        i = i + 1
    return cnt


def ref_count_increment(ref_counts: list[int], obj: int) -> int:
    """Increment reference count."""
    old: int = ref_counts[obj]
    ref_counts[obj] = old + 1
    return old + 1


def ref_count_decrement(ref_counts: list[int], obj: int) -> int:
    """Decrement reference count. Returns new count."""
    old: int = ref_counts[obj]
    if old > 0:
        ref_counts[obj] = old - 1
        return old - 1
    return 0


def test_module() -> int:
    """Test garbage collector."""
    ok: int = 0
    n: int = 5
    heap: list[int] = create_heap(n)
    refs: list[int] = []
    add_reference(refs, 0, 1)
    add_reference(refs, 1, 2)
    roots: list[int] = [0]
    result: list[int] = collect(heap, refs, roots, n)
    marked: int = result[0]
    freed: int = result[1]
    if marked == 3:
        ok = ok + 1
    if freed == 2:
        ok = ok + 1
    alive: int = count_alive(heap, n)
    if alive == 3:
        ok = ok + 1
    rc: list[int] = [0, 0, 0]
    ref_count_increment(rc, 1)
    ref_count_increment(rc, 1)
    r1: int = rc[1]
    if r1 == 2:
        ok = ok + 1
    ref_count_decrement(rc, 1)
    r1b: int = rc[1]
    if r1b == 1:
        ok = ok + 1
    return ok
