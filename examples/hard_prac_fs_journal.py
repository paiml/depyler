"""File system journal (write-ahead log) simulation.

Implements transaction logging for crash recovery.
Journal entries record operations before committing to disk.
"""


def jrn_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def jrn_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def jrn_begin_txn(txn_ids: list[int], txn_count: list[int], txn_id: int) -> int:
    """Begin a transaction. Returns transaction slot."""
    idx: int = txn_count[0]
    txn_ids[idx] = txn_id
    txn_count[0] = idx + 1
    return idx


def jrn_log_op(ops: list[int], op_txn: list[int], op_types: list[int],
               op_targets: list[int], op_values: list[int],
               op_count: list[int], txn_id: int,
               op_kind: int, target: int, value: int) -> int:
    """Log an operation. op_kind: 0=write, 1=delete, 2=mkdir.
    Returns op index."""
    idx: int = op_count[0]
    op_txn[idx] = txn_id
    op_types[idx] = op_kind
    op_targets[idx] = target
    op_values[idx] = value
    op_count[0] = idx + 1
    return idx


def jrn_commit_txn(committed: list[int], txn_slot: int) -> int:
    """Mark transaction as committed. Returns 1."""
    committed[txn_slot] = 1
    return 1


def jrn_is_committed(committed: list[int], txn_slot: int) -> int:
    """Check if transaction is committed. Returns 1 if yes."""
    c: int = committed[txn_slot]
    return c


def jrn_replay(op_txn: list[int], op_types: list[int], op_targets: list[int],
               op_values: list[int], op_count: int,
               committed: list[int], txn_ids: list[int], txn_count: int,
               disk: list[int], disk_size: int) -> int:
    """Replay committed operations to disk. Returns count of replayed ops."""
    replayed: int = 0
    i: int = 0
    while i < op_count:
        tid: int = op_txn[i]
        slot: int = 0 - 1
        j: int = 0
        while j < txn_count:
            t: int = txn_ids[j]
            if t == tid:
                slot = j
                j = txn_count
            j = j + 1
        if slot >= 0:
            c: int = committed[slot]
            if c == 1:
                target: int = op_targets[i]
                value: int = op_values[i]
                if target >= 0:
                    if target < disk_size:
                        disk[target] = value
                replayed = replayed + 1
        i = i + 1
    return replayed


def jrn_count_ops(op_txn: list[int], op_count: int, txn_id: int) -> int:
    """Count operations in a transaction."""
    total: int = 0
    i: int = 0
    while i < op_count:
        tid: int = op_txn[i]
        if tid == txn_id:
            total = total + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test journal operations."""
    passed: int = 0
    cap: int = 20
    txn_ids: list[int] = jrn_init(cap)
    committed: list[int] = jrn_init_zeros(cap)
    txn_cnt: list[int] = [0]
    op_txn: list[int] = jrn_init(cap)
    op_types: list[int] = jrn_init(cap)
    op_targets: list[int] = jrn_init(cap)
    op_values: list[int] = jrn_init(cap)
    op_cnt: list[int] = [0]
    disk: list[int] = jrn_init_zeros(10)

    # Test 1: begin transaction
    slot: int = jrn_begin_txn(txn_ids, txn_cnt, 100)
    if slot == 0:
        passed = passed + 1

    # Test 2: log operations
    jrn_log_op(op_txn, op_txn, op_types, op_targets, op_values, op_cnt, 100, 0, 0, 42)
    jrn_log_op(op_txn, op_txn, op_types, op_targets, op_values, op_cnt, 100, 0, 1, 99)
    count: int = jrn_count_ops(op_txn, op_cnt[0], 100)
    if count == 2:
        passed = passed + 1

    # Test 3: uncommitted transaction not replayed
    replayed: int = jrn_replay(op_txn, op_types, op_targets, op_values, op_cnt[0], committed, txn_ids, txn_cnt[0], disk, 10)
    if replayed == 0:
        passed = passed + 1

    # Test 4: commit and replay
    jrn_commit_txn(committed, slot)
    replayed2: int = jrn_replay(op_txn, op_types, op_targets, op_values, op_cnt[0], committed, txn_ids, txn_cnt[0], disk, 10)
    if replayed2 == 2:
        passed = passed + 1

    # Test 5: disk has correct values after replay
    d0: int = disk[0]
    d1: int = disk[1]
    if d0 == 42:
        if d1 == 99:
            passed = passed + 1

    return passed
