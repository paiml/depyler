def trap_rain_water(heights: list[int]) -> int:
    n: int = len(heights)
    if n < 3:
        return 0
    left_max: list[int] = []
    right_max: list[int] = []
    i: int = 0
    while i < n:
        left_max.append(0)
        right_max.append(0)
        i = i + 1
    left_max[0] = heights[0]
    j: int = 1
    while j < n:
        prev: int = left_max[j - 1]
        curr: int = heights[j]
        if prev > curr:
            left_max[j] = prev
        else:
            left_max[j] = curr
        j = j + 1
    last: int = n - 1
    right_max[last] = heights[last]
    k: int = n - 2
    while k >= 0:
        nxt: int = right_max[k + 1]
        curr2: int = heights[k]
        if nxt > curr2:
            right_max[k] = nxt
        else:
            right_max[k] = curr2
        k = k - 1
    water: int = 0
    m: int = 0
    while m < n:
        lm: int = left_max[m]
        rm: int = right_max[m]
        bound: int = lm
        if rm < lm:
            bound = rm
        diff: int = bound - heights[m]
        if diff > 0:
            water = water + diff
        m = m + 1
    return water

def container_most_water(heights: list[int]) -> int:
    n: int = len(heights)
    if n < 2:
        return 0
    left: int = 0
    right: int = n - 1
    best: int = 0
    while left < right:
        hl: int = heights[left]
        hr: int = heights[right]
        h: int = hl
        if hr < hl:
            h = hr
        area: int = h * (right - left)
        if area > best:
            best = area
        if hl < hr:
            left = left + 1
        else:
            right = right - 1
    return best

def test_module() -> int:
    passed: int = 0
    r1: int = trap_rain_water([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    if r1 == 6:
        passed = passed + 1
    r2: int = trap_rain_water([4, 2, 0, 3, 2, 5])
    if r2 == 9:
        passed = passed + 1
    r3: int = container_most_water([1, 8, 6, 2, 5, 4, 8, 3, 7])
    if r3 == 49:
        passed = passed + 1
    r4: int = trap_rain_water([1, 2, 3])
    if r4 == 0:
        passed = passed + 1
    r5: int = container_most_water([1, 1])
    if r5 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
