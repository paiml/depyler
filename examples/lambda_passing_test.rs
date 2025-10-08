#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn apply_func(x: i32, f: DynamicType)  -> i32 {
    return f(x);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_lambda_passing()  -> DynamicType {
    let mut _inline_x = 10;
    let mut _inline_f = | x | x * 2;
    let result1 = f(_inline_x);
    _inline_x = 5;
    _inline_f = | x | x + 10;
    let result2 = f(_inline_x);
    let triple = | x | x * 3;
    _inline_x = 7;
    _inline_f = triple;
    let result3 = f(_inline_x);
    let _cse_temp_0 = result1 + result2 + result3;
    return _cse_temp_0
}