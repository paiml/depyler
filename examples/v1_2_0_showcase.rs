#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
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
        self.balance = self.balance.clone() + amount;
        self.transaction_count = self.transaction_count.clone() + 1;
        return self.balance.clone();
    }
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if amount <= self.balance.clone() {
            self.balance = self.balance.clone() - amount;
            self.transaction_count = self.transaction_count.clone() + 1;
            return true;
        };
        return false;
    }
    pub fn calculate_interest(principal: f64, rate: f64, years: i32) -> f64 {
        return principal * rate * (years as f64);
    }
    pub fn summary(&self) -> String {
        return format!("{}: ${}", self.owner.clone(), self.balance.clone());
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
        return self.amount.clone() > 0.0;
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
        let interest = self.balance.clone() * self.interest_rate.clone();
        self.balance = self.balance.clone() + interest;
        return self.balance.clone();
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
    let mut account = BankAccount::new("Alice".to_string(), 1000.0);
    let new_balance = account.deposit(500.0);
    let success = account.withdraw(200.0);
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
pub fn demo_dataclass() -> bool {
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100.0);
    let valid = tx.is_valid();
    valid
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
    let balance = demo_instance_methods();
    let interest = demo_static_methods();
    let valid = demo_dataclass();
    let r#final = demo_all_features();
    let _ = r#final;
}
