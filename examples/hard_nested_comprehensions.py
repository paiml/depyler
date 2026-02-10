"""Hard nested comprehensions: triple-nested, walrus operator, conditional, dict/set comprehensions."""

from typing import List, Dict, Set, Tuple, Optional


class Matrix:
    """Matrix class using nested comprehensions internally."""

    def __init__(self, rows: int, cols: int, fill: float = 0.0) -> None:
        self.rows = rows
        self.cols = cols
        self.data: List[List[float]] = [
            [fill for _ in range(cols)] for _ in range(rows)
        ]

    def get(self, r: int, c: int) -> float:
        return self.data[r][c]

    def set(self, r: int, c: int, value: float) -> None:
        self.data[r][c] = value

    def transpose(self) -> "Matrix":
        result = Matrix(self.cols, self.rows)
        result.data = [
            [self.data[r][c] for r in range(self.rows)]
            for c in range(self.cols)
        ]
        return result

    def flatten(self) -> List[float]:
        return [val for row in self.data for val in row]

    def row_sums(self) -> List[float]:
        return [sum(row) for row in self.data]

    def col_sums(self) -> List[float]:
        return [
            sum(self.data[r][c] for r in range(self.rows))
            for c in range(self.cols)
        ]

    def to_dict(self) -> Dict[Tuple[int, int], float]:
        return {
            (r, c): self.data[r][c]
            for r in range(self.rows)
            for c in range(self.cols)
            if self.data[r][c] != 0.0
        }

    def __repr__(self) -> str:
        rows_str = [
            "[" + ", ".join(f"{v:6.2f}" for v in row) + "]"
            for row in self.data
        ]
        return "\n".join(rows_str)


# --- Double nested list comprehensions ---

def cartesian_product(xs: List[int], ys: List[int]) -> List[Tuple[int, int]]:
    """All pairs from two lists."""
    return [(x, y) for x in xs for y in ys]


def filtered_cartesian(
    xs: List[int], ys: List[int], max_sum: int
) -> List[Tuple[int, int]]:
    """Cartesian product filtered by sum constraint."""
    return [(x, y) for x in xs for y in ys if x + y <= max_sum]


def matrix_from_func(
    rows: int, cols: int, func_id: str
) -> List[List[float]]:
    """Generate matrix using a named function pattern."""
    if func_id == "identity":
        return [
            [1.0 if r == c else 0.0 for c in range(cols)]
            for r in range(rows)
        ]
    elif func_id == "checkerboard":
        return [
            [float((r + c) % 2) for c in range(cols)]
            for r in range(rows)
        ]
    elif func_id == "diagonal":
        return [
            [float(min(r, c)) for c in range(cols)]
            for r in range(rows)
        ]
    return [[0.0 for _ in range(cols)] for _ in range(rows)]


# --- Triple nested comprehensions ---

def cube_coordinates(n: int) -> List[Tuple[int, int, int]]:
    """All 3D coordinates in an n x n x n cube."""
    return [
        (x, y, z)
        for x in range(n)
        for y in range(n)
        for z in range(n)
    ]


def filtered_cube(n: int, max_dist: float) -> List[Tuple[int, int, int]]:
    """3D coordinates within a given distance from origin."""
    return [
        (x, y, z)
        for x in range(n)
        for y in range(n)
        for z in range(n)
        if (x * x + y * y + z * z) ** 0.5 <= max_dist
    ]


def triple_nested_sum(
    data: List[List[List[int]]],
) -> int:
    """Sum all elements in a 3D list using nested comprehension."""
    return sum(
        val
        for plane in data
        for row in plane
        for val in row
    )


def nested_string_patterns(words: List[str]) -> List[str]:
    """Generate patterns from nested iteration over strings."""
    return [
        f"{w1}-{w2}-{c}"
        for w1 in words
        for w2 in words
        if w1 != w2
        for c in w1
        if c in w2
    ]


# --- Dict comprehensions ---

def invert_dict(d: Dict[str, int]) -> Dict[int, List[str]]:
    """Invert a dict: values become keys, keys become value lists."""
    unique_vals = {v for v in d.values()}
    return {
        v: [k for k, val in d.items() if val == v]
        for v in unique_vals
    }


def group_by_length(words: List[str]) -> Dict[int, List[str]]:
    """Group words by their length."""
    lengths = {len(w) for w in words}
    return {
        length: [w for w in words if len(w) == length]
        for length in sorted(lengths)
    }


def nested_dict_comprehension(
    categories: List[str], items: List[Tuple[str, str, float]]
) -> Dict[str, Dict[str, float]]:
    """Build nested dict from flat tuples."""
    return {
        cat: {
            name: price
            for cat2, name, price in items
            if cat2 == cat
        }
        for cat in categories
    }


