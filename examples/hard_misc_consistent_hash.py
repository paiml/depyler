def simple_hash(val: int, ring_size: int) -> int:
    h: int = ((val * 2654435761) % ring_size)
    if h < 0:
        h = 0 - h
    return h

def create_ring(nodes: list[int], ring_size: int) -> list[int]:
    ring: list[int] = []
    i: int = 0
    while i < ring_size:
        ring.append(0 - 1)
        i = i + 1
    j: int = 0
    nn: int = len(nodes)
    while j < nn:
        pos: int = simple_hash(nodes[j], ring_size)
        ring[pos] = nodes[j]
        j = j + 1
    return ring

def find_node(ring: list[int], item_hash: int) -> int:
    size: int = len(ring)
    pos: int = item_hash % size
    i: int = 0
    while i < size:
        idx: int = (pos + i) % size
        v: int = ring[idx]
        if v >= 0:
            return v
        i = i + 1
    return 0 - 1

def assign_item(ring: list[int], item: int, ring_size: int) -> int:
    h: int = simple_hash(item, ring_size)
    return find_node(ring, h)

def count_assignments(ring: list[int], items: list[int], ring_size: int, target_node: int) -> int:
    count: int = 0
    n: int = len(items)
    i: int = 0
    while i < n:
        assigned: int = assign_item(ring, items[i], ring_size)
        if assigned == target_node:
            count = count + 1
        i = i + 1
    return count

def ring_utilization(ring: list[int]) -> float:
    size: int = len(ring)
    used: int = 0
    i: int = 0
    while i < size:
        v: int = ring[i]
        if v >= 0:
            used = used + 1
        i = i + 1
    return used * 1.0 / (size * 1.0)

def test_module() -> int:
    passed: int = 0
    h: int = simple_hash(42, 100)
    if h >= 0 and h < 100:
        passed = passed + 1
    nodes: list[int] = [1, 2, 3]
    ring: list[int] = create_ring(nodes, 100)
    nr: int = len(ring)
    if nr == 100:
        passed = passed + 1
    n: int = find_node(ring, 0)
    if n >= 0:
        passed = passed + 1
    util: float = ring_utilization(ring)
    if util > 0.0:
        passed = passed + 1
    a: int = assign_item(ring, 99, 100)
    if a >= 0:
        passed = passed + 1
    return passed
