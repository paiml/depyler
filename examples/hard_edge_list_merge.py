"""Merging sorted lists and k-way merge patterns."""


def merge_two(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted lists into one sorted list."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i = i + 1
        else:
            result.append(b[j])
            j = j + 1
    while i < len(a):
        result.append(a[i])
        i = i + 1
    while j < len(b):
        result.append(b[j])
        j = j + 1
    return result


def merge_three(a: list[int], b: list[int], c: list[int]) -> list[int]:
    """Merge three sorted lists."""
    ab: list[int] = merge_two(a, b)
    return merge_two(ab, c)


def merge_k_lists(lists: list[list[int]]) -> list[int]:
    """Merge k sorted lists using pairwise merge."""
    k: int = len(lists)
    if k == 0:
        return []
    if k == 1:
        result: list[int] = []
        i: int = 0
        while i < len(lists[0]):
            item: int = lists[0][i]
            result.append(item)
            i = i + 1
        return result
    current: list[list[int]] = []
    i = 0
    while i < k:
        sub: list[int] = []
        j: int = 0
        lst: list[int] = lists[i]
        while j < len(lst):
            sub.append(lst[j])
            j = j + 1
        current.append(sub)
        i = i + 1
    while len(current) > 1:
        next_round: list[list[int]] = []
        idx: int = 0
        while idx < len(current):
            if idx + 1 < len(current):
                merged: list[int] = merge_two(current[idx], current[idx + 1])
                next_round.append(merged)
            else:
                next_round.append(current[idx])
            idx = idx + 2
        current = next_round
    return current[0]


def merge_sorted_remove_dupes(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted lists and remove duplicates."""
    merged: list[int] = merge_two(a, b)
    if len(merged) == 0:
        return []
    result: list[int] = [merged[0]]
    i: int = 1
    while i < len(merged):
        prev: int = result[len(result) - 1]
        if merged[i] != prev:
            result.append(merged[i])
        i = i + 1
    return result


def sorted_intersection(a: list[int], b: list[int]) -> list[int]:
    """Find intersection of two sorted lists."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] == b[j]:
            if len(result) == 0:
                result.append(a[i])
            else:
                last: int = result[len(result) - 1]
                if a[i] != last:
                    result.append(a[i])
            i = i + 1
            j = j + 1
        elif a[i] < b[j]:
            i = i + 1
        else:
            j = j + 1
    return result


def sorted_union(a: list[int], b: list[int]) -> list[int]:
    """Find union of two sorted lists (no duplicates)."""
    return merge_sorted_remove_dupes(a, b)


def sorted_difference(a: list[int], b: list[int]) -> list[int]:
    """Find elements in a but not in b (both sorted)."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(a):
        if j >= len(b):
            result.append(a[i])
            i = i + 1
        elif a[i] < b[j]:
            result.append(a[i])
            i = i + 1
        elif a[i] > b[j]:
            j = j + 1
        else:
            i = i + 1
            j = j + 1
    return result


def test_module() -> int:
    """Test all merge functions."""
    passed: int = 0
    r1: list[int] = merge_two([1, 3, 5], [2, 4, 6])
    if r1 == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1
    r2: list[int] = merge_two([], [1, 2])
    if r2 == [1, 2]:
        passed = passed + 1
    r3: list[int] = merge_three([1, 4], [2, 5], [3, 6])
    if r3 == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1
    r4: list[int] = merge_k_lists([[1, 5], [2, 6], [3, 7], [4, 8]])
    if r4 == [1, 2, 3, 4, 5, 6, 7, 8]:
        passed = passed + 1
    r5: list[int] = merge_k_lists([])
    if len(r5) == 0:
        passed = passed + 1
    r6: list[int] = merge_sorted_remove_dupes([1, 2, 3], [2, 3, 4])
    if r6 == [1, 2, 3, 4]:
        passed = passed + 1
    r7: list[int] = sorted_intersection([1, 2, 3, 4], [2, 4, 6])
    if r7 == [2, 4]:
        passed = passed + 1
    r8: list[int] = sorted_union([1, 3, 5], [2, 3, 4])
    if r8 == [1, 2, 3, 4, 5]:
        passed = passed + 1
    r9: list[int] = sorted_difference([1, 2, 3, 4], [2, 4])
    if r9 == [1, 3]:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
