#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Return a 2D point as tuple."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_point() -> (f64, f64) {
    let x: f64 = 1.0;
    let y: f64 = 2.0;
    (x, y)
}
#[doc = "Use the point tuple."]
#[doc = " Depyler: proven to terminate"]
pub fn use_point() -> Result<f64, Box<dyn std::error::Error>> {
    let point: (f64, f64) = get_point();
    Ok(point.0 + point.1)
}
