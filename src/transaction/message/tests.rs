use rust_decimal::Decimal;

use crate::transaction::templates::Templates;
use crate::transaction::Transaction;
use super::Message;


#[test]
fn default_expenses_is_food() {
    let ts = Templates::new();
    let tr = Transaction::parse_message(&ts, "");
    assert_eq!(&tr.to_posting.account, "expenses:food");
}

#[test]
fn default_assets_is_cash() {
    let ts = Templates::new();
    let tr = Transaction::parse_message(&ts, "");
    assert_eq!(&tr.from_posting.account, "assets:cash");
}

#[test]
fn assets_can_be_abbreviated() {
    let ts = Templates::new();
    let tr = Transaction::parse_message(&ts, "stuff 3.40 bank-account");
    assert_eq!(&tr.from_posting.account, "assets:bank-account");
}

#[test]
fn expenses_can_be_abbreviated() {
    let ts = Templates::new();
    let tr = Transaction::parse_message(&ts, "stuff 3.40 bank-account life");
    assert_eq!(&tr.to_posting.account, "expenses:life");
}

#[test]
fn german_comma_is_accepted() {
    let ts = Templates::new();
    let tr = Transaction::parse_message(&ts, "stuff 3,40 bank-account life");
    assert_eq!(tr.from_posting.amount, -Decimal::new(340, 2));
}

#[test]
fn amount_is_negated() {
    let ts = Templates::new();
    let tr = Transaction::parse_message(&ts, "stuff 3.40 bank-account life");
    assert_eq!(tr.from_posting.amount, -Decimal::new(340, 2));
    assert_eq!(tr.to_posting.amount, Decimal::new(340, 2));
}
