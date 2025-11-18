use std::collections::HashMap;
use std::collections::IndexMap;
use std::collections::VecDeque;
const STR_B: &'static str = "b";
const STR_APPLE: &'static str = "apple";
const STR_A: &'static str = "a";
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
#[doc = "Test basic deque operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deque_basic() -> Vec<i32> {
    let mut d: deque = VecDeque::from(vec![1, 2, 3]);
    d.push(4);
    d.appendleft(0);
    let result: Vec<i32> = d.into_iter().collect::<Vec<_>>();
    result
}
#[doc = "Test deque pop operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deque_pop() -> (i32, i32) {
    let mut d: deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
    let right: i32 = d.pop().unwrap_or_default();
    let left: i32 = d.popleft();
    (left, right)
}
#[doc = "Test deque extend operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deque_extend() -> Vec<i32> {
    let mut d: deque = VecDeque::from(vec![1, 2, 3]);
    d.extend(vec![4, 5].iter().cloned());
    d.extendleft(vec![0, -1]);
    let result: Vec<i32> = d.into_iter().collect::<Vec<_>>();
    result
}
#[doc = "Test deque rotation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deque_rotate() -> Vec<i32> {
    let mut d: deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
    for _i in 0..2 {
        let item: i32 = d.pop().unwrap_or_default();
        d.appendleft(item);
    }
    let result: Vec<i32> = d.into_iter().collect::<Vec<_>>();
    result
}
#[doc = "Test Counter basic functionality"]
pub fn test_counter_basic() -> Result<HashMap<String, i32>, IndexError> {
    let items: Vec<String> = vec![
        STR_APPLE.to_string(),
        "banana".to_string(),
        STR_APPLE.to_string(),
        "cherry".to_string(),
        "banana".to_string(),
        STR_APPLE.to_string(),
    ];
    let mut counts: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for item in items.iter().cloned() {
        if counts.contains_key(item) {
            {
                let _key = item;
                let _old_val = counts.get(&_key).cloned().unwrap_or_default();
                counts.insert(_key, _old_val + 1);
            }
        } else {
            counts.insert(item, 1);
        }
    }
    Ok(counts)
}
#[doc = "Test getting most common elements"]
pub fn test_counter_most_common(
    items: &Vec<String>,
    n: i32,
) -> Result<Vec<(String, i32)>, IndexError> {
    let mut counts: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for item in items.iter().cloned() {
        if counts.contains_key(item) {
            {
                let _key = item;
                let _old_val = counts.get(&_key).cloned().unwrap_or_default();
                counts.insert(_key, _old_val + 1);
            }
        } else {
            counts.insert(item, 1);
        }
    }
    let mut count_list: Vec<(String, i32)> = vec![];
    for key in counts.keys().cloned().collect::<Vec<_>>() {
        let pair: (String, i32) = (key, counts.get(&key).cloned().unwrap_or_default());
        count_list.push(pair);
    }
    for i in 0..count_list.len() as i32 {
        for j in i + 1..count_list.len() as i32 {
            if count_list.get(j as usize).cloned().unwrap_or_default().1
                > count_list.get(i as usize).cloned().unwrap_or_default().1
            {
                let temp: (String, i32) = count_list.get(i as usize).cloned().unwrap_or_default();
                count_list.insert(
                    (i) as usize,
                    count_list.get(j as usize).cloned().unwrap_or_default(),
                );
                count_list.insert((j) as usize, temp);
            }
        }
    }
    let result: Vec<(String, i32)> = {
        let base = count_list;
        let stop = (n).max(0) as usize;
        base[..stop.min(base.len())].to_vec()
    };
    Ok(result)
}
#[doc = "Test Counter arithmetic operations"]
pub fn test_counter_arithmetic() -> Result<HashMap<String, i32>, IndexError> {
    let counter1: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 3);
        map.insert(STR_B.to_string(), 1);
        map
    };
    let counter2: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map.insert("c".to_string(), 1);
        map
    };
    let mut result: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for key in counter1.keys().cloned().collect::<Vec<_>>() {
        result.insert(key, counter1.get(&key).cloned().unwrap_or_default());
    }
    for key in counter2.keys().cloned().collect::<Vec<_>>() {
        if result.contains_key(&key) {
            {
                let _key = key;
                let _old_val = result.get(&_key).cloned().unwrap_or_default();
                result.insert(
                    _key,
                    _old_val + counter2.get(&key).cloned().unwrap_or_default(),
                );
            }
        } else {
            result.insert(key, counter2.get(&key).cloned().unwrap_or_default());
        }
    }
    Ok(result)
}
#[doc = "Test defaultdict with int default"]
#[doc = " Depyler: verified panic-free"]
pub fn test_defaultdict_int() -> HashMap<String, i32> {
    let mut counts: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    let words: Vec<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "hello".to_string(),
        "python".to_string(),
        "world".to_string(),
        "hello".to_string(),
    ];
    for word in words.iter().cloned() {
        let current: i32 = counts.get(word).cloned().unwrap_or(0);
        counts.insert(word, current + 1);
    }
    counts
}
#[doc = "Test defaultdict with list default"]
pub fn test_defaultdict_list() -> Result<HashMap<String, Vec<i32>>, IndexError> {
    let mut groups: HashMap<String, Vec<i32>> = {
        let map = HashMap::new();
        map
    };
    let pairs: Vec<(String, i32)> =
        vec![(STR_A, 1), (STR_B, 2), (STR_A, 3), (STR_B, 4), (STR_A, 5)];
    for pair in pairs.iter().cloned() {
        let key: String = pair.0;
        let value: i32 = pair.1;
        if !groups.contains_key(&key) {
            groups.insert(key, vec![]);
        }
        groups.get(&key).cloned().unwrap_or_default().push(value);
    }
    Ok(groups)
}
#[doc = "Test OrderedDict basic operations"]
#[doc = " Depyler: verified panic-free"]
pub fn test_ordereddict_basic() -> Vec<(String, i32)> {
    let mut od: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    od.insert("first".to_string(), 1);
    od.insert("second".to_string(), 2);
    od.insert("third".to_string(), 3);
    let mut result: Vec<(String, i32)> = vec![];
    for key in od.keys().cloned().collect::<Vec<_>>() {
        let pair: (String, i32) = (key, od.get(&key).cloned().unwrap_or_default());
        result.push(pair);
    }
    result
}
#[doc = "Test moving item to end in OrderedDict"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_ordereddict_move_to_end() -> Vec<String> {
    let mut od: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert(STR_A.to_string(), 1);
        map.insert(STR_B.to_string(), 2);
        map.insert("c".to_string(), 3);
        map
    };
    let value: i32 = od.remove(STR_A).expect("KeyError: key not found");
    od.insert(STR_A, value);
    let keys: Vec<String> = od.keys().cloned().collect::<Vec<_>>();
    keys
}
#[doc = "Test ChainMap-like lookup(manual)"]
#[doc = " Depyler: proven to terminate"]
pub fn test_chainmap<'a, 'b>(
    dict1: &'a HashMap<String, i32>,
    dict2: &'b HashMap<String, i32>,
) -> Result<i32, IndexError> {
    let key: String = "x".to_string();
    let _cse_temp_0 = dict1.contains_key(&key);
    if _cse_temp_0 {
        Ok(dict1.get(&key).cloned().unwrap_or_default())
    } else {
        let _cse_temp_1 = dict2.contains_key(&key);
        if _cse_temp_1 {
            Ok(dict2.get(&key).cloned().unwrap_or_default())
        } else {
            Ok(-1)
        }
    }
}
#[doc = "Count word frequencies using Counter concept"]
pub fn word_frequency_counter(text: &str) -> Result<HashMap<String, i32>, IndexError> {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut freq: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        if freq.contains_key(word) {
            {
                let _key = word;
                let _old_val = freq.get(&_key).cloned().unwrap_or_default();
                freq.insert(_key, _old_val + 1);
            }
        } else {
            freq.insert(word, 1);
        }
    }
    Ok(freq)
}
#[doc = "Group words by first letter using defaultdict concept"]
pub fn group_by_first_letter(
    words: &Vec<String>,
) -> Result<HashMap<String, Vec<String>>, IndexError> {
    let mut groups: HashMap<String, Vec<String>> = {
        let map = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        if word.len() as i32 == 0 {
            continue;
        }
        let first_letter: String = {
            let base = &word;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        };
        if !groups.contains_key(&first_letter) {
            groups.insert(first_letter, vec![]);
        }
        groups
            .get(first_letter as usize)
            .cloned()
            .unwrap_or_default()
            .push(word);
    }
    Ok(groups)
}
#[doc = "Use deque as a stack(LIFO)"]
#[doc = " Depyler: verified panic-free"]
pub fn test_deque_as_stack() -> Vec<i32> {
    let mut stack: deque = VecDeque::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    let mut result: Vec<i32> = vec![];
    while stack.len() as i32 > 0 {
        let item: i32 = stack.pop().unwrap_or_default();
        result.push(item);
    }
    result
}
#[doc = "Use deque as a queue(FIFO)"]
#[doc = " Depyler: verified panic-free"]
pub fn test_deque_as_queue() -> Vec<i32> {
    let mut queue: deque = VecDeque::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    let mut result: Vec<i32> = vec![];
    while queue.len() as i32 > 0 {
        let item: i32 = queue.popleft();
        result.push(item);
    }
    result
}
#[doc = "Manual implementation of LRU cache concept using deque"]
#[doc = " Depyler: verified panic-free"]
pub fn test_lru_cache_manual(cache_size: i32) -> Vec<i32> {
    let mut cache: deque = VecDeque::new();
    let max_size: i32 = cache_size;
    let items: Vec<i32> = vec![1, 2, 3, 1, 4, 2, 5, 1, 6];
    let mut result: Vec<i32> = vec![];
    for item in items.iter().cloned() {
        let mut found: bool = false;
        for cached in cache.iter().cloned() {
            if cached == item {
                found = true;
                break;
            }
        }
        if !found {
            cache.push(item);
            if cache.len() as i32 > max_size {
                let evicted: i32 = cache.popleft();
            }
        }
        result.push(item);
    }
    result
}
#[doc = "Run all collections module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_collections_features() -> Result<(), Box<dyn std::error::Error>> {
    let deque_basic: Vec<i32> = test_deque_basic();
    let deque_pops: (i32, i32) = test_deque_pop();
    let deque_extended: Vec<i32> = test_deque_extend();
    let deque_rotated: Vec<i32> = test_deque_rotate();
    let mut counts: HashMap<String, i32> = test_counter_basic();
    let items: Vec<String> = vec![
        STR_A.to_string(),
        STR_B.to_string(),
        STR_A.to_string(),
        "c".to_string(),
        STR_A.to_string(),
        STR_B.to_string(),
        "d".to_string(),
        STR_A.to_string(),
    ];
    let most_common: Vec<(String, i32)> = test_counter_most_common(&items, 2);
    let merged: HashMap<String, i32> = test_counter_arithmetic();
    let int_default: HashMap<String, i32> = test_defaultdict_int();
    let list_default: HashMap<String, Vec<i32>> = test_defaultdict_list();
    let ordered: Vec<(String, i32)> = test_ordereddict_basic();
    let moved: Vec<String> = test_ordereddict_move_to_end();
    let d1: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("x".to_string(), 1);
        map.insert("y".to_string(), 2);
        map
    };
    let d2: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("y".to_string(), 3);
        map.insert("z".to_string(), 4);
        map
    };
    let chain_result: i32 = test_chainmap(&d1, &d2);
    let text: String = "hello world hello python world".to_string();
    let mut freq: HashMap<String, i32> = word_frequency_counter(text);
    let words: Vec<String> = vec![
        STR_APPLE.to_string(),
        "banana".to_string(),
        "apricot".to_string(),
        "blueberry".to_string(),
        "cherry".to_string(),
    ];
    let grouped: HashMap<String, Vec<String>> = group_by_first_letter(&words);
    let stack_result: Vec<i32> = test_deque_as_stack();
    let queue_result: Vec<i32> = test_deque_as_queue();
    let lru_result: Vec<i32> = test_lru_cache_manual(3);
    println!("{}", "All collections module tests completed successfully");
    Ok(())
}
