"""Pathological iterator protocol patterns for transpiler stress testing.

Tests __iter__/__next__, iterator chaining, infinite iterators,
tuple-producing iterators, and custom range/fibonacci iterators.
"""

from typing import Iterator, List, Tuple, Optional, TypeVar, Generic, Callable

T = TypeVar("T")


class FibonacciIterator:
    """Iterator that yields Fibonacci numbers up to a maximum value."""

    def __init__(self, max_value: int):
        self.max_value = max_value
        self._a = 0
        self._b = 1

    def __iter__(self):
        self._a = 0
        self._b = 1
        return self

    def __next__(self) -> int:
        if self._a > self.max_value:
            raise StopIteration
        current = self._a
        self._a, self._b = self._b, self._a + self._b
        return current


class RangeIterator:
    """Custom range-like iterator with start, stop, step."""

    def __init__(self, start: int, stop: int, step: int = 1):
        if step == 0:
            raise ValueError("step must not be zero")
        self.start = start
        self.stop = stop
        self.step = step
        self._current = start

    def __iter__(self):
        self._current = self.start
        return self

    def __next__(self) -> int:
        if self.step > 0 and self._current >= self.stop:
            raise StopIteration
        if self.step < 0 and self._current <= self.stop:
            raise StopIteration
        value = self._current
        self._current += self.step
        return value

    def __len__(self) -> int:
        if self.step > 0:
            return max(0, (self.stop - self.start + self.step - 1) // self.step)
        else:
            return max(0, (self.start - self.stop - self.step - 1) // (-self.step))


class InfiniteCounter:
    """Infinite iterator that counts from a start value."""

    def __init__(self, start: int = 0, step: int = 1):
        self.start = start
        self.step = step
        self._current = start

    def __iter__(self):
        self._current = self.start
        return self

    def __next__(self) -> int:
        value = self._current
        self._current += self.step
        return value


class PairIterator:
    """Iterator that yields consecutive pairs as tuples."""

    def __init__(self, data: List[int]):
        self._data = data
        self._index = 0

    def __iter__(self):
        self._index = 0
        return self

    def __next__(self) -> Tuple[int, int]:
        if self._index + 1 >= len(self._data):
            raise StopIteration
        pair = (self._data[self._index], self._data[self._index + 1])
        self._index += 1
        return pair


class ChainedIterator:
    """Chains multiple iterables into a single iterator."""

    def __init__(self, *iterables):
        self._iterables = list(iterables)
        self._current_index = 0
        self._current_iter = None

    def __iter__(self):
        self._current_index = 0
        if self._iterables:
            self._current_iter = iter(self._iterables[0])
        else:
            self._current_iter = None
        return self

    def __next__(self):
        while self._current_iter is not None:
            try:
                return next(self._current_iter)
            except StopIteration:
                self._current_index += 1
                if self._current_index < len(self._iterables):
                    self._current_iter = iter(self._iterables[self._current_index])
                else:
                    self._current_iter = None
        raise StopIteration


class EnumerateIterator:
    """Custom enumerate implementation."""

    def __init__(self, iterable, start: int = 0):
        self._iter = iter(iterable)
        self._index = start

    def __iter__(self):
        return self

    def __next__(self) -> Tuple[int, object]:
        value = next(self._iter)  # Will raise StopIteration naturally
        idx = self._index
        self._index += 1
        return (idx, value)


class FilterIterator:
    """Iterator that filters elements based on a predicate."""

    def __init__(self, predicate, iterable):
        self._predicate = predicate
        self._iter = iter(iterable)

    def __iter__(self):
        return self

    def __next__(self):
        while True:
            value = next(self._iter)  # Propagates StopIteration
            if self._predicate(value):
                return value


class MapIterator:
    """Iterator that applies a function to each element."""

    def __init__(self, func, iterable):
        self._func = func
        self._iter = iter(iterable)

    def __iter__(self):
        return self

    def __next__(self):
        value = next(self._iter)
        return self._func(value)


class ZipIterator:
    """Custom zip implementation for two iterables."""

    def __init__(self, iter1, iter2):
        self._iter1 = iter(iter1)
        self._iter2 = iter(iter2)

    def __iter__(self):
        return self

    def __next__(self) -> Tuple:
        v1 = next(self._iter1)
        v2 = next(self._iter2)
        return (v1, v2)


# --- Untyped helper functions ---

def take(iterator, n):
    """Take the first n elements from an iterator - untyped."""
    results = []
    count = 0
    for item in iterator:
        if count >= n:
            break
        results.append(item)
        count += 1
    return results


def collect_to_list(iterator):
    """Collect all elements from a finite iterator into a list - untyped."""
    result = []
    for item in iterator:
        result.append(item)
    return result


def reduce_iterator(func, iterator, initial):
    """Reduce an iterator using func and initial value - untyped."""
    accumulator = initial
    for item in iterator:
        accumulator = func(accumulator, item)
    return accumulator


def window_pairs(data):
    """Yield sliding window pairs from data - untyped."""
    result = []
    for i in range(len(data) - 1):
        result.append((data[i], data[i + 1]))
    return result


def flatten(nested_lists):
    """Flatten a list of lists into a single list - untyped."""
    result = []
    for sublist in nested_lists:
        for item in sublist:
            result.append(item)
    return result


def unique(iterable):
    """Return unique elements preserving order - untyped."""
    seen = set()
    result = []
    for item in iterable:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result


def accumulate_iter(iterable, func=None):
    """Running accumulation, like itertools.accumulate - untyped."""
    result = []
    it = iter(iterable)
    try:
        total = next(it)
    except StopIteration:
        return result
    result.append(total)
    for element in it:
        if func is not None:
            total = func(total, element)
        else:
            total = total + element
        result.append(total)
    return result


# --- Generator functions ---

def count_up_generator(start: int, stop: int) -> Iterator[int]:
    """Generator that counts from start to stop (exclusive)."""
    current = start
    while current < stop:
        yield current
        current += 1


def fibonacci_generator(max_count: int) -> Iterator[int]:
    """Generator that yields fibonacci numbers."""
    a, b = 0, 1
    count = 0
    while count < max_count:
        yield a
        a, b = b, a + b
        count += 1


def repeat_generator(value: int, times: int) -> Iterator[int]:
    """Generator that yields value repeated times."""
    for _ in range(times):
        yield value


# --- Typed test functions ---

def test_fibonacci_iterator() -> bool:
    """Test FibonacciIterator."""
    fibs = collect_to_list(FibonacciIterator(20))
    assert fibs == [0, 1, 1, 2, 3, 5, 8, 13]

    fibs_small = collect_to_list(FibonacciIterator(1))
    assert fibs_small == [0, 1, 1]

    fibs_zero = collect_to_list(FibonacciIterator(0))
    assert fibs_zero == [0]
    return True


def test_range_iterator() -> bool:
    """Test RangeIterator with various step sizes."""
    forward = collect_to_list(RangeIterator(0, 10, 2))
    assert forward == [0, 2, 4, 6, 8]

    backward = collect_to_list(RangeIterator(10, 0, -3))
    assert backward == [10, 7, 4, 1]

    single = collect_to_list(RangeIterator(5, 6))
    assert single == [5]

    empty = collect_to_list(RangeIterator(5, 5))
    assert empty == []

    assert len(RangeIterator(0, 10, 2)) == 5
    assert len(RangeIterator(10, 0, -3)) == 4
    return True


def test_infinite_counter() -> bool:
    """Test InfiniteCounter with take."""
    first_five = take(InfiniteCounter(0), 5)
    assert first_five == [0, 1, 2, 3, 4]

    odds = take(InfiniteCounter(1, 2), 5)
    assert odds == [1, 3, 5, 7, 9]

    negatives = take(InfiniteCounter(0, -1), 4)
    assert negatives == [0, -1, -2, -3]
    return True


def test_pair_iterator() -> bool:
    """Test PairIterator."""
    pairs = collect_to_list(PairIterator([1, 2, 3, 4, 5]))
    assert pairs == [(1, 2), (2, 3), (3, 4), (4, 5)]

    single = collect_to_list(PairIterator([1]))
    assert single == []

    two = collect_to_list(PairIterator([10, 20]))
    assert two == [(10, 20)]
    return True


def test_chained_iterator() -> bool:
    """Test ChainedIterator."""
    chained = collect_to_list(ChainedIterator([1, 2], [3, 4], [5]))
    assert chained == [1, 2, 3, 4, 5]

    empty_chain = collect_to_list(ChainedIterator([], [1], []))
    assert empty_chain == [1]

    all_empty = collect_to_list(ChainedIterator([], [], []))
    assert all_empty == []
    return True


def test_filter_map_zip() -> bool:
    """Test FilterIterator, MapIterator, and ZipIterator."""
    evens = collect_to_list(FilterIterator(lambda x: x % 2 == 0, [1, 2, 3, 4, 5, 6]))
    assert evens == [2, 4, 6]

    doubled = collect_to_list(MapIterator(lambda x: x * 2, [1, 2, 3]))
    assert doubled == [2, 4, 6]

    zipped = collect_to_list(ZipIterator([1, 2, 3], [10, 20, 30]))
    assert zipped == [(1, 10), (2, 20), (3, 30)]

    # Zip with different lengths (stops at shorter)
    zipped_short = collect_to_list(ZipIterator([1, 2], [10, 20, 30]))
    assert zipped_short == [(1, 10), (2, 20)]
    return True


def test_enumerate_iterator() -> bool:
    """Test EnumerateIterator."""
    enumerated = collect_to_list(EnumerateIterator(["a", "b", "c"]))
    assert enumerated == [(0, "a"), (1, "b"), (2, "c")]

    from_five = collect_to_list(EnumerateIterator(["x", "y"], start=5))
    assert from_five == [(5, "x"), (6, "y")]
    return True


def test_generators() -> bool:
    """Test generator functions."""
    counted = list(count_up_generator(3, 8))
    assert counted == [3, 4, 5, 6, 7]

    fibs = list(fibonacci_generator(7))
    assert fibs == [0, 1, 1, 2, 3, 5, 8]

    repeated = list(repeat_generator(42, 3))
    assert repeated == [42, 42, 42]
    return True


def test_helper_functions() -> bool:
    """Test untyped helper functions."""
    pairs = window_pairs([10, 20, 30, 40])
    assert pairs == [(10, 20), (20, 30), (30, 40)]

    flat = flatten([[1, 2], [3], [4, 5, 6]])
    assert flat == [1, 2, 3, 4, 5, 6]

    uniq = unique([1, 2, 2, 3, 1, 4, 3])
    assert uniq == [1, 2, 3, 4]

    acc = accumulate_iter([1, 2, 3, 4, 5])
    assert acc == [1, 3, 6, 10, 15]

    acc_mul = accumulate_iter([1, 2, 3, 4], lambda a, b: a * b)
    assert acc_mul == [1, 2, 6, 24]

    total = reduce_iterator(lambda a, b: a + b, iter([1, 2, 3, 4]), 0)
    assert total == 10
    return True


def test_composed_iterators() -> bool:
    """Test composing multiple iterator types."""
    # Filter evens from range, then double them
    ranged = RangeIterator(1, 11)
    evens = FilterIterator(lambda x: x % 2 == 0, ranged)
    doubled = MapIterator(lambda x: x * 2, evens)
    result = collect_to_list(doubled)
    assert result == [4, 8, 12, 16, 20]

    # Chain fibonacci with range, take first 10
    chained = ChainedIterator(
        FibonacciIterator(10),
        RangeIterator(100, 105)
    )
    result2 = collect_to_list(chained)
    assert result2 == [0, 1, 1, 2, 3, 5, 8, 100, 101, 102, 103, 104]
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_fibonacci_iterator()
    assert test_range_iterator()
    assert test_infinite_counter()
    assert test_pair_iterator()
    assert test_chained_iterator()
    assert test_filter_map_zip()
    assert test_enumerate_iterator()
    assert test_generators()
    assert test_helper_functions()
    assert test_composed_iterators()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_iterator_protocol: ALL TESTS PASSED")
    else:
        print("hard_iterator_protocol: TESTS FAILED")


if __name__ == "__main__":
    main()
