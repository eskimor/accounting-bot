// extern crate accounting_bot;
use accounting_bot::cmds::transaction::{*};

use std::io;
use std::convert::TryFrom;

fn main() {
    loop {
        println!("Please give me some command:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Reading user input failed");
        let msg = Message::parse(&input);
        println!("Your msg was: {:?}", msg);
        let tr: Transaction = TryFrom::try_from(msg).expect("Invalid transaction");
        println!("Resulting in the following transaction:\n{}", tr);
    }
}
