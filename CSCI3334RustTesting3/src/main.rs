#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: if initial_balance >= 0.0 { initial_balance } else { 0.0 },
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new(100.0);
    account.deposit(50.0);
    println!("Current balance: {}", account.balance());

    account.withdraw(30.0);
    println!("Balance after withdrawal: {}", account.balance());
}
