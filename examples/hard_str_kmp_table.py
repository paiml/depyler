def kmp_failure(needle: str) -> list[int]:
    n: int = len(needle)
    fail: list[int] = []
    i: int = 0
    while i < n:
        fail.append(0)
        i = i + 1
    if n == 0:
        return fail
    fail[0] = 0
    length: int = 0
    i = 1
    while i < n:
        while length > 0 and needle[i] != needle[length]:
            length = fail[length - 1]
        if needle[i] == needle[length]:
            length = length + 1
        fail[i] = length
        i = i + 1
    return fail

def kmp_search(text: str, needle: str) -> list[int]:
    fail: list[int] = kmp_failure(needle)
    results: list[int] = []
    m: int = len(needle)
    n: int = len(text)
    j: int = 0
    i: int = 0
    while i < n:
        while j > 0 and text[i] != needle[j]:
            j = fail[j - 1]
        if text[i] == needle[j]:
            j = j + 1
        if j == m:
            results.append(i - m + 1)
            j = fail[j - 1]
        i = i + 1
    return results

def kmp_count(text: str, needle: str) -> int:
    matches: list[int] = kmp_search(text, needle)
    return len(matches)

def max_prefix_suffix(needle: str) -> int:
    fail: list[int] = kmp_failure(needle)
    if len(fail) == 0:
        return 0
    idx: int = len(fail) - 1
    return fail[idx]

def test_module() -> int:
    passed: int = 0
    f: list[int] = kmp_failure("abcabd")
    if f[3] == 1 and f[4] == 2:
        passed = passed + 1
    m: list[int] = kmp_search("abcabcabc", "abc")
    if len(m) == 3 and m[0] == 0:
        passed = passed + 1
    if kmp_count("aaaa", "aa") == 3:
        passed = passed + 1
    if kmp_count("hello", "xyz") == 0:
        passed = passed + 1
    if max_prefix_suffix("abab") == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
