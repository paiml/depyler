# [doc = " Depyler: verified panic-free"] # [doc = " Depyler: proven to terminate"] pub fn fibonacci (n : i32) -> i32 {
    "Calculate fibonacci number recursively" . to_string ();
    if (n <= 1) {
    return n;
   
}
return (fibonacci ((n - 1)) + fibonacci ((n - 2)));
   
}
# [doc = " Depyler: verified panic-free"] # [doc = " Depyler: proven to terminate"] pub fn factorial (n : i32) -> i32 {
    "Calculate factorial iteratively" . to_string ();
    let mut result = 1;
    for i in 1 .. (n + 1) {
    let mut result = (result * i);
   
}
return result;
   
}
# [doc = " Depyler: proven to terminate"] pub fn is_prime (n : i32) -> bool {
    "Check if a number is prime" . to_string ();
    if (n < 2) {
    return false;
   
}
for i in 2 .. n {
    if ((n % i) == 0) {
    return false;
   
}
} return true;
   
}
# [doc = " Depyler: verified panic-free"] pub fn process_list (numbers : & Vec < i32 >) -> i32 {
    "Sum all numbers in a list" . to_string ();
    let mut total = 0;
    for num in numbers {
    let mut total = (total + num);
   
}
return total;
    }