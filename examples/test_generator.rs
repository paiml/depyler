#[doc = "A simple generator that yields numbers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct SimpleGeneratorState {
    state: usize,
    i: i32,
    n: i32,
}
#[doc = " Generator function - returns Iterator"]
pub fn simple_generator(n: i32) -> impl Iterator<Item = i32> {
    SimpleGeneratorState {
        state: 0,
        i: 0,
        n: n,
    }
}
impl Iterator for SimpleGeneratorState {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.state = 1;
                self.i = 0;
                while self.i < self.n {
                    return Some(self.i);
                    self.i = self.i + 1;
                }
                None
            }
            _ => None,
        }
    }
}
#[doc = "Generate Fibonacci numbers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    count: i32,
    n: i32,
}
#[doc = " Generator function - returns Iterator"]
pub fn fibonacci_generator(n: i32) -> impl Iterator<Item = i32> {
    FibonacciGeneratorState {
        state: 0,
        count: 0,
        n: n,
    }
}
impl Iterator for FibonacciGeneratorState {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.state = 1;
                let (mut a, mut b) = (0, 1);
                self.count = 0;
                while self.count < self.n {
                    return Some(a);
                    (a, b) = (b, a + b);
                    self.count = self.count + 1;
                }
                None
            }
            _ => None,
        }
    }
}
#[doc = "Test generator usage"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_generator() -> i32 {
    return 42 as i32;
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_simple_generator_examples() {
        assert_eq!(simple_generator(0), 0);
        assert_eq!(simple_generator(1), 1);
        assert_eq!(simple_generator(-1), -1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_generator_examples() {
        assert_eq!(fibonacci_generator(0), 0);
        assert_eq!(fibonacci_generator(1), 1);
        assert_eq!(fibonacci_generator(-1), -1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_generator_examples() {
        let _ = test_generator();
    }
}
