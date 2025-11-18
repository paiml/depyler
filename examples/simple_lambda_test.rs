#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_basic_lambdas() -> i32 {
    let square = |x| x * x;
    let result1 = square(5);
    let add = |x, y| x + y;
    let result2 = add(3, 4);
    let get_value = || 42;
    let result3 = get_value();
    result1 + result2 + result3
}
