"""Depyler v1.2.0 Basic OOP Features

Demonstrates working OOP features in v1.2.0
"""

class Point:
    """A 2D point with methods"""
    
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    
    def translate(self, dx: int, dy: int):
        """Move the point by dx, dy"""
        self.x += dx
        self.y += dy
    
    def distance_squared(self) -> int:
        """Get squared distance from origin"""
        return self.x * self.x + self.y * self.y
    
    @staticmethod
    def origin() -> int:
        """Return a point at origin (simplified for now)"""
        # Would return Point(0, 0) but constructors need work
        return 0

class Rectangle:
    """A rectangle defined by width and height"""
    
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
    
    def area(self) -> int:
        """Calculate rectangle area"""
        return self.width * self.height
    
    def perimeter(self) -> int:
        """Calculate rectangle perimeter"""
        return 2 * (self.width + self.height)
    
    def is_square(self) -> bool:
        """Check if rectangle is a square"""
        return self.width == self.height

def test_point():
    """Test Point class"""
    p = Point(3, 4)
    p.translate(1, 2)
    dist_sq = p.distance_squared()
    return dist_sq

def test_rectangle():
    """Test Rectangle class"""
    rect = Rectangle(10, 20)
    area = rect.area()
    perim = rect.perimeter()
    square = rect.is_square()
    
    # Create a square
    sq = Rectangle(5, 5)
    is_sq = sq.is_square()
    
    return area + perim

def test_static():
    """Test static methods"""
    zero = Point.origin()
    return zero

def main():
    """Run all tests"""
    point_result = test_point()
    rect_result = test_rectangle()
    static_result = test_static()
    
    return point_result + rect_result + static_result