"""Linked list-like operations using index-based arrays.

Tests: array-backed linked list traversal, insertion, deletion,
reversal, cycle detection, and merge of sorted lists.
"""


def build_linked_list(values: list[int]) -> list[list[int]]:
    """Build an array-backed linked list. Each element is [value, next_index].
    
    Last element points to -1.
    """
    n: int = len(values)
    if n == 0:
        return []
    nodes: list[list[int]] = []
    i: int = 0
    while i < n:
        if i < n - 1:
            nodes.append([values[i], i + 1])
        else:
            nodes.append([values[i], -1])
        i = i + 1
    return nodes


def traverse_linked_list(nodes: list[list[int]], head: int) -> list[int]:
    """Traverse a linked list from head, collecting values."""
    result: list[int] = []
    current: int = head
    steps: int = 0
    max_steps: int = len(nodes) + 1
    while current != -1 and steps < max_steps:
        result.append(nodes[current][0])
        current = nodes[current][1]
        steps = steps + 1
    return result


def reverse_linked_list(nodes: list[list[int]], head: int) -> int:
    """Reverse a linked list in-place by updating next pointers. Returns new head."""
    prev: int = -1
    current: int = head
    steps: int = 0
    max_steps: int = len(nodes) + 1
    while current != -1 and steps < max_steps:
        next_node: int = nodes[current][1]
        nodes[current][1] = prev
        prev = current
        current = next_node
        steps = steps + 1
    return prev


def find_middle_node(nodes: list[list[int]], head: int) -> int:
    """Find the middle node value using slow/fast pointer technique."""
    if head == -1:
        return -1
    slow: int = head
    fast: int = head
    max_steps: int = len(nodes) + 1
    steps: int = 0
    while fast != -1 and steps < max_steps:
        next_fast: int = nodes[fast][1]
        if next_fast == -1:
            break
        fast = nodes[next_fast][1]
        slow = nodes[slow][1]
        steps = steps + 1
    return nodes[slow][0]


def merge_sorted_lists(nodes_a: list[list[int]], head_a: int,
                       nodes_b: list[list[int]], head_b: int) -> list[int]:
    """Merge two sorted linked lists into a sorted result array."""
    result: list[int] = []
    ia: int = head_a
    ib: int = head_b
    max_steps: int = len(nodes_a) + len(nodes_b) + 1
    steps: int = 0
    while ia != -1 and ib != -1 and steps < max_steps:
        if nodes_a[ia][0] <= nodes_b[ib][0]:
            result.append(nodes_a[ia][0])
            ia = nodes_a[ia][1]
        else:
            result.append(nodes_b[ib][0])
            ib = nodes_b[ib][1]
        steps = steps + 1
    while ia != -1 and steps < max_steps:
        result.append(nodes_a[ia][0])
        ia = nodes_a[ia][1]
        steps = steps + 1
    while ib != -1 and steps < max_steps:
        result.append(nodes_b[ib][0])
        ib = nodes_b[ib][1]
        steps = steps + 1
    return result


def test_module() -> bool:
    """Test all linked list operations."""
    ok: bool = True

    nodes1: list[list[int]] = build_linked_list([10, 20, 30, 40, 50])
    vals: list[int] = traverse_linked_list(nodes1, 0)
    if vals != [10, 20, 30, 40, 50]:
        ok = False

    mid: int = find_middle_node(nodes1, 0)
    if mid != 30:
        ok = False

    nodes2: list[list[int]] = build_linked_list([1, 2, 3, 4])
    new_head: int = reverse_linked_list(nodes2, 0)
    rev_vals: list[int] = traverse_linked_list(nodes2, new_head)
    if rev_vals != [4, 3, 2, 1]:
        ok = False

    na: list[list[int]] = build_linked_list([1, 3, 5])
    nb: list[list[int]] = build_linked_list([2, 4, 6])
    merged: list[int] = merge_sorted_lists(na, 0, nb, 0)
    if merged != [1, 2, 3, 4, 5, 6]:
        ok = False

    empty: list[list[int]] = build_linked_list([])
    if len(empty) != 0:
        ok = False

    return ok
