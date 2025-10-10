#[derive(Debug, Clone)] pub struct ZeroDivisionError {
    message: String ,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "division by zero: {}", self.message)
}
} impl std::error::Error for ZeroDivisionError {
   
}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "\n    Binary search implementation with contracts.\n    \n    @requires items is not None\n    @requires all(items[i] <= items[i+1] for i in range(len(items)-1))\n    @ensures result>= -1\n    @ensures result<len(items)\n    @invariant low <= high\n    "] pub fn binary_search<'a>(items: & 'a Vec<i32>, target: i32)  -> Result<i32, Box<dyn std::error::Error>>{
    let mut low = 0;
    let _cse_temp_0 = items.len();
    let mut high = _cse_temp_0 - 1;
    while low <= high {
    let mid = ((low + high) / 2) as i32;
    if items.get(mid as usize).copied().unwrap_or_default() == target {
    return Ok(mid);
   
}
else {
    if items.get(mid as usize).copied().unwrap_or_default()<target {
    low = mid + 1;
   
}
else {
    high = mid - 1;
   
}
}
}
return Ok(- 1);
   
}
#[doc = "\n    Safe division with contracts.\n    \n    @requires denominator != 0\n    @ensures result == numerator / denominator\n    "] #[doc = " Depyler: proven to terminate"] pub fn safe_divide(numerator: f64, denominator: f64)  -> Result<f64, ZeroDivisionError>{
    let _cse_temp_0 = numerator / denominator;
    return Ok(_cse_temp_0);
   
}
#[doc = "\n    Sum all numbers in a list.\n    \n    @requires numbers is not None\n    @ensures result>= 0 if all(n>= 0 for n in numbers) else True\n    "] #[doc = " Depyler: verified panic-free"] pub fn list_sum<'a>(numbers: & 'a Vec<f64>)  -> f64 {
    let mut total = 0.0;
    for num in numbers.iter() {
    total = total + num;

}
return total
}