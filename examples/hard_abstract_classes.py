"""Hard abstract classes: ABC, abstractmethod, multiple inheritance, concrete overrides."""

from abc import ABC, abstractmethod
from typing import List, Dict, Optional, Tuple


class Shape(ABC):
    """Abstract base class for geometric shapes."""

    def __init__(self, name: str) -> None:
        self._name = name

    @abstractmethod
    def area(self) -> float:
        ...

    @abstractmethod
    def perimeter(self) -> float:
        ...

    @abstractmethod
    def contains_point(self, x: float, y: float) -> bool:
        ...

    def describe(self) -> str:
        return f"{self._name}: area={self.area():.2f}, perimeter={self.perimeter():.2f}"

    def scale_info(self, factor: float) -> str:
        scaled_area = self.area() * factor * factor
        return f"{self._name} scaled by {factor}: area={scaled_area:.2f}"


class Transformable(ABC):
    """Abstract mixin for transformable objects."""

    @abstractmethod
    def translate(self, dx: float, dy: float) -> "Transformable":
        ...

    @abstractmethod
    def rotate(self, angle_degrees: float) -> "Transformable":
        ...

    def rotate_around(self, cx: float, cy: float, angle_degrees: float) -> "Transformable":
        moved = self.translate(-cx, -cy)
        rotated = moved.rotate(angle_degrees)
        return rotated.translate(cx, cy)


class Renderable(ABC):
    """Abstract mixin for renderable objects."""

    @abstractmethod
    def render_ascii(self, width: int, height: int) -> List[str]:
        ...

    @abstractmethod
    def bounding_box(self) -> Tuple[float, float, float, float]:
        ...

    def fits_in(self, max_width: float, max_height: float) -> bool:
        x1, y1, x2, y2 = self.bounding_box()
        return (x2 - x1) <= max_width and (y2 - y1) <= max_height


class Serializable(ABC):
    """Abstract mixin for serializable objects."""

    @abstractmethod
    def to_dict(self) -> Dict[str, float]:
        ...

    @classmethod
    @abstractmethod
    def field_names(cls) -> List[str]:
        ...

    def to_csv_row(self) -> str:
        d = self.to_dict()
        return ",".join(f"{v}" for v in d.values())


class ConcreteCircle(Shape, Transformable, Renderable, Serializable):
    """Concrete circle implementing all abstract bases."""

    PI = 3.141592653589793

    def __init__(self, cx: float, cy: float, radius: float) -> None:
        super().__init__("Circle")
        self.cx = cx
        self.cy = cy
        self.radius = radius

    def area(self) -> float:
        return self.PI * self.radius * self.radius

    def perimeter(self) -> float:
        return 2.0 * self.PI * self.radius

    def contains_point(self, x: float, y: float) -> bool:
        dx = x - self.cx
        dy = y - self.cy
        return (dx * dx + dy * dy) <= (self.radius * self.radius)

    def translate(self, dx: float, dy: float) -> "ConcreteCircle":
        return ConcreteCircle(self.cx + dx, self.cy + dy, self.radius)

    def rotate(self, angle_degrees: float) -> "ConcreteCircle":
        # Circles are rotationally symmetric
        return ConcreteCircle(self.cx, self.cy, self.radius)

    def render_ascii(self, width: int, height: int) -> List[str]:
        lines: List[str] = []
        for row in range(height):
            line_chars: List[str] = []
            for col in range(width):
                fx = (col / max(width - 1, 1)) * 2.0 * self.radius - self.radius + self.cx
                fy = (row / max(height - 1, 1)) * 2.0 * self.radius - self.radius + self.cy
                if self.contains_point(fx, fy):
                    line_chars.append("#")
                else:
                    line_chars.append(".")
            lines.append("".join(line_chars))
        return lines

    def bounding_box(self) -> Tuple[float, float, float, float]:
        return (
            self.cx - self.radius,
            self.cy - self.radius,
            self.cx + self.radius,
            self.cy + self.radius,
        )

    def to_dict(self) -> Dict[str, float]:
        return {"cx": self.cx, "cy": self.cy, "radius": self.radius}

    @classmethod
    def field_names(cls) -> List[str]:
        return ["cx", "cy", "radius"]


