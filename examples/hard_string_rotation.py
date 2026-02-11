"""String rotation detection and operations."""


def is_rotation(s1: str, s2: str) -> int:
    """Check if s2 is a rotation of s1. Returns 1 if yes, 0 if no."""
    len1: int = len(s1)
    len2: int = len(s2)
    if len1 != len2:
        return 0
    if len1 == 0:
        return 1
    doubled: str = s1 + s1
    i: int = 0
    limit: int = len(doubled) - len2 + 1
    while i < limit:
        match: int = 1
        j: int = 0
        while j < len2:
            if doubled[i + j] != s2[j]:
                match = 0
                break
            j = j + 1
        if match == 1:
            return 1
        i = i + 1
    return 0


def rotate_left(s: str, positions: int) -> str:
    """Rotate string left by given positions."""
    length: int = len(s)
    if length == 0:
        return s
    actual: int = positions % length
    result: str = ""
    i: int = actual
    while i < length:
        result = result + s[i]
        i = i + 1
    j: int = 0
    while j < actual:
        result = result + s[j]
        j = j + 1
    return result


def rotate_right(s: str, positions: int) -> str:
    """Rotate string right by given positions."""
    length: int = len(s)
    if length == 0:
        return s
    actual: int = positions % length
    left_amount: int = length - actual
    result: str = rotate_left(s, left_amount)
    return result


def min_rotation_distance(s1: str, s2: str) -> int:
    """Find minimum rotation distance to transform s1 to s2. Returns -1 if impossible."""
    len1: int = len(s1)
    if len1 != len(s2):
        return -1
    if s1 == s2:
        return 0
    i: int = 1
    while i < len1:
        rotated: str = rotate_left(s1, i)
        if rotated == s2:
            return i
        i = i + 1
    return -1


def test_module() -> int:
    """Test string rotation operations."""
    passed: int = 0

    if is_rotation("abcde", "cdeab") == 1:
        passed = passed + 1

    if is_rotation("abcde", "abced") == 0:
        passed = passed + 1

    r3: str = rotate_left("abcde", 2)
    if r3 == "cdeab":
        passed = passed + 1

    r4: str = rotate_right("abcde", 2)
    if r4 == "deabc":
        passed = passed + 1

    if is_rotation("", "") == 1:
        passed = passed + 1

    if min_rotation_distance("abcde", "cdeab") == 2:
        passed = passed + 1

    if min_rotation_distance("abc", "xyz") == -1:
        passed = passed + 1

    r8: str = rotate_left("a", 5)
    if r8 == "a":
        passed = passed + 1

    return passed
