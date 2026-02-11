"""Pathological tree/trie/heap data structure patterns for transpiler stress testing.

Tests: binary tree ops on list representations, BST validation, tree height/depth/balance,
LCA finding, trie via nested dicts, segment trees, Fenwick/BIT trees, manual heap ops,
tree serialization, diameter, path sum, mirror check, level order traversal, priority queues.

All trees use array-based (list) representations where:
- Root is at index 0
- Left child of node i is at 2*i + 1
- Right child of node i is at 2*i + 2
- Parent of node i is at (i - 1) // 2
- A value of -1 signals an empty/null node (sentinel)
"""

from typing import Dict, List, Optional, Tuple


def bt_insert_level_order(tree: List[int], value: int) -> List[int]:
    """Insert a value into a binary tree using level-order placement.

    Finds the first empty slot (-1 sentinel) in level order and places
    the value there. If no empty slot exists, appends to extend the tree.
    """
    result: List[int] = []
    i: int = 0
    while i < len(tree):
        result.append(tree[i])
        i = i + 1
    if len(result) == 0:
        result.append(value)
        return result
    idx: int = 0
    while idx < len(result):
        if result[idx] == -1:
            result[idx] = value
            return result
        idx = idx + 1
    result.append(value)
    return result


def bt_search(tree: List[int], target: int) -> bool:
    """Search for a value in a list-based binary tree via linear scan."""
    idx: int = 0
    while idx < len(tree):
        if tree[idx] == target and tree[idx] != -1:
            return True
        idx = idx + 1
    return False


def bt_preorder(tree: List[int]) -> List[int]:
    """Preorder traversal using iterative stack. Root -> Left -> Right."""
    result: List[int] = []
    if len(tree) == 0:
        return result
    stack: List[int] = [0]
    while len(stack) > 0:
        idx: int = stack[len(stack) - 1]
        stack = stack[: len(stack) - 1]
        if idx >= len(tree) or tree[idx] == -1:
            continue
        result.append(tree[idx])
        right: int = 2 * idx + 2
        if right < len(tree) and tree[right] != -1:
            stack.append(right)
        left: int = 2 * idx + 1
        if left < len(tree) and tree[left] != -1:
            stack.append(left)
    return result


def bt_postorder(tree: List[int]) -> List[int]:
    """Postorder traversal using two stacks. Left -> Right -> Root."""
    result: List[int] = []
    if len(tree) == 0:
        return result
    stack1: List[int] = [0]
    stack2: List[int] = []
    while len(stack1) > 0:
        idx: int = stack1[len(stack1) - 1]
        stack1 = stack1[: len(stack1) - 1]
        if idx >= len(tree) or tree[idx] == -1:
            continue
        stack2.append(idx)
        left: int = 2 * idx + 1
        if left < len(tree) and tree[left] != -1:
            stack1.append(left)
        right: int = 2 * idx + 2
        if right < len(tree) and tree[right] != -1:
            stack1.append(right)
    while len(stack2) > 0:
        top: int = stack2[len(stack2) - 1]
        stack2 = stack2[: len(stack2) - 1]
        result.append(tree[top])
    return result


def bst_insert(tree: List[int], value: int) -> List[int]:
    """Insert a value into a BST represented as a list.

    Navigates left/right based on comparison, extending the list as needed.
    """
    result: List[int] = []
    i: int = 0
    while i < len(tree):
        result.append(tree[i])
        i = i + 1
    if len(result) == 0:
        result.append(value)
        return result
    idx: int = 0
    while idx < len(result):
        if result[idx] == -1:
            result[idx] = value
            return result
        if value < result[idx]:
            next_idx: int = 2 * idx + 1
            while next_idx >= len(result):
                result.append(-1)
            idx = next_idx
        elif value > result[idx]:
            next_idx2: int = 2 * idx + 2
            while next_idx2 >= len(result):
                result.append(-1)
            idx = next_idx2
        else:
            return result
    return result


def bst_search(tree: List[int], target: int) -> bool:
    """Search for a value in a BST using O(log n) navigation."""
    if len(tree) == 0:
        return False
    idx: int = 0
    while idx < len(tree):
        if tree[idx] == -1:
            return False
        if target == tree[idx]:
            return True
        if target < tree[idx]:
            idx = 2 * idx + 1
        else:
            idx = 2 * idx + 2
    return False


