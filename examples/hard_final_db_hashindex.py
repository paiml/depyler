"""Hash index with chaining for collision resolution.

Simulates a hash table with separate chaining using flat arrays.
Buckets store linked-list chains as parallel arrays.
"""


def hash_func(val: int, num_buckets: int) -> int:
    """Simple hash function for integer keys."""
    if val < 0:
        val = 0 - val
    return val % num_buckets


def create_index(num_buckets: int) -> list[int]:
    """Create empty hash index. bucket_heads[i] = -1 means empty."""
    heads: list[int] = []
    i: int = 0
    while i < num_buckets:
        heads.append(0 - 1)
        i = i + 1
    return heads


def insert_entry(bucket_heads: list[int], chain_keys: list[int], chain_vals: list[int], chain_next: list[int], insert_key: int, insert_val: int, num_buckets: int) -> int:
    """Insert key-value pair. Returns new chain length."""
    h: int = hash_func(insert_key, num_buckets)
    new_idx: int = len(chain_keys)
    chain_keys.append(insert_key)
    chain_vals.append(insert_val)
    old_head: int = bucket_heads[h]
    chain_next.append(old_head)
    bucket_heads[h] = new_idx
    return new_idx + 1


def lookup_entry(bucket_heads: list[int], chain_keys: list[int], chain_vals: list[int], chain_next: list[int], search_key: int, num_buckets: int) -> int:
    """Lookup value by key. Returns value or -1 if not found."""
    h: int = hash_func(search_key, num_buckets)
    idx: int = bucket_heads[h]
    while idx >= 0:
        ck: int = chain_keys[idx]
        if ck == search_key:
            return chain_vals[idx]
        idx = chain_next[idx]
    return 0 - 1


def delete_entry(bucket_heads: list[int], chain_keys: list[int], chain_vals: list[int], chain_next: list[int], del_key: int, num_buckets: int) -> int:
    """Delete entry by key. Returns 1 if deleted, 0 if not found.

    Marks key as -1 (tombstone).
    """
    h: int = hash_func(del_key, num_buckets)
    idx: int = bucket_heads[h]
    while idx >= 0:
        ck: int = chain_keys[idx]
        if ck == del_key:
            chain_keys[idx] = 0 - 1
            return 1
        idx = chain_next[idx]
    return 0


def count_bucket(bucket_heads: list[int], chain_next: list[int], chain_keys: list[int], bucket: int) -> int:
    """Count non-tombstone entries in a bucket chain."""
    cnt: int = 0
    idx: int = bucket_heads[bucket]
    while idx >= 0:
        ck: int = chain_keys[idx]
        if ck >= 0:
            cnt = cnt + 1
        idx = chain_next[idx]
    return cnt


def load_factor_pct(total_entries: int, num_buckets: int) -> int:
    """Load factor as percentage (0-infinity)."""
    if num_buckets == 0:
        return 0
    return total_entries * 100 // num_buckets


def test_module() -> int:
    """Test hash index."""
    ok: int = 0
    nb: int = 4
    heads: list[int] = create_index(nb)
    ck: list[int] = []
    cv: list[int] = []
    cn: list[int] = []
    insert_entry(heads, ck, cv, cn, 10, 100, nb)
    insert_entry(heads, ck, cv, cn, 20, 200, nb)
    insert_entry(heads, ck, cv, cn, 14, 140, nb)
    v10: int = lookup_entry(heads, ck, cv, cn, 10, nb)
    if v10 == 100:
        ok = ok + 1
    v20: int = lookup_entry(heads, ck, cv, cn, 20, nb)
    if v20 == 200:
        ok = ok + 1
    v14: int = lookup_entry(heads, ck, cv, cn, 14, nb)
    if v14 == 140:
        ok = ok + 1
    v99: int = lookup_entry(heads, ck, cv, cn, 99, nb)
    if v99 == 0 - 1:
        ok = ok + 1
    del_res: int = delete_entry(heads, ck, cv, cn, 10, nb)
    if del_res == 1:
        ok = ok + 1
    return ok
