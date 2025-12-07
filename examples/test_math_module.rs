use std::f64 as math;
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
    let sqrt_result: f64 = (16.0 as f64).sqrt();
    let pow_result: f64 = (2.0 as f64).powf(3.0 as f64);
    let floor_result: f64 = (3.7 as f64).floor() as i32;
    let ceil_result: f64 = (3.2 as f64).ceil() as i32;
    let abs_result: f64 = (-5.5 as f64).abs();
    sqrt_result + pow_result + floor_result + ceil_result + abs_result
}
#[doc = "Test trigonometric functions"]
#[doc = " Depyler: proven to terminate"]
pub fn test_trigonometric_functions() -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = (std::f64::consts::PI as f64) / (4.0 as f64);
    let angle: f64 = _cse_temp_0;
    let sin_result: f64 = (angle as f64).sin();
    let cos_result: f64 = (angle as f64).cos();
    let tan_result: f64 = (angle as f64).tan();
    Ok(sin_result + cos_result + tan_result)
}
#[doc = "Test logarithmic and exponential functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_logarithmic_functions() -> f64 {
    let ln_result: f64 = (std::f64::consts::E as f64).ln();
    let log10_result: f64 = (100.0 as f64).log10();
    let exp_result: f64 = (1.0 as f64).exp();
    ln_result + log10_result + exp_result
}
#[doc = "Test various rounding operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_rounding_functions() -> f64 {
    let value: f64 = 3.14159;
    let floored: f64 = (value as f64).floor() as i32;
    let ceiled: f64 = (value as f64).ceil() as i32;
    let truncated: f64 = (value as f64).trunc();
    floored + ceiled + truncated
}
#[doc = "Test mathematical constants"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_constants() -> f64 {
    let pi_value: f64 = std::f64::consts::PI;
    let e_value: f64 = std::f64::consts::E;
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
    let x: f64 = 1.0;
    let sinh_result: f64 = (x as f64).sinh();
    let cosh_result: f64 = (x as f64).cosh();
    let tanh_result: f64 = (x as f64).tanh();
    sinh_result + cosh_result + tanh_result
}
#[doc = "Test special mathematical functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_special_functions() -> f64 {
    let fact_5: i32 = {
        let n = 5 as i32;
        let mut result = 1i64;
        for i in 1..=n {
            result *= i as i64;
        }
        result as i32
    };
    let gcd_result: i32 = {
        let mut a = (48 as i64).abs();
        let mut b = (18 as i64).abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a as i32
    };
    (fact_5 + gcd_result) as f64
}
#[doc = "Test degree/radian conversions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_angle_conversions() -> f64 {
    let degrees: f64 = 180.0;
    let radians: f64 = std::f64::consts::PI;
    let deg_to_rad: f64 = (degrees as f64).to_radians();
    let rad_to_deg: f64 = (radians as f64).to_degrees();
    deg_to_rad + rad_to_deg
}
#[doc = "Calculate Euclidean distance between two points"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx: f64 = x2 - x1;
    let dy: f64 = y2 - y1;
    let distance: f64 = (dx * dx + dy * dy as f64).sqrt();
    distance
}
#[doc = "Calculate hypotenuse using Pythagorean theorem"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_hypotenuse(a: f64, b: f64) -> f64 {
    (a * a + b * b as f64).sqrt()
}
#[doc = "Test various power operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_power_operations() -> f64 {
    let basic_pow: f64 = (2.0 as f64).powf(8.0 as f64);
    let sqrt_as_pow: f64 = (25.0 as f64).powf(0.5 as f64);
    let cube_root: f64 = (27.0 as f64).powf(0.3333333333333333 as f64);
    basic_pow + sqrt_as_pow + cube_root
}
#[doc = "Test min/max with math operations"]
pub fn test_comparison_functions(values: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = values.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    let mut min_val: f64 = values.get(0usize).cloned().unwrap_or_default();
    let mut max_val: f64 = values.get(0usize).cloned().unwrap_or_default();
    for val in values.iter().cloned() {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
    }
    let value_range: f64 = max_val - min_val;
    let geometric_mean: f64 = (min_val * max_val as f64).sqrt();
    Ok(value_range + geometric_mean)
}
#[doc = "Calculate statistical values using math operations"]
pub fn test_statistical_math(numbers: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
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
    let std_dev: f64 = (variance as f64).sqrt();
    Ok(mean + std_dev)
}
#[doc = "Test sign-related functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sign_and_copysign() -> f64 {
    let abs1: f64 = (-10.5 as f64).abs();
    let abs2: f64 = (7.3 as f64).abs();
    let result1: f64 = (5.0 as f64).copysign(-1.0 as f64);
    let result2: f64 = (5.0 as f64).copysign(1.0 as f64);
    abs1 + abs2 + result1 + result2
}
#[doc = "Test modulo and remainder operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_remainder_operations() -> f64 {
    let mod_result: f64 = (10.5 as f64) % (3.0 as f64);
    let remainder: f64 = {
        let x = 10.0 as f64;
        let y = 3.0 as f64;
        let n = (x / y).round();
        x - n * y
    };
    mod_result + remainder
}
#[doc = "Test integer-specific math operations"]
#[doc = " Depyler: proven to terminate"]
pub fn test_integer_operations() -> Result<i32, Box<dyn std::error::Error>> {
    let fact: i32 = {
        let n = 6 as i32;
        let mut result = 1i64;
        for i in 1..=n {
            result *= i as i64;
        }
        result as i32
    };
    let gcd1: i32 = {
        let mut a = (48 as i64).abs();
        let mut b = (18 as i64).abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a as i32
    };
    let gcd2: i32 = {
        let mut a = ({
            let mut a = (24 as i64).abs();
            let mut b = (36 as i64).abs();
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a as i32
        } as i64)
            .abs();
        let mut b = (48 as i64).abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a as i32
    };
    let a: i32 = 12;
    let b: i32 = 18;
    let _cse_temp_0 = a * b;
    let _cse_temp_1 = _cse_temp_0.abs();
    let _cse_temp_2 = {
        let a = _cse_temp_1;
        let b = {
            let mut a = (a as i64).abs();
            let mut b = (b as i64).abs();
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a as i32
        };
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
    let lcm: i32 = _cse_temp_2;
    Ok(fact + gcd1 + gcd2 + lcm)
}
#[doc = "Run all math module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_math_features() -> Result<(), Box<dyn std::error::Error>> {
    let _basic_result: f64 = test_basic_math_functions();
    let _trig_result: f64 = test_trigonometric_functions()?;
    let _log_result: f64 = test_logarithmic_functions();
    let _round_result: f64 = test_rounding_functions();
    let _const_result: f64 = test_constants();
    let _hyper_result: f64 = test_hyperbolic_functions();
    let _special_result: f64 = test_special_functions();
    let _angle_result: f64 = test_angle_conversions();
    let _dist: f64 = calculate_distance(0.0, 0.0, 3.0, 4.0);
    let _hyp: f64 = calculate_hypotenuse(3.0, 4.0);
    let _power_result: f64 = test_power_operations();
    let sample_values: Vec<f64> = vec![1.5, 2.7, 3.2, 4.8, 5.1];
    let _comp_result: f64 = test_comparison_functions(&sample_values)?;
    let _stat_result: f64 = test_statistical_math(&sample_values)?;
    let _sign_result: f64 = test_sign_and_copysign();
    let _remainder_result: f64 = test_remainder_operations();
    let _int_result: i32 = test_integer_operations()?;
    println!("{}", "All math module tests completed successfully");
    Ok(())
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
