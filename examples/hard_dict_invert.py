def invert_dict(keys: list[str], vals: list[str]) -> dict[str, str]:
    result: dict[str, str] = {}
    i: int = 0
    while i < len(keys):
        result[vals[i]] = keys[i]
        i = i + 1
    return result

def is_invertible(vals: list[str]) -> int:
    seen: dict[str, int] = {}
    i: int = 0
    while i < len(vals):
        v: str = vals[i]
        if v in seen:
            return 0
        seen[v] = 1
        i = i + 1
    return 1

def invert_int_dict(src: dict[int, int]) -> dict[int, int]:
    result: dict[int, int] = {}
    items_k: list[int] = [1, 2, 3, 4, 5]
    i: int = 0
    while i < len(items_k):
        ik: int = items_k[i]
        if ik in src:
            result[src[ik]] = ik
        i = i + 1
    return result

def count_unique_values(vals: list[str]) -> int:
    seen: dict[str, int] = {}
    i: int = 0
    while i < len(vals):
        seen[vals[i]] = 1
        i = i + 1
    return len(seen)

def test_module() -> int:
    passed: int = 0
    inv: dict[str, str] = invert_dict(["a", "b", "c"], ["x", "y", "z"])
    if inv["x"] == "a":
        passed = passed + 1
    if inv["z"] == "c":
        passed = passed + 1
    if is_invertible(["x", "y", "z"]) == 1:
        passed = passed + 1
    if is_invertible(["x", "y", "x"]) == 0:
        passed = passed + 1
    if count_unique_values(["a", "b", "a", "c"]) == 3:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
