"""Longest common prefix computation for string arrays."""


def common_prefix_two(s1: str, s2: str) -> str:
    """Find common prefix between two strings."""
    result: str = ""
    i: int = 0
    len1: int = len(s1)
    len2: int = len(s2)
    limit: int = len1
    if len2 < limit:
        limit = len2
    while i < limit:
        if s1[i] == s2[i]:
            result = result + s1[i]
        else:
            break
        i = i + 1
    return result


def longest_common_prefix(strings: list[str]) -> str:
    """Find the longest common prefix among all strings."""
    length: int = len(strings)
    if length == 0:
        return ""
    if length == 1:
        return strings[0]
    prefix: str = strings[0]
    i: int = 1
    while i < length:
        prefix = common_prefix_two(prefix, strings[i])
        if len(prefix) == 0:
            return ""
        i = i + 1
    return prefix


def common_suffix_two(s1: str, s2: str) -> str:
    """Find common suffix between two strings."""
    result: str = ""
    i1: int = len(s1) - 1
    i2: int = len(s2) - 1
    while i1 >= 0 and i2 >= 0:
        if s1[i1] == s2[i2]:
            result = s1[i1] + result
        else:
            break
        i1 = i1 - 1
        i2 = i2 - 1
    return result


def longest_common_suffix(strings: list[str]) -> str:
    """Find the longest common suffix among all strings."""
    length: int = len(strings)
    if length == 0:
        return ""
    if length == 1:
        return strings[0]
    suffix: str = strings[0]
    i: int = 1
    while i < length:
        suffix = common_suffix_two(suffix, strings[i])
        if len(suffix) == 0:
            return ""
        i = i + 1
    return suffix


def test_module() -> int:
    """Test longest common prefix operations."""
    passed: int = 0

    r1: str = common_prefix_two("flower", "flow")
    if r1 == "flow":
        passed = passed + 1

    r2: str = longest_common_prefix(["flower", "flow", "flight"])
    if r2 == "fl":
        passed = passed + 1

    r3: str = longest_common_prefix(["dog", "car", "race"])
    if r3 == "":
        passed = passed + 1

    r4: str = longest_common_prefix(["abc"])
    if r4 == "abc":
        passed = passed + 1

    r5: str = longest_common_prefix([])
    if r5 == "":
        passed = passed + 1

    r6: str = common_suffix_two("testing", "running")
    if r6 == "ning":
        passed = passed + 1

    r7: str = longest_common_suffix(["testing", "running", "jumping"])
    if r7 == "ing":
        passed = passed + 1

    r8: str = common_prefix_two("", "abc")
    if r8 == "":
        passed = passed + 1

    return passed
