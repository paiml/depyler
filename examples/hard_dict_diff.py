def dict_diff_keys(a_keys: list[str], b_keys: list[str]) -> list[str]:
    b_set: dict[str, int] = {}
    i: int = 0
    while i < len(b_keys):
        b_set[b_keys[i]] = 1
        i = i + 1
    result: list[str] = []
    j: int = 0
    while j < len(a_keys):
        ak: str = a_keys[j]
        if ak not in b_set:
            result.append(ak)
        j = j + 1
    return result

def dict_intersection_keys(a_keys: list[str], b_keys: list[str]) -> list[str]:
    b_set: dict[str, int] = {}
    i: int = 0
    while i < len(b_keys):
        b_set[b_keys[i]] = 1
        i = i + 1
    result: list[str] = []
    j: int = 0
    while j < len(a_keys):
        ak: str = a_keys[j]
        if ak in b_set:
            result.append(ak)
        j = j + 1
    return result

def symmetric_diff_keys(a_keys: list[str], b_keys: list[str]) -> list[str]:
    left: list[str] = dict_diff_keys(a_keys, b_keys)
    right: list[str] = dict_diff_keys(b_keys, a_keys)
    result: list[str] = []
    i: int = 0
    while i < len(left):
        result.append(left[i])
        i = i + 1
    j: int = 0
    while j < len(right):
        result.append(right[j])
        j = j + 1
    return result

def dict_union_keys(a_keys: list[str], b_keys: list[str]) -> list[str]:
    seen: dict[str, int] = {}
    result: list[str] = []
    i: int = 0
    while i < len(a_keys):
        ak: str = a_keys[i]
        if ak not in seen:
            seen[ak] = 1
            result.append(ak)
        i = i + 1
    j: int = 0
    while j < len(b_keys):
        bk: str = b_keys[j]
        if bk not in seen:
            seen[bk] = 1
            result.append(bk)
        j = j + 1
    return result

def test_module() -> int:
    passed: int = 0
    a: list[str] = ["x", "y", "z"]
    b: list[str] = ["y", "z", "w"]
    d: list[str] = dict_diff_keys(a, b)
    if len(d) == 1 and d[0] == "x":
        passed = passed + 1
    inter: list[str] = dict_intersection_keys(a, b)
    if len(inter) == 2:
        passed = passed + 1
    sym: list[str] = symmetric_diff_keys(a, b)
    if len(sym) == 2:
        passed = passed + 1
    u: list[str] = dict_union_keys(a, b)
    if len(u) == 4:
        passed = passed + 1
    empty: list[str] = dict_diff_keys([], b)
    if len(empty) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
