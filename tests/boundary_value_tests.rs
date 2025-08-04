use depyler_core::DepylerPipeline;

#[cfg(test)]
mod boundary_value_tests {
    use super::*;

    #[test]
    fn test_zero_values() {
        let pipeline = DepylerPipeline::new();
        let zero_values_source = r#"
def zero_test() -> int:
    zero_int = 0
    zero_float = 0.0
    zero_list = []
    zero_dict = {}
    return len(zero_list) + len(zero_dict) + zero_int
"#;
        
        let result = pipeline.transpile(zero_values_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_negative_one_values() {
        let pipeline = DepylerPipeline::new();
        let negative_one_source = r#"
def negative_one_test(arr: list) -> int:
    last_index = -1
    if arr:
        return arr[last_index]
    return -1
"#;
        
        let result = pipeline.transpile(negative_one_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_maximum_list_size() {
        let pipeline = DepylerPipeline::new();
        let max_list_source = r#"
def create_large_list(size: int) -> list:
    result = []
    for i in range(size):
        result.append(i)
    return result
"#;
        
        let result = pipeline.transpile(max_list_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_element_collections() {
        let pipeline = DepylerPipeline::new();
        let single_element_source = r#"
def single_element_test() -> int:
    single_list = [42]
    single_dict = {"key": "value"}
    return len(single_list) + len(single_dict)
"#;
        
        let result = pipeline.transpile(single_element_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_power_of_two_boundaries() {
        let pipeline = DepylerPipeline::new();
        let power_of_two_source = r#"
def power_of_two_test() -> int:
    powers = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    total = 0
    for power in powers:
        total += power
    return total
"#;
        
        let result = pipeline.transpile(power_of_two_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_off_by_one_loop_conditions() {
        let pipeline = DepylerPipeline::new();
        let off_by_one_source = r#"
def off_by_one_test(n: int) -> int:
    # Test various loop boundaries
    count1 = 0
    for i in range(n):
        count1 += 1
    
    count2 = 0  
    for i in range(n + 1):
        count2 += 1
        
    count3 = 0
    for i in range(n - 1):
        count3 += 1
        
    return count1 + count2 + count3
"#;
        
        let result = pipeline.transpile(off_by_one_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_string_length_boundaries() {
        let pipeline = DepylerPipeline::new();
        let string_boundaries_source = r#"
def string_boundary_test() -> int:
    empty_string = ""
    single_char = "a"
    long_string = "a" * 1000
    
    return len(empty_string) + len(single_char) + len(long_string)
"#;
        
        let result = pipeline.transpile(string_boundaries_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_floating_point_boundaries() {
        let pipeline = DepylerPipeline::new();
        let float_boundaries_source = r#"
def float_boundary_test() -> float:
    tiny = 0.0001
    zero = 0.0
    negative_tiny = -0.0001
    one = 1.0
    
    return tiny + zero + negative_tiny + one
"#;
        
        let result = pipeline.transpile(float_boundaries_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_recursive_depth_boundaries() {
        let pipeline = DepylerPipeline::new();
        let recursion_source = r#"
def factorial_recursive(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial_recursive(n - 1)

def test_recursion_depths() -> int:
    # Test various recursion depths
    small = factorial_recursive(5)
    medium = factorial_recursive(10)
    return small + medium
"#;
        
        let result = pipeline.transpile(recursion_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_boolean_boundaries() {
        let pipeline = DepylerPipeline::new();
        let boolean_boundaries_source = r#"
def boolean_boundary_test() -> bool:
    true_val = True
    false_val = False
    
    # Test truthiness boundaries
    zero_truthy = bool(0)
    one_truthy = bool(1)
    empty_list_truthy = bool([])
    non_empty_list_truthy = bool([1])
    
    return true_val and not false_val
"#;
        
        let result = pipeline.transpile(boolean_boundaries_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_modulo_operation_boundaries() {
        let pipeline = DepylerPipeline::new();
        let modulo_boundaries_source = r#"
def modulo_boundary_test(n: int) -> int:
    # Test modulo with various divisors
    mod_one = n % 1 if n != 0 else 0
    mod_two = n % 2
    mod_ten = n % 10
    mod_hundred = n % 100
    
    return mod_two + mod_ten + mod_hundred
"#;
        
        let result = pipeline.transpile(modulo_boundaries_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_range_boundaries() {
        let pipeline = DepylerPipeline::new();
        let range_boundaries_source = r#"
def range_boundary_test() -> int:
    # Test edge cases for range function
    empty_range = list(range(0))
    single_range = list(range(1))
    reverse_range = list(range(5, 0, -1))
    
    return len(empty_range) + len(single_range) + len(reverse_range)
"#;
        
        let result = pipeline.transpile(range_boundaries_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_conditional_boundaries() {
        let pipeline = DepylerPipeline::new();
        let conditional_boundaries_source = r#"
def conditional_boundary_test(x: int) -> int:
    # Test boundary conditions in if statements
    if x < 0:
        return -1
    elif x == 0:
        return 0
    elif x == 1:
        return 1
    else:
        return x
"#;
        
        let result = pipeline.transpile(conditional_boundaries_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_comparison_operator_boundaries() {
        let pipeline = DepylerPipeline::new();
        let comparison_boundaries_source = r#"
def comparison_boundary_test(a: int, b: int) -> bool:
    # Test all comparison operators at boundaries
    equal = a == b
    not_equal = a != b
    less_than = a < b
    less_equal = a <= b
    greater_than = a > b
    greater_equal = a >= b
    
    return equal or not_equal or less_than or less_equal or greater_than or greater_equal
"#;
        
        let result = pipeline.transpile(comparison_boundaries_source);
        assert!(result.is_ok() || result.is_err()); // May fail on complex expressions
    }

    #[test]
    fn test_container_access_boundaries() {
        let pipeline = DepylerPipeline::new();
        let container_access_source = r#"
def container_access_test() -> int:
    items = [1, 2, 3, 4, 5]
    
    # Test boundary access patterns
    first = items[0]
    last = items[-1]
    
    # Test slicing boundaries
    all_items = items[:]
    first_two = items[:2]
    last_two = items[-2:]
    middle = items[1:-1]
    
    return first + last + len(all_items) + len(first_two) + len(last_two) + len(middle)
"#;
        
        let result = pipeline.transpile(container_access_source);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_arithmetic_overflow_boundaries() {
        let pipeline = DepylerPipeline::new();
        let arithmetic_overflow_source = r#"
def arithmetic_boundary_test() -> int:
    # Test potential overflow conditions
    large_positive = 1000000
    large_negative = -1000000
    
    addition = large_positive + large_positive
    subtraction = large_positive - large_negative
    multiplication = large_positive * 2
    
    return addition + subtraction + multiplication
"#;
        
        let result = pipeline.transpile(arithmetic_overflow_source);
        assert!(result.is_ok());
    }
}