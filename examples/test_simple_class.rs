#[derive(Debug, Clone)] pub struct Point {
   
}
impl Point {
    pub fn new(x: i32, y: i32)  -> Self {
    Self {
    x, y
}
} pub fn move_by(& mut self, dx: i32, dy: i32) {
   
}
pub fn distance_to(& mut self, other: DynamicType)  -> f64 {
   
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_point()  -> DynamicType {
    let mut p1 = Point(0, 0);
    let mut p2 = Point(3, 4);
    p1.move_by(1, 1);
    let mut dist = p1.distance_to(p2);
    return dist
}