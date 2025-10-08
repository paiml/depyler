use std::collections::HashMap;
    #[doc = "Test dictionary with tuple keys"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_tuple_keys()  -> HashMap<Tuple<i32, i32>, String>{
    let d = {
    let mut map = HashMap::new();
    map };
    d.insert((0, 0), "origin");
    d.insert((1, 0), "right");
    d.insert((0, 1), "up");
    d.insert((1, 1), "diagonal");
    d.insert((x, y), "dynamic");
    return d
}