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

#[test]
fn transaction_string_has_expected_format() {
    let t = Transaction {
        description: String::from("Test"),
        date: NaiveDate::from_ymd(2020, 02, 23),
        from_posting: Posting {
            account: "assets:giro".to_string(),
            amount: Decimal::new(-314, 2),
        },
        to_posting: Posting {
            account: "expenses:other".to_string(),
            amount: Decimal::new(314, 2),
        },
    };
    let expected_output = indoc!("
        2020/02/23   Test
            assets:giro      -3.14
            expenses:other

        ");
    assert_eq!(format!("{}", t), expected_output);
}
