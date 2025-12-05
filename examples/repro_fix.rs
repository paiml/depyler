#[doc = "Count punctuation characters in text."]
#[doc = " Depyler: verified panic-free"]
pub fn find_punctuation<'b, 'a>(text: &'a str, punctuation: &'b str) -> i32 {
    let mut count: i32 = 0;
    for char in text.chars() {
        for p in punctuation.chars() {
            if char == p {
                count = count + 1;
            }
        }
    }
    count
}
