#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn simple_async() -> i32 {
    42
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn call_async() -> i32 {
    let result: i32 = simple_async().await;
    result
}
#[doc = r" DEPYLER-1216: Auto-generated entry point for standalone compilation"] #[doc = r" This file was transpiled from a Python module without an explicit main."] #[doc = r#" Add a main () function or `if __name__ == "__main__":` block to customize."#] pub fn main () -> Result <(), Box<dyn std::error::Error>>{
    Ok(())
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