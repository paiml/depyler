def demo_function_kwargs():
    """Test function calls with keyword arguments"""
    result1 = greet(name="Alice", greeting="Hello")
    result2 = calculate(10, 20, operation="add", verbose=True)    
    result3 = configure(width=800, height=600, title="My App")
    
    return result1, result2, result3

def demo_method_kwargs():
    """Test method calls with keyword arguments"""
    obj = MyObject()
    obj.setup(mode="advanced", timeout=30, retry=True)
    
    text = "hello world"
    formatted = text.replace("world", "Python", count=1)
    
    return formatted

def demo_builtin_kwargs():
    """Test builtin functions with keyword arguments"""
    f = open("data.txt", mode="r", encoding="utf-8")
    numbers = [3, 1, 4, 1, 5, 9, 2, 6]
    sorted_desc = sorted(numbers, reverse=True)
    config = dict(host="localhost", port=8080, debug=True)
    
    return config

def demo_nested_kwargs():
    """Test nested function calls with kwargs"""
    result = outer(
        inner(x=10, y=20),
        scale=2.0,
        offset=inner(x=5, y=5)
    )
    
    return result

def demo_complex_kwargs():
    """Test kwargs with complex expressions"""
    settings = configure(
        width=100 + 200,
        height=get_height(),
        enabled=True and not False,
        title="App " + str(42)
    )
    
    return settings

def greet(name: str, greeting: str = "Hi") -> str:
    return f"{greeting}, {name}!"

def calculate(a: int, b: int, operation: str = "add", verbose: bool = False) -> int:
    if operation == "add":
        result = a + b
    else:
        result = a - b
    
    if verbose:
        print(f"Result: {result}")
    
    return result

def configure(width: int = 640, height: int = 480, title: str = "Window") -> dict:
    return {"width": width, "height": height, "title": title}

class MyObject:
    def setup(self, mode: str = "basic", timeout: int = 10, retry: bool = False):
        self.mode = mode
        self.timeout = timeout
        self.retry = retry

def outer(inner_result, scale: float = 1.0, offset = None):
    return inner_result * scale + (offset if offset else 0)

def inner(x: int, y: int) -> int:
    return x + y

def get_height() -> int:
    return 600
