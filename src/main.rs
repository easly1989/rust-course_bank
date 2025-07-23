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

fn change_account(account: &mut Account) {
    account.balance += 10;
}

fn main() {
    let mut account = Account::new(1, "Alice".to_string());

    change_account(&mut account);

    print_account(account);
}
