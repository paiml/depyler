"""Hard property/descriptor patterns: @property, setter, deleter, cached_property, __slots__."""

from typing import List, Dict, Optional, Tuple
from functools import cached_property
import math


class Temperature:
    """Temperature with Celsius/Fahrenheit conversion via properties."""

    def __init__(self, celsius: float) -> None:
        self._celsius: float = celsius
        self._history: List[float] = [celsius]

    @property
    def celsius(self) -> float:
        return self._celsius

    @celsius.setter
    def celsius(self, value: float) -> None:
        if value < -273.15:
            raise ValueError("Temperature below absolute zero")
        self._celsius = value
        self._history.append(value)

    @property
    def fahrenheit(self) -> float:
        return self._celsius * 9.0 / 5.0 + 32.0

    @fahrenheit.setter
    def fahrenheit(self, value: float) -> None:
        self.celsius = (value - 32.0) * 5.0 / 9.0

    @property
    def kelvin(self) -> float:
        return self._celsius + 273.15

    @kelvin.setter
    def kelvin(self, value: float) -> None:
        self.celsius = value - 273.15

    @property
    def history(self) -> List[float]:
        return list(self._history)

    @property
    def history_range(self) -> Tuple[float, float]:
        return (min(self._history), max(self._history))

    def __repr__(self) -> str:
        return f"Temperature({self._celsius:.2f}C)"


class Vector2D:
    """2D vector with computed properties for magnitude and angle."""

    __slots__ = ("_x", "_y")

    def __init__(self, x: float, y: float) -> None:
        self._x = x
        self._y = y

    @property
    def x(self) -> float:
        return self._x

    @x.setter
    def x(self, value: float) -> None:
        self._x = value

    @property
    def y(self) -> float:
        return self._y

    @y.setter
    def y(self, value: float) -> None:
        self._y = value

    @property
    def magnitude(self) -> float:
        return math.sqrt(self._x * self._x + self._y * self._y)

    @property
    def angle(self) -> float:
        return math.atan2(self._y, self._x)

    @property
    def angle_degrees(self) -> float:
        return math.degrees(self.angle)

    @property
    def unit(self) -> "Vector2D":
        mag = self.magnitude
        if mag == 0.0:
            return Vector2D(0.0, 0.0)
        return Vector2D(self._x / mag, self._y / mag)

    def dot(self, other: "Vector2D") -> float:
        return self._x * other._x + self._y * other._y

    def add(self, other: "Vector2D") -> "Vector2D":
        return Vector2D(self._x + other._x, self._y + other._y)

    def scale(self, factor: float) -> "Vector2D":
        return Vector2D(self._x * factor, self._y * factor)

    def rotate(self, angle_rad: float) -> "Vector2D":
        cos_a = math.cos(angle_rad)
        sin_a = math.sin(angle_rad)
        new_x = self._x * cos_a - self._y * sin_a
        new_y = self._x * sin_a + self._y * cos_a
        return Vector2D(new_x, new_y)

    def __repr__(self) -> str:
        return f"Vector2D({self._x:.3f}, {self._y:.3f})"


class BankAccount:
    """Bank account with property-based validation and computed fields."""

    def __init__(self, owner: str, initial_balance: float = 0.0) -> None:
        self._owner = owner
        self._balance = max(0.0, initial_balance)
        self._transactions: List[Tuple[str, float]] = []
        self._frozen = False
        self._overdraft_limit = 0.0

    @property
    def owner(self) -> str:
        return self._owner

    @property
    def balance(self) -> float:
        return self._balance

    @property
    def is_frozen(self) -> bool:
        return self._frozen

    @is_frozen.setter
    def is_frozen(self, value: bool) -> None:
        self._frozen = value

    @property
    def overdraft_limit(self) -> float:
        return self._overdraft_limit

    @overdraft_limit.setter
    def overdraft_limit(self, value: float) -> None:
        self._overdraft_limit = max(0.0, value)

    @property
    def available_balance(self) -> float:
        return self._balance + self._overdraft_limit

    @property
    def transaction_count(self) -> int:
        return len(self._transactions)

    @property
    def transaction_summary(self) -> Dict[str, float]:
        deposits = sum(amt for kind, amt in self._transactions if kind == "deposit")
        withdrawals = sum(amt for kind, amt in self._transactions if kind == "withdraw")
        return {"deposits": deposits, "withdrawals": withdrawals, "net": deposits - withdrawals}

    def deposit(self, amount: float) -> bool:
        if self._frozen or amount <= 0:
            return False
        self._balance += amount
        self._transactions.append(("deposit", amount))
        return True

    def withdraw(self, amount: float) -> bool:
        if self._frozen or amount <= 0:
            return False
        if amount > self.available_balance:
            return False
        self._balance -= amount
        self._transactions.append(("withdraw", amount))
        return True


