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
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: Vec::new() }
    }
}

fn print_account(account: Account) {
    println!("Account ID: {}, Holder: {}, Balance: {}", account.id, account.holder, account.balance);
}

fn main() {
    // TODO: Implement functionality to add accounts and manage the bank
}
