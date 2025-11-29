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
#[derive(Debug, Clone)]
pub struct Queue {
    pub _items: Vec<i32>,
    pub _front: i32,
}
impl Queue {
    pub fn new() -> Self {
        Self {
            _items: Vec::new(),
            _front: 0,
        }
    }
    pub fn enqueue(&self, item: i32) {
        self._items.push(item);
    }
    pub fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return ();
        };
        let mut item = self._items[self._front as usize];
        self._front = self._front + 1;
        if self._front > {
            let a = self._items.len();
            let b = 2;
            let q = a / b;
            let r = a % b;
            let r_negative = r < 0;
            let b_negative = b < 0;
            let r_nonzero = r != 0;
            let signs_differ = r_negative != b_negative;
            let needs_adjustment = r_nonzero && signs_differ;
            if needs_adjustment {
                q - 1
            } else {
                q
            }
        } {
            self._items = {
                let s = &self._items;
                let len = s.chars().count() as isize;
                let start_idx = (self._front) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
            self._front = 0;
        };
        return item;
    }
    pub fn front(&self) -> Option<i32> {
        if self.is_empty() {
            return ();
        };
        return self._items[self._front as usize];
    }
    pub fn is_empty(&self) -> bool {
        return self._front >= self._items.len();
    }
    pub fn size(&self) -> i32 {
        return self._items.len().saturating_sub(self._front);
    }
}
#[doc = "Process binary tree level by level using queue"]
pub fn level_order_values(
    tree_values: &Vec<Option<i32>>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = (tree_values.is_empty())
        || (tree_values
            .get(0usize)
            .cloned()
            .unwrap_or_default()
            .is_none());
    if _cse_temp_0 {
        return Ok(vec![]);
    }
    let mut queue = Queue::new();
    let mut result: Vec<i32> = vec![];
    queue.enqueue(0);
    while !queue.is_empty() {
        let index = queue.dequeue();
        if (index.is_none()) || (index >= tree_values.len() as i32) {
            continue;
        }
        let value = tree_values.get(index as usize).cloned().unwrap_or_default();
        if value.is_some() {
            result.push(value);
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            if left_child < tree_values.len() as i32 {
                queue.enqueue(left_child);
            }
            if right_child < tree_values.len() as i32 {
                queue.enqueue(right_child);
            }
        }
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_level_order_values_examples() {
        assert_eq!(level_order_values(vec![]), vec![]);
        assert_eq!(level_order_values(vec![1]), vec![1]);
    }
}
