"""Hard list algorithm patterns for transpiler stress-testing.

Tests: two-pointer, sliding window, rotation, partitioning,
merge sorted, dedup, flattening, running stats, set operations,
Kadane's, even/odd reorder, spiral matrix, Pascal's triangle,
Dutch national flag, run-length encoding.
"""


def two_pointer_pair_sum(nums: list[int], target: int) -> list[list[int]]:
    """Find all unique pairs that sum to target using two pointers.

    Assumes input is sorted. Returns pairs in ascending order.
    """
    result: list[list[int]] = []
    left: int = 0
    right: int = len(nums) - 1
    while left < right:
        current_sum: int = nums[left] + nums[right]
        if current_sum == target:
            result.append([nums[left], nums[right]])
            left += 1
            right -= 1
            while left < right and nums[left] == nums[left - 1]:
                left += 1
            while left < right and nums[right] == nums[right + 1]:
                right -= 1
        elif current_sum < target:
            left += 1
        else:
            right -= 1
    return result


def sliding_window_max_sum(nums: list[int], k: int) -> int:
    """Find maximum sum of any contiguous subarray of size k.

    Returns 0 if k is larger than the list or list is empty.
    """
    n: int = len(nums)
    if n == 0 or k <= 0 or k > n:
        return 0
    window_sum: int = 0
    for i in range(k):
        window_sum += nums[i]
    max_sum: int = window_sum
    for i in range(k, n):
        window_sum += nums[i] - nums[i - k]
        if window_sum > max_sum:
            max_sum = window_sum
    return max_sum


def sliding_window_average(nums: list[int], k: int) -> list[float]:
    """Compute sliding window averages of size k.

    Returns a list of averages for each window position.
    """
    n: int = len(nums)
    if n == 0 or k <= 0 or k > n:
        return []
    result: list[float] = []
    window_sum: int = 0
    for i in range(k):
        window_sum += nums[i]
    result.append(float(window_sum) / float(k))
    for i in range(k, n):
        window_sum += nums[i] - nums[i - k]
        result.append(float(window_sum) / float(k))
    return result


def rotate_left(arr: list[int], k: int) -> list[int]:
    """Rotate list left by k positions.

    Uses the reversal algorithm for in-place-style rotation.
    """
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
    if k == 0:
        return arr[:]
    result: list[int] = arr[:]
    result = reverse_sublist(result, 0, k - 1)
    result = reverse_sublist(result, k, n - 1)
    result = reverse_sublist(result, 0, n - 1)
    return result


def rotate_right(arr: list[int], k: int) -> list[int]:
    """Rotate list right by k positions.

    Uses the reversal algorithm for in-place-style rotation.
    """
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
    if k == 0:
        return arr[:]
    result: list[int] = arr[:]
    result = reverse_sublist(result, 0, n - 1)
    result = reverse_sublist(result, 0, k - 1)
    result = reverse_sublist(result, k, n - 1)
    return result


def reverse_sublist(arr: list[int], start: int, end: int) -> list[int]:
    """Reverse a sublist in place from start to end indices."""
    result: list[int] = arr[:]
    lo: int = start
    hi: int = end
    while lo < hi:
        temp: int = result[lo]
        result[lo] = result[hi]
        result[hi] = temp
        lo += 1
        hi -= 1
    return result


def partition_around_pivot(arr: list[int], pivot: int) -> list[int]:
    """Partition list so elements < pivot come first, then >= pivot.

    Preserves relative order within each partition (stable).
    """
    less: list[int] = []
    greater_eq: list[int] = []
    for val in arr:
        if val < pivot:
            less.append(val)
        else:
            greater_eq.append(val)
    result: list[int] = []
    for val in less:
        result.append(val)
    for val in greater_eq:
        result.append(val)
    return result


