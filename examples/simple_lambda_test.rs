#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_basic_lambdas()  -> DynamicType {
    let result1 = square(5);
    let result2 = add(3, 4);
    let result3 = get_value();
    let _cse_temp_0 = result1 + result2 + result3;
    return _cse_temp_0
}