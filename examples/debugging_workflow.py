#!/usr/bin/env python3
"""
Example: Debugging Workflow with Depyler

This example demonstrates how to use Depyler's debugging features
to debug transpiled Rust code while maintaining Python source context.
"""

# Sample Python code to debug
def fibonacci(n: int) -> int:
    """Calculate fibonacci number recursively"""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def factorial(n: int) -> int:
    """Calculate factorial iteratively"""
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result

def find_max(numbers: list) -> int:
    """Find maximum in a list"""
    if not numbers:
        return 0
    
    max_val = numbers[0]
    for num in numbers:
        if num > max_val:
            max_val = num
    return max_val

def main():
    """Main function demonstrating various algorithms"""
    # Test fibonacci
    print(f"Fibonacci(10) = {fibonacci(10)}")
    
    # Test factorial
    print(f"Factorial(5) = {factorial(5)}")
    
    # Test find_max
    numbers = [3, 7, 2, 9, 1, 5]
    print(f"Max of {numbers} = {find_max(numbers)}")

if __name__ == "__main__":
    main()

"""
Debugging Steps:
================

1. Transpile with debug information:
   $ depyler transpile debugging_workflow.py --debug --source-map

2. Compile the Rust code with debug symbols:
   $ rustc -g debugging_workflow.rs -o debugging_workflow

3. Generate debugger script:
   $ depyler debug debugging_workflow.py debugging_workflow.rs --debugger gdb

4. Start debugging session:
   $ rust-gdb ./debugging_workflow
   (gdb) source debugging_workflow.gdb
   
5. Set breakpoints on Python function names:
   (gdb) break fibonacci
   (gdb) break factorial
   (gdb) break find_max
   
6. Run and debug:
   (gdb) run
   (gdb) next
   (gdb) print n
   (gdb) backtrace
   
7. For LLDB users:
   $ rust-lldb ./debugging_workflow
   (lldb) command source debugging_workflow.lldb
   (lldb) breakpoint set --name fibonacci
   (lldb) run

Tips:
-----
- Variable names are preserved from Python
- Function names map directly
- Line numbers correspond to Rust source
- Use source map for Python line mapping
"""