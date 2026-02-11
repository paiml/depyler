"""Deep recursion patterns: fibonacci variants, power, factorial."""


def factorial_recursive(n: int) -> int:
    """Compute factorial recursively."""
    if n <= 1:
        return 1
    return n * factorial_recursive(n - 1)


def fibonacci_recursive(n: int) -> int:
    """Naive recursive fibonacci (small n only)."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)


def power_recursive(base_val: int, exp: int) -> int:
    """Compute base^exp recursively using fast exponentiation."""
    if exp == 0:
        return 1
    if exp == 1:
        return base_val
    if exp % 2 == 0:
        half: int = power_recursive(base_val, exp // 2)
        return half * half
    else:
        half2: int = power_recursive(base_val, exp // 2)
        return half2 * half2 * base_val


def sum_recursive(arr: list[int], idx: int) -> int:
    """Sum array elements recursively from index."""
    if idx >= len(arr):
        return 0
    return arr[idx] + sum_recursive(arr, idx + 1)


def max_recursive(arr: list[int], idx: int, current_max: int) -> int:
    """Find max element recursively."""
    if idx >= len(arr):
        return current_max
    new_max: int = current_max
    if arr[idx] > current_max:
        new_max = arr[idx]
    return max_recursive(arr, idx + 1, new_max)


def binary_search_recursive(arr: list[int], target: int, lo: int, hi: int) -> int:
    """Binary search recursively."""
    if lo > hi:
        return -1
    mid: int = (lo + hi) // 2
    if arr[mid] == target:
        return mid
    if arr[mid] < target:
        return binary_search_recursive(arr, target, mid + 1, hi)
    return binary_search_recursive(arr, target, lo, mid - 1)


def tower_of_hanoi_count(n: int) -> int:
    """Count moves needed for Tower of Hanoi with n disks."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    return 2 * tower_of_hanoi_count(n - 1) + 1


def gcd_recursive(a: int, b: int) -> int:
    """GCD using recursion."""
    if b == 0:
        return a
    return gcd_recursive(b, a % b)


def ackermann_bounded(m: int, n: int) -> int:
    """Ackermann function with bounds to prevent overflow."""
    if m > 3:
        return -1
    if n > 10:
        return -1
    if m == 0:
        return n + 1
    if n == 0:
        return ackermann_bounded(m - 1, 1)
    inner: int = ackermann_bounded(m, n - 1)
    if inner < 0:
        return -1
    return ackermann_bounded(m - 1, inner)


def test_module() -> int:
    """Test all recursion functions."""
    passed: int = 0
    if factorial_recursive(5) == 120:
        passed = passed + 1
    if factorial_recursive(0) == 1:
        passed = passed + 1
    if fibonacci_recursive(10) == 55:
        passed = passed + 1
    if fibonacci_recursive(0) == 0:
        passed = passed + 1
    if power_recursive(2, 10) == 1024:
        passed = passed + 1
    if power_recursive(3, 0) == 1:
        passed = passed + 1
    s: int = sum_recursive([1, 2, 3, 4, 5], 0)
    if s == 15:
        passed = passed + 1
    m: int = max_recursive([3, 1, 4, 1, 5], 0, 0)
    if m == 5:
        passed = passed + 1
    bs: int = binary_search_recursive([1, 3, 5, 7, 9], 5, 0, 4)
    if bs == 2:
        passed = passed + 1
    if tower_of_hanoi_count(3) == 7:
        passed = passed + 1
    if gcd_recursive(48, 18) == 6:
        passed = passed + 1
    ack: int = ackermann_bounded(2, 3)
    if ack == 9:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
