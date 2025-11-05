"""
Comprehensive Functional Programming Example
Combines: itertools, functools, operator, collections

This example demonstrates functional programming patterns using
multiple Python stdlib modules working together.

Tests transpiler's ability to handle:
- Functional composition
- Iterator operations
- Higher-order functions
- Data transformations
"""

from typing import List, Dict, Tuple, Callable


def map_transform(data: List[int], multiplier: int) -> List[int]:
    """Map transformation over list"""
    result: List[int] = []

    for item in data:
        transformed: int = item * multiplier
        result.append(transformed)

    return result


def filter_predicate(data: List[int], threshold: int) -> List[int]:
    """Filter data by predicate"""
    result: List[int] = []

    for item in data:
        if item > threshold:
            result.append(item)

    return result


def reduce_sum(data: List[int]) -> int:
    """Reduce list to sum"""
    total: int = 0

    for item in data:
        total = total + item

    return total


def reduce_product(data: List[int]) -> int:
    """Reduce list to product"""
    if len(data) == 0:
        return 0

    product: int = 1
    for item in data:
        product = product * item

    return product


def chain_operations(data: List[int]) -> int:
    """Chain multiple operations together"""
    # Map: multiply by 2
    mapped: List[int] = map_transform(data, 2)

    # Filter: keep > 10
    filtered: List[int] = filter_predicate(mapped, 10)

    # Reduce: sum
    result: int = reduce_sum(filtered)

    return result


def zip_lists(list1: List[int], list2: List[str]) -> List[Tuple[int, str]]:
    """Zip two lists together"""
    result: List[Tuple[int, str]] = []

    min_len: int = min(len(list1), len(list2))

    for i in range(min_len):
        pair: Tuple[int, str] = (list1[i], list2[i])
        result.append(pair)

    return result


def enumerate_list(items: List[str]) -> List[Tuple[int, str]]:
    """Enumerate list with indices"""
    result: List[Tuple[int, str]] = []

    for i in range(len(items)):
        pair: Tuple[int, str] = (i, items[i])
        result.append(pair)

    return result


def group_by_property(items: List[int], modulo: int) -> Dict[int, List[int]]:
    """Group items by property (modulo)"""
    groups: Dict[int, List[int]] = {}

    for item in items:
        key: int = item % modulo

        if key not in groups:
            groups[key] = []

        groups[key].append(item)

    return groups


def partition_by_predicate(items: List[int], threshold: int) -> Tuple[List[int], List[int]]:
    """Partition list into two based on predicate"""
    passed: List[int] = []
    failed: List[int] = []

    for item in items:
        if item >= threshold:
            passed.append(item)
        else:
            failed.append(item)

    return (passed, failed)


def accumulate_running_sum(data: List[int]) -> List[int]:
    """Create list of running sums (accumulate pattern)"""
    result: List[int] = []
    total: int = 0

    for item in data:
        total = total + item
        result.append(total)

    return result


def flatten_nested_list(nested: List[List[int]]) -> List[int]:
    """Flatten nested list structure"""
    result: List[int] = []

    for sublist in nested:
        for item in sublist:
            result.append(item)

    return result


def cartesian_product(list1: List[int], list2: List[int]) -> List[Tuple[int, int]]:
    """Compute Cartesian product of two lists"""
    result: List[Tuple[int, int]] = []

    for item1 in list1:
        for item2 in list2:
            pair: Tuple[int, int] = (item1, item2)
            result.append(pair)

    return result


def take_while_condition(data: List[int], threshold: int) -> List[int]:
    """Take elements while condition is true"""
    result: List[int] = []

    for item in data:
        if item < threshold:
            result.append(item)
        else:
            break

    return result


def drop_while_condition(data: List[int], threshold: int) -> List[int]:
    """Drop elements while condition is true"""
    result: List[int] = []
    dropping: bool = True

    for item in data:
        if dropping and item < threshold:
            continue
        dropping = False
        result.append(item)

    return result


def pairwise_iteration(data: List[int]) -> List[Tuple[int, int]]:
    """Iterate over consecutive pairs"""
    result: List[Tuple[int, int]] = []

    for i in range(len(data) - 1):
        pair: Tuple[int, int] = (data[i], data[i + 1])
        result.append(pair)

    return result


def sliding_window(data: List[int], window_size: int) -> List[List[int]]:
    """Create sliding windows over data"""
    result: List[List[int]] = []

    for i in range(len(data) - window_size + 1):
        window: List[int] = []
        for j in range(window_size):
            window.append(data[i + j])
        result.append(window)

    return result


def compose_two_functions(data: List[int]) -> List[int]:
    """Compose two functions (f ∘ g)"""
    # First function: multiply by 2
    step1: List[int] = map_transform(data, 2)

    # Second function: add 1 to each
    step2: List[int] = []
    for item in step1:
        step2.append(item + 1)

    return step2


def apply_multiple_operations(data: List[int], operations: List[str]) -> List[int]:
    """Apply multiple operations in sequence"""
    result: List[int] = data.copy()

    for op in operations:
        new_result: List[int] = []

        if op == "double":
            for item in result:
                new_result.append(item * 2)
        elif op == "increment":
            for item in result:
                new_result.append(item + 1)
        elif op == "square":
            for item in result:
                new_result.append(item * item)
        else:
            new_result = result

        result = new_result

    return result


