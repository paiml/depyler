def merge_dicts(a: dict[str, int], b: dict[str, int], b_keys: list[str]) -> dict[str, int]:
    result: dict[str, int] = {}
    i: int = 0
    while i < len(b_keys):
        bk: str = b_keys[i]
        if bk in a and bk in b:
            result[bk] = a[bk] + b[bk]
        elif bk in b:
            result[bk] = b[bk]
        i = i + 1
    return result

def merge_prefer_left(a: dict[str, int], b: dict[str, int], all_keys: list[str]) -> dict[str, int]:
    result: dict[str, int] = {}
    i: int = 0
    while i < len(all_keys):
        ak: str = all_keys[i]
        if ak in a:
            result[ak] = a[ak]
        elif ak in b:
            result[ak] = b[ak]
        i = i + 1
    return result

def merge_prefer_right(a: dict[str, int], b: dict[str, int], all_keys: list[str]) -> dict[str, int]:
    result: dict[str, int] = {}
    i: int = 0
    while i < len(all_keys):
        ak: str = all_keys[i]
        if ak in b:
            result[ak] = b[ak]
        elif ak in a:
            result[ak] = a[ak]
        i = i + 1
    return result

def dict_size(d: dict[str, int], probe_keys: list[str]) -> int:
    count: int = 0
    i: int = 0
    while i < len(probe_keys):
        if probe_keys[i] in d:
            count = count + 1
        i = i + 1
    return count

def test_module() -> int:
    passed: int = 0
    a: dict[str, int] = {"x": 1, "y": 2}
    b: dict[str, int] = {"y": 3, "z": 4}
    m: dict[str, int] = merge_dicts(a, b, ["y", "z"])
    if m["y"] == 5:
        passed = passed + 1
    if m["z"] == 4:
        passed = passed + 1
    ml: dict[str, int] = merge_prefer_left(a, b, ["x", "y", "z"])
    if ml["y"] == 2:
        passed = passed + 1
    mr: dict[str, int] = merge_prefer_right(a, b, ["x", "y", "z"])
    if mr["y"] == 3:
        passed = passed + 1
    if dict_size(a, ["x", "y", "z"]) == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
