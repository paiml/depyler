// Test Case 1: Simple addition
pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Test Case 2: Simple subtraction
pub fn subtract_numbers(a: i32, b: i32) -> i32 {
    a - b
}

// Test Case 3: Multiplication
pub fn multiply_numbers(a: i32, b: i32) -> i32 {
    a * b
}

// Test Case 4: Integer division
pub fn divide_numbers(a: i32, b: i32) -> i32 {
    a / b
}

// Test Case 5: Modulo operation
pub fn modulo_operation(a: i32, b: i32) -> i32 {
    a % b
}

// Test Case 6: Power operation
pub fn power_operation(base: i32, exponent: i32) -> i32 {
    base.pow(exponent as u32)
}

// Test Case 7: Absolute value
pub fn absolute_value(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

// Test Case 8: Maximum of two numbers
pub fn max_two_numbers(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// Test Case 9: Minimum of two numbers
pub fn min_two_numbers(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// Test Case 10: Sign function
pub fn sign_function(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}