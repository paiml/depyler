"""Simple suffix array construction and LCP computation."""


def build_suffix_array(s: str) -> list[int]:
    """Build suffix array using simple O(n^2 log n) sorting."""
    n: int = len(s)
    sa: list[int] = []
    i: int = 0
    while i < n:
        sa.append(i)
        i = i + 1
    changed: int = 1
    while changed == 1:
        changed = 0
        j: int = 0
        while j < n - 1:
            k: int = j + 1
            while k < n:
                should_swap: int = 0
                a_pos: int = sa[j]
                b_pos: int = sa[k]
                ci: int = 0
                while a_pos + ci < n and b_pos + ci < n:
                    if s[a_pos + ci] < s[b_pos + ci]:
                        break
                    if s[a_pos + ci] > s[b_pos + ci]:
                        should_swap = 1
                        break
                    ci = ci + 1
                if should_swap == 0 and a_pos + ci >= n:
                    should_swap = 0
                elif should_swap == 0 and b_pos + ci >= n:
                    should_swap = 1
                if should_swap == 1:
                    tmp: int = sa[j]
                    sa[j] = sa[k]
                    sa[k] = tmp
                    changed = 1
                k = k + 1
            j = j + 1
    return sa


def lcp_of_two(s: str, i: int, j: int) -> int:
    """Compute longest common prefix length between suffixes at i and j."""
    n: int = len(s)
    length: int = 0
    while i + length < n and j + length < n:
        if s[i + length] != s[j + length]:
            break
        length = length + 1
    return length


def longest_repeated_substring_len(s: str) -> int:
    """Find length of longest repeated substring using suffix array."""
    n: int = len(s)
    if n < 2:
        return 0
    sa: list[int] = build_suffix_array(s)
    best: int = 0
    i: int = 0
    while i < n - 1:
        lcp: int = lcp_of_two(s, sa[i], sa[i + 1])
        if lcp > best:
            best = lcp
        i = i + 1
    return best


def test_module() -> int:
    passed: int = 0

    sa1: list[int] = build_suffix_array("banana")
    if sa1[0] == 5:
        passed = passed + 1

    if sa1[1] == 3:
        passed = passed + 1

    if lcp_of_two("banana", 1, 3) == 3:
        passed = passed + 1

    lrs: int = longest_repeated_substring_len("banana")
    if lrs == 3:
        passed = passed + 1

    lrs2: int = longest_repeated_substring_len("abcdef")
    if lrs2 == 0:
        passed = passed + 1

    lrs3: int = longest_repeated_substring_len("aabaa")
    if lrs3 == 2:
        passed = passed + 1

    sa2: list[int] = build_suffix_array("abc")
    if sa2[0] == 0 and sa2[1] == 1 and sa2[2] == 2:
        passed = passed + 1

    return passed
