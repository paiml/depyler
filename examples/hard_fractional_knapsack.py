"""Fractional knapsack using integer arithmetic (values scaled by 1000)."""


def sort_by_ratio_desc(values: list[int], weights: list[int]) -> list[int]:
    """Return indices sorted by value/weight ratio descending.
    Uses cross multiplication to avoid floating point."""
    length: int = len(values)
    indices: list[int] = []
    idx: int = 0
    while idx < length:
        indices.append(idx)
        idx = idx + 1
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            ratio_i: int = values[indices[i]] * weights[indices[j]]
            ratio_j: int = values[indices[j]] * weights[indices[i]]
            if ratio_j > ratio_i:
                temp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = temp
            j = j + 1
        i = i + 1
    return indices


def fractional_knapsack_scaled(values: list[int], weights: list[int], capacity: int) -> int:
    """Fractional knapsack returning value * 1000 for precision."""
    length: int = len(values)
    if length == 0:
        return 0
    order: list[int] = sort_by_ratio_desc(values, weights)
    remaining: int = capacity
    total_value_scaled: int = 0
    idx: int = 0
    while idx < length and remaining > 0:
        item: int = order[idx]
        if weights[item] <= remaining:
            total_value_scaled = total_value_scaled + values[item] * 1000
            remaining = remaining - weights[item]
        else:
            fraction_scaled: int = (remaining * values[item] * 1000) // weights[item]
            total_value_scaled = total_value_scaled + fraction_scaled
            remaining = 0
        idx = idx + 1
    return total_value_scaled


def max_items_fitting(weights: list[int], capacity: int) -> int:
    """Count maximum number of items that fit (take lightest first)."""
    length: int = len(weights)
    sorted_w: list[int] = []
    wi: int = 0
    while wi < length:
        sorted_w.append(weights[wi])
        wi = wi + 1
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            if sorted_w[j] < sorted_w[i]:
                temp: int = sorted_w[i]
                sorted_w[i] = sorted_w[j]
                sorted_w[j] = temp
            j = j + 1
        i = i + 1
    count: int = 0
    remaining: int = capacity
    idx: int = 0
    while idx < length:
        if sorted_w[idx] <= remaining:
            remaining = remaining - sorted_w[idx]
            count = count + 1
        idx = idx + 1
    return count


def test_module() -> int:
    passed: int = 0

    values: list[int] = [60, 100, 120]
    weights: list[int] = [10, 20, 30]

    result: int = fractional_knapsack_scaled(values, weights, 50)
    if result == 240000:
        passed = passed + 1

    result2: int = fractional_knapsack_scaled(values, weights, 60)
    if result2 == 280000:
        passed = passed + 1

    if fractional_knapsack_scaled([], [], 10) == 0:
        passed = passed + 1

    if max_items_fitting([10, 20, 30], 25) == 1:
        passed = passed + 1
    if max_items_fitting([5, 5, 5], 15) == 3:
        passed = passed + 1

    v2: list[int] = [10]
    w2: list[int] = [5]
    if fractional_knapsack_scaled(v2, w2, 5) == 10000:
        passed = passed + 1

    return passed
