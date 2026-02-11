"""Message Authentication Code (MAC) using hash-based construction.

Implements HMAC-like construction: MAC = H(key || message || key).
Uses simple integer hash for demonstration.
"""


def simple_mix(state: int, word: int) -> int:
    """Mix a word into state."""
    state = (state * 31 + word) % 65536
    state = state + (state // 128)
    return state % 65536


def compute_hash(data: list[int]) -> int:
    """Simple hash of integer array to single 16-bit value."""
    state: int = 5381
    i: int = 0
    while i < len(data):
        dv: int = data[i]
        state = simple_mix(state, dv)
        i = i + 1
    return state % 65536


def compute_mac(msg: list[int], mac_key: list[int]) -> int:
    """Compute MAC = H(key || msg || key)."""
    combined: list[int] = []
    i: int = 0
    while i < len(mac_key):
        kv: int = mac_key[i]
        combined.append(kv)
        i = i + 1
    j: int = 0
    while j < len(msg):
        mv: int = msg[j]
        combined.append(mv)
        j = j + 1
    k: int = 0
    while k < len(mac_key):
        kv2: int = mac_key[k]
        combined.append(kv2)
        k = k + 1
    return compute_hash(combined)


def verify_mac(msg: list[int], mac_key: list[int], expected_mac: int) -> int:
    """Verify MAC. Returns 1 if valid, 0 otherwise."""
    computed: int = compute_mac(msg, mac_key)
    if computed == expected_mac:
        return 1
    return 0


def truncate_mac(full_mac: int, num_bits: int) -> int:
    """Truncate MAC to fewer bits."""
    mask: int = 1
    i: int = 0
    while i < num_bits:
        mask = mask * 2
        i = i + 1
    mask = mask - 1
    return full_mac % (mask + 1)


def keyed_hash_chain(msg: list[int], mac_key: list[int], rounds: int) -> int:
    """Apply MAC repeatedly for key stretching."""
    current: int = compute_mac(msg, mac_key)
    r: int = 1
    while r < rounds:
        msg_list: list[int] = [current]
        current = compute_mac(msg_list, mac_key)
        r = r + 1
    return current


def test_module() -> int:
    """Test MAC implementation."""
    ok: int = 0
    mac_key: list[int] = [42, 99, 17]
    msg: list[int] = [1, 2, 3, 4, 5]
    mac1: int = compute_mac(msg, mac_key)
    mac2: int = compute_mac(msg, mac_key)
    if mac1 == mac2:
        ok = ok + 1
    if verify_mac(msg, mac_key, mac1) == 1:
        ok = ok + 1
    if verify_mac(msg, mac_key, mac1 + 1) == 0:
        ok = ok + 1
    diff_msg: list[int] = [1, 2, 3, 4, 6]
    mac3: int = compute_mac(diff_msg, mac_key)
    if mac3 != mac1:
        ok = ok + 1
    trunc: int = truncate_mac(65535, 8)
    if trunc == 255:
        ok = ok + 1
    return ok
