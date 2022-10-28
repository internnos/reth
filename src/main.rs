use reth::{Account, Tx};


fn main() {
    let sender = Account::new(String::from("0x158"));
    let receiver = Account::new(String::from("0x522"));

    let transaction_bundle = Tx::new(sender, receiver, 1000, String::from("")).reward();
    println!("{:?}", transaction_bundle);
    // let tx = Tx::new()
}
