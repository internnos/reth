struct Account {
    address: String,
}

struct Tx {
    from: Account,
    to: Account,
    value: u64,
    data: String,
}

fn main() {
    println!("Hello, world!");
}
