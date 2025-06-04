from typing import List

# Test Case 51: Factorial calculation
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)

# Test Case 52: Fibonacci sequence
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for _ in range(2, n + 1):
        temp: int = a + b
        a = b
        b = temp
    return b

# Test Case 53: Binary search
def binary_search(arr: List[int], target: int) -> int:
    left: int = 0
    right: int = len(arr) - 1
    
    while left <= right:
        mid: int = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1

# Test Case 54: Count down loop
def count_down(start: int) -> List[int]:
    result: List[int] = []
    current: int = start
    while current > 0:
        result.append(current)
        current -= 1
    return result

# Test Case 55: Prime number check
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            return False
    return True

# Test Case 56: Greatest common divisor
def gcd(a: int, b: int) -> int:
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a

# Test Case 57: Power calculation (iterative)
def power_iterative(base: int, exponent: int) -> int:
    result: int = 1
    for _ in range(exponent):
        result *= base
    return result

# Test Case 58: Sum of digits
def sum_of_digits(n: int) -> int:
    total: int = 0
    while n > 0:
        total += n % 10
        n //= 10
    return total

# Test Case 59: Reverse integer
def reverse_integer(n: int) -> int:
    result: int = 0
    while n > 0:
        result = result * 10 + n % 10
        n //= 10
    return result

# Test Case 60: Linear search
def linear_search(arr: List[int], target: int) -> int:
    for i in range(len(arr)):
        if arr[i] == target:
            return i
    return -1

# Test Case 61: Bubble sort
def bubble_sort(arr: List[int]) -> List[int]:
    n: int = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                temp: int = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = temp
    return arr

# Test Case 62: Count vowels
def count_vowels(text: str) -> int:
    vowels: str = "aeiouAEIOU"
    count: int = 0
    for char in text:
        if char in vowels:
            count += 1
    return count

# Test Case 63: Generate range
def generate_range(start: int, end: int, step: int) -> List[int]:
    result: List[int] = []
    current: int = start
    while current < end:
        result.append(current)
        current += step
    return result

# Test Case 64: Nested loop sum
def nested_sum(matrix: List[List[int]]) -> int:
    total: int = 0
    for row in matrix:
        for value in row:
            total += value
    return total

# Test Case 65: Multiple conditions
def classify_age(age: int) -> str:
    if age < 0:
        return "invalid"
    elif age < 13:
        return "child"
    elif age < 20:
        return "teenager"
    elif age < 65:
        return "adult"
    else:
        return "senior"