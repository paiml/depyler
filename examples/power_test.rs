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
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_integer_power() -> (i32, i32, i32, i32) {
    let _cse_temp_0 = ({ 2 } as i32)
        .checked_pow({ 3 } as u32)
        .expect("Power operation overflowed");
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 10 } as i32)
        .checked_pow({ 2 } as u32)
        .expect("Power operation overflowed");
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 5 } as i32)
        .checked_pow({ 0 } as u32)
        .expect("Power operation overflowed");
    let c = _cse_temp_2;
    let base = 3;
    let exp = 4;
    let _cse_temp_3 = {
        if exp >= 0 && (exp as i64) <= (u32::MAX as i64) {
            ({ base } as i32)
                .checked_pow({ exp } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ base } as f64).powf({ exp } as f64) as i32
        }
    };
    let d = _cse_temp_3;
    (a, b, c, d)
}
#[doc = " Depyler: proven to terminate"]
pub fn test_float_power() -> Result<(f64, f64, f64, i32), Box<dyn std::error::Error>> {
    let _cse_temp_0 = ({ 2.5 } as f64).powf({ 2 } as f64);
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 10.0 } as f64).powf({ 3 } as f64);
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 4 } as f64).powf({ 0.5 } as f64);
    let c = _cse_temp_2;
    let _cse_temp_3 = ({ 8 } as i32)
        .checked_pow({ 0 } as u32)
        .expect("Power operation overflowed");
    let d = _cse_temp_3;
    Ok((a, b, c, d))
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_negative_exponent() -> (i32, i32, i32) {
    let _cse_temp_0 = ({ 2 } as f64).powf({ -1 } as f64);
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 10 } as f64).powf({ -2 } as f64);
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 5 } as f64).powf({ -3 } as f64);
    let c = _cse_temp_2;
    (a, b, c)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_large_powers() -> (i32, i32, i32) {
    let _cse_temp_0 = ({ 2 } as i32)
        .checked_pow({ 10 } as u32)
        .expect("Power operation overflowed");
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 2 } as i32)
        .checked_pow({ 20 } as u32)
        .expect("Power operation overflowed");
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 10 } as i32)
        .checked_pow({ 6 } as u32)
        .expect("Power operation overflowed");
    let c = _cse_temp_2;
    (a, b, c)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_mixed_operations() -> (i32, i32, i32, i32) {
    let _cse_temp_0 = ({ 3 } as i32)
        .checked_pow({ 2 } as u32)
        .expect("Power operation overflowed");
    let a = 2 + _cse_temp_0;
    let _cse_temp_1 = ({ 5 } as i32)
        .checked_pow({ 2 } as u32)
        .expect("Power operation overflowed");
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 2 } as i32)
        .checked_pow({ 3 } as u32)
        .expect("Power operation overflowed");
    let _cse_temp_3 = _cse_temp_2 * 4;
    let c = _cse_temp_3;
    let _cse_temp_4 = ({ 2 } as i32)
        .checked_pow({ 6 } as u32)
        .expect("Power operation overflowed");
    let d = _cse_temp_4;
    (a, b, c, d)
}
#[doc = "Test power with function parameters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn compute_power(base: i32, exp: i32) -> i32 {
    {
        if exp >= 0 && (exp as i64) <= (u32::MAX as i64) {
            ({ base } as i32)
                .checked_pow({ exp } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ base } as f64).powf({ exp } as f64) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_compute_power_examples() {
        assert_eq!(compute_power(0, 0), 0);
        assert_eq!(compute_power(1, 2), 3);
        assert_eq!(compute_power(-1, 1), 0);
    }
}
