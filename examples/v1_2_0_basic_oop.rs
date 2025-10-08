#[derive(Debug, Clone)] pub struct Point {
    pub x: i32, pub y: i32
}
impl Point {
    pub fn new(x: i32, y: i32)  -> Self {
    Self {
    x, y
}
} pub fn translate(& mut self, dx: i32, dy: i32) {
    self.x = self.x + dx;
    self.y = self.y + dy;
   
}
pub fn distance_squared(& mut self)  -> i32 {
    return self.x * self.x + self.y * self.y;
   
}
pub fn origin ()  -> i32 {
    return 0;
   
}
} #[derive(Debug, Clone)] pub struct Rectangle {
    pub width: i32, pub height: i32
}
impl Rectangle {
    pub fn new(width: i32, height: i32)  -> Self {
    Self {
    width, height
}
} pub fn area(& mut self)  -> i32 {
    return self.width * self.height;
   
}
pub fn perimeter(& mut self)  -> i32 {
    return 2 * self.width + self.height;
   
}
pub fn is_square(& mut self)  -> bool {
    return self.width == self.height;
   
}
} #[doc = "Test Point class"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_point()  -> DynamicType {
    let p = Point::new(3, 4);
    p.translate(1, 2);
    let dist_sq = p.distance_squared();
    return dist_sq;
   
}
#[doc = "Test Rectangle class"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_rectangle()  -> DynamicType {
    let rect = Rectangle::new(10, 20);
    let area = rect.area();
    let perim = rect.perimeter();
    let sq = Rectangle::ne📄 Source: examples/v1_2_0_basic_oop.py (1916 bytes)
📝 Output: examples/v1_2_0_basic_oop.rs (2006 bytes)
⏱️  Parse time: 11ms
📊 Throughput: 158.3 KB/s
⏱️  Total time: 11ms
e {
    let zero = Point.origin ();
    return zero;
   
}
#[doc = "Run all tests"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn main ()  -> DynamicType {
    let point_result = test_point();
    let rect_result = test_rectangle();
    let zero = Point.origin ();
    let static_result = zero;
    let _cse_temp_0 = point_result + rect_result + static_result;
    return _cse_temp_0
}