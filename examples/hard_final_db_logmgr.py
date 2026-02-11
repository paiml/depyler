"""Write-Ahead Log (WAL) manager simulation.

Manages a sequential log of operations with LSN tracking,
checkpoint support, and undo/redo recovery simulation.
"""


def create_log() -> list[int]:
    """Create empty log. Format: [lsn, txn_id, op_type, old_val, new_val, ...]."""
    return []


def append_log_entry(log_data: list[int], txn_id: int, op_val: int, old_val: int, new_val: int) -> int:
    """Append entry to log. Returns LSN (entry number)."""
    lsn: int = len(log_data) // 5
    log_data.append(lsn)
    log_data.append(txn_id)
    log_data.append(op_val)
    log_data.append(old_val)
    log_data.append(new_val)
    return lsn


def get_log_lsn(log_data: list[int], entry: int) -> int:
    """Get LSN of log entry."""
    return log_data[entry * 5]


def get_log_txn(log_data: list[int], entry: int) -> int:
    """Get transaction ID of log entry."""
    return log_data[entry * 5 + 1]


def get_log_op(log_data: list[int], entry: int) -> int:
    """Get operation type of log entry."""
    return log_data[entry * 5 + 2]


def get_log_old(log_data: list[int], entry: int) -> int:
    """Get old value of log entry."""
    return log_data[entry * 5 + 3]


def get_log_new(log_data: list[int], entry: int) -> int:
    """Get new value of log entry."""
    return log_data[entry * 5 + 4]


def count_entries(log_data: list[int]) -> int:
    """Count number of log entries."""
    return len(log_data) // 5


def undo_transaction(log_data: list[int], txn_id: int, store: list[int]) -> int:
    """Undo all operations of a transaction (reverse order). Returns count of undone ops."""
    n: int = count_entries(log_data)
    undone: int = 0
    i: int = n - 1
    while i >= 0:
        tid: int = get_log_txn(log_data, i)
        if tid == txn_id:
            op_idx: int = get_log_op(log_data, i)
            old_v: int = get_log_old(log_data, i)
            if op_idx >= 0:
                if op_idx < len(store):
                    store[op_idx] = old_v
            undone = undone + 1
        i = i - 1
    return undone


def redo_transaction(log_data: list[int], txn_id: int, store: list[int]) -> int:
    """Redo all operations of a transaction (forward order). Returns count."""
    n: int = count_entries(log_data)
    redone: int = 0
    i: int = 0
    while i < n:
        tid: int = get_log_txn(log_data, i)
        if tid == txn_id:
            op_idx: int = get_log_op(log_data, i)
            new_v: int = get_log_new(log_data, i)
            if op_idx >= 0:
                if op_idx < len(store):
                    store[op_idx] = new_v
            redone = redone + 1
        i = i + 1
    return redone


def checkpoint_lsn(log_data: list[int], active_txns: list[int]) -> int:
    """Find safe checkpoint LSN: earliest LSN of any active transaction."""
    min_lsn: int = count_entries(log_data)
    n: int = count_entries(log_data)
    i: int = 0
    while i < n:
        tid: int = get_log_txn(log_data, i)
        j: int = 0
        while j < len(active_txns):
            at: int = active_txns[j]
            if tid == at:
                lsn: int = get_log_lsn(log_data, i)
                if lsn < min_lsn:
                    min_lsn = lsn
            j = j + 1
        i = i + 1
    return min_lsn


def test_module() -> int:
    """Test WAL manager."""
    ok: int = 0
    log_data: list[int] = create_log()
    lsn0: int = append_log_entry(log_data, 1, 0, 10, 20)
    lsn1: int = append_log_entry(log_data, 1, 1, 30, 40)
    lsn2: int = append_log_entry(log_data, 2, 0, 50, 60)
    if lsn0 == 0:
        ok = ok + 1
    if count_entries(log_data) == 3:
        ok = ok + 1
    store: list[int] = [20, 40, 0]
    undone: int = undo_transaction(log_data, 1, store)
    if undone == 2:
        ok = ok + 1
    sv0: int = store[0]
    if sv0 == 10:
        ok = ok + 1
    store2: list[int] = [0, 0, 0]
    redone: int = redo_transaction(log_data, 2, store2)
    if redone == 1:
        ok = ok + 1
    return ok
