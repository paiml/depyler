# @depyler: optimization_level = "size"
# @depyler: bounds_checking = "explicit"
from typing import Tuple

class Point:
    """2D point representation"""
    
    def __init__(self, x: float, y: float) -> None:
        self.x = x
        self.y = y
    
    def distance_to(self, other: 'Point') -> float:
        """Calculate Euclidean distance to another point"""
        dx = self.x - other.x
        dy = self.y - other.y
        distance_squared = dx * dx + dy * dy
        
        # Newton's method for square root
        if distance_squared == 0.0:
            return 0.0
        
        result = distance_squared / 2.0
        for _ in range(10):
            result = (result + distance_squared / result) / 2.0
        
        return result

class Rectangle:
    """Rectangle representation"""
    
    def __init__(self, width: float, height: float) -> None:
        self.width = width
        self.height = height
    
    def area(self) -> float:
        """Calculate rectangle area"""
        return self.width * self.height
    
    def perimeter(self) -> float:
        """Calculate rectangle perimeter"""
        return 2.0 * (self.width + self.height)
    
    def is_square(self) -> bool:
        """Check if rectangle is a square"""
        return abs(self.width - self.height) < 0.0001

class Circle:
    """Circle representation"""
    
    def __init__(self, radius: float) -> None:
        self.radius = radius
    
    def area(self) -> float:
        """Calculate circle area (using π ≈ 3.14159)"""
        pi = 3.14159
        return pi * self.radius * self.radius
    
    def circumference(self) -> float:
        """Calculate circle circumference"""
        pi = 3.14159
        return 2.0 * pi * self.radius
    
    def contains_point(self, point: Point) -> bool:
        """Check if point is inside circle (assuming circle centered at origin)"""
        distance_squared = point.x * point.x + point.y * point.y
        radius_squared = self.radius * self.radius
        return distance_squared <= radius_squared

def triangle_area(base: float, height: float) -> float:
    """Calculate triangle area"""
    return 0.5 * base * height

def triangle_area_heron(a: float, b: float, c: float) -> float:
    """Calculate triangle area using Heron's formula"""
    # Check if valid triangle
    if a + b <= c or a + c <= b or b + c <= a:
        return 0.0
    
    s = (a + b + c) / 2.0  # semi-perimeter
    area_squared = s * (s - a) * (s - b) * (s - c)
    
    # Newton's method for square root
    if area_squared <= 0.0:
        return 0.0
    
    result = area_squared / 2.0
    for _ in range(10):
        result = (result + area_squared / result) / 2.0
    
    return result

def line_intersection(p1: Point, p2: Point, p3: Point, p4: Point) -> Tuple[bool, Point]:
    """Find intersection of two lines defined by point pairs"""
    # Line 1: (p1, p2), Line 2: (p3, p4)
    
    x1, y1 = p1.x, p1.y
    x2, y2 = p2.x, p2.y
    x3, y3 = p3.x, p3.y
    x4, y4 = p4.x, p4.y
    
    denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
    
    if abs(denominator) < 0.0001:  # Lines are parallel
        return False, Point(0.0, 0.0)
    
    t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator
    
    intersection_x = x1 + t * (x2 - x1)
    intersection_y = y1 + t * (y2 - y1)
    
    return True, Point(intersection_x, intersection_y)