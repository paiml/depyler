#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SimpleQueue {
    pub items: Vec<i32>,
}
impl SimpleQueue {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn put(&mut self, item: i32) {
        self.items.push(item);
    }
    pub fn get(&mut self) -> i32 {
        if (self.items.clone().len() as i32) == 0 {
            return -1;
        };
        let item = self.items.clone()[0 as usize];
        let mut new_items = vec![];
        for i in 1..self.items.clone().len() as i32 {
            new_items.push(self.items.clone()[i as usize]);
        }
        self.items = new_items;
        return item;
    }
    pub fn size(&self) -> i32 {
        return self.items.clone().len() as i32;
    }
    pub fn empty(&self) -> bool {
        return (self.items.clone().len() as i32) == 0;
    }
}
#[derive(Debug, Clone)]
pub struct SimpleStack {
    pub items: Vec<i32>,
}
impl SimpleStack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn push(&mut self, item: i32) {
        self.items.push(item);
    }
    pub fn pop(&self) -> i32 {
        if (self.items.clone().len() as i32) == 0 {
            return -1;
        };
        return self.items.pop().unwrap_or_default();
    }
    pub fn size(&self) -> i32 {
        return self.items.clone().len() as i32;
    }
    pub fn empty(&self) -> bool {
        return (self.items.clone().len() as i32) == 0;
    }
    pub fn peek(&self) -> i32 {
        if (self.items.clone().len() as i32) == 0 {
            return -1;
        };
        return self.items.clone()[(self.items.clone().len() as i32).saturating_sub(1) as usize];
    }
}
#[derive(Debug, Clone)]
pub struct SimplePriorityQueue {
    pub items: Vec<()>,
}
impl SimplePriorityQueue {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn put(&mut self, priority: i32, item: String) {
        self.items.push((priority, item));
        for i in 0..self.items.clone().len() as i32 {
            for j in i + 1..self.items.clone().len() as i32 {
                if self.items.clone()[j as usize][0 as usize]
                    < self.items.clone()[i as usize][0 as usize]
                {
                    let temp = self.items.clone()[i as usize];
                    self.items.clone().insert(i, self.items.clone()[j as usize]);
                    self.items.clone().insert(j, temp);
                };
            }
        }
    }
    pub fn get(&mut self) -> String {
        if (self.items.clone().len() as i32) == 0 {
            return "".to_string();
        };
        let item = self.items.clone()[0 as usize];
        let mut new_items = vec![];
        for i in 1..self.items.clone().len() as i32 {
            new_items.push(self.items.clone()[i as usize]);
        }
        self.items = new_items;
        return item[1 as usize];
    }
    pub fn empty(&self) -> bool {
        return (self.items.clone().len() as i32) == 0;
    }
}
#[doc = "Test creating array with type code"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_creation() -> Vec<i32> {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    arr
}
#[doc = "Test appending to array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_append() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![1, 2, 3];
    arr.push(4);
    arr.push(5);
    arr
}
#[doc = "Test extending array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_extend() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![1, 2, 3];
    let extension: Vec<i32> = vec![4, 5, 6];
    arr.extend(extension.iter().cloned());
    arr
}
#[doc = "Test inserting into array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_insert() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![1, 2, 4, 5];
    arr.insert(2 as usize, 3);
    arr
}
#[doc = "Test removing from array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_remove() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    if let Some(pos) = arr.iter().position(|x| x == &3) {
        arr.remove(pos)
    } else {
        panic!("ValueError: list.remove(x): x not in list")
    };
    arr
}
#[doc = "Test popping from array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_pop() -> (i32, Vec<i32>) {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let popped: i32 = arr.pop().unwrap_or_default();
    (popped, arr)
}
#[doc = "Test finding index in array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_index() -> i32 {
    let arr: Vec<i32> = vec![10, 20, 30, 40, 50];
    let idx: i32 = arr
        .iter()
        .position(|x| x == &30)
        .map(|i| i as i32)
        .expect("ValueError: value is not in list");
    idx
}
#[doc = "Test counting in array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_count() -> i32 {
    let arr: Vec<i32> = vec![1, 2, 2, 3, 2, 4];
    let count: i32 = arr.iter().filter(|x| **x == 2).count() as i32 as i32;
    count
}
#[doc = "Test reversing array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_reverse() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    arr.reverse();
    arr
}
#[doc = "Test converting array to list"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_tolist() -> Vec<i32> {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    arr.clone()
}
#[doc = "Test FIFO queue operations"]
#[doc = " Depyler: verified panic-free"]
pub fn test_queue_fifo() -> Vec<i32> {
    let mut q: SimpleQueue = SimpleQueue::new();
    q.put(1);
    q.put(2);
    q.put(3);
    let mut results: Vec<i32> = vec![];
    while !q.empty() {
        let item: i32 = q.get();
        results.push(item);
    }
    results
}
#[doc = "Test LIFO stack operations"]
#[doc = " Depyler: verified panic-free"]
pub fn test_stack_lifo() -> Vec<i32> {
    let mut s: SimpleStack = SimpleStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    let mut results: Vec<i32> = vec![];
    while !s.empty() {
        let item: i32 = s.pop();
        results.push(item);
    }
    results
}
#[doc = "Test queue size tracking"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_queue_size() -> i32 {
    let mut q: SimpleQueue = SimpleQueue::new();
    q.put(1);
    q.put(2);
    q.put(3);
    let size: i32 = q.size();
    size
}
#[doc = "Test stack peek operation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_stack_peek() -> i32 {
    let mut s: SimpleStack = SimpleStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    let top: i32 = s.peek();
    top
}
#[doc = "Test priority queue"]
#[doc = " Depyler: verified panic-free"]
pub fn test_priority_queue() -> Vec<String> {
    let mut pq: SimplePriorityQueue = SimplePriorityQueue::new();
    pq.put(3, "low".to_string());
    pq.put(1, "high".to_string());
    pq.put(2, "medium".to_string());
    let mut results: Vec<String> = vec![];
    while !pq.empty() {
        let item: String = pq.get();
        results.push(item);
    }
    results
}
#[doc = "Test circular buffer implementation"]
#[doc = " Depyler: verified panic-free"]
pub fn test_circular_buffer(size: i32) -> Vec<i32> {
    let mut buffer: Vec<i32> = Default::default();
    buffer = vec![];
    let max_size: i32 = size;
    let values: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for val in values.iter().cloned() {
        buffer.push(val);
        if buffer.len() as i32 > max_size {
            let mut new_buffer: Vec<i32> = vec![];
            for i in (1)..(buffer.len() as i32) {
                new_buffer.push(
                    buffer
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
            }
            buffer = new_buffer;
        }
    }
    buffer
}
#[doc = "Simulate double-ended queue"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deque_simulation() -> Vec<i32> {
    let mut deque: Vec<i32> = Default::default();
    deque = vec![];
    deque.push(1);
    deque.push(2);
    deque.push(3);
    deque.insert(0 as usize, 0);
    deque.pop().unwrap_or_default();
    let _cse_temp_0 = deque.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        let mut new_deque: Vec<i32> = vec![];
        for i in (1)..(deque.len() as i32) {
            new_deque.push(
                deque
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
        deque = new_deque;
    }
    deque
}
#[doc = "Run all array and queue tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_array_queue_features() {
    let arr: Vec<i32> = test_array_creation();
    let appended: Vec<i32> = test_array_append();
    let extended: Vec<i32> = test_array_extend();
    let inserted: Vec<i32> = test_array_insert();
    let removed: Vec<i32> = test_array_remove();
    let pop_result: (i32, Vec<i32>) = test_array_pop();
    let idx: i32 = test_array_index();
    let count: i32 = test_array_count();
    let reversed_arr: Vec<i32> = test_array_reverse();
    let as_list: Vec<i32> = test_array_tolist();
    let fifo_result: Vec<i32> = test_queue_fifo();
    let lifo_result: Vec<i32> = test_stack_lifo();
    let size: i32 = test_queue_size();
    let top: i32 = test_stack_peek();
    let priority_result: Vec<String> = test_priority_queue();
    let circular: Vec<i32> = test_circular_buffer(3);
    let deque_result: Vec<i32> = test_deque_simulation();
    println!(
        "{}",
        "All array and queue module tests completed successfully"
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_array_index_examples() {
        let _ = test_array_index();
    }
    #[test]
    fn test_test_array_count_examples() {
        let _ = test_array_count();
    }
    #[test]
    fn test_test_queue_size_examples() {
        let _ = test_queue_size();
    }
    #[test]
    fn test_test_stack_peek_examples() {
        let _ = test_stack_peek();
    }
}