def is_valid_bst(tree: List[int]) -> bool:
    """Check if a list-based binary tree is a valid BST.

    Uses iterative inorder traversal and checks strictly increasing values.
    """
    if len(tree) == 0:
        return True
    values: List[int] = []
    stack: List[int] = []
    idx: int = 0
    while idx < len(tree) or len(stack) > 0:
        while idx < len(tree) and tree[idx] != -1:
            stack.append(idx)
            idx = 2 * idx + 1
        if len(stack) == 0:
            break
        node: int = stack[len(stack) - 1]
        stack = stack[: len(stack) - 1]
        if tree[node] != -1:
            if len(values) > 0 and tree[node] <= values[len(values) - 1]:
                return False
            values.append(tree[node])
        right: int = 2 * node + 2
        if right < len(tree):
            idx = right
        else:
            idx = len(tree)
    return True


def tree_height(tree: List[int], idx: int) -> int:
    """Compute the height of a subtree rooted at idx using BFS."""
    if idx >= len(tree) or len(tree) == 0 or tree[idx] == -1:
        return 0
    queue: List[Tuple[int, int]] = [(idx, 1)]
    max_h: int = 0
    front: int = 0
    while front < len(queue):
        current: int = queue[front][0]
        depth: int = queue[front][1]
        front = front + 1
        if current >= len(tree) or tree[current] == -1:
            continue
        if depth > max_h:
            max_h = depth
        left: int = 2 * current + 1
        if left < len(tree) and tree[left] != -1:
            queue.append((left, depth + 1))
        right: int = 2 * current + 2
        if right < len(tree) and tree[right] != -1:
            queue.append((right, depth + 1))
    return max_h


def tree_depth_of_node(tree: List[int], target: int) -> int:
    """Find the depth of a target value using BFS. Returns -1 if not found."""
    if len(tree) == 0:
        return -1
    queue: List[Tuple[int, int]] = [(0, 0)]
    front: int = 0
    while front < len(queue):
        idx: int = queue[front][0]
        d: int = queue[front][1]
        front = front + 1
        if idx >= len(tree) or tree[idx] == -1:
            continue
        if tree[idx] == target:
            return d
        left: int = 2 * idx + 1
        if left < len(tree) and tree[left] != -1:
            queue.append((left, d + 1))
        right: int = 2 * idx + 2
        if right < len(tree) and tree[right] != -1:
            queue.append((right, d + 1))
    return -1


def balance_factor(tree: List[int], idx: int) -> int:
    """Compute the balance factor (left height - right height)."""
    if idx >= len(tree) or tree[idx] == -1:
        return 0
    left_h: int = tree_height(tree, 2 * idx + 1)
    right_h: int = tree_height(tree, 2 * idx + 2)
    return left_h - right_h


def find_lca(tree: List[int], val_a: int, val_b: int) -> int:
    """Find the Lowest Common Ancestor in a BST array. Returns -1 if not found."""
    if len(tree) == 0:
        return -1
    found_a: bool = bst_search(tree, val_a)
    found_b: bool = bst_search(tree, val_b)
    if not found_a or not found_b:
        return -1
    idx: int = 0
    while idx < len(tree):
        if tree[idx] == -1:
            return -1
        current: int = tree[idx]
        if val_a < current and val_b < current:
            idx = 2 * idx + 1
        elif val_a > current and val_b > current:
            idx = 2 * idx + 2
        else:
            return current
    return -1


def trie_insert(trie: Dict[str, Dict[str, str]], word: str) -> Dict[str, Dict[str, str]]:
    """Insert a word into a flat trie represented as nested dicts.

    Each key is a prefix path. A special key '#' marks end of word.
    """
    result: Dict[str, Dict[str, str]] = {}
    for k in trie:
        inner: Dict[str, str] = {}
        for ik in trie[k]:
            inner[ik] = trie[k][ik]
        result[k] = inner
    prefix: str = ""
    i: int = 0
    while i < len(word):
        ch: str = word[i]
        if prefix not in result:
            result[prefix] = {}
        result[prefix][ch] = ch
        prefix = prefix + ch
        i = i + 1
    if prefix not in result:
        result[prefix] = {}
    result[prefix]["#"] = "#"
    return result


