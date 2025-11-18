use serde_json;
use std::collections::HashSet;
#[doc = "Test frozenset functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_frozenset() -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    let fs1 = std::sync::Arc::new(HashSet::<i32>::new());
    let fs2 = std::sync::Arc::new(vec![1, 2, 3].into_iter().collect::<HashSet<_>>());
    let fs3 = std::sync::Arc::new((4, 5, 6).into_iter().collect::<HashSet<_>>());
    (fs1, fs2, fs3)
}
