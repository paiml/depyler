def majority_vote(arr: list[int]) -> int:
    candidate: int = 0
    count: int = 0
    i: int = 0
    while i < len(arr):
        if count == 0:
            candidate = arr[i]
            count = 1
        elif arr[i] == candidate:
            count = count + 1
        else:
            count = count - 1
        i = i + 1
    return candidate

def verify_majority(arr: list[int], candidate: int) -> int:
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == candidate:
            count = count + 1
        i = i + 1
    if count > len(arr) // 2:
        return 1
    return 0

def find_majority(arr: list[int]) -> int:
    cand: int = majority_vote(arr)
    if verify_majority(arr, cand) == 1:
        return cand
    return -1

def count_occurrences(arr: list[int], target: int) -> int:
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count

def test_module() -> int:
    passed: int = 0
    if majority_vote([1, 1, 2, 1, 3]) == 1:
        passed = passed + 1
    if verify_majority([1, 1, 2, 1, 3], 1) == 1:
        passed = passed + 1
    if find_majority([1, 2, 3]) == -1:
        passed = passed + 1
    if find_majority([3, 3, 3, 2, 1]) == 3:
        passed = passed + 1
    if count_occurrences([5, 5, 5, 5], 5) == 4:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
