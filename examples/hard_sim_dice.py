def dice_expected_value(sides: int) -> float:
    total: float = 0.0
    i: int = 1
    while i <= sides:
        total = total + i * 1.0
        i = i + 1
    return total / (sides * 1.0)

def two_dice_sum_prob(target: int) -> float:
    count: int = 0
    i: int = 1
    while i <= 6:
        j: int = 1
        while j <= 6:
            s: int = i + j
            if s == target:
                count = count + 1
            j = j + 1
        i = i + 1
    return count * 1.0 / 36.0

def dice_variance(sides: int) -> float:
    ev: float = dice_expected_value(sides)
    total: float = 0.0
    i: int = 1
    while i <= sides:
        diff: float = (i * 1.0) - ev
        total = total + diff * diff
        i = i + 1
    return total / (sides * 1.0)

def dice_at_least_one_six(num_rolls: int) -> float:
    prob_no_six: float = 1.0
    i: int = 0
    while i < num_rolls:
        prob_no_six = prob_no_six * (5.0 / 6.0)
        i = i + 1
    return 1.0 - prob_no_six

def sum_distribution(sides: int, num_dice: int) -> list[int]:
    max_sum: int = sides * num_dice
    counts: list[int] = []
    i: int = 0
    while i <= max_sum:
        counts.append(0)
        i = i + 1
    if num_dice == 2:
        a: int = 1
        while a <= sides:
            b: int = 1
            while b <= sides:
                s: int = a + b
                old: int = counts[s]
                counts[s] = old + 1
                b = b + 1
            a = a + 1
    return counts

def test_module() -> int:
    passed: int = 0
    ev: float = dice_expected_value(6)
    if ev == 3.5:
        passed = passed + 1
    p7: float = two_dice_sum_prob(7)
    diff: float = p7 - 0.16666666
    if diff < 0.001 and diff > (0.0 - 0.001):
        passed = passed + 1
    p2: float = two_dice_sum_prob(2)
    diff2: float = p2 - 0.02777777
    if diff2 < 0.001 and diff2 > (0.0 - 0.001):
        passed = passed + 1
    v: float = dice_variance(6)
    diff3: float = v - 2.9166666
    if diff3 < 0.01 and diff3 > (0.0 - 0.01):
        passed = passed + 1
    p: float = dice_at_least_one_six(1)
    diff4: float = p - 0.16666666
    if diff4 < 0.001 and diff4 > (0.0 - 0.001):
        passed = passed + 1
    return passed
