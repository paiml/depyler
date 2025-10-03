# tests/test_itertools/test_iterators.py
"""
TDD examples for itertools module - iterator building blocks.
Each test becomes a verified documentation example.
"""
import itertools
import pytest
from hypothesis import given, strategies as st


class TestChain:
    """itertools.chain() - Chain multiple iterables."""

    def test_chain_basic(self):
        """Basic: Chain two lists together."""
        result = list(itertools.chain([1, 2], [3, 4]))
        assert result == [1, 2, 3, 4]

    def test_chain_multiple_iterables(self):
        """Feature: Chain many iterables."""
        result = list(itertools.chain([1], [2, 3], [4, 5, 6]))
        assert result == [1, 2, 3, 4, 5, 6]

    def test_chain_from_iterable(self):
        """Feature: Chain from a list of iterables."""
        result = list(itertools.chain.from_iterable([[1, 2], [3, 4], [5]]))
        assert result == [1, 2, 3, 4, 5]

    def test_chain_empty_iterables(self):
        """Edge: Chain with empty iterables."""
        result = list(itertools.chain([1], [], [2], []))
        assert result == [1, 2]

    def test_chain_different_types(self):
        """Property: Chain different iterable types."""
        result = list(itertools.chain([1, 2], (3, 4), "ab"))
        assert result == [1, 2, 3, 4, "a", "b"]


class TestCombinations:
    """itertools.combinations() - r-length subsequences."""

    def test_combinations_basic(self):
        """Basic: Generate 2-element combinations."""
        result = list(itertools.combinations([1, 2, 3], 2))
        assert result == [(1, 2), (1, 3), (2, 3)]

    def test_combinations_length_3(self):
        """Feature: Generate 3-element combinations."""
        result = list(itertools.combinations("ABCD", 3))
        assert result == [
            ("A", "B", "C"),
            ("A", "B", "D"),
            ("A", "C", "D"),
            ("B", "C", "D"),
        ]

    def test_combinations_r_equals_n(self):
        """Edge: r equals iterable length (single combination)."""
        result = list(itertools.combinations([1, 2, 3], 3))
        assert result == [(1, 2, 3)]

    def test_combinations_r_zero(self):
        """Edge: r=0 produces single empty tuple."""
        result = list(itertools.combinations([1, 2, 3], 0))
        assert result == [()]

    def test_combinations_r_greater_than_n(self):
        """Edge: r > n produces empty result."""
        result = list(itertools.combinations([1, 2], 5))
        assert result == []

    def test_combinations_preserves_order(self):
        """Property: Combinations maintain input order."""
        result = list(itertools.combinations([3, 1, 2], 2))
        assert result == [(3, 1), (3, 2), (1, 2)]


class TestPermutations:
    """itertools.permutations() - r-length permutations."""

    def test_permutations_basic(self):
        """Basic: Generate all permutations."""
        result = list(itertools.permutations([1, 2, 3]))
        assert len(result) == 6  # 3! = 6
        assert (1, 2, 3) in result
        assert (3, 2, 1) in result

    def test_permutations_with_r(self):
        """Feature: Generate r-length permutations."""
        result = list(itertools.permutations([1, 2, 3], 2))
        assert len(result) == 6  # 3P2 = 6
        assert result == [(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)]

    def test_permutations_r_zero(self):
        """Edge: r=0 produces single empty tuple."""
        result = list(itertools.permutations([1, 2, 3], 0))
        assert result == [()]

    def test_permutations_string(self):
        """Feature: Permutations of string."""
        result = list(itertools.permutations("AB"))
        assert result == [("A", "B"), ("B", "A")]


