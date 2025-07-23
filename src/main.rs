#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) {
        if amount > 0 {
            self.balance += amount;
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: i32) {
        if amount > 0 && amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Withdrawal amount must be positive and less than or equal to the balance.");
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: Vec::new() }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Alice"));

    account.deposit(100);
    account.withdraw(50);

    bank.add_account(account);
    println!("{:#?}", bank);
}
