#!/usr/bin/env python3
"""
Example: AST Converters Demonstration

This example shows various Python constructs that Depyler's AST converters
can transform from Python AST to HIR (High-level Intermediate Representation).
"""

# Expression Examples

# Literals
int_literal = 42
float_literal = 3.14
string_literal = "hello world"
bool_literal = True
none_literal = None

# Binary operations
addition = 10 + 20
subtraction = 50 - 15
multiplication = 7 * 8
division = 100 / 4
modulo = 17 % 5
power = 2 ** 8

# Comparison operations
greater_than = 10 > 5
less_than = 3 < 7
equal_to = 42 == 42
not_equal = "a" != "b"
greater_equal = 100 >= 100
less_equal = 50 <= 60

# Logical operations
and_op = True and False
or_op = True or False
not_op = not True

# Unary operations
negation = -42
positive = +42
bitwise_not = ~0xFF

# Collections
list_example = [1, 2, 3, 4, 5]
tuple_example = (1, "hello", 3.14, True)
dict_example = {"name": "John", "age": 30, "city": "NYC"}
set_example = {1, 2, 3, 4, 5}

# Indexing and slicing
list_index = list_example[0]
dict_access = dict_example["name"]
slice_example = list_example[1:4]
slice_with_step = list_example[::2]
slice_reverse = list_example[::-1]

# Comprehensions
list_comp = [x * 2 for x in range(10)]
list_comp_filtered = [x for x in range(20) if x % 2 == 0]
set_comp = {x ** 2 for x in range(5)}
dict_comp = {x: x ** 2 for x in range(5)}

# Function calls
simple_call = print("Hello")
method_call = "hello".upper()
chained_calls = "  hello  ".strip().upper()

# Attribute access
import math
pi_value = math.pi
module_function = math.sqrt(16)

# Lambda expressions
square = lambda x: x ** 2
add = lambda x, y: x + y
conditional_lambda = lambda x: x if x > 0 else -x

# Statement Examples

def demonstrate_statements():
    """Show various statement types."""
    
    # Simple assignment
    x = 10
    y = 20
    
    # Augmented assignment
    x += 5
    y *= 2
    x //= 3
    
    # Annotated assignment
    count: int = 0
    name: str = "Python"
    
    # Multiple operations
    result = (x + y) * 2 - 10
    
    # If statements
    if x > 0:
        print("Positive")
    elif x < 0:
        print("Negative")
    else:
        print("Zero")
    
    # Nested if
    if x > 0:
        if x > 10:
            print("Greater than 10")
        else:
            print("Between 1 and 10")
    
    # While loop
    counter = 0
    while counter < 5:
        print(counter)
        counter += 1
    
    # For loop
    for i in range(10):
        if i == 5:
            continue
        if i == 8:
            break
        print(i)
    
    # For with else
    for i in range(3):
        print(i)
    else:
        print("Loop completed")
    
    # Nested loops
    for i in range(3):
        for j in range(3):
            print(f"({i}, {j})")
    
    # Return statements
    if x > 100:
        return x
    elif x > 50:
        return x * 2
    else:
        return None

def demonstrate_advanced():
    """Show advanced statement types."""
    
    # With statement
    with open("file.txt", "w") as f:
        f.write("Hello, World!")
    
    # Exception handling (raise)
    if False:
        raise ValueError("Something went wrong")
    
    # Await expression (in async context)
    # await some_async_function()
    
    # Complex expressions
    complex_expr = (
        [x for x in range(10) if x % 2 == 0]
        + [x * 2 for x in range(5)]
    )
    
    # Nested data structures
    nested = {
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ],
        "count": 2,
        "active": True
    }
    
    # Method chaining
    result = (
        "  Hello World  "
        .strip()
        .lower()
        .replace(" ", "_")
    )
    
    return result

def demonstrate_comprehensions():
    """Show various comprehension types."""
    
    # Basic list comprehension
    squares = [x ** 2 for x in range(10)]
    
    # Filtered comprehension
    evens = [x for x in range(20) if x % 2 == 0]
    
    # Nested comprehension
    matrix = [[i + j for j in range(3)] for i in range(3)]
    
    # Set comprehension
    unique_squares = {x ** 2 for x in [-2, -1, 0, 1, 2]}
    
    # Dict comprehension
    square_map = {x: x ** 2 for x in range(5)}
    
    # Comprehension with multiple conditions
    filtered = [
        x for x in range(100)
        if x % 2 == 0
        if x % 3 == 0
    ]
    
    # Complex element expression
    transformed = [
        (x, x ** 2, x ** 3)
        for x in range(5)
    ]
    
    return transformed

class DemoClass:
    """Class to demonstrate attribute access conversion."""
    
    def __init__(self, value):
        self.value = value
        self.data = []
    
    def method(self):
        return self.value * 2
    
    def chain_example(self):
        return self.method() + 10

# Demonstrate all conversions
if __name__ == "__main__":
    print("AST Converters Demo")
    print("=" * 40)
    
    # Test expressions
    print(f"Literals: {int_literal}, {float_literal}, {string_literal}")
    print(f"Operations: {addition}, {multiplication}, {greater_than}")
    print(f"Collections: {list_example}, {dict_example}")
    print(f"Comprehensions: {list_comp}")
    print(f"Lambda: {square(5)}, {add(3, 4)}")
    
    # Test statements
    result = demonstrate_statements()
    print(f"Statement result: {result}")
    
    # Test advanced features
    advanced = demonstrate_advanced()
    print(f"Advanced result: {advanced}")
    
    # Test comprehensions
    comps = demonstrate_comprehensions()
    print(f"Comprehensions: {comps}")
    
    # Test class
    obj = DemoClass(10)
    print(f"Object method: {obj.method()}")
    print(f"Chained: {obj.chain_example()}")