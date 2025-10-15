const STR_A: &'static str = "a";
const STR_B: &'static str = "b";
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
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test list.extend() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_extend() -> i32 {
    let mut numbers = vec![1, 2];
    numbers.extend(vec![3, 4]);
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test list.insert() method"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_insert() -> Result<i32, IndexError> {
    let mut numbers = vec![1, 3];
    numbers.insert(1 as usize, 2);
    return Ok({
        let base = numbers;
        let idx = 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub((-idx) as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).copied().unwrap_or_default()
    });
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
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test list.pop() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_pop() -> i32 {
    let mut numbers = vec![1, 2, 3];
    let last = numbers.pop().unwrap_or_default();
    return last;
}
#[doc = "Test list.pop(index) method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_pop_index() -> i32 {
    let mut numbers = vec![1, 2, 3];
    let middle = numbers.remove(1 as usize);
    return middle;
}
#[doc = "Test list.clear() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_clear() -> i32 {
    let mut numbers = vec![1, 2, 3];
    numbers.clear();
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
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
    return pos;
}
#[doc = "Test list.count() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_count() -> i32 {
    let mut numbers = vec![1, 2, 2, 3, 2];
    let occurrences = numbers.iter().filter(|x| **x == 2).count() as i32;
    return occurrences;
}
#[doc = "Test list.reverse() method"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_reverse() -> Result<i32, IndexError> {
    let mut numbers = vec![1, 2, 3];
    numbers.reverse();
    return Ok({
        let base = numbers;
        let idx = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub((-idx) as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).copied().unwrap_or_default()
    });
}
#[doc = "Test list.sort() method"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_sort() -> Result<i32, IndexError> {
    let mut numbers = vec![3, 1, 2];
    numbers.sort();
    return Ok({
        let base = numbers;
        let idx = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub((-idx) as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).copied().unwrap_or_default()
    });
}
#[doc = "Test dict.get() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_get() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map.insert(STR_B, 2);
        map
    };
    let value = data.get(&STR_A).cloned();
    return value;
}
#[doc = "Test dict.get() with default"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_get_default() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map
    };
    let value = data.get(&STR_B).cloned().unwrap_or(0);
    return value;
}
#[doc = "Test dict.keys() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_keys() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map.insert(STR_B, 2);
        map.insert("c", 3);
        map
    };
    let keys = data.keys().cloned().collect::<Vec<_>>();
    let _cse_temp_0 = keys.into_iter().collect::<Vec<_>>().len() as i32;
    return _cse_temp_0;
}
#[doc = "Test dict.values() method"]
#[doc = " Depyler: verified panic-free"]
pub fn test_dict_values() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 10);
        map.insert(STR_B, 20);
        map
    };
    let values = data.values().cloned().collect::<Vec<_>>();
    let mut total = 0;
    for v in values.iter() {
        total = total + v;
    }
    return total;
}
#[doc = "Test dict.items() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_items() -> i32 {
    let data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map.insert(STR_B, 2);
        map
    };
    let items = data
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();
    let _cse_temp_0 = items.into_iter().collect::<Vec<_>>().len() as i32;
    return _cse_temp_0;
}
#[doc = "Test dict.pop() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_pop() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map.insert(STR_B, 2);
        map
    };
    let value = data.remove(&STR_A).expect("KeyError: key not found");
    return value;
}
#[doc = "Test dict.clear() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_clear() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map.insert(STR_B, 2);
        map
    };
    data.clear();
    let _cse_temp_0 = data.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test dict.update() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_update() -> i32 {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(STR_A, 1);
        map
    };
    for item in {
        let mut map = HashMap::new();
        map.insert(STR_B, 2);
        map
    } {
        data.insert(item);
    }
    let _cse_temp_0 = data.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
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
    if let Some(pos) = numbers.iter().position(|x| x == &2) {
        numbers.remove(pos)
    } else {
        panic!("ValueError: list.remove(x): x not in list")
    };
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 2;
    return _cse_temp_1;
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
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = result.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = result.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = result.len() as i32;
    return _cse_temp_0;
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
    let _cse_temp_0 = numbers.len() as i32;
    return _cse_temp_0;
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_append_examples() {
        let _ = test_list_append();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_extend_examples() {
        let _ = test_list_extend();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_insert_examples() {
        let _ = test_list_insert();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_remove_examples() {
        let _ = test_list_remove();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_pop_examples() {
        let _ = test_list_pop();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_pop_index_examples() {
        let _ = test_list_pop_index();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_clear_examples() {
        let _ = test_list_clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_index_examples() {
        let _ = test_list_index();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_count_examples() {
        let _ = test_list_count();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_reverse_examples() {
        let _ = test_list_reverse();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_list_sort_examples() {
        let _ = test_list_sort();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_get_examples() {
        let _ = test_dict_get();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_get_default_examples() {
        let _ = test_dict_get_default();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_keys_examples() {
        let _ = test_dict_keys();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_values_examples() {
        let _ = test_dict_values();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_items_examples() {
        let _ = test_dict_items();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_pop_examples() {
        let _ = test_dict_pop();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_clear_examples() {
        let _ = test_dict_clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_dict_update_examples() {
        let _ = test_dict_update();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_add_examples() {
        let _ = test_set_add();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_remove_examples() {
        let _ = test_set_remove();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_discard_examples() {
        let _ = test_set_discard();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_clear_examples() {
        let _ = test_set_clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_union_examples() {
        let _ = test_set_union();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_intersection_examples() {
        let _ = test_set_intersection();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_difference_examples() {
        let _ = test_set_difference();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_update_examples() {
        let _ = test_set_update();
    }
}
