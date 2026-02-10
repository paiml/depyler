"""Hard protocol types: structural subtyping, runtime_checkable, generic protocols."""

from typing import (
    Protocol,
    TypeVar,
    Generic,
    runtime_checkable,
    Iterator,
    Sequence,
    Optional,
    List,
    Dict,
    Tuple,
)


T = TypeVar("T")
T_co = TypeVar("T_co", covariant=True)
T_contra = TypeVar("T_contra", contravariant=True)


@runtime_checkable
class Drawable(Protocol):
    """Protocol for objects that can be drawn to a buffer."""

    def draw(self, buffer: List[str]) -> None:
        ...

    def bounding_box(self) -> Tuple[int, int, int, int]:
        ...


@runtime_checkable
class Measurable(Protocol):
    """Protocol for objects with a measurable size."""

    def area(self) -> float:
        ...

    def perimeter(self) -> float:
        ...


class Serializable(Protocol[T_co]):
    """Generic covariant protocol for serialization."""

    def serialize(self) -> T_co:
        ...


class Deserializable(Protocol[T_contra]):
    """Generic contravariant protocol for deserialization."""

    def deserialize(self, data: T_contra) -> None:
        ...


class Comparable(Protocol):
    """Protocol for comparable objects with full ordering."""

    def __lt__(self, other: "Comparable") -> bool:
        ...

    def __le__(self, other: "Comparable") -> bool:
        ...

    def __gt__(self, other: "Comparable") -> bool:
        ...

    def __ge__(self, other: "Comparable") -> bool:
        ...


class SupportsAdd(Protocol[T]):
    """Protocol for objects supporting addition."""

    def __add__(self, other: T) -> T:
        ...


class Rectangle:
    """Concrete class satisfying Drawable, Measurable protocols."""

    def __init__(self, x: int, y: int, width: int, height: int) -> None:
        self.x = x
        self.y = y
        self.width = width
        self.height = height

    def draw(self, buffer: List[str]) -> None:
        top_bottom = "+" + "-" * self.width + "+"
        middle = "|" + " " * self.width + "|"
        buffer.append(top_bottom)
        for _ in range(self.height):
            buffer.append(middle)
        buffer.append(top_bottom)

    def bounding_box(self) -> Tuple[int, int, int, int]:
        return (self.x, self.y, self.x + self.width, self.y + self.height)

    def area(self) -> float:
        return float(self.width * self.height)

    def perimeter(self) -> float:
        return float(2 * (self.width + self.height))


class Circle:
    """Concrete class satisfying Measurable but not Drawable."""

    def __init__(self, cx: int, cy: int, radius: float) -> None:
        self.cx = cx
        self.cy = cy
        self.radius = radius

    def area(self) -> float:
        pi = 3.141592653589793
        return pi * self.radius * self.radius

    def perimeter(self) -> float:
        pi = 3.141592653589793
        return 2.0 * pi * self.radius


class ScoredItem(Generic[T]):
    """Generic class implementing Comparable protocol."""

    def __init__(self, value: T, score: float) -> None:
        self.value = value
        self.score = score

    def __lt__(self, other: "ScoredItem[T]") -> bool:
        return self.score < other.score

    def __le__(self, other: "ScoredItem[T]") -> bool:
        return self.score <= other.score

    def __gt__(self, other: "ScoredItem[T]") -> bool:
        return self.score > other.score

    def __ge__(self, other: "ScoredItem[T]") -> bool:
        return self.score >= other.score

    def __repr__(self) -> str:
        return f"ScoredItem(score={self.score})"


class JsonSerializable:
    """Satisfies Serializable[str] protocol."""

    def __init__(self, data: Dict[str, int]) -> None:
        self.data = data

    def serialize(self) -> str:
        pairs = [f'"{k}": {v}' for k, v in self.data.items()]
        return "{" + ", ".join(pairs) + "}"


def render_shapes(shapes: List[Drawable], width: int, height: int) -> str:
    """Render multiple drawable shapes into a text buffer."""
    buffer: List[str] = []
    header = f"Canvas({width}x{height})"
    buffer.append(header)
    for shape in shapes:
        bbox = shape.bounding_box()
        label = f"  Shape at ({bbox[0]},{bbox[1]})-({bbox[2]},{bbox[3]})"
        buffer.append(label)
        shape.draw(buffer)
    return "\n".join(buffer)


