"""
Comprehensive test of Python itertools module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's itertools module
functions to their Rust equivalents (using iterator adapters).

Expected Rust mappings:
- itertools.chain() -> Iterator::chain()
- zip() -> Iterator::zip()
- enumerate() -> Iterator::enumerate()
- filter() -> Iterator::filter()
- map() -> Iterator::map()
- itertools.count() -> std::ops::Range or RangeFrom
- itertools.cycle() -> Iterator::cycle()
- itertools.repeat() -> std::iter::repeat()

Note: Some advanced itertools functions (product, combinations, permutations)
may not be fully supported yet.
"""

import itertools
from typing import List, Tuple


def test_chain_iterables() -> List[int]:
    """Test chaining multiple iterables together"""
    list1: List[int] = [1, 2, 3]
    list2: List[int] = [4, 5, 6]
    list3: List[int] = [7, 8, 9]

    # Chain iterables together
    chained: List[int] = list(itertools.chain(list1, list2, list3))

    return chained


def test_zip_iterables() -> List[Tuple[int, str]]:
    """Test zipping iterables together"""
    numbers: List[int] = [1, 2, 3, 4, 5]
    letters: List[str] = ["a", "b", "c", "d", "e"]

    # Zip them together
    zipped: List[Tuple[int, str]] = list(zip(numbers, letters))

    return zipped


def test_enumerate() -> List[Tuple[int, str]]:
    """Test enumerate for indexed iteration"""
    items: List[str] = ["apple", "banana", "cherry"]

    # Enumerate items
    enumerated: List[Tuple[int, str]] = list(enumerate(items))

    return enumerated


