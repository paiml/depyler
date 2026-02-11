def parse_major(version: str) -> int:
    result: int = 0
    n: int = len(version)
    i: int = 0
    while i < n:
        ch: str = version[i]
        if ch == ".":
            return result
        result = result * 10 + ord(ch) - 48
        i = i + 1
    return result

def parse_minor(version: str) -> int:
    n: int = len(version)
    i: int = 0
    dots: int = 0
    while i < n:
        if version[i] == ".":
            dots = dots + 1
            if dots == 1:
                i = i + 1
                result: int = 0
                while i < n:
                    ch: str = version[i]
                    if ch == ".":
                        return result
                    result = result * 10 + ord(ch) - 48
                    i = i + 1
                return result
        i = i + 1
    return 0

def parse_patch(version: str) -> int:
    n: int = len(version)
    i: int = 0
    dots: int = 0
    while i < n:
        if version[i] == ".":
            dots = dots + 1
            if dots == 2:
                i = i + 1
                result: int = 0
                while i < n:
                    ch: str = version[i]
                    result = result * 10 + ord(ch) - 48
                    i = i + 1
                return result
        i = i + 1
    return 0

def compare_versions(v1: str, v2: str) -> int:
    maj1: int = parse_major(v1)
    maj2: int = parse_major(v2)
    if maj1 > maj2:
        return 1
    if maj1 < maj2:
        return 0 - 1
    min1: int = parse_minor(v1)
    min2: int = parse_minor(v2)
    if min1 > min2:
        return 1
    if min1 < min2:
        return 0 - 1
    pat1: int = parse_patch(v1)
    pat2: int = parse_patch(v2)
    if pat1 > pat2:
        return 1
    if pat1 < pat2:
        return 0 - 1
    return 0

def is_compatible(v1: str, v2: str) -> int:
    maj1: int = parse_major(v1)
    maj2: int = parse_major(v2)
    if maj1 == maj2:
        return 1
    return 0

def bump_patch(major: int, minor: int, patch: int) -> int:
    return patch + 1

def bump_minor(minor: int) -> int:
    return minor + 1

def test_module() -> int:
    passed: int = 0
    m: int = parse_major("1.2.3")
    if m == 1:
        passed = passed + 1
    mi: int = parse_minor("1.2.3")
    if mi == 2:
        passed = passed + 1
    p: int = parse_patch("1.2.3")
    if p == 3:
        passed = passed + 1
    c: int = compare_versions("1.2.3", "1.2.4")
    if c == (0 - 1):
        passed = passed + 1
    ic: int = is_compatible("1.2.3", "1.5.0")
    if ic == 1:
        passed = passed + 1
    return passed