def merge_sorted_lists(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted lists into a single sorted list."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    while i < len(a):
        result.append(a[i])
        i += 1
    while j < len(b):
        result.append(b[j])
        j += 1
    return result


def remove_duplicates_sorted(arr: list[int]) -> list[int]:
    """Remove duplicates from a sorted list, keeping first occurrence."""
    if len(arr) == 0:
        return []
    result: list[int] = [arr[0]]
    for i in range(1, len(arr)):
        if arr[i] != arr[i - 1]:
            result.append(arr[i])
    return result


def flatten_nested(nested: list[list[int]]) -> list[int]:
    """Flatten a list of lists into a single list."""
    result: list[int] = []
    for sublist in nested:
        for item in sublist:
            result.append(item)
    return result


def running_max(nums: list[int]) -> list[int]:
    """Compute running maximum at each position.

    running_max([3,1,4,1,5]) returns [3,3,4,4,5].
    """
    if len(nums) == 0:
        return []
    result: list[int] = [nums[0]]
    current_max: int = nums[0]
    for i in range(1, len(nums)):
        if nums[i] > current_max:
            current_max = nums[i]
        result.append(current_max)
    return result


def running_min(nums: list[int]) -> list[int]:
    """Compute running minimum at each position.

    running_min([3,1,4,1,5]) returns [3,1,1,1,1].
    """
    if len(nums) == 0:
        return []
    result: list[int] = [nums[0]]
    current_min: int = nums[0]
    for i in range(1, len(nums)):
        if nums[i] < current_min:
            current_min = nums[i]
        result.append(current_min)
    return result


def running_mean(nums: list[int]) -> list[float]:
    """Compute running mean at each position.

    running_mean([2,4,6]) returns [2.0, 3.0, 4.0].
    """
    if len(nums) == 0:
        return []
    result: list[float] = []
    total: int = 0
    for i in range(len(nums)):
        total += nums[i]
        result.append(float(total) / float(i + 1))
    return result


def list_intersection(a: list[int], b: list[int]) -> list[int]:
    """Find elements present in both lists (no duplicates in output)."""
    seen: dict[int, bool] = {}
    for val in b:
        seen[val] = True
    result: list[int] = []
    added: dict[int, bool] = {}
    for val in a:
        if val in seen and val not in added:
            result.append(val)
            added[val] = True
    return result


def list_difference(a: list[int], b: list[int]) -> list[int]:
    """Find elements in a but not in b (no duplicates in output)."""
    exclude: dict[int, bool] = {}
    for val in b:
        exclude[val] = True
    result: list[int] = []
    added: dict[int, bool] = {}
    for val in a:
        if val not in exclude and val not in added:
            result.append(val)
            added[val] = True
    return result


def kadane_max_subarray(nums: list[int]) -> int:
    """Find maximum subarray sum using Kadane's algorithm.

    Returns 0 for empty input.
    """
    if len(nums) == 0:
        return 0
    max_ending_here: int = nums[0]
    max_so_far: int = nums[0]
    for i in range(1, len(nums)):
        if max_ending_here + nums[i] > nums[i]:
            max_ending_here = max_ending_here + nums[i]
        else:
            max_ending_here = nums[i]
        if max_ending_here > max_so_far:
            max_so_far = max_ending_here
    return max_so_far


def even_odd_partition(nums: list[int]) -> list[int]:
    """Reorder list so all even numbers come before odd numbers.

    Preserves relative order within each group (stable).
    """
    evens: list[int] = []
    odds: list[int] = []
    for val in nums:
        if val % 2 == 0:
            evens.append(val)
        else:
            odds.append(val)
    result: list[int] = []
    for val in evens:
        result.append(val)
    for val in odds:
        result.append(val)
    return result


def spiral_order(matrix: list[list[int]]) -> list[int]:
    """Return elements of a matrix in spiral order.

    Traverses: right across top, down right side, left across bottom,
    up left side, then inward.
    """
    result: list[int] = []
    if len(matrix) == 0:
        return result
    top: int = 0
    bottom: int = len(matrix) - 1
    left: int = 0
    right: int = len(matrix[0]) - 1
    while top <= bottom and left <= right:
        col: int = left
        while col <= right:
            result.append(matrix[top][col])
            col += 1
        top += 1
        row: int = top
        while row <= bottom:
            result.append(matrix[row][right])
            row += 1
        right -= 1
        if top <= bottom:
            col = right
            while col >= left:
                result.append(matrix[bottom][col])
                col -= 1
            bottom -= 1
        if left <= right:
            row = bottom
            while row >= top:
                result.append(matrix[row][left])
                row -= 1
            left += 1
    return result


def pascals_triangle(n: int) -> list[list[int]]:
    """Generate first n rows of Pascal's triangle.

    Row 0 is [1], row 1 is [1,1], row 2 is [1,2,1], etc.
    """
    if n <= 0:
        return []
    triangle: list[list[int]] = [[1]]
    for i in range(1, n):
        prev: list[int] = triangle[i - 1]
        row: list[int] = [1]
        for j in range(1, i):
            row.append(prev[j - 1] + prev[j])
        row.append(1)
        triangle.append(row)
    return triangle


def dutch_national_flag(arr: list[int]) -> list[int]:
    """Sort an array containing only 0s, 1s, and 2s.

    Uses the Dutch National Flag algorithm (three-way partition).
    Single pass, O(n) time, O(n) space for the copy.
    """
    result: list[int] = arr[:]
    lo: int = 0
    mid: int = 0
    hi: int = len(result) - 1
    while mid <= hi:
        if result[mid] == 0:
            temp: int = result[lo]
            result[lo] = result[mid]
            result[mid] = temp
            lo += 1
            mid += 1
        elif result[mid] == 1:
            mid += 1
        else:
            temp2: int = result[mid]
            result[mid] = result[hi]
            result[hi] = temp2
            hi -= 1
    return result


def run_length_encode(arr: list[int]) -> list[list[int]]:
    """Run-length encode a list.

    Returns list of [value, count] pairs.
    run_length_encode([1,1,2,2,2,3]) returns [[1,2],[2,3],[3,1]].
    """
    if len(arr) == 0:
        return []
    result: list[list[int]] = []
    current: int = arr[0]
    count: int = 1
    for i in range(1, len(arr)):
        if arr[i] == current:
            count += 1
        else:
            result.append([current, count])
            current = arr[i]
            count = 1
    result.append([current, count])
    return result


def run_length_decode(encoded: list[list[int]]) -> list[int]:
    """Decode a run-length encoded list.

    Takes list of [value, count] pairs and expands them.
    """
    result: list[int] = []
    for pair in encoded:
        val: int = pair[0]
        count: int = pair[1]
        for _ in range(count):
            result.append(val)
    return result


def test_all() -> bool:
    """Comprehensive test suite for all list algorithm functions."""
    # Test two_pointer_pair_sum
    pairs: list[list[int]] = two_pointer_pair_sum([1, 2, 3, 4, 5, 6], 7)
    assert len(pairs) == 3, "pair_sum count"
    assert pairs[0] == [1, 6], "pair_sum first"
    assert pairs[1] == [2, 5], "pair_sum second"
    assert pairs[2] == [3, 4], "pair_sum third"
    empty_pairs: list[list[int]] = two_pointer_pair_sum([], 5)
    assert len(empty_pairs) == 0, "pair_sum empty"

    # Test sliding_window_max_sum
    assert sliding_window_max_sum([1, 4, 2, 10, 2, 3, 1, 0, 20], 4) == 24, "sw max"
    assert sliding_window_max_sum([1, 2, 3], 3) == 6, "sw max full"
    assert sliding_window_max_sum([], 3) == 0, "sw max empty"
    assert sliding_window_max_sum([5], 1) == 5, "sw max single"
    assert sliding_window_max_sum([1, 2], 5) == 0, "sw max k>n"

    # Test sliding_window_average
    avgs: list[float] = sliding_window_average([1, 3, 5, 7], 2)
    assert len(avgs) == 3, "sw avg count"
    assert avgs[0] == 2.0, "sw avg first"
    assert avgs[1] == 4.0, "sw avg second"
    assert avgs[2] == 6.0, "sw avg third"

    # Test rotate_left
    assert rotate_left([1, 2, 3, 4, 5], 2) == [3, 4, 5, 1, 2], "rotate left 2"
    assert rotate_left([1, 2, 3], 0) == [1, 2, 3], "rotate left 0"
    assert rotate_left([1, 2, 3], 3) == [1, 2, 3], "rotate left full"
    assert rotate_left([], 5) == [], "rotate left empty"

    # Test rotate_right
    assert rotate_right([1, 2, 3, 4, 5], 2) == [4, 5, 1, 2, 3], "rotate right 2"
    assert rotate_right([1, 2, 3], 0) == [1, 2, 3], "rotate right 0"
    assert rotate_right([], 5) == [], "rotate right empty"

    # Test reverse_sublist
    assert reverse_sublist([1, 2, 3, 4, 5], 1, 3) == [1, 4, 3, 2, 5], "rev sub"

    # Test partition_around_pivot
    partitioned: list[int] = partition_around_pivot([3, 1, 4, 1, 5, 9, 2, 6], 4)
    less_part: list[int] = [3, 1, 1, 2]
    geq_part: list[int] = [4, 5, 9, 6]
    assert partitioned == less_part + geq_part, "partition"

    # Test merge_sorted_lists
    merged: list[int] = merge_sorted_lists([1, 3, 5], [2, 4, 6])
    assert merged == [1, 2, 3, 4, 5, 6], "merge sorted"
    assert merge_sorted_lists([], [1, 2]) == [1, 2], "merge empty left"
    assert merge_sorted_lists([1, 2], []) == [1, 2], "merge empty right"

    # Test remove_duplicates_sorted
    assert remove_duplicates_sorted([1, 1, 2, 3, 3, 3, 4]) == [1, 2, 3, 4], "dedup"
    assert remove_duplicates_sorted([]) == [], "dedup empty"
    assert remove_duplicates_sorted([5]) == [5], "dedup single"

    # Test flatten_nested
    assert flatten_nested([[1, 2], [3], [4, 5, 6]]) == [1, 2, 3, 4, 5, 6], "flatten"
    assert flatten_nested([]) == [], "flatten empty"
    assert flatten_nested([[], [1], []]) == [1], "flatten sparse"

    # Test running_max
    assert running_max([3, 1, 4, 1, 5]) == [3, 3, 4, 4, 5], "running max"
    assert running_max([]) == [], "running max empty"
    assert running_max([7]) == [7], "running max single"

    # Test running_min
    assert running_min([3, 1, 4, 1, 5]) == [3, 1, 1, 1, 1], "running min"
    assert running_min([]) == [], "running min empty"

    # Test running_mean
    means: list[float] = running_mean([2, 4, 6])
    assert means[0] == 2.0, "running mean first"
    assert means[1] == 3.0, "running mean second"
    assert means[2] == 4.0, "running mean third"
    assert running_mean([]) == [], "running mean empty"

    # Test list_intersection
    assert list_intersection([1, 2, 3, 4], [3, 4, 5, 6]) == [3, 4], "intersection"
    assert list_intersection([1, 2], [3, 4]) == [], "intersection disjoint"
    assert list_intersection([], [1, 2]) == [], "intersection empty"

    # Test list_difference
    assert list_difference([1, 2, 3, 4], [3, 4, 5]) == [1, 2], "difference"
    assert list_difference([1, 2], []) == [1, 2], "difference empty b"
    assert list_difference([], [1, 2]) == [], "difference empty a"

    # Test kadane_max_subarray
    assert kadane_max_subarray([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6, "kadane"
    assert kadane_max_subarray([1, 2, 3]) == 6, "kadane all pos"
    assert kadane_max_subarray([-1, -2, -3]) == -1, "kadane all neg"
    assert kadane_max_subarray([]) == 0, "kadane empty"
    assert kadane_max_subarray([42]) == 42, "kadane single"

    # Test even_odd_partition
    eo: list[int] = even_odd_partition([3, 2, 5, 4, 1, 6])
    assert eo == [2, 4, 6, 3, 5, 1], "even odd"
    assert even_odd_partition([]) == [], "even odd empty"
    assert even_odd_partition([2, 4]) == [2, 4], "even odd all even"

    # Test spiral_order
    matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    assert spiral_order(matrix) == [1, 2, 3, 6, 9, 8, 7, 4, 5], "spiral 3x3"
    assert spiral_order([]) == [], "spiral empty"
    assert spiral_order([[1, 2, 3, 4]]) == [1, 2, 3, 4], "spiral single row"
    col_matrix: list[list[int]] = [[1], [2], [3]]
    assert spiral_order(col_matrix) == [1, 2, 3], "spiral single col"

    # Test pascals_triangle
    tri: list[list[int]] = pascals_triangle(5)
    assert len(tri) == 5, "pascal rows"
    assert tri[0] == [1], "pascal row 0"
    assert tri[1] == [1, 1], "pascal row 1"
    assert tri[2] == [1, 2, 1], "pascal row 2"
    assert tri[3] == [1, 3, 3, 1], "pascal row 3"
    assert tri[4] == [1, 4, 6, 4, 1], "pascal row 4"
    assert pascals_triangle(0) == [], "pascal zero"
    assert pascals_triangle(1) == [[1]], "pascal one"

    # Test dutch_national_flag
    assert dutch_national_flag([2, 0, 1, 2, 0, 1]) == [0, 0, 1, 1, 2, 2], "dnf"
    assert dutch_national_flag([0, 0, 0]) == [0, 0, 0], "dnf all zeros"
    assert dutch_national_flag([2, 2, 1, 1, 0, 0]) == [0, 0, 1, 1, 2, 2], "dnf rev"
    assert dutch_national_flag([]) == [], "dnf empty"

    # Test run_length_encode
    encoded: list[list[int]] = run_length_encode([1, 1, 2, 2, 2, 3])
    assert encoded == [[1, 2], [2, 3], [3, 1]], "rle encode"
    assert run_length_encode([5]) == [[5, 1]], "rle single"
    assert run_length_encode([]) == [], "rle empty"

    # Test run_length_decode
    decoded: list[int] = run_length_decode([[1, 2], [2, 3], [3, 1]])
    assert decoded == [1, 1, 2, 2, 2, 3], "rle decode"
    assert run_length_decode([]) == [], "rle decode empty"

    # Test encode-decode roundtrip
    original: list[int] = [7, 7, 7, 3, 3, 1, 1, 1, 1]
    roundtrip: list[int] = run_length_decode(run_length_encode(original))
    assert roundtrip == original, "rle roundtrip"

    return True


if __name__ == "__main__":
    result: bool = test_all()
    if result:
        print("All list algorithm tests passed!")
