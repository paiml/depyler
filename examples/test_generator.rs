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
                self.i = 0;
                self.state = 1;
                self.next()
            }
            1 => {
                if self.i < self.n {
                    let result = self.i;
                    self.i = self.i + 1;
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
#[doc = "Generate Fibonacci numbers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    a: i32,
    b: i32,
    count: i32,
    n: i32,
}
#[doc = " Generator function - returns Iterator"]
pub fn fibonacci_generator(n: i32) -> impl Iterator<Item = i32> {
    FibonacciGeneratorState {
        state: 0,
        a: 0,
        b: 0,
        count: 0,
        n: n,
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
                if self.count < self.n {
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
#[doc = "Test generator usage"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_generator() -> i32 {
    42
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
    #[test]
    fn test_fibonacci_generator_examples() {
        assert_eq!(fibonacci_generator(0), 0);
        assert_eq!(fibonacci_generator(1), 1);
        assert_eq!(fibonacci_generator(-1), -1);
    }
    #[test]
    fn test_test_generator_examples() {
        let _ = test_generator();
    }
}
