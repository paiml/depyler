# Pathological class pattern: Multi-method accumulator with branching logic
# Tests: class with __init__, 5+ methods, list[int] state, conditional accumulation


class Accumulator:
    def __init__(self) -> None:
        self.values: list[int] = []
        self.total: int = 0
        self.min_val: int = 999999
        self.max_val: int = 0 - 999999
        self.count: int = 0

    def add_value(self, val: int) -> None:
        self.values.append(val)
        self.total = self.total + val
        self.count = self.count + 1
        if val < self.min_val:
            self.min_val = val
        if val > self.max_val:
            self.max_val = val

    def get_average(self) -> int:
        if self.count == 0:
            return 0
        return self.total // self.count

    def get_range(self) -> int:
        if self.count == 0:
            return 0
        return self.max_val - self.min_val

    def get_sum_above(self, threshold: int) -> int:
        result: int = 0
        i: int = 0
        while i < len(self.values):
            if self.values[i] > threshold:
                result = result + self.values[i]
            i = i + 1
        return result

    def get_count_below(self, threshold: int) -> int:
        result: int = 0
        i: int = 0
        while i < len(self.values):
            if self.values[i] < threshold:
                result = result + 1
            i = i + 1
        return result

    def reset(self) -> None:
        self.values = []
        self.total = 0
        self.min_val = 999999
        self.max_val = 0 - 999999
        self.count = 0


def test_module() -> int:
    passed: int = 0
    acc: Accumulator = Accumulator()
    acc.add_value(10)
    acc.add_value(20)
    acc.add_value(30)
    acc.add_value(5)
    acc.add_value(50)
    # Test 1: total
    if acc.total == 115:
        passed = passed + 1
    # Test 2: count
    if acc.count == 5:
        passed = passed + 1
    # Test 3: average
    if acc.get_average() == 23:
        passed = passed + 1
    # Test 4: range
    if acc.get_range() == 45:
        passed = passed + 1
    # Test 5: sum above threshold
    if acc.get_sum_above(15) == 100:
        passed = passed + 1
    # Test 6: count below threshold
    if acc.get_count_below(25) == 3:
        passed = passed + 1
    # Test 7: reset works
    acc.reset()
    if acc.count == 0:
        passed = passed + 1
    return passed
