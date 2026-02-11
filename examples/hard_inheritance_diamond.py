"""Pathological diamond inheritance and MRO patterns for transpiler stress testing.

Tests diamond inheritance, Method Resolution Order, super() in multi-level
hierarchies, mixin classes, property override, and abstract-like patterns.
"""

from typing import List, Dict, Optional, Any


# --- Diamond inheritance base ---

class Shape:
    """Base class for all shapes."""

    def __init__(self, name: str = "shape"):
        self.name = name
        self._init_order: List[str] = ["Shape"]

    def area(self) -> float:
        return 0.0

    def perimeter(self) -> float:
        return 0.0

    def describe(self) -> str:
        return f"{self.name}: area={self.area():.2f}, perimeter={self.perimeter():.2f}"

    def get_init_order(self) -> List[str]:
        return list(self._init_order)


class Colorable(Shape):
    """Mixin that adds color to shapes - first arm of diamond."""

    def __init__(self, color: str = "black", **kwargs):
        super().__init__(**kwargs)
        self.color = color
        self._init_order.append("Colorable")

    def color_describe(self) -> str:
        return f"{self.color} {self.name}"


class Resizable(Shape):
    """Mixin that adds resize capability - second arm of diamond."""

    def __init__(self, scale: float = 1.0, **kwargs):
        super().__init__(**kwargs)
        self.scale = scale
        self._init_order.append("Resizable")

    def resize(self, factor: float):
        self.scale *= factor

    def effective_area(self) -> float:
        return self.area() * self.scale * self.scale


class ColoredResizableShape(Colorable, Resizable):
    """Diamond tip: inherits from both Colorable and Resizable, which both inherit Shape."""

    def __init__(self, name: str = "unknown", color: str = "black", scale: float = 1.0):
        super().__init__(name=name, color=color, scale=scale)
        self._init_order.append("ColoredResizableShape")

    def full_describe(self) -> str:
        return f"{self.color} {self.name} (scale={self.scale:.1f}, area={self.effective_area():.2f})"


# --- Concrete shapes with diamond inheritance ---

class Circle(ColoredResizableShape):
    """Circle that inherits the full diamond."""

    def __init__(self, radius: float, color: str = "red"):
        super().__init__(name="circle", color=color, scale=1.0)
        self.radius = radius
        self._init_order.append("Circle")

    def area(self) -> float:
        return 3.14159265358979 * self.radius * self.radius

    def perimeter(self) -> float:
        return 2.0 * 3.14159265358979 * self.radius


class Rectangle(ColoredResizableShape):
    """Rectangle with diamond inheritance."""

    def __init__(self, width: float, height: float, color: str = "blue"):
        super().__init__(name="rectangle", color=color, scale=1.0)
        self.width = width
        self.height = height
        self._init_order.append("Rectangle")

    def area(self) -> float:
        return self.width * self.height

    def perimeter(self) -> float:
        return 2.0 * (self.width + self.height)

    def is_square(self) -> bool:
        return abs(self.width - self.height) < 1e-9


# --- Multi-level hierarchy with super() ---

class Animal:
    """Base animal class."""

    def __init__(self, species: str):
        self.species = species
        self._sounds: List[str] = []

    def make_sound(self) -> str:
        return "..."

    def move(self) -> str:
        return "moves"

    def describe(self) -> str:
        return f"{self.species}: {self.make_sound()}, {self.move()}"


class Mammal(Animal):
    """Mammals are warm-blooded animals."""

    def __init__(self, species: str, legs: int = 4):
        super().__init__(species)
        self.legs = legs
        self.warm_blooded = True

    def move(self) -> str:
        return f"walks on {self.legs} legs"


class Swimmer:
    """Mixin for swimming ability."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.can_swim = True

    def swim(self) -> str:
        return "swims gracefully"


class Flyer:
    """Mixin for flying ability."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.can_fly = True
        self.wing_span: float = 0.0

    def fly(self) -> str:
        return f"flies with wingspan {self.wing_span:.1f}m"


class Dog(Swimmer, Mammal):
    """Dog: mammal that can swim."""

    def __init__(self, breed: str):
        super().__init__(species="dog", legs=4)
        self.breed = breed

    def make_sound(self) -> str:
        return "woof"

    def fetch(self) -> str:
        return f"{self.breed} fetches the ball"


class Bat(Flyer, Mammal):
    """Bat: mammal that can fly."""

    def __init__(self):
        super().__init__(species="bat", legs=2)
        self.wing_span = 0.3

    def make_sound(self) -> str:
        return "screech"

    def move(self) -> str:
        return self.fly()