def char_frequency_map(texts: List[str]) -> Dict[str, Dict[str, int]]:
    """Build char frequency for each text."""
    return {
        text[:20]: {
            ch: text.count(ch)
            for ch in sorted(set(text))
            if ch.isalpha()
        }
        for text in texts
        if len(text) > 0
    }


# --- Set comprehensions ---

def unique_pairs(items: List[int]) -> Set[Tuple[int, int]]:
    """All unique unordered pairs."""
    return {
        (min(a, b), max(a, b))
        for i, a in enumerate(items)
        for j, b in enumerate(items)
        if i < j
    }


def prime_factors_set(numbers: List[int]) -> Set[int]:
    """Collect all prime factors from a list of numbers."""
    def factors(n: int) -> List[int]:
        result: List[int] = []
        d = 2
        while d * d <= abs(n):
            while n % d == 0:
                result.append(d)
                n //= d
            d += 1
        if abs(n) > 1:
            result.append(abs(n))
        return result

    return {
        f
        for num in numbers
        if num > 1
        for f in factors(num)
    }


def shared_chars(strings: List[str]) -> Set[str]:
    """Characters that appear in ALL strings."""
    if not strings:
        return set()
    sets = [set(s) for s in strings]
    common = sets[0]
    for s in sets[1:]:
        common = common & s
    return {ch for ch in common if ch.isalpha()}


# --- Walrus operator in comprehensions ---

def find_long_words_with_lengths(
    text: str, min_length: int
) -> List[Tuple[str, int]]:
    """Find words exceeding min_length, capturing length with walrus."""
    words = text.split()
    return [
        (word, length)
        for word in words
        if (length := len(word)) >= min_length
    ]


def filter_and_transform(
    values: List[int],
) -> List[Tuple[int, int]]:
    """Filter and transform with walrus operator."""
    return [
        (v, squared)
        for v in values
        if (squared := v * v) > 10
    ]


def running_average_filter(
    data: List[float], threshold: float
) -> List[Tuple[int, float]]:
    """Compute running sum via walrus and filter by threshold."""
    running_sum = 0.0
    results: List[Tuple[int, float]] = []
    for i, val in enumerate(data):
        running_sum += val
        avg = running_sum / (i + 1)
        if avg > threshold:
            results.append((i, avg))
    return results


def walrus_nested_filter(
    matrix: List[List[int]],
) -> List[Tuple[int, int, int]]:
    """Nested comprehension with walrus operator for row sums."""
    return [
        (i, j, val)
        for i, row in enumerate(matrix)
        if (row_sum := sum(row)) > 10
        for j, val in enumerate(row)
        if val > row_sum // len(row)
    ]


# --- Conditional nested comprehensions ---

def conditional_matrix(
    n: int, mode: str
) -> List[List[str]]:
    """Build string matrix with conditional element generation."""
    if mode == "checkerboard":
        return [
            ["X" if (r + c) % 2 == 0 else "O" for c in range(n)]
            for r in range(n)
        ]
    elif mode == "border":
        return [
            [
                "#" if (r == 0 or r == n - 1 or c == 0 or c == n - 1) else "."
                for c in range(n)
            ]
            for r in range(n)
        ]
    else:
        return [
            [str(r * n + c) for c in range(n)]
            for r in range(n)
        ]


def multi_level_filter(
    data: Dict[str, List[Dict[str, int]]]
) -> Dict[str, List[int]]:
    """Extract and filter values from nested structure."""
    return {
        key: [
            item["value"]
            for item in records
            if "value" in item and item["value"] > 0
        ]
        for key, records in data.items()
        if len(records) > 0
    }


def zip_comprehension(
    keys: List[str], values: List[List[int]]
) -> Dict[str, int]:
    """Dict comprehension over zipped iterables."""
    return {
        k: sum(v)
        for k, v in zip(keys, values)
        if len(v) > 0
    }


def nested_ternary_comprehension(
    data: List[Tuple[int, str]],
) -> List[str]:
    """Comprehension with nested ternary expressions."""
    return [
        (
            f"HIGH:{label}"
            if val > 100
            else (
                f"MED:{label}"
                if val > 50
                else (
                    f"LOW:{label}"
                    if val > 0
                    else f"ZERO:{label}"
                )
            )
        )
        for val, label in data
        if label != ""
    ]


# Untyped function 1: test inference on complex comprehension
def build_adjacency(edges):
    nodes = {n for e in edges for n in e}
    return {
        node: [
            other
            for src, dst in edges
            for other in ([dst] if src == node else ([src] if dst == node else []))
        ]
        for node in nodes
    }


