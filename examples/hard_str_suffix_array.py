def build_suffix_array(s: str) -> list[int]:
    n: int = len(s)
    sa: list[int] = []
    i: int = 0
    while i < n:
        sa.append(i)
        i = i + 1
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            si: int = sa[i]
            sj: int = sa[j]
            if suffix_compare(s, sj, si) < 0:
                tmp: int = sa[i]
                sa[i] = sa[j]
                sa[j] = tmp
            j = j + 1
        i = i + 1
    return sa

def suffix_compare(s: str, a: int, b: int) -> int:
    n: int = len(s)
    while a < n and b < n:
        if s[a] < s[b]:
            return -1
        if s[a] > s[b]:
            return 1
        a = a + 1
        b = b + 1
    la: int = n - a
    lb: int = n - b
    if la < lb:
        return -1
    if la > lb:
        return 1
    return 0

def suffix_at(s: str, idx: int) -> str:
    return s[idx:]

def count_suffixes(s: str) -> int:
    return len(s)

def test_module() -> int:
    passed: int = 0
    sa: list[int] = build_suffix_array("banana")
    if sa[0] == 5:
        passed = passed + 1
    if sa[1] == 3:
        passed = passed + 1
    if count_suffixes("hello") == 5:
        passed = passed + 1
    if suffix_at("abcd", 2) == "cd":
        passed = passed + 1
    sa2: list[int] = build_suffix_array("abc")
    if sa2[0] == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
