"""Text processing: String distance and similarity metrics.

Tests: edit distance, Hamming distance, longest common subsequence,
Jaccard similarity, character frequency comparison.
"""

from typing import Dict, List, Tuple


def hamming_distance(a: str, b: str) -> int:
    """Hamming distance between equal-length strings."""
    if len(a) != len(b):
        return -1
    dist: int = 0
    i: int = 0
    while i < len(a):
        if a[i] != b[i]:
            dist += 1
        i += 1
    return dist


def levenshtein_distance(a: str, b: str) -> int:
    """Compute Levenshtein edit distance between two strings."""
    na: int = len(a)
    nb: int = len(b)
    if na == 0:
        return nb
    if nb == 0:
        return na
    prev: List[int] = []
    i: int = 0
    while i <= nb:
        prev.append(i)
        i += 1
    i = 1
    while i <= na:
        curr: List[int] = [i]
        j: int = 1
        while j <= nb:
            cost: int = 0
            if a[i - 1] != b[j - 1]:
                cost = 1
            insert_c: int = curr[j - 1] + 1
            delete_c: int = prev[j] + 1
            replace_c: int = prev[j - 1] + cost
            min_c: int = insert_c
            if delete_c < min_c:
                min_c = delete_c
            if replace_c < min_c:
                min_c = replace_c
            curr.append(min_c)
            j += 1
        prev = curr
        i += 1
    return prev[nb]


def longest_common_subseq_len(a: str, b: str) -> int:
    """Length of longest common subsequence."""
    na: int = len(a)
    nb: int = len(b)
    if na == 0 or nb == 0:
        return 0
    prev: List[int] = []
    i: int = 0
    while i <= nb:
        prev.append(0)
        i += 1
    i = 1
    while i <= na:
        curr: List[int] = [0]
        j: int = 1
        while j <= nb:
            if a[i - 1] == b[j - 1]:
                curr.append(prev[j - 1] + 1)
            else:
                val: int = prev[j]
                if curr[j - 1] > val:
                    val = curr[j - 1]
                curr.append(val)
            j += 1
        prev = curr
        i += 1
    return prev[nb]


def common_char_count(a: str, b: str) -> int:
    """Count characters that appear in both strings."""
    freq_a: Dict[str, int] = {}
    for ch in a:
        if ch in freq_a:
            freq_a[ch] = freq_a[ch] + 1
        else:
            freq_a[ch] = 1
    freq_b: Dict[str, int] = {}
    for ch in b:
        if ch in freq_b:
            freq_b[ch] = freq_b[ch] + 1
        else:
            freq_b[ch] = 1
    count: int = 0
    for ch in freq_a:
        if ch in freq_b:
            va: int = freq_a[ch]
            vb: int = freq_b[ch]
            if va < vb:
                count += va
            else:
                count += vb
    return count


def longest_common_prefix(a: str, b: str) -> str:
    """Find longest common prefix of two strings."""
    result: List[str] = []
    n: int = len(a)
    if len(b) < n:
        n = len(b)
    i: int = 0
    while i < n:
        if a[i] == b[i]:
            result.append(a[i])
        else:
            break
        i += 1
    return "".join(result)


def test_distances() -> bool:
    """Test string distance functions."""
    ok: bool = True
    hd: int = hamming_distance("karolin", "kathrin")
    if hd != 3:
        ok = False
    ld: int = levenshtein_distance("kitten", "sitting")
    if ld != 3:
        ok = False
    lcs: int = longest_common_subseq_len("ABCBDAB", "BDCAB")
    if lcs != 4:
        ok = False
    return ok
