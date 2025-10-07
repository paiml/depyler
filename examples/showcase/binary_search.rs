#[doc = "Find target in sorted array, return -1 if not found."] pub fn binary_search<'a>(arr: & 'a Vec<i32>, target: i32)  -> Result<i32, Box<dyn std::error::Error>>{
    let mut _cse_temp_0 = arr.len();
    let mut right = _cse_temp_0 - 1;
    while 0 <= right {
    let mut mid = {
    let a = 0 + right;
    let b = 2;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    if arr.get(mid as usize).copied().unwrap_or_default() == target {
    return Ok(mid);
   
}
else {
    if arr.get(mid as usize).copied().unwrap_or_default()<target {
    let mut left = mid + 1;
   
}
else {
    right = mid - 1;
   
}
}
}
return Ok(- 1)
}