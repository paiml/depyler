# [doc = "Find target in sorted array, return -1 if not found."] pub fn binary_search (arr : Vec < i32 > , target : i32) -> i32 {
    let mut left = 0;
    let mut right = (arr . len () - 1);
    while (left <= right) {
    let mut mid = ((left + right) / 2);
    if (arr . get (mid as usize) . copied () . unwrap_or_default () == target) {
    return mid;
   
}
else {
    if (arr . get (mid as usize) . copied () . unwrap_or_default () < target) {
    left = (mid + 1);
   
}
else {
    right = (mid - 1);
   
}
}
}
return - 1;
    }