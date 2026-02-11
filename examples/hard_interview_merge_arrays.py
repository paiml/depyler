def merge_sorted(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    j: int = 0
    na: int = len(a)
    nb: int = len(b)
    while i < na and j < nb:
        if a[i] <= b[j]:
            result.append(a[i])
            i = i + 1
        else:
            result.append(b[j])
            j = j + 1
    while i < na:
        result.append(a[i])
        i = i + 1
    while j < nb:
        result.append(b[j])
        j = j + 1
    return result

def intersection_sorted(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    j: int = 0
    na: int = len(a)
    nb: int = len(b)
    while i < na and j < nb:
        if a[i] == b[j]:
            result.append(a[i])
            i = i + 1
            j = j + 1
        elif a[i] < b[j]:
            i = i + 1
        else:
            j = j + 1
    return result

def union_sorted(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    j: int = 0
    na: int = len(a)
    nb: int = len(b)
    while i < na and j < nb:
        if a[i] < b[j]:
            result.append(a[i])
            i = i + 1
        elif a[i] > b[j]:
            result.append(b[j])
            j = j + 1
        else:
            result.append(a[i])
            i = i + 1
            j = j + 1
    while i < na:
        result.append(a[i])
        i = i + 1
    while j < nb:
        result.append(b[j])
        j = j + 1
    return result

def test_module() -> int:
    passed: int = 0
    r1: list[int] = merge_sorted([1, 3, 5], [2, 4, 6])
    if r1 == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1
    r2: list[int] = intersection_sorted([1, 2, 3, 4], [2, 4, 6])
    if r2 == [2, 4]:
        passed = passed + 1
    r3: list[int] = union_sorted([1, 3, 5], [2, 3, 6])
    if r3 == [1, 2, 3, 5, 6]:
        passed = passed + 1
    r4: list[int] = merge_sorted([], [1, 2])
    if r4 == [1, 2]:
        passed = passed + 1
    r5: list[int] = intersection_sorted([1, 2], [3, 4])
    if r5 == []:
        passed = passed + 1
    r6: list[int] = union_sorted([1, 2, 3], [1, 2, 3])
    if r6 == [1, 2, 3]:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
