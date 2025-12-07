use std::collections::HashMap;
#[doc = "Test dictionary with tuple keys"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_tuple_keys() -> HashMap<(i32, i32), String> {
    let mut d: std::collections::HashMap<(i32, i32), String> = {
        let map = HashMap::new();
        map
    };
    d.insert((0, 0), "origin".to_string());
    d.insert((1, 0), "right".to_string());
    d.insert((0, 1), "up".to_string());
    d.insert((1, 1), "diagonal".to_string());
    let x = 2;
    let y = 3;
    d.insert((x, y), "dynamic".to_string());
    d
}
