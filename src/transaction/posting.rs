
//! A `Posting` as it occurres in `Transaction`s.

use std::fmt;
use rust_decimal::Decimal;

/// How to format a `Transaction`.
#[derive(Clone,Copy,Debug)]
pub enum FormatOption {
    /// Render with amount.
    WithAmount,
    /// Only render the account name.
    WithoutAmount
}

pub struct Posting {
    pub account: String,
    pub amount: Decimal,
}


impl Posting {
    pub fn render(&self, opts: FormatOption) -> String {
        use FormatOption::{*};
        match opts {
            WithAmount => format!("{}      {}", self.account, self.amount),
            WithoutAmount => format!("{}", self.account)
        }
    }
}

impl fmt::Display for Posting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render(FormatOption::WithAmount))
    }
}
