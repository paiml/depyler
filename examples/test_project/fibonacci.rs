use serde_json;
use std::collections::HashMap;
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
#[doc = "Calculate nth Fibonacci number recursively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_recursive(n: i32) -> i32 {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        0
    } else {
        let _cse_temp_1 = n == 1;
        if _cse_temp_1 {
            1
        } else {
            fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
        }
    }
}
#[doc = "Calculate nth Fibonacci number iteratively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_iterative(n: i32) -> i32 {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return 0;
    } else {
        let _cse_temp_1 = n == 1;
        if _cse_temp_1 {
            return 1;
        }
    }
    let (mut prev, mut curr) = (0, 1);
    for __sanitized in 2..n + 1 {
        (prev, curr) = (curr, prev + curr);
    }
    curr
}
#[doc = "Generate Fibonacci sequence up to n terms."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_sequence(limit: i32) -> Vec<i32> {
    let _cse_temp_0 = limit <= 0;
    if _cse_temp_0 {
        return vec![];
    }
    let mut sequence: Vec<i32> = vec![];
    let (mut a, mut b) = (0, 1);
    for __sanitized in 0..limit {
        sequence.push(a);
        (a, b) = (b, a + b);
    }
    sequence
}
#[doc = "Generate Fibonacci numbers as an iterator."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    a: i32,
    b: i32,
    count: i32,
    limit: Option<i32>,
}
#[doc = " Generator function - returns Iterator"]
pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = i32> {
    FibonacciGeneratorState {
        state: 0,
        a: 0,
        b: 0,
        count: 0,
        limit: *limit,
    }
}
impl Iterator for FibonacciGeneratorState {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                let _tuple_temp = (0, 1);
                self.a = _tuple_temp.0;
                self.b = _tuple_temp.1;
                self.count = 0;
                self.state = 1;
                self.next()
            }
            1 => {
                if (self.limit.is_none()) || (self.count < self.limit.unwrap_or(i32::MAX)) {
                    let result = self.a;
                    let _tuple_temp = (self.b, self.a + self.b);
                    self.a = _tuple_temp.0;
                    self.b = _tuple_temp.1;
                    self.count = self.count + 1;
                    return Some(result);
                } else {
                    self.state = 2;
                    None
                }
            }
            _ => None,
        }
    }
}
#[doc = "Calculate Fibonacci with memoization."]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_memoized(
    n: i32,
    mut memo: Option<HashMap<serde_json::Value, serde_json::Value>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    if memo.is_none() {
        memo = {
            let map = HashMap::new();
            map
        };
    }
    let _cse_temp_0 = memo.get(&n).is_some();
    if _cse_temp_0 {
        return Ok(memo.get(&n).cloned().unwrap_or_default());
    }
    let _cse_temp_1 = n <= 0;
    if _cse_temp_1 {
        return Ok(0);
    } else {
        let _cse_temp_2 = n == 1;
        if _cse_temp_2 {
            return Ok(1);
        }
    }
    let _cse_temp_3 = fibonacci_memoized(n - 1, &memo)? + fibonacci_memoized(n - 2, &memo)?;
    let result = _cse_temp_3;
    memo.insert(n, serde_json::json!(result));
    Ok(result)
}
#[doc = "Find the index of a target value in Fibonacci sequence."]
#[doc = " Depyler: verified panic-free"]
pub fn find_fibonacci_index(target: i32) -> Option<i32> {
    let _cse_temp_0 = target < 0;
    if _cse_temp_0 {
        return None;
    }
    let (mut a, mut b) = (0, 1);
    let mut index = 0;
    while a < target {
        (a, b) = (b, a + b);
        index = index + 1;
    }
    if a == target {
        Some(index)
    } else {
        None
    }
}
#[doc = "Check if a number is in the Fibonacci sequence."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_fibonacci_number(num: i32) -> bool {
    let mut is_perfect_square;
    is_perfect_square = |x: i64| -> bool {
        let root = ((x as f64).powf(0.5 as f64)) as i32;
        root * root == x
    };
    let _cse_temp_0 = num < 0;
    if _cse_temp_0 {
        return false;
    }
    (is_perfect_square(5 * num * num + 4)) || (is_perfect_square(5 * num * num - 4))
}
#[doc = "Test the Fibonacci functions."]
#[doc = " Depyler: verified panic-free"]
pub fn main() {
    let n = 10;
    println!(
        "{}",
        format!("Fibonacci({}) recursive: {}", n, fibonacci_recursive(n))
    );
    println!(
        "{}",
        format!("Fibonacci({}) iterative: {}", n, fibonacci_iterative(n))
    );
    println!(
        "{}",
        format!(
            "Fibonacci({}) memoized: {:?}",
            n,
            fibonacci_memoized(n, None)
        )
    );
    println!(
        "{}",
        format!("\nFirst {} Fibonacci numbers: {}", n, fibonacci_sequence(n))
    );
    println!("{}", "\nUsing generator:");
    for (i, fib) in fibonacci_generator(n).iter().cloned().enumerate() {
        let i = i as i32;
        println!("{}", format!("  F({:?}) = {:?}", i, fib));
    }
    let target = 21;
    let mut index = find_fibonacci_index(target);
    if index.is_some() {
        println!(
            "{}",
            format!("\n{} is at index {:?} in Fibonacci sequence", target, index)
        );
    } else {
        println!("{}", format!("\n{} is not in Fibonacci sequence", target));
    }
    let test_nums = vec![0, 1, 2, 3, 4, 5, 8, 13, 20, 21];
    println!("{}", "\nFibonacci number check:");
    for num in test_nums.iter().cloned() {
        println!("{}", format!("  {}: {}", num, is_fibonacci_number(num)));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_recursive_examples() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(-1), -1);
    }
    #[test]
    fn test_fibonacci_iterative_examples() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(-1), -1);
    }
    #[test]
    fn test_is_fibonacci_number_examples() {
        let _ = is_fibonacci_number(Default::default());
    }
}