def total_area(shapes: List[Measurable]) -> float:
    """Sum the areas of all measurable shapes."""
    return sum(s.area() for s in shapes)


def find_largest_perimeter(shapes: List[Measurable]) -> float:
    """Find the shape with the largest perimeter."""
    if not shapes:
        return 0.0
    return max(s.perimeter() for s in shapes)


def sort_scored_items(items: List[ScoredItem[T]]) -> List[ScoredItem[T]]:
    """Sort scored items by their score using protocol-based comparison."""
    n = len(items)
    result = list(items)
    for i in range(n):
        for j in range(i + 1, n):
            if result[j] < result[i]:
                result[i], result[j] = result[j], result[i]
    return result


def check_protocol_conformance(obj: object) -> Dict[str, bool]:
    """Check which protocols an object conforms to at runtime."""
    results: Dict[str, bool] = {}
    results["drawable"] = isinstance(obj, Drawable)
    results["measurable"] = isinstance(obj, Measurable)
    return results


def serialize_all(items: List[Serializable[str]]) -> List[str]:
    """Serialize a list of serializable objects to strings."""
    return [item.serialize() for item in items]


# Untyped function 1: tests type inference on protocol usage
def make_shape_report(shapes):
    report_lines = []
    for i, shape in enumerate(shapes):
        line = f"Shape {i}: area={shape.area():.2f}, perimeter={shape.perimeter():.2f}"
        report_lines.append(line)
    total = sum(s.area() for s in shapes)
    report_lines.append(f"Total area: {total:.2f}")
    return "\n".join(report_lines)


# Untyped function 2: tests inference on generic containers
def merge_scored_items(left, right):
    merged = []
    i, j = 0, 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i])
            i += 1
        else:
            merged.append(right[j])
            j += 1
    while i < len(left):
        merged.append(left[i])
        i += 1
    while j < len(right):
        merged.append(right[j])
        j += 1
    return merged


def build_shape_index(shapes: List[Measurable]) -> Dict[str, List[float]]:
    """Build an index mapping area ranges to shapes."""
    index: Dict[str, List[float]] = {
        "small": [],
        "medium": [],
        "large": [],
    }
    for shape in shapes:
        a = shape.area()
        bucket = "small" if a < 100.0 else ("medium" if a < 1000.0 else "large")
        index[bucket].append(a)
    return index


def create_test_shapes() -> Tuple[List[Rectangle], List[Circle]]:
    """Create a set of test shapes for validation."""
    rects = [
        Rectangle(0, 0, 10, 5),
        Rectangle(10, 10, 20, 15),
        Rectangle(5, 5, 3, 3),
    ]
    circles = [
        Circle(0, 0, 5.0),
        Circle(10, 10, 10.0),
        Circle(5, 5, 1.0),
    ]
    return (rects, circles)


def main() -> None:
    rects, circles = create_test_shapes()

    # Test Drawable protocol
    buffer: List[str] = []
    for r in rects:
        assert isinstance(r, Drawable)
        r.draw(buffer)

    # Test Measurable protocol
    all_shapes: List[Measurable] = []
    for r in rects:
        all_shapes.append(r)
    for c in circles:
        all_shapes.append(c)

    total = total_area(all_shapes)
    largest = find_largest_perimeter(all_shapes)
    assert total > 0.0
    assert largest > 0.0

    # Test protocol conformance checking
    conf = check_protocol_conformance(rects[0])
    assert conf["drawable"] is True
    assert conf["measurable"] is True

    conf2 = check_protocol_conformance(circles[0])
    assert conf2["drawable"] is False
    assert conf2["measurable"] is True

    # Test ScoredItem sorting
    items = [
        ScoredItem("alpha", 3.0),
        ScoredItem("beta", 1.0),
        ScoredItem("gamma", 2.0),
    ]
    sorted_items = sort_scored_items(items)
    assert sorted_items[0].score <= sorted_items[1].score
    assert sorted_items[1].score <= sorted_items[2].score

    # Test serialization protocol
    js = JsonSerializable({"a": 1, "b": 2})
    serialized = js.serialize()
    assert "{" in serialized

    # Test untyped functions
    report = make_shape_report(all_shapes)
    assert "Total area" in report

    index = build_shape_index(all_shapes)
    assert isinstance(index, dict)


if __name__ == "__main__":
    main()