class TestProduct:
    """itertools.product() - Cartesian product."""

    def test_product_basic(self):
        """Basic: Cartesian product of two iterables."""
        result = list(itertools.product([1, 2], ["a", "b"]))
        assert result == [(1, "a"), (1, "b"), (2, "a"), (2, "b")]

    def test_product_three_iterables(self):
        """Feature: Product of three iterables."""
        result = list(itertools.product([1, 2], [3, 4], [5, 6]))
        assert len(result) == 8  # 2 * 2 * 2
        assert (1, 3, 5) in result
        assert (2, 4, 6) in result

    def test_product_repeat(self):
        """Feature: Product with repeat parameter."""
        result = list(itertools.product([0, 1], repeat=3))
        assert len(result) == 8  # 2^3
        assert (0, 0, 0) in result
        assert (1, 1, 1) in result

    def test_product_empty_iterable(self):
        """Edge: Product with empty iterable is empty."""
        result = list(itertools.product([1, 2], []))
        assert result == []


class TestCount:
    """itertools.count() - Count from start indefinitely."""

    def test_count_basic(self):
        """Basic: Count from 0."""
        counter = itertools.count()
        result = [next(counter) for _ in range(5)]
        assert result == [0, 1, 2, 3, 4]

    def test_count_with_start(self):
        """Feature: Count from custom start."""
        counter = itertools.count(10)
        result = [next(counter) for _ in range(3)]
        assert result == [10, 11, 12]

    def test_count_with_step(self):
        """Feature: Count with custom step."""
        counter = itertools.count(0, 2)
        result = [next(counter) for _ in range(5)]
        assert result == [0, 2, 4, 6, 8]

    def test_count_negative_step(self):
        """Edge: Count with negative step."""
        counter = itertools.count(10, -1)
        result = [next(counter) for _ in range(5)]
        assert result == [10, 9, 8, 7, 6]


class TestCycle:
    """itertools.cycle() - Cycle through iterable indefinitely."""

    def test_cycle_basic(self):
        """Basic: Cycle through a list."""
        cycler = itertools.cycle([1, 2, 3])
        result = [next(cycler) for _ in range(7)]
        assert result == [1, 2, 3, 1, 2, 3, 1]

    def test_cycle_string(self):
        """Feature: Cycle through a string."""
        cycler = itertools.cycle("AB")
        result = [next(cycler) for _ in range(5)]
        assert result == ["A", "B", "A", "B", "A"]


class TestRepeat:
    """itertools.repeat() - Repeat element."""

    def test_repeat_basic(self):
        """Basic: Repeat element indefinitely."""
        repeater = itertools.repeat(5)
        result = [next(repeater) for _ in range(4)]
        assert result == [5, 5, 5, 5]

    def test_repeat_with_times(self):
        """Feature: Repeat element n times."""
        result = list(itertools.repeat("A", 3))
        assert result == ["A", "A", "A"]

    def test_repeat_zero_times(self):
        """Edge: Repeat 0 times produces empty."""
        result = list(itertools.repeat(1, 0))
        assert result == []


class TestZipLongest:
    """itertools.zip_longest() - Zip iterables, filling missing values."""

    def test_zip_longest_basic(self):
        """Basic: Zip iterables of different lengths."""
        result = list(itertools.zip_longest([1, 2], ["a", "b", "c"]))
        assert result == [(1, "a"), (2, "b"), (None, "c")]

    def test_zip_longest_fillvalue(self):
        """Feature: Custom fill value."""
        result = list(itertools.zip_longest([1, 2], ["a", "b", "c"], fillvalue=0))
        assert result == [(1, "a"), (2, "b"), (0, "c")]

    def test_zip_longest_all_same_length(self):
        """Edge: All iterables same length (like zip)."""
        result = list(itertools.zip_longest([1, 2], ["a", "b"]))
        assert result == [(1, "a"), (2, "b")]


