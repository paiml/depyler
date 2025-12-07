#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[derive(Debug, Clone)]
pub struct BankAccount {
    pub owner: String,
    pub balance: f64,
    pub transaction_count: i32,
}
impl BankAccount {
    pub fn new(owner: String, _initial_balance: f64) -> Self {
        Self {
            owner,
            balance: 0.0,
            transaction_count: 0,
        }
    }
    pub fn deposit(&mut self, amount: f64) -> f64 {
        self.balance = self.balance + amount;
        self.transaction_count = self.transaction_count + 1;
        return self.balance;
    }
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if amount <= self.balance {
            self.balance = self.balance - amount;
            self.transaction_count = self.transaction_count + 1;
            return true;
        };
        return false;
    }
    pub fn calculate_interest(principal: f64, rate: f64, years: i32) -> f64 {
        return principal * rate * (years as f64);
    }
    pub fn summary(&self) -> String {
        return format!("{}: ${}", self.owner, self.balance);
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub from_account: String,
    pub to_account: String,
    pub amount: f64,
}
impl Transaction {
    pub fn new(from_account: String, to_account: String, amount: f64) -> Self {
        Self {
            from_account,
            to_account,
            amount,
        }
    }
    pub fn is_valid(&self) -> bool {
        return self.amount > 0;
    }
}
#[derive(Debug, Clone)]
pub struct SavingsAccount {
    pub interest_rate: f64,
}
impl SavingsAccount {
    pub fn new(_owner: String, interest_rate: f64) -> Self {
        Self { interest_rate }
    }
    pub fn apply_interest(&mut self) -> f64 {
        let interest = self.balance * self.interest_rate;
        self.balance = self.balance + interest;
        return self.balance;
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Demonstrate instance methods and attribute access"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_instance_methods() {
    let account = BankAccount::new("Alice".to_string(), 1000.0);
    let balance = account.balance;
    let _ = balance;
}
#[doc = "Demonstrate static method calls"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_static_methods() {
    let interest = BankAccount::calculate_interest(1000.0, 0.05, 3);
    let _ = interest;
}
#[doc = "Demonstrate dataclass functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_dataclass() {
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100.0);
    let valid = tx.is_valid();
    let _ = valid;
}
#[doc = "Comprehensive demo of all v1.2.0 OOP features"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_all_features() {
    let mut checking = BankAccount::new("Charlie".to_string(), 5000.0);
    checking.deposit(1000.0);
    let transaction = Transaction::new("Charlie".to_string(), "Dana".to_string(), 500.0);
    if transaction.is_valid() {
        checking.withdraw(transaction.amount);
    }
    let future_value = BankAccount::calculate_interest(checking.balance, 0.03, 5);
    let _ = checking.balance + future_value;
}
#[doc = "Run all demonstrations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let _balance = demo_instance_methods();
    let _interest = demo_static_methods();
    let _valid = demo_dataclass();
    let r#final = demo_all_features();
    let _ = r#final;
}