def test_filter() -> List[int]:
    """Test filtering iterables"""
    numbers: List[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # Filter even numbers (using manual filtering for type safety)
    evens: List[int] = []
    for num in numbers:
        if num % 2 == 0:
            evens.append(num)

    return evens


def test_map() -> List[int]:
    """Test mapping function over iterable"""
    numbers: List[int] = [1, 2, 3, 4, 5]

    # Map square function (manual implementation)
    squared: List[int] = []
    for num in numbers:
        squared.append(num * num)

    return squared


def test_count(start: int, step: int, limit: int) -> List[int]:
    """Test count() infinite iterator with limit"""
    # Simulate itertools.count() with manual range
    result: List[int] = []

    current: int = start
    count: int = 0

    while count < limit:
        result.append(current)
        current = current + step
        count = count + 1

    return result


def test_cycle(items: List[str], num_items: int) -> List[str]:
    """Test cycle() to repeat iterable indefinitely"""
    # Simulate itertools.cycle() manually
    result: List[str] = []

    idx: int = 0
    for i in range(num_items):
        result.append(items[idx])
        idx = (idx + 1) % len(items)

    return result


def test_repeat(value: int, times: int) -> List[int]:
    """Test repeat() to repeat value multiple times"""
    # Simulate itertools.repeat()
    result: List[int] = []

    for i in range(times):
        result.append(value)

    return result


def test_islice(items: List[int], start: int, stop: int) -> List[int]:
    """Test islice() to slice an iterable"""
    # Simulate itertools.islice() using list slicing
    result: List[int] = items[start:stop]

    return result


def test_takewhile(numbers: List[int], threshold: int) -> List[int]:
    """Test takewhile() to take items while condition is true"""
    # Manual implementation of takewhile
    result: List[int] = []

    for num in numbers:
        if num < threshold:
            result.append(num)
        else:
            break

    return result


def test_dropwhile(numbers: List[int], threshold: int) -> List[int]:
    """Test dropwhile() to drop items while condition is true"""
    # Manual implementation of dropwhile
    result: List[int] = []
    dropping: bool = True

    for num in numbers:
        if dropping and num < threshold:
            continue
        dropping = False
        result.append(num)

    return result


def test_accumulate(numbers: List[int]) -> List[int]:
    """Test accumulate() for running totals"""
    # Manual implementation of accumulate (sum)
    result: List[int] = []
    total: int = 0

    for num in numbers:
        total = total + num
        result.append(total)

    return result


def test_pairwise(items: List[int]) -> List[Tuple[int, int]]:
    """Test pairwise iteration (sliding window of 2)"""
    result: List[Tuple[int, int]] = []

    for i in range(len(items) - 1):
        pair: Tuple[int, int] = (items[i], items[i + 1])
        result.append(pair)

    return result


def test_groupby_manual(items: List[int]) -> List[Tuple[bool, List[int]]]:
    """Test groupby-like functionality (manual implementation)"""
    # Group consecutive even and odd numbers
    groups: List[Tuple[bool, List[int]]] = []

    if len(items) == 0:
        return groups

    current_is_even: bool = items[0] % 2 == 0
    current_group: List[int] = [items[0]]

    for i in range(1, len(items)):
        item_is_even: bool = items[i] % 2 == 0

        if item_is_even == current_is_even:
            current_group.append(items[i])
        else:
            groups.append((current_is_even, current_group))
            current_is_even = item_is_even
            current_group = [items[i]]

    # Add last group
    groups.append((current_is_even, current_group))

    return groups


def test_compress(data: List[str], selectors: List[bool]) -> List[str]:
    """Test compress() to filter data by selectors"""
    result: List[str] = []

    for i in range(min(len(data), len(selectors))):
        if selectors[i]:
            result.append(data[i])

    return result


def test_chain_from_iterable(lists: List[List[int]]) -> List[int]:
    """Test chain.from_iterable() to flatten list of lists"""
    result: List[int] = []

    for sublist in lists:
        for item in sublist:
            result.append(item)

    return result


def flatten_nested_lists(nested: List[List[int]]) -> List[int]:
    """Flatten nested lists using chain concept"""
    flattened: List[int] = []

    for sublist in nested:
        for item in sublist:
            flattened.append(item)

    return flattened


def cartesian_product_manual(list1: List[int], list2: List[int]) -> List[Tuple[int, int]]:
    """Manual implementation of Cartesian product"""
    result: List[Tuple[int, int]] = []

    for item1 in list1:
        for item2 in list2:
            pair: Tuple[int, int] = (item1, item2)
            result.append(pair)

    return result


def test_zip_longest(list1: List[int], list2: List[int], fillvalue: int) -> List[Tuple[int, int]]:
    """Manual implementation of zip_longest"""
    result: List[Tuple[int, int]] = []

    max_len: int = max(len(list1), len(list2))

    for i in range(max_len):
        val1: int = fillvalue
        val2: int = fillvalue

        if i < len(list1):
            val1 = list1[i]
        if i < len(list2):
            val2 = list2[i]

        pair: Tuple[int, int] = (val1, val2)
        result.append(pair)

    return result


def test_batching(items: List[int], batch_size: int) -> List[List[int]]:
    """Split iterable into batches of fixed size"""
    batches: List[List[int]] = []

    current_batch: List[int] = []
    for item in items:
        current_batch.append(item)

        if len(current_batch) == batch_size:
            batches.append(current_batch)
            current_batch = []

    # Add remaining items
    if len(current_batch) > 0:
        batches.append(current_batch)

    return batches


def test_sliding_window(items: List[int], window_size: int) -> List[List[int]]:
    """Create sliding windows over iterable"""
    windows: List[List[int]] = []

    for i in range(len(items) - window_size + 1):
        window: List[int] = items[i:i + window_size]
        windows.append(window)

    return windows


def test_unique_justseen(items: List[int]) -> List[int]:
    """Remove consecutive duplicates"""
    if len(items) == 0:
        return []

    result: List[int] = [items[0]]

    for i in range(1, len(items)):
        if items[i] != items[i - 1]:
            result.append(items[i])

    return result


def test_nth_item(items: List[int], n: int, default: int) -> int:
    """Get nth item from iterable"""
    if n < 0 or n >= len(items):
        return default

    return items[n]


def test_all_equal(items: List[int]) -> bool:
    """Check if all items in iterable are equal"""
    if len(items) == 0:
        return True

    first: int = items[0]
    for item in items:
        if item != first:
            return False

    return True


def test_quantify(items: List[int], threshold: int) -> int:
    """Count how many items meet a condition"""
    count: int = 0

    for item in items:
        if item > threshold:
            count = count + 1

    return count


def test_all_itertools_features() -> None:
    """Run all itertools tests"""
    # Chain test
    chained: List[int] = test_chain_iterables()

    # Zip test
    numbers: List[int] = [1, 2, 3]
    letters: List[str] = ["a", "b", "c"]
    zipped: List[Tuple[int, str]] = test_zip_iterables()

    # Enumerate test
    items: List[str] = ["x", "y", "z"]
    enumerated: List[Tuple[int, str]] = test_enumerate()

    # Filter test
    evens: List[int] = test_filter()

    # Map test
    squared: List[int] = test_map()

    # Count test
    counted: List[int] = test_count(0, 2, 5)

    # Cycle test
    colors: List[str] = ["red", "green", "blue"]
    cycled: List[str] = test_cycle(colors, 10)

    # Repeat test
    repeated: List[int] = test_repeat(42, 5)

    # Islice test
    data: List[int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    sliced: List[int] = test_islice(data, 2, 7)

    # Takewhile test
    numbers2: List[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    taken: List[int] = test_takewhile(numbers2, 5)

    # Dropwhile test
    dropped: List[int] = test_dropwhile(numbers2, 5)

    # Accumulate test
    accumulated: List[int] = test_accumulate([1, 2, 3, 4, 5])

    # Pairwise test
    pairs: List[Tuple[int, int]] = test_pairwise([1, 2, 3, 4, 5])

    # Groupby test
    grouped: List[Tuple[bool, List[int]]] = test_groupby_manual([1, 1, 2, 2, 2, 3, 4, 4])

    # Compress test
    data_str: List[str] = ["a", "b", "c", "d", "e"]
    selectors: List[bool] = [True, False, True, False, True]
    compressed: List[str] = test_compress(data_str, selectors)

    # Flatten test
    nested: List[List[int]] = [[1, 2], [3, 4], [5, 6]]
    flattened: List[int] = test_chain_from_iterable(nested)
    flattened2: List[int] = flatten_nested_lists(nested)

    # Cartesian product test
    list1: List[int] = [1, 2, 3]
    list2: List[int] = [10, 20]
    product: List[Tuple[int, int]] = cartesian_product_manual(list1, list2)

    # Zip longest test
    short_list: List[int] = [1, 2, 3]
    long_list: List[int] = [10, 20, 30, 40, 50]
    zip_long: List[Tuple[int, int]] = test_zip_longest(short_list, long_list, 0)

    # Batching test
    batch_data: List[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    batches: List[List[int]] = test_batching(batch_data, 3)

    # Sliding window test
    window_data: List[int] = [1, 2, 3, 4, 5]
    windows: List[List[int]] = test_sliding_window(window_data, 3)

    # Unique justseen test
    duplicates: List[int] = [1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4, 5]
    unique: List[int] = test_unique_justseen(duplicates)

    # Nth item test
    nth: int = test_nth_item([10, 20, 30, 40, 50], 2, -1)

    # All equal test
    all_same: bool = test_all_equal([5, 5, 5, 5])
    not_same: bool = test_all_equal([1, 2, 3])

    # Quantify test
    above_threshold: int = test_quantify([1, 5, 10, 3, 8, 2, 15], 5)

    print("All itertools tests completed successfully")
