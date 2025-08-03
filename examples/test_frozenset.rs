use std::collections::HashSet;
    #[doc = "Test frozenset functionality"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_frozenset()  -> serde_json::Value {
    let mut fs1 = std::sync::Arc::new(HashSet::new());
    let mut fs2 = std::sync::Arc::new([1, 2, 3].into_iter().collect::<HashSet<_>>());
    let mut fs3 = std::sync::Arc::new((4, 5, 6).into_iter().collect::<HashSet<_>>());
    return(fs1, fs2, fs3)
}