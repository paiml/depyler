#[doc = "Find target in sorted array, return -1 if not found."] pub fn binary_search<'a>(arr: & 'a Vec<i32>, target: i32)  -> Result<i32, Box<dyn std::error::Error>>{
    let mut left = 0;
    let mut right = arr.len().saturating_sub(1);
    while(left <= right) {
    let mut mid = {
    let a  = (left + right);
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
    if(arr.get(mid as usize).copied().unwrap_or_default() == target) {
    return Ok(mid);
   
}
else {
    if(arr.get(mid as usize).copied().unwrap_or_default()<target) {
    left  = (mid + 1);
   
}
else {
    right  = (mid - 1);
   
}
}
}
return Ok(- 1)
}