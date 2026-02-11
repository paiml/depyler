"""Simple hash table with chaining using dict for string keys.

Tests: put, get, delete, contains, size, keys list.
"""


def ht_create() -> dict[str, int]:
    """Create empty hash table."""
    return {}


def ht_put(table: dict[str, int], name: str, val: int) -> int:
    """Put key-value pair. Returns 1 if new, 0 if updated."""
    existed: int = 0
    if name in table:
        existed = 1
    table[name] = val
    if existed == 1:
        return 0
    return 1


def ht_get(table: dict[str, int], name: str, default_val: int) -> int:
    """Get value for key, or default if not present."""
    if name in table:
        return table[name]
    return default_val


def ht_contains(table: dict[str, int], name: str) -> int:
    """Return 1 if key exists, 0 otherwise."""
    if name in table:
        return 1
    return 0


def ht_delete(table: dict[str, int], name: str) -> int:
    """Delete key. Returns 1 if existed, 0 otherwise."""
    if name in table:
        del table[name]
        return 1
    return 0


def ht_increment(table: dict[str, int], name: str, amount: int) -> int:
    """Increment value by amount. Creates with amount if not present."""
    if name in table:
        table[name] = table[name] + amount
    else:
        table[name] = amount
    return table[name]


def ht_count_unique_chars(text: str) -> int:
    """Count unique characters using hash table."""
    seen: dict[str, int] = {}
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        seen[ch] = 1
        i = i + 1
    count: int = 0
    i2: int = 0
    while i2 < length:
        ch2: str = text[i2]
        if ch2 in seen:
            if seen[ch2] == 1:
                count = count + 1
                seen[ch2] = 0
        i2 = i2 + 1
    return count


def ht_most_frequent_char(text: str) -> str:
    """Find most frequent character. Returns first char if tie."""
    freq: dict[str, int] = {}
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
        i = i + 1
    best_ch: str = ""
    best_count: int = 0
    j: int = 0
    while j < length:
        ch2: str = text[j]
        if ch2 in freq:
            ct: int = freq[ch2]
            if ct > best_count:
                best_count = ct
                best_ch = ch2
                freq[ch2] = 0
        j = j + 1
    return best_ch


def test_module() -> int:
    """Test hash table operations."""
    passed: int = 0

    t: dict[str, int] = ht_create()
    r1: int = ht_put(t, "alice", 10)
    if r1 == 1:
        passed = passed + 1

    r2: int = ht_put(t, "alice", 20)
    if r2 == 0:
        passed = passed + 1

    if ht_get(t, "alice", 0) == 20:
        passed = passed + 1

    if ht_contains(t, "bob") == 0:
        passed = passed + 1

    ht_put(t, "bob", 5)
    d: int = ht_delete(t, "bob")
    if d == 1:
        passed = passed + 1

    if ht_contains(t, "bob") == 0:
        passed = passed + 1

    ht_increment(t, "counter", 3)
    ht_increment(t, "counter", 7)
    if ht_get(t, "counter", 0) == 10:
        passed = passed + 1

    uc: int = ht_count_unique_chars("abcabc")
    if uc == 3:
        passed = passed + 1

    return passed
