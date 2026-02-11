"""
Test itertools-like functionality using pure functions.
No import of itertools - manual implementations of chain, combinations,
group_consecutive, and take_while_positive.
"""


def chain_lists(list1: list[int], list2: list[int]) -> list[int]:
    """Chain two lists together."""
    combined: list[int] = []
    for val in list1:
        combined.append(val)
    for val2 in list2:
        combined.append(val2)
    return combined


def combinations_two(items: list[int]) -> list[list[int]]:
    """Get all 2-element combinations from a list of ints."""
    combos: list[list[int]] = []
    length: int = len(items)
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            pair: list[int] = [items[i], items[j]]
            combos.append(pair)
            j = j + 1
        i = i + 1
    return combos


def group_consecutive(numbers: list[int]) -> list[list[int]]:
    """Group consecutive numbers into sublists."""
    groups: list[list[int]] = []
    if len(numbers) == 0:
        return groups
    current_group: list[int] = [numbers[0]]
    i: int = 1
    length: int = len(numbers)
    while i < length:
        prev: int = numbers[i - 1]
        curr: int = numbers[i]
        diff: int = curr - prev
        if diff == 1:
            current_group.append(curr)
        else:
            groups.append(current_group)
            current_group = [curr]
        i = i + 1
    groups.append(current_group)
    return groups


def take_while_positive(numbers: list[int]) -> list[int]:
    """Take numbers while they are positive."""
    taken: list[int] = []
    for val in numbers:
        if val > 0:
            taken.append(val)
        else:
            return taken
    return taken


def accumulate_sums(numbers: list[int]) -> list[int]:
    """Compute running sum of the list (like itertools.accumulate)."""
    sums: list[int] = []
    total: int = 0
    for val in numbers:
        total = total + val
        sums.append(total)
    return sums


def repeat_value(val: int, times: int) -> list[int]:
    """Repeat a value n times (like itertools.repeat)."""
    result: list[int] = []
    i: int = 0
    while i < times:
        result.append(val)
        i = i + 1
    return result


def lists_equal(a: list[int], b: list[int]) -> int:
    """Return 1 if lists are equal, 0 otherwise."""
    if len(a) != len(b):
        return 0
    i: int = 0
    length: int = len(a)
    while i < length:
        if a[i] != b[i]:
            return 0
        i = i + 1
    return 1


def test_chain() -> int:
    """Test chain_lists."""
    passed: int = 0
    a: list[int] = [1, 2, 3]
    b: list[int] = [4, 5, 6]
    chained: list[int] = chain_lists(a, b)
    expected: list[int] = [1, 2, 3, 4, 5, 6]
    eq: int = lists_equal(chained, expected)
    passed = passed + eq

    empty1: list[int] = []
    c: list[int] = chain_lists(empty1, b)
    eq2: int = lists_equal(c, b)
    passed = passed + eq2

    return passed


def test_combinations() -> int:
    """Test combinations_two."""
    passed: int = 0
    items: list[int] = [10, 20, 30]
    combos: list[list[int]] = combinations_two(items)
    if len(combos) == 3:
        passed = passed + 1
    first: list[int] = combos[0]
    if first[0] == 10 and first[1] == 20:
        passed = passed + 1
    return passed


def test_group_consecutive() -> int:
    """Test group_consecutive."""
    passed: int = 0
    nums: list[int] = [1, 2, 3, 5, 6, 8]
    groups: list[list[int]] = group_consecutive(nums)
    if len(groups) == 3:
        passed = passed + 1
    first_group: list[int] = groups[0]
    expected_first: list[int] = [1, 2, 3]
    eq: int = lists_equal(first_group, expected_first)
    passed = passed + eq
    return passed


def test_take_while() -> int:
    """Test take_while_positive."""
    passed: int = 0
    nums: list[int] = [3, 5, 7, 0, 9, 11]
    taken: list[int] = take_while_positive(nums)
    expected: list[int] = [3, 5, 7]
    eq: int = lists_equal(taken, expected)
    passed = passed + eq

    all_pos: list[int] = [1, 2, 3]
    taken2: list[int] = take_while_positive(all_pos)
    eq2: int = lists_equal(taken2, all_pos)
    passed = passed + eq2
    return passed


def test_accumulate() -> int:
    """Test accumulate_sums."""
    passed: int = 0
    nums: list[int] = [1, 2, 3, 4]
    sums: list[int] = accumulate_sums(nums)
    expected: list[int] = [1, 3, 6, 10]
    eq: int = lists_equal(sums, expected)
    passed = passed + eq

    single: list[int] = [5]
    sums2: list[int] = accumulate_sums(single)
    if sums2[0] == 5:
        passed = passed + 1
    return passed


def test_repeat() -> int:
    """Test repeat_value."""
    passed: int = 0
    rep: list[int] = repeat_value(7, 4)
    expected: list[int] = [7, 7, 7, 7]
    eq: int = lists_equal(rep, expected)
    passed = passed + eq

    empty_rep: list[int] = repeat_value(1, 0)
    if len(empty_rep) == 0:
        passed = passed + 1
    return passed


def test_module() -> int:
    """Test itertools-like operations. Returns count of passed tests."""
    passed: int = 0

    r1: int = test_chain()
    passed = passed + r1

    r2: int = test_combinations()
    passed = passed + r2

    r3: int = test_group_consecutive()
    passed = passed + r3

    r4: int = test_take_while()
    passed = passed + r4

    r5: int = test_accumulate()
    passed = passed + r5

    r6: int = test_repeat()
    passed = passed + r6

    return passed
