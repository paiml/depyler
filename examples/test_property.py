"""Test property decorator support"""

class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    
    @property
    def magnitude(self) -> int:
        """Get the magnitude of the point"""
        return self.x * self.x + self.y * self.y

def test_property():
    p = Point(3, 4)
    m = p.magnitude
    return m