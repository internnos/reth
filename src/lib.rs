#[derive(Debug, PartialEq, PartialOrd)]
pub struct Account {
    pub address: String,
}


impl Account {
    pub fn new(address: String) -> Self {
        Account {
            address
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Tx {
    pub from: Account,
    pub to: Account,
    pub value: u64,
    pub data: String,
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
            from: Account { address: (String::from("vitalik.eth")) },
            data: String::from("reward"),
            ..self
        }
    }
} 
