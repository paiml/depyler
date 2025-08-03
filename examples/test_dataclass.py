"""Dataclass example for v1.2.0"""
from dataclasses import dataclass

@dataclass
class Point:
    """A 2D point dataclass"""
    x: int
    y: int
    
    def move_by(self, dx: int, dy: int):
        """Move point by delta"""
        self.x += dx
        self.y += dy
    
    def distance_to(self, other) -> float:
        """Calculate distance to another point"""
        dx = self.x - other.x
        dy = self.y - other.y
        return (dx * dx + dy * dy) ** 0.5
    
    @staticmethod
    def origin():
        """Create a point at origin"""
        return Point(0, 0)
    
    @classmethod
    def from_tuple(cls, coords: tuple[int, int]):
        """Create from tuple"""
        return cls(coords[0], coords[1])
    
    @property
    def magnitude(self) -> float:
        """Distance from origin"""
        return (self.x * self.x + self.y * self.y) ** 0.5

# Test function
def test_point():
    # Create points
    p1 = Point(0, 0)
    p2 = Point(3, 4)
    
    # Instance method
    p1.move_by(1, 1)
    
    # Static method
    origin = Point.origin()
    
    # Class method
    p3 = Point.from_tuple((5, 5))
    
    # Property access
    mag = p2.magnitude
    
    # Calculate distance
    dist = p1.distance_to(p2)
    
    return dist, mag