def trie_search(trie: Dict[str, Dict[str, str]], word: str) -> bool:
    """Search for a complete word in the flat trie."""
    prefix: str = ""
    i: int = 0
    while i < len(word):
        ch: str = word[i]
        if prefix not in trie:
            return False
        if ch not in trie[prefix]:
            return False
        prefix = prefix + ch
        i = i + 1
    if prefix not in trie:
        return False
    if "#" not in trie[prefix]:
        return False
    return True


def trie_prefix_search(trie: Dict[str, Dict[str, str]], prefix_query: str) -> List[str]:
    """Find all words in the trie that start with the given prefix.

    Walks the prefix path, then collects all complete words reachable.
    Uses iterative DFS with a stack of (prefix, node_key) pairs.
    """
    prefix: str = ""
    i: int = 0
    while i < len(prefix_query):
        ch: str = prefix_query[i]
        if prefix not in trie:
            return []
        if ch not in trie[prefix]:
            return []
        prefix = prefix + ch
        i = i + 1
    words: List[str] = []
    stack: List[str] = [prefix]
    while len(stack) > 0:
        cur: str = stack[len(stack) - 1]
        stack = stack[: len(stack) - 1]
        if cur not in trie:
            continue
        if "#" in trie[cur]:
            words.append(cur)
        for ch2 in trie[cur]:
            if ch2 != "#":
                stack.append(cur + ch2)
    return words


def segment_tree_build(arr: List[int]) -> List[int]:
    """Build a segment tree for range sum queries. Iterative bottom-up."""
    n: int = len(arr)
    if n == 0:
        return []
    size: int = 4 * n
    seg: List[int] = []
    i: int = 0
    while i < size:
        seg.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        seg[n + j] = arr[j]
        j = j + 1
    k: int = n - 1
    while k >= 1:
        seg[k] = seg[2 * k] + seg[2 * k + 1]
        k = k - 1
    return seg


def segment_tree_query(seg: List[int], n: int, left: int, right: int) -> int:
    """Query the sum in range [left, right) on a segment tree."""
    if n == 0 or left >= right:
        return 0
    result: int = 0
    l: int = left + n
    r: int = right + n
    while l < r:
        if l % 2 == 1:
            if l < len(seg):
                result = result + seg[l]
            l = l + 1
        if r % 2 == 1:
            r = r - 1
            if r < len(seg):
                result = result + seg[r]
        l = l // 2
        r = r // 2
    return result


def segment_tree_update(seg: List[int], n: int, pos: int, value: int) -> List[int]:
    """Update a position in the segment tree. Returns new list."""
    result: List[int] = []
    i: int = 0
    while i < len(seg):
        result.append(seg[i])
        i = i + 1
    idx: int = pos + n
    if idx >= len(result):
        return result
    result[idx] = value
    idx = idx // 2
    while idx >= 1:
        left_child: int = 2 * idx
        right_child: int = 2 * idx + 1
        left_val: int = 0
        right_val: int = 0
        if left_child < len(result):
            left_val = result[left_child]
        if right_child < len(result):
            right_val = result[right_child]
        result[idx] = left_val + right_val
        idx = idx // 2
    return result


def fenwick_build(arr: List[int]) -> List[int]:
    """Build a Fenwick (Binary Indexed) Tree. 1-indexed, uses lowbit."""
    n: int = len(arr)
    bit: List[int] = []
    i: int = 0
    while i <= n:
        bit.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        idx: int = j + 1
        bit[idx] = bit[idx] + arr[j]
        parent: int = idx + (idx & (-idx))
        if parent <= n:
            bit[parent] = bit[parent] + bit[idx]
        j = j + 1
    return bit


def fenwick_query_and_update(bit: List[int], query_idx: int, update_idx: int, delta: int) -> Tuple[int, List[int]]:
    """Combined prefix sum query [0, query_idx] and point update at update_idx.

    Returns (prefix_sum, updated_bit). Combines two operations for complexity.
    """
    result_sum: int = 0
    i: int = query_idx + 1
    while i > 0:
        if i < len(bit):
            result_sum = result_sum + bit[i]
        i = i - (i & (-i))
    new_bit: List[int] = []
    k: int = 0
    while k < len(bit):
        new_bit.append(bit[k])
        k = k + 1
    n: int = len(new_bit) - 1
    j: int = update_idx + 1
    while j <= n:
        new_bit[j] = new_bit[j] + delta
        j = j + (j & (-j))
    return (result_sum, new_bit)


