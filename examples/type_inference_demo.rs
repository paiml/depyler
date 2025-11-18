use serde_json;
#[doc = "Numeric operations suggest int/float types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_numbers<'a, 'b>(a: &'a serde_json::Value, b: &'b serde_json::Value) -> i32 {
    let mut result = a + b;
    let _cse_temp_0 = result * 2;
    result = _cse_temp_0;
    result
}
#[doc = "String methods suggest str type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn handle_text(message: &str) -> String {
    let formatted = message.to_uppercase();
    formatted
}
#[doc = "List operations suggest list type."]
#[doc = " Depyler: verified panic-free"]
pub fn work_with_list(data: &mut serde_json::Value) -> i32 {
    data.push(100);
    let mut total = 0;
    for item in data.iter().cloned() {
        total = total + item;
    }
    total
}
#[doc = "Boolean context suggests bool type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn check_condition(flag: &serde_json::Value) -> i32 {
    if flag {
        1
    } else {
        0
    }
}
