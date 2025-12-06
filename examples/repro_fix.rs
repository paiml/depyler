#[derive(Debug, Clone)]
pub struct Account {
    pub balance: f64,
}
impl Account {
    pub fn new(balance: f64) -> Self {
        Self { balance }
    }
    pub fn is_positive(&self) -> bool {
        return self.balance > 0f64;
    }
}
