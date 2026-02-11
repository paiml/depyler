"""Mark-compact garbage collector simulation.

Two-phase GC: mark reachable objects from roots, then compact
by sliding live objects to eliminate gaps.
"""


def gc_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def gc_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def gc_mark(reachable: list[int], refs: list[int], num_objects: int,
            roots: list[int], num_roots: int) -> int:
    """Mark reachable objects from roots. refs[i] = object i references.
    Returns count of marked objects."""
    i: int = 0
    while i < num_objects:
        reachable[i] = 0
        i = i + 1
    marked: int = 0
    stack: list[int] = gc_init_neg(num_objects)
    sp: int = 0
    r: int = 0
    while r < num_roots:
        root: int = roots[r]
        if root >= 0:
            if root < num_objects:
                stack[sp] = root
                sp = sp + 1
        r = r + 1
    while sp > 0:
        sp = sp - 1
        obj: int = stack[sp]
        m: int = reachable[obj]
        if m == 0:
            reachable[obj] = 1
            marked = marked + 1
            ref: int = refs[obj]
            if ref >= 0:
                if ref < num_objects:
                    rm: int = reachable[ref]
                    if rm == 0:
                        stack[sp] = ref
                        sp = sp + 1
    return marked


def gc_sweep_count(reachable: list[int], num_objects: int) -> int:
    """Count unreachable objects (garbage)."""
    count: int = 0
    i: int = 0
    while i < num_objects:
        r: int = reachable[i]
        if r == 0:
            count = count + 1
        i = i + 1
    return count


def gc_compact(objects: list[int], reachable: list[int],
               num_objects: int) -> int:
    """Compact: slide live objects to front. Returns new count of live objects."""
    write: int = 0
    read: int = 0
    while read < num_objects:
        r: int = reachable[read]
        if r == 1:
            obj_val: int = objects[read]
            objects[write] = obj_val
            write = write + 1
        read = read + 1
    clear: int = write
    while clear < num_objects:
        objects[clear] = 0
        clear = clear + 1
    return write


def gc_collect(objects: list[int], refs: list[int], num_objects: int,
               roots: list[int], num_roots: int) -> int:
    """Full GC cycle: mark + compact. Returns live object count."""
    reachable: list[int] = gc_init_zeros(num_objects)
    gc_mark(reachable, refs, num_objects, roots, num_roots)
    live: int = gc_compact(objects, reachable, num_objects)
    return live


def test_module() -> int:
    """Test mark-compact GC."""
    passed: int = 0
    num_obj: int = 8
    objects: list[int] = [10, 20, 30, 40, 50, 60, 70, 80]
    # refs: 0->1, 1->2, 2->-1, 3->4, 4->-1, 5->-1, 6->7, 7->-1
    refs: list[int] = [1, 2, 0 - 1, 4, 0 - 1, 0 - 1, 7, 0 - 1]
    reachable: list[int] = gc_init_zeros(num_obj)
    roots: list[int] = [0, 6]

    # Test 1: mark from roots
    marked: int = gc_mark(reachable, refs, num_obj, roots, 2)
    if marked == 5:
        passed = passed + 1

    # Test 2: garbage count
    garbage: int = gc_sweep_count(reachable, num_obj)
    if garbage == 3:
        passed = passed + 1

    # Test 3: compact
    objects2: list[int] = [10, 20, 30, 40, 50, 60, 70, 80]
    live: int = gc_compact(objects2, reachable, num_obj)
    if live == 5:
        passed = passed + 1

    # Test 4: live objects at front
    first: int = objects2[0]
    if first == 10:
        passed = passed + 1

    # Test 5: full GC cycle
    objects3: list[int] = [10, 20, 30, 40, 50, 60, 70, 80]
    refs3: list[int] = [1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1]
    roots3: list[int] = [0]
    live3: int = gc_collect(objects3, refs3, num_obj, roots3, 1)
    if live3 == 2:
        passed = passed + 1

    return passed
