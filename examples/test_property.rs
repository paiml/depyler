#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn magnitude(&self) -> i32 {
        return self.x * self.x + self.y * self.y;
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_property() {
    let p = Point::new(3, 4);
    let m = p.magnitude;
    m
}
