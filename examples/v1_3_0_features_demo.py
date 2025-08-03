"""Depyler v1.3.0 Advanced Type System Features Demo

This demonstrates the features implemented so far in v1.3.0
"""

# 1. With Statement Support
class ResourceManager:
    """A simple resource manager for with statements"""
    
    def __init__(self, name: str):
        self.name = name
        self.is_open = False
    
    def __enter__(self):
        """Enter the context"""
        self.is_open = True
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Exit the context"""
        self.is_open = False
        return False
    
    def use_resource(self) -> int:
        """Use the resource"""
        if self.is_open:
            return 42
        return 0

def demo_with_statement():
    """Demonstrate with statement support"""
    with ResourceManager("test") as rm:
        result = rm.use_resource()
    return result

# 2. Iterator Protocol Support
class Counter:
    """A simple counter iterator"""
    
    def __init__(self, max_count: int):
        self.max_count = max_count
        self.count = 0
    
    def __iter__(self):
        """Return self as iterator"""
        return self
    
    def __next__(self) -> int:
        """Get next value"""
        if self.count < self.max_count:
            self.count += 1
            return self.count
        return -1  # Simplified StopIteration

def demo_iterator():
    """Demonstrate iterator protocol"""
    counter = Counter(3)
    
    # Manual iteration (for loops with custom iterators need more work)
    total = 0
    val = counter.__next__()
    while val != -1:
        total += val
        val = counter.__next__()
    
    return total

# Main demo function
def main():
    """Run all v1.3.0 feature demos"""
    with_result = demo_with_statement()
    iter_result = demo_iterator()
    
    return with_result + iter_result