"""Hard generic containers: Generic[T], TypeVar bounds, covariant/contravariant, nested generics."""

from typing import (
    TypeVar,
    Generic,
    List,
    Dict,
    Tuple,
    Optional,
    Callable,
    Set,
    Iterator,
    Sequence,
    Union,
)


T = TypeVar("T")
U = TypeVar("U")
K = TypeVar("K")
V = TypeVar("V")
T_co = TypeVar("T_co", covariant=True)
Numeric = TypeVar("Numeric", int, float)


class Stack(Generic[T]):
    """Generic stack implementation."""

    def __init__(self) -> None:
        self._items: List[T] = []

    def push(self, item: T) -> None:
        self._items.append(item)

    def pop(self) -> Optional[T]:
        if self._items:
            return self._items.pop()
        return None

    def peek(self) -> Optional[T]:
        if self._items:
            return self._items[-1]
        return None

    def size(self) -> int:
        return len(self._items)

    def is_empty(self) -> bool:
        return len(self._items) == 0

    def to_list(self) -> List[T]:
        return list(reversed(self._items))

    def drain(self) -> List[T]:
        result = list(reversed(self._items))
        self._items.clear()
        return result


class OrderedMap(Generic[K, V]):
    """Generic ordered map preserving insertion order."""

    def __init__(self) -> None:
        self._keys: List[K] = []
        self._values: List[V] = []

    def put(self, key: K, value: V) -> None:
        for i, k in enumerate(self._keys):
            if k == key:
                self._values[i] = value
                return
        self._keys.append(key)
        self._values.append(value)

    def get(self, key: K) -> Optional[V]:
        for i, k in enumerate(self._keys):
            if k == key:
                return self._values[i]
        return None

    def remove(self, key: K) -> Optional[V]:
        for i, k in enumerate(self._keys):
            if k == key:
                self._keys.pop(i)
                return self._values.pop(i)
        return None

    def keys(self) -> List[K]:
        return list(self._keys)

    def values(self) -> List[V]:
        return list(self._values)

    def items(self) -> List[Tuple[K, V]]:
        return list(zip(self._keys, self._values))

    def size(self) -> int:
        return len(self._keys)

    def contains_key(self, key: K) -> bool:
        return key in self._keys


class Result(Generic[T]):
    """Result type wrapping success/failure (covariant read-only)."""

    def __init__(self, value: Optional[T], error: Optional[str]) -> None:
        self._value = value
        self._error = error

    @classmethod
    def ok(cls, value: T) -> "Result[T]":
        return cls(value, None)

    @classmethod
    def err(cls, error: str) -> "Result[T]":
        return cls(None, error)

    def is_ok(self) -> bool:
        return self._error is None

    def is_err(self) -> bool:
        return self._error is not None

    def unwrap(self) -> T:
        if self._value is not None:
            return self._value
        raise ValueError(f"Unwrap on error: {self._error}")

    def unwrap_or(self, default: T) -> T:
        if self._value is not None:
            return self._value
        return default

    def map(self, func: Callable[[T], U]) -> "Result[U]":
        if self._value is not None:
            return Result.ok(func(self._value))
        return Result.err(self._error if self._error else "unknown")

    def error_message(self) -> str:
        return self._error if self._error else ""


class TreeNode(Generic[T]):
    """Generic binary tree node."""

    def __init__(self, value: T) -> None:
        self.value = value
        self.left: Optional["TreeNode[T]"] = None
        self.right: Optional["TreeNode[T]"] = None

    def insert_left(self, value: T) -> "TreeNode[T]":
        node: TreeNode[T] = TreeNode(value)
        self.left = node
        return node

    def insert_right(self, value: T) -> "TreeNode[T]":
        node: TreeNode[T] = TreeNode(value)
        self.right = node
        return node

    def is_leaf(self) -> bool:
        return self.left is None and self.right is None

    def height(self) -> int:
        left_h = self.left.height() if self.left else 0
        right_h = self.right.height() if self.right else 0
        return 1 + max(left_h, right_h)

    def inorder(self) -> List[T]:
        result: List[T] = []
        if self.left:
            result.extend(self.left.inorder())
        result.append(self.value)
        if self.right:
            result.extend(self.right.inorder())
        return result

    def preorder(self) -> List[T]:
        result: List[T] = [self.value]
        if self.left:
            result.extend(self.left.preorder())
        if self.right:
            result.extend(self.right.preorder())
        return result

    def node_count(self) -> int:
        count = 1
        if self.left:
            count += self.left.node_count()
        if self.right:
            count += self.right.node_count()
        return count


