"""Simple class example for v1.2.0"""

class Point:
    """A 2D point class"""
    
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    
    def move_by(self, dx: int, dy: int):
        """Move point by delta"""
        self.x += dx
        self.y += dy
    
    def distance_to(self, other) -> float:
        """Calculate distance to another point"""
        dx = self.x - other.x
        dy = self.y - other.y
        return (dx * dx + dy * dy) ** 0.5

# Test function
def test_point():
    p1 = Point(0, 0)
    p2 = Point(3, 4)
    
    # Move first point
    p1.move_by(1, 1)
    
    # Calculate distance
    dist = p1.distance_to(p2)
    
    return dist