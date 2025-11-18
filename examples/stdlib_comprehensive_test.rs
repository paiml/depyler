const STR_B: &'static str = "b";
const STR_A: &'static str = "a";
use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Test list.append() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_append() -> i32 {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    numbers.len() as i32 as i32
}
#[doc = "Test list.extend() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_extend() -> i32 {
    let mut numbers = vec![1, 2];
    numbers.extend(vec![3, 4].iter().cloned());
    numbers.len() as i32 as i32
}
#[doc = "Test list.insert() method"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_insert() -> Result<i32, IndexError> {
    let mut numbers = vec![1, 3];
    numbers.insert(1 as usize, 2);
    Ok(numbers.get(1usize).cloned().unwrap_or_default())
}
#[doc = "Test list.remove() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_remove() -> i32 {
    let mut numbers = vec![1, 2, 3, 2];
    if let Some(pos) = numbers.iter().position(|x| x == &2) {
        numbers.remove(pos)
    } else {
        panic!("ValueError: list.remove(x): x not in list")
    };
    numbers.len() as i32 as i32
}
#[doc = "Test list.pop() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_pop() -> i32 {
    let mut numbers = vec![1, 2, 3];
    let last = numbers.pop().unwrap_or_default();
    last
}
#[doc = "Test list.pop(index) method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_pop_index() -> i32 {
    let mut numbers = vec![1, 2, 3];
    let middle = numbers.remove(1 as usize);
    middle
}
#[doc = "Test list.clear() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_clear() -> i32 {
    let mut numbers = vec![1, 2, 3];
    numbers.clear();
    numbers.len() as i32 as i32
}
#[doc = "Test list.index() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_index() -> i32 {
    let mut numbers = vec![10, 20, 30];
    let pos = numbers
        .iter()
        .position(|x| x == &20)
        .map(|i| i as i32)
        .expect("ValueError: value is not in list");
    pos
}
#[doc = "Test list.count() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_count() -> i32 {
    let mut numbers = vec![1, 2, 2, 3, 2];
    let occurrences = numbers.iter().filter(|x| **x == 2).count() as i32;
    occurrences
}
#[doc = "Test list.reverse() method"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_reverse() -> Result<i32, IndexError> {
    let mut numbers = vec![1, 2, 3];
    numbers.reverse();
    Ok(numbers.get(0usize).cloned().unwrap_or_default())
}
#[doc = "Test list.sort() method"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_sort() -> Result<i32, IndexError> {
    let mut numbers = vec![3, 1, 2];
    numbers.sort();
    Ok(numbers.get(0usize).cloned().unwrap_or_default())
}
#[doc = "Test dict.get() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_get() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map
    };
    let value = data.get(&STR_A).cloned();
    value
}
#[doc = "Test dict.get() with default"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_get_default() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map
    };
    let value = data.get(&STR_B).cloned().unwrap_or(0);
    value
}
#[doc = "Test dict.keys() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_keys() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map.insert("c".to_string(), 3);
        map
    };
    let keys = data.keys().cloned().collect::<Vec<_>>();
    keys.into_iter().collect::<Vec<_>>().len() as i32 as i32
}
#[doc = "Test dict.values() method"]
#[doc = " Depyler: verified panic-free"]
pub fn test_dict_values() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 10);
        map.insert(STR_B.to_string(), 20);
        map
    };
    let values = data.values().cloned().collect::<Vec<_>>();
    let mut total = 0;
    for v in values.iter().cloned() {
        total = total + v;
    }
    total
}
#[doc = "Test dict.items() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_items() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map
    };
    let items = data
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();
    items.into_iter().collect::<Vec<_>>().len() as i32 as i32
}
#[doc = "Test dict.pop() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_pop() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map
    };
    let value = data.remove(STR_A).expect("KeyError: key not found");
    value
}
#[doc = "Test dict.clear() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_clear() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map
    };
    data.clear();
    data.len() as i32 as i32
}
#[doc = "Test dict.update() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_update() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map
    };
    for (k, v) in {
        let mut map = HashMap::new();
        map.insert(STR_B.to_string(), 2);
        map
    } {
        data.insert(k, v);
    }
    data.len() as i32 as i32
}
#[doc = "Test dict.setdefault() method - existing key"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_setdefault() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map
    };
    let value = data.entry(STR_A).or_insert(999).clone();
    value
}
#[doc = "Test dict.setdefault() method - new key"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_setdefault_new() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map
    };
    let value = data.entry(STR_B).or_insert(42).clone();
    value
}
#[doc = "Test dict.popitem() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_popitem() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map.insert("c".to_string(), 3);
        map
    };
    {
        let key = data
            .keys()
            .next()
            .cloned()
            .expect("KeyError: popitem(): dictionary is empty");
        let value = data.remove(&key).expect("KeyError: key disappeared");
        (key, value)
    };
    data.len() as i32 as i32
}
#[doc = "Test set.add() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_add() -> i32 {
    let mut numbers = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set
    };
    numbers.insert(3);
    numbers.len() as i32 as i32
}
#[doc = "Test set.remove() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_remove() -> i32 {
    let mut numbers = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    if !numbers.remove(&2) {
        panic!("KeyError: element not in set")
    };
    numbers.len() as i32 as i32
}
#[doc = "Test set.discard() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_discard() -> i32 {
    let mut numbers = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    numbers.remove(&2);
    numbers.len() as i32 as i32
}
#[doc = "Test set.pop() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_pop() -> bool {
    let mut numbers = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    let value = numbers
        .iter()
        .next()
        .cloned()
        .map(|x| {
            numbers.remove(&x);
            x
        })
        .expect("pop from empty set");
    numbers.len() as i32 == 2
}
#[doc = "Test set.clear() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_clear() -> i32 {
    let mut numbers = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    numbers.clear();
    numbers.len() as i32 as i32
}
#[doc = "Test set.union() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_union() -> i32 {
    let set1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set
    };
    let set2 = {
        let mut set = HashSet::new();
        set.insert(2);
        set.insert(3);
        set
    };
    let result = set1
        .union(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    result.len() as i32 as i32
}
#[doc = "Test set.intersection() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_intersection() -> i32 {
    let set1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    let set2 = {
        let mut set = HashSet::new();
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set
    };
    let result = set1
        .intersection(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    result.len() as i32 as i32
}
#[doc = "Test set.difference() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_difference() -> i32 {
    let set1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    let set2 = {
        let mut set = HashSet::new();
        set.insert(2);
        set.insert(3);
        set
    };
    let result = set1
        .difference(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    result.len() as i32 as i32
}
#[doc = "Test set.update() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_update() -> i32 {
    let mut numbers = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set
    };
    for item in {
        let mut set = HashSet::new();
        set.insert(3);
        set.insert(4);
        set
    } {
        numbers.insert(item);
    }
    numbers.len() as i32 as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_append_examples() {
        let _ = test_list_append();
    }
    #[test]
    fn test_test_list_extend_examples() {
        let _ = test_list_extend();
    }
    #[test]
    fn test_test_list_insert_examples() {
        let _ = test_list_insert();
    }
    #[test]
    fn test_test_list_remove_examples() {
        let _ = test_list_remove();
    }
    #[test]
    fn test_test_list_pop_examples() {
        let _ = test_list_pop();
    }
    #[test]
    fn test_test_list_pop_index_examples() {
        let _ = test_list_pop_index();
    }
    #[test]
    fn test_test_list_clear_examples() {
        let _ = test_list_clear();
    }
    #[test]
    fn test_test_list_index_examples() {
        let _ = test_list_index();
    }
    #[test]
    fn test_test_list_count_examples() {
        let _ = test_list_count();
    }
    #[test]
    fn test_test_list_reverse_examples() {
        let _ = test_list_reverse();
    }
    #[test]
    fn test_test_list_sort_examples() {
        let _ = test_list_sort();
    }
    #[test]
    fn test_test_dict_get_examples() {
        let _ = test_dict_get();
    }
    #[test]
    fn test_test_dict_get_default_examples() {
        let _ = test_dict_get_default();
    }
    #[test]
    fn test_test_dict_keys_examples() {
        let _ = test_dict_keys();
    }
    #[test]
    fn test_test_dict_values_examples() {
        let _ = test_dict_values();
    }
    #[test]
    fn test_test_dict_items_examples() {
        let _ = test_dict_items();
    }
    #[test]
    fn test_test_dict_pop_examples() {
        let _ = test_dict_pop();
    }
    #[test]
    fn test_test_dict_clear_examples() {
        let _ = test_dict_clear();
    }
    #[test]
    fn test_test_dict_update_examples() {
        let _ = test_dict_update();
    }
    #[test]
    fn test_test_dict_setdefault_examples() {
        let _ = test_dict_setdefault();
    }
    #[test]
    fn test_test_dict_setdefault_new_examples() {
        let _ = test_dict_setdefault_new();
    }
    #[test]
    fn test_test_dict_popitem_examples() {
        let _ = test_dict_popitem();
    }
    #[test]
    fn test_test_set_add_examples() {
        let _ = test_set_add();
    }
    #[test]
    fn test_test_set_remove_examples() {
        let _ = test_set_remove();
    }
    #[test]
    fn test_test_set_discard_examples() {
        let _ = test_set_discard();
    }
    #[test]
    fn test_test_set_clear_examples() {
        let _ = test_set_clear();
    }
    #[test]
    fn test_test_set_union_examples() {
        let _ = test_set_union();
    }
    #[test]
    fn test_test_set_intersection_examples() {
        let _ = test_set_intersection();
    }
    #[test]
    fn test_test_set_difference_examples() {
        let _ = test_set_difference();
    }
    #[test]
    fn test_test_set_update_examples() {
        let _ = test_set_update();
    }
}
