"""Test basic class support in Depyler"""

class Point:
    """A simple 2D point class"""
    
    def __init__(self, x: int, y: int):
        """Initialize a point with x and y coordinates"""
        self.x = x
        self.y = y
    
    def distance_from_origin(self) -> float:
        """Calculate distance from origin"""
        return (self.x * self.x + self.y * self.y) ** 0.5
    
    def translate(self, dx: int, dy: int):
        """Translate the point by dx and dy"""
        self.x = self.x + dx
        self.y = self.y + dy


class Rectangle:
    """A rectangle defined by width and height"""
    
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
    
    def area(self) -> int:
        """Calculate the area of the rectangle"""
        return self.width * self.height
    
    def perimeter(self) -> int:
        """Calculate the perimeter of the rectangle"""
        return 2 * (self.width + self.height)
    
    def is_square(self) -> bool:
        """Check if the rectangle is a square"""
        return self.width == self.height


# Dataclass example
from dataclasses import dataclass

@dataclass
class Person:
    """A person with name and age"""
    name: str
    age: int
    
    def greet(self) -> str:
        """Return a greeting message"""
        return f"Hello, my name is {self.name}"
    
    def is_adult(self) -> bool:
        """Check if person is an adult"""
        return self.age >= 18


def test_point():
    """Test Point class functionality"""
    p = Point(3, 4)
    assert p.x == 3
    assert p.y == 4
    assert p.distance_from_origin() == 5.0
    
    p.translate(1, 1)
    assert p.x == 4
    assert p.y == 5


def test_rectangle():
    """Test Rectangle class functionality"""
    r = Rectangle(10, 20)
    assert r.area() == 200
    assert r.perimeter() == 60
    assert not r.is_square()
    
    square = Rectangle(15, 15)
    assert square.is_square()


def test_person():
    """Test Person dataclass functionality"""
    p = Person("Alice", 25)
    assert p.name == "Alice"
    assert p.age == 25
    assert p.is_adult()
    assert p.greet() == "Hello, my name is Alice"
    
    child = Person("Bob", 10)
    assert not child.is_adult()