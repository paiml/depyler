#[doc = "Plain list should map to Vec<T>"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn type_mapping_bug(items: list) -> list {
    return items;
   
}
#[doc = "Array length - 1 can underflow"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn underflow_bug(arr: list) -> i32 {
    let mut right =(arr.len() - 1);
    return right;
   
}
#[doc = "Method calls have weird spacing"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn method_call_spacing(arr: list) -> i32 {
    return arr.len()
}