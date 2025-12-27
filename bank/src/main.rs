#[derive(Debug)]
struct Account {
    id: i32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Account {
    fn new(id: i32, holder: String) -> Self {
        Self { id, holder, balance: 0 }
    }
}

impl Bank {
    fn new() -> Self {
        Self { accounts: vec![] }
    }
}

fn print_bank(bank: Bank) {
    println!("Bank: {:#?}", bank);
}

fn print_accounts(accounts: Vec<Account>) {
    println!("Account: {:#?}", accounts);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, "John".to_string());

    print_accounts(bank.accounts);
}
