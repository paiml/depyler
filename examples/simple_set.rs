use std::collections::HashSet;
    #[doc = "Test just set literal"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_simple_set()  -> DynamicType {
    let s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    return s
}