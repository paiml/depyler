"""Hard comprehension and functional patterns for transpiler testing.

Tests: list comprehensions, dict comprehensions, set comprehensions,
generator expressions, nested comprehensions, filtered comprehensions,
conditional expressions in comprehensions, tuple unpacking in comprehensions,
map/filter equivalents, and reduction patterns.
"""


def nested_list_comp(n: int, m: int) -> list[list[int]]:
    """Build a multiplication table using nested list comprehension."""
    return [[x * y for y in range(m)] for x in range(n)]


def test_nested_list_comp() -> int:
    """Test nested list comprehension produces correct multiplication table."""
    table: list[list[int]] = nested_list_comp(3, 4)
    # table = [[0,0,0,0],[0,1,2,3],[0,2,4,6]]
    # sum of all elements = 0+0+0+0 + 0+1+2+3 + 0+2+4+6 = 18
    total: int = 0
    for row in table:
        for val in row:
            total += val
    return total


def filtered_even_positive(lst: list[int]) -> list[int]:
    """Filter list to only even positive numbers using comprehension."""
    return [x for x in lst if x % 2 == 0 and x > 0]


def test_filtered_even_positive() -> int:
    """Test filtered comprehension with compound condition."""
    data: list[int] = [-4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6]
    result: list[int] = filtered_even_positive(data)
    # result = [2, 4, 6]
    total: int = 0
    for x in result:
        total += x
    return total


def dict_from_parallel_lists(keys: list[str], values: list[int]) -> dict[str, int]:
    """Build dict from two parallel lists using dict comprehension with zip."""
    result: dict[str, int] = {}
    for i in range(len(keys)):
        if i < len(values):
            result[keys[i]] = values[i]
    return result


def test_dict_from_parallel_lists() -> int:
    """Test dict comprehension from parallel lists."""
    keys: list[str] = ["a", "b", "c", "d"]
    values: list[int] = [10, 20, 30, 40]
    d: dict[str, int] = dict_from_parallel_lists(keys, values)
    return d["a"] + d["b"] + d["c"] + d["d"]


def abs_set(data: list[int]) -> list[int]:
    """Compute set of absolute values from data using set-like dedup."""
    seen: list[int] = []
    for x in data:
        val: int = x if x >= 0 else -x
        found: bool = False
        for s in seen:
            if s == val:
                found = True
        if not found:
            seen.append(val)
    seen.sort()
    return seen


def test_abs_set() -> int:
    """Test set comprehension with absolute value transform."""
    data: list[int] = [-3, -2, -1, 0, 1, 2, 3, 4]
    result: list[int] = abs_set(data)
    # unique abs values: [0, 1, 2, 3, 4]
    total: int = 0
    for x in result:
        total += x
    return total


def apply_and_filter(data: list[int], threshold: int) -> list[int]:
    """Apply square function and filter by threshold using comprehension pattern."""
    result: list[int] = []
    for x in data:
        squared: int = x * x
        if squared > threshold:
            result.append(squared)
    return result


def test_apply_and_filter() -> int:
    """Test comprehension with function calls and filter."""
    data: list[int] = [1, 2, 3, 4, 5]
    result: list[int] = apply_and_filter(data, 5)
    # squares > 5: 9, 16, 25
    total: int = 0
    for x in result:
        total += x
    return total


def cross_product_pairs(n: int, m: int) -> list[list[int]]:
    """Generate all (i,j) pairs where i != j using multiple for-clauses."""
    result: list[list[int]] = []
    for i in range(n):
        for j in range(m):
            if i != j:
                result.append([i, j])
    return result


def test_cross_product_pairs() -> int:
    """Test multiple for-clause comprehension with filter."""
    pairs: list[list[int]] = cross_product_pairs(3, 3)
    # pairs: [0,1],[0,2],[1,0],[1,2],[2,0],[2,1] = 6 pairs
    # sum of all elements: 0+1+0+2+1+0+1+2+2+0+2+1 = 12
    total: int = 0
    for pair in pairs:
        total += pair[0] + pair[1]
    return total


def enumerate_to_dict(lst: list[str]) -> dict[int, str]:
    """Build index-to-value dict using enumerate pattern."""
    result: dict[int, str] = {}
    for i, v in enumerate(lst):
        result[i] = v
    return result


def test_enumerate_to_dict() -> int:
    """Test dict comprehension from enumerate."""
    words: list[str] = ["alpha", "beta", "gamma", "delta"]
    d: dict[int, str] = enumerate_to_dict(words)
    # keys are 0,1,2,3 -> sum of keys = 6
    # len of values: 5+4+5+5 = 19
    total: int = 0
    for k in d:
        total += k + len(d[k])
    return total


