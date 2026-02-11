def z_array(s: str) -> list[int]:
    n: int = len(s)
    z: list[int] = []
    i: int = 0
    while i < n:
        z.append(0)
        i = i + 1
    if n == 0:
        return z
    z[0] = n
    left: int = 0
    right: int = 0
    i = 1
    while i < n:
        if i < right:
            diff: int = right - i
            zi: int = i - left
            if z[zi] < diff:
                z[i] = z[zi]
            else:
                z[i] = diff
        k: int = z[i]
        while i + k < n and s[k] == s[i + k]:
            k = k + 1
        z[i] = k
        if i + k > right:
            left = i
            right = i + k
        i = i + 1
    return z

def z_search(text: str, needle: str) -> list[int]:
    combined: str = needle + "$" + text
    z: list[int] = z_array(combined)
    plen: int = len(needle)
    results: list[int] = []
    i: int = plen + 1
    while i < len(combined):
        if z[i] == plen:
            results.append(i - plen - 1)
        i = i + 1
    return results

def z_count_matches(text: str, needle: str) -> int:
    matches: list[int] = z_search(text, needle)
    return len(matches)

def test_module() -> int:
    passed: int = 0
    z1: list[int] = z_array("aabxaa")
    if z1[0] == 6 and z1[1] == 1:
        passed = passed + 1
    m1: list[int] = z_search("abcabcabc", "abc")
    if len(m1) == 3:
        passed = passed + 1
    if m1[0] == 0:
        passed = passed + 1
    if z_count_matches("aaaa", "aa") == 3:
        passed = passed + 1
    if z_count_matches("hello", "xyz") == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
