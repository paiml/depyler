#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance_from_origin(&self) -> f64 {
        return (self.x * self.x + self.y * self.y as f64).powf(0.5 as f64);
    }
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
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
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: i32,
}
impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
    pub fn greet(&self) -> String {
        return format!("Hello, my name is {}", self.name);
    }
    pub fn is_adult(&self) -> bool {
        return self.age >= 18;
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Test Point class functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_point() {
    let mut p = Point::new(3, 4);
    assert!(p.x == 3);
    assert!(p.y == 4);
    assert!(p.distance_from_origin() == 5.0);
    p.translate(1, 1);
    assert!(p.x == 4);
    assert!(p.y == 5);
}
#[doc = "Test Rectangle class functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_rectangle() {
    assert!(r.area() == 200);
    assert!(r.perimeter() == 60);
    assert!(!r.is_square());
    assert!(square.is_square());
}
#[doc = "Test Person dataclass functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_person() {
    assert!(
        p.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
            == "Alice".to_string()
    );
    assert!(p.age == 25);
    assert!(p.is_adult());
    assert!(p.greet() == "Hello, my name is Alice".to_string());
    assert!(!child.is_adult());
}
