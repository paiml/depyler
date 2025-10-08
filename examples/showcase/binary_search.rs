#[doc = "Find target in sorted array, return -1 if not found."] pub fn binary_search<'a>(arr: & 'a Vec<i32>, target: i32)  -> Result<i32, Box<dyn std::error::Error>>{
    let mut left = 0;
    let _cse_temp_0 = arr.len();
    let mut right = _cse_temp_0 - 1;
    while left <= right {
    let mid = {
    let a = left + right;
    let b = 2;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
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
    left = mid + 1;
   
}
else {
    right = mid - 1;
   
}
}
}
return Ok(- 1)
}