fn main() {
    let mut account1 = BankAccount{
        account_number:String::from("123456"),
        holder_name:String::from("Furkan"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: String::from("135790"),
        holder_name: String::from("Joseph"),
        balance: 2000.0,
    };

    account1.deposit(1500.0);
    account2.withdraw(1000.0);

    println!("Account 1 Balance: ${:.2}", account1.balance());
    println!("Account 2 Balance: ${:.2}", account2.balance());
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount{
    account_number: String,
    holder_name: String,
    balance:f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: f64) {
        if 
        amount <= self.balance {
           self.balance -= amount; 
        }else {
            println!("You don't have enough balance to make this withdrawal");
        }
    }

    fn balance(&self)->f64 {
        self.balance
    }
}