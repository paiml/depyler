#[doc = "Test basic break statement"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_simple_break()  -> serde_json::Value {
    for i in (0..5) {
    if(i == 3) {
    break;
   
}
} }