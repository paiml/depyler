def lcp_of_two(s: str, i: int, j: int) -> int:
    n: int = len(s)
    length: int = 0
    while i < n and j < n:
        if s[i] != s[j]:
            return length
        length = length + 1
        i = i + 1
        j = j + 1
    return length

def build_lcp_from_sa(s: str, sa: list[int]) -> list[int]:
    n: int = len(sa)
    lcp: list[int] = []
    i: int = 1
    while i < n:
        prev: int = sa[i - 1]
        curr: int = sa[i]
        slen: int = len(s)
        length: int = 0
        a: int = prev
        b: int = curr
        while a < slen and b < slen:
            if s[a] != s[b]:
                a = slen
            else:
                length = length + 1
                a = a + 1
                b = b + 1
        lcp.append(length)
        i = i + 1
    return lcp

def max_lcp(lcp: list[int]) -> int:
    if len(lcp) == 0:
        return 0
    mx: int = lcp[0]
    i: int = 1
    while i < len(lcp):
        if lcp[i] > mx:
            mx = lcp[i]
        i = i + 1
    return mx

def sum_lcp(lcp: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(lcp):
        total = total + lcp[i]
        i = i + 1
    return total

def count_unique_substrings(s: str, sa: list[int]) -> int:
    n: int = len(s)
    lcp: list[int] = build_lcp_from_sa(s, sa)
    total: int = n * (n + 1) // 2
    total = total - sum_lcp(lcp)
    return total

def test_module() -> int:
    passed: int = 0
    if lcp_of_two("abcabd", 0, 3) == 2:
        passed = passed + 1
    sa: list[int] = [5, 3, 1, 0, 4, 2]
    lcp: list[int] = build_lcp_from_sa("banana", sa)
    if len(lcp) == 5:
        passed = passed + 1
    if max_lcp(lcp) == 3:
        passed = passed + 1
    if lcp_of_two("aaa", 0, 1) == 2:
        passed = passed + 1
    if count_unique_substrings("banana", sa) == 15:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
