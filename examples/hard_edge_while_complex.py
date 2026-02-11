"""Complex while loop conditions and nested while patterns."""


def gcd_while(a: int, b: int) -> int:
    """GCD using while loop with compound condition."""
    x: int = a
    y: int = b
    if x < 0:
        x = 0 - x
    if y < 0:
        y = 0 - y
    while x != 0 and y != 0:
        if x > y:
            x = x % y
        else:
            y = y % x
    return x + y


def converge_to_one(n: int) -> int:
    """Count steps for Collatz sequence to reach 1."""
    if n <= 1:
        return 0
    val: int = n
    steps: int = 0
    while val != 1:
        if val % 2 == 0:
            val = val // 2
        else:
            val = 3 * val + 1
        steps = steps + 1
        if steps > 1000:
            break
    return steps


def newton_isqrt(n: int) -> int:
    """Integer square root using Newton's method."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def binary_search_while(arr: list[int], target: int) -> int:
    """Binary search with complex while condition."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi and lo < len(arr) and hi >= 0:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] == target:
            return mid
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def two_pointer_sum(arr: list[int], target: int) -> list[int]:
    """Two pointer technique with while loop."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo < hi:
        current: int = arr[lo] + arr[hi]
        if current == target:
            return [lo, hi]
        if current < target:
            lo = lo + 1
        else:
            hi = hi - 1
    return []


def remove_while_condition(arr: list[int], val: int) -> list[int]:
    """Remove all occurrences of val using while-based approach."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        while i < len(arr) and arr[i] == val:
            i = i + 1
        if i < len(arr):
            result.append(arr[i])
        i = i + 1
    return result


def count_while_nested(n: int) -> int:
    """Nested while loops with interdependent conditions."""
    total: int = 0
    i: int = 1
    while i <= n:
        j: int = i
        while j <= n:
            total = total + 1
            j = j + i
        i = i + 1
    return total


def simulate_queue(ops: list[int]) -> list[int]:
    """Simulate queue operations: positive=enqueue, 0=dequeue."""
    queue_data: list[int] = []
    dequeued: list[int] = []
    i: int = 0
    while i < len(ops):
        if ops[i] > 0:
            queue_data.append(ops[i])
        elif ops[i] == 0 and len(queue_data) > 0:
            front: int = queue_data[0]
            dequeued.append(front)
            new_queue: list[int] = []
            qi: int = 1
            while qi < len(queue_data):
                new_queue.append(queue_data[qi])
                qi = qi + 1
            queue_data = new_queue
        i = i + 1
    return dequeued


def test_module() -> int:
    """Test all complex while loop functions."""
    passed: int = 0
    if gcd_while(48, 18) == 6:
        passed = passed + 1
    if gcd_while(0, 5) == 5:
        passed = passed + 1
    if converge_to_one(6) == 8:
        passed = passed + 1
    if converge_to_one(1) == 0:
        passed = passed + 1
    if newton_isqrt(16) == 4:
        passed = passed + 1
    if newton_isqrt(15) == 3:
        passed = passed + 1
    bs: int = binary_search_while([1, 3, 5, 7, 9], 7)
    if bs == 3:
        passed = passed + 1
    tp: list[int] = two_pointer_sum([1, 2, 3, 4, 5], 7)
    if len(tp) == 2:
        passed = passed + 1
    rm: list[int] = remove_while_condition([1, 2, 3, 2, 4, 2], 2)
    if rm == [1, 3, 4]:
        passed = passed + 1
    cn: int = count_while_nested(6)
    if cn > 0:
        passed = passed + 1
    dq: list[int] = simulate_queue([1, 2, 0, 3, 0])
    if dq == [1, 2]:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
