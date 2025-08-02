#[doc = "Test floor division with positive operands"] #[doc = " Depyler: proven to terminate"] pub fn test_floor_division_positive()  -> Result<serde_json::Value, ZeroDivisionError>{
    let mut a = 7;
    let mut b = 3;
    let mut result = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok(result);
   
}
#[doc = "Test floor division with negative dividend"] #[doc = " Depyler: proven to terminate"] pub fn test_floor_division_negative()  -> Result<serde_json::Value, ZeroDivisionError>{
    let mut a = - 7;
    let mut b = 3;
    let mut result = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok(result);
   
}
#[doc = "Test floor division with negative divisor"] #[doc = " Depyler: proven to terminate"] pub fn test_floor_division_negative_divisor()  -> Result<serde_json::Value, ZeroDivisionError>{
    let mut a = 7;
    let mut b = - 3;
    let mut result = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok(result);
   
}
#[doc = "Test floor division with both operands negative"] #[doc = " Depyler: proven to terminate"] pub fn test_floor_division_both_negative()  -> Result<serde_json::Value, ZeroDivisionError>{
    let mut a = - 7;
    let mut b = - 3;
    let mut result = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok(result);
   
}
#[doc = "Test floor division with exact result"] #[doc = " Depyler: proven to terminate"] pub fn test_floor_division_exact()  -> Result<serde_json::Value, ZeroDivisionError>{
    let mut a = 9;
    let mut b = 3;
    let mut result = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok(result);
   
}
#[doc = "Test floor division with zero remainder edge case"] #[doc = " Depyler: proven to terminate"] pub fn test_floor_division_zero_remainder()  -> Result<serde_json::Value, ZeroDivisionError>{
    let mut a = - 9;
    let mut b = 3;
    let mut result = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok(result)
}