class ConcreteRectangle(Shape, Transformable, Renderable, Serializable):
    """Concrete rectangle implementing all abstract bases."""

    def __init__(self, x: float, y: float, width: float, height: float) -> None:
        super().__init__("Rectangle")
        self.x = x
        self.y = y
        self.w = width
        self.h = height

    def area(self) -> float:
        return self.w * self.h

    def perimeter(self) -> float:
        return 2.0 * (self.w + self.h)

    def contains_point(self, px: float, py: float) -> bool:
        return self.x <= px <= self.x + self.w and self.y <= py <= self.y + self.h

    def translate(self, dx: float, dy: float) -> "ConcreteRectangle":
        return ConcreteRectangle(self.x + dx, self.y + dy, self.w, self.h)

    def rotate(self, angle_degrees: float) -> "ConcreteRectangle":
        # Simplified: 90-degree rotations swap width and height
        turns = int(angle_degrees / 90.0) % 4
        if turns % 2 == 1:
            return ConcreteRectangle(self.x, self.y, self.h, self.w)
        return ConcreteRectangle(self.x, self.y, self.w, self.h)

    def render_ascii(self, width: int, height: int) -> List[str]:
        lines: List[str] = []
        top_bottom = "+" + "-" * (width - 2) + "+"
        middle = "|" + " " * (width - 2) + "|"
        lines.append(top_bottom)
        for _ in range(height - 2):
            lines.append(middle)
        if height > 1:
            lines.append(top_bottom)
        return lines

    def bounding_box(self) -> Tuple[float, float, float, float]:
        return (self.x, self.y, self.x + self.w, self.y + self.h)

    def to_dict(self) -> Dict[str, float]:
        return {"x": self.x, "y": self.y, "width": self.w, "height": self.h}

    @classmethod
    def field_names(cls) -> List[str]:
        return ["x", "y", "width", "height"]


class ConcreteTriangle(Shape, Serializable):
    """Triangle implementing Shape and Serializable."""

    def __init__(
        self, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float
    ) -> None:
        super().__init__("Triangle")
        self.x1, self.y1 = x1, y1
        self.x2, self.y2 = x2, y2
        self.x3, self.y3 = x3, y3

    def _side_length(self, ax: float, ay: float, bx: float, by: float) -> float:
        return ((bx - ax) ** 2 + (by - ay) ** 2) ** 0.5

    def area(self) -> float:
        # Shoelace formula
        val = abs(
            self.x1 * (self.y2 - self.y3)
            + self.x2 * (self.y3 - self.y1)
            + self.x3 * (self.y1 - self.y2)
        )
        return val / 2.0

    def perimeter(self) -> float:
        a = self._side_length(self.x1, self.y1, self.x2, self.y2)
        b = self._side_length(self.x2, self.y2, self.x3, self.y3)
        c = self._side_length(self.x3, self.y3, self.x1, self.y1)
        return a + b + c

    def contains_point(self, px: float, py: float) -> bool:
        def sign(ax: float, ay: float, bx: float, by: float, cx: float, cy: float) -> float:
            return (ax - cx) * (by - cy) - (bx - cx) * (ay - cy)

        d1 = sign(px, py, self.x1, self.y1, self.x2, self.y2)
        d2 = sign(px, py, self.x2, self.y2, self.x3, self.y3)
        d3 = sign(px, py, self.x3, self.y3, self.x1, self.y1)

        has_neg = (d1 < 0) or (d2 < 0) or (d3 < 0)
        has_pos = (d1 > 0) or (d2 > 0) or (d3 > 0)
        return not (has_neg and has_pos)

    def to_dict(self) -> Dict[str, float]:
        return {
            "x1": self.x1, "y1": self.y1,
            "x2": self.x2, "y2": self.y2,
            "x3": self.x3, "y3": self.y3,
        }

    @classmethod
    def field_names(cls) -> List[str]:
        return ["x1", "y1", "x2", "y2", "x3", "y3"]

    def centroid(self) -> Tuple[float, float]:
        cx = (self.x1 + self.x2 + self.x3) / 3.0
        cy = (self.y1 + self.y2 + self.y3) / 3.0
        return (cx, cy)


class ShapeCollection:
    """Collection that works with abstract Shape references."""

    def __init__(self) -> None:
        self._shapes: List[Shape] = []

    def add(self, shape: Shape) -> None:
        self._shapes.append(shape)

    def total_area(self) -> float:
        return sum(s.area() for s in self._shapes)

    def total_perimeter(self) -> float:
        return sum(s.perimeter() for s in self._shapes)

    def shapes_containing(self, x: float, y: float) -> List[Shape]:
        return [s for s in self._shapes if s.contains_point(x, y)]

    def describe_all(self) -> List[str]:
        return [s.describe() for s in self._shapes]

    def count(self) -> int:
        return len(self._shapes)

    def largest_by_area(self) -> Optional[Shape]:
        if not self._shapes:
            return None
        best = self._shapes[0]
        for s in self._shapes[1:]:
            if s.area() > best.area():
                best = s
        return best