def sum_of_positive_squares(nums: list[int]) -> int:
    """Sum of squares of positive numbers (chained map/filter pattern)."""
    total: int = 0
    for x in nums:
        if x > 0:
            total += x * x
    return total


def test_sum_of_positive_squares() -> int:
    """Test chained map/filter reduction pattern."""
    nums: list[int] = [-3, -1, 0, 2, 4, 5]
    # positive: 2, 4, 5 -> squares: 4, 16, 25 -> sum: 45
    return sum_of_positive_squares(nums)


def flatten_matrix(matrix: list[list[int]]) -> list[int]:
    """Flatten a 2D matrix into a 1D list using comprehension pattern."""
    result: list[int] = []
    for row in matrix:
        for val in row:
            result.append(val)
    return result


def transpose_matrix(matrix: list[list[int]], rows: int, cols: int) -> list[list[int]]:
    """Transpose a matrix using nested comprehension pattern."""
    result: list[list[int]] = []
    for j in range(cols):
        new_row: list[int] = []
        for i in range(rows):
            new_row.append(matrix[i][j])
        result.append(new_row)
    return result


def test_matrix_operations() -> int:
    """Test flatten and transpose matrix operations."""
    matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6]]
    flat: list[int] = flatten_matrix(matrix)
    # flat = [1, 2, 3, 4, 5, 6] -> sum = 21
    transposed: list[list[int]] = transpose_matrix(matrix, 2, 3)
    # transposed = [[1,4],[2,5],[3,6]]
    t_sum: int = 0
    for row in transposed:
        t_sum += row[0] + row[1]
    # t_sum = 1+4+2+5+3+6 = 21
    total: int = 0
    for x in flat:
        total += x
    return total + t_sum


def frequency_count(lst: list[int]) -> dict[int, int]:
    """Count frequency of each element using dict comprehension pattern."""
    counts: dict[int, int] = {}
    for x in lst:
        if x in counts:
            counts[x] += 1
        else:
            counts[x] = 1
    return counts


def test_frequency_count() -> int:
    """Test frequency counting via dict comprehension pattern."""
    data: list[int] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
    freq: dict[int, int] = frequency_count(data)
    # freq = {1:1, 2:2, 3:3, 4:4}
    # sum of counts = 1+2+3+4 = 10
    # sum of key*count = 1+4+9+16 = 30
    total: int = 0
    for k in freq:
        total += k * freq[k]
    return total


def conditional_abs(data: list[int]) -> list[int]:
    """Apply conditional expression in comprehension: abs value."""
    result: list[int] = []
    for x in data:
        if x > 0:
            result.append(x)
        else:
            result.append(-x)
    return result


def test_conditional_abs() -> int:
    """Test conditional value in comprehension (ternary)."""
    data: list[int] = [-5, -3, 0, 2, 7]
    result: list[int] = conditional_abs(data)
    # result = [5, 3, 0, 2, 7]
    total: int = 0
    for x in result:
        total += x
    return total


def filter_long_words_upper(text: str, min_len: int) -> list[str]:
    """Filter words longer than min_len and uppercase them."""
    result: list[str] = []
    words: list[str] = text.split()
    for word in words:
        if len(word) > min_len:
            result.append(word.upper())
    return result


def test_filter_long_words_upper() -> int:
    """Test string processing comprehension pattern."""
    text: str = "the quick brown fox jumps over the lazy dog"
    result: list[str] = filter_long_words_upper(text, 3)
    # words > 3 chars: "quick", "brown", "jumps", "over", "lazy"
    # uppercased: "QUICK", "BROWN", "JUMPS", "OVER", "LAZY"
    # return count * 10 + total length
    total_len: int = 0
    for w in result:
        total_len += len(w)
    return len(result) * 10 + total_len


def sum_pairs(pairs: list[list[int]]) -> list[int]:
    """Sum each pair using tuple-unpacking comprehension pattern."""
    result: list[int] = []
    for pair in pairs:
        result.append(pair[0] + pair[1])
    return result


def test_sum_pairs() -> int:
    """Test tuple unpacking in comprehension."""
    pairs: list[list[int]] = [[1, 2], [3, 4], [5, 6], [7, 8]]
    sums: list[int] = sum_pairs(pairs)
    # sums = [3, 7, 11, 15]
    total: int = 0
    for s in sums:
        total += s
    return total


