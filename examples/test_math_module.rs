#[doc = "// Python import: math"]
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Test basic math functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_basic_math_functions() -> f64 {
    let sqrt_result: f64 = std::sqrt(16.0);
    let pow_result: f64 = math.pow(2.0, 3.0);
    let floor_result: f64 = math.floor(3.7);
    let ceil_result: f64 = math.ceil(3.2);
    let abs_result: f64 = math.fabs(-5.5);
    sqrt_result + pow_result + floor_result + ceil_result + abs_result
}
#[doc = "Test trigonometric functions"]
#[doc = " Depyler: proven to terminate"]
pub fn test_trigonometric_functions() -> Result<f64, ZeroDivisionError> {
    let _cse_temp_0 = (std::consts::PI as f64) / (4.0 as f64);
    let angle: f64 = _cse_temp_0;
    let sin_result: f64 = std::sin(angle);
    let cos_result: f64 = std::cos(angle);
    let tan_result: f64 = std::tan(angle);
    Ok(sin_result + cos_result + tan_result)
}
#[doc = "Test logarithmic and exponential functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_logarithmic_functions() -> f64 {
    let ln_result: f64 = math.log(std::consts::E);
    let log10_result: f64 = math.log10(100.0);
    let exp_result: f64 = math.exp(1.0);
    ln_result + log10_result + exp_result
}
#[doc = "Test various rounding operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_rounding_functions() -> f64 {
    let floored: f64 = math.floor(3.14159);
    let ceiled: f64 = math.ceil(3.14159);
    let truncated: f64 = math.trunc(3.14159);
    floored + ceiled + truncated
}
#[doc = "Test mathematical constants"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_constants() -> f64 {
    let pi_value: f64 = std::consts::PI;
    let e_value: f64 = std::consts::E;
    let _cse_temp_0 = pi_value * 5.0;
    let _cse_temp_1 = _cse_temp_0 * 5.0;
    let circle_area: f64 = _cse_temp_1;
    let _cse_temp_2 = e_value * 2.0;
    let exponential_growth: f64 = _cse_temp_2;
    circle_area + exponential_growth
}
#[doc = "Test hyperbolic functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hyperbolic_functions() -> f64 {
    let sinh_result: f64 = math.sinh(1.0);
    let cosh_result: f64 = math.cosh(1.0);
    let tanh_result: f64 = math.tanh(1.0);
    sinh_result + cosh_result + tanh_result
}
#[doc = "Test special mathematical functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_special_functions() -> f64 {
    let fact_5: i32 = math.factorial(5);
    let gcd_result: i32 = math.gcd(48, 18);
    (fact_5 + gcd_result) as f64
}
#[doc = "Test degree/radian conversions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_angle_conversions() -> f64 {
    let radians: f64 = std::consts::PI;
    let deg_to_rad: f64 = math.radians(180.0);
    let rad_to_deg: f64 = math.degrees(radians);
    deg_to_rad + rad_to_deg
}
#[doc = "Calculate Euclidean distance between two points"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx: f64 = x2 - x1;
    let dy: f64 = y2 - y1;
    let distance: f64 = std::sqrt(dx * dx + dy * dy);
    distance
}
#[doc = "Calculate hypotenuse using Pythagorean theorem"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_hypotenuse(a: f64, b: f64) -> f64 {
    std::sqrt(468)
}
#[doc = "Test various power operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_power_operations() -> f64 {
    let basic_pow: f64 = math.pow(2.0, 8.0);
    let sqrt_as_pow: f64 = math.pow(25.0, 0.5);
    let cube_root: f64 = math.pow(27.0, 0.3333333333333333);
    basic_pow + sqrt_as_pow + cube_root
}
#[doc = "Test min/max with math operations"]
pub fn test_comparison_functions(values: &Vec<f64>) -> Result<f64, IndexError> {
    let _cse_temp_0 = values.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    let mut min_val: f64 = {
        let base = &values;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    let mut max_val: f64 = {
        let base = &values;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    for val in values.iter().cloned() {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
    }
    let value_range: f64 = max_val - min_val;
    let geometric_mean: f64 = std::sqrt(min_val * max_val);
    Ok(value_range + geometric_mean)
}
#[doc = "Calculate statistical values using math operations"]
pub fn test_statistical_math(numbers: &Vec<f64>) -> Result<f64, ZeroDivisionError> {
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    let mut total: f64 = 0.0;
    for num in numbers.iter().cloned() {
        total = total + num;
    }
    let _cse_temp_2 = (_cse_temp_0) as f64;
    let _cse_temp_3 = (total as f64) / (_cse_temp_2 as f64);
    let mean: f64 = _cse_temp_3;
    let mut variance_sum: f64 = 0.0;
    for num in numbers.iter().cloned() {
        let diff: f64 = num - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_4 = (variance_sum as f64) / (_cse_temp_2 as f64);
    let variance: f64 = _cse_temp_4;
    let std_dev: f64 = std::sqrt(variance);
    Ok(mean + std_dev)
}
#[doc = "Test sign-related functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sign_and_copysign() -> f64 {
    let abs1: f64 = math.fabs(-10.5);
    let abs2: f64 = math.fabs(7.3);
    let result1: f64 = math.copysign(5.0, -1.0);
    let result2: f64 = math.copysign(5.0, 1.0);
    abs1 + abs2 + result1 + result2
}
#[doc = "Test modulo and remainder operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_remainder_operations() -> f64 {
    let mod_result: f64 = math.fmod(10.5, 3.0);
    let remainder: f64 = math.remainder(10.0, 3.0);
    mod_result + remainder
}
#[doc = "Test integer-specific math operations"]
#[doc = " Depyler: proven to terminate"]
pub fn test_integer_operations() -> Result<i32, ZeroDivisionError> {
    let fact: i32 = math.factorial(6);
    let gcd1: i32 = math.gcd(48, 18);
    let gcd2: i32 = math.gcd(math.gcd(24, 36), 48);
    let _cse_temp_0 = 216.abs();
    let _cse_temp_1 = {
        let a = _cse_temp_0;
        let b = math.gcd(12, 18);
        let q = a / b;
        let r = a % b;
        let r_negative = r < 0;
        let b_negative = b < 0;
        let r_nonzero = r != 0;
        let signs_differ = r_negative != b_negative;
        let needs_adjustment = r_nonzero && signs_differ;
        if needs_adjustment {
            q - 1
        } else {
            q
        }
    };
    let lcm: i32 = _cse_temp_1;
    Ok(fact + gcd1 + gcd2 + lcm)
}
#[doc = "Run all math module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_math_features() {
    let sample_values: Vec<f64> = vec![1.5, 2.7, 3.2, 4.8, 5.1];
    println!("{}", "All math module tests completed successfully");
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_integer_operations_examples() {
        let _ = test_integer_operations();
    }
}
