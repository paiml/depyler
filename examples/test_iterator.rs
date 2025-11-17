#[derive(Debug, Clone)]
pub struct Range {
    pub start: i32,
    pub stop: i32,
    pub current: i32,
}
impl Range {
    pub fn new(start: i32, stop: i32) -> Self {
        Self {
            start,
            stop,
            current: 0,
        }
    }
    pub fn __iter__(&self) -> &Self {
        return self;
    }
    pub fn __next__(&mut self) -> i32 {
        if self.current < self.stop {
            let mut value = self.current;
            self.current = self.current + 1;
            return value;
        } else {
            return -1;
        };
    }
}
#[doc = "Test custom iterator"]
#[doc = " Depyler: verified panic-free"]
pub fn test_custom_iterator() {
    let mut r = Range::new(0, 5);
    let mut total = 0;
    let mut value = r.__next__();
    while value != -1 {
        total = total + value;
        value = r.__next__();
    }
    total
}
#[doc = "Test for loop with iterator"]
#[doc = " Depyler: verified panic-free"]
pub fn test_for_with_iterator() {
    let mut r = Range::new(0, 5);
    let mut total = 0;
    for i in r.iter().cloned() {
        total = total + i;
    }
    total
}