class PriorityQueue(Generic[T]):
    """Generic min-heap priority queue."""

    def __init__(self) -> None:
        self._heap: List[Tuple[float, int, T]] = []
        self._counter: int = 0

    def push(self, priority: float, item: T) -> None:
        entry = (priority, self._counter, item)
        self._heap.append(entry)
        self._counter += 1
        self._sift_up(len(self._heap) - 1)

    def pop(self) -> Optional[T]:
        if not self._heap:
            return None
        if len(self._heap) == 1:
            return self._heap.pop()[2]
        top = self._heap[0][2]
        self._heap[0] = self._heap.pop()
        self._sift_down(0)
        return top

    def peek(self) -> Optional[T]:
        if self._heap:
            return self._heap[0][2]
        return None

    def size(self) -> int:
        return len(self._heap)

    def _sift_up(self, idx: int) -> None:
        while idx > 0:
            parent = (idx - 1) // 2
            if self._heap[idx] < self._heap[parent]:
                self._heap[idx], self._heap[parent] = self._heap[parent], self._heap[idx]
                idx = parent
            else:
                break

    def _sift_down(self, idx: int) -> None:
        n = len(self._heap)
        while True:
            smallest = idx
            left = 2 * idx + 1
            right = 2 * idx + 2
            if left < n and self._heap[left] < self._heap[smallest]:
                smallest = left
            if right < n and self._heap[right] < self._heap[smallest]:
                smallest = right
            if smallest != idx:
                self._heap[idx], self._heap[smallest] = self._heap[smallest], self._heap[idx]
                idx = smallest
            else:
                break


# Complex nested generic type aliases
NestedDict = Dict[str, List[Tuple[int, float]]]
Matrix = List[List[float]]
Graph = Dict[str, List[Tuple[str, float]]]
LookupTable = Dict[str, Dict[str, List[int]]]


def build_nested_dict(
    keys: List[str], values: List[List[Tuple[int, float]]]
) -> NestedDict:
    """Build a Dict[str, List[Tuple[int, float]]]."""
    result: NestedDict = {}
    for k, v in zip(keys, values):
        result[k] = list(v)
    return result


def flatten_nested_dict(data: NestedDict) -> List[Tuple[str, int, float]]:
    """Flatten a nested dict into a list of triples."""
    result: List[Tuple[str, int, float]] = []
    for key, tuples in data.items():
        for i, f in tuples:
            result.append((key, i, f))
    return result


def matrix_multiply(a: Matrix, b: Matrix) -> Matrix:
    """Multiply two matrices represented as List[List[float]]."""
    rows_a = len(a)
    cols_a = len(a[0]) if a else 0
    cols_b = len(b[0]) if b else 0

    result: Matrix = [[0.0] * cols_b for _ in range(rows_a)]
    for i in range(rows_a):
        for j in range(cols_b):
            total = 0.0
            for k in range(cols_a):
                total += a[i][k] * b[k][j]
            result[i][j] = total
    return result


def dijkstra_shortest(graph: Graph, start: str, end: str) -> Tuple[float, List[str]]:
    """Shortest path using priority queue on generic graph type."""
    distances: Dict[str, float] = {start: 0.0}
    previous: Dict[str, str] = {}
    pq: PriorityQueue[str] = PriorityQueue()
    pq.push(0.0, start)

    while pq.size() > 0:
        current = pq.pop()
        if current is None:
            break
        if current == end:
            break
        current_dist = distances.get(current, float("inf"))
        neighbors = graph.get(current, [])
        for neighbor, weight in neighbors:
            new_dist = current_dist + weight
            if new_dist < distances.get(neighbor, float("inf")):
                distances[neighbor] = new_dist
                previous[neighbor] = current
                pq.push(new_dist, neighbor)

    # Reconstruct path
    path: List[str] = []
    node = end
    while node in previous:
        path.append(node)
        node = previous[node]
    if node == start:
        path.append(start)
    path.reverse()

    dist = distances.get(end, float("inf"))
    return (dist, path)


def build_lookup_table(
    records: List[Tuple[str, str, int]]
) -> LookupTable:
    """Build Dict[str, Dict[str, List[int]]] from flat records."""
    table: LookupTable = {}
    for cat, subcat, val in records:
        if cat not in table:
            table[cat] = {}
        if subcat not in table[cat]:
            table[cat][subcat] = []
        table[cat][subcat].append(val)
    return table


def map_values(omap: OrderedMap[K, V], func: Callable[[V], U]) -> OrderedMap[K, U]:
    """Apply a function to all values in an ordered map."""
    result: OrderedMap[K, U] = OrderedMap()
    for key, value in omap.items():
        result.put(key, func(value))
    return result


def chain_results(
    values: List[T], transform: Callable[[T], Result[U]]
) -> Result[List[U]]:
    """Apply a transform to each value, short-circuiting on first error."""
    results: List[U] = []
    for v in values:
        r = transform(v)
        if r.is_err():
            return Result.err(r.error_message())
        results.append(r.unwrap())
    return Result.ok(results)


