
use super::{*};

use indoc::indoc;
use rust_decimal::Decimal;

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
