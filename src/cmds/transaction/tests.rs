use rust_decimal::Decimal;
use chrono::naive::NaiveDate;
use indoc::indoc;

use super::{*};

#[test]
fn default_expenses_is_food() {
    let tr = mk_transaction("slkdjfsl 2,30");
    assert_eq!(&tr.to_account, "expenses:food");
}

#[test]
fn default_assets_is_cash() {
    let tr = mk_transaction("sdkajskl 2");
    assert_eq!(&tr.from_account, "assets:cash");
}

#[test]
fn assets_can_be_abbreviated() {
    let tr = mk_transaction("stuff 3.40 bank-account");
    assert_eq!(&tr.from_account, "assets:bank-account");
}

#[test]
fn expenses_can_be_abbreviated() {
    let tr = mk_transaction("stuff 3.40 bank-account life");
    assert_eq!(&tr.to_account, "expenses:life");
}

#[test]
fn german_comma_is_accepted() {
    let tr = mk_transaction("stuff 3,40 bank-account life");
    assert_eq!(tr.amount, Decimal::new(340, 2));
}


#[test]
fn transaction_string_has_expected_format() {
    let t = Transaction {
        description: String::from("Test"),
        date: NaiveDate::from_ymd(2020, 02, 23),
        amount: Decimal::new(314, 2),
        from_account: "assets:giro".to_string(),
        to_account: "expenses:other".to_string(),
    };
    let expected_output = indoc!("
        2020/02/23   Test
            assets:giro      -3.14 EUR
            expenses:other

        ");
    assert_eq!(format!("{}", t), expected_output);
}

fn mk_transaction(msg: &str) -> Transaction {
    Transaction::try_from(Message::parse(msg)).expect("Invalid Transaction")
}