class Rectangle:
    """Rectangle with interdependent properties."""

    def __init__(self, x: float, y: float, width: float, height: float) -> None:
        self._x = x
        self._y = y
        self._width = max(0.0, width)
        self._height = max(0.0, height)

    @property
    def x(self) -> float:
        return self._x

    @x.setter
    def x(self, value: float) -> None:
        self._x = value

    @property
    def y(self) -> float:
        return self._y

    @y.setter
    def y(self, value: float) -> None:
        self._y = value

    @property
    def width(self) -> float:
        return self._width

    @width.setter
    def width(self, value: float) -> None:
        self._width = max(0.0, value)

    @property
    def height(self) -> float:
        return self._height

    @height.setter
    def height(self, value: float) -> None:
        self._height = max(0.0, value)

    @property
    def area(self) -> float:
        return self._width * self._height

    @property
    def perimeter(self) -> float:
        return 2.0 * (self._width + self._height)

    @property
    def center(self) -> Tuple[float, float]:
        return (self._x + self._width / 2.0, self._y + self._height / 2.0)

    @center.setter
    def center(self, pos: Tuple[float, float]) -> None:
        cx, cy = pos
        self._x = cx - self._width / 2.0
        self._y = cy - self._height / 2.0

    @property
    def top_right(self) -> Tuple[float, float]:
        return (self._x + self._width, self._y + self._height)

    @property
    def diagonal(self) -> float:
        return math.sqrt(self._width ** 2 + self._height ** 2)

    @property
    def aspect_ratio(self) -> float:
        if self._height == 0.0:
            return 0.0
        return self._width / self._height

    def contains_point(self, px: float, py: float) -> bool:
        return (
            self._x <= px <= self._x + self._width
            and self._y <= py <= self._y + self._height
        )

    def intersects(self, other: "Rectangle") -> bool:
        return not (
            self._x + self._width < other._x
            or other._x + other._width < self._x
            or self._y + self._height < other._y
            or other._y + other._height < self._y
        )


