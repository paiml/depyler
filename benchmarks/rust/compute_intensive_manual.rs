#!/usr/bin/env rust
//! Compute-intensive benchmark: Fibonacci calculation and sum operations.
//!
//! Manual Rust implementation matching benchmarks/python/compute_intensive.py
//! for fair performance comparison.

use std::collections::HashMap;

/// Calculate nth Fibonacci number iteratively.
fn fibonacci_iterative(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

/// Sum first 'limit' Fibonacci numbers.
fn sum_fibonacci_numbers(limit: i32) -> i32 {
    let mut total = 0;
    for i in 0..limit {
        total += fibonacci_iterative(i);
    }
    total
}

/// Calculate basic statistics on a list of numbers.
fn calculate_statistics(numbers: &[i32]) -> HashMap<String, i32> {
    if numbers.is_empty() {
        let mut map = HashMap::new();
        map.insert("count".to_string(), 0);
        map.insert("sum".to_string(), 0);
        map.insert("min".to_string(), 0);
        map.insert("max".to_string(), 0);
        return map;
    }

    let count = numbers.len() as i32;
    let mut total = 0;
    let mut min_val = numbers[0];
    let mut max_val = numbers[0];

    for &num in numbers {
        total += num;
        if num < min_val {
            min_val = num;
        }
        if num > max_val {
            max_val = num;
        }
    }

    let mut map = HashMap::new();
    map.insert("count".to_string(), count);
    map.insert("sum".to_string(), total);
    map.insert("min".to_string(), min_val);
    map.insert("max".to_string(), max_val);
    map
}

fn main() {
    let limits = vec![25, 30, 35];

    for limit in limits {
        let result = sum_fibonacci_numbers(limit);

        // Generate fibonacci sequence for statistics
        let mut fib_sequence = Vec::new();
        for i in 0..limit {
            fib_sequence.push(fibonacci_iterative(i));
        }

        let stats = calculate_statistics(&fib_sequence);

        println!(
            "Limit: {} | Sum: {} | Count: {} | Max: {}",
            limit,
            result,
            stats.get("count").unwrap_or(&0),
            stats.get("max").unwrap_or(&0)
        );
    }
}
