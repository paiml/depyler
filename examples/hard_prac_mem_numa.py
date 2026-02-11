"""NUMA-aware memory allocator simulation.

Allocates from local NUMA node first. Falls back to remote nodes
with distance-based cost tracking.
"""


def numa_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def numa_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def numa_alloc_local(node_free: list[int], node_id: int, amount: int) -> int:
    """Try to allocate from local node. Returns 1 on success, 0 if insufficient."""
    avail: int = node_free[node_id]
    if avail >= amount:
        node_free[node_id] = avail - amount
        return 1
    return 0


def numa_alloc_remote(node_free: list[int], num_nodes: int,
                      local_node: int, amount: int) -> int:
    """Allocate from nearest remote node. Returns node_id or -1."""
    i: int = 0
    while i < num_nodes:
        if i != local_node:
            avail: int = node_free[i]
            if avail >= amount:
                node_free[i] = avail - amount
                return i
        i = i + 1
    return 0 - 1


def numa_alloc(node_free: list[int], num_nodes: int,
               local_node: int, amount: int, stats: list[int]) -> int:
    """Allocate with NUMA awareness.
    stats[0]=local_allocs, stats[1]=remote_allocs.
    Returns source node or -1."""
    ok: int = numa_alloc_local(node_free, local_node, amount)
    if ok == 1:
        stats[0] = stats[0] + 1
        return local_node
    remote: int = numa_alloc_remote(node_free, num_nodes, local_node, amount)
    if remote >= 0:
        stats[1] = stats[1] + 1
    return remote


def numa_free_to_node(node_free: list[int], node_id: int, amount: int) -> int:
    """Return memory to a node. Returns new free amount."""
    node_free[node_id] = node_free[node_id] + amount
    result: int = node_free[node_id]
    return result


def numa_distance(from_node: int, to_node: int) -> int:
    """NUMA distance: 10 for local, 20 for adjacent, 30 for far."""
    if from_node == to_node:
        return 10
    diff: int = from_node - to_node
    if diff < 0:
        diff = 0 - diff
    if diff == 1:
        return 20
    return 30


def numa_total_free(node_free: list[int], num_nodes: int) -> int:
    """Total free memory across all nodes."""
    total: int = 0
    i: int = 0
    while i < num_nodes:
        f: int = node_free[i]
        total = total + f
        i = i + 1
    return total


def test_module() -> int:
    """Test NUMA allocator."""
    passed: int = 0
    num_nodes: int = 4
    node_free: list[int] = [1000, 1000, 1000, 1000]
    stats: list[int] = [0, 0]

    # Test 1: local allocation
    src: int = numa_alloc(node_free, num_nodes, 0, 200, stats)
    if src == 0:
        passed = passed + 1

    # Test 2: verify local node decreased
    f0: int = node_free[0]
    if f0 == 800:
        passed = passed + 1

    # Test 3: exhaust local and fallback to remote
    numa_alloc(node_free, num_nodes, 0, 800, stats)
    src2: int = numa_alloc(node_free, num_nodes, 0, 100, stats)
    if src2 == 1:
        passed = passed + 1

    # Test 4: NUMA distance
    d_local: int = numa_distance(0, 0)
    d_adj: int = numa_distance(0, 1)
    if d_local == 10:
        if d_adj == 20:
            passed = passed + 1

    # Test 5: stats tracking
    local_cnt: int = stats[0]
    remote_cnt: int = stats[1]
    if local_cnt == 2:
        if remote_cnt == 1:
            passed = passed + 1

    return passed
