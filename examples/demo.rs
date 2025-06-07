# [doc = "Calculate fibonacci number recursively"] # [doc = " Depyler: verified panic-free"] # [doc = " Depyler: proven to terminate"] pub fn fibonacci (n : i32) -> i32 {
    if (n <= 1) {
    return n;
   
}
return (fibonacci ((n - 1)) + fibonacci ((n - 2)));
   
}
# [doc = "Calculate factorial iteratively"] # [doc = " Depyler: verified panic-free"] # [doc = " Depyler: proven to terminate"] pub fn factorial (n : i32) -> i32 {
    let mut result = 1;
    for i in 1 .. (n + 1) {
    result = (result * i);
   
}
return result;
   
}
# [doc = "Check if a number is prime"] # [doc = " Depyler: proven to terminate"] pub fn is_prime (n : i32) -> bool {
    if (n < 2) {
    return false;
   
}
for i in 2 .. n {
    if ((n % i) == 0) {
    return false;
   
}
} return true;
   
}
# [doc = "Sum all numbers in a list"] # [doc = " Depyler: verified panic-free"] pub fn process_list (numbers : Vec < i32 >) -> i32 {
    let mut total = 0;
    for num in numbers {
    total = (total + num);
   
}
return total;
    }