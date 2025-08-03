#[derive(Debug, Clone)] pub struct Counter {
    pub count: i32
}
impl Counter {
    pub fn new(n: i32)  -> Self {
    Self {
    count: 0
}
} pub fn increment(& mut self) {
    self.count = self.count + 1;
   
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_counter()  -> DynamicType {
    let mut c = Counter::new(0);
    c.increment();
    return c.count
}