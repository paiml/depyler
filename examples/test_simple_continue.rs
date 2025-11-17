#[doc = "Test basic continue statement"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_continue() {
    for i in 0..5 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }
}
