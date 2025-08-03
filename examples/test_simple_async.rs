#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn simple_async()  -> i32 {
    return 42;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn call_async()  -> i32 {
    let mut result = simple_async().await;
    return result;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_simple_async_examples() {
    let _ = simple_async();
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_call_async_examples() {
    let _ = call_async();
   
}
}