#[derive(Debug, Clone)] pub struct Point {
    pub x: i32, pub y: i32
}
impl Point {
    pub fn new(x: i32, y: i32)  -> Self {
    Self {
    x, y
}
} pub fn move_by(& mut self, dx: i32, dy: i32) {
    self.x = self.x + dx;
    self.y = self.y + dy;
   
}
pub fn distance_to(& mut self, other: DynamicType)  -> f64 {
    let mut dx = self.x - other.x;
    let mut dy = self.y - other.y;
    return(dx * dx + dy * dy as f64).powf(0.5);
   
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_point()  -> DynamicType {
    let p1 = 📄 Source: examples/test_simple_class.py (690 bytes)
📝 Output: examples/test_simple_class.rs (699 bytes)
⏱️  Parse time: 9ms
📊 Throughput: 70.8 KB/s
⏱️  Total time: 9ms