class Duck(Swimmer, Flyer, Animal):
    """Duck: animal that can both swim and fly - triple inheritance."""

    def __init__(self):
        super().__init__(species="duck")
        self.wing_span = 0.8

    def make_sound(self) -> str:
        return "quack"

    def move(self) -> str:
        return "waddles, swims, or flies"


# --- Property inheritance and override ---

class BaseConfig:
    """Base configuration with properties."""

    def __init__(self):
        self._values: Dict[str, Any] = {}
        self._defaults: Dict[str, Any] = {"timeout": 30, "retries": 3, "verbose": False}

    @property
    def timeout(self) -> int:
        return self._values.get("timeout", self._defaults["timeout"])

    @timeout.setter
    def timeout(self, value: int):
        if value < 0:
            raise ValueError("timeout must be non-negative")
        self._values["timeout"] = value

    @property
    def retries(self) -> int:
        return self._values.get("retries", self._defaults["retries"])

    @retries.setter
    def retries(self, value: int):
        self._values["retries"] = max(0, value)

    def get(self, key: str, default: Any = None) -> Any:
        return self._values.get(key, self._defaults.get(key, default))

    def set(self, key: str, value: Any):
        self._values[key] = value

    def keys(self) -> List[str]:
        all_keys = set(self._defaults.keys()) | set(self._values.keys())
        return sorted(all_keys)


class ServerConfig(BaseConfig):
    """Server config that extends base config with additional properties."""

    def __init__(self, host: str = "localhost", port: int = 8080):
        super().__init__()
        self._values["host"] = host
        self._values["port"] = port
        self._defaults["max_connections"] = 100

    @property
    def host(self) -> str:
        return self._values.get("host", "localhost")

    @property
    def port(self) -> int:
        return self._values.get("port", 8080)

    @property
    def timeout(self) -> int:
        """Override: server timeout defaults to 60."""
        return self._values.get("timeout", 60)

    @timeout.setter
    def timeout(self, value: int):
        if value < 1:
            raise ValueError("server timeout must be at least 1")
        self._values["timeout"] = value

    def address(self) -> str:
        return f"{self.host}:{self.port}"


class DebugServerConfig(ServerConfig):
    """Debug config that further extends server config."""

    def __init__(self):
        super().__init__(host="127.0.0.1", port=9090)
        self._values["verbose"] = True
        self._defaults["log_level"] = "DEBUG"

    @property
    def timeout(self) -> int:
        """Override: debug server timeout is always 120."""
        return self._values.get("timeout", 120)

    @timeout.setter
    def timeout(self, value: int):
        self._values["timeout"] = value


# --- Untyped functions ---

def collect_mro(cls):
    """Return the MRO of a class as a list of names - untyped."""
    return [c.__name__ for c in cls.__mro__]


def find_common_ancestor(cls1, cls2):
    """Find the first common ancestor of two classes - untyped."""
    mro1 = cls1.__mro__
    mro2_set = set(cls2.__mro__)
    for cls in mro1:
        if cls in mro2_set and cls is not object:
            return cls.__name__
    return "object"


def shape_summary(shapes):
    """Generate a summary of shapes - untyped."""
    total_area = 0.0
    summaries = []
    for shape in shapes:
        a = shape.area()
        total_area += a
        summaries.append(f"{shape.name}: {a:.2f}")
    summaries.append(f"total: {total_area:.2f}")
    return summaries


def animal_census(animals):
    """Count animals by species - untyped."""
    census = {}
    for animal in animals:
        species = animal.species
        if species in census:
            census[species] += 1
        else:
            census[species] = 1
    return census


def merge_configs(*configs):
    """Merge multiple configs, later ones override earlier - untyped."""
    result = {}
    for config in configs:
        for key in config.keys():
            result[key] = config.get(key)
    return result


# --- Typed test functions ---

def test_diamond_inheritance():
    """Test diamond inheritance MRO and method resolution."""
    c = Circle(5.0, color="red")
    assert abs(c.area() - 78.5398) < 0.01
    assert c.color == "red"
    assert c.name == "circle"

    # Test resize from Resizable (through diamond)
    c.resize(2.0)
    assert abs(c.scale - 2.0) < 1e-9
    assert abs(c.effective_area() - c.area() * 4.0) < 0.01

    # Test that init_order shows the diamond was traversed
    order = c.get_init_order()
    assert "Shape" in order
    assert "Colorable" in order
    assert "Resizable" in order
    assert "Circle" in order
    return True


