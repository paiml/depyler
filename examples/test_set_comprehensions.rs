use std::collections::HashSet;
#[doc = "Basic set comprehension"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_set_comp() -> HashSet<i32> {
    (0..5).into_iter().map(|x| x * 2).collect::<HashSet<_>>()
}
#[doc = "Set comprehension with condition"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_comp_with_condition() -> HashSet<i32> {
    (0..10)
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x)
        .collect::<HashSet<_>>()
}
#[doc = "Set comprehension from list"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_comp_from_list() -> HashSet<String> {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "hello".to_string(),
        "python".to_string(),
    ];
    words
        .iter()
        .copied()
        .map(|w| w.to_uppercase())
        .collect::<HashSet<_>>()
}
#[doc = "Set comprehension with complex expression"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_comp_complex_expr() -> HashSet<i32> {
    let numbers = vec![1, 2, 3, 4, 5];
    numbers
        .iter()
        .copied()
        .filter(|x| x < 4)
        .map(|x| x * x + 1)
        .collect::<HashSet<_>>()
}
#[doc = "Set comprehension with expression"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_comp_with_expression() -> HashSet<i32> {
    (0..5)
        .into_iter()
        .filter(|x| x > 0)
        .map(|x| x * x)
        .collect::<HashSet<_>>()
}
