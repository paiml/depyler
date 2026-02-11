def counter_from_list(arr: list[int]) -> dict[int, int]:
    freq: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        i = i + 1
    return freq

def counter_add(a: dict[int, int], b: dict[int, int], all_keys: list[int]) -> dict[int, int]:
    result: dict[int, int] = {}
    i: int = 0
    while i < len(all_keys):
        ak: int = all_keys[i]
        total: int = 0
        if ak in a:
            total = total + a[ak]
        if ak in b:
            total = total + b[ak]
        if total > 0:
            result[ak] = total
        i = i + 1
    return result

def counter_most_common(freq: dict[int, int], candidates: list[int]) -> int:
    best_val: int = -1
    best_count: int = 0
    i: int = 0
    while i < len(candidates):
        c: int = candidates[i]
        if c in freq:
            cnt: int = freq[c]
            if cnt > best_count:
                best_count = cnt
                best_val = c
        i = i + 1
    return best_val

def counter_total(freq: dict[int, int], candidates: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(candidates):
        c: int = candidates[i]
        if c in freq:
            total = total + freq[c]
        i = i + 1
    return total

def test_module() -> int:
    passed: int = 0
    c: dict[int, int] = counter_from_list([1, 2, 2, 3, 3, 3])
    if c[3] == 3:
        passed = passed + 1
    if c[1] == 1:
        passed = passed + 1
    mc: int = counter_most_common(c, [1, 2, 3])
    if mc == 3:
        passed = passed + 1
    if counter_total(c, [1, 2, 3]) == 6:
        passed = passed + 1
    c2: dict[int, int] = counter_from_list([2, 2])
    merged: dict[int, int] = counter_add(c, c2, [1, 2, 3])
    if merged[2] == 4:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