class DataSeries:
    """Time series data with cached computed properties."""

    def __init__(self, name: str, values: List[float]) -> None:
        self._name = name
        self._values = list(values)

    @property
    def name(self) -> str:
        return self._name

    @property
    def values(self) -> List[float]:
        return list(self._values)

    @property
    def length(self) -> int:
        return len(self._values)

    @cached_property
    def mean(self) -> float:
        if not self._values:
            return 0.0
        return sum(self._values) / len(self._values)

    @cached_property
    def variance(self) -> float:
        if len(self._values) < 2:
            return 0.0
        m = self.mean
        return sum((v - m) ** 2 for v in self._values) / (len(self._values) - 1)

    @cached_property
    def std_dev(self) -> float:
        return math.sqrt(self.variance)

    @cached_property
    def sorted_values(self) -> List[float]:
        return sorted(self._values)

    @cached_property
    def median(self) -> float:
        sv = self.sorted_values
        n = len(sv)
        if n == 0:
            return 0.0
        if n % 2 == 1:
            return sv[n // 2]
        return (sv[n // 2 - 1] + sv[n // 2]) / 2.0

    @cached_property
    def min_max(self) -> Tuple[float, float]:
        if not self._values:
            return (0.0, 0.0)
        return (min(self._values), max(self._values))

    @cached_property
    def range_span(self) -> float:
        lo, hi = self.min_max
        return hi - lo

    def percentile(self, p: float) -> float:
        if not self._values or p < 0.0 or p > 100.0:
            return 0.0
        sv = self.sorted_values
        idx = (p / 100.0) * (len(sv) - 1)
        lower = int(idx)
        upper = min(lower + 1, len(sv) - 1)
        frac = idx - lower
        return sv[lower] * (1.0 - frac) + sv[upper] * frac

    def z_scores(self) -> List[float]:
        if self.std_dev == 0.0:
            return [0.0] * len(self._values)
        return [(v - self.mean) / self.std_dev for v in self._values]


class Config:
    """Configuration with property-based access to nested settings."""

    def __init__(self) -> None:
        self._settings: Dict[str, str] = {}
        self._defaults: Dict[str, str] = {
            "timeout": "30",
            "retries": "3",
            "verbose": "false",
            "log_level": "INFO",
        }

    def set(self, key: str, value: str) -> None:
        self._settings[key] = value

    def get(self, key: str) -> Optional[str]:
        if key in self._settings:
            return self._settings[key]
        return self._defaults.get(key)

    @property
    def timeout(self) -> int:
        val = self.get("timeout")
        return int(val) if val else 30

    @timeout.setter
    def timeout(self, value: int) -> None:
        self.set("timeout", str(max(1, value)))

    @property
    def retries(self) -> int:
        val = self.get("retries")
        return int(val) if val else 3

    @retries.setter
    def retries(self, value: int) -> None:
        self.set("retries", str(max(0, value)))

    @property
    def verbose(self) -> bool:
        val = self.get("verbose")
        return val == "true" if val else False

    @verbose.setter
    def verbose(self, value: bool) -> None:
        self.set("verbose", "true" if value else "false")

    @property
    def log_level(self) -> str:
        val = self.get("log_level")
        return val if val else "INFO"

    @log_level.setter
    def log_level(self, value: str) -> None:
        allowed = {"DEBUG", "INFO", "WARNING", "ERROR"}
        if value in allowed:
            self.set("log_level", value)

    @property
    def all_settings(self) -> Dict[str, str]:
        merged = dict(self._defaults)
        merged.update(self._settings)
        return merged


# Untyped function 1: test inference on property access patterns
def analyze_temperatures(temp_readings):
    results = []
    for t in temp_readings:
        c = t.celsius
        f = t.fahrenheit
        k = t.kelvin
        results.append({
            "celsius": c,
            "fahrenheit": f,
            "kelvin": k,
            "hot": c > 30.0,
        })
    return results


# Untyped function 2: test inference on computed property chains
def compare_rectangles(rect_a, rect_b):
    overlap = rect_a.intersects(rect_b)
    area_ratio = rect_a.area / rect_b.area if rect_b.area > 0 else 0.0
    center_a = rect_a.center
    center_b = rect_b.center
    dist = ((center_a[0] - center_b[0]) ** 2 + (center_a[1] - center_b[1]) ** 2) ** 0.5
    return {
        "overlap": overlap,
        "area_ratio": area_ratio,
        "center_distance": dist,
        "aspect_a": rect_a.aspect_ratio,
        "aspect_b": rect_b.aspect_ratio,
    }


def run_vector_operations(vectors: List[Vector2D]) -> Dict[str, float]:
    """Aggregate vector properties."""
    total_magnitude = sum(v.magnitude for v in vectors)
    avg_angle = sum(v.angle_degrees for v in vectors) / max(len(vectors), 1)
    max_mag = max((v.magnitude for v in vectors), default=0.0)
    return {
        "total_magnitude": total_magnitude,
        "avg_angle": avg_angle,
        "max_magnitude": max_mag,
        "count": float(len(vectors)),
    }


def series_correlation(a: DataSeries, b: DataSeries) -> float:
    """Compute Pearson correlation between two data series."""
    if a.length != b.length or a.length == 0:
        return 0.0
    za = a.z_scores()
    zb = b.z_scores()
    n = a.length
    return sum(x * y for x, y in zip(za, zb)) / max(n - 1, 1)


def main() -> None:
    # Test Temperature properties
    t = Temperature(100.0)
    assert abs(t.fahrenheit - 212.0) < 0.01
    assert abs(t.kelvin - 373.15) < 0.01
    t.fahrenheit = 32.0
    assert abs(t.celsius - 0.0) < 0.01
    assert len(t.history) == 2

    # Test Vector2D with __slots__
    v = Vector2D(3.0, 4.0)
    assert abs(v.magnitude - 5.0) < 0.01
    unit = v.unit
    assert abs(unit.magnitude - 1.0) < 0.01

    v2 = Vector2D(1.0, 0.0)
    dot = v.dot(v2)
    assert abs(dot - 3.0) < 0.01

    rotated = v2.rotate(math.pi / 2)
    assert abs(rotated.x) < 0.01
    assert abs(rotated.y - 1.0) < 0.01

    # Test BankAccount
    acc = BankAccount("Alice", 1000.0)
    assert acc.balance == 1000.0
    assert acc.deposit(500.0)
    assert acc.balance == 1500.0
    assert acc.withdraw(200.0)
    assert acc.balance == 1300.0

    acc.overdraft_limit = 500.0
    assert acc.available_balance == 1800.0

    summary = acc.transaction_summary
    assert summary["deposits"] == 500.0
    assert summary["withdrawals"] == 200.0

    acc.is_frozen = True
    assert not acc.deposit(100.0)

    # Test Rectangle properties
    r = Rectangle(0.0, 0.0, 10.0, 5.0)
    assert abs(r.area - 50.0) < 0.01
    assert abs(r.perimeter - 30.0) < 0.01
    assert abs(r.diagonal - math.sqrt(125)) < 0.01

    cx, cy = r.center
    assert abs(cx - 5.0) < 0.01
    r.center = (10.0, 10.0)
    assert abs(r.x - 5.0) < 0.01

    r2 = Rectangle(8.0, 3.0, 5.0, 5.0)
    assert r.intersects(r2)

    # Test DataSeries with cached_property
    ds = DataSeries("test", [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0])
    assert abs(ds.mean - 5.0) < 0.01
    assert ds.std_dev > 0.0
    assert ds.median == 4.5
    lo, hi = ds.min_max
    assert lo == 2.0 and hi == 9.0
    assert abs(ds.range_span - 7.0) < 0.01

    p50 = ds.percentile(50.0)
    assert abs(p50 - ds.median) < 0.5

    zscores = ds.z_scores()
    assert len(zscores) == 8

    # Test Config
    cfg = Config()
    assert cfg.timeout == 30
    cfg.timeout = 60
    assert cfg.timeout == 60
    cfg.verbose = True
    assert cfg.verbose is True
    cfg.log_level = "DEBUG"
    assert cfg.log_level == "DEBUG"
    all_s = cfg.all_settings
    assert "timeout" in all_s

    # Test untyped functions
    temps = [Temperature(0.0), Temperature(100.0), Temperature(37.0)]
    analysis = analyze_temperatures(temps)
    assert len(analysis) == 3
    assert analysis[1]["fahrenheit"] == 212.0

    comparison = compare_rectangles(r, r2)
    assert comparison["overlap"] is True

    # Test vector operations
    vectors = [Vector2D(1.0, 0.0), Vector2D(0.0, 1.0), Vector2D(1.0, 1.0)]
    vec_stats = run_vector_operations(vectors)
    assert vec_stats["count"] == 3.0

    # Test series correlation
    s1 = DataSeries("a", [1.0, 2.0, 3.0, 4.0, 5.0])
    s2 = DataSeries("b", [2.0, 4.0, 6.0, 8.0, 10.0])
    corr = series_correlation(s1, s2)
    assert abs(corr - 1.0) < 0.01


if __name__ == "__main__":
    main()
