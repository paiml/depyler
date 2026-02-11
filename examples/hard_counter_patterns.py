# Counter/frequency patterns
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def count_frequency(nums: list[int]) -> dict[int, int]:
    """Count the frequency of each element."""
    freq: dict[int, int] = {}
    i: int = 0
    while i < len(nums):
        val: int = nums[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        i = i + 1
    return freq


def most_frequent(nums: list[int]) -> int:
    """Find the most frequent element. Returns -1 if empty."""
    if len(nums) == 0:
        return -1
    freq: dict[int, int] = count_frequency(nums)
    best: int = nums[0]
    best_count: int = 0
    for k in freq:
        if freq[k] > best_count:
            best_count = freq[k]
            best = k
    return best


def has_duplicates(nums: list[int]) -> bool:
    """Check if the list contains any duplicate values."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(nums):
        val: int = nums[i]
        if val in seen:
            return True
        seen[val] = 1
        i = i + 1
    return False


def unique_count(nums: list[int]) -> int:
    """Count the number of unique elements in a list."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(nums):
        seen[nums[i]] = 1
        i = i + 1
    count: int = 0
    for k in seen:
        count = count + 1
    return count


def test_module() -> int:
    freq: dict[int, int] = count_frequency([1, 2, 2, 3, 3, 3])
    assert freq[1] == 1
    assert freq[2] == 2
    assert freq[3] == 3
    assert most_frequent([1, 2, 2, 3, 3, 3]) == 3
    assert most_frequent([5]) == 5
    assert most_frequent([]) == -1
    assert has_duplicates([1, 2, 3, 2]) == True
    assert has_duplicates([1, 2, 3]) == False
    assert has_duplicates([]) == False
    assert unique_count([1, 2, 2, 3, 3, 3]) == 3
    assert unique_count([]) == 0
    assert unique_count([7, 7, 7]) == 1
    return 0


if __name__ == "__main__":
    test_module()
