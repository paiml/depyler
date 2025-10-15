#[doc = "Classify a number as zero, positive, or negative."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn classify_number(n: i32) -> String {
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
        return "zero".to_string();
    } else {
        let _cse_temp_1 = n > 0;
        if _cse_temp_1 {
            return "positive".to_string();
        } else {
            return "negative".to_string();
        }
    }
}
