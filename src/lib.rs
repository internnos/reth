#[derive(Debug)]
pub struct Account {
    address: String,
}


impl Account {
    pub fn new(address: String) -> Self {
        Account {
            address
        }
    }
}

#[derive(Debug)]
pub struct Tx {
    from: Account,
    to: Account,
    value: u64,
    data: String,
}


impl Tx {
    pub fn new(from: Account, to: Account, value: u64, data: String) -> Self {
        Tx {
            from,
            to,
            value,
            data
        }
    }
    pub fn reward(self) -> Self {
        Tx {
            data: String::from("reward"),
            ..self
        }
    }
} 
