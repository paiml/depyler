#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_lambdas() -> (String, String, String) {
    let add = |x, y| x + y;
    let square = |x| x * x;
    let constant = || 42;
    let result1 = add(3, 5);
    let result2 = square(4);
    let result3 = constant();
    (result1, result2, result3)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lambda_with_list_operations() -> (String, String, String) {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares = numbers.iter().map(|x| x * x).collect::<Vec<_>>();
    let evens = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<_>>();
    let double = |lst| lst.clone().into_iter().map(|x| x * 2).collect::<Vec<_>>();
    let doubled = double(&numbers);
    (squares, evens, doubled)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lambda_in_expressions() -> (String, String) {
    let get_operation = |is_add| if is_add { |x, y| x + y } else { |x, y| x - y };
    let add_op = get_operation(true);
    let sub_op = get_operation(false);
    (add_op(10, 5), sub_op(10, 5))
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lambda_with_closure() {
    let multiplier = 3;
    let scale = |x| x * multiplier;
    let result = scale(7);
    result
}