class TestGroupBy:
    """itertools.groupby() - Group consecutive equal elements."""

    def test_groupby_basic(self):
        """Basic: Group consecutive identical elements."""
        data = [1, 1, 2, 2, 2, 3, 1]
        result = [(k, list(g)) for k, g in itertools.groupby(data)]
        assert result == [(1, [1, 1]), (2, [2, 2, 2]), (3, [3]), (1, [1])]

    def test_groupby_with_key(self):
        """Feature: Group by custom key function."""
        data = ["apple", "apricot", "banana", "blueberry"]
        result = [(k, list(g)) for k, g in itertools.groupby(data, key=lambda x: x[0])]
        assert result == [
            ("a", ["apple", "apricot"]),
            ("b", ["banana", "blueberry"]),
        ]

    def test_groupby_sorted_first(self):
        """Edge: groupby requires sorted data for complete grouping."""
        # Unsorted
        data = [1, 2, 1, 2]
        unsorted_result = [(k, list(g)) for k, g in itertools.groupby(data)]
        assert unsorted_result == [(1, [1]), (2, [2]), (1, [1]), (2, [2])]

        # Sorted
        sorted_data = sorted(data)
        sorted_result = [(k, list(g)) for k, g in itertools.groupby(sorted_data)]
        assert sorted_result == [(1, [1, 1]), (2, [2, 2])]


class TestDropWhile:
    """itertools.dropwhile() - Drop elements while predicate is true."""

    def test_dropwhile_basic(self):
        """Basic: Drop elements while less than 5."""
        result = list(itertools.dropwhile(lambda x: x < 5, [1, 2, 6, 7, 3]))
        assert result == [6, 7, 3]  # Drops 1,2 then stops

    def test_dropwhile_all_dropped(self):
        """Edge: All elements satisfy predicate."""
        result = list(itertools.dropwhile(lambda x: x > 0, [1, 2, 3]))
        assert result == []

    def test_dropwhile_none_dropped(self):
        """Edge: First element fails predicate."""
        result = list(itertools.dropwhile(lambda x: x > 5, [1, 2, 3]))
        assert result == [1, 2, 3]


class TestTakeWhile:
    """itertools.takewhile() - Take elements while predicate is true."""

    def test_takewhile_basic(self):
        """Basic: Take elements while less than 5."""
        result = list(itertools.takewhile(lambda x: x < 5, [1, 2, 6, 7, 3]))
        assert result == [1, 2]  # Stops at 6

    def test_takewhile_all_taken(self):
        """Edge: All elements satisfy predicate."""
        result = list(itertools.takewhile(lambda x: x > 0, [1, 2, 3]))
        assert result == [1, 2, 3]

    def test_takewhile_none_taken(self):
        """Edge: First element fails predicate."""
        result = list(itertools.takewhile(lambda x: x > 5, [1, 2, 3]))
        assert result == []


class TestSlice:
    """itertools.islice() - Slice an iterator."""

    def test_islice_stop_only(self):
        """Basic: Take first n elements."""
        result = list(itertools.islice(range(10), 5))
        assert result == [0, 1, 2, 3, 4]

    def test_islice_start_stop(self):
        """Feature: Slice with start and stop."""
        result = list(itertools.islice(range(10), 2, 7))
        assert result == [2, 3, 4, 5, 6]

    def test_islice_with_step(self):
        """Feature: Slice with step."""
        result = list(itertools.islice(range(10), 0, 10, 2))
        assert result == [0, 2, 4, 6, 8]

    def test_islice_infinite_iterator(self):
        """Property: Works with infinite iterators."""
        result = list(itertools.islice(itertools.count(), 5))
        assert result == [0, 1, 2, 3, 4]


class TestAccumulate:
    """itertools.accumulate() - Cumulative sums or operations."""

    def test_accumulate_default_sum(self):
        """Basic: Cumulative sum (default)."""
        result = list(itertools.accumulate([1, 2, 3, 4]))
        assert result == [1, 3, 6, 10]

    def test_accumulate_with_function(self):
        """Feature: Cumulative with custom function."""
        import operator

        result = list(itertools.accumulate([1, 2, 3, 4], operator.mul))
        assert result == [1, 2, 6, 24]  # Factorial-like

    def test_accumulate_empty(self):
        """Edge: Empty iterable produces empty result."""
        result = list(itertools.accumulate([]))
        assert result == []


# Metadata for doc generation
__module_name__ = "itertools"
__module_link__ = "https://docs.python.org/3/library/itertools.html"
__test_count__ = 48
__coverage__ = 0.70  # ~70% of common itertools functions