def sum_numeric(values: List[Numeric]) -> Numeric:
    """Sum a list of numeric values (int or float)."""
    total: Numeric = type(values[0])(0) if values else 0
    for v in values:
        total = total + v  # type: ignore
    return total


# Untyped function 1: test inference on generic tree
def build_balanced_tree(items):
    if not items:
        return None
    mid = len(items) // 2
    node = TreeNode(items[mid])
    left_items = items[:mid]
    right_items = items[mid + 1:]
    if left_items:
        left_node = build_balanced_tree(left_items)
        node.left = left_node
    if right_items:
        right_node = build_balanced_tree(right_items)
        node.right = right_node
    return node


# Untyped function 2: test inference on generic stack operations
def evaluate_rpn(tokens):
    stack = Stack()
    for token in tokens:
        if isinstance(token, (int, float)):
            stack.push(float(token))
        elif token == "+":
            b = stack.pop()
            a = stack.pop()
            if a is not None and b is not None:
                stack.push(a + b)
        elif token == "-":
            b = stack.pop()
            a = stack.pop()
            if a is not None and b is not None:
                stack.push(a - b)
        elif token == "*":
            b = stack.pop()
            a = stack.pop()
            if a is not None and b is not None:
                stack.push(a * b)
        elif token == "/":
            b = stack.pop()
            a = stack.pop()
            if a is not None and b is not None and b != 0.0:
                stack.push(a / b)
    result = stack.pop()
    return result if result is not None else 0.0


def main() -> None:
    # Test Stack
    s: Stack[int] = Stack()
    s.push(1)
    s.push(2)
    s.push(3)
    assert s.size() == 3
    assert s.pop() == 3
    assert s.peek() == 2

    # Test OrderedMap
    om: OrderedMap[str, int] = OrderedMap()
    om.put("a", 1)
    om.put("b", 2)
    om.put("c", 3)
    assert om.get("b") == 2
    assert om.size() == 3
    assert om.contains_key("a")
    om.put("b", 20)
    assert om.get("b") == 20

    # Test Result
    r1: Result[int] = Result.ok(42)
    assert r1.is_ok()
    assert r1.unwrap() == 42

    r2: Result[int] = Result.err("not found")
    assert r2.is_err()
    assert r2.unwrap_or(0) == 0

    r3 = r1.map(lambda x: x * 2)
    assert r3.unwrap() == 84

    # Test TreeNode
    root: TreeNode[int] = TreeNode(5)
    root.insert_left(3)
    root.insert_right(7)
    assert root.height() == 2
    assert root.node_count() == 3
    assert root.inorder() == [3, 5, 7]

    # Test PriorityQueue
    pq: PriorityQueue[str] = PriorityQueue()
    pq.push(3.0, "low")
    pq.push(1.0, "high")
    pq.push(2.0, "medium")
    assert pq.pop() == "high"
    assert pq.pop() == "medium"

    # Test nested dict
    nd = build_nested_dict(["x", "y"], [[(1, 0.5), (2, 0.7)], [(3, 0.9)]])
    flat = flatten_nested_dict(nd)
    assert len(flat) == 3

    # Test matrix multiply
    a: Matrix = [[1.0, 2.0], [3.0, 4.0]]
    b: Matrix = [[5.0, 6.0], [7.0, 8.0]]
    c = matrix_multiply(a, b)
    assert abs(c[0][0] - 19.0) < 0.01
    assert abs(c[1][1] - 50.0) < 0.01

    # Test graph shortest path
    graph: Graph = {
        "A": [("B", 1.0), ("C", 4.0)],
        "B": [("C", 2.0), ("D", 5.0)],
        "C": [("D", 1.0)],
        "D": [],
    }
    dist, path = dijkstra_shortest(graph, "A", "D")
    assert abs(dist - 4.0) < 0.01
    assert path == ["A", "B", "C", "D"]

    # Test lookup table
    records = [("fruit", "apple", 1), ("fruit", "banana", 2), ("veggie", "carrot", 3)]
    lt = build_lookup_table(records)
    assert lt["fruit"]["apple"] == [1]

    # Test chain_results
    def safe_div(x: int) -> Result[float]:
        if x == 0:
            return Result.err("division by zero")
        return Result.ok(100.0 / x)

    chained = chain_results([2, 5, 10], safe_div)
    assert chained.is_ok()

    chained_err = chain_results([2, 0, 10], safe_div)
    assert chained_err.is_err()

    # Test untyped functions
    tree = build_balanced_tree([1, 2, 3, 4, 5, 6, 7])
    assert tree is not None
    assert tree.height() == 3

    rpn_result = evaluate_rpn([3, 4, "+", 2, "*"])
    assert abs(rpn_result - 14.0) < 0.01

    # Test map_values
    mapped = map_values(om, lambda v: v * 10)
    assert mapped.get("a") == 10


if __name__ == "__main__":
    main()
