#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn simple_async() -> i32 {
    42
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn call_async() -> i32 {
    let result = simple_async().await;
    result
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_simple_async_examples() {
    let _ = simple_async();
   
}
#[test] fn test_call_async_examples() {
    let _ = call_async();
   
}
}