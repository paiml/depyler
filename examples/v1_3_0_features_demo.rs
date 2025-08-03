#[derive(Debug, Clone)] pub struct ResourceManager {
    pub name: String, pub is_open: bool
}
impl ResourceManager {
    pub fn new(name: String)  -> Self {
    Self {
    name, is_open: false
}
} pub fn __enter__(& mut self) {
    self.is_open = true;
    return self;
   
}
pub fn __exit__(& mut self, exc_type: DynamicType, exc_val: DynamicType, exc_tb: DynamicType) {
    self.is_open = false;
    return false;
   
}
pub fn use_resource(& mut self)  -> i32 {
    if self.is_open {
    return 42
};
    return 0;
   
}
} #[derive(Debug, Clone)] pub struct Counter {
    pub max_count: i32, pub count: i32
}
impl Counter {
    pub fn new(max_count: i32)  -> Self {
    Self {
    max_count, count: 0
}
} pub fn __iter__(& mut self) {
    return self;
   
}
pub fn __next__(& mut self)  -> i32 {
    if self.count<self.max_count {
    self.count = self.count + 1;
    return self.count
};
    return - 1;
   
}
} #[doc = "Demonstrate with statement support"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn demo_with_statement()  -> DynamicType {
    { let mut rm = ResourceManager::new("test".to_string());
    let mut result = rm.use_resource();
   
}
return result;
   
}
#[doc = "Demonstrate iterator protocol"] #[doc = " Depyler: verified panic-free"] pub fn demo_iterator()  -> DynamicType {
    let mut counter = Counter::new(3);
    let mut total = 0;
    let mut val = counter.__next__();
    while(val != - 1) {
    total  = (total + val);
    val = counter.__next__();
   
}
return total;
   
}
#[doc = "Run all v1.3.0 feature demos"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn main ()  -> DynamicType {
    let mut with_result = demo_with_statement();
    let mut iter_result = demo_iterator();
    return(with_result + iter_result)
}