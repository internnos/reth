use reth::{Account, Tx};


#[test]
fn test_default_reward_off() {
    let sender = Account::new(String::from("0x158"));
    let receiver = Account::new(String::from("0x522"));

    let transaction_bundle = Tx::new(sender, receiver, 1000, String::from(""));
    let expected = Tx {
        from: Account {
            address: String::from("0x158"),
        },
        to: Account {
            address: String::from("0x522"),
        },
        value: 1000,
        data: String::from(""),
    };
    assert_eq!(transaction_bundle, expected);
}

#[test]
fn test_reward_on() {
    let sender = Account::new(String::from("0x158"));
    let receiver = Account::new(String::from("0x522"));

    let transaction_bundle = Tx::new(sender, receiver, 1000, String::from("")).reward();
    let expected = Tx {
        from: Account {
            address: String::from("vitalik.eth"),
        },
        to: Account {
            address: String::from("0x522"),
        },
        value: 1000,
        data: String::from("reward"),
    };
    assert_eq!(transaction_bundle, expected);
}