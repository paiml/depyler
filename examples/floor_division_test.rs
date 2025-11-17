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
#[doc = "Test floor division with positive operands"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division_positive() -> Result<(), ZeroDivisionError> {
    let a = 7;
    let b = 3;
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let result = _cse_temp_0;
    Ok(result)
}
#[doc = "Test floor division with negative dividend"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division_negative() -> Result<(), ZeroDivisionError> {
    let a = -7;
    let b = 3;
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let result = _cse_temp_0;
    Ok(result)
}
#[doc = "Test floor division with negative divisor"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division_negative_divisor() -> Result<(), ZeroDivisionError> {
    let a = 7;
    let b = -3;
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let result = _cse_temp_0;
    Ok(result)
}
#[doc = "Test floor division with both operands negative"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division_both_negative() -> Result<(), ZeroDivisionError> {
    let a = -7;
    let b = -3;
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let result = _cse_temp_0;
    Ok(result)
}
#[doc = "Test floor division with exact result"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division_exact() -> Result<(), ZeroDivisionError> {
    let a = 9;
    let b = 3;
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let result = _cse_temp_0;
    Ok(result)
}
#[doc = "Test floor division with zero remainder edge case"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division_zero_remainder() -> Result<(), ZeroDivisionError> {
    let a = -9;
    let b = 3;
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let result = _cse_temp_0;
    Ok(result)
}
