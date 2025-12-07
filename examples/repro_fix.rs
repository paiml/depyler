#[doc = "Function with optional parameter."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process(value: i32, name: &Option<String>) -> String {
    if name.is_none() {
        return format!("value={}", value);
    }
    format!("value={}, name={:?}", value, name)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let maybe_name: Option<String> = None;
    if maybe_name.is_none() {
        println!("{}", "No name provided");
    }
    let result = process(42, maybe_name.as_ref().unwrap());
    println!("{}", result);
}
