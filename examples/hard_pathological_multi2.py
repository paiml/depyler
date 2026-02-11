# Pathological multi-function: Functions returning lists used by other functions
# Tests: list transformation pipelines, chained function composition


def generate_range(start: int, end: int) -> list[int]:
    """Generate list of integers from start to end-1."""
    result: list[int] = []
    i: int = start
    while i < end:
        result.append(i)
        i = i + 1
    return result


def filter_even(nums: list[int]) -> list[int]:
    """Keep only even numbers."""
    result: list[int] = []
    i: int = 0
    while i < len(nums):
        if nums[i] % 2 == 0:
            result.append(nums[i])
        i = i + 1
    return result


def map_square(nums: list[int]) -> list[int]:
    """Square each number."""
    result: list[int] = []
    i: int = 0
    while i < len(nums):
        result.append(nums[i] * nums[i])
        i = i + 1
    return result


def take_first_n(nums: list[int], n: int) -> list[int]:
    """Take first n elements."""
    result: list[int] = []
    i: int = 0
    limit: int = n
    if len(nums) < limit:
        limit = len(nums)
    while i < limit:
        result.append(nums[i])
        i = i + 1
    return result


def sum_list(nums: list[int]) -> int:
    """Sum all elements."""
    total: int = 0
    i: int = 0
    while i < len(nums):
        total = total + nums[i]
        i = i + 1
    return total


def zip_sum(a: list[int], b: list[int]) -> list[int]:
    """Element-wise sum of two lists (shorter length)."""
    result: list[int] = []
    i: int = 0
    limit: int = len(a)
    if len(b) < limit:
        limit = len(b)
    while i < limit:
        result.append(a[i] + b[i])
        i = i + 1
    return result


def concat_lists(a: list[int], b: list[int]) -> list[int]:
    """Concatenate two lists."""
    result: list[int] = []
    i: int = 0
    while i < len(a):
        result.append(a[i])
        i = i + 1
    j: int = 0
    while j < len(b):
        result.append(b[j])
        j = j + 1
    return result


def pipeline_process(start: int, end: int, take_n: int) -> int:
    """Full pipeline: generate -> filter even -> square -> take n -> sum."""
    generated: list[int] = generate_range(start, end)
    evens: list[int] = filter_even(generated)
    squared: list[int] = map_square(evens)
    taken: list[int] = take_first_n(squared, take_n)
    return sum_list(taken)


def double_pipeline(start: int, mid: int, end: int) -> list[int]:
    """Two pipelines zipped together."""
    pipe1: list[int] = map_square(generate_range(start, mid))
    pipe2: list[int] = filter_even(generate_range(mid, end))
    return zip_sum(pipe1, pipe2)


def test_module() -> int:
    passed: int = 0
    # Test 1: generate range
    r: list[int] = generate_range(0, 5)
    if len(r) == 5:
        passed = passed + 1
    # Test 2: filter even
    evens: list[int] = filter_even([1, 2, 3, 4, 5, 6])
    if len(evens) == 3:
        passed = passed + 1
    # Test 3: map square
    sq: list[int] = map_square([1, 2, 3])
    if sq[2] == 9:
        passed = passed + 1
    # Test 4: pipeline: range(0,10) -> evens [0,2,4,6,8] -> sq [0,4,16,36,64] -> take 3 [0,4,16] -> sum 20
    if pipeline_process(0, 10, 3) == 20:
        passed = passed + 1
    # Test 5: zip sum
    zs: list[int] = zip_sum([1, 2, 3], [10, 20, 30])
    if zs[0] == 11 and zs[1] == 22 and zs[2] == 33:
        passed = passed + 1
    # Test 6: concat
    c: list[int] = concat_lists([1, 2], [3, 4])
    if len(c) == 4:
        passed = passed + 1
    # Test 7: sum
    if sum_list([10, 20, 30]) == 60:
        passed = passed + 1
    return passed