def heap_push_pop(heap: List[int], values: List[int]) -> Tuple[List[int], List[int]]:
    """Push all values onto a min-heap, then pop all off in sorted order.

    Combines sift-up and sift-down in a single function for stress testing.
    Returns (sorted_output, empty_heap).
    """
    h: List[int] = []
    i: int = 0
    while i < len(heap):
        h.append(heap[i])
        i = i + 1
    vi: int = 0
    while vi < len(values):
        h.append(values[vi])
        current: int = len(h) - 1
        while current > 0:
            parent: int = (current - 1) // 2
            if h[current] < h[parent]:
                temp: int = h[current]
                h[current] = h[parent]
                h[parent] = temp
                current = parent
            else:
                break
        vi = vi + 1
    sorted_out: List[int] = []
    while len(h) > 0:
        sorted_out.append(h[0])
        if len(h) == 1:
            h = []
        else:
            h[0] = h[len(h) - 1]
            h = h[: len(h) - 1]
            current2: int = 0
            while True:
                smallest: int = current2
                left: int = 2 * current2 + 1
                right: int = 2 * current2 + 2
                if left < len(h) and h[left] < h[smallest]:
                    smallest = left
                if right < len(h) and h[right] < h[smallest]:
                    smallest = right
                if smallest == current2:
                    break
                temp2: int = h[current2]
                h[current2] = h[smallest]
                h[smallest] = temp2
                current2 = smallest
    return (sorted_out, h)


