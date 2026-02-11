def longest_unique_substr(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    best: int = 0
    start: int = 0
    last_seen: list[int] = []
    i: int = 0
    while i < 128:
        last_seen.append(0 - 1)
        i = i + 1
    j: int = 0
    while j < n:
        c: int = ord(s[j])
        prev: int = last_seen[c]
        if prev >= start:
            start = prev + 1
        last_seen[c] = j
        window: int = j - start + 1
        if window > best:
            best = window
        j = j + 1
    return best

def longest_substr_k_distinct(s: str, limit: int) -> int:
    n: int = len(s)
    if n == 0 or limit == 0:
        return 0
    counts: list[int] = []
    i: int = 0
    while i < 128:
        counts.append(0)
        i = i + 1
    distinct: int = 0
    start: int = 0
    best: int = 0
    j: int = 0
    while j < n:
        c: int = ord(s[j])
        if counts[c] == 0:
            distinct = distinct + 1
        counts[c] = counts[c] + 1
        while distinct > limit:
            sc: int = ord(s[start])
            counts[sc] = counts[sc] - 1
            if counts[sc] == 0:
                distinct = distinct - 1
            start = start + 1
        window: int = j - start + 1
        if window > best:
            best = window
        j = j + 1
    return best

def count_unique_chars(s: str) -> int:
    seen: list[int] = []
    i: int = 0
    while i < 128:
        seen.append(0)
        i = i + 1
    j: int = 0
    n: int = len(s)
    while j < n:
        c: int = ord(s[j])
        seen[c] = 1
        j = j + 1
    cnt: int = 0
    k: int = 0
    while k < 128:
        cnt = cnt + seen[k]
        k = k + 1
    return cnt

def test_module() -> int:
    passed: int = 0
    r1: int = longest_unique_substr("abcabcbb")
    if r1 == 3:
        passed = passed + 1
    r2: int = longest_unique_substr("bbbbb")
    if r2 == 1:
        passed = passed + 1
    r3: int = longest_substr_k_distinct("eceba", 2)
    if r3 == 3:
        passed = passed + 1
    r4: int = count_unique_chars("aabbcc")
    if r4 == 3:
        passed = passed + 1
    r5: int = longest_unique_substr("")
    if r5 == 0:
        passed = passed + 1
    r6: int = longest_unique_substr("abcdef")
    if r6 == 6:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