def compute_area_ratios(shapes: List[Shape]) -> List[float]:
    """Compute the ratio of each shape's area to total area."""
    total = sum(s.area() for s in shapes)
    if total == 0.0:
        return [0.0] * len(shapes)
    return [s.area() / total for s in shapes]


def serialize_collection(shapes: List[Serializable]) -> List[str]:
    """Serialize all shapes to CSV rows."""
    return [s.to_csv_row() for s in shapes]


# Untyped function 1: test inference on abstract class usage
def find_overlapping_shapes(shapes, test_points):
    results = {}
    for i, (px, py) in enumerate(test_points):
        containing = []
        for j, shape in enumerate(shapes):
            if shape.contains_point(px, py):
                containing.append(j)
        if len(containing) > 1:
            results[i] = containing
    return results


# Untyped function 2: test inference on polymorphic dispatch
def shape_summary_table(shapes):
    rows = []
    for shape in shapes:
        name = shape._name
        area = shape.area()
        perim = shape.perimeter()
        ratio = area / perim if perim > 0 else 0.0
        rows.append(f"{name:>12} | {area:8.2f} | {perim:8.2f} | {ratio:6.3f}")
    header = f"{'Shape':>12} | {'Area':>8} | {'Perim':>8} | {'Ratio':>6}"
    separator = "-" * len(header)
    return "\n".join([header, separator] + rows)


def render_all_shapes(
    shapes: List[Renderable], canvas_w: int, canvas_h: int
) -> List[str]:
    """Render all renderable shapes and concatenate output."""
    all_lines: List[str] = []
    for shape in shapes:
        lines = shape.render_ascii(canvas_w, canvas_h)
        all_lines.extend(lines)
        all_lines.append("")
    return all_lines


def transform_pipeline(
    shape: Transformable,
    translations: List[Tuple[float, float]],
    rotations: List[float],
) -> Transformable:
    """Apply a pipeline of translations and rotations."""
    current = shape
    for (dx, dy), angle in zip(translations, rotations):
        current = current.translate(dx, dy)
        current = current.rotate(angle)
    return current


def main() -> None:
    # Test concrete circle
    c = ConcreteCircle(0.0, 0.0, 5.0)
    assert abs(c.area() - 78.5398) < 0.01
    assert c.contains_point(0.0, 0.0)
    assert not c.contains_point(10.0, 10.0)

    # Test concrete rectangle
    r = ConcreteRectangle(0.0, 0.0, 10.0, 5.0)
    assert abs(r.area() - 50.0) < 0.01
    assert abs(r.perimeter() - 30.0) < 0.01

    # Test concrete triangle
    t = ConcreteTriangle(0.0, 0.0, 4.0, 0.0, 2.0, 3.0)
    assert abs(t.area() - 6.0) < 0.01
    cx, cy = t.centroid()
    assert abs(cx - 2.0) < 0.01

    # Test shape collection
    coll = ShapeCollection()
    coll.add(c)
    coll.add(r)
    coll.add(t)
    assert coll.count() == 3
    assert coll.total_area() > 0.0

    largest = coll.largest_by_area()
    assert largest is not None

    descs = coll.describe_all()
    assert len(descs) == 3

    # Test transformations
    c2 = c.translate(5.0, 5.0)
    assert abs(c2.cx - 5.0) < 0.01

    r2 = r.rotate(90.0)
    assert abs(r2.w - 5.0) < 0.01  # width and height swapped

    # Test rendering
    ascii_art = c.render_ascii(10, 10)
    assert len(ascii_art) == 10

    # Test serialization
    csv_rows = serialize_collection([c, r, t])
    assert len(csv_rows) == 3

    # Test area ratios
    ratios = compute_area_ratios([c, r, t])
    assert abs(sum(ratios) - 1.0) < 0.01

    # Test untyped functions
    overlaps = find_overlapping_shapes([c, r], [(1.0, 1.0), (100.0, 100.0)])
    assert isinstance(overlaps, dict)

    table = shape_summary_table([c, r, t])
    assert "Circle" in table

    # Test transform pipeline
    result = transform_pipeline(
        c,
        [(1.0, 2.0), (3.0, 4.0)],
        [0.0, 90.0],
    )
    assert result is not None


if __name__ == "__main__":
    main()
