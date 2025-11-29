use std::collections::HashMap;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Count word frequencies using Counter"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn count_words(text: &str) -> HashMap<String, i32> {
    let words = text
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    words
        .into_iter()
        .fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .collect::<HashMap<_, _>>()
}
#[doc = "Group words by their length using defaultdict"]
#[doc = " Depyler: verified panic-free"]
pub fn group_by_length(words: &Vec<String>) -> HashMap<i32, Vec<String>> {
    let groups = HashMap::new();
    for word in words.iter().cloned() {
        {
            let base = &groups;
            let idx: i32 = word.len() as i32;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .push(word);
    }
    groups.into_iter().collect::<HashMap<_, _>>()
}
#[doc = "Process items using a deque"]
pub fn process_queue(items: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut queue = VecDeque::from(items);
    let mut results = vec![];
    while queue {
        if queue.len() as i32 % 2 == 0 {
            results.push(queue.popleft());
        } else {
            results.push(queue.pop().unwrap_or_default());
        }
    }
    Ok(results)
}
#[doc = "Create sliding windows using deque"]
#[doc = " Depyler: verified panic-free"]
pub fn sliding_window(data: &Vec<i32>, window_size: i32) -> Vec<Vec<i32>> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = window_size > _cse_temp_0;
    if _cse_temp_1 {
        return vec![];
    }
    let mut window = VecDeque::from({
        let base = &data;
        let stop_idx = window_size as isize;
        let stop = if stop_idx < 0 {
            (base.len() as isize + stop_idx).max(0) as usize
        } else {
            stop_idx as usize
        };
        base[..stop.min(base.len())].to_vec()
    });
    let mut windows = vec![window.into_iter().collect::<Vec<_>>()];
    for item in {
        let base = &data;
        let start_idx = window_size as isize;
        let start = if start_idx < 0 {
            (base.len() as isize + start_idx).max(0) as usize
        } else {
            start_idx as usize
        };
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    } {
        window.push(item);
        windows.push(window.into_iter().collect::<Vec<_>>());
    }
    windows
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_process_queue_examples() {
        assert_eq!(process_queue(vec![]), vec![]);
        assert_eq!(process_queue(vec![1]), vec![1]);
    }
}
