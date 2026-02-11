def rotation_at(s: str, start: int) -> str:
    n: int = len(s)
    result: str = ""
    i: int = 0
    while i < n:
        idx: int = (start + i) % n
        result = result + s[idx]
        i = i + 1
    return result

def smallest_rotation(s: str) -> str:
    n: int = len(s)
    if n == 0:
        return ""
    best: str = s
    i: int = 1
    while i < n:
        rot: str = rotation_at(s, i)
        if rot < best:
            best = rot
        i = i + 1
    return best

def smallest_rotation_index(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    best: str = s
    best_idx: int = 0
    i: int = 1
    while i < n:
        rot: str = rotation_at(s, i)
        if rot < best:
            best = rot
            best_idx = i
        i = i + 1
    return best_idx

def count_distinct_rotations(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    seen: dict[str, int] = {}
    i: int = 0
    while i < n:
        rot: str = rotation_at(s, i)
        seen[rot] = 1
        i = i + 1
    return len(seen)

def is_rotation(a: str, b: str) -> int:
    if len(a) != len(b):
        return 0
    n: int = len(a)
    i: int = 0
    while i < n:
        if rotation_at(a, i) == b:
            return 1
        i = i + 1
    return 0

def test_module() -> int:
    passed: int = 0
    if smallest_rotation("cab") == "abc":
        passed = passed + 1
    if smallest_rotation_index("cab") == 1:
        passed = passed + 1
    if count_distinct_rotations("abc") == 3:
        passed = passed + 1
    if count_distinct_rotations("aaa") == 1:
        passed = passed + 1
    if is_rotation("abc", "bca") == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
