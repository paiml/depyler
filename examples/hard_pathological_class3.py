# Pathological class pattern: Parallel array counter registry
# Tests: class with multiple list[int] + list[str] parallel arrays, lookups
# Workaround: avoid dict[str, int] in class (transpiler generates ambiguous
# .into() on dict read in class methods). Use parallel arrays instead.


class CounterRegistry:
    def __init__(self) -> None:
        self.names: list[str] = []
        self.values: list[int] = []
        self.total_ops: int = 0

    def find_index(self, name: str) -> int:
        i: int = 0
        while i < len(self.names):
            if self.names[i] == name:
                return i
            i = i + 1
        return 0 - 1

    def create_counter(self, name: str) -> None:
        self.names.append(name)
        self.values.append(0)
        self.total_ops = self.total_ops + 1

    def increment(self, name: str, amount: int) -> int:
        idx: int = self.find_index(name)
        if idx >= 0:
            old_val: int = self.values[idx]
            new_val: int = old_val + amount
            # Rebuild values list with updated index
            new_vals: list[int] = []
            j: int = 0
            while j < len(self.values):
                if j == idx:
                    new_vals.append(new_val)
                else:
                    new_vals.append(self.values[j])
                j = j + 1
            self.values = new_vals
            self.total_ops = self.total_ops + 1
            return new_val
        return 0 - 1

    def decrement(self, name: str, amount: int) -> int:
        idx: int = self.find_index(name)
        if idx >= 0:
            old_val: int = self.values[idx]
            new_val: int = old_val - amount
            new_vals: list[int] = []
            j: int = 0
            while j < len(self.values):
                if j == idx:
                    new_vals.append(new_val)
                else:
                    new_vals.append(self.values[j])
                j = j + 1
            self.values = new_vals
            self.total_ops = self.total_ops + 1
            return new_val
        return 0 - 1

    def get_value(self, name: str) -> int:
        idx: int = self.find_index(name)
        if idx >= 0:
            return self.values[idx]
        return 0 - 1

    def get_total_ops(self) -> int:
        return self.total_ops

    def sum_all(self) -> int:
        total: int = 0
        i: int = 0
        while i < len(self.values):
            total = total + self.values[i]
            i = i + 1
        return total

    def counter_count(self) -> int:
        return len(self.names)


def test_module() -> int:
    passed: int = 0
    reg: CounterRegistry = CounterRegistry()
    reg.create_counter("alpha")
    reg.create_counter("beta")
    reg.create_counter("gamma")
    # Test 1: initial value is 0
    if reg.get_value("alpha") == 0:
        passed = passed + 1
    # Test 2: increment
    reg.increment("alpha", 5)
    if reg.get_value("alpha") == 5:
        passed = passed + 1
    # Test 3: decrement
    reg.decrement("alpha", 2)
    if reg.get_value("alpha") == 3:
        passed = passed + 1
    # Test 4: multiple counters
    reg.increment("beta", 10)
    reg.increment("gamma", 7)
    if reg.get_value("beta") == 10:
        passed = passed + 1
    # Test 5: sum all (3+10+7 = 20)
    if reg.sum_all() == 20:
        passed = passed + 1
    # Test 6: counter count
    if reg.counter_count() == 3:
        passed = passed + 1
    # Test 7: non-existent counter
    if reg.get_value("missing") == 0 - 1:
        passed = passed + 1
    return passed
