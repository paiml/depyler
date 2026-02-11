def rk_hash(s: str, start: int, length: int) -> int:
    h: int = 0
    i: int = 0
    while i < length:
        idx: int = start + i
        h = h * 31 + ord(s[idx])
        h = h % 1000000007
        i = i + 1
    return h

def rk_search(text: str, needle: str) -> list[int]:
    results: list[int] = []
    n: int = len(text)
    m: int = len(needle)
    if m > n or m == 0:
        return results
    target_hash: int = rk_hash(needle, 0, m)
    i: int = 0
    while i <= n - m:
        h: int = rk_hash(text, i, m)
        if h == target_hash:
            match: int = 1
            j: int = 0
            while j < m:
                ti: int = i + j
                if text[ti] != needle[j]:
                    match = 0
                    j = m
                j = j + 1
            if match == 1:
                results.append(i)
        i = i + 1
    return results

def rk_count(text: str, needle: str) -> int:
    matches: list[int] = rk_search(text, needle)
    return len(matches)

def rk_first_match(text: str, needle: str) -> int:
    matches: list[int] = rk_search(text, needle)
    if len(matches) == 0:
        return -1
    return matches[0]

def test_module() -> int:
    passed: int = 0
    m1: list[int] = rk_search("abcabcabc", "abc")
    if len(m1) == 3:
        passed = passed + 1
    if m1[0] == 0 and m1[1] == 3:
        passed = passed + 1
    if rk_count("hello world", "o") == 2:
        passed = passed + 1
    if rk_first_match("abcdef", "cd") == 2:
        passed = passed + 1
    if rk_first_match("abcdef", "xyz") == -1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
