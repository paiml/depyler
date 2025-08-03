"""Test with statement support for v1.3.0"""

class FileManager:
    """A simple file manager context manager"""
    
    def __init__(self, filename: str):
        self.filename = filename
        self.file = None
    
    def __enter__(self):
        """Enter the context"""
        # In real code, would open file
        self.file = self.filename
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Exit the context"""
        # In real code, would close file
        self.file = None
        return False
    
    def write(self, data: str):
        """Write data to file"""
        # In real code, would write to file
        return len(data)

def test_simple_with():
    """Test basic with statement"""
    with FileManager("test.txt") as fm:
        result = fm.write("Hello, World!")
    return result

def test_with_builtin():
    """Test with built-in open"""
    with open("test.txt", "w") as f:
        f.write("Hello, World!")
    return 1