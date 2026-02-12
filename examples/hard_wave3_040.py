"""Text processing: Levenshtein automaton and fuzzy matching.

Tests: approximate string matching, typo detection, spell checking
primitives, similarity scoring, candidate ranking.
"""

from typing import Dict, List, Tuple


def edit_distance_threshold(a: str, b: str, threshold: int) -> bool:
    """Check if edit distance is within threshold (early exit)."""
    na: int = len(a)
    nb: int = len(b)
    diff: int = na - nb
    if diff < 0:
        diff = -diff
    if diff > threshold:
        return False
    if na == 0:
        return nb <= threshold
    if nb == 0:
        return na <= threshold
    prev: List[int] = []
    i: int = 0
    while i <= nb:
        prev.append(i)
        i += 1
    i = 1
    while i <= na:
        curr: List[int] = [i]
        min_val: int = i
        j: int = 1
        while j <= nb:
            cost: int = 0
            if a[i - 1] != b[j - 1]:
                cost = 1
            ins: int = curr[j - 1] + 1
            dl: int = prev[j] + 1
            rep: int = prev[j - 1] + cost
            mn: int = ins
            if dl < mn:
                mn = dl
            if rep < mn:
                mn = rep
            curr.append(mn)
            if mn < min_val:
                min_val = mn
            j += 1
        if min_val > threshold:
            return False
        prev = curr
        i += 1
    return prev[nb] <= threshold


def find_closest(word: str, dictionary: List[str]) -> str:
    """Find closest word in dictionary by edit distance."""
    best: str = ""
    best_dist: int = 999999
    for candidate in dictionary:
        na: int = len(word)
        nb: int = len(candidate)
        if na == 0:
            dist: int = nb
        elif nb == 0:
            dist = na
        else:
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
                    if word[i - 1] != candidate[j - 1]:
                        cost = 1
                    ins: int = curr[j - 1] + 1
                    dl: int = prev[j] + 1
                    rep: int = prev[j - 1] + cost
                    mn: int = ins
                    if dl < mn:
                        mn = dl
                    if rep < mn:
                        mn = rep
                    curr.append(mn)
                    j += 1
                prev = curr
                i += 1
            dist = prev[nb]
        if dist < best_dist:
            best_dist = dist
            best = candidate
    return best


def similarity_score(a: str, b: str) -> float:
    """Compute similarity score (0 to 1) between two strings."""
    na: int = len(a)
    nb: int = len(b)
    max_len: int = na
    if nb > max_len:
        max_len = nb
    if max_len == 0:
        return 1.0
    if na == 0 or nb == 0:
        return 0.0
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
            ins: int = curr[j - 1] + 1
            dl: int = prev[j] + 1
            rep: int = prev[j - 1] + cost
            mn: int = ins
            if dl < mn:
                mn = dl
            if rep < mn:
                mn = rep
            curr.append(mn)
            j += 1
        prev = curr
        i += 1
    dist: int = prev[nb]
    return 1.0 - float(dist) / float(max_len)


def common_prefix_length(a: str, b: str) -> int:
    """Length of common prefix."""
    n: int = len(a)
    if len(b) < n:
        n = len(b)
    i: int = 0
    while i < n:
        if a[i] != b[i]:
            return i
        i += 1
    return n


def test_fuzzy() -> bool:
    """Test fuzzy matching functions."""
    ok: bool = True
    if not edit_distance_threshold("kitten", "sitting", 3):
        ok = False
    if edit_distance_threshold("kitten", "sitting", 2):
        ok = False
    closest: str = find_closest("helo", ["hello", "world", "help"])
    if closest != "hello" and closest != "help":
        ok = False
    sim: float = similarity_score("hello", "hello")
    diff: float = sim - 1.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    return ok