def test_mro_resolution():
    """Test Method Resolution Order is correct."""
    mro = collect_mro(Circle)
    assert mro[0] == "Circle"
    assert "ColoredResizableShape" in mro
    assert "Colorable" in mro
    assert "Resizable" in mro
    assert "Shape" in mro
    assert mro[-1] == "object"

    # Shape should appear exactly once in MRO (diamond resolved)
    shape_count = sum(1 for c in mro if c == "Shape")
    assert shape_count == 1
    return True


def test_multiple_inheritance_animals():
    """Test multi-level inheritance with mixins."""
    dog = Dog("Labrador")
    assert dog.make_sound() == "woof"
    assert "walks on 4 legs" in dog.move()
    assert dog.can_swim
    assert dog.swim() == "swims gracefully"
    assert dog.fetch() == "Labrador fetches the ball"

    bat = Bat()
    assert bat.make_sound() == "screech"
    assert "flies" in bat.move()
    assert bat.can_fly

    duck = Duck()
    assert duck.make_sound() == "quack"
    assert duck.can_swim
    assert duck.can_fly
    return True


def test_common_ancestor():
    """Test finding common ancestors."""
    assert find_common_ancestor(Dog, Bat) == "Mammal"
    assert find_common_ancestor(Circle, Rectangle) == "ColoredResizableShape"
    assert find_common_ancestor(Dog, Duck) == "Swimmer"
    return True


def test_property_inheritance():
    """Test property override across inheritance levels."""
    base = BaseConfig()
    assert base.timeout == 30
    assert base.retries == 3

    server = ServerConfig("example.com", 443)
    assert server.timeout == 60  # Overridden default
    assert server.host == "example.com"
    assert server.port == 443
    assert server.address() == "example.com:443"

    debug = DebugServerConfig()
    assert debug.timeout == 120  # Further overridden
    assert debug.host == "127.0.0.1"
    assert debug.port == 9090
    assert debug.get("verbose") == True
    return True


def test_config_mutation():
    """Test config setter inheritance."""
    server = ServerConfig()
    server.timeout = 90
    assert server.timeout == 90

    # Server requires timeout >= 1
    raised = False
    try:
        server.timeout = 0
    except ValueError:
        raised = True
    assert raised

    debug = DebugServerConfig()
    debug.timeout = 5  # Debug allows any value
    assert debug.timeout == 5
    return True


def test_rectangle_diamond():
    """Test Rectangle with diamond inheritance."""
    r = Rectangle(4.0, 6.0, color="green")
    assert abs(r.area() - 24.0) < 1e-9
    assert abs(r.perimeter() - 20.0) < 1e-9
    assert r.color == "green"
    assert not r.is_square()

    sq = Rectangle(5.0, 5.0)
    assert sq.is_square()

    r.resize(0.5)
    assert abs(r.effective_area() - 6.0) < 1e-9  # 24 * 0.25
    return True


def test_shape_summary():
    """Test untyped shape summary function."""
    shapes = [
        Circle(1.0),
        Rectangle(3.0, 4.0),
        Circle(2.0),
    ]
    summary = shape_summary(shapes)
    assert len(summary) == 4  # 3 shapes + total
    assert "total:" in summary[-1]
    return True


def test_animal_census():
    """Test untyped animal census."""
    animals = [Dog("Lab"), Dog("Poodle"), Bat(), Duck(), Duck()]
    census = animal_census(animals)
    assert census["dog"] == 2
    assert census["bat"] == 1
    assert census["duck"] == 2
    return True


def test_config_merge():
    """Test untyped config merge."""
    c1 = BaseConfig()
    c1.set("custom_key", "value1")
    c2 = ServerConfig("prod.com", 443)
    merged = merge_configs(c1, c2)
    assert merged["host"] == "prod.com"
    assert merged["port"] == 443
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_diamond_inheritance()
    assert test_mro_resolution()
    assert test_multiple_inheritance_animals()
    assert test_common_ancestor()
    assert test_property_inheritance()
    assert test_config_mutation()
    assert test_rectangle_diamond()
    assert test_shape_summary()
    assert test_animal_census()
    assert test_config_merge()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_inheritance_diamond: ALL TESTS PASSED")
    else:
        print("hard_inheritance_diamond: TESTS FAILED")


if __name__ == "__main__":
    main()
