def mm_add(store: dict[int, int], counts: dict[int, int], tag: int, value: int) -> int:
    if tag in store:
        prev: int = store[tag]
        store[tag] = prev + value
        counts[tag] = counts[tag] + 1
    else:
        store[tag] = value
        counts[tag] = 1
    return 1

def mm_count(counts: dict[int, int], tag: int) -> int:
    if tag in counts:
        return counts[tag]
    return 0

def mm_sum(store: dict[int, int], tag: int) -> int:
    if tag in store:
        return store[tag]
    return 0

def mm_has(counts: dict[int, int], tag: int) -> int:
    if tag in counts:
        return 1
    return 0

def mm_remove(store: dict[int, int], counts: dict[int, int], tag: int) -> int:
    if tag in store:
        del store[tag]
        del counts[tag]
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    st: dict[int, int] = {}
    ct: dict[int, int] = {}
    mm_add(st, ct, 1, 10)
    mm_add(st, ct, 1, 20)
    mm_add(st, ct, 2, 100)
    if mm_count(ct, 1) == 2:
        passed = passed + 1
    if mm_count(ct, 2) == 1:
        passed = passed + 1
    if mm_sum(st, 1) == 30:
        passed = passed + 1
    if mm_has(ct, 1) == 1:
        passed = passed + 1
    mm_remove(st, ct, 2)
    if mm_has(ct, 2) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
