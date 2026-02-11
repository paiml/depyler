"""Collections patterns using pure functions, no imports.

Implements counter, group-by-length, queue processing, and sliding window
without any stdlib imports or class definitions.
"""


def count_chars(text: str) -> dict[str, int]:
    """Count character frequencies in text."""
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
    return freq


def count_word_lengths(words: list[str]) -> dict[str, int]:
    """Count how many words have each length. Key is length as string."""
    counts: dict[str, int] = {}
    i: int = 0
    while i < len(words):
        w: str = words[i]
        wlen: int = len(w)
        key: str = str(wlen)
        if key in counts:
            counts[key] = counts[key] + 1
        else:
            counts[key] = 1
        i = i + 1
    return counts


def process_queue_alt(items: list[int]) -> list[int]:
    """Process items alternating pop-front / pop-back based on remaining size."""
    buf: list[int] = []
    k: int = 0
    while k < len(items):
        v: int = items[k]
        buf.append(v)
        k = k + 1
    results: list[int] = []
    while len(buf) > 0:
        remaining: int = len(buf)
        if remaining % 2 == 0:
            first: int = buf[0]
            results.append(first)
            new_buf: list[int] = []
            m: int = 1
            while m < len(buf):
                bv: int = buf[m]
                new_buf.append(bv)
                m = m + 1
            buf = new_buf
        else:
            last_idx: int = len(buf) - 1
            last: int = buf[last_idx]
            results.append(last)
            trimmed: list[int] = []
            n: int = 0
            while n < last_idx:
                tv: int = buf[n]
                trimmed.append(tv)
                n = n + 1
            buf = trimmed
    return results


def sliding_window_sums(data: list[int], window_size: int) -> list[int]:
    """Compute sum for each sliding window of given size."""
    if window_size > len(data):
        return []
    if window_size <= 0:
        return []
    sums: list[int] = []
    i: int = 0
    limit: int = len(data) - window_size + 1
    while i < limit:
        total: int = 0
        j: int = 0
        while j < window_size:
            idx: int = i + j
            val: int = data[idx]
            total = total + val
            j = j + 1
        sums.append(total)
        i = i + 1
    return sums


def defaultdict_sim(keys: list[str], vals: list[int]) -> dict[str, int]:
    """Simulate defaultdict(int): sum values by key."""
    result: dict[str, int] = {}
    i: int = 0
    while i < len(keys):
        k: str = keys[i]
        v: int = vals[i]
        if k in result:
            result[k] = result[k] + v
        else:
            result[k] = v
        i = i + 1
    return result


def counter_most_common(text: str) -> str:
    """Find most common character (simulated Counter.most_common)."""
    if len(text) == 0:
        return ""
    freq: dict[str, int] = count_chars(text)
    best_ch: str = text[0]
    best_count: int = 0
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        c: int = freq[ch]
        if c > best_count:
            best_count = c
            best_ch = ch
        i = i + 1
    return best_ch


def deque_rotate(items: list[int], n: int) -> list[int]:
    """Rotate list right by n positions."""
    length: int = len(items)
    if length == 0:
        return items
    shift: int = n % length
    if shift == 0:
        return items
    result: list[int] = []
    start_idx: int = length - shift
    i: int = start_idx
    while i < length:
        v: int = items[i]
        result.append(v)
        i = i + 1
    j: int = 0
    while j < start_idx:
        v2: int = items[j]
        result.append(v2)
        j = j + 1
    return result


def test_module() -> int:
    """Test all collection patterns."""
    ok: int = 0

    freq: dict[str, int] = count_chars("aabbc")
    if freq["a"] == 2:
        ok = ok + 1
    if freq["b"] == 2:
        ok = ok + 1
    if freq["c"] == 1:
        ok = ok + 1

    words: list[str] = ["hi", "hey", "go", "run", "a"]
    wl: dict[str, int] = count_word_lengths(words)
    if wl["2"] == 2:
        ok = ok + 1
    if wl["3"] == 2:
        ok = ok + 1
    if wl["1"] == 1:
        ok = ok + 1

    data: list[int] = [1, 2, 3, 4, 5]
    ws: list[int] = sliding_window_sums(data, 3)
    if len(ws) == 3:
        ok = ok + 1
    if ws[0] == 6:
        ok = ok + 1
    if ws[1] == 9:
        ok = ok + 1

    pq: list[int] = [10, 20, 30, 40]
    alt: list[int] = process_queue_alt(pq)
    if len(alt) == 4:
        ok = ok + 1

    dk: list[str] = ["a", "b", "a", "c", "b"]
    dv: list[int] = [1, 2, 3, 4, 5]
    dd: dict[str, int] = defaultdict_sim(dk, dv)
    if dd["a"] == 4:
        ok = ok + 1
    if dd["b"] == 7:
        ok = ok + 1

    mc: str = counter_most_common("aabbbcc")
    if mc == "b":
        ok = ok + 1

    rot: list[int] = deque_rotate([1, 2, 3, 4, 5], 2)
    if rot[0] == 4:
        ok = ok + 1
    if rot[1] == 5:
        ok = ok + 1

    return ok
