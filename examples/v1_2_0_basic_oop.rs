#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
    pub fn distance_squared(&self) -> i32 {
        return self.x * self.x + self.y * self.y;
    }
    pub fn origin() -> i32 {
        return 0;
    }
}
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}
impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    pub fn area(&self) -> i32 {
        return self.width * self.height;
    }
    pub fn perimeter(&self) -> i32 {
        return 2 * self.width + self.height;
    }
    pub fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}
#[doc = "Test Point class"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_point() {
    let mut p = Point::new(3, 4);
    p.translate(1, 2);
    let dist_sq = p.distance_squared();
    let _ = dist_sq;
}
#[doc = "Test Rectangle class"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_rectangle() {
    let rect = Rectangle::new(10, 20);
    let area = rect.area();
    let perim = rect.perimeter();
    let _square = rect.is_square();
    let sq = Rectangle::new(5, 5);
    let _is_sq = sq.is_square();
    let _ = area + perim;
}
#[doc = "Test static methods"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_static() {
    let zero = Point::origin();
    let _ = zero;
}
#[doc = "Run all tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let point_result = test_point();
    let rect_result = test_rectangle();
    let static_result = test_static();
    let _ = point_result + rect_result + static_result;
}
