#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn distance_to(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy;
        if distance_squared == 0 {
            return 0;
        };
        let mut result = distance_squared / 2;
        for _ in 0..10 {
            let result = result + distance_squared / result / 2;
        }
        return result;
    }
}
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    pub fn area(&self) -> f64 {
        return self.width * self.height;
    }
    pub fn perimeter(&self) -> f64 {
        return 2 * self.width + self.height;
    }
    pub fn is_square(&self) -> bool {
        return self.width - self.height.abs() < 0.0001;
    }
}
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
    pub fn area(&self) -> f64 {
        let pi = 3.14159;
        return pi * self.radius * self.radius;
    }
    pub fn circumference(&self) -> f64 {
        let pi = 3.14159;
        return 2 * pi * self.radius;
    }
    pub fn contains_point(&self, point: Point) -> bool {
        let distance_squared = point.x * point.x + point.y * point.y;
        let radius_squared = self.radius * self.radius;
        return distance_squared <= radius_squared;
    }
}
#[doc = "Calculate triangle area"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn triangle_area(base: f64, height: f64) -> f64 {
    0.5 * base * height
}
#[doc = "Calculate triangle area using Heron's formula"]
#[doc = " Depyler: proven to terminate"]
pub fn triangle_area_heron(a: f64, b: f64, c: f64) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = a + b <= c;
    let _cse_temp_1 = a + c <= b;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    let _cse_temp_3 = b + c <= a;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let _cse_temp_5 = a + b + c;
    let _cse_temp_6 = (_cse_temp_5 as f64) / (2.0 as f64);
    let s = _cse_temp_6;
    let _cse_temp_7 = s * (s - a);
    let _cse_temp_8 = _cse_temp_7 * (s - b);
    let _cse_temp_9 = _cse_temp_8 * (s - c);
    let area_squared = _cse_temp_9;
    let _cse_temp_10 = area_squared <= 0.0;
    if _cse_temp_10 {
        return Ok(0.0);
    }
    let _cse_temp_11 = (area_squared as f64) / (2.0 as f64);
    let mut result = _cse_temp_11.clone();
    for __sanitized in 0..10 {
        result = Vector::from_vec(
            result
                + (area_squared as f64)
                    / (result as f64)
                        .as_slice()
                        .iter()
                        .map(|&x| x / 2.0)
                        .collect(),
        );
    }
    Ok(result)
}
#[doc = "Find intersection of two lines defined by point pairs"]
#[doc = " Depyler: proven to terminate"]
pub fn line_intersection<'b, 'l1, 'c, 'a>(
    p1: &'a Point,
    p2: &'b Point,
    p3: &'c Point,
    p4: &'l1 Point,
) -> Result<(bool, Point), Box<dyn std::error::Error>> {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);
    let (x4, y4) = (p4.x, p4.y);
    let _cse_temp_0 = (x1 - x2) * (y3 - y4);
    let _cse_temp_1 = (y1 - y2) * (x3 - x4);
    let denominator = _cse_temp_0 - _cse_temp_1;
    let _cse_temp_2 = denominator.abs();
    let _cse_temp_3 = _cse_temp_2 < 0.0001;
    if _cse_temp_3 {
        return Ok((false, Point::new(0.0, 0.0)));
    }
    let _cse_temp_4 = (x1 - x3) * (y3 - y4);
    let _cse_temp_5 = (y1 - y3) * (x3 - x4);
    let _cse_temp_6 = (_cse_temp_4 - _cse_temp_5 as f64) / (denominator as f64);
    let t = _cse_temp_6;
    let _cse_temp_7 = t * (x2 - x1);
    let intersection_x = x1 + _cse_temp_7;
    let _cse_temp_8 = t * (y2 - y1);
    let intersection_y = y1 + _cse_temp_8;
    Ok((true, Point::new(intersection_x, intersection_y)))
}