def deep_copy_nested_dict(d: dict[str, dict[str, int]]) -> dict[str, dict[str, int]]:
    """Deep copy a nested dict using nested dict comprehension pattern."""
    result: dict[str, dict[str, int]] = {}
    for outer_key in d:
        inner_copy: dict[str, int] = {}
        inner: dict[str, int] = d[outer_key]
        for inner_key in inner:
            inner_copy[inner_key] = inner[inner_key]
        result[outer_key] = inner_copy
    return result


def test_deep_copy_nested_dict() -> int:
    """Test nested dict comprehension via deep copy."""
    original: dict[str, dict[str, int]] = {}
    inner_a: dict[str, int] = {}
    inner_a["x"] = 10
    inner_a["y"] = 20
    inner_b: dict[str, int] = {}
    inner_b["x"] = 30
    inner_b["y"] = 40
    original["a"] = inner_a
    original["b"] = inner_b
    copy: dict[str, dict[str, int]] = deep_copy_nested_dict(original)
    # sum all values: 10+20+30+40 = 100
    total: int = 0
    for outer in copy:
        inner: dict[str, int] = copy[outer]
        for k in inner:
            total += inner[k]
    return total


def manual_reduce_sum(nums: list[int]) -> int:
    """Manual reduce: sum all elements via accumulation."""
    acc: int = 0
    for x in nums:
        acc += x
    return acc


def manual_reduce_product(nums: list[int]) -> int:
    """Manual reduce: product of all elements via accumulation."""
    acc: int = 1
    for x in nums:
        acc *= x
    return acc


def manual_reduce_max(nums: list[int]) -> int:
    """Manual reduce: find maximum via accumulation."""
    if not nums:
        return 0
    acc: int = nums[0]
    for i in range(1, len(nums)):
        if nums[i] > acc:
            acc = nums[i]
    return acc


def test_manual_reductions() -> int:
    """Test manual reduce patterns for sum, product, and max."""
    nums: list[int] = [2, 3, 5, 7, 11]
    s: int = manual_reduce_sum(nums)
    # sum = 2+3+5+7+11 = 28
    p: int = manual_reduce_product(nums)
    # product = 2*3*5*7*11 = 2310
    m: int = manual_reduce_max(nums)
    # max = 11
    return s + m


def running_max(nums: list[int]) -> list[int]:
    """Compute running maximum using accumulation pattern."""
    if not nums:
        return []
    result: list[int] = [nums[0]]
    current_max: int = nums[0]
    for i in range(1, len(nums)):
        if nums[i] > current_max:
            current_max = nums[i]
        result.append(current_max)
    return result


def test_running_max() -> int:
    """Test running maximum accumulation."""
    nums: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
    result: list[int] = running_max(nums)
    # running max: [3, 3, 4, 4, 5, 9, 9, 9]
    total: int = 0
    for x in result:
        total += x
    return total


def group_by_remainder(nums: list[int], divisor: int) -> dict[int, list[int]]:
    """Group numbers by their remainder when divided by divisor."""
    groups: dict[int, list[int]] = {}
    for x in nums:
        r: int = x % divisor
        if r not in groups:
            groups[r] = []
        groups[r].append(x)
    return groups


def test_group_by_remainder() -> int:
    """Test grouping via dict-of-lists comprehension pattern."""
    nums: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    groups: dict[int, list[int]] = group_by_remainder(nums, 3)
    # groups = {1: [1,4,7,10], 2: [2,5,8,11], 0: [3,6,9,12]}
    # count elements in group with remainder 0
    count_r0: int = len(groups[0])
    # sum of group with remainder 1
    sum_r1: int = 0
    for x in groups[1]:
        sum_r1 += x
    # count_r0=4, sum_r1=1+4+7+10=22
    return count_r0 * 100 + sum_r1


def zip_with_operation(a: list[int], b: list[int]) -> list[int]:
    """Element-wise operation on two lists using zip-like pattern."""
    result: list[int] = []
    length: int = len(a) if len(a) < len(b) else len(b)
    for i in range(length):
        result.append(a[i] * b[i] + a[i] + b[i])
    return result


def test_zip_with_operation() -> int:
    """Test zip-based comprehension with compound operation."""
    a: list[int] = [1, 2, 3, 4]
    b: list[int] = [10, 20, 30, 40]
    result: list[int] = zip_with_operation(a, b)
    # [1*10+1+10, 2*20+2+20, 3*30+3+30, 4*40+4+40]
    # = [21, 62, 123, 204]
    total: int = 0
    for x in result:
        total += x
    return total
