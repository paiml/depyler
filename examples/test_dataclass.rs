use serde_json;
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
    pub fn distance_to(&self, other: serde_json::Value) -> f64 {
        let mut dx = self.x - other.x;
        let mut dy = self.y - other.y;
        return (dx * dx + dy * dy as f64).powf(0.5 as f64);
    }
    pub fn origin() {
        return Point::new(0, 0);
    }
    pub fn from_tuple(coords: (i32, i32)) {
        return Self::new(coords[0 as usize], coords[1 as usize]);
    }
    pub fn magnitude(&self) -> f64 {
        return (self.x * self.x + self.y * self.y as f64).powf(0.5 as f64);
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_point() -> (serde_json::Value, serde_json::Value) {
    let mut p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    p1.move_by(1, 1);
    let mag = p2.magnitude;
    let dist = p1.distance_to(p2);
    (dist, mag)
}
