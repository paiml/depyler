def dedup_preserve_order(arr: list[int]) -> list[int]:
    result: list[int] = []
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val not in seen:
            seen[val] = 1
            result.append(val)
        i = i + 1
    return result

def count_duplicates(arr: list[int]) -> int:
    freq: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        i = i + 1
    dups: int = 0
    j: int = 0
    while j < len(arr):
        val2: int = arr[j]
        if val2 in freq:
            if freq[val2] > 1:
                dups = dups + 1
                freq[val2] = 0
        j = j + 1
    return dups

def has_duplicates(arr: list[int]) -> int:
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in seen:
            return 1
        seen[val] = 1
        i = i + 1
    return 0

def test_module() -> int:
    passed: int = 0
    d1: list[int] = dedup_preserve_order([1, 2, 3, 2, 1, 4])
    if len(d1) == 4:
        passed = passed + 1
    if d1[0] == 1 and d1[3] == 4:
        passed = passed + 1
    d2: list[int] = dedup_preserve_order([5, 5, 5])
    if len(d2) == 1:
        passed = passed + 1
    if count_duplicates([1, 2, 2, 3, 3, 3]) == 2:
        passed = passed + 1
    if has_duplicates([1, 2, 3]) == 0:
        passed = passed + 1
    if has_duplicates([1, 2, 1]) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