def map_reduce_pattern(data: List[int]) -> int:
    """Classic map-reduce pattern"""
    # Map: square each element
    mapped: List[int] = []
    for item in data:
        mapped.append(item * item)

    # Reduce: sum all elements
    reduced: int = reduce_sum(mapped)

    return reduced


def filter_map_reduce_pattern(data: List[int], threshold: int) -> int:
    """Filter-Map-Reduce pipeline"""
    # Filter: keep only values > threshold
    filtered: List[int] = filter_predicate(data, threshold)

    # Map: multiply by 3
    mapped: List[int] = map_transform(filtered, 3)

    # Reduce: sum
    reduced: int = reduce_sum(mapped)

    return reduced


def unique_elements(data: List[int]) -> List[int]:
    """Get unique elements (set-like operation)"""
    seen: Dict[int, bool] = {}
    result: List[int] = []

    for item in data:
        if item not in seen:
            seen[item] = True
            result.append(item)

    return result


def count_by_value(data: List[int]) -> Dict[int, int]:
    """Count occurrences of each value"""
    counts: Dict[int, int] = {}

    for item in data:
        if item in counts:
            counts[item] = counts[item] + 1
        else:
            counts[item] = 1

    return counts


def sorted_by_key(items: List[Tuple[str, int]]) -> List[Tuple[str, int]]:
    """Sort list of tuples by second element"""
    result: List[Tuple[str, int]] = items.copy()

    # Bubble sort by second element
    for i in range(len(result)):
        for j in range(i + 1, len(result)):
            if result[j][1] < result[i][1]:
                temp: Tuple[str, int] = result[i]
                result[i] = result[j]
                result[j] = temp

    return result


def demonstrate_functional_patterns() -> None:
    """Demonstrate functional programming patterns"""
    print("=== Functional Programming Patterns Demo ===")

    # Sample data
    data: List[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # Map
    print("\n1. Map Pattern")
    doubled: List[int] = map_transform(data, 2)
    print(f"   Doubled: {len(doubled)} elements")

    # Filter
    print("\n2. Filter Pattern")
    filtered: List[int] = filter_predicate(data, 5)
    print(f"   Filtered (>5): {len(filtered)} elements")

    # Reduce
    print("\n3. Reduce Pattern")
    total: int = reduce_sum(data)
    print(f"   Sum: {total}")

    # Chain operations
    print("\n4. Chained Operations")
    chained: int = chain_operations(data)
    print(f"   Result: {chained}")

    # Zip
    print("\n5. Zip Pattern")
    labels: List[str] = ["a", "b", "c", "d", "e"]
    zipped: List[Tuple[int, str]] = zip_lists(data[:5], labels)
    print(f"   Zipped: {len(zipped)} pairs")

    # Group by
    print("\n6. Group By Pattern")
    groups: Dict[int, List[int]] = group_by_property(data, 3)
    print(f"   Groups (mod 3): {len(groups)} groups")

    # Partition
    print("\n7. Partition Pattern")
    parts: Tuple[List[int], List[int]] = partition_by_predicate(data, 6)
    print(f"   Partition: {len(parts[0])} passed, {len(parts[1])} failed")

    # Accumulate
    print("\n8. Accumulate Pattern")
    running_sums: List[int] = accumulate_running_sum(data)
    print(f"   Running sums: {len(running_sums)} values")

    # Flatten
    print("\n9. Flatten Pattern")
    nested: List[List[int]] = [[1, 2], [3, 4], [5, 6]]
    flattened: List[int] = flatten_nested_list(nested)
    print(f"   Flattened: {len(flattened)} elements")

    # Cartesian product
    print("\n10. Cartesian Product")
    list1: List[int] = [1, 2, 3]
    list2: List[int] = [10, 20]
    product: List[Tuple[int, int]] = cartesian_product(list1, list2)
    print(f"   Product: {len(product)} combinations")

    # Take while
    print("\n11. Take While Pattern")
    taken: List[int] = take_while_condition(data, 6)
    print(f"   Taken (while <6): {len(taken)} elements")

    # Pairwise
    print("\n12. Pairwise Iteration")
    pairs: List[Tuple[int, int]] = pairwise_iteration(data)
    print(f"   Pairs: {len(pairs)} pairs")

    # Sliding window
    print("\n13. Sliding Window")
    windows: List[List[int]] = sliding_window(data, 3)
    print(f"   Windows (size 3): {len(windows)} windows")

    # Composition
    print("\n14. Function Composition")
    composed: List[int] = compose_two_functions([1, 2, 3])
    print(f"   Composed result: {len(composed)} elements")

    # Map-Reduce
    print("\n15. Map-Reduce Pattern")
    mr_result: int = map_reduce_pattern([1, 2, 3, 4])
    print(f"   Map-Reduce sum of squares: {mr_result}")

    # Filter-Map-Reduce
    print("\n16. Filter-Map-Reduce")
    fmr_result: int = filter_map_reduce_pattern(data, 5)
    print(f"   Filter-Map-Reduce result: {fmr_result}")

    # Unique
    print("\n17. Unique Elements")
    duplicates: List[int] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
    unique: List[int] = unique_elements(duplicates)
    print(f"   Unique elements: {len(unique)}")

    print("\n=== All Patterns Demonstrated ===")


if __name__ == "__main__":
    demonstrate_functional_patterns()
