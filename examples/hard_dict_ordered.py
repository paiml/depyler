def ordered_insert(keys: list[int], vals: list[int], idx_map: dict[int, int], tag: int, val: int) -> int:
    if tag in idx_map:
        pos: int = idx_map[tag]
        vals[pos] = val
        return 0
    idx_map[tag] = len(keys)
    keys.append(tag)
    vals.append(val)
    return 1

def ordered_get(idx_map: dict[int, int], vals: list[int], tag: int) -> int:
    if tag in idx_map:
        pos: int = idx_map[tag]
        return vals[pos]
    return -1

def ordered_size(keys: list[int]) -> int:
    return len(keys)

def ordered_first(keys: list[int]) -> int:
    if len(keys) == 0:
        return -1
    return keys[0]

def ordered_last(keys: list[int]) -> int:
    if len(keys) == 0:
        return -1
    idx: int = len(keys) - 1
    return keys[idx]

def test_module() -> int:
    passed: int = 0
    ks: list[int] = []
    vs: list[int] = []
    im: dict[int, int] = {}
    ordered_insert(ks, vs, im, 10, 100)
    ordered_insert(ks, vs, im, 20, 200)
    ordered_insert(ks, vs, im, 30, 300)
    if ordered_get(im, vs, 20) == 200:
        passed = passed + 1
    if ordered_first(ks) == 10:
        passed = passed + 1
    if ordered_last(ks) == 30:
        passed = passed + 1
    if ordered_size(ks) == 3:
        passed = passed + 1
    ordered_insert(ks, vs, im, 20, 999)
    if ordered_get(im, vs, 20) == 999:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
