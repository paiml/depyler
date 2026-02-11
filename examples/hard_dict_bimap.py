def bimap_put_fwd(fwd: dict[int, int], left: int, right: int) -> int:
    fwd[left] = right
    return 1

def bimap_put_rev(rev: dict[int, int], right: int, left: int) -> int:
    rev[right] = left
    return 1

def bimap_get_right(fwd: dict[int, int], left: int) -> int:
    if left in fwd:
        return fwd[left]
    return -1

def bimap_get_left(rev: dict[int, int], right: int) -> int:
    if right in rev:
        return rev[right]
    return -1

def bimap_size(fwd: dict[int, int]) -> int:
    return len(fwd)

def bimap_contains_left(fwd: dict[int, int], left: int) -> int:
    if left in fwd:
        return 1
    return 0

def bimap_contains_right(rev: dict[int, int], right: int) -> int:
    if right in rev:
        return 1
    return 0

def bimap_remove_left(fwd: dict[int, int], rev: dict[int, int], left: int) -> int:
    if left in fwd:
        right: int = fwd[left]
        del fwd[left]
        if right in rev:
            del rev[right]
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    fwd: dict[int, int] = {}
    rev: dict[int, int] = {}
    bimap_put_fwd(fwd, 1, 100)
    bimap_put_rev(rev, 100, 1)
    bimap_put_fwd(fwd, 2, 200)
    bimap_put_rev(rev, 200, 2)
    bimap_put_fwd(fwd, 3, 300)
    bimap_put_rev(rev, 300, 3)
    if bimap_get_right(fwd, 1) == 100:
        passed = passed + 1
    if bimap_get_left(rev, 200) == 2:
        passed = passed + 1
    if bimap_size(fwd) == 3:
        passed = passed + 1
    bimap_remove_left(fwd, rev, 3)
    if bimap_contains_left(fwd, 3) == 0:
        passed = passed + 1
    if bimap_contains_right(rev, 300) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