def heapify(arr: List[int]) -> List[int]:
    """Build a min-heap using bottom-up heapification with inline sift-down."""
    result: List[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    n: int = len(result)
    start: int = (n // 2) - 1
    while start >= 0:
        current: int = start
        while True:
            smallest: int = current
            left: int = 2 * current + 1
            right: int = 2 * current + 2
            if left < n and result[left] < result[smallest]:
                smallest = left
            if right < n and result[right] < result[smallest]:
                smallest = right
            if smallest == current:
                break
            temp: int = result[current]
            result[current] = result[smallest]
            result[smallest] = temp
            current = smallest
        start = start - 1
    return result


def tree_serialize(tree: List[int]) -> str:
    """Serialize a list-based binary tree to a comma-separated string.

    Trims trailing nulls for compact output.
    """
    if len(tree) == 0:
        return ""
    last_valid: int = len(tree) - 1
    while last_valid >= 0 and tree[last_valid] == -1:
        last_valid = last_valid - 1
    if last_valid < 0:
        return ""
    parts: List[str] = []
    i: int = 0
    while i <= last_valid:
        if tree[i] == -1:
            parts.append("null")
        else:
            parts.append(str(tree[i]))
        i = i + 1
    result: str = ""
    p: int = 0
    while p < len(parts):
        if p > 0:
            result = result + ","
        result = result + parts[p]
        p = p + 1
    return result


def tree_deserialize(data: str) -> List[int]:
    """Deserialize a comma-separated string into a list-based binary tree.

    Uses manual tokenization with complex conditional logic.
    """
    if len(data) == 0:
        return []
    result: List[int] = []
    current_token: str = ""
    i: int = 0
    while i < len(data):
        ch: str = data[i]
        if ch == ",":
            if current_token == "null":
                result.append(-1)
            else:
                result.append(int(current_token))
            current_token = ""
        else:
            current_token = current_token + ch
        i = i + 1
    if len(current_token) > 0:
        if current_token == "null":
            result.append(-1)
        else:
            result.append(int(current_token))
    return result


def tree_diameter(tree: List[int]) -> int:
    """Compute the diameter (longest root-to-leaf distance) via BFS from root."""
    if len(tree) == 0:
        return 0
    valid_count: int = 0
    c: int = 0
    while c < len(tree):
        if tree[c] != -1:
            valid_count = valid_count + 1
        c = c + 1
    if valid_count <= 1:
        return 0
    max_dist: int = 0
    queue: List[Tuple[int, int]] = [(0, 0)]
    front: int = 0
    while front < len(queue):
        cur: int = queue[front][0]
        dist: int = queue[front][1]
        front = front + 1
        if cur >= len(tree) or tree[cur] == -1:
            continue
        if dist > max_dist:
            max_dist = dist
        left: int = 2 * cur + 1
        if left < len(tree) and tree[left] != -1:
            queue.append((left, dist + 1))
        right: int = 2 * cur + 2
        if right < len(tree) and tree[right] != -1:
            queue.append((right, dist + 1))
    return max_dist


def path_sum_exists(tree: List[int], target_sum: int) -> bool:
    """Check if there exists a root-to-leaf path with the given sum.

    Uses iterative DFS with a stack tracking (index, running_sum).
    """
    if len(tree) == 0 or tree[0] == -1:
        return False
    stack: List[Tuple[int, int]] = [(0, tree[0])]
    while len(stack) > 0:
        idx: int = stack[len(stack) - 1][0]
        current_sum: int = stack[len(stack) - 1][1]
        stack = stack[: len(stack) - 1]
        left: int = 2 * idx + 1
        right: int = 2 * idx + 2
        has_left: bool = left < len(tree) and tree[left] != -1
        has_right: bool = right < len(tree) and tree[right] != -1
        if not has_left and not has_right:
            if current_sum == target_sum:
                return True
        if has_left:
            stack.append((left, current_sum + tree[left]))
        if has_right:
            stack.append((right, current_sum + tree[right]))
    return False


def is_mirror(tree: List[int]) -> bool:
    """Check if a binary tree is symmetric around its center."""
    if len(tree) == 0 or tree[0] == -1:
        return True
    queue: List[Tuple[int, int]] = [(1, 2)]
    front: int = 0
    while front < len(queue):
        left_idx: int = queue[front][0]
        right_idx: int = queue[front][1]
        front = front + 1
        left_val: int = -1
        right_val: int = -1
        if left_idx < len(tree):
            left_val = tree[left_idx]
        if right_idx < len(tree):
            right_val = tree[right_idx]
        if left_val == -1 and right_val == -1:
            continue
        if left_val == -1 or right_val == -1:
            return False
        if left_val != right_val:
            return False
        queue.append((2 * left_idx + 1, 2 * right_idx + 2))
        queue.append((2 * left_idx + 2, 2 * right_idx + 1))
    return True


def level_order_traversal(tree: List[int]) -> List[List[int]]:
    """Collect nodes level by level. Returns list of lists per level."""
    result: List[List[int]] = []
    if len(tree) == 0 or tree[0] == -1:
        return result
    queue: List[Tuple[int, int]] = [(0, 0)]
    front: int = 0
    while front < len(queue):
        idx: int = queue[front][0]
        level: int = queue[front][1]
        front = front + 1
        if idx >= len(tree) or tree[idx] == -1:
            continue
        while len(result) <= level:
            result.append([])
        result[level].append(tree[idx])
        left: int = 2 * idx + 1
        if left < len(tree) and tree[left] != -1:
            queue.append((left, level + 1))
        right: int = 2 * idx + 2
        if right < len(tree) and tree[right] != -1:
            queue.append((right, level + 1))
    return result


def count_nodes_per_depth(tree: List[int]) -> Dict[int, int]:
    """Count valid nodes at each depth level. Returns dict depth -> count."""
    counts: Dict[int, int] = {}
    if len(tree) == 0 or tree[0] == -1:
        return counts
    queue: List[Tuple[int, int]] = [(0, 0)]
    front: int = 0
    while front < len(queue):
        idx: int = queue[front][0]
        depth: int = queue[front][1]
        front = front + 1
        if idx >= len(tree) or tree[idx] == -1:
            continue
        if depth in counts:
            counts[depth] = counts[depth] + 1
        else:
            counts[depth] = 1
        left: int = 2 * idx + 1
        if left < len(tree) and tree[left] != -1:
            queue.append((left, depth + 1))
        right: int = 2 * idx + 2
        if right < len(tree) and tree[right] != -1:
            queue.append((right, depth + 1))
    return counts


def priority_queue_ops(operations: List[Tuple[int, int]]) -> List[int]:
    """Execute a sequence of priority queue operations on a min-heap.

    Each operation is (op_code, value):
    - op_code 0: push value onto the queue
    - op_code 1: pop minimum and append to output (value ignored)
    Returns the list of popped values in order.
    """
    heap: List[Tuple[int, int]] = []
    output: List[int] = []
    op_idx: int = 0
    while op_idx < len(operations):
        op_code: int = operations[op_idx][0]
        op_val: int = operations[op_idx][1]
        op_idx = op_idx + 1
        if op_code == 0:
            heap.append((op_val, op_val))
            current: int = len(heap) - 1
            while current > 0:
                parent: int = (current - 1) // 2
                if heap[current][0] < heap[parent][0]:
                    temp: Tuple[int, int] = heap[current]
                    heap[current] = heap[parent]
                    heap[parent] = temp
                    current = parent
                else:
                    break
        elif op_code == 1 and len(heap) > 0:
            output.append(heap[0][0])
            if len(heap) == 1:
                heap = []
            else:
                heap[0] = heap[len(heap) - 1]
                heap = heap[: len(heap) - 1]
                cur: int = 0
                while True:
                    smallest: int = cur
                    left: int = 2 * cur + 1
                    right: int = 2 * cur + 2
                    if left < len(heap) and heap[left][0] < heap[smallest][0]:
                        smallest = left
                    if right < len(heap) and heap[right][0] < heap[smallest][0]:
                        smallest = right
                    if smallest == cur:
                        break
                    temp2: Tuple[int, int] = heap[cur]
                    heap[cur] = heap[smallest]
                    heap[smallest] = temp2
                    cur = smallest
    return output


def test_all() -> bool:
    """Test all tree/trie/heap functions with concrete data."""
    all_pass: bool = True

    # bt_insert_level_order
    t1: List[int] = bt_insert_level_order([], 10)
    t1 = bt_insert_level_order(t1, 5)
    t1 = bt_insert_level_order(t1, 15)
    if len(t1) != 3 or t1[0] != 10:
        all_pass = False

    # bt_search
    tree_a: List[int] = [10, 5, 15, 3, 7, -1, 20]
    if not bt_search(tree_a, 7) or bt_search(tree_a, 99):
        all_pass = False

    # bt_preorder
    pre: List[int] = bt_preorder(tree_a)
    if len(pre) == 0 or pre[0] != 10:
        all_pass = False

    # bt_postorder
    post: List[int] = bt_postorder(tree_a)
    if len(post) == 0 or post[len(post) - 1] != 10:
        all_pass = False

    # bst_insert and bst_search
    bst2: List[int] = bst_insert([], 50)
    bst2 = bst_insert(bst2, 30)
    bst2 = bst_insert(bst2, 70)
    if not bst_search(bst2, 50) or not bst_search(bst2, 30) or bst_search(bst2, 99):
        all_pass = False

    # is_valid_bst
    if not is_valid_bst([8, 4, 12, 2, 6, 10, 14]):
        all_pass = False
    if is_valid_bst([8, 12, 4]):
        all_pass = False
    if not is_valid_bst([]):
        all_pass = False

    # tree_height
    if tree_height(tree_a, 0) < 2:
        all_pass = False
    if tree_height([], 0) != 0:
        all_pass = False

    # tree_depth_of_node
    if tree_depth_of_node(tree_a, 10) != 0:
        all_pass = False
    if tree_depth_of_node(tree_a, 7) != 2:
        all_pass = False
    if tree_depth_of_node(tree_a, 99) != -1:
        all_pass = False

    # balance_factor
    balanced: List[int] = [10, 5, 15, 3, 7, 12, 20]
    bf: int = balance_factor(balanced, 0)
    if bf < -1 or bf > 1:
        all_pass = False

    # find_lca
    lca_tree: List[int] = [20, 10, 30, 5, 15, 25, 35]
    if find_lca(lca_tree, 5, 15) != 10:
        all_pass = False
    if find_lca(lca_tree, 5, 35) != 20:
        all_pass = False

    # trie operations
    trie: Dict[str, Dict[str, str]] = {}
    trie = trie_insert(trie, "cat")
    trie = trie_insert(trie, "car")
    trie = trie_insert(trie, "dog")
    if not trie_search(trie, "cat") or not trie_search(trie, "dog"):
        all_pass = False
    if trie_search(trie, "ca") or trie_search(trie, "cats"):
        all_pass = False

    # trie_prefix_search
    ca_words: List[str] = trie_prefix_search(trie, "ca")
    if len(ca_words) != 2:
        all_pass = False
    empty_words: List[str] = trie_prefix_search(trie, "xyz")
    if len(empty_words) != 0:
        all_pass = False

    # segment tree
    seg_arr: List[int] = [1, 3, 5, 7, 9, 11]
    seg: List[int] = segment_tree_build(seg_arr)
    n_seg: int = len(seg_arr)
    if segment_tree_query(seg, n_seg, 0, 3) != 9:
        all_pass = False
    if segment_tree_query(seg, n_seg, 1, 4) != 15:
        all_pass = False
    seg = segment_tree_update(seg, n_seg, 2, 10)
    if segment_tree_query(seg, n_seg, 0, 3) != 14:
        all_pass = False

    # fenwick tree
    fen_arr: List[int] = [3, 2, -1, 6, 5, 4, -3, 3, 7, 2, 3]
    bit: List[int] = fenwick_build(fen_arr)
    result: Tuple[int, List[int]] = fenwick_query_and_update(bit, 4, 2, 5)
    if result[0] != 15:
        all_pass = False
    result2: Tuple[int, List[int]] = fenwick_query_and_update(result[1], 4, 0, 0)
    if result2[0] != 20:
        all_pass = False

    # heap_push_pop
    sorted_vals: Tuple[List[int], List[int]] = heap_push_pop([], [9, 1, 5, 3, 7])
    if len(sorted_vals[0]) != 5:
        all_pass = False
    if sorted_vals[0][0] != 1 or sorted_vals[0][1] != 3:
        all_pass = False

    # heapify
    heaped: List[int] = heapify([9, 5, 6, 2, 3])
    if heaped[0] != 2:
        all_pass = False

    # tree serialization round-trip
    orig: List[int] = [10, 5, 15, 3, 7, -1, 20]
    serialized: str = tree_serialize(orig)
    deserialized: List[int] = tree_deserialize(serialized)
    if len(deserialized) != 7 or deserialized[0] != 10 or deserialized[5] != -1:
        all_pass = False

    # tree_diameter
    if tree_diameter([1, 2, 3, 4, 5, 6, 7]) < 2:
        all_pass = False
    if tree_diameter([]) != 0:
        all_pass = False

    # path_sum_exists
    ps_tree: List[int] = [10, 5, 15, 3, 7, -1, 20]
    if not path_sum_exists(ps_tree, 18) or not path_sum_exists(ps_tree, 22):
        all_pass = False
    if path_sum_exists(ps_tree, 100):
        all_pass = False

    # is_mirror
    if not is_mirror([1, 2, 2, 3, 4, 4, 3]):
        all_pass = False
    if is_mirror([1, 2, 2, -1, 3, -1, 3]):
        all_pass = False

    # level_order_traversal
    levels: List[List[int]] = level_order_traversal([1, 2, 3, 4, 5, 6, 7])
    if len(levels) != 3:
        all_pass = False
    if len(levels) > 2 and len(levels[2]) != 4:
        all_pass = False
    if len(level_order_traversal([])) != 0:
        all_pass = False

    # count_nodes_per_depth
    depth_counts: Dict[int, int] = count_nodes_per_depth([1, 2, 3, 4, 5, 6, 7])
    if 0 not in depth_counts or depth_counts[0] != 1:
        all_pass = False
    if 2 not in depth_counts or depth_counts[2] != 4:
        all_pass = False

    # priority_queue_ops
    ops: List[Tuple[int, int]] = [
        (0, 5), (0, 1), (0, 3), (0, 2),
        (1, 0), (1, 0), (1, 0), (1, 0),
    ]
    pq_out: List[int] = priority_queue_ops(ops)
    if len(pq_out) != 4:
        all_pass = False
    if pq_out[0] != 1 or pq_out[1] != 2 or pq_out[2] != 3 or pq_out[3] != 5:
        all_pass = False

    return all_pass


if __name__ == "__main__":
    passed: bool = test_all()
    if passed:
        print("ALL TESTS PASSED")
    else:
        print("SOME TESTS FAILED")
