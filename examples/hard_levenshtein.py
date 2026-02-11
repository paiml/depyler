"""Levenshtein edit distance computation."""


def min_of_three(a: int, b: int, c: int) -> int:
    """Return the minimum of three integers."""
    result: int = a
    if b < result:
        result = b
    if c < result:
        result = c
    return result


def levenshtein_distance(s1: str, s2: str) -> int:
    """Compute Levenshtein edit distance between two strings."""
    len1: int = len(s1)
    len2: int = len(s2)
    if len1 == 0:
        return len2
    if len2 == 0:
        return len1
    prev_row: list[int] = []
    j: int = 0
    while j <= len2:
        prev_row.append(j)
        j = j + 1
    i: int = 0
    while i < len1:
        curr_row: list[int] = [i + 1]
        j2: int = 0
        while j2 < len2:
            cost: int = 0
            if s1[i] != s2[j2]:
                cost = 1
            insert_cost: int = curr_row[j2] + 1
            delete_cost: int = prev_row[j2 + 1] + 1
            replace_cost: int = prev_row[j2] + cost
            curr_row.append(min_of_three(insert_cost, delete_cost, replace_cost))
            j2 = j2 + 1
        prev_row = curr_row
        i = i + 1
    return prev_row[len2]


def is_one_edit_away(s1: str, s2: str) -> int:
    """Check if strings are at most 1 edit apart. Returns 1 if yes, 0 if no."""
    dist: int = levenshtein_distance(s1, s2)
    if dist <= 1:
        return 1
    return 0


def similarity_ratio(s1: str, s2: str) -> int:
    """Compute similarity as percentage (0-100)."""
    max_len: int = len(s1)
    if len(s2) > max_len:
        max_len = len(s2)
    if max_len == 0:
        return 100
    dist: int = levenshtein_distance(s1, s2)
    ratio: int = ((max_len - dist) * 100) // max_len
    return ratio


def test_module() -> int:
    """Test Levenshtein distance operations."""
    passed: int = 0

    if levenshtein_distance("", "") == 0:
        passed = passed + 1

    if levenshtein_distance("abc", "") == 3:
        passed = passed + 1

    if levenshtein_distance("kitten", "sitting") == 3:
        passed = passed + 1

    if levenshtein_distance("abc", "abc") == 0:
        passed = passed + 1

    if is_one_edit_away("cat", "bat") == 1:
        passed = passed + 1

    if is_one_edit_away("cat", "car") == 1:
        passed = passed + 1

    if is_one_edit_away("cat", "dog") == 0:
        passed = passed + 1

    if similarity_ratio("abc", "abc") == 100:
        passed = passed + 1

    return passed
