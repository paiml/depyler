#[doc = "// TODO: Map Python module 'dataclasses'"]
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
    pub fn distance_to(&self, other: String) -> f64 {
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
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_point() -> (String, String) {
    let mut p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    p1.move_by(1, 1);
    let origin = Point::origin();
    let p3 = Point::from_tuple((5, 5));
    let mag = p2.magnitude;
    let dist = p1.distance_to(p2);
    (dist, mag)
}
