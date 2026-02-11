def min_of_three(a: int, b: int, c: int) -> int:
    result: int = a
    if b < result:
        result = b
    if c < result:
        result = c
    return result

def levenshtein(s1: str, s2: str) -> int:
    m: int = len(s1)
    n: int = len(s2)
    prev: list[int] = []
    curr: list[int] = []
    j: int = 0
    while j <= n:
        prev.append(j)
        curr.append(0)
        j = j + 1
    i: int = 1
    while i <= m:
        curr[0] = i
        j = 1
        while j <= n:
            ci: int = i - 1
            cj: int = j - 1
            if s1[ci] == s2[cj]:
                curr[j] = prev[j - 1]
            else:
                curr[j] = 1 + min_of_three(prev[j], curr[j - 1], prev[j - 1])
            j = j + 1
        k: int = 0
        while k <= n:
            prev[k] = curr[k]
            k = k + 1
        i = i + 1
    return prev[n]

def are_similar(s1: str, s2: str, threshold: int) -> int:
    d: int = levenshtein(s1, s2)
    if d <= threshold:
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    if levenshtein("kitten", "sitting") == 3:
        passed = passed + 1
    if levenshtein("", "abc") == 3:
        passed = passed + 1
    if levenshtein("abc", "abc") == 0:
        passed = passed + 1
    if levenshtein("a", "b") == 1:
        passed = passed + 1
    if are_similar("cat", "car", 1) == 1:
        passed = passed + 1
    if are_similar("cat", "dog", 1) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
