use std::io::{self, Write};

#[warn(dead_code)]
struct Account {
    id: u8,
    name: String,
    balance: f64,
    pin: u16,
}

impl Account {
    pub fn create(name: String, pin: u16) -> Self {
        Self {
            id: 1,
            name,
            balance: 0.0,
            pin,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount < 1.0 {
            println!("Minimum withdrawal amount is $1.00.");
            return;
        }
        if amount > self.balance {
            println!("Insufficient funds.");
            return;
        }

        self.balance -= amount;
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = Account::create("John Doe".to_owned(), 1337);

    loop {
        println!("\nSelect an option:");
        println!("1: Check Balance");
        println!("2: Deposit");
        println!("3: Withdraw");
        println!("4: Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Current balance: ${:.2}", account.get_balance());
            }
            2 => {
                print!("Enter deposit amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read input");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount, please enter a number.");
                        continue;
                    }
                };
                account.deposit(amount);
                println!(
                    "Deposited ${:.2}. New balance: ${:.2}",
                    amount,
                    account.get_balance()
                );
            }
            3 => {
                print!("Enter withdrawal amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read input");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount, please enter a number.");
                        continue;
                    }
                };
                account.withdraw(amount);
                println!(
                    "Attempted to withdraw ${:.2}. New balance: ${:.2}",
                    amount,
                    account.get_balance()
                );
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
