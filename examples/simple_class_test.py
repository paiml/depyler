"""Test simple class support in Depyler"""

class Point:
    """A simple 2D point class"""
    x: int
    y: int
    
    def __init__(self, x: int, y: int):
        """Initialize a point with x and y coordinates"""
        pass
    
    def distance(self) -> int:
        """Calculate distance squared"""
        return 0


class Rectangle:
    """A rectangle defined by width and height"""
    width: int
    height: int
    
    def __init__(self, width: int, height: int):
        pass
    
    def area(self) -> int:
        """Calculate the area of the rectangle"""
        return 0