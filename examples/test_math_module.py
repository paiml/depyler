"""
Comprehensive test of Python math module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's math module
functions to their Rust equivalents (std::f64 methods).

Expected Rust mappings:
- math.sqrt() -> f64::sqrt()
- math.pow() -> f64::powf()
- math.floor() -> f64::floor()
- math.ceil() -> f64::ceil()
- math.abs() -> f64::abs() or i32::abs()
- math.pi -> std::f64::consts::PI
- math.e -> std::f64::consts::E
"""

import math
from typing import List


def test_basic_math_functions() -> float:
    """Test basic math functions"""
    # Square root
    sqrt_result: float = math.sqrt(16.0)

    # Power
    pow_result: float = math.pow(2.0, 3.0)

    # Floor and ceiling
    floor_result: float = math.floor(3.7)
    ceil_result: float = math.ceil(3.2)

    # Absolute value
    abs_result: float = math.fabs(-5.5)

    return sqrt_result + pow_result + floor_result + ceil_result + abs_result


def test_trigonometric_functions() -> float:
    """Test trigonometric functions"""
    angle: float = math.pi / 4.0  # 45 degrees in radians

    sin_result: float = math.sin(angle)
    cos_result: float = math.cos(angle)
    tan_result: float = math.tan(angle)

    return sin_result + cos_result + tan_result


def test_logarithmic_functions() -> float:
    """Test logarithmic and exponential functions"""
    # Natural logarithm
    ln_result: float = math.log(math.e)

    # Base-10 logarithm
    log10_result: float = math.log10(100.0)

    # Exponential
    exp_result: float = math.exp(1.0)

    return ln_result + log10_result + exp_result


def test_rounding_functions() -> float:
    """Test various rounding operations"""
    value: float = 3.14159

    # Floor - round down
    floored: float = math.floor(value)

    # Ceiling - round up
    ceiled: float = math.ceil(value)

    # Truncate - round towards zero
    truncated: float = math.trunc(value)

    return floored + ceiled + truncated


def test_constants() -> float:
    """Test mathematical constants"""
    # Pi constant
    pi_value: float = math.pi

    # Euler's number
    e_value: float = math.e

    # Use constants in calculation
    circle_area: float = pi_value * 5.0 * 5.0
    exponential_growth: float = e_value * 2.0

    return circle_area + exponential_growth


def test_hyperbolic_functions() -> float:
    """Test hyperbolic functions"""
    x: float = 1.0

    sinh_result: float = math.sinh(x)
    cosh_result: float = math.cosh(x)
    tanh_result: float = math.tanh(x)

    return sinh_result + cosh_result + tanh_result


def test_special_functions() -> float:
    """Test special mathematical functions"""
    # Factorial (integer factorial)
    fact_5: int = math.factorial(5)

    # GCD (greatest common divisor)
    gcd_result: int = math.gcd(48, 18)

    # Convert to float for return
    return float(fact_5 + gcd_result)


def test_angle_conversions() -> float:
    """Test degree/radian conversions"""
    degrees: float = 180.0
    radians: float = math.pi

    # Convert degrees to radians
    deg_to_rad: float = math.radians(degrees)

    # Convert radians to degrees
    rad_to_deg: float = math.degrees(radians)

    return deg_to_rad + rad_to_deg


def calculate_distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Calculate Euclidean distance between two points"""
    dx: float = x2 - x1
    dy: float = y2 - y1

    distance: float = math.sqrt(dx * dx + dy * dy)
    return distance


def calculate_hypotenuse(a: float, b: float) -> float:
    """Calculate hypotenuse using Pythagorean theorem"""
    return math.sqrt(a * a + b * b)


def test_power_operations() -> float:
    """Test various power operations"""
    # Basic power
    basic_pow: float = math.pow(2.0, 8.0)

    # Square root is power of 0.5
    sqrt_as_pow: float = math.pow(25.0, 0.5)

    # Cube root is power of 1/3
    cube_root: float = math.pow(27.0, 1.0 / 3.0)

    return basic_pow + sqrt_as_pow + cube_root


def test_comparison_functions(values: List[float]) -> float:
    """Test min/max with math operations"""
    if len(values) == 0:
        return 0.0

    # Find extremes
    min_val: float = values[0]
    max_val: float = values[0]

    for val in values:
        if val < min_val:
            min_val = val
        if val > max_val:
            max_val = val

    # Calculate range
    value_range: float = max_val - min_val

    # Calculate geometric mean of min and max
    geometric_mean: float = math.sqrt(min_val * max_val)

    return value_range + geometric_mean


def test_statistical_math(numbers: List[float]) -> float:
    """Calculate statistical values using math operations"""
    if len(numbers) == 0:
        return 0.0

    # Sum
    total: float = 0.0
    for num in numbers:
        total = total + num

    # Mean
    mean: float = total / float(len(numbers))

    # Variance
    variance_sum: float = 0.0
    for num in numbers:
        diff: float = num - mean
        variance_sum = variance_sum + diff * diff

    variance: float = variance_sum / float(len(numbers))

    # Standard deviation
    std_dev: float = math.sqrt(variance)

    return mean + std_dev


def test_sign_and_copysign() -> float:
    """Test sign-related functions"""
    # Absolute value
    abs1: float = math.fabs(-10.5)
    abs2: float = math.fabs(7.3)

    # Copy sign from one number to another
    # copysign(magnitude, sign)
    result1: float = math.copysign(5.0, -1.0)  # Returns -5.0
    result2: float = math.copysign(5.0, 1.0)   # Returns 5.0

    return abs1 + abs2 + result1 + result2


def test_remainder_operations() -> float:
    """Test modulo and remainder operations"""
    # Floating-point modulo
    mod_result: float = math.fmod(10.5, 3.0)

    # Remainder (IEEE)
    remainder: float = math.remainder(10.0, 3.0)

    return mod_result + remainder


def test_integer_operations() -> int:
    """Test integer-specific math operations"""
    # Factorial
    fact: int = math.factorial(6)

    # GCD of two numbers
    gcd1: int = math.gcd(48, 18)

    # GCD of multiple numbers (pairwise)
    gcd2: int = math.gcd(math.gcd(24, 36), 48)

    # LCM calculation using GCD
    # lcm(a, b) = abs(a * b) / gcd(a, b)
    a: int = 12
    b: int = 18
    lcm: int = abs(a * b) // math.gcd(a, b)

    return fact + gcd1 + gcd2 + lcm


def test_all_math_features() -> None:
    """Run all math module tests"""
    basic_result: float = test_basic_math_functions()
    trig_result: float = test_trigonometric_functions()
    log_result: float = test_logarithmic_functions()
    round_result: float = test_rounding_functions()
    const_result: float = test_constants()
    hyper_result: float = test_hyperbolic_functions()
    special_result: float = test_special_functions()
    angle_result: float = test_angle_conversions()

    # Test utility functions
    dist: float = calculate_distance(0.0, 0.0, 3.0, 4.0)
    hyp: float = calculate_hypotenuse(3.0, 4.0)

    power_result: float = test_power_operations()

    # Test with sample data
    sample_values: List[float] = [1.5, 2.7, 3.2, 4.8, 5.1]
    comp_result: float = test_comparison_functions(sample_values)
    stat_result: float = test_statistical_math(sample_values)

    sign_result: float = test_sign_and_copysign()
    remainder_result: float = test_remainder_operations()
    int_result: int = test_integer_operations()

    print("All math module tests completed successfully")
