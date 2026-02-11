# Interval operations (overlap, merge, contains)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def intervals_overlap(a_start: int, a_end: int, b_start: int, b_end: int) -> bool:
    """Check if two intervals [a_start, a_end] and [b_start, b_end] overlap."""
    if a_start > b_end:
        return False
    if b_start > a_end:
        return False
    return True


def interval_contains(outer_start: int, outer_end: int, inner_start: int, inner_end: int) -> bool:
    """Check if the outer interval fully contains the inner interval."""
    return outer_start <= inner_start and outer_end >= inner_end


def interval_length(start: int, end: int) -> int:
    """Compute the length of an interval. Returns 0 if invalid."""
    if end < start:
        return 0
    return end - start


def overlap_length(a_start: int, a_end: int, b_start: int, b_end: int) -> int:
    """Compute the length of overlap between two intervals."""
    if not intervals_overlap(a_start, a_end, b_start, b_end):
        return 0
    start: int = a_start
    if b_start > start:
        start = b_start
    end: int = a_end
    if b_end < end:
        end = b_end
    return end - start


def count_overlapping_pairs(starts: list[int], ends: list[int]) -> int:
    """Count the number of overlapping pairs among a list of intervals."""
    count: int = 0
    n: int = len(starts)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if intervals_overlap(starts[i], ends[i], starts[j], ends[j]):
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    assert intervals_overlap(1, 5, 3, 7) == True
    assert intervals_overlap(1, 5, 6, 10) == False
    assert intervals_overlap(1, 5, 5, 10) == True
    assert intervals_overlap(3, 7, 1, 4) == True
    assert interval_contains(1, 10, 3, 7) == True
    assert interval_contains(1, 10, 0, 5) == False
    assert interval_contains(1, 10, 1, 10) == True
    assert interval_length(1, 5) == 4
    assert interval_length(5, 5) == 0
    assert interval_length(5, 1) == 0
    assert overlap_length(1, 5, 3, 7) == 2
    assert overlap_length(1, 5, 6, 10) == 0
    assert overlap_length(1, 10, 3, 7) == 4
    assert count_overlapping_pairs([1, 3, 6], [5, 7, 10]) == 2
    assert count_overlapping_pairs([1, 10], [2, 20]) == 1
    assert count_overlapping_pairs([1, 5], [3, 8]) == 0
    return 0


if __name__ == "__main__":
    test_module()