# Untyped function 2: test inference on nested walrus + comprehension
def extract_valid_records(raw_data):
    results = []
    for record in raw_data:
        valid_fields = {
            k: v
            for k, v in record.items()
            if v is not None and v != ""
        }
        if len(valid_fields) >= 2:
            results.append(valid_fields)
    return results


def generate_multiplication_table(n: int) -> List[List[int]]:
    """Classic multiplication table via nested comprehension."""
    return [
        [i * j for j in range(1, n + 1)]
        for i in range(1, n + 1)
    ]


def pascal_triangle(rows: int) -> List[List[int]]:
    """Generate Pascal's triangle using comprehension patterns."""
    triangle: List[List[int]] = [[1]]
    for i in range(1, rows):
        prev = triangle[i - 1]
        row = [1] + [
            prev[j] + prev[j + 1]
            for j in range(len(prev) - 1)
        ] + [1]
        triangle.append(row)
    return triangle


def flatten_and_unique(nested: List[List[int]]) -> List[int]:
    """Flatten nested list and remove duplicates preserving order."""
    seen: Set[int] = set()
    return [
        x
        for sublist in nested
        for x in sublist
        if x not in seen and not seen.add(x)  # type: ignore
    ]


def main() -> None:
    # Test Matrix class
    m = Matrix(3, 3, 1.0)
    m.set(0, 0, 5.0)
    assert m.get(0, 0) == 5.0
    t = m.transpose()
    assert t.get(0, 0) == 5.0
    assert len(m.flatten()) == 9
    sparse = m.to_dict()
    assert len(sparse) > 0

    # Test cartesian products
    pairs = cartesian_product([1, 2], [3, 4])
    assert len(pairs) == 4
    filtered = filtered_cartesian([1, 2, 3], [1, 2, 3], 4)
    assert all(x + y <= 4 for x, y in filtered)

    # Test matrix generation
    identity = matrix_from_func(3, 3, "identity")
    assert identity[0][0] == 1.0 and identity[0][1] == 0.0

    # Test triple nested
    coords = cube_coordinates(3)
    assert len(coords) == 27
    sphere = filtered_cube(5, 3.0)
    assert all((x**2 + y**2 + z**2) ** 0.5 <= 3.0 for x, y, z in sphere)

    cube_data = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
    assert triple_nested_sum(cube_data) == 36

    # Test dict comprehensions
    inverted = invert_dict({"a": 1, "b": 2, "c": 1})
    assert sorted(inverted[1]) == ["a", "c"]

    grouped = group_by_length(["hi", "hello", "hey", "world"])
    assert len(grouped[2]) == 1
    assert len(grouped[5]) == 2

    nested = nested_dict_comprehension(
        ["fruit", "veggie"],
        [("fruit", "apple", 1.5), ("fruit", "banana", 0.5), ("veggie", "carrot", 0.8)],
    )
    assert nested["fruit"]["apple"] == 1.5

    # Test set comprehensions
    pairs_set = unique_pairs([1, 2, 3])
    assert len(pairs_set) == 3

    primes = prime_factors_set([12, 15, 20])
    assert 2 in primes and 3 in primes and 5 in primes

    common = shared_chars(["hello", "world", "hold"])
    assert "l" in common or "o" in common

    # Test walrus operator
    long_words = find_long_words_with_lengths("the quick brown fox jumps", 4)
    assert all(length >= 4 for _, length in long_words)

    filtered_sq = filter_and_transform([1, 2, 3, 4, 5])
    assert all(sq > 10 for _, sq in filtered_sq)

    # Test conditional matrix
    checker = conditional_matrix(4, "checkerboard")
    assert checker[0][0] == "X"
    assert checker[0][1] == "O"

    border = conditional_matrix(4, "border")
    assert border[0][0] == "#"
    assert border[1][1] == "."

    # Test multi-level filter
    data = {
        "group1": [{"value": 5}, {"value": -1}, {"value": 10}],
        "group2": [{"value": 3}],
    }
    filtered_data = multi_level_filter(data)
    assert filtered_data["group1"] == [5, 10]

    # Test pascal triangle
    pascal = pascal_triangle(5)
    assert pascal[4] == [1, 4, 6, 4, 1]

    # Test multiplication table
    table = generate_multiplication_table(4)
    assert table[2][3] == 12

    # Test flatten_and_unique
    flat = flatten_and_unique([[1, 2, 3], [2, 3, 4], [4, 5]])
    assert flat == [1, 2, 3, 4, 5]

    # Test untyped functions
    adj = build_adjacency([(1, 2), (2, 3), (1, 3)])
    assert isinstance(adj, dict)

    records = extract_valid_records([
        {"name": "alice", "age": 30, "email": None},
        {"name": "bob", "age": 25, "email": "bob@test.com"},
        {"x": None, "y": None},
    ])
    assert len(records) >= 1


if __name__ == "__main__":
    main()
