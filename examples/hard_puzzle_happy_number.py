def digit_square_sum(n: int) -> int:
    total: int = 0
    num: int = n
    while num > 0:
        d: int = num % 10
        total = total + d * d
        num = num // 10
    return total

def is_happy(n: int) -> int:
    slow: int = n
    fast: int = digit_square_sum(n)
    while fast != 1 and slow != fast:
        slow = digit_square_sum(slow)
        fast = digit_square_sum(digit_square_sum(fast))
    if fast == 1:
        return 1
    return 0

def happy_steps(n: int) -> int:
    steps: int = 0
    current: int = n
    limit: int = 200
    while current != 1 and steps < limit:
        current = digit_square_sum(current)
        steps = steps + 1
    if current == 1:
        return steps
    return 0 - 1

def is_ugly(n: int) -> int:
    if n <= 0:
        return 0
    num: int = n
    while num % 2 == 0:
        num = num // 2
    while num % 3 == 0:
        num = num // 3
    while num % 5 == 0:
        num = num // 5
    if num == 1:
        return 1
    return 0

def nth_ugly(n: int) -> int:
    ugly: list[int] = [1]
    i2: int = 0
    i3: int = 0
    i5: int = 0
    while len(ugly) < n:
        v2: int = ugly[i2] * 2
        v3: int = ugly[i3] * 3
        v5: int = ugly[i5] * 5
        next_val: int = v2
        if v3 < next_val:
            next_val = v3
        if v5 < next_val:
            next_val = v5
        ugly.append(next_val)
        if next_val == v2:
            i2 = i2 + 1
        if next_val == v3:
            i3 = i3 + 1
        if next_val == v5:
            i5 = i5 + 1
    last: int = len(ugly) - 1
    return ugly[last]

def test_module() -> int:
    passed: int = 0
    r1: int = is_happy(19)
    if r1 == 1:
        passed = passed + 1
    r2: int = is_happy(2)
    if r2 == 0:
        passed = passed + 1
    r3: int = happy_steps(7)
    if r3 > 0:
        passed = passed + 1
    r4: int = is_ugly(6)
    if r4 == 1:
        passed = passed + 1
    r5: int = nth_ugly(10)
    if r5 == 12:
        passed = passed + 1
    r6: int = is_ugly(14)
    if r6 == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
