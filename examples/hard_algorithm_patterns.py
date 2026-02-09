"""Hard algorithm patterns that stress numeric and collection handling.

Tests: sorting, searching, graph algorithms, dynamic programming,
numeric precision, nested collections.
"""


def bubble_sort(arr: list[int]) -> list[int]:
    """Bubble sort implementation."""
    n: int = len(arr)
    result: list[int] = arr.copy()
    for i in range(n):
        for j in range(0, n - i - 1):
            if result[j] > result[j + 1]:
                temp: int = result[j]
                result[j] = result[j + 1]
                result[j + 1] = temp
    return result


def binary_search(arr: list[int], target: int) -> int:
    """Binary search returning index or -1."""
    left: int = 0
    right: int = len(arr) - 1
    while left <= right:
        mid: int = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1


def merge_sort(arr: list[int]) -> list[int]:
    """Merge sort implementation."""
    if len(arr) <= 1:
        return arr
    mid: int = len(arr) // 2
    left: list[int] = merge_sort(arr[:mid])
    right: list[int] = merge_sort(arr[mid:])
    return merge(left, right)


def merge(left: list[int], right: list[int]) -> list[int]:
    """Merge two sorted lists."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    while i < len(left):
        result.append(left[i])
        i += 1
    while j < len(right):
        result.append(right[j])
        j += 1
    return result


def fibonacci_memo(n: int) -> int:
    """Fibonacci with memoization."""
    memo: dict[int, int] = {}
    return fib_helper(n, memo)


def fib_helper(n: int, memo: dict[int, int]) -> int:
    """Helper for memoized fibonacci."""
    if n <= 1:
        return n
    if n in memo:
        return memo[n]
    result: int = fib_helper(n - 1, memo) + fib_helper(n - 2, memo)
    memo[n] = result
    return result


def two_sum(nums: list[int], target: int) -> list[int]:
    """Find two indices that sum to target."""
    seen: dict[int, int] = {}
    for i, num in enumerate(nums):
        complement: int = target - num
        if complement in seen:
            return [seen[complement], i]
        seen[num] = i
    return []


def max_subarray(nums: list[int]) -> int:
    """Maximum subarray sum (Kadane's algorithm)."""
    if not nums:
        return 0
    max_sum: int = nums[0]
    current: int = nums[0]
    for i in range(1, len(nums)):
        if current + nums[i] > nums[i]:
            current = current + nums[i]
        else:
            current = nums[i]
        if current > max_sum:
            max_sum = current
    return max_sum


def is_prime(n: int) -> bool:
    """Check if number is prime."""
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i: int = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def matrix_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Matrix multiplication."""
    rows_a: int = len(a)
    cols_b: int = len(b[0])
    cols_a: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows_a):
        row: list[int] = []
        for j in range(cols_b):
            total: int = 0
            for k in range(cols_a):
                total += a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result
