#[derive(Debug)]
struct Account {
    address: String,
}


impl Account {
    fn new(address: String) -> Self {
        Account {
            address
        }
    }
}

#[derive(Debug)]
struct Tx {
    from: Account,
    to: Account,
    value: u64,
    data: String,
}


impl Tx {
    fn new(from: Account, to: Account, value: u64, data: String) -> Self {
        Tx {
            from,
            to,
            value,
            data
        }
    }
    fn reward(self) -> Self {
        Tx {
            data: String::from("reward"),
            ..self
        }
    }
} 




fn main() {
    let sender = Account::new(String::from("0x158"));
    let receiver = Account::new(String::from("0x522"));

    let transaction_bundle = Tx::new(sender, receiver, 1000, String::from("")).reward();
    println!("{:?}", transaction_bundle);
    // let tx = Tx::new()
}
