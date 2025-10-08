use std::collections::HashSet;
    #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_all_set_features()  -> HashSet<i32>{
    let s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    let s2 = vec ! [4, 5, 6].into_iter().collect::<HashSet<_>>();
    let _cse_temp_0 = s1 | s2;
    let union = _cse_temp_0;
    let _cse_temp_1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set
}
. intersection(& {
    let mut set = HashSet::new();
📄 Source: examples/test_set_summary.py (528 bytes)
📝 Output: examples/test_set_summary.rs (1571 bytes)
⏱️  Parse time: 10ms
📊 Throughput: 48.3 KB/s
⏱️  Total time: 10ms
set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set
}
. difference(& {
    let mut set = HashSet::new();
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set }).cloned().collect();
    let difference = _cse_temp_2;
    let _cse_temp_3 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set
}
. symmetric_difference(& {
    let mut set = HashSet::new();
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set }).cloned().collect();
    let symmetric_diff = _cse_temp_3;
    s1.insert(4);
    s2.remove(& 5);
    let _cse_temp_4 = union | intersection;
    let _cse_temp_5 = _cse_temp_4 | difference;
    let _cse_temp_6 = _cse_temp_5 | symmetric_diff;
    let result = _cse_temp_6;
    return result